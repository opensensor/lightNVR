#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include "video/timestamp_manager.h"
#include "core/logger.h"
#include <pthread.h>
#include <string.h>
#include <time.h>
#include <errno.h>
#include <unistd.h>  // For usleep
#include <libavutil/avutil.h>

// Structure to track timestamp information per stream
typedef struct {
    char stream_name[MAX_STREAM_NAME];
    int64_t last_pts;
    int64_t last_dts;
    int64_t pts_discontinuity_count;
    int64_t expected_next_pts;
    bool is_udp_stream;
    bool initialized;
    time_t last_keyframe_time;  // Time when the last keyframe was received
    time_t last_detection_time; // Time when the last detection was performed
} timestamp_tracker_t;

// Array to track timestamps for multiple streams
#define MAX_TIMESTAMP_TRACKERS 16
static timestamp_tracker_t timestamp_trackers[MAX_TIMESTAMP_TRACKERS];

/**
 * Get or create a timestamp tracker for a stream
 */
void *get_timestamp_tracker(const char *stream_name) {
    if (!stream_name) {
        log_error("get_timestamp_tracker: NULL stream name");
        return NULL;
    }
    
    // Make a local copy of the stream name to avoid issues with concurrent access
    char local_stream_name[MAX_STREAM_NAME];
    strncpy(local_stream_name, stream_name, MAX_STREAM_NAME - 1);
    local_stream_name[MAX_STREAM_NAME - 1] = '\0';
    

    // Look for existing tracker
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].initialized && 
            strcmp(timestamp_trackers[i].stream_name, local_stream_name) == 0) {
            return &timestamp_trackers[i];
        }
    }
    
    // Create new tracker
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (!timestamp_trackers[i].initialized) {
            strncpy(timestamp_trackers[i].stream_name, local_stream_name, MAX_STREAM_NAME - 1);
            timestamp_trackers[i].stream_name[MAX_STREAM_NAME - 1] = '\0';
            timestamp_trackers[i].last_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i].last_dts = AV_NOPTS_VALUE;
            timestamp_trackers[i].pts_discontinuity_count = 0;
            timestamp_trackers[i].expected_next_pts = AV_NOPTS_VALUE;
            
            // We'll set this based on the actual protocol when processing packets
            timestamp_trackers[i].is_udp_stream = false;
            
            // Initialize last keyframe time to 0
            timestamp_trackers[i].last_keyframe_time = 0;
            
            // Initialize last detection time to 0
            timestamp_trackers[i].last_detection_time = 0;
            
            timestamp_trackers[i].initialized = true;
            
            log_info("Created new timestamp tracker for stream %s at index %d", 
                    local_stream_name, i);
            
            return &timestamp_trackers[i];
        }
    }
    
    // If we get here, all slots are in use. Try to find a stale tracker to reuse.
    // A tracker is considered stale if it hasn't received a keyframe in the last 5 minutes
    time_t current_time = time(NULL);
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].initialized && 
            timestamp_trackers[i].last_keyframe_time > 0 &&
            (current_time - timestamp_trackers[i].last_keyframe_time) > 300) { // 5 minutes
            
            log_warn("Reusing stale timestamp tracker for stream %s (previously used by %s, last keyframe %ld seconds ago)",
                    local_stream_name, timestamp_trackers[i].stream_name, 
                    (long)(current_time - timestamp_trackers[i].last_keyframe_time));
            
            // Reset the tracker for the new stream
            strncpy(timestamp_trackers[i].stream_name, local_stream_name, MAX_STREAM_NAME - 1);
            timestamp_trackers[i].stream_name[MAX_STREAM_NAME - 1] = '\0';
            timestamp_trackers[i].last_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i].last_dts = AV_NOPTS_VALUE;
            timestamp_trackers[i].pts_discontinuity_count = 0;
            timestamp_trackers[i].expected_next_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i].is_udp_stream = false;
            timestamp_trackers[i].last_keyframe_time = 0;
            timestamp_trackers[i].last_detection_time = 0;
            
            return &timestamp_trackers[i];
        }
    }
    
    // No slots available
    log_error("No available slots for timestamp tracker for stream %s", local_stream_name);
    return NULL;
}

/**
 * Initialize timestamp trackers
 */
void init_timestamp_trackers(void) {

    // Initialize all trackers to unused state
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        timestamp_trackers[i].initialized = false;
        timestamp_trackers[i].last_pts = AV_NOPTS_VALUE;
        timestamp_trackers[i].last_dts = AV_NOPTS_VALUE;
        timestamp_trackers[i].pts_discontinuity_count = 0;
        timestamp_trackers[i].expected_next_pts = AV_NOPTS_VALUE;
        timestamp_trackers[i].is_udp_stream = false;
        timestamp_trackers[i].last_keyframe_time = 0;
        timestamp_trackers[i].last_detection_time = 0;
        timestamp_trackers[i].stream_name[0] = '\0';
    }
    
    log_info("Timestamp trackers initialized");
}

