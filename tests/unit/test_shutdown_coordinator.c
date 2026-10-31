/**
 * @file test_shutdown_coordinator.c
 * @brief Layer 2 unit tests — shutdown coordinator lifecycle
 *
 * Tests init/cleanup, register_component, state transitions,
 * initiate_shutdown, is_shutdown_initiated, and
 * wait_for_all_components_stopped.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <string.h>
#include <time.h>
#include <pthread.h>
#include "unity.h"
#include "core/shutdown_coordinator.h"
#include "core/logger.h"

/* ---- Unity boilerplate: reinit coordinator for each test ---- */
void setUp(void) {
    init_shutdown_coordinator();
}
void tearDown(void) {
    shutdown_coordinator_cleanup();
}

/* ================================================================
 * init / cleanup lifecycle
 * ================================================================ */

void test_init_succeeds(void) {
    /* setUp already called init; just check it doesn't corrupt state */
    TEST_ASSERT_FALSE(is_shutdown_initiated());
}

void test_double_cleanup_safe(void) {
    shutdown_coordinator_cleanup();
    shutdown_coordinator_cleanup(); /* should not crash */
    init_shutdown_coordinator();   /* re-init so tearDown works */
    TEST_PASS();
}

/* ================================================================
 * register_component
 * ================================================================ */

void test_register_component_returns_valid_id(void) {
    int id = register_component("test_comp", COMPONENT_OTHER, NULL, 5);
    TEST_ASSERT_GREATER_OR_EQUAL(0, id);
}

void test_register_multiple_components(void) {
    int id1 = register_component("comp1", COMPONENT_DETECTION_THREAD, NULL, 3);
    int id2 = register_component("comp2", COMPONENT_SERVER_THREAD, NULL, 7);
    TEST_ASSERT_GREATER_OR_EQUAL(0, id1);
    TEST_ASSERT_GREATER_OR_EQUAL(0, id2);
    TEST_ASSERT_NOT_EQUAL(id1, id2);
}

/* ================================================================
 * update / get component state
 * ================================================================ */

void test_component_starts_running(void) {
    int id = register_component("runner", COMPONENT_OTHER, NULL, 1);
    TEST_ASSERT_EQUAL_INT(COMPONENT_RUNNING, get_component_state(id));
}

void test_component_state_transition_stopping(void) {
    int id = register_component("runner", COMPONENT_OTHER, NULL, 1);
    update_component_state(id, COMPONENT_STOPPING);
    TEST_ASSERT_EQUAL_INT(COMPONENT_STOPPING, get_component_state(id));
}

void test_component_state_transition_stopped(void) {
    int id = register_component("runner", COMPONENT_OTHER, NULL, 1);
    update_component_state(id, COMPONENT_STOPPED);
    TEST_ASSERT_EQUAL_INT(COMPONENT_STOPPED, get_component_state(id));
}

/* ================================================================
 * initiate_shutdown / is_shutdown_initiated
 * ================================================================ */

void test_shutdown_not_initiated_initially(void) {
    TEST_ASSERT_FALSE(is_shutdown_initiated());
}

void test_initiate_shutdown_sets_flag(void) {
    initiate_shutdown();
    TEST_ASSERT_TRUE(is_shutdown_initiated());
}

/* ================================================================
 * wait_for_all_components_stopped
 * ================================================================ */

void test_wait_all_stopped_no_components(void) {
    /* No components registered → already "all stopped" */
    bool result = wait_for_all_components_stopped(1);
    TEST_ASSERT_TRUE(result);
}

/* The coordinator starts in the "all stopped" state, so when nothing ever
 * registers, wait_for_all_components_stopped() must return immediately rather
 * than blocking for the full timeout. This guards the shutdown-latency fix:
 * with a 10s timeout the call should complete in well under a second. */
void test_wait_all_stopped_no_components_returns_immediately(void) {
    struct timespec start, end;
    clock_gettime(CLOCK_MONOTONIC, &start);

    bool result = wait_for_all_components_stopped(10);

    clock_gettime(CLOCK_MONOTONIC, &end);
    long elapsed_ms = (end.tv_sec - start.tv_sec) * 1000 +
                      (end.tv_nsec - start.tv_nsec) / 1000000;

    TEST_ASSERT_TRUE(result);
    TEST_ASSERT_LESS_THAN_INT(500, elapsed_ms);
}

/* After a component registers, the coordinator is no longer "all stopped", so a
 * wait must block until the timeout (and then force-stop). This is the
 * complementary guard ensuring the fast-path doesn't fire while work is live. */
void test_wait_blocks_until_timeout_when_component_registered(void) {
    register_component("late_stopper", COMPONENT_OTHER, NULL, 50);

    struct timespec start, end;
    clock_gettime(CLOCK_MONOTONIC, &start);

    bool stopped = wait_for_all_components_stopped(1);

    clock_gettime(CLOCK_MONOTONIC, &end);
    long elapsed_ms = (end.tv_sec - start.tv_sec) * 1000 +
                      (end.tv_nsec - start.tv_nsec) / 1000000;

    /* Should have actually waited for roughly the 1s timeout, not returned
     * instantly via the fast-path. */
    TEST_ASSERT_GREATER_OR_EQUAL_INT(900, elapsed_ms);
    TEST_ASSERT_LESS_THAN_INT(2500, elapsed_ms);
    TEST_ASSERT_FALSE_MESSAGE(stopped, "A deadline is not graceful completion");
}

