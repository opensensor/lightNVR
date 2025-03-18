#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <dlfcn.h>
#include <pthread.h>
#include <unistd.h>
#include <sys/stat.h>

#include "video/detection.h"
#include "video/sod_realnet.h"
#include "video/motion_detection.h"
#include "core/logger.h"

// Include SOD header if SOD is enabled at compile time
#ifdef SOD_ENABLED
#include "sod/sod.h"
#endif

// Define model types
#define MODEL_TYPE_SOD "sod"
#define MODEL_TYPE_SOD_REALNET "sod_realnet"
#define MODEL_TYPE_TFLITE "tflite"

// SOD library function pointers for dynamic loading
typedef struct {
    void *handle;
    int (*sod_cnn_create)(void **ppOut, const char *zArch, const char *zModelPath, const char **pzErr);
    int (*sod_cnn_config)(void *pNet, int conf, ...);
    int (*sod_cnn_predict)(void *pNet, float *pInput, void ***paBox, int *pnBox);
    void (*sod_cnn_destroy)(void *pNet);
    float * (*sod_cnn_prepare_image)(void *pNet, void *in);
    int (*sod_cnn_get_network_size)(void *pNet, int *pWidth, int *pHeight, int *pChannels);
    void * (*sod_make_image)(int w, int h, int c);
    void (*sod_free_image)(void *m);
    
    // RealNet functions
    int (*sod_realnet_create)(void **ppOut);
    int (*sod_realnet_load_model_from_mem)(void *pNet, const void *pModel, unsigned int nBytes, unsigned int *pOutHandle);
    int (*sod_realnet_model_config)(void *pNet, unsigned int handle, int conf, ...);
    int (*sod_realnet_detect)(void *pNet, const unsigned char *zGrayImg, int width, int height, void ***apBox, int *pnBox);
    void (*sod_realnet_destroy)(void *pNet);
} sod_functions_t;

// Global SOD functions
static sod_functions_t sod_funcs = {0};
static bool sod_available = false;

// SOD model structure
typedef struct {
    void *handle;                // Dynamic library handle
    void *model;                 // SOD model handle
    float threshold;             // Detection threshold
    void *(*load_model)(const char *);  // Function pointer for loading model
    void (*free_model)(void *);  // Function pointer for freeing model
    void *(*detect)(void *, const unsigned char *, int, int, int, int *, float); // Function pointer for detection
} sod_model_t;

