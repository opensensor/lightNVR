#include "unity.h"

#include "telemetry/system_health_collector.h"
#include "telemetry/system_health_types.h"

#include <string.h>

void setUp(void) {}
void tearDown(void) {}

static void test_unavailable_observation_has_no_numeric_value(void) {
    system_health_observation_t observation;
    memset(&observation, 0, sizeof(observation));

    system_health_observation_set_available(
        &observation, 0.0, SYSTEM_HEALTH_UNIT_BYTES);
    TEST_ASSERT_TRUE(observation.value_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                      observation.capability);

    system_health_observation_set_unavailable(
        &observation, SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED);
    TEST_ASSERT_FALSE(observation.value_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_UNIT_NONE, observation.unit);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED,
                      observation.capability);
}

static void test_condition_registry_is_stable_and_reversible(void) {
    for (int i = 0; i < SYSTEM_HEALTH_CONDITION_COUNT; ++i) {
        system_health_condition_t parsed = SYSTEM_HEALTH_CONDITION_COUNT;
        const char *code = system_health_condition_code(
            (system_health_condition_t)i);
        TEST_ASSERT_NOT_NULL(code);
        TEST_ASSERT_TRUE(system_health_condition_from_code(code, &parsed));
        TEST_ASSERT_EQUAL_INT(i, parsed);
    }
    TEST_ASSERT_NULL(system_health_condition_code(SYSTEM_HEALTH_CONDITION_COUNT));
}

static void test_bounded_sinks_count_overflow(void) {
    system_health_observation_t items[1];
    system_health_observation_t observation;
    system_health_observation_sink_t sink = {
        .items = items,
        .capacity = 1,
        .count = 0,
        .dropped = 0
    };
    memset(&observation, 0, sizeof(observation));

    TEST_ASSERT_TRUE(system_health_observation_sink_append(&sink, &observation));
    TEST_ASSERT_FALSE(system_health_observation_sink_append(&sink, &observation));
    TEST_ASSERT_EQUAL_size_t(1, sink.count);
    TEST_ASSERT_EQUAL_size_t(1, sink.dropped);
}

static void test_time_domains_remain_distinct(void) {
    system_health_observation_t observation;
    memset(&observation, 0, sizeof(observation));
    observation.sampled_monotonic_ms = 42;
    observation.observed_wall_time_ms = 1700000000000LL;
    TEST_ASSERT_EQUAL_UINT64(42, observation.sampled_monotonic_ms);
    TEST_ASSERT_EQUAL_INT64(1700000000000LL, observation.observed_wall_time_ms);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_unavailable_observation_has_no_numeric_value);
    RUN_TEST(test_condition_registry_is_stable_and_reversible);
    RUN_TEST(test_bounded_sinks_count_overflow);
    RUN_TEST(test_time_domains_remain_distinct);
    return UNITY_END();
}
