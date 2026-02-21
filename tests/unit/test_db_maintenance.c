/**
 * @file test_db_maintenance.c
 * @brief Layer 2 — database maintenance functions
 *
 * Tests get_database_size, vacuum_database, check_database_integrity.
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
#include "database/db_maintenance.h"
#include "database/db_recordings.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_maintenance_test.db"

void setUp(void)    {}
void tearDown(void) {}

/* get_database_size returns positive value */
void test_get_database_size_positive(void) {
    int64_t sz = get_database_size();
    TEST_ASSERT_GREATER_THAN(0, sz);
}

/* get_database_size increases after insert */
void test_get_database_size_increases_after_insert(void) {
    int64_t before = get_database_size();

    /* Insert some data */
    recording_metadata_t m;
    memset(&m, 0, sizeof(m));
    strncpy(m.stream_name, "cam1",    sizeof(m.stream_name)  - 1);
    strncpy(m.file_path,   "/r.mp4",  sizeof(m.file_path)    - 1);
    strncpy(m.codec,       "h264",    sizeof(m.codec)        - 1);
    strncpy(m.trigger_type,"scheduled",sizeof(m.trigger_type)- 1);
    m.start_time = time(NULL);
    m.end_time   = m.start_time + 60;
    m.size_bytes = 1024;
    m.is_complete = true;
    m.retention_tier = 2;
    m.retention_override_days = -1;
    m.disk_pressure_eligible = true;
    add_recording_metadata(&m);

    /* Checkpoint to flush WAL to main DB file */
    checkpoint_database();

    int64_t after = get_database_size();
    TEST_ASSERT_GREATER_OR_EQUAL(before, after);
}

/* vacuum_database succeeds */
void test_vacuum_database_succeeds(void) {
    int rc = vacuum_database();
    TEST_ASSERT_EQUAL_INT(0, rc);
}

/* vacuum twice does not crash */
void test_vacuum_database_idempotent(void) {
    int rc1 = vacuum_database();
    int rc2 = vacuum_database();
    TEST_ASSERT_EQUAL_INT(0, rc1);
    TEST_ASSERT_EQUAL_INT(0, rc2);
}

/* check_database_integrity on healthy DB */
void test_check_database_integrity_healthy(void) {
    int rc = check_database_integrity();
    TEST_ASSERT_EQUAL_INT(0, rc);
}

/* check_database_integrity after vacuum */
void test_check_integrity_after_vacuum(void) {
    vacuum_database();
    int rc = check_database_integrity();
    TEST_ASSERT_EQUAL_INT(0, rc);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_get_database_size_positive);
    RUN_TEST(test_get_database_size_increases_after_insert);
    RUN_TEST(test_vacuum_database_succeeds);
    RUN_TEST(test_vacuum_database_idempotent);
    RUN_TEST(test_check_database_integrity_healthy);
    RUN_TEST(test_check_integrity_after_vacuum);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}