void test_wait_all_stopped_after_marking_stopped(void) {
    int id = register_component("worker", COMPONENT_OTHER, NULL, 1);
    update_component_state(id, COMPONENT_STOPPED);
    bool result = wait_for_all_components_stopped(1);
    TEST_ASSERT_TRUE(result);
}

void test_wait_all_stopped_timeout_when_running(void) {
    int id = register_component("persistent", COMPONENT_OTHER, NULL, 1);
    bool result = wait_for_all_components_stopped(0);
    TEST_ASSERT_FALSE_MESSAGE(result, "Timeout must remain distinguishable from stopped");
    TEST_ASSERT_EQUAL_INT_MESSAGE(COMPONENT_RUNNING, get_component_state(id),
                                 "A timeout cannot assert that a live component exited");
}

static void *finish_component(void *argument) {
    int id = *(int *)argument;
    struct timespec delay = { .tv_nsec = 50000000 };
    nanosleep(&delay, NULL);
    update_component_state(id, COMPONENT_STOPPED);
    return NULL;
}

void test_wait_observes_actual_component_completion(void) {
    int id = register_component("finishes", COMPONENT_OTHER, NULL, 1);
    pthread_t worker;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&worker, NULL, finish_component, &id));
    bool stopped = wait_for_all_components_stopped(2);
    TEST_ASSERT_EQUAL_INT(0, pthread_join(worker, NULL));
    TEST_ASSERT_TRUE(stopped);
    TEST_ASSERT_EQUAL_INT(COMPONENT_STOPPED, get_component_state(id));
}

typedef struct { int id; atomic_bool finished; } spurious_context_t;
static void *wake_then_finish(void *argument) {
    spurious_context_t *context = argument;
    struct timespec delay = { .tv_nsec = 50000000 };
    nanosleep(&delay, NULL);
    shutdown_coordinator_t *coordinator = get_shutdown_coordinator();
    pthread_mutex_lock(&coordinator->mutex);
    pthread_cond_broadcast(&coordinator->all_stopped_cond);
    pthread_mutex_unlock(&coordinator->mutex);
    nanosleep(&delay, NULL);
    atomic_store(&context->finished, true);
    update_component_state(context->id, COMPONENT_STOPPED);
    return NULL;
}

void test_spurious_wake_does_not_claim_completion(void) {
    spurious_context_t context = {
        .id = register_component("spurious", COMPONENT_OTHER, NULL, 1),
        .finished = false,
    };
    pthread_t worker;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&worker, NULL, wake_then_finish, &context));
    bool stopped = wait_for_all_components_stopped(2);
    bool actually_finished = atomic_load(&context.finished);
    TEST_ASSERT_EQUAL_INT(0, pthread_join(worker, NULL));
    TEST_ASSERT_TRUE(stopped);
    TEST_ASSERT_TRUE_MESSAGE(actually_finished, "Condition wake is not component completion");
}

void test_repeated_restart_reuses_stopped_slots(void) {
    int first = -1;
    for (int generation = 0; generation < MAX_COMPONENTS * 2; generation++) {
        int id = register_component("restarting_recorder", COMPONENT_OTHER, NULL, 1);
        TEST_ASSERT_GREATER_OR_EQUAL_INT(0, id);
        if (generation == 0) first = id;
        TEST_ASSERT_EQUAL_INT(first, id);
        TEST_ASSERT_EQUAL_INT(COMPONENT_RUNNING, get_component_state(id));
        update_component_state(id, COMPONENT_STOPPED);
        TEST_ASSERT_TRUE(wait_for_all_components_stopped(0));
    }
}

/* ================================================================
 * main
 * ================================================================ */

int main(void) {
    init_logger();

    UNITY_BEGIN();

    RUN_TEST(test_init_succeeds);
    RUN_TEST(test_double_cleanup_safe);

    RUN_TEST(test_register_component_returns_valid_id);
    RUN_TEST(test_register_multiple_components);

    RUN_TEST(test_component_starts_running);
    RUN_TEST(test_component_state_transition_stopping);
    RUN_TEST(test_component_state_transition_stopped);

    RUN_TEST(test_shutdown_not_initiated_initially);
    RUN_TEST(test_initiate_shutdown_sets_flag);

    RUN_TEST(test_wait_all_stopped_no_components);
    RUN_TEST(test_wait_all_stopped_no_components_returns_immediately);
    RUN_TEST(test_wait_blocks_until_timeout_when_component_registered);
    RUN_TEST(test_wait_all_stopped_after_marking_stopped);
    RUN_TEST(test_wait_all_stopped_timeout_when_running);
    RUN_TEST(test_wait_observes_actual_component_completion);
    RUN_TEST(test_repeated_restart_reuses_stopped_slots);
    RUN_TEST(test_spurious_wake_does_not_claim_completion);

    int result = UNITY_END();
    shutdown_coordinator_cleanup();
    shutdown_logger();
    return result;
}

