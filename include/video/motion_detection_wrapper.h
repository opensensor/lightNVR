#ifndef MOTION_DETECTION_WRAPPER_H
#define MOTION_DETECTION_WRAPPER_H

#include <stdbool.h>

/**
 * Initialize motion detection systems
 * This function should be called at system startup
 * 
 * @return 0 on success, non-zero on failure
 */
int init_motion_detection_systems(void);

/**
 * Shutdown motion detection systems
 * This function should be called at system shutdown
 */
void shutdown_motion_detection_systems(void);

/**
 * Enable motion detection for a stream
 * 
 * @param stream_name The name of the stream
 * @param sensitivity Sensitivity level (0.0-1.0, higher is more sensitive)
 * @param min_motion_area Minimum area of motion to trigger detection (0.0-1.0, fraction of frame)
 * @param cooldown_time Cooldown time between detections in seconds
 * @return 0 on success, non-zero on failure
 */
int enable_motion_detection(const char *stream_name, float sensitivity, float min_motion_area, int cooldown_time);

/**
 * Disable motion detection for a stream
 * 
 * @param stream_name The name of the stream
 * @return 0 on success, non-zero on failure
 */
int disable_motion_detection(const char *stream_name);

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
                                     int cooldown_time, int downscale_factor);

/**
 * Disable optimized motion detection for a stream
 * 
 * @param stream_name The name of the stream
 * @return 0 on success, non-zero on failure
 */
int disable_optimized_motion_detection(const char *stream_name);

#endif /* MOTION_DETECTION_WRAPPER_H */
