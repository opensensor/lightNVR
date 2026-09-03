#define _POSIX_C_SOURCE 200809L

#include <cjson/cJSON.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "core/config.h"
#include "database/db_core.h"
#include "database/db_system_health_incidents.h"
#include "telemetry/system_health.h"
#include "unity.h"
#include "utils/strings.h"
#include "web/api_handlers_health.h"
#include "web/api_handlers_system_health.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_system_health_api.db"

extern config_t g_config;

static cJSON *response_json(const http_response_t *response) {
    TEST_ASSERT_NOT_NULL(response->body);
    cJSON *root = cJSON_Parse((const char *)response->body);
    TEST_ASSERT_NOT_NULL(root);
    return root;
}

static void clear_incidents(void) {
    sqlite3_exec(get_db_handle(),
        "DELETE FROM system_health_incident_transitions;"
        "DELETE FROM system_health_incidents;"
        "DELETE FROM system_health_process_runs;", NULL, NULL, NULL);
}

void setUp(void) {
    g_config.web_auth_enabled = false;
    clear_incidents();
}

void tearDown(void) {}

static int collect_fixture(void *state,
                           const system_health_collect_context_t *context,
                           system_health_observation_sink_t *sink) {
    (void)state;
    system_health_observation_t observation;
    memset(&observation, 0, sizeof(observation));
    safe_strcpy(observation.metric, "host.cpu.busy_ratio",
                sizeof(observation.metric), 0);
    safe_strcpy(observation.resource_id, "host",
                sizeof(observation.resource_id), 0);
    observation.scope = SYSTEM_HEALTH_SCOPE_HOST;
    observation.sampled_monotonic_ms = context->monotonic_ms;
    observation.observed_wall_time_ms = context->wall_time_ms;
    system_health_observation_set_available(&observation, .25,
                                             SYSTEM_HEALTH_UNIT_RATIO);
    TEST_ASSERT_TRUE(system_health_observation_sink_append(sink, &observation));

    memset(&observation, 0, sizeof(observation));
    safe_strcpy(observation.metric, "device.temperature_celsius",
                sizeof(observation.metric), 0);
    safe_strcpy(observation.resource_id, "device:abc123",
                sizeof(observation.resource_id), 0);
    observation.scope = SYSTEM_HEALTH_SCOPE_DEVICE;
    observation.sampled_monotonic_ms = context->monotonic_ms;
    observation.observed_wall_time_ms = context->wall_time_ms;
    system_health_observation_set_unavailable(
        &observation, SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED);
    TEST_ASSERT_TRUE(system_health_observation_sink_append(sink, &observation));
    return 0;
}

void test_operational_health_is_unknown_before_first_baseline(void) {
    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    handle_get_system_health(&request, &response);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    cJSON *root = response_json(&response);
    TEST_ASSERT_EQUAL_STRING("unknown",
        cJSON_GetObjectItemCaseSensitive(root, "overall_state")->valuestring);
    cJSON *snapshot = cJSON_GetObjectItemCaseSensitive(root, "snapshot");
    TEST_ASSERT_FALSE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        snapshot, "available")));
    TEST_ASSERT_TRUE(cJSON_IsNull(cJSON_GetObjectItemCaseSensitive(
        snapshot, "sequence")));
    TEST_ASSERT_EQUAL_INT(0, cJSON_GetArraySize(
        cJSON_GetObjectItemCaseSensitive(root, "observations")));
    cJSON_Delete(root);
    http_response_free(&response);
}

void test_operational_health_serializes_snapshot_without_sampling_requests(void) {
    system_health_options_t options;
    system_health_options_defaults(&options);
    options.register_builtin_collectors = false;
    TEST_ASSERT_EQUAL_INT(0, system_health_init(&options));
    system_health_collector_t collector = {
        .name = "api_fixture",
        .scope = SYSTEM_HEALTH_SCOPE_HOST,
        .tier = SYSTEM_HEALTH_TIER_FAST,
        .interval_seconds = 10,
        .stale_after_seconds = 30,
        .collect = collect_fixture,
    };
    TEST_ASSERT_TRUE(system_health_register_collector(&collector));
    TEST_ASSERT_EQUAL_INT(0,
        system_health_collect_tier(SYSTEM_HEALTH_TIER_FAST));

    system_health_stats_t before;
    system_health_get_stats(&before);
    for (int request_index = 0; request_index < 2; ++request_index) {
        http_request_t request;
        http_response_t response;
        http_request_init(&request);
        http_response_init(&response);
        handle_get_system_health(&request, &response);
        TEST_ASSERT_EQUAL_INT(200, response.status_code);
        cJSON *root = response_json(&response);
        TEST_ASSERT_EQUAL_STRING("healthy",
            cJSON_GetObjectItemCaseSensitive(root,
                                              "overall_state")->valuestring);
        cJSON *observations = cJSON_GetObjectItemCaseSensitive(
            root, "observations");
        TEST_ASSERT_EQUAL_INT(2, cJSON_GetArraySize(observations));
        cJSON *unavailable = cJSON_GetArrayItem(observations, 1);
        TEST_ASSERT_TRUE(cJSON_IsNull(cJSON_GetObjectItemCaseSensitive(
            unavailable, "value")));
        TEST_ASSERT_EQUAL_STRING("permission_denied",
            cJSON_GetObjectItemCaseSensitive(unavailable,
                                              "capability")->valuestring);
        TEST_ASSERT_EQUAL_STRING("device:abc123",
            cJSON_GetObjectItemCaseSensitive(unavailable,
                                              "resource")->valuestring);
        TEST_ASSERT_EQUAL_INT(SYSTEM_HEALTH_CONDITION_COUNT,
            cJSON_GetArraySize(cJSON_GetObjectItemCaseSensitive(
                root, "thresholds")));
        cJSON_Delete(root);
        http_response_free(&response);
    }
    system_health_stats_t after;
    system_health_get_stats(&after);
    TEST_ASSERT_EQUAL_UINT64(before.generations_completed,
                             after.generations_completed);
    TEST_ASSERT_EQUAL_UINT64(before.collections_completed,
                             after.collections_completed);
}

