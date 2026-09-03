/** @file test_system_health_storage.c Storage slow-probe integration tests. */

#define _POSIX_C_SOURCE 200809L

#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "database/db_core.h"
#include "database/db_storage_targets.h"
#include "storage/storage_target_health.h"
#include "unity.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_system_health_storage.db"
#define TEST_MOUNTINFO_PATH "/tmp/lightnvr_unit_health_mountinfo.txt"

typedef enum {
    FAKE_SUCCESS = 0,
    FAKE_DD_TIMEOUT,
    FAKE_STAT_MALFORMED,
    FAKE_RM_FAILURE
} fake_mode_t;

static fake_mode_t fake_mode;
static unsigned stat_calls;
static unsigned dd_calls;
static unsigned rm_calls;
static char recording_root[] = "/tmp/lightnvr-health-storage-XXXXXX";
static char default_uuid[LIGHTNVR_UUID_STRING_SIZE];

static int fake_run_helper(const health_helper_request_t *request,
                           health_helper_result_t *result) {
    memset(result, 0, sizeof(*result));
    if (strcmp(request->program, "/fake/stat") == 0) {
        stat_calls++;
        result->outcome = HEALTH_HELPER_OK;
        result->latency_ms = 5U;
        snprintf(result->output, sizeof(result->output), "%s",
                 fake_mode == FAKE_STAT_MALFORMED
                     ? "4096 18446744073709551615 1 100 80 abc"
                     : "4096 1000 200 100 80 abc");
        result->output_length = strlen(result->output);
        return 0;
    }
    if (strcmp(request->program, "/fake/dd") == 0) {
        dd_calls++;
        result->latency_ms = fake_mode == FAKE_DD_TIMEOUT ? 3000U : 7U;
        result->outcome = fake_mode == FAKE_DD_TIMEOUT
            ? HEALTH_HELPER_TIMED_OUT : HEALTH_HELPER_OK;
        result->abandoned = fake_mode == FAKE_DD_TIMEOUT;
        return 0;
    }
    if (strcmp(request->program, "/fake/rm") == 0) {
        rm_calls++;
        result->latency_ms = 3U;
        result->outcome = fake_mode == FAKE_RM_FAILURE
            ? HEALTH_HELPER_EXITED : HEALTH_HELPER_OK;
        if (fake_mode == FAKE_RM_FAILURE) {
            result->exit_code = 1;
            snprintf(result->output, sizeof(result->output),
                     "Input/output error");
            result->output_length = strlen(result->output);
        }
        return 0;
    }
    result->outcome = HEALTH_HELPER_EXEC_ERROR;
    return 0;
}

static storage_target_probe_ops_t fake_ops(void) {
    storage_target_probe_ops_t ops = {
        .run_helper = fake_run_helper,
        .stat_program = "/fake/stat",
        .dd_program = "/fake/dd",
        .rm_program = "/fake/rm",
        .mountinfo_path = TEST_MOUNTINFO_PATH
    };
    return ops;
}

static linux_filesystem_resource_t resource_for(const char *logical_id,
                                                const char *path) {
    linux_filesystem_resource_t resource;
    memset(&resource, 0, sizeof(resource));
    snprintf(resource.logical_id, sizeof(resource.logical_id), "%s",
             logical_id);
    snprintf(resource.path, sizeof(resource.path), "%s", path);
    return resource;
}

void setUp(void) {
    fake_mode = FAKE_SUCCESS;
    stat_calls = 0U;
    dd_calls = 0U;
    rm_calls = 0U;
    FILE *mountinfo = fopen(TEST_MOUNTINFO_PATH, "w");
    TEST_ASSERT_NOT_NULL(mountinfo);
    fputs("36 25 0:31 / / rw,relatime - overlay overlay rw\n", mountinfo);
    fclose(mountinfo);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(
        get_db_handle(), "DELETE FROM detections;DELETE FROM recordings;"
                         "DELETE FROM storage_targets;", NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(
        0, db_storage_target_bootstrap_default(recording_root, default_uuid));
}

void tearDown(void) { unlink(TEST_MOUNTINFO_PATH); }

static void test_successful_probe_reports_bounded_fsync_unlink_and_capacity(void) {
    linux_filesystem_resource_t resource =
        resource_for("recording", "/private/storage");
    storage_target_probe_ops_t ops = fake_ops();
    storage_target_health_sample_t sample;
    TEST_ASSERT_EQUAL_INT(0, storage_target_health_probe_with_ops(
        &resource, true, 3000U, &ops, &sample));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                      sample.filesystem.capacity_bytes.capability);
    TEST_ASSERT_EQUAL_UINT64(4096000U,
                             sample.filesystem.capacity_bytes.value);
    TEST_ASSERT_EQUAL_UINT64(819200U,
                             sample.filesystem.available_bytes.value);
    TEST_ASSERT_EQUAL_UINT64(100U,
                             sample.filesystem.capacity_inodes.value);
    TEST_ASSERT_EQUAL_UINT64(80U,
                             sample.filesystem.available_inodes.value);
    TEST_ASSERT_EQUAL_STRING("linux-fs-0000000000000abc",
                             sample.filesystem.device_key);
    TEST_ASSERT_TRUE(sample.writeable.value);
    TEST_ASSERT_TRUE(sample.probe.write_completed);
    TEST_ASSERT_TRUE(sample.probe.fsync_completed);
    TEST_ASSERT_TRUE(sample.probe.unlink_completed);
    TEST_ASSERT_EQUAL_UINT32(15U, sample.probe.latency_ms);
    TEST_ASSERT_EQUAL_UINT(1U, stat_calls);
    TEST_ASSERT_EQUAL_UINT(1U, dd_calls);
    TEST_ASSERT_EQUAL_UINT(1U, rm_calls);
}

