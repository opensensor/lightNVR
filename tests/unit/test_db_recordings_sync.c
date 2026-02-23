/**
 * @file test_db_recordings_sync.c
 * @brief Layer 2 Unity tests for database/db_recordings_sync.c
 *
 * Tests:
 *   force_recording_sync()        - immediate sync of recordings needing size update
 *   start_recording_sync_thread() - background thread lifecycle
 *   stop_recording_sync_thread()  - thread join
 *
 * The background thread performs an initial sync then sleeps.
 * Because the minimum interval is 10 s we start/stop it quickly in tests
 * and rely mostly on force_recording_sync() for functional coverage.
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
#include "database/db_recordings.h"
#include "database/db_recordings_sync.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_recordings_sync_test.db"

/* ---- helpers ---- */

static recording_metadata_t make_zero_size_rec(const char *stream,
                                               const char *path,
                                               time_t start) {
    recording_metadata_t m;
    memset(&m, 0, sizeof(m));
    strncpy(m.stream_name, stream, sizeof(m.stream_name) - 1);
    strncpy(m.file_path,   path,   sizeof(m.file_path)   - 1);
    strncpy(m.codec,       "h264", sizeof(m.codec)       - 1);
    strncpy(m.trigger_type, "scheduled", sizeof(m.trigger_type) - 1);
    m.start_time  = start;
    m.end_time    = start + 60;
    m.size_bytes  = 0;        /* <-- needs sync */
    m.width = 1920; m.height = 1080; m.fps = 30;
    m.is_complete = true;     /* <-- only complete recordings are synced */
    m.protected   = false;
    m.retention_override_days = -1;
    m.retention_tier = 2;
    m.disk_pressure_eligible = true;
    return m;
}

static void clear_recordings(void) {
    sqlite3_exec(get_db_handle(), "DELETE FROM recordings;", NULL, NULL, NULL);
}

/* ---- Unity boilerplate ---- */
void setUp(void)    { clear_recordings(); }
void tearDown(void) {}

/* ================================================================
 * force_recording_sync — basic cases
 * ================================================================ */

void test_force_sync_empty_db_returns_zero(void) {
    int rc = force_recording_sync();
    /* No recordings → 0 updated */
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_force_sync_with_nonexistent_file_returns_zero(void) {
    /* Insert a recording pointing to a file that does not exist on disk */
    time_t now = time(NULL);
    recording_metadata_t m = make_zero_size_rec(
        "sync_cam", "/tmp/lightnvr_unit_no_such_file.mp4", now);
    uint64_t id = add_recording_metadata(&m);
    TEST_ASSERT_TRUE(id != 0);

    /* sync_recording_file_size() will stat() the path, find it missing,
       and return -1.  force_recording_sync() returns updated_count (0). */
    int rc = force_recording_sync();
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_force_sync_incomplete_recording_skipped(void) {
    /* is_complete = false → not selected by the query */
    time_t now = time(NULL);
    recording_metadata_t m = make_zero_size_rec("sync_cam2", "/tmp/inc.mp4", now);
    m.is_complete = false;
    add_recording_metadata(&m);

    int rc = force_recording_sync();
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_force_sync_already_sized_recording_skipped(void) {
    /* size_bytes > 0 → not selected by the query */
    time_t now = time(NULL);
    recording_metadata_t m = make_zero_size_rec("sync_cam3", "/tmp/sized.mp4", now);
    m.size_bytes = 1024 * 1024;
    add_recording_metadata(&m);

    int rc = force_recording_sync();
    TEST_ASSERT_EQUAL_INT(0, rc);
}

/* ================================================================
 * Thread lifecycle
 * ================================================================ */

void test_start_thread_succeeds(void) {
    /* Use minimum interval so we can stop quickly */
    int rc = start_recording_sync_thread(10);
    TEST_ASSERT_EQUAL_INT(0, rc);

    /* Clean up */
    stop_recording_sync_thread();
}

void test_start_thread_twice_returns_zero(void) {
    start_recording_sync_thread(10);
    /* Calling start again when already running must return 0 */
    int rc = start_recording_sync_thread(10);
    TEST_ASSERT_EQUAL_INT(0, rc);

    stop_recording_sync_thread();
}

void test_stop_thread_succeeds(void) {
    start_recording_sync_thread(10);
    int rc = stop_recording_sync_thread();
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_stop_thread_when_not_running_returns_zero(void) {
    /* Thread not started: stop should be a no-op returning 0 */
    int rc = stop_recording_sync_thread();
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_start_stop_start_stop_cycle(void) {
    TEST_ASSERT_EQUAL_INT(0, start_recording_sync_thread(10));
    TEST_ASSERT_EQUAL_INT(0, stop_recording_sync_thread());
    TEST_ASSERT_EQUAL_INT(0, start_recording_sync_thread(10));
    TEST_ASSERT_EQUAL_INT(0, stop_recording_sync_thread());
}

/* ================================================================
 * main
 * ================================================================ */

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }

    UNITY_BEGIN();
    RUN_TEST(test_force_sync_empty_db_returns_zero);
    RUN_TEST(test_force_sync_with_nonexistent_file_returns_zero);
    RUN_TEST(test_force_sync_incomplete_recording_skipped);
    RUN_TEST(test_force_sync_already_sized_recording_skipped);
    RUN_TEST(test_start_thread_succeeds);
    RUN_TEST(test_start_thread_twice_returns_zero);
    RUN_TEST(test_stop_thread_succeeds);
    RUN_TEST(test_stop_thread_when_not_running_returns_zero);
    RUN_TEST(test_start_stop_start_stop_cycle);
    int result = UNITY_END();

    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}