/**
 * Set the UDP flag for a stream's timestamp tracker
 * Creates the tracker if it doesn't exist
 */
void set_timestamp_tracker_udp_flag(const char *stream_name, bool is_udp) {
    if (!stream_name) {
        log_error("set_timestamp_tracker_udp_flag: NULL stream name");
        return;
    }
    
    // Make a local copy of the stream name to avoid issues with concurrent access
    char local_stream_name[MAX_STREAM_NAME];
    strncpy(local_stream_name, stream_name, MAX_STREAM_NAME - 1);
    local_stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Look for existing tracker
    int found = 0;
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].initialized && 
            strcmp(timestamp_trackers[i].stream_name, local_stream_name) == 0) {
            // Set the UDP flag
            timestamp_trackers[i].is_udp_stream = is_udp;
            log_info("Set UDP flag to %s for stream %s timestamp tracker", 
                    is_udp ? "true" : "false", local_stream_name);
            found = 1;
            break;
        }
    }
    
    // If not found, create a new tracker
    if (!found) {
        for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
            if (!timestamp_trackers[i].initialized) {
                // Initialize the new tracker
                strncpy(timestamp_trackers[i].stream_name, local_stream_name, MAX_STREAM_NAME - 1);
                timestamp_trackers[i].stream_name[MAX_STREAM_NAME - 1] = '\0';
                timestamp_trackers[i].last_pts = AV_NOPTS_VALUE;
                timestamp_trackers[i].last_dts = AV_NOPTS_VALUE;
                timestamp_trackers[i].pts_discontinuity_count = 0;
                timestamp_trackers[i].expected_next_pts = AV_NOPTS_VALUE;
                timestamp_trackers[i].is_udp_stream = is_udp;
                timestamp_trackers[i].last_keyframe_time = 0;
                timestamp_trackers[i].last_detection_time = 0;
                timestamp_trackers[i].initialized = true;
                
                log_info("Created new timestamp tracker for stream %s at index %d with UDP flag %s", 
                        local_stream_name, i, is_udp ? "true" : "false");
                found = 1;
                break;
            }
        }
        
        if (!found) {
            log_error("No available slots for timestamp tracker for stream %s", local_stream_name);
        }
    }
}

/**
 * Reset timestamp tracker for a specific stream
 * This should be called when a stream is stopped to ensure clean state when restarted
 */
void reset_timestamp_tracker(const char *stream_name) {
    if (!stream_name) {
        log_error("reset_timestamp_tracker: NULL stream name");
        return;
    }

    // Find the tracker for this stream
    bool found = false;
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].initialized && 
            strcmp(timestamp_trackers[i].stream_name, stream_name) == 0) {
            // Reset the tracker but keep the stream name and initialized flag
            // This ensures we don't lose the UDP flag setting
            timestamp_trackers[i].last_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i].last_dts = AV_NOPTS_VALUE;
            timestamp_trackers[i].pts_discontinuity_count = 0;
            timestamp_trackers[i].expected_next_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i].last_keyframe_time = 0;
            timestamp_trackers[i].last_detection_time = 0;
            
            log_info("Reset timestamp tracker for stream %s (UDP flag: %s)", 
                    stream_name, timestamp_trackers[i].is_udp_stream ? "true" : "false");
            found = true;
            break;
        }
    }
    
    if (!found) {
        log_debug("No timestamp tracker found for stream %s during reset", stream_name);
    }
}

/**
 * Remove timestamp tracker for a specific stream
 * This should be called when a stream is completely removed
 */
void remove_timestamp_tracker(const char *stream_name) {
    if (!stream_name) {
        log_error("remove_timestamp_tracker: NULL stream name");
        return;
    }

    // Find the tracker for this stream
    bool found = false;
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].initialized && 
            strcmp(timestamp_trackers[i].stream_name, stream_name) == 0) {
            // Completely reset the tracker
            timestamp_trackers[i].initialized = false;
            timestamp_trackers[i].stream_name[0] = '\0';
            timestamp_trackers[i].last_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i].last_dts = AV_NOPTS_VALUE;
            timestamp_trackers[i].pts_discontinuity_count = 0;
            timestamp_trackers[i].expected_next_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i].is_udp_stream = false;
            timestamp_trackers[i].last_keyframe_time = 0;
            timestamp_trackers[i].last_detection_time = 0;
            
            log_info("Removed timestamp tracker for stream %s", stream_name);
            found = true;
            break;
        }
    }
    
    if (!found) {
        log_debug("No timestamp tracker found for stream %s during removal", stream_name);
    }
}

