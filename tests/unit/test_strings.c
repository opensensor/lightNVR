/**
 * @file test_strings.c
 * @brief Layer 2 unit tests — ends_with() string helper
 *
 * Tests all edge cases for the ends_with() function:
 *   - matching and non-matching suffixes
 *   - empty string inputs
 *   - NULL inputs (safe fallback)
 *   - suffix longer than the string
 */

#define _POSIX_C_SOURCE 200809L

#include "unity.h"
#include "utils/strings.h"

/* ---- Unity boilerplate ---- */
void setUp(void)    {}
void tearDown(void) {}

/* ================================================================
 * ends_with — matching cases
 * ================================================================ */

void test_ends_with_simple_match(void) {
    TEST_ASSERT_TRUE(ends_with("hello.mp4", ".mp4"));
}

void test_ends_with_exact_match(void) {
    TEST_ASSERT_TRUE(ends_with(".mp4", ".mp4"));
}

void test_ends_with_long_suffix(void) {
    TEST_ASSERT_TRUE(ends_with("/path/to/file.ts", ".ts"));
}

void test_ends_with_full_string_is_suffix(void) {
    /* str == suffix → match */
    TEST_ASSERT_TRUE(ends_with("abc", "abc"));
}

/* ================================================================
 * ends_with — non-matching cases
 * ================================================================ */

void test_ends_with_no_match(void) {
    TEST_ASSERT_FALSE(ends_with("hello.mp4", ".ts"));
}

void test_ends_with_partial_suffix(void) {
    /* "mp4" is not ".mp4" */
    TEST_ASSERT_FALSE(ends_with("hello.mp4", "mp4x"));
}

void test_ends_with_case_sensitive(void) {
    /* Should be case-sensitive */
    TEST_ASSERT_FALSE(ends_with("hello.MP4", ".mp4"));
}

/* ================================================================
 * ends_with — empty string edge cases
 * ================================================================ */

void test_ends_with_empty_suffix(void) {
    /* Empty suffix → every string ends with it */
    TEST_ASSERT_TRUE(ends_with("hello", ""));
}

void test_ends_with_empty_str_empty_suffix(void) {
    TEST_ASSERT_TRUE(ends_with("", ""));
}

void test_ends_with_empty_str_nonempty_suffix(void) {
    /* Empty str cannot end with non-empty suffix */
    TEST_ASSERT_FALSE(ends_with("", ".mp4"));
}

/* ================================================================
 * ends_with — NULL safety
 * ================================================================ */

void test_ends_with_null_str(void) {
    TEST_ASSERT_FALSE(ends_with(NULL, ".mp4"));
}

void test_ends_with_null_suffix(void) {
    TEST_ASSERT_FALSE(ends_with("hello.mp4", NULL));
}

void test_ends_with_both_null(void) {
    TEST_ASSERT_FALSE(ends_with(NULL, NULL));
}

/* ================================================================
 * ends_with — suffix longer than string
 * ================================================================ */

void test_ends_with_suffix_longer_than_str(void) {
    TEST_ASSERT_FALSE(ends_with("hi", "hello"));
}

/* ================================================================
 * main
 * ================================================================ */

int main(void) {
    UNITY_BEGIN();

    RUN_TEST(test_ends_with_simple_match);
    RUN_TEST(test_ends_with_exact_match);
    RUN_TEST(test_ends_with_long_suffix);
    RUN_TEST(test_ends_with_full_string_is_suffix);

    RUN_TEST(test_ends_with_no_match);
    RUN_TEST(test_ends_with_partial_suffix);
    RUN_TEST(test_ends_with_case_sensitive);

    RUN_TEST(test_ends_with_empty_suffix);
    RUN_TEST(test_ends_with_empty_str_empty_suffix);
    RUN_TEST(test_ends_with_empty_str_nonempty_suffix);

    RUN_TEST(test_ends_with_null_str);
    RUN_TEST(test_ends_with_null_suffix);
    RUN_TEST(test_ends_with_both_null);

    RUN_TEST(test_ends_with_suffix_longer_than_str);

    return UNITY_END();
}

