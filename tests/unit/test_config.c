/**
 * @file test_config.c
 * @brief Layer 2 unit tests — config loading and validation
 *
 * Tests load_default_config() for sane defaults and validate_config()
 * for rejection of invalid values (bad port, empty paths, bad buffer).
 * Links against lightnvr_lib so the logger is available for error paths.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>

#include "unity.h"
#include "core/config.h"
#include "core/logger.h"

/* ---- Unity boilerplate ---- */
void setUp(void)    {}
void tearDown(void) {}

/* ================================================================
 * load_default_config
 * ================================================================ */

void test_default_config_web_port(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(8080, cfg.web_port);
}

void test_default_config_log_level(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(LOG_LEVEL_INFO, cfg.log_level);
}

void test_default_config_retention_days(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(30, cfg.retention_days);
}

void test_default_config_buffer_size(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_GREATER_THAN_INT(0, cfg.buffer_size);
}

void test_default_config_storage_path_nonempty(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_GREATER_THAN(0, (int)strlen(cfg.storage_path));
}

void test_default_config_db_path_nonempty(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_GREATER_THAN(0, (int)strlen(cfg.db_path));
}

void test_default_config_models_path_nonempty(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_GREATER_THAN(0, (int)strlen(cfg.models_path));
}

void test_default_config_null_safe(void) {
    /* Should not crash when passed NULL */
    load_default_config(NULL);
    TEST_PASS();
}

/* ================================================================
 * validate_config
 * ================================================================ */

void test_validate_config_valid_defaults(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
}

void test_validate_config_null(void) {
    TEST_ASSERT_NOT_EQUAL(0, validate_config(NULL));
}

