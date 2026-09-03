/**
 * @file test_db_event_routes.c
 * @brief Event route validation, persistence, and optimistic concurrency tests.
 */

#define _POSIX_C_SOURCE 200809L

#include <sqlite3.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "database/db_core.h"
#include "database/db_event_destinations.h"
#include "database/db_event_routes.h"
#include "unity.h"
#include "utils/strings.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_event_routes.db"
#define DETECTION_TYPE "io.lightnvr.detection.object.v1"
#define OFFLINE_TYPE "io.lightnvr.camera.offline.v1"
#define HEALTH_ALERT_TYPE "io.lightnvr.system.health_alert.v1"
#define HEALTH_RECOVERED_TYPE "io.lightnvr.system.health_recovered.v1"

static event_route_t valid_route(const char *name) {
    event_route_t route;
    memset(&route, 0, sizeof(route));
    safe_strcpy(route.name, name, sizeof(route.name), 0);
    safe_strcpy(route.description, "Notify the operations bridge",
                sizeof(route.description), 0);
    route.enabled = true;
    safe_strcpy(route.destination_key, EVENT_ROUTE_DEFAULT_DESTINATION,
                sizeof(route.destination_key), 0);
    safe_strcpy(route.scope_type, "all", sizeof(route.scope_type), 0);
    safe_strcpy(route.predicate_json, "{\"version\":1}",
                sizeof(route.predicate_json), 0);
    safe_strcpy(route.schedule_json,
                "{\"version\":1,\"timezone\":\"America/New_York\","
                "\"windows\":[{\"days\":[1,2,3,4,5],"
                "\"start\":\"08:00\",\"end\":\"18:00\"}]}",
                sizeof(route.schedule_json), 0);
    safe_strcpy(route.event_types[0], DETECTION_TYPE,
                sizeof(route.event_types[0]), 0);
    safe_strcpy(route.event_types[1], OFFLINE_TYPE,
                sizeof(route.event_types[1]), 0);
    route.event_type_count = 2;
    route.debounce_seconds = 2;
    route.cooldown_seconds = 30;
    route.grouping_window_seconds = 10;
    route.max_events_per_minute = 120;
    return route;
}

static event_destination_t create_destination(void) {
    event_destination_t destination;
    memset(&destination, 0, sizeof(destination));
    safe_strcpy(destination.name, "Operations broker",
                sizeof(destination.name), 0);
    destination.enabled = true;
    safe_strcpy(destination.destination_type, "mqtt",
                sizeof(destination.destination_type), 0);
    safe_strcpy(destination.broker_host, "mqtt.example.test",
                sizeof(destination.broker_host), 0);
    destination.broker_port = 1883;
    safe_strcpy(destination.client_id, "lightnvr-routes-test",
                sizeof(destination.client_id), 0);
    safe_strcpy(destination.topic_template, "routes/{type}/{subject_id}",
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

void setUp(void) {
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK, sqlite3_exec(get_db_handle(), "DELETE FROM event_routes;",
                                NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_exec(get_db_handle(), "DELETE FROM event_destinations;",
                     NULL, NULL, NULL));
}

void tearDown(void) {}

void test_route_crud_uses_revision_compare_and_swap(void) {
    event_route_t route = valid_route("  SJC after-hours  ");
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK,
                          db_event_route_create(&route));
    TEST_ASSERT_EQUAL_UINT(36, strlen(route.uuid));
    TEST_ASSERT_EQUAL_STRING("SJC after-hours", route.name);
    TEST_ASSERT_EQUAL_INT64(1, route.revision);
    TEST_ASSERT_EQUAL_INT(2, route.event_type_count);
    TEST_ASSERT_EQUAL_INT(1, db_event_route_count());

    event_route_t loaded;
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK,
                          db_event_route_get(route.uuid, &loaded));
    TEST_ASSERT_EQUAL_STRING(route.uuid, loaded.uuid);
    TEST_ASSERT_NOT_NULL(strstr(loaded.schedule_json, "America/New_York"));

    safe_strcpy(loaded.name, "SJC overnight", sizeof(loaded.name), 0);
    loaded.enabled = false;
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK,
                          db_event_route_update(&loaded, 1));
    TEST_ASSERT_EQUAL_INT64(2, loaded.revision);
    TEST_ASSERT_FALSE(loaded.enabled);

    safe_strcpy(loaded.description, "stale write",
                sizeof(loaded.description), 0);
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_STALE,
                          db_event_route_update(&loaded, 1));
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_STALE,
                          db_event_route_delete(loaded.uuid, 1));
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK,
                          db_event_route_delete(loaded.uuid, 2));
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_NOT_FOUND,
                          db_event_route_get(loaded.uuid, &route));
}

void test_route_name_is_unique_case_insensitively(void) {
    event_route_t first = valid_route("Critical cameras");
    event_route_t second = valid_route("critical cameras");
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK,
                          db_event_route_create(&first));
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_CONFLICT,
                          db_event_route_create(&second));
}

