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
#include <signal.h>
#include <libgen.h>

#include "web/api_handlers_settings.h"
#include "web/api_handlers_common.h"
#include "core/config.h"
#include "core/logger.h"
#include "storage/storage_manager.h"

// Forward declaration of helper function to get current configuration
static config_t* get_current_config(void);

// Flag to track if a save operation timed out
static volatile int save_timeout_occurred = 0;

// Signal handler for save timeout
static void handle_save_timeout(int sig) {
    save_timeout_occurred = 1;
    log_error("Save operation timed out (signal received)");
}

// Recursive directory creation function
static int create_directory_recursive(const char *path) {
    char tmp[MAX_PATH_LENGTH];
    char *p = NULL;
    size_t len;
    
    if (!path || strlen(path) == 0) {
        return -1;
    }
    
    snprintf(tmp, sizeof(tmp), "%s", path);
    len = strlen(tmp);
    
    // Remove trailing slash
    if (tmp[len - 1] == '/') {
        tmp[len - 1] = 0;
    }
    
    // Check if directory already exists
    struct stat st = {0};
    if (stat(tmp, &st) == 0) {
        if (S_ISDIR(st.st_mode)) {
            return 0; // Directory exists
        } else {
            return -1; // Path exists but is not a directory
        }
    }
    
    // Create parent directories recursively
    for (p = tmp + 1; *p; p++) {
        if (*p == '/') {
            *p = 0;
            
            // Create this directory segment
            if (stat(tmp, &st) != 0) {
                if (mkdir(tmp, 0755) != 0 && errno != EEXIST) {
                    log_error("Failed to create directory: %s (error: %s)", tmp, strerror(errno));
                    return -1;
                }
            } else if (!S_ISDIR(st.st_mode)) {
                log_error("Path exists but is not a directory: %s", tmp);
                return -1;
            }
            
            *p = '/';
        }
    }
    
    // Create the final directory
    if (stat(tmp, &st) != 0) {
        if (mkdir(tmp, 0755) != 0 && errno != EEXIST) {
            log_error("Failed to create directory: %s (error: %s)", tmp, strerror(errno));
            return -1;
        }
    } else if (!S_ISDIR(st.st_mode)) {
        log_error("Path exists but is not a directory: %s", tmp);
        return -1;
    }
    
    return 0;
}

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

    // Get the global config
    config_t *global_config = get_current_config();

    if (!global_config) {
        log_error("Failed to get global config - null pointer returned");
        create_json_response(response, 500, "{\"error\": \"Failed to access global configuration\"}");
        return;
    }

    // Safely copy the configuration
    memset(&local_config, 0, sizeof(config_t));  // Clear first to prevent partial data
    memcpy(&local_config, global_config, sizeof(config_t));

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

