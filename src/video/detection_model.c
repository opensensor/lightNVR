#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <dlfcn.h>
#include <pthread.h>
#include <unistd.h>
#include <sys/stat.h>
#include <time.h>
#include "core/logger.h"
#include "core/config.h"
#include "core/shutdown_coordinator.h"
#include "utils/memory.h"  // For get_total_memory_allocated
#include "utils/strings.h"
#include "video/detection_model.h"
#include "video/sod_detection.h"
#include "video/sod_realnet.h"
#include "video/api_detection.h"
#include "video/onvif_detection.h"
#include "sod/sod.h"  // For sod_cnn_destroy

// Static variable to track if we're in shutdown mode
static bool in_shutdown_mode = false;

// Define maximum model size for embedded devices (in MB)
#define MAX_MODEL_SIZE_MB 50

// Global variables
static bool initialized = false;

// TFLite model structure
typedef struct {
    void *handle;                // Dynamic library handle
    void *model;                 // TFLite model handle
    float threshold;             // Detection threshold
    void *(*load_model)(const char *);  // Function pointer for loading model
    void (*free_model)(void *);  // Function pointer for freeing model
    void *(*detect)(void *, const unsigned char *, int, int, int, int *, float); // Function pointer for detection
} tflite_model_t;

// Generic model structure
typedef struct {
    char type[16];               // Model type (sod, sod_realnet, tflite)
    union {
        void *sod;               // SOD model handle
        void *sod_realnet;       // SOD RealNet model handle
        tflite_model_t tflite;   // TFLite model handle
    };
    float threshold;             // Detection threshold
    char path[MAX_PATH_LENGTH];  // Path to the model file (for reference)
} model_t;

/**
 * Initialize the model system
 */
int init_detection_model_system(void) {
    if (initialized) {
        return 0;  // Already initialized
    }

    // Initialize SOD detection system
    int sod_ret = init_sod_detection_system();
    if (sod_ret != 0) {
        log_error("Failed to initialize SOD detection system");
    }

    // Check for TFLite library
    void *tflite_handle = dlopen("libtensorflowlite.so", RTLD_LAZY);
    if (tflite_handle) {
        log_info("TensorFlow Lite library found and loaded");
        dlclose(tflite_handle);
    } else {
        log_warn("TensorFlow Lite library not found: %s", dlerror());
    }

    initialized = true;
    log_info("Detection model system initialized");
    return 0;
}

/**
 * Shutdown the model system
 */
void shutdown_detection_model_system(void) {
    if (!initialized) {
        return;
    }

    // Set the in_shutdown_mode flag to true
    in_shutdown_mode = true;

    // Log that we're shutting down
    log_info("Detection model system shutting down");

    // Shutdown SOD detection system
    shutdown_sod_detection_system();

    initialized = false;
    log_info("Detection model system shutdown");
}

/**
 * Check if a model file is supported
 */
bool is_model_supported(const char *model_path) {
    if (!model_path) {
        return false;
    }

    // Check file extension
    const char *ext = strrchr(model_path, '.');
    if (!ext) {
        return false;
    }

    // Check for SOD RealNet models
    if (strstr(model_path, ".realnet.sod") != NULL) {
        return is_sod_available();
    }

    // Check for regular SOD models
    if (strcasecmp(ext, ".sod") == 0) {
        return is_sod_available();
    }

    // Check for TFLite models
    if (strcasecmp(ext, ".tflite") == 0) {
        void *handle = dlopen("libtensorflowlite.so", RTLD_LAZY);
        if (handle) {
            dlclose(handle);
            return true;
        }
        return false;
    }

    return false;
}

/**
 * Get the type of a model file
 */
const char* get_model_type(const char *model_path) {
    if (!model_path) {
        return "unknown";
    }
    if (ends_with(model_path, "api-detection")) {
        return MODEL_TYPE_API;
    }
    // Also recognize HTTP/HTTPS URLs as API detection (e.g., http://localhost:9001/api/v1/detect)
    if (strncmp(model_path, "http://", 7) == 0 || strncmp(model_path, "https://", 8) == 0) {
        return MODEL_TYPE_API;
    }
    if (ends_with(model_path, "onvif")) {
        return MODEL_TYPE_ONVIF;
    }

    // Check file extension
    const char *ext = strrchr(model_path, '.');
    if (!ext) {
        return "unknown";
    }

    // Check for SOD RealNet models
    if (strstr(model_path, ".realnet.sod") != NULL) {
        return is_sod_available() ? MODEL_TYPE_SOD_REALNET : "unknown";
    }

    // Check for regular SOD models
    if (strcasecmp(ext, ".sod") == 0) {
        return is_sod_available() ? MODEL_TYPE_SOD : "unknown";
    }

    // Check for TFLite models
    if (strcasecmp(ext, ".tflite") == 0) {
        return MODEL_TYPE_TFLITE;
    }

    return "unknown";
}

