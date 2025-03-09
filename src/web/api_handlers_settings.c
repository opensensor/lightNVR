#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <errno.h>
#include <unistd.h>

#include "web/api_handlers_settings.h"
#include "web/api_handlers_common.h"
#include "core/config.h"
#include "core/logger.h"

// Forward declaration of helper function to get current configuration
static config_t* get_current_config(void);

// Mutex for thread-safe access to configuration
static pthread_mutex_t config_mutex = PTHREAD_MUTEX_INITIALIZER;

// Helper function to get current configuration
static config_t* get_current_config(void) {
    // The global config is declared in streams.c
    extern config_t global_config;
    
    // Add defensive checks here
    log_debug("Accessing global config structure");

    // Make sure the memory is valid
    if (global_config.web_port == 0) {
        log_warn("Global config appears to be uninitialized (web_port is 0)");
    }

    // Return a pointer to the global config
    return &global_config;
}

void handle_get_settings(const http_request_t *request, http_response_t *response) {
    log_info("Processing settings request...");

    // Use a local config variable to work with
    config_t local_config;
    
    // Lock the mutex for thread-safe access
    pthread_mutex_lock(&config_mutex);
    
    // Get the global config
    config_t *global_config = get_current_config();

    if (!global_config) {
        log_error("Failed to get global config - null pointer returned");
        pthread_mutex_unlock(&config_mutex);
        create_json_response(response, 500, "{\"error\": \"Failed to access global configuration\"}");
        return;
    }

    // Safely copy the configuration
    memset(&local_config, 0, sizeof(config_t));  // Clear first to prevent partial data
    memcpy(&local_config, global_config, sizeof(config_t));
    
    // Unlock the mutex as we now have a local copy
    pthread_mutex_unlock(&config_mutex);

    // Add logging to debug the issue
    log_debug("Creating JSON for settings: log_level=%d, web_port=%d, storage_path=%s",
              local_config.log_level, local_config.web_port, local_config.storage_path);

    // Create JSON string with settings
    char *json_str = create_json_string(&local_config);
    if (!json_str) {
        log_error("Failed to create settings JSON - memory allocation error");
        create_json_response(response, 500, "{\"error\": \"Failed to serialize settings\"}");
        return;
    }

    // Log a portion of the JSON for debugging (safe truncation)
    size_t json_len = strlen(json_str);
    size_t log_len = json_len > 300 ? 300 : json_len;
    char *debug_json = malloc(log_len + 4);  // +4 for "...\0"

    if (debug_json) {
        memcpy(debug_json, json_str, log_len);
        if (json_len > 300) {
            strcpy(debug_json + log_len, "...");
        } else {
            debug_json[log_len] = '\0';
        }

        log_debug("Settings JSON (may be truncated): %s", debug_json);
        free(debug_json);
    }

    // Create response
    log_debug("Creating JSON response, status 200");
    create_json_response(response, 200, json_str);

    // Free resources
    free(json_str);
    log_info("Settings request processed successfully");
}

/**
 * Handle POST request for settings
 */
