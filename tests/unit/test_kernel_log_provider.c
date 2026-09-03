#define _POSIX_C_SOURCE 200809L

#include "unity.h"

#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <string.h>

#include "telemetry/providers/kernel_log.h"

#ifndef TEST_KERNEL_LOG_FIXTURE_DIR
#define TEST_KERNEL_LOG_FIXTURE_DIR \
    "tests/fixtures/system_health/kernel_log"
#endif

static int fake_open_error;
static int fake_flags;
static size_t fake_read_index;
static size_t fake_read_calls;

void setUp(void) {
    fake_open_error = 0;
    fake_flags = 0;
    fake_read_index = 0U;
    fake_read_calls = 0U;
}

void tearDown(void) {}

static const system_health_observation_t *find_observation(
    const system_health_observation_t *items, size_t count,
    const char *metric) {
    for (size_t index = 0U; index < count; ++index)
        if (strcmp(items[index].metric, metric) == 0) return &items[index];
    return NULL;
}

static void test_classifier_normalizes_known_evidence_without_exporting_line(void) {
    const char *line = "3,42,1000,-;EXT4-fs: remounting filesystem "
                       "read-only after Buffer I/O error /secret/path";
    bool matches[KERNEL_LOG_CATEGORY_COUNT];
    uint64_t sequence = 0U;
    bool sequence_valid = false;
    TEST_ASSERT_EQUAL_INT(
        0, kernel_log_classify_line(line, strlen(line), matches, &sequence,
                                    &sequence_valid));
    TEST_ASSERT_TRUE(sequence_valid);
    TEST_ASSERT_EQUAL_UINT64(42U, sequence);
    TEST_ASSERT_TRUE(matches[KERNEL_LOG_FILESYSTEM_REMOUNT]);
    TEST_ASSERT_TRUE(matches[KERNEL_LOG_BLOCK_IO_ERROR]);
    TEST_ASSERT_FALSE(matches[KERNEL_LOG_MACHINE_CHECK]);

    const char *overflow =
        "3,18446744073709551616,1,-;Out of memory: killed process";
    TEST_ASSERT_EQUAL_INT(
        0, kernel_log_classify_line(overflow, strlen(overflow), matches,
                                    &sequence, &sequence_valid));
    TEST_ASSERT_FALSE(sequence_valid);
    TEST_ASSERT_TRUE(matches[KERNEL_LOG_OOM_KILL]);
}

static void test_fixture_records_are_bounded_counted_and_deduplicated(void) {
    char path[512];
    snprintf(path, sizeof(path), "%s/messages.txt",
             TEST_KERNEL_LOG_FIXTURE_DIR);
    kernel_log_state_t state;
    kernel_log_state_init(&state, path);
    system_health_collect_context_t context = {
        .monotonic_ms = 1000U, .wall_time_ms = 1700000000000LL
    };
    system_health_provider_inventory_t inventory;
    TEST_ASSERT_EQUAL_INT(0, kernel_log_discover(&state, &context,
                                                &inventory));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_AVAILABLE, state.capability);
    system_health_observation_t items[32];
    system_health_observation_sink_t sink = {
        .items = items, .capacity = 32U
    };
    TEST_ASSERT_EQUAL_INT(0, kernel_log_collect(&state, &context, &sink));
    static const char *const metrics[] = {
        "kernel.filesystem_remount_delta",
        "kernel.block_io_error_delta",
        "kernel.machine_check_delta",
        "kernel.thermal_shutdown_delta",
        "kernel.oom_kill_delta"
    };
    for (size_t index = 0U; index < sizeof(metrics) / sizeof(metrics[0]);
         ++index) {
        const system_health_observation_t *observation =
            find_observation(items, sink.count, metrics[index]);
        TEST_ASSERT_NOT_NULL(observation);
        TEST_ASSERT_TRUE(observation->value_valid);
        TEST_ASSERT_FLOAT_WITHIN(0.001f, 1.0f,
                                 (float)observation->value);
        TEST_ASSERT_EQUAL_STRING("kernel", observation->resource_id);
        TEST_ASSERT_NULL(strstr(observation->resource_id, "secret"));
        TEST_ASSERT_NULL(strstr(observation->metric, "secret"));
    }
    context.monotonic_ms = 2000U;
    sink.count = 0U;
    TEST_ASSERT_EQUAL_INT(0, kernel_log_collect(&state, &context, &sink));
    const system_health_observation_t *oom = find_observation(
        items, sink.count, "kernel.oom_kill_delta");
    TEST_ASSERT_NOT_NULL(oom);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.0f, (float)oom->value);
    kernel_log_destroy(&state);
}

