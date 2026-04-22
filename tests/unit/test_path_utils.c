/**
 * @file test_path_utils.c
 * @brief Layer 2 unit tests — stream name validation and sanitization
 *
 * Covers is_valid_stream_name() (added for #396) and sanitize_stream_name().
 *
 * Stream names flow through filesystem paths, URLs, and the go2rtc stream
 * table, so they must survive URL-encoding round-trips unchanged. Names
 * containing spaces or other special characters produce "mse: stream not
 * found" errors because the client sends the URL-encoded form while
 * go2rtc registered the decoded form.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <string.h>

#include "unity.h"
#include "core/path_utils.h"

void setUp(void)    {}
void tearDown(void) {}

/* ================================================================
 * is_valid_stream_name — accepts
 * ================================================================ */

void test_valid_simple_lowercase(void) {
    TEST_ASSERT_TRUE(is_valid_stream_name("front_door"));
}

void test_valid_mixed_case(void) {
    TEST_ASSERT_TRUE(is_valid_stream_name("FrontDoor"));
}

void test_valid_digits(void) {
    TEST_ASSERT_TRUE(is_valid_stream_name("cam01"));
}

void test_valid_hyphen(void) {
    TEST_ASSERT_TRUE(is_valid_stream_name("front-door"));
}

void test_valid_underscore(void) {
    TEST_ASSERT_TRUE(is_valid_stream_name("front_door_camera"));
}

void test_valid_dot_in_middle(void) {
    TEST_ASSERT_TRUE(is_valid_stream_name("cam.1"));
}

void test_valid_single_char(void) {
    TEST_ASSERT_TRUE(is_valid_stream_name("a"));
}

/* ================================================================
 * is_valid_stream_name — rejects (the #396 cases)
 * ================================================================ */

void test_reject_null(void) {
    TEST_ASSERT_FALSE(is_valid_stream_name(NULL));
}

void test_reject_empty(void) {
    TEST_ASSERT_FALSE(is_valid_stream_name(""));
}

void test_reject_space_in_middle(void) {
    /* The primary #396 case: "Front Door Camera" breaks MSE/HLS. */
    TEST_ASSERT_FALSE(is_valid_stream_name("Front Door Camera"));
}

void test_reject_leading_space(void) {
    TEST_ASSERT_FALSE(is_valid_stream_name(" front_door"));
}

void test_reject_trailing_space(void) {
    TEST_ASSERT_FALSE(is_valid_stream_name("front_door "));
}

void test_reject_tab(void) {
    TEST_ASSERT_FALSE(is_valid_stream_name("front\tdoor"));
}

void test_reject_slash(void) {
    /* Would break path routing (/api/streams/{name}/...). */
    TEST_ASSERT_FALSE(is_valid_stream_name("front/door"));
}

void test_reject_backslash(void) {
    TEST_ASSERT_FALSE(is_valid_stream_name("front\\door"));
}

void test_reject_question_mark(void) {
    /* Would become a query-string boundary. */
    TEST_ASSERT_FALSE(is_valid_stream_name("front?door"));
}

void test_reject_ampersand(void) {
    TEST_ASSERT_FALSE(is_valid_stream_name("front&door"));
}

void test_reject_hash(void) {
    /* URL fragment boundary. */
    TEST_ASSERT_FALSE(is_valid_stream_name("front#door"));
}

void test_reject_percent(void) {
    /* Looks like URL-encoding. */
    TEST_ASSERT_FALSE(is_valid_stream_name("front%20door"));
}

void test_reject_leading_dot(void) {
    /* Would produce a hidden-file path. */
    TEST_ASSERT_FALSE(is_valid_stream_name(".hidden"));
}

void test_reject_dot_dot(void) {
    /* Path traversal. */
    TEST_ASSERT_FALSE(is_valid_stream_name(".."));
}

void test_reject_non_ascii(void) {
    /* Non-ASCII round-trips poorly through URL encoding. */
    TEST_ASSERT_FALSE(is_valid_stream_name("caméra"));
}

void test_reject_null_byte_is_not_applicable(void) {
    /* C strings terminate at NUL; nothing to test beyond empty case. */
    TEST_PASS();
}

/* ================================================================
 * sanitize_stream_name regression coverage
 *
 * The validator and sanitizer must agree on the "safe" character set:
 * sanitize_stream_name() maps spaces to underscores, which means
 * "Front Door" and "Front_Door" both collapse to "Front_Door" on disk.
 * That collision is exactly why is_valid_stream_name() rejects spaces.
 * ================================================================ */

void test_sanitize_maps_space_to_underscore(void) {
    char out[64];
    sanitize_stream_name("Front Door", out, sizeof(out));
    TEST_ASSERT_EQUAL_STRING("Front_Door", out);
}

void test_sanitize_preserves_valid_names(void) {
    /* A name that passes is_valid_stream_name must be unchanged by sanitize. */
    char out[64];
    sanitize_stream_name("front_door-1", out, sizeof(out));
    TEST_ASSERT_EQUAL_STRING("front_door-1", out);
}

void test_sanitize_two_names_that_would_collide(void) {
    /* Demonstrates the collision that motivates rejecting spaces at creation. */
    char out_a[64], out_b[64];
    sanitize_stream_name("Front Door",  out_a, sizeof(out_a));
    sanitize_stream_name("Front_Door",  out_b, sizeof(out_b));
    TEST_ASSERT_EQUAL_STRING(out_a, out_b);
}

/* ================================================================
 * main
 * ================================================================ */

int main(void) {
    UNITY_BEGIN();

    RUN_TEST(test_valid_simple_lowercase);
    RUN_TEST(test_valid_mixed_case);
    RUN_TEST(test_valid_digits);
    RUN_TEST(test_valid_hyphen);
    RUN_TEST(test_valid_underscore);
    RUN_TEST(test_valid_dot_in_middle);
    RUN_TEST(test_valid_single_char);

    RUN_TEST(test_reject_null);
    RUN_TEST(test_reject_empty);
    RUN_TEST(test_reject_space_in_middle);
    RUN_TEST(test_reject_leading_space);
    RUN_TEST(test_reject_trailing_space);
    RUN_TEST(test_reject_tab);
    RUN_TEST(test_reject_slash);
    RUN_TEST(test_reject_backslash);
    RUN_TEST(test_reject_question_mark);
    RUN_TEST(test_reject_ampersand);
    RUN_TEST(test_reject_hash);
    RUN_TEST(test_reject_percent);
    RUN_TEST(test_reject_leading_dot);
    RUN_TEST(test_reject_dot_dot);
    RUN_TEST(test_reject_non_ascii);
    RUN_TEST(test_reject_null_byte_is_not_applicable);

    RUN_TEST(test_sanitize_maps_space_to_underscore);
    RUN_TEST(test_sanitize_preserves_valid_names);
    RUN_TEST(test_sanitize_two_names_that_would_collide);

    return UNITY_END();
}
