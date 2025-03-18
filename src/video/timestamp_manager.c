#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <libavutil/avutil.h>

#include "core/logger.h"
#include "video/timestamp_manager.h"

// Maximum number of streams that can have timestamp trackers
#define MAX_TIMESTAMP_TRACKERS 32
#define MAX_STREAM_NAME_LENGTH 64

// Timestamp tracker structure
typedef struct {
    char stream_name[MAX_STREAM_NAME_LENGTH];
    int64_t last_pts;
    int64_t last_dts;
    int64_t expected_next_pts;
    bool timestamps_initialized;
    bool is_udp;
    int pts_discontinuity_count;
    bool in_use;  // Flag to indicate if this tracker is in use
} timestamp_tracker_t;

// Array of timestamp trackers
static timestamp_tracker_t timestamp_trackers[MAX_TIMESTAMP_TRACKERS];
static pthread_mutex_t timestamp_mutex = PTHREAD_MUTEX_INITIALIZER;
static bool trackers_initialized = false;

/**
 * Initialize all timestamp trackers
 */
int init_timestamp_trackers(void) {
    pthread_mutex_lock(&timestamp_mutex);
    
    if (trackers_initialized) {
        pthread_mutex_unlock(&timestamp_mutex);
        log_debug("Timestamp trackers already initialized");
        return 0;
    }
    
    // Clear all trackers
    memset(timestamp_trackers, 0, sizeof(timestamp_trackers));
    trackers_initialized = true;
    
    pthread_mutex_unlock(&timestamp_mutex);
    
    log_info("Timestamp trackers initialized");
    return 0;
}

/**
 * Cleanup all timestamp trackers
 */
int cleanup_timestamp_trackers(void) {
    pthread_mutex_lock(&timestamp_mutex);
    
    // Clear all trackers
    memset(timestamp_trackers, 0, sizeof(timestamp_trackers));
    trackers_initialized = false;
    
    pthread_mutex_unlock(&timestamp_mutex);
    
    log_info("Timestamp trackers cleaned up");
    return 0;
}

/**
 * Initialize the timestamp tracker for a stream
 */
int init_timestamp_tracker(const char *stream_name) {
    if (!stream_name) {
        log_error("Cannot initialize timestamp tracker: NULL stream name");
        return -1;
    }
    
    // Initialize trackers if not already done
    if (!trackers_initialized) {
        init_timestamp_trackers();
    }
    
    pthread_mutex_lock(&timestamp_mutex);
    
    // Check if tracker already exists
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].in_use && 
            strcmp(timestamp_trackers[i].stream_name, stream_name) == 0) {
            // Already exists, just reset it
            timestamp_trackers[i].last_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i].last_dts = AV_NOPTS_VALUE;
            timestamp_trackers[i].expected_next_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i].timestamps_initialized = false;
            timestamp_trackers[i].pts_discontinuity_count = 0;
            // Don't change the is_udp flag
            
            pthread_mutex_unlock(&timestamp_mutex);
            log_info("Reset existing timestamp tracker for stream %s", stream_name);
            return 0;
        }
    }
    
    // Find empty slot
    int slot = -1;
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (!timestamp_trackers[i].in_use) {
            slot = i;
            break;
        }
    }
    
    if (slot == -1) {
        pthread_mutex_unlock(&timestamp_mutex);
        log_error("No slot available for new timestamp tracker");
        return -1;
    }
    
    // Initialize tracker
    memset(&timestamp_trackers[slot], 0, sizeof(timestamp_tracker_t));
    strncpy(timestamp_trackers[slot].stream_name, stream_name, MAX_STREAM_NAME_LENGTH - 1);
    timestamp_trackers[slot].stream_name[MAX_STREAM_NAME_LENGTH - 1] = '\0';
    timestamp_trackers[slot].last_pts = AV_NOPTS_VALUE;
    timestamp_trackers[slot].last_dts = AV_NOPTS_VALUE;
    timestamp_trackers[slot].expected_next_pts = AV_NOPTS_VALUE;
    timestamp_trackers[slot].timestamps_initialized = false;
    timestamp_trackers[slot].is_udp = false;
    timestamp_trackers[slot].pts_discontinuity_count = 0;
    timestamp_trackers[slot].in_use = true;
    
    pthread_mutex_unlock(&timestamp_mutex);
    
    log_info("Initialized timestamp tracker for stream %s in slot %d", stream_name, slot);
    return 0;
}

/**
 * Reset the timestamp tracker for a stream
 */
int reset_timestamp_tracker(const char *stream_name) {
    if (!stream_name) {
        log_error("Cannot reset timestamp tracker: NULL stream name");
        return -1;
    }
    
    // Initialize trackers if not already done
    if (!trackers_initialized) {
        init_timestamp_trackers();
    }
    
    pthread_mutex_lock(&timestamp_mutex);
    
    // Find tracker
    int found = 0;
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].in_use && 
            strcmp(timestamp_trackers[i].stream_name, stream_name) == 0) {
            // Reset tracker
            timestamp_trackers[i].last_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i].last_dts = AV_NOPTS_VALUE;
            timestamp_trackers[i].expected_next_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i].timestamps_initialized = false;
            timestamp_trackers[i].pts_discontinuity_count = 0;
            // Don't change the is_udp flag
            
            found = 1;
            log_info("Reset timestamp tracker for stream %s", stream_name);
            break;
        }
    }
    
    pthread_mutex_unlock(&timestamp_mutex);
    
    // If not found, create a new tracker
    if (!found) {
        log_info("Timestamp tracker not found for stream %s, creating new one", stream_name);
        return init_timestamp_tracker(stream_name);
    }
    
    return 0;
}

