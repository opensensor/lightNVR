#include "video/timestamp_manager.h"
#include "core/logger.h"
#include <pthread.h>
#include <string.h>
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
} timestamp_tracker_t;

// Array to track timestamps for multiple streams
#define MAX_TIMESTAMP_TRACKERS 16
static timestamp_tracker_t timestamp_trackers[MAX_TIMESTAMP_TRACKERS];
static pthread_mutex_t timestamp_mutex = PTHREAD_MUTEX_INITIALIZER;

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
    
    pthread_mutex_lock(&timestamp_mutex);
    
    // Look for existing tracker
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].initialized && 
            strcmp(timestamp_trackers[i].stream_name, local_stream_name) == 0) {
            pthread_mutex_unlock(&timestamp_mutex);
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
            
            timestamp_trackers[i].initialized = true;
            
            log_info("Created new timestamp tracker for stream %s at index %d", 
                    local_stream_name, i);
            
            pthread_mutex_unlock(&timestamp_mutex);
            return &timestamp_trackers[i];
        }
    }
    
    // No slots available
    log_error("No available slots for timestamp tracker for stream %s", local_stream_name);
    pthread_mutex_unlock(&timestamp_mutex);
    return NULL;
}

/**
 * Initialize timestamp trackers
 */
void init_timestamp_trackers(void) {
    pthread_mutex_lock(&timestamp_mutex);
    
    // Initialize all trackers to unused state
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        timestamp_trackers[i].initialized = false;
        timestamp_trackers[i].last_pts = AV_NOPTS_VALUE;
        timestamp_trackers[i].last_dts = AV_NOPTS_VALUE;
        timestamp_trackers[i].pts_discontinuity_count = 0;
        timestamp_trackers[i].expected_next_pts = AV_NOPTS_VALUE;
        timestamp_trackers[i].is_udp_stream = false;
        timestamp_trackers[i].stream_name[0] = '\0';
    }
    
    pthread_mutex_unlock(&timestamp_mutex);
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
    
    pthread_mutex_lock(&timestamp_mutex);
    
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
    
    pthread_mutex_unlock(&timestamp_mutex);
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
    
    pthread_mutex_lock(&timestamp_mutex);
    
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
            
            log_info("Reset timestamp tracker for stream %s (UDP flag: %s)", 
                    stream_name, timestamp_trackers[i].is_udp_stream ? "true" : "false");
            found = true;
            break;
        }
    }
    
    if (!found) {
        log_debug("No timestamp tracker found for stream %s during reset", stream_name);
    }
    
    pthread_mutex_unlock(&timestamp_mutex);
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
    
    pthread_mutex_lock(&timestamp_mutex);
    
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
            
            log_info("Removed timestamp tracker for stream %s", stream_name);
            found = true;
            break;
        }
    }
    
    if (!found) {
        log_debug("No timestamp tracker found for stream %s during removal", stream_name);
    }
    
    pthread_mutex_unlock(&timestamp_mutex);
}

/**
 * Cleanup all timestamp trackers
 */
void cleanup_timestamp_trackers(void) {
    pthread_mutex_lock(&timestamp_mutex);
    
    // Reset all trackers to unused state
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        timestamp_trackers[i].initialized = false;
        timestamp_trackers[i].stream_name[0] = '\0';
        timestamp_trackers[i].last_pts = AV_NOPTS_VALUE;
        timestamp_trackers[i].last_dts = AV_NOPTS_VALUE;
        timestamp_trackers[i].pts_discontinuity_count = 0;
        timestamp_trackers[i].expected_next_pts = AV_NOPTS_VALUE;
        timestamp_trackers[i].is_udp_stream = false;
    }
    
    pthread_mutex_unlock(&timestamp_mutex);
    log_info("All timestamp trackers cleaned up");
}
