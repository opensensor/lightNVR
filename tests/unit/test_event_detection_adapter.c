/**
 * @file test_event_detection_adapter.c
 * @brief Persistent installation identity and detection adapter integration.
 */

#define _POSIX_C_SOURCE 200809L

#include <pthread.h>
#include <stdio.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

#include <sqlite3.h>

#include "core/config.h"
#include "core/event_bus.h"
#include "core/event_identity.h"
#include "core/event_producers.h"
#include "core/mqtt_delivery_worker.h"
#include "core/mqtt_event_adapter.h"
#include "database/db_core.h"
#include "database/db_event_routes.h"
#include "database/db_event_outbox.h"
#include "database/db_streams.h"
#include "database/db_system_settings.h"
#include "unity.h"
#include "utils/strings.h"
#include "utils/uuid.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_event_detection_adapter.db"

typedef struct {
    int calls;
    bool ran_on_producer_thread;
    pthread_t producer_thread;
    char source[EVENT_SOURCE_MAX];
    char subject[EVENT_SUBJECT_MAX];
    char stream_name[MAX_STREAM_NAME];
    detection_result_t detections;
} capture_t;

static capture_t capture;
static config_t mqtt_adapter_config;

static int capture_event(const event_envelope_t *event, void *context) {
    capture_t *result = context;
    result->calls++;
    result->ran_on_producer_thread =
        pthread_equal(pthread_self(), result->producer_thread) != 0;
    safe_strcpy(result->source, event->source, sizeof(result->source), 0);
    safe_strcpy(result->subject, event->subject, sizeof(result->subject), 0);
    return mqtt_event_adapter_decode_detection(
        event, result->stream_name, sizeof(result->stream_name),
        &result->detections);
}

static stream_config_t create_camera(const char *name) {
    stream_config_t stream;
    memset(&stream, 0, sizeof(stream));
    safe_strcpy(stream.name, name, sizeof(stream.name), 0);
    safe_strcpy(stream.url, "rtsp://camera/stream", sizeof(stream.url), 0);
    safe_strcpy(stream.codec, "h264", sizeof(stream.codec), 0);
    stream.enabled = true;
    stream.streaming_enabled = true;
    stream.width = 1920;
    stream.height = 1080;
    stream.fps = 25;
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&stream));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name(name, &stream));
    return stream;
}

void setUp(void) {
    event_bus_shutdown(false);
    event_bus_unsubscribe("capture-detection");
    mqtt_event_adapter_unregister();
    event_identity_shutdown();
    sqlite3 *db = get_db_handle();
    sqlite3_exec(db, "DELETE FROM streams;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM event_outbox;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM event_routes;", NULL, NULL, NULL);
    sqlite3_exec(db,
                 "DELETE FROM system_settings "
                 "WHERE key='event_installation_uuid';",
                 NULL, NULL, NULL);
    memset(&capture, 0, sizeof(capture));
    memset(&mqtt_adapter_config, 0, sizeof(mqtt_adapter_config));
    mqtt_adapter_config.mqtt_enabled = true;
    safe_strcpy(mqtt_adapter_config.mqtt_topic_prefix, "lightnvr",
                sizeof(mqtt_adapter_config.mqtt_topic_prefix), 0);
}

void tearDown(void) {
    event_bus_shutdown(false);
    event_bus_unsubscribe("capture-detection");
    mqtt_event_adapter_unregister();
    event_identity_shutdown();
}

void test_installation_identity_is_persisted_across_reinitialization(void) {
    TEST_ASSERT_EQUAL_INT(0, event_identity_init());
    char first[EVENT_SOURCE_MAX];
    char second[EVENT_SOURCE_MAX];
    TEST_ASSERT_EQUAL_INT(
        0, event_identity_get_source(first, sizeof(first)));
    TEST_ASSERT_EQUAL_INT(0, strncmp(first, "urn:lightnvr:", 13));
    TEST_ASSERT_TRUE(strlen(first) == 13 + 36);
    TEST_ASSERT_TRUE(lightnvr_uuid_is_valid(first + 13));

    event_identity_shutdown();
    TEST_ASSERT_EQUAL_INT(0, event_identity_init());
    TEST_ASSERT_EQUAL_INT(
        0, event_identity_get_source(second, sizeof(second)));
    TEST_ASSERT_EQUAL_STRING(first, second);

    char persisted[LIGHTNVR_UUID_STRING_SIZE];
    TEST_ASSERT_EQUAL_INT(
        0, db_get_system_setting("event_installation_uuid", persisted,
                                 sizeof(persisted)));
    TEST_ASSERT_EQUAL_STRING(first + 13, persisted);
}

