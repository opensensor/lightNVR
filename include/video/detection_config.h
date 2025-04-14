#ifndef DETECTION_CONFIG_H
#define DETECTION_CONFIG_H

#include <stdbool.h>
#include <stddef.h>

// Model types
#define MODEL_TYPE_SOD "sod"
#define MODEL_TYPE_SOD_REALNET "sod_realnet"
#define MODEL_TYPE_TFLITE "tflite"
#define MODEL_TYPE_API "api"
#define MODEL_TYPE_ONVIF "onvif"

// System configuration
typedef struct {
    // Memory constraints
    int buffer_pool_size;         // Maximum number of buffers in the pool
    int concurrent_detections;    // Maximum number of concurrent detections
    int buffer_allocation_retries; // Number of retries for buffer allocation
    
    // Downscaling factors
    int downscale_factor_default; // No downscaling by default
    int downscale_factor_cnn;     // Downscaling for CNN models
    int downscale_factor_realnet; // Downscaling for RealNet models
    
    // Thresholds
    float threshold_cnn;          // Detection threshold for CNN models
    float threshold_cnn_embedded; // Detection threshold for CNN models on embedded devices
    float threshold_realnet;      // Detection threshold for RealNet models
    
    // Debug options
    bool save_frames_for_debug;   // Enable/disable frame saving
} detection_config_t;

// Default configurations
extern detection_config_t default_config;
extern detection_config_t embedded_config;

/**
 * Initialize detection configuration
 * This should be called at startup
 * 
 * @return 0 on success, non-zero on failure
 */
int init_detection_config(void);

/**
 * Get the current detection configuration
 * 
 * @return Pointer to the current configuration
 */
detection_config_t* get_detection_config(void);

/**
 * Set custom detection configuration
 * 
 * @param config The configuration to set
 * @return 0 on success, non-zero on failure
 */
int set_detection_config(const detection_config_t* config);

/**
 * Get current memory usage statistics for detection
 * 
 * @param total_memory Pointer to store total allocated memory
 * @param peak_memory Pointer to store peak allocated memory
 */
void get_detection_memory_usage(size_t *total_memory, size_t *peak_memory);

#endif /* DETECTION_CONFIG_H */
