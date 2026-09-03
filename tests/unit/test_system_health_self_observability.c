#define _POSIX_C_SOURCE 200809L

#include <cjson/cJSON.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

#include "core/config.h"
#include "core/event_envelope.h"
#include "core/mqtt_delivery_worker.h"
#include "core/mqtt_presence.h"
#include "database/db_core.h"
#include "database/db_event_destinations.h"
#include "database/db_event_outbox.h"
#include "telemetry/system_health.h"
#include "unity.h"
#include "web/api_handlers_system_health.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_health_self_observability.db"

extern config_t g_config;
char *api_metrics_render_self_observability(void);

typedef struct {
    pthread_mutex_t lock;
    pthread_cond_t condition;
    bool entered;
    bool release;
} blocking_collector_state_t;

static int blocking_collector(
    void *opaque, const system_health_collect_context_t *context,
    system_health_observation_sink_t *sink) {
    (void)context;
    (void)sink;
    blocking_collector_state_t *state = opaque;
    pthread_mutex_lock(&state->lock);
    state->entered = true;
    pthread_cond_broadcast(&state->condition);
    while (!state->release)
        pthread_cond_wait(&state->condition, &state->lock);
    pthread_mutex_unlock(&state->lock);
    return 0;
}

static void *collect_fast(void *unused) {
    (void)unused;
    (void)system_health_collect_tier(SYSTEM_HEALTH_TIER_FAST);
    return NULL;
}

static int failing_collector(
    void *state, const system_health_collect_context_t *context,
    system_health_observation_sink_t *sink) {
    (void)state;
    (void)context;
    (void)sink;
    struct timespec delay = {.tv_sec = 0, .tv_nsec = 3 * 1000 * 1000};
    nanosleep(&delay, NULL);
    return -1;
}

static event_envelope_t pending_event(time_t now) {
    cJSON *data = cJSON_CreateObject();
    cJSON_AddStringToObject(data, "level", "warning");
    cJSON_AddNumberToObject(data, "used_percent", 85.0);
    event_envelope_t event;
    char error[128] = {0};
    TEST_ASSERT_EQUAL_INT(0, event_envelope_create(
        &event, "io.lightnvr.storage.pressure.v1",
        "urn:lightnvr:11111111-1111-4111-8111-111111111111",
        "system/storage", now, data, error, sizeof(error)));
    cJSON_Delete(data);
    return event;
}

void setUp(void) {
    system_health_shutdown();
    mqtt_presence_reset();
    sqlite3_exec(get_db_handle(), "DELETE FROM event_outbox;", NULL, NULL,
                 NULL);
    sqlite3_exec(get_db_handle(), "DELETE FROM event_routes;", NULL, NULL,
                 NULL);
    sqlite3_exec(get_db_handle(), "DELETE FROM event_destinations;", NULL,
                 NULL, NULL);
    g_config.web_auth_enabled = false;
}

void tearDown(void) {
    system_health_shutdown();
    mqtt_presence_reset();
}

