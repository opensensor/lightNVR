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

void test_tiered_retention_orders_by_tier_then_age(void) {
    time_t now = time(NULL);
    const double multipliers[4] = {3.0, 2.0, 1.0, 0.25};

    recording_metadata_t critical = make_recording("cam3", "/rec/tier-critical.mp4",
                                                   now - 40 * 86400, "scheduled", false);
    critical.retention_tier = RETENTION_TIER_CRITICAL;
    add_recording_metadata(&critical);

    recording_metadata_t important = make_recording("cam3", "/rec/tier-important.mp4",
                                                    now - 25 * 86400, "scheduled", false);
    important.retention_tier = RETENTION_TIER_IMPORTANT;
    add_recording_metadata(&important);

    recording_metadata_t standard = make_recording("cam3", "/rec/tier-standard.mp4",
                                                   now - 12 * 86400, "scheduled", false);
    standard.retention_tier = RETENTION_TIER_STANDARD;
    add_recording_metadata(&standard);

    recording_metadata_t ephemeral = make_recording("cam3", "/rec/tier-ephemeral.mp4",
                                                    now - 3 * 86400, "scheduled", false);
    ephemeral.retention_tier = RETENTION_TIER_EPHEMERAL;
    add_recording_metadata(&ephemeral);

    recording_metadata_t out[10];
    int n = get_recordings_for_tiered_retention("cam3", 10, multipliers, out, 10);
    TEST_ASSERT_EQUAL_INT(4, n);
    TEST_ASSERT_EQUAL_STRING("/rec/tier-ephemeral.mp4", out[0].file_path);
    TEST_ASSERT_EQUAL_STRING("/rec/tier-standard.mp4", out[1].file_path);
    TEST_ASSERT_EQUAL_STRING("/rec/tier-important.mp4", out[2].file_path);
    TEST_ASSERT_EQUAL_STRING("/rec/tier-critical.mp4", out[3].file_path);
}

void test_tiered_retention_respects_retention_override_days(void) {
    time_t now = time(NULL);
    const double multipliers[4] = {3.0, 2.0, 1.0, 0.25};

    recording_metadata_t keep = make_recording("cam4", "/rec/override-keep.mp4",
                                               now - 20 * 86400, "scheduled", false);
    uint64_t keep_id = add_recording_metadata(&keep);
    TEST_ASSERT_NOT_EQUAL(0, keep_id);
    TEST_ASSERT_EQUAL_INT(0, set_recording_retention_override(keep_id, 30));

    recording_metadata_t delete_me = make_recording("cam4", "/rec/override-delete.mp4",
                                                    now - 21 * 86400, "scheduled", false);
    uint64_t delete_id = add_recording_metadata(&delete_me);
    TEST_ASSERT_NOT_EQUAL(0, delete_id);
    TEST_ASSERT_EQUAL_INT(0, set_recording_retention_override(delete_id, 5));

    recording_metadata_t plain = make_recording("cam4", "/rec/no-override.mp4",
                                                now - 22 * 86400, "scheduled", false);
    add_recording_metadata(&plain);

    recording_metadata_t out[10];
    int n = get_recordings_for_tiered_retention("cam4", 7, multipliers, out, 10);
    TEST_ASSERT_EQUAL_INT(2, n);
    TEST_ASSERT_EQUAL_STRING("/rec/no-override.mp4", out[0].file_path);
    TEST_ASSERT_EQUAL_STRING("/rec/override-delete.mp4", out[1].file_path);
}

void test_tiered_retention_with_null_stream_name_includes_all_streams(void) {
    time_t now = time(NULL);
    const double multipliers[4] = {3.0, 2.0, 1.0, 0.25};

    recording_metadata_t cam_a = make_recording("camA", "/rec/global-a.mp4",
                                                now - 15 * 86400, "scheduled", false);
    add_recording_metadata(&cam_a);

    recording_metadata_t cam_b = make_recording("camB", "/rec/global-b.mp4",
                                                now - 16 * 86400, "scheduled", false);
    add_recording_metadata(&cam_b);

    recording_metadata_t out[10];
    int n = get_recordings_for_tiered_retention(NULL, 7, multipliers, out, 10);
    TEST_ASSERT_EQUAL_INT(2, n);
    TEST_ASSERT_EQUAL_STRING("/rec/global-b.mp4", out[0].file_path);
    TEST_ASSERT_EQUAL_STRING("/rec/global-a.mp4", out[1].file_path);
}

