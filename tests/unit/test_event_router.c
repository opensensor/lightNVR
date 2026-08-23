/**
 * @file test_event_router.c
 * @brief Runtime event type, Fleet scope, predicate, and schedule routing tests.
 */

#define _POSIX_C_SOURCE 200809L

#include <cjson/cJSON.h>
#include <sqlite3.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "core/event_envelope.h"
#include "core/event_router.h"
#include "database/db_core.h"
#include "database/db_event_routes.h"
#include "database/db_streams.h"
#include "unity.h"
#include "utils/strings.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_event_router.db"
#define DETECTION_TYPE "io.lightnvr.detection.object.v1"
#define OFFLINE_TYPE "io.lightnvr.camera.offline.v1"
#define EVENT_SOURCE \
    "urn:lightnvr:11111111-1111-4111-8111-111111111111"

static stream_config_t create_camera(const char *name) {
    stream_config_t stream;
    memset(&stream, 0, sizeof(stream));
    safe_strcpy(stream.name, name, sizeof(stream.name), 0);
    safe_strcpy(stream.url, "rtsp://camera/live", sizeof(stream.url), 0);
    safe_strcpy(stream.codec, "h264", sizeof(stream.codec), 0);
    stream.enabled = true;
    stream.streaming_enabled = true;
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&stream));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name(name, &stream));
    return stream;
}

static event_route_t route_definition(const char *name, const char *type) {
    event_route_t route;
    memset(&route, 0, sizeof(route));
    safe_strcpy(route.name, name, sizeof(route.name), 0);
    route.enabled = true;
    safe_strcpy(route.destination_key, EVENT_ROUTE_DEFAULT_DESTINATION,
                sizeof(route.destination_key), 0);
    safe_strcpy(route.scope_type, "all", sizeof(route.scope_type), 0);
    safe_strcpy(route.predicate_json, "{\"version\":1}",
                sizeof(route.predicate_json), 0);
    safe_strcpy(route.schedule_json,
                "{\"version\":1,\"timezone\":\"UTC\",\"windows\":[]}",
                sizeof(route.schedule_json), 0);
    safe_strcpy(route.event_types[0], type,
                sizeof(route.event_types[0]), 0);
    route.event_type_count = 1;
    return route;
}

static event_envelope_t detection_event(const char *camera_uuid,
                                        const char *label,
                                        double confidence,
                                        const char *zone,
                                        time_t occurred_at) {
    cJSON *data = cJSON_CreateObject();
    cJSON *detections = cJSON_AddArrayToObject(data, "detections");
    cJSON *detection = cJSON_CreateObject();
    cJSON_AddStringToObject(data, "stream_name", "test-camera");
    cJSON_AddNumberToObject(data, "count", 1);
    cJSON_AddStringToObject(detection, "label", label);
    cJSON_AddNumberToObject(detection, "confidence", confidence);
    if (zone) cJSON_AddStringToObject(detection, "zone_id", zone);
    cJSON_AddItemToArray(detections, detection);
    char subject[EVENT_SUBJECT_MAX];
    snprintf(subject, sizeof(subject), "camera/%s", camera_uuid);
    event_envelope_t event;
    char error[256] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, event_envelope_create(&event, DETECTION_TYPE, EVENT_SOURCE, subject,
                                 occurred_at, data, error, sizeof(error)));
    cJSON_Delete(data);
    return event;
}

void setUp(void) {
    event_router_shutdown();
    sqlite3 *db = get_db_handle();
    sqlite3_exec(db, "DELETE FROM event_routes;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM streams;", NULL, NULL, NULL);
}

void tearDown(void) {
    event_router_shutdown();
}

void test_empty_routes_default_but_disabled_route_is_quiet(void) {
    event_envelope_t event = detection_event(
        "22222222-2222-4222-8222-222222222222", "person", 0.9, NULL,
        1786991400);
    TEST_ASSERT_EQUAL_INT(EVENT_ROUTER_DEFAULT,
                          event_router_evaluate(&event));

    event_route_t disabled = route_definition("Disabled", DETECTION_TYPE);
    disabled.enabled = false;
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK,
                          db_event_route_create(&disabled));
    TEST_ASSERT_EQUAL_INT(EVENT_ROUTER_NO_MATCH,
                          event_router_evaluate(&event));
    event_envelope_clear(&event);
}

