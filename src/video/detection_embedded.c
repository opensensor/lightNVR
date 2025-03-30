#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <fcntl.h>
#include <errno.h>
#include <time.h>

#include "../../include/core/logger.h"
#include "../../include/video/detection_config.h"
#include "../../include/video/detection_embedded.h"

/**
 * Check if we're running on an embedded device
 */
bool is_embedded_device(void) {
    static bool checked = false;
    static bool is_embedded = false;
    
    // Only check once
    if (checked) {
        return is_embedded;
    }
    
    // Check available system memory
    FILE *meminfo = fopen("/proc/meminfo", "r");
    if (meminfo) {
        char line[256];
        unsigned long total_mem_kb = 0;
        
        while (fgets(line, sizeof(line), meminfo)) {
            if (strncmp(line, "MemTotal:", 9) == 0) {
                sscanf(line, "MemTotal: %lu", &total_mem_kb);
                break;
            }
        }
        fclose(meminfo);
        
        // If total memory is less than 512MB, consider it an embedded device
        if (total_mem_kb > 0 && total_mem_kb < 512 * 1024) {
            log_info("Detected embedded device with %lu KB RAM", total_mem_kb);
            is_embedded = true;
            checked = true;
            return true;
        }
    }
    
    // Check number of CPU cores
    int num_cores = sysconf(_SC_NPROCESSORS_ONLN);
    if (num_cores > 0 && num_cores <= 2) {
        log_info("Detected embedded device with %d CPU cores", num_cores);
        is_embedded = true;
        checked = true;
        return true;
    }
    
    // Not an embedded device
    checked = true;
    return false;
}

/**
 * Get the appropriate downscale factor based on model type and device
 */
int get_downscale_factor(const char *model_type) {
    // Get configuration
    detection_config_t *config = get_detection_config();
    if (!config) {
        log_error("Failed to get detection configuration");
        return 1; // Default to no downscaling
    }
    
    // Check if we're running on an embedded device
    bool embedded = is_embedded_device();
    
    // For non-embedded devices, use appropriate factors based on model type
    if (!embedded) {
        if (strcmp(model_type, MODEL_TYPE_SOD) == 0) {
            return config->downscale_factor_cnn;
        } else if (strcmp(model_type, MODEL_TYPE_SOD_REALNET) == 0) {
            return config->downscale_factor_realnet;
        } else {
            return config->downscale_factor_default;
        }
    }
    
    //  For embedded devices, use more aggressive downscaling for CNN models
    if (strcmp(model_type, MODEL_TYPE_SOD) == 0) {
        // For CNN models on embedded devices, use a higher downscale factor
        // This significantly reduces memory usage and processing time
        int downscale = config->downscale_factor_cnn;
        
        // Check available memory to determine if we need more aggressive downscaling
        FILE *meminfo = fopen("/proc/meminfo", "r");
        if (meminfo) {
            char line[256];
            unsigned long available_mem_kb = 0;
            
            while (fgets(line, sizeof(line), meminfo)) {
                if (strncmp(line, "MemAvailable:", 13) == 0) {
                    sscanf(line, "MemAvailable: %lu", &available_mem_kb);
                    break;
                }
            }
            fclose(meminfo);
            
            // If available memory is very low (less than 50MB), use more aggressive downscaling
            if (available_mem_kb > 0 && available_mem_kb < 50 * 1024) {
                log_warn("Very low memory available (%lu KB), using more aggressive downscaling", 
                        available_mem_kb);
                downscale = 4; // More aggressive downscaling
            }
        }
        
        log_info("Using downscale factor %d for CNN model on embedded device", downscale);
        return downscale;
    } else if (strcmp(model_type, MODEL_TYPE_SOD_REALNET) == 0) {
        // RealNet models are already optimized for embedded devices
        return config->downscale_factor_realnet;
    }
    
    // Default for unknown model types
    return config->downscale_factor_default;
}

/**
 * Get the appropriate detection threshold based on model type and device
 */
float get_detection_threshold(const char *model_type, float configured_threshold) {
    // Get configuration
    detection_config_t *config = get_detection_config();
    if (!config) {
        log_error("Failed to get detection configuration");
        return 0.3f; // Default threshold
    }
    
    // If threshold is configured, use it
    if (configured_threshold > 0.0f) {
        return configured_threshold;
    }
    
    // Check if we're running on an embedded device
    bool embedded = is_embedded_device();
    
    // Use model-specific thresholds
    if (strcmp(model_type, MODEL_TYPE_SOD_REALNET) == 0) {
        return config->threshold_realnet;
    } else if (strcmp(model_type, MODEL_TYPE_SOD) == 0) {
        // For embedded devices, use a higher threshold
        if (embedded) {
            return config->threshold_cnn_embedded;
        } else {
            return config->threshold_cnn;
        }
    }
    
    // Default for unknown model types
    return config->threshold_cnn;
}

