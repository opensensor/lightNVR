/**
 * @file test_db_motion_config.c
 * @brief Layer 2 Unity tests for database/db_motion_config.c
 *
 * Tests motion recording configuration CRUD, motion recording metadata
 * insertion, stats queries, and disk usage aggregation.
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
#include "database/db_motion_config.h"
#include "database/db_streams.h"
#include "video/onvif_motion_recording.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_motion_config_test.db"

/* ---- helpers ---- */

static motion_recording_config_t make_config(bool enabled) {
    motion_recording_config_t cfg;
    memset(&cfg, 0, sizeof(cfg));
    cfg.enabled           = enabled;
    cfg.pre_buffer_seconds  = 10;
    cfg.post_buffer_seconds = 20;
    cfg.max_file_duration   = 300;
    strncpy(cfg.codec,   "h264",   sizeof(cfg.codec) - 1);
    strncpy(cfg.quality, "medium", sizeof(cfg.quality) - 1);
    cfg.retention_days = 7;
    return cfg;
}

static void clear_all(void) {
    sqlite3 *db = get_db_handle();
    sqlite3_exec(db, "DELETE FROM motion_recording_config;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM motion_recordings;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM streams;", NULL, NULL, NULL);
}

static void ensure_stream(const char *name) {
    stream_config_t s;
    memset(&s, 0, sizeof(s));
    strncpy(s.name, name, sizeof(s.name) - 1);
    strncpy(s.url, "rtsp://localhost/test", sizeof(s.url) - 1);
    s.enabled  = true;
    s.width    = 1920;
    s.height   = 1080;
    s.fps      = 30;
    s.protocol = STREAM_PROTOCOL_TCP;
    add_stream_config(&s);
}

/* ---- Unity boilerplate ---- */
void setUp(void)    { clear_all(); }
void tearDown(void) {}

/* ================================================================
 * save_motion_config / load_motion_config
 * ================================================================ */

void test_save_and_load_round_trip(void) {
    ensure_stream("cam1");
    motion_recording_config_t cfg = make_config(true);
    int rc = save_motion_config("cam1", &cfg);
    TEST_ASSERT_EQUAL_INT(0, rc);

    motion_recording_config_t loaded;
    memset(&loaded, 0, sizeof(loaded));
    rc = load_motion_config("cam1", &loaded);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_TRUE(loaded.enabled);
    TEST_ASSERT_EQUAL_INT(10, loaded.pre_buffer_seconds);
    TEST_ASSERT_EQUAL_INT(20, loaded.post_buffer_seconds);
    TEST_ASSERT_EQUAL_INT(300, loaded.max_file_duration);
    TEST_ASSERT_EQUAL_STRING("h264", loaded.codec);
    TEST_ASSERT_EQUAL_STRING("medium", loaded.quality);
    TEST_ASSERT_EQUAL_INT(7, loaded.retention_days);
}

