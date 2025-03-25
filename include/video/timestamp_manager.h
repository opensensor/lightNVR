#ifndef TIMESTAMP_MANAGER_H
#define TIMESTAMP_MANAGER_H

#include <stdbool.h>
#include <stdint.h>
#include <time.h>

// Include config.h for consistent MAX_STREAM_NAME definition
#include "core/config.h"

// Use MAX_STREAM_NAME from config.h (256)

// Initialize timestamp trackers
void init_timestamp_trackers(void);

// Set the UDP flag for a stream's timestamp tracker
void set_timestamp_tracker_udp_flag(const char *stream_name, bool is_udp);

// Get timestamp tracker for a stream (exposed for packet processor)
void *get_timestamp_tracker(const char *stream_name);

// Reset timestamp tracker for a specific stream
void reset_timestamp_tracker(const char *stream_name);

// Remove timestamp tracker for a specific stream
void remove_timestamp_tracker(const char *stream_name);

// Cleanup all timestamp trackers
void cleanup_timestamp_trackers(void);

// Update the last keyframe time for a stream
// This should be called when a keyframe is received
void update_keyframe_time(const char *stream_name);

// Check if a keyframe was received for a stream after a specific time
// Returns 1 if a keyframe was received after the specified time, 0 otherwise
// If keyframe_time is not NULL, it will be set to the time of the last keyframe
int last_keyframe_received(const char *stream_name, time_t *keyframe_time);

// Get the last detection time for a stream
// Returns 0 if no detection has been performed yet
time_t get_last_detection_time(const char *stream_name);

// Update the last detection time for a stream
// This should be called when a detection is performed
void update_last_detection_time(const char *stream_name, time_t detection_time);

#endif // TIMESTAMP_MANAGER_H
