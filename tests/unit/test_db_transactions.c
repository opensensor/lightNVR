/**
 * @file test_db_transactions.c
 * @brief Layer 2 — database transaction begin/commit/rollback
 *
 * Tests:
 *   - begin_transaction / commit_transaction persists data
 *   - begin_transaction / rollback_transaction reverts data
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <sqlite3.h>

#include "unity.h"
#include "database/db_core.h"
#include "database/db_transaction.h"
#include "database/db_recordings.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_transactions_test.db"

static recording_metadata_t make_rec(const char *path, time_t start) {
    recording_metadata_t m;
    memset(&m, 0, sizeof(m));
    strncpy(m.stream_name,  "cam1",      sizeof(m.stream_name)  - 1);
    strncpy(m.file_path,    path,        sizeof(m.file_path)    - 1);
    strncpy(m.codec,        "h264",      sizeof(m.codec)        - 1);
    strncpy(m.trigger_type, "scheduled", sizeof(m.trigger_type) - 1);
    m.start_time  = start;
    m.end_time    = start + 60;
    m.size_bytes  = 1024;
    m.is_complete = true;
    m.retention_tier = 2;
    m.retention_override_days = -1;
    m.disk_pressure_eligible = true;
    return m;
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

    recording_metadata_t m = make_rec("/rec/tx_commit.mp4", time(NULL));
    uint64_t id = add_recording_metadata(&m);
    TEST_ASSERT_NOT_EQUAL(0, id);

    rc = commit_transaction();
    TEST_ASSERT_EQUAL_INT(0, rc);

    /* Verify it exists after commit */
    recording_metadata_t got;
    rc = get_recording_metadata_by_id(id, &got);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("/rec/tx_commit.mp4", got.file_path);
}

/* begin + rollback reverts data */
void test_rollback_reverts_data(void) {
    int rc = begin_transaction();
    TEST_ASSERT_EQUAL_INT(0, rc);

    recording_metadata_t m = make_rec("/rec/tx_rollback.mp4", time(NULL));
    uint64_t id = add_recording_metadata(&m);
    TEST_ASSERT_NOT_EQUAL(0, id);

    rc = rollback_transaction();
    TEST_ASSERT_EQUAL_INT(0, rc);

    /* Verify it is gone after rollback */
    recording_metadata_t got;
    rc = get_recording_metadata_by_id(id, &got);
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

/* begin returns 0 on success */
void test_begin_transaction_succeeds(void) {
    int rc = begin_transaction();
    TEST_ASSERT_EQUAL_INT(0, rc);
    rollback_transaction();  /* clean up */
}

/* commit without begin does not crash */
void test_commit_without_begin(void) {
    /* Calling commit outside a transaction should either succeed (noop)
       or return a non-zero error — either way it must not crash */
    int rc = commit_transaction();
    (void)rc;  /* result undefined; just check no crash */
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

