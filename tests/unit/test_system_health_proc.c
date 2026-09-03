#include "unity.h"

#include "telemetry/collectors/linux_proc.h"

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

static void fixture_path(char *out, size_t capacity, const char *name) {
    (void)snprintf(out, capacity, "%s/proc/%s", TEST_FIXTURE_ROOT, name);
}

static void test_bounded_proc_parsers(void) {
    linux_proc_cpu_times_t cpu;
    linux_proc_loadavg_t load;
    linux_proc_memory_t memory;
    linux_proc_vmstat_t vmstat;
    linux_proc_pressure_t pressure;

    TEST_ASSERT_EQUAL_INT(0, linux_proc_parse_cpu_stat(
        "cpu 1 2 3 4 5 6 7 8\n", &cpu));
    TEST_ASSERT_EQUAL_UINT64(4, cpu.idle);
    TEST_ASSERT_EQUAL_INT(-1, linux_proc_parse_cpu_stat(
        "cpu 18446744073709551616 2 3 4 5 6 7 8\n", &cpu));
    TEST_ASSERT_EQUAL_INT(0, linux_proc_parse_loadavg(
        "1.25 2.50 3.75 1/5 10\n", &load));
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 2.5f, (float)load.five);
    TEST_ASSERT_EQUAL_INT(0, linux_proc_parse_meminfo(
        "MemTotal: 100 kB\nMemAvailable: 60 kB\n"
        "SwapTotal: 20 kB\nSwapFree: 5 kB\n", &memory));
    TEST_ASSERT_EQUAL_UINT64(60U * 1024U, memory.available_bytes);
    TEST_ASSERT_EQUAL_INT(1, linux_proc_parse_meminfo(
        "MemTotal: 100 kB\nMemFree: 60 kB\n"
        "SwapTotal: 0 kB\nSwapFree: 0 kB\n", &memory));
    TEST_ASSERT_EQUAL_INT(0, linux_proc_parse_vmstat(
        "pgmajfault 8\npswpin 2\npswpout 3\n", &vmstat));
    TEST_ASSERT_EQUAL_UINT64(8, vmstat.major_faults);
    TEST_ASSERT_EQUAL_INT(1, linux_proc_parse_vmstat(
        "pgmajfault 8\n", &vmstat));
    TEST_ASSERT_EQUAL_INT(0, linux_proc_parse_pressure(
        "some avg10=12.50 avg60=1.0 avg300=0.0 total=999\n"
        "full avg10=2.00 avg60=1.0 avg300=0.0 total=100\n",
        &pressure));
    TEST_ASSERT_TRUE(pressure.full_present);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 0.125f,
                             (float)pressure.some_avg10_ratio);
    TEST_ASSERT_EQUAL_INT(-1, linux_proc_parse_pressure(
        "some avg10=101.0 avg60=0 avg300=0 total=1\n", &pressure));
}

static void test_collector_uses_consecutive_samples_and_memavailable(void) {
    linux_proc_state_t state;
    system_health_collector_t collector;
    system_health_observation_t items[64];
    system_health_observation_sink_t sink = {
        .items = items, .capacity = 64U
    };
    system_health_collect_context_t context = {
        .monotonic_ms = 1000U, .wall_time_ms = 1700000000000LL
    };
    char root[512];
    const system_health_observation_t *observation;

    TEST_ASSERT_TRUE(linux_proc_collector_init(&collector, &state));
    fixture_path(root, sizeof(root), "sample1");
    context.proc_root = root;
    TEST_ASSERT_EQUAL_INT(0, collector.collect(collector.state, &context, &sink));
    observation = find_observation(items, sink.count, "host.cpu.busy_ratio");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FALSE(observation->value_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_STALE,
                      observation->capability);
    observation = find_observation(items, sink.count,
                                   "host.memory.available_ratio");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 0.6f, (float)observation->value);

    sink.count = 0;
    fixture_path(root, sizeof(root), "sample2");
    context.proc_root = root;
    context.monotonic_ms = 2000U;
    TEST_ASSERT_EQUAL_INT(0, collector.collect(collector.state, &context, &sink));
    observation = find_observation(items, sink.count, "host.cpu.busy_ratio");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_TRUE(observation->value_valid);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 75.0f / 135.0f,
                             (float)observation->value);
    observation = find_observation(items, sink.count,
                                   "host.vm.major_faults_delta");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_EQUAL_UINT64(12U, (uint64_t)observation->value);
    observation = find_observation(items, sink.count,
                                   "host.pressure.io.some_seconds_delta");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 0.4f, (float)observation->value);

    sink.count = 0;
    fixture_path(root, sizeof(root), "sample1");
    context.proc_root = root;
    context.monotonic_ms = 3000U;
    TEST_ASSERT_EQUAL_INT(0, collector.collect(collector.state, &context, &sink));
    observation = find_observation(items, sink.count, "host.cpu.busy_ratio");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FALSE(observation->value_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR,
                      observation->capability);
}

static void test_old_kernel_omissions_are_not_numeric_zeroes(void) {
    linux_proc_state_t state;
    system_health_collector_t collector;
    system_health_observation_t items[64];
    system_health_observation_sink_t sink = {
        .items = items, .capacity = 64U
    };
    system_health_collect_context_t context = {.monotonic_ms = 1000U};
    char root[512];
    const system_health_observation_t *observation;

    TEST_ASSERT_TRUE(linux_proc_collector_init(&collector, &state));
    fixture_path(root, sizeof(root), "old_kernel");
    context.proc_root = root;
    TEST_ASSERT_EQUAL_INT(0, collector.collect(collector.state, &context, &sink));
    observation = find_observation(items, sink.count,
                                   "host.memory.available_bytes");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FALSE(observation->value_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
                      observation->capability);
    observation = find_observation(items, sink.count,
                                   "host.pressure.cpu.some_ratio");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
                      observation->capability);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_bounded_proc_parsers);
    RUN_TEST(test_collector_uses_consecutive_samples_and_memavailable);
    RUN_TEST(test_old_kernel_omissions_are_not_numeric_zeroes);
    return UNITY_END();
}