void test_route_validates_types_scope_predicates_and_schedule(void) {
    char error[EVENT_ROUTE_VALIDATION_ERROR_MAX];
    event_route_t route = valid_route("Validation");
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_ROUTE_OK,
        db_event_route_validate(&route, error, sizeof(error)));

    safe_strcpy(route.event_types[1], DETECTION_TYPE,
                sizeof(route.event_types[1]), 0);
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_ROUTE_INVALID,
        db_event_route_validate(&route, error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "duplicate"));

    route = valid_route("Validation");
    safe_strcpy(route.scope_type, "selector", sizeof(route.scope_type), 0);
    safe_strcpy(route.selector_json,
                "{\"version\":1,\"expression\":{\"op\":\"all\"}}",
                sizeof(route.selector_json), 0);
    safe_strcpy(route.predicate_json,
                "{\"version\":1,\"detection\":{"
                "\"labels_any\":[\"person\",\"vehicle\"],"
                "\"min_confidence\":0.75,"
                "\"zone_ids_any\":[\"entry\"]}}",
                sizeof(route.predicate_json), 0);
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_ROUTE_OK,
        db_event_route_validate(&route, error, sizeof(error)));

    safe_strcpy(route.predicate_json,
                "{\"version\":1,\"detection\":{"
                "\"min_confidence\":1.5}}",
                sizeof(route.predicate_json), 0);
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_ROUTE_INVALID,
        db_event_route_validate(&route, error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "min_confidence"));

    route = valid_route("Validation");
    safe_strcpy(route.schedule_json,
                "{\"version\":1,\"timezone\":\"UTC\","
                "\"windows\":[{\"days\":[1,1],"
                "\"start\":\"08:00\",\"end\":\"09:00\"}]}",
                sizeof(route.schedule_json), 0);
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_ROUTE_INVALID,
        db_event_route_validate(&route, error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "unique"));

    route = valid_route("Validation");
    safe_strcpy(route.schedule_json,
                "{\"version\":1,\"timezone\":\"Mars/Olympus\","
                "\"windows\":[]}",
                sizeof(route.schedule_json), 0);
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_ROUTE_INVALID,
        db_event_route_validate(&route, error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "available timezone"));

    route = valid_route("Validation");
    safe_strcpy(route.destination_key, "mqtt:unknown",
                sizeof(route.destination_key), 0);
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_ROUTE_INVALID,
        db_event_route_validate(&route, error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "mqtt:default"));

    event_destination_t destination = create_destination();
    TEST_ASSERT_EQUAL_INT(
        0, db_event_destination_make_key(destination.uuid,
                                         route.destination_key));
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_ROUTE_OK,
        db_event_route_validate(&route, error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK, db_event_route_create(&route));
}

void test_health_predicates_are_bounded_and_system_scope_is_explicit(void) {
    char error[EVENT_ROUTE_VALIDATION_ERROR_MAX] = {0};
    event_route_t route = valid_route("Host health");
    safe_strcpy(route.event_types[0], HEALTH_ALERT_TYPE,
                sizeof(route.event_types[0]), 0);
    safe_strcpy(route.event_types[1], HEALTH_RECOVERED_TYPE,
                sizeof(route.event_types[1]), 0);
    safe_strcpy(route.predicate_json,
                "{\"version\":1,\"health\":{"
                "\"condition_codes_any\":[\"memory.available_low\"],"
                "\"severities_any\":[\"warning\",\"critical\"]}}",
                sizeof(route.predicate_json), 0);
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_ROUTE_OK,
        db_event_route_validate(&route, error, sizeof(error)));

    safe_strcpy(route.predicate_json,
                "{\"version\":1,\"health\":{"
                "\"condition_codes_any\":[\"memory.typo\"]}}",
                sizeof(route.predicate_json), 0);
    error[0] = '\0';
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_ROUTE_INVALID,
        db_event_route_validate(&route, error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "unknown value"));

    safe_strcpy(route.predicate_json,
                "{\"version\":1,\"health\":{"
                "\"severities_any\":[\"emergency\"]}}",
                sizeof(route.predicate_json), 0);
    error[0] = '\0';
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_ROUTE_INVALID,
        db_event_route_validate(&route, error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "unknown value"));

    safe_strcpy(route.event_types[1], OFFLINE_TYPE,
                sizeof(route.event_types[1]), 0);
    safe_strcpy(route.predicate_json,
                "{\"version\":1,\"health\":{"
                "\"severities_any\":[\"warning\"]}}",
                sizeof(route.predicate_json), 0);
    error[0] = '\0';
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_ROUTE_INVALID,
        db_event_route_validate(&route, error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "only system health"));

    route.event_type_count = 1;
    safe_strcpy(route.scope_type, "selector", sizeof(route.scope_type), 0);
    safe_strcpy(route.selector_json,
                "{\"version\":1,\"expression\":{\"op\":\"all\"}}",
                sizeof(route.selector_json), 0);
    safe_strcpy(route.predicate_json, "{\"version\":1}",
                sizeof(route.predicate_json), 0);
    error[0] = '\0';
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_ROUTE_INVALID,
        db_event_route_validate(&route, error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "only camera event types"));
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_route_crud_uses_revision_compare_and_swap);
    RUN_TEST(test_route_name_is_unique_case_insensitively);
    RUN_TEST(test_route_validates_types_scope_predicates_and_schedule);
    RUN_TEST(test_health_predicates_are_bounded_and_system_scope_is_explicit);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
