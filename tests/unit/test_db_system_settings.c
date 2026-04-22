/**
 * @file test_db_system_settings.c
 * @brief Layer 2 — system_settings key/value store (SQLite-backed).
 *
 * Covers db_get_system_setting / db_set_system_setting round-trip and the
 * new heap-allocating helper db_get_system_setting_alloc, which is required
 * to safely read large payloads (e.g. go2rtc_config_override up to ~64 KB)
 * without stack-buffer truncation on musl/Alpine worker threads.
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
#include "database/db_system_settings.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_system_settings_test.db"

void setUp(void)    {}
void tearDown(void) {}

/* Sanity: set then get a small ASCII value. */
void test_set_and_get_small_roundtrip(void) {
    int rc = db_set_system_setting("unit_small", "hello-world");
    TEST_ASSERT_EQUAL_INT(0, rc);

    char buf[64] = {0};
    rc = db_get_system_setting("unit_small", buf, sizeof(buf));
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("hello-world", buf);
}

/*
 * RED-driver test for db_get_system_setting_alloc:
 *
 * Roundtrip a 50 KB payload through the system_settings table using the
 * heap-backed helper. The legacy fixed-buffer helper silently truncates
 * values larger than the caller's stack buffer (which, in the GET handler
 * path, was 4 KB). The new helper must allocate and return the full value.
 */
void test_get_alloc_large_payload_roundtrip(void) {
    const size_t payload_len = 50 * 1024; /* 50 KB */
    char *payload = malloc(payload_len + 1);
    TEST_ASSERT_NOT_NULL(payload);

    /* Fill with a repeating printable pattern so truncation shows up. */
    for (size_t i = 0; i < payload_len; i++) {
        payload[i] = (char)('A' + (i % 26));
    }
    payload[payload_len] = '\0';

    int rc = db_set_system_setting("unit_large_override", payload);
    TEST_ASSERT_EQUAL_INT(0, rc);

    char *out = NULL;
    size_t out_len = 0;
    rc = db_get_system_setting_alloc("unit_large_override", &out, &out_len);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_NOT_NULL(out);
    TEST_ASSERT_EQUAL_UINT(payload_len, out_len);
    TEST_ASSERT_EQUAL_INT(0, memcmp(payload, out, payload_len));
    TEST_ASSERT_EQUAL_INT('\0', out[payload_len]);

    free(out);
    free(payload);
}

/* Missing key path: helper returns 1, clears out pointer and length. */
void test_get_alloc_missing_key(void) {
    char *out = (char *)(uintptr_t)0xdeadbeef; /* must be reset to NULL */
    size_t out_len = 999;

    int rc = db_get_system_setting_alloc("unit_missing_key_xyz", &out, &out_len);
    TEST_ASSERT_EQUAL_INT(1, rc);
    TEST_ASSERT_NULL(out);
    TEST_ASSERT_EQUAL_UINT(0, out_len);
}

/* Invalid-args path: must not crash, returns -1. */
void test_get_alloc_invalid_args(void) {
    char *out = NULL;
    size_t out_len = 0;
    TEST_ASSERT_EQUAL_INT(-1, db_get_system_setting_alloc(NULL, &out, &out_len));
    TEST_ASSERT_EQUAL_INT(-1, db_get_system_setting_alloc("k", NULL, &out_len));
    TEST_ASSERT_EQUAL_INT(-1, db_get_system_setting_alloc("k", &out, NULL));
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }

    UNITY_BEGIN();
    RUN_TEST(test_set_and_get_small_roundtrip);
    RUN_TEST(test_get_alloc_large_payload_roundtrip);
    RUN_TEST(test_get_alloc_missing_key);
    RUN_TEST(test_get_alloc_invalid_args);
    int result = UNITY_END();

    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
