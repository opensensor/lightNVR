#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#include "core/logger.h"
#include "video/streams.h"
#include "video/motion_detection.h"
#include "video/motion_detection_wrapper.h"

// Forward declarations for functions from detection_recording.c
extern int start_detection_recording(const char *stream_name, const char *model_path, float threshold,
                                   int pre_buffer, int post_buffer);
extern int stop_detection_recording(const char *stream_name);

/**
 * Initialize motion detection systems
 * This function should be called at system startup
 * 
 * @return 0 on success, non-zero on failure
 */
int init_motion_detection_systems(void) {
    int ret = init_motion_detection_system();
    if (ret != 0) {
        log_error("Failed to initialize optimized motion detection");
        return ret;
    }
    
    log_info("Motion detection system initialized successfully");
    return 0;
}

/**
 * Shutdown motion detection systems
 * This function should be called at system shutdown
 */
void shutdown_motion_detection_systems(void) {
    shutdown_motion_detection_system();
    log_info("Motion detection system shut down");
}

/**
 * Enable motion detection for a stream
 * 
 * @param stream_name The name of the stream
 * @param sensitivity Sensitivity level (0.0-1.0, higher is more sensitive)
 * @param min_motion_area Minimum area of motion to trigger detection (0.0-1.0, fraction of frame)
 * @param cooldown_time Cooldown time between detections in seconds
 * @return 0 on success, non-zero on failure
 */
int enable_motion_detection(const char *stream_name, float sensitivity, float min_motion_area, int cooldown_time) {
    if (!stream_name) {
        log_error("Invalid stream name for enable_motion_detection");
        return -1;
    }
    
    // Use the optimized implementation instead
    return enable_optimized_motion_detection(stream_name, sensitivity, min_motion_area, cooldown_time, 2);
}

/**
 * Disable motion detection for a stream
 * 
 * @param stream_name The name of the stream
 * @return 0 on success, non-zero on failure
 */
int disable_motion_detection(const char *stream_name) {
    if (!stream_name) {
        log_error("Invalid stream name for disable_motion_detection");
        return -1;
    }
    
    // Use the optimized implementation instead
    return disable_optimized_motion_detection(stream_name);
}

/**
 * Enable optimized motion detection for a stream
 * 
 * @param stream_name The name of the stream
 * @param sensitivity Sensitivity level (0.0-1.0, higher is more sensitive)
 * @param min_motion_area Minimum area of motion to trigger detection (0.0-1.0, fraction of frame)
 * @param cooldown_time Cooldown time between detections in seconds
 * @param downscale_factor Factor by which to downscale (2 = half size, 1 = no downscaling)
 * @return 0 on success, non-zero on failure
 */
int enable_optimized_motion_detection(const char *stream_name, float sensitivity, float min_motion_area, 
                                     int cooldown_time, int downscale_factor) {
    if (!stream_name) {
        log_error("Invalid stream name for enable_optimized_motion_detection");
        return -1;
    }
    
    // Configure optimized motion detection
    int ret = configure_motion_detection(stream_name, sensitivity, min_motion_area, cooldown_time);
    if (ret != 0) {
        log_error("Failed to configure optimized motion detection for stream %s", stream_name);
        return ret;
    }
    
    // Configure optimizations
    ret = configure_motion_detection_optimizations(stream_name, true, downscale_factor);
    if (ret != 0) {
        log_error("Failed to configure optimizations for stream %s", stream_name);
        return ret;
    }
    
    // Enable optimized motion detection
    ret = set_motion_detection_enabled(stream_name, true);
    if (ret != 0) {
        log_error("Failed to enable optimized motion detection for stream %s", stream_name);
        return ret;
    }
    
    // Start detection-based recording with "motion_optimized" as the model path
    ret = start_detection_recording(stream_name, "motion_optimized", sensitivity, 5, 10);
    if (ret != 0) {
        log_error("Failed to start optimized motion-based recording for stream %s", stream_name);
        set_motion_detection_enabled(stream_name, false);
        return ret;
    }
    
    log_info("Optimized motion detection enabled for stream %s with sensitivity=%.2f, min_area=%.2f, cooldown=%d, downscale=%d",
             stream_name, sensitivity, min_motion_area, cooldown_time, downscale_factor);
    
    return 0;
}

/**
 * Disable optimized motion detection for a stream
 * 
 * @param stream_name The name of the stream
 * @return 0 on success, non-zero on failure
 */
int disable_optimized_motion_detection(const char *stream_name) {
    if (!stream_name) {
        log_error("Invalid stream name for disable_optimized_motion_detection");
        return -1;
    }
    
    // Disable optimized motion detection
    int ret = set_motion_detection_enabled(stream_name, false);
    if (ret != 0) {
        log_error("Failed to disable optimized motion detection for stream %s", stream_name);
        return ret;
    }
    
    // Stop detection-based recording
    ret = stop_detection_recording(stream_name);
    if (ret != 0) {
        log_error("Failed to stop optimized motion-based recording for stream %s", stream_name);
        return ret;
    }
    
    log_info("Optimized motion detection disabled for stream %s", stream_name);
    
    return 0;
}