static int fake_open(const char *path, int flags) {
    TEST_ASSERT_EQUAL_STRING("/fixture/kmsg", path);
    fake_flags = flags;
    if (fake_open_error) {
        errno = fake_open_error;
        return -1;
    }
    return 7;
}

static ssize_t fake_read(int descriptor, void *buffer, size_t size) {
    static const char *const lines[] = {
        "3,200,1,-;Out of memory: Killed process 9\n",
        "3,200,1,-;Out of memory: Killed process 9\n",
        "3,201,2,-;ordinary message\n"
    };
    TEST_ASSERT_EQUAL_INT(7, descriptor);
    fake_read_calls++;
    if (fake_read_index >= sizeof(lines) / sizeof(lines[0])) {
        errno = EAGAIN;
        return -1;
    }
    size_t length = strlen(lines[fake_read_index]);
    TEST_ASSERT_TRUE(length <= size);
    memcpy(buffer, lines[fake_read_index], length);
    fake_read_index++;
    return (ssize_t)length;
}

static int fake_close(int descriptor) {
    TEST_ASSERT_EQUAL_INT(7, descriptor);
    return 0;
}

static void test_injected_reader_is_nonblocking_bounded_and_deduplicated(void) {
    kernel_log_state_t state;
    kernel_log_state_init(&state, "/fixture/kmsg");
    state.ops.open_log = fake_open;
    state.ops.read_log = fake_read;
    state.ops.close_log = fake_close;
    system_health_collect_context_t context = {.monotonic_ms = 1U};
    system_health_provider_inventory_t inventory;
    TEST_ASSERT_EQUAL_INT(0, kernel_log_discover(&state, &context,
                                                &inventory));
    TEST_ASSERT_TRUE((fake_flags & O_NONBLOCK) != 0);
    TEST_ASSERT_TRUE((fake_flags & O_CLOEXEC) != 0);
    system_health_observation_t items[16];
    system_health_observation_sink_t sink = {
        .items = items, .capacity = 16U
    };
    TEST_ASSERT_EQUAL_INT(0, kernel_log_collect(&state, &context, &sink));
    TEST_ASSERT_TRUE(fake_read_calls <= KERNEL_LOG_READS_PER_CYCLE);
    const system_health_observation_t *oom = find_observation(
        items, sink.count, "kernel.oom_kill_delta");
    TEST_ASSERT_NOT_NULL(oom);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 1.0f, (float)oom->value);
    kernel_log_destroy(&state);
}

static void test_permission_denial_is_a_capability_not_an_event(void) {
    kernel_log_state_t state;
    kernel_log_state_init(&state, "/fixture/kmsg");
    state.ops.open_log = fake_open;
    state.ops.read_log = fake_read;
    state.ops.close_log = fake_close;
    fake_open_error = EACCES;
    system_health_collect_context_t context = {.monotonic_ms = 1U};
    system_health_provider_inventory_t inventory;
    TEST_ASSERT_EQUAL_INT(0, kernel_log_discover(&state, &context,
                                                &inventory));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED,
                      state.capability);
    system_health_observation_t items[16];
    system_health_observation_sink_t sink = {
        .items = items, .capacity = 16U
    };
    TEST_ASSERT_EQUAL_INT(0, kernel_log_collect(&state, &context, &sink));
    const system_health_observation_t *oom = find_observation(
        items, sink.count, "kernel.oom_kill_delta");
    TEST_ASSERT_NOT_NULL(oom);
    TEST_ASSERT_FALSE(oom->value_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED,
                      oom->capability);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_classifier_normalizes_known_evidence_without_exporting_line);
    RUN_TEST(test_fixture_records_are_bounded_counted_and_deduplicated);
    RUN_TEST(test_injected_reader_is_nonblocking_bounded_and_deduplicated);
    RUN_TEST(test_permission_denial_is_a_capability_not_an_event);
    return UNITY_END();
}
