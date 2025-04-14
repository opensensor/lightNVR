#ifndef DETECTION_INTEGRATION_H
#define DETECTION_INTEGRATION_H

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <pthread.h>
#include <stdbool.h>

// Flag to indicate if we're using the new stream-based detection system
extern bool use_stream_based_detection;

/**
 * Initialize the detection integration system
 * This should be called at startup
 *
 * @return 0 on success, non-zero on failure
 */
int init_detection_integration(void);


/**
 * Force cleanup of all SOD models in the global cache
 * This is needed to prevent memory leaks when the program exits
 */
void force_cleanup_sod_models(void);

/**
 * Cleanup detection resources when shutting down
 * This should be called when the application is exiting
 */
void cleanup_detection_resources(void);

/**
 * Cleanup ONVIF detection integration resources
 * This should be called when the application is exiting
 */
void cleanup_onvif_detection_integration(void);

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

/**
 * Check if a detection is already in progress for a specific stream
 *
 * @param stream_name The name of the stream to check
 * @return true if detection is in progress, false otherwise
 */
bool is_detection_in_progress(const char *stream_name);

#endif /* DETECTION_INTEGRATION_H */
