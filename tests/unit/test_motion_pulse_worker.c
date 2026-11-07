/* Exercise the real deferred worker with a competing trigger at each stop
 * side effect. No camera, database, or timing-dependent sleeps are needed. */
#include "../../src/web/api_handlers_motion.c"
#include "unity.h"
#include <errno.h>

static int close_calls, notify_calls, fanout_calls;
static int concurrent_attempts, accepted_actions, unexpected_lock_errors;
static int fixture_errors;

static void *attempt_new_action(void *unused) {
    (void)unused;
    int rc = pthread_mutex_trylock(&g_pulse_gen_mutex);
    if (rc == 0) {
        pulse_gen_increment("camera");
        accepted_actions++;
        pthread_mutex_unlock(&g_pulse_gen_mutex);
    } else if (rc != EBUSY) {
        unexpected_lock_errors++;
    }
    return NULL;
}

static void compete_with_stop(void) {
    pthread_t thread;
    concurrent_attempts++;
    if (pthread_create(&thread, NULL, attempt_new_action, NULL) != 0) {
        fixture_errors++;
        return;
    }
    if (pthread_join(thread, NULL) != 0) fixture_errors++;
}

int __wrap_close_external_motion_detections(const char *uuid, const char *stream, time_t end) {
    if (strcmp(uuid, "captured-camera") || strcmp(stream, "camera") || end <= 0) fixture_errors++;
    close_calls++;
    compete_with_stop();
    return 0;
}

void __wrap_unified_detection_notify_motion(const char *stream, bool active) {
    if (strcmp(stream, "camera") || active) fixture_errors++;
    notify_calls++;
    compete_with_stop();
}

int __wrap_process_motion_event(const char *stream, bool active, time_t timestamp, bool propagated) {
    if (strcmp(stream, "camera") || active || propagated || timestamp <= 0) fixture_errors++;
    fanout_calls++;
    compete_with_stop();
    return 0;
}

void setUp(void) {
    close_calls = notify_calls = fanout_calls = 0;
    concurrent_attempts = accepted_actions = unexpected_lock_errors = fixture_errors = 0;
    memset(g_pulse_gen, 0, sizeof(g_pulse_gen));
    g_pulse_gen_count = 0;
    pthread_mutex_lock(&g_pulse_gen_mutex);
    pulse_gen_increment("camera");
    pthread_mutex_unlock(&g_pulse_gen_mutex);
}

void tearDown(void) {
    // All worker paths must release the generation mutex.
    int rc = pthread_mutex_trylock(&g_pulse_gen_mutex);
    if (rc == 0) pthread_mutex_unlock(&g_pulse_gen_mutex);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_INT(0, fixture_errors);
    TEST_ASSERT_EQUAL_INT(0, unexpected_lock_errors);
}

static void run_worker(uint64_t generation, bool recording) {
    motion_pulse_task_t *task = calloc(1, sizeof(*task));
    TEST_ASSERT_NOT_NULL(task);
    safe_strcpy(task->stream_name, "camera", sizeof(task->stream_name), 0);
    safe_strcpy(task->camera_uuid, "captured-camera", sizeof(task->camera_uuid), 0);
    task->generation = generation;
    task->trigger_recording = recording;
    // Zero delay exercises dispatch directly; the production sleep is unchanged.
    motion_pulse_worker(task);
}

static void test_current_pulse_serializes_every_stop_effect(void) {
    run_worker(1, true);
    TEST_ASSERT_EQUAL_INT(1, close_calls);
    TEST_ASSERT_EQUAL_INT(1, notify_calls);
    TEST_ASSERT_EQUAL_INT(1, fanout_calls);
    TEST_ASSERT_EQUAL_INT(3, concurrent_attempts);
    TEST_ASSERT_EQUAL_INT(0, accepted_actions);
    // A new trigger can advance the generation after the stop is complete.
    attempt_new_action(NULL);
    TEST_ASSERT_EQUAL_INT(1, accepted_actions);
}

static void test_superseded_pulse_has_no_stop_effects(void) {
    pthread_mutex_lock(&g_pulse_gen_mutex);
    pulse_gen_increment("camera");
    pthread_mutex_unlock(&g_pulse_gen_mutex);
    run_worker(1, true);
    TEST_ASSERT_EQUAL_INT(0, close_calls + notify_calls + fanout_calls);
    TEST_ASSERT_EQUAL_INT(0, concurrent_attempts);
}

static void test_untracked_pulse_still_serializes_stop_effects(void) {
    run_worker(0, true);
    TEST_ASSERT_EQUAL_INT(3, concurrent_attempts);
    TEST_ASSERT_EQUAL_INT(0, accepted_actions);
}

static void test_untracked_stale_pulse_has_no_stop_effects_after_newer_action(void) {
    pthread_mutex_lock(&g_pulse_gen_mutex);
    pulse_gen_increment("camera");
    pthread_mutex_unlock(&g_pulse_gen_mutex);
    run_worker(0, true);
    TEST_ASSERT_EQUAL_INT(0, close_calls + notify_calls + fanout_calls);
    TEST_ASSERT_EQUAL_INT(0, concurrent_attempts);
    TEST_ASSERT_EQUAL_INT(0, accepted_actions);
}

static void test_annotation_only_pulse_skips_recording_notification(void) {
    run_worker(1, false);
    TEST_ASSERT_EQUAL_INT(1, close_calls);
    TEST_ASSERT_EQUAL_INT(0, notify_calls);
    TEST_ASSERT_EQUAL_INT(1, fanout_calls);
    TEST_ASSERT_EQUAL_INT(2, concurrent_attempts);
    TEST_ASSERT_EQUAL_INT(0, accepted_actions);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_current_pulse_serializes_every_stop_effect);
    RUN_TEST(test_superseded_pulse_has_no_stop_effects);
    RUN_TEST(test_untracked_pulse_still_serializes_stop_effects);
    RUN_TEST(test_untracked_stale_pulse_has_no_stop_effects_after_newer_action);
    RUN_TEST(test_annotation_only_pulse_skips_recording_notification);
    return UNITY_END();
}