void handle_post_settings(const http_request_t *request, http_response_t *response) {
    // Ensure we have a request body
    if (!request->body || request->content_length == 0) {
        create_json_response(response, 400, "{\"error\": \"Empty request body\"}");
        return;
    }
    
    // Use a local config variable to work with
    config_t local_config;
    
    // Lock the mutex for thread-safe access
    pthread_mutex_lock(&config_mutex);
    
    // Get the global config
    config_t *global_config = get_current_config();
    
    // Make a copy of the current configuration
    memcpy(&local_config, global_config, sizeof(config_t));
    
    // Unlock the mutex while we process the request
    pthread_mutex_unlock(&config_mutex);
    
    // Make a null-terminated copy of the request body
    char *json = malloc(request->content_length + 1);
    if (!json) {
        create_json_response(response, 500, "{\"error\": \"Memory allocation failed\"}");
        return;
    }
    
    memcpy(json, request->body, request->content_length);
    json[request->content_length] = '\0';
    
    // Log the received JSON for debugging
    log_debug("Received settings JSON: %.*s", 
              request->content_length > 300 ? 300 : request->content_length, 
              request->body);
    
    // Update configuration with new settings
    
    // General settings
    int log_level = get_json_integer_value(json, "log_level", local_config.log_level);
    local_config.log_level = log_level;
    
    // Storage settings
    char *storage_path = get_json_string_value(json, "storage_path");
    if (storage_path) {
        strncpy(local_config.storage_path, storage_path, MAX_PATH_LENGTH - 1);
        local_config.storage_path[MAX_PATH_LENGTH - 1] = '\0';
        free(storage_path);
    }
    
    long long max_storage = get_json_integer_value(json, "max_storage", local_config.max_storage_size / (1024 * 1024 * 1024));
    local_config.max_storage_size = max_storage * 1024 * 1024 * 1024; // Convert from GB to bytes
    
    int retention = get_json_integer_value(json, "retention", local_config.retention_days);
    local_config.retention_days = retention;
    
    int auto_delete = get_json_boolean_value(json, "auto_delete", local_config.auto_delete_oldest);
    local_config.auto_delete_oldest = auto_delete;
    
    // Web server settings
    int web_port = get_json_integer_value(json, "web_port", local_config.web_port);
    local_config.web_port = web_port;
    
    int auth_enabled = get_json_boolean_value(json, "auth_enabled", local_config.web_auth_enabled);
    local_config.web_auth_enabled = auth_enabled;
    
    char *username = get_json_string_value(json, "username");
    if (username) {
        strncpy(local_config.web_username, username, 31);
        local_config.web_username[31] = '\0';
        free(username);
    }
    
    char *password = get_json_string_value(json, "password");
    if (password && strcmp(password, "********") != 0) {
        strncpy(local_config.web_password, password, 31);
        local_config.web_password[31] = '\0';
        free(password);
    }
    
    // Memory optimization settings
    int buffer_size = get_json_integer_value(json, "buffer_size", local_config.buffer_size);
    local_config.buffer_size = buffer_size;
    
    int use_swap = get_json_boolean_value(json, "use_swap", local_config.use_swap);
    local_config.use_swap = use_swap;
    
    long long swap_size = get_json_integer_value(json, "swap_size", local_config.swap_size / (1024 * 1024));
    local_config.swap_size = swap_size * 1024 * 1024; // Convert from MB to bytes
    
    // Free the JSON string
    free(json);
    
    // Get the config directory path
    char config_dir[MAX_PATH_LENGTH];
    strncpy(config_dir, "/etc/lightnvr", MAX_PATH_LENGTH - 1);
    config_dir[MAX_PATH_LENGTH - 1] = '\0';
    
    // Check if the directory exists, create it if it doesn't
    struct stat st = {0};
    if (stat(config_dir, &st) == -1) {
        log_info("Creating config directory: %s", config_dir);
        if (mkdir(config_dir, 0755) != 0) {
            log_error("Failed to create config directory: %s (error: %s)", config_dir, strerror(errno));
            create_json_response(response, 500, "{\"error\": \"Failed to create config directory\"}");
            return;
        }
    }
    
    // Save configuration to file
    char config_path[MAX_PATH_LENGTH];
    snprintf(config_path, MAX_PATH_LENGTH, "%s/lightnvr.conf", config_dir);
    
    log_info("Saving configuration to: %s", config_path);
    if (save_config(&local_config, config_path) != 0) {
        // Failed to save configuration
        log_error("Failed to save configuration to: %s", config_path);
        create_json_response(response, 500, "{\"error\": \"Failed to save configuration\"}");
        return;
    }
    
    // Lock the mutex again to update the global configuration
    pthread_mutex_lock(&config_mutex);
    
    // Update the global configuration with our local changes
    memcpy(global_config, &local_config, sizeof(config_t));
    
    // Unlock the mutex
    pthread_mutex_unlock(&config_mutex);
    
    // Create success response
    create_json_response(response, 200, "{\"success\": true}");
    
    log_info("Settings updated successfully");
}
