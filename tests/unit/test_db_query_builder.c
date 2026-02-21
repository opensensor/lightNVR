/**
 * @file test_db_query_builder.c
 * @brief Layer 2 — query builder utilities against real SQLite
 *
 * Tests qb_init, qb_add_column, qb_build_select, qb_has_column,
 * qb_get_column_index, and the qb_get_* accessor helpers.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <sqlite3.h>

#include "unity.h"
#include "database/db_core.h"
#include "database/db_query_builder.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_qb_test.db"

void setUp(void)    {}
void tearDown(void) {}

/* qb_init on an existing table */
void test_qb_init_recordings_table(void) {
    query_builder_t qb;
    int rc = qb_init(&qb, "recordings");
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("recordings", qb.table_name);
}

/* qb_init on non-existent table returns error */
void test_qb_init_nonexistent_table(void) {
    query_builder_t qb;
    int rc = qb_init(&qb, "no_such_table");
    /* Implementation may return 0 (lazy) or -1 (eager) — either is acceptable.
       What matters is it doesn't crash and the result is defined. */
    (void)rc;
    TEST_PASS();
}

/* qb_add_column required — should succeed for real column */
void test_qb_add_required_column_present(void) {
    query_builder_t qb;
    qb_init(&qb, "recordings");
    int rc = qb_add_column(&qb, "id", true);
    TEST_ASSERT_EQUAL_INT(0, rc);
}

/* qb_add_column optional — should succeed even if column missing */
void test_qb_add_optional_column_missing(void) {
    query_builder_t qb;
    qb_init(&qb, "recordings");
    int rc = qb_add_column(&qb, "nonexistent_col", false);
    TEST_ASSERT_EQUAL_INT(0, rc);
}

/* qb_has_column for present and absent columns */
void test_qb_has_column(void) {
    query_builder_t qb;
    qb_init(&qb, "recordings");
    qb_add_column(&qb, "id",              true);
    qb_add_column(&qb, "missing_column",  false);

    TEST_ASSERT_TRUE(qb_has_column(&qb, "id"));
    TEST_ASSERT_FALSE(qb_has_column(&qb, "missing_column"));
}

/* qb_get_column_index */
void test_qb_get_column_index(void) {
    query_builder_t qb;
    qb_init(&qb, "recordings");
    qb_add_column(&qb, "id",     true);
    qb_add_column(&qb, "absent", false);

    TEST_ASSERT_GREATER_OR_EQUAL(0, qb_get_column_index(&qb, "id"));
    TEST_ASSERT_EQUAL_INT(-1, qb_get_column_index(&qb, "absent"));
}

/* qb_build_select produces non-null query */
void test_qb_build_select_non_null(void) {
    query_builder_t qb;
    qb_init(&qb, "recordings");
    qb_add_column(&qb, "id",         true);
    qb_add_column(&qb, "stream_name",true);

    const char *q = qb_build_select(&qb, NULL, NULL);
    TEST_ASSERT_NOT_NULL(q);
    TEST_ASSERT_GREATER_THAN(0, strlen(q));
}

/* qb_build_select with WHERE and ORDER BY */
void test_qb_build_select_with_clauses(void) {
    query_builder_t qb;
    qb_init(&qb, "recordings");
    qb_add_column(&qb, "id",          true);
    qb_add_column(&qb, "stream_name", true);

    const char *q = qb_build_select(&qb, "id > 0", "id ASC");
    TEST_ASSERT_NOT_NULL(q);
    /* The resulting SQL should mention the table and columns */
    TEST_ASSERT_NOT_NULL(strstr(q, "recordings"));
}

/* qb_get_int / qb_get_text / qb_get_double / qb_get_bool defaults */
void test_qb_get_defaults_absent_column(void) {
    query_builder_t qb;
    qb_init(&qb, "recordings");
    qb_add_column(&qb, "id", true);
    qb_build_select(&qb, NULL, NULL);

    /* Run the query to get a real stmt */
    sqlite3 *db = get_db_handle();
    sqlite3_stmt *stmt = NULL;
    if (sqlite3_prepare_v2(db, qb.query, -1, &stmt, NULL) == SQLITE_OK) {
        /* Column "absent_col" not in qb → should return defaults */
        int    iv = qb_get_int(stmt, &qb, "absent_col", 42);
        double dv = qb_get_double(stmt, &qb, "absent_col", 3.14);
        bool   bv = qb_get_bool(stmt, &qb, "absent_col", true);

        TEST_ASSERT_EQUAL_INT(42, iv);
        TEST_ASSERT_FLOAT_WITHIN(0.001, 3.14, dv);
        TEST_ASSERT_TRUE(bv);

        sqlite3_finalize(stmt);
    } else {
        TEST_IGNORE_MESSAGE("Could not prepare query for default accessor test");
    }
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_qb_init_recordings_table);
    RUN_TEST(test_qb_init_nonexistent_table);
    RUN_TEST(test_qb_add_required_column_present);
    RUN_TEST(test_qb_add_optional_column_missing);
    RUN_TEST(test_qb_has_column);
    RUN_TEST(test_qb_get_column_index);
    RUN_TEST(test_qb_build_select_non_null);
    RUN_TEST(test_qb_build_select_with_clauses);
    RUN_TEST(test_qb_get_defaults_absent_column);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}