static void test_timed_out_write_probe_is_explicit_and_never_blocks_caller(void) {
    fake_mode = FAKE_DD_TIMEOUT;
    linux_filesystem_resource_t resource =
        resource_for("target:550e8400-e29b-41d4-a716-446655440000",
                     "/private/nas");
    storage_target_probe_ops_t ops = fake_ops();
    storage_target_health_sample_t sample;
    TEST_ASSERT_EQUAL_INT(0, storage_target_health_probe_with_ops(
        &resource, true, 3000U, &ops, &sample));
    TEST_ASSERT_EQUAL(LINUX_FILESYSTEM_PROBE_ERROR_TIMED_OUT,
                      sample.probe.error);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR,
                      sample.probe.capability);
    TEST_ASSERT_EQUAL_UINT32(3005U, sample.probe.latency_ms);
    TEST_ASSERT_EQUAL_UINT(0U, rm_calls);
    TEST_ASSERT_TRUE(sample.cleanup_failed);
    TEST_ASSERT_FALSE(sample.probe.unlink_completed);
}

static void test_malformed_overflow_and_cleanup_failure_are_safe(void) {
    linux_filesystem_resource_t resource =
        resource_for("recording", "/private/storage");
    storage_target_probe_ops_t ops = fake_ops();
    storage_target_health_sample_t sample;
    fake_mode = FAKE_STAT_MALFORMED;
    TEST_ASSERT_EQUAL_INT(0, storage_target_health_probe_with_ops(
        &resource, true, 3000U, &ops, &sample));
    TEST_ASSERT_EQUAL(LINUX_FILESYSTEM_PROBE_ERROR_INVALID,
                      sample.probe.error);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR,
                      sample.probe.capability);
    TEST_ASSERT_EQUAL_UINT(0U, dd_calls);

    fake_mode = FAKE_RM_FAILURE;
    TEST_ASSERT_EQUAL_INT(0, storage_target_health_probe_with_ops(
        &resource, true, 3000U, &ops, &sample));
    TEST_ASSERT_TRUE(sample.writeable.value);
    TEST_ASSERT_TRUE(sample.cleanup_failed);
    TEST_ASSERT_EQUAL(LINUX_FILESYSTEM_PROBE_ERROR_IO, sample.probe.error);
    TEST_ASSERT_FALSE(sample.probe.unlink_completed);
}

static size_t count_metric(const system_health_observation_t *items,
                           size_t count, const char *metric) {
    size_t matches = 0U;
    for (size_t index = 0; index < count; ++index)
        if (strcmp(items[index].metric, metric) == 0) matches++;
    return matches;
}

static void test_collector_exposes_each_logical_resource_and_deduplicates_device(void) {
    storage_target_health_collector_state_t state;
    storage_target_health_collector_state_init(
        &state, "/", recording_root, false);
    state.ops = fake_ops();
    state.max_probes_per_cycle = SYSTEM_HEALTH_MAX_FILESYSTEMS;
    system_health_observation_t observations[SYSTEM_HEALTH_MAX_OBSERVATIONS];
    system_health_observation_sink_t sink = {
        .items = observations, .capacity = SYSTEM_HEALTH_MAX_OBSERVATIONS
    };
    system_health_collect_context_t context = {
        .monotonic_ms = 1000U, .wall_time_ms = 1700000000000LL,
        .proc_root = "/unused"
    };
    TEST_ASSERT_EQUAL_INT(
        0, storage_target_health_collect(&state, &context, &sink));
    TEST_ASSERT_EQUAL_size_t(3U, count_metric(
        observations, sink.count, "storage.filesystem.capacity_bytes"));
    TEST_ASSERT_EQUAL_size_t(3U, count_metric(
        observations, sink.count, "storage.filesystem.available_inodes"));
    TEST_ASSERT_EQUAL_size_t(3U, count_metric(
        observations, sink.count, "storage.filesystem.writeable"));
    TEST_ASSERT_EQUAL_size_t(3U, count_metric(
        observations, sink.count, "storage.filesystem.probe_latency_seconds"));
    TEST_ASSERT_EQUAL_size_t(0U, count_metric(
        observations, sink.count, "filesystem.capacity_bytes"));
    TEST_ASSERT_EQUAL_size_t(1U, count_metric(
        observations, sink.count, "filesystem.write_failed"));
    TEST_ASSERT_EQUAL_UINT(3U, stat_calls);
}

int main(void) {
    TEST_ASSERT_NOT_NULL(mkdtemp(recording_root));
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_successful_probe_reports_bounded_fsync_unlink_and_capacity);
    RUN_TEST(test_timed_out_write_probe_is_explicit_and_never_blocks_caller);
    RUN_TEST(test_malformed_overflow_and_cleanup_failure_are_safe);
    RUN_TEST(test_collector_exposes_each_logical_resource_and_deduplicates_device);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    rmdir(recording_root);
    return result;
}
