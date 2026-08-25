/**
 * @file test_annotation_writer_registry.c
 * @brief Layer 3 Unity tests for the continuous-writer probe that annotation
 *        mode depends on (issue #547).
 *
 * Annotation mode (`record=1` together with `detection_based_recording=1`) must
 * link detections to the continuous recording instead of writing a second,
 * duplicate MP4 alongside it. The detection thread decides that in
 * should_annotate_continuous(), which is static; what it rests on, and what is
 * exercised here, is the pair of registry probes in mp4_recording_writer.c:
 *
 *   - get_current_recording_id_for_stream() — the *momentary* recording ID,
 *     which legitimately drops to 0 on every segment rotation and stays 0 for
 *     the whole of an RTSP reconnect.
 *   - stream_has_continuous_writer() — whether a continuous writer is
 *     registered at all, which holds across both of those gaps and is dropped
 *     only when the recording thread itself stops.
 *
 * The regression: annotation mode used to gate on the recording ID. A single
 * sample landing in one of those gaps read "no continuous recording" and
 * committed a full-length detection clip that then ran for the rest of the
 * motion event. test_registered_writer_between_segments_still_counts() is the
 * case that used to get this wrong.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdlib.h>
#include <string.h>

#include "unity.h"
#include "core/config.h"
#include "core/logger.h"
#include "video/mp4_writer.h"
#include "video/mp4_recording.h"

extern config_t g_config;

/* Stream names touched by the tests; unregistered in tearDown so the static
 * registry never leaks an entry from one test into the next. */
static const char *kNames[] = { "annot_a", "annot_b" };

/* The registry stores the pointer without taking ownership, and
 * unregister_mp4_writer_for_stream() deliberately does not close the writer,
 * so the test owns this allocation for the whole run. */
static mp4_writer_t *g_writer;

void setUp(void) {
    g_config.max_streams = MAX_STREAMS;
    g_writer = calloc(1, sizeof(*g_writer));
    TEST_ASSERT_NOT_NULL(g_writer);
}

void tearDown(void) {
    for (size_t i = 0; i < sizeof(kNames) / sizeof(kNames[0]); i++) {
        unregister_mp4_writer_for_stream(kNames[i]);
    }
    free(g_writer);
    g_writer = NULL;
}

/* A stream with no continuous recording thread has nothing to annotate. */
static void test_unregistered_stream_has_no_writer(void) {
    TEST_ASSERT_FALSE(stream_has_continuous_writer("annot_a"));
    TEST_ASSERT_EQUAL_UINT64(0, get_current_recording_id_for_stream("annot_a"));
}

/* While a segment is open, both probes agree: recording, with a known ID. */
static void test_registered_writer_with_open_segment(void) {
    g_writer->current_recording_id = 42;
    TEST_ASSERT_EQUAL_INT(0, register_mp4_writer_for_stream("annot_a", g_writer));

    TEST_ASSERT_TRUE(stream_has_continuous_writer("annot_a"));
    TEST_ASSERT_EQUAL_UINT64(42, get_current_recording_id_for_stream("annot_a"));
}

/*
 * The regression case. Between segments — and for the whole of an RTSP
 * reconnect — the writer stays registered but current_recording_id is 0.
 * Annotation mode must still see a continuous recording here; reading the ID
 * alone is what used to start a duplicate clip.
 */
static void test_registered_writer_between_segments_still_counts(void) {
    g_writer->current_recording_id = 0;
    TEST_ASSERT_EQUAL_INT(0, register_mp4_writer_for_stream("annot_a", g_writer));

    TEST_ASSERT_TRUE(stream_has_continuous_writer("annot_a"));
    TEST_ASSERT_EQUAL_UINT64(0, get_current_recording_id_for_stream("annot_a"));
}

/* The probe is per-stream: one stream recording says nothing about another. */
static void test_writer_probe_is_per_stream(void) {
    TEST_ASSERT_EQUAL_INT(0, register_mp4_writer_for_stream("annot_a", g_writer));

    TEST_ASSERT_TRUE(stream_has_continuous_writer("annot_a"));
    TEST_ASSERT_FALSE(stream_has_continuous_writer("annot_b"));
}

/*
 * Stopping the continuous recording thread unregisters the writer. That is the
 * durable "no continuous recording" signal — the one case where detection
 * legitimately falls back to writing its own clips.
 */
static void test_unregister_drops_the_writer(void) {
    TEST_ASSERT_EQUAL_INT(0, register_mp4_writer_for_stream("annot_a", g_writer));
    TEST_ASSERT_TRUE(stream_has_continuous_writer("annot_a"));

    unregister_mp4_writer_for_stream("annot_a");

    TEST_ASSERT_FALSE(stream_has_continuous_writer("annot_a"));
    TEST_ASSERT_EQUAL_UINT64(0, get_current_recording_id_for_stream("annot_a"));
}

/* A NULL or empty stream name must not be reported as recording. */
static void test_invalid_stream_name_has_no_writer(void) {
    TEST_ASSERT_FALSE(stream_has_continuous_writer(NULL));
    TEST_ASSERT_FALSE(stream_has_continuous_writer(""));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_unregistered_stream_has_no_writer);
    RUN_TEST(test_registered_writer_with_open_segment);
    RUN_TEST(test_registered_writer_between_segments_still_counts);
    RUN_TEST(test_writer_probe_is_per_stream);
    RUN_TEST(test_unregister_drops_the_writer);
    RUN_TEST(test_invalid_stream_name_has_no_writer);
    return UNITY_END();
}
