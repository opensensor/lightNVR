/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#ifndef LIGHTNVR_CORE_EVENT_PRODUCERS_H
#define LIGHTNVR_CORE_EVENT_PRODUCERS_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <time.h>

#include "core/event_envelope.h"
#include "telemetry/system_health_evaluator.h"
#include "video/detection_result.h"

typedef enum {
    EVENT_PRODUCER_ENQUEUE_FAILED = -1,       /* router error or outbox error */
    EVENT_PRODUCER_ENQUEUE_ENQUEUED = 0,      /* at least one new outbox row */
    EVENT_PRODUCER_ENQUEUE_DUPLICATE = 1,     /* every destination already had it */
    EVENT_PRODUCER_ENQUEUE_NO_ROUTE = 2,      /* routes exist; none matched or suppressed */
    EVENT_PRODUCER_ENQUEUE_MQTT_DISABLED = 3, /* only the default broker, which is off */
    EVENT_PRODUCER_ENQUEUE_FULL = 4           /* outbox refused for capacity */
} event_producer_enqueue_result_t;

const char *event_producer_enqueue_result_name(event_producer_enqueue_result_t result);

/*
 * Route an already-built envelope and persist it in the durable outbox for
 * every accepted destination (the path system-health transitions use). It
 * performs no network I/O and does not publish to the in-process bus; callers
 * do that after a non-failure result. Replaying the same event ID is safe:
 * the outbox deduplicates on source + event ID + destination.
 */
event_producer_enqueue_result_t event_producer_enqueue_durable(
    const event_envelope_t *event, char *error, size_t error_size);

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
 * Publish a restricted notification that a protected LPR read exists. The
 * envelope contains only the opaque read UUID and operational context; plate
 * text and derived plate identifiers are forbidden from the event bus.
 */
int event_producer_publish_lpr_read(
    const char *camera_uuid, const char *stream_name, const char *read_uuid,
    const char *source, time_t occurred_at, char *error, size_t error_size);

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

int event_producer_publish_storage_target_unavailable(
    const char *target_uuid, const char *previous_state, const char *reason,
    bool is_default, time_t occurred_at, char *error, size_t error_size);

int event_producer_publish_storage_target_recovered(
    const char *target_uuid, const char *current_state, int64_t downtime_ms,
    bool is_default, time_t occurred_at, char *error, size_t error_size);

/*
 * Publish one already-persisted evaluator transition using the event identity
 * stored with that transition. Replaying the same event_id is safe: durable
 * outbox destinations deduplicate by source, event identity, and destination.
 */
int event_producer_publish_system_health_transition(
    const system_health_transition_t *transition, const char *event_id,
    char *error, size_t error_size);

#endif /* LIGHTNVR_CORE_EVENT_PRODUCERS_H */
