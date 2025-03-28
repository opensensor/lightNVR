#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <fcntl.h>
#include <errno.h>
#include <time.h>
#include <dirent.h>
#include <stdbool.h>

#include "core/logger.h"
#include "core/config.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/detection.h"
#include "video/detection_result.h"
#include "video/motion_detection.h"
#include "video/motion_detection_wrapper.h"
#include "video/sod_integration.h"
#include "utils/memory.h"

// Include our new modules
#include "video/detection_config.h"
#include "video/detection_buffer.h"
#include "video/detection_embedded.h"
#include "video/detection_integration.h"
#include "video/detection_frame_processing.h"

// Track which streams are currently being processed for detection
char *active_detection_streams = NULL; // Accessible from detection_frame_processing.c
int active_detections = 0; // Accessible from detection_frame_processing.c
static int max_detections = 0;

/**
 * Initialize the detection integration system
 */
int init_detection_integration(void) {
    // Initialize configuration
    if (init_detection_config() != 0) {
        log_error("Failed to initialize detection configuration");
        return -1;
    }
    
    // Initialize buffer pool
    if (init_buffer_pool() != 0) {
        log_error("Failed to initialize buffer pool");
        return -1;
    }
    
    // Get configuration
    detection_config_t *config = get_detection_config();
    if (!config) {
        log_error("Failed to get detection configuration");
        return -1;
    }
    
    // Allocate active detection streams array
    max_detections = config->concurrent_detections;
    active_detection_streams = (char *)calloc(max_detections, MAX_STREAM_NAME);
    if (!active_detection_streams) {
        log_error("Failed to allocate active detection streams array");
        return -1;
    }
    
    log_info("Detection integration initialized with %d max concurrent detections", max_detections);
    return 0;
}

/**
 * Cleanup detection resources when shutting down
 */
void cleanup_detection_resources(void) {
    log_info("Starting detection resources cleanup...");
    
    // Free active detection streams array
    if (active_detection_streams) {
        free(active_detection_streams);
        active_detection_streams = NULL;
        active_detections = 0;
    }
    
    // Cleanup buffer pool
    cleanup_buffer_pool();
    
    // Ensure all detection models are unloaded
    shutdown_detection_system();
    
    // Ensure motion detection is cleaned up
    shutdown_motion_detection_system();
    
    // Explicitly call memory cleanup for any remaining buffers
    emergency_buffer_pool_cleanup();
    
    log_info("Detection resources cleaned up successfully");
}

/**
 * Get the number of active detections
 */
int get_active_detection_count(void) {
    return active_detections;
}

/**
 * Get the maximum number of concurrent detections
 */
int get_max_detection_count(void) {
    return max_detections;
}

/**
 * Check if a detection is already in progress for a specific stream
 */
bool is_detection_in_progress(const char *stream_name) {
    if (!stream_name || !active_detection_streams) {
        return false;
    }
    
    // Check if this stream is in the active list
    for (int i = 0; i < max_detections; i++) {
        char *active_stream = active_detection_streams + i * MAX_STREAM_NAME;
        if (active_stream[0] != '\0' && strcmp(active_stream, stream_name) == 0) {
            return true;
        }
    }
    
    return false;
}
