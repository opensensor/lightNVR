#ifndef LIGHTNVR_CORE_EVENT_PRODUCERS_H
#define LIGHTNVR_CORE_EVENT_PRODUCERS_H

#include <stddef.h>
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

#endif /* LIGHTNVR_CORE_EVENT_PRODUCERS_H */
