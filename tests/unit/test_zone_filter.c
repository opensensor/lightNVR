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

#include <pthread.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <sqlite3.h>

#include "unity.h"
#include "core/config.h"
#include "utils/strings.h"
#include "database/db_core.h"
#include "database/db_zones.h"
#include "database/db_streams.h"
#include "video/zone_filter.h"
#include "video/detection_result.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_zone_filter_test.db"

/* ---- helpers ---- */

static detection_result_t make_result_1det(const char *label, float x, float y,
                                           float w, float h, float conf) {
    detection_result_t r;
    memset(&r, 0, sizeof(r));
    r.count = 1;
    safe_strcpy(r.detections[0].label, label, MAX_LABEL_LENGTH, 0);
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
    safe_strcpy(z.id,          "zone-test-1",  sizeof(z.id), 0);
    safe_strcpy(z.stream_name, stream,         sizeof(z.stream_name), 0);
    safe_strcpy(z.name,        name,           sizeof(z.name), 0);
    z.enabled       = enabled;
    z.min_confidence = min_conf;
    if (classes)
        safe_strcpy(z.filter_classes, classes, sizeof(z.filter_classes), 0);
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
    safe_strcpy(s.name, name, sizeof(s.name), 0);
    safe_strcpy(s.url, "rtsp://localhost/test", sizeof(s.url), 0);
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
    safe_strcpy(s.name, "cam_inc", sizeof(s.name), 0);
    safe_strcpy(s.url, "rtsp://localhost/inc", sizeof(s.url), 0);
    s.enabled = true; s.width = 1920; s.height = 1080; s.fps = 30;
    s.protocol = STREAM_PROTOCOL_TCP;
    safe_strcpy(s.detection_object_filter, "include", sizeof(s.detection_object_filter), 0);
    safe_strcpy(s.detection_object_filter_list, "person,car", sizeof(s.detection_object_filter_list), 0);
    add_stream_config(&s);

    detection_result_t r = make_result_1det("person", 0.1f, 0.1f, 0.1f, 0.1f, 0.9f);
    filter_detections_by_stream_objects("cam_inc", &r);
    TEST_ASSERT_EQUAL_INT(1, r.count);
}

void test_stream_object_include_drops_unmatched_label(void) {
    stream_config_t s;
    memset(&s, 0, sizeof(s));
    safe_strcpy(s.name, "cam_inc2", sizeof(s.name), 0);
    safe_strcpy(s.url, "rtsp://localhost/inc2", sizeof(s.url), 0);
    s.enabled = true; s.width = 1920; s.height = 1080; s.fps = 30;
    s.protocol = STREAM_PROTOCOL_TCP;
    safe_strcpy(s.detection_object_filter, "include", sizeof(s.detection_object_filter), 0);
    safe_strcpy(s.detection_object_filter_list, "person,car", sizeof(s.detection_object_filter_list), 0);
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
    safe_strcpy(s.name, "cam_exc", sizeof(s.name), 0);
    safe_strcpy(s.url, "rtsp://localhost/exc", sizeof(s.url), 0);
    s.enabled = true; s.width = 1920; s.height = 1080; s.fps = 30;
    s.protocol = STREAM_PROTOCOL_TCP;
    safe_strcpy(s.detection_object_filter, "exclude", sizeof(s.detection_object_filter), 0);
    safe_strcpy(s.detection_object_filter_list, "cat", sizeof(s.detection_object_filter_list), 0);
    add_stream_config(&s);

    detection_result_t r = make_result_1det("cat", 0.1f, 0.1f, 0.1f, 0.1f, 0.9f);
    filter_detections_by_stream_objects("cam_exc", &r);
    TEST_ASSERT_EQUAL_INT(0, r.count);
}

