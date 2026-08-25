#ifndef LIGHTNVR_CORE_EVENT_PRODUCERS_H
#define LIGHTNVR_CORE_EVENT_PRODUCERS_H

#include <stddef.h>
#include <stdint.h>
#include <time.h>

#include "video/detection_result.h"

/*
 * Normalize and enqueue an object-detection fact. This function performs no
 * MQTT or other network I/O. camera_uuid is the immutable fleet identity;
 * stream_name is compatibility/display metadata only.
 */
int event_producer_publish_detection(
    const char *camera_uuid, const char *stream_name,
    const detection_result_t *result, time_t occurred_at,
    char *error, size_t error_size);

/* Resolve the current stream name to its immutable UUID, then enqueue. */
int event_producer_publish_detection_for_stream(
    const char *stream_name, const detection_result_t *result,
    time_t occurred_at, char *error, size_t error_size);

/*
 * Operational producers resolve mutable stream names to immutable camera UUIDs
 * before enqueueing. They perform no MQTT or other network I/O.
 */
int event_producer_publish_camera_offline_for_stream(
    const char *stream_name, const char *reason, int consecutive_failures,
    time_t occurred_at, char *error, size_t error_size);

int event_producer_publish_camera_recovered_for_stream(
    const char *stream_name, int64_t downtime_ms, time_t occurred_at,
    char *error, size_t error_size);

int event_producer_publish_stream_degraded_for_stream(
    const char *stream_name, const char *reason, double observed_fps,
    double expected_fps, time_t occurred_at, char *error, size_t error_size);

int event_producer_publish_stream_recovered_for_stream(
    const char *stream_name, double observed_fps, double expected_fps,
    time_t occurred_at, char *error, size_t error_size);

int event_producer_publish_recording_gap_for_stream(
    const char *stream_name, time_t started_at, int64_t duration_ms,
    time_t occurred_at, char *error, size_t error_size);

int event_producer_publish_storage_pressure(
    const char *level, const char *previous_level, double used_percent,
    uint64_t free_bytes, time_t occurred_at, char *error, size_t error_size);

int event_producer_publish_storage_recovered(
    const char *previous_level, double used_percent, uint64_t free_bytes,
    time_t occurred_at, char *error, size_t error_size);

#endif /* LIGHTNVR_CORE_EVENT_PRODUCERS_H */
