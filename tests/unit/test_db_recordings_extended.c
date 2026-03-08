/**
 * @file test_db_recordings_extended.c
 * @brief Layer 2 — recording metadata CRUD, tiers, retention, storage bytes
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <sqlite3.h>

#include "unity.h"
#include "database/db_core.h"
#include "database/db_detections.h"
#include "database/db_recording_tags.h"
#include "database/db_recordings.h"
#include "video/detection_result.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_recordings_ext_test.db"

static recording_metadata_t make_rec(const char *stream, const char *path, time_t start) {
    recording_metadata_t m;
    memset(&m, 0, sizeof(m));
    strncpy(m.stream_name, stream, sizeof(m.stream_name) - 1);
    strncpy(m.file_path,   path,   sizeof(m.file_path)   - 1);
    strncpy(m.codec,       "h264", sizeof(m.codec)       - 1);
    strncpy(m.trigger_type, "scheduled", sizeof(m.trigger_type) - 1);
    m.start_time  = start;
    m.end_time    = start + 60;
    m.size_bytes  = 1024 * 1024;
    m.width = 1920; m.height = 1080; m.fps = 30;
    m.is_complete = true;
    m.protected   = false;
    m.retention_override_days = -1;
    m.retention_tier = RETENTION_TIER_STANDARD;
    m.disk_pressure_eligible = true;
    return m;
}

static void clear_recordings(void) {
    sqlite3_exec(get_db_handle(), "DELETE FROM detections;", NULL, NULL, NULL);
    sqlite3_exec(get_db_handle(), "DELETE FROM recording_tags;", NULL, NULL, NULL);
    sqlite3_exec(get_db_handle(), "DELETE FROM recordings;", NULL, NULL, NULL);
}

static detection_result_t make_detection_result(const char *label) {
    detection_result_t result;
    memset(&result, 0, sizeof(result));
    result.count = 1;
    strncpy(result.detections[0].label, label, sizeof(result.detections[0].label) - 1);
    result.detections[0].confidence = 0.9f;
    result.detections[0].width = 0.3f;
    result.detections[0].height = 0.3f;
    return result;
}

void setUp(void)    { clear_recordings(); }
void tearDown(void) {}

/* add / get by id */
void test_add_and_get_by_id(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/a.mp4", now);
    uint64_t id = add_recording_metadata(&m);
    TEST_ASSERT_NOT_EQUAL(0, id);

    recording_metadata_t got;
    int rc = get_recording_metadata_by_id(id, &got);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("cam1", got.stream_name);
}

/* update_recording_metadata */
void test_update_recording_metadata(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/b.mp4", now);
    uint64_t id = add_recording_metadata(&m);
    TEST_ASSERT_NOT_EQUAL(0, id);

    int rc = update_recording_metadata(id, now + 120, 2048 * 1024, true);
    TEST_ASSERT_EQUAL_INT(0, rc);

    recording_metadata_t got;
    get_recording_metadata_by_id(id, &got);
    TEST_ASSERT_EQUAL_INT(now + 120, got.end_time);
    TEST_ASSERT_TRUE(got.is_complete);
}

/* get_recording_metadata stream filter */
void test_get_recording_metadata_stream_filter(void) {
    time_t now = time(NULL);
    recording_metadata_t m1 = make_rec("cam1", "/rec/c1.mp4", now);
    recording_metadata_t m2 = make_rec("cam2", "/rec/c2.mp4", now);
    add_recording_metadata(&m1);
    add_recording_metadata(&m2);

    recording_metadata_t out[10];
    int n = get_recording_metadata(0, 0, "cam1", out, 10);
    TEST_ASSERT_EQUAL_INT(1, n);
    TEST_ASSERT_EQUAL_STRING("cam1", out[0].stream_name);
}

/* get_recording_metadata_by_path */
void test_get_recording_metadata_by_path(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/bypath.mp4", now);
    add_recording_metadata(&m);

    recording_metadata_t got;
    int rc = get_recording_metadata_by_path("/rec/bypath.mp4", &got);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("/rec/bypath.mp4", got.file_path);
}

/* get_recording_count */
void test_get_recording_count(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/cnt.mp4", now);
    add_recording_metadata(&m);
    int cnt = get_recording_count(0, 0, "cam1", 0, NULL, -1, NULL, 0, NULL, NULL);
    TEST_ASSERT_EQUAL_INT(1, cnt);
}

void test_get_recording_count_supports_multi_value_stream_tag_and_capture_filters(void) {
    time_t now = time(NULL);

    recording_metadata_t scheduled = make_rec("cam1", "/rec/multi-1.mp4", now);
    recording_metadata_t detection = make_rec("cam2", "/rec/multi-2.mp4", now + 60);
    recording_metadata_t manual = make_rec("cam3", "/rec/multi-3.mp4", now + 120);
    strncpy(detection.trigger_type, "detection", sizeof(detection.trigger_type) - 1);
    strncpy(manual.trigger_type, "manual", sizeof(manual.trigger_type) - 1);

    uint64_t scheduled_id = add_recording_metadata(&scheduled);
    uint64_t detection_id = add_recording_metadata(&detection);
    uint64_t manual_id = add_recording_metadata(&manual);

    TEST_ASSERT_EQUAL_INT(0, db_recording_tag_add(scheduled_id, "important"));
    TEST_ASSERT_EQUAL_INT(0, db_recording_tag_add(detection_id, "review"));
    TEST_ASSERT_EQUAL_INT(0, db_recording_tag_add(manual_id, "urgent"));

    int cnt = get_recording_count(0, 0, "cam1,cam3", 0, NULL, -1, NULL, 0,
                                  "important,urgent", "scheduled,manual");
    TEST_ASSERT_EQUAL_INT(2, cnt);
}

