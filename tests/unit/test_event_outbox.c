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

#include "core/event_envelope.h"
#include "database/db_core.h"
#include "database/db_event_outbox.h"
#include "unity.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_event_outbox.db"
#define INSTALLATION_SOURCE \
    "urn:lightnvr:11111111-1111-4111-8111-111111111111"
#define CAMERA_SUBJECT "camera/22222222-2222-4222-8222-222222222222"
#define DESTINATION "mqtt:default"
#define TOPIC "lightnvr/v1/events/io.lightnvr.detection.object.v1/camera"

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

void setUp(void) {
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK, sqlite3_exec(get_db_handle(), "DELETE FROM event_outbox;",
                                NULL, NULL, NULL));
}

void tearDown(void) {}

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
    RUN_TEST(test_retry_schedule_and_stale_lease_recovery);
    RUN_TEST(test_expired_event_moves_to_dead_letter_without_claim);
    RUN_TEST(test_capacity_sheds_lower_priority_only_for_critical_arrival);
    RUN_TEST(test_terminal_rows_prune_in_bounded_batches);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
