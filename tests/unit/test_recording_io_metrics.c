#include "unity.h"

#include <errno.h>
#include <pthread.h>
#include <stdint.h>

#include "telemetry/recording_io_metrics.h"
#include "telemetry/system_health_evaluator.h"

static uint32_t immediate_calls;
static system_health_condition_t immediate_condition;
static system_health_immediate_resource_t immediate_resource;

/* The focused Layer-1-style target links the metrics source directly. */
void system_health_evaluator_note_immediate(
    system_health_condition_t condition,
    system_health_immediate_resource_t resource) {
    immediate_calls++;
    immediate_condition = condition;
    immediate_resource = resource;
}

void setUp(void) {
    (void)recording_io_take_device_refresh_request();
    immediate_calls = 0U;
    immediate_condition = SYSTEM_HEALTH_CONDITION_COUNT;
    immediate_resource = SYSTEM_HEALTH_IMMEDIATE_RESOURCE_COUNT;
}

void tearDown(void) {}

static uint64_t reason_total(recording_io_resource_t resource,
                             recording_io_reason_t reason) {
    recording_io_metrics_snapshot_t snapshot;
    recording_io_metrics_snapshot(&snapshot);
    return snapshot.reason_totals[resource][reason];
}

static void test_normalizes_errno_and_ffmpeg_sign(void) {
    TEST_ASSERT_EQUAL(RECORDING_IO_REASON_NO_SPACE,
                      recording_io_reason_from_error(ENOSPC));
    TEST_ASSERT_EQUAL(RECORDING_IO_REASON_NO_SPACE,
                      recording_io_reason_from_error(-ENOSPC));
#ifdef EDQUOT
    TEST_ASSERT_EQUAL(RECORDING_IO_REASON_QUOTA,
                      recording_io_reason_from_error(-EDQUOT));
#endif
    TEST_ASSERT_EQUAL(RECORDING_IO_REASON_READ_ONLY,
                      recording_io_reason_from_error(-EROFS));
    TEST_ASSERT_EQUAL(RECORDING_IO_REASON_IO,
                      recording_io_reason_from_error(-EIO));
    TEST_ASSERT_EQUAL(RECORDING_IO_REASON_TIMEOUT,
                      recording_io_reason_from_error(-ETIMEDOUT));
    TEST_ASSERT_EQUAL(RECORDING_IO_REASON_FD_LIMIT,
                      recording_io_reason_from_error(-EMFILE));
    TEST_ASSERT_EQUAL(RECORDING_IO_REASON_FD_LIMIT,
                      recording_io_reason_from_error(-ENFILE));
    TEST_ASSERT_EQUAL(RECORDING_IO_REASON_ALLOCATION,
                      recording_io_reason_from_error(-ENOMEM));
    TEST_ASSERT_EQUAL(RECORDING_IO_REASON_OTHER,
                      recording_io_reason_from_error(-EINVAL));
}

static void test_io_failure_updates_counter_record_and_evaluator(void) {
    uint64_t before = reason_total(RECORDING_IO_RESOURCE_RECORDING,
                                   RECORDING_IO_REASON_IO);
    recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                RECORDING_IO_OPERATION_PACKET, -EIO);

    recording_io_metrics_snapshot_t snapshot;
    recording_io_metrics_snapshot(&snapshot);
    TEST_ASSERT_EQUAL_UINT64(before + 1U,
        snapshot.reason_totals[RECORDING_IO_RESOURCE_RECORDING]
                              [RECORDING_IO_REASON_IO]);
    TEST_ASSERT_TRUE(snapshot.last_error.valid);
    TEST_ASSERT_EQUAL(RECORDING_IO_RESOURCE_RECORDING,
                      snapshot.last_error.resource);
    TEST_ASSERT_EQUAL(RECORDING_IO_OPERATION_PACKET,
                      snapshot.last_error.operation);
    TEST_ASSERT_EQUAL(RECORDING_IO_REASON_IO, snapshot.last_error.reason);
    TEST_ASSERT_EQUAL_INT(EIO, snapshot.last_error.error_code);
    TEST_ASSERT_EQUAL_UINT32(1U, immediate_calls);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CONDITION_FILESYSTEM_WRITE_FAILED,
                      immediate_condition);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_IMMEDIATE_RECORDING, immediate_resource);
    TEST_ASSERT_TRUE(recording_io_take_device_refresh_request());
    TEST_ASSERT_FALSE(recording_io_take_device_refresh_request());
}