/* get_recording_metadata_paginated */
void test_get_recording_metadata_paginated(void) {
    time_t now = time(NULL);
    for (int i = 0; i < 5; i++) {
        char path[64];
        snprintf(path, sizeof(path), "/rec/page%d.mp4", i);
        recording_metadata_t m = make_rec("cam1", path, now - i * 100);
        add_recording_metadata(&m);
    }
    recording_metadata_t out[10];
    int n = get_recording_metadata_paginated(0, 0, "cam1", 0, NULL, -1,
                                             "start_time", "desc", out, 3, 0,
                                             NULL, 0, NULL, NULL);
    TEST_ASSERT_EQUAL_INT(3, n);
}

void test_get_recording_metadata_paginated_supports_multi_value_detection_labels_and_tags(void) {
    time_t now = time(NULL);

    recording_metadata_t rec1 = make_rec("cam1", "/rec/filter-1.mp4", now);
    recording_metadata_t rec2 = make_rec("cam2", "/rec/filter-2.mp4", now + 60);
    recording_metadata_t rec3 = make_rec("cam3", "/rec/filter-3.mp4", now + 120);

    uint64_t rec1_id = add_recording_metadata(&rec1);
    uint64_t rec2_id = add_recording_metadata(&rec2);
    uint64_t rec3_id = add_recording_metadata(&rec3);

    TEST_ASSERT_EQUAL_INT(0, db_recording_tag_add(rec1_id, "alpha"));
    TEST_ASSERT_EQUAL_INT(0, db_recording_tag_add(rec2_id, "beta"));
    TEST_ASSERT_EQUAL_INT(0, db_recording_tag_add(rec3_id, "gamma"));

    detection_result_t person = make_detection_result("person");
    detection_result_t car = make_detection_result("car");
    detection_result_t dog = make_detection_result("dog");
    TEST_ASSERT_EQUAL_INT(0, store_detections_in_db("cam1", &person, rec1.start_time + 1, rec1_id));
    TEST_ASSERT_EQUAL_INT(0, store_detections_in_db("cam2", &car, rec2.start_time + 1, rec2_id));
    TEST_ASSERT_EQUAL_INT(0, store_detections_in_db("cam3", &dog, rec3.start_time + 1, rec3_id));

    recording_metadata_t out[10];
    int n = get_recording_metadata_paginated(0, 0, NULL, 1, "person,car", -1,
                                             "id", "asc", out, 10, 0,
                                             NULL, 0, "alpha,beta", "scheduled");

    TEST_ASSERT_EQUAL_INT(2, n);
    TEST_ASSERT_EQUAL_STRING("cam1", out[0].stream_name);
    TEST_ASSERT_EQUAL_STRING("cam2", out[1].stream_name);
}

/* set_recording_retention_tier */
void test_set_recording_retention_tier(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/tier.mp4", now);
    uint64_t id = add_recording_metadata(&m);
    int rc = set_recording_retention_tier(id, RETENTION_TIER_CRITICAL);
    TEST_ASSERT_EQUAL_INT(0, rc);
    recording_metadata_t got;
    get_recording_metadata_by_id(id, &got);
    TEST_ASSERT_EQUAL_INT(RETENTION_TIER_CRITICAL, got.retention_tier);
}

/* set_recording_disk_pressure_eligible */
void test_set_recording_disk_pressure_eligible(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/dp.mp4", now);
    uint64_t id = add_recording_metadata(&m);
    int rc = set_recording_disk_pressure_eligible(id, false);
    TEST_ASSERT_EQUAL_INT(0, rc);
    recording_metadata_t got;
    get_recording_metadata_by_id(id, &got);
    TEST_ASSERT_FALSE(got.disk_pressure_eligible);
}

/* set_recording_retention_override */
void test_set_recording_retention_override(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/ov.mp4", now);
    uint64_t id = add_recording_metadata(&m);
    int rc = set_recording_retention_override(id, 90);
    TEST_ASSERT_EQUAL_INT(0, rc);
}

/* get_stream_storage_bytes */
void test_get_stream_storage_bytes(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam_sb", "/rec/sb.mp4", now);
    add_recording_metadata(&m);
    int64_t bytes = get_stream_storage_bytes("cam_sb");
    TEST_ASSERT_GREATER_THAN(0, bytes);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_add_and_get_by_id);
    RUN_TEST(test_update_recording_metadata);
    RUN_TEST(test_get_recording_metadata_stream_filter);
    RUN_TEST(test_get_recording_metadata_by_path);
    RUN_TEST(test_get_recording_count);
    RUN_TEST(test_get_recording_count_supports_multi_value_stream_tag_and_capture_filters);
    RUN_TEST(test_get_recording_metadata_paginated);
    RUN_TEST(test_get_recording_metadata_paginated_supports_multi_value_detection_labels_and_tags);
    RUN_TEST(test_set_recording_retention_tier);
    RUN_TEST(test_set_recording_disk_pressure_eligible);
    RUN_TEST(test_set_recording_retention_override);
    RUN_TEST(test_get_stream_storage_bytes);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}

