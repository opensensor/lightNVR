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
#include "utils/strings.h"
#include "database/db_core.h"
#include "database/db_detections.h"
#include "database/db_recordings.h"
#include "video/detection_result.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_detections_test.db"

static detection_result_t make_result(const char *label, float conf) {
    detection_result_t r;
    memset(&r, 0, sizeof(r));
    r.count = 1;
    safe_strcpy(r.detections[0].label, label, MAX_LABEL_LENGTH, 0);
    r.detections[0].confidence = conf;
    r.detections[0].x = 0.1f; r.detections[0].y = 0.1f;
    r.detections[0].width = 0.2f; r.detections[0].height = 0.2f;
    r.detections[0].track_id = -1;
    return r;
}

static void clear_detections(void) {
    sqlite3_exec(get_db_handle(), "DELETE FROM detections;", NULL, NULL, NULL);
}

typedef struct {
    int calls;
    int abort_after;
} query_progress_t;

static int abort_runaway_query(void *context) {
    query_progress_t *progress = (query_progress_t *)context;
    progress->calls++;
    return progress->calls > progress->abort_after;
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

    /* Create a real recording entry to satisfy the FK constraint */
    recording_metadata_t rec;
    memset(&rec, 0, sizeof(rec));
    safe_strcpy(rec.stream_name, "cam6",        sizeof(rec.stream_name),  0);
    safe_strcpy(rec.file_path,   "/tmp/t.mp4",  sizeof(rec.file_path),    0);
    safe_strcpy(rec.codec,       "h264",        sizeof(rec.codec),        0);
    safe_strcpy(rec.trigger_type,"detection",   sizeof(rec.trigger_type), 0);
    rec.start_time   = now - 10;
    rec.end_time     = now;
    rec.size_bytes   = 1000;
    rec.width        = 1920;
    rec.height       = 1080;
    rec.fps          = 30;
    rec.is_complete  = true;
    rec.retention_tier = RETENTION_TIER_STANDARD;

    uint64_t rec_id = add_recording_metadata(&rec);
    TEST_ASSERT_NOT_EQUAL(0, rec_id);

    int updated = update_detections_recording_id("cam6", rec_id, now - 10);
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

void test_external_motion_interval_spans_later_recording_segment(void) {
    time_t now = time(NULL);
    detection_result_t r = make_result("person", 1.0f);
    TEST_ASSERT_EQUAL_INT(0, store_external_motion_detections(
        "cam_interval", &r, now - 120, 0));

    detection_interval_t active_intervals[4];
    int active_count = get_external_motion_detection_intervals(
        "cam_interval", now - 60, now, active_intervals, 4);
    TEST_ASSERT_EQUAL_INT(1, active_count);
    TEST_ASSERT_TRUE(active_intervals[0].end_time >= now);

    TEST_ASSERT_EQUAL_INT(1, close_external_motion_detections(
        "cam_interval", now - 10));

    /* The detection began before this segment but remained active inside it. */
    TEST_ASSERT_EQUAL_INT(1, has_detections_in_time_range(
        "cam_interval", now - 60, now));

    detection_result_t out;
    TEST_ASSERT_EQUAL_INT(1, get_detections_from_db_time_range(
        "cam_interval", &out, 0, now - 60, now));
    TEST_ASSERT_EQUAL_STRING("person", out.detections[0].label);

    detection_label_summary_t labels[4];
    TEST_ASSERT_EQUAL_INT(1, get_detection_labels_summary(
        "cam_interval", now - 60, now, labels, 4));
    TEST_ASSERT_EQUAL_STRING("person", labels[0].label);

    time_t starts[MAX_DETECTIONS] = {0};
    time_t ends[MAX_DETECTIONS] = {0};
    TEST_ASSERT_EQUAL_INT(0, get_detection_time_ranges(
        "cam_interval", &out, starts, ends, 0, now - 60, now));
    TEST_ASSERT_EQUAL_INT64(now - 120, starts[0]);
    TEST_ASSERT_EQUAL_INT64(now - 10, ends[0]);

    detection_interval_t intervals[4];
    int count = get_external_motion_detection_intervals(
        "cam_interval", now - 60, now, intervals, 4);
    TEST_ASSERT_EQUAL_INT(1, count);
    TEST_ASSERT_EQUAL_INT64(now - 120, intervals[0].start_time);
    TEST_ASSERT_EQUAL_INT64(now - 10, intervals[0].end_time);
}

/* Regression for #491: point-detection lookups must seek to the beginning of
 * the requested range instead of scanning the stream's entire history. */
void test_recent_detection_queries_do_not_scan_full_stream_history(void) {
    sqlite3 *db = get_db_handle();
    time_t now = time(NULL);
    sqlite3_stmt *stmt = NULL;

    TEST_ASSERT_EQUAL_INT(SQLITE_OK,
        sqlite3_exec(db, "BEGIN TRANSACTION;", NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(db,
        "INSERT INTO detections "
        "(stream_name, timestamp, label, confidence, source, event_end_time) "
        "VALUES ('cam_large', ?, 'old', 0.5, '', ?);",
        -1, &stmt, NULL));

    for (int i = 0; i < 20000; i++) {
        sqlite3_int64 timestamp = (sqlite3_int64)(now - 100000 - i);
        sqlite3_bind_int64(stmt, 1, timestamp);
        sqlite3_bind_int64(stmt, 2, timestamp);
        TEST_ASSERT_EQUAL_INT(SQLITE_DONE, sqlite3_step(stmt));
        sqlite3_reset(stmt);
        sqlite3_clear_bindings(stmt);
    }
    sqlite3_finalize(stmt);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK,
        sqlite3_exec(db, "COMMIT;", NULL, NULL, NULL));

    detection_result_t recent = make_result("person", 0.9f);
    TEST_ASSERT_EQUAL_INT(0, store_detections_in_db(
        "cam_large", &recent, now - 5, 0));

    query_progress_t progress = {.calls = 0, .abort_after = 200};
    sqlite3_progress_handler(db, 100, abort_runaway_query, &progress);

    detection_label_summary_t labels[4];
    int label_count = get_detection_labels_summary(
        "cam_large", now - 30, now, labels, 4);

    sqlite3_progress_handler(db, 0, NULL, NULL);
    TEST_ASSERT_EQUAL_INT(1, label_count);
    TEST_ASSERT_EQUAL_STRING("person", labels[0].label);
    TEST_ASSERT_LESS_OR_EQUAL_INT(progress.abort_after, progress.calls);

    progress.calls = 0;
    sqlite3_progress_handler(db, 100, abort_runaway_query, &progress);

    detection_result_t out;
    int detection_count = get_detections_from_db(
        "cam_large", &out, 30);

    sqlite3_progress_handler(db, 0, NULL, NULL);
    TEST_ASSERT_EQUAL_INT(1, detection_count);
    TEST_ASSERT_EQUAL_STRING("person", out.detections[0].label);
    TEST_ASSERT_LESS_OR_EQUAL_INT(progress.abort_after, progress.calls);
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
    RUN_TEST(test_external_motion_interval_spans_later_recording_segment);
    RUN_TEST(test_recent_detection_queries_do_not_scan_full_stream_history);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