void test_detection_producer_uses_camera_uuid_and_dispatches_off_thread(void) {
    stream_config_t camera = create_camera("North Door");
    TEST_ASSERT_EQUAL_INT(0, event_identity_init());
    capture.producer_thread = pthread_self();
    TEST_ASSERT_EQUAL_INT(
        0, mqtt_event_adapter_register(&mqtt_adapter_config));
    TEST_ASSERT_EQUAL_INT(
        0, event_bus_subscribe("capture-detection", capture_event, &capture));
    TEST_ASSERT_EQUAL_INT(0, event_bus_init(0, 0));

    detection_result_t detections;
    memset(&detections, 0, sizeof(detections));
    detections.count = 2;
    safe_strcpy(detections.detections[0].label, "person",
                sizeof(detections.detections[0].label), 0);
    detections.detections[0].confidence = 0.94f;
    detections.detections[0].x = 0.1f;
    detections.detections[0].y = 0.2f;
    detections.detections[0].width = 0.3f;
    detections.detections[0].height = 0.4f;
    detections.detections[0].track_id = 17;
    safe_strcpy(detections.detections[0].zone_id, "loading-bay",
                sizeof(detections.detections[0].zone_id), 0);
    safe_strcpy(detections.detections[1].label, "motion",
                sizeof(detections.detections[1].label), 0);
    detections.detections[1].confidence = 1.0f;
    detections.detections[1].track_id = -1;

    char error[256] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, event_producer_publish_detection_for_stream(
               camera.name, &detections, 1787466600,
               error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(0, event_bus_wait_until_idle(2000));

    char expected_subject[EVENT_SUBJECT_MAX];
    snprintf(expected_subject, sizeof(expected_subject), "camera/%s",
             camera.camera_uuid);
    TEST_ASSERT_EQUAL_INT(1, capture.calls);
    TEST_ASSERT_FALSE(capture.ran_on_producer_thread);
    TEST_ASSERT_EQUAL_STRING(expected_subject, capture.subject);
    TEST_ASSERT_TRUE(lightnvr_uuid_is_valid(capture.source + 13));
    TEST_ASSERT_EQUAL_STRING(camera.name, capture.stream_name);
    TEST_ASSERT_EQUAL_INT(2, capture.detections.count);
    TEST_ASSERT_EQUAL_STRING("person", capture.detections.detections[0].label);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 0.3f,
                             capture.detections.detections[0].width);
    TEST_ASSERT_EQUAL_INT(17, capture.detections.detections[0].track_id);
    TEST_ASSERT_EQUAL_STRING("loading-bay",
                             capture.detections.detections[0].zone_id);
    TEST_ASSERT_EQUAL_STRING("motion", capture.detections.detections[1].label);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 0.0f,
                             capture.detections.detections[1].width);
    TEST_ASSERT_EQUAL_INT(-1, capture.detections.detections[1].track_id);
#ifdef ENABLE_MQTT
    event_outbox_stats_t outbox;
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_get_stats(
               MQTT_EVENT_OUTBOX_DESTINATION, (int64_t)time(NULL), &outbox));
    TEST_ASSERT_EQUAL_INT64(1, outbox.pending_rows);
#endif
}

void test_producer_fails_closed_without_identity_or_running_bus(void) {
    detection_result_t detections;
    memset(&detections, 0, sizeof(detections));
    detections.count = 1;
    safe_strcpy(detections.detections[0].label, "person",
                sizeof(detections.detections[0].label), 0);
    detections.detections[0].confidence = 0.9f;
    detections.detections[0].width = 1.0f;
    detections.detections[0].height = 1.0f;
    char error[256] = {0};
    TEST_ASSERT_EQUAL_INT(
        -1, event_producer_publish_detection(
                "11111111-1111-4111-8111-111111111111", "camera",
                &detections, 0, error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "identity"));

    TEST_ASSERT_EQUAL_INT(0, event_identity_init());
    TEST_ASSERT_EQUAL_INT(
        -1, event_producer_publish_detection(
                "11111111-1111-4111-8111-111111111111", "camera",
                &detections, 0, error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "not running"));
}

