#define _XOPEN_SOURCE 700
#include "unity.h"

#include "telemetry/collectors/linux_cgroup.h"

#include <ftw.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <unistd.h>
#include <string.h>

#ifndef TEST_FIXTURE_ROOT
#define TEST_FIXTURE_ROOT "tests/fixtures/system_health"
#endif

static char temporary_root[128];

static int remove_fixture(const char *path, const struct stat *status,
                          int type, struct FTW *walk) {
    (void)status; (void)type; (void)walk;
    return remove(path);
}

void setUp(void) { temporary_root[0] = '\0'; }
void tearDown(void) {
    if (temporary_root[0]) nftw(temporary_root, remove_fixture, 16, FTW_DEPTH | FTW_PHYS);
}

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

static void test_v1_separate_cpu_paths_and_optional_metrics(void) {
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
    fixture_paths(proc, sizeof(proc), sys, sizeof(sys), "v1separate");
    context.proc_root = proc;
    context.cgroup_root = sys;
    TEST_ASSERT_EQUAL_INT(0, collector.collect(collector.state, &context, &sink));

    observation = find_observation(items, sink.count,
                                   "container.cpu.quota_cores");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 1.0f, (float)observation->value);
    observation = find_observation(items, sink.count,
                                   "container.cpu.usage_ratio");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FALSE(observation->value_valid);

    sink.count = 0;
    context.monotonic_ms = 2000U;
    TEST_ASSERT_EQUAL_INT(0, collector.collect(collector.state, &context, &sink));
    observation = find_observation(items, sink.count,
                                   "container.cpu.usage_ratio");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_TRUE(observation->value_valid);
    /* Both samples read the same cpuacct.usage fixture, so no CPU time was
     * consumed between them and the usage ratio must be exactly zero. */
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 0.0f, (float)observation->value);

    observation = find_observation(items, sink.count,
                                   "container.memory.limit_bytes");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_TRUE(observation->value_valid);
    observation = find_observation(items, sink.count,
                                   "container.memory.current_bytes");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FALSE(observation->value_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
                      observation->capability);
    observation = find_observation(items, sink.count,
                                   "container.memory.available_bytes");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FALSE(observation->value_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
                      observation->capability);
    observation = find_observation(items, sink.count,
                                   "container.memory.available_ratio");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FALSE(observation->value_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
                      observation->capability);
    observation = find_observation(items, sink.count,
                                   "container.memory.oom_kills_delta");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FALSE(observation->value_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
                      observation->capability);
    observation = find_observation(items, sink.count, "container.pids.current");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FALSE(observation->value_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
                      observation->capability);
    observation = find_observation(items, sink.count, "container.pids.limit");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FALSE(observation->value_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
                      observation->capability);
}

static void test_v2_rejects_trailing_dotdot_membership(void) {
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
    const system_health_observation_t *discovery;

    /* "/camera/.." would resolve to the cgroup root, whose limits are
     * planted in the fixture; it must be rejected, not followed. */
    TEST_ASSERT_TRUE(linux_cgroup_collector_init(&collector, &state));
    fixture_paths(proc, sizeof(proc), sys, sizeof(sys), "v2dotdot");
    context.proc_root = proc;
    context.cgroup_root = sys;
    /* A rejected membership is reported as an unavailable discovery
     * observation; the collection pass itself still succeeds. */
    TEST_ASSERT_EQUAL_INT(0, collector.collect(collector.state, &context, &sink));

    discovery = find_observation(items, sink.count, "container.cgroup.discovery");
    TEST_ASSERT_NOT_NULL(discovery);
    TEST_ASSERT_FALSE(discovery->value_valid);

    observation = find_observation(items, sink.count,
                                   "container.memory.available_ratio");
    TEST_ASSERT_TRUE(observation == NULL || !observation->value_valid);
}

/* Exercise the public collector with forged proc membership and real symlinks,
 * including v1's fallback to a controller mounted directly at cgroup_root. */