void test_validate_config_empty_storage_path(void) {
    config_t cfg;
    load_default_config(&cfg);
    cfg.storage_path[0] = '\0';
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

void test_validate_config_empty_models_path(void) {
    config_t cfg;
    load_default_config(&cfg);
    cfg.models_path[0] = '\0';
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

void test_validate_config_empty_db_path(void) {
    config_t cfg;
    load_default_config(&cfg);
    cfg.db_path[0] = '\0';
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

void test_validate_config_empty_web_root(void) {
    config_t cfg;
    load_default_config(&cfg);
    cfg.web_root[0] = '\0';
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

void test_validate_config_port_zero(void) {
    config_t cfg;
    load_default_config(&cfg);
    cfg.web_port = 0;
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

void test_validate_config_port_too_high(void) {
    config_t cfg;
    load_default_config(&cfg);
    cfg.web_port = 99999;
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

void test_validate_config_port_max_valid(void) {
    config_t cfg;
    load_default_config(&cfg);
    cfg.web_port = 65535;
    TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
}

void test_validate_config_port_min_valid(void) {
    config_t cfg;
    load_default_config(&cfg);
    cfg.web_port = 1;
    TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
}

void test_validate_config_buffer_size_zero(void) {
    config_t cfg;
    load_default_config(&cfg);
    cfg.buffer_size = 0;
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

/* ================================================================
 * additional default field checks
 * ================================================================ */

void test_default_config_web_auth_enabled(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_TRUE(cfg.web_auth_enabled);
}

void test_default_config_username(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_STRING("admin", cfg.web_username);
}

void test_default_config_syslog_disabled(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_FALSE(cfg.syslog_enabled);
}

void test_default_config_go2rtc_enabled(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_TRUE(cfg.go2rtc_enabled);
}

void test_default_config_go2rtc_api_port(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(1984, cfg.go2rtc_api_port);
}

void test_default_config_go2rtc_webrtc_enabled(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_TRUE(cfg.go2rtc_webrtc_enabled);
}

void test_default_config_go2rtc_stun_enabled(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_TRUE(cfg.go2rtc_stun_enabled);
}

void test_default_config_turn_disabled(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_FALSE(cfg.turn_enabled);
}

void test_default_config_mqtt_disabled(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_FALSE(cfg.mqtt_enabled);
}

void test_default_config_mqtt_port(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(1883, cfg.mqtt_broker_port);
}

void test_default_config_mp4_segment_duration(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(900, cfg.mp4_segment_duration);
}

void test_default_config_stream_defaults(void) {
    config_t cfg;
    load_default_config(&cfg);
    /* All streams should default to streaming enabled, no detection */
    for (int i = 0; i < MAX_STREAMS; i++) {
        TEST_ASSERT_TRUE(cfg.streams[i].streaming_enabled);
        TEST_ASSERT_FALSE(cfg.streams[i].detection_based_recording);
    }
}

void test_default_config_auth_timeout(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(24, cfg.auth_timeout_hours);
}

void test_default_config_web_compression_enabled(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_TRUE(cfg.web_compression_enabled);
}

/* ================================================================
 * validate_config — swap_size zero with use_swap true
 * ================================================================ */

void test_validate_config_swap_size_zero_with_use_swap(void) {
    config_t cfg;
    load_default_config(&cfg);
    cfg.use_swap = true;
    cfg.swap_size = 0;
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

void test_validate_config_swap_disabled_size_zero_ok(void) {
    config_t cfg;
    load_default_config(&cfg);
    cfg.use_swap = false;
    cfg.swap_size = 0;
    /* Swap size check only applies when use_swap is true */
    TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
}

/* ================================================================
 * set_custom_config_path / get_custom_config_path
 * ================================================================ */

void test_custom_config_path_empty_not_stored(void) {
    /* set_custom_config_path ignores empty strings (does not overwrite). */
    /* At the start of the test binary, g_custom_config_path is zeroed.   */
    /* This test must run before any test that sets a non-empty path.     */
    set_custom_config_path("");
    /* getter returns NULL when nothing has been stored yet */
    const char *path = get_custom_config_path();
    TEST_ASSERT_NULL(path);
}

void test_custom_config_path_null_not_stored(void) {
    /* NULL is also silently ignored — must not crash and must not store. */
    set_custom_config_path(NULL);
    const char *path = get_custom_config_path();
    TEST_ASSERT_NULL(path);
}

void test_custom_config_path_roundtrip(void) {
    /* Run AFTER the empty/null tests so that g_custom_config_path is still
       empty when those tests run.  Here we confirm a valid path is stored. */
    set_custom_config_path("/tmp/test_lightnvr.ini");
    const char *path = get_custom_config_path();
    TEST_ASSERT_NOT_NULL(path);
    TEST_ASSERT_EQUAL_STRING("/tmp/test_lightnvr.ini", path);
}

/* ================================================================
 * get_loaded_config_path — initially NULL (no file loaded yet)
 * ================================================================ */

void test_get_loaded_config_path_initially(void) {
    /* Without calling load_config(), the loaded path should be NULL */
    const char *path = get_loaded_config_path();
    TEST_ASSERT_NULL(path);
}

/* ================================================================
 * main
 * ================================================================ */

int main(void) {
    init_logger();

    UNITY_BEGIN();

    RUN_TEST(test_default_config_web_port);
    RUN_TEST(test_default_config_log_level);
    RUN_TEST(test_default_config_retention_days);
    RUN_TEST(test_default_config_buffer_size);
    RUN_TEST(test_default_config_storage_path_nonempty);
    RUN_TEST(test_default_config_db_path_nonempty);
    RUN_TEST(test_default_config_models_path_nonempty);
    RUN_TEST(test_default_config_null_safe);

    RUN_TEST(test_validate_config_valid_defaults);
    RUN_TEST(test_validate_config_null);
    RUN_TEST(test_validate_config_empty_storage_path);
    RUN_TEST(test_validate_config_empty_models_path);
    RUN_TEST(test_validate_config_empty_db_path);
    RUN_TEST(test_validate_config_empty_web_root);
    RUN_TEST(test_validate_config_port_zero);
    RUN_TEST(test_validate_config_port_too_high);
    RUN_TEST(test_validate_config_port_max_valid);
    RUN_TEST(test_validate_config_port_min_valid);
    RUN_TEST(test_validate_config_buffer_size_zero);

    RUN_TEST(test_default_config_web_auth_enabled);
    RUN_TEST(test_default_config_username);
    RUN_TEST(test_default_config_syslog_disabled);
    RUN_TEST(test_default_config_go2rtc_enabled);
    RUN_TEST(test_default_config_go2rtc_api_port);
    RUN_TEST(test_default_config_go2rtc_webrtc_enabled);
    RUN_TEST(test_default_config_go2rtc_stun_enabled);
    RUN_TEST(test_default_config_turn_disabled);
    RUN_TEST(test_default_config_mqtt_disabled);
    RUN_TEST(test_default_config_mqtt_port);
    RUN_TEST(test_default_config_mp4_segment_duration);
    RUN_TEST(test_default_config_stream_defaults);
    RUN_TEST(test_default_config_auth_timeout);
    RUN_TEST(test_default_config_web_compression_enabled);

    RUN_TEST(test_validate_config_swap_size_zero_with_use_swap);
    RUN_TEST(test_validate_config_swap_disabled_size_zero_ok);

    RUN_TEST(test_custom_config_path_empty_not_stored);
    RUN_TEST(test_custom_config_path_null_not_stored);
    RUN_TEST(test_custom_config_path_roundtrip);
    RUN_TEST(test_get_loaded_config_path_initially);

    int result = UNITY_END();
    shutdown_logger();
    return result;
}

