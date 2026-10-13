#define _GNU_SOURCE
#define _POSIX_C_SOURCE 200809L

#include <limits.h>
#include "unity.h"

/* Exercise the production delay and interruptible wait without exporting
 * private thread functions just for the test. */
#include "../../src/video/hls/hls_unified_thread.c"

static unsigned sleep_calls;
static uint64_t slept_us;
static hls_unified_thread_ctx_t context;
static bool stop_on_sleep;

int __wrap_av_usleep(unsigned usec) {
    sleep_calls++;
    slept_us += usec;
    if (stop_on_sleep) atomic_store(&context.running, 0);
    return 0;
}

void setUp(void) {
    load_default_config(&g_config);
    init_shutdown_coordinator();
    /* No stream registry is needed to test the private waiting function. */
    g_config.max_streams = 0;
    memset(&context, 0, sizeof(context));
    atomic_store(&context.running, 1);
    sleep_calls = 0;
    slept_us = 0;
    stop_on_sleep = false;
}

void tearDown(void) {}

void test_delay_grows_to_thirty_seconds_without_overflow(void) {
    const int expected[] = {500, 1000, 2000, 4000, 8000, 16000, 30000};
    TEST_ASSERT_EQUAL_INT(500, calculate_reconnect_delay(0));
    TEST_ASSERT_EQUAL_INT(500, calculate_reconnect_delay(INT_MIN));
    for (int attempt = 1; attempt <= 7; attempt++)
        TEST_ASSERT_EQUAL_INT(expected[attempt - 1], calculate_reconnect_delay(attempt));
    TEST_ASSERT_EQUAL_INT(30000, calculate_reconnect_delay(24));
    TEST_ASSERT_EQUAL_INT(30000, calculate_reconnect_delay(32));
    TEST_ASSERT_EQUAL_INT(30000, calculate_reconnect_delay(1000));
    TEST_ASSERT_EQUAL_INT(30000, calculate_reconnect_delay(INT_MAX));
}

void test_absent_camera_keeps_a_capped_delay_after_thousands_of_failures(void) {
    for (int attempt = 7; attempt <= 10000; attempt++)
        TEST_ASSERT_EQUAL_INT(30000, calculate_reconnect_delay(attempt));
}

void test_retry_wait_is_not_skipped_after_thirty_two_failures(void) {
    int result = interruptible_sleep_ms(calculate_reconnect_delay(32),
                                        &context, NULL, "offline-camera");
    TEST_ASSERT_EQUAL_INT(0, result);
    TEST_ASSERT_EQUAL_UINT(300, sleep_calls);
    TEST_ASSERT_EQUAL_UINT64(30000000, slept_us);
}

void test_capped_wait_still_stops_promptly_when_camera_is_disabled(void) {
    stop_on_sleep = true;
    int result = interruptible_sleep_ms(calculate_reconnect_delay(1000),
                                        &context, NULL, "offline-camera");
    TEST_ASSERT_EQUAL_INT(1, result);
    TEST_ASSERT_EQUAL_UINT(1, sleep_calls);
    TEST_ASSERT_EQUAL_UINT64(100000, slept_us);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_delay_grows_to_thirty_seconds_without_overflow);
    RUN_TEST(test_absent_camera_keeps_a_capped_delay_after_thousands_of_failures);
    RUN_TEST(test_retry_wait_is_not_skipped_after_thirty_two_failures);
    RUN_TEST(test_capped_wait_still_stops_promptly_when_camera_is_disabled);
    return UNITY_END();
}