/**
 * Load a TFLite model
 */
static detection_model_t load_tflite_model(const char *model_path, float threshold) {
    // Open TFLite library
    void *handle = dlopen("libtensorflowlite.so", RTLD_LAZY);
    if (!handle) {
        log_error("Failed to load TensorFlow Lite library: %s", dlerror());
        return NULL;
    }

    // Clear any existing error
    dlerror();

    // Load TFLite functions
    void *(*tflite_load_model)(const char *) = dlsym(handle, "tflite_load_model");
    const char *dlsym_error = dlerror();
    if (dlsym_error) {
        log_error("Failed to load TFLite function 'tflite_load_model': %s", dlsym_error);
        dlclose(handle);
        return NULL;
    }

    void (*tflite_free_model)(void *) = dlsym(handle, "tflite_free_model");
    dlsym_error = dlerror();
    if (dlsym_error) {
        log_error("Failed to load TFLite function 'tflite_free_model': %s", dlsym_error);
        dlclose(handle);
        return NULL;
    }

    void *(*tflite_detect)(void *, const unsigned char *, int, int, int, int *, float) =
        dlsym(handle, "tflite_detect");
    dlsym_error = dlerror();
    if (dlsym_error) {
        log_error("Failed to load TFLite function 'tflite_detect': %s", dlsym_error);
        dlclose(handle);
        return NULL;
    }

    // Load the model
    void *tflite_model = tflite_load_model(model_path);
    if (!tflite_model) {
        log_error("Failed to load TFLite model: %s", model_path);
        dlclose(handle);
        return NULL;
    }

    // Create model structure
    model_t *model = (model_t *)malloc(sizeof(model_t));
    if (!model) {
        log_error("Failed to allocate memory for model structure");
        tflite_free_model(tflite_model);
        dlclose(handle);
        return NULL;
    }

    // Initialize model structure
    strncpy(model->type, MODEL_TYPE_TFLITE, sizeof(model->type) - 1);
    model->tflite.handle = handle;
    model->tflite.model = tflite_model;
    model->tflite.threshold = threshold;
    model->tflite.load_model = tflite_load_model;
    model->tflite.free_model = tflite_free_model;
    model->tflite.detect = tflite_detect;
    strncpy(model->path, model_path, MAX_PATH_LENGTH - 1);
    model->path[MAX_PATH_LENGTH - 1] = '\0';  // Ensure null termination

    log_info("TFLite model loaded: %s", model_path);
    return model;
}

/**
 * Get the path of a loaded model
 */
const char* get_model_path(detection_model_t model) {
    if (!model) {
        return NULL;
    }

    // Return the path directly from the model structure
    model_t *m = (model_t *)model;
    return m->path;
}

/**
 * Get the RealNet model handle from a detection model
 */
void* get_realnet_model_handle(detection_model_t model) {
    if (!model) {
        return NULL;
    }

    model_t *m = (model_t *)model;

    if (strcmp(m->type, MODEL_TYPE_SOD_REALNET) == 0) {
        return m->sod_realnet;
    }

    return NULL;
}

/**
 * Get the type of a loaded model
 */
const char* get_model_type_from_handle(detection_model_t model) {
    if (!model) {
        return "unknown";
    }

    model_t *m = (model_t *)model;
    return m->type;
}

/**
 * Clean up old models in the global cache
 *
 * This function is kept for API compatibility but does nothing in the thread-local model approach
 */
void cleanup_old_detection_models(time_t max_age) {
    // This function is now a no-op since we're using thread-local models
    // Each thread is responsible for managing its own model

    // Log memory usage for monitoring
    log_info("Thread-local model approach: No global cache to clean. Current memory usage: %zu bytes", get_total_memory_allocated());
}

/**
 * Load a detection model
 *
 * Simplified implementation that directly loads the model without caching
 * Each thread will manage its own model instance
 */
