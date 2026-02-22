/**
 * @file test_zone_filter.c
 * @brief Layer 2 Unity tests for video/zone_filter.c
 *
 * Tests:
 *   filter_detections_by_zones     — zone polygon + class/confidence gate
 *   filter_detections_by_stream_objects — include/exclude object list
 *
 * Both functions query SQLite (zones / streams tables) so we use a real DB.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <sqlite3.h>

#include "unity.h"
#include "database/db_core.h"
#include "database/db_zones.h"
#include "database/db_streams.h"
#include "video/zone_filter.h"
#include "video/detection_result.h"
#include "core/config.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_zone_filter_test.db"

/* ---- helpers ---- */

static detection_result_t make_result_1det(const char *label, float x, float y,
                                           float w, float h, float conf) {
    detection_result_t r;
    memset(&r, 0, sizeof(r));
    r.count = 1;
    strncpy(r.detections[0].label, label, MAX_LABEL_LENGTH - 1);
    r.detections[0].x          = x;
    r.detections[0].y          = y;
    r.detections[0].width      = w;
    r.detections[0].height     = h;
    r.detections[0].confidence = conf;
    r.detections[0].track_id   = -1;
    return r;
}

/* Square zone that covers [0.0-0.5] × [0.0-0.5] */
static detection_zone_t make_square_zone(const char *stream, const char *name,
                                         bool enabled, const char *classes,
                                         float min_conf) {
    detection_zone_t z;
    memset(&z, 0, sizeof(z));
    strncpy(z.id,          "zone-test-1",  sizeof(z.id) - 1);
    strncpy(z.stream_name, stream,         sizeof(z.stream_name) - 1);
    strncpy(z.name,        name,           sizeof(z.name) - 1);
    z.enabled       = enabled;
    z.min_confidence = min_conf;
    if (classes)
        strncpy(z.filter_classes, classes, sizeof(z.filter_classes) - 1);
    /* Square polygon: TL→TR→BR→BL */
    z.polygon[0] = (zone_point_t){0.0f, 0.0f};
    z.polygon[1] = (zone_point_t){0.5f, 0.0f};
    z.polygon[2] = (zone_point_t){0.5f, 0.5f};
    z.polygon[3] = (zone_point_t){0.0f, 0.5f};
    z.polygon_count = 4;
    return z;
}

static void ensure_stream(const char *name) {
    stream_config_t s;
    memset(&s, 0, sizeof(s));
    strncpy(s.name, name, sizeof(s.name) - 1);
    strncpy(s.url, "rtsp://localhost/test", sizeof(s.url) - 1);
    s.enabled  = true;
    s.width    = 1920;
    s.height   = 1080;
    s.fps      = 30;
    s.protocol = STREAM_PROTOCOL_TCP;
    add_stream_config(&s);
}

static void clear_all(void) {
    sqlite3 *db = get_db_handle();
    sqlite3_exec(db, "DELETE FROM detection_zones;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM streams;", NULL, NULL, NULL);
}

/* ---- Unity boilerplate ---- */
void setUp(void)    { clear_all(); }
void tearDown(void) {}

/* ================================================================
 * filter_detections_by_zones — no zones configured
 * ================================================================ */

void test_no_zones_allows_all_detections(void) {
    detection_result_t r = make_result_1det("person", 0.1f, 0.1f, 0.1f, 0.1f, 0.9f);
    int rc = filter_detections_by_zones("cam_nozone", &r);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_INT(1, r.count); /* unchanged */
}

void test_empty_result_returns_unchanged(void) {
    detection_result_t r;
    memset(&r, 0, sizeof(r));
    r.count = 0;
    int rc = filter_detections_by_zones("cam_empty", &r);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_INT(0, r.count);
}

void test_null_params_return_error(void) {
    detection_result_t r;
    memset(&r, 0, sizeof(r));
    TEST_ASSERT_EQUAL_INT(-1, filter_detections_by_zones(NULL, &r));
    TEST_ASSERT_EQUAL_INT(-1, filter_detections_by_zones("s", NULL));
}

/* ================================================================
 * filter_detections_by_zones — detection inside zone passes
 * ================================================================ */

void test_detection_inside_zone_passes(void) {
    ensure_stream("cam_in");
    detection_zone_t z = make_square_zone("cam_in", "zone1", true, NULL, 0.0f);
    save_detection_zones("cam_in", &z, 1);

    /* Center at (0.15, 0.15) — well inside [0,0.5]×[0,0.5] */
    detection_result_t r = make_result_1det("car", 0.1f, 0.1f, 0.1f, 0.1f, 0.8f);
    int rc = filter_detections_by_zones("cam_in", &r);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_INT(1, r.count);
}

