#ifndef SOD_DETECTION_H
#define SOD_DETECTION_H

#include <stdbool.h>
#include "video/detection_result.h"
#include "video/detection_model.h"

/**
 * Initialize the SOD detection system
 *
 * @return 0 on success, non-zero on failure
 */
int init_sod_detection_system(void);

/**
 * Shutdown the SOD detection system
 */
void shutdown_sod_detection_system(void);

/**
 * Load a SOD model
 *
 * @param model_path Path to the model file
 * @param threshold Detection confidence threshold (0.0-1.0)
 * @return Model handle or NULL on failure
 */
detection_model_t load_sod_model(const char *model_path, float threshold);

/**
 * Run detection on a frame using SOD
 *
 * @param model SOD model handle
 * @param frame_data Frame data
 * @param width Frame width
 * @param height Frame height
 * @param channels Number of color channels
 * @param result Pointer to detection result structure to fill
 * @return 0 on success, non-zero on failure
 */
int detect_with_sod_model(detection_model_t model, const unsigned char *frame_data,
                         int width, int height, int channels, detection_result_t *result);

/**
 * Check if SOD is available
 *
 * @return true if SOD is available, false otherwise
 */
bool is_sod_available(void);

/**
 * Safely clean up a SOD model
 * This function ensures proper cleanup of SOD model resources
 *
 * @param model The SOD model to clean up
 */
void cleanup_sod_model(detection_model_t model);

#endif /* SOD_DETECTION_H */
