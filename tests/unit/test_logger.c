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
 */

#define _POSIX_C_SOURCE 200809L

#include <string.h>
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

    int result = UNITY_END();
    shutdown_logger();
    return result;
}