void test_load_missing_stream_fails(void) {
    motion_recording_config_t cfg;
    int rc = load_motion_config("nonexistent", &cfg);
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

/* ================================================================
 * update_motion_config
 * ================================================================ */

void test_update_overwrites_fields(void) {
    ensure_stream("cam2");
    motion_recording_config_t cfg = make_config(true);
    save_motion_config("cam2", &cfg);

    cfg.enabled = false;
    cfg.retention_days = 14;
    int rc = update_motion_config("cam2", &cfg);
    TEST_ASSERT_EQUAL_INT(0, rc);

    motion_recording_config_t loaded;
    load_motion_config("cam2", &loaded);
    TEST_ASSERT_FALSE(loaded.enabled);
    TEST_ASSERT_EQUAL_INT(14, loaded.retention_days);
}

/* ================================================================
 * delete_motion_config
 * ================================================================ */

void test_delete_removes_config(void) {
    ensure_stream("cam3");
    motion_recording_config_t cfg = make_config(true);
    save_motion_config("cam3", &cfg);

    int rc = delete_motion_config("cam3");
    TEST_ASSERT_EQUAL_INT(0, rc);

    motion_recording_config_t loaded;
    rc = load_motion_config("cam3", &loaded);
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

/* ================================================================
 * load_all_motion_configs
 * ================================================================ */

void test_load_all_returns_saved_configs(void) {
    ensure_stream("streamA");
    ensure_stream("streamB");
    motion_recording_config_t a = make_config(true);
    motion_recording_config_t b = make_config(false);
    save_motion_config("streamA", &a);
    save_motion_config("streamB", &b);

    motion_recording_config_t out[8];
    char names[8][256];
    int count = load_all_motion_configs(out, names, 8);
    TEST_ASSERT_EQUAL_INT(2, count);
}

/* ================================================================
 * is_motion_recording_enabled_in_db
 * ================================================================ */

void test_is_enabled_returns_correct_value(void) {
    ensure_stream("on_cam");
    ensure_stream("off_cam");
    motion_recording_config_t on  = make_config(true);
    motion_recording_config_t off = make_config(false);
    save_motion_config("on_cam",  &on);
    save_motion_config("off_cam", &off);

    TEST_ASSERT_EQUAL_INT(1, is_motion_recording_enabled_in_db("on_cam"));
    TEST_ASSERT_EQUAL_INT(0, is_motion_recording_enabled_in_db("off_cam"));
    /* Implementation returns 0 (disabled/not-found) for unknown streams, not -1 */
    TEST_ASSERT_EQUAL_INT(0, is_motion_recording_enabled_in_db("unknown_cam"));
}

/* ================================================================
 * add_motion_recording / get_motion_recording_db_stats
 * ================================================================ */

void test_add_recording_and_stats(void) {
    ensure_stream("cam_stats");
    time_t now = time(NULL);
    uint64_t id = add_motion_recording("cam_stats", "/rec/motion1.mp4",
                                       now, 1920, 1080, 30, "h264");
    TEST_ASSERT_NOT_EQUAL(0, id);

    /* stats query counts only is_complete=1 rows; mark complete first */
    mark_motion_recording_complete("/rec/motion1.mp4", now + 10, 512);

    uint64_t total_recs = 0, total_bytes = 0;
    time_t oldest = 0, newest = 0;
    int rc = get_motion_recording_db_stats("cam_stats",
                                           &total_recs, &total_bytes,
                                           &oldest, &newest);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_UINT(1, (unsigned int)total_recs);
}

/* ================================================================
 * mark_motion_recording_complete
 * ================================================================ */

void test_mark_complete_updates_size(void) {
    ensure_stream("cam_mc");
    time_t now = time(NULL);
    add_motion_recording("cam_mc", "/rec/mc.mp4", now, 1280, 720, 25, "h264");

    int rc = mark_motion_recording_complete("/rec/mc.mp4", now + 60, 1024 * 1024);
    TEST_ASSERT_EQUAL_INT(0, rc);
}

/* ================================================================
 * get_motion_recordings_disk_usage
 * ================================================================ */

void test_disk_usage_sums_sizes(void) {
    ensure_stream("cam_du");
    time_t now = time(NULL);
    add_motion_recording("cam_du", "/rec/du1.mp4", now, 1920, 1080, 30, "h264");
    mark_motion_recording_complete("/rec/du1.mp4", now + 60, 2048);

    add_motion_recording("cam_du", "/rec/du2.mp4", now + 1, 1920, 1080, 30, "h264");
    mark_motion_recording_complete("/rec/du2.mp4", now + 120, 4096);

    int64_t usage = get_motion_recordings_disk_usage("cam_du");
    TEST_ASSERT_EQUAL_INT(6144, (int)usage);
}

/* ================================================================
 * cleanup_old_motion_recordings
 * ================================================================ */

void test_cleanup_removes_old_recordings(void) {
    ensure_stream("cam_clean");
    /* Create a recording with a start_time far in the past */
    time_t old_time = time(NULL) - (40 * 24 * 3600); /* 40 days ago */
    add_motion_recording("cam_clean", "/rec/old.mp4", old_time, 1920, 1080, 30, "h264");

    int deleted = cleanup_old_motion_recordings("cam_clean", 30);
    TEST_ASSERT_GREATER_OR_EQUAL(1, deleted);
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
    RUN_TEST(test_save_and_load_round_trip);
    RUN_TEST(test_load_missing_stream_fails);
    RUN_TEST(test_update_overwrites_fields);
    RUN_TEST(test_delete_removes_config);
    RUN_TEST(test_load_all_returns_saved_configs);
    RUN_TEST(test_is_enabled_returns_correct_value);
    RUN_TEST(test_add_recording_and_stats);
    RUN_TEST(test_mark_complete_updates_size);
    RUN_TEST(test_disk_usage_sums_sizes);
    RUN_TEST(test_cleanup_removes_old_recordings);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}

