/**
 * @file test_audio_voice_enhancement.c
 * @brief Layer 3 Unity tests for the voice-enhancement opt-in staging logic in
 *        src/video/mp4_writer_utils.c (discussion #395).
 *
 * The real filter graph is deferred to a follow-up PR; what's exercised here is
 * the lifecycle plumbing that routes a per-stream opt-in to the audio
 * transcoder pool: set_audio_voice_enhancement() (live-flip vs. staged),
 * get_audio_voice_enhancement() (read-back), and cleanup_audio_transcoder()
 * dropping any staged entry.  No transcoder slot is ever initialized here (that
 * needs a real PCM frame), so every code path under test takes the "staged"
 * branch — which is exactly the branch that has to be correct on the first
 * packet of a recording session.
 */

#define _POSIX_C_SOURCE 200809L

#include <string.h>

#include "unity.h"
#include "core/config.h"
#include "core/logger.h"
#include "video/mp4_writer_internal.h"

extern config_t g_config;

/* Stream names used across the tests; cleaned up in tearDown so the static
 * staging table never leaks state from one test into the next. */
static const char *kNames[] = {
    "ve_a", "ve_b", "ve_c", "ve_full_0", "ve_full_1",
};

void setUp(void) {
    g_config.max_streams = MAX_STREAMS;
}

void tearDown(void) {
    /* Drop any staged opt-ins so tests stay independent. */
    g_config.max_streams = MAX_STREAMS;
    for (size_t i = 0; i < sizeof(kNames) / sizeof(kNames[0]); i++) {
        cleanup_audio_transcoder(kNames[i]);
    }
}

/* A stream with nothing staged reports "not enhanced". */
static void test_default_is_disabled(void) {
    TEST_ASSERT_FALSE(get_audio_voice_enhancement("ve_a"));
}

/* Staging an opt-in before any transcoder slot exists is readable back. */
static void test_stage_enable_then_read(void) {
    set_audio_voice_enhancement("ve_a", true);
    TEST_ASSERT_TRUE(get_audio_voice_enhancement("ve_a"));
}

/* Re-setting an already-staged stream updates the value in place (no duplicate
 * slot), covering the "matching pending entry" branch of the setter. */
static void test_restage_updates_in_place(void) {
    set_audio_voice_enhancement("ve_b", true);
    TEST_ASSERT_TRUE(get_audio_voice_enhancement("ve_b"));

    set_audio_voice_enhancement("ve_b", false);
    TEST_ASSERT_FALSE(get_audio_voice_enhancement("ve_b"));

    set_audio_voice_enhancement("ve_b", true);
    TEST_ASSERT_TRUE(get_audio_voice_enhancement("ve_b"));
}

/* Independent streams keep independent staged state. */
static void test_multiple_streams_independent(void) {
    set_audio_voice_enhancement("ve_a", true);
    set_audio_voice_enhancement("ve_b", false);
    set_audio_voice_enhancement("ve_c", true);

    TEST_ASSERT_TRUE(get_audio_voice_enhancement("ve_a"));
    TEST_ASSERT_FALSE(get_audio_voice_enhancement("ve_b"));
    TEST_ASSERT_TRUE(get_audio_voice_enhancement("ve_c"));
}

/* cleanup_audio_transcoder() drops the staged opt-in, so a later read reverts
 * to the default. */
static void test_cleanup_drops_staged_optin(void) {
    set_audio_voice_enhancement("ve_a", true);
    TEST_ASSERT_TRUE(get_audio_voice_enhancement("ve_a"));

    cleanup_audio_transcoder("ve_a");
    TEST_ASSERT_FALSE(get_audio_voice_enhancement("ve_a"));
}

/* NULL / empty stream names are ignored by the setter and read back false. */
static void test_null_and_empty_names_are_ignored(void) {
    set_audio_voice_enhancement(NULL, true);
    set_audio_voice_enhancement("", true);

    TEST_ASSERT_FALSE(get_audio_voice_enhancement(NULL));
    TEST_ASSERT_FALSE(get_audio_voice_enhancement(""));
}

/* When every staging slot is occupied, a new distinct stream can't be staged —
 * exercises the "no free slot" branch.  Shrinking max_streams to 1 makes the
 * single usable slot fill immediately. */
static void test_no_free_staging_slot(void) {
    g_config.max_streams = 1;

    set_audio_voice_enhancement("ve_full_0", true);
    TEST_ASSERT_TRUE(get_audio_voice_enhancement("ve_full_0"));

    /* No slot left for a different stream; it stays at the default. */
    set_audio_voice_enhancement("ve_full_1", true);
    TEST_ASSERT_FALSE(get_audio_voice_enhancement("ve_full_1"));

    /* The occupant is untouched. */
    TEST_ASSERT_TRUE(get_audio_voice_enhancement("ve_full_0"));

    cleanup_audio_transcoder("ve_full_0");
    g_config.max_streams = MAX_STREAMS;
}

int main(void) {
    init_logger();

    UNITY_BEGIN();
    RUN_TEST(test_default_is_disabled);
    RUN_TEST(test_stage_enable_then_read);
    RUN_TEST(test_restage_updates_in_place);
    RUN_TEST(test_multiple_streams_independent);
    RUN_TEST(test_cleanup_drops_staged_optin);
    RUN_TEST(test_null_and_empty_names_are_ignored);
    RUN_TEST(test_no_free_staging_slot);
    int result = UNITY_END();

    shutdown_logger();
    return result;
}
