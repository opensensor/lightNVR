/**
 * @file test_storage_retention_sqlite.c
 * @brief Layer 2 integration tests — retention queries against in-memory SQLite
 *
 * Uses a real SQLite database (temp file, full schema via embedded migrations)
 * to verify:
 *   - get_recordings_for_retention()      (time-based culling)
 *   - get_recordings_for_quota_enforcement() (oldest-first quota)
 *   - set_recording_protected()            (protection prevents deletion)
 *   - delete_recording_metadata()          (record gone after delete)
 *
 * The database is initialised once per process; setUp() clears the
 * recordings table so each test starts from a blank slate.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <sqlite3.h>

#include "unity.h"
#include "database/db_core.h"
#include "database/db_recordings.h"

/* ---- test database path ---- */
#define TEST_DB_PATH "/tmp/lightnvr_unit_retention_test.db"

/* ---- helpers ---- */

/** Build a minimal, complete recording_metadata_t for insertion. */
static recording_metadata_t make_recording(const char *stream,
                                           const char *path,
                                           time_t start,
                                           const char *trigger,
                                           bool protected_flag) {
    recording_metadata_t m;
    memset(&m, 0, sizeof(m));
    strncpy(m.stream_name,  stream,  sizeof(m.stream_name)  - 1);
    strncpy(m.file_path,    path,    sizeof(m.file_path)    - 1);
    strncpy(m.codec,        "h264",  sizeof(m.codec)        - 1);
    strncpy(m.trigger_type, trigger, sizeof(m.trigger_type) - 1);
    m.start_time         = start;
    m.end_time           = start + 60;   /* 1-minute clip */
    m.size_bytes         = 1024 * 1024;  /* 1 MB */
    m.width              = 1920;
    m.height             = 1080;
    m.fps                = 30;
    m.is_complete        = true;
    m.protected          = protected_flag;
    m.retention_override_days = -1;
    m.retention_tier     = RETENTION_TIER_STANDARD;
    m.disk_pressure_eligible = true;
    return m;
}

/** Wipe the recordings table between tests. */
static void clear_recordings(void) {
    sqlite3 *db = get_db_handle();
    sqlite3_exec(db, "DELETE FROM recordings;", NULL, NULL, NULL);
}

/* ---- Unity boilerplate ---- */
void setUp(void)    { clear_recordings(); }
void tearDown(void) {}

/* ================================================================
 * Tests
 * ================================================================ */

void test_empty_db_returns_zero_for_retention(void) {
    recording_metadata_t out[10];
    int n = get_recordings_for_retention("cam1", 7, 14, out, 10);
    TEST_ASSERT_EQUAL_INT(0, n);
}

void test_old_recording_is_returned_by_retention(void) {
    time_t now = time(NULL);
    /* 10 days old, retention = 7 days */
    recording_metadata_t m = make_recording("cam1", "/rec/a.mp4",
                                            now - 10 * 86400,
                                            "scheduled", false);
    uint64_t id = add_recording_metadata(&m);
    TEST_ASSERT_NOT_EQUAL(0, id);

    recording_metadata_t out[10];
    int n = get_recordings_for_retention("cam1", 7, 14, out, 10);
    TEST_ASSERT_EQUAL_INT(1, n);
    TEST_ASSERT_EQUAL_STRING("cam1", out[0].stream_name);
}

void test_recent_recording_is_not_returned_by_retention(void) {
    time_t now = time(NULL);
    /* 3 days old, retention = 7 days */
    recording_metadata_t m = make_recording("cam1", "/rec/b.mp4",
                                            now - 3 * 86400,
                                            "scheduled", false);
    uint64_t id = add_recording_metadata(&m);
    TEST_ASSERT_NOT_EQUAL(0, id);

    recording_metadata_t out[10];
    int n = get_recordings_for_retention("cam1", 7, 14, out, 10);
    TEST_ASSERT_EQUAL_INT(0, n);
}

void test_protected_recording_is_never_returned(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_recording("cam1", "/rec/c.mp4",
                                            now - 10 * 86400,
                                            "scheduled", true /* protected */);
    uint64_t id = add_recording_metadata(&m);
    TEST_ASSERT_NOT_EQUAL(0, id);

    /* Mark as protected via the setter too, for belt-and-suspenders */
    set_recording_protected(id, true);

    recording_metadata_t out[10];
    int n = get_recordings_for_retention("cam1", 7, 14, out, 10);
    TEST_ASSERT_EQUAL_INT(0, n);
}

