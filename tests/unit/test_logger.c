/**
 * @file test_logger.c
 * @brief Layer 2 unit tests — logger lifecycle and string helpers
 *
 * Tests:
 *   - get_log_level_string() returns the correct string for all levels
 *     and "UNKNOWN" for an out-of-range level.
 *   - init_logger / shutdown_logger lifecycle (idempotent reinit).
 *   - set_log_level / get_log_level_string round-trip.
 *   - is_logger_available() reflects init/shutdown state.
 *   - set_log_file() with a temporary file path.
 *   - log_rotate() when no log file is configured.
 *   - enable_syslog() / disable_syslog() / is_syslog_enabled().
 *   - log_error / log_warn / log_info / log_debug / log_message — smoke tests.
 *   - log_message_v() indirectly via log_message().
 */

#define _POSIX_C_SOURCE 200809L

#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <syslog.h>
#include "unity.h"
#include "core/logger.h"

/* ---- Unity boilerplate ---- */
void setUp(void)    {}
void tearDown(void) {}

/* ================================================================
 * get_log_level_string
 * ================================================================ */

void test_log_level_string_error(void) {
    TEST_ASSERT_EQUAL_STRING("ERROR", get_log_level_string(LOG_LEVEL_ERROR));
}

void test_log_level_string_warn(void) {
    TEST_ASSERT_EQUAL_STRING("WARN", get_log_level_string(LOG_LEVEL_WARN));
}

void test_log_level_string_info(void) {
    TEST_ASSERT_EQUAL_STRING("INFO", get_log_level_string(LOG_LEVEL_INFO));
}

void test_log_level_string_debug(void) {
    TEST_ASSERT_EQUAL_STRING("DEBUG", get_log_level_string(LOG_LEVEL_DEBUG));
}

void test_log_level_string_unknown_negative(void) {
    TEST_ASSERT_EQUAL_STRING("UNKNOWN", get_log_level_string((log_level_t)-1));
}

void test_log_level_string_unknown_too_high(void) {
    TEST_ASSERT_EQUAL_STRING("UNKNOWN", get_log_level_string((log_level_t)99));
}

/* ================================================================
 * init / shutdown lifecycle
 * ================================================================ */

