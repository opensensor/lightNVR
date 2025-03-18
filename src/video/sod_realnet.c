#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <dlfcn.h>

#include "video/detection.h"
#include "core/logger.h"

// Include SOD header if SOD is enabled at compile time
#ifdef SOD_ENABLED
#include "sod/sod.h"
#else
// Define SOD constants for when SOD is not available
#define SOD_OK 0
#endif

// SOD RealNet function pointers for dynamic loading
typedef struct {
    void *handle;
    int (*sod_realnet_create)(void **ppOut);
    int (*sod_realnet_load_model_from_mem)(void *pNet, const void *pModel, unsigned int nBytes, unsigned int *pOutHandle);
    int (*sod_realnet_model_config)(void *pNet, unsigned int handle, int conf, ...);
    int (*sod_realnet_detect)(void *pNet, const unsigned char *zGrayImg, int width, int height, void ***apBox, int *pnBox);
    void (*sod_realnet_destroy)(void *pNet);
} sod_realnet_functions_t;

// Global SOD RealNet functions
static sod_realnet_functions_t sod_realnet_funcs = {0};
static bool sod_realnet_available = false;

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

// SOD RealNet model structure
typedef struct {
    #ifdef SOD_ENABLED
    sod_realnet *net;            // SOD RealNet handle
    #else
    void *net;                   // SOD RealNet handle (void* for dynamic loading)
    #endif
    float threshold;             // Detection threshold
} sod_realnet_model_t;

/**
 * Initialize SOD RealNet functions
 */
static bool init_sod_realnet_functions(void) {
    #ifdef SOD_ENABLED
    // SOD is directly linked
    return true;
    #else
    // Try to dynamically load SOD library
    if (sod_realnet_funcs.handle) {
        // Already initialized
        return sod_realnet_available;
    }
    
    sod_realnet_funcs.handle = dlopen("libsod.so", RTLD_LAZY);
    if (!sod_realnet_funcs.handle) {
        log_warn("SOD library not found: %s", dlerror());
        return false;
    }
    
    // Load SOD RealNet functions
    sod_realnet_funcs.sod_realnet_create = dlsym(sod_realnet_funcs.handle, "sod_realnet_create");
    sod_realnet_funcs.sod_realnet_load_model_from_mem = dlsym(sod_realnet_funcs.handle, "sod_realnet_load_model_from_mem");
    sod_realnet_funcs.sod_realnet_model_config = dlsym(sod_realnet_funcs.handle, "sod_realnet_model_config");
    sod_realnet_funcs.sod_realnet_detect = dlsym(sod_realnet_funcs.handle, "sod_realnet_detect");
    sod_realnet_funcs.sod_realnet_destroy = dlsym(sod_realnet_funcs.handle, "sod_realnet_destroy");
    
    // Check if all required functions were loaded
    if (sod_realnet_funcs.sod_realnet_create && sod_realnet_funcs.sod_realnet_load_model_from_mem && 
        sod_realnet_funcs.sod_realnet_model_config && sod_realnet_funcs.sod_realnet_detect && 
        sod_realnet_funcs.sod_realnet_destroy) {
        log_info("SOD RealNet functions dynamically loaded");
        sod_realnet_available = true;
        return true;
    } else {
        log_warn("SOD library found but missing required RealNet functions");
        dlclose(sod_realnet_funcs.handle);
        sod_realnet_funcs.handle = NULL;
        return false;
    }
    #endif
}

/**
 * Load a SOD RealNet model
 */
void* load_sod_realnet_model(const char *model_path, float threshold) {
    // Initialize SOD RealNet functions
    if (!init_sod_realnet_functions()) {
        log_error("SOD RealNet functions not available");
        return NULL;
    }
    
    // Create RealNet handle
    void *net = NULL;
    int rc;
    
    #ifdef SOD_ENABLED
    rc = sod_realnet_create((sod_realnet**)&net);
    #else
    rc = sod_realnet_funcs.sod_realnet_create(&net);
    #endif
    
    if (rc != 0) { // SOD_OK is 0
        log_error("Failed to create SOD RealNet handle: %d", rc);
        return NULL;
    }
    
    // Load the model
    unsigned int handle;
    
    // Use sod_realnet_load_model_from_mem since sod_realnet_load_model_from_disk requires SOD_NO_MMAP
    // First, read the model file into memory
    FILE *fp = fopen(model_path, "rb");
    if (!fp) {
        log_error("Failed to open SOD RealNet model file: %s", model_path);
        #ifdef SOD_ENABLED
        sod_realnet_destroy(net);
        #else
        sod_realnet_funcs.sod_realnet_destroy(net);
        #endif
        return NULL;
    }
    
    // Get file size
    fseek(fp, 0, SEEK_END);
    long file_size = ftell(fp);
    fseek(fp, 0, SEEK_SET);
    
    // Allocate memory for model data
    void *model_data = malloc(file_size);
    if (!model_data) {
        log_error("Failed to allocate memory for SOD RealNet model data");
        fclose(fp);
        #ifdef SOD_ENABLED
        sod_realnet_destroy(net);
        #else
        sod_realnet_funcs.sod_realnet_destroy(net);
        #endif
        return NULL;
    }
    
    // Read model data
    if (fread(model_data, 1, file_size, fp) != file_size) {
        log_error("Failed to read SOD RealNet model data");
        free(model_data);
        fclose(fp);
        #ifdef SOD_ENABLED
        sod_realnet_destroy(net);
        #else
        sod_realnet_funcs.sod_realnet_destroy(net);
        #endif
        return NULL;
    }
    
    fclose(fp);
    
    // Load model from memory
    #ifdef SOD_ENABLED
    rc = sod_realnet_load_model_from_mem(net, model_data, file_size, &handle);
    #else
    rc = sod_realnet_funcs.sod_realnet_load_model_from_mem(net, model_data, file_size, &handle);
    #endif
    
    // Free model data
    free(model_data);
    if (rc != 0) { // SOD_OK is 0
        log_error("Failed to load SOD RealNet model: %s (error: %d)", model_path, rc);
        #ifdef SOD_ENABLED
        sod_realnet_destroy(net);
        #else
        sod_realnet_funcs.sod_realnet_destroy(net);
        #endif
        return NULL;
    }
    
    // Create model structure
    sod_realnet_model_t *model = (sod_realnet_model_t *)malloc(sizeof(sod_realnet_model_t));
    if (!model) {
        log_error("Failed to allocate memory for SOD RealNet model structure");
        #ifdef SOD_ENABLED
        sod_realnet_destroy(net);
        #else
        sod_realnet_funcs.sod_realnet_destroy(net);
        #endif
        return NULL;
    }
    
    // Initialize model structure
    model->net = net;
    model->threshold = threshold;
    
    log_info("SOD RealNet model loaded: %s", model_path);
    return model;
}

