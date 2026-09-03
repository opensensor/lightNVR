/** @file recording_io_metrics.h Lock-free recording write-failure evidence. */

#ifndef LIGHTNVR_RECORDING_IO_METRICS_H
#define LIGHTNVR_RECORDING_IO_METRICS_H

#include <stdbool.h>
#include <stdint.h>

typedef enum {
    RECORDING_IO_RESOURCE_RECORDING = 0,
    RECORDING_IO_RESOURCE_HLS,
    RECORDING_IO_RESOURCE_COUNT
} recording_io_resource_t;

typedef enum {
    RECORDING_IO_OPERATION_ALLOCATE = 0,
    RECORDING_IO_OPERATION_OPEN,
    RECORDING_IO_OPERATION_HEADER,
    RECORDING_IO_OPERATION_PACKET,
    RECORDING_IO_OPERATION_TRAILER,
    RECORDING_IO_OPERATION_CLOSE,
    RECORDING_IO_OPERATION_FILESYSTEM,
    RECORDING_IO_OPERATION_COUNT
} recording_io_operation_t;

typedef enum {
    RECORDING_IO_REASON_NONE = 0,
    RECORDING_IO_REASON_NO_SPACE,
    RECORDING_IO_REASON_QUOTA,
    RECORDING_IO_REASON_READ_ONLY,
    RECORDING_IO_REASON_IO,
    RECORDING_IO_REASON_TIMEOUT,
    RECORDING_IO_REASON_FD_LIMIT,
    RECORDING_IO_REASON_ALLOCATION,
    RECORDING_IO_REASON_OTHER,
    RECORDING_IO_REASON_COUNT
} recording_io_reason_t;

typedef struct {
    bool valid;
    recording_io_resource_t resource;
    recording_io_operation_t operation;
    recording_io_reason_t reason;
    int error_code;
} recording_io_last_error_t;

typedef struct {
    uint64_t reason_totals[RECORDING_IO_RESOURCE_COUNT]
                          [RECORDING_IO_REASON_COUNT];
    recording_io_last_error_t last_error;
} recording_io_metrics_snapshot_t;

/** Normalize a positive errno or negative FFmpeg AVERROR(errno). */
recording_io_reason_t recording_io_reason_from_error(int error_code);

/**
 * Record one failed writer operation. This performs only lock-free atomic
 * updates and an evaluator atomic increment; it never logs or allocates.
 */
void recording_io_report_failure(recording_io_resource_t resource,
                                 recording_io_operation_t operation,
                                 int error_code);

/** Copy the bounded counters and coherent packed last-error record. */
void recording_io_metrics_snapshot(recording_io_metrics_snapshot_t *snapshot);

/** Consume a coalesced device-health refresh hint after real EIO/timeout. */
bool recording_io_take_device_refresh_request(void);

const char *recording_io_reason_name(recording_io_reason_t reason);

#endif /* LIGHTNVR_RECORDING_IO_METRICS_H */
