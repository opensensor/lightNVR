#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#include "video/detection.h"
#include "core/logger.h"
#include "sod/sod.h"

// SOD RealNet model structure
typedef struct {
    sod_realnet *net;            // SOD RealNet handle
    float threshold;             // Detection threshold
} sod_realnet_model_t;

/**
 * Load a SOD RealNet model
 */
void* load_sod_realnet_model(const char *model_path, float threshold) {
    // Create RealNet handle
    sod_realnet *net = NULL;
    int rc = sod_realnet_create(&net);
    if (rc != SOD_OK) {
        log_error("Failed to create SOD RealNet handle: %d", rc);
        return NULL;
    }
    
    // Load the model
    sod_realnet_model_handle handle;
    
    // Use sod_realnet_load_model_from_mem since sod_realnet_load_model_from_disk requires SOD_NO_MMAP
    // First, read the model file into memory
    FILE *fp = fopen(model_path, "rb");
    if (!fp) {
        log_error("Failed to open SOD RealNet model file: %s", model_path);
        sod_realnet_destroy(net);
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
        sod_realnet_destroy(net);
        return NULL;
    }
    
    // Read model data
    if (fread(model_data, 1, file_size, fp) != file_size) {
        log_error("Failed to read SOD RealNet model data");
        free(model_data);
        fclose(fp);
        sod_realnet_destroy(net);
        return NULL;
    }
    
    fclose(fp);
    
    // Load model from memory
    rc = sod_realnet_load_model_from_mem(net, model_data, file_size, &handle);
    
    // Free model data
    free(model_data);
    if (rc != SOD_OK) {
        log_error("Failed to load SOD RealNet model: %s (error: %d)", model_path, rc);
        sod_realnet_destroy(net);
        return NULL;
    }
    
    // Create model structure
    sod_realnet_model_t *model = (sod_realnet_model_t *)malloc(sizeof(sod_realnet_model_t));
    if (!model) {
        log_error("Failed to allocate memory for SOD RealNet model structure");
        sod_realnet_destroy(net);
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
    sod_realnet_destroy(m->net);
    
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
    sod_box *boxes = NULL;
    int box_count = 0;
    int rc = sod_realnet_detect(m->net, blob, width, height, &boxes, &box_count);
    
    if (rc != SOD_OK) {
        log_error("SOD RealNet detection failed: %d", rc);
        free(blob);
        return -1;
    }
    
    // Process detections
    int valid_count = 0;
    for (int i = 0; i < box_count && valid_count < MAX_DETECTIONS; i++) {
        // Apply threshold
        if (boxes[i].score < m->threshold) {
            continue;
        }
        
        // Copy detection data
        strncpy(result->detections[valid_count].label, 
                boxes[i].zName ? boxes[i].zName : "face", 
                MAX_LABEL_LENGTH - 1);
        
        // Convert confidence from SOD score to 0.0-1.0 range
        // SOD RealNet typically uses a score > 5.0 for good detections
        result->detections[valid_count].confidence = (float)(boxes[i].score / 10.0);
        if (result->detections[valid_count].confidence > 1.0f) {
            result->detections[valid_count].confidence = 1.0f;
        }
        
        // Normalize coordinates to 0.0-1.0 range
        result->detections[valid_count].x = (float)boxes[i].x / width;
        result->detections[valid_count].y = (float)boxes[i].y / height;
        result->detections[valid_count].width = (float)boxes[i].w / width;
        result->detections[valid_count].height = (float)boxes[i].h / height;
        
        valid_count++;
    }
    
    result->count = valid_count;
    
    // Free the blob
    free(blob);
    
    return 0;
}
