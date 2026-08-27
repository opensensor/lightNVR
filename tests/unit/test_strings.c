/**
 * @file test_strings.c
 * @brief Layer 2 unit tests — ends_with() string helper
 *
 * Tests safe_strdup, safe_strcpy, safe_strcat
 * 
 * Tests all edge cases for the ends_with() function:
 *   - matching and non-matching suffixes
 *   - empty string inputs
 *   - NULL inputs (safe fallback)
 *   - suffix longer than the string
 */

#define _POSIX_C_SOURCE 200809L

#include <string.h>
#include "unity.h"
#include "utils/strings.h"

/* ---- Unity boilerplate ---- */
void setUp(void)    {}
void tearDown(void) {}

/* ================================================================
 * safe_strdup
 * ================================================================ */

void test_safe_strdup_basic(void) {
    char *dup = safe_strdup("hello");
    TEST_ASSERT_NOT_NULL(dup);
    TEST_ASSERT_EQUAL_STRING("hello", dup);
    free(dup);
}

void test_safe_strdup_empty_string(void) {
    char *dup = safe_strdup("");
    TEST_ASSERT_NOT_NULL(dup);
    TEST_ASSERT_EQUAL_STRING("", dup);
    free(dup);
}

void test_safe_strdup_null_returns_null(void) {
    char *dup = safe_strdup(NULL);
    TEST_ASSERT_NULL(dup);
}

/* ================================================================
 * safe_strcpy
 * ================================================================ */

void test_safe_strcpy_success(void) {
    char buf[16];
    int rc = safe_strcpy(buf, "hello", sizeof(buf), 0);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("hello", buf);
}

void test_safe_strcpy_truncation_returns_success(void) {
    char buf[4];
    memset(buf, 0, sizeof(buf));
    int rc = safe_strcpy(buf, "hello_world", sizeof(buf), 0);
    TEST_ASSERT_EQUAL_INT(0, rc);
    /* buf should still be null-terminated and contain truncated data */
    TEST_ASSERT_EQUAL_INT('\0', buf[sizeof(buf) - 1]);
    TEST_ASSERT_EQUAL_STRING("hel", buf);
}

void test_safe_strcpy_unterminated_source(void) {
    char buf[16];
    memset(buf, 0, sizeof(buf));
    int rc = safe_strcpy(buf, "hello_world", sizeof(buf), 5);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("hello", buf);
}

void test_safe_strcpy_unterminated_source_truncation(void) {
    char buf[4];
    memset(buf, 0, sizeof(buf));
    int rc = safe_strcpy(buf, "hello_world", sizeof(buf), 5);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("hel", buf);
}

/* ================================================================
 * safe_strcpy — additional NULL guard cases
 * ================================================================ */

