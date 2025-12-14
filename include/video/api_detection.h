#ifndef LIGHTNVR_API_DETECTION_H
#define LIGHTNVR_API_DETECTION_H

#include "video/detection_result.h"

// Model type for API-based detection
#define MODEL_TYPE_API "api"

/**
 * Initialize the API detection system
 * 
 * @return 0 on success, non-zero on failure
 */
int init_api_detection_system(void);

/**
 * Shutdown the API detection system
 */
void shutdown_api_detection_system(void);

/**
 * Detect objects using the API
 *
 * @param api_url The URL of the detection API
 * @param frame_data The frame data to detect objects in
 * @param width The width of the frame
 * @param height The height of the frame
 * @param channels The number of channels in the frame
 * @param result Pointer to a detection_result_t structure to store the results
 * @param stream_name The name of the stream (for database storage)
 * @param threshold Confidence threshold for detection (0.0-1.0, use negative for default)
 * @return 0 on success, non-zero on failure
 */
int detect_objects_api(const char *api_url, const unsigned char *frame_data,
                      int width, int height, int channels, detection_result_t *result,
                      const char *stream_name, float threshold);

#endif /* LIGHTNVR_API_DETECTION_H */