/**
 * Downscale a frame for detection
 * Simple bilinear interpolation implementation
 */
int downscale_frame(const uint8_t *src_data, int src_width, int src_height, int src_channels,
                   uint8_t *dst_data, int dst_width, int dst_height, int dst_channels) {
    // Validate parameters
    if (!src_data || !dst_data || src_width <= 0 || src_height <= 0 || dst_width <= 0 || dst_height <= 0 ||
        src_channels <= 0 || dst_channels <= 0) {
        log_error("Invalid parameters for downscale_frame");
        return -1;
    }
    
    // Check if source and destination channels match
    if (src_channels != dst_channels) {
        log_error("Source and destination channels must match");
        return -1;
    }
    
    // Calculate scaling factors
    float scale_x = (float)src_width / dst_width;
    float scale_y = (float)src_height / dst_height;
    
    // Perform downscaling
    for (int y = 0; y < dst_height; y++) {
        for (int x = 0; x < dst_width; x++) {
            // Calculate source coordinates
            float src_x = x * scale_x;
            float src_y = y * scale_y;
            
            // Calculate integer source coordinates
            int src_x_int = (int)src_x;
            int src_y_int = (int)src_y;
            
            // Calculate fractional parts
            float src_x_frac = src_x - src_x_int;
            float src_y_frac = src_y - src_y_int;
            
            // Ensure we don't go out of bounds
            int src_x_int_p1 = (src_x_int + 1 < src_width) ? src_x_int + 1 : src_x_int;
            int src_y_int_p1 = (src_y_int + 1 < src_height) ? src_y_int + 1 : src_y_int;
            
            // Process each channel
            for (int c = 0; c < dst_channels; c++) {
                // Get the four surrounding pixels
                uint8_t p00 = src_data[(src_y_int * src_width + src_x_int) * src_channels + c];
                uint8_t p01 = src_data[(src_y_int * src_width + src_x_int_p1) * src_channels + c];
                uint8_t p10 = src_data[(src_y_int_p1 * src_width + src_x_int) * src_channels + c];
                uint8_t p11 = src_data[(src_y_int_p1 * src_width + src_x_int_p1) * src_channels + c];
                
                // Bilinear interpolation
                float top = p00 * (1 - src_x_frac) + p01 * src_x_frac;
                float bottom = p10 * (1 - src_x_frac) + p11 * src_x_frac;
                float value = top * (1 - src_y_frac) + bottom * src_y_frac;
                
                // Set the destination pixel
                dst_data[(y * dst_width + x) * dst_channels + c] = (uint8_t)value;
            }
        }
    }
    
    return 0;
}

/**
 * Calculate the memory requirements for detection
 */
size_t calculate_detection_memory_requirements(int width, int height, int channels, const char *model_type) {
    // Get downscale factor
    int downscale_factor = get_downscale_factor(model_type);
    
    // Calculate downscaled dimensions
    int downscaled_width = width / downscale_factor;
    int downscaled_height = height / downscale_factor;
    
    // Ensure dimensions are even
    downscaled_width = (downscaled_width / 2) * 2;
    downscaled_height = (downscaled_height / 2) * 2;
    
    // Calculate memory requirements
    size_t frame_size = downscaled_width * downscaled_height * channels;
    
    // Add memory for model and other data structures
    size_t model_size = 0;
    if (strcmp(model_type, MODEL_TYPE_SOD) == 0) {
        // CNN models are larger
        model_size = 20 * 1024 * 1024; // 20MB for CNN model
    } else if (strcmp(model_type, MODEL_TYPE_SOD_REALNET) == 0) {
        // RealNet models are smaller
        model_size = 5 * 1024 * 1024; // 5MB for RealNet model
    }
    
    // Add memory for buffer pool
    detection_config_t *config = get_detection_config();
    size_t buffer_pool_size = config ? config->buffer_pool_size * frame_size : 4 * frame_size;
    
    // Total memory requirements
    return frame_size + model_size + buffer_pool_size;
}
