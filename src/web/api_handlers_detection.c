#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <errno.h>
#include <ctype.h>
#include <dirent.h>

#include "web/api_handlers_detection.h"
#include "web/api_handlers_common.h"
#include "core/logger.h"
#include "core/config.h"
#include "video/detection.h"
#include "video/motion_detection.h"
#include "video/stream_manager.h"

// Global detection settings
typedef struct {
    char models_path[MAX_PATH_LENGTH];
    float default_threshold;
    int default_pre_buffer;
    int default_post_buffer;
} detection_settings_t;

static detection_settings_t detection_settings = {
    .models_path = "",
    .default_threshold = 0.5f,
    .default_pre_buffer = 5,
    .default_post_buffer = 10
};

/**
 * Initialize detection settings
 */
void init_detection_settings(void) {
    // Set default values
    char cwd[MAX_PATH_LENGTH];
    if (getcwd(cwd, sizeof(cwd)) != NULL) {
        snprintf(detection_settings.models_path, MAX_PATH_LENGTH - 1, "%s/models", cwd);
    } else {
        // Fallback to absolute path if getcwd fails
        strncpy(detection_settings.models_path, "/var/lib/lightnvr/models", MAX_PATH_LENGTH - 1);
    }
    log_info("Using models directory: %s", detection_settings.models_path);
    
    detection_settings.default_threshold = 0.5f;
    detection_settings.default_pre_buffer = 5;
    detection_settings.default_post_buffer = 10;
    
    // Create models directory if it doesn't exist
    struct stat st;
    if (stat(detection_settings.models_path, &st) != 0) {
        log_info("Creating detection models directory: %s", detection_settings.models_path);
        if (mkdir(detection_settings.models_path, 0755) != 0) {
            log_error("Failed to create detection models directory: %s (error: %s)",
                     detection_settings.models_path, strerror(errno));
        }
    }
    
    // Initialize detection system
    if (init_detection_system() != 0) {
        log_error("Failed to initialize detection system");
    } else {
        log_info("Detection system initialized successfully");
    }
}

/**
 * Handle GET request for detection settings
 */
void handle_get_detection_settings(const http_request_t *request, http_response_t *response) {
    // Create JSON response with detection settings
    char json[1024];
    snprintf(json, sizeof(json),
            "{"
            "\"models_path\":\"%s\","
            "\"default_threshold\":%.2f,"
            "\"default_pre_buffer\":%d,"
            "\"default_post_buffer\":%d,"
            "\"supported_models\":["
            "\"motion\","
            "\"motion_optimized\","
            "\"sod\","
            "\"tflite\""
            "]"
            "}",
            detection_settings.models_path,
            detection_settings.default_threshold,
            detection_settings.default_pre_buffer,
            detection_settings.default_post_buffer);
    
    create_json_response(response, 200, json);
}

/**
 * Handle POST request to update detection settings
 */
void handle_post_detection_settings(const http_request_t *request, http_response_t *response) {
    if (!request->body) {
        create_json_response(response, 400, "{\"error\":\"No request body\"}");
        return;
    }
    
    // Parse JSON request body using the project's JSON utilities
    char models_path[MAX_PATH_LENGTH] = "";
    float threshold = 0.5f;
    int pre_buffer = 5;
    int post_buffer = 10;
    
    // Extract settings from JSON using the project's JSON utilities
    char *value = get_json_string_value(request->body, "models_path");
    if (value) {
        strncpy(models_path, value, MAX_PATH_LENGTH - 1);
        models_path[MAX_PATH_LENGTH - 1] = '\0';
        free(value);
    }
    
    threshold = (float)get_json_integer_value(request->body, "default_threshold", 50) / 100.0f;
    log_info("Setting detection threshold to %.2f (from %lld%%)", 
             threshold, get_json_integer_value(request->body, "default_threshold", 50));
    pre_buffer = (int)get_json_integer_value(request->body, "default_pre_buffer", 5);
    post_buffer = (int)get_json_integer_value(request->body, "default_post_buffer", 10);
    
    // Update settings if provided
    if (models_path[0] != '\0') {
        strncpy(detection_settings.models_path, models_path, MAX_PATH_LENGTH - 1);
        detection_settings.models_path[MAX_PATH_LENGTH - 1] = '\0';
        
        // Create models directory if it doesn't exist
        struct stat st;
        if (stat(detection_settings.models_path, &st) != 0) {
            log_info("Creating detection models directory: %s", detection_settings.models_path);
            if (mkdir(detection_settings.models_path, 0755) != 0) {
                log_error("Failed to create detection models directory: %s (error: %s)",
                         detection_settings.models_path, strerror(errno));
            }
        }
    }
    
    // Update threshold if valid
    if (threshold >= 0.0f && threshold <= 1.0f) {
        detection_settings.default_threshold = threshold;
    }
    
    // Update pre-buffer if valid
    if (pre_buffer >= 0) {
        detection_settings.default_pre_buffer = pre_buffer;
    }
    
    // Update post-buffer if valid
    if (post_buffer >= 0) {
        detection_settings.default_post_buffer = post_buffer;
    }
    
    // Return updated settings
    handle_get_detection_settings(request, response);
}

/**
 * Handle GET request to list available detection models
 */