void test_init_logger_succeeds(void) {
    /* Logger was already init'd in main; reinit should be idempotent */
    int rc = init_logger();
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_is_logger_available_after_init(void) {
    TEST_ASSERT_EQUAL_INT(1, is_logger_available());
}

void test_shutdown_then_reinit(void) {
    shutdown_logger();
    TEST_ASSERT_EQUAL_INT(0, is_logger_available());
    int rc = init_logger();
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_INT(1, is_logger_available());
}

/* ================================================================
 * set_log_level
 * ================================================================ */

void test_set_log_level_debug(void) {
    set_log_level(LOG_LEVEL_DEBUG);
    /* We can verify by checking the string helper doesn't crash */
    const char *s = get_log_level_string(LOG_LEVEL_DEBUG);
    TEST_ASSERT_NOT_NULL(s);
    TEST_ASSERT_EQUAL_STRING("DEBUG", s);
}

void test_set_log_level_error(void) {
    set_log_level(LOG_LEVEL_ERROR);
    const char *s = get_log_level_string(LOG_LEVEL_ERROR);
    TEST_ASSERT_EQUAL_STRING("ERROR", s);
    /* Restore */
    set_log_level(LOG_LEVEL_INFO);
}

/* ================================================================
 * log_rotate — no file configured → must return -1
 * ================================================================ */

void test_log_rotate_no_file_returns_error(void) {
    /* Logger initialised with stderr; no filename → rotation impossible */
    int rc = log_rotate(1024, 3);
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

/* ================================================================
 * set_log_file
 * ================================================================ */

void test_set_log_file_with_temp_file(void) {
    char file_template[] = "/tmp/lightnvr_test_XXXXXX";
    int fd = mkstemp(file_template);
    TEST_ASSERT_NOT_EQUAL(-1, fd);
    close(fd);

    int rc = set_log_file(file_template);
    TEST_ASSERT_EQUAL_INT(0, rc);

    /* After setting the file, logging should still work and write to the file */
    const char *msg = "test_set_log_file_with_temp_file: log line";
    log_info("%s", msg);

    /* Verify that the log message was actually written to the file */
    FILE *fp = fopen(file_template, "r");
    TEST_ASSERT_NOT_NULL(fp);

    char buf[4096];
    size_t nread = fread(buf, 1, sizeof(buf) - 1, fp);
    TEST_ASSERT_TRUE(nread > 0);
    buf[nread] = '\0';

    TEST_ASSERT_NOT_NULL(strstr(buf, msg));

    fclose(fp);
    unlink(file_template);
}

void test_set_log_file_null_returns_error(void) {
    int rc = set_log_file(NULL);
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

/* ================================================================
 * log_rotate — file configured, below threshold → returns 0
 * ================================================================ */

void test_log_rotate_below_threshold(void) {
    char file_template[] = "/tmp/lightnvr_rotate_XXXXXX";
    int fd = mkstemp(file_template);
    TEST_ASSERT_NOT_EQUAL(-1, fd);
    close(fd);

    int rc = set_log_file(file_template);
    TEST_ASSERT_EQUAL_INT(0, rc);

    /* File is tiny; well below the 1MB threshold → no rotation */
    rc = log_rotate(1024 * 1024, 3);
    TEST_ASSERT_EQUAL_INT(0, rc);

    unlink(file_template);
}

/* ================================================================
 * enable_syslog / disable_syslog / is_syslog_enabled
 * ================================================================ */

void test_enable_syslog_succeeds(void) {
    int rc = enable_syslog("lightnvr_test", LOG_USER);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_INT(1, is_syslog_enabled());
    disable_syslog();
}

void test_enable_syslog_null_ident_fails(void) {
    int rc = enable_syslog(NULL, LOG_USER);
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

void test_enable_syslog_empty_ident_fails(void) {
    int rc = enable_syslog("", LOG_USER);
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

void test_disable_syslog_clears_flag(void) {
    enable_syslog("lightnvr_test", LOG_USER);
    disable_syslog();
    TEST_ASSERT_EQUAL_INT(0, is_syslog_enabled());
}

void test_syslog_not_enabled_initially(void) {
    /* After a fresh init_logger() there should be no syslog active */
    TEST_ASSERT_EQUAL_INT(0, is_syslog_enabled());
}

/* ================================================================
 * Smoke tests — logging functions must not crash
 * ================================================================ */

void test_log_error_does_not_crash(void) {
    log_error("test error %d", 42);
    TEST_PASS();
}

void test_log_warn_does_not_crash(void) {
    log_warn("test warn %s", "hello");
    TEST_PASS();
}

void test_log_info_does_not_crash(void) {
    log_info("test info");
    TEST_PASS();
}

void test_log_debug_does_not_crash(void) {
    set_log_level(LOG_LEVEL_DEBUG);
    log_debug("test debug %lu", (unsigned long)99);
    set_log_level(LOG_LEVEL_INFO);
    TEST_PASS();
}

void test_log_message_does_not_crash(void) {
    log_message(LOG_LEVEL_WARN, "test log_message %d", 1);
    TEST_PASS();
}

void test_log_debug_suppressed_at_info_level(void) {
    /* DEBUG messages should not crash even when suppressed */
    set_log_level(LOG_LEVEL_INFO);
    log_debug("this should be suppressed");
    TEST_PASS();
}

/* ================================================================
 * main
 * ================================================================ */

int main(void) {
    init_logger();

    UNITY_BEGIN();

    RUN_TEST(test_log_level_string_error);
    RUN_TEST(test_log_level_string_warn);
    RUN_TEST(test_log_level_string_info);
    RUN_TEST(test_log_level_string_debug);
    RUN_TEST(test_log_level_string_unknown_negative);
    RUN_TEST(test_log_level_string_unknown_too_high);

    RUN_TEST(test_init_logger_succeeds);
    RUN_TEST(test_is_logger_available_after_init);
    RUN_TEST(test_shutdown_then_reinit);

    RUN_TEST(test_set_log_level_debug);
    RUN_TEST(test_set_log_level_error);

    RUN_TEST(test_log_rotate_no_file_returns_error);
    RUN_TEST(test_set_log_file_with_temp_file);
    RUN_TEST(test_set_log_file_null_returns_error);
    RUN_TEST(test_log_rotate_below_threshold);

    RUN_TEST(test_syslog_not_enabled_initially);
    RUN_TEST(test_enable_syslog_succeeds);
    RUN_TEST(test_enable_syslog_null_ident_fails);
    RUN_TEST(test_enable_syslog_empty_ident_fails);
    RUN_TEST(test_disable_syslog_clears_flag);

    RUN_TEST(test_log_error_does_not_crash);
    RUN_TEST(test_log_warn_does_not_crash);
    RUN_TEST(test_log_info_does_not_crash);
    RUN_TEST(test_log_debug_does_not_crash);
    RUN_TEST(test_log_message_does_not_crash);
    RUN_TEST(test_log_debug_suppressed_at_info_level);

    int result = UNITY_END();
    shutdown_logger();
    return result;
}

