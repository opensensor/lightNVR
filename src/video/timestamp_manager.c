#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>

#include "core/logger.h"
#include "video/timestamp_manager.h"

// Maximum number of streams that can have timestamp trackers
#define MAX_TIMESTAMP_TRACKERS 32

// Timestamp tracker structure
typedef struct {
    char stream_name[64];
    int64_t last_pts;
    int64_t expected_next_pts;
    bool timestamps_initialized;
    bool is_udp;
    int pts_discontinuity_count;
} timestamp_tracker_t;

// Array of timestamp trackers
static timestamp_tracker_t timestamp_trackers[MAX_TIMESTAMP_TRACKERS];
static pthread_mutex_t timestamp_mutex = PTHREAD_MUTEX_INITIALIZER;

/**
 * Initialize all timestamp trackers
 */
int init_timestamp_trackers(void) {
    pthread_mutex_lock(&timestamp_mutex);
    
    // Clear all trackers
    memset(timestamp_trackers, 0, sizeof(timestamp_trackers));
    
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
    
    pthread_mutex_lock(&timestamp_mutex);
    
    // Check if tracker already exists
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].stream_name[0] != '\0' && 
            strcmp(timestamp_trackers[i].stream_name, stream_name) == 0) {
            // Already exists, just reset it
            timestamp_trackers[i].last_pts = 0;
            timestamp_trackers[i].expected_next_pts = 0;
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
        if (timestamp_trackers[i].stream_name[0] == '\0') {
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
    strncpy(timestamp_trackers[slot].stream_name, stream_name, sizeof(timestamp_trackers[slot].stream_name) - 1);
    timestamp_trackers[slot].stream_name[sizeof(timestamp_trackers[slot].stream_name) - 1] = '\0';
    timestamp_trackers[slot].last_pts = 0;
    timestamp_trackers[slot].expected_next_pts = 0;
    timestamp_trackers[slot].timestamps_initialized = false;
    timestamp_trackers[slot].is_udp = false;
    timestamp_trackers[slot].pts_discontinuity_count = 0;
    
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
    
    pthread_mutex_lock(&timestamp_mutex);
    
    // Find tracker
    int found = 0;
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].stream_name[0] != '\0' && 
            strcmp(timestamp_trackers[i].stream_name, stream_name) == 0) {
            // Reset tracker
            timestamp_trackers[i].last_pts = 0;
            timestamp_trackers[i].expected_next_pts = 0;
            timestamp_trackers[i].timestamps_initialized = false;
            timestamp_trackers[i].pts_discontinuity_count = 0;
            // Don't change the is_udp flag
            
            found = 1;
            break;
        }
    }
    
    // If not found, create a new tracker
    if (!found) {
        // Find empty slot
        int slot = -1;
        for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
            if (timestamp_trackers[i].stream_name[0] == '\0') {
                slot = i;
                break;
            }
        }
        
        if (slot != -1) {
            // Initialize tracker
            strncpy(timestamp_trackers[slot].stream_name, stream_name, sizeof(timestamp_trackers[slot].stream_name) - 1);
            timestamp_trackers[slot].stream_name[sizeof(timestamp_trackers[slot].stream_name) - 1] = '\0';
            timestamp_trackers[slot].last_pts = 0;
            timestamp_trackers[slot].expected_next_pts = 0;
            timestamp_trackers[slot].timestamps_initialized = false;
            timestamp_trackers[slot].is_udp = false;
            timestamp_trackers[slot].pts_discontinuity_count = 0;
            
            found = 1;
            log_info("Created new timestamp tracker for stream %s during reset", stream_name);
        } else {
            log_warn("No slot available for new timestamp tracker during reset");
        }
    }
    
    pthread_mutex_unlock(&timestamp_mutex);
    
    if (!found) {
        log_warn("Timestamp tracker not found for stream %s and no slots available", stream_name);
        return -1;
    }
    
    log_info("Reset timestamp tracker for stream %s", stream_name);
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
    
    pthread_mutex_lock(&timestamp_mutex);
    
    // Find tracker
    int found = 0;
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].stream_name[0] != '\0' && 
            strcmp(timestamp_trackers[i].stream_name, stream_name) == 0) {
            // Clear tracker
            memset(&timestamp_trackers[i], 0, sizeof(timestamp_tracker_t));
            
            found = 1;
            break;
        }
    }
    
    pthread_mutex_unlock(&timestamp_mutex);
    
    if (!found) {
        log_warn("Timestamp tracker not found for stream %s, nothing to remove", stream_name);
        // Return success even if not found, since the end result is the same
        // (no timestamp tracker for this stream)
        return 0;
    }
    
    log_info("Removed timestamp tracker for stream %s", stream_name);
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
    
    pthread_mutex_lock(&timestamp_mutex);
    
    // Find tracker
    int found = 0;
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].stream_name[0] != '\0' && 
            strcmp(timestamp_trackers[i].stream_name, stream_name) == 0) {
            // Set UDP flag
            timestamp_trackers[i].is_udp = is_udp;
            
            found = 1;
            break;
        }
    }
    
    // If not found, create a new tracker
    if (!found) {
        // Find empty slot
        int slot = -1;
        for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
            if (timestamp_trackers[i].stream_name[0] == '\0') {
                slot = i;
                break;
            }
        }
        
        if (slot != -1) {
            // Initialize tracker
            strncpy(timestamp_trackers[slot].stream_name, stream_name, sizeof(timestamp_trackers[slot].stream_name) - 1);
            timestamp_trackers[slot].stream_name[sizeof(timestamp_trackers[slot].stream_name) - 1] = '\0';
            timestamp_trackers[slot].last_pts = 0;
            timestamp_trackers[slot].expected_next_pts = 0;
            timestamp_trackers[slot].timestamps_initialized = false;
            timestamp_trackers[slot].is_udp = is_udp;
            timestamp_trackers[slot].pts_discontinuity_count = 0;
            
            found = 1;
            log_info("Created new timestamp tracker for stream %s with UDP flag %s", 
                    stream_name, is_udp ? "true" : "false");
        }
    }
    
    pthread_mutex_unlock(&timestamp_mutex);
    
    if (!found) {
        log_error("No slot available for new timestamp tracker");
        return -1;
    }
    
    log_info("Set UDP flag to %s for stream %s timestamp tracker", 
            is_udp ? "true" : "false", stream_name);
    return 0;
}