/**
 * Cleanup all timestamp trackers
 */
void cleanup_timestamp_trackers(void) {
    log_info("Cleaning up timestamp trackers...");
    // Reset all trackers to unused state
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        timestamp_trackers[i].initialized = false;
        timestamp_trackers[i].stream_name[0] = '\0';
        timestamp_trackers[i].last_pts = AV_NOPTS_VALUE;
        timestamp_trackers[i].last_dts = AV_NOPTS_VALUE;
        timestamp_trackers[i].pts_discontinuity_count = 0;
        timestamp_trackers[i].expected_next_pts = AV_NOPTS_VALUE;
        timestamp_trackers[i].is_udp_stream = false;
        timestamp_trackers[i].last_keyframe_time = 0;
        timestamp_trackers[i].last_detection_time = 0;
    }
    
    log_info("All timestamp trackers cleaned up");
}

/**
 * Update the last keyframe time for a stream
 * This should be called when a keyframe is received
 */
void update_keyframe_time(const char *stream_name) {
    if (!stream_name) {
        log_error("update_keyframe_time: NULL stream name");
        return;
    }

    // Find the tracker for this stream
    bool found = false;
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].initialized && 
            strcmp(timestamp_trackers[i].stream_name, stream_name) == 0) {
            // Enhanced logging for keyframe tracking
            time_t prev_keyframe_time = timestamp_trackers[i].last_keyframe_time;
            timestamp_trackers[i].last_keyframe_time = time(NULL);
            
            // Only log at debug level to avoid filling logs
            log_debug("Updated keyframe time for stream %s: previous=%ld, new=%ld, delta=%ld seconds", 
                    stream_name, 
                    (long)prev_keyframe_time, 
                    (long)timestamp_trackers[i].last_keyframe_time,
                    prev_keyframe_time > 0 ? (long)(timestamp_trackers[i].last_keyframe_time - prev_keyframe_time) : 0);
            found = true;
            break;
        }
    }
    
    if (!found) {
        // Create a new tracker if it doesn't exist
        bool created = false;
        for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
            if (!timestamp_trackers[i].initialized) {
                strncpy(timestamp_trackers[i].stream_name, stream_name, MAX_STREAM_NAME - 1);
                timestamp_trackers[i].stream_name[MAX_STREAM_NAME - 1] = '\0';
                timestamp_trackers[i].last_pts = AV_NOPTS_VALUE;
                timestamp_trackers[i].last_dts = AV_NOPTS_VALUE;
                timestamp_trackers[i].pts_discontinuity_count = 0;
                timestamp_trackers[i].expected_next_pts = AV_NOPTS_VALUE;
                timestamp_trackers[i].is_udp_stream = false;
                timestamp_trackers[i].last_keyframe_time = time(NULL);
                timestamp_trackers[i].last_detection_time = 0;
                timestamp_trackers[i].initialized = true;
                
                log_info("Created new timestamp tracker for stream %s with keyframe time %ld", 
                        stream_name, (long)timestamp_trackers[i].last_keyframe_time);
                created = true;
                break;
            }
        }
        
        // If we couldn't create a new tracker, try to reuse an existing one
        if (!created) {
            // Find the oldest tracker (with the oldest keyframe time)
            int oldest_idx = -1;
            time_t oldest_time = time(NULL);
            
            for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
                if (timestamp_trackers[i].initialized && 
                    timestamp_trackers[i].last_keyframe_time < oldest_time) {
                    oldest_time = timestamp_trackers[i].last_keyframe_time;
                    oldest_idx = i;
                }
            }
            
            if (oldest_idx >= 0) {
                log_warn("Reusing oldest timestamp tracker (from stream %s, last keyframe %ld seconds ago) for stream %s",
                        timestamp_trackers[oldest_idx].stream_name, 
                        (long)(time(NULL) - timestamp_trackers[oldest_idx].last_keyframe_time),
                        stream_name);
                
                // Reset the tracker for the new stream
                strncpy(timestamp_trackers[oldest_idx].stream_name, stream_name, MAX_STREAM_NAME - 1);
                timestamp_trackers[oldest_idx].stream_name[MAX_STREAM_NAME - 1] = '\0';
                timestamp_trackers[oldest_idx].last_pts = AV_NOPTS_VALUE;
                timestamp_trackers[oldest_idx].last_dts = AV_NOPTS_VALUE;
                timestamp_trackers[oldest_idx].pts_discontinuity_count = 0;
                timestamp_trackers[oldest_idx].expected_next_pts = AV_NOPTS_VALUE;
                timestamp_trackers[oldest_idx].is_udp_stream = false;
                timestamp_trackers[oldest_idx].last_keyframe_time = time(NULL);
                timestamp_trackers[oldest_idx].last_detection_time = 0;
            } else {
                log_error("Failed to find or create timestamp tracker for stream %s", stream_name);
            }
        }
    }
}

