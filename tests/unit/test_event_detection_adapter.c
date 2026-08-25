/**
 * @file test_event_detection_adapter.c
 * @brief Persistent installation identity and detection adapter integration.
 */

#define _POSIX_C_SOURCE 200809L

#include <pthread.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

#include <sqlite3.h>

#include "core/config.h"
#include "core/event_bus.h"
#include "core/event_identity.h"
#include "core/event_producers.h"
#include "core/event_router.h"
#include "core/mqtt_delivery_worker.h"
#include "core/mqtt_event_adapter.h"
#include "database/db_core.h"
#include "database/db_event_destinations.h"
#include "database/db_event_routes.h"
#include "database/db_event_outbox.h"
#include "database/db_storage_targets.h"
#include "database/db_streams.h"
#include "database/db_system_settings.h"
#include "storage/storage_target_health.h"
#include "telemetry/stream_metrics.h"
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

typedef struct {
    int calls;
    char types[12][EVENT_TYPE_MAX];
    char subjects[12][EVENT_SUBJECT_MAX];
} operational_capture_t;

static operational_capture_t operational_capture;
static char target_test_root[MAX_PATH_LENGTH];

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

static int capture_operational_event(const event_envelope_t *event,
                                     void *context) {
    operational_capture_t *result = context;
    if (result->calls >= 12) return -1;
    safe_strcpy(result->types[result->calls], event->type,
                sizeof(result->types[result->calls]), 0);
    safe_strcpy(result->subjects[result->calls], event->subject,
                sizeof(result->subjects[result->calls]), 0);
    result->calls++;
    return 0;
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

#ifdef ENABLE_MQTT
static event_destination_t create_destination(
    const char *name, const char *host, const char *client_id,
    const char *topic_template) {
    event_destination_t destination;
    memset(&destination, 0, sizeof(destination));
    safe_strcpy(destination.name, name, sizeof(destination.name), 0);
    destination.enabled = true;
    safe_strcpy(destination.destination_type, "mqtt",
                sizeof(destination.destination_type), 0);
    safe_strcpy(destination.broker_host, host,
                sizeof(destination.broker_host), 0);
    destination.broker_port = 1883;
    safe_strcpy(destination.client_id, client_id,
                sizeof(destination.client_id), 0);
    safe_strcpy(destination.topic_template, topic_template,
                sizeof(destination.topic_template), 0);
    safe_strcpy(destination.tls_mode, "disabled",
                sizeof(destination.tls_mode), 0);
    destination.keepalive_seconds = 60;
    destination.qos = 1;
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_DESTINATION_OK,
        db_event_destination_create(&destination, NULL));
    return destination;
}

static event_route_t create_detection_route(
    const char *name, const char *destination_key, int cooldown_seconds) {
    event_route_t route;
    memset(&route, 0, sizeof(route));
    safe_strcpy(route.name, name, sizeof(route.name), 0);
    route.enabled = true;
    safe_strcpy(route.destination_key, destination_key,
                sizeof(route.destination_key), 0);
    safe_strcpy(route.scope_type, "all", sizeof(route.scope_type), 0);
    safe_strcpy(route.predicate_json, "{\"version\":1}",
                sizeof(route.predicate_json), 0);
    safe_strcpy(route.schedule_json,
                "{\"version\":1,\"timezone\":\"UTC\",\"windows\":[]}",
                sizeof(route.schedule_json), 0);
    safe_strcpy(route.event_types[0],
                "io.lightnvr.detection.object.v1",
                sizeof(route.event_types[0]), 0);
    route.event_type_count = 1;
    route.cooldown_seconds = cooldown_seconds;
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK, db_event_route_create(&route));
    return route;
}

static int outbox_topic_count(const char *destination, const char *topic) {
    sqlite3_stmt *statement = NULL;
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_prepare_v2(
            get_db_handle(),
            "SELECT count(*) FROM event_outbox WHERE destination=? AND topic=?;",
            -1, &statement, NULL));
    sqlite3_bind_text(statement, 1, destination, -1, SQLITE_TRANSIENT);
    sqlite3_bind_text(statement, 2, topic, -1, SQLITE_TRANSIENT);
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(statement));
    int count = sqlite3_column_int(statement, 0);
    sqlite3_finalize(statement);
    return count;
}
#endif

