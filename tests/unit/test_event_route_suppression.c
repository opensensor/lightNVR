/**
 * @file test_event_route_suppression.c
 * @brief Durable event-route suppression policy tests.
 */

#define _POSIX_C_SOURCE 200809L

#include <sqlite3.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "database/db_core.h"
#include "database/db_event_route_suppression.h"
#include "database/db_event_routes.h"
#include "unity.h"
#include "utils/strings.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_event_route_suppression.db"
#define EVENT_TYPE "io.lightnvr.camera.offline.v1"
#define SUBJECT "camera/22222222-2222-4222-8222-222222222222"
#define EVENT_ID_1 "33333333-3333-4333-8333-333333333331"
#define EVENT_ID_2 "33333333-3333-4333-8333-333333333332"
#define EVENT_ID_3 "33333333-3333-4333-8333-333333333333"

static event_route_t route_definition(const char *name) {
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
    safe_strcpy(route.event_types[0], EVENT_TYPE,
                sizeof(route.event_types[0]), 0);
    route.event_type_count = 1;
    return route;
}

static void record_allowed(const event_route_t *route, const char *event_id,
                           int64_t now) {
    TEST_ASSERT_EQUAL_INT(
        EVENT_SUPPRESSION_PERMIT,
        db_event_route_suppression_record_allowed(
            route->uuid, route->revision, EVENT_TYPE, SUBJECT, event_id, now));
}

void setUp(void) {
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK, sqlite3_exec(get_db_handle(), "DELETE FROM event_routes;",
                                NULL, NULL, NULL));
}

void tearDown(void) {}

void test_permit_is_not_committed_until_outbox_acknowledgement(void) {
    event_route_t route = route_definition("Acknowledged delivery");
    route.cooldown_seconds = 30;
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK, db_event_route_create(&route));

    TEST_ASSERT_EQUAL_INT(
        EVENT_SUPPRESSION_PERMIT,
        db_event_route_suppression_check(&route, EVENT_TYPE, SUBJECT, 100));
    event_route_suppression_state_t state;
    TEST_ASSERT_EQUAL_INT(
        0, db_event_route_suppression_get(route.uuid, EVENT_TYPE, SUBJECT,
                                          &state));

    record_allowed(&route, EVENT_ID_1, 100);
    TEST_ASSERT_EQUAL_INT(
        EVENT_SUPPRESSION_COOLDOWN,
        db_event_route_suppression_check(&route, EVENT_TYPE, SUBJECT, 101));
    TEST_ASSERT_EQUAL_INT(
        1, db_event_route_suppression_get(route.uuid, EVENT_TYPE, SUBJECT,
                                          &state));
    TEST_ASSERT_EQUAL_INT64(100, state.last_allowed_at);
    TEST_ASSERT_EQUAL_INT64(1, state.suppressed_count);
}

void test_debounce_extends_from_the_latest_observation(void) {
    event_route_t route = route_definition("Debounce");
    route.debounce_seconds = 10;
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK, db_event_route_create(&route));
    record_allowed(&route, EVENT_ID_1, 100);

    TEST_ASSERT_EQUAL_INT(
        EVENT_SUPPRESSION_DEBOUNCE,
        db_event_route_suppression_check(&route, EVENT_TYPE, SUBJECT, 105));
    TEST_ASSERT_EQUAL_INT(
        EVENT_SUPPRESSION_DEBOUNCE,
        db_event_route_suppression_check(&route, EVENT_TYPE, SUBJECT, 114));
    TEST_ASSERT_EQUAL_INT(
        EVENT_SUPPRESSION_PERMIT,
        db_event_route_suppression_check(&route, EVENT_TYPE, SUBJECT, 124));
}

