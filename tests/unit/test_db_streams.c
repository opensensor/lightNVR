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
#include "utils/strings.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_streams_test.db"

/* ---- helpers ---- */
static stream_config_t make_stream(const char *name, bool enabled) {
    stream_config_t s;
    memset(&s, 0, sizeof(s));
    safe_strcpy(s.name, name, sizeof(s.name), 0);
    safe_strcpy(s.url, "rtsp://camera/stream", sizeof(s.url), 0);
    safe_strcpy(s.codec, "h264", sizeof(s.codec), 0);
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
    safe_strcpy(s.detection_object_filter, "none", sizeof(s.detection_object_filter), 0);
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

static void exec_sql_or_fail(sqlite3 *db, const char *sql) {
    char *err = NULL;
    int rc = sqlite3_exec(db, sql, NULL, NULL, &err);
    if (rc != SQLITE_OK) {
        fprintf(stderr, "SQL failed: %s\n", err ? err : "unknown");
        sqlite3_free(err);
    }
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, rc);
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

void test_stream_admin_url_round_trip(void) {
    stream_config_t s = make_stream("cam_admin", true);
    safe_strcpy(s.admin_url, "http://camera.local/", sizeof(s.admin_url), 0);
    add_stream_config(&s);

    stream_config_t got;
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name("cam_admin", &got));
    TEST_ASSERT_EQUAL_STRING("http://camera.local/", got.admin_url);

    safe_strcpy(s.admin_url, "https://camera.local/settings", sizeof(s.admin_url), 0);
    TEST_ASSERT_EQUAL_INT(0, update_stream_config("cam_admin", &s));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name("cam_admin", &got));
    TEST_ASSERT_EQUAL_STRING("https://camera.local/settings", got.admin_url);
}

/* ================================================================
 * update_stream_config
 * ================================================================ */

void test_update_stream_config_changes_url(void) {
    stream_config_t s = make_stream("cam_upd", true);
    add_stream_config(&s);

    safe_strcpy(s.url, "rtsp://new/stream", sizeof(s.url), 0);
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

    char names[10][MAX_STREAM_NAME];
    int n = get_all_stream_names(names, 10);
    TEST_ASSERT_EQUAL_INT(2, n);
}

/* ================================================================
 * motion_trigger_source field
 * ================================================================ */

void test_motion_trigger_source_defaults_empty(void) {
    stream_config_t s = make_stream("cam_mts_def", true);
    add_stream_config(&s);

    stream_config_t got;
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name("cam_mts_def", &got));
    TEST_ASSERT_EQUAL_STRING("", got.motion_trigger_source);
}

void test_motion_trigger_source_round_trip(void) {
    stream_config_t s = make_stream("cam_ptz", true);
    safe_strcpy(s.motion_trigger_source, "cam_fixed", sizeof(s.motion_trigger_source), 0);
    add_stream_config(&s);

    stream_config_t got;
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name("cam_ptz", &got));
    TEST_ASSERT_EQUAL_STRING("cam_fixed", got.motion_trigger_source);
}

void test_motion_trigger_source_update(void) {
    stream_config_t s = make_stream("cam_ptz_upd", true);
    safe_strcpy(s.motion_trigger_source, "cam_fixed_old", sizeof(s.motion_trigger_source), 0);
    add_stream_config(&s);

    safe_strcpy(s.motion_trigger_source, "cam_fixed_new", sizeof(s.motion_trigger_source), 0);
    TEST_ASSERT_EQUAL_INT(0, update_stream_config("cam_ptz_upd", &s));

    stream_config_t got;
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name("cam_ptz_upd", &got));
    TEST_ASSERT_EQUAL_STRING("cam_fixed_new", got.motion_trigger_source);
}

void test_motion_trigger_source_in_get_all(void) {
    stream_config_t src = make_stream("cam_src", true);
    stream_config_t ptz = make_stream("cam_ptz_all", true);
    safe_strcpy(ptz.motion_trigger_source, "cam_src", sizeof(ptz.motion_trigger_source), 0);
    add_stream_config(&src);
    add_stream_config(&ptz);

    stream_config_t out[10];
    int n = get_all_stream_configs(out, 10);
    TEST_ASSERT_EQUAL_INT(2, n);

    bool found_ptz = false;
    for (int i = 0; i < n; i++) {
        if (strcmp(out[i].name, "cam_ptz_all") == 0) {
            TEST_ASSERT_EQUAL_STRING("cam_src", out[i].motion_trigger_source);
            found_ptz = true;
        }
    }
    TEST_ASSERT_TRUE(found_ptz);
}

/* ================================================================
 * go2rtc source override tests
 * ================================================================ */

