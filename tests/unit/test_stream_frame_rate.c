#include "unity.h"
#include "video/stream_frame_rate.h"

#include <limits.h>
#include <math.h>

void setUp(void) {}
void tearDown(void) {}

static void test_axis_direct_prefers_average(void) {
    bool provisional = true;
    double fps = stream_frame_rate_select((AVRational){10, 1},
                                          (AVRational){30, 1}, &provisional);
    TEST_ASSERT_EQUAL_INT(10, stream_frame_rate_round(fps));
    TEST_ASSERT_FALSE(provisional);
}

static void test_axis_proxy_nominal_is_provisional(void) {
    bool provisional = false;
    double fps = stream_frame_rate_select((AVRational){0, 0},
                                          (AVRational){6, 1}, &provisional);
    TEST_ASSERT_EQUAL_INT(6, stream_frame_rate_round(fps));
    TEST_ASSERT_TRUE(provisional);
}

static void test_clock_rate_is_never_fps(void) {
    bool provisional = false;
    TEST_ASSERT_EQUAL_INT(0, stream_frame_rate_round(stream_frame_rate_select(
        (AVRational){0, 0}, (AVRational){90000, 1}, &provisional)));
    TEST_ASSERT_TRUE(provisional);
    TEST_ASSERT_EQUAL_INT(0, stream_frame_rate_round(stream_frame_rate_select(
        (AVRational){90000, 1}, (AVRational){90000, 1}, &provisional)));
    TEST_ASSERT_TRUE(provisional);
    TEST_ASSERT_EQUAL_INT(10, stream_frame_rate_round(stream_frame_rate_select(
        (AVRational){90000, 1}, (AVRational){10, 1}, &provisional)));
    TEST_ASSERT_TRUE(provisional);
}

static void test_invalid_ratios_and_extreme_rates(void) {
    const AVRational invalid[] = {{0, 0}, {10, 0}, {-10, 1}, {10, -1},
                                  {-10, -1}, {INT_MAX, 1}, {241, 1}};
    for (unsigned i = 0; i < sizeof(invalid) / sizeof(invalid[0]); i++) {
        bool provisional = false;
        TEST_ASSERT_EQUAL_INT(0, stream_frame_rate_round(stream_frame_rate_select(
            invalid[i], invalid[i], &provisional)));
        TEST_ASSERT_TRUE(provisional);
    }
    TEST_ASSERT_EQUAL_INT(0, stream_frame_rate_round(NAN));
    TEST_ASSERT_EQUAL_INT(0, stream_frame_rate_round(INFINITY));
    TEST_ASSERT_EQUAL_INT(0, stream_frame_rate_round(-1));
}

static void test_fractional_and_high_rates(void) {
    TEST_ASSERT_EQUAL_INT(30, stream_frame_rate_round(stream_frame_rate_select(
        (AVRational){30000, 1001}, (AVRational){0, 0}, NULL)));
    TEST_ASSERT_EQUAL_INT(1, stream_frame_rate_round(stream_frame_rate_select(
        (AVRational){1, 2}, (AVRational){0, 0}, NULL)));
    TEST_ASSERT_EQUAL_INT(240, stream_frame_rate_round(stream_frame_rate_select(
        (AVRational){240, 1}, (AVRational){0, 0}, NULL)));
}

static void test_arrivals_refine_proxy_rate_to_ten(void) {
    stream_frame_rate_measurement_t m = {0};
    for (int i = 0; i < 70; i++) {
        TEST_ASSERT_EQUAL_INT(0, stream_frame_rate_observe(&m, i * 100000LL));
    }
    TEST_ASSERT_EQUAL_INT(10, stream_frame_rate_observe(&m, 7000000));
    // A completed measurement is stable until the connection is reset.
    TEST_ASSERT_EQUAL_INT(10, stream_frame_rate_observe(&m, 20000000));
}

static void test_probe_backlog_is_excluded(void) {
    stream_frame_rate_measurement_t m = {0};
    for (int i = 0; i < 10000; i++) {
        TEST_ASSERT_EQUAL_INT(0, stream_frame_rate_observe(&m, i * 100LL));
    }
    for (int i = 0; i <= 50; i++) {
        stream_frame_rate_observe(&m, 2000000 + i * 100000LL);
    }
    TEST_ASSERT_EQUAL_INT(10, m.fps);
}

static void test_excessive_arrival_rate_retries_window(void) {
    stream_frame_rate_measurement_t m = {0};
    stream_frame_rate_observe(&m, 0);
    for (int i = 0; i <= 2000; i++) {
        TEST_ASSERT_EQUAL_INT(0, stream_frame_rate_observe(&m, 2000000 + i * 2500LL));
    }
    for (int i = 0; i <= 50; i++) {
        stream_frame_rate_observe(&m, 7100000 + i * 100000LL);
    }
    TEST_ASSERT_EQUAL_INT(10, m.fps);
}

static void test_connection_reset_discards_old_measurement(void) {
    stream_frame_rate_measurement_t m = {0};
    for (int i = 0; i <= 70; i++) stream_frame_rate_observe(&m, i * 100000LL);
    TEST_ASSERT_EQUAL_INT(10, m.fps);
    m = (stream_frame_rate_measurement_t){0};
    for (int i = 0; i <= 140; i++) {
        stream_frame_rate_observe(&m, 10000000 + i * 50000LL);
    }
    TEST_ASSERT_EQUAL_INT(20, m.fps);
}

static void test_sparse_arrivals_do_not_become_default_fps(void) {
    stream_frame_rate_measurement_t m = {0};
    for (int i = 0; i <= 4; i++) stream_frame_rate_observe(&m, i * 2000000LL);
    TEST_ASSERT_EQUAL_INT(1, m.fps); // integer schema rounds 0.5 FPS up
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_axis_direct_prefers_average);
    RUN_TEST(test_axis_proxy_nominal_is_provisional);
    RUN_TEST(test_clock_rate_is_never_fps);
    RUN_TEST(test_invalid_ratios_and_extreme_rates);
    RUN_TEST(test_fractional_and_high_rates);
    RUN_TEST(test_arrivals_refine_proxy_rate_to_ten);
    RUN_TEST(test_probe_backlog_is_excluded);
    RUN_TEST(test_excessive_arrival_rate_retries_window);
    RUN_TEST(test_connection_reset_discards_old_measurement);
    RUN_TEST(test_sparse_arrivals_do_not_become_default_fps);
    return UNITY_END();
}