void test_collector_runtime_stats_and_api_reads_are_bounded_and_read_only(void) {
    system_health_options_t options;
    system_health_options_defaults(&options);
    options.register_builtin_collectors = false;
    strcpy(options.hardware_provider, "disabled");
    options.collector_deadline_ms[SYSTEM_HEALTH_TIER_FAST] = 1U;
    TEST_ASSERT_EQUAL_INT(0, system_health_init(&options));
    system_health_collector_t collector;
    memset(&collector, 0, sizeof(collector));
    strcpy(collector.name, "bounded_probe");
    collector.scope = SYSTEM_HEALTH_SCOPE_HOST;
    collector.tier = SYSTEM_HEALTH_TIER_FAST;
    collector.interval_seconds = 1U;
    collector.stale_after_seconds = 1U;
    collector.collect = failing_collector;
    TEST_ASSERT_TRUE(system_health_register_collector(&collector));
    TEST_ASSERT_EQUAL_INT(-1,
        system_health_collect_tier(SYSTEM_HEALTH_TIER_FAST));

    system_health_collector_stats_t before[2];
    size_t count = system_health_collector_stats_copy(before, 2U);
    TEST_ASSERT_EQUAL_UINT64(1U, count);
    TEST_ASSERT_EQUAL_STRING("bounded_probe", before[0].name);
    TEST_ASSERT_EQUAL_UINT64(1U, before[0].attempts);
    TEST_ASSERT_EQUAL_UINT64(1U, before[0].completions);
    TEST_ASSERT_EQUAL_UINT64(1U, before[0].failures);
    TEST_ASSERT_EQUAL_UINT64(1U, before[0].timeouts);
    TEST_ASSERT_TRUE(before[0].stale);

    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    handle_get_system_health(&request, &response);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    cJSON *root = cJSON_Parse((const char *)response.body);
    TEST_ASSERT_NOT_NULL(root);
    cJSON *self = cJSON_GetObjectItemCaseSensitive(
        root, "self_observability");
    TEST_ASSERT_TRUE(cJSON_IsObject(self));
    cJSON *collectors = cJSON_GetObjectItemCaseSensitive(self, "collectors");
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(collectors));
    TEST_ASSERT_EQUAL_STRING("bounded_probe",
        cJSON_GetObjectItemCaseSensitive(cJSON_GetArrayItem(collectors, 0),
                                         "collector")->valuestring);
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(
        cJSON_GetObjectItemCaseSensitive(root, "recent_samples")));
    cJSON_Delete(root);
    http_response_free(&response);

    system_health_collector_stats_t after[2];
    TEST_ASSERT_EQUAL_UINT64(1U,
        system_health_collector_stats_copy(after, 2U));
    TEST_ASSERT_EQUAL_UINT64(before[0].attempts, after[0].attempts);
    TEST_ASSERT_EQUAL_UINT64(before[0].completions, after[0].completions);
}

void test_pending_only_destination_is_visible_locally_and_in_prometheus(void) {
    TEST_ASSERT_EQUAL_INT(0, mqtt_presence_configure(
        "private/topic", "11111111-1111-4111-8111-111111111111",
        "22222222-2222-4222-8222-222222222222", "boot-a", "host",
        "test"));
    int64_t now = (int64_t)time(NULL);
    event_envelope_t event = pending_event((time_t)now);
    TEST_ASSERT_EQUAL_INT(EVENT_OUTBOX_ENQUEUED,
        db_event_outbox_enqueue(&event, MQTT_EVENT_OUTBOX_DESTINATION,
            "private/topic/v1/events/storage/system", NULL, NULL, NULL));
    event_outbox_stats_t before;
    TEST_ASSERT_EQUAL_INT(0,
        db_event_outbox_get_stats(NULL, now, &before));

    char *metrics = api_metrics_render_self_observability();
    TEST_ASSERT_NOT_NULL(metrics);
    TEST_ASSERT_NOT_NULL(strstr(metrics,
        "lightnvr_event_outbox_rows{destination=\"all\",state=\"pending\"} 1\n"));
    TEST_ASSERT_NOT_NULL(strstr(metrics,
        "lightnvr_event_delivery_degraded 1\n"));
    TEST_ASSERT_NOT_NULL(strstr(metrics,
        "lightnvr_event_delivery_circular_report_path 1\n"));
    TEST_ASSERT_NULL(strstr(metrics, "private/topic"));
    free(metrics);

    event_outbox_stats_t after;
    TEST_ASSERT_EQUAL_INT(0,
        db_event_outbox_get_stats(NULL, now, &after));
    TEST_ASSERT_EQUAL_INT64(before.pending_rows, after.pending_rows);
    TEST_ASSERT_EQUAL_INT64(before.total_bytes, after.total_bytes);
    event_envelope_clear(&event);
}