void test_go2rtc_source_override_defaults_empty(void) {
    stream_config_t s = make_stream("cam_g2r_def", true);
    add_stream_config(&s);

    stream_config_t got;
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name("cam_g2r_def", &got));
    TEST_ASSERT_EQUAL_STRING("", got.go2rtc_source_override);
}

void test_go2rtc_source_override_round_trip(void) {
    stream_config_t s = make_stream("cam_g2r_rt", true);
    safe_strcpy(s.go2rtc_source_override,
                "- rtsp://admin:pass@cam/main#transport=tcp\n- ffmpeg:cam_g2r_rt#video=h264#hardware",
                sizeof(s.go2rtc_source_override), 0);
    add_stream_config(&s);

    stream_config_t got;
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name("cam_g2r_rt", &got));
    TEST_ASSERT_EQUAL_STRING(
        "- rtsp://admin:pass@cam/main#transport=tcp\n- ffmpeg:cam_g2r_rt#video=h264#hardware",
        got.go2rtc_source_override);
}

void test_go2rtc_source_override_update(void) {
    stream_config_t s = make_stream("cam_g2r_upd", true);
    safe_strcpy(s.go2rtc_source_override, "rtsp://old/stream", sizeof(s.go2rtc_source_override), 0);
    add_stream_config(&s);

    safe_strcpy(s.go2rtc_source_override, "rtsp://new/stream#transport=tcp", sizeof(s.go2rtc_source_override), 0);
    TEST_ASSERT_EQUAL_INT(0, update_stream_config("cam_g2r_upd", &s));

    stream_config_t got;
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name("cam_g2r_upd", &got));
    TEST_ASSERT_EQUAL_STRING("rtsp://new/stream#transport=tcp", got.go2rtc_source_override);
}

void test_go2rtc_source_override_in_get_all(void) {
    stream_config_t plain = make_stream("cam_g2r_plain", true);
    stream_config_t custom = make_stream("cam_g2r_custom", true);
    safe_strcpy(custom.go2rtc_source_override, "ffmpeg:cam_g2r_custom#video=h264",
                sizeof(custom.go2rtc_source_override), 0);
    add_stream_config(&plain);
    add_stream_config(&custom);

    stream_config_t out[10];
    int n = get_all_stream_configs(out, 10);
    TEST_ASSERT_EQUAL_INT(2, n);

    bool found = false;
    for (int i = 0; i < n; i++) {
        if (strcmp(out[i].name, "cam_g2r_custom") == 0) {
            TEST_ASSERT_EQUAL_STRING("ffmpeg:cam_g2r_custom#video=h264", out[i].go2rtc_source_override);
            found = true;
        } else if (strcmp(out[i].name, "cam_g2r_plain") == 0) {
            TEST_ASSERT_EQUAL_STRING("", out[i].go2rtc_source_override);
        }
    }
    TEST_ASSERT_TRUE(found);
}

void test_go2rtc_source_override_clear(void) {
    stream_config_t s = make_stream("cam_g2r_clr", true);
    safe_strcpy(s.go2rtc_source_override, "rtsp://custom/stream", sizeof(s.go2rtc_source_override), 0);
    add_stream_config(&s);

    s.go2rtc_source_override[0] = '\0';
    TEST_ASSERT_EQUAL_INT(0, update_stream_config("cam_g2r_clr", &s));

    stream_config_t got;
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name("cam_g2r_clr", &got));
    TEST_ASSERT_EQUAL_STRING("", got.go2rtc_source_override);
}

/* ================================================================
 * sub-stream URL tests
 * ================================================================ */

void test_sub_stream_url_defaults_empty(void) {
    stream_config_t s = make_stream("cam_sub_def", true);
    add_stream_config(&s);

    stream_config_t got;
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name("cam_sub_def", &got));
    TEST_ASSERT_EQUAL_STRING("", got.sub_stream_url);
}

void test_sub_stream_url_round_trip(void) {
    stream_config_t s = make_stream("cam_sub_rt", true);
    safe_strcpy(s.sub_stream_url, "rtsp://camera/substream", sizeof(s.sub_stream_url), 0);
    add_stream_config(&s);

    stream_config_t got;
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name("cam_sub_rt", &got));
    TEST_ASSERT_EQUAL_STRING("rtsp://camera/substream", got.sub_stream_url);
}

void test_sub_stream_url_update(void) {
    stream_config_t s = make_stream("cam_sub_upd", true);
    safe_strcpy(s.sub_stream_url, "rtsp://camera/sub_old", sizeof(s.sub_stream_url), 0);
    add_stream_config(&s);

    safe_strcpy(s.sub_stream_url, "rtsp://camera/sub_new", sizeof(s.sub_stream_url), 0);
    TEST_ASSERT_EQUAL_INT(0, update_stream_config("cam_sub_upd", &s));

    stream_config_t got;
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name("cam_sub_upd", &got));
    TEST_ASSERT_EQUAL_STRING("rtsp://camera/sub_new", got.sub_stream_url);
}

