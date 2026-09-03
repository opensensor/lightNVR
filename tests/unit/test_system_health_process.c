#define _POSIX_C_SOURCE 200809L

#include "unity.h"

#include "telemetry/collectors/linux_process.h"

#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

static char temporary_root[] = "/tmp/lightnvr-health-process-XXXXXX";

void setUp(void) {}
void tearDown(void) {}

static void path_join(char *output, size_t output_size, const char *directory,
                      const char *name) {
    int length = snprintf(output, output_size, "%s/%s", directory, name);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, length);
    TEST_ASSERT_LESS_THAN_size_t(output_size, (size_t)length);
}

static void write_text(const char *path, const char *contents) {
    FILE *output = fopen(path, "wb");
    TEST_ASSERT_NOT_NULL(output);
    size_t length = strlen(contents);
    TEST_ASSERT_EQUAL_size_t(length, fwrite(contents, 1U, length, output));
    TEST_ASSERT_EQUAL_INT(0, fclose(output));
}

static const system_health_observation_t *find_observation(
    const system_health_observation_t *items, size_t count,
    const char *metric) {
    for (size_t index = 0; index < count; ++index) {
        if (strcmp(items[index].metric, metric) == 0) return &items[index];
    }
    return NULL;
}

static void test_status_parser_preserves_zero_and_rejects_overflow(void) {
    const char valid[] = "Name:\tlightnvr\nVmRSS:\t0 kB\nThreads:\t7\n";
    linux_process_sample_t sample;
    memset(&sample, 0, sizeof(sample));
    TEST_ASSERT_EQUAL_INT(0, linux_process_parse_status_text(
        valid, sizeof(valid) - 1U, &sample));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                      sample.rss_bytes.capability);
    TEST_ASSERT_EQUAL_UINT64(0, sample.rss_bytes.value);
    TEST_ASSERT_EQUAL_UINT64(7, sample.thread_count.value);

    const char overflow[] =
        "VmRSS: 18014398509481984 kB\nThreads: 18446744073709551616\n";
    TEST_ASSERT_EQUAL_INT(0, linux_process_parse_status_text(
        overflow, sizeof(overflow) - 1U, &sample));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR,
                      sample.rss_bytes.capability);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR,
                      sample.thread_count.capability);
}

static void test_sample_uses_bounded_fd_walk_and_effective_pid_limit(void) {
    char status_path[512];
    char pids_path[512];
    char fd_path[512];
    char first_fd[512];
    char second_fd[512];
    path_join(status_path, sizeof(status_path), temporary_root, "status");
    path_join(pids_path, sizeof(pids_path), temporary_root, "pids.max");
    path_join(fd_path, sizeof(fd_path), temporary_root, "fd");
    TEST_ASSERT_EQUAL_INT(0, mkdir(fd_path, 0700));
    path_join(first_fd, sizeof(first_fd), fd_path, "3");
    path_join(second_fd, sizeof(second_fd), fd_path, "4");
    write_text(status_path, "VmRSS: 123 kB\nThreads: 5\n");
    write_text(pids_path, "80\n");
    write_text(first_fd, "");
    write_text(second_fd, "");

    linux_process_sample_request_t request;
    memset(&request, 0, sizeof(request));
    request.status_path = status_path;
    request.fd_directory_path = fd_path;
    request.pids_max_path = pids_path;
    request.fd_scan_limit = 8U;
    request.nofile_limit_supplied = true;
    request.nofile_limit = 16U;
    request.nproc_limit_supplied = true;
    request.nproc_limit = 200U;

    linux_process_sample_t sample;
    TEST_ASSERT_EQUAL_INT(0, linux_process_sample(&request, &sample));
    TEST_ASSERT_EQUAL_UINT64(123U * 1024U, sample.rss_bytes.value);
    TEST_ASSERT_EQUAL_UINT64(5, sample.thread_count.value);
    TEST_ASSERT_EQUAL_UINT64(2, sample.open_fd_count.value);
    TEST_ASSERT_EQUAL_UINT64(16, sample.effective_fd_limit.value);
    TEST_ASSERT_EQUAL_UINT64(80, sample.effective_pid_limit.value);

    request.fd_scan_limit = 1U;
    TEST_ASSERT_EQUAL_INT(0, linux_process_sample(&request, &sample));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR,
                      sample.open_fd_count.capability);

    TEST_ASSERT_EQUAL_INT(0, unlink(first_fd));
    TEST_ASSERT_EQUAL_INT(0, unlink(second_fd));
    TEST_ASSERT_EQUAL_INT(0, unlink(status_path));
    TEST_ASSERT_EQUAL_INT(0, unlink(pids_path));
    TEST_ASSERT_EQUAL_INT(0, rmdir(fd_path));
}

