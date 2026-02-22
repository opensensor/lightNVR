/**
 * @file test_batch_delete_progress.c
 * @brief Layer 2 Unity tests for web/batch_delete_progress.c
 *
 * Tests the full state-machine lifecycle of the batch delete progress
 * tracker: init, create, update, complete, error, get, delete, cleanup.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "unity.h"
#include "web/batch_delete_progress.h"

/* ---- Unity boilerplate ---- */
void setUp(void) {
    /* Re-initialise before every test so state is clean */
    batch_delete_progress_cleanup();
    batch_delete_progress_init();
}

void tearDown(void) {
    batch_delete_progress_cleanup();
}

/* ================================================================
 * init / cleanup
 * ================================================================ */

void test_init_succeeds(void) {
    /* Already initialised in setUp; calling again should be idempotent */
    int rc = batch_delete_progress_init();
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_operations_fail_when_not_initialized(void) {
    batch_delete_progress_cleanup();  /* explicitly uninitialise */

    char job_id[64];
    TEST_ASSERT_EQUAL_INT(-1, batch_delete_progress_create_job(5, job_id));
    TEST_ASSERT_EQUAL_INT(-1, batch_delete_progress_update("x", 1, 1, 0, NULL));
    TEST_ASSERT_EQUAL_INT(-1, batch_delete_progress_complete("x", 5, 0));
    TEST_ASSERT_EQUAL_INT(-1, batch_delete_progress_error("x", "oops"));

    batch_delete_progress_t out;
    TEST_ASSERT_EQUAL_INT(-1, batch_delete_progress_get("x", &out));
    TEST_ASSERT_EQUAL_INT(-1, batch_delete_progress_delete("x"));

    /* Re-init for tearDown */
    batch_delete_progress_init();
}

/* ================================================================
 * create_job
 * ================================================================ */

void test_create_job_returns_zero_and_fills_id(void) {
    char job_id[64] = {0};
    int rc = batch_delete_progress_create_job(10, job_id);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_TRUE(strlen(job_id) > 0);
}

void test_create_job_null_out_param_fails(void) {
    int rc = batch_delete_progress_create_job(10, NULL);
    TEST_ASSERT_EQUAL_INT(-1, rc);
}

void test_created_job_starts_pending(void) {
    char job_id[64];
    batch_delete_progress_create_job(7, job_id);

    batch_delete_progress_t info;
    int rc = batch_delete_progress_get(job_id, &info);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_INT(BATCH_DELETE_STATUS_PENDING, info.status);
    TEST_ASSERT_EQUAL_INT(7, info.total);
    TEST_ASSERT_EQUAL_INT(0, info.current);
    TEST_ASSERT_TRUE(info.is_active);
}

/* ================================================================
 * update
 * ================================================================ */

void test_update_sets_running_status(void) {
    char job_id[64];
    batch_delete_progress_create_job(10, job_id);

    int rc = batch_delete_progress_update(job_id, 3, 3, 0, "Processing...");
    TEST_ASSERT_EQUAL_INT(0, rc);

    batch_delete_progress_t info;
    batch_delete_progress_get(job_id, &info);
    TEST_ASSERT_EQUAL_INT(BATCH_DELETE_STATUS_RUNNING, info.status);
    TEST_ASSERT_EQUAL_INT(3, info.current);
    TEST_ASSERT_EQUAL_INT(3, info.succeeded);
    TEST_ASSERT_EQUAL_INT(0, info.failed);
    TEST_ASSERT_EQUAL_STRING("Processing...", info.status_message);
}

void test_update_unknown_job_fails(void) {
    int rc = batch_delete_progress_update("no-such-id", 1, 1, 0, NULL);
    TEST_ASSERT_EQUAL_INT(-1, rc);
}

/* ================================================================
 * complete
 * ================================================================ */

void test_complete_sets_status_and_counts(void) {
    char job_id[64];
    batch_delete_progress_create_job(5, job_id);
    batch_delete_progress_update(job_id, 3, 3, 0, NULL);

    int rc = batch_delete_progress_complete(job_id, 5, 0);
    TEST_ASSERT_EQUAL_INT(0, rc);

    batch_delete_progress_t info;
    batch_delete_progress_get(job_id, &info);
    TEST_ASSERT_EQUAL_INT(BATCH_DELETE_STATUS_COMPLETE, info.status);
    TEST_ASSERT_EQUAL_INT(5, info.succeeded);
    TEST_ASSERT_EQUAL_INT(0, info.failed);
    TEST_ASSERT_EQUAL_INT(5, info.current); /* set to total on complete */
}

/* ================================================================
 * error
 * ================================================================ */

void test_error_sets_error_status_and_message(void) {
    char job_id[64];
    batch_delete_progress_create_job(5, job_id);

    int rc = batch_delete_progress_error(job_id, "disk full");
    TEST_ASSERT_EQUAL_INT(0, rc);

    batch_delete_progress_t info;
    batch_delete_progress_get(job_id, &info);
    TEST_ASSERT_EQUAL_INT(BATCH_DELETE_STATUS_ERROR, info.status);
    TEST_ASSERT_EQUAL_STRING("disk full", info.error_message);
}

/* ================================================================
 * get / delete
 * ================================================================ */

void test_get_unknown_job_fails(void) {
    batch_delete_progress_t info;
    int rc = batch_delete_progress_get("ghost-id", &info);
    TEST_ASSERT_EQUAL_INT(-1, rc);
}

void test_delete_removes_job(void) {
    char job_id[64];
    batch_delete_progress_create_job(3, job_id);

    int rc = batch_delete_progress_delete(job_id);
    TEST_ASSERT_EQUAL_INT(0, rc);

    batch_delete_progress_t info;
    rc = batch_delete_progress_get(job_id, &info);
    TEST_ASSERT_EQUAL_INT(-1, rc); /* should be gone */
}

void test_delete_unknown_job_fails(void) {
    int rc = batch_delete_progress_delete("nonexistent");
    TEST_ASSERT_EQUAL_INT(-1, rc);
}

/* ================================================================
 * multiple jobs
 * ================================================================ */

void test_multiple_jobs_independent(void) {
    char id1[64], id2[64];
    batch_delete_progress_create_job(10, id1);
    batch_delete_progress_create_job(20, id2);

    batch_delete_progress_complete(id1, 10, 0);
    batch_delete_progress_error(id2, "timeout");

    batch_delete_progress_t info1, info2;
    batch_delete_progress_get(id1, &info1);
    batch_delete_progress_get(id2, &info2);

    TEST_ASSERT_EQUAL_INT(BATCH_DELETE_STATUS_COMPLETE, info1.status);
    TEST_ASSERT_EQUAL_INT(BATCH_DELETE_STATUS_ERROR,    info2.status);
    TEST_ASSERT_EQUAL_INT(10, info1.total);
    TEST_ASSERT_EQUAL_INT(20, info2.total);
}

/* ================================================================
 * main
 * ================================================================ */

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_init_succeeds);
    RUN_TEST(test_operations_fail_when_not_initialized);
    RUN_TEST(test_create_job_returns_zero_and_fills_id);
    RUN_TEST(test_create_job_null_out_param_fails);
    RUN_TEST(test_created_job_starts_pending);
    RUN_TEST(test_update_sets_running_status);
    RUN_TEST(test_update_unknown_job_fails);
    RUN_TEST(test_complete_sets_status_and_counts);
    RUN_TEST(test_error_sets_error_status_and_message);
    RUN_TEST(test_get_unknown_job_fails);
    RUN_TEST(test_delete_removes_job);
    RUN_TEST(test_delete_unknown_job_fails);
    RUN_TEST(test_multiple_jobs_independent);
    return UNITY_END();
}