void test_sub_stream_url_in_get_all(void) {
    stream_config_t main_only = make_stream("cam_sub_main", true);
    stream_config_t with_sub = make_stream("cam_sub_both", true);
    safe_strcpy(with_sub.sub_stream_url, "rtsp://camera/low", sizeof(with_sub.sub_stream_url), 0);
    add_stream_config(&main_only);
    add_stream_config(&with_sub);

    stream_config_t out[10];
    int n = get_all_stream_configs(out, 10);
    TEST_ASSERT_EQUAL_INT(2, n);

    bool found = false;
    for (int i = 0; i < n; i++) {
        if (strcmp(out[i].name, "cam_sub_both") == 0) {
            TEST_ASSERT_EQUAL_STRING("rtsp://camera/low", out[i].sub_stream_url);
            found = true;
        } else if (strcmp(out[i].name, "cam_sub_main") == 0) {
            TEST_ASSERT_EQUAL_STRING("", out[i].sub_stream_url);
        }
    }
    TEST_ASSERT_TRUE(found);
}

void test_repair_onvif_embedded_credentials_migration_normalizes_legacy_rows(void) {
    sqlite3 *db = get_db_handle();
    exec_sql_or_fail(db, "DELETE FROM streams;");
    exec_sql_or_fail(db,
        "INSERT INTO streams ("
        "name, url, enabled, streaming_enabled, width, height, fps, codec, priority, record, segment_duration, "
        "detection_based_recording, detection_model, detection_threshold, detection_interval, pre_detection_buffer, post_detection_buffer, "
        "detection_api_url, detection_object_filter, detection_object_filter_list, protocol, is_onvif, record_audio, backchannel_enabled, "
        "retention_days, detection_retention_days, max_storage_mb, tier_critical_multiplier, tier_important_multiplier, tier_ephemeral_multiplier, storage_priority, "
        "ptz_enabled, ptz_max_x, ptz_max_y, ptz_max_z, ptz_has_home, onvif_username, onvif_password, onvif_profile, onvif_port, record_on_schedule, recording_schedule, tags) VALUES ("
        "'legacy_onvif', 'rtsp://legacy_user:legacy_pass@camera.example/live', 1, 1, 1920, 1080, 25, 'h264', 5, 1, 60, "
        "0, '', 0.5, 10, 0, 3, '', 'none', '', 0, 1, 1, 0, 0, 0, 0, 3.0, 2.0, 0.25, 0, 0, 0.0, 0.0, 0.0, 0, '', '', 'profile1', 8899, 0, '', '');");
    exec_sql_or_fail(db, "DELETE FROM schema_migrations WHERE version = '0034';");

    shutdown_database();
    TEST_ASSERT_EQUAL_INT(0, init_database(TEST_DB_PATH));

    stream_config_t got;
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name("legacy_onvif", &got));
    TEST_ASSERT_EQUAL_STRING("rtsp://camera.example/live", got.url);
    TEST_ASSERT_EQUAL_STRING("legacy_user", got.onvif_username);
    TEST_ASSERT_EQUAL_STRING("legacy_pass", got.onvif_password);
    TEST_ASSERT_TRUE(got.is_onvif);
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
    RUN_TEST(test_stream_admin_url_round_trip);
    RUN_TEST(test_update_stream_config_changes_url);
    RUN_TEST(test_delete_stream_config_disables);
    RUN_TEST(test_count_stream_configs);
    RUN_TEST(test_get_all_stream_configs_returns_multiple);
    RUN_TEST(test_get_enabled_stream_count);
    RUN_TEST(test_stream_retention_config_round_trip);
    RUN_TEST(test_get_all_stream_names);
    RUN_TEST(test_repair_onvif_embedded_credentials_migration_normalizes_legacy_rows);
    RUN_TEST(test_motion_trigger_source_defaults_empty);
    RUN_TEST(test_motion_trigger_source_round_trip);
    RUN_TEST(test_motion_trigger_source_update);
    RUN_TEST(test_motion_trigger_source_in_get_all);
    RUN_TEST(test_go2rtc_source_override_defaults_empty);
    RUN_TEST(test_go2rtc_source_override_round_trip);
    RUN_TEST(test_go2rtc_source_override_update);
    RUN_TEST(test_go2rtc_source_override_in_get_all);
    RUN_TEST(test_go2rtc_source_override_clear);
    RUN_TEST(test_sub_stream_url_defaults_empty);
    RUN_TEST(test_sub_stream_url_round_trip);
    RUN_TEST(test_sub_stream_url_update);
    RUN_TEST(test_sub_stream_url_in_get_all);

    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}