void test_detection_recording_uses_longer_detection_retention(void) {
    time_t now = time(NULL);
    /* 10 days old, detection retention = 14 → still within window, not returned */
    recording_metadata_t m = make_recording("cam1", "/rec/d.mp4",
                                            now - 10 * 86400,
                                            "detection", false);
    uint64_t id = add_recording_metadata(&m);
    TEST_ASSERT_NOT_EQUAL(0, id);

    recording_metadata_t out[10];
    /* regular=7, detection=14: the recording is 10 days old, detection window=14 → kept */
    int n = get_recordings_for_retention("cam1", 7, 14, out, 10);
    TEST_ASSERT_EQUAL_INT(0, n);
}

void test_detection_recording_expired_detection_retention(void) {
    time_t now = time(NULL);
    /* 20 days old, detection retention = 14 → expired, must be returned */
    recording_metadata_t m = make_recording("cam1", "/rec/e.mp4",
                                            now - 20 * 86400,
                                            "detection", false);
    uint64_t id = add_recording_metadata(&m);
    TEST_ASSERT_NOT_EQUAL(0, id);

    recording_metadata_t out[10];
    int n = get_recordings_for_retention("cam1", 7, 14, out, 10);
    TEST_ASSERT_EQUAL_INT(1, n);
}

void test_quota_enforcement_returns_oldest_first(void) {
    time_t now = time(NULL);
    /* Insert three recordings at different ages */
    recording_metadata_t new_rec  = make_recording("cam2", "/rec/new.mp4",  now - 1 * 86400, "scheduled", false);
    recording_metadata_t mid_rec  = make_recording("cam2", "/rec/mid.mp4",  now - 5 * 86400, "scheduled", false);
    recording_metadata_t old_rec  = make_recording("cam2", "/rec/old.mp4",  now - 9 * 86400, "scheduled", false);
    add_recording_metadata(&new_rec);
    add_recording_metadata(&mid_rec);
    add_recording_metadata(&old_rec);

    recording_metadata_t out[10];
    int n = get_recordings_for_quota_enforcement("cam2", out, 10);
    TEST_ASSERT_EQUAL_INT(3, n);
    /* Oldest must come first */
    TEST_ASSERT_EQUAL_STRING("/rec/old.mp4", out[0].file_path);
}

void test_delete_recording_removes_it(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_recording("cam1", "/rec/del.mp4",
                                            now - 10 * 86400,
                                            "scheduled", false);
    uint64_t id = add_recording_metadata(&m);
    TEST_ASSERT_NOT_EQUAL(0, id);

    /* Verify it exists */
    recording_metadata_t out[10];
    int before = get_recordings_for_retention("cam1", 7, 14, out, 10);
    TEST_ASSERT_EQUAL_INT(1, before);

    /* Delete it */
    int rc = delete_recording_metadata(id);
    TEST_ASSERT_EQUAL_INT(0, rc);

    /* Verify it's gone */
    int after = get_recordings_for_retention("cam1", 7, 14, out, 10);
    TEST_ASSERT_EQUAL_INT(0, after);
}

/* ================================================================
 * main — init DB once, run all tests, shutdown DB
 * ================================================================ */

int main(void) {
    /* Remove stale test DB from a previous run */
    unlink(TEST_DB_PATH);

    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database() failed\n");
        return 1;
    }

    UNITY_BEGIN();

    RUN_TEST(test_empty_db_returns_zero_for_retention);
    RUN_TEST(test_old_recording_is_returned_by_retention);
    RUN_TEST(test_recent_recording_is_not_returned_by_retention);
    RUN_TEST(test_protected_recording_is_never_returned);
    RUN_TEST(test_detection_recording_uses_longer_detection_retention);
    RUN_TEST(test_detection_recording_expired_detection_retention);
    RUN_TEST(test_quota_enforcement_returns_oldest_first);
    RUN_TEST(test_delete_recording_removes_it);

    int result = UNITY_END();

    shutdown_database();
    unlink(TEST_DB_PATH);

    return result;
}