/**
 * Check if a keyframe was received for a stream after a specific time
 * Returns 1 if a keyframe was received after the specified time, 0 otherwise
 * If keyframe_time is not NULL, it will be set to the time of the last keyframe
 * 
 * BUGFIX: Fixed the function to properly handle the rotation check
 * The keyframe_time parameter is used both as input (check time) and output (last keyframe time)
 */
int last_keyframe_received(const char *stream_name, time_t *keyframe_time) {
    if (!stream_name) {
        log_error("last_keyframe_received: NULL stream name");
        return 0;
    }

    // Find the tracker for this stream
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].initialized && 
            strcmp(timestamp_trackers[i].stream_name, stream_name) == 0) {
            
            // Get the check time from keyframe_time parameter (if provided)
            time_t check_time = 0;
            if (keyframe_time && *keyframe_time > 0) {
                check_time = *keyframe_time;
            }
            
            // Store the last keyframe time for this stream
            time_t last_kf_time = timestamp_trackers[i].last_keyframe_time;
            
            // If keyframe_time is not NULL, set it to the time of the last keyframe (output)
            if (keyframe_time) {
                *keyframe_time = last_kf_time;
            }
            
            // Check if a keyframe was received after the check_time
            int result = 0;
            if (last_kf_time > 0) {
                if (check_time == 0 || last_kf_time > check_time) {
                    result = 1;
                }
            }
            
            // Log the result for debugging
            log_debug("Keyframe check for stream %s: last_keyframe_time=%ld, check_time=%ld, result=%d", 
                    stream_name, (long)last_kf_time, (long)check_time, result);
            
            return result;
        }
    }
    
    // No tracker found for this stream, create one
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (!timestamp_trackers[i].initialized) {
            log_info("Creating new timestamp tracker for stream %s during keyframe check", stream_name);
            
            strncpy(timestamp_trackers[i].stream_name, stream_name, MAX_STREAM_NAME - 1);
            timestamp_trackers[i].stream_name[MAX_STREAM_NAME - 1] = '\0';
            timestamp_trackers[i].last_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i].last_dts = AV_NOPTS_VALUE;
            timestamp_trackers[i].pts_discontinuity_count = 0;
            timestamp_trackers[i].expected_next_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i].is_udp_stream = false;
            timestamp_trackers[i].last_keyframe_time = 0; // No keyframe yet
            timestamp_trackers[i].last_detection_time = 0; // No detection yet
            timestamp_trackers[i].initialized = true;
            
            // If keyframe_time is not NULL, set it to 0
            if (keyframe_time) {
                *keyframe_time = 0;
            }
            
            return 0; // No keyframe yet
        }
    }

    // No tracker found and couldn't create one
    log_warn("No timestamp tracker found for stream %s during last_keyframe_received and couldn't create one", stream_name);
    
    // If keyframe_time is not NULL, set it to 0
    if (keyframe_time) {
        *keyframe_time = 0;
    }
    
    return 0;
}

/**
 * Get the last detection time for a stream
 * Returns 0 if no detection has been performed yet
 */
time_t get_last_detection_time(const char *stream_name) {
    if (!stream_name) {
        log_error("get_last_detection_time: NULL stream name");
        return 0;
    }

    // Find the tracker for this stream
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].initialized && 
            strcmp(timestamp_trackers[i].stream_name, stream_name) == 0) {
            
            // Get the last detection time
            time_t last_detection_time = timestamp_trackers[i].last_detection_time;
            
            // Log the result for debugging
            log_debug("Last detection time for stream %s: %ld", 
                    stream_name, (long)last_detection_time);
            
            return last_detection_time;
        }
    }
    
    // No tracker found for this stream, create one
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (!timestamp_trackers[i].initialized) {
            log_info("Creating new timestamp tracker for stream %s during get_last_detection_time", stream_name);
            
            strncpy(timestamp_trackers[i].stream_name, stream_name, MAX_STREAM_NAME - 1);
            timestamp_trackers[i].stream_name[MAX_STREAM_NAME - 1] = '\0';
            timestamp_trackers[i].last_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i].last_dts = AV_NOPTS_VALUE;
            timestamp_trackers[i].pts_discontinuity_count = 0;
            timestamp_trackers[i].expected_next_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i].is_udp_stream = false;
            timestamp_trackers[i].last_keyframe_time = 0; // No keyframe yet
            timestamp_trackers[i].last_detection_time = 0; // No detection yet
            timestamp_trackers[i].initialized = true;
            
            return 0; // No detection yet
        }
    }

    // No tracker found and couldn't create one
    log_warn("No timestamp tracker found for stream %s during get_last_detection_time and couldn't create one", stream_name);
    
    return 0;
}