void test_pressure_cleanup_filters_and_orders_candidates(void) {
    time_t now = time(NULL);

    recording_metadata_t critical = make_recording("cam5", "/rec/pressure-critical.mp4",
                                                   now - 50 * 86400, "scheduled", false);
    uint64_t critical_id = add_recording_metadata(&critical);
    TEST_ASSERT_EQUAL_INT(0, set_recording_retention_tier(critical_id, RETENTION_TIER_CRITICAL));

    recording_metadata_t important = make_recording("cam5", "/rec/pressure-important.mp4",
                                                    now - 40 * 86400, "scheduled", false);
    uint64_t important_id = add_recording_metadata(&important);
    TEST_ASSERT_EQUAL_INT(0, set_recording_retention_tier(important_id, RETENTION_TIER_IMPORTANT));

    recording_metadata_t standard = make_recording("cam5", "/rec/pressure-standard.mp4",
                                                   now - 30 * 86400, "scheduled", false);
    uint64_t standard_id = add_recording_metadata(&standard);
    TEST_ASSERT_EQUAL_INT(0, set_recording_retention_tier(standard_id, RETENTION_TIER_STANDARD));

    recording_metadata_t eph_old = make_recording("cam5", "/rec/pressure-eph-old.mp4",
                                                  now - 20 * 86400, "scheduled", false);
    uint64_t eph_old_id = add_recording_metadata(&eph_old);
    TEST_ASSERT_EQUAL_INT(0, set_recording_retention_tier(eph_old_id, RETENTION_TIER_EPHEMERAL));

    recording_metadata_t eph_new = make_recording("cam5", "/rec/pressure-eph-new.mp4",
                                                  now - 10 * 86400, "scheduled", false);
    uint64_t eph_new_id = add_recording_metadata(&eph_new);
    TEST_ASSERT_EQUAL_INT(0, set_recording_retention_tier(eph_new_id, RETENTION_TIER_EPHEMERAL));

    recording_metadata_t protected_eph = make_recording("cam5", "/rec/pressure-protected.mp4",
                                                        now - 25 * 86400, "scheduled", true);
    uint64_t protected_id = add_recording_metadata(&protected_eph);
    TEST_ASSERT_EQUAL_INT(0, set_recording_retention_tier(protected_id, RETENTION_TIER_EPHEMERAL));
    TEST_ASSERT_EQUAL_INT(0, set_recording_protected(protected_id, true));

    recording_metadata_t ineligible_eph = make_recording("cam5", "/rec/pressure-ineligible.mp4",
                                                         now - 24 * 86400, "scheduled", false);
    uint64_t ineligible_id = add_recording_metadata(&ineligible_eph);
    TEST_ASSERT_EQUAL_INT(0, set_recording_retention_tier(ineligible_id, RETENTION_TIER_EPHEMERAL));
    TEST_ASSERT_EQUAL_INT(0, set_recording_disk_pressure_eligible(ineligible_id, false));

    recording_metadata_t incomplete_eph = make_recording("cam5", "/rec/pressure-incomplete.mp4",
                                                         now - 23 * 86400, "scheduled", false);
    incomplete_eph.is_complete = false;
    incomplete_eph.end_time = 0;
    incomplete_eph.retention_tier = RETENTION_TIER_EPHEMERAL;
    add_recording_metadata(&incomplete_eph);

    recording_metadata_t out[10];
    int n = get_recordings_for_pressure_cleanup(out, 10);
    TEST_ASSERT_EQUAL_INT(5, n);
    TEST_ASSERT_EQUAL_STRING("/rec/pressure-eph-old.mp4", out[0].file_path);
    TEST_ASSERT_EQUAL_STRING("/rec/pressure-eph-new.mp4", out[1].file_path);
    TEST_ASSERT_EQUAL_STRING("/rec/pressure-standard.mp4", out[2].file_path);
    TEST_ASSERT_EQUAL_STRING("/rec/pressure-important.mp4", out[3].file_path);
    TEST_ASSERT_EQUAL_STRING("/rec/pressure-critical.mp4", out[4].file_path);

    n = get_recordings_for_pressure_cleanup(out, 3);
    TEST_ASSERT_EQUAL_INT(3, n);
    TEST_ASSERT_EQUAL_STRING("/rec/pressure-eph-old.mp4", out[0].file_path);
    TEST_ASSERT_EQUAL_STRING("/rec/pressure-eph-new.mp4", out[1].file_path);
    TEST_ASSERT_EQUAL_STRING("/rec/pressure-standard.mp4", out[2].file_path);
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
    RUN_TEST(test_tiered_retention_orders_by_tier_then_age);
    RUN_TEST(test_tiered_retention_respects_retention_override_days);
    RUN_TEST(test_tiered_retention_with_null_stream_name_includes_all_streams);
    RUN_TEST(test_pressure_cleanup_filters_and_orders_candidates);
    RUN_TEST(test_delete_recording_removes_it);

    int result = UNITY_END();

    shutdown_database();
    unlink(TEST_DB_PATH);

    return result;
}

