#include "unity.h"

#include "telemetry/collectors/linux_cgroup.h"

#include <stdio.h>
#include <string.h>

#ifndef TEST_FIXTURE_ROOT
#define TEST_FIXTURE_ROOT "tests/fixtures/system_health"
#endif

void setUp(void) {}
void tearDown(void) {}

static const system_health_observation_t *find_observation(
    const system_health_observation_t *items, size_t count,
    const char *metric) {
    for (size_t i = 0; i < count; ++i) {
        if (strcmp(items[i].metric, metric) == 0) return &items[i];
    }
    return NULL;
}

static void fixture_paths(char *proc, size_t proc_capacity, char *sys,
                          size_t sys_capacity, const char *name) {
    (void)snprintf(proc, proc_capacity, "%s/cgroup/%s/proc",
                   TEST_FIXTURE_ROOT, name);
    (void)snprintf(sys, sys_capacity, "%s/cgroup/%s/sys",
                   TEST_FIXTURE_ROOT, name);
}

static void test_cgroup_parsers_reject_bad_limits_and_overflow(void) {
    linux_cgroup_limit_t limit;
    linux_cgroup_cpu_max_t cpu_max;
    linux_cgroup_cpu_stat_t cpu_stat;
    linux_cgroup_memory_events_t events;

    TEST_ASSERT_EQUAL_INT(0, linux_cgroup_parse_limit("max\n", &limit));
    TEST_ASSERT_TRUE(limit.unlimited);
    TEST_ASSERT_EQUAL_INT(0, linux_cgroup_parse_limit("42\n", &limit));
    TEST_ASSERT_FALSE(limit.unlimited);
    TEST_ASSERT_EQUAL_UINT64(42, limit.value);
    TEST_ASSERT_EQUAL_INT(-1, linux_cgroup_parse_limit(
        "18446744073709551616\n", &limit));
    TEST_ASSERT_EQUAL_INT(0, linux_cgroup_parse_cpu_max(
        "200000 100000\n", &cpu_max));
    TEST_ASSERT_EQUAL_INT(-1, linux_cgroup_parse_cpu_max(
        "0 100000\n", &cpu_max));
    TEST_ASSERT_EQUAL_INT(0, linux_cgroup_parse_cpu_stat(
        "usage_usec 10\nnr_periods 2\nnr_throttled 1\n"
        "throttled_usec 3\n", &cpu_stat));
    TEST_ASSERT_EQUAL_UINT64(10, cpu_stat.usage_usec);
    TEST_ASSERT_EQUAL_INT(0, linux_cgroup_parse_memory_events(
        "low 0\nhigh 1\nmax 2\noom 3\noom_kill 4\n", &events));
    TEST_ASSERT_EQUAL_UINT64(4, events.oom_kill);
    TEST_ASSERT_EQUAL_INT(1, linux_cgroup_parse_memory_events(
        "low 0\nhigh 1\nmax 2\noom 3\n", &events));
}

static void test_v2_effective_limits_headroom_and_safe_deltas(void) {
    linux_cgroup_state_t state;
    system_health_collector_t collector;
    system_health_observation_t items[32];
    system_health_observation_sink_t sink = {
        .items = items, .capacity = 32U
    };
    system_health_collect_context_t context = {
        .monotonic_ms = 1000U, .wall_time_ms = 1700000000000LL
    };
    char proc[512];
    char sys[512];
    const system_health_observation_t *observation;

    TEST_ASSERT_TRUE(linux_cgroup_collector_init(&collector, &state));
    fixture_paths(proc, sizeof(proc), sys, sizeof(sys), "v2a");
    context.proc_root = proc;
    context.cgroup_root = sys;
    TEST_ASSERT_EQUAL_INT(0, collector.collect(collector.state, &context, &sink));
    observation = find_observation(items, sink.count,
                                   "container.memory.available_ratio");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_SCOPE_CONTAINER, observation->scope);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 0.70703125f,
                             (float)observation->value);
    observation = find_observation(items, sink.count,
                                   "container.cpu.usage_ratio");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_STALE,
                      observation->capability);

    sink.count = 0;
    fixture_paths(proc, sizeof(proc), sys, sizeof(sys), "v2b");
    context.proc_root = proc;
    context.cgroup_root = sys;
    context.monotonic_ms = 2000U;
    TEST_ASSERT_EQUAL_INT(0, collector.collect(collector.state, &context, &sink));
    observation = find_observation(items, sink.count,
                                   "container.cpu.usage_ratio");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 0.25f, (float)observation->value);
    observation = find_observation(items, sink.count,
                                   "container.cpu.throttled_ratio");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 0.1f, (float)observation->value);
    observation = find_observation(items, sink.count,
                                   "container.memory.oom_kills_delta");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_EQUAL_UINT64(2, (uint64_t)observation->value);

    sink.count = 0;
    fixture_paths(proc, sizeof(proc), sys, sizeof(sys), "v2a");
    context.proc_root = proc;
    context.cgroup_root = sys;
    context.monotonic_ms = 3000U;
    TEST_ASSERT_EQUAL_INT(0, collector.collect(collector.state, &context, &sink));
    observation = find_observation(items, sink.count,
                                   "container.memory.oom_kills_delta");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FALSE(observation->value_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR,
                      observation->capability);
}

