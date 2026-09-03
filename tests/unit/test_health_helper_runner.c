#define _POSIX_C_SOURCE 200809L

#include <string.h>
#include <time.h>

#include "telemetry/health_helper_runner.h"
#include "unity.h"

static uint64_t now_ms(void) {
    struct timespec value;
    clock_gettime(CLOCK_MONOTONIC, &value);
    return (uint64_t)value.tv_sec * 1000U + (uint64_t)value.tv_nsec / 1000000U;
}

void setUp(void) { health_helper_reap_abandoned(); }
void tearDown(void) { health_helper_reap_abandoned(); }

static void test_direct_exec_captures_bounded_output(void) {
    char *arguments[] = {"echo", "health-ok", NULL};
    health_helper_request_t request = {
        .program = "/bin/echo", .argv = arguments, .timeout_ms = 500U,
        .terminate_grace_ms = 20U, .output_limit = 64U};
    health_helper_result_t result;
    TEST_ASSERT_EQUAL_INT(0, health_helper_run(&request, &result));
    TEST_ASSERT_EQUAL_INT(HEALTH_HELPER_OK, result.outcome);
    TEST_ASSERT_EQUAL_INT(0, result.exit_code);
    TEST_ASSERT_EQUAL_STRING("health-ok\n", result.output);
    TEST_ASSERT_FALSE(result.output_truncated);
}

static void test_output_is_truncated_without_blocking_child(void) {
    char *arguments[] = {"echo", "abcdefghijklmnopqrstuvwxyz", NULL};
    health_helper_request_t request = {
        .program = "/bin/echo", .argv = arguments, .timeout_ms = 500U,
        .terminate_grace_ms = 20U, .output_limit = 8U};
    health_helper_result_t result;
    TEST_ASSERT_EQUAL_INT(0, health_helper_run(&request, &result));
    TEST_ASSERT_EQUAL_INT(HEALTH_HELPER_OK, result.outcome);
    TEST_ASSERT_TRUE(result.output_truncated);
    TEST_ASSERT_EQUAL_UINT64(8U, result.output_length);
    TEST_ASSERT_EQUAL_MEMORY("abcdefgh", result.output, 8U);
}

static void test_timeout_terminates_child_within_bound(void) {
    char *arguments[] = {"sleep", "2", NULL};
    health_helper_request_t request = {
        .program = "/bin/sleep", .argv = arguments, .timeout_ms = 40U,
        .terminate_grace_ms = 30U, .output_limit = 64U};
    health_helper_result_t result;
    uint64_t started = now_ms();
    TEST_ASSERT_EQUAL_INT(0, health_helper_run(&request, &result));
    uint64_t elapsed = now_ms() - started;
    TEST_ASSERT_EQUAL_INT(HEALTH_HELPER_TIMED_OUT, result.outcome);
    TEST_ASSERT_LESS_THAN_UINT64(500U, elapsed);
    TEST_ASSERT_FALSE(result.abandoned);
}

static void test_exec_failure_is_explicit(void) {
    char *arguments[] = {"missing-health-helper", NULL};
    health_helper_request_t request = {
        .program = "/definitely/missing-health-helper", .argv = arguments,
        .timeout_ms = 500U, .terminate_grace_ms = 20U, .output_limit = 64U};
    health_helper_result_t result;
    TEST_ASSERT_EQUAL_INT(0, health_helper_run(&request, &result));
    TEST_ASSERT_EQUAL_INT(HEALTH_HELPER_EXEC_ERROR, result.outcome);
    TEST_ASSERT_EQUAL_INT(127, result.exit_code);
}

static void test_environment_is_minimal_and_request_is_validated(void) {
    char *arguments[] = {"env", NULL};
    health_helper_request_t request = {
        .program = "/usr/bin/env", .argv = arguments, .timeout_ms = 500U,
        .terminate_grace_ms = 20U, .output_limit = 512U};
    health_helper_result_t result;
    TEST_ASSERT_EQUAL_INT(0, health_helper_run(&request, &result));
    TEST_ASSERT_EQUAL_INT(HEALTH_HELPER_OK, result.outcome);
    TEST_ASSERT_NOT_NULL(strstr(result.output, "PATH="));
    TEST_ASSERT_NOT_NULL(strstr(result.output, "LC_ALL=C"));
    TEST_ASSERT_NULL(strstr(result.output, "HOME="));
    request.program = "relative/program";
    TEST_ASSERT_EQUAL_INT(-1, health_helper_run(&request, &result));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_direct_exec_captures_bounded_output);
    RUN_TEST(test_output_is_truncated_without_blocking_child);
    RUN_TEST(test_timeout_terminates_child_within_bound);
    RUN_TEST(test_exec_failure_is_explicit);
    RUN_TEST(test_environment_is_minimal_and_request_is_validated);
    return UNITY_END();
}
