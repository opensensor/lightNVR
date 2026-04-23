#ifndef LIGHTNVR_CROSS_STREAM_MOTION_TRIGGER_H
#define LIGHTNVR_CROSS_STREAM_MOTION_TRIGGER_H

#include <stdint.h>
#include <stdbool.h>
#include <time.h>
#include <pthread.h>
#include <libavformat/avformat.h>
#include "core/config.h"
#include "video/packet_buffer.h"

/**
 * Cross-Stream Motion Trigger Module
 *
 * Fans an incoming motion event out to any streams linked via the
 * `motion_trigger_source` configuration, so a silent PTZ lens can start
 * recording when its paired wide-angle lens fires a motion event, and
 * vice-versa for dual-lens devices that share a single ONVIF endpoint.
 *
 * This used to be onvif_motion_recording.{c,h}; the bulk of that module
 * (per-camera motion recording configuration with its own buffers and
 * retention) was removed along with the separate "Motion Recording (ONVIF)"
 * UI section. What remained was just the cross-stream propagation hook,
 * hence the rename.
 */

// Maximum number of motion events in queue
#define MAX_MOTION_EVENT_QUEUE 100

// Motion event structure
typedef struct {
    char stream_name[MAX_STREAM_NAME];
    time_t timestamp;
    char event_type[64];            // Type of motion event
    float confidence;               // Event confidence (if available)
    bool active;                    // Whether motion is currently active
    bool is_propagated;             // True if this event was already propagated from
                                    // another stream — prevents infinite cross-stream loops
} motion_event_t;

/**
 * Process a motion event (called by ONVIF detection system)
 * 
 * @param stream_name Name of the stream
 * @param motion_detected Whether motion was detected
 * @param timestamp Event timestamp
 * @return 0 on success, non-zero on failure
 */
int process_motion_event(const char *stream_name, bool motion_detected, time_t timestamp, bool is_propagated);

#endif /* LIGHTNVR_CROSS_STREAM_MOTION_TRIGGER_H */

