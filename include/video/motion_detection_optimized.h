#ifndef MOTION_DETECTION_OPTIMIZED_H
#define MOTION_DETECTION_OPTIMIZED_H

#include <stdbool.h>
#include <stdint.h>
#include <time.h>
#include "../video/detection_result.h"

/**
 * Initialize the motion detection system with optimizations for embedded devices
 * 
 * @return 0 on success, non-zero on failure
 */
int init_motion_detection_optimized(void);

/**
 * Shutdown the optimized motion detection system
 */
void shutdown_motion_detection_optimized(void);

/**
 * Configure motion detection for a stream with optimizations
 * 
 * @param stream_name The name of the stream
 * @param sensitivity Sensitivity level (0.0-1.0, higher is more sensitive)
 * @param min_motion_area Minimum area of motion to trigger detection (0.0-1.0, fraction of frame)
 * @param cooldown_time Cooldown time between detections in seconds
 * @return 0 on success, non-zero on failure
 */
int configure_motion_detection_optimized(const char *stream_name, float sensitivity, 
                              float min_motion_area, int cooldown_time);

/**
 * Configure advanced motion detection parameters with optimizations
 * 
 * @param stream_name The name of the stream
 * @param blur_radius Blur radius for noise reduction (0-5)
 * @param noise_threshold Threshold for noise filtering (0-50)
 * @param use_grid_detection Whether to use grid-based detection
 * @param grid_size Size of detection grid (2-32)
 * @param history_size Size of frame history buffer (1-10)
 * @return 0 on success, non-zero on failure
 */
int configure_advanced_motion_detection_optimized(const char *stream_name, int blur_radius,
                                       int noise_threshold, bool use_grid_detection,
                                       int grid_size, int history_size);

/**
 * Configure embedded device optimizations for motion detection
 * 
 * @param stream_name The name of the stream
 * @param downscale_enabled Whether to enable frame downscaling for processing
 * @param downscale_factor Factor by which to downscale (2 = half size, 1 = no downscaling)
 * @return 0 on success, non-zero on failure
 */
int configure_motion_detection_optimizations(const char *stream_name, bool downscale_enabled, int downscale_factor);

/**
 * Enable or disable optimized motion detection for a stream
 * 
 * @param stream_name The name of the stream
 * @param enabled Whether motion detection should be enabled
 * @return 0 on success, non-zero on failure
 */
int set_motion_detection_enabled_optimized(const char *stream_name, bool enabled);

/**
 * Check if optimized motion detection is enabled for a stream
 * 
 * @param stream_name The name of the stream
 * @return true if enabled, false otherwise
 */
bool is_motion_detection_enabled_optimized(const char *stream_name);

/**
 * Process a frame for motion detection with optimizations for embedded devices
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
int detect_motion_optimized(const char *stream_name, const unsigned char *frame_data,
                 int width, int height, int channels, time_t frame_time,
                 detection_result_t *result);

/**
 * Get memory usage statistics for motion detection
 * 
 * @param stream_name The name of the stream
 * @param allocated_memory Pointer to store total allocated memory in bytes
 * @param peak_memory Pointer to store peak memory usage in bytes
 * @return 0 on success, non-zero on failure
 */
int get_motion_detection_memory_usage(const char *stream_name, size_t *allocated_memory, size_t *peak_memory);

/**
 * Get CPU usage statistics for motion detection
 * 
 * @param stream_name The name of the stream
 * @param avg_processing_time Pointer to store average frame processing time in milliseconds
 * @param peak_processing_time Pointer to store peak frame processing time in milliseconds
 * @return 0 on success, non-zero on failure
 */
int get_motion_detection_cpu_usage(const char *stream_name, float *avg_processing_time, float *peak_processing_time);

/**
 * Reset performance statistics for motion detection
 * 
 * @param stream_name The name of the stream
 * @return 0 on success, non-zero on failure
 */
int reset_motion_detection_statistics(const char *stream_name);

#endif /* MOTION_DETECTION_OPTIMIZED_H */