void test_stats_copy_observes_busy_and_overlap_without_waiting_for_collector(void) {
    system_health_options_t options;
    system_health_options_defaults(&options);
    options.register_builtin_collectors = false;
    strcpy(options.hardware_provider, "disabled");
    blocking_collector_state_t state = {
        .lock = PTHREAD_MUTEX_INITIALIZER,
        .condition = PTHREAD_COND_INITIALIZER,
    };
    system_health_collector_t collector;
    memset(&collector, 0, sizeof(collector));
    strcpy(collector.name, "blocking_probe");
    collector.scope = SYSTEM_HEALTH_SCOPE_HOST;
    collector.tier = SYSTEM_HEALTH_TIER_FAST;
    collector.interval_seconds = 1U;
    collector.stale_after_seconds = 3U;
    collector.state = &state;
    collector.collect = blocking_collector;
    TEST_ASSERT_EQUAL_INT(0, system_health_init(&options));
    TEST_ASSERT_TRUE(system_health_register_collector(&collector));

    pthread_t thread;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&thread, NULL, collect_fast, NULL));
    pthread_mutex_lock(&state.lock);
    while (!state.entered)
        pthread_cond_wait(&state.condition, &state.lock);
    pthread_mutex_unlock(&state.lock);

    system_health_collector_stats_t stats[1];
    TEST_ASSERT_EQUAL_UINT64(1U,
        system_health_collector_stats_copy(stats, 1U));
    TEST_ASSERT_TRUE(stats[0].busy);
    TEST_ASSERT_EQUAL_UINT64(1U, stats[0].attempts);
    TEST_ASSERT_EQUAL_INT(0,
        system_health_collect_tier(SYSTEM_HEALTH_TIER_FAST));
    TEST_ASSERT_EQUAL_UINT64(1U,
        system_health_collector_stats_copy(stats, 1U));
    TEST_ASSERT_EQUAL_UINT64(1U, stats[0].overlap_skips);

    pthread_mutex_lock(&state.lock);
    state.release = true;
    pthread_cond_broadcast(&state.condition);
    pthread_mutex_unlock(&state.lock);
    TEST_ASSERT_EQUAL_INT(0, pthread_join(thread, NULL));
    TEST_ASSERT_EQUAL_UINT64(1U,
        system_health_collector_stats_copy(stats, 1U));
    TEST_ASSERT_FALSE(stats[0].busy);
    TEST_ASSERT_EQUAL_UINT64(1U, stats[0].completions);
    pthread_cond_destroy(&state.condition);
    pthread_mutex_destroy(&state.lock);
}

void test_enabled_profile_without_runtime_client_is_degraded_and_private(void) {
    event_destination_t destination;
    memset(&destination, 0, sizeof(destination));
    strcpy(destination.name, "Secret operations bridge");
    destination.enabled = true;
    strcpy(destination.destination_type, "mqtt");
    strcpy(destination.broker_host, "secret-broker.example.test");
    destination.broker_port = 1883;
    strcpy(destination.client_id, "private-client-id");
    strcpy(destination.topic_template,
           EVENT_DESTINATION_DEFAULT_TOPIC_TEMPLATE);
    strcpy(destination.tls_mode, "disabled");
    destination.keepalive_seconds = 60;
    destination.qos = 1;
    TEST_ASSERT_EQUAL_INT(DB_EVENT_DESTINATION_OK,
        db_event_destination_create(&destination, NULL));

    char *metrics = api_metrics_render_self_observability();
    TEST_ASSERT_NOT_NULL(metrics);
    char expected[160];
    snprintf(expected, sizeof(expected),
        "lightnvr_event_destination_connected{destination=\"%s\"} 0\n",
        destination.uuid);
    TEST_ASSERT_NOT_NULL(strstr(metrics, expected));
    TEST_ASSERT_NOT_NULL(strstr(metrics,
        "lightnvr_event_delivery_degraded 1\n"));
    TEST_ASSERT_NOT_NULL(strstr(metrics,
        "lightnvr_event_delivery_circular_report_path 1\n"));
    TEST_ASSERT_NULL(strstr(metrics, destination.name));
    TEST_ASSERT_NULL(strstr(metrics, destination.broker_host));
    TEST_ASSERT_NULL(strstr(metrics, destination.client_id));
    free(metrics);

    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    handle_get_system_health(&request, &response);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    TEST_ASSERT_NOT_NULL(strstr((const char *)response.body, destination.uuid));
    TEST_ASSERT_NOT_NULL(strstr((const char *)response.body,
                                "\"last_failure\":\"configuration\""));
    TEST_ASSERT_NULL(strstr((const char *)response.body, destination.name));
    TEST_ASSERT_NULL(strstr((const char *)response.body,
                            destination.broker_host));
    TEST_ASSERT_NULL(strstr((const char *)response.body,
                            destination.client_id));
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
    RUN_TEST(test_collector_runtime_stats_and_api_reads_are_bounded_and_read_only);
    RUN_TEST(test_pending_only_destination_is_visible_locally_and_in_prometheus);
    RUN_TEST(test_stats_copy_observes_busy_and_overlap_without_waiting_for_collector);
    RUN_TEST(test_enabled_profile_without_runtime_client_is_degraded_and_private);
    int result = UNITY_END();
    system_health_shutdown();
    mqtt_presence_reset();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
