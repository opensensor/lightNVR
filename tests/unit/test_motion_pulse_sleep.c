/* Exercise real pulse timing without wall-clock sleeps. */
#include "../../src/web/api_handlers_motion.c"
#include "unity.h"
#include <errno.h>

static int sleep_calls, interruptions, effects;
static int64_t elapsed_ns, expected_ns;
static bool fail_sleep, supersede;

static int simulate_sleep(int64_t requested, struct timespec *remaining) {
    TEST_ASSERT_GREATER_THAN_INT64(0, requested);
    TEST_ASSERT_LESS_OR_EQUAL_INT64(500000000, requested);
    TEST_ASSERT_EQUAL_INT(0, effects);
    /* Waiting must not block acceptance of a newer motion action. */
    TEST_ASSERT_EQUAL_INT(0, pthread_mutex_trylock(&g_pulse_gen_mutex));
    if (supersede && sleep_calls == 0) pulse_gen_increment("camera");
    pthread_mutex_unlock(&g_pulse_gen_mutex);
    sleep_calls++;
    if (fail_sleep) { errno = EINVAL; return -1; }
    if (interruptions > 0) {
        interruptions--;
        int64_t elapsed = requested / 2;
        elapsed_ns += elapsed;
        if (remaining) {
            remaining->tv_sec = 0;
            remaining->tv_nsec = (long)(requested - elapsed);
        }
        errno = EINTR;
        return -1;
    }
    elapsed_ns += requested;
    return 0;
}
int __wrap_nanosleep(const struct timespec *requested, struct timespec *remaining) {
    return simulate_sleep((int64_t)requested->tv_sec * 1000000000 + requested->tv_nsec, remaining);
}
int __wrap_usleep(useconds_t usec) {
    return simulate_sleep((int64_t)usec * 1000, NULL);
}
static void stop_effect(const char *name) {
    TEST_ASSERT_EQUAL_STRING("camera", name);
    if (!fail_sleep) TEST_ASSERT_EQUAL_INT64(expected_ns, elapsed_ns);
    effects++;
}
int __wrap_close_external_motion_detections(const char *uuid, const char *name, time_t end) {
    (void)uuid; (void)end; stop_effect(name); return 0;
}
void __wrap_unified_detection_notify_motion(const char *name, bool active) {
    TEST_ASSERT_FALSE(active); stop_effect(name);
}
int __wrap_process_motion_event(const char *name, bool active, time_t now, bool propagated) {
    (void)now; TEST_ASSERT_FALSE(active); TEST_ASSERT_FALSE(propagated); stop_effect(name); return 0;
}
void setUp(void) {
    sleep_calls = interruptions = effects = 0;
    elapsed_ns = expected_ns = 0;
    fail_sleep = supersede = false;
    memset(g_pulse_gen, 0, sizeof(g_pulse_gen));
    g_pulse_gen_count = 0;
    g_pulse_gen_sequence = 0;
    pulse_gen_increment("camera");
}
void tearDown(void) {
    TEST_ASSERT_EQUAL_INT(0, pthread_mutex_trylock(&g_pulse_gen_mutex));
    pthread_mutex_unlock(&g_pulse_gen_mutex);
}
static void run_pulse(int milliseconds) {
    motion_pulse_task_t *task = calloc(1, sizeof(*task));
    TEST_ASSERT_NOT_NULL(task);
    strcpy(task->stream_name, "camera");
    task->generation = 1;
    task->trigger_recording = true;
    task->duration_ms = milliseconds;
    expected_ns = (int64_t)milliseconds * 1000000;
    motion_pulse_worker(task);
}
static void test_interruptions_preserve_remaining_time(void) {
    interruptions = 2;
    run_pulse(1201);
    TEST_ASSERT_EQUAL_INT64(1201000000, elapsed_ns);
    TEST_ASSERT_EQUAL_INT(5, sleep_calls);
    TEST_ASSERT_EQUAL_INT(3, effects);
}
static void test_subchunk_and_maximum_duration(void) {
    run_pulse(1);
    TEST_ASSERT_EQUAL_INT(1, sleep_calls);
    TEST_ASSERT_EQUAL_INT(3, effects);
    effects = sleep_calls = 0; elapsed_ns = 0;
    run_pulse(MOTION_PULSE_MAX_MS);
    TEST_ASSERT_EQUAL_INT(MOTION_PULSE_MAX_MS / 500, sleep_calls);
    TEST_ASSERT_EQUAL_INT(3, effects);
}
static void test_superseded_pulse_does_not_stop_new_action(void) {
    supersede = true; interruptions = 1;
    run_pulse(501);
    TEST_ASSERT_EQUAL_INT64(501000000, elapsed_ns);
    TEST_ASSERT_EQUAL_INT(0, effects);
}
static void test_sleep_failure_ends_current_pulse_without_retry_loop(void) {
    fail_sleep = true;
    run_pulse(1201);
    TEST_ASSERT_EQUAL_INT(1, sleep_calls);
    TEST_ASSERT_EQUAL_INT(3, effects);
}
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_interruptions_preserve_remaining_time);
    RUN_TEST(test_subchunk_and_maximum_duration);
    RUN_TEST(test_superseded_pulse_does_not_stop_new_action);
    RUN_TEST(test_sleep_failure_ends_current_pulse_without_retry_loop);
    return UNITY_END();
}
