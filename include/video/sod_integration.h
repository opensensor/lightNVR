#ifndef SOD_INTEGRATION_H
#define SOD_INTEGRATION_H

#include <stdbool.h>
#include <stddef.h>
#include "../video/detection_result.h"
#include "../video/detection_model.h"

/**
 * Detect model type based on file name
 *
 * @param model_path Path to the model file
 * @return String describing the model type (MODEL_TYPE_SOD_REALNET, MODEL_TYPE_SOD, etc.)
 */
const char* detect_model_type(const char *model_path);

/**
 * Check if a file exists
 *
 * @param filename Path to the file
 * @return 1 if file exists, 0 otherwise
 */
int file_exists(const char *filename);

/**
 * Load a SOD model for detection
 *
 * @param model_path Path to the model file
 * @param threshold Detection confidence threshold
 * @param full_model_path Buffer to store the full model path (if found)
 * @param max_path_length Maximum length of the full_model_path buffer
 * @return Model handle or NULL on failure
 */
void* load_sod_model_for_detection(const char *model_path, float threshold,
                                  char *full_model_path, size_t max_path_length);

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
int detect_with_sod(void *model, const unsigned char *frame_data,
                   int width, int height, int channels, detection_result_t *result);

/**
 * Ensure proper cleanup of SOD models to prevent memory leaks
 * This function should be called when a detection thread is stopping
 * or when the application is shutting down
 *
 * @param model The detection model to clean up
 */
void ensure_sod_model_cleanup(detection_model_t model);

/**
 * Force cleanup of all SOD models to prevent memory leaks
 * This function should be called during application shutdown
 */
void force_sod_models_cleanup(void);

#endif /* SOD_INTEGRATION_H */
