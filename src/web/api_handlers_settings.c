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
    
    // Lock the mutex for thread-safe access - with timeout protection
    struct timespec timeout;
    clock_gettime(CLOCK_REALTIME, &timeout);
    timeout.tv_sec += 5; // 5 second timeout
    
    int lock_result = pthread_mutex_timedlock(&config_mutex, &timeout);
    if (lock_result != 0) {
        log_error("Failed to acquire config mutex: %s", strerror(lock_result));
        free(json);
        create_json_response(response, 500, "{\"error\": \"Failed to acquire configuration lock\"}");
        return;
    }
    
    // Get the global config
    config_t *global_config = get_current_config();
    if (!global_config) {
        log_error("Failed to get global config - null pointer returned");
        pthread_mutex_unlock(&config_mutex);
        free(json);
        create_json_response(response, 500, "{\"error\": \"Failed to access global configuration\"}");
        return;
    }
    
    // Make a copy of the current configuration
    memcpy(&local_config, global_config, sizeof(config_t));
    
    // Unlock the mutex while we process the request
    pthread_mutex_unlock(&config_mutex);
    
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
    
    // Check if the storage path exists and create it if it doesn't
    struct stat storage_st = {0};
    if (stat(local_config.storage_path, &storage_st) == -1) {
        log_info("Creating storage directory: %s", local_config.storage_path);
        if (create_directory_recursive(local_config.storage_path) != 0) {
            log_error("Failed to create storage directory: %s (error: %s)", local_config.storage_path, strerror(errno));
            create_json_response(response, 500, "{\"error\": \"Failed to create storage directory\"}");
            return;
        }
    }
    
    // Create HLS directory inside storage path
    char hls_dir[MAX_PATH_LENGTH];
    snprintf(hls_dir, MAX_PATH_LENGTH, "%s/hls", local_config.storage_path);
    if (stat(hls_dir, &storage_st) == -1) {
        log_info("Creating HLS directory: %s", hls_dir);
        if (create_directory_recursive(hls_dir) != 0) {
            log_error("Failed to create HLS directory: %s (error: %s)", hls_dir, strerror(errno));
            create_json_response(response, 500, "{\"error\": \"Failed to create HLS directory\"}");
            return;
        }
    }
    
    // Create stream-specific directories inside HLS directory
    // Get the global config to access stream configurations
    config_t *global_config_for_streams = get_current_config();
    if (global_config_for_streams) {
        for (int i = 0; i < global_config_for_streams->max_streams; i++) {
            if (strlen(global_config_for_streams->streams[i].name) > 0) {
                char stream_dir[MAX_PATH_LENGTH];
                snprintf(stream_dir, MAX_PATH_LENGTH, "%s/hls/%s", 
                         local_config.storage_path, 
                         global_config_for_streams->streams[i].name);
                
                if (stat(stream_dir, &storage_st) == -1) {
                    log_info("Creating stream HLS directory: %s", stream_dir);
                    if (create_directory_recursive(stream_dir) != 0) {
                        log_error("Failed to create stream HLS directory: %s (error: %s)", 
                                 stream_dir, strerror(errno));
                        // Continue anyway, don't fail the whole operation
                    }
                }
            }
        }
    }
    
    // Setup a signal handler for the alarm instead of letting it terminate the process
    struct sigaction sa;
    struct sigaction old_sa;
    sigemptyset(&sa.sa_mask);
    sa.sa_flags = 0;
    sa.sa_handler = handle_save_timeout;
    sigaction(SIGALRM, &sa, &old_sa);
    
    // Set a flag to track if the save operation timed out
    save_timeout_occurred = 0;
    
    // Set a timeout for the save operation
    alarm(15); // 15 second timeout
    
    log_info("Saving configuration to: %s", config_path);
    int save_result = save_config(&local_config, config_path);
    
    // Cancel the timeout
    alarm(0);
    
    // Restore the original signal handler
    sigaction(SIGALRM, &old_sa, NULL);
    
    if (save_timeout_occurred) {
        log_error("Save operation timed out");
        create_json_response(response, 500, "{\"error\": \"Save operation timed out\"}");
        return;
    }
    
    if (save_result != 0) {
        // Failed to save configuration
        log_error("Failed to save configuration to: %s", config_path);
        create_json_response(response, 500, "{\"error\": \"Failed to save configuration\"}");
        return;
    }
    
    // Lock the mutex again to update the global configuration - with timeout protection
    clock_gettime(CLOCK_REALTIME, &timeout);
    timeout.tv_sec += 5; // 5 second timeout
    
    lock_result = pthread_mutex_timedlock(&config_mutex, &timeout);
    if (lock_result != 0) {
        log_error("Failed to acquire config mutex for update: %s", strerror(lock_result));
        create_json_response(response, 500, "{\"error\": \"Failed to update global configuration\"}");
        return;
    }
    
    // Update the global configuration with our local changes
    memcpy(global_config, &local_config, sizeof(config_t));
    
    // Unlock the mutex
    pthread_mutex_unlock(&config_mutex);
    
    // Create success response
    create_json_response(response, 200, "{\"success\": true}");
    
    log_info("Settings updated successfully");
}
