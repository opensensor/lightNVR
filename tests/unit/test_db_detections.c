/**
 * @file test_db_detections.c
 * @brief Layer 2 — detection storage and retrieval via SQLite
 *
 * Tests store_detections_in_db, get_detections_from_db,
 * get_detections_from_db_time_range, has_detections_in_time_range,
 * delete_old_detections, get_detection_labels_summary,
 * and update_detections_recording_id.
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
#include "database/db_detections.h"
#include "video/detection_result.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_detections_test.db"

static detection_result_t make_result(const char *label, float conf) {
    detection_result_t r;
    memset(&r, 0, sizeof(r));
    r.count = 1;
    strncpy(r.detections[0].label, label, MAX_LABEL_LENGTH - 1);
    r.detections[0].confidence = conf;
    r.detections[0].x = 0.1f; r.detections[0].y = 0.1f;
    r.detections[0].width = 0.2f; r.detections[0].height = 0.2f;
    r.detections[0].track_id = -1;
    return r;
}

static void clear_detections(void) {
    sqlite3_exec(get_db_handle(), "DELETE FROM detections;", NULL, NULL, NULL);
}

void setUp(void)    { clear_detections(); }
void tearDown(void) {}

/* store and retrieve */
void test_store_and_get_detections(void) {
    detection_result_t r = make_result("person", 0.9f);
    time_t now = time(NULL);
    int rc = store_detections_in_db("cam1", &r, now, 0);
    TEST_ASSERT_EQUAL_INT(0, rc);

    detection_result_t out;
    int n = get_detections_from_db("cam1", &out, 0);
    TEST_ASSERT_GREATER_THAN(0, n);
}

/* time range filter */
void test_get_detections_time_range(void) {
    time_t now = time(NULL);
    detection_result_t r = make_result("car", 0.8f);
    store_detections_in_db("cam2", &r, now - 100, 0);

    detection_result_t out;
    int n = get_detections_from_db_time_range("cam2", &out, 0,
                                               now - 200, now);
    TEST_ASSERT_GREATER_THAN(0, n);
}

/* has_detections_in_time_range positive */
void test_has_detections_in_time_range_found(void) {
    time_t now = time(NULL);
    detection_result_t r = make_result("dog", 0.7f);
    store_detections_in_db("cam3", &r, now - 50, 0);

    int rc = has_detections_in_time_range("cam3", now - 100, now);
    TEST_ASSERT_EQUAL_INT(1, rc);
}

/* has_detections_in_time_range negative */
void test_has_detections_in_time_range_not_found(void) {
    int rc = has_detections_in_time_range("cam_empty", time(NULL) - 100, time(NULL));
    TEST_ASSERT_EQUAL_INT(0, rc);
}

/* delete_old_detections */
void test_delete_old_detections(void) {
    time_t now = time(NULL);
    detection_result_t r = make_result("cat", 0.6f);
    /* Insert with a very old timestamp by directly inserting via raw SQL */
    store_detections_in_db("cam4", &r, now - 10000, 0);

    int deleted = delete_old_detections(100); /* max_age = 100 seconds */
    TEST_ASSERT_GREATER_OR_EQUAL(0, deleted);
}

/* get_detection_labels_summary */
void test_get_detection_labels_summary(void) {
    time_t now = time(NULL);
    detection_result_t r1 = make_result("person", 0.9f);
    detection_result_t r2 = make_result("car",    0.8f);
    store_detections_in_db("cam5", &r1, now - 10, 0);
    store_detections_in_db("cam5", &r2, now - 5,  0);

    detection_label_summary_t labels[MAX_DETECTION_LABELS];
    int n = get_detection_labels_summary("cam5", now - 100, now,
                                         labels, MAX_DETECTION_LABELS);
    TEST_ASSERT_GREATER_THAN(0, n);
}

/* update_detections_recording_id */
void test_update_detections_recording_id(void) {
    time_t now = time(NULL);
    detection_result_t r = make_result("person", 0.95f);
    store_detections_in_db("cam6", &r, now - 5, 0);

    int updated = update_detections_recording_id("cam6", 42, now - 10);
    TEST_ASSERT_GREATER_OR_EQUAL(0, updated);
}

/* max detections boundary */
void test_store_max_detections(void) {
    detection_result_t r;
    memset(&r, 0, sizeof(r));
    r.count = MAX_DETECTIONS;
    for (int i = 0; i < MAX_DETECTIONS; i++) {
        snprintf(r.detections[i].label, MAX_LABEL_LENGTH, "obj%d", i);
        r.detections[i].confidence = 0.5f;
        r.detections[i].track_id = -1;
    }
    int rc = store_detections_in_db("cam7", &r, time(NULL), 0);
    TEST_ASSERT_EQUAL_INT(0, rc);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_store_and_get_detections);
    RUN_TEST(test_get_detections_time_range);
    RUN_TEST(test_has_detections_in_time_range_found);
    RUN_TEST(test_has_detections_in_time_range_not_found);
    RUN_TEST(test_delete_old_detections);
    RUN_TEST(test_get_detection_labels_summary);
    RUN_TEST(test_update_detections_recording_id);
    RUN_TEST(test_store_max_detections);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}

