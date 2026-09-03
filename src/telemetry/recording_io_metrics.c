#include "telemetry/recording_io_metrics.h"

#include <errno.h>
#include <limits.h>
#include <stdatomic.h>
#include <stddef.h>

#include "telemetry/system_health_evaluator.h"

/* Writer callbacks must never fall back to an implementation lock. */
_Static_assert(ATOMIC_INT_LOCK_FREE == 2,
               "recording I/O metrics require lock-free 32-bit atomics");

static _Atomic uint32_t reason_totals[RECORDING_IO_RESOURCE_COUNT]
                                      [RECORDING_IO_REASON_COUNT];
static _Atomic uint32_t last_error_packed;
static atomic_bool device_refresh_requested;

/* One 32-bit store publishes a coherent last-error record. */
#define LAST_VALID_SHIFT 31U
#define LAST_RESOURCE_SHIFT 28U
#define LAST_OPERATION_SHIFT 24U
#define LAST_REASON_SHIFT 20U
#define LAST_ERROR_MASK 0x000fffffU

static int normalized_errno(int error_code) {
    if (error_code >= 0) return error_code;
    if (error_code == INT_MIN) return 0;
    return -error_code;
}

recording_io_reason_t recording_io_reason_from_error(int error_code) {
    int value = normalized_errno(error_code);
    switch (value) {
        case ENOSPC: return RECORDING_IO_REASON_NO_SPACE;
#ifdef EDQUOT
        case EDQUOT: return RECORDING_IO_REASON_QUOTA;
#endif
        case EROFS: return RECORDING_IO_REASON_READ_ONLY;
        case EIO: return RECORDING_IO_REASON_IO;
        case ETIMEDOUT: return RECORDING_IO_REASON_TIMEOUT;
        case EMFILE:
        case ENFILE: return RECORDING_IO_REASON_FD_LIMIT;
        case ENOMEM: return RECORDING_IO_REASON_ALLOCATION;
        default: return RECORDING_IO_REASON_OTHER;
    }
}

static void increment_saturating(_Atomic uint32_t *counter) {
    uint32_t current = atomic_load_explicit(counter, memory_order_relaxed);
    while (current != UINT32_MAX &&
           !atomic_compare_exchange_weak_explicit(
               counter, &current, current + 1U, memory_order_relaxed,
               memory_order_relaxed)) {
    }
}

static void notify_evaluator(recording_io_resource_t resource,
                             recording_io_reason_t reason) {
    system_health_condition_t condition;
    switch (reason) {
        case RECORDING_IO_REASON_NO_SPACE:
        case RECORDING_IO_REASON_QUOTA:
            condition = SYSTEM_HEALTH_CONDITION_FILESYSTEM_BYTES_LOW;
            break;
        case RECORDING_IO_REASON_READ_ONLY:
            condition = SYSTEM_HEALTH_CONDITION_FILESYSTEM_READ_ONLY;
            break;
        case RECORDING_IO_REASON_IO:
        case RECORDING_IO_REASON_TIMEOUT:
            condition = SYSTEM_HEALTH_CONDITION_FILESYSTEM_WRITE_FAILED;
            break;
        case RECORDING_IO_REASON_FD_LIMIT:
            system_health_evaluator_note_immediate(
                SYSTEM_HEALTH_CONDITION_PROCESS_FD_EXHAUSTION,
                SYSTEM_HEALTH_IMMEDIATE_PROCESS);
            return;
        case RECORDING_IO_REASON_ALLOCATION:
            system_health_evaluator_note_immediate(
                SYSTEM_HEALTH_CONDITION_PROCESS_ALLOCATION_FAILED,
                SYSTEM_HEALTH_IMMEDIATE_PROCESS);
            return;
        default:
            return;
    }

    /* The current evaluator has root/recording subjects. HLS is ephemeral
     * recording output, so it intentionally correlates with recording health. */
    (void)resource;
    system_health_evaluator_note_immediate(
        condition, SYSTEM_HEALTH_IMMEDIATE_RECORDING);
}

void recording_io_report_failure(recording_io_resource_t resource,
                                 recording_io_operation_t operation,
                                 int error_code) {
    if (resource < 0 || resource >= RECORDING_IO_RESOURCE_COUNT ||
        operation < 0 || operation >= RECORDING_IO_OPERATION_COUNT) {
        return;
    }

    recording_io_reason_t reason = recording_io_reason_from_error(error_code);
    increment_saturating(&reason_totals[resource][reason]);

    uint32_t positive_error = (uint32_t)normalized_errno(error_code);
    if (positive_error > LAST_ERROR_MASK) positive_error = LAST_ERROR_MASK;
    uint32_t packed = (1U << LAST_VALID_SHIFT) |
        ((uint32_t)resource << LAST_RESOURCE_SHIFT) |
        ((uint32_t)operation << LAST_OPERATION_SHIFT) |
        ((uint32_t)reason << LAST_REASON_SHIFT) | positive_error;
    atomic_store_explicit(&last_error_packed, packed, memory_order_release);
    if (reason == RECORDING_IO_REASON_IO ||
        reason == RECORDING_IO_REASON_TIMEOUT)
        atomic_store_explicit(&device_refresh_requested, true,
                              memory_order_release);
    notify_evaluator(resource, reason);
}

bool recording_io_take_device_refresh_request(void) {
    return atomic_exchange_explicit(&device_refresh_requested, false,
                                    memory_order_acq_rel);
}

void recording_io_metrics_snapshot(recording_io_metrics_snapshot_t *snapshot) {
    if (!snapshot) return;
    uint32_t packed = atomic_load_explicit(&last_error_packed,
                                           memory_order_acquire);
    for (size_t resource = 0; resource < RECORDING_IO_RESOURCE_COUNT;
         ++resource) {
        for (size_t reason = 0; reason < RECORDING_IO_REASON_COUNT; ++reason) {
            snapshot->reason_totals[resource][reason] = atomic_load_explicit(
                &reason_totals[resource][reason], memory_order_relaxed);
        }
    }
    snapshot->last_error.valid = ((packed >> LAST_VALID_SHIFT) & 1U) != 0U;
    snapshot->last_error.resource = (recording_io_resource_t)(
        (packed >> LAST_RESOURCE_SHIFT) & 0x7U);
    snapshot->last_error.operation = (recording_io_operation_t)(
        (packed >> LAST_OPERATION_SHIFT) & 0xfU);
    snapshot->last_error.reason = (recording_io_reason_t)(
        (packed >> LAST_REASON_SHIFT) & 0xfU);
    snapshot->last_error.error_code = (int)(packed & LAST_ERROR_MASK);
}

const char *recording_io_reason_name(recording_io_reason_t reason) {
    static const char *const names[RECORDING_IO_REASON_COUNT] = {
        "none", "no_space", "quota", "read_only", "io", "timeout",
        "fd_limit", "allocation", "other"
    };
    if (reason < 0 || reason >= RECORDING_IO_REASON_COUNT) return "unknown";
    return names[reason];
}