/**
 * Remove the timestamp tracker for a stream
 */
int remove_timestamp_tracker(const char *stream_name) {
    if (!stream_name) {
        log_error("Cannot remove timestamp tracker: NULL stream name");
        return -1;
    }
    
    if (!trackers_initialized) {
        // Nothing to remove if trackers aren't initialized
        return 0;
    }
    
    pthread_mutex_lock(&timestamp_mutex);
    
    // Find tracker
    int found = 0;
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].in_use && 
            strcmp(timestamp_trackers[i].stream_name, stream_name) == 0) {
            // Clear tracker
            memset(&timestamp_trackers[i], 0, sizeof(timestamp_tracker_t));
            
            found = 1;
            log_info("Removed timestamp tracker for stream %s", stream_name);
            break;
        }
    }
    
    pthread_mutex_unlock(&timestamp_mutex);
    
    if (!found) {
        log_debug("Timestamp tracker not found for stream %s, nothing to remove", stream_name);
    }
    
    return 0;
}

/**
 * Set the UDP flag for a stream's timestamp tracker
 */
int set_timestamp_tracker_udp_flag(const char *stream_name, bool is_udp) {
    if (!stream_name) {
        log_error("Cannot set UDP flag: NULL stream name");
        return -1;
    }
    
    // Initialize trackers if not already done
    if (!trackers_initialized) {
        init_timestamp_trackers();
    }
    
    pthread_mutex_lock(&timestamp_mutex);
    
    // Find tracker
    int found = 0;
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].in_use && 
            strcmp(timestamp_trackers[i].stream_name, stream_name) == 0) {
            // Set UDP flag
            timestamp_trackers[i].is_udp = is_udp;
            
            found = 1;
            log_info("Set UDP flag to %s for stream %s timestamp tracker", 
                    is_udp ? "true" : "false", stream_name);
            break;
        }
    }
    
    pthread_mutex_unlock(&timestamp_mutex);
    
    // If not found, create a new tracker
    if (!found) {
        log_info("Timestamp tracker not found for stream %s, creating new one with UDP flag %s", 
                stream_name, is_udp ? "true" : "false");
        
        // Create a new tracker
        pthread_mutex_lock(&timestamp_mutex);
        
        // Find empty slot
        int slot = -1;
        for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
            if (!timestamp_trackers[i].in_use) {
                slot = i;
                break;
            }
        }
        
        if (slot == -1) {
            pthread_mutex_unlock(&timestamp_mutex);
            log_error("No slot available for new timestamp tracker");
            return -1;
        }
        
        // Initialize tracker
        memset(&timestamp_trackers[slot], 0, sizeof(timestamp_tracker_t));
        strncpy(timestamp_trackers[slot].stream_name, stream_name, MAX_STREAM_NAME_LENGTH - 1);
        timestamp_trackers[slot].stream_name[MAX_STREAM_NAME_LENGTH - 1] = '\0';
        timestamp_trackers[slot].last_pts = AV_NOPTS_VALUE;
        timestamp_trackers[slot].last_dts = AV_NOPTS_VALUE;
        timestamp_trackers[slot].expected_next_pts = AV_NOPTS_VALUE;
        timestamp_trackers[slot].timestamps_initialized = false;
        timestamp_trackers[slot].is_udp = is_udp;
        timestamp_trackers[slot].pts_discontinuity_count = 0;
        timestamp_trackers[slot].in_use = true;
        
        pthread_mutex_unlock(&timestamp_mutex);
        
        log_info("Created new timestamp tracker for stream %s with UDP flag %s in slot %d", 
                stream_name, is_udp ? "true" : "false", slot);
    }
    
    return 0;
}

/**
 * Get timestamp tracker for a stream
 * This function is used internally by the packet processor
 * 
 * @param stream_name Name of the stream
 * @return Pointer to timestamp tracker, or NULL if not found
 */
timestamp_tracker_t *get_timestamp_tracker_internal(const char *stream_name) {
    if (!stream_name || !trackers_initialized) {
        return NULL;
    }
    
    pthread_mutex_lock(&timestamp_mutex);
    
    timestamp_tracker_t *tracker = NULL;
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].in_use && 
            strcmp(timestamp_trackers[i].stream_name, stream_name) == 0) {
            tracker = &timestamp_trackers[i];
            break;
        }
    }
    
    pthread_mutex_unlock(&timestamp_mutex);
    
    return tracker;
}

/**
 * Get timestamp tracker for a stream
 * This function is provided for backward compatibility with code that expects
 * the old get_timestamp_tracker function
 * 
 * @param stream_name Name of the stream
 * @return Pointer to timestamp tracker, or NULL if not found
 */
void *get_timestamp_tracker(const char *stream_name) {
    // Initialize trackers if not already done
    if (!trackers_initialized) {
        init_timestamp_trackers();
    }
    
    // Try to get existing tracker
    timestamp_tracker_t *tracker = get_timestamp_tracker_internal(stream_name);
    
    // If not found, create a new one
    if (!tracker) {
        // Create a new tracker
        init_timestamp_tracker(stream_name);
        
        // Try again
        tracker = get_timestamp_tracker_internal(stream_name);
    }
    
    return (void *)tracker;
}
