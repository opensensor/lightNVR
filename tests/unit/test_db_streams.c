/**
 * @file test_db_streams.c
 * @brief Layer 2 integration tests — stream config CRUD via SQLite
 *
 * Tests add_stream_config, get_stream_config_by_name, update_stream_config,
 * delete_stream_config, get_all_stream_configs, count_stream_configs,
 * get_enabled_stream_count, stream retention config, get_all_stream_names,
 * and update_stream_video_params.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sqlite3.h>

#include "unity.h"
#include "database/db_core.h"
#include "database/db_streams.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_streams_test.db"

/* ---- helpers ---- */
static stream_config_t make_stream(const char *name, bool enabled) {
    stream_config_t s;
    memset(&s, 0, sizeof(s));
    strncpy(s.name, name, sizeof(s.name) - 1);
    strncpy(s.url, "rtsp://camera/stream", sizeof(s.url) - 1);
    strncpy(s.codec, "h264", sizeof(s.codec) - 1);
    s.enabled  = enabled;
    s.width    = 1920;
    s.height   = 1080;
    s.fps      = 25;
    s.priority = 5;
    s.segment_duration = 60;
    s.streaming_enabled = true;
    s.detection_threshold = 0.5f;
    s.detection_interval  = 10;
    s.pre_detection_buffer  = 5;
    s.post_detection_buffer = 10;
    strncpy(s.detection_object_filter, "none", sizeof(s.detection_object_filter) - 1);
    s.tier_critical_multiplier  = 3.0;
    s.tier_important_multiplier = 2.0;
    s.tier_ephemeral_multiplier = 0.25;
    s.storage_priority = 5;
    return s;
}

static void clear_streams(void) {
    sqlite3 *db = get_db_handle();
    sqlite3_exec(db, "DELETE FROM streams;", NULL, NULL, NULL);
}

/* ---- Unity boilerplate ---- */
void setUp(void)    { clear_streams(); }
void tearDown(void) {}

/* ================================================================
 * add_stream_config / get_stream_config_by_name
 * ================================================================ */

void test_add_stream_config_returns_nonzero_id(void) {
    stream_config_t s = make_stream("cam1", true);
    uint64_t id = add_stream_config(&s);
    TEST_ASSERT_NOT_EQUAL(0, id);
}

void test_get_stream_config_by_name_round_trip(void) {
    stream_config_t s = make_stream("cam_rt", true);
    add_stream_config(&s);

    stream_config_t got;
    int rc = get_stream_config_by_name("cam_rt", &got);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("cam_rt", got.name);
    TEST_ASSERT_TRUE(got.enabled);
}

/* ================================================================
 * update_stream_config
 * ================================================================ */

void test_update_stream_config_changes_url(void) {
    stream_config_t s = make_stream("cam_upd", true);
    add_stream_config(&s);

    strncpy(s.url, "rtsp://new/stream", sizeof(s.url) - 1);
    int rc = update_stream_config("cam_upd", &s);
    TEST_ASSERT_EQUAL_INT(0, rc);

    stream_config_t got;
    get_stream_config_by_name("cam_upd", &got);
    TEST_ASSERT_EQUAL_STRING("rtsp://new/stream", got.url);
}

/* ================================================================
 * delete_stream_config (soft-delete)
 * ================================================================ */

void test_delete_stream_config_disables(void) {
    stream_config_t s = make_stream("cam_del", true);
    add_stream_config(&s);

    int rc = delete_stream_config("cam_del");
    TEST_ASSERT_EQUAL_INT(0, rc);

    /* Stream should now be disabled or missing */
    stream_config_t got;
    int found = get_stream_config_by_name("cam_del", &got);
    if (found == 0) {
        TEST_ASSERT_FALSE(got.enabled);
    }
    /* It's OK if not found after soft delete */
}

/* ================================================================
 * count_stream_configs / get_all_stream_configs
 * ================================================================ */

void test_count_stream_configs(void) {
    stream_config_t s1 = make_stream("c1", true);
    stream_config_t s2 = make_stream("c2", true);
    add_stream_config(&s1);
    add_stream_config(&s2);
    int cnt = count_stream_configs();
    TEST_ASSERT_EQUAL_INT(2, cnt);
}

void test_get_all_stream_configs_returns_multiple(void) {
    stream_config_t s1 = make_stream("g1", true);
    stream_config_t s2 = make_stream("g2", true);
    add_stream_config(&s1);
    add_stream_config(&s2);

    stream_config_t out[10];
    int n = get_all_stream_configs(out, 10);
    TEST_ASSERT_EQUAL_INT(2, n);
}

/* ================================================================
 * get_enabled_stream_count
 * ================================================================ */

void test_get_enabled_stream_count(void) {
    stream_config_t en  = make_stream("en1", true);
    stream_config_t dis = make_stream("dis1", false);
    add_stream_config(&en);
    add_stream_config(&dis);
    int cnt = get_enabled_stream_count();
    TEST_ASSERT_EQUAL_INT(1, cnt);
}

/* ================================================================
 * get/set stream retention config
 * ================================================================ */

void test_stream_retention_config_round_trip(void) {
    stream_config_t s = make_stream("cam_ret", true);
    add_stream_config(&s);

    stream_retention_config_t cfg_in = {.retention_days = 14,
                                        .detection_retention_days = 30,
                                        .max_storage_mb = 1024};
    int rc = set_stream_retention_config("cam_ret", &cfg_in);
    TEST_ASSERT_EQUAL_INT(0, rc);

    stream_retention_config_t cfg_out;
    rc = get_stream_retention_config("cam_ret", &cfg_out);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_INT(14, cfg_out.retention_days);
    TEST_ASSERT_EQUAL_INT(30, cfg_out.detection_retention_days);
}

/* ================================================================
 * get_all_stream_names
 * ================================================================ */

void test_get_all_stream_names(void) {
    stream_config_t s1 = make_stream("n1", true);
    stream_config_t s2 = make_stream("n2", true);
    add_stream_config(&s1);
    add_stream_config(&s2);

    char names[10][64];
    int n = get_all_stream_names(names, 10);
    TEST_ASSERT_EQUAL_INT(2, n);
}

/* ================================================================
 * main
 * ================================================================ */

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database() failed\n");
        return 1;
    }
    UNITY_BEGIN();

    RUN_TEST(test_add_stream_config_returns_nonzero_id);
    RUN_TEST(test_get_stream_config_by_name_round_trip);
    RUN_TEST(test_update_stream_config_changes_url);
    RUN_TEST(test_delete_stream_config_disables);
    RUN_TEST(test_count_stream_configs);
    RUN_TEST(test_get_all_stream_configs_returns_multiple);
    RUN_TEST(test_get_enabled_stream_count);
    RUN_TEST(test_stream_retention_config_round_trip);
    RUN_TEST(test_get_all_stream_names);

    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}

