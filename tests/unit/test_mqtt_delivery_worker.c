/**
 * @file test_mqtt_delivery_worker.c
 * @brief Durable MQTT delivery retry, acknowledgement, and expiry tests.
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
#include "core/mqtt_delivery_worker.h"
#include "database/db_core.h"
#include "database/db_event_outbox.h"
#include "unity.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_mqtt_delivery_worker.db"
#define INSTALLATION_SOURCE \
    "urn:lightnvr:11111111-1111-4111-8111-111111111111"
#define CAMERA_SUBJECT "camera/22222222-2222-4222-8222-222222222222"
#define EXPECTED_TOPIC \
    "lightnvr/v1/events/io.lightnvr.detection.object.v1/" \
    "22222222-2222-4222-8222-222222222222"

typedef struct {
    int calls;
    int result;
    bool retain;
    int timeout_ms;
    char topic[EVENT_OUTBOX_TOPIC_MAX];
    char *payload;
} publisher_t;

static event_envelope_t detection_event(time_t occurred_at) {
    cJSON *data = cJSON_CreateObject();
    cJSON_AddStringToObject(data, "stream_name", "North Door");
    cJSON_AddNumberToObject(data, "count", 1);
    cJSON *detections = cJSON_AddArrayToObject(data, "detections");
    cJSON *detection = cJSON_CreateObject();
    cJSON_AddStringToObject(detection, "label", "person");
    cJSON_AddNumberToObject(detection, "confidence", 0.94);
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

static int fake_publish(const char *topic, const char *payload, bool retain,
                        int timeout_ms, void *context) {
    publisher_t *publisher = context;
    publisher->calls++;
    publisher->retain = retain;
    publisher->timeout_ms = timeout_ms;
    snprintf(publisher->topic, sizeof(publisher->topic), "%s", topic);
    free(publisher->payload);
    publisher->payload = strdup(payload);
    return publisher->payload ? publisher->result : -1;
}

static int64_t next_attempt_for(int64_t row_id, char state[16],
                                int *attempt_count) {
    sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_prepare_v2(
            get_db_handle(),
            "SELECT next_attempt_at,state,attempt_count FROM event_outbox "
            "WHERE id=?;", -1, &stmt, NULL));
    sqlite3_bind_int64(stmt, 1, row_id);
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(stmt));
    int64_t next_attempt_at = sqlite3_column_int64(stmt, 0);
    snprintf(state, 16, "%s",
             (const char *)sqlite3_column_text(stmt, 1));
    *attempt_count = sqlite3_column_int(stmt, 2);
    sqlite3_finalize(stmt);
    return next_attempt_at;
}

void setUp(void) {
    mqtt_delivery_worker_shutdown();
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK, sqlite3_exec(get_db_handle(), "DELETE FROM event_outbox;",
                                NULL, NULL, NULL));
}

void tearDown(void) {
    mqtt_delivery_worker_shutdown();
}

void test_enqueue_freezes_topic_and_acknowledged_publish_delivers(void) {
    int64_t now = (int64_t)time(NULL);
    event_envelope_t event = detection_event((time_t)now);
    char error[256] = {0};
    char *serialized = event_envelope_serialize(&event, error, sizeof(error));
    TEST_ASSERT_NOT_NULL(serialized);

    mqtt_delivery_worker_stats_t before;
    mqtt_delivery_worker_get_stats(&before);
    int64_t row_id = 0;
    TEST_ASSERT_EQUAL_INT(
        EVENT_OUTBOX_ENQUEUED,
        mqtt_delivery_worker_enqueue(&event, "lightnvr", &row_id));
    TEST_ASSERT_GREATER_THAN_INT64(0, row_id);

    publisher_t publisher = {.result = 0};
    TEST_ASSERT_EQUAL_INT(
        1, mqtt_delivery_worker_process_once(
               now, true, fake_publish, &publisher));
    TEST_ASSERT_EQUAL_INT(1, publisher.calls);
    TEST_ASSERT_FALSE(publisher.retain);
    TEST_ASSERT_EQUAL_INT(MQTT_DELIVERY_ACK_TIMEOUT_MS,
                          publisher.timeout_ms);
    TEST_ASSERT_EQUAL_STRING(EXPECTED_TOPIC, publisher.topic);
    TEST_ASSERT_NOT_NULL(publisher.payload);
    TEST_ASSERT_EQUAL_STRING(serialized, publisher.payload);

    event_outbox_stats_t outbox;
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_get_stats(
               MQTT_EVENT_OUTBOX_DESTINATION, now, &outbox));
    TEST_ASSERT_EQUAL_INT64(1, outbox.delivered_rows);
    TEST_ASSERT_EQUAL_INT64(0, outbox.pending_rows);

    int64_t duplicate_row = 0;
    TEST_ASSERT_EQUAL_INT(
        EVENT_OUTBOX_DUPLICATE,
        mqtt_delivery_worker_enqueue(&event, "changed-prefix",
                                     &duplicate_row));
    TEST_ASSERT_EQUAL_INT64(row_id, duplicate_row);
    mqtt_delivery_worker_stats_t after;
    mqtt_delivery_worker_get_stats(&after);
    TEST_ASSERT_EQUAL_UINT64(before.enqueued + 1, after.enqueued);
    TEST_ASSERT_EQUAL_UINT64(before.duplicates + 1, after.duplicates);
    TEST_ASSERT_EQUAL_UINT64(before.delivered + 1, after.delivered);

    free(publisher.payload);
    free(serialized);
    event_envelope_clear(&event);
}

void test_disconnected_broker_does_not_claim_pending_work(void) {
    int64_t now = (int64_t)time(NULL);
    event_envelope_t event = detection_event((time_t)now);
    TEST_ASSERT_EQUAL_INT(
        EVENT_OUTBOX_ENQUEUED,
        mqtt_delivery_worker_enqueue(&event, "lightnvr", NULL));
    publisher_t publisher = {.result = 0};
    TEST_ASSERT_EQUAL_INT(
        0, mqtt_delivery_worker_process_once(
               now, false, fake_publish, &publisher));
    TEST_ASSERT_EQUAL_INT(0, publisher.calls);
    event_outbox_stats_t outbox;
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_get_stats(
               MQTT_EVENT_OUTBOX_DESTINATION, now, &outbox));
    TEST_ASSERT_EQUAL_INT64(1, outbox.pending_rows);
    TEST_ASSERT_EQUAL_INT64(1, outbox.due_rows);
    event_envelope_clear(&event);
}

void test_unacknowledged_publish_retries_at_persisted_backoff(void) {
    int64_t now = (int64_t)time(NULL);
    event_envelope_t event = detection_event((time_t)now);
    int64_t row_id = 0;
    TEST_ASSERT_EQUAL_INT(
        EVENT_OUTBOX_ENQUEUED,
        mqtt_delivery_worker_enqueue(&event, "lightnvr", &row_id));
    publisher_t publisher = {.result = -1};
    TEST_ASSERT_EQUAL_INT(
        1, mqtt_delivery_worker_process_once(
               now, true, fake_publish, &publisher));

    char state[16];
    int attempt_count = 0;
    int64_t retry_at = next_attempt_for(row_id, state, &attempt_count);
    TEST_ASSERT_EQUAL_STRING("pending", state);
    TEST_ASSERT_EQUAL_INT(1, attempt_count);
    TEST_ASSERT_GREATER_THAN_INT64(now, retry_at);
    TEST_ASSERT_LESS_OR_EQUAL_INT64(
        now + MQTT_DELIVERY_MAX_BACKOFF_SECONDS, retry_at);
    TEST_ASSERT_EQUAL_INT(
        0, mqtt_delivery_worker_process_once(
               retry_at - 1, true, fake_publish, &publisher));

    publisher.result = 0;
    TEST_ASSERT_EQUAL_INT(
        1, mqtt_delivery_worker_process_once(
               retry_at, true, fake_publish, &publisher));
    TEST_ASSERT_EQUAL_INT(2, publisher.calls);
    next_attempt_for(row_id, state, &attempt_count);
    TEST_ASSERT_EQUAL_STRING("delivered", state);
    TEST_ASSERT_EQUAL_INT(2, attempt_count);
    free(publisher.payload);
    event_envelope_clear(&event);
}

void test_retry_that_would_cross_expiry_becomes_dead(void) {
    int64_t now = (int64_t)time(NULL);
    event_envelope_t event = detection_event((time_t)(now - 3599));
    TEST_ASSERT_EQUAL_INT64(now + 1, event.expires_at);
    int64_t row_id = 0;
    TEST_ASSERT_EQUAL_INT(
        EVENT_OUTBOX_ENQUEUED,
        mqtt_delivery_worker_enqueue(&event, "lightnvr", &row_id));
    publisher_t publisher = {.result = -1};
    TEST_ASSERT_EQUAL_INT(
        1, mqtt_delivery_worker_process_once(
               now, true, fake_publish, &publisher));
    char state[16];
    int attempt_count = 0;
    next_attempt_for(row_id, state, &attempt_count);
    TEST_ASSERT_EQUAL_STRING("dead", state);
    TEST_ASSERT_EQUAL_INT(1, attempt_count);
    free(publisher.payload);
    event_envelope_clear(&event);
}

void test_expiry_is_swept_even_while_broker_is_disconnected(void) {
    int64_t now = (int64_t)time(NULL);
    event_envelope_t event = detection_event((time_t)(now - 4000));
    TEST_ASSERT_LESS_THAN_INT64(now, event.expires_at);
    TEST_ASSERT_EQUAL_INT(
        EVENT_OUTBOX_ENQUEUED,
        mqtt_delivery_worker_enqueue(&event, "lightnvr", NULL));
    publisher_t publisher = {.result = 0};
    TEST_ASSERT_EQUAL_INT(
        0, mqtt_delivery_worker_process_once(
               now, false, fake_publish, &publisher));
    event_outbox_stats_t outbox;
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_get_stats(
               MQTT_EVENT_OUTBOX_DESTINATION, now, &outbox));
    TEST_ASSERT_EQUAL_INT64(1, outbox.dead_rows);
    TEST_ASSERT_EQUAL_INT(0, publisher.calls);
    event_envelope_clear(&event);
}

void test_backoff_is_bounded_and_exponential(void) {
    int previous = 0;
    for (int attempt = 1; attempt <= 8; attempt++) {
        int delay = mqtt_delivery_backoff_seconds(
            attempt, (uint32_t)(attempt * 7919));
        TEST_ASSERT_GREATER_OR_EQUAL_INT(1, delay);
        TEST_ASSERT_LESS_OR_EQUAL_INT(
            MQTT_DELIVERY_MAX_BACKOFF_SECONDS, delay);
        TEST_ASSERT_GREATER_THAN_INT(previous, delay);
        previous = delay;
    }
    for (int attempt = 16; attempt <= 32; attempt++) {
        int delay = mqtt_delivery_backoff_seconds(
            attempt, (uint32_t)(attempt * 104729));
        TEST_ASSERT_GREATER_OR_EQUAL_INT(225, delay);
        TEST_ASSERT_LESS_OR_EQUAL_INT(300, delay);
    }
}

void test_worker_lifecycle_is_idempotent(void) {
#ifdef ENABLE_MQTT
    TEST_ASSERT_EQUAL_INT(0, mqtt_delivery_worker_start());
    TEST_ASSERT_EQUAL_INT(0, mqtt_delivery_worker_start());
    mqtt_delivery_worker_stats_t stats;
    mqtt_delivery_worker_get_stats(&stats);
    TEST_ASSERT_TRUE(stats.running);
    mqtt_delivery_worker_shutdown();
    mqtt_delivery_worker_shutdown();
    mqtt_delivery_worker_get_stats(&stats);
    TEST_ASSERT_FALSE(stats.running);
#else
    TEST_ASSERT_EQUAL_INT(-1, mqtt_delivery_worker_start());
#endif
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_enqueue_freezes_topic_and_acknowledged_publish_delivers);
    RUN_TEST(test_disconnected_broker_does_not_claim_pending_work);
    RUN_TEST(test_unacknowledged_publish_retries_at_persisted_backoff);
    RUN_TEST(test_retry_that_would_cross_expiry_becomes_dead);
    RUN_TEST(test_expiry_is_swept_even_while_broker_is_disconnected);
    RUN_TEST(test_backoff_is_bounded_and_exponential);
    RUN_TEST(test_worker_lifecycle_is_idempotent);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
