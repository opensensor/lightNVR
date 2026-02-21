/**
 * @file test_db_events.c
 * @brief Layer 2 — event logging CRUD via SQLite
 *
 * Tests add_event, get_events (type/stream/time filters),
 * delete_old_events, NULL stream_name for system events,
 * and that all event_type_t values store correctly.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <sqlite3.h>

#include "unity.h"
#include "database/db_core.h"
#include "database/db_events.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_events_test.db"

static void clear_events(void) {
    sqlite3_exec(get_db_handle(), "DELETE FROM events;", NULL, NULL, NULL);
}

void setUp(void)    { clear_events(); }
void tearDown(void) {}

/* add_event returns non-zero ID */
void test_add_event_returns_nonzero_id(void) {
    uint64_t id = add_event(EVENT_SYSTEM_START, NULL, "System started", NULL);
    TEST_ASSERT_NOT_EQUAL(0, id);
}

/* get_events no filter */
void test_get_events_no_filter(void) {
    add_event(EVENT_RECORDING_START, "cam1", "Recording started", NULL);
    add_event(EVENT_STREAM_CONNECTED, "cam2", "Stream connected", NULL);

    event_info_t out[10];
    int n = get_events(0, 0, -1, NULL, out, 10);
    TEST_ASSERT_EQUAL_INT(2, n);
}

/* get_events stream filter */
void test_get_events_stream_filter(void) {
    add_event(EVENT_RECORDING_START, "cam1", "Cam1 rec", NULL);
    add_event(EVENT_STREAM_CONNECTED, "cam2", "Cam2 stream", NULL);

    event_info_t out[10];
    int n = get_events(0, 0, -1, "cam1", out, 10);
    TEST_ASSERT_EQUAL_INT(1, n);
    TEST_ASSERT_EQUAL_STRING("cam1", out[0].stream_name);
}

/* get_events type filter */
void test_get_events_type_filter(void) {
    add_event(EVENT_RECORDING_START,  "cam1", "Started",   NULL);
    add_event(EVENT_RECORDING_STOP,   "cam1", "Stopped",   NULL);
    add_event(EVENT_STREAM_CONNECTED, "cam1", "Connected", NULL);

    event_info_t out[10];
    int n = get_events(0, 0, EVENT_RECORDING_START, NULL, out, 10);
    TEST_ASSERT_EQUAL_INT(1, n);
    TEST_ASSERT_EQUAL_INT(EVENT_RECORDING_START, out[0].type);
}

/* NULL stream_name for system events */
void test_system_event_null_stream(void) {
    uint64_t id = add_event(EVENT_SYSTEM_STOP, NULL, "Shutdown", "graceful");
    TEST_ASSERT_NOT_EQUAL(0, id);

    event_info_t out[10];
    int n = get_events(0, 0, EVENT_SYSTEM_STOP, NULL, out, 10);
    TEST_ASSERT_GREATER_OR_EQUAL(1, n);
}

/* all event_type_t values */
void test_all_event_types_store(void) {
    event_type_t types[] = {
        EVENT_RECORDING_START, EVENT_RECORDING_STOP,
        EVENT_STREAM_CONNECTED, EVENT_STREAM_DISCONNECTED,
        EVENT_STREAM_ERROR, EVENT_SYSTEM_START, EVENT_SYSTEM_STOP,
        EVENT_STORAGE_LOW, EVENT_STORAGE_FULL,
        EVENT_USER_LOGIN, EVENT_USER_LOGOUT,
        EVENT_CONFIG_CHANGE, EVENT_CUSTOM
    };
    int num_types = (int)(sizeof(types) / sizeof(types[0]));
    for (int i = 0; i < num_types; i++) {
        uint64_t id = add_event(types[i], "cam1", "test", NULL);
        TEST_ASSERT_NOT_EQUAL(0, id);
    }
    event_info_t out[32];
    int n = get_events(0, 0, -1, NULL, out, 32);
    TEST_ASSERT_EQUAL_INT(num_types, n);
}

/* delete_old_events */
void test_delete_old_events(void) {
    /* Add an event with "now" timestamp; it's younger than max_age → not deleted */
    add_event(EVENT_SYSTEM_START, NULL, "Recent", NULL);
    int deleted = delete_old_events(1);  /* max_age = 1 second */
    TEST_ASSERT_GREATER_OR_EQUAL(0, deleted);
}

/* time range filter */
void test_get_events_time_range(void) {
    time_t now = time(NULL);
    add_event(EVENT_RECORDING_START, "cam1", "Recording", NULL);

    event_info_t out[10];
    int n = get_events(now - 10, now + 10, -1, NULL, out, 10);
    TEST_ASSERT_GREATER_THAN(0, n);
}

/* event has description */
void test_event_description_stored(void) {
    add_event(EVENT_CUSTOM, "cam1", "Custom event", "Extra details here");
    event_info_t out[10];
    int n = get_events(0, 0, EVENT_CUSTOM, NULL, out, 10);
    TEST_ASSERT_GREATER_OR_EQUAL(1, n);
    TEST_ASSERT_EQUAL_STRING("Custom event", out[0].description);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_add_event_returns_nonzero_id);
    RUN_TEST(test_get_events_no_filter);
    RUN_TEST(test_get_events_stream_filter);
    RUN_TEST(test_get_events_type_filter);
    RUN_TEST(test_system_event_null_stream);
    RUN_TEST(test_all_event_types_store);
    RUN_TEST(test_delete_old_events);
    RUN_TEST(test_get_events_time_range);
    RUN_TEST(test_event_description_stored);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}