void handle_get_detection_models(const http_request_t *request, http_response_t *response) {
    // List files in the models directory
    DIR *dir = opendir(detection_settings.models_path);
    if (!dir) {
        create_json_response(response, 500, 
                            "{\"error\":\"Failed to open models directory\"}");
        return;
    }
    
    // Build JSON array of model files
    char json[4096] = "{\"models\":[";
    int count = 0;
    
    // Add motion detection as a special "model"
    strcat(json, "{\"name\":\"motion\",\"type\":\"motion\",\"size\":0,\"supported\":true}");
    count++;
    
    // Add optimized motion detection as a special "model"
    strcat(json, ",{\"name\":\"motion_optimized\",\"type\":\"motion_optimized\",\"size\":0,\"supported\":true}");
    count++;
    
    struct dirent *entry;
    
    while ((entry = readdir(dir)) != NULL) {
        // Skip . and .. directories
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) {
            continue;
        }
        
        // Check if file has a supported extension
        const char *ext = strrchr(entry->d_name, '.');
        if (!ext) continue;
        
        if (strcasecmp(ext, ".sod") == 0 || strcasecmp(ext, ".tflite") == 0) {
            // Build full path to check if it's a file
            char full_path[MAX_PATH_LENGTH];
            snprintf(full_path, MAX_PATH_LENGTH, "%s/%s", 
                    detection_settings.models_path, entry->d_name);
            
            struct stat st;
            if (stat(full_path, &st) == 0 && S_ISREG(st.st_mode)) {
                // Add comma if not the first model
                if (count > 0) {
                    strcat(json, ",");
                }
                
                // Add model info to JSON array
                char model_info[512];
                const char *model_type = get_model_type(full_path);
                snprintf(model_info, sizeof(model_info),
                        "{\"name\":\"%s\",\"type\":\"%s\",\"size\":%lld,\"supported\":%s}",
                        entry->d_name, model_type, (long long)st.st_size,
                        is_model_supported(full_path) ? "true" : "false");
                
                strcat(json, model_info);
                count++;
            }
        }
    }
    
    closedir(dir);
    
    // Close JSON array and add total count
    char json_end[128];
    snprintf(json_end, sizeof(json_end), "],\"count\":%d}", count);
    strcat(json, json_end);
    
    create_json_response(response, 200, json);
}

/**
 * Handle POST request to test a detection model
 */
void handle_post_test_detection_model(const http_request_t *request, http_response_t *response) {
    if (!request->body) {
        create_json_response(response, 400, "{\"error\":\"No request body\"}");
        return;
    }
    
    // Parse JSON request body using the project's JSON utilities
    char model_name[MAX_PATH_LENGTH] = "";
    float threshold = 0.5f;
    
    // Extract model name from JSON
    char *model_value = get_json_string_value(request->body, "model_name");
    if (!model_value) {
        create_json_response(response, 400, "{\"error\":\"Missing or invalid model_name\"}");
        return;
    }
    
    strncpy(model_name, model_value, MAX_PATH_LENGTH - 1);
    model_name[MAX_PATH_LENGTH - 1] = '\0';
    free(model_value);
    
    // Extract threshold if provided
    threshold = (float)get_json_integer_value(request->body, "threshold", 50) / 100.0f;
    
    // Special case for motion detection
    if (strcmp(model_name, "motion") == 0) {
        // Motion detection is always supported
        create_json_response(response, 200, 
                            "{\"success\":true,\"message\":\"Motion detection is supported\"}");
        return;
    }
    
    // Special case for optimized motion detection
    if (strcmp(model_name, "motion_optimized") == 0) {
        // Optimized motion detection is always supported
        create_json_response(response, 200, 
                            "{\"success\":true,\"message\":\"Optimized motion detection is supported\"}");
        return;
    }
    
    // Build full path to model file
    char model_path[MAX_PATH_LENGTH];
    snprintf(model_path, MAX_PATH_LENGTH, "%s/%s", detection_settings.models_path, model_name);
    
    // Check if model file exists
    struct stat st;
    if (stat(model_path, &st) != 0 || !S_ISREG(st.st_mode)) {
        create_json_response(response, 404, "{\"error\":\"Model file not found\"}");
        return;
    }
    
    // Check if model is supported
    if (!is_model_supported(model_path)) {
        create_json_response(response, 400, 
                            "{\"error\":\"Model type not supported on this system\"}");
        return;
    }
    
    // Try to load the model
    detection_model_t model = load_detection_model(model_path, threshold);
    if (!model) {
        create_json_response(response, 500, "{\"error\":\"Failed to load detection model\"}");
        return;
    }
    
    // Model loaded successfully, unload it
    unload_detection_model(model);
    
    // Return success response
    create_json_response(response, 200, 
                        "{\"success\":true,\"message\":\"Model loaded successfully\"}");
}

/**
 * Register detection API handlers
 */
void register_detection_api_handlers(void) {
    // Initialize detection settings
    init_detection_settings();
    
    // Register API handlers
    register_request_handler("/api/detection/settings", "GET", handle_get_detection_settings);
    register_request_handler("/api/detection/settings", "POST", handle_post_detection_settings);
    register_request_handler("/api/detection/models", "GET", handle_get_detection_models);
    register_request_handler("/api/detection/test", "POST", handle_post_test_detection_model);
    
    log_info("Detection API handlers registered");
}
