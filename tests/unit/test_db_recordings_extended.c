/**
 * @file test_db_recordings_extended.c
 * @brief Layer 2 — recording metadata CRUD, tiers, retention, storage bytes
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

#define TEST_DB_PATH "/tmp/lightnvr_unit_recordings_ext_test.db"

static recording_metadata_t make_rec(const char *stream, const char *path, time_t start) {
    recording_metadata_t m;
    memset(&m, 0, sizeof(m));
    strncpy(m.stream_name, stream, sizeof(m.stream_name) - 1);
    strncpy(m.file_path,   path,   sizeof(m.file_path)   - 1);
    strncpy(m.codec,       "h264", sizeof(m.codec)       - 1);
    strncpy(m.trigger_type, "scheduled", sizeof(m.trigger_type) - 1);
    m.start_time  = start;
    m.end_time    = start + 60;
    m.size_bytes  = 1024 * 1024;
    m.width = 1920; m.height = 1080; m.fps = 30;
    m.is_complete = true;
    m.protected   = false;
    m.retention_override_days = -1;
    m.retention_tier = RETENTION_TIER_STANDARD;
    m.disk_pressure_eligible = true;
    return m;
}

static void clear_recordings(void) {
    sqlite3_exec(get_db_handle(), "DELETE FROM recordings;", NULL, NULL, NULL);
}

void setUp(void)    { clear_recordings(); }
void tearDown(void) {}

/* add / get by id */
void test_add_and_get_by_id(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/a.mp4", now);
    uint64_t id = add_recording_metadata(&m);
    TEST_ASSERT_NOT_EQUAL(0, id);

    recording_metadata_t got;
    int rc = get_recording_metadata_by_id(id, &got);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("cam1", got.stream_name);
}

/* update_recording_metadata */
void test_update_recording_metadata(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/b.mp4", now);
    uint64_t id = add_recording_metadata(&m);
    TEST_ASSERT_NOT_EQUAL(0, id);

    int rc = update_recording_metadata(id, now + 120, 2048 * 1024, true);
    TEST_ASSERT_EQUAL_INT(0, rc);

    recording_metadata_t got;
    get_recording_metadata_by_id(id, &got);
    TEST_ASSERT_EQUAL_INT(now + 120, got.end_time);
    TEST_ASSERT_TRUE(got.is_complete);
}

/* get_recording_metadata stream filter */
void test_get_recording_metadata_stream_filter(void) {
    time_t now = time(NULL);
    recording_metadata_t m1 = make_rec("cam1", "/rec/c1.mp4", now);
    recording_metadata_t m2 = make_rec("cam2", "/rec/c2.mp4", now);
    add_recording_metadata(&m1);
    add_recording_metadata(&m2);

    recording_metadata_t out[10];
    int n = get_recording_metadata(0, 0, "cam1", out, 10);
    TEST_ASSERT_EQUAL_INT(1, n);
    TEST_ASSERT_EQUAL_STRING("cam1", out[0].stream_name);
}

/* get_recording_metadata_by_path */
void test_get_recording_metadata_by_path(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/bypath.mp4", now);
    add_recording_metadata(&m);

    recording_metadata_t got;
    int rc = get_recording_metadata_by_path("/rec/bypath.mp4", &got);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("/rec/bypath.mp4", got.file_path);
}

/* get_recording_count */
void test_get_recording_count(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/cnt.mp4", now);
    add_recording_metadata(&m);
    int cnt = get_recording_count(0, 0, "cam1", 0, NULL);
    TEST_ASSERT_EQUAL_INT(1, cnt);
}

/* get_recording_metadata_paginated */
void test_get_recording_metadata_paginated(void) {
    time_t now = time(NULL);
    for (int i = 0; i < 5; i++) {
        char path[64];
        snprintf(path, sizeof(path), "/rec/page%d.mp4", i);
        recording_metadata_t m = make_rec("cam1", path, now - i * 100);
        add_recording_metadata(&m);
    }
    recording_metadata_t out[10];
    int n = get_recording_metadata_paginated(0, 0, "cam1", 0, NULL,
                                             "start_time", "desc", out, 3, 0);
    TEST_ASSERT_EQUAL_INT(3, n);
}

/* set_recording_retention_tier */
void test_set_recording_retention_tier(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/tier.mp4", now);
    uint64_t id = add_recording_metadata(&m);
    int rc = set_recording_retention_tier(id, RETENTION_TIER_CRITICAL);
    TEST_ASSERT_EQUAL_INT(0, rc);
    recording_metadata_t got;
    get_recording_metadata_by_id(id, &got);
    TEST_ASSERT_EQUAL_INT(RETENTION_TIER_CRITICAL, got.retention_tier);
}

/* set_recording_disk_pressure_eligible */
void test_set_recording_disk_pressure_eligible(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/dp.mp4", now);
    uint64_t id = add_recording_metadata(&m);
    int rc = set_recording_disk_pressure_eligible(id, false);
    TEST_ASSERT_EQUAL_INT(0, rc);
    recording_metadata_t got;
    get_recording_metadata_by_id(id, &got);
    TEST_ASSERT_FALSE(got.disk_pressure_eligible);
}

/* set_recording_retention_override */
void test_set_recording_retention_override(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/ov.mp4", now);
    uint64_t id = add_recording_metadata(&m);
    int rc = set_recording_retention_override(id, 90);
    TEST_ASSERT_EQUAL_INT(0, rc);
}

/* get_stream_storage_bytes */
void test_get_stream_storage_bytes(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam_sb", "/rec/sb.mp4", now);
    add_recording_metadata(&m);
    int64_t bytes = get_stream_storage_bytes("cam_sb");
    TEST_ASSERT_GREATER_THAN(0, bytes);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_add_and_get_by_id);
    RUN_TEST(test_update_recording_metadata);
    RUN_TEST(test_get_recording_metadata_stream_filter);
    RUN_TEST(test_get_recording_metadata_by_path);
    RUN_TEST(test_get_recording_count);
    RUN_TEST(test_get_recording_metadata_paginated);
    RUN_TEST(test_set_recording_retention_tier);
    RUN_TEST(test_set_recording_disk_pressure_eligible);
    RUN_TEST(test_set_recording_retention_override);
    RUN_TEST(test_get_stream_storage_bytes);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}