static system_health_incident_signal_t incident_signal(
    const char *subject, int64_t observed_at) {
    system_health_incident_signal_t signal;
    memset(&signal, 0, sizeof(signal));
    signal.condition = SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_PREFAIL;
    safe_strcpy(signal.subject, subject, sizeof(signal.subject), 0);
    signal.scope = SYSTEM_HEALTH_SCOPE_DEVICE;
    signal.state = SYSTEM_HEALTH_STATE_CLOSED;
    signal.severity = SYSTEM_HEALTH_SEVERITY_WARNING;
    signal.observed_at_ms = observed_at;
    safe_strcpy(signal.observation_json,
        "{\"metric\":\"device.wear_ratio\",\"resource\":\"safe\","
        "\"value\":0.9,\"unit\":\"ratio\","
        "\"raw_path\":\"/dev/private\",\"serial\":\"secret\"}",
        sizeof(signal.observation_json), 0);
    signal.reconciliation = SYSTEM_HEALTH_RECONCILIATION_RECONCILED;
    safe_strcpy(signal.boot_id, "boot-a", sizeof(signal.boot_id), 0);
    safe_strcpy(signal.run_id, "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa",
                sizeof(signal.run_id), 0);
    return signal;
}

void test_incident_history_is_deterministic_bounded_and_privacy_safe(void) {
    const char *subjects[] = {"device:a", "device:b", "device:c"};
    for (int index = 0; index < 3; ++index) {
        system_health_incident_signal_t signal = incident_signal(
            subjects[index], 1000 + index);
        system_health_incident_record_t record;
        TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_OK,
            db_system_health_incident_apply(SYSTEM_HEALTH_INCIDENT_ONE_SHOT,
                                             &signal, &record));
    }

    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    safe_strcpy(request.query_string, "limit=2", sizeof(request.query_string), 0);
    handle_get_system_health_incidents(&request, &response);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    cJSON *root = response_json(&response);
    cJSON *items = cJSON_GetObjectItemCaseSensitive(root, "incidents");
    TEST_ASSERT_EQUAL_INT(2, cJSON_GetArraySize(items));
    TEST_ASSERT_EQUAL_STRING("device:c", cJSON_GetObjectItemCaseSensitive(
        cJSON_GetArrayItem(items, 0), "subject")->valuestring);
    const char *body = (const char *)response.body;
    TEST_ASSERT_NULL(strstr(body, "/dev/private"));
    TEST_ASSERT_NULL(strstr(body, "secret"));
    cJSON *next = cJSON_GetObjectItemCaseSensitive(root, "next_cursor");
    TEST_ASSERT_TRUE(cJSON_IsString(next));
    char cursor[96];
    safe_strcpy(cursor, next->valuestring, sizeof(cursor), 0);
    cJSON_Delete(root);
    http_response_free(&response);

    http_request_init(&request);
    http_response_init(&response);
    snprintf(request.query_string, sizeof(request.query_string),
             "limit=2&cursor=%s", cursor);
    handle_get_system_health_incidents(&request, &response);
    root = response_json(&response);
    items = cJSON_GetObjectItemCaseSensitive(root, "incidents");
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(items));
    TEST_ASSERT_EQUAL_STRING("device:a", cJSON_GetObjectItemCaseSensitive(
        cJSON_GetArrayItem(items, 0), "subject")->valuestring);
    cJSON_Delete(root);
    http_response_free(&response);
}

void test_incident_history_validates_auth_limit_and_cursor(void) {
    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    safe_strcpy(request.query_string, "limit=101",
                sizeof(request.query_string), 0);
    handle_get_system_health_incidents(&request, &response);
    TEST_ASSERT_EQUAL_INT(400, response.status_code);
    http_response_free(&response);

    http_request_init(&request);
    http_response_init(&response);
    safe_strcpy(request.query_string, "cursor=not-a-cursor",
                sizeof(request.query_string), 0);
    handle_get_system_health_incidents(&request, &response);
    TEST_ASSERT_EQUAL_INT(400, response.status_code);
    http_response_free(&response);

    g_config.web_auth_enabled = true;
    http_request_init(&request);
    http_response_init(&response);
    handle_get_system_health(&request, &response);
    TEST_ASSERT_TRUE(response.status_code == 401 || response.status_code == 403);
    http_response_free(&response);
    g_config.web_auth_enabled = false;
}

void test_operational_data_does_not_change_liveness_status(void) {
    system_health_incident_signal_t signal = incident_signal("device:risk", 2000);
    system_health_incident_record_t record;
    TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_OK,
        db_system_health_incident_apply(SYSTEM_HEALTH_INCIDENT_ONE_SHOT,
                                         &signal, &record));
    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    handle_get_health(&request, &response);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    cJSON *root = response_json(&response);
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        root, "healthy")));
    cJSON_Delete(root);
    http_response_free(&response);
}

int main(void) {
    unlink(TEST_DB_PATH);
    load_default_config(&g_config);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_operational_health_is_unknown_before_first_baseline);
    RUN_TEST(test_operational_health_serializes_snapshot_without_sampling_requests);
    RUN_TEST(test_incident_history_is_deterministic_bounded_and_privacy_safe);
    RUN_TEST(test_incident_history_validates_auth_limit_and_cursor);
    RUN_TEST(test_operational_data_does_not_change_liveness_status);
    int result = UNITY_END();
    system_health_shutdown();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