/**
 * Update the last detection time for a stream
 * This should be called when a detection is performed
 */
void update_last_detection_time(const char *stream_name, time_t detection_time) {
    if (!stream_name) {
        log_error("update_last_detection_time: NULL stream name");
        return;
    }

    // Find the tracker for this stream
    bool found = false;
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].initialized && 
            strcmp(timestamp_trackers[i].stream_name, stream_name) == 0) {
            // Enhanced logging for detection tracking
            time_t prev_detection_time = timestamp_trackers[i].last_detection_time;
            timestamp_trackers[i].last_detection_time = detection_time;
            
            // Only log at debug level to avoid filling logs
            log_debug("Updated detection time for stream %s: previous=%ld, new=%ld, delta=%ld seconds", 
                    stream_name, 
                    (long)prev_detection_time, 
                    (long)timestamp_trackers[i].last_detection_time,
                    prev_detection_time > 0 ? (long)(timestamp_trackers[i].last_detection_time - prev_detection_time) : 0);
            found = true;
            break;
        }
    }
    
    if (!found) {
        // Create a new tracker if it doesn't exist
        bool created = false;
        for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
            if (!timestamp_trackers[i].initialized) {
                strncpy(timestamp_trackers[i].stream_name, stream_name, MAX_STREAM_NAME - 1);
                timestamp_trackers[i].stream_name[MAX_STREAM_NAME - 1] = '\0';
                timestamp_trackers[i].last_pts = AV_NOPTS_VALUE;
                timestamp_trackers[i].last_dts = AV_NOPTS_VALUE;
                timestamp_trackers[i].pts_discontinuity_count = 0;
                timestamp_trackers[i].expected_next_pts = AV_NOPTS_VALUE;
                timestamp_trackers[i].is_udp_stream = false;
                timestamp_trackers[i].last_keyframe_time = 0;
                timestamp_trackers[i].last_detection_time = detection_time;
                timestamp_trackers[i].initialized = true;
                
                log_info("Created new timestamp tracker for stream %s with detection time %ld", 
                        stream_name, (long)timestamp_trackers[i].last_detection_time);
                created = true;
                break;
            }
        }
        
        // If we couldn't create a new tracker, try to reuse an existing one
        if (!created) {
            // Find the oldest tracker (with the oldest keyframe time)
            int oldest_idx = -1;
            time_t oldest_time = time(NULL);
            
            for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
                if (timestamp_trackers[i].initialized && 
                    timestamp_trackers[i].last_keyframe_time < oldest_time) {
                    oldest_time = timestamp_trackers[i].last_keyframe_time;
                    oldest_idx = i;
                }
            }
            
            if (oldest_idx >= 0) {
                log_warn("Reusing oldest timestamp tracker (from stream %s, last keyframe %ld seconds ago) for stream %s",
                        timestamp_trackers[oldest_idx].stream_name, 
                        (long)(time(NULL) - timestamp_trackers[oldest_idx].last_keyframe_time),
                        stream_name);
                
                // Reset the tracker for the new stream
                strncpy(timestamp_trackers[oldest_idx].stream_name, stream_name, MAX_STREAM_NAME - 1);
                timestamp_trackers[oldest_idx].stream_name[MAX_STREAM_NAME - 1] = '\0';
                timestamp_trackers[oldest_idx].last_pts = AV_NOPTS_VALUE;
                timestamp_trackers[oldest_idx].last_dts = AV_NOPTS_VALUE;
                timestamp_trackers[oldest_idx].pts_discontinuity_count = 0;
                timestamp_trackers[oldest_idx].expected_next_pts = AV_NOPTS_VALUE;
                timestamp_trackers[oldest_idx].is_udp_stream = false;
                timestamp_trackers[oldest_idx].last_keyframe_time = 0;
                timestamp_trackers[oldest_idx].last_detection_time = detection_time;
            } else {
                log_error("Failed to find or create timestamp tracker for stream %s", stream_name);
            }
        }
    }
}