/**
 * Free a SOD RealNet model
 */
void free_sod_realnet_model(void *model) {
    if (!model) {
        return;
    }
    
    sod_realnet_model_t *m = (sod_realnet_model_t *)model;
    
    // Destroy RealNet handle
    #ifdef SOD_ENABLED
    sod_realnet_destroy(m->net);
    #else
    if (sod_realnet_funcs.sod_realnet_destroy) {
        sod_realnet_funcs.sod_realnet_destroy(m->net);
    }
    #endif
    
    // Free model structure
    free(m);
}

/**
 * Run detection on a frame using SOD RealNet
 */
int detect_with_sod_realnet(void *model, const unsigned char *frame_data, 
                           int width, int height, int channels, detection_result_t *result) {
    if (!model || !frame_data || !result) {
        return -1;
    }
    
    // Check if SOD RealNet is available
    if (!init_sod_realnet_functions()) {
        log_error("SOD RealNet functions not available");
        return -1;
    }
    
    sod_realnet_model_t *m = (sod_realnet_model_t *)model;
    
    // Initialize result
    result->count = 0;
    
    // Create a copy of the frame data for processing
    unsigned char *blob = malloc(width * height * channels);
    if (!blob) {
        log_error("Failed to allocate memory for frame blob");
        return -1;
    }
    memcpy(blob, frame_data, width * height * channels);
    
    // Run detection
    void *boxes = NULL;
    int box_count = 0;
    int rc;
    
    #ifdef SOD_ENABLED
    rc = sod_realnet_detect(m->net, blob, width, height, (sod_box**)&boxes, &box_count);
    #else
    rc = sod_realnet_funcs.sod_realnet_detect(m->net, blob, width, height, (void***)&boxes, &box_count);
    #endif
    
    if (rc != 0) { // SOD_OK is 0
        log_error("SOD RealNet detection failed: %d", rc);
        free(blob);
        return -1;
    }
    
    // Process detections
    int valid_count = 0;
    for (int i = 0; i < box_count && valid_count < MAX_DETECTIONS; i++) {
        // Get box data
        #ifdef SOD_ENABLED
        sod_box *box_array = (sod_box*)boxes;
        float score = box_array[i].score;
        const char *name = box_array[i].zName;
        int x = box_array[i].x;
        int y = box_array[i].y;
        int w = box_array[i].w;
        int h = box_array[i].h;
        #else
        // For dynamic loading, we need to access the box array differently
        // FIXED: boxes is an array of sod_box structures, not pointers to structures
        sod_box_dynamic *box_array = (sod_box_dynamic*)boxes;
        float score = box_array[i].score;
        const char *name = box_array[i].zName;
        int x = box_array[i].x;
        int y = box_array[i].y;
        int w = box_array[i].w;
        int h = box_array[i].h;
        #endif
        
        // Apply threshold
        if (score < m->threshold) {
            continue;
        }
        
        // Copy detection data
        strncpy(result->detections[valid_count].label, 
                name ? name : "face", 
                MAX_LABEL_LENGTH - 1);
        
        // Convert confidence from SOD score to 0.0-1.0 range
        // SOD RealNet typically uses a score > 5.0 for good detections
        result->detections[valid_count].confidence = (float)(score / 10.0);
        if (result->detections[valid_count].confidence > 1.0f) {
            result->detections[valid_count].confidence = 1.0f;
        }
        
        // Normalize coordinates to 0.0-1.0 range
        result->detections[valid_count].x = (float)x / width;
        result->detections[valid_count].y = (float)y / height;
        result->detections[valid_count].width = (float)w / width;
        result->detections[valid_count].height = (float)h / height;
        
        valid_count++;
    }
    
    result->count = valid_count;
    
    // Free the blob
    free(blob);
    
    return 0;
}
