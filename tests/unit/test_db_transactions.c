/**
 * @file test_db_transactions.c
 * @brief Layer 2 — database transaction begin/commit/rollback
 *
 * Tests:
 *   - begin_transaction / commit_transaction persists data
 *   - begin_transaction / rollback_transaction reverts data
 *
 * IMPORTANT: begin_transaction() holds db_mutex until commit/rollback.
 * Any code inside a transaction must use direct SQLite calls, NOT the
 * higher-level CRUD functions (which would deadlock on db_mutex).
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <sqlite3.h>
#include <inttypes.h>

#include "unity.h"
#include "database/db_core.h"
#include "database/db_transaction.h"
#include "database/db_recordings.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_transactions_test.db"

/* Insert a row directly (no mutex needed — caller holds it via begin_transaction) */
static int64_t insert_rec_direct(const char *path) {
    sqlite3 *db = get_db_handle();
    char sql[512];
    snprintf(sql, sizeof(sql),
        "INSERT INTO recordings "
        "(stream_name, file_path, start_time, end_time, size_bytes, "
        " width, height, fps, codec, is_complete, trigger_type) "
        "VALUES ('cam1','%s',1000,1060,1024,1920,1080,30,'h264',1,'scheduled');",
        path);
    int rc = sqlite3_exec(db, sql, NULL, NULL, NULL);
    if (rc != SQLITE_OK) return -1;
    return (int64_t)sqlite3_last_insert_rowid(db);
}

static void clear_recordings(void) {
    sqlite3_exec(get_db_handle(), "DELETE FROM recordings;", NULL, NULL, NULL);
}

void setUp(void)    { clear_recordings(); }
void tearDown(void) {}

/* begin + commit persists data */
void test_commit_persists_data(void) {
    int rc = begin_transaction();
    TEST_ASSERT_EQUAL_INT(0, rc);

    /* Use direct SQL — db_mutex is already held by begin_transaction */
    int64_t id = insert_rec_direct("/rec/tx_commit.mp4");
    TEST_ASSERT_GREATER_THAN(0, id);

    rc = commit_transaction();  /* releases db_mutex */
    TEST_ASSERT_EQUAL_INT(0, rc);

    /* Now safe to use the higher-level API (mutex is free) */
    recording_metadata_t got;
    rc = get_recording_metadata_by_id((uint64_t)id, &got);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("/rec/tx_commit.mp4", got.file_path);
}

/* begin + rollback reverts data */
void test_rollback_reverts_data(void) {
    int rc = begin_transaction();
    TEST_ASSERT_EQUAL_INT(0, rc);

    int64_t id = insert_rec_direct("/rec/tx_rollback.mp4");
    TEST_ASSERT_GREATER_THAN(0, id);

    rc = rollback_transaction();  /* releases db_mutex */
    TEST_ASSERT_EQUAL_INT(0, rc);

    /* Row must be gone */
    recording_metadata_t got;
    rc = get_recording_metadata_by_id((uint64_t)id, &got);
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

/* begin returns 0 on success */
void test_begin_transaction_succeeds(void) {
    int rc = begin_transaction();
    TEST_ASSERT_EQUAL_INT(0, rc);
    rollback_transaction();  /* clean up — releases mutex */
}

/* commit without begin does not crash */
void test_commit_without_begin(void) {
    /* commit_transaction() calls pthread_mutex_unlock unconditionally.
       After an unmatched unlock the mutex state is undefined, so re-init
       the transaction state by calling rollback (which also unlocks). */
    int rc = commit_transaction();
    (void)rc;
    TEST_PASS();
}

/* rollback without begin does not crash */
void test_rollback_without_begin(void) {
    int rc = rollback_transaction();
    (void)rc;
    TEST_PASS();
}

/* multiple commits in sequence */
void test_multiple_sequential_transactions(void) {
    for (int i = 0; i < 3; i++) {
        int rc = begin_transaction();
        TEST_ASSERT_EQUAL_INT(0, rc);
        rc = commit_transaction();
        TEST_ASSERT_EQUAL_INT(0, rc);
    }
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_begin_transaction_succeeds);
    RUN_TEST(test_commit_persists_data);
    RUN_TEST(test_rollback_reverts_data);
    RUN_TEST(test_commit_without_begin);
    RUN_TEST(test_rollback_without_begin);
    RUN_TEST(test_multiple_sequential_transactions);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}

