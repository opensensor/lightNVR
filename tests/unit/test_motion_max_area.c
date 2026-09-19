/**
 * @file test_motion_max_area.c
 * @brief Layer 2 unit tests — motion detector max-area sanity cap
 *
 * The frame-differencing motion detector had a minimum area to trigger
 * detection but no maximum, so an event where the *entire* frame changes at
 * once (e.g. a camera's IR-cut filter flipping between day/night mode) reads
 * as maximal-confidence motion identical to a real, localized subject.
 * Production evidence: on the reference install, ~40% of all "motion"
 * events over 24h were the exact ceiling reading (area=100%, single
 * cluster spanning the whole frame), spread evenly across every hour of
 * the day and night rather than clustered at dawn/dusk light transitions.
 *
 * These tests use uniform synthetic frames to isolate the area computation
 * from real image content: a whole-frame brightness step (no real subject
 * could ever produce this) must not be flagged, while a localized change
 * confined to part of the frame (what a real subject looks like) must
 * still be detected normally.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "unity.h"
#include "core/logger.h"
#include "video/motion_detection.h"
#include "video/detection_result.h"

#define FRAME_W 64
#define FRAME_H 64
#define FRAME_BYTES (FRAME_W * FRAME_H)

void setUp(void) {
    init_logger();
    TEST_ASSERT_EQUAL_INT(0, init_motion_detection_system());
}

void tearDown(void) {
    shutdown_motion_detection_system();
}

static unsigned char *uniform_frame(unsigned char value) {
    unsigned char *frame = malloc(FRAME_BYTES);
    TEST_ASSERT_NOT_NULL(frame);
    memset(frame, value, FRAME_BYTES);
    return frame;
}

/* Establishes a calm baseline: frame 1 initializes background/prev_frame
 * (always returns 0, "skip on first frame" per the detector's own contract);
 * frame 2 is identical, confirming a settled zero-motion baseline before
 * the test's real frame is introduced. `use_grid_detection` selects which
 * of the two detect_motion() code paths the cap is exercised against --
 * both were patched with the same max-area check. */
static void establish_calm_baseline_ex(const char *stream_name, unsigned char value,
                                        bool use_grid_detection) {
    /* Motion streams default to disabled on creation; detect_motion() is a
     * silent no-op (returns 0, result untouched) until this is set. */
    TEST_ASSERT_EQUAL_INT(0, set_motion_detection_enabled(stream_name, true));
    /* Matches this module's own DEFAULT_BLUR_RADIUS/NOISE_THRESHOLD/
     * GRID_SIZE/MOTION_HISTORY -- only use_grid_detection is under test. */
    TEST_ASSERT_EQUAL_INT(0, configure_advanced_motion_detection(
        stream_name, 1, 10, use_grid_detection, 6, 2));

    unsigned char *frame = uniform_frame(value);
    detection_result_t result;

    memset(&result, 0, sizeof(result));
    TEST_ASSERT_EQUAL_INT(0, detect_motion(stream_name, frame, FRAME_W, FRAME_H, 1, 1000, &result));
    TEST_ASSERT_EQUAL_INT(0, result.count);

    memset(&result, 0, sizeof(result));
    TEST_ASSERT_EQUAL_INT(0, detect_motion(stream_name, frame, FRAME_W, FRAME_H, 1, 1001, &result));
    TEST_ASSERT_EQUAL_INT(0, result.count);

    free(frame);
}

static void establish_calm_baseline(const char *stream_name, unsigned char value) {
    establish_calm_baseline_ex(stream_name, value, true);
}

void test_whole_frame_brightness_step_is_not_flagged_as_motion(void) {
    const char *stream_name = "test_max_area_whole_frame";
    establish_calm_baseline(stream_name, 50);

    /* Every pixel jumps at once -- exactly what an IR-cut filter flip looks
     * like to a frame-differencing algorithm, never what a real subject
     * confined to part of the scene looks like. */
    unsigned char *shifted = uniform_frame(200);
    detection_result_t result;
    memset(&result, 0, sizeof(result));

    TEST_ASSERT_EQUAL_INT(0, detect_motion(stream_name, shifted, FRAME_W, FRAME_H, 1, 1002, &result));
    TEST_ASSERT_EQUAL_INT(0, result.count);

    free(shifted);
}

void test_localized_change_is_still_detected(void) {
    const char *stream_name = "test_max_area_localized";
    establish_calm_baseline(stream_name, 50);

    /* Only the top-left quarter of the frame changes -- a real subject
     * occupying part of the scene -- must still trigger detection. */
    unsigned char *partial = uniform_frame(50);
    for (int y = 0; y < FRAME_H / 2; y++) {
        for (int x = 0; x < FRAME_W / 2; x++) {
            partial[y * FRAME_W + x] = 220;
        }
    }
    detection_result_t result;
    memset(&result, 0, sizeof(result));

    TEST_ASSERT_EQUAL_INT(0, detect_motion(stream_name, partial, FRAME_W, FRAME_H, 1, 1002, &result));
    TEST_ASSERT_GREATER_THAN_INT(0, result.count);

    free(partial);
}

/* Same two scenarios, but against the simple frame-differencing path
 * (use_grid_detection=false) rather than the default grid-based one --
 * both were patched with the same max_motion_area check, and a regression
 * that removed or misapplied the cap in only one of the two would
 * otherwise pass every other test in this file. */

void test_whole_frame_brightness_step_is_not_flagged_non_grid(void) {
    const char *stream_name = "test_max_area_whole_frame_non_grid";
    establish_calm_baseline_ex(stream_name, 50, false);

    unsigned char *shifted = uniform_frame(200);
    detection_result_t result;
    memset(&result, 0, sizeof(result));

    TEST_ASSERT_EQUAL_INT(0, detect_motion(stream_name, shifted, FRAME_W, FRAME_H, 1, 1002, &result));
    TEST_ASSERT_EQUAL_INT(0, result.count);

    free(shifted);
}

void test_localized_change_is_still_detected_non_grid(void) {
    const char *stream_name = "test_max_area_localized_non_grid";
    establish_calm_baseline_ex(stream_name, 50, false);

    unsigned char *partial = uniform_frame(50);
    for (int y = 0; y < FRAME_H / 2; y++) {
        for (int x = 0; x < FRAME_W / 2; x++) {
            partial[y * FRAME_W + x] = 220;
        }
    }
    detection_result_t result;
    memset(&result, 0, sizeof(result));

    TEST_ASSERT_EQUAL_INT(0, detect_motion(stream_name, partial, FRAME_W, FRAME_H, 1, 1002, &result));
    TEST_ASSERT_GREATER_THAN_INT(0, result.count);

    free(partial);
}

int main(void) {
    UNITY_BEGIN();

    RUN_TEST(test_whole_frame_brightness_step_is_not_flagged_as_motion);
    RUN_TEST(test_localized_change_is_still_detected);
    RUN_TEST(test_whole_frame_brightness_step_is_not_flagged_non_grid);
    RUN_TEST(test_localized_change_is_still_detected_non_grid);

    return UNITY_END();
}