/* ================================================================
 * filter_detections_by_zones — detection outside zone is rejected
 * ================================================================ */

void test_detection_outside_zone_filtered(void) {
    ensure_stream("cam_out");
    detection_zone_t z = make_square_zone("cam_out", "zone2", true, NULL, 0.0f);
    save_detection_zones("cam_out", &z, 1);

    /* Center at (0.75, 0.75) — outside [0,0.5]×[0,0.5] */
    detection_result_t r = make_result_1det("dog", 0.7f, 0.7f, 0.1f, 0.1f, 0.8f);
    int rc = filter_detections_by_zones("cam_out", &r);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_INT(0, r.count);
}

/* ================================================================
 * filter_detections_by_zones — disabled zone allows all
 * ================================================================ */

void test_disabled_zone_allows_all(void) {
    ensure_stream("cam_dis");
    detection_zone_t z = make_square_zone("cam_dis", "zone3", false, NULL, 0.0f);
    save_detection_zones("cam_dis", &z, 1);

    detection_result_t r = make_result_1det("person", 0.8f, 0.8f, 0.1f, 0.1f, 0.9f);
    int rc = filter_detections_by_zones("cam_dis", &r);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_INT(1, r.count);
}

/* ================================================================
 * filter_detections_by_zones — class filter
 * ================================================================ */

void test_class_filter_allows_matching_class(void) {
    ensure_stream("cam_cls");
    detection_zone_t z = make_square_zone("cam_cls", "zone4", true, "person", 0.0f);
    save_detection_zones("cam_cls", &z, 1);

    detection_result_t r = make_result_1det("person", 0.1f, 0.1f, 0.1f, 0.1f, 0.9f);
    filter_detections_by_zones("cam_cls", &r);
    TEST_ASSERT_EQUAL_INT(1, r.count);
}

void test_class_filter_rejects_wrong_class(void) {
    ensure_stream("cam_cls2");
    detection_zone_t z = make_square_zone("cam_cls2", "zone5", true, "person", 0.0f);
    save_detection_zones("cam_cls2", &z, 1);

    /* "car" is not in the filter list */
    detection_result_t r = make_result_1det("car", 0.1f, 0.1f, 0.1f, 0.1f, 0.9f);
    filter_detections_by_zones("cam_cls2", &r);
    TEST_ASSERT_EQUAL_INT(0, r.count);
}

/* ================================================================
 * filter_detections_by_zones — confidence threshold
 * ================================================================ */

void test_confidence_filter_rejects_low_confidence(void) {
    ensure_stream("cam_conf");
    detection_zone_t z = make_square_zone("cam_conf", "zone6", true, NULL, 0.8f);
    save_detection_zones("cam_conf", &z, 1);

    detection_result_t r = make_result_1det("person", 0.1f, 0.1f, 0.1f, 0.1f, 0.5f);
    filter_detections_by_zones("cam_conf", &r);
    TEST_ASSERT_EQUAL_INT(0, r.count);
}

void test_confidence_filter_passes_sufficient_confidence(void) {
    ensure_stream("cam_conf2");
    detection_zone_t z = make_square_zone("cam_conf2", "zone7", true, NULL, 0.8f);
    save_detection_zones("cam_conf2", &z, 1);

    detection_result_t r = make_result_1det("person", 0.1f, 0.1f, 0.1f, 0.1f, 0.95f);
    filter_detections_by_zones("cam_conf2", &r);
    TEST_ASSERT_EQUAL_INT(1, r.count);
}

/* ================================================================
 * filter_detections_by_stream_objects — no filter configured
 * ================================================================ */

void test_stream_object_filter_none_allows_all(void) {
    ensure_stream("cam_nofilter");
    detection_result_t r = make_result_1det("cat", 0.1f, 0.1f, 0.1f, 0.1f, 0.9f);
    int rc = filter_detections_by_stream_objects("cam_nofilter", &r);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_INT(1, r.count);
}

/* ================================================================
 * filter_detections_by_stream_objects — include list
 * ================================================================ */

