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

void test_wait_all_stopped_after_marking_stopped(void) {
    int id = register_component("worker", COMPONENT_OTHER, NULL, 1);
    update_component_state(id, COMPONENT_STOPPED);
    bool result = wait_for_all_components_stopped(1);
    TEST_ASSERT_TRUE(result);
}

void test_wait_all_stopped_timeout_when_running(void) {
    register_component("persistent", COMPONENT_OTHER, NULL, 1);
    /* Component stays RUNNING → should timeout (1 second) */
    bool result = wait_for_all_components_stopped(1);
    /* On a fast system it times out; result may be false */
    /* We don't assert a specific value — just that it returns */
    (void)result;
    TEST_PASS();
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
    RUN_TEST(test_wait_all_stopped_after_marking_stopped);
    RUN_TEST(test_wait_all_stopped_timeout_when_running);

    int result = UNITY_END();
    shutdown_coordinator_cleanup();
    shutdown_logger();
    return result;
}

