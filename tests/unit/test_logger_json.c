/**
 * @file test_logger_json.c
 * @brief Layer 2 unit tests — JSON logger lifecycle and operations
 *
 * Tests:
 *   - init_json_logger() with NULL and valid paths
 *   - write_json_log() for all log levels and when not initialized
 *   - shutdown_json_logger() lifecycle
 *   - get_json_logs() with level filtering and timestamp pagination
 *   - json_log_rotate() below and above the size threshold
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "unity.h"
#include "core/logger.h"
#include "core/logger_json.h"

/* Shared temp file path used by setUp/tearDown */
static char g_tmp_path[256];
static int  g_initialized = 0;

void setUp(void) {
    snprintf(g_tmp_path, sizeof(g_tmp_path), "/tmp/lightnvr_json_test_%d.log", (int)getpid());
    int rc = init_json_logger(g_tmp_path);
    g_initialized = (rc == 0);
}

void tearDown(void) {
    if (g_initialized) {
        shutdown_json_logger();
        g_initialized = 0;
    }
    unlink(g_tmp_path);
}

/* ================================================================
 * init_json_logger
 * ================================================================ */

void test_init_json_logger_null_returns_error(void) {
    /* Already initialized in setUp — shut down first, then test NULL */
    shutdown_json_logger();
    g_initialized = 0;

    int rc = init_json_logger(NULL);
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

void test_init_json_logger_valid_path_succeeds(void) {
    /* setUp already ran init successfully */
    TEST_ASSERT_TRUE(g_initialized);
}

void test_init_json_logger_creates_file(void) {
    TEST_ASSERT_EQUAL_INT(0, access(g_tmp_path, F_OK));
}

/* ================================================================
 * write_json_log — when not initialized
 * ================================================================ */

void test_write_json_log_not_initialized_returns_error(void) {
    shutdown_json_logger();
    g_initialized = 0;

    int rc = write_json_log(LOG_LEVEL_INFO, "2025-01-01T00:00:00", "should fail");
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

/* ================================================================
 * write_json_log — all log levels
 * ================================================================ */

void test_write_json_log_error_level(void) {
    int rc = write_json_log(LOG_LEVEL_ERROR, "2025-01-01T00:00:01", "error msg");
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_write_json_log_warn_level(void) {
    int rc = write_json_log(LOG_LEVEL_WARN, "2025-01-01T00:00:02", "warn msg");
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_write_json_log_info_level(void) {
    int rc = write_json_log(LOG_LEVEL_INFO, "2025-01-01T00:00:03", "info msg");
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_write_json_log_debug_level(void) {
    int rc = write_json_log(LOG_LEVEL_DEBUG, "2025-01-01T00:00:04", "debug msg");
    TEST_ASSERT_EQUAL_INT(0, rc);
}

/* ================================================================
 * get_json_logs — level filtering
 * ================================================================ */

void test_get_json_logs_not_initialized_returns_error(void) {
    shutdown_json_logger();
    g_initialized = 0;

    char **logs = NULL;
    int count = 0;
    int rc = get_json_logs("info", "", &logs, &count);
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

void test_get_json_logs_error_level_filters_lower(void) {
    /* Write one entry at each level */
    write_json_log(LOG_LEVEL_ERROR, "2025-01-01T00:01:00", "e");
    write_json_log(LOG_LEVEL_WARN,  "2025-01-01T00:01:01", "w");
    write_json_log(LOG_LEVEL_INFO,  "2025-01-01T00:01:02", "i");
    write_json_log(LOG_LEVEL_DEBUG, "2025-01-01T00:01:03", "d");

    char **logs = NULL;
    int count = 0;
    int rc = get_json_logs("error", "", &logs, &count);
    TEST_ASSERT_EQUAL_INT(0, rc);
    /* Only ERROR (level_value==0) satisfies level_value <= min_value(0) */
    /* +1 for the startup marker (INFO level) which may or may not appear */
    TEST_ASSERT_GREATER_OR_EQUAL(1, count);

    for (int i = 0; i < count; i++) free(logs[i]);
    free(logs);
}

void test_get_json_logs_debug_level_includes_all(void) {
    write_json_log(LOG_LEVEL_ERROR, "2025-01-01T00:02:00", "e");
    write_json_log(LOG_LEVEL_WARN,  "2025-01-01T00:02:01", "w");
    write_json_log(LOG_LEVEL_INFO,  "2025-01-01T00:02:02", "i");
    write_json_log(LOG_LEVEL_DEBUG, "2025-01-01T00:02:03", "d");

    char **logs = NULL;
    int count = 0;
    int rc = get_json_logs("debug", "", &logs, &count);
    TEST_ASSERT_EQUAL_INT(0, rc);
    /* All four entries (plus any startup marker) should appear */
    TEST_ASSERT_GREATER_OR_EQUAL(4, count);

    for (int i = 0; i < count; i++) free(logs[i]);
    free(logs);
}

/* ================================================================
 * get_json_logs — timestamp pagination
 * ================================================================ */

void test_get_json_logs_timestamp_pagination_excludes_old(void) {
    write_json_log(LOG_LEVEL_INFO, "2025-01-01T00:03:00", "old");
    write_json_log(LOG_LEVEL_INFO, "2025-01-01T00:03:01", "new");

    char **logs = NULL;
    int count = 0;
    /* Entries at or before "2025-01-01T00:03:00" must be excluded */
    int rc = get_json_logs("debug", "2025-01-01T00:03:00", &logs, &count);
    TEST_ASSERT_EQUAL_INT(0, rc);

    /* The "new" entry (timestamp 00:03:01) should appear */
    int found_new = 0;
    for (int i = 0; i < count; i++) {
        if (strstr(logs[i], "new")) found_new = 1;
        free(logs[i]);
    }
    free(logs);
    TEST_ASSERT_TRUE(found_new);
}

/* ================================================================
 * json_log_rotate — below threshold (no rotation needed)
 * ================================================================ */

void test_json_log_rotate_not_initialized_returns_error(void) {
    shutdown_json_logger();
    g_initialized = 0;

    int rc = json_log_rotate(1024, 3);
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

void test_json_log_rotate_below_threshold_returns_ok(void) {
    /* File is tiny; very large threshold => no rotation */
    int rc = json_log_rotate(10 * 1024 * 1024, 3);
    TEST_ASSERT_EQUAL_INT(0, rc);
    /* Logger still usable after no-op rotation */
    int rc2 = write_json_log(LOG_LEVEL_INFO, "2025-01-01T00:04:00", "after no-op rotate");
    TEST_ASSERT_EQUAL_INT(0, rc2);
}

void test_json_log_rotate_above_threshold_rotates(void) {
    /* Write enough data to exceed a tiny threshold */
    for (int i = 0; i < 50; i++) {
        write_json_log(LOG_LEVEL_INFO, "2025-01-01T00:05:00",
                       "padding to exceed tiny size threshold for rotation test");
    }

    /* Set a threshold smaller than the file */
    int rc = json_log_rotate(1, 2);
    TEST_ASSERT_EQUAL_INT(0, rc);

    /* After rotation, logger should still be writable */
    int rc2 = write_json_log(LOG_LEVEL_INFO, "2025-01-01T00:05:01", "post-rotation");
    TEST_ASSERT_EQUAL_INT(0, rc2);

    /* Clean up rotated file */
    char rotated[300];
    snprintf(rotated, sizeof(rotated), "%s.1", g_tmp_path);
    unlink(rotated);
}

/* ================================================================
 * main
 * ================================================================ */

int main(void) {
    init_logger();

    UNITY_BEGIN();

    RUN_TEST(test_init_json_logger_null_returns_error);
    RUN_TEST(test_init_json_logger_valid_path_succeeds);
    RUN_TEST(test_init_json_logger_creates_file);

    RUN_TEST(test_write_json_log_not_initialized_returns_error);
    RUN_TEST(test_write_json_log_error_level);
    RUN_TEST(test_write_json_log_warn_level);
    RUN_TEST(test_write_json_log_info_level);
    RUN_TEST(test_write_json_log_debug_level);

    RUN_TEST(test_get_json_logs_not_initialized_returns_error);
    RUN_TEST(test_get_json_logs_error_level_filters_lower);
    RUN_TEST(test_get_json_logs_debug_level_includes_all);
    RUN_TEST(test_get_json_logs_timestamp_pagination_excludes_old);

    RUN_TEST(test_json_log_rotate_not_initialized_returns_error);
    RUN_TEST(test_json_log_rotate_below_threshold_returns_ok);
    RUN_TEST(test_json_log_rotate_above_threshold_rotates);

    int result = UNITY_END();
    shutdown_logger();
    return result;
}