static void test_unlimited_and_missing_limits_are_not_reported_as_zero(void) {
    char missing[512];
    path_join(missing, sizeof(missing), temporary_root, "missing-pids.max");
    linux_process_sample_request_t request;
    memset(&request, 0, sizeof(request));
    request.status_path = missing;
    request.fd_directory_path = missing;
    request.pids_max_path = missing;
    request.nofile_limit_supplied = true;
    request.nofile_limit_unlimited = true;
    request.nproc_limit_supplied = true;
    request.nproc_limit_unlimited = true;

    linux_process_sample_t sample;
    TEST_ASSERT_EQUAL_INT(0, linux_process_sample(&request, &sample));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
                      sample.effective_fd_limit.capability);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
                      sample.effective_pid_limit.capability);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR,
                      sample.rss_bytes.capability);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR,
                      sample.open_fd_count.capability);
}

static void test_zero_limit_ratio_is_available_and_saturated(void) {
    char status_path[512];
    char pids_path[512];
    char fd_path[512];
    char fd_file[512];
    path_join(status_path, sizeof(status_path), temporary_root, "status-zero");
    path_join(pids_path, sizeof(pids_path), temporary_root, "pids-zero");
    path_join(fd_path, sizeof(fd_path), temporary_root, "fd-zero");
    TEST_ASSERT_EQUAL_INT(0, mkdir(fd_path, 0700));
    path_join(fd_file, sizeof(fd_file), fd_path, "7");
    write_text(status_path, "VmRSS: 1 kB\nThreads: 1\n");
    write_text(pids_path, "0\n");
    write_text(fd_file, "");

    linux_process_collector_state_t state;
    memset(&state, 0, sizeof(state));
    snprintf(state.status_path, sizeof(state.status_path), "%s", status_path);
    snprintf(state.fd_directory_path, sizeof(state.fd_directory_path), "%s",
             fd_path);
    snprintf(state.pids_max_path, sizeof(state.pids_max_path), "%s",
             pids_path);
    state.fd_scan_limit = 8U;

    system_health_observation_t items[8];
    system_health_observation_sink_t sink = {
        .items = items, .capacity = 8U, .count = 0U, .dropped = 0U
    };
    system_health_collect_context_t context = {
        .monotonic_ms = 25U, .wall_time_ms = 50
    };
    TEST_ASSERT_EQUAL_INT(0, linux_process_collect(&state, &context, &sink));
    const system_health_observation_t *pid_ratio =
        find_observation(items, sink.count, "process.pid_ratio");
    TEST_ASSERT_NOT_NULL(pid_ratio);
    TEST_ASSERT_TRUE(pid_ratio->value_valid);
    TEST_ASSERT_TRUE(pid_ratio->value == 1.0);
    TEST_ASSERT_EQUAL_STRING("lightnvr", pid_ratio->resource_id);

    TEST_ASSERT_EQUAL_INT(0, unlink(fd_file));
    TEST_ASSERT_EQUAL_INT(0, unlink(status_path));
    TEST_ASSERT_EQUAL_INT(0, unlink(pids_path));
    TEST_ASSERT_EQUAL_INT(0, rmdir(fd_path));
}

int main(void) {
    char *created = mkdtemp(temporary_root);
    if (!created) return 2;
    UNITY_BEGIN();
    RUN_TEST(test_status_parser_preserves_zero_and_rejects_overflow);
    RUN_TEST(test_sample_uses_bounded_fd_walk_and_effective_pid_limit);
    RUN_TEST(test_unlimited_and_missing_limits_are_not_reported_as_zero);
    RUN_TEST(test_zero_limit_ratio_is_available_and_saturated);
    int result = UNITY_END();
    rmdir(temporary_root);
    return result;
}