// SOD box structure (for dynamic loading)
typedef struct {
    int x;
    int y;
    int w;
    int h;
    float score;
    const char *zName;
    void *pUserData;
} sod_box_dynamic;

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
    
    // Check for SOD library
    #ifdef SOD_ENABLED
    // SOD is directly linked
    log_info("SOD library directly linked");
    sod_available = true;
    
    // Check for RealNet support
    log_info("SOD RealNet support available");
    #else
    // Try to dynamically load SOD library
    sod_funcs.handle = dlopen("libsod.so", RTLD_LAZY);
    if (sod_funcs.handle) {
        // Load SOD functions
        sod_funcs.sod_cnn_create = dlsym(sod_funcs.handle, "sod_cnn_create");
        sod_funcs.sod_cnn_config = dlsym(sod_funcs.handle, "sod_cnn_config");
        sod_funcs.sod_cnn_predict = dlsym(sod_funcs.handle, "sod_cnn_predict");
        sod_funcs.sod_cnn_destroy = dlsym(sod_funcs.handle, "sod_cnn_destroy");
        sod_funcs.sod_cnn_prepare_image = dlsym(sod_funcs.handle, "sod_cnn_prepare_image");
        sod_funcs.sod_cnn_get_network_size = dlsym(sod_funcs.handle, "sod_cnn_get_network_size");
        sod_funcs.sod_make_image = dlsym(sod_funcs.handle, "sod_make_image");
        sod_funcs.sod_free_image = dlsym(sod_funcs.handle, "sod_free_image");
        
        // Load RealNet functions
        sod_funcs.sod_realnet_create = dlsym(sod_funcs.handle, "sod_realnet_create");
        sod_funcs.sod_realnet_load_model_from_mem = dlsym(sod_funcs.handle, "sod_realnet_load_model_from_mem");
        sod_funcs.sod_realnet_model_config = dlsym(sod_funcs.handle, "sod_realnet_model_config");
        sod_funcs.sod_realnet_detect = dlsym(sod_funcs.handle, "sod_realnet_detect");
        sod_funcs.sod_realnet_destroy = dlsym(sod_funcs.handle, "sod_realnet_destroy");
        
        // Check if all required functions were loaded
        if (sod_funcs.sod_cnn_create && sod_funcs.sod_cnn_config && 
            sod_funcs.sod_cnn_predict && sod_funcs.sod_cnn_destroy && 
            sod_funcs.sod_cnn_prepare_image && sod_funcs.sod_make_image && 
            sod_funcs.sod_free_image) {
            log_info("SOD library dynamically loaded");
            sod_available = true;
        } else {
            log_warn("SOD library found but missing required functions");
            dlclose(sod_funcs.handle);
            sod_funcs.handle = NULL;
        }
    } else {
        log_warn("SOD library not found: %s", dlerror());
    }
    #endif
    
    // Check for TFLite library
    void *tflite_handle = dlopen("libtensorflowlite.so", RTLD_LAZY);
    if (tflite_handle) {
        log_info("TensorFlow Lite library found and loaded");
        dlclose(tflite_handle);
    } else {
        log_warn("TensorFlow Lite library not found: %s", dlerror());
    }
    
    // Initialize motion detection system
    int motion_ret = init_motion_detection_system();
    if (motion_ret != 0) {
        log_error("Failed to initialize motion detection system");
    } else {
        log_info("Motion detection system initialized");
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
    
    // Close dynamically loaded SOD library if needed
    #ifndef SOD_ENABLED
    if (sod_funcs.handle) {
        dlclose(sod_funcs.handle);
        sod_funcs.handle = NULL;
    }
    #endif
    
    // Shutdown motion detection system
    shutdown_motion_detection_system();
    
    initialized = false;
    sod_available = false;
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
        return sod_available;
    }
    
    // Check for regular SOD models
    if (strcasecmp(ext, ".sod") == 0) {
        return sod_available;
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
        return sod_available ? MODEL_TYPE_SOD_REALNET : "unknown";
    }
    
    // Check for regular SOD models
    if (strcasecmp(ext, ".sod") == 0) {
        return sod_available ? MODEL_TYPE_SOD : "unknown";
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
    if (!sod_available) {
        log_error("SOD library not available");
        return NULL;
    }

    // Load the model using SOD API
    void *sod_model = NULL;
    const char *err_msg = NULL;

    // Create CNN model
    int rc;

    // Check if this is a face detection model based on filename or path
    const char *arch = "default";
    if (strstr(model_path, "face") != NULL ||
        strstr(model_path, "Face") != NULL ||
        strstr(model_path, "FACE") != NULL) {
        arch = ":face";
        log_info("Using :face architecture for CNN model: %s", model_path);
    }

    // If the model file has the same name as the one in the spec, force face architecture
    const char *filename = strrchr(model_path, '/');
    if (filename) {
        filename++; // Skip the '/'
    } else {
        filename = model_path; // No '/' in the path
    }

    if (strcmp(filename, "face_cnn.sod") == 0) {
        arch = ":face";
        log_info("Detected face_cnn.sod, forcing :face architecture");
    }

    #ifdef SOD_ENABLED
    rc = sod_cnn_create((sod_cnn**)&sod_model, arch, model_path, &err_msg);
    #else
    rc = sod_funcs.sod_cnn_create(&sod_model, arch, model_path, &err_msg);
    #endif

    if (rc != 0 || !sod_model) {  // SOD_OK is 0
        log_error("Failed to load SOD model: %s - %s", model_path, err_msg ? err_msg : "Unknown error");
        return NULL;
    }

    // Set detection threshold - use same threshold as spec if not specified
    if (threshold <= 0.0f) {
        threshold = 0.3f; // Default threshold from spec
        log_info("Using default threshold of 0.3 for model %s", model_path);
    }

    #ifdef SOD_ENABLED
    sod_cnn_config(sod_model, SOD_CNN_DETECTION_THRESHOLD, threshold);
    #else
    sod_funcs.sod_cnn_config(sod_model, 2 /* SOD_CNN_DETECTION_THRESHOLD */, threshold);
    #endif

    // Create model structure
    model_t *model = (model_t *)malloc(sizeof(model_t));
    if (!model) {
        log_error("Failed to allocate memory for model structure");
        #ifdef SOD_ENABLED
        sod_cnn_destroy(sod_model);
        #else
        sod_funcs.sod_cnn_destroy(sod_model);
        #endif
        return NULL;
    }

    // Initialize model structure
    strncpy(model->type, MODEL_TYPE_SOD, sizeof(model->type) - 1);
    model->sod.model = sod_model;
    model->sod.threshold = threshold;

    log_info("SOD model loaded: %s with threshold %.2f", model_path, threshold);
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

    // Check if file exists
    FILE *model_file = fopen(model_path, "r");
    if (!model_file) {
        log_error("MODEL FILE NOT FOUND IN DETECTION.C: %s", model_path);
        return NULL;
    } else {
        fclose(model_file);
        log_error("MODEL FILE EXISTS IN DETECTION.C: %s", model_path);
        
        // Get file size
        struct stat st;
        if (stat(model_path, &st) == 0) {
            log_error("MODEL FILE SIZE: %ld bytes", (long)st.st_size);
        }
    }

    // Get model type
    const char *model_type = get_model_type(model_path);
    log_error("MODEL TYPE: %s", model_type);

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
        #ifdef SOD_ENABLED
        sod_cnn_destroy(m->sod.model);
        #else
        if (sod_funcs.sod_cnn_destroy) {
            sod_funcs.sod_cnn_destroy(m->sod.model);
        }
        #endif
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
 * Run detection on a frame, specifically optimized for face detection
 * This implementation exactly matches the SOD image loading process
 */
int detect_objects(detection_model_t model, const unsigned char *frame_data,
                  int width, int height, int channels, detection_result_t *result) {
    if (!model || !frame_data || !result) {
        log_error("Invalid parameters for detect_objects");
        return -1;
    }

    model_t *m = (model_t *)model;

    log_info("Detecting objects using model type: %s (dimensions: %dx%d, channels: %d)",
             m->type, width, height, channels);

    // Initialize result
    result->count = 0;

    if (strcmp(m->type, MODEL_TYPE_SOD) == 0) {
        if (!sod_available) {
            log_error("SOD library not available");
            return -1;
        }

#ifdef SOD_ENABLED
        // Create a color image from the frame data
        // IMPORTANT: Follow the exact approach used in sod_img_load_from_file()
        log_info("Creating SOD image: %dx%d with %d channels", width, height, channels);
        sod_img sod_image = sod_make_image(width, height, channels);
        if (!sod_image.data) {
            log_error("Failed to create SOD image for detection");
            return -1;
        }

        // Convert pixel data with the SAME algorithm as sod_img_load_from_file()
        // This converts from interleaved (RGB RGB) to planar (RRR GGG BBB) format
        log_info("Converting frame data to SOD image format (planar)");
        for (int c_idx = 0; c_idx < channels; c_idx++) {
            for (int y = 0; y < height; y++) {
                for (int x = 0; x < width; x++) {
                    // Destination index in planar format (RRR..., GGG..., BBB...)
                    int dst_index = x + width * y + width * height * c_idx;

                    // Source index in interleaved format (RGB, RGB, RGB...)
                    int src_index = c_idx + channels * x + channels * width * y;

                    // Normalize to 0-1 range like SOD does
                    sod_image.data[dst_index] = (float)frame_data[src_index] / 255.0f;
                }
            }
        }

        // Debug: log a sample of pixels to verify format
        log_info("Sample of normalized image data (first 5 pixels):");
        for (int i = 0; i < 5 && i < width; i++) {
            if (channels >= 3) {
                // In planar format, R values are first, then G, then B
                int r_idx = i;                          // First part of the buffer has all R values
                int g_idx = i + width * height;         // Second part has all G values
                int b_idx = i + 2 * width * height;     // Third part has all B values

                log_info("Pixel %d: R=%.3f, G=%.3f, B=%.3f",
                        i, sod_image.data[r_idx], sod_image.data[g_idx], sod_image.data[b_idx]);
            } else {
                log_info("Pixel %d: %.3f", i, sod_image.data[i]);
            }
        }

        // Prepare for CNN detection
        log_info("Preparing image for CNN detection");
        float *prepared_data = sod_cnn_prepare_image((sod_cnn*)m->sod.model, sod_image);
        if (!prepared_data) {
            log_error("Failed to prepare image for CNN detection");
            sod_free_image(sod_image);
            return -1;
        }

        log_info("Successfully prepared image for CNN detection");

        // Free SOD image as it's no longer needed after prepare
        sod_free_image(sod_image);

        // Run detection
        log_info("Running CNN detection");
        int count = 0;
        sod_box *boxes = NULL;
        int rc = sod_cnn_predict((sod_cnn*)m->sod.model, prepared_data, &boxes, &count);
        if (rc != SOD_OK) {
            log_error("CNN detection failed with error code: %d", rc);
            return -1;
        }

        log_info("CNN detection found %d boxes", count);

        // Process detection results
        int valid_count = 0;
        for (int i = 0; i < count && valid_count < MAX_DETECTIONS; i++) {
            // Get detection data
            char label[MAX_LABEL_LENGTH];
            strncpy(label, boxes[i].zName ? boxes[i].zName : "face", MAX_LABEL_LENGTH - 1);
            label[MAX_LABEL_LENGTH - 1] = '\0';

            float confidence = boxes[i].score;
            if (confidence > 1.0f) confidence = 1.0f;

            // Convert pixel coordinates to normalized 0-1 range
            float x = (float)boxes[i].x / width;
            float y = (float)boxes[i].y / height;
            float w = (float)boxes[i].w / width;
            float h = (float)boxes[i].h / height;

            // Apply threshold
            if (confidence < m->sod.threshold) {
                log_info("Detection %d below threshold: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]",
                       i, label, confidence * 100.0f, x, y, w, h);
                continue;
            }

            // Add valid detection to result
            strncpy(result->detections[valid_count].label, label, MAX_LABEL_LENGTH - 1);
            result->detections[valid_count].confidence = confidence;
            result->detections[valid_count].x = x;
            result->detections[valid_count].y = y;
            result->detections[valid_count].width = w;
            result->detections[valid_count].height = h;

            log_info("Valid detection %d: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]",
                   valid_count, label, confidence * 100.0f, x, y, w, h);

            valid_count++;
        }

        result->count = valid_count;
        log_info("Detection found %d valid objects out of %d total", valid_count, count);
#else
        // When SOD is not directly linked, use function pointers
        if (!sod_funcs.sod_make_image || !sod_funcs.sod_cnn_prepare_image || 
            !sod_funcs.sod_cnn_predict || !sod_funcs.sod_free_image) {
            log_error("Required SOD functions not available");
            return -1;
        }

        // Create a struct to hold image data
        typedef struct {
            int h;
            int w;
            int c;
            float* data;
        } sod_img_dynamic;

        // Create a color image from the frame data
        log_info("Creating SOD image: %dx%d with %d channels", width, height, channels);
        sod_img_dynamic sod_image;
        void *img_ptr = sod_funcs.sod_make_image(width, height, channels);
        if (!img_ptr) {
            log_error("Failed to create SOD image for detection");
            return -1;
        }
        
        // Cast the void pointer to our dynamic structure
        sod_image.h = height;
        sod_image.w = width;
        sod_image.c = channels;
        sod_image.data = (float*)img_ptr;

        // Convert pixel data with the SAME algorithm as sod_img_load_from_file()
        // This converts from interleaved (RGB RGB) to planar (RRR GGG BBB) format
        log_info("Converting frame data to SOD image format (planar)");
        for (int c_idx = 0; c_idx < channels; c_idx++) {
            for (int y = 0; y < height; y++) {
                for (int x = 0; x < width; x++) {
                    // Destination index in planar format (RRR..., GGG..., BBB...)
                    int dst_index = x + width * y + width * height * c_idx;

                    // Source index in interleaved format (RGB, RGB, RGB...)
                    int src_index = c_idx + channels * x + channels * width * y;

                    // Normalize to 0-1 range like SOD does
                    ((float*)img_ptr)[dst_index] = (float)frame_data[src_index] / 255.0f;
                }
            }
        }

        // Debug: log a sample of pixels to verify format
        log_info("Sample of normalized image data (first 5 pixels):");
        for (int i = 0; i < 5 && i < width; i++) {
            if (channels >= 3) {
                // In planar format, R values are first, then G, then B
                int r_idx = i;                          // First part of the buffer has all R values
                int g_idx = i + width * height;         // Second part has all G values
                int b_idx = i + 2 * width * height;     // Third part has all B values

                log_info("Pixel %d: R=%.3f, G=%.3f, B=%.3f",
                        i, ((float*)img_ptr)[r_idx], ((float*)img_ptr)[g_idx], ((float*)img_ptr)[b_idx]);
            } else {
                log_info("Pixel %d: %.3f", i, ((float*)img_ptr)[i]);
            }
        }

        // Prepare for CNN detection
        log_info("Preparing image for CNN detection");
        float *prepared_data = sod_funcs.sod_cnn_prepare_image(m->sod.model, img_ptr);
        if (!prepared_data) {
            log_error("Failed to prepare image for CNN detection");
            sod_funcs.sod_free_image(img_ptr);
            return -1;
        }

        log_info("Successfully prepared image for CNN detection");

        // Free SOD image as it's no longer needed after prepare
        sod_funcs.sod_free_image(img_ptr);

        // Run detection
        log_info("Running CNN detection");
        int count = 0;
        void **boxes_ptr = NULL;
        int rc = sod_funcs.sod_cnn_predict(m->sod.model, prepared_data, &boxes_ptr, &count);
        if (rc != 0) { // SOD_OK is 0
            log_error("CNN detection failed with error code: %d", rc);
            return -1;
        }

        log_info("CNN detection found %d boxes", count);

        // Process detection results
        int valid_count = 0;
        for (int i = 0; i < count && valid_count < MAX_DETECTIONS; i++) {
            // Get detection data from the dynamic box structure
            // FIXED: boxes_ptr is an array of sod_box structures, not pointers to structures
            sod_box_dynamic *boxes_array = (sod_box_dynamic*)boxes_ptr;
            
            char label[MAX_LABEL_LENGTH];
            strncpy(label, boxes_array[i].zName ? boxes_array[i].zName : "face", MAX_LABEL_LENGTH - 1);
            label[MAX_LABEL_LENGTH - 1] = '\0';

            float confidence = boxes_array[i].score;
            if (confidence > 1.0f) confidence = 1.0f;

            // Convert pixel coordinates to normalized 0-1 range
            float x = (float)boxes_array[i].x / width;
            float y = (float)boxes_array[i].y / height;
            float w = (float)boxes_array[i].w / width;
            float h = (float)boxes_array[i].h / height;

            // Apply threshold
            if (confidence < m->sod.threshold) {
                log_info("Detection %d below threshold: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]",
                       i, label, confidence * 100.0f, x, y, w, h);
                continue;
            }

            // Add valid detection to result
            strncpy(result->detections[valid_count].label, label, MAX_LABEL_LENGTH - 1);
            result->detections[valid_count].confidence = confidence;
            result->detections[valid_count].x = x;
            result->detections[valid_count].y = y;
            result->detections[valid_count].width = w;
            result->detections[valid_count].height = h;

            log_info("Valid detection %d: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]",
                   valid_count, label, confidence * 100.0f, x, y, w, h);

            valid_count++;
        }

        result->count = valid_count;
        log_info("Detection found %d valid objects out of %d total", valid_count, count);
#endif
        return 0;
    }
    else if (strcmp(m->type, MODEL_TYPE_SOD_REALNET) == 0) {
        // Run SOD RealNet detection using the existing implementation
        return detect_with_sod_realnet(m->sod_realnet.model, frame_data, width, height, channels, result);
    }
    else if (strcmp(m->type, MODEL_TYPE_TFLITE) == 0) {
        // TFLite detection placeholder - to be implemented later
        log_error("TFLite detection not implemented yet");
        return -1;
    }
    else {
        log_error("Unknown model type: %s", m->type);
        return -1;
    }
}
