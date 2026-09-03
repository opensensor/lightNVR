/**
 * @file test_event_outbox.c
 * @brief Durable event outbox capacity, lease, retry, and expiry tests.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

#include <cjson/cJSON.h>
#include <sqlite3.h>

#include "core/config.h"
#include "core/event_bus.h"
#include "core/event_envelope.h"
#include "core/event_identity.h"
#include "core/event_producers.h"
#include "core/event_router.h"
#include "database/db_core.h"
#include "database/db_event_outbox.h"
#include "unity.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_event_outbox.db"
#define INSTALLATION_SOURCE \
    "urn:lightnvr:11111111-1111-4111-8111-111111111111"
#define CAMERA_SUBJECT "camera/22222222-2222-4222-8222-222222222222"
#define DESTINATION "mqtt:default"
#define TOPIC "lightnvr/v1/events/io.lightnvr.detection.object.v1/camera"

static event_envelope_t captured_event;
static int captured_event_count;

static int capture_event(const event_envelope_t *event, void *context) {
    (void)context;
    event_envelope_clear(&captured_event);
    if (event_envelope_clone(&captured_event, event, NULL, 0) != 0) return -1;
    captured_event_count++;
    return 0;
}

static event_envelope_t detection_event(const char *label, time_t occurred_at) {
    cJSON *data = cJSON_CreateObject();
    cJSON_AddStringToObject(data, "stream_name", "North Door");
    cJSON_AddNumberToObject(data, "count", 1);
    cJSON *detections = cJSON_AddArrayToObject(data, "detections");
    cJSON *detection = cJSON_CreateObject();
    cJSON_AddStringToObject(detection, "label", label);
    cJSON_AddNumberToObject(detection, "confidence", 0.9);
    cJSON_AddNumberToObject(detection, "x", 0.1);
    cJSON_AddNumberToObject(detection, "y", 0.2);
    cJSON_AddNumberToObject(detection, "width", 0.3);
    cJSON_AddNumberToObject(detection, "height", 0.4);
    cJSON_AddItemToArray(detections, detection);
    event_envelope_t event;
    char error[256] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, event_envelope_create(&event,
                                 "io.lightnvr.detection.object.v1",
                                 INSTALLATION_SOURCE, CAMERA_SUBJECT,
                                 occurred_at, data, error, sizeof(error)));
    cJSON_Delete(data);
    return event;
}

static event_envelope_t storage_event(time_t occurred_at) {
    cJSON *data = cJSON_CreateObject();
    cJSON_AddStringToObject(data, "level", "critical");
    cJSON_AddNumberToObject(data, "used_percent", 96.0);
    event_envelope_t event;
    char error[256] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, event_envelope_create(&event,
                                 "io.lightnvr.storage.pressure.v1",
                                 INSTALLATION_SOURCE, "system/storage",
                                 occurred_at, data, error, sizeof(error)));
    cJSON_Delete(data);
    return event;
}

static event_envelope_t health_alert_event(time_t occurred_at,
                                           event_severity_t severity,
                                           const char *event_id) {
    cJSON *data = cJSON_CreateObject();
    cJSON_AddStringToObject(data, "incident_id",
                            "33333333-3333-4333-8333-333333333333");
    cJSON_AddStringToObject(data, "code", "memory.available_low");
    cJSON_AddStringToObject(data, "scope", "host");
    cJSON_AddStringToObject(data, "resource", "host");
    cJSON_AddStringToObject(data, "state", "open");
    cJSON_AddStringToObject(data, "severity", event_severity_name(severity));
    cJSON_AddObjectToObject(data, "observed");
    cJSON_AddObjectToObject(data, "threshold");
    event_envelope_t event;
    char error[256] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, event_envelope_create_with_severity_and_id(
               &event, "io.lightnvr.system.health_alert.v1",
               INSTALLATION_SOURCE, "system/host", occurred_at, data,
               severity, event_id, error, sizeof(error)));
    cJSON_Delete(data);
    return event;
}

void setUp(void) {
    event_bus_shutdown(false);
    (void)event_bus_unsubscribe("capture-health");
    event_identity_shutdown();
    event_router_shutdown();
    event_envelope_clear(&captured_event);
    captured_event_count = 0;
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK, sqlite3_exec(get_db_handle(), "DELETE FROM event_outbox;",
                                NULL, NULL, NULL));
}

void tearDown(void) {
    event_bus_shutdown(false);
    (void)event_bus_unsubscribe("capture-health");
    event_identity_shutdown();
    event_router_shutdown();
    event_envelope_clear(&captured_event);
}

void test_enqueue_round_trip_and_duplicate_identity(void) {
    time_t now = time(NULL);
    event_envelope_t event = detection_event("person", now);
    char error[256] = {0};
    char *serialized = event_envelope_serialize(&event, error, sizeof(error));
    TEST_ASSERT_NOT_NULL(serialized);
    int64_t row_id = 0;
    int shed = -1;
    TEST_ASSERT_EQUAL_INT(
        EVENT_OUTBOX_ENQUEUED,
        db_event_outbox_enqueue(&event, DESTINATION, TOPIC, NULL,
                                &row_id, &shed));
    TEST_ASSERT_GREATER_THAN_INT64(0, row_id);
    TEST_ASSERT_EQUAL_INT(0, shed);

    int64_t duplicate_id = 0;
    TEST_ASSERT_EQUAL_INT(
        EVENT_OUTBOX_DUPLICATE,
        db_event_outbox_enqueue(&event, DESTINATION, TOPIC, NULL,
                                &duplicate_id, NULL));
    TEST_ASSERT_EQUAL_INT64(row_id, duplicate_id);
    TEST_ASSERT_EQUAL_INT(
        EVENT_OUTBOX_ENQUEUED,
        db_event_outbox_enqueue(&event, "mqtt:secondary", TOPIC, NULL,
                                NULL, NULL));

    event_outbox_item_t item;
    TEST_ASSERT_EQUAL_INT(
        1, db_event_outbox_claim_due(DESTINATION, now, 30, &item));
    TEST_ASSERT_EQUAL_STRING(event.id, item.event_id);
    TEST_ASSERT_EQUAL_STRING(event.source, item.event_source);
    TEST_ASSERT_EQUAL_STRING(event.type, item.event_type);
    TEST_ASSERT_EQUAL_STRING(event.subject, item.subject);
    TEST_ASSERT_EQUAL_STRING(TOPIC, item.topic);
    TEST_ASSERT_EQUAL_STRING(serialized, item.envelope_json);
    TEST_ASSERT_EQUAL_INT(1, item.attempt_count);
    db_event_outbox_item_clear(&item);
    free(serialized);
    event_envelope_clear(&event);
}

void test_dynamic_health_severity_and_persisted_identity_survive_outbox(void) {
    time_t now = time(NULL);
    const char *event_id = "44444444-4444-4444-8444-444444444444";
    event_envelope_t event = health_alert_event(
        now, EVENT_SEVERITY_ERROR, event_id);
    int64_t row_id = 0;
    TEST_ASSERT_EQUAL_INT(
        EVENT_OUTBOX_ENQUEUED,
        db_event_outbox_enqueue(&event, DESTINATION,
                                "lightnvr/v1/events/system-health", NULL,
                                &row_id, NULL));
    TEST_ASSERT_EQUAL_STRING(event_id, event.id);
    int64_t duplicate_id = 0;
    TEST_ASSERT_EQUAL_INT(
        EVENT_OUTBOX_DUPLICATE,
        db_event_outbox_enqueue(&event, DESTINATION,
                                "lightnvr/v1/events/system-health", NULL,
                                &duplicate_id, NULL));
    TEST_ASSERT_EQUAL_INT64(row_id, duplicate_id);

    event_outbox_item_t item;
    TEST_ASSERT_EQUAL_INT(
        1, db_event_outbox_claim_due(DESTINATION, now, 30, &item));
    TEST_ASSERT_EQUAL_STRING(event_id, item.event_id);
    TEST_ASSERT_EQUAL_INT(EVENT_SEVERITY_ERROR, item.severity);
    db_event_outbox_item_clear(&item);
    event_envelope_clear(&event);
}

void test_health_transition_producer_normalizes_alert_and_recovery(void) {
    bool mqtt_enabled = g_config.mqtt_enabled;
    g_config.mqtt_enabled = false;
    TEST_ASSERT_EQUAL_INT(0, event_identity_init());
    TEST_ASSERT_EQUAL_INT(
        0, event_bus_subscribe("capture-health", capture_event, NULL));
    TEST_ASSERT_EQUAL_INT(0, event_bus_init(0, 0));

    system_health_transition_t transition;
    memset(&transition, 0, sizeof(transition));
    transition.action = SYSTEM_HEALTH_INCIDENT_OPEN;
    strcpy(transition.incident_id,
           "33333333-3333-4333-8333-333333333333");
    strcpy(transition.event_id, "55555555-5555-4555-8555-555555555555");
    transition.condition = SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW;
    strcpy(transition.subject, "host");
    transition.scope = SYSTEM_HEALTH_SCOPE_HOST;
    transition.state = SYSTEM_HEALTH_STATE_OPEN;
    transition.severity = SYSTEM_HEALTH_SEVERITY_ERROR;
    transition.observation.value_valid = true;
    transition.observation.value = 0.08;
    transition.observation.unit = SYSTEM_HEALTH_UNIT_RATIO;
    transition.threshold_direction = SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE;
    transition.threshold_value = 0.10;
    transition.threshold_for_ms = 120000;
    transition.first_observed_at_ms = INT64_C(1786991300) * 1000;
    transition.observed_at_ms = INT64_C(1786991400) * 1000;
    transition.persisted = true;
    char error[256] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, event_producer_publish_system_health_transition(
               &transition, "55555555-5555-4555-8555-555555555555",
               error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(0, event_bus_wait_until_idle(2000));
    TEST_ASSERT_EQUAL_INT(1, captured_event_count);
    TEST_ASSERT_EQUAL_STRING(
        "io.lightnvr.system.health_alert.v1", captured_event.type);
    TEST_ASSERT_EQUAL_STRING("55555555-5555-4555-8555-555555555555",
                             captured_event.id);
    TEST_ASSERT_EQUAL_INT(EVENT_SEVERITY_ERROR, captured_event.severity);
    TEST_ASSERT_EQUAL_STRING(
        transition.incident_id,
        cJSON_GetObjectItemCaseSensitive(captured_event.data,
                                         "incident_id")->valuestring);
    TEST_ASSERT_EQUAL_STRING(
        "memory.available_low",
        cJSON_GetObjectItemCaseSensitive(captured_event.data,
                                         "code")->valuestring);
    TEST_ASSERT_EQUAL_STRING(
        "error",
        cJSON_GetObjectItemCaseSensitive(captured_event.data,
                                         "severity")->valuestring);

    transition.action = SYSTEM_HEALTH_INCIDENT_RECOVER;
    strcpy(transition.event_id, "66666666-6666-4666-8666-666666666666");
    transition.previous_severity = SYSTEM_HEALTH_SEVERITY_ERROR;
    transition.state = SYSTEM_HEALTH_STATE_CLOSED;
    transition.incident_duration_ms = 100000;
    transition.observed_at_ms += 100000;
    TEST_ASSERT_EQUAL_INT(
        0, event_producer_publish_system_health_transition(
               &transition, "66666666-6666-4666-8666-666666666666",
               error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(0, event_bus_wait_until_idle(2000));
    TEST_ASSERT_EQUAL_INT(2, captured_event_count);
    TEST_ASSERT_EQUAL_STRING(
        "io.lightnvr.system.health_recovered.v1", captured_event.type);
    TEST_ASSERT_EQUAL_INT(EVENT_SEVERITY_INFO, captured_event.severity);
    TEST_ASSERT_EQUAL_STRING(
        "error",
        cJSON_GetObjectItemCaseSensitive(captured_event.data,
                                         "previous_severity")->valuestring);
    TEST_ASSERT_EQUAL_INT64(
        100000,
        (int64_t)cJSON_GetObjectItemCaseSensitive(
            captured_event.data, "duration_ms")->valuedouble);

    TEST_ASSERT_EQUAL_INT(
        0, event_producer_publish_storage_pressure(
               "warning", "normal", 85.0, UINT64_C(1024), 1786991600,
               error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(0, event_bus_wait_until_idle(2000));
    TEST_ASSERT_EQUAL_INT(3, captured_event_count);
    TEST_ASSERT_EQUAL_STRING("io.lightnvr.storage.pressure.v1",
                             captured_event.type);
    TEST_ASSERT_EQUAL_INT(EVENT_SEVERITY_WARNING, captured_event.severity);
    TEST_ASSERT_EQUAL_STRING(
        "warning",
        cJSON_GetObjectItemCaseSensitive(captured_event.data,
                                         "level")->valuestring);

    event_bus_shutdown(true);
    TEST_ASSERT_EQUAL_INT(0, event_bus_unsubscribe("capture-health"));
    event_identity_shutdown();
    g_config.mqtt_enabled = mqtt_enabled;
}

void test_health_transition_producer_replay_is_durable_and_idempotent(void) {
    bool mqtt_enabled = g_config.mqtt_enabled;
    char topic_prefix[sizeof(g_config.mqtt_topic_prefix)];
    strcpy(topic_prefix, g_config.mqtt_topic_prefix);
    g_config.mqtt_enabled = true;
    strcpy(g_config.mqtt_topic_prefix, "lightnvr-test");
    TEST_ASSERT_EQUAL_INT(0, event_identity_init());

    time_t now = time(NULL);
    system_health_transition_t transition;
    memset(&transition, 0, sizeof(transition));
    transition.action = SYSTEM_HEALTH_INCIDENT_OPEN;
    strcpy(transition.incident_id,
           "33333333-3333-4333-8333-333333333333");
    strcpy(transition.event_id, "77777777-7777-4777-8777-777777777777");
    transition.condition = SYSTEM_HEALTH_CONDITION_THERMAL_HIGH;
    strcpy(transition.subject, "host");
    transition.scope = SYSTEM_HEALTH_SCOPE_HOST;
    transition.state = SYSTEM_HEALTH_STATE_OPEN;
    transition.severity = SYSTEM_HEALTH_SEVERITY_WARNING;
    transition.observation.value_valid = true;
    transition.observation.value = 80.0;
    transition.observation.unit = SYSTEM_HEALTH_UNIT_CELSIUS;
    transition.threshold_direction = SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE;
    transition.threshold_value = 78.0;
    transition.first_observed_at_ms = (int64_t)now * 1000;
    transition.observed_at_ms = (int64_t)now * 1000;
    transition.persisted = true;
    char error[256] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, event_producer_publish_system_health_transition(
               &transition, transition.event_id, error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(
        0, event_producer_publish_system_health_transition(
               &transition, transition.event_id, error, sizeof(error)));

    event_outbox_item_t item;
    TEST_ASSERT_EQUAL_INT(
        1, db_event_outbox_claim_due(DESTINATION, now, 30, &item));
    TEST_ASSERT_EQUAL_STRING(transition.event_id, item.event_id);
    TEST_ASSERT_EQUAL_INT(EVENT_SEVERITY_WARNING, item.severity);
    db_event_outbox_item_clear(&item);
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_claim_due(DESTINATION, now, 30, &item));

    event_identity_shutdown();
    g_config.mqtt_enabled = mqtt_enabled;
    strcpy(g_config.mqtt_topic_prefix, topic_prefix);
}

void test_retry_schedule_and_stale_lease_recovery(void) {
    int64_t now = (int64_t)time(NULL);
    event_envelope_t first = detection_event("vehicle", (time_t)now);
    TEST_ASSERT_EQUAL_INT(
        EVENT_OUTBOX_ENQUEUED,
        db_event_outbox_enqueue(&first, DESTINATION, TOPIC, NULL,
                                NULL, NULL));
    event_outbox_item_t item;
    TEST_ASSERT_EQUAL_INT(
        1, db_event_outbox_claim_due(DESTINATION, now, 5, &item));
    int64_t row_id = item.row_id;
    TEST_ASSERT_EQUAL_INT(1, item.attempt_count);
    db_event_outbox_item_clear(&item);
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_mark_retry(row_id, now + 10,
                                      "broker unavailable"));
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_claim_due(DESTINATION, now + 9, 5, &item));
    TEST_ASSERT_EQUAL_INT(
        1, db_event_outbox_claim_due(DESTINATION, now + 10, 5, &item));
    TEST_ASSERT_EQUAL_INT(2, item.attempt_count);
    db_event_outbox_item_clear(&item);

    /* Leave attempt two leased. It becomes claimable after lease expiry. */
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_claim_due(DESTINATION, now + 14, 5, &item));
    TEST_ASSERT_EQUAL_INT(
        1, db_event_outbox_claim_due(DESTINATION, now + 15, 5, &item));
    TEST_ASSERT_EQUAL_INT(3, item.attempt_count);
    TEST_ASSERT_EQUAL_INT(0,
                          db_event_outbox_mark_delivered(item.row_id,
                                                         now + 15));
    db_event_outbox_item_clear(&item);

    event_outbox_stats_t stats;
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_get_stats(DESTINATION, now + 15, &stats));
    TEST_ASSERT_EQUAL_INT64(1, stats.delivered_rows);
    TEST_ASSERT_EQUAL_INT64(0, stats.pending_rows);
    event_envelope_clear(&first);
}

