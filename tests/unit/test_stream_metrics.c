#include "unity.h"
#include "telemetry/stream_metrics.h"

#include <string.h>

void setUp(void) {}
void tearDown(void) {}

void test_parallel_consumers_are_counted_once(void) {
    uint64_t current_frames[METRICS_SOURCE_COUNT] = {0};
    uint64_t current_bytes[METRICS_SOURCE_COUNT] = {0};
    uint64_t previous_frames[METRICS_SOURCE_COUNT] = {0};
    uint64_t previous_bytes[METRICS_SOURCE_COUNT] = {0};

    current_frames[METRICS_SOURCE_RECORDING] = 150;
    current_frames[METRICS_SOURCE_HLS] = 150;
    current_bytes[METRICS_SOURCE_RECORDING] = 2000000;
    current_bytes[METRICS_SOURCE_HLS] = 2000000;

    metrics_counter_delta_t merged = metrics_merge_source_deltas(
        current_frames, current_bytes, previous_frames, previous_bytes);

    TEST_ASSERT_EQUAL_UINT64(150, merged.frames);
    TEST_ASSERT_EQUAL_UINT64(2000000, merged.bytes);
}

void test_sampler_uses_best_available_source_each_interval(void) {
    uint64_t current_frames[METRICS_SOURCE_COUNT] = {0};
    uint64_t current_bytes[METRICS_SOURCE_COUNT] = {0};
    uint64_t previous_frames[METRICS_SOURCE_COUNT] = {0};
    uint64_t previous_bytes[METRICS_SOURCE_COUNT] = {0};

    current_frames[METRICS_SOURCE_RECORDING] = 100;
    current_frames[METRICS_SOURCE_HLS] = 125;
    current_bytes[METRICS_SOURCE_RECORDING] = 1000000;
    current_bytes[METRICS_SOURCE_HLS] = 1250000;
    metrics_merge_source_deltas(current_frames, current_bytes,
                                previous_frames, previous_bytes);

    current_frames[METRICS_SOURCE_RECORDING] += 120;
    current_frames[METRICS_SOURCE_HLS] += 100;
    current_bytes[METRICS_SOURCE_RECORDING] += 1200000;
    current_bytes[METRICS_SOURCE_HLS] += 1000000;

    metrics_counter_delta_t merged = metrics_merge_source_deltas(
        current_frames, current_bytes, previous_frames, previous_bytes);

    TEST_ASSERT_EQUAL_UINT64(120, merged.frames);
    TEST_ASSERT_EQUAL_UINT64(1200000, merged.bytes);
}

void test_counter_reset_does_not_underflow(void) {
    uint64_t current_frames[METRICS_SOURCE_COUNT] = {0};
    uint64_t current_bytes[METRICS_SOURCE_COUNT] = {0};
    uint64_t previous_frames[METRICS_SOURCE_COUNT] = {0};
    uint64_t previous_bytes[METRICS_SOURCE_COUNT] = {0};

    previous_frames[METRICS_SOURCE_DETECTION] = 500;
    previous_bytes[METRICS_SOURCE_DETECTION] = 5000000;
    current_frames[METRICS_SOURCE_DETECTION] = 25;
    current_bytes[METRICS_SOURCE_DETECTION] = 250000;

    metrics_counter_delta_t merged = metrics_merge_source_deltas(
        current_frames, current_bytes, previous_frames, previous_bytes);

    TEST_ASSERT_EQUAL_UINT64(25, merged.frames);
    TEST_ASSERT_EQUAL_UINT64(250000, merged.bytes);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_parallel_consumers_are_counted_once);
    RUN_TEST(test_sampler_uses_best_available_source_each_interval);
    RUN_TEST(test_counter_reset_does_not_underflow);
    return UNITY_END();
}
