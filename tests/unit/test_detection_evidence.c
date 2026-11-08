/* INT_01 L6: pure batch building, label eligibility, box clamping and the
 * inference transform for local detector evidence (contract sections 3, 4.4). */
#define _GNU_SOURCE
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/time.h>

#include "unity.h"
#include "core/logger.h"
#include "core/observation_evidence.h"
#include "utils/uuid.h"
#include "video/detection_evidence.h"
#include "video/ffmpeg_utils.h"

/* Unity is built without double support. */
#define ASSERT_NEAR(tol, expected, actual) \
    TEST_ASSERT_TRUE_MESSAGE(fabs((double)(expected) - (double)(actual)) <= (tol), #actual)

#define CAMERA "22222222-2222-4222-8222-222222222222"

static detection_evidence_camera_t camera(const char *labels, bool enabled) {
    detection_evidence_camera_t c = {
        .camera_uuid = CAMERA, .stream_name = "feeder-east",
        .evidence_enabled = enabled, .labels_csv = labels,
    };
    return c;
}

static detection_evidence_engine_t engine(const char *key, const char *type) {
    detection_evidence_engine_t e = {
        .engine_key = key, .engine_type = type,
        .model_path = "/etc/lightnvr/models/ssd_mobilenet_v2.sod",
        .model_version = NULL,
    };
    return e;
}

static detection_t det(const char *label, float conf, float x, float y, float w,
                       float h, int track) {
    detection_t d;
    memset(&d, 0, sizeof(d));
    snprintf(d.label, sizeof(d.label), "%s", label);
    d.confidence = conf;
    d.x = x; d.y = y; d.width = w; d.height = h;
    d.track_id = track;
    return d;
}

void setUp(void) {}
void tearDown(void) {}

static void test_label_eligibility(void) {
    TEST_ASSERT_TRUE(detection_evidence_label_eligible(NULL, "bird"));
    TEST_ASSERT_TRUE(detection_evidence_label_eligible("", "bird"));
    TEST_ASSERT_TRUE(detection_evidence_label_eligible("  ", "squirrel"));
    TEST_ASSERT_TRUE(detection_evidence_label_eligible("bird", "bird"));
    TEST_ASSERT_TRUE(detection_evidence_label_eligible("cat, Bird ,dog", "bird"));
    TEST_ASSERT_TRUE(detection_evidence_label_eligible("cat,dog", "DOG"));
    TEST_ASSERT_FALSE(detection_evidence_label_eligible("bird", "birds"));
    TEST_ASSERT_FALSE(detection_evidence_label_eligible("birds", "bird"));
    TEST_ASSERT_FALSE(detection_evidence_label_eligible("cat,dog", "bird"));
    TEST_ASSERT_FALSE(detection_evidence_label_eligible(NULL, ""));
    TEST_ASSERT_FALSE(detection_evidence_label_eligible("bird", NULL));
}

static void test_box_clamping(void) {
    double x, y, w, h;
    TEST_ASSERT_TRUE(detection_evidence_clamp_box(0.1, 0.2, 0.3, 0.4, &x, &y, &w, &h));
    ASSERT_NEAR(1e-9, 0.1, x);
    ASSERT_NEAR(1e-9, 0.4, h);
    /* Spills over the right/bottom edge and starts left of 0. */
    TEST_ASSERT_TRUE(detection_evidence_clamp_box(-0.1, 0.8, 0.5, 0.5, &x, &y, &w, &h));
    ASSERT_NEAR(1e-9, 0.0, x);
    ASSERT_NEAR(1e-9, 0.4, w);
    ASSERT_NEAR(1e-9, 0.8, y);
    ASSERT_NEAR(1e-9, 0.2, h);
    TEST_ASSERT_TRUE(x + w <= 1.0 && y + h <= 1.0);
    /* Entirely outside, zero area, NaN. */
    TEST_ASSERT_FALSE(detection_evidence_clamp_box(1.2, 0.1, 0.3, 0.3, &x, &y, &w, &h));
    TEST_ASSERT_FALSE(detection_evidence_clamp_box(0.1, 0.1, 0.0, 0.3, &x, &y, &w, &h));
    TEST_ASSERT_FALSE(detection_evidence_clamp_box(NAN, 0.1, 0.2, 0.3, &x, &y, &w, &h));
}

static void test_build_batch_fields_tracks_and_eligibility(void) {
    detection_t d[4] = {
        det("bird", 0.87f, 0.412f, 0.288f, 0.12f, 0.165f, 17),
        det("squirrel", 0.9f, 0.1f, 0.1f, 0.2f, 0.2f, -1),
        det("bird", 1.4f, 0.9f, 0.9f, 0.3f, 0.3f, -1),   /* clamp conf + box */
        det("bird", 0.6f, 2.0f, 2.0f, 0.1f, 0.1f, 3),    /* box outside: no box */
    };
    detection_evidence_camera_t c = camera("bird", true);
    detection_evidence_engine_t e = engine("birds-sod", "object");
    observation_batch_input_t b;
    int n = detection_evidence_build_batch(&c, &e, d, 4, 1793000000123LL, &b);
    TEST_ASSERT_EQUAL_INT(3, n);
    TEST_ASSERT_EQUAL_INT(3, b.object_count);
    TEST_ASSERT_TRUE(lightnvr_uuid_is_valid(b.batch_id));
    TEST_ASSERT_EQUAL_STRING(CAMERA, b.camera_uuid);
    TEST_ASSERT_EQUAL_STRING("feeder-east", b.stream_name);
    TEST_ASSERT_EQUAL_INT64(1793000000123LL, b.observed_at_ms);
    TEST_ASSERT_EQUAL_STRING("birds-sod", b.engine_key);
    TEST_ASSERT_EQUAL_STRING("object", b.engine_type);
    TEST_ASSERT_EQUAL_STRING("ssd_mobilenet_v2.sod", b.model);
    TEST_ASSERT_EQUAL_STRING("", b.model_version);
    TEST_ASSERT_EQUAL_INT64(0, b.recording_id);

    for (int i = 0; i < 3; ++i) {
        TEST_ASSERT_EQUAL_INT(i, b.objects[i].object_index);
        TEST_ASSERT_EQUAL_STRING("bird", b.objects[i].label);
    }
    /* Object 0: track with namespace = engine key and session = boot UUID. */
    TEST_ASSERT_TRUE(b.objects[0].has_track);
    TEST_ASSERT_EQUAL_STRING("17", b.objects[0].track_id);
    TEST_ASSERT_EQUAL_STRING("birds-sod", b.objects[0].track_namespace);
    TEST_ASSERT_EQUAL_STRING(detection_evidence_boot_session(), b.objects[0].track_session);
    TEST_ASSERT_TRUE(lightnvr_uuid_is_valid(b.objects[0].track_session));
    ASSERT_NEAR(1e-6, 0.87, b.objects[0].confidence);
    TEST_ASSERT_TRUE(b.objects[0].has_box);
    ASSERT_NEAR(1e-6, 0.412, b.objects[0].box_x);
    /* Object 1: no track (track_id < 0), confidence and box clamped. */
    TEST_ASSERT_FALSE(b.objects[1].has_track);
    TEST_ASSERT_EQUAL_STRING("", b.objects[1].track_id);
    ASSERT_NEAR(1e-9, 1.0, b.objects[1].confidence);
    TEST_ASSERT_TRUE(b.objects[1].has_box);
    TEST_ASSERT_TRUE(b.objects[1].box_x + b.objects[1].box_width <= 1.0);
    ASSERT_NEAR(1e-6, 0.1, b.objects[1].box_width);
    /* Object 2: box clamped away entirely -> null box, still an observation. */
    TEST_ASSERT_FALSE(b.objects[2].has_box);
    TEST_ASSERT_TRUE(b.objects[2].has_track);

    /* Boot session is stable for the process. */
    const char *s1 = detection_evidence_boot_session();
    TEST_ASSERT_EQUAL_STRING(s1, detection_evidence_boot_session());

    /* Each accepted result gets its own batch_id. */
    observation_batch_input_t b2;
    TEST_ASSERT_EQUAL_INT(3, detection_evidence_build_batch(&c, &e, d, 4, 1793000000124LL, &b2));
    TEST_ASSERT_NOT_EQUAL(0, strcmp(b.batch_id, b2.batch_id));
}

static void test_build_batch_defaults_and_limits(void) {
    detection_t d[OBS_MAX_BATCH + 3];
    for (int i = 0; i < OBS_MAX_BATCH + 3; ++i)
        d[i] = det("bird", 0.5f, 0.1f, 0.1f, 0.1f, 0.1f, -1);
    detection_evidence_camera_t c = camera(NULL, true);
    detection_evidence_engine_t e = { .engine_key = NULL, .engine_type = "object",
                                      .model_path = NULL, .model_version = "v3" };
    observation_batch_input_t b;
    TEST_ASSERT_EQUAL_INT(OBS_MAX_BATCH,
        detection_evidence_build_batch(&c, &e, d, OBS_MAX_BATCH + 3, 1000, &b));
    TEST_ASSERT_EQUAL_STRING("legacy-primary", b.engine_key);
    TEST_ASSERT_EQUAL_STRING("", b.model);
    TEST_ASSERT_EQUAL_STRING("v3", b.model_version);

    /* Nothing eligible: 0 and no batch id. */
    detection_evidence_camera_t only_cats = camera("cat", true);
    TEST_ASSERT_EQUAL_INT(0, detection_evidence_build_batch(&only_cats, &e, d, 2, 1000, &b));
    TEST_ASSERT_EQUAL_STRING("", b.batch_id);

    /* Invalid input. */
    detection_evidence_camera_t no_uuid = camera(NULL, true);
    no_uuid.camera_uuid = "";
    TEST_ASSERT_EQUAL_INT(-1, detection_evidence_build_batch(&no_uuid, &e, d, 1, 1000, &b));
    TEST_ASSERT_EQUAL_INT(-1, detection_evidence_build_batch(&c, &e, d, 1, 0, &b));
    TEST_ASSERT_EQUAL_INT(-1, detection_evidence_build_batch(&c, &e, NULL, 1, 1000, &b));
}

static void test_transform_modes(void) {
    observation_evidence_transform_t t;
    detection_evidence_transform_compute(640, 360, 0, 0, false, &t);
    TEST_ASSERT_EQUAL_INT(OBS_TRANSFORM_NONE, t.mode);
    TEST_ASSERT_EQUAL_INT(640, t.source_width);
    TEST_ASSERT_EQUAL_INT(640, t.inference_width);
    ASSERT_NEAR(1e-12, 1.0, t.scale_x);

    detection_evidence_transform_compute(640, 360, 640, 360, true, &t);
    TEST_ASSERT_EQUAL_INT(OBS_TRANSFORM_NONE, t.mode);

    /* SOD: independent per-axis resize to the network size. */
    detection_evidence_transform_compute(1280, 720, 416, 416, false, &t);
    TEST_ASSERT_EQUAL_INT(OBS_TRANSFORM_RESIZE, t.mode);
    TEST_ASSERT_EQUAL_INT(1280, t.source_width);
    TEST_ASSERT_EQUAL_INT(720, t.source_height);
    TEST_ASSERT_EQUAL_INT(416, t.inference_width);
    TEST_ASSERT_EQUAL_INT(416, t.inference_height);
    ASSERT_NEAR(1e-9, 416.0 / 1280.0, t.scale_x);
    ASSERT_NEAR(1e-9, 416.0 / 720.0, t.scale_y);
    ASSERT_NEAR(1e-12, 0.0, t.pad_x);

    /* LiteRT: uniform scale + centred padding, as letterbox_rgb24(). */
    detection_evidence_transform_compute(1280, 720, 640, 640, true, &t);
    TEST_ASSERT_EQUAL_INT(OBS_TRANSFORM_LETTERBOX, t.mode);
    ASSERT_NEAR(1e-6, 0.5, t.scale_x);
    ASSERT_NEAR(1e-6, 0.5, t.scale_y);
    ASSERT_NEAR(1e-12, 0.0, t.pad_x);
    ASSERT_NEAR(1e-12, 140.0, t.pad_y);  /* (640 - 360) / 2 */

    /* Mapping an inference pixel back lands on the stored frame (section 4.4). */
    double u = 320.0, v = 320.0;  /* centre of the tensor */
    ASSERT_NEAR(1e-6, 640.0, (u - t.pad_x) / t.scale_x);
    ASSERT_NEAR(1e-6, 360.0, (v - t.pad_y) / t.scale_y);

    /* The transform serializes through the L3 helper. */
    char json[OBS_TRANSFORM_SIZE];
    TEST_ASSERT_EQUAL_INT(0, observation_evidence_transform_json(&t, 1280, 720, json, sizeof(json)));
    TEST_ASSERT_NOT_NULL(strstr(json, "\"mode\":\"letterbox\""));
}

static void test_model_geometry_without_model(void) {
    int w = -1, h = -1;
    bool lb = true;
    TEST_ASSERT_EQUAL_INT(-1, detection_evidence_model_geometry(NULL, &w, &h, &lb));
}

static void test_observed_at_ms(void) {
    struct timeval tv;
    gettimeofday(&tv, NULL);
    int64_t ms = detection_evidence_observed_at_ms(tv.tv_sec);
    TEST_ASSERT_TRUE(ms / 1000 == (int64_t)tv.tv_sec || ms / 1000 == (int64_t)tv.tv_sec + 1 ||
                     ms == (int64_t)tv.tv_sec * 1000);
    /* An old frame second keeps that second exactly. */
    TEST_ASSERT_EQUAL_INT64(1700000000000LL, detection_evidence_observed_at_ms(1700000000));
}

static uint8_t *rgb_buffer(int w, int h) {
    uint8_t *buf = calloc(1, (size_t)w * h * 3 + RGB24_SWS_TAIL_PADDING);
    TEST_ASSERT_NOT_NULL(buf);
    return buf;
}

/* With evidence disabled, motion engines or nothing eligible, capture frees
 * the buffer (ASan would flag a leak) and never starts the L3 queue. */
static void test_capture_skips_without_allocating(void) {
    detection_t d[1] = { det("bird", 0.9f, 0.1f, 0.1f, 0.2f, 0.2f, -1) };
    detection_evidence_camera_t off = camera(NULL, false);
    detection_evidence_camera_t on = camera(NULL, true);
    detection_evidence_camera_t cats = camera("cat", true);
    detection_evidence_engine_t obj = engine("legacy-primary", "object");
    detection_evidence_engine_t motion = engine("motion", "motion");
    detection_evidence_engine_t api = engine("api", "api");
    char asset[OBS_UUID_SIZE] = "x";

    TEST_ASSERT_EQUAL_INT(DETECTION_EVIDENCE_NOT_ENABLED,
        detection_evidence_capture(&off, &obj, d, 1, 1000, rgb_buffer(8, 8), 8, 8, NULL, NULL, asset));
    TEST_ASSERT_EQUAL_STRING("", asset);
    TEST_ASSERT_EQUAL_INT(DETECTION_EVIDENCE_NOT_OBJECT_ENGINE,
        detection_evidence_capture(&on, &motion, d, 1, 1000, rgb_buffer(8, 8), 8, 8, NULL, NULL, asset));
    TEST_ASSERT_EQUAL_INT(DETECTION_EVIDENCE_NOT_OBJECT_ENGINE,
        detection_evidence_capture(&on, &api, d, 1, 1000, rgb_buffer(8, 8), 8, 8, NULL, NULL, asset));
    TEST_ASSERT_EQUAL_INT(DETECTION_EVIDENCE_NOT_ELIGIBLE,
        detection_evidence_capture(&cats, &obj, d, 1, 1000, rgb_buffer(8, 8), 8, 8, NULL, NULL, asset));
    TEST_ASSERT_EQUAL_INT(DETECTION_EVIDENCE_ERROR,
        detection_evidence_capture(&on, &obj, d, 1, 1000, NULL, 8, 8, NULL, NULL, asset));

    /* Not initialized: submitted, L3 says DISABLED, still nothing allocated. */
    observation_evidence_result_t r = OBS_EVIDENCE_ACCEPTED;
    TEST_ASSERT_EQUAL_INT(DETECTION_EVIDENCE_SUBMITTED,
        detection_evidence_capture(&on, &obj, d, 1, 1000, rgb_buffer(8, 8), 8, 8, NULL, &r, asset));
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_DISABLED, r);

    observation_evidence_stats_t s;
    observation_evidence_get_stats(&s);
    TEST_ASSERT_FALSE(s.allocated);
    TEST_ASSERT_FALSE(s.worker_running);
    TEST_ASSERT_EQUAL_UINT64(0, s.accepted);
}

int main(void) {
    init_logger();
    set_log_level(LOG_LEVEL_ERROR);
    UNITY_BEGIN();
    RUN_TEST(test_label_eligibility);
    RUN_TEST(test_box_clamping);
    RUN_TEST(test_build_batch_fields_tracks_and_eligibility);
    RUN_TEST(test_build_batch_defaults_and_limits);
    RUN_TEST(test_transform_modes);
    RUN_TEST(test_model_geometry_without_model);
    RUN_TEST(test_observed_at_ms);
    RUN_TEST(test_capture_skips_without_allocating);
    return UNITY_END();
}