static void check_symlink_escape(bool v1, bool leaf) {
    snprintf(temporary_root, sizeof(temporary_root), "/tmp/lightnvr-cgroup-XXXXXX");
    TEST_ASSERT_NOT_NULL(mkdtemp(temporary_root));
    const char *directories[] = {"proc", "proc/self", "sys", "outside", "outside/nested"};
    char path[512], outside[512], destination[512], proc[512], sys[512];
    for (size_t i = 0; i < sizeof(directories) / sizeof(directories[0]); ++i) {
        snprintf(path, sizeof(path), "%s/%s", temporary_root, directories[i]);
        TEST_ASSERT_EQUAL_INT(0, mkdir(path, 0700));
    }
    snprintf(proc, sizeof(proc), "%s/proc", temporary_root);
    snprintf(sys, sizeof(sys), "%s/sys", temporary_root);
    snprintf(path, sizeof(path), "%s/proc/self/cgroup", temporary_root);
    FILE *file = fopen(path, "w");
    TEST_ASSERT_NOT_NULL(file);
    fputs(v1 ? "5:memory:/escape/nested\n" : "0::/escape/nested\n", file);
    TEST_ASSERT_EQUAL_INT(0, fclose(file));
    const char *metric_file = v1 ? "memory.usage_in_bytes" : "memory.current";
    snprintf(outside, sizeof(outside), "%s/outside/nested/%s", temporary_root, metric_file);
    file = fopen(outside, "w");
    TEST_ASSERT_NOT_NULL(file);
    fputs("12345\n", file);
    TEST_ASSERT_EQUAL_INT(0, fclose(file));

    /* v1 checks both the named controller and the root-mounted fallback. */
    for (int i = 0; i < (v1 ? 2 : 1); ++i) {
        const char *controller = i == 1 ? "/memory" : "";
        if (i == 1) {
            snprintf(path, sizeof(path), "%s/sys/memory", temporary_root);
            TEST_ASSERT_EQUAL_INT(0, mkdir(path, 0700));
        }
        snprintf(path, sizeof(path), "%s/sys%s/escape", temporary_root, controller);
        if (leaf) {
            TEST_ASSERT_EQUAL_INT(0, mkdir(path, 0700));
            snprintf(path, sizeof(path), "%s/sys%s/escape/nested", temporary_root, controller);
            TEST_ASSERT_EQUAL_INT(0, mkdir(path, 0700));
            snprintf(path, sizeof(path), "%s/sys%s/escape/nested/%s",
                     temporary_root, controller, metric_file);
            TEST_ASSERT_EQUAL_INT(0, symlink(outside, path));
        } else {
            snprintf(destination, sizeof(destination), "%s/outside", temporary_root);
            TEST_ASSERT_EQUAL_INT(0, symlink(destination, path));
        }
    }
    linux_cgroup_state_t state;
    system_health_collector_t collector;
    system_health_observation_t items[32];
    system_health_observation_sink_t sink = {.items = items, .capacity = 32U};
    system_health_collect_context_t context = {
        .monotonic_ms = 1000U, .proc_root = proc, .cgroup_root = sys
    };
    TEST_ASSERT_TRUE(linux_cgroup_collector_init(&collector, &state));
    TEST_ASSERT_EQUAL_INT(0, collector.collect(collector.state, &context, &sink));
    const system_health_observation_t *observation = find_observation(
        items, sink.count, "container.memory.current_bytes");
    if (!observation) observation = find_observation(items, sink.count, "host.memory.current_bytes");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FALSE_MESSAGE(observation->value_valid,
                              "must not read a metric through a symlink outside the cgroup root");
}

static void test_v2_rejects_intermediate_symlinks(void) { check_symlink_escape(false, false); }
static void test_v1_rejects_intermediate_symlinks(void) { check_symlink_escape(true, false); }
static void test_v2_rejects_leaf_symlinks(void) { check_symlink_escape(false, true); }
static void test_v1_rejects_leaf_symlinks(void) { check_symlink_escape(true, true); }

static void write_membership_fixture(const char *relative, const char *text) {
    char path[1024];
    snprintf(path, sizeof(path), "%s/%s", temporary_root, relative);
    for (char *cursor = path + strlen(temporary_root) + 1; *cursor; ++cursor) {
        if (*cursor != '/') continue;
        *cursor = '\0';
        TEST_ASSERT_TRUE(mkdir(path, 0700) == 0 || errno == EEXIST);
        *cursor = '/';
    }
    FILE *file = fopen(path, "w");
    TEST_ASSERT_NOT_NULL(file);
    TEST_ASSERT_TRUE(fputs(text, file) >= 0);
    TEST_ASSERT_EQUAL_INT(0, fclose(file));
}

static void assert_delta_state(const system_health_observation_sink_t *sink,
                                const char *metric, bool valid, float value) {
    const system_health_observation_t *observation = find_observation(sink->items, sink->count, metric);
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_EQUAL(valid, observation->value_valid);
    if (valid) TEST_ASSERT_FLOAT_WITHIN(0.0001f, value, (float)observation->value);
    else TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_STALE, observation->capability);
}

