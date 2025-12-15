/**
 * Pre-Detection Buffer Strategy Implementation
 * 
 * Factory and common functions for buffer strategies.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>

#include "video/pre_detection_buffer.h"
#include "core/logger.h"
#include "core/config.h"

// Forward declarations for strategy constructors
extern pre_buffer_strategy_t* create_go2rtc_strategy(const char *stream_name, const buffer_config_t *config);
extern pre_buffer_strategy_t* create_hls_segment_strategy(const char *stream_name, const buffer_config_t *config);
extern pre_buffer_strategy_t* create_memory_packet_strategy(const char *stream_name, const buffer_config_t *config);
extern pre_buffer_strategy_t* create_mmap_hybrid_strategy(const char *stream_name, const buffer_config_t *config);

// Strategy type names
static const char* strategy_names[] = {
    [BUFFER_STRATEGY_NONE] = "none",
    [BUFFER_STRATEGY_GO2RTC_NATIVE] = "go2rtc",
    [BUFFER_STRATEGY_HLS_SEGMENT] = "hls_segment",
    [BUFFER_STRATEGY_MEMORY_PACKET] = "memory_packet",
    [BUFFER_STRATEGY_MMAP_HYBRID] = "mmap_hybrid",
    [BUFFER_STRATEGY_AUTO] = "auto",
};

/**
 * Get the default/recommended strategy type based on system resources
 */
buffer_strategy_type_t get_recommended_strategy_type(void) {
    // Check available memory
    FILE *meminfo = fopen("/proc/meminfo", "r");
    if (meminfo) {
        char line[256];
        long available_kb = 0;
        while (fgets(line, sizeof(line), meminfo)) {
            if (strncmp(line, "MemAvailable:", 13) == 0) {
                sscanf(line + 13, "%ld", &available_kb);
                break;
            }
        }
        fclose(meminfo);
        
        // If less than 256MB available, prefer disk-based strategies
        if (available_kb > 0 && available_kb < 256 * 1024) {
            log_info("Low memory detected (%ld KB), recommending HLS segment strategy", available_kb);
            return BUFFER_STRATEGY_HLS_SEGMENT;
        }
    }
    
    // Check if go2rtc is available by checking for config
    extern config_t g_config;
    if (g_config.go2rtc_api_port > 0) {
        log_info("go2rtc detected, recommending go2rtc native strategy");
        return BUFFER_STRATEGY_GO2RTC_NATIVE;
    }
    
    // Default to HLS segment tracking (good balance)
    return BUFFER_STRATEGY_HLS_SEGMENT;
}

/**
 * Convert strategy type to string name
 */
const char* buffer_strategy_type_to_string(buffer_strategy_type_t type) {
    if (type >= 0 && type < BUFFER_STRATEGY_COUNT) {
        return strategy_names[type];
    }
    return "unknown";
}

/**
 * Parse strategy type from string name
 */
buffer_strategy_type_t buffer_strategy_type_from_string(const char *name) {
    if (!name || !name[0]) {
        return BUFFER_STRATEGY_AUTO;
    }
    
    for (int i = 0; i < BUFFER_STRATEGY_COUNT; i++) {
        if (strategy_names[i] && strcasecmp(name, strategy_names[i]) == 0) {
            return (buffer_strategy_type_t)i;
        }
    }
    
    log_warn("Unknown buffer strategy type: %s, defaulting to auto", name);
    return BUFFER_STRATEGY_AUTO;
}

/**
 * Create a buffer strategy for a stream
 */
pre_buffer_strategy_t* create_buffer_strategy(buffer_strategy_type_t type,
                                               const char *stream_name,
                                               const buffer_config_t *config) {
    if (!stream_name || !config) {
        log_error("Invalid parameters for create_buffer_strategy");
        return NULL;
    }
    
    // Handle AUTO by selecting recommended type
    if (type == BUFFER_STRATEGY_AUTO) {
        type = get_recommended_strategy_type();
        log_info("Auto-selected buffer strategy: %s for stream %s",
                 buffer_strategy_type_to_string(type), stream_name);
    }
    
    // Handle NONE - return NULL (no buffering)
    if (type == BUFFER_STRATEGY_NONE) {
        log_info("Pre-detection buffering disabled for stream %s", stream_name);
        return NULL;
    }
    
    pre_buffer_strategy_t *strategy = NULL;
    
    switch (type) {
        case BUFFER_STRATEGY_GO2RTC_NATIVE:
            strategy = create_go2rtc_strategy(stream_name, config);
            break;
            
        case BUFFER_STRATEGY_HLS_SEGMENT:
            strategy = create_hls_segment_strategy(stream_name, config);
            break;
            
        case BUFFER_STRATEGY_MEMORY_PACKET:
            strategy = create_memory_packet_strategy(stream_name, config);
            break;
            
        case BUFFER_STRATEGY_MMAP_HYBRID:
            strategy = create_mmap_hybrid_strategy(stream_name, config);
            break;
            
        default:
            log_error("Unknown buffer strategy type: %d", type);
            return NULL;
    }
    
    if (strategy) {
        log_info("Created %s buffer strategy for stream %s (buffer: %ds)",
                 strategy->name, stream_name, config->buffer_seconds);
    }
    
    return strategy;
}

/**
 * Destroy a buffer strategy and free resources
 */
void destroy_buffer_strategy(pre_buffer_strategy_t *strategy) {
    if (!strategy) {
        return;
    }
    
    log_info("Destroying %s buffer strategy for stream %s",
             strategy->name, strategy->stream_name);
    
    if (strategy->destroy) {
        strategy->destroy(strategy);
    }
    
    free(strategy);
}

