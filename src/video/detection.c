#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <dlfcn.h>
#include <pthread.h>
#include <unistd.h>

#include "video/detection.h"
#include "video/sod_realnet.h"
#include "core/logger.h"
#include "sod/sod.h"

// Define model types
#define MODEL_TYPE_SOD "sod"
#define MODEL_TYPE_SOD_REALNET "sod_realnet"
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

// SOD RealNet model structure (wrapper for external implementation)
typedef struct {
    void *model;                 // SOD RealNet model handle
} sod_realnet_model_t;

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
        sod_model_t sod;
        sod_realnet_model_t sod_realnet;
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
    
    // SOD is now directly linked, no need to check for library
    log_info("SOD library directly linked");
    
    // Check for RealNet support
    log_info("SOD RealNet support available");
    
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
    
    // Check for SOD RealNet models
    if (strstr(model_path, ".realnet.sod") != NULL) {
        return true;
    }
    
    // Check for regular SOD models
    if (strcasecmp(ext, ".sod") == 0) {
        return true;
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
        return MODEL_TYPE_SOD_REALNET;
    }
    
    // Check for regular SOD models
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
    // Load the model using SOD API
    sod_cnn *sod_model = NULL;
    const char *err_msg = NULL;
    
    // Create CNN model
    int rc = sod_cnn_create(&sod_model, "default", model_path, &err_msg);
    if (rc != SOD_OK || !sod_model) {
        log_error("Failed to load SOD model: %s - %s", model_path, err_msg ? err_msg : "Unknown error");
        return NULL;
    }
    
    // Set detection threshold
    sod_cnn_config(sod_model, SOD_CNN_DETECTION_THRESHOLD, threshold);
    
    // Create model structure
    model_t *model = (model_t *)malloc(sizeof(model_t));
    if (!model) {
        log_error("Failed to allocate memory for model structure");
        sod_cnn_destroy(sod_model);
        return NULL;
    }
    
    // Initialize model structure
    strncpy(model->type, MODEL_TYPE_SOD, sizeof(model->type) - 1);
    model->sod.model = sod_model;
    model->sod.threshold = threshold;
    
    log_info("SOD model loaded: %s", model_path);
    return model;
}

/**
 * Load a SOD RealNet model
 */
static detection_model_t load_sod_realnet_model_internal(const char *model_path, float threshold) {
    // Load the model using the external implementation
    void *realnet_model = load_sod_realnet_model(model_path, threshold);
    if (!realnet_model) {
        return NULL;
    }
    
    // Create model structure
    model_t *model = (model_t *)malloc(sizeof(model_t));
    if (!model) {
        log_error("Failed to allocate memory for SOD RealNet model structure");
        free_sod_realnet_model(realnet_model);
        return NULL;
    }
    
    // Initialize model structure
    strncpy(model->type, MODEL_TYPE_SOD_REALNET, sizeof(model->type) - 1);
    model->sod_realnet.model = realnet_model;
    
    log_info("SOD RealNet model loaded: %s", model_path);
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
    
    // Get model type
    const char *model_type = get_model_type(model_path);
    
    // Load appropriate model type
    if (strcmp(model_type, MODEL_TYPE_SOD_REALNET) == 0) {
        return load_sod_realnet_model_internal(model_path, threshold);
    } else if (strcmp(model_type, MODEL_TYPE_SOD) == 0) {
        return load_sod_model(model_path, threshold);
    } else if (strcmp(model_type, MODEL_TYPE_TFLITE) == 0) {
        return load_tflite_model(model_path, threshold);
    } else {
        log_error("Unsupported model type: %s", model_type);
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
        sod_cnn_destroy(m->sod.model);
    } else if (strcmp(m->type, MODEL_TYPE_SOD_REALNET) == 0) {
        // Unload SOD RealNet model
        free_sod_realnet_model(m->sod_realnet.model);
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
        log_error("Invalid parameters for detect_objects");
        return -1;
    }
    
    model_t *m = (model_t *)model;
    
    log_info("Detecting objects using model type: %s", m->type);
    
    // Initialize result
    result->count = 0;
    
    // Run detection based on model type
    if (strcmp(m->type, MODEL_TYPE_SOD) == 0) {
        // Run SOD detection
        sod_box *boxes = NULL;
        int count = 0;
        
        // Create a sod_img from the frame data
        sod_img img = sod_make_image(width, height, channels);
        if (!img.data) {
            log_error("Failed to create image for SOD detection");
            return -1;
        }
        
        // Convert frame data to float and copy to image
        for (int i = 0; i < width * height * channels; i++) {
            img.data[i] = frame_data[i] / 255.0f;
        }
        
        // Prepare image for CNN
        float *prepared_data = sod_cnn_prepare_image(m->sod.model, img);
        
        // Free the image since we have the prepared data
        sod_free_image(img);
        
        if (!prepared_data) {
            log_error("Failed to prepare image for SOD detection");
            return -1;
        }
        
        // Run detection
        int rc = sod_cnn_predict(m->sod.model, prepared_data, &boxes, &count);
        if (rc != SOD_OK) {
            log_error("SOD detection failed: %d", rc);
            return -1;
        }
        
        // Process detections
        int valid_count = 0;
        for (int i = 0; i < count && valid_count < MAX_DETECTIONS; i++) {
            // Apply threshold
            if (boxes[i].score < m->sod.threshold) {
                log_info("Detection %d below threshold: %f < %f", i, boxes[i].score, m->sod.threshold);
                continue;
            }
            
            // Copy detection data
            strncpy(result->detections[valid_count].label, 
                    boxes[i].zName ? boxes[i].zName : "object", 
                    MAX_LABEL_LENGTH - 1);
            
            result->detections[valid_count].confidence = boxes[i].score;
            if (result->detections[valid_count].confidence > 1.0f) {
                result->detections[valid_count].confidence = 1.0f;
            }
            
            // Normalize coordinates to 0.0-1.0 range
            result->detections[valid_count].x = (float)boxes[i].x / width;
            result->detections[valid_count].y = (float)boxes[i].y / height;
            result->detections[valid_count].width = (float)boxes[i].w / width;
            result->detections[valid_count].height = (float)boxes[i].h / height;
            
            log_info("Valid detection %d: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]", 
                    valid_count, result->detections[valid_count].label, 
                    result->detections[valid_count].confidence * 100.0f,
                    result->detections[valid_count].x, result->detections[valid_count].y,
                    result->detections[valid_count].width, result->detections[valid_count].height);
            
            valid_count++;
        }
        
        result->count = valid_count;
        log_info("SOD detection found %d valid objects out of %d total detections", valid_count, count);
        
        return 0;
    } else if (strcmp(m->type, MODEL_TYPE_SOD_REALNET) == 0) {
        // Run SOD RealNet detection using the external implementation
        return detect_with_sod_realnet(m->sod_realnet.model, frame_data, width, height, channels, result);
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