void test_stream_object_include_keeps_matching_label(void) {
    /* Insert stream with include filter */
    stream_config_t s;
    memset(&s, 0, sizeof(s));
    strncpy(s.name, "cam_inc", sizeof(s.name) - 1);
    strncpy(s.url, "rtsp://localhost/inc", sizeof(s.url) - 1);
    s.enabled = true; s.width = 1920; s.height = 1080; s.fps = 30;
    s.protocol = STREAM_PROTOCOL_TCP;
    strncpy(s.detection_object_filter, "include", sizeof(s.detection_object_filter) - 1);
    strncpy(s.detection_object_filter_list, "person,car", sizeof(s.detection_object_filter_list) - 1);
    add_stream_config(&s);

    detection_result_t r = make_result_1det("person", 0.1f, 0.1f, 0.1f, 0.1f, 0.9f);
    filter_detections_by_stream_objects("cam_inc", &r);
    TEST_ASSERT_EQUAL_INT(1, r.count);
}

void test_stream_object_include_drops_unmatched_label(void) {
    stream_config_t s;
    memset(&s, 0, sizeof(s));
    strncpy(s.name, "cam_inc2", sizeof(s.name) - 1);
    strncpy(s.url, "rtsp://localhost/inc2", sizeof(s.url) - 1);
    s.enabled = true; s.width = 1920; s.height = 1080; s.fps = 30;
    s.protocol = STREAM_PROTOCOL_TCP;
    strncpy(s.detection_object_filter, "include", sizeof(s.detection_object_filter) - 1);
    strncpy(s.detection_object_filter_list, "person,car", sizeof(s.detection_object_filter_list) - 1);
    add_stream_config(&s);

    detection_result_t r = make_result_1det("bicycle", 0.1f, 0.1f, 0.1f, 0.1f, 0.9f);
    filter_detections_by_stream_objects("cam_inc2", &r);
    TEST_ASSERT_EQUAL_INT(0, r.count);
}

/* ================================================================
 * filter_detections_by_stream_objects — exclude list
 * ================================================================ */

void test_stream_object_exclude_drops_matching_label(void) {
    stream_config_t s;
    memset(&s, 0, sizeof(s));
    strncpy(s.name, "cam_exc", sizeof(s.name) - 1);
    strncpy(s.url, "rtsp://localhost/exc", sizeof(s.url) - 1);
    s.enabled = true; s.width = 1920; s.height = 1080; s.fps = 30;
    s.protocol = STREAM_PROTOCOL_TCP;
    strncpy(s.detection_object_filter, "exclude", sizeof(s.detection_object_filter) - 1);
    strncpy(s.detection_object_filter_list, "cat", sizeof(s.detection_object_filter_list) - 1);
    add_stream_config(&s);

    detection_result_t r = make_result_1det("cat", 0.1f, 0.1f, 0.1f, 0.1f, 0.9f);
    filter_detections_by_stream_objects("cam_exc", &r);
    TEST_ASSERT_EQUAL_INT(0, r.count);
}

void test_stream_object_exclude_keeps_unmatched_label(void) {
    stream_config_t s;
    memset(&s, 0, sizeof(s));
    strncpy(s.name, "cam_exc2", sizeof(s.name) - 1);
    strncpy(s.url, "rtsp://localhost/exc2", sizeof(s.url) - 1);
    s.enabled = true; s.width = 1920; s.height = 1080; s.fps = 30;
    s.protocol = STREAM_PROTOCOL_TCP;
    strncpy(s.detection_object_filter, "exclude", sizeof(s.detection_object_filter) - 1);
    strncpy(s.detection_object_filter_list, "cat", sizeof(s.detection_object_filter_list) - 1);
    add_stream_config(&s);

    detection_result_t r = make_result_1det("dog", 0.1f, 0.1f, 0.1f, 0.1f, 0.9f);
    filter_detections_by_stream_objects("cam_exc2", &r);
    TEST_ASSERT_EQUAL_INT(1, r.count);
}

/* ================================================================
 * main
 * ================================================================ */

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }

    UNITY_BEGIN();
    RUN_TEST(test_no_zones_allows_all_detections);
    RUN_TEST(test_empty_result_returns_unchanged);
    RUN_TEST(test_null_params_return_error);
    RUN_TEST(test_detection_inside_zone_passes);
    RUN_TEST(test_detection_outside_zone_filtered);
    RUN_TEST(test_disabled_zone_allows_all);
    RUN_TEST(test_class_filter_allows_matching_class);
    RUN_TEST(test_class_filter_rejects_wrong_class);
    RUN_TEST(test_confidence_filter_rejects_low_confidence);
    RUN_TEST(test_confidence_filter_passes_sufficient_confidence);
    RUN_TEST(test_stream_object_filter_none_allows_all);
    RUN_TEST(test_stream_object_include_keeps_matching_label);
    RUN_TEST(test_stream_object_include_drops_unmatched_label);
    RUN_TEST(test_stream_object_exclude_drops_matching_label);
    RUN_TEST(test_stream_object_exclude_keeps_unmatched_label);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}

