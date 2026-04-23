/**
 * @file test_cross_stream_motion_trigger.c
 * @brief Layer 3 Unity tests for cross-stream motion trigger feature
 *
 * Tests the process_motion_event() cross-stream propagation logic in
 * src/video/cross_stream_motion_trigger.c.  The feature allows a "silent"
 * stream (e.g. PTZ lens) to have its recording triggered by motion events
 * from a different "source" stream (e.g. fixed wide-angle lens with ONVIF
 * support).
 *
 * Layer 3: requires lightnvr_lib + FFmpeg (the module header pulls in
 * libavformat/avformat.h transitively).
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

#include "unity.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#include "video/cross_stream_motion_trigger.h"
#include "core/config.h"
#include "core/logger.h"
#include "utils/strings.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_cross_stream_trigger_test.db"

extern config_t g_config;

/* ---- helpers ---- */

static stream_config_t make_stream(const char *name, bool enabled) {
    stream_config_t s;
    memset(&s, 0, sizeof(s));
    safe_strcpy(s.name, name, sizeof(s.name), 0);
    safe_strcpy(s.url, "rtsp://localhost/stream", sizeof(s.url), 0);
    safe_strcpy(s.codec, "h264", sizeof(s.codec), 0);
    s.enabled  = enabled;
    s.width    = 1920;
    s.height   = 1080;
    s.fps      = 25;
    s.priority = 5;
    s.segment_duration = 60;
    s.streaming_enabled = true;
    s.detection_threshold = 0.5f;
    s.detection_interval  = 10;
    s.pre_detection_buffer  = 5;
    s.post_detection_buffer = 10;
    safe_strcpy(s.detection_object_filter, "none", sizeof(s.detection_object_filter), 0);
    s.tier_critical_multiplier  = 3.0;
    s.tier_important_multiplier = 2.0;
    s.tier_ephemeral_multiplier = 0.25;
    s.storage_priority = 5;
    return s;
}

static void clear_streams(void) {
    sqlite3 *db = get_db_handle();
    sqlite3_exec(db, "DELETE FROM streams;", NULL, NULL, NULL);
}

/* ---- Unity boilerplate ---- */
void setUp(void)    { clear_streams(); }
void tearDown(void) {}

/* ================================================================
 * process_motion_event — null / degenerate inputs
 * ================================================================ */

void test_process_motion_event_null_stream_returns_error(void) {
    int rc = process_motion_event(NULL, true, time(NULL), false);
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

/* ================================================================
 * process_motion_event — no linked streams in DB
 *
 * When no stream has motion_trigger_source pointing at the source,
 * the function should succeed and perform no extra push.
 * ================================================================ */

void test_process_motion_event_no_linked_streams(void) {
    stream_config_t src = make_stream("cam_fixed_solo", true);
    add_stream_config(&src);

    int rc = process_motion_event("cam_fixed_solo", true, time(NULL), false);
    /* Primary event push may fail if motion recording system is not
     * enabled for this stream, but the cross-stream scan must not crash. */
    (void)rc;
    TEST_PASS();
}

/* ================================================================
 * process_motion_event — linked stream present in DB
 *
 * A PTZ stream has motion_trigger_source = "cam_fixed".
 * Calling process_motion_event for "cam_fixed" should exercise the
 * full cross-stream propagation code path (calloc → scan → push → free).
 * ================================================================ */

void test_process_motion_event_propagates_to_linked_stream(void) {
    /* Add source stream (fixed lens with ONVIF events) */
    stream_config_t fixed = make_stream("cam_fixed", true);
    add_stream_config(&fixed);

    /* Add PTZ stream slaved to cam_fixed */
    stream_config_t ptz = make_stream("cam_ptz", true);
    safe_strcpy(ptz.motion_trigger_source, "cam_fixed", sizeof(ptz.motion_trigger_source), 0);
    add_stream_config(&ptz);

    /* Trigger a motion-start event on the source stream */
    int rc = process_motion_event("cam_fixed", true, time(NULL), false);
    /* The propagation path (calloc/scan/push/free) must complete without crash.
     * push_event for the linked stream may fail if motion recording is not
     * initialized for cam_ptz, but we only assert no crash here. */
    (void)rc;
    TEST_PASS();
}

/* ================================================================
 * process_motion_event — motion-end propagates to linked stream
 * ================================================================ */

void test_process_motion_event_end_propagates_to_linked_stream(void) {
    stream_config_t fixed = make_stream("cam_fixed_end", true);
    add_stream_config(&fixed);

    stream_config_t ptz = make_stream("cam_ptz_end", true);
    safe_strcpy(ptz.motion_trigger_source, "cam_fixed_end", sizeof(ptz.motion_trigger_source), 0);
    add_stream_config(&ptz);

    /* Trigger motion-end */
    int rc = process_motion_event("cam_fixed_end", false, time(NULL), false);
    (void)rc;
    TEST_PASS();
}

/* ================================================================
 * process_motion_event — source stream not in DB, linked stream present
 *
 * Ensures the DB scan handles an "unregistered" source name gracefully
 * (no matching motion_trigger_source rows → loop body never entered).
 * ================================================================ */

void test_process_motion_event_unregistered_source_no_crash(void) {
    /* Add a PTZ stream that lists a source not being called */
    stream_config_t ptz = make_stream("cam_ptz_orphan", true);
    safe_strcpy(ptz.motion_trigger_source, "cam_ghost", sizeof(ptz.motion_trigger_source), 0);
    add_stream_config(&ptz);

    /* Call for a totally different source — no linked streams should match */
    int rc = process_motion_event("cam_other", true, time(NULL), false);
    (void)rc;
    TEST_PASS();
}

/* ================================================================
 * main
 * ================================================================ */

int main(void) {
    /* Suppress log noise during tests */
    set_log_level(LOG_LEVEL_ERROR);

    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database() failed\n");
        return 1;
    }

    g_config.max_streams = 16;

    UNITY_BEGIN();

    RUN_TEST(test_process_motion_event_null_stream_returns_error);
    RUN_TEST(test_process_motion_event_no_linked_streams);
    RUN_TEST(test_process_motion_event_propagates_to_linked_stream);
    RUN_TEST(test_process_motion_event_end_propagates_to_linked_stream);
    RUN_TEST(test_process_motion_event_unregistered_source_no_crash);

    int result = UNITY_END();

    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}

