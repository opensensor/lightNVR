/**
 * @file test_db_schema_cache.c
 * @brief Layer 2 Unity tests for database/db_schema_cache.c
 *
 * Exercises the schema-column-existence cache:
 *   - init_schema_cache (idempotent)
 *   - cached_column_exists — hit on pre-cached columns
 *   - cached_column_exists — miss populates cache for real columns
 *   - cached_column_exists — returns false for non-existent columns
 *   - cached_column_exists — null argument guards
 *   - free_schema_cache + re-init cycle
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "unity.h"
#include "database/db_core.h"
#include "database/db_schema_cache.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_schema_cache_test.db"

/* setUp / tearDown reset the cache around every test so each test
   starts with a clean, freshly-initialised cache state. */
void setUp(void) {
    free_schema_cache();
    init_schema_cache();
}

void tearDown(void) {
    free_schema_cache();
}

/* ------------------------------------------------------------------ tests */

/* init_schema_cache must be callable a second time without crashing */
void test_init_schema_cache_is_idempotent(void) {
    init_schema_cache();   /* double init */
    /* If we reach here without a crash, the test passes. */
    TEST_PASS();
}

/* The streams table is created by the schema migrations; the columns
   pre-cached by init_schema_cache() must all report as existing. */
void test_cached_column_exists_detection_based_recording(void) {
    TEST_ASSERT_TRUE(cached_column_exists("streams", "detection_based_recording"));
}

void test_cached_column_exists_protocol(void) {
    TEST_ASSERT_TRUE(cached_column_exists("streams", "protocol"));
}

void test_cached_column_exists_is_onvif(void) {
    TEST_ASSERT_TRUE(cached_column_exists("streams", "is_onvif"));
}

void test_cached_column_exists_record_audio(void) {
    TEST_ASSERT_TRUE(cached_column_exists("streams", "record_audio"));
}

/* The "name" column has always existed in the streams table. */
void test_cached_column_exists_name_col_in_streams(void) {
    TEST_ASSERT_TRUE(cached_column_exists("streams", "name"));
}

/* A column that does not exist must return false. */
void test_cached_column_exists_nonexistent_column(void) {
    TEST_ASSERT_FALSE(cached_column_exists("streams", "column_that_does_not_exist_xyz"));
}

/* A table that does not exist must return false. */
void test_cached_column_exists_nonexistent_table(void) {
    TEST_ASSERT_FALSE(cached_column_exists("no_such_table_abc", "id"));
}

/* Null arguments must be handled without crashing. */
void test_cached_column_exists_null_table(void) {
    TEST_ASSERT_FALSE(cached_column_exists(NULL, "id"));
}

void test_cached_column_exists_null_column(void) {
    TEST_ASSERT_FALSE(cached_column_exists("streams", NULL));
}

void test_cached_column_exists_both_null(void) {
    TEST_ASSERT_FALSE(cached_column_exists(NULL, NULL));
}

/* Repeated lookup of the same column returns a consistent answer
   (tests the cache-hit path on the second call). */
void test_cached_column_exists_consistent_on_second_call(void) {
    bool first  = cached_column_exists("streams", "name");
    bool second = cached_column_exists("streams", "name");
    TEST_ASSERT_EQUAL(first, second);
}

/* free_schema_cache + re-init must not crash and must still work. */
void test_free_and_reinit_schema_cache(void) {
    free_schema_cache();
    init_schema_cache();
    TEST_ASSERT_TRUE(cached_column_exists("streams", "protocol"));
}

/* ------------------------------------------------------------------ main */

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }

    UNITY_BEGIN();
    RUN_TEST(test_init_schema_cache_is_idempotent);
    RUN_TEST(test_cached_column_exists_detection_based_recording);
    RUN_TEST(test_cached_column_exists_protocol);
    RUN_TEST(test_cached_column_exists_is_onvif);
    RUN_TEST(test_cached_column_exists_record_audio);
    RUN_TEST(test_cached_column_exists_name_col_in_streams);
    RUN_TEST(test_cached_column_exists_nonexistent_column);
    RUN_TEST(test_cached_column_exists_nonexistent_table);
    RUN_TEST(test_cached_column_exists_null_table);
    RUN_TEST(test_cached_column_exists_null_column);
    RUN_TEST(test_cached_column_exists_both_null);
    RUN_TEST(test_cached_column_exists_consistent_on_second_call);
    RUN_TEST(test_free_and_reinit_schema_cache);
    int result = UNITY_END();

    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}