detection_model_t load_detection_model(const char *model_path, float threshold) {
    if (!model_path) {
        log_error("Invalid model path");
        return NULL;
    }

    // Check if this is an API URL (starts with http:// or https://) or the special "api-detection" string
    bool is_api_detection = ends_with(model_path, "api-detection") ||
                            strncmp(model_path, "http://", 7) == 0 ||
                            strncmp(model_path, "https://", 8) == 0;
    bool is_onvif_detection = ends_with(model_path, "onvif");

    // Only check file existence if it's not an API URL or ONVIF
    if (is_api_detection) {
        log_info("API DETECTION: Using API for detection instead of a local model file");
    } else if (is_onvif_detection) {
        log_info("ONVIF DETECTION: Using ONVIF for detection instead of a local model file");
    } else {
        // Check if file exists and get its size
        struct stat st;
        if (stat(model_path, &st) != 0) {
            log_error("MODEL FILE NOT FOUND: %s", model_path);
            return NULL;
        }

        log_info("MODEL FILE EXISTS: %s", model_path);
        log_info("MODEL FILE SIZE: %ld bytes", (long)st.st_size);

        // Check if this is a large model (just for logging)
        double model_size_mb = (double)st.st_size / (1024 * 1024);
        if (model_size_mb > MAX_MODEL_SIZE_MB) {
            log_warn("Large model detected: %.1f MB (limit: %d MB)", model_size_mb, MAX_MODEL_SIZE_MB);
        }
    }

    // Get model type
    const char *model_type = get_model_type(model_path);
    log_info("MODEL TYPE: %s", model_type);

    // Load appropriate model type
    detection_model_t model = NULL;

    if (strcmp(model_type, MODEL_TYPE_API) == 0) {
        // For API models, we just need to store the URL
        model_t *m = (model_t *)malloc(sizeof(model_t));
        if (m) {
            strncpy(m->type, MODEL_TYPE_API, sizeof(m->type) - 1);
            m->sod = NULL; // We don't need a model handle for API
            m->threshold = threshold;
            strncpy(m->path, model_path, MAX_PATH_LENGTH - 1);
            m->path[MAX_PATH_LENGTH - 1] = '\0';  // Ensure null termination
            model = m;

            // Initialize the API detection system
            init_api_detection_system();
        }
    }
    else if (strcmp(model_type, MODEL_TYPE_ONVIF) == 0) {
        // For ONVIF models, we just need to store the URL
        model_t *m = (model_t *)malloc(sizeof(model_t));
        if (m) {
            strncpy(m->type, MODEL_TYPE_ONVIF, sizeof(m->type) - 1);
            m->sod = NULL; // We don't need a model handle for ONVIF
            m->threshold = threshold;
            strncpy(m->path, model_path, MAX_PATH_LENGTH - 1);
            m->path[MAX_PATH_LENGTH - 1] = '\0';  // Ensure null termination
            model = m;

            // Initialize the ONVIF detection system
            init_onvif_detection_system();
            log_info("ONVIF model created: %s", model_path);
        }
    }
    else if (strcmp(model_type, MODEL_TYPE_SOD_REALNET) == 0) {
        void *realnet_model = load_sod_realnet_model(model_path, threshold);
        if (realnet_model) {
            // Create model structure
            model_t *m = (model_t *)malloc(sizeof(model_t));
            if (m) {
                strncpy(m->type, MODEL_TYPE_SOD_REALNET, sizeof(m->type) - 1);
                m->sod_realnet = realnet_model;
                m->threshold = threshold;
                strncpy(m->path, model_path, MAX_PATH_LENGTH - 1);
                m->path[MAX_PATH_LENGTH - 1] = '\0';  // Ensure null termination
                model = m;
            } else {
                free_sod_realnet_model(realnet_model);
            }
        }
    } else if (strcmp(model_type, MODEL_TYPE_SOD) == 0) {
        model = load_sod_model(model_path, threshold);
    } else if (strcmp(model_type, MODEL_TYPE_TFLITE) == 0) {
        model = load_tflite_model(model_path, threshold);
    } else {
        log_error("Unsupported model type: %s", model_type);
    }

    if (model) {
        log_info("Successfully loaded model: %s", model_path);
    } else {
        log_error("Failed to load model: %s", model_path);
    }

    return model;
}

/**
 * Unload a detection model
 */
