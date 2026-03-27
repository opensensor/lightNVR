/**
 * @file test_motion_detection.c
 * @brief Layer 3 Unity tests for video/motion_detection.c
 *
 * Tests the pixel-level motion detection pipeline using synthetic
 * greyscale frames — no live camera or FFmpeg decode required.
 *
 * Layer 3: lightnvr_lib + FFmpeg (motion_detection.c includes streams.h
 * which transitively pulls in libavformat headers).
 *
 * The database is initialised so that the zone-mask helper
 * (build_motion_zone_mask → get_detection_zones) can query it; with no
 * zones configured the mask defaults to all-cells-active.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

#include "unity.h"
#include "core/logger.h"
#include "database/db_core.h"
#include "video/motion_detection.h"

#define TEST_DB_PATH  "/tmp/lightnvr_unit_motion_detection_test.db"
#define FRAME_W 64
#define FRAME_H 48
#define FRAME_PIXELS (FRAME_W * FRAME_H)

/* ---- synthetic frames ---- */

/* A solid mid-grey frame — used as the "no change" baseline. */
static unsigned char g_grey_frame[FRAME_PIXELS];

/* A frame that differs substantially from g_grey_frame:
   ~50 % of pixels are set to white (255). */
static unsigned char g_white_frame[FRAME_PIXELS];

static void build_frames(void) {
    memset(g_grey_frame, 128, FRAME_PIXELS);

    /* Alternate rows: dark grey vs near-white to create large difference. */
    for (int y = 0; y < FRAME_H; y++) {
        unsigned char val = (y % 2 == 0) ? 200 : 30;
        memset(&g_white_frame[y * FRAME_W], val, FRAME_W);
    }
}

/* ---- Unity boilerplate ---- */

void setUp(void)    { shutdown_motion_detection_system(); init_motion_detection_system(); }
void tearDown(void) { shutdown_motion_detection_system(); }

/* ------------------------------------------------------------------ tests */