static void test_unlimited_v2_reports_host_scope_without_fake_limits(void) {
    linux_cgroup_state_t state;
    system_health_collector_t collector;
    system_health_observation_t items[32];
    system_health_observation_sink_t sink = {
        .items = items, .capacity = 32U
    };
    system_health_collect_context_t context = {.monotonic_ms = 1000U};
    char proc[512];
    char sys[512];
    const system_health_observation_t *observation;

    TEST_ASSERT_TRUE(linux_cgroup_collector_init(&collector, &state));
    fixture_paths(proc, sizeof(proc), sys, sizeof(sys), "v2max");
    context.proc_root = proc;
    context.cgroup_root = sys;
    TEST_ASSERT_EQUAL_INT(0, collector.collect(collector.state, &context, &sink));
    observation = find_observation(items, sink.count, "host.memory.limit_bytes");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_SCOPE_HOST, observation->scope);
    TEST_ASSERT_FALSE(observation->value_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
                      observation->capability);
    observation = find_observation(items, sink.count,
                                   "host.pids.available_ratio");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FALSE(observation->value_valid);
}

static void test_v2_uses_tighter_ancestor_limits(void) {
    linux_cgroup_state_t state;
    system_health_collector_t collector;
    system_health_observation_t items[32];
    system_health_observation_sink_t sink = {
        .items = items, .capacity = 32U
    };
    system_health_collect_context_t context = {.monotonic_ms = 1000U};
    char proc[512];
    char sys[512];
    const system_health_observation_t *observation;

    TEST_ASSERT_TRUE(linux_cgroup_collector_init(&collector, &state));
    fixture_paths(proc, sizeof(proc), sys, sizeof(sys), "v2ancestor");
    context.proc_root = proc;
    context.cgroup_root = sys;
    TEST_ASSERT_EQUAL_INT(0, collector.collect(collector.state, &context, &sink));
    observation = find_observation(items, sink.count,
                                   "container.cpu.quota_cores");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 0.5f, (float)observation->value);
    observation = find_observation(items, sink.count,
                                   "container.memory.available_ratio");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 0.5f, (float)observation->value);
    observation = find_observation(items, sink.count,
                                   "container.pids.available_ratio");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 0.75f, (float)observation->value);
}

static void test_v1_common_combined_controller_layout(void) {
    linux_cgroup_state_t state;
    system_health_collector_t collector;
    system_health_observation_t items[32];
    system_health_observation_sink_t sink = {
        .items = items, .capacity = 32U
    };
    system_health_collect_context_t context = {.monotonic_ms = 1000U};
    char proc[512];
    char sys[512];
    const system_health_observation_t *observation;

    TEST_ASSERT_TRUE(linux_cgroup_collector_init(&collector, &state));
    fixture_paths(proc, sizeof(proc), sys, sizeof(sys), "v1");
    context.proc_root = proc;
    context.cgroup_root = sys;
    TEST_ASSERT_EQUAL_INT(0, collector.collect(collector.state, &context, &sink));
    observation = find_observation(items, sink.count,
                                   "container.cpu.quota_cores");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 1.0f, (float)observation->value);
    observation = find_observation(items, sink.count,
                                   "container.memory.available_ratio");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 0.5f, (float)observation->value);
    observation = find_observation(items, sink.count,
                                   "container.pids.available_ratio");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 0.8f, (float)observation->value);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_cgroup_parsers_reject_bad_limits_and_overflow);
    RUN_TEST(test_v2_effective_limits_headroom_and_safe_deltas);
    RUN_TEST(test_unlimited_v2_reports_host_scope_without_fake_limits);
    RUN_TEST(test_v2_uses_tighter_ancestor_limits);
    RUN_TEST(test_v1_common_combined_controller_layout);
    return UNITY_END();
}
