#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "web/api_handlers_system.h"
#include "web/api_handlers_common.h"
#include "core/config.h"
#include "core/logger.h"
#include "video/stream_manager.h"

#define LIGHTNVR_VERSION_STRING "0.2.0"

// Define a local config variable to work with
static config_t local_config;

// Forward declaration of helper function to get current configuration
static config_t* get_current_config(void);

// Helper function to get current configuration
static config_t* get_current_config(void) {
    // This should return a reference to the actual global config
    extern config_t global_config;  // Declared in streams.c
    return &global_config;
}

/**
 * Handle GET request for system information
 */
void handle_get_system_info(const http_request_t *request, http_response_t *response) {
    // Copy current config settings from global config
    memcpy(&local_config, get_current_config(), sizeof(config_t));
    
    // Get system information
    char json[1024];
    
    // In a real implementation, we would gather actual system information
    // For now, we'll just create a placeholder JSON
    
    snprintf(json, sizeof(json),
             "{"
             "\"version\": \"%s\","
             "\"uptime\": %ld,"
             "\"cpu_usage\": 15,"
             "\"memory_usage\": 128,"
             "\"memory_total\": 256,"
             "\"storage_usage\": 2.5,"
             "\"storage_total\": 32,"
             "\"active_streams\": %d,"
             "\"max_streams\": %d,"
             "\"recording_streams\": 2,"
             "\"data_received\": 1200,"
             "\"data_recorded\": 850"
             "}",
             LIGHTNVR_VERSION_STRING,
             (long)(time(NULL) - time(NULL)),  // For now, just use 0 as uptime
             get_active_stream_count(),
             local_config.max_streams);
    
    create_json_response(response, 200, json);
}

/**
 * Handle GET request for system logs
 */
void handle_get_system_logs(const http_request_t *request, http_response_t *response) {
    // In a real implementation, we would read from the actual log file
    // For now, we'll just create a placeholder JSON with simulated logs
    
    const char *logs =
        "{"
        "\"logs\": ["
        "\"[2025-03-06 22:30:15] [INFO] LightNVR started\","
        "\"[2025-03-06 22:30:16] [INFO] Loaded configuration from /etc/lightnvr/lightnvr.conf\","
        "\"[2025-03-06 22:30:17] [INFO] Initialized database\","
        "\"[2025-03-06 22:30:18] [INFO] Initialized storage manager\","
        "\"[2025-03-06 22:30:19] [INFO] Initialized stream manager\","
        "\"[2025-03-06 22:30:20] [INFO] Initialized web server on port 8080\","
        "\"[2025-03-06 22:30:21] [INFO] Added stream: Front Door\","
        "\"[2025-03-06 22:30:22] [INFO] Added stream: Back Yard\","
        "\"[2025-03-06 22:30:23] [INFO] Started recording: Front Door\","
        "\"[2025-03-06 22:30:24] [INFO] Started recording: Back Yard\""
        "]"
        "}";
    
    create_json_response(response, 200, logs);
}