static void test_v2_membership_changes_and_discovery_gaps_reset_deltas(void) {
    snprintf(temporary_root, sizeof(temporary_root), "/tmp/lightnvr-cgroup-XXXXXX");
    TEST_ASSERT_NOT_NULL(mkdtemp(temporary_root));
    const char *groups[] = {"first", "second"};
    for (size_t i = 0; i < 2; ++i) {
        char path[128];
        snprintf(path, sizeof(path), "sys/%s/cpu.max", groups[i]);
        write_membership_fixture(path, "100000 100000\n");
        snprintf(path, sizeof(path), "sys/%s/cpu.stat", groups[i]);
        write_membership_fixture(path, i == 0
            ? "usage_usec 100000\nnr_periods 10\nnr_throttled 1\nthrottled_usec 100\n"
            : "usage_usec 9000000\nnr_periods 100\nnr_throttled 50\nthrottled_usec 9000\n");
        snprintf(path, sizeof(path), "sys/%s/memory.events", groups[i]);
        write_membership_fixture(path, i == 0
            ? "low 0\nhigh 0\nmax 0\noom 0\noom_kill 2\n"
            : "low 0\nhigh 0\nmax 0\noom 0\noom_kill 20\n");
    }
    linux_cgroup_state_t state;
    system_health_collector_t collector;
    system_health_observation_t items[32];
    system_health_observation_sink_t sink = {.items = items, .capacity = 32U};
    char proc[256], sys[256];
    snprintf(proc, sizeof(proc), "%s/proc", temporary_root);
    snprintf(sys, sizeof(sys), "%s/sys", temporary_root);
    system_health_collect_context_t context = {.proc_root = proc, .cgroup_root = sys};
    TEST_ASSERT_TRUE(linux_cgroup_collector_init(&collector, &state));
    const char *memberships[] = {"0::/first\n", "0::/second\n", "0::/second\n",
                                  "invalid\n", "0::/second\n", "0::/second\n"};
    for (size_t i = 0; i < sizeof(memberships) / sizeof(memberships[0]); ++i) {
        write_membership_fixture("proc/self/cgroup", memberships[i]);
        if (i == 2) {
            write_membership_fixture("sys/second/cpu.stat",
                "usage_usec 9250000\nnr_periods 110\nnr_throttled 51\nthrottled_usec 9100\n");
            write_membership_fixture("sys/second/memory.events",
                "low 0\nhigh 0\nmax 0\noom 0\noom_kill 23\n");
        }
        context.monotonic_ms = (i + 1) * 1000;
        sink.count = 0;
        TEST_ASSERT_EQUAL_INT(0, collector.collect(collector.state, &context, &sink));
        if (i == 3) {
            const system_health_observation_t *discovery = find_observation(items, sink.count,
                "container.cgroup.discovery");
            TEST_ASSERT_NOT_NULL(discovery);
            TEST_ASSERT_FALSE(discovery->value_valid);
            continue;
        }
        bool valid = i == 2 || i == 5;
        assert_delta_state(&sink, "container.cpu.usage_ratio", valid, i == 2 ? 0.25f : 0.0f);
        assert_delta_state(&sink, "container.cpu.throttled_ratio", valid, i == 2 ? 0.1f : 0.0f);
        assert_delta_state(&sink, "container.memory.oom_kills_delta", valid, i == 2 ? 3.0f : 0.0f);
    }
}

