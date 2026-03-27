/**
 * @file test_db_system_settings.c
 * @brief Layer 2 Unity tests for database/db_system_settings.c
 *
 * Covers the complete db_system_settings API:
 *   - db_get_system_setting
 *   - db_set_system_setting
 *   - db_is_setup_complete
 *   - db_mark_setup_complete
 * Including null argument guards and overwrite semantics.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <sqlite3.h>

#include "unity.h"
#include "database/db_core.h"
#include "database/db_system_settings.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_system_settings_test.db"

/* ---- helpers ---- */

static void clear_settings(void) {
    sqlite3 *db = get_db_handle();
    sqlite3_exec(db, "DELETE FROM system_settings;", NULL, NULL, NULL);
}

void setUp(void)    { clear_settings(); }
void tearDown(void) {}

/* ------------------------------------------------------------------ tests */

void test_get_missing_key_returns_minus_one(void) {
    char val[64] = {0};
    int rc = db_get_system_setting("no_such_key", val, sizeof(val));
    TEST_ASSERT_EQUAL_INT(-1, rc);
}

void test_set_and_get_setting(void) {
    int rc = db_set_system_setting("test_key", "hello");
    TEST_ASSERT_EQUAL_INT(0, rc);

    char val[64] = {0};
    rc = db_get_system_setting("test_key", val, sizeof(val));
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("hello", val);
}

void test_set_overwrites_existing(void) {
    db_set_system_setting("overwrite_key", "first");
    db_set_system_setting("overwrite_key", "second");

    char val[64] = {0};
    db_get_system_setting("overwrite_key", val, sizeof(val));
    TEST_ASSERT_EQUAL_STRING("second", val);
}

void test_multiple_independent_keys(void) {
    db_set_system_setting("key_a", "value_a");
    db_set_system_setting("key_b", "value_b");

    char va[64] = {0}, vb[64] = {0};
    db_get_system_setting("key_a", va, sizeof(va));
    db_get_system_setting("key_b", vb, sizeof(vb));

    TEST_ASSERT_EQUAL_STRING("value_a", va);
    TEST_ASSERT_EQUAL_STRING("value_b", vb);
}

void test_is_setup_complete_initially_false(void) {
    TEST_ASSERT_FALSE(db_is_setup_complete());
}

void test_mark_setup_complete_sets_flag(void) {
    int rc = db_mark_setup_complete();
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_TRUE(db_is_setup_complete());
}

void test_mark_setup_complete_is_idempotent(void) {
    TEST_ASSERT_EQUAL_INT(0, db_mark_setup_complete());
    TEST_ASSERT_EQUAL_INT(0, db_mark_setup_complete());
    TEST_ASSERT_TRUE(db_is_setup_complete());
}

void test_mark_setup_complete_records_timestamp(void) {
    db_mark_setup_complete();

    char ts[32] = {0};
    int rc = db_get_system_setting("setup_completed_at", ts, sizeof(ts));
    TEST_ASSERT_EQUAL_INT(0, rc);
    /* The timestamp should be a non-empty numeric string */
    TEST_ASSERT_TRUE(ts[0] >= '0' && ts[0] <= '9');
}

void test_get_null_key_fails(void) {
    char val[64] = {0};
    TEST_ASSERT_EQUAL_INT(-1, db_get_system_setting(NULL, val, sizeof(val)));
}

void test_get_null_out_fails(void) {
    TEST_ASSERT_EQUAL_INT(-1, db_get_system_setting("any_key", NULL, 64));
}

void test_get_zero_out_len_fails(void) {
    char val[64] = {0};
    TEST_ASSERT_EQUAL_INT(-1, db_get_system_setting("any_key", val, 0));
}

void test_set_null_key_fails(void) {
    TEST_ASSERT_EQUAL_INT(-1, db_set_system_setting(NULL, "value"));
}

void test_set_null_value_fails(void) {
    TEST_ASSERT_EQUAL_INT(-1, db_set_system_setting("some_key", NULL));
}

void test_value_truncated_to_buffer_size(void) {
    db_set_system_setting("long_key", "abcdefghij");

    char small[5] = {0}; /* can only hold 4 chars + NUL */
    int rc = db_get_system_setting("long_key", small, sizeof(small));
    TEST_ASSERT_EQUAL_INT(0, rc);
    /* Buffer should be NUL-terminated and at most (sizeof-1) chars long */
    TEST_ASSERT_EQUAL_INT(4, (int)strlen(small));
    TEST_ASSERT_EQUAL_STRING("abcd", small);
}

/* ------------------------------------------------------------------ main */

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }

    UNITY_BEGIN();
    RUN_TEST(test_get_missing_key_returns_minus_one);
    RUN_TEST(test_set_and_get_setting);
    RUN_TEST(test_set_overwrites_existing);
    RUN_TEST(test_multiple_independent_keys);
    RUN_TEST(test_is_setup_complete_initially_false);
    RUN_TEST(test_mark_setup_complete_sets_flag);
    RUN_TEST(test_mark_setup_complete_is_idempotent);
    RUN_TEST(test_mark_setup_complete_records_timestamp);
    RUN_TEST(test_get_null_key_fails);
    RUN_TEST(test_get_null_out_fails);
    RUN_TEST(test_get_zero_out_len_fails);
    RUN_TEST(test_set_null_key_fails);
    RUN_TEST(test_set_null_value_fails);
    RUN_TEST(test_value_truncated_to_buffer_size);
    int result = UNITY_END();

    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}

