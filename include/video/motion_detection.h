#ifndef MOTION_DETECTION_H
#define MOTION_DETECTION_H

#include <stdbool.h>
#include <stdint.h>
#include <time.h>
#include "../video/detection_result.h"

/**
 * Initialize the motion detection system
 * 
 * @return 0 on success, non-zero on failure
 */
int init_motion_detection_system(void);

/**
 * Shutdown the motion detection system
 */
void shutdown_motion_detection_system(void);

/**
 * Configure motion detection for a stream
 * 
 * @param stream_name The name of the stream
 * @param sensitivity Sensitivity level (0.0-1.0, higher is more sensitive)
 * @param min_motion_area Minimum area of motion to trigger detection (0.0-1.0, fraction of frame)
 * @param cooldown_time Cooldown time between detections in seconds
 * @return 0 on success, non-zero on failure
 */
int configure_motion_detection(const char *stream_name, float sensitivity, 
                              float min_motion_area, int cooldown_time);

/**
 * Process a frame for motion detection
 * 
 * @param stream_name The name of the stream
 * @param frame_data Frame data (grayscale or RGB)
 * @param width Frame width
 * @param height Frame height
 * @param channels Number of color channels (1 for grayscale, 3 for RGB)
 * @param frame_time Timestamp of the frame
 * @param result Pointer to detection result structure to fill
 * @return 0 on success, non-zero on failure
 */
int detect_motion(const char *stream_name, const unsigned char *frame_data,
                 int width, int height, int channels, time_t frame_time,
                 detection_result_t *result);

/**
 * Enable or disable motion detection for a stream
 * 
 * @param stream_name The name of the stream
 * @param enabled Whether motion detection should be enabled
 * @return 0 on success, non-zero on failure
 */
int set_motion_detection_enabled(const char *stream_name, bool enabled);

/**
 * Check if motion detection is enabled for a stream
 * 
 * @param stream_name The name of the stream
 * @return true if enabled, false otherwise
 */
bool is_motion_detection_enabled(const char *stream_name);

#endif /* MOTION_DETECTION_H */