void handle_post_settings(const http_request_t *request, http_response_t *response) {
    log_info("Processing settings update request");

    // Basic request check and JSON parsing
    if (!request->body || request->content_length == 0) {
        create_json_response(response, 400, "{\"error\": \"Empty request body\"}");
        return;
    }

    char *json = malloc(request->content_length + 1);
    if (!json) {
        create_json_response(response, 500, "{\"error\": \"Memory allocation failed\"}");
        return;
    }

    memcpy(json, request->body, request->content_length);
    json[request->content_length] = '\0';

    // Get config and modify it
    config_t *global_config = get_current_config();
    if (!global_config) {
        log_error("Failed to get global config");
        free(json);
        create_json_response(response, 500, "{\"error\": \"Failed to access global configuration\"}");
        return;
    }

    config_t local_config;
    memcpy(&local_config, global_config, sizeof(config_t));

    // Update all settings from the JSON request
    log_debug("Parsing settings from JSON request");

    // General settings
    local_config.log_level = get_json_integer_value(json, "log_level", local_config.log_level);
    
    // Storage settings
    char *storage_path = get_json_string_value(json, "storage_path");
    if (storage_path) {
        strncpy(local_config.storage_path, storage_path, sizeof(local_config.storage_path) - 1);
        local_config.storage_path[sizeof(local_config.storage_path) - 1] = '\0';
        free(storage_path);
    }
    
    // Convert max_storage from GB to bytes
    long long max_storage_gb = get_json_integer_value(json, "max_storage", 
                                                     local_config.max_storage_size / (1024 * 1024 * 1024));
    local_config.max_storage_size = max_storage_gb * (uint64_t)(1024 * 1024 * 1024);
    
    local_config.retention_days = get_json_integer_value(json, "retention", local_config.retention_days);
    local_config.auto_delete_oldest = get_json_boolean_value(json, "auto_delete", local_config.auto_delete_oldest);
    
    // Web server settings
    local_config.web_port = get_json_integer_value(json, "web_port", local_config.web_port);
    local_config.web_auth_enabled = get_json_boolean_value(json, "auth_enabled", local_config.web_auth_enabled);
    
    char *username = get_json_string_value(json, "username");
    if (username) {
        strncpy(local_config.web_username, username, sizeof(local_config.web_username) - 1);
        local_config.web_username[sizeof(local_config.web_username) - 1] = '\0';
        free(username);
    }
    
    char *password = get_json_string_value(json, "password");
    if (password && strcmp(password, "********") != 0) {
        // Only update password if it's not the placeholder
        strncpy(local_config.web_password, password, sizeof(local_config.web_password) - 1);
        local_config.web_password[sizeof(local_config.web_password) - 1] = '\0';
        free(password);
    }
    
    // Memory optimization
    local_config.buffer_size = get_json_integer_value(json, "buffer_size", local_config.buffer_size);
    local_config.use_swap = get_json_boolean_value(json, "use_swap", local_config.use_swap);
    
    // Convert swap_size from MB to bytes
    long long swap_size_mb = get_json_integer_value(json, "swap_size", 
                                                  local_config.swap_size / (1024 * 1024));
    local_config.swap_size = swap_size_mb * (uint64_t)(1024 * 1024);

    // Update global config
    memcpy(global_config, &local_config, sizeof(config_t));
    
    // Update storage manager with new settings
    set_max_storage_size(local_config.max_storage_size);
    set_retention_days(local_config.retention_days);
    
    // Save configuration to disk
    const char *config_paths[] = {
        "./lightnvr.conf",
        "/etc/lightnvr/lightnvr.conf",
        NULL
    };
    
    // Try to save to the first existing config file
    int saved = 0;
    for (int i = 0; config_paths[i] != NULL; i++) {
        if (access(config_paths[i], F_OK) == 0) {
            if (save_config(global_config, config_paths[i]) == 0) {
                log_info("Saved configuration to %s", config_paths[i]);
                saved = 1;
                break;
            } else {
                log_error("Failed to save configuration to %s", config_paths[i]);
            }
        }
    }
    
    // If no existing config file was found, save to the default location
    if (!saved) {
        if (save_config(global_config, "/etc/lightnvr/lightnvr.conf") == 0) {
            log_info("Saved configuration to /etc/lightnvr/lightnvr.conf");
        } else {
            log_error("Failed to save configuration to /etc/lightnvr/lightnvr.conf");
            // Try to save to the current directory as a fallback
            if (save_config(global_config, "./lightnvr.conf") == 0) {
                log_info("Saved configuration to ./lightnvr.conf");
            } else {
                log_error("Failed to save configuration to ./lightnvr.conf");
                // Don't return an error to the client, as the in-memory settings were updated
            }
        }
    }
    
    log_info("Settings updated: log_level=%d, max_storage=%lu GB, retention=%d days", 
             local_config.log_level, 
             (unsigned long long)(local_config.max_storage_size / (1024 * 1024 * 1024)),
             local_config.retention_days);

    free(json);

    // Create success response
    create_json_response(response, 200, "{\"success\": true}");
    log_info("Settings update completed successfully");
}