void unload_detection_model(detection_model_t model) {
    if (!model) {
        return;
    }

    model_t *m = (model_t *)model;
    char model_path[MAX_PATH_LENGTH] = {0};

    // Save the path for logging
    if (m->path[0] != '\0') {
        strncpy(model_path, m->path, MAX_PATH_LENGTH - 1);
    } else {
        strcpy(model_path, "unknown");
    }

    // Enhanced model cleanup with better memory management
    if (strcmp(m->type, MODEL_TYPE_API) == 0) {
        // For API models, we need to ensure the API detection system is properly cleaned up
        // when the last API model is unloaded
        log_info("Unloading API model: %s", model_path);

        // We don't need to call shutdown_api_detection_system() here because
        // it will be called during program shutdown, and we want to keep the
        // API detection system initialized for other API models that might be in use

        // Just set the model pointer to NULL to prevent double-free
        m->sod = NULL;
    }
    else if (strcmp(m->type, MODEL_TYPE_ONVIF) == 0) {
        // For ONVIF models, we need to ensure the ONVIF detection system is properly cleaned up
        // when the last ONVIF model is unloaded
        log_info("Unloading ONVIF model: %s", model_path);

        // We don't need to call shutdown_onvif_detection_system() here because
        // it will be called during program shutdown, and we want to keep the
        // ONVIF detection system initialized for other ONVIF models that might be in use

        // Just set the model pointer to NULL to prevent double-free
        m->sod = NULL;
    }
    else if (strcmp(m->type, MODEL_TYPE_SOD) == 0) {
        // Use our new safer cleanup function for SOD models
        log_info("Using cleanup_sod_model for safer SOD model cleanup");

        // We need to remove the model from the cache first to prevent double-free
        // So we'll create a copy of the model structure
        model_t *model_copy = malloc(sizeof(model_t));
        if (model_copy) {
            // Copy the model structure
            memcpy(model_copy, m, sizeof(model_t));

            // Set the original model's SOD pointer to NULL to prevent double-free
            m->sod = NULL;

            // Free the original model structure
            free(m);

            // Call our safer cleanup function on the copy
            cleanup_sod_model(model_copy);

            // Return early since we've already freed the model
            return;
        } else {
            // If we couldn't allocate memory for the copy, fall back to the old method
            log_warn("Failed to allocate memory for model copy, falling back to direct cleanup");
            void *sod_model = m->sod;
            if (sod_model) {
                log_info("Destroying SOD model");
                sod_cnn_destroy(sod_model);
                // Set the model pointer to NULL to prevent double-free
                m->sod = NULL;
            }
        }
    } else if (strcmp(m->type, MODEL_TYPE_SOD_REALNET) == 0) {
        // Free SOD RealNet model - also try during shutdown
        if (m->sod_realnet) {
            if (is_shutdown_initiated() || in_shutdown_mode) {
                log_info("MEMORY LEAK FIX: Destroying SOD RealNet model during shutdown");
                free_sod_realnet_model(m->sod_realnet);
            } else {
                free_sod_realnet_model(m->sod_realnet);
            }
            m->sod_realnet = NULL;
        }
    } else if (strcmp(m->type, MODEL_TYPE_TFLITE) == 0) {
        // Unload TFLite model - also try during shutdown
        if (m->tflite.model && m->tflite.free_model) {
            if (is_shutdown_initiated() || in_shutdown_mode) {
                log_info("MEMORY LEAK FIX: Destroying TFLite model during shutdown");
                m->tflite.free_model(m->tflite.model);
                if (m->tflite.handle) {
                    dlclose(m->tflite.handle);
                }
            } else {
                m->tflite.free_model(m->tflite.model);
                if (m->tflite.handle) {
                    dlclose(m->tflite.handle);
                }
            }
            m->tflite.model = NULL;
            m->tflite.handle = NULL;
        }
    }

    // Log that we're unloading the model
    log_info("Unloading model: %s", model_path);

    // Always free the model structure itself
    free(m);
}

/**
 * Force cleanup of all models in the global cache
 *
 * This function is kept for API compatibility but does nothing in the thread-local model approach
 */
void force_cleanup_model_cache(void) {
    // This function is now a no-op since we're using thread-local models
    // Each thread is responsible for managing its own model

    // Set the shutdown mode flag to true for any remaining cleanup operations
    in_shutdown_mode = true;

    log_info("Thread-local model approach: No global cache to clean up");
}
