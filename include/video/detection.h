#ifndef DETECTION_H
#define DETECTION_H

#include <stdbool.h>
#include "video/detection_result.h"

// Opaque handle for detection models
typedef void* detection_model_t;

/**
 * Initialize the detection system
 * 
 * @return 0 on success, non-zero on failure
 */
int init_detection_system(void);

/**
 * Shutdown the detection system
 */
void shutdown_detection_system(void);

/**
 * Check if a model file is supported
 * 
 * @param model_path Path to the model file
 * @return true if supported, false otherwise
 */
bool is_model_supported(const char *model_path);

/**
 * Get the type of a model file
 * 
 * @param model_path Path to the model file
 * @return String describing the model type (e.g., "sod", "tflite")
 */
const char* get_model_type(const char *model_path);

/**
 * Load a detection model
 * 
 * @param model_path Path to the model file
 * @param threshold Detection confidence threshold (0.0-1.0)
 * @return Model handle or NULL on failure
 */
detection_model_t load_detection_model(const char *model_path, float threshold);

/**
 * Unload a detection model
 * 
 * @param model Detection model handle
 */
void unload_detection_model(detection_model_t model);

/**
 * Run detection on a frame
 * 
 * @param model Detection model handle
 * @param frame_data Frame data (format depends on model)
 * @param width Frame width
 * @param height Frame height
 * @param channels Number of color channels (1 for grayscale, 3 for RGB)
 * @param result Pointer to detection result structure to fill
 * @return 0 on success, non-zero on failure
 */
int detect_objects(detection_model_t model, const unsigned char *frame_data, 
                  int width, int height, int channels, detection_result_t *result);

#endif /* DETECTION_H */
