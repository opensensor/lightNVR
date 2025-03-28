#ifndef DETECTION_RECORDING_H
#define DETECTION_RECORDING_H

#include <stdbool.h>
#include <time.h>
#include "video/detection_result.h"

/**
 * Initialize detection-based recording system
 */
void init_detection_recording_system(void);

/**
 * Shutdown detection-based recording system
 */
void shutdown_detection_recording_system(void);

/**
 * Start detection-based recording for a stream
 * 
 * @param stream_name The name of the stream
 * @param model_path Path to the detection model file
 * @param threshold Detection confidence threshold (0.0-1.0)
 * @param pre_buffer Seconds to keep before detection
 * @param post_buffer Seconds to keep after detection
 * @return 0 on success, -1 on error
 */
int start_detection_recording(const char *stream_name, const char *model_path, float threshold,
                             int pre_buffer, int post_buffer);

/**
 * Stop detection-based recording for a stream
 * 
 * @param stream_name The name of the stream
 * @return 0 on success, -1 on error
 */
int stop_detection_recording(const char *stream_name);

/**
 * Process detection results for recording
 * This function manages recording decisions based on detection results
 *
 * @param stream_name The name of the stream
 * @param frame_data The frame data (for debugging only)
 * @param width Frame width
 * @param height Frame height
 * @param channels Number of color channels
 * @param frame_time Timestamp of the frame
 * @param result Detection results from detect_objects call
 * @return 0 on success, -1 on error
 */
int process_frame_for_recording(const char *stream_name, const unsigned char *frame_data,
                               int width, int height, int channels, time_t frame_time,
                               detection_result_t *result);

/**
 * Get detection recording state for a stream
 * Returns 1 if detection recording is active, 0 if not, -1 on error
 * 
 * @param stream_name The name of the stream
 * @param recording_active Pointer to bool to store recording state
 * @return 1 if detection recording is enabled, 0 if not, -1 on error
 */
int get_detection_recording_state(const char *stream_name, bool *recording_active);

/**
 * Monitor HLS segments for a stream and submit them to the detection thread pool
 * This function is called periodically to check for new HLS segments
 * 
 * @param stream_name The name of the stream
 * @return 0 on success, -1 on error
 */
int monitor_hls_segments_for_detection(const char *stream_name);

/**
 * Start monitoring HLS segments for all streams with detection enabled
 * This function should be called periodically to check for new segments
 */
void monitor_all_hls_segments_for_detection(void);

#endif /* DETECTION_RECORDING_H */
