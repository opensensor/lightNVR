#ifndef DETECTION_INTEGRATION_H
#define DETECTION_INTEGRATION_H

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <pthread.h>
#include <stdbool.h>

/**
 * Initialize the detection integration system
 * This should be called at startup
 * 
 * @return 0 on success, non-zero on failure
 */
int init_detection_integration(void);

/**
 * Process a decoded frame for detection
 * This function should be called from the HLS streaming code with already decoded frames
 * 
 * @param stream_name The name of the stream
 * @param frame The decoded AVFrame
 * @param detection_interval How often to process frames (e.g., every 10 frames)
 * @return 0 on success, -1 on error
 */
int process_decoded_frame_for_detection(const char *stream_name, AVFrame *frame, int detection_interval);

/**
 * Cleanup detection resources when shutting down
 * This should be called when the application is exiting
 */
void cleanup_detection_resources(void);

/**
 * Get the number of active detections
 * 
 * @return Number of active detections
 */
int get_active_detection_count(void);

/**
 * Get the maximum number of concurrent detections
 * 
 * @return Maximum number of concurrent detections
 */
int get_max_detection_count(void);

#endif /* DETECTION_INTEGRATION_H */
