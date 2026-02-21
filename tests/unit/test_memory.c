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
    /* buf should still be null-terminated and contain truncated data */
    TEST_ASSERT_EQUAL_INT('\0', buf[sizeof(buf) - 1]);
    TEST_ASSERT_EQUAL_STRING("hel", buf);
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
    /* Buffer should remain a valid, null-terminated string and keep its safe content */
    TEST_ASSERT_EQUAL_INT('\0', buf[sizeof(buf) - 1] == '\0' ? '\0' : buf[sizeof(buf) - 1]);
    TEST_ASSERT_EQUAL_STRING_LEN("hello", buf, 5);
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
 * safe_realloc
 * ================================================================ */

void test_safe_realloc_grow(void) {
    char *p = (char *)safe_malloc(16);
    TEST_ASSERT_NOT_NULL(p);
    p[0] = 'A';
    p = (char *)safe_realloc(p, 64);
    TEST_ASSERT_NOT_NULL(p);
    TEST_ASSERT_EQUAL_CHAR('A', p[0]); /* data preserved */
    free(p);
}

void test_safe_realloc_shrink(void) {
    char *p = (char *)safe_malloc(64);
    TEST_ASSERT_NOT_NULL(p);
    p = (char *)safe_realloc(p, 8);
    TEST_ASSERT_NOT_NULL(p);
    free(p);
}

void test_safe_realloc_zero_frees_and_returns_null(void) {
    char *p = (char *)safe_malloc(32);
    TEST_ASSERT_NOT_NULL(p);
    void *result = safe_realloc(p, 0);
    /* safe_realloc(ptr, 0) must return NULL */
    TEST_ASSERT_NULL(result);
    /* p has been freed internally; don't free again */
}

/* ================================================================
 * safe_free
 * ================================================================ */

void test_safe_free_null_is_safe(void) {
    safe_free(NULL); /* must not crash */
    TEST_PASS();
}

void test_safe_free_valid_pointer(void) {
    void *p = safe_malloc(32);
    TEST_ASSERT_NOT_NULL(p);
    safe_free(p); /* must not crash */
    TEST_PASS();
}

/* ================================================================
 * safe_strcpy — additional NULL guard cases
 * ================================================================ */

void test_safe_strcpy_null_dest_returns_error(void) {
    int rc = safe_strcpy(NULL, "hello", 10);
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

void test_safe_strcpy_null_src_returns_error(void) {
    char buf[16];
    int rc = safe_strcpy(buf, NULL, sizeof(buf));
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

void test_safe_strcpy_zero_size_returns_error(void) {
    char buf[16];
    int rc = safe_strcpy(buf, "hello", 0);
    TEST_ASSERT_NOT_EQUAL(0, rc);
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
 * secure_zero_memory — edge cases
 * ================================================================ */

void test_secure_zero_memory_null_no_crash(void) {
    secure_zero_memory(NULL, 16); /* must not crash */
    TEST_PASS();
}

void test_secure_zero_memory_zero_size_no_crash(void) {
    char buf[4] = {0x1, 0x2, 0x3, 0x4};
    secure_zero_memory(buf, 0); /* must not crash or alter buffer */
    TEST_PASS();
}

/* ================================================================
 * track_memory_allocation — underflow handled gracefully
 * ================================================================ */

void test_track_memory_underflow_handled(void) {
    /* Preserve original total so we can restore it after the test. */
    size_t original_total = get_total_memory_allocated();

    /* Reset total to 0 first by freeing anything we know about */
    if (original_total > 0) {
        track_memory_allocation(original_total, false);
    }

    /* Now freeing more than tracked should not crash and reset to 0 */
    track_memory_allocation(9999, false);
    TEST_ASSERT_EQUAL_UINT(0, get_total_memory_allocated());

    /* Restore original total so this test does not affect others. */
    if (original_total > 0) {
        track_memory_allocation(original_total, true);
    }

    /* Verify that restoration succeeded so later tests see original state. */
    TEST_ASSERT_EQUAL_UINT(original_total, get_total_memory_allocated());
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
    RUN_TEST(test_safe_strcpy_null_dest_returns_error);
    RUN_TEST(test_safe_strcpy_null_src_returns_error);
    RUN_TEST(test_safe_strcpy_zero_size_returns_error);

    RUN_TEST(test_safe_strcat_success);
    RUN_TEST(test_safe_strcat_overflow_returns_error);
    RUN_TEST(test_safe_strcat_null_dest_returns_error);
    RUN_TEST(test_safe_strcat_null_src_returns_error);
    RUN_TEST(test_safe_strcat_zero_size_returns_error);

    RUN_TEST(test_safe_realloc_grow);
    RUN_TEST(test_safe_realloc_shrink);
    RUN_TEST(test_safe_realloc_zero_frees_and_returns_null);

    RUN_TEST(test_safe_free_null_is_safe);
    RUN_TEST(test_safe_free_valid_pointer);

    RUN_TEST(test_secure_zero_memory_clears_buffer);
    RUN_TEST(test_secure_zero_memory_null_no_crash);
    RUN_TEST(test_secure_zero_memory_zero_size_no_crash);

    RUN_TEST(test_track_memory_allocation_increases_total);
    RUN_TEST(test_peak_memory_never_decreases);
    RUN_TEST(test_track_memory_underflow_handled);

    int result = UNITY_END();
    shutdown_logger();
    return result;
}

