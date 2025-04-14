#ifndef DETECTION_MODEL_H
#define DETECTION_MODEL_H

#include <stdbool.h>
#include <time.h>
#include "video/detection_result.h"

// Opaque handle for detection models
typedef void* detection_model_t;

// Model type definitions
#define MODEL_TYPE_SOD "sod"
#define MODEL_TYPE_SOD_REALNET "sod_realnet"
#define MODEL_TYPE_TFLITE "tflite"
#define MODEL_TYPE_API "api"
#define MODEL_TYPE_ONVIF "onvif"

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
 * Get the path of a loaded model
 *
 * @param model Detection model handle
 * @return Path to the model file or NULL if not found
 */
const char* get_model_path(detection_model_t model);

/**
 * Get the RealNet model handle from a detection model
 *
 * @param model Detection model handle
 * @return RealNet model handle or NULL if not a RealNet model
 */
void* get_realnet_model_handle(detection_model_t model);

/**
 * Get the type of a loaded model
 *
 * @param model Detection model handle
 * @return String describing the model type (e.g., "sod", "tflite") or "unknown" if not found
 */
const char* get_model_type_from_handle(detection_model_t model);

/**
 * Clean up old models in the global cache
 *
 * This function is kept for API compatibility but does nothing in the thread-local model approach
 * Each thread is responsible for managing its own model
 *
 * @param max_age Maximum age in seconds (ignored in thread-local approach)
 */
void cleanup_old_detection_models(time_t max_age);

/**
 * Initialize the model system
 *
 * @return 0 on success, non-zero on failure
 */
int init_detection_model_system(void);

/**
 * Shutdown the model system
 */
void shutdown_detection_model_system(void);

/**
 * Force cleanup of all models in the global cache
 *
 * This function is kept for API compatibility but does nothing in the thread-local model approach
 * Each thread is responsible for managing its own model
 */
void force_cleanup_model_cache(void);

#endif /* DETECTION_MODEL_H */