void test_enabled_routes_gate_the_normalized_outbox(void) {
#ifdef ENABLE_MQTT
    stream_config_t camera = create_camera("Route Camera");
    event_route_t route;
    memset(&route, 0, sizeof(route));
    safe_strcpy(route.name, "Offline only", sizeof(route.name), 0);
    route.enabled = true;
    safe_strcpy(route.destination_key, EVENT_ROUTE_DEFAULT_DESTINATION,
                sizeof(route.destination_key), 0);
    safe_strcpy(route.scope_type, "all", sizeof(route.scope_type), 0);
    safe_strcpy(route.predicate_json, "{\"version\":1}",
                sizeof(route.predicate_json), 0);
    safe_strcpy(route.schedule_json,
                "{\"version\":1,\"timezone\":\"UTC\",\"windows\":[]}",
                sizeof(route.schedule_json), 0);
    safe_strcpy(route.event_types[0], "io.lightnvr.camera.offline.v1",
                sizeof(route.event_types[0]), 0);
    route.event_type_count = 1;
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK, db_event_route_create(&route));

    TEST_ASSERT_EQUAL_INT(0, event_identity_init());
    TEST_ASSERT_EQUAL_INT(0, mqtt_event_adapter_register(&mqtt_adapter_config));
    TEST_ASSERT_EQUAL_INT(0, event_bus_init(0, 0));
    detection_result_t detection;
    memset(&detection, 0, sizeof(detection));
    detection.count = 1;
    safe_strcpy(detection.detections[0].label, "person",
                sizeof(detection.detections[0].label), 0);
    detection.detections[0].confidence = 0.9f;
    detection.detections[0].width = 1.0f;
    detection.detections[0].height = 1.0f;
    detection.detections[0].track_id = -1;
    char error[256] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, event_producer_publish_detection_for_stream(
               camera.name, &detection, time(NULL), error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(0, event_bus_wait_until_idle(2000));
    event_outbox_stats_t outbox;
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_get_stats(
               MQTT_EVENT_OUTBOX_DESTINATION, (int64_t)time(NULL), &outbox));
    TEST_ASSERT_EQUAL_INT64(0, outbox.pending_rows);

    safe_strcpy(route.event_types[0],
                "io.lightnvr.detection.object.v1",
                sizeof(route.event_types[0]), 0);
    route.cooldown_seconds = 30;
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK,
                          db_event_route_update(&route, route.revision));
    TEST_ASSERT_EQUAL_INT(
        0, event_producer_publish_detection_for_stream(
               camera.name, &detection, time(NULL), error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(0, event_bus_wait_until_idle(2000));
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_get_stats(
               MQTT_EVENT_OUTBOX_DESTINATION, (int64_t)time(NULL), &outbox));
    TEST_ASSERT_EQUAL_INT64(1, outbox.pending_rows);

    /* A new event ID for the same subject is suppressed after outbox accept. */
    TEST_ASSERT_EQUAL_INT(
        0, event_producer_publish_detection_for_stream(
               camera.name, &detection, time(NULL), error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(0, event_bus_wait_until_idle(2000));
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_get_stats(
               MQTT_EVENT_OUTBOX_DESTINATION, (int64_t)time(NULL), &outbox));
    TEST_ASSERT_EQUAL_INT64(1, outbox.pending_rows);
#endif
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }

    UNITY_BEGIN();
    RUN_TEST(test_installation_identity_is_persisted_across_reinitialization);
    RUN_TEST(test_detection_producer_uses_camera_uuid_and_dispatches_off_thread);
    RUN_TEST(test_producer_fails_closed_without_identity_or_running_bus);
    RUN_TEST(test_enabled_routes_gate_the_normalized_outbox);
    int result = UNITY_END();

    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