static void test_v1_tracks_each_controller_and_long_memberships(void) {
    snprintf(temporary_root, sizeof(temporary_root), "/tmp/lightnvr-cgroup-XXXXXX");
    TEST_ASSERT_NOT_NULL(mkdtemp(temporary_root));
    const char *groups[] = {"first", "second"};
    for (size_t i = 0; i < 2; ++i) {
        char path[128];
        const char *files[] = {"cpu.cfs_quota_us", "cpu.cfs_period_us", "cpu.stat"};
        const char *values[] = {"100000\n", "100000\n",
            "nr_periods 10\nnr_throttled 1\nthrottled_time 100000\n"};
        for (size_t j = 0; j < 3; ++j) {
            snprintf(path, sizeof(path), "sys/cpu/%s/%s", groups[i], files[j]);
            write_membership_fixture(path, values[j]);
        }
        snprintf(path, sizeof(path), "sys/cpuacct/%s/cpuacct.usage", groups[i]);
        write_membership_fixture(path, i ? "9000000000\n" : "100000000\n");
    }
    linux_cgroup_state_t state;
    system_health_collector_t collector;
    system_health_observation_t items[32];
    system_health_observation_sink_t sink = {.items = items, .capacity = 32U};
    char proc[256], sys[256], membership[2048], memory[512], pids[512];
    snprintf(proc, sizeof(proc), "%s/proc", temporary_root);
    snprintf(sys, sizeof(sys), "%s/sys", temporary_root);
    /* Long paths differ only at the end; their components remain valid. */
    memset(memory, 'm', sizeof(memory) - 1);
    memory[0] = memory[200] = memory[400] = '/';
    memory[511] = '\0';
    memcpy(pids, memory, sizeof(pids));
    system_health_collect_context_t context = {.proc_root = proc, .cgroup_root = sys};
    TEST_ASSERT_TRUE(linux_cgroup_collector_init(&collector, &state));
    for (size_t i = 0; i < 6; ++i) {
        if (i == 3) memory[510] = 'n';
        if (i == 4) pids[510] = 'n';
        snprintf(membership, sizeof(membership),
                 "2:cpu:/%s\n3:cpuacct:/%s\n4:memory:%s\n5:pids:%s\n",
                 i >= 2 ? "second" : "first", i >= 1 ? "second" : "first", memory, pids);
        write_membership_fixture("proc/self/cgroup", membership);
        context.monotonic_ms = (i + 1) * 1000;
        sink.count = 0;
        TEST_ASSERT_EQUAL_INT(0, collector.collect(collector.state, &context, &sink));
        assert_delta_state(&sink, "container.cpu.usage_ratio", i == 5, 0.0f);
        assert_delta_state(&sink, "container.cpu.throttled_ratio", i == 5, 0.0f);
    }
}


/* Quota edits and read gaps must not normalize an interval with a new quota.
 * Throttling remains usable because its period counters do not use the quota. */
