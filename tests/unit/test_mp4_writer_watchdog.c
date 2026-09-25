#include "unity.h"

#include <pthread.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "core/config.h"
#include "core/logger.h"
#include "video/mp4_writer.h"
#include "video/mp4_writer_thread.h"

static mp4_writer_t writer;
static mp4_writer_thread_t thread_ctx;

extern int register_mp4_writer_for_stream(const char *stream_name, mp4_writer_t *writer);
extern void unregister_mp4_writer_for_stream(const char *stream_name);
extern mp4_writer_t *get_mp4_writer_for_stream(const char *stream_name);

static void *exit_immediately(void *arg) {
    (void)arg;
    return NULL;
}

void setUp(void) {
    memset(&writer, 0, sizeof(writer));
    memset(&thread_ctx, 0, sizeof(thread_ctx));

    strcpy(writer.stream_name, "watchdog-test");
    writer.creation_time = time(NULL);
    writer.thread_ctx = &thread_ctx;
    thread_ctx.running = 1;
    g_config.audio_disabled = false;
}

void tearDown(void) {
}

static void test_null_writer_is_not_recording(void) {
    TEST_ASSERT_EQUAL_INT(0, mp4_writer_is_recording(NULL));
}

static void test_recent_activity_is_recording(void) {
    writer.last_packet_time = time(NULL);

    TEST_ASSERT_EQUAL_INT(1, mp4_writer_is_recording(&writer));
}

static void test_default_timeout_marks_stale_recording_dead(void) {
    writer.segment_duration = 30;
    writer.last_packet_time = time(NULL) - 46;

    TEST_ASSERT_EQUAL_INT(0, mp4_writer_is_recording(&writer));
}

static void test_long_segment_extends_watchdog_timeout(void) {
    writer.segment_duration = 60;
    writer.last_packet_time = time(NULL) - 50;

    TEST_ASSERT_EQUAL_INT(1, mp4_writer_is_recording(&writer));
}

static void test_long_segment_still_marks_truly_stale_recording_dead(void) {
    writer.segment_duration = 60;
    writer.last_packet_time = time(NULL) - 76;

    TEST_ASSERT_EQUAL_INT(0, mp4_writer_is_recording(&writer));
}

static void test_audio_setting_is_enabled_when_policy_allows_it(void) {
    mp4_writer_set_audio(&writer, 1);
    TEST_ASSERT_EQUAL_INT(1, writer.has_audio);
}

static void test_global_audio_policy_overrides_writer_setting(void) {
    g_config.audio_disabled = true;
    mp4_writer_set_audio(&writer, 1);
    TEST_ASSERT_EQUAL_INT(0, writer.has_audio);
}

static void test_stop_recording_thread_claims_the_context_exactly_once(void) {
    // Two stoppers racing on one writer (the outer recording thread restarting
    // its reader and a close from another thread) used to both join and free
    // the same context. The first caller now claims it; the second sees NULL.
    mp4_writer_thread_t *tctx = calloc(1, sizeof(*tctx));
    TEST_ASSERT_NOT_NULL(tctx);
    tctx->running = 1;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&tctx->thread, NULL, exit_immediately, NULL));
    writer.thread_ctx = tctx;
    writer.shutdown_component_id = -1;

    mp4_writer_stop_recording_thread(&writer);
    TEST_ASSERT_NULL(writer.thread_ctx);
    TEST_ASSERT_EQUAL_INT(0, mp4_writer_is_recording(&writer));

    // Idempotent: nothing left to claim, so nothing is joined or freed twice.
    mp4_writer_stop_recording_thread(&writer);
    TEST_ASSERT_NULL(writer.thread_ctx);
}

static void test_registry_refuses_a_second_writer_for_a_live_stream(void) {
    // A registered writer belongs to a live recording thread. Replacing and
    // closing it from a second thread freed memory the first was still using
    // (the 0.42.5 crash loop on unreachable cameras).
    static mp4_writer_t first, second;
    memset(&first, 0, sizeof(first));
    memset(&second, 0, sizeof(second));
    strcpy(first.stream_name, "registry-test");
    strcpy(second.stream_name, "registry-test");
    g_config.max_streams = 4;

    TEST_ASSERT_EQUAL_INT(0, register_mp4_writer_for_stream("registry-test", &first));
    TEST_ASSERT_EQUAL_INT(-1, register_mp4_writer_for_stream("registry-test", &second));
    TEST_ASSERT_EQUAL_PTR(&first, get_mp4_writer_for_stream("registry-test"));
    // Re-registering the same writer is harmless.
    TEST_ASSERT_EQUAL_INT(0, register_mp4_writer_for_stream("registry-test", &first));

    unregister_mp4_writer_for_stream("registry-test");
    TEST_ASSERT_NULL(get_mp4_writer_for_stream("registry-test"));
    TEST_ASSERT_EQUAL_INT(0, register_mp4_writer_for_stream("registry-test", &second));
    TEST_ASSERT_EQUAL_PTR(&second, get_mp4_writer_for_stream("registry-test"));
    unregister_mp4_writer_for_stream("registry-test");
}

int main(void) {
    init_logger();

    UNITY_BEGIN();
    RUN_TEST(test_null_writer_is_not_recording);
    RUN_TEST(test_recent_activity_is_recording);
    RUN_TEST(test_default_timeout_marks_stale_recording_dead);
    RUN_TEST(test_long_segment_extends_watchdog_timeout);
    RUN_TEST(test_long_segment_still_marks_truly_stale_recording_dead);
    RUN_TEST(test_audio_setting_is_enabled_when_policy_allows_it);
    RUN_TEST(test_global_audio_policy_overrides_writer_setting);
    RUN_TEST(test_stop_recording_thread_claims_the_context_exactly_once);
    RUN_TEST(test_registry_refuses_a_second_writer_for_a_live_stream);
    int result = UNITY_END();

    shutdown_logger();

    return result;
}