static void test_capacity_and_read_only_map_to_storage_conditions(void) {
    recording_io_report_failure(RECORDING_IO_RESOURCE_HLS,
                                RECORDING_IO_OPERATION_OPEN, -ENOSPC);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CONDITION_FILESYSTEM_BYTES_LOW,
                      immediate_condition);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_IMMEDIATE_RECORDING, immediate_resource);

    recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                RECORDING_IO_OPERATION_HEADER, -EROFS);
    TEST_ASSERT_EQUAL_UINT32(2U, immediate_calls);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CONDITION_FILESYSTEM_READ_ONLY,
                      immediate_condition);
}

static void test_process_resource_failures_are_separate(void) {
    recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                RECORDING_IO_OPERATION_OPEN, -EMFILE);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CONDITION_PROCESS_FD_EXHAUSTION,
                      immediate_condition);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_IMMEDIATE_PROCESS, immediate_resource);

    recording_io_report_failure(RECORDING_IO_RESOURCE_HLS,
                                RECORDING_IO_OPERATION_ALLOCATE, ENOMEM);
    TEST_ASSERT_EQUAL_UINT32(2U, immediate_calls);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CONDITION_PROCESS_ALLOCATION_FAILED,
                      immediate_condition);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_IMMEDIATE_PROCESS, immediate_resource);
}

static void test_mux_error_is_counted_without_disk_evidence(void) {
    uint64_t before = reason_total(RECORDING_IO_RESOURCE_RECORDING,
                                   RECORDING_IO_REASON_OTHER);
    recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                RECORDING_IO_OPERATION_PACKET, -EINVAL);
    TEST_ASSERT_EQUAL_UINT64(before + 1U,
                            reason_total(RECORDING_IO_RESOURCE_RECORDING,
                                         RECORDING_IO_REASON_OTHER));
    TEST_ASSERT_EQUAL_UINT32(0U, immediate_calls);
}

static void *report_other_failures(void *argument) {
    uintptr_t count = (uintptr_t)argument;
    for (uintptr_t index = 0; index < count; ++index) {
        recording_io_report_failure(RECORDING_IO_RESOURCE_HLS,
                                    RECORDING_IO_OPERATION_PACKET, EINVAL);
    }
    return NULL;
}

static void test_concurrent_writers_do_not_lose_reason_counts(void) {
    enum { THREAD_COUNT = 4, REPORTS_PER_THREAD = 1000 };
    pthread_t threads[THREAD_COUNT];
    uint64_t before = reason_total(RECORDING_IO_RESOURCE_HLS,
                                   RECORDING_IO_REASON_OTHER);
    for (size_t index = 0; index < THREAD_COUNT; ++index) {
        TEST_ASSERT_EQUAL_INT(0, pthread_create(
            &threads[index], NULL, report_other_failures,
            (void *)(uintptr_t)REPORTS_PER_THREAD));
    }
    for (size_t index = 0; index < THREAD_COUNT; ++index) {
        TEST_ASSERT_EQUAL_INT(0, pthread_join(threads[index], NULL));
    }
    TEST_ASSERT_EQUAL_UINT64(
        before + (uint64_t)THREAD_COUNT * REPORTS_PER_THREAD,
        reason_total(RECORDING_IO_RESOURCE_HLS, RECORDING_IO_REASON_OTHER));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_normalizes_errno_and_ffmpeg_sign);
    RUN_TEST(test_io_failure_updates_counter_record_and_evaluator);
    RUN_TEST(test_capacity_and_read_only_map_to_storage_conditions);
    RUN_TEST(test_process_resource_failures_are_separate);
    RUN_TEST(test_mux_error_is_counted_without_disk_evidence);
    RUN_TEST(test_concurrent_writers_do_not_lose_reason_counts);
    return UNITY_END();
}