void setUp(void) {
    metrics_shutdown();
    event_bus_shutdown(false);
    event_bus_unsubscribe("capture-detection");
    event_bus_unsubscribe("capture-operational");
    mqtt_event_adapter_unregister();
    event_identity_shutdown();
    event_router_shutdown();
    sqlite3 *db = get_db_handle();
    sqlite3_exec(db, "DELETE FROM storage_policies;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM recordings;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM storage_targets;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM streams;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM event_outbox;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM event_routes;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM event_destinations;", NULL, NULL, NULL);
    sqlite3_exec(db,
                 "DELETE FROM system_settings "
                 "WHERE key='event_installation_uuid';",
                 NULL, NULL, NULL);
    memset(&capture, 0, sizeof(capture));
    memset(&operational_capture, 0, sizeof(operational_capture));
    target_test_root[0] = '\0';
    memset(&mqtt_adapter_config, 0, sizeof(mqtt_adapter_config));
    mqtt_adapter_config.mqtt_enabled = true;
    safe_strcpy(mqtt_adapter_config.mqtt_topic_prefix, "lightnvr",
                sizeof(mqtt_adapter_config.mqtt_topic_prefix), 0);
}

void tearDown(void) {
    metrics_shutdown();
    event_bus_shutdown(false);
    event_bus_unsubscribe("capture-detection");
    event_bus_unsubscribe("capture-operational");
    mqtt_event_adapter_unregister();
    event_identity_shutdown();
    event_router_shutdown();
    if (target_test_root[0]) {
        rmdir(target_test_root);
        target_test_root[0] = '\0';
    }
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

void test_operational_producers_use_registered_schemas_and_stable_subjects(void) {
    stream_config_t camera = create_camera("Operational Camera");
    TEST_ASSERT_EQUAL_INT(0, event_identity_init());
    TEST_ASSERT_EQUAL_INT(
        0, event_bus_subscribe("capture-operational",
                               capture_operational_event,
                               &operational_capture));
    TEST_ASSERT_EQUAL_INT(0, event_bus_init(0, 0));

    const time_t occurred_at = 1787466600;
    char error[256] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, event_producer_publish_camera_offline_for_stream(
               camera.name, "frame_timeout", 3, occurred_at,
               error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(
        0, event_producer_publish_camera_recovered_for_stream(
               camera.name, 15000, occurred_at, error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(
        0, event_producer_publish_stream_degraded_for_stream(
               camera.name, "low_fps", 7.5, 25.0, occurred_at,
               error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(
        0, event_producer_publish_stream_recovered_for_stream(
               camera.name, 24.5, 25.0, occurred_at,
               error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(
        0, event_producer_publish_recording_gap_for_stream(
               camera.name, occurred_at - 12, 12000, occurred_at,
               error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(
        0, event_producer_publish_storage_pressure(
               "critical", "warning", 94.5, 1073741824,
               occurred_at, error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(
        0, event_producer_publish_storage_recovered(
               "critical", 72.0, 3221225472ULL, occurred_at,
               error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(
        0, event_producer_publish_storage_target_unavailable(
               "33333333-3333-4333-8333-333333333333", "healthy",
               "mount_unavailable", false, occurred_at,
               error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(
        0, event_producer_publish_storage_target_recovered(
               "33333333-3333-4333-8333-333333333333", "healthy",
               60000, false, occurred_at, error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(0, event_bus_wait_until_idle(2000));

    TEST_ASSERT_EQUAL_INT(9, operational_capture.calls);
    const char *expected_types[] = {
        "io.lightnvr.camera.offline.v1",
        "io.lightnvr.camera.recovered.v1",
        "io.lightnvr.stream.degraded.v1",
        "io.lightnvr.stream.recovered.v1",
        "io.lightnvr.stream.recording_gap.v1",
        "io.lightnvr.storage.pressure.v1",
        "io.lightnvr.storage.recovered.v1",
        "io.lightnvr.storage.target_unavailable.v1",
        "io.lightnvr.storage.target_recovered.v1",
    };
    char camera_subject[EVENT_SUBJECT_MAX];
    snprintf(camera_subject, sizeof(camera_subject), "camera/%s",
             camera.camera_uuid);
    for (int index = 0; index < 9; index++) {
        TEST_ASSERT_EQUAL_STRING(expected_types[index],
                                 operational_capture.types[index]);
        TEST_ASSERT_EQUAL_STRING(index < 5 ? camera_subject : "system/storage",
                                 operational_capture.subjects[index]);
    }

    TEST_ASSERT_EQUAL_INT(
        -1, event_producer_publish_stream_degraded_for_stream(
                camera.name, "unstable", 1.0, 25.0, occurred_at,
                error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "stream degraded data is invalid"));

    TEST_ASSERT_EQUAL_INT(
        -1, event_producer_publish_storage_target_unavailable(
                "33333333-3333-4333-8333-333333333333", "healthy",
                "raw_probe_error", false, occurred_at,
                error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "event input is invalid"));
}

void test_storage_target_health_emits_only_availability_transitions(void) {
    char root_template[] = "/tmp/lightnvr-event-target-XXXXXX";
    char *created_root = mkdtemp(root_template);
    TEST_ASSERT_NOT_NULL(created_root);
    safe_strcpy(target_test_root, created_root, sizeof(target_test_root), 0);

    storage_target_t target;
    memset(&target, 0, sizeof(target));
    safe_strcpy(target.name, "Event target", sizeof(target.name), 0);
    safe_strcpy(target.target_type, "filesystem",
                sizeof(target.target_type), 0);
    safe_strcpy(target.root_path, target_test_root,
                sizeof(target.root_path), 0);
    safe_strcpy(target.storage_class, "hot",
                sizeof(target.storage_class), 0);
    target.enabled = true;
    target.high_watermark_pct = 99.0;
    target.low_watermark_pct = 98.0;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK,
                          db_storage_target_create(&target));

    TEST_ASSERT_EQUAL_INT(0, event_identity_init());
    TEST_ASSERT_EQUAL_INT(
        0, event_bus_subscribe("capture-operational",
                               capture_operational_event,
                               &operational_capture));
    TEST_ASSERT_EQUAL_INT(0, event_bus_init(0, 0));

    TEST_ASSERT_EQUAL_INT(
        DB_STORAGE_TARGET_OK,
        storage_target_probe_and_publish(target.uuid, false, NULL));
    TEST_ASSERT_EQUAL_INT(0, rmdir(target_test_root));
    TEST_ASSERT_EQUAL_INT(
        DB_STORAGE_TARGET_UNAVAILABLE,
        storage_target_probe_and_publish(target.uuid, false, NULL));
    TEST_ASSERT_EQUAL_INT(
        DB_STORAGE_TARGET_UNAVAILABLE,
        storage_target_probe_and_publish(target.uuid, false, NULL));

    TEST_ASSERT_EQUAL_INT(0, mkdir(target_test_root, 0700));
    TEST_ASSERT_EQUAL_INT(
        DB_STORAGE_TARGET_OK,
        storage_target_probe_and_publish(target.uuid, false, NULL));
    TEST_ASSERT_EQUAL_INT(
        DB_STORAGE_TARGET_OK,
        storage_target_probe_and_publish(target.uuid, false, NULL));
    TEST_ASSERT_EQUAL_INT(0, event_bus_wait_until_idle(2000));

    TEST_ASSERT_EQUAL_INT(2, operational_capture.calls);
    TEST_ASSERT_EQUAL_STRING(
        "io.lightnvr.storage.target_unavailable.v1",
        operational_capture.types[0]);
    TEST_ASSERT_EQUAL_STRING(
        "io.lightnvr.storage.target_recovered.v1",
        operational_capture.types[1]);
    TEST_ASSERT_EQUAL_STRING("system/storage",
                             operational_capture.subjects[0]);
    TEST_ASSERT_EQUAL_STRING("system/storage",
                             operational_capture.subjects[1]);
}

void test_operational_event_route_persists_to_outbox(void) {
#ifdef ENABLE_MQTT
    stream_config_t camera = create_camera("Offline Route Camera");
    event_route_t route;
    memset(&route, 0, sizeof(route));
    safe_strcpy(route.name, "Offline route", sizeof(route.name), 0);
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
    TEST_ASSERT_EQUAL_INT(0,
                          mqtt_event_adapter_register(&mqtt_adapter_config));
    TEST_ASSERT_EQUAL_INT(0, event_bus_init(0, 0));
    char error[256] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, event_producer_publish_camera_offline_for_stream(
               camera.name, "frame_timeout", 3, time(NULL),
               error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(0, event_bus_wait_until_idle(2000));

    event_outbox_stats_t stats;
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_get_stats(MQTT_EVENT_OUTBOX_DESTINATION,
                                     (int64_t)time(NULL), &stats));
    TEST_ASSERT_EQUAL_INT64(1, stats.pending_rows);
#endif
}

void test_recording_gap_hook_ignores_deliberate_session_boundaries(void) {
    stream_config_t camera = create_camera("Gap Camera");
    TEST_ASSERT_EQUAL_INT(0, event_identity_init());
    TEST_ASSERT_EQUAL_INT(
        0, event_bus_subscribe("capture-operational",
                               capture_operational_event,
                               &operational_capture));
    TEST_ASSERT_EQUAL_INT(0, event_bus_init(0, 0));
    TEST_ASSERT_EQUAL_INT(0, metrics_init(8));

    metrics_set_recording_active(camera.name, true);
    metrics_record_segment_complete(camera.name, 100, 110, 1024);
    metrics_record_segment_complete(camera.name, 120, 130, 2048);
    TEST_ASSERT_EQUAL_INT(0, event_bus_wait_until_idle(2000));
    TEST_ASSERT_EQUAL_INT(1, operational_capture.calls);
    TEST_ASSERT_EQUAL_STRING("io.lightnvr.stream.recording_gap.v1",
                             operational_capture.types[0]);

    metrics_set_recording_active(camera.name, false);
    metrics_set_recording_active(camera.name, true);
    metrics_record_segment_complete(camera.name, 1000, 1010, 4096);
    TEST_ASSERT_EQUAL_INT(0, event_bus_wait_until_idle(2000));
    TEST_ASSERT_EQUAL_INT(1, operational_capture.calls);

    metrics_shutdown();
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

void test_managed_routes_fan_out_with_per_destination_suppression(void) {
#ifdef ENABLE_MQTT
    stream_config_t camera = create_camera("Fanout Camera");
    event_destination_t first = create_destination(
        "First bridge", "first.example.test", "lightnvr-first-test",
        "first/{subject_id}/{type}");
    event_destination_t second = create_destination(
        "Second bridge", "second.example.test", "lightnvr-second-test",
        "second/{type}/{subject_id}");
    char first_key[EVENT_DESTINATION_KEY_MAX];
    char second_key[EVENT_DESTINATION_KEY_MAX];
    TEST_ASSERT_EQUAL_INT(
        0, db_event_destination_make_key(first.uuid, first_key));
    TEST_ASSERT_EQUAL_INT(
        0, db_event_destination_make_key(second.uuid, second_key));
    create_detection_route("First cooldown", first_key, 30);
    create_detection_route("Second unsuppressed", second_key, 0);

    mqtt_adapter_config.mqtt_enabled = false;
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
    time_t now = time(NULL);
    TEST_ASSERT_EQUAL_INT(
        0, event_producer_publish_detection_for_stream(
               camera.name, &detection, now, error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(0, event_bus_wait_until_idle(2000));

    event_outbox_stats_t stats;
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_get_stats(first_key, now, &stats));
    TEST_ASSERT_EQUAL_INT64(1, stats.pending_rows);
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_get_stats(second_key, now, &stats));
    TEST_ASSERT_EQUAL_INT64(1, stats.pending_rows);
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_get_stats(
               MQTT_EVENT_OUTBOX_DESTINATION, now, &stats));
    TEST_ASSERT_EQUAL_INT64(0, stats.pending_rows);

    char first_topic[EVENT_OUTBOX_TOPIC_MAX];
    char second_topic[EVENT_OUTBOX_TOPIC_MAX];
    snprintf(first_topic, sizeof(first_topic),
             "first/%s/io.lightnvr.detection.object.v1",
             camera.camera_uuid);
    snprintf(second_topic, sizeof(second_topic),
             "second/io.lightnvr.detection.object.v1/%s",
             camera.camera_uuid);
    TEST_ASSERT_EQUAL_INT(1, outbox_topic_count(first_key, first_topic));
    TEST_ASSERT_EQUAL_INT(1, outbox_topic_count(second_key, second_topic));

    TEST_ASSERT_EQUAL_INT(
        0, event_producer_publish_detection_for_stream(
               camera.name, &detection, now + 1, error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(0, event_bus_wait_until_idle(2000));
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_get_stats(first_key, now + 1, &stats));
    TEST_ASSERT_EQUAL_INT64(1, stats.pending_rows);
    TEST_ASSERT_EQUAL_INT(
        0, db_event_outbox_get_stats(second_key, now + 1, &stats));
    TEST_ASSERT_EQUAL_INT64(2, stats.pending_rows);
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
    RUN_TEST(test_operational_producers_use_registered_schemas_and_stable_subjects);
    RUN_TEST(test_storage_target_health_emits_only_availability_transitions);
    RUN_TEST(test_operational_event_route_persists_to_outbox);
    RUN_TEST(test_recording_gap_hook_ignores_deliberate_session_boundaries);
    RUN_TEST(test_enabled_routes_gate_the_normalized_outbox);
    RUN_TEST(test_managed_routes_fan_out_with_per_destination_suppression);
    int result = UNITY_END();

    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