void test_expired_event_moves_to_dead_letter_without_claim(void) {
    int64_t now = (int64_t)time(NULL);
    event_envelope_t expired = detection_event("expired", (time_t)(now - 4000));
    TEST_ASSERT_LESS_THAN_INT64(now, expired.expires_at);
    TEST_ASSERT_EQUAL_INT(
        EVENT_OUTBOX_ENQUEUED,
        db_event_outbox_enqueue(&expired, DESTINATION, TOPIC, NULL,
                                NULL, NULL));
    event_outbox_item_t item;
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_claim_due(DESTINATION, now, 30, &item));
    event_outbox_stats_t stats;
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_get_stats(DESTINATION, now, &stats));
    TEST_ASSERT_EQUAL_INT64(1, stats.dead_rows);
    TEST_ASSERT_EQUAL_INT64(0, stats.due_rows);
    event_envelope_clear(&expired);
}

void test_capacity_sheds_lower_priority_only_for_critical_arrival(void) {
    int64_t now = (int64_t)time(NULL);
    event_outbox_limits_t limits = {.max_rows = 2, .max_bytes = 1024 * 1024};
    event_envelope_t first = detection_event("first", (time_t)now);
    event_envelope_t second = detection_event("second", (time_t)now);
    event_envelope_t critical = storage_event((time_t)now);
    TEST_ASSERT_EQUAL_INT(
        EVENT_OUTBOX_ENQUEUED,
        db_event_outbox_enqueue(&first, DESTINATION, TOPIC, &limits,
                                NULL, NULL));
    TEST_ASSERT_EQUAL_INT(
        EVENT_OUTBOX_ENQUEUED,
        db_event_outbox_enqueue(&second, DESTINATION, TOPIC, &limits,
                                NULL, NULL));
    int shed = 0;
    TEST_ASSERT_EQUAL_INT(
        EVENT_OUTBOX_ENQUEUED,
        db_event_outbox_enqueue(&critical, DESTINATION,
                                "lightnvr/v1/events/storage", &limits,
                                NULL, &shed));
    TEST_ASSERT_EQUAL_INT(1, shed);

    event_outbox_item_t item;
    TEST_ASSERT_EQUAL_INT(
        1, db_event_outbox_claim_due(DESTINATION, now, 30, &item));
    TEST_ASSERT_EQUAL_STRING(critical.id, item.event_id);
    TEST_ASSERT_EQUAL_INT(EVENT_SEVERITY_CRITICAL, item.severity);
    db_event_outbox_item_clear(&item);

    event_envelope_t overflow = detection_event("overflow", (time_t)now);
    TEST_ASSERT_EQUAL_INT(
        EVENT_OUTBOX_FULL,
        db_event_outbox_enqueue(&overflow, DESTINATION, TOPIC, &limits,
                                NULL, NULL));
    event_outbox_stats_t stats;
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_get_stats(DESTINATION, now, &stats));
    TEST_ASSERT_EQUAL_INT64(2, stats.total_rows);
    TEST_ASSERT_EQUAL_INT64(1, stats.pending_rows);
    TEST_ASSERT_EQUAL_INT64(1, stats.delivering_rows);

    event_envelope_clear(&first);
    event_envelope_clear(&second);
    event_envelope_clear(&critical);
    event_envelope_clear(&overflow);
}

