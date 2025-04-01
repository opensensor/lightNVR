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
#include <signal.h>
#include <setjmp.h>

// Global variables for alarm handling
static jmp_buf alarm_jmp_buf;
static volatile sig_atomic_t alarm_triggered = 0;

// Signal handler for SIGALRM
static void alarm_handler(int sig) {
    alarm_triggered = 1;
    longjmp(alarm_jmp_buf, 1);
}

#include "core/logger.h"
#include "core/config.h"
#include "core/shutdown_coordinator.h"
#include "utils/memory.h"  // For get_total_memory_allocated
#include "video/detection_model.h"
#include "video/sod_detection.h"
#include "video/sod_realnet.h"
#include "sod/sod.h"  // For sod_cnn_destroy

// Static variable to track if we're in shutdown mode
static bool in_shutdown_mode = false;

// Define maximum model size for embedded devices (in MB)
#define MAX_MODEL_SIZE_MB 50
#define MAX_LARGE_MODELS 32  // Maximum number of large models to load simultaneously (one per stream)
                            // Increased from 16 to 32 to handle more concurrent streams

// Track large models that are currently loaded
static int large_models_loaded = 0;
static pthread_mutex_t large_models_mutex = PTHREAD_MUTEX_INITIALIZER;

// Global model cache to share across all detection functions
static struct {
    char path[MAX_PATH_LENGTH];
    detection_model_t model;
    time_t last_used;
    bool is_large_model;
} global_model_cache[MAX_STREAMS] = {{{0}}};
static pthread_mutex_t global_model_cache_mutex = PTHREAD_MUTEX_INITIALIZER;

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

    // During shutdown, we'll just clear the cache entries without trying to unload models
    // This prevents hanging during shutdown due to model destruction
    pthread_mutex_lock(&global_model_cache_mutex);
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (global_model_cache[i].path[0] != '\0' && global_model_cache[i].model) {
            log_info("Clearing model from global cache during shutdown (skipping unload): %s", global_model_cache[i].path);
            
            // Just mark the model as unloaded in our cache without calling unload_detection_model
            // This avoids the potentially hanging sod_cnn_destroy call
            global_model_cache[i].path[0] = '\0';
            global_model_cache[i].model = NULL;
            global_model_cache[i].is_large_model = false;
        }
    }
    pthread_mutex_unlock(&global_model_cache_mutex);

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
 * Internal function to clean up old models in the global cache
 * @param max_age Maximum age in seconds for a model to be considered active
 */
static void cleanup_old_models_internal(time_t max_age) {
    time_t current_time = time(NULL);
    
    pthread_mutex_lock(&global_model_cache_mutex);
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (global_model_cache[i].path[0] != '\0') {
            // Check if model hasn't been used for a while
            if (current_time - global_model_cache[i].last_used > max_age) {
                log_info("Cleaning up unused model from global cache: %s (unused for %ld seconds)",
                         global_model_cache[i].path, current_time - global_model_cache[i].last_used);
                
                // Unload the model
                unload_detection_model(global_model_cache[i].model);
                
                // Clear the cache entry
                global_model_cache[i].path[0] = '\0';
                global_model_cache[i].model = NULL;
                global_model_cache[i].is_large_model = false;
            }
        }
    }
    pthread_mutex_unlock(&global_model_cache_mutex);
}

/**
 * Clean up old models in the global cache
 */
void cleanup_old_detection_models(time_t max_age) {
    // MEMORY LEAK FIX: Add more aggressive cleanup for models
    // Use a shorter max_age if the provided one is too long
    time_t effective_max_age = max_age;
    if (effective_max_age > 300) { // 5 minutes max
        log_info("Reducing model cache max age from %ld to 300 seconds to prevent memory leaks", max_age);
        effective_max_age = 300;
    }
    
    // Call the internal cleanup function with the effective max age
    cleanup_old_models_internal(effective_max_age);
    
    // Log memory usage after cleanup
    log_info("Model cache cleanup completed. Current memory usage: %zu bytes", get_total_memory_allocated());
}