static void check_cpu_quota_transitions(bool v1) {
    snprintf(temporary_root, sizeof(temporary_root), "/tmp/lightnvr-cgroup-XXXXXX");
    TEST_ASSERT_NOT_NULL(mkdtemp(temporary_root));
    write_membership_fixture("proc/self/cgroup", v1
        ? "2:cpu,cpuacct:/camera\n4:memory:/camera\n" : "0::/camera\n");
    /* Keep the scope stable even while the CPU quota is unavailable. */
    write_membership_fixture(v1 ? "sys/memory/camera/memory.limit_in_bytes"
                               : "sys/camera/memory.max", "1048576\n");
    linux_cgroup_state_t state;
    system_health_collector_t collector;
    system_health_observation_t items[32];
    system_health_observation_sink_t sink = {.items = items, .capacity = 32U};
    char proc[256], sys[256];
    snprintf(proc, sizeof(proc), "%s/proc", temporary_root);
    snprintf(sys, sizeof(sys), "%s/sys", temporary_root);
    system_health_collect_context_t context = {.proc_root = proc, .cgroup_root = sys};
    TEST_ASSERT_TRUE(linux_cgroup_collector_init(&collector, &state));
    const struct {
        const char *quota;
        const char *period;
        system_health_capability_t capability;
        float ratio;
    } steps[] = {
        {"100000", "100000", SYSTEM_HEALTH_CAPABILITY_STALE, 0},
        {"100000", "100000", SYSTEM_HEALTH_CAPABILITY_AVAILABLE, 0.25f},
        {"200000", "100000", SYSTEM_HEALTH_CAPABILITY_STALE, 0},
        {"200000", "100000", SYSTEM_HEALTH_CAPABILITY_AVAILABLE, 0.125f},
        {"200000", "200000", SYSTEM_HEALTH_CAPABILITY_STALE, 0},
        {"200000", "200000", SYSTEM_HEALTH_CAPABILITY_AVAILABLE, 0.25f},
        {"max", "100000", SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED, 0},
        {"max", "100000", SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED, 0},
        {"100000", "100000", SYSTEM_HEALTH_CAPABILITY_STALE, 0},
        {"100000", "100000", SYSTEM_HEALTH_CAPABILITY_AVAILABLE, 0.25f},
        {"invalid", "100000", SYSTEM_HEALTH_CAPABILITY_ERROR, 0},
        {"100000", "100000", SYSTEM_HEALTH_CAPABILITY_STALE, 0},
        {"100000", "100000", SYSTEM_HEALTH_CAPABILITY_AVAILABLE, 0.25f},
        {NULL, "100000", v1 ? SYSTEM_HEALTH_CAPABILITY_ERROR
                             : SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED, 0},
        {"100000", "100000", SYSTEM_HEALTH_CAPABILITY_STALE, 0},
        {"100000", "100000", SYSTEM_HEALTH_CAPABILITY_AVAILABLE, 0.25f},
    };
    for (size_t i = 0; i < sizeof(steps) / sizeof(steps[0]); ++i) {
        char text[256];
        const char *quota_file = v1 ? "sys/cpu,cpuacct/camera/cpu.cfs_quota_us"
                                   : "sys/camera/cpu.max";
        if (!steps[i].quota) {
            char path[512];
            snprintf(path, sizeof(path), "%s/%s", temporary_root, quota_file);
            TEST_ASSERT_EQUAL_INT(0, unlink(path));
        } else if (v1) {
            write_membership_fixture(quota_file,
                strcmp(steps[i].quota, "max") == 0 ? "-1" : steps[i].quota);
            write_membership_fixture("sys/cpu,cpuacct/camera/cpu.cfs_period_us", steps[i].period);
        } else {
            snprintf(text, sizeof(text), "%s %s\n", steps[i].quota, steps[i].period);
            write_membership_fixture(quota_file, text);
        }
        if (v1) {
            snprintf(text, sizeof(text), "%llu\n", (unsigned long long)(i + 1) * 250000000);
            write_membership_fixture("sys/cpu,cpuacct/camera/cpuacct.usage", text);
            snprintf(text, sizeof(text), "nr_periods %zu\nnr_throttled %zu\nthrottled_time %zu\n",
                     (i + 1) * 10, i + 1, (i + 1) * 100000);
            write_membership_fixture("sys/cpu,cpuacct/camera/cpu.stat", text);
        } else {
            snprintf(text, sizeof(text),
                     "usage_usec %zu\nnr_periods %zu\nnr_throttled %zu\nthrottled_usec %zu\n",
                     (i + 1) * 250000, (i + 1) * 10, i + 1, (i + 1) * 100);
            write_membership_fixture("sys/camera/cpu.stat", text);
        }
        context.monotonic_ms = (i + 1) * 1000;
        sink.count = 0;
        TEST_ASSERT_EQUAL_INT(0, collector.collect(collector.state, &context, &sink));
        const system_health_observation_t *usage = find_observation(
            items, sink.count, "container.cpu.usage_ratio");
        TEST_ASSERT_NOT_NULL(usage);
        TEST_ASSERT_EQUAL(steps[i].capability, usage->capability);
        TEST_ASSERT_EQUAL(steps[i].capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                          usage->value_valid);
        if (usage->value_valid)
            TEST_ASSERT_FLOAT_WITHIN(0.0001f, steps[i].ratio, (float)usage->value);
        assert_delta_state(&sink, "container.cpu.throttled_ratio", i > 0, 0.1f);
    }
}

static void test_v1_cpu_quota_changes_and_read_gaps(void) { check_cpu_quota_transitions(true); }
static void test_v2_cpu_quota_changes_and_read_gaps(void) { check_cpu_quota_transitions(false); }

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_v1_cpu_quota_changes_and_read_gaps);
    RUN_TEST(test_v2_cpu_quota_changes_and_read_gaps);
    RUN_TEST(test_cgroup_parsers_reject_bad_limits_and_overflow);
    RUN_TEST(test_v2_effective_limits_headroom_and_safe_deltas);
    RUN_TEST(test_unlimited_v2_reports_host_scope_without_fake_limits);
    RUN_TEST(test_v2_uses_tighter_ancestor_limits);
    RUN_TEST(test_v1_common_combined_controller_layout);
    RUN_TEST(test_v1_separate_cpu_paths_and_optional_metrics);
    RUN_TEST(test_v2_rejects_trailing_dotdot_membership);
    RUN_TEST(test_v2_rejects_intermediate_symlinks);
    RUN_TEST(test_v1_rejects_intermediate_symlinks);
    RUN_TEST(test_v2_rejects_leaf_symlinks);
    RUN_TEST(test_v1_rejects_leaf_symlinks);
    RUN_TEST(test_v2_membership_changes_and_discovery_gaps_reset_deltas);
    RUN_TEST(test_v1_tracks_each_controller_and_long_memberships);
    return UNITY_END();
}
