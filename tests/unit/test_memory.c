/**
 * @file test_memory.c
 * @brief Layer 2 unit tests — memory utility functions
 *
 * Tests safe_malloc, safe_calloc, safe_strdup, safe_strcpy, safe_strcat,
 * secure_zero_memory, and the memory tracking counters.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <string.h>
#include <stdlib.h>
#include "unity.h"
#include "utils/memory.h"
#include "core/logger.h"

/* ---- Unity boilerplate ---- */
void setUp(void)    {}
void tearDown(void) {}

/* ================================================================
 * safe_malloc
 * ================================================================ */

void test_safe_malloc_normal(void) {
    void *p = safe_malloc(64);
    TEST_ASSERT_NOT_NULL(p);
    free(p);
}

void test_safe_malloc_zero_returns_null(void) {
    void *p = safe_malloc(0);
    TEST_ASSERT_NULL(p);
}

void test_safe_malloc_large(void) {
    void *p = safe_malloc(1024 * 1024);
    TEST_ASSERT_NOT_NULL(p);
    free(p);
}

/* ================================================================
 * safe_calloc
 * ================================================================ */

void test_safe_calloc_zeroes_memory(void) {
    char *p = (char *)safe_calloc(32, 1);
    TEST_ASSERT_NOT_NULL(p);
    for (int i = 0; i < 32; i++) {
        TEST_ASSERT_EQUAL_INT(0, p[i]);
    }
    free(p);
}

void test_safe_calloc_nmemb_zero_returns_null(void) {
    void *p = safe_calloc(0, 8);
    TEST_ASSERT_NULL(p);
}

void test_safe_calloc_size_zero_returns_null(void) {
    void *p = safe_calloc(8, 0);
    TEST_ASSERT_NULL(p);
}

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
    int rc = safe_strcpy(buf, "hello", sizeof(buf));
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("hello", buf);
}

void test_safe_strcpy_truncation_returns_error(void) {
    char buf[4];
    int rc = safe_strcpy(buf, "hello_world", sizeof(buf));
    TEST_ASSERT_NOT_EQUAL(0, rc);
    /* buf should still be null-terminated */
    buf[sizeof(buf) - 1] = '\0';
    TEST_ASSERT_NOT_NULL(buf);
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

void test_safe_strcat_overflow_returns_error(void) {
    char buf[8] = "hello";
    int rc = safe_strcat(buf, "_overflow", sizeof(buf));
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

/* ================================================================
 * secure_zero_memory
 * ================================================================ */

void test_secure_zero_memory_clears_buffer(void) {
    char buf[16];
    memset(buf, 0xAB, sizeof(buf));
    secure_zero_memory(buf, sizeof(buf));
    for (size_t i = 0; i < sizeof(buf); i++) {
        TEST_ASSERT_EQUAL_INT(0, (unsigned char)buf[i]);
    }
}

/* ================================================================
 * track_memory_allocation / counters
 * ================================================================ */

void test_track_memory_allocation_increases_total(void) {
    size_t before = get_total_memory_allocated();
    track_memory_allocation(1024, true);
    size_t after = get_total_memory_allocated();
    TEST_ASSERT_GREATER_OR_EQUAL(before + 1024, after);
    /* Clean up: release what we tracked */
    track_memory_allocation(1024, false);
}

void test_peak_memory_never_decreases(void) {
    size_t peak_before = get_peak_memory_allocated();
    track_memory_allocation(4096, true);
    size_t peak_after = get_peak_memory_allocated();
    TEST_ASSERT_GREATER_OR_EQUAL(peak_before, peak_after);
    track_memory_allocation(4096, false);
}

/* ================================================================
 * main
 * ================================================================ */

int main(void) {
    init_logger();

    UNITY_BEGIN();

    RUN_TEST(test_safe_malloc_normal);
    RUN_TEST(test_safe_malloc_zero_returns_null);
    RUN_TEST(test_safe_malloc_large);

    RUN_TEST(test_safe_calloc_zeroes_memory);
    RUN_TEST(test_safe_calloc_nmemb_zero_returns_null);
    RUN_TEST(test_safe_calloc_size_zero_returns_null);

    RUN_TEST(test_safe_strdup_basic);
    RUN_TEST(test_safe_strdup_empty_string);
    RUN_TEST(test_safe_strdup_null_returns_null);

    RUN_TEST(test_safe_strcpy_success);
    RUN_TEST(test_safe_strcpy_truncation_returns_error);

    RUN_TEST(test_safe_strcat_success);
    RUN_TEST(test_safe_strcat_overflow_returns_error);

    RUN_TEST(test_secure_zero_memory_clears_buffer);

    RUN_TEST(test_track_memory_allocation_increases_total);
    RUN_TEST(test_peak_memory_never_decreases);

    int result = UNITY_END();
    shutdown_logger();
    return result;
}