/**
 * Load a detection model
 */
detection_model_t load_detection_model(const char *model_path, float threshold) {
    if (!model_path) {
        log_error("Invalid model path");
        return NULL;
    }

    // First check if the model is already loaded in the global cache
    pthread_mutex_lock(&global_model_cache_mutex);
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (global_model_cache[i].path[0] != '\0' && 
            strcmp(global_model_cache[i].path, model_path) == 0) {
            detection_model_t cached_model = global_model_cache[i].model;
            global_model_cache[i].last_used = time(NULL);
            log_info("Using globally cached model: %s", model_path);
            pthread_mutex_unlock(&global_model_cache_mutex);
            return cached_model;
        }
    }
    pthread_mutex_unlock(&global_model_cache_mutex);

    // If not in cache, check if file exists and get its size
    struct stat st;
    if (stat(model_path, &st) != 0) {
        log_error("MODEL FILE NOT FOUND: %s", model_path);
        return NULL;
    }
    
    log_info("MODEL FILE EXISTS: %s", model_path);
    log_info("MODEL FILE SIZE: %ld bytes", (long)st.st_size);
    
    // Check if this is a large model
    double model_size_mb = (double)st.st_size / (1024 * 1024);
    bool is_large_model = model_size_mb > MAX_MODEL_SIZE_MB;
    
    if (is_large_model) {
        log_warn("Large model detected: %.1f MB (limit: %d MB)", model_size_mb, MAX_MODEL_SIZE_MB);
        
        // Check if we can load another large model
        pthread_mutex_lock(&large_models_mutex);
        if (large_models_loaded >= MAX_LARGE_MODELS) {
            log_error("Cannot load another large model, already at limit (%d)", MAX_LARGE_MODELS);
            pthread_mutex_unlock(&large_models_mutex);
            return NULL;
        }
        large_models_loaded++;
        pthread_mutex_unlock(&large_models_mutex);
    }

    // Get model type
    const char *model_type = get_model_type(model_path);
    log_info("MODEL TYPE: %s", model_type);

    // Load appropriate model type
    detection_model_t model = NULL;
    
    if (strcmp(model_type, MODEL_TYPE_SOD_REALNET) == 0) {
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
    
    // If loading failed and this was a large model, decrement the counter
    if (!model && is_large_model) {
        pthread_mutex_lock(&large_models_mutex);
        if (large_models_loaded > 0) {  // Ensure we don't decrement below zero
            large_models_loaded--;
        }
        pthread_mutex_unlock(&large_models_mutex);
    }
    
    // MEMORY LEAK FIX: Be more selective about adding models to the global cache
    // If loading succeeded, add to global cache only if it's not a large model
    if (model) {
        if (!is_large_model) {
            pthread_mutex_lock(&global_model_cache_mutex);
            // Find an empty slot or the oldest used model in the cache
            int oldest_idx = -1;
            time_t oldest_time = time(NULL);
            
            for (int i = 0; i < MAX_STREAMS; i++) {
                if (global_model_cache[i].path[0] == '\0') {
                    oldest_idx = i;
                    break;
                } else if (global_model_cache[i].last_used < oldest_time) {
                    oldest_time = global_model_cache[i].last_used;
                    oldest_idx = i;
                }
            }
            
            // If we found a slot, add the model to the cache
            if (oldest_idx >= 0) {
                // If slot was used, unload the old model first to prevent memory leaks
                if (global_model_cache[oldest_idx].path[0] != '\0') {
                    // Check if this model is still in use by any other cache
                    bool still_in_use = false;
                    for (int j = 0; j < MAX_STREAMS; j++) {
                        if (j != oldest_idx && 
                            global_model_cache[j].path[0] != '\0' && 
                            global_model_cache[j].model == global_model_cache[oldest_idx].model) {
                            still_in_use = true;
                            break;
                        }
                    }
                    
                    // Only unload if not still in use
                    if (!still_in_use) {
                        log_info("Unloading model from global cache: %s", global_model_cache[oldest_idx].path);
                        unload_detection_model(global_model_cache[oldest_idx].model);
                        
                        // If it was a large model, decrement the counter
                        if (global_model_cache[oldest_idx].is_large_model) {
                            pthread_mutex_lock(&large_models_mutex);
                            if (large_models_loaded > 0) {
                                large_models_loaded--;
                            }
                            pthread_mutex_unlock(&large_models_mutex);
                        }
                    } else {
                        log_info("Model %s is still in use by another cache entry, not unloading", 
                                 global_model_cache[oldest_idx].path);
                    }
                }
                
                // Add the new model to the cache
                strncpy(global_model_cache[oldest_idx].path, model_path, MAX_PATH_LENGTH - 1);
                global_model_cache[oldest_idx].path[MAX_PATH_LENGTH - 1] = '\0';  // Ensure null termination
                global_model_cache[oldest_idx].model = model;
                global_model_cache[oldest_idx].last_used = time(NULL);
                global_model_cache[oldest_idx].is_large_model = is_large_model;
                log_info("Added model to global cache: %s", model_path);
            }
            pthread_mutex_unlock(&global_model_cache_mutex);
        } else {
            log_info("Not adding large model to global cache to prevent memory leaks: %s", model_path);
        }
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
    
    // Check if this model is in the global cache
    bool in_global_cache = false;
    char model_path[MAX_PATH_LENGTH] = {0};
    bool is_large_model = false;
    
    pthread_mutex_lock(&global_model_cache_mutex);
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (global_model_cache[i].model == model) {
            in_global_cache = true;
            strncpy(model_path, global_model_cache[i].path, MAX_PATH_LENGTH - 1);
            is_large_model = global_model_cache[i].is_large_model;
            
            // Remove from global cache
            global_model_cache[i].path[0] = '\0';
            global_model_cache[i].model = NULL;
            global_model_cache[i].is_large_model = false;
            log_info("Removed model from global cache: %s", model_path);
            break;
        }
    }
    pthread_mutex_unlock(&global_model_cache_mutex);
    
    // If not found in global cache, assume it's a large model for safety
    if (!in_global_cache) {
        is_large_model = true;
    }

    // Simple model cleanup without complex signal handling
    if (strcmp(m->type, MODEL_TYPE_SOD) == 0) {
        // Free SOD model - skip actual destruction during shutdown to avoid hangs
        void *sod_model = m->sod;
        if (sod_model) {
            if (is_shutdown_initiated() || in_shutdown_mode) {
                // During shutdown, skip the actual model destruction as it can hang
                log_info("Skipping SOD model destruction during shutdown to avoid potential hangs");
            } else {
                // During normal operation, try to destroy the model
                log_info("Destroying SOD model");
                sod_cnn_destroy(sod_model);
            }
        }
    } else if (strcmp(m->type, MODEL_TYPE_SOD_REALNET) == 0) {
        // Free SOD RealNet model - also skip during shutdown
        if (is_shutdown_initiated() || in_shutdown_mode) {
            log_info("Skipping SOD RealNet model destruction during shutdown to avoid potential hangs");
        } else {
            free_sod_realnet_model(m->sod_realnet);
        }
    } else if (strcmp(m->type, MODEL_TYPE_TFLITE) == 0) {
        // Unload TFLite model - also skip during shutdown
        if (is_shutdown_initiated() || in_shutdown_mode) {
            log_info("Skipping TFLite model destruction during shutdown to avoid potential hangs");
        } else {
            m->tflite.free_model(m->tflite.model);
            dlclose(m->tflite.handle);
        }
    }

    // If this was a large model, decrement the counter
    if (is_large_model) {
        pthread_mutex_lock(&large_models_mutex);
        if (large_models_loaded > 0) {
            large_models_loaded--;
            log_info("Unloaded large model, %d large models still loaded", large_models_loaded);
        }
        pthread_mutex_unlock(&large_models_mutex);
    }

    // Always free the model structure itself
    free(m);
}