void test_safe_strcpy_null_dest_returns_error(void) {
    int rc = safe_strcpy(NULL, "hello", 10, 0);
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

void test_safe_strcpy_null_src_returns_error(void) {
    char buf[16];
    int rc = safe_strcpy(buf, NULL, sizeof(buf), 0);
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

void test_safe_strcpy_zero_size_returns_error(void) {
    char buf[16];
    int rc = safe_strcpy(buf, "hello", 0, 0);
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

/* ================================================================
 * safe_strcat
 * ================================================================ */

void test_safe_strcat_success(void) {
    char buf[32] = "hello";
    int rc = safe_strcat(buf, " world", sizeof(buf));
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("hello world", buf);
}

void test_safe_strcat_overflow_returns_success(void) {
    char buf[8] = "hello";
    int rc = safe_strcat(buf, "_overflow", sizeof(buf));
    TEST_ASSERT_EQUAL_INT(0, rc);
    /* Buffer should remain a valid, null-terminated string and keep its safe content */
    TEST_ASSERT_EQUAL_INT('\0', buf[sizeof(buf) - 1] == '\0' ? '\0' : buf[sizeof(buf) - 1]);
    TEST_ASSERT_EQUAL_STRING("hello_o", buf);
}

/* ================================================================
 * safe_strcat — additional NULL guard cases
 * ================================================================ */

void test_safe_strcat_null_dest_returns_error(void) {
    int rc = safe_strcat(NULL, "world", 10);
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

void test_safe_strcat_null_src_returns_error(void) {
    char buf[16] = "hello";
    int rc = safe_strcat(buf, NULL, sizeof(buf));
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

void test_safe_strcat_zero_size_returns_error(void) {
    char buf[16] = "hello";
    int rc = safe_strcat(buf, " world", 0);
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

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
 * trim_ascii_whitespace
 * ================================================================ */

void test_trim_ascii_whitespace_null(void) {
    TEST_ASSERT_NULL(trim_ascii_whitespace(NULL));
}

void test_trim_ascii_whitespace_empty(void) {
    char msg[] = "";
    char *trim = trim_ascii_whitespace(msg);
    TEST_ASSERT_EQUAL_PTR(msg, trim);
}

void test_trim_ascii_whitespace_spaces(void) {
    char msg[] = "   \t\t  ";
    char *trim = trim_ascii_whitespace(msg);
    TEST_ASSERT_EQUAL_STRING("", trim);
}

void test_trim_ascii_whitespace_newline(void) {
    char msg[] = "\r\nhello_world\r\n\n";
    char *trim = trim_ascii_whitespace(msg);
    TEST_ASSERT_EQUAL_STRING("hello_world", trim);
}

void test_trim_ascii_whitespace_nonprint(void) {
    char msg[] = "\a\ehello   world\b\f";
    char *trim = trim_ascii_whitespace(msg);
    TEST_ASSERT_EQUAL_STRING("hello   world", trim);
}

/* ================================================================
 * string_has_non_whitespace
 * ================================================================ */

void test_string_has_non_whitespace_null(void) {
    TEST_ASSERT_FALSE(string_has_non_whitespace(NULL));
}

void test_string_has_non_whitespace_empty(void) {
    TEST_ASSERT_FALSE(string_has_non_whitespace(""));
}

void test_string_has_non_whitespace_only(void) {
    TEST_ASSERT_FALSE(string_has_non_whitespace(" \t\r\n\v\f"));
}

void test_string_has_non_whitespace_content(void) {
    TEST_ASSERT_TRUE(string_has_non_whitespace(" \n  - rtsp://camera/ch0\n"));
}

/* ================================================================
 * copy_trimmed_value
 * ================================================================ */

void test_copy_trimmed_value_null_output(void) {
    TEST_ASSERT_EQUAL_size_t(0, copy_trimmed_value(NULL, 20, "text", 0));
}

void test_copy_trimmed_value_zero_output_size(void) {
    char buf[32];
    TEST_ASSERT_EQUAL_size_t(0, copy_trimmed_value(buf, 0, "text", 0));
}

void test_copy_trimmed_value_null_input(void) {
    char buf[32];
    TEST_ASSERT_EQUAL_size_t(0, copy_trimmed_value(buf, 0, NULL, 0));
}

void test_copy_trimmed_value_success(void) {
    char buf[32];
    size_t ret = copy_trimmed_value(buf, sizeof(buf), "  hello world  ", 0);
    char expected[] = "hello world";
    TEST_ASSERT_EQUAL_STRING(buf, expected);
    TEST_ASSERT_EQUAL_size_t(strlen(expected), ret);
}

void test_copy_trimmed_value_truncate(void) {
    char buf[4];
    size_t ret = copy_trimmed_value(buf, sizeof(buf), "  hello world  ", 0);
    char expected[] = "hel";
    TEST_ASSERT_EQUAL_STRING(buf, expected);
    TEST_ASSERT_EQUAL_size_t(strlen(expected), ret);
}

void test_copy_trimmed_value_unterminated(void) {
    char buf[32];
    size_t ret = copy_trimmed_value(buf, sizeof(buf), "  hello world  ", 8);
    // Destination should *not* include the additional whitespace present in the
    // unterminated input.
    char expected[] = "hello";
    TEST_ASSERT_EQUAL_STRING(buf, expected);
    TEST_ASSERT_EQUAL_size_t(strlen(expected), ret);
}

void test_copy_trimmed_value_truncate_with_whitespace(void) {
    char buf[7];
    size_t ret = copy_trimmed_value(buf, sizeof(buf), "  hello world  ", 0);
    // Note that the (truncated) output *does* include whitespace at the end; that's
    // because the truncation occurs after trimming the whitespace on the source.
    char expected[] = "hello ";
    TEST_ASSERT_EQUAL_STRING(buf, expected);
    TEST_ASSERT_EQUAL_size_t(strlen(expected), ret);
}

/* ================================================================
 * main
 * ================================================================ */

int main(void) {
    UNITY_BEGIN();

    RUN_TEST(test_safe_strdup_basic);
    RUN_TEST(test_safe_strdup_empty_string);
    RUN_TEST(test_safe_strdup_null_returns_null);

    RUN_TEST(test_safe_strcpy_success);
    RUN_TEST(test_safe_strcpy_truncation_returns_success);
    RUN_TEST(test_safe_strcpy_unterminated_source);
    RUN_TEST(test_safe_strcpy_unterminated_source_truncation);
    RUN_TEST(test_safe_strcpy_null_dest_returns_error);
    RUN_TEST(test_safe_strcpy_null_src_returns_error);
    RUN_TEST(test_safe_strcpy_zero_size_returns_error);

    RUN_TEST(test_safe_strcat_success);
    RUN_TEST(test_safe_strcat_overflow_returns_success);
    RUN_TEST(test_safe_strcat_null_dest_returns_error);
    RUN_TEST(test_safe_strcat_null_src_returns_error);
    RUN_TEST(test_safe_strcat_zero_size_returns_error);

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

    RUN_TEST(test_trim_ascii_whitespace_null);
    RUN_TEST(test_trim_ascii_whitespace_empty);
    RUN_TEST(test_trim_ascii_whitespace_spaces);
    RUN_TEST(test_trim_ascii_whitespace_newline);
    RUN_TEST(test_trim_ascii_whitespace_nonprint);

    RUN_TEST(test_string_has_non_whitespace_null);
    RUN_TEST(test_string_has_non_whitespace_empty);
    RUN_TEST(test_string_has_non_whitespace_only);
    RUN_TEST(test_string_has_non_whitespace_content);

    RUN_TEST(test_copy_trimmed_value_null_output);
    RUN_TEST(test_copy_trimmed_value_zero_output_size);
    RUN_TEST(test_copy_trimmed_value_null_input);
    RUN_TEST(test_copy_trimmed_value_success);
    RUN_TEST(test_copy_trimmed_value_truncate);
    RUN_TEST(test_copy_trimmed_value_unterminated);
    RUN_TEST(test_copy_trimmed_value_truncate_with_whitespace);

    return UNITY_END();
}