void test_cooldown_and_grouping_do_not_extend_on_suppression(void) {
    event_route_t cooldown = route_definition("Cooldown");
    cooldown.cooldown_seconds = 30;
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK,
                          db_event_route_create(&cooldown));
    record_allowed(&cooldown, EVENT_ID_1, 100);
    TEST_ASSERT_EQUAL_INT(
        EVENT_SUPPRESSION_COOLDOWN,
        db_event_route_suppression_check(&cooldown, EVENT_TYPE, SUBJECT, 129));
    TEST_ASSERT_EQUAL_INT(
        EVENT_SUPPRESSION_PERMIT,
        db_event_route_suppression_check(&cooldown, EVENT_TYPE, SUBJECT, 130));

    event_route_t grouping = route_definition("Grouping");
    grouping.grouping_window_seconds = 20;
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK,
                          db_event_route_create(&grouping));
    record_allowed(&grouping, EVENT_ID_1, 200);
    TEST_ASSERT_EQUAL_INT(
        EVENT_SUPPRESSION_GROUPING,
        db_event_route_suppression_check(&grouping, EVENT_TYPE, SUBJECT, 219));
    TEST_ASSERT_EQUAL_INT(
        EVENT_SUPPRESSION_PERMIT,
        db_event_route_suppression_check(&grouping, EVENT_TYPE, SUBJECT, 220));
}

void test_rate_limit_is_fixed_window_and_ack_is_idempotent(void) {
    event_route_t route = route_definition("Rate");
    route.max_events_per_minute = 2;
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK, db_event_route_create(&route));

    record_allowed(&route, EVENT_ID_1, 100);
    record_allowed(&route, EVENT_ID_1, 101);
    TEST_ASSERT_EQUAL_INT(
        EVENT_SUPPRESSION_PERMIT,
        db_event_route_suppression_check(&route, EVENT_TYPE, SUBJECT, 101));
    record_allowed(&route, EVENT_ID_2, 101);
    TEST_ASSERT_EQUAL_INT(
        EVENT_SUPPRESSION_RATE,
        db_event_route_suppression_check(&route, EVENT_TYPE, SUBJECT, 102));
    TEST_ASSERT_EQUAL_INT(
        EVENT_SUPPRESSION_PERMIT,
        db_event_route_suppression_check(&route, EVENT_TYPE, SUBJECT, 160));
}

void test_route_update_invalidates_state_and_stale_acknowledgements(void) {
    event_route_t route = route_definition("Mutable");
    route.cooldown_seconds = 30;
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK, db_event_route_create(&route));
    int64_t old_revision = route.revision;
    record_allowed(&route, EVENT_ID_1, 100);

    safe_strcpy(route.description, "new policy", sizeof(route.description), 0);
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK,
                          db_event_route_update(&route, old_revision));
    event_route_suppression_state_t state;
    TEST_ASSERT_EQUAL_INT(
        0, db_event_route_suppression_get(route.uuid, EVENT_TYPE, SUBJECT,
                                          &state));
    TEST_ASSERT_EQUAL_INT(
        EVENT_SUPPRESSION_STALE,
        db_event_route_suppression_record_allowed(
            route.uuid, old_revision, EVENT_TYPE, SUBJECT, EVENT_ID_2, 101));
    TEST_ASSERT_EQUAL_INT(
        EVENT_SUPPRESSION_PERMIT,
        db_event_route_suppression_check(&route, EVENT_TYPE, SUBJECT, 101));
}

void test_expired_state_is_pruned_in_bounded_batches(void) {
    event_route_t route = route_definition("Prunable");
    route.cooldown_seconds = 30;
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK, db_event_route_create(&route));
    record_allowed(&route, EVENT_ID_3, 100);

    int deleted = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_event_route_suppression_prune(101, 1, &deleted));
    TEST_ASSERT_EQUAL_INT(1, deleted);
    event_route_suppression_state_t state;
    TEST_ASSERT_EQUAL_INT(
        0, db_event_route_suppression_get(route.uuid, EVENT_TYPE, SUBJECT,
                                          &state));
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_permit_is_not_committed_until_outbox_acknowledgement);
    RUN_TEST(test_debounce_extends_from_the_latest_observation);
    RUN_TEST(test_cooldown_and_grouping_do_not_extend_on_suppression);
    RUN_TEST(test_rate_limit_is_fixed_window_and_ack_is_idempotent);
    RUN_TEST(test_route_update_invalidates_state_and_stale_acknowledgements);
    RUN_TEST(test_expired_state_is_pruned_in_bounded_batches);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
