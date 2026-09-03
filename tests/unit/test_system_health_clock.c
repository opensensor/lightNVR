#include "unity.h"

#include "telemetry/collectors/linux_clock.h"

#include <errno.h>
#include <stdint.h>
#include <string.h>

void setUp(void) {}
void tearDown(void) {}

static linux_clock_sample_t sample(int64_t real, uint64_t mono, bool sync) {
    linux_clock_sample_t value;
    memset(&value, 0, sizeof(value));
    value.capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    value.synchronization_known = true;
    value.synchronized = sync;
    value.realtime_ms = real;
    value.monotonic_ms = mono;
    return value;
}

static void test_startup_grace_and_slew_do_not_report_jump(void) {
    linux_clock_state_t state = {0};
    linux_clock_sample_t first = sample(100000, 1000, false);
    linux_clock_result_t result = linux_clock_evaluate(&state, &first, 600000, 2000);
    TEST_ASSERT_TRUE(result.startup_grace_active);
    TEST_ASSERT_FALSE(result.synchronized);

    linux_clock_sample_t slewed = sample(110025, 11000, true);
    result = linux_clock_evaluate(&state, &slewed, 600000, 2000);
    TEST_ASSERT_FALSE(result.jump_detected);
    TEST_ASSERT_EQUAL_INT64(25, result.jump_ms);

    linux_clock_sample_t after_grace = sample(700025, 601000, true);
    result = linux_clock_evaluate(&state, &after_grace, 600000, 2000);
    TEST_ASSERT_FALSE(result.startup_grace_active);
    TEST_ASSERT_FALSE(result.jump_detected);
}

static void test_backward_and_forward_discontinuities_are_separate(void) {
    linux_clock_state_t state = {0};
    linux_clock_sample_t value = sample(100000, 1000, true);
    linux_clock_evaluate(&state, &value, 0, 2000);
    value = sample(96000, 2000, true);
    linux_clock_result_t result = linux_clock_evaluate(&state, &value, 0, 2000);
    TEST_ASSERT_TRUE(result.jump_detected);
    TEST_ASSERT_LESS_THAN_INT64(0, result.jump_ms);
    value = sample(110000, 3000, true);
    result = linux_clock_evaluate(&state, &value, 0, 2000);
    TEST_ASSERT_TRUE(result.jump_detected);
    TEST_ASSERT_GREATER_THAN_INT64(0, result.jump_ms);
}

typedef struct { int64_t real; int64_t mono; int sync_result; } fake_clock_t;
static int fake_real(void *context, int64_t *out) { *out = ((fake_clock_t *)context)->real; return 0; }
static int fake_mono(void *context, int64_t *out) { *out = ((fake_clock_t *)context)->mono; return 0; }
static int fake_sync(void *context, bool *out) {
    fake_clock_t *clock = context;
    if (clock->sync_result) { errno = ENOSYS; return -1; }
    *out = true;
    return 0;
}

static void test_unsupported_adjtimex_is_explicit(void) {
    fake_clock_t fake = {.real = 100, .mono = 50, .sync_result = -1};
    linux_clock_source_t source = {
        .context = &fake, .realtime = fake_real, .monotonic = fake_mono,
        .synchronization = fake_sync
    };
    linux_clock_sample_t value;
    TEST_ASSERT_EQUAL_INT(0, linux_clock_sample(&source, &value));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED, value.capability);
    TEST_ASSERT_FALSE(value.synchronization_known);
}

static void test_extreme_values_do_not_overflow_jump_arithmetic(void) {
    linux_clock_state_t state = {0};
    linux_clock_sample_t value = sample(INT64_MAX, 1, true);
    linux_clock_evaluate(&state, &value, 0, INT64_MIN);
    value = sample(INT64_MIN, 2, true);
    linux_clock_result_t result = linux_clock_evaluate(
        &state, &value, 0, INT64_MIN);
    TEST_ASSERT_EQUAL_INT64(INT64_MIN, result.jump_ms);
    TEST_ASSERT_TRUE(result.jump_detected);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_startup_grace_and_slew_do_not_report_jump);
    RUN_TEST(test_backward_and_forward_discontinuities_are_separate);
    RUN_TEST(test_unsupported_adjtimex_is_explicit);
    RUN_TEST(test_extreme_values_do_not_overflow_jump_arithmetic);
    return UNITY_END();
}
