#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <fcntl.h>
#include <errno.h>
#include <time.h>
#include <dirent.h>


#include "../../include/core/logger.h"
#include "../../include/core/config.h"
#include "../../include/video/detection.h"
#include "../../include/video/detection_result.h"
#include "../../include/video/sod_integration.h"
#include "../../include/video/detection_model.h"

// Define model types
#define MODEL_TYPE_SOD "sod"
#define MODEL_TYPE_SOD_REALNET "sod_realnet"
#define MODEL_TYPE_TFLITE "tflite"

/**
 * Check if a file exists
 */
int file_exists(const char *filename) {
    FILE *file = fopen(filename, "r");
    if (file) {
        fclose(file);
        return 1;
    }
    return 0;
}

/**
 * Detect model type based on file name
 */
const char* detect_model_type(const char *model_path) {
    if (!model_path) {
        return "unknown";
    }

    // Check for SOD RealNet models
    if (strstr(model_path, ".realnet.sod") != NULL) {
        return MODEL_TYPE_SOD_REALNET;
    }

    // Check for regular SOD models
    const char *ext = strrchr(model_path, '.');
    if (ext && strcasecmp(ext, ".sod") == 0) {
        return MODEL_TYPE_SOD;
    }

    // Check for TFLite models
    if (ext && strcasecmp(ext, ".tflite") == 0) {
        return MODEL_TYPE_TFLITE;
    }

    return "unknown";
}

/**
 * Load a SOD model for detection
 */
void* load_sod_model_for_detection(const char *model_path, float threshold,
                                  char *full_model_path, size_t max_path_length) {
    if (!model_path || !full_model_path) {
        log_error("Invalid parameters for load_sod_model_for_detection");
        return NULL;
    }

    extern config_t g_config;

    // Check if model_path is a relative path
    if (model_path[0] != '/') {
        // Construct full path using configured models path from INI if it exists
        if (g_config.models_path && strlen(g_config.models_path) > 0) {
            snprintf(full_model_path, max_path_length, "%s/%s", g_config.models_path, model_path);
        } else {
            // Fall back to default path if INI config doesn't exist
            snprintf(full_model_path, max_path_length, "/etc/lightnvr/models/%s", model_path);
        }

        // Validate path exists
        if (!file_exists(full_model_path)) {
            log_error("Model file does not exist: %s", full_model_path);

            // Try alternative locations
            char alt_path[MAX_PATH_LENGTH];

            // Get current working directory
            char cwd[MAX_PATH_LENGTH];
            if (getcwd(cwd, sizeof(cwd)) != NULL) {
                const char *locations[] = {
                    "/var/lib/lightnvr/models/", // Default system location
                };

                for (int i = 0; i < sizeof(locations)/sizeof(locations[0]); i++) {
                    snprintf(alt_path, MAX_PATH_LENGTH, "%s%s",
                             locations[i],
                             model_path);
                    if (file_exists(alt_path)) {
                        log_info("Found model at alternative location: %s", alt_path);
                        strncpy(full_model_path, alt_path, max_path_length - 1);
                        break;
                    }
                }
            }
        }
    } else {
        // Already an absolute path
        strncpy(full_model_path, model_path, max_path_length - 1);
    }

    log_info("Using model path: %s", full_model_path);

    // Get the appropriate threshold for the model type
    const char *model_type = detect_model_type(full_model_path);
    if (threshold <= 0.0f) {
        if (strcmp(model_type, MODEL_TYPE_SOD_REALNET) == 0) {
            threshold = 5.0f; // RealNet models typically use 5.0
            log_info("Using default threshold of 5.0 for RealNet model");
        } else {
            threshold = 0.3f; // CNN models typically use 0.3
            log_info("Using default threshold of 0.3 for CNN model");
        }
    } else {
        log_info("Using configured threshold of %.2f for model", threshold);
    }

    // Load the model using the detection system
    return load_detection_model(full_model_path, threshold);
}

/**
 * Run detection on a frame using SOD
 */
int detect_with_sod(void *model, const unsigned char *frame_data,
                   int width, int height, int channels, detection_result_t *result) {
    if (!model || !frame_data || !result) {
        log_error("Invalid parameters for detect_with_sod");
        return -1;
    }

    // Use the unified detection function
    return detect_objects(model, frame_data, width, height, channels, result);
}

/**
 * Ensure proper cleanup of SOD models to prevent memory leaks
 * This function should be called when a detection thread is stopping
 * or when the application is shutting down
 *
 * @param model The detection model to clean up
 */
void ensure_sod_model_cleanup(detection_model_t model) {
    if (!model) {
        log_warn("Attempted to clean up NULL SOD model");
        return;
    }

    // Get the model type
    const char *model_type = get_model_type_from_handle(model);

    // Only proceed if this is a SOD model
    if (strcmp(model_type, MODEL_TYPE_SOD) == 0) {
        log_info("Ensuring proper cleanup of SOD model to prevent memory leaks");

        // Use our new safer cleanup function
        cleanup_sod_model(model);
    } else {
        // For non-SOD models, use the standard unload function
        log_info("Using standard unload for non-SOD model type: %s", model_type);
        unload_detection_model(model);
    }
}

/**
 * Force cleanup of all SOD models to prevent memory leaks
 * This function should be called during application shutdown
 */
void force_sod_models_cleanup(void) {
    log_info("Forcing cleanup of all SOD models to prevent memory leaks");

    // Call the global model cache cleanup function
    force_cleanup_model_cache();
}
