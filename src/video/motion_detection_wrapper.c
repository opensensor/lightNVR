#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#include "core/logger.h"
#include "video/streams.h"
#include "video/motion_detection.h"

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
    
    // Configure motion detection
    int ret = configure_motion_detection(stream_name, sensitivity, min_motion_area, cooldown_time);
    if (ret != 0) {
        log_error("Failed to configure motion detection for stream %s", stream_name);
        return ret;
    }
    
    // Enable motion detection
    ret = set_motion_detection_enabled(stream_name, true);
    if (ret != 0) {
        log_error("Failed to enable motion detection for stream %s", stream_name);
        return ret;
    }
    
    // Start detection-based recording with "motion" as the model path
    // This special value will be recognized by the detection system to use motion detection
    ret = start_detection_recording(stream_name, "motion", sensitivity, 5, 10);
    if (ret != 0) {
        log_error("Failed to start motion-based recording for stream %s", stream_name);
        set_motion_detection_enabled(stream_name, false);
        return ret;
    }
    
    log_info("Motion detection enabled for stream %s with sensitivity=%.2f, min_area=%.2f, cooldown=%d",
             stream_name, sensitivity, min_motion_area, cooldown_time);
    
    return 0;
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
    
    // Disable motion detection
    int ret = set_motion_detection_enabled(stream_name, false);
    if (ret != 0) {
        log_error("Failed to disable motion detection for stream %s", stream_name);
        return ret;
    }
    
    // Stop detection-based recording
    ret = stop_detection_recording(stream_name);
    if (ret != 0) {
        log_error("Failed to stop motion-based recording for stream %s", stream_name);
        return ret;
    }
    
    log_info("Motion detection disabled for stream %s", stream_name);
    
    return 0;
}
