/* Focused registry-disposal regressions; media stop calls are deterministic
 * link seams so no camera, worker thread or external service is needed. */
#define _POSIX_C_SOURCE 200809L
#include <pthread.h>
#include <signal.h>
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>

#include "unity.h"
#include "core/config.h"
#include "video/stream_state.h"

static uintptr_t tracked[4];
static unsigned destroys[4];
static int destroy_error;
static bool check_reentry;
static unsigned stop_calls;

int __real_pthread_mutex_destroy(pthread_mutex_t *mutex);
int __wrap_pthread_mutex_destroy(pthread_mutex_t *mutex) {
    for (size_t i = 0; i < 4; i++) {
        if (tracked[i] == (uintptr_t)mutex) destroys[i]++;
    }
    int result = __real_pthread_mutex_destroy(mutex);
    if (result != 0) destroy_error = result;
    return result;
}

static stream_config_t config_for(const char *name) {
    stream_config_t config;
    memset(&config, 0, sizeof(config));
    snprintf(config.name, sizeof(config.name), "%s", name);
    snprintf(config.url, sizeof(config.url), "rtsp://127.0.0.1/synthetic");
    config.enabled = true;
    config.streaming_enabled = true;
    return config;
}

int __wrap_stream_stop_hls(const char *name) {
    stop_calls++;
    if (check_reentry) {
        /* Real HLS shutdown performs this same registry lookup. */
        stream_state_manager_t *state = get_stream_state_by_name(name);
        if (!state || strcmp(state->name, name) != 0) _exit(11);
        if (get_stream_state_count() != 1) _exit(12);
        stream_config_t late = config_for("late-admission");
        if (create_stream_state(&late) != NULL) _exit(13);
        if (remove_stream_state(state) != -1) _exit(14);
        shutdown_stream_state_manager(); /* Must be a no-op during teardown. */
        if (get_stream_state_by_name(name) != state) _exit(15);
    }
    return 0;
}

int __wrap_stop_mp4_recording(const char *name) { (void)name; return 0; }
void __wrap_reset_timestamp_tracker(const char *name) { (void)name; }
void __wrap_remove_timestamp_tracker(const char *name) { (void)name; }

void setUp(void) {
    memset(tracked, 0, sizeof(tracked));
    memset(destroys, 0, sizeof(destroys));
    destroy_error = 0;
    check_reentry = false;
    stop_calls = 0;
}
void tearDown(void) { shutdown_stream_state_manager(); }

static void track(stream_state_manager_t *state, size_t slot) {
    tracked[slot] = (uintptr_t)&state->mutex;
    tracked[slot + 1] = (uintptr_t)&state->state_mutex;
}

void test_removal_destroys_both_mutexes_once(void) {
    TEST_ASSERT_EQUAL_INT(0, init_stream_state_manager(2));
    stream_config_t config = config_for("remove");
    stream_state_manager_t *state = create_stream_state(&config);
    TEST_ASSERT_NOT_NULL(state);
    track(state, 0);
    TEST_ASSERT_EQUAL_INT(0, remove_stream_state(state));
    TEST_ASSERT_EQUAL_UINT(1, destroys[0]);
    TEST_ASSERT_EQUAL_UINT(1, destroys[1]);
    TEST_ASSERT_EQUAL_INT(0, destroy_error);
    TEST_ASSERT_NULL(get_stream_state_by_name("remove"));
}

void test_extra_reference_refusal_destroys_neither_mutex(void) {
    TEST_ASSERT_EQUAL_INT(0, init_stream_state_manager(2));
    stream_config_t config = config_for("borrowed");
    stream_state_manager_t *state = create_stream_state(&config);
    TEST_ASSERT_NOT_NULL(state);
    track(state, 0);
    TEST_ASSERT_EQUAL_INT(2, stream_state_add_ref(state, STREAM_COMPONENT_HLS));
    TEST_ASSERT_EQUAL_INT(-1, remove_stream_state(state));
    TEST_ASSERT_EQUAL_UINT(0, destroys[0]);
    TEST_ASSERT_EQUAL_UINT(0, destroys[1]);
    TEST_ASSERT_EQUAL_PTR(state, get_stream_state_by_name("borrowed"));
    TEST_ASSERT_EQUAL_INT(1, stream_state_release_ref(state, STREAM_COMPONENT_HLS));
    TEST_ASSERT_EQUAL_INT(0, remove_stream_state(state));
    TEST_ASSERT_EQUAL_UINT(1, destroys[0]);
    TEST_ASSERT_EQUAL_UINT(1, destroys[1]);
    TEST_ASSERT_EQUAL_INT(0, destroy_error);
}

