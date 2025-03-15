#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <dlfcn.h>
#include <pthread.h>
#include <unistd.h>

#include "video/detection.h"
#include "core/logger.h"

// Define model types
#define MODEL_TYPE_SOD "sod"
#define MODEL_TYPE_TFLITE "tflite"

// SOD model structure
typedef struct {
    void *handle;                // Dynamic library handle
    void *model;                 // SOD model handle
    float threshold;             // Detection threshold
    void *(*load_model)(const char *);  // Function pointer for loading model
    void (*free_model)(void *);  // Function pointer for freeing model
    void *(*detect)(void *, const unsigned char *, int, int, int, int *, float); // Function pointer for detection
} sod_model_t;

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
    char type[16];               // Model type (sod, tflite)
    union {
        sod_model_t sod;
        tflite_model_t tflite;
    };
} model_t;

// Global variables
static pthread_mutex_t detection_mutex = PTHREAD_MUTEX_INITIALIZER;
static bool initialized = false;

/**
 * Initialize the detection system
 */
int init_detection_system(void) {
    if (initialized) {
        return 0;  // Already initialized
    }
    
    pthread_mutex_lock(&detection_mutex);
    
    // Check for SOD library
    void *sod_handle = dlopen("libsod.so", RTLD_LAZY);
    if (sod_handle) {
        log_info("SOD library found and loaded");
        dlclose(sod_handle);
    } else {
        log_warn("SOD library not found: %s", dlerror());
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
    pthread_mutex_unlock(&detection_mutex);
    
    log_info("Detection system initialized");
    return 0;
}

/**
 * Shutdown the detection system
 */
void shutdown_detection_system(void) {
    if (!initialized) {
        return;
    }
    
    pthread_mutex_lock(&detection_mutex);
    initialized = false;
    pthread_mutex_unlock(&detection_mutex);
    
    log_info("Detection system shutdown");
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
    
    // Check for SOD models
    if (strcasecmp(ext, ".sod") == 0) {
        void *handle = dlopen("libsod.so", RTLD_LAZY);
        if (handle) {
            dlclose(handle);
            return true;
        }
        return false;
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
    
    // Check for SOD models
    if (strcasecmp(ext, ".sod") == 0) {
        return MODEL_TYPE_SOD;
    }
    
    // Check for TFLite models
    if (strcasecmp(ext, ".tflite") == 0) {
        return MODEL_TYPE_TFLITE;
    }
    
    return "unknown";
}

/**
 * Load a SOD model
 */
static detection_model_t load_sod_model(const char *model_path, float threshold) {
    // Open SOD library
    void *handle = dlopen("libsod.so", RTLD_LAZY);
    if (!handle) {
        log_error("Failed to load SOD library: %s", dlerror());
        return NULL;
    }
    
    // Clear any existing error
    dlerror();
    
    // Load SOD functions
    void *(*sod_load_model)(const char *) = dlsym(handle, "sod_load_model");
    const char *dlsym_error = dlerror();
    if (dlsym_error) {
        log_error("Failed to load SOD function 'sod_load_model': %s", dlsym_error);
        dlclose(handle);
        return NULL;
    }
    
    void (*sod_free_model)(void *) = dlsym(handle, "sod_free_model");
    dlsym_error = dlerror();
    if (dlsym_error) {
        log_error("Failed to load SOD function 'sod_free_model': %s", dlsym_error);
        dlclose(handle);
        return NULL;
    }
    
    void *(*sod_detect)(void *, const unsigned char *, int, int, int, int *, float) = 
        dlsym(handle, "sod_detect");
    dlsym_error = dlerror();
    if (dlsym_error) {
        log_error("Failed to load SOD function 'sod_detect': %s", dlsym_error);
        dlclose(handle);
        return NULL;
    }
    
    // Load the model
    void *sod_model = sod_load_model(model_path);
    if (!sod_model) {
        log_error("Failed to load SOD model: %s", model_path);
        dlclose(handle);
        return NULL;
    }
    
    // Create model structure
    model_t *model = (model_t *)malloc(sizeof(model_t));
    if (!model) {
        log_error("Failed to allocate memory for model structure");
        sod_free_model(sod_model);
        dlclose(handle);
        return NULL;
    }
    
    // Initialize model structure
    strncpy(model->type, MODEL_TYPE_SOD, sizeof(model->type) - 1);
    model->sod.handle = handle;
    model->sod.model = sod_model;
    model->sod.threshold = threshold;
    model->sod.load_model = sod_load_model;
    model->sod.free_model = sod_free_model;
    model->sod.detect = sod_detect;
    
    log_info("SOD model loaded: %s", model_path);
    return model;
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
    
    log_info("TFLite model loaded: %s", model_path);
    return model;
}

/**
 * Load a detection model
 */
detection_model_t load_detection_model(const char *model_path, float threshold) {
    if (!model_path) {
        log_error("Invalid model path");
        return NULL;
    }
    
    // Check file extension
    const char *ext = strrchr(model_path, '.');
    if (!ext) {
        log_error("Invalid model file: %s", model_path);
        return NULL;
    }
    
    // Load appropriate model type
    if (strcasecmp(ext, ".sod") == 0) {
        return load_sod_model(model_path, threshold);
    } else if (strcasecmp(ext, ".tflite") == 0) {
        return load_tflite_model(model_path, threshold);
    } else {
        log_error("Unsupported model type: %s", ext);
        return NULL;
    }
}

/**
 * Unload a detection model
 */
void unload_detection_model(detection_model_t model) {
    if (!model) {
        return;
    }
    
    model_t *m = (model_t *)model;
    
    if (strcmp(m->type, MODEL_TYPE_SOD) == 0) {
        // Unload SOD model
        m->sod.free_model(m->sod.model);
        dlclose(m->sod.handle);
    } else if (strcmp(m->type, MODEL_TYPE_TFLITE) == 0) {
        // Unload TFLite model
        m->tflite.free_model(m->tflite.model);
        dlclose(m->tflite.handle);
    }
    
    free(m);
}

/**
 * Run detection on a frame
 */
int detect_objects(detection_model_t model, const unsigned char *frame_data, 
                  int width, int height, int channels, detection_result_t *result) {
    if (!model || !frame_data || !result) {
        return -1;
    }
    
    model_t *m = (model_t *)model;
    
    // Initialize result
    result->count = 0;
    
    // Run detection based on model type
    if (strcmp(m->type, MODEL_TYPE_SOD) == 0) {
        // Run SOD detection
        int count = 0;
        void *detections = m->sod.detect(m->sod.model, frame_data, width, height, channels, 
                                        &count, m->sod.threshold);
        
        // Process detections
        // This is a simplified example - actual implementation would depend on SOD API
        // and would extract label, confidence, and bounding box for each detection
        
        // For now, just set a dummy detection
        if (count > 0 && count <= MAX_DETECTIONS) {
            result->count = count;
            for (int i = 0; i < count; i++) {
                strncpy(result->detections[i].label, "object", MAX_LABEL_LENGTH - 1);
                result->detections[i].confidence = 0.9f;
                result->detections[i].x = 0.5f;
                result->detections[i].y = 0.5f;
                result->detections[i].width = 0.2f;
                result->detections[i].height = 0.2f;
            }
        }
        
        return 0;
    } else if (strcmp(m->type, MODEL_TYPE_TFLITE) == 0) {
        // Run TFLite detection
        int count = 0;
        void *detections = m->tflite.detect(m->tflite.model, frame_data, width, height, channels, 
                                           &count, m->tflite.threshold);
        
        // Process detections
        // This is a simplified example - actual implementation would depend on TFLite API
        // and would extract label, confidence, and bounding box for each detection
        
        // For now, just set a dummy detection
        if (count > 0 && count <= MAX_DETECTIONS) {
            result->count = count;
            for (int i = 0; i < count; i++) {
                strncpy(result->detections[i].label, "object", MAX_LABEL_LENGTH - 1);
                result->detections[i].confidence = 0.9f;
                result->detections[i].x = 0.5f;
                result->detections[i].y = 0.5f;
                result->detections[i].width = 0.2f;
                result->detections[i].height = 0.2f;
            }
        }
        
        return 0;
    }
    
    return -1;
}