void test_selector_predicate_and_utc_schedule_must_all_match(void) {
    stream_config_t north = create_camera("North Door");
    stream_config_t south = create_camera("South Door");
    event_route_t route = route_definition("North people", DETECTION_TYPE);
    safe_strcpy(route.scope_type, "selector", sizeof(route.scope_type), 0);
    snprintf(route.selector_json, sizeof(route.selector_json),
             "{\"version\":1,\"expression\":{\"op\":\"camera_uuid\","
             "\"values\":[\"%s\"]}}", north.camera_uuid);
    safe_strcpy(route.predicate_json,
                "{\"version\":1,\"detection\":{"
                "\"labels_any\":[\"person\"],\"min_confidence\":0.8,"
                "\"zone_ids_any\":[\"entry\"]}}",
                sizeof(route.predicate_json), 0);
    safe_strcpy(route.schedule_json,
                "{\"version\":1,\"timezone\":\"UTC\",\"windows\":[{"
                "\"days\":[1],\"start\":\"18:00\",\"end\":\"19:00\"}]}",
                sizeof(route.schedule_json), 0);
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK, db_event_route_create(&route));

    event_envelope_t event = detection_event(
        north.camera_uuid, "person", 0.91, "entry", 1786991400);
    TEST_ASSERT_EQUAL_INT(EVENT_ROUTER_MATCH,
                          event_router_evaluate(&event));
    event_envelope_clear(&event);

    event = detection_event(south.camera_uuid, "person", 0.91, "entry",
                            1786991400);
    TEST_ASSERT_EQUAL_INT(EVENT_ROUTER_NO_MATCH,
                          event_router_evaluate(&event));
    event_envelope_clear(&event);

    event = detection_event(north.camera_uuid, "person", 0.79, "entry",
                            1786991400);
    TEST_ASSERT_EQUAL_INT(EVENT_ROUTER_NO_MATCH,
                          event_router_evaluate(&event));
    event_envelope_clear(&event);

    event = detection_event(north.camera_uuid, "person", 0.91, "entry",
                            1786951800);
    TEST_ASSERT_EQUAL_INT(EVENT_ROUTER_NO_MATCH,
                          event_router_evaluate(&event));
    event_envelope_clear(&event);
}

void test_iana_timezone_and_overnight_windows_use_occurrence_time(void) {
    event_route_t route = route_definition("Early Sunday", DETECTION_TYPE);
    safe_strcpy(route.schedule_json,
                "{\"version\":1,\"timezone\":\"America/New_York\","
                "\"windows\":[{\"days\":[6],\"start\":\"22:00\","
                "\"end\":\"05:00\"}]}",
                sizeof(route.schedule_json), 0);
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK, db_event_route_create(&route));

    /* 2026-08-16 08:30 UTC is Sunday 04:30 EDT. The window began Saturday. */
    event_envelope_t event = detection_event(
        "22222222-2222-4222-8222-222222222222", "person", 0.9, NULL,
        1786869000);
    TEST_ASSERT_EQUAL_INT(EVENT_ROUTER_MATCH,
                          event_router_evaluate(&event));
    event_envelope_clear(&event);

    /* The same local window uses EST (UTC-5) during winter. */
    event = detection_event(
        "22222222-2222-4222-8222-222222222222", "person", 0.9, NULL,
        1768728600);
    TEST_ASSERT_EQUAL_INT(EVENT_ROUTER_MATCH,
                          event_router_evaluate(&event));
    event_envelope_clear(&event);
    event = detection_event(
        "22222222-2222-4222-8222-222222222222", "person", 0.9, NULL,
        1768732200);
    TEST_ASSERT_EQUAL_INT(EVENT_ROUTER_NO_MATCH,
                          event_router_evaluate(&event));
    event_envelope_clear(&event);
}

void test_route_mutation_invalidates_cache_and_timezone_failure_is_closed(void) {
    event_envelope_t event = detection_event(
        "22222222-2222-4222-8222-222222222222", "person", 0.9, NULL,
        1786991400);
    TEST_ASSERT_EQUAL_INT(EVENT_ROUTER_DEFAULT,
                          event_router_evaluate(&event));

    event_route_t route = route_definition("Offline only", OFFLINE_TYPE);
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK, db_event_route_create(&route));
    TEST_ASSERT_EQUAL_INT(EVENT_ROUTER_NO_MATCH,
                          event_router_evaluate(&event));

    safe_strcpy(route.event_types[0], DETECTION_TYPE,
                sizeof(route.event_types[0]), 0);
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK,
                          db_event_route_update(&route, route.revision));

    /* Simulate tzdata disappearing or a corrupt out-of-band database edit. */
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_exec(
            get_db_handle(),
            "UPDATE event_routes SET schedule_json='{"
            "\"version\":1,\"timezone\":\"Mars/Olympus\",\"windows\":[{"
            "\"days\":[1],\"start\":\"18:00\",\"end\":\"19:00\"}]}' "
            "WHERE enabled=1;",
            NULL, NULL, NULL));
    event_route_t generation_bump = route_definition("Disabled", OFFLINE_TYPE);
    generation_bump.enabled = false;
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK,
                          db_event_route_create(&generation_bump));
    TEST_ASSERT_EQUAL_INT(EVENT_ROUTER_ERROR,
                          event_router_evaluate(&event));
    event_envelope_clear(&event);

    event_router_stats_t stats;
    event_router_get_stats(&stats);
    TEST_ASSERT_GREATER_OR_EQUAL_UINT64(3, stats.cache_reloads);
    TEST_ASSERT_EQUAL_UINT64(1, stats.evaluation_errors);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_empty_routes_default_but_disabled_route_is_quiet);
    RUN_TEST(test_selector_predicate_and_utc_schedule_must_all_match);
    RUN_TEST(test_iana_timezone_and_overnight_windows_use_occurrence_time);
    RUN_TEST(test_route_mutation_invalidates_cache_and_timezone_failure_is_closed);
    int result = UNITY_END();
    event_router_shutdown();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