void test_stream_object_exclude_keeps_unmatched_label(void) {
    stream_config_t s;
    memset(&s, 0, sizeof(s));
    safe_strcpy(s.name, "cam_exc2", sizeof(s.name), 0);
    safe_strcpy(s.url, "rtsp://localhost/exc2", sizeof(s.url), 0);
    s.enabled = true; s.width = 1920; s.height = 1080; s.fps = 30;
    s.protocol = STREAM_PROTOCOL_TCP;
    safe_strcpy(s.detection_object_filter, "exclude", sizeof(s.detection_object_filter), 0);
    safe_strcpy(s.detection_object_filter_list, "cat", sizeof(s.detection_object_filter_list), 0);
    add_stream_config(&s);

    detection_result_t r = make_result_1det("dog", 0.1f, 0.1f, 0.1f, 0.1f, 0.9f);
    filter_detections_by_stream_objects("cam_exc2", &r);
    TEST_ASSERT_EQUAL_INT(1, r.count);
}

/* ================================================================
 * filter_detections_by_stream_objects — concurrent label parsing
 * ================================================================ */

typedef struct {
    const char *stream_name;
    const char *label;
    pthread_barrier_t *start_barrier;
    int failures;
} concurrent_filter_ctx_t;

static void ensure_filtered_stream(const char *name, const char *url,
                                   const char *filter_list) {
    stream_config_t stream;
    memset(&stream, 0, sizeof(stream));
    safe_strcpy(stream.name, name, sizeof(stream.name), 0);
    safe_strcpy(stream.url, url, sizeof(stream.url), 0);
    stream.enabled = true;
    stream.width = 1920;
    stream.height = 1080;
    stream.fps = 30;
    stream.protocol = STREAM_PROTOCOL_TCP;
    safe_strcpy(stream.detection_object_filter, "include",
                sizeof(stream.detection_object_filter), 0);
    safe_strcpy(stream.detection_object_filter_list, filter_list,
                sizeof(stream.detection_object_filter_list), 0);
    add_stream_config(&stream);
}

static void *run_concurrent_filter(void *opaque) {
    concurrent_filter_ctx_t *ctx = opaque;
    pthread_barrier_wait(ctx->start_barrier);

    for (int iteration = 0; iteration < 100; iteration++) {
        detection_result_t result;
        memset(&result, 0, sizeof(result));
        result.count = MAX_DETECTIONS;
        for (int index = 0; index < MAX_DETECTIONS; index++) {
            safe_strcpy(result.detections[index].label, ctx->label,
                        sizeof(result.detections[index].label), 0);
        }

        if (filter_detections_by_stream_objects(ctx->stream_name, &result) != 0 ||
            result.count != MAX_DETECTIONS) {
            ctx->failures++;
        }
    }
    return NULL;
}

void test_stream_object_filters_are_reentrant(void) {
    static const char list_a[] =
        "a00,a01,a02,a03,a04,a05,a06,a07,a08,a09,a10,a11,a12,a13,a14,a15,keep_a";
    static const char list_b[] =
        "b00,b01,b02,b03,b04,b05,b06,b07,b08,b09,b10,b11,b12,b13,b14,b15,keep_b";
    ensure_filtered_stream("cam_parallel_a", "rtsp://localhost/parallel-a", list_a);
    ensure_filtered_stream("cam_parallel_b", "rtsp://localhost/parallel-b", list_b);

    enum { THREAD_COUNT = 8 };
    pthread_t threads[THREAD_COUNT];
    concurrent_filter_ctx_t contexts[THREAD_COUNT];
    pthread_barrier_t start_barrier;
    TEST_ASSERT_EQUAL_INT(0, pthread_barrier_init(&start_barrier, NULL, THREAD_COUNT));

    for (int index = 0; index < THREAD_COUNT; index++) {
        bool use_a = (index % 2) == 0;
        contexts[index] = (concurrent_filter_ctx_t){
            .stream_name = use_a ? "cam_parallel_a" : "cam_parallel_b",
            .label = use_a ? "keep_a" : "keep_b",
            .start_barrier = &start_barrier,
            .failures = 0,
        };
        TEST_ASSERT_EQUAL_INT(
            0, pthread_create(&threads[index], NULL, run_concurrent_filter,
                              &contexts[index]));
    }

    for (int index = 0; index < THREAD_COUNT; index++) {
        TEST_ASSERT_EQUAL_INT(0, pthread_join(threads[index], NULL));
        TEST_ASSERT_EQUAL_INT(0, contexts[index].failures);
    }
    TEST_ASSERT_EQUAL_INT(0, pthread_barrier_destroy(&start_barrier));
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
    RUN_TEST(test_stream_object_filters_are_reentrant);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