void test_init_motion_detection_succeeds(void) {
    /* Already done in setUp; calling again must be idempotent. */
    int rc = init_motion_detection_system();
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_shutdown_after_init_does_not_crash(void) {
    shutdown_motion_detection_system();
    /* If we reach here without a crash, the test passes. */
    TEST_PASS();
}

void test_configure_motion_detection_returns_zero(void) {
    int rc = configure_motion_detection("test_stream", 0.2f, 0.01f, 3);
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_configure_null_stream_fails(void) {
    int rc = configure_motion_detection(NULL, 0.2f, 0.01f, 3);
    TEST_ASSERT_EQUAL_INT(-1, rc);
}

void test_configure_advanced_returns_zero(void) {
    int rc = configure_advanced_motion_detection("test_stream", 1, 10, true, 6, 2);
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_configure_optimizations_returns_zero(void) {
    int rc = configure_motion_detection_optimizations("test_stream", false, 1);
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_set_enabled_and_check(void) {
    configure_motion_detection("enable_stream", 0.15f, 0.005f, 2);

    int rc = set_motion_detection_enabled("enable_stream", true);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_TRUE(is_motion_detection_enabled("enable_stream"));
}

void test_set_disabled_and_check(void) {
    configure_motion_detection("disable_stream", 0.15f, 0.005f, 2);
    set_motion_detection_enabled("disable_stream", true);
    set_motion_detection_enabled("disable_stream", false);
    TEST_ASSERT_FALSE(is_motion_detection_enabled("disable_stream"));
}

void test_is_enabled_defaults_to_false(void) {
    /* A new, unconfigured stream must default to disabled. */
    TEST_ASSERT_FALSE(is_motion_detection_enabled("brand_new_stream_xyz"));
}

void test_detect_null_stream_fails(void) {
    detection_result_t result;
    int rc = detect_motion(NULL, g_grey_frame, FRAME_W, FRAME_H, 1, time(NULL), &result);
    TEST_ASSERT_EQUAL_INT(-1, rc);
}

void test_detect_null_frame_fails(void) {
    detection_result_t result;
    int rc = detect_motion("s", NULL, FRAME_W, FRAME_H, 1, time(NULL), &result);
    TEST_ASSERT_EQUAL_INT(-1, rc);
}

void test_detect_null_result_fails(void) {
    int rc = detect_motion("s", g_grey_frame, FRAME_W, FRAME_H, 1, time(NULL), NULL);
    TEST_ASSERT_EQUAL_INT(-1, rc);
}

void test_detect_first_frame_initialises_and_returns_zero(void) {
    configure_motion_detection("init_stream", 0.15f, 0.005f, 2);
    set_motion_detection_enabled("init_stream", true);

    detection_result_t result;
    int rc = detect_motion("init_stream", g_grey_frame, FRAME_W, FRAME_H, 1, time(NULL), &result);
    /* First frame seeds the background model; no detection is expected. */
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_INT(0, result.count);
}

void test_detect_constant_frame_no_motion(void) {
    configure_motion_detection("const_stream", 0.15f, 0.005f, 1);
    /* Disable downscaling: the default factor of 2 reduces FRAME_H=48 to 24,
     * which gets clamped to the 32-px minimum, padding rows 24-31 with zeros.
     * Box blur then smooths those zero-padded boundary rows, producing a
     * non-zero diff between the unblurred prev_frame (set on the first call)
     * and the blurred blur_buffer (used on the second call), causing a false
     * motion detection even when both input frames are identical.
     * With downscaling disabled the frame stays 64x48 uniformly at 128, so
     * the blur output is also uniform 128 and the diff is exactly zero. */
    configure_motion_detection_optimizations("const_stream", false, 1);
    set_motion_detection_enabled("const_stream", true);

    detection_result_t result;
    time_t t = time(NULL);

    /* Frame 1: seed */
    detect_motion("const_stream", g_grey_frame, FRAME_W, FRAME_H, 1, t, &result);
    /* Frame 2: identical — no motion */
    int rc = detect_motion("const_stream", g_grey_frame, FRAME_W, FRAME_H, 1, t + 2, &result);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_INT(0, result.count);
}

void test_detect_very_different_frame_triggers_motion(void) {
    /* High sensitivity, tiny min-area, no cooldown — maximise detection. */
    configure_motion_detection("diff_stream", 0.05f, 0.001f, 1);
    set_motion_detection_enabled("diff_stream", true);

    detection_result_t result;
    time_t t = time(NULL);

    /* Frame 1: seed with grey */
    detect_motion("diff_stream", g_grey_frame, FRAME_W, FRAME_H, 1, t, &result);
    /* Frame 2: dramatically different (alternating dark/light rows) */
    int rc = detect_motion("diff_stream", g_white_frame, FRAME_W, FRAME_H, 1, t + 2, &result);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_GREATER_THAN(0, result.count);
}

void test_detect_rgb_frame_accepted(void) {
    configure_motion_detection("rgb_stream", 0.15f, 0.005f, 2);
    set_motion_detection_enabled("rgb_stream", true);

    /* Minimal 3-channel (RGB) frame */
    unsigned char rgb[FRAME_PIXELS * 3];
    memset(rgb, 100, sizeof(rgb));

    detection_result_t result;
    int rc = detect_motion("rgb_stream", rgb, FRAME_W, FRAME_H, 3, time(NULL), &result);
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_detect_unsupported_channels_fails(void) {
    configure_motion_detection("chan_stream", 0.15f, 0.005f, 2);
    set_motion_detection_enabled("chan_stream", true);

    /* Seed the stream with a valid first frame */
    detection_result_t result;
    detect_motion("chan_stream", g_grey_frame, FRAME_W, FRAME_H, 1, time(NULL), &result);

    /* 2-channel is not supported */
    unsigned char two_ch[FRAME_PIXELS * 2];
    memset(two_ch, 128, sizeof(two_ch));
    int rc = detect_motion("chan_stream", two_ch, FRAME_W, FRAME_H, 2, time(NULL) + 2, &result);
    TEST_ASSERT_EQUAL_INT(-1, rc);
}

/* ------------------------------------------------------------------ main */

int main(void) {
    build_frames();

    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    init_logger();

    UNITY_BEGIN();
    RUN_TEST(test_init_motion_detection_succeeds);
    RUN_TEST(test_shutdown_after_init_does_not_crash);
    RUN_TEST(test_configure_motion_detection_returns_zero);
    RUN_TEST(test_configure_null_stream_fails);
    RUN_TEST(test_configure_advanced_returns_zero);
    RUN_TEST(test_configure_optimizations_returns_zero);
    RUN_TEST(test_set_enabled_and_check);
    RUN_TEST(test_set_disabled_and_check);
    RUN_TEST(test_is_enabled_defaults_to_false);
    RUN_TEST(test_detect_null_stream_fails);
    RUN_TEST(test_detect_null_frame_fails);
    RUN_TEST(test_detect_null_result_fails);
    RUN_TEST(test_detect_first_frame_initialises_and_returns_zero);
    RUN_TEST(test_detect_constant_frame_no_motion);
    RUN_TEST(test_detect_very_different_frame_triggers_motion);
    RUN_TEST(test_detect_rgb_frame_accepted);
    RUN_TEST(test_detect_unsupported_channels_fails);
    int result = UNITY_END();

    shutdown_logger();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}