void test_terminal_rows_prune_in_bounded_batches(void) {
    int64_t now = (int64_t)time(NULL);
    event_envelope_t event = detection_event("person", (time_t)now);
    TEST_ASSERT_EQUAL_INT(
        EVENT_OUTBOX_ENQUEUED,
        db_event_outbox_enqueue(&event, DESTINATION, TOPIC, NULL,
                                NULL, NULL));
    event_outbox_item_t item;
    TEST_ASSERT_EQUAL_INT(
        1, db_event_outbox_claim_due(DESTINATION, now, 30, &item));
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_mark_dead(item.row_id, now, "permanent failure"));
    db_event_outbox_item_clear(&item);
    int deleted = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_prune_terminal(now + 1, 1, &deleted));
    TEST_ASSERT_EQUAL_INT(1, deleted);
    event_outbox_stats_t stats;
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_get_stats(NULL, now, &stats));
    TEST_ASSERT_EQUAL_INT64(0, stats.total_rows);
    event_envelope_clear(&event);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_enqueue_round_trip_and_duplicate_identity);
    RUN_TEST(test_dynamic_health_severity_and_persisted_identity_survive_outbox);
    RUN_TEST(test_health_transition_producer_normalizes_alert_and_recovery);
    RUN_TEST(test_health_transition_producer_replay_is_durable_and_idempotent);
    RUN_TEST(test_retry_schedule_and_stale_lease_recovery);
    RUN_TEST(test_expired_event_moves_to_dead_letter_without_claim);
    RUN_TEST(test_capacity_sheds_lower_priority_only_for_critical_arrival);
    RUN_TEST(test_terminal_rows_prune_in_bounded_batches);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
