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

// Default configuration for standard systems
detection_config_t default_config = {
    // Memory constraints
    .concurrent_detections = 16,    // 16 concurrent detections

    // Downscaling factors
    .downscale_factor_default = 1,   // No downscaling by default
    .downscale_factor_cnn = 2,       // Moderate downscaling for CNN models on non-embedded devices
    .downscale_factor_realnet = 1,   // No downscaling for RealNet models

    // Thresholds
    .threshold_cnn = 0.3f,           // Standard threshold for CNN models
    .threshold_cnn_embedded = 0.3f,  // Same threshold for embedded devices
    .threshold_realnet = 5.0f,       // Standard threshold for RealNet models

    // Debug options
    .save_frames_for_debug = false   // Disable frame saving
};

// Configuration for embedded systems (256MB RAM, 2 cores)
detection_config_t embedded_config = {
    // Memory constraints
    .concurrent_detections = 2,      // 2 concurrent detections

    // Downscaling factors
    .downscale_factor_default = 1,   // No downscaling by default
    .downscale_factor_cnn = 2,       // Reduced downscaling for CNN models (was 6)
    .downscale_factor_realnet = 1,   // No downscaling for RealNet models

    // Thresholds
    .threshold_cnn = 0.3f,           // Standard threshold for CNN models
    .threshold_cnn_embedded = 0.3f,  // Using same threshold as standard (was 0.4f)
    .threshold_realnet = 5.0f,       // Standard threshold for RealNet models

    // Debug options
    .save_frames_for_debug = false   // Disable frame saving
};

// Current active configuration
static detection_config_t *current_config = NULL;

// External memory tracking functions
extern void track_memory_allocation(size_t size, bool is_allocation);
extern size_t get_total_memory_allocated(void);
extern size_t get_peak_memory_allocated(void);

/**
 * Initialize detection configuration
 */
int init_detection_config(void) {
    // Check if already initialized
    if (current_config) {
        return 0;
    }

    // Default to standard configuration
    current_config = &default_config;

    // Check if we're running on an embedded device
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

        // If total memory is less than 512MB, use embedded configuration
        if (total_mem_kb > 0 && total_mem_kb < 512 * 1024) {
            log_info("Detected embedded device with %lu KB RAM, using embedded configuration", total_mem_kb);
            current_config = &embedded_config;
        }
    }

    // Check number of CPU cores
    int num_cores = sysconf(_SC_NPROCESSORS_ONLN);
    if (num_cores > 0 && num_cores <= 2) {
        log_info("Detected embedded device with %d CPU cores, using embedded configuration", num_cores);
        current_config = &embedded_config;
    }

    log_info("Detection configuration initialized");
    return 0;
}

/**
 * Get the current detection configuration
 */
detection_config_t* get_detection_config(void) {
    // Initialize if not already done
    if (!current_config) {
        init_detection_config();
    }

    return current_config;
}

/**
 * Set custom detection configuration
 */
int set_detection_config(const detection_config_t* config) {
    if (!config) {
        log_error("Invalid configuration");
        return -1;
    }

    // Validate configuration
    if (config->concurrent_detections <= 0) {
        log_error("Invalid configuration parameters");
        return -1;
    }

    // Copy configuration
    if (current_config == &default_config) {
        memcpy(&default_config, config, sizeof(detection_config_t));
    } else if (current_config == &embedded_config) {
        memcpy(&embedded_config, config, sizeof(detection_config_t));
    } else {
        // Allocate new configuration if needed
        if (!current_config) {
            current_config = malloc(sizeof(detection_config_t));
            if (!current_config) {
                log_error("Failed to allocate memory for configuration");
                return -1;
            }
        }

        memcpy(current_config, config, sizeof(detection_config_t));
    }

    log_info("Detection configuration updated");
    return 0;
}

/**
 * Get current memory usage statistics for detection
 */
void get_detection_memory_usage(size_t *total_memory, size_t *peak_memory) {
    if (total_memory) {
        *total_memory = get_total_memory_allocated();
    }
    if (peak_memory) {
        *peak_memory = get_peak_memory_allocated();
    }
}