void test_shutdown_destroys_every_mutex_and_is_idempotent(void) {
    TEST_ASSERT_EQUAL_INT(0, init_stream_state_manager(2));
    stream_config_t first = config_for("first"), second = config_for("second");
    stream_state_manager_t *a = create_stream_state(&first);
    stream_state_manager_t *b = create_stream_state(&second);
    TEST_ASSERT_NOT_NULL(a);
    TEST_ASSERT_NOT_NULL(b);
    track(a, 0); track(b, 2);
    shutdown_stream_state_manager();
    for (size_t i = 0; i < 4; i++) TEST_ASSERT_EQUAL_UINT(1, destroys[i]);
    TEST_ASSERT_EQUAL_INT(0, destroy_error);
    TEST_ASSERT_EQUAL_INT(0, get_stream_state_count());
    shutdown_stream_state_manager();
    for (size_t i = 0; i < 4; i++) TEST_ASSERT_EQUAL_UINT(1, destroys[i]);
}

void test_repeated_lifecycle_disposes_once_per_generation(void) {
    for (int cycle = 0; cycle < 32; cycle++) {
        memset(tracked, 0, sizeof(tracked));
        memset(destroys, 0, sizeof(destroys));
        TEST_ASSERT_EQUAL_INT(0, init_stream_state_manager(1));
        stream_config_t config = config_for("repeat");
        stream_state_manager_t *state = create_stream_state(&config);
        TEST_ASSERT_NOT_NULL(state);
        track(state, 0);
        if (cycle % 2 == 0) TEST_ASSERT_EQUAL_INT(0, remove_stream_state(state));
        shutdown_stream_state_manager();
        TEST_ASSERT_EQUAL_UINT(1, destroys[0]);
        TEST_ASSERT_EQUAL_UINT(1, destroys[1]);
        TEST_ASSERT_EQUAL_INT(0, destroy_error);
    }
}

void test_stop_callback_can_lookup_but_cannot_mutate_tearing_down_registry(void) {
    fflush(NULL);
    pid_t child = fork();
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, child);
    if (child == 0) {
        alarm(3); /* A registry lock recursion must fail, not hang ctest. */
        if (init_stream_state_manager(1) != 0) _exit(20);
        stream_config_t config = config_for("reentrant");
        stream_state_manager_t *state = create_stream_state(&config);
        if (!state) _exit(21);
        track(state, 0);
        state->state = STREAM_STATE_ACTIVE;
        check_reentry = true;
        shutdown_stream_state_manager();
        if (stop_calls != 1 || destroys[0] != 1 || destroys[1] != 1 || destroy_error) _exit(22);
        if (get_stream_state_count() != 0) _exit(23);
        alarm(0);
        _exit(0);
    }
    int status = 0;
    TEST_ASSERT_EQUAL_INT(child, waitpid(child, &status, 0));
    TEST_ASSERT_TRUE_MESSAGE(WIFEXITED(status), "shutdown stop callback exceeded its deadline (registry lock recursion)");
    TEST_ASSERT_EQUAL_INT_MESSAGE(0, WEXITSTATUS(status), "shutdown violated callback lookup or teardown ownership");
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_removal_destroys_both_mutexes_once);
    RUN_TEST(test_extra_reference_refusal_destroys_neither_mutex);
    RUN_TEST(test_shutdown_destroys_every_mutex_and_is_idempotent);
    RUN_TEST(test_repeated_lifecycle_disposes_once_per_generation);
    RUN_TEST(test_stop_callback_can_lookup_but_cannot_mutate_tearing_down_registry);
    return UNITY_END();
}
