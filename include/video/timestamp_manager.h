#ifndef TIMESTAMP_MANAGER_H
#define TIMESTAMP_MANAGER_H

#include <stdbool.h>
#include <stdint.h>

// Maximum length of stream name
#define MAX_STREAM_NAME 128

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

#endif // TIMESTAMP_MANAGER_H
