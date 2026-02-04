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
#include "video/api_detection.h"
#include "video/onvif_detection.h"
#include "video/onvif_detection_integration.h"
#include "video/onvif_motion_recording.h"
#include "video/motion_storage_manager.h"
#include "utils/memory.h"

// Include our new modules
#include "video/detection_config.h"
#include "video/detection_embedded.h"
#include "video/detection_integration.h"
#include "video/detection_frame_processing.h"
#include "video/unified_detection_thread.h"

// Track which streams are currently being processed for detection
char *active_detection_streams = NULL; // Accessible from detection_frame_processing.c
int active_detections = 0; // Accessible from detection_frame_processing.c
static int max_detections = 0;

// Flag to indicate if we're using the new stream-based detection system
bool use_stream_based_detection = true;

/**
 * Initialize the detection integration system
 */
int init_detection_integration(void) {
    // Initialize configuration
    if (init_detection_config() != 0) {
        log_error("Failed to initialize detection configuration");
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

    // Initialize the unified detection system
    if (init_unified_detection_system() != 0) {
        log_error("Failed to initialize unified detection system");
        // This is a critical error
        free(active_detection_streams);
        active_detection_streams = NULL;
        return -1;
    } else {
        log_info("Unified detection system initialized");
    }
    
    // Initialize the API detection system
    if (init_api_detection_system() != 0) {
        log_error("Failed to initialize API detection system");
        // This is not a critical error, as we can still use other detection methods
        log_warn("API detection will not be available");
    } else {
        log_info("API detection system initialized");
    }
    
    // Initialize the ONVIF detection system
    if (init_onvif_detection_integration() != 0) {
        log_error("Failed to initialize ONVIF detection system");
        // This is not a critical error, as we can still use other detection methods
        log_warn("ONVIF detection will not be available");
    } else {
        log_info("ONVIF detection system initialized");
        // ONVIF detection is now integrated with the existing detection thread system
    }

    // Initialize the ONVIF motion recording system
    if (init_onvif_motion_recording() != 0) {
        log_error("Failed to initialize ONVIF motion recording system");
        // This is not a critical error
        log_warn("ONVIF motion recording will not be available");
    } else {
        log_info("ONVIF motion recording system initialized");
    }

    // Initialize the motion storage manager
    if (init_motion_storage_manager() != 0) {
        log_error("Failed to initialize motion storage manager");
        // This is not a critical error
        log_warn("Motion storage cleanup will not be available");
    } else {
        log_info("Motion storage manager initialized");
    }

    log_info("Detection integration initialized with %d max concurrent detections", max_detections);
    return 0;
}

/**
 * Force cleanup of all SOD models in the global cache
 * This is needed to prevent memory leaks when the program exits
 */
void force_cleanup_sod_models(void) {
    log_info("Forcing cleanup of all SOD models in global cache...");

    // Call the function to clean up the global model cache
    // This function is defined in detection_model.c
    extern void force_cleanup_model_cache(void);
    force_cleanup_model_cache();

    log_info("SOD model cleanup completed");
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

    // Force cleanup of all SOD models to prevent memory leaks
    force_cleanup_sod_models();

    // Ensure all detection models are unloaded
    // Note: shutdown_detection_system() also shuts down the unified detection system
    shutdown_detection_system();

    // Ensure motion detection is cleaned up
    shutdown_motion_detection_system();
    
    // Shutdown the API detection system
    shutdown_api_detection_system();
    log_info("API detection system has shutdown");
    
    // Shutdown the ONVIF detection system
    cleanup_onvif_detection_integration();
    log_info("ONVIF detection system has shutdown");

    // Shutdown the ONVIF motion recording system
    cleanup_onvif_motion_recording();
    log_info("ONVIF motion recording system has shutdown");

    // Shutdown the motion storage manager
    shutdown_motion_storage_manager();
    log_info("Motion storage manager has shutdown");
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
