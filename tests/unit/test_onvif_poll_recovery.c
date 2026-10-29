/* Exercise the production polling loop, as in test_detection_io_recovery. */
#include "../../src/video/unified_detection_thread.c"
#include "unity.h"

static unified_detection_ctx_t *ctx;
static int poll_result, event_count, polls, sleeps;
static unsigned int total_sleep_us, longest_sleep_us;
static bool stop_during_sleep;

int __wrap_detect_motion_onvif(const char *url, const char *username,
        const char *password, detection_result_t *result, const char *stream_name) {
    (void)url; (void)username; (void)password; (void)stream_name;
    memset(result, 0, sizeof(*result));
    result->count = event_count;
    if (++polls == 2) atomic_store(&ctx->onvif_thread_running, 0);
    return poll_result;
}

int __wrap_av_usleep(unsigned int usec) {
    ++sleeps;
    total_sleep_us += usec;
    if (usec > longest_sleep_us) longest_sleep_us = usec;
    if (stop_during_sleep) atomic_store(&ctx->onvif_thread_running, 0);
    return 0;
}

void setUp(void) {
    ctx = calloc(1, sizeof(*ctx));
    TEST_ASSERT_NOT_NULL(ctx);
    snprintf(ctx->stream_name, sizeof(ctx->stream_name), "poll-recovery");
    snprintf(ctx->onvif_url_cached, sizeof(ctx->onvif_url_cached), "http://127.0.0.1");
    atomic_store(&ctx->onvif_thread_running, 1);
    poll_result = event_count = polls = sleeps = 0;
    total_sleep_us = longest_sleep_us = 0;
    stop_during_sleep = false;
}

void tearDown(void) {
    free(ctx);
}

static void test_dropped_connection_continues_without_five_second_gap(void) {
    poll_result = ONVIF_DETECT_RETRY;
    onvif_detection_thread_func(ctx);
    TEST_ASSERT_EQUAL_INT(2, polls);
    TEST_ASSERT_EQUAL_UINT(250000, total_sleep_us);
    TEST_ASSERT_LESS_OR_EQUAL_UINT(100000, longest_sleep_us);
    TEST_ASSERT_EQUAL_INT(0, atomic_load(&ctx->onvif_motion_detected));
}

static void test_other_failure_keeps_five_second_backoff(void) {
    poll_result = -1;
    onvif_detection_thread_func(ctx);
    TEST_ASSERT_EQUAL_INT(2, polls);
    TEST_ASSERT_EQUAL_UINT(5000000, total_sleep_us);
    TEST_ASSERT_LESS_OR_EQUAL_UINT(100000, longest_sleep_us);
}

static void test_stop_interrupts_drop_retry_delay(void) {
    poll_result = ONVIF_DETECT_RETRY;
    stop_during_sleep = true;
    onvif_detection_thread_func(ctx);
    TEST_ASSERT_EQUAL_INT(1, polls);
    TEST_ASSERT_EQUAL_INT(1, sleeps);
    TEST_ASSERT_LESS_OR_EQUAL_UINT(100000, total_sleep_us);
}

static void test_drop_keeps_recent_motion_until_hold_expires(void) {
    poll_result = ONVIF_DETECT_RETRY;
    atomic_store(&ctx->onvif_motion_detected, 1);
    atomic_store(&ctx->onvif_motion_timestamp, (long long)time(NULL));
    onvif_detection_thread_func(ctx);
    TEST_ASSERT_EQUAL_INT(1, atomic_load(&ctx->onvif_motion_detected));
}

static void test_empty_success_still_throttles_fast_camera(void) {
    onvif_detection_thread_func(ctx);
    TEST_ASSERT_EQUAL_INT(2, polls);
    TEST_ASSERT_GREATER_OR_EQUAL_UINT(ONVIF_MIN_POLL_INTERVAL_MS * 1000, total_sleep_us);
    TEST_ASSERT_LESS_OR_EQUAL_UINT((ONVIF_MIN_POLL_INTERVAL_MS + 100) * 1000, total_sleep_us);
}

static void test_motion_success_still_throttles_event_bursts(void) {
    event_count = 1;
    onvif_detection_thread_func(ctx);
    TEST_ASSERT_EQUAL_INT(2, polls);
    TEST_ASSERT_EQUAL_UINT(2000000, total_sleep_us);
    TEST_ASSERT_EQUAL_INT(1, atomic_load(&ctx->onvif_motion_detected));
}

int main(void) {
    init_logger();
    set_log_level(LOG_LEVEL_ERROR);
    UNITY_BEGIN();
    RUN_TEST(test_dropped_connection_continues_without_five_second_gap);
    RUN_TEST(test_other_failure_keeps_five_second_backoff);
    RUN_TEST(test_stop_interrupts_drop_retry_delay);
    RUN_TEST(test_drop_keeps_recent_motion_until_hold_expires);
    RUN_TEST(test_empty_success_still_throttles_fast_camera);
    RUN_TEST(test_motion_success_still_throttles_event_bursts);
    int result = UNITY_END();
    shutdown_logger();
    return result;
}
