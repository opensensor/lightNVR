#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <ctype.h>
#include <time.h>
#include <sys/stat.h>
#include <errno.h>
#include <sys/types.h>
#include <fcntl.h>
#include <glob.h>

#include "web/web_server.h"
#include "web/api_handlers.h"
#include "core/config.h"
#include "core/logger.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "database/database_manager.h"
#include "storage/storage_manager.h"

#define LIGHTNVR_VERSION_STRING "0.1.0"

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

// Helper function to create a simple JSON string
char* create_json_string(const config_t *config) {
    char *json = malloc(4096); // Allocate enough space for the JSON
    if (!json) {
        return NULL;
    }
    
    // Format the JSON string manually
    int len = snprintf(json, 4096,
        "{\n"
        "  \"log_level\": %d,\n"
        "  \"storage_path\": \"%s\",\n"
        "  \"max_storage\": %lu,\n"
        "  \"retention\": %d,\n"
        "  \"auto_delete\": %s,\n"
        "  \"web_port\": %d,\n"
        "  \"auth_enabled\": %s,\n"
        "  \"username\": \"%s\",\n"
        "  \"password\": \"********\",\n"
        "  \"buffer_size\": %d,\n"
        "  \"use_swap\": %s,\n"
        "  \"swap_size\": %lu\n"
        "}",
        config->log_level,
        config->storage_path,
        (unsigned long)(config->max_storage_size / (1024 * 1024 * 1024)), // Convert to GB
        config->retention_days,
        config->auto_delete_oldest ? "true" : "false",
        config->web_port,
        config->web_auth_enabled ? "true" : "false",
        config->web_username,
        config->buffer_size,
        config->use_swap ? "true" : "false",
        (unsigned long)(config->swap_size / (1024 * 1024)) // Convert to MB
    );
    
    if (len >= 4096) {
        // Buffer was too small
        free(json);
        return NULL;
    }
    
    return json;
}

// Helper function to parse a JSON value
char* get_json_string_value(const char *json, const char *key) {
    char search_key[256];
    snprintf(search_key, sizeof(search_key), "\"%s\"", key);
    
    char *key_pos = strstr(json, search_key);
    if (!key_pos) {
        return NULL;
    }
    
    // Move past the key and colon
    key_pos += strlen(search_key);
    while (*key_pos && (*key_pos == ' ' || *key_pos == ':')) {
        key_pos++;
    }
    
    // Check if it's a string value
    if (*key_pos == '"') {
        key_pos++; // Skip opening quote
        
        // Find closing quote
        char *end_pos = key_pos;
        while (*end_pos && *end_pos != '"') {
            if (*end_pos == '\\' && *(end_pos + 1)) {
                end_pos += 2; // Skip escaped character
            } else {
                end_pos++;
            }
        }
        
        if (*end_pos == '"') {
            size_t len = end_pos - key_pos;
            char *value = malloc(len + 1);
            if (value) {
                strncpy(value, key_pos, len);
                value[len] = '\0';
                return value;
            }
        }
    }
    
    return NULL;
}

// Helper function to get a boolean value from JSON
int get_json_boolean_value(const char *json, const char *key, int default_value) {
    char search_key[256];
    snprintf(search_key, sizeof(search_key), "\"%s\"", key);
    
    char *key_pos = strstr(json, search_key);
    if (!key_pos) {
        return default_value;
    }
    
    // Move past the key and colon
    key_pos += strlen(search_key);
    while (*key_pos && (*key_pos == ' ' || *key_pos == ':')) {
        key_pos++;
    }
    
    if (strncmp(key_pos, "true", 4) == 0) {
        return 1;
    } else if (strncmp(key_pos, "false", 5) == 0) {
        return 0;
    }
    
    return default_value;
}

// Helper function to get an integer value from JSON
long long get_json_integer_value(const char *json, const char *key, long long default_value) {
    char search_key[256];
    snprintf(search_key, sizeof(search_key), "\"%s\"", key);
    
    char *key_pos = strstr(json, search_key);
    if (!key_pos) {
        return default_value;
    }
    
    // Move past the key and colon
    key_pos += strlen(search_key);
    while (*key_pos && (*key_pos == ' ' || *key_pos == ':')) {
        key_pos++;
    }
    
    // Check if it's a number
    if (isdigit(*key_pos) || *key_pos == '-') {
        return strtoll(key_pos, NULL, 10);
    }
    
    return default_value;
}

// Forward declarations
static char* create_stream_json(const stream_config_t *stream);
static char* create_streams_json_array();
static int parse_stream_json(const char *json, stream_config_t *stream);

/**
 * Handle GET request for settings
 */
void handle_get_settings(const http_request_t *request, http_response_t *response) {
    // Copy current config settings from global config
    memcpy(&local_config, get_current_config(), sizeof(config_t));
    
    // Create JSON string with settings
    char *json_str = create_json_string(&local_config);
    if (!json_str) {
        create_json_response(response, 500, "{\"error\": \"Failed to create settings JSON\"}");
        return;
    }
    
    // Create response
    create_json_response(response, 200, json_str);
    
    // Free resources
    free(json_str);
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
    
    // Copy current config settings from global config
    memcpy(&local_config, get_current_config(), sizeof(config_t));
    
    // Make a null-terminated copy of the request body
    char *json = malloc(request->content_length + 1);
    if (!json) {
        create_json_response(response, 500, "{\"error\": \"Memory allocation failed\"}");
        return;
    }
    
    memcpy(json, request->body, request->content_length);
    json[request->content_length] = '\0';
    
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
    
    // Save configuration to file
    if (save_config(&local_config, "/etc/lightnvr/lightnvr.conf") != 0) {
        // Failed to save configuration
        create_json_response(response, 500, "{\"error\": \"Failed to save configuration\"}");
        return;
    }
    
    // Create success response
    create_json_response(response, 200, "{\"success\": true}");
    
    log_info("Settings updated successfully");
}

/**
 * Handle GET request for streams
 */
void handle_get_streams(const http_request_t *request, http_response_t *response) {
    char *json_array = create_streams_json_array();
    if (!json_array) {
        create_json_response(response, 500, "{\"error\": \"Failed to create streams JSON\"}");
        return;
    }
    
    // Create response
    create_json_response(response, 200, json_array);
    
    // Free resources
    free(json_array);
}

/**
 * Handle GET request for a specific stream
 */
void handle_get_stream(const http_request_t *request, http_response_t *response) {
    // Extract stream name from the URL
    // URL format: /api/streams/{stream_name}
    const char *stream_name = strrchr(request->path, '/');
    if (!stream_name || strlen(stream_name) <= 1) {
        create_json_response(response, 400, "{\"error\": \"Invalid stream name\"}");
        return;
    }
    
    // Skip the '/'
    stream_name++;
    
    // Find the stream
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        create_json_response(response, 404, "{\"error\": \"Stream not found\"}");
        return;
    }
    
    // Get stream configuration
    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        create_json_response(response, 500, "{\"error\": \"Failed to get stream configuration\"}");
        return;
    }
    
    // Create JSON response
    char *json = create_stream_json(&config);
    if (!json) {
        create_json_response(response, 500, "{\"error\": \"Failed to create stream JSON\"}");
        return;
    }
    
    // Create response
    create_json_response(response, 200, json);
    
    // Free resources
    free(json);
}

/**
 * Handle POST request to create a new stream
 */
void handle_post_stream(const http_request_t *request, http_response_t *response) {
    // Copy current config settings from global config
    memcpy(&local_config, get_current_config(), sizeof(config_t));

    // Ensure we have a request body
    if (!request->body || request->content_length == 0) {
        create_json_response(response, 400, "{\"error\": \"Empty request body\"}");
        return;
    }

    // Make a null-terminated copy of the request body
    char *json = malloc(request->content_length + 1);
    if (!json) {
        create_json_response(response, 500, "{\"error\": \"Memory allocation failed\"}");
        return;
    }

    memcpy(json, request->body, request->content_length);
    json[request->content_length] = '\0';

    // Parse JSON into stream configuration
    stream_config_t config;
    if (parse_stream_json(json, &config) != 0) {
        free(json);
        create_json_response(response, 400, "{\"error\": \"Invalid stream configuration\"}");
        return;
    }

    free(json);

    // Check if stream already exists
    if (get_stream_by_name(config.name) != NULL) {
        create_json_response(response, 409, "{\"error\": \"Stream with this name already exists\"}");
        return;
    }

    // Add the stream
    stream_handle_t stream = add_stream(&config);
    if (!stream) {
        create_json_response(response, 500, "{\"error\": \"Failed to add stream\"}");
        return;
    }

    // Start the stream if enabled
    if (config.enabled) {
        if (start_stream(stream) != 0) {
            log_warn("Failed to start stream: %s", config.name);
            // Continue anyway, the stream is added
        } else {
            // Start recording if record flag is set
            if (config.record) {
                if (start_hls_stream(config.name) == 0) {
                    log_info("Recording started for stream: %s", config.name);
                } else {
                    log_warn("Failed to start recording for stream: %s", config.name);
                }
            }
        }
    }

    // Add the stream to local_config.streams
    bool stream_added_to_config = false;
    for (int i = 0; i < local_config.max_streams; i++) {
        if (local_config.streams[i].name[0] == '\0') {
            // Found an empty slot
            memcpy(&local_config.streams[i], &config, sizeof(stream_config_t));
            log_info("Added stream '%s' to configuration at index %d", config.name, i);
            stream_added_to_config = true;
            break;
        }
    }

    if (!stream_added_to_config) {
        log_warn("Couldn't find empty slot in config for stream '%s'", config.name);
    }

    // Save configuration to ensure the new stream is persisted
    if (save_config(&local_config, "/etc/lightnvr/lightnvr.conf") != 0) {
        log_warn("Failed to save configuration after adding stream");
        // Continue anyway, the stream is added
    }

    // Create success response
    char response_json[256];
    snprintf(response_json, sizeof(response_json), "{\"success\": true, \"name\": \"%s\"}", config.name);
    create_json_response(response, 201, response_json);

    log_info("Stream added: %s", config.name);
}

/**
 * Handle PUT request to update a stream
 */
void handle_put_stream(const http_request_t *request, http_response_t *response) {
    // Copy current config settings from global config
    memcpy(&local_config, get_current_config(), sizeof(config_t));

    // Extract stream name from the URL
    // URL format: /api/streams/{stream_name}
    const char *stream_name = strrchr(request->path, '/');
    if (!stream_name || strlen(stream_name) <= 1) {
        create_json_response(response, 400, "{\"error\": \"Invalid stream name\"}");
        return;
    }

    // Skip the '/'
    stream_name++;

    // Find the stream
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        create_json_response(response, 404, "{\"error\": \"Stream not found\"}");
        return;
    }

    // Ensure we have a request body
    if (!request->body || request->content_length == 0) {
        create_json_response(response, 400, "{\"error\": \"Empty request body\"}");
        return;
    }

    // Make a null-terminated copy of the request body
    char *json = malloc(request->content_length + 1);
    if (!json) {
        create_json_response(response, 500, "{\"error\": \"Memory allocation failed\"}");
        return;
    }

    memcpy(json, request->body, request->content_length);
    json[request->content_length] = '\0';

    // Parse JSON into stream configuration
    stream_config_t config;
    if (parse_stream_json(json, &config) != 0) {
        free(json);
        create_json_response(response, 400, "{\"error\": \"Invalid stream configuration\"}");
        return;
    }

    free(json);

    // Ensure the stream name in the URL matches the one in the body
    if (strcmp(stream_name, config.name) != 0) {
        create_json_response(response, 400, "{\"error\": \"Stream name in URL does not match the one in the body\"}");
        return;
    }

    // Get current stream status
    stream_status_t current_status = get_stream_status(stream);

    // Stop the stream if it's running
    if (current_status == STREAM_STATUS_RUNNING || current_status == STREAM_STATUS_STARTING) {
        if (stop_stream(stream) != 0) {
            log_warn("Failed to stop stream: %s", config.name);
            // Continue anyway, we'll try to update
        }
    }

    // Update the stream configuration
    if (update_stream_config(stream, &config) != 0) {
        create_json_response(response, 500, "{\"error\": \"Failed to update stream configuration\"}");
        return;
    }

    // Start the stream if enabled
    if (config.enabled) {
        if (start_stream(stream) != 0) {
            log_warn("Failed to start stream: %s", config.name);
            // Continue anyway, the configuration is updated
        } else {
            // Start recording if record flag is set
            if (config.record) {
                if (start_hls_stream(config.name) == 0) {
                    log_info("Recording started for stream: %s", config.name);
                } else {
                    log_warn("Failed to start recording for stream: %s", config.name);
                }
            }
        }
    }

    // Update the stream in local_config.streams
    bool stream_updated_in_config = false;
    for (int i = 0; i < local_config.max_streams; i++) {
        if (strcmp(local_config.streams[i].name, stream_name) == 0) {
            // Found the stream in config
            memcpy(&local_config.streams[i], &config, sizeof(stream_config_t));
            log_info("Updated stream '%s' in configuration at index %d", config.name, i);
            stream_updated_in_config = true;
            break;
        }
    }

    if (!stream_updated_in_config) {
        // If we didn't find the stream in config (shouldn't happen normally), try to add it
        for (int i = 0; i < local_config.max_streams; i++) {
            if (local_config.streams[i].name[0] == '\0') {
                memcpy(&local_config.streams[i], &config, sizeof(stream_config_t));
                log_info("Added missing stream '%s' to configuration at index %d", config.name, i);
                stream_updated_in_config = true;
                break;
            }
        }

        if (!stream_updated_in_config) {
            log_warn("Couldn't find stream '%s' or empty slot in config", stream_name);
        }
    }
    
    // Save configuration to ensure the changes are persisted
    if (save_config(&local_config, "/etc/lightnvr/lightnvr.conf") != 0) {
        log_warn("Failed to save configuration after updating stream");
        // Continue anyway, the stream is updated
    }
    
    // Create success response
    char response_json[256];
    snprintf(response_json, sizeof(response_json), "{\"success\": true, \"name\": \"%s\"}", config.name);
    create_json_response(response, 200, response_json);
    
    log_info("Stream updated: %s", config.name);
}

/**
 * Handle DELETE request to remove a stream
 */
void handle_delete_stream(const http_request_t *request, http_response_t *response) {
    // Copy current config settings from global config
    memcpy(&local_config, get_current_config(), sizeof(config_t));

    // Extract stream name from the URL
    // URL format: /api/streams/{stream_name}
    const char *stream_name = strrchr(request->path, '/');
    if (!stream_name || strlen(stream_name) <= 1) {
        create_json_response(response, 400, "{\"error\": \"Invalid stream name\"}");
        return;
    }

    // Skip the '/'
    stream_name++;

    // Find the stream
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        create_json_response(response, 404, "{\"error\": \"Stream not found\"}");
        return;
    }

    // Stop and remove the stream
    if (stop_stream(stream) != 0) {
        log_warn("Failed to stop stream: %s", stream_name);
        // Continue anyway, we'll try to remove it
    }

    if (remove_stream(stream) != 0) {
        create_json_response(response, 500, "{\"error\": \"Failed to remove stream\"}");
        return;
    }

    // Remove the stream from local_config.streams
    bool stream_removed_from_config = false;
    for (int i = 0; i < local_config.max_streams; i++) {
        if (strcmp(local_config.streams[i].name, stream_name) == 0) {
            // Found the stream in config, clear it
            memset(&local_config.streams[i], 0, sizeof(stream_config_t));
            log_info("Removed stream '%s' from configuration at index %d", stream_name, i);
            stream_removed_from_config = true;
            break;
        }
    }

    if (!stream_removed_from_config) {
        log_warn("Couldn't find stream '%s' in config to remove", stream_name);
    }
    
    // Save configuration to ensure the changes are persisted
    if (save_config(&local_config, "/etc/lightnvr/lightnvr.conf") != 0) {
        log_warn("Failed to save configuration after removing stream");
        // Continue anyway, the stream is removed
    }
    
    // Create success response
    char response_json[256];
    snprintf(response_json, sizeof(response_json), "{\"success\": true, \"name\": \"%s\"}", stream_name);
    create_json_response(response, 200, response_json);
    
    log_info("Stream removed: %s", stream_name);
}

/**
 * Handle POST request to test a stream connection
 */
void handle_test_stream(const http_request_t *request, http_response_t *response) {
    // Ensure we have a request body
    if (!request->body || request->content_length == 0) {
        create_json_response(response, 400, "{\"error\": \"Empty request body\"}");
        return;
    }
    
    // Make a null-terminated copy of the request body
    char *json = malloc(request->content_length + 1);
    if (!json) {
        create_json_response(response, 500, "{\"error\": \"Memory allocation failed\"}");
        return;
    }
    
    memcpy(json, request->body, request->content_length);
    json[request->content_length] = '\0';
    
    // Parse the URL from the JSON
    char *url = get_json_string_value(json, "url");
    if (!url) {
        free(json);
        create_json_response(response, 400, "{\"error\": \"URL not provided\"}");
        return;
    }
    
    free(json);
    
    // In a real implementation, here we would attempt to connect to the stream and verify it works
    // For now, we'll just simulate a successful connection
    // In a more complete implementation, you'd use libavformat/ffmpeg to test the connection
    
    log_info("Testing stream connection: %s", url);
    
    // Simulate success
    char response_json[512];
    snprintf(response_json, sizeof(response_json), 
             "{\"success\": true, \"url\": \"%s\", \"details\": {\"codec\": \"h264\", \"width\": 1280, \"height\": 720, \"fps\": 30}}", 
             url);
    
    create_json_response(response, 200, response_json);
    
    free(url);
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

/**
 * Create a JSON string for stream configuration
 */
static char* create_stream_json(const stream_config_t *stream) {
    if (!stream) return NULL;
    
    char *json = malloc(1024);
    if (!json) return NULL;
    
    snprintf(json, 1024,
             "{"
             "\"name\": \"%s\","
             "\"url\": \"%s\","
             "\"enabled\": %s,"
             "\"width\": %d,"
             "\"height\": %d,"
             "\"fps\": %d,"
             "\"codec\": \"%s\","
             "\"priority\": %d,"
             "\"record\": %s,"
             "\"segment_duration\": %d,"
             "\"status\": \"%s\""
             "}",
             stream->name,
             stream->url,
             stream->enabled ? "true" : "false",
             stream->width,
             stream->height,
             stream->fps,
             stream->codec,
             stream->priority,
             stream->record ? "true" : "false",
             stream->segment_duration,
             "Running"); // In a real implementation, we would get the actual status
    
    return json;
}

/**
 * Create a JSON array of all streams
 */
static char* create_streams_json_array() {
    // Copy current config settings from global config
    memcpy(&local_config, get_current_config(), sizeof(config_t));
    
    // Determine total size needed
    int count = get_total_stream_count();
    if (count == 0) {
        // Return empty array
        char *json = malloc(32);
        if (!json) return NULL;
        strcpy(json, "[]");
        return json;
    }
    
    // Allocate buffer for JSON array with estimated size
    char *json = malloc(1024 * count + 32);
    if (!json) return NULL;
    
    strcpy(json, "[");
    int pos = 1;
    
    // Iterate through all streams
    for (int i = 0; i < local_config.max_streams; i++) {
        stream_handle_t stream = get_stream_by_index(i);
        if (!stream) continue;
        
        stream_config_t config;
        if (get_stream_config(stream, &config) != 0) continue;
        
        // Add comma if not first element
        if (pos > 1) {
            json[pos++] = ',';
        }
        
        // Create stream JSON
        char *stream_json = create_stream_json(&config);
        if (!stream_json) continue;
        
        // Append to array
        strcpy(json + pos, stream_json);
        pos += strlen(stream_json);
        
        free(stream_json);
    }
    
    // Close array
    json[pos++] = ']';
    json[pos] = '\0';
    
    return json;
}

/**
 * Parse JSON into stream configuration
 */
static int parse_stream_json(const char *json, stream_config_t *stream) {
    if (!json || !stream) return -1;
    
    memset(stream, 0, sizeof(stream_config_t));
    
    // Parse JSON to extract stream configuration
    char *name = get_json_string_value(json, "name");
    if (!name) return -1;
    
    char *url = get_json_string_value(json, "url");
    if (!url) {
        free(name);
        return -1;
    }
    
    strncpy(stream->name, name, MAX_STREAM_NAME - 1);
    stream->name[MAX_STREAM_NAME - 1] = '\0';
    
    strncpy(stream->url, url, MAX_URL_LENGTH - 1);
    stream->url[MAX_URL_LENGTH - 1] = '\0';
    
    free(name);
    free(url);
    
    stream->enabled = get_json_boolean_value(json, "enabled", true);
    stream->width = get_json_integer_value(json, "width", 1280);
    stream->height = get_json_integer_value(json, "height", 720);
    stream->fps = get_json_integer_value(json, "fps", 15);
    
    char *codec = get_json_string_value(json, "codec");
    if (codec) {
        strncpy(stream->codec, codec, sizeof(stream->codec) - 1);
        stream->codec[sizeof(stream->codec) - 1] = '\0';
        free(codec);
    } else {
        strcpy(stream->codec, "h264");
    }
    
    stream->priority = get_json_integer_value(json, "priority", 5);
    stream->record = get_json_boolean_value(json, "record", true);
    stream->segment_duration = get_json_integer_value(json, "segment_duration", 900);
    
    return 0;
}


// New combined handler function
void handle_streaming_request(const http_request_t *request, http_response_t *response) {
    log_info("Streaming request received: %s", request->path);

    // Check if this is an HLS manifest request
    if (strstr(request->path, "/hls/index.m3u8")) {
        handle_hls_manifest(request, response);
        return;
    }

    // Check if this is an HLS segment request
    if (strstr(request->path, "/hls/index") && strstr(request->path, ".ts")) {
        handle_hls_segment(request, response);
        return;
    }

    // Check if this is a WebRTC offer request
    if (strstr(request->path, "/webrtc/offer")) {
        handle_webrtc_offer(request, response);
        return;
    }

    // Check if this is a WebRTC ICE request
    if (strstr(request->path, "/webrtc/ice")) {
        handle_webrtc_ice(request, response);
        return;
    }

    // If we get here, it's an unknown streaming request
    create_stream_error_response(response, 404, "Unknown streaming request");
}

/**
 * Handle GET request for recordings
 */
void handle_get_recordings(const http_request_t *request, http_response_t *response) {
    // Get query parameters
    char date_str[32] = {0};
    char stream_name[MAX_STREAM_NAME] = {0};
    time_t start_time = 0;
    time_t end_time = 0;
    
    // Get date filter if provided
    if (get_query_param(request, "date", date_str, sizeof(date_str)) == 0) {
        // Parse date string (format: YYYY-MM-DD)
        struct tm tm = {0};
        if (strptime(date_str, "%Y-%m-%d", &tm) != NULL) {
            // Set start time to beginning of day
            tm.tm_hour = 0;
            tm.tm_min = 0;
            tm.tm_sec = 0;
            start_time = mktime(&tm);
            
            // Set end time to end of day
            tm.tm_hour = 23;
            tm.tm_min = 59;
            tm.tm_sec = 59;
            end_time = mktime(&tm);
        } else {
            log_warn("Invalid date format: %s", date_str);
        }
    }
    
    // Get stream filter if provided
    get_query_param(request, "stream", stream_name, sizeof(stream_name));
    
    // If no stream name provided or "all" specified, set to NULL for all streams
    if (stream_name[0] == '\0' || strcmp(stream_name, "all") == 0) {
        stream_name[0] = '\0';
    }
    
    // Get recordings from database
    recording_metadata_t recordings[100]; // Limit to 100 recordings
    int count = get_recording_metadata(start_time, end_time, 
                                     stream_name[0] ? stream_name : NULL, 
                                     recordings, 100);
    
    if (count < 0) {
        create_json_response(response, 500, "{\"error\": \"Failed to get recordings\"}");
        return;
    }
    
    // Build JSON response
    char *json = malloc(count * 512 + 32); // Allocate enough space for all recordings
    if (!json) {
        create_json_response(response, 500, "{\"error\": \"Memory allocation failed\"}");
        return;
    }
    
    strcpy(json, "[");
    int pos = 1;
    
    for (int i = 0; i < count; i++) {
        char recording_json[512];
        
        // Format start and end times
        char start_time_str[32];
        char end_time_str[32];
        strftime(start_time_str, sizeof(start_time_str), "%Y-%m-%d %H:%M:%S", localtime(&recordings[i].start_time));
        strftime(end_time_str, sizeof(end_time_str), "%Y-%m-%d %H:%M:%S", localtime(&recordings[i].end_time));
        
        // Calculate duration in seconds
        int duration_sec = recordings[i].end_time - recordings[i].start_time;
        
        // Format duration as HH:MM:SS
        char duration_str[16];
        int hours = duration_sec / 3600;
        int minutes = (duration_sec % 3600) / 60;
        int seconds = duration_sec % 60;
        snprintf(duration_str, sizeof(duration_str), "%02d:%02d:%02d", hours, minutes, seconds);
        
        // Format size in human-readable format
        char size_str[16];
        if (recordings[i].size_bytes < 1024) {
            snprintf(size_str, sizeof(size_str), "%llu B", (unsigned long long)recordings[i].size_bytes);
        } else if (recordings[i].size_bytes < 1024 * 1024) {
            snprintf(size_str, sizeof(size_str), "%.1f KB", (float)recordings[i].size_bytes / 1024);
        } else if (recordings[i].size_bytes < 1024 * 1024 * 1024) {
            snprintf(size_str, sizeof(size_str), "%.1f MB", (float)recordings[i].size_bytes / (1024 * 1024));
        } else {
            snprintf(size_str, sizeof(size_str), "%.1f GB", (float)recordings[i].size_bytes / (1024 * 1024 * 1024));
        }
        
        // Create JSON for this recording
        snprintf(recording_json, sizeof(recording_json),
                 "{"
                 "\"id\": %llu,"
                 "\"stream\": \"%s\","
                 "\"start_time\": \"%s\","
                 "\"end_time\": \"%s\","
                 "\"duration\": \"%s\","
                 "\"size\": \"%s\","
                 "\"path\": \"%s\","
                 "\"width\": %d,"
                 "\"height\": %d,"
                 "\"fps\": %d,"
                 "\"codec\": \"%s\","
                 "\"complete\": %s"
                 "}",
                 (unsigned long long)recordings[i].id,
                 recordings[i].stream_name,
                 start_time_str,
                 end_time_str,
                 duration_str,
                 size_str,
                 recordings[i].file_path,
                 recordings[i].width,
                 recordings[i].height,
                 recordings[i].fps,
                 recordings[i].codec,
                 recordings[i].is_complete ? "true" : "false");
        
        // Add comma if not first element
        if (i > 0) {
            json[pos++] = ',';
        }
        
        // Append to JSON array
        strcpy(json + pos, recording_json);
        pos += strlen(recording_json);
    }
    
    // Close array
    json[pos++] = ']';
    json[pos] = '\0';
    
    // Create response
    create_json_response(response, 200, json);
    
    // Free resources
    free(json);
}

/**
 * Handle GET request for a specific recording
 */
void handle_get_recording(const http_request_t *request, http_response_t *response) {
    // Extract recording ID from the URL
    // URL format: /api/recordings/{id}
    const char *id_str = strrchr(request->path, '/');
    if (!id_str || strlen(id_str) <= 1) {
        create_json_response(response, 400, "{\"error\": \"Invalid recording ID\"}");
        return;
    }
    
    // Skip the '/'
    id_str++;
    
    // Convert ID to integer
    uint64_t id = strtoull(id_str, NULL, 10);
    if (id == 0) {
        create_json_response(response, 400, "{\"error\": \"Invalid recording ID\"}");
        return;
    }
    
    // Get recording metadata from database
    recording_metadata_t metadata;
    int result = get_recording_metadata_by_id(id, &metadata);
    
    if (result != 0) {
        create_json_response(response, 404, "{\"error\": \"Recording not found\"}");
        return;
    }
    
    // Format start and end times
    char start_time_str[32];
    char end_time_str[32];
    strftime(start_time_str, sizeof(start_time_str), "%Y-%m-%d %H:%M:%S", localtime(&metadata.start_time));
    strftime(end_time_str, sizeof(end_time_str), "%Y-%m-%d %H:%M:%S", localtime(&metadata.end_time));
    
    // Calculate duration in seconds
    int duration_sec = metadata.end_time - metadata.start_time;
    
    // Format duration as HH:MM:SS
    char duration_str[16];
    int hours = duration_sec / 3600;
    int minutes = (duration_sec % 3600) / 60;
    int seconds = duration_sec % 60;
    snprintf(duration_str, sizeof(duration_str), "%02d:%02d:%02d", hours, minutes, seconds);
    
    // Format size in human-readable format
    char size_str[16];
    if (metadata.size_bytes < 1024) {
        snprintf(size_str, sizeof(size_str), "%llu B", (unsigned long long)metadata.size_bytes);
    } else if (metadata.size_bytes < 1024 * 1024) {
        snprintf(size_str, sizeof(size_str), "%.1f KB", (float)metadata.size_bytes / 1024);
    } else if (metadata.size_bytes < 1024 * 1024 * 1024) {
        snprintf(size_str, sizeof(size_str), "%.1f MB", (float)metadata.size_bytes / (1024 * 1024));
    } else {
        snprintf(size_str, sizeof(size_str), "%.1f GB", (float)metadata.size_bytes / (1024 * 1024 * 1024));
    }
    
    // Create JSON response
    char json[1024];
    snprintf(json, sizeof(json),
             "{"
             "\"id\": %llu,"
             "\"stream\": \"%s\","
             "\"start_time\": \"%s\","
             "\"end_time\": \"%s\","
             "\"duration\": \"%s\","
             "\"size\": \"%s\","
             "\"path\": \"%s\","
             "\"width\": %d,"
             "\"height\": %d,"
             "\"fps\": %d,"
             "\"codec\": \"%s\","
             "\"complete\": %s,"
             "\"url\": \"/api/recordings/%llu/download\""
             "}",
             (unsigned long long)metadata.id,
             metadata.stream_name,
             start_time_str,
             end_time_str,
             duration_str,
             size_str,
             metadata.file_path,
             metadata.width,
             metadata.height,
             metadata.fps,
             metadata.codec,
             metadata.is_complete ? "true" : "false",
             (unsigned long long)metadata.id);
    
    // Create response
    create_json_response(response, 200, json);
}

/**
 * Handle DELETE request to remove a recording
 */
void handle_delete_recording(const http_request_t *request, http_response_t *response) {
    // Extract recording ID from the URL
    // URL format: /api/recordings/{id}
    const char *id_str = strrchr(request->path, '/');
    if (!id_str || strlen(id_str) <= 1) {
        create_json_response(response, 400, "{\"error\": \"Invalid recording ID\"}");
        return;
    }
    
    // Skip the '/'
    id_str++;
    
    // Convert ID to integer
    uint64_t id = strtoull(id_str, NULL, 10);
    if (id == 0) {
        create_json_response(response, 400, "{\"error\": \"Invalid recording ID\"}");
        return;
    }
    
    log_info("Attempting to delete recording with ID: %llu", (unsigned long long)id);
    
    // Get recording metadata from database
    recording_metadata_t metadata;
    int result = get_recording_metadata_by_id(id, &metadata);
    
    if (result != 0) {
        log_error("Recording with ID %llu not found in database", (unsigned long long)id);
        create_json_response(response, 404, "{\"error\": \"Recording not found\"}");
        return;
    }
    
    log_info("Found recording in database: ID=%llu, Path=%s", (unsigned long long)id, metadata.file_path);
    
    // Check if file exists first
    struct stat st;
    if (stat(metadata.file_path, &st) != 0) {
        log_warn("Recording file not found on disk: %s (error: %s)",
                metadata.file_path, strerror(errno));

        // Check for MP4 file as an alternative
        char mp4_path[MAX_PATH_LENGTH];
        const char *dir_end = strrchr(metadata.file_path, '/');
        if (dir_end) {
            // Extract directory path
            size_t dir_len = dir_end - metadata.file_path + 1;
            strncpy(mp4_path, metadata.file_path, dir_len);
            mp4_path[dir_len] = '\0';
            strcat(mp4_path, "recording_*.mp4");

            // Try to find matching MP4 files
            glob_t glob_result;
            if (glob(mp4_path, GLOB_NOSORT, NULL, &glob_result) == 0) {
                for (size_t i = 0; i < glob_result.gl_pathc; i++) {
                    log_info("Attempting to delete alternate file: %s", glob_result.gl_pathv[i]);
                    if (unlink(glob_result.gl_pathv[i]) == 0) {
                        log_info("Successfully deleted alternate file: %s", glob_result.gl_pathv[i]);
                    }
                }
                globfree(&glob_result);
            }
        }
    }

    // Delete the recording metadata from database
    if (delete_recording_metadata(id) != 0) {
        log_error("Failed to delete recording metadata for ID: %llu", (unsigned long long)id);
        create_json_response(response, 500, "{\"error\": \"Failed to delete recording metadata\"}");
        return;
    }
    
    // Create success response
    char json[256];
    snprintf(json, sizeof(json), 
             "{\"success\": true, \"id\": %llu, \"message\": \"Recording deleted successfully\"}", 
             (unsigned long long)id);
    
    create_json_response(response, 200, json);
    
    log_info("Recording deleted successfully: ID=%llu, Path=%s", (unsigned long long)id, metadata.file_path);
}

/**
 * Serve an MP4 file with proper headers for download
 */
void serve_mp4_file(http_response_t *response, const char *file_path, const char *filename) {
    // Verify file exists and is readable
    struct stat st;
    if (stat(file_path, &st) != 0 || access(file_path, R_OK) != 0) {
        log_error("MP4 file not accessible: %s (error: %s)", file_path, strerror(errno));
        create_json_response(response, 404, "{\"error\": \"Recording file not found\"}");
        return;
    }

    // Check file size
    if (st.st_size == 0) {
        log_error("MP4 file is empty: %s", file_path);
        create_json_response(response, 500, "{\"error\": \"Recording file is empty\"}");
        return;
    }

    log_info("Serving MP4 file: %s, size: %lld bytes", file_path, (long long)st.st_size);

    // Set content type header
    set_response_header(response, "Content-Type", "video/mp4");

    // Set content length header
    char content_length[32];
    snprintf(content_length, sizeof(content_length), "%lld", (long long)st.st_size);
    set_response_header(response, "Content-Length", content_length);

    // Set disposition header for download
    char disposition[256];
    snprintf(disposition, sizeof(disposition), "attachment; filename=\"%s\"", filename);
    set_response_header(response, "Content-Disposition", disposition);

    // Set status code
    response->status_code = 200;

    // Open file for reading
    int fd = open(file_path, O_RDONLY);
    if (fd < 0) {
        log_error("Failed to open MP4 file: %s (error: %s)", file_path, strerror(errno));
        create_json_response(response, 500, "{\"error\": \"Failed to read recording file\"}");
        return;
    }

    // Allocate response body
    response->body = malloc(st.st_size);
    if (!response->body) {
        log_error("Failed to allocate memory for response body");
        close(fd);
        create_json_response(response, 500, "{\"error\": \"Server memory allocation failed\"}");
        return;
    }

    // Read file content into response body
    ssize_t bytes_read = read(fd, response->body, st.st_size);
    close(fd);

    if (bytes_read != st.st_size) {
        log_error("Failed to read complete file: %s (read %zd of %lld bytes)",
                file_path, bytes_read, (long long)st.st_size);
        free(response->body);
        create_json_response(response, 500, "{\"error\": \"Failed to read complete recording file\"}");
        return;
    }

    // Set response body length
    response->body_length = st.st_size;

    log_info("Successfully read MP4 file into response: %s (%lld bytes)",
            file_path, (long long)st.st_size);
}


/**
 * Serve a file for download with proper headers to force browser download
 */
void serve_file_for_download(http_response_t *response, const char *file_path, const char *filename, off_t file_size) {
    // Open the file for reading
    int fd = open(file_path, O_RDONLY);
    if (fd < 0) {
        log_error("Failed to open file for download: %s (error: %s)",
                file_path, strerror(errno));
        create_json_response(response, 500, "{\"error\": \"Failed to read file\"}");
        return;
    }

    // Set response status
    response->status_code = 200;

    // Set headers to force download
    set_response_header(response, "Content-Type", "application/octet-stream");

    // Set content length
    char content_length[32];
    snprintf(content_length, sizeof(content_length), "%lld", (long long)file_size);
    set_response_header(response, "Content-Length", content_length);

    // Force download with Content-Disposition
    char disposition[256];
    snprintf(disposition, sizeof(disposition), "attachment; filename=\"%s\"", filename);
    set_response_header(response, "Content-Disposition", disposition);

    // Prevent caching
    set_response_header(response, "Cache-Control", "no-cache, no-store, must-revalidate");
    set_response_header(response, "Pragma", "no-cache");
    set_response_header(response, "Expires", "0");

    log_info("Serving file for download: %s, size: %lld bytes", file_path, (long long)file_size);

    // Allocate memory for the file content
    response->body = malloc(file_size);
    if (!response->body) {
        log_error("Failed to allocate memory for file: %s (size: %lld bytes)",
                file_path, (long long)file_size);
        close(fd);
        create_json_response(response, 500, "{\"error\": \"Server memory allocation failed\"}");
        return;
    }

    // Read the file content
    ssize_t bytes_read = read(fd, response->body, file_size);
    close(fd);

    if (bytes_read != file_size) {
        log_error("Failed to read complete file: %s (read %zd of %lld bytes)",
                file_path, bytes_read, (long long)file_size);
        free(response->body);
        create_json_response(response, 500, "{\"error\": \"Failed to read complete file\"}");
        return;
    }

    // Set response body length
    response->body_length = file_size;

    log_info("File prepared for download: %s (%lld bytes)", file_path, (long long)file_size);
}


/**
 * Serve the direct file download
 */
void serve_direct_download(http_response_t *response, uint64_t id, recording_metadata_t *metadata) {
    // Determine if this is an HLS stream (m3u8)
    const char *ext = strrchr(metadata->file_path, '.');
    bool is_hls = (ext && strcasecmp(ext, ".m3u8") == 0);

    if (is_hls) {
        // Check if a direct MP4 recording already exists in the same directory
        char dir_path[256];
        const char *last_slash = strrchr(metadata->file_path, '/');
        if (last_slash) {
            size_t dir_len = last_slash - metadata->file_path;
            strncpy(dir_path, metadata->file_path, dir_len);
            dir_path[dir_len] = '\0';
        } else {
            // If no slash, use the current directory
            strcpy(dir_path, ".");
        }

        // Check for an existing MP4 file
        char mp4_path[256];
        snprintf(mp4_path, sizeof(mp4_path), "%s/recording.mp4", dir_path);

        struct stat mp4_stat;
        if (stat(mp4_path, &mp4_stat) == 0 && mp4_stat.st_size > 0) {
            // Direct MP4 exists, serve it
            log_info("Found direct MP4 recording: %s (%lld bytes)",
                   mp4_path, (long long)mp4_stat.st_size);

            // Create filename for download
            char filename[128];
            snprintf(filename, sizeof(filename), "%s_%lld.mp4",
                   metadata->stream_name, (long long)metadata->start_time);

            // Set necessary headers
            set_response_header(response, "Content-Type", "application/octet-stream");
            char content_length[32];
            snprintf(content_length, sizeof(content_length), "%lld", (long long)mp4_stat.st_size);
            set_response_header(response, "Content-Length", content_length);
            char disposition[256];
            snprintf(disposition, sizeof(disposition), "attachment; filename=\"%s\"", filename);
            set_response_header(response, "Content-Disposition", disposition);

            log_info("Serving direct MP4 recording for download: %s", mp4_path);

            // Use existing file serving mechanism
            int result = create_file_response(response, 200, mp4_path, "application/octet-stream");
            if (result != 0) {
                log_error("Failed to create file response: %s", mp4_path);
                create_json_response(response, 500, "{\"error\": \"Failed to serve recording file\"}");
                return;
            }

            log_info("Direct MP4 recording download started: ID=%llu, Path=%s, Filename=%s",
                   (unsigned long long)id, mp4_path, filename);
            return;
        }

        // No direct MP4 found, create one
        char output_path[256];
        snprintf(output_path, sizeof(output_path), "%s/download_%llu.mp4",
                dir_path, (unsigned long long)id);

        log_info("Converting HLS stream to MP4: %s -> %s", metadata->file_path, output_path);

        // Create a more robust FFmpeg command
        char ffmpeg_cmd[512];
        snprintf(ffmpeg_cmd, sizeof(ffmpeg_cmd),
                "ffmpeg -y -i %s -c copy -bsf:a aac_adtstoasc -movflags +faststart %s 2>/dev/null",
                metadata->file_path, output_path);

        log_info("Running FFmpeg command: %s", ffmpeg_cmd);

        // Execute FFmpeg command
        int cmd_result = system(ffmpeg_cmd);
        if (cmd_result != 0) {
            log_error("FFmpeg command failed with status %d", cmd_result);

            // Try alternative approach with TS files directly
            log_info("Trying alternative conversion method with TS files");
            snprintf(ffmpeg_cmd, sizeof(ffmpeg_cmd),
                    "cd %s && ffmpeg -y -pattern_type glob -i \"*.ts\" -c copy -bsf:a aac_adtstoasc -movflags +faststart %s 2>/dev/null",
                    dir_path, output_path);

            log_info("Running alternative FFmpeg command: %s", ffmpeg_cmd);

            cmd_result = system(ffmpeg_cmd);
            if (cmd_result != 0) {
                log_error("Alternative FFmpeg command failed with status %d", cmd_result);
                create_json_response(response, 500, "{\"error\": \"Failed to convert recording\"}");
                return;
            }
        }

        // Verify the output file was created and has content
        struct stat st;
        if (stat(output_path, &st) != 0 || st.st_size == 0) {
            log_error("Converted MP4 file not found or empty: %s", output_path);
            create_json_response(response, 500, "{\"error\": \"Failed to convert recording\"}");
            return;
        }

        log_info("Successfully converted HLS to MP4: %s (%lld bytes)",
                output_path, (long long)st.st_size);

        // Create filename for download
        char filename[128];
        snprintf(filename, sizeof(filename), "%s_%lld.mp4",
               metadata->stream_name, (long long)metadata->start_time);

        // Set necessary headers
        set_response_header(response, "Content-Type", "application/octet-stream");
        char content_length[32];
        snprintf(content_length, sizeof(content_length), "%lld", (long long)st.st_size);
        set_response_header(response, "Content-Length", content_length);
        char disposition[256];
        snprintf(disposition, sizeof(disposition), "attachment; filename=\"%s\"", filename);
        set_response_header(response, "Content-Disposition", disposition);

        // Serve the converted file
        int result = create_file_response(response, 200, output_path, "application/octet-stream");
        if (result != 0) {
            log_error("Failed to create file response: %s", output_path);
            create_json_response(response, 500, "{\"error\": \"Failed to serve converted MP4 file\"}");
            return;
        }

        log_info("Converted MP4 download started: ID=%llu, Path=%s, Filename=%s",
               (unsigned long long)id, output_path, filename);
    } else {
        // For non-HLS files, serve directly
        // Create filename for download
        char filename[128];
        snprintf(filename, sizeof(filename), "%s_%lld%s",
               metadata->stream_name, (long long)metadata->start_time,
               ext ? ext : ".mp4");

        // Get file size
        struct stat st;
        if (stat(metadata->file_path, &st) != 0) {
            log_error("Failed to stat file: %s", metadata->file_path);
            create_json_response(response, 500, "{\"error\": \"Failed to access recording file\"}");
            return;
        }

        // Set necessary headers
        set_response_header(response, "Content-Type", "application/octet-stream");
        char content_length[32];
        snprintf(content_length, sizeof(content_length), "%lld", (long long)st.st_size);
        set_response_header(response, "Content-Length", content_length);
        char disposition[256];
        snprintf(disposition, sizeof(disposition), "attachment; filename=\"%s\"", filename);
        set_response_header(response, "Content-Disposition", disposition);

        // Serve the file
        int result = create_file_response(response, 200, metadata->file_path, "application/octet-stream");
        if (result != 0) {
            log_error("Failed to create file response: %s", metadata->file_path);
            create_json_response(response, 500, "{\"error\": \"Failed to serve recording file\"}");
            return;
        }

        log_info("Original file download started: ID=%llu, Path=%s, Filename=%s",
               (unsigned long long)id, metadata->file_path, filename);
    }
}


/**
 * Generate a download page with a download link
 */
void generate_download_page(http_response_t *response, uint64_t id, recording_metadata_t *metadata) {
    // Create download URL
    char download_url[256];
    snprintf(download_url, sizeof(download_url), "/api/recordings/download/%llu?direct=1",
             (unsigned long long)id);

    // Determine if this is an HLS stream (m3u8)
    const char *ext = strrchr(metadata->file_path, '.');
    bool is_hls = (ext && strcasecmp(ext, ".m3u8") == 0);

    // Format timestamp for display
    char timestamp[32];
    time_t time = (time_t)metadata->start_time;
    struct tm *tm_info = localtime(&time);
    strftime(timestamp, sizeof(timestamp), "%Y-%m-%d %H:%M:%S", tm_info);

    // Create HTML page with download link
    char html[2048];
    snprintf(html, sizeof(html),
            "<!DOCTYPE html>\n"
            "<html>\n"
            "<head>\n"
            "    <title>Download Recording</title>\n"
            "    <style>\n"
            "        body { font-family: Arial, sans-serif; margin: 40px; }\n"
            "        .container { max-width: 800px; margin: 0 auto; padding: 20px; border: 1px solid #ddd; border-radius: 5px; }\n"
            "        h1 { color: #333; }\n"
            "        .info { margin: 20px 0; }\n"
            "        .download-btn { display: inline-block; background-color: #4CAF50; color: white; padding: 10px 20px; text-decoration: none; border-radius: 4px; font-weight: bold; }\n"
            "        .download-btn:hover { background-color: #45a049; }\n"
            "    </style>\n"
            "</head>\n"
            "<body>\n"
            "    <div class=\"container\">\n"
            "        <h1>Download Recording</h1>\n"
            "        <div class=\"info\">\n"
            "            <p><strong>Stream:</strong> %s</p>\n"
            "            <p><strong>Recording ID:</strong> %llu</p>\n"
            "            <p><strong>Timestamp:</strong> %s</p>\n"
            "            <p><strong>Type:</strong> %s</p>\n"
            "        </div>\n"
            "        <p>Click the button below to download the recording:</p>\n"
            "        <a href=\"%s\" class=\"download-btn\" download>Download Recording</a>\n"
            "    </div>\n"
            "</body>\n"
            "</html>",
            metadata->stream_name,
            (unsigned long long)id,
            timestamp,
            is_hls ? "HLS Stream (will be converted to MP4)" : "Direct Recording",
            download_url);

    // Set response
    response->status_code = 200;
    response->body = strdup(html);
    response->body_length = strlen(html);

    // Set content type
    set_response_header(response, "Content-Type", "text/html");

    log_info("Generated download page for recording ID %llu", (unsigned long long)id);
}


/**
 * Serve a file for download with proper headers
 */
void serve_download_file(http_response_t *response, const char *file_path, const char *content_type,
                       const char *stream_name, time_t timestamp) {
    // Verify file exists and is readable
    struct stat st;
    if (stat(file_path, &st) != 0) {
        log_error("File not found: %s", file_path);
        create_json_response(response, 404, "{\"error\": \"Recording file not found\"}");
        return;
    }

    if (access(file_path, R_OK) != 0) {
        log_error("File not readable: %s", file_path);
        create_json_response(response, 403, "{\"error\": \"Recording file not readable\"}");
        return;
    }

    if (st.st_size == 0) {
        log_error("File is empty: %s", file_path);
        create_json_response(response, 500, "{\"error\": \"Recording file is empty\"}");
        return;
    }

    // Generate filename for download
    char filename[128];
    char *file_ext = strrchr(file_path, '.');
    if (!file_ext) {
        // Default to .mp4 if no extension
        file_ext = ".mp4";
    }

    snprintf(filename, sizeof(filename), "%s_%lld%s",
           stream_name, (long long)timestamp, file_ext);

    // Set response headers for download
    set_response_header(response, "Content-Type", "application/octet-stream");

    // Set Content-Disposition to force download
    char disposition[256];
    snprintf(disposition, sizeof(disposition), "attachment; filename=\"%s\"", filename);
    set_response_header(response, "Content-Disposition", disposition);

    // Set Cache-Control
    set_response_header(response, "Cache-Control", "no-cache, no-store, must-revalidate");
    set_response_header(response, "Pragma", "no-cache");
    set_response_header(response, "Expires", "0");

    // Log the download attempt
    log_info("Serving file for download: %s, size: %lld bytes, type: %s",
           file_path, (long long)st.st_size, content_type);

    // Serve the file - use your existing file serving function
    int result = create_file_response(response, 200, file_path, "application/octet-stream");
    if (result != 0) {
        log_error("Failed to serve file: %s", file_path);
        create_json_response(response, 500, "{\"error\": \"Failed to serve recording file\"}");
        return;
    }

    log_info("Download started: Path=%s, Filename=%s", file_path, filename);
}

/**
 * Schedule a file for deletion after it has been served
 * This function should be customized based on your application's architecture
 */
void schedule_file_deletion(const char *file_path) {
    // Simple implementation: create a background task to delete the file after a delay
    char delete_cmd[512];

    // Wait 5 minutes before deleting to ensure the file has been fully downloaded
    snprintf(delete_cmd, sizeof(delete_cmd),
            "(sleep 300 && rm -f %s) > /dev/null 2>&1 &",
            file_path);

    system(delete_cmd);
    log_info("Scheduled temporary file for deletion: %s", file_path);
}
/**
 * Serve a video file with support for HTTP range requests (essential for video streaming)
 */
void serve_video_file(http_response_t *response, const char *file_path, const char *content_type,
                      const char *filename, const http_request_t *request) {
    // Verify file exists and is readable
    struct stat st;
    if (stat(file_path, &st) != 0) {
        log_error("Video file not found: %s (error: %s)", file_path, strerror(errno));
        create_json_response(response, 404, "{\"error\": \"Video file not found\"}");
        return;
    }

    if (access(file_path, R_OK) != 0) {
        log_error("Video file not readable: %s (error: %s)", file_path, strerror(errno));
        create_json_response(response, 403, "{\"error\": \"Video file not accessible\"}");
        return;
    }

    // Check file size
    if (st.st_size == 0) {
        log_error("Video file is empty: %s", file_path);
        create_json_response(response, 500, "{\"error\": \"Video file is empty\"}");
        return;
    }

    log_info("Serving video file: %s, size: %lld bytes", file_path, (long long)st.st_size);

    // Open file
    int fd = open(file_path, O_RDONLY);
    if (fd < 0) {
        log_error("Failed to open video file: %s (error: %s)", file_path, strerror(errno));
        create_json_response(response, 500, "{\"error\": \"Failed to open video file\"}");
        return;
    }

    // Parse Range header if present
    const char *range_header = get_request_header(request, "Range");
    off_t start_pos = 0;
    off_t end_pos = st.st_size - 1;
    bool is_range_request = false;

    if (range_header && strncmp(range_header, "bytes=", 6) == 0) {
        is_range_request = true;
        // Parse range header - format is typically "bytes=start-end"
        char range_value[256];
        strncpy(range_value, range_header + 6, sizeof(range_value) - 1);
        range_value[sizeof(range_value) - 1] = '\0';

        char *dash = strchr(range_value, '-');
        if (dash) {
            *dash = '\0'; // Split the string at dash

            // Parse start position
            if (range_value[0] != '\0') {
                start_pos = atoll(range_value);
                // Ensure start_pos is within file bounds
                if (start_pos >= st.st_size) {
                    start_pos = 0;
                }
            }

            // Parse end position if provided
            if (dash[1] != '\0') {
                end_pos = atoll(dash + 1);
                // Ensure end_pos is within file bounds
                if (end_pos >= st.st_size) {
                    end_pos = st.st_size - 1;
                }
            }

            // Sanity check
            if (start_pos > end_pos) {
                start_pos = 0;
                end_pos = st.st_size - 1;
                is_range_request = false;
            }
        }
    }

    // Allocate our chunk buffer (for reading from file)
    size_t content_length = end_pos - start_pos + 1;
    size_t chunk_size = 64 * 1024; // 64KB chunks for streaming
    char *buffer = malloc(chunk_size);

    if (!buffer) {
        log_error("Failed to allocate buffer for streaming file");
        close(fd);
        create_json_response(response, 500, "{\"error\": \"Server memory allocation failed\"}");
        return;
    }

    // Set content type header
    set_response_header(response, "Content-Type", content_type);

    // Set content length header
    char content_length_str[32];
    snprintf(content_length_str, sizeof(content_length_str), "%zu", content_length);
    set_response_header(response, "Content-Length", content_length_str);

    // Set Accept-Ranges header to indicate range support
    set_response_header(response, "Accept-Ranges", "bytes");

    // Set disposition header for download if requested
    if (filename) {
        char disposition[256];
        snprintf(disposition, sizeof(disposition), "attachment; filename=\"%s\"", filename);
        set_response_header(response, "Content-Disposition", disposition);
    }

    // For range requests, set status code and Content-Range header
    if (is_range_request) {
        response->status_code = 206; // Partial Content

        char content_range[64];
        snprintf(content_range, sizeof(content_range), "bytes %lld-%lld/%lld",
                 (long long)start_pos, (long long)end_pos, (long long)st.st_size);
        set_response_header(response, "Content-Range", content_range);

        log_info("Serving range request: %s", content_range);
    } else {
        response->status_code = 200; // OK
    }

    // Seek to start position
    if (lseek(fd, start_pos, SEEK_SET) == -1) {
        log_error("Failed to seek in file: %s (error: %s)", file_path, strerror(errno));
        free(buffer);
        close(fd);
        create_json_response(response, 500, "{\"error\": \"Failed to read from video file\"}");
        return;
    }

    // Allocate response body to exact content length
    response->body = malloc(content_length);
    if (!response->body) {
        log_error("Failed to allocate response body of size %zu bytes", content_length);
        free(buffer);
        close(fd);
        create_json_response(response, 500, "{\"error\": \"Server memory allocation failed\"}");
        return;
    }

    // Read the file in chunks and fill response body
    size_t total_read = 0;
    size_t bytes_left = content_length;
    char *write_ptr = response->body;

    while (bytes_left > 0) {
        size_t to_read = bytes_left < chunk_size ? bytes_left : chunk_size;
        ssize_t bytes_read = read(fd, buffer, to_read);

        if (bytes_read <= 0) {
            if (bytes_read == 0) {
                // End of file reached prematurely
                log_warn("Unexpected EOF in file: %s (read %zu of %zu bytes)",
                      file_path, total_read, content_length);
                break;
            } else {
                // Read error
                log_error("Error reading file: %s (error: %s)", file_path, strerror(errno));
                free(buffer);
                free(response->body);
                response->body = NULL;
                close(fd);
                create_json_response(response, 500, "{\"error\": \"Failed to read from video file\"}");
                return;
            }
        }

        // Copy this chunk to response body
        memcpy(write_ptr, buffer, bytes_read);
        write_ptr += bytes_read;
        total_read += bytes_read;
        bytes_left -= bytes_read;
    }

    // If we didn't read the expected amount, resize the response body
    if (total_read != content_length) {
        log_warn("Partial read: expected %zu bytes, read %zu bytes", content_length, total_read);
        void *resized = realloc(response->body, total_read);
        if (resized) {
            response->body = resized;
        }

        // Update content length header
        snprintf(content_length_str, sizeof(content_length_str), "%zu", total_read);
        set_response_header(response, "Content-Length", content_length_str);
    }

    response->body_length = total_read;

    // Clean up
    free(buffer);
    close(fd);

    log_info("Successfully prepared video file for response: %s (%zu of %zu bytes)",
            file_path, total_read, content_length);
}

/**
 * Handle download request for a recording in a way that supports video streaming
 */
void handle_download_recording(const http_request_t *request, http_response_t *response) {
    // Extract recording ID from the URL
    // URL format: /api/recordings/download/{id}
    const char *path = request->path;
    const char *prefix = "/api/recordings/download/";

    // Log the request path for debugging
    log_debug("Download recording request path: %s", path);

    // Check if path starts with the expected prefix
    if (strncmp(path, prefix, strlen(prefix)) != 0) {
        log_error("Invalid request path prefix: %s", path);
        create_json_response(response, 400, "{\"error\": \"Invalid request path\"}");
        return;
    }

    // Extract the ID part (everything after the prefix)
    const char *id_start = path + strlen(prefix);

    // Find query string if present
    const char *query_start = strchr(id_start, '?');
    size_t id_len = query_start ? (query_start - id_start) : strlen(id_start);

    // Validate ID length
    if (id_len == 0 || id_len >= 32) {
        log_error("Invalid ID length: %zu", id_len);
        create_json_response(response, 400, "{\"error\": \"Invalid recording ID format\"}");
        return;
    }

    // Copy ID string
    char id_str[32];
    strncpy(id_str, id_start, id_len);
    id_str[id_len] = '\0';

    // Convert ID to integer
    uint64_t id = strtoull(id_str, NULL, 10);
    if (id == 0) {
        log_error("Failed to parse recording ID: %s", id_str);
        create_json_response(response, 400, "{\"error\": \"Invalid recording ID\"}");
        return;
    }

    log_info("Attempting to download recording with ID: %llu", (unsigned long long)id);

    // Get recording metadata from database
    recording_metadata_t metadata;
    int result = get_recording_metadata_by_id(id, &metadata);

    if (result != 0) {
        log_error("Recording with ID %llu not found in database", (unsigned long long)id);
        create_json_response(response, 404, "{\"error\": \"Recording not found\"}");
        return;
    }

    log_info("Found recording in database: ID=%llu, Path=%s",
            (unsigned long long)id, metadata.file_path);

    // Determine if this is an HLS stream (m3u8)
    const char *ext = strrchr(metadata.file_path, '.');
    bool is_hls = (ext && strcasecmp(ext, ".m3u8") == 0);

    // Get query parameters
    bool direct_download = false;
    char param_value[8];
    if (get_query_param(request, "direct", param_value, sizeof(param_value)) == 0) {
        direct_download = (strcmp(param_value, "1") == 0 ||
                          strcasecmp(param_value, "true") == 0);
    }

    if (is_hls) {
        // Get the directory containing the recording
        char dir_path[256];
        const char *last_slash = strrchr(metadata.file_path, '/');
        if (last_slash) {
            size_t dir_len = last_slash - metadata.file_path;
            strncpy(dir_path, metadata.file_path, dir_len);
            dir_path[dir_len] = '\0';
        } else {
            // If no slash, use the current directory
            strcpy(dir_path, ".");
        }

        // Check for the direct MP4 recording
        char mp4_path[256];
        snprintf(mp4_path, sizeof(mp4_path), "%s/recording.mp4", dir_path);

        struct stat mp4_stat;
        if (stat(mp4_path, &mp4_stat) == 0 && mp4_stat.st_size > 0) {
            // Direct MP4 exists, serve it
            log_info("Found direct MP4 recording: %s (%lld bytes)",
                   mp4_path, (long long)mp4_stat.st_size);

            // Create filename for download
            char filename[128];
            snprintf(filename, sizeof(filename), "%s_%lld.mp4",
                   metadata.stream_name, (long long)metadata.start_time);

            // Serve the file
            serve_video_file(response, mp4_path, "video/mp4",
                             direct_download ? filename : NULL, request);
            return;
        }

        // No direct MP4 available - convert the HLS stream
        char output_path[256];
        snprintf(output_path, sizeof(output_path), "/tmp/lightnvr_download_%llu.mp4",
                (unsigned long long)id);

        log_info("No direct MP4 found. Converting HLS to MP4: %s -> %s",
                metadata.file_path, output_path);

        // Check if the file already exists (from a previous conversion)
        struct stat output_stat;
        bool conversion_needed = true;

        if (stat(output_path, &output_stat) == 0 && output_stat.st_size > 0) {
            log_info("Found existing converted MP4: %s (%lld bytes)",
                   output_path, (long long)output_stat.st_size);
            conversion_needed = false;
        }

        if (conversion_needed) {
            // Create FFmpeg command to convert HLS to MP4
            char ffmpeg_cmd[512];
            snprintf(ffmpeg_cmd, sizeof(ffmpeg_cmd),
                    "ffmpeg -y -i %s -c copy -bsf:a aac_adtstoasc -movflags +faststart %s > /dev/null 2>&1",
                    metadata.file_path, output_path);

            log_info("Running FFmpeg command: %s", ffmpeg_cmd);

            int cmd_result = system(ffmpeg_cmd);
            if (cmd_result != 0) {
                log_error("FFmpeg command failed with status %d", cmd_result);

                // Try alternative approach - directly use the TS files
                char fallback_cmd[512];
                snprintf(fallback_cmd, sizeof(fallback_cmd),
                        "cd %s && cat $(ls -v index*.ts) > /tmp/combined.ts && "
                        "ffmpeg -y -i /tmp/combined.ts -c copy -bsf:a aac_adtstoasc -movflags +faststart %s "
                        " > /dev/null 2>&1 && rm /tmp/combined.ts",
                        dir_path, output_path);

                log_info("Trying alternative conversion with direct TS files: %s", fallback_cmd);

                cmd_result = system(fallback_cmd);
                if (cmd_result != 0) {
                    log_error("Alternative conversion failed with status %d", cmd_result);
                    create_json_response(response, 500, "{\"error\": \"Failed to convert recording to MP4\"}");
                    return;
                }
            }

            // Verify the MP4 file was created
            struct stat st;
            if (stat(output_path, &st) != 0 || st.st_size == 0) {
                log_error("MP4 file not found or empty after conversion: %s", output_path);
                create_json_response(response, 500, "{\"error\": \"Failed to convert recording to MP4\"}");
                return;
            }

            log_info("Successfully converted recording to MP4: %s (%lld bytes)",
                    output_path, (long long)st.st_size);
        }

        // Create filename for download
        char filename[128];
        snprintf(filename, sizeof(filename), "%s_%lld.mp4",
               metadata.stream_name, (long long)metadata.start_time);

        // Serve the converted file
        serve_video_file(response, output_path, "video/mp4",
                       direct_download ? filename : NULL, request);

        // If temporary file was created, schedule deletion after a certain period
        // We don't delete right away to allow for streaming and seeking
        if (conversion_needed) {
            // Schedule file for deletion after 24 hours
            schedule_file_deletion(output_path);
        }
    } else {
        // Not an HLS recording - serve the original file
        const char *content_type = "video/mp4"; // Default

        // Determine content type based on extension
        if (ext) {
            if (strcasecmp(ext, ".mp4") == 0) {
                content_type = "video/mp4";
            } else if (strcasecmp(ext, ".ts") == 0) {
                content_type = "video/mp2t";
            } else if (strcasecmp(ext, ".webm") == 0) {
                content_type = "video/webm";
            } else if (strcasecmp(ext, ".mkv") == 0) {
                content_type = "video/x-matroska";
            }
        }

        // Create filename for download if needed
        char filename[128] = {0};
        if (direct_download) {
            snprintf(filename, sizeof(filename), "%s_%lld%s",
                   metadata.stream_name, (long long)metadata.start_time,
                   ext ? ext : ".mp4");
        }

        // Serve the file directly
        serve_video_file(response, metadata.file_path, content_type,
                       direct_download ? filename : NULL, request);
    }

    log_info("Video serving complete for recording ID: %llu", (unsigned long long)id);
}

/**
 * Callback to remove temporary files after they've been sent
 */
void remove_temp_file_callback(void *data) {
    if (!data) return;

    char *temp_file_path = (char *)data;
    log_info("Removing temporary file: %s", temp_file_path);

    if (remove(temp_file_path) != 0) {
        log_warn("Failed to remove temporary file: %s (error: %s)",
               temp_file_path, strerror(errno));
    }

    free(temp_file_path);
}

/**
 * Handle GET request for debug database info
 */
void handle_get_debug_recordings(const http_request_t *request, http_response_t *response) {
    // Get recordings from database with no filters
    recording_metadata_t recordings[100]; // Limit to 100 recordings
    int count = get_recording_metadata(0, 0, NULL, recordings, 100);

    if (count < 0) {
        log_error("DEBUG: Failed to get recordings from database");
        create_json_response(response, 500, "{\"error\": \"Failed to get recordings\", \"count\": -1}");
        return;
    }

    // Create a detailed debug response
    char *debug_json = malloc(10000); // Allocate a large buffer
    if (!debug_json) {
        create_json_response(response, 500, "{\"error\": \"Memory allocation failed\"}");
        return;
    }

    // Start building JSON
    int pos = sprintf(debug_json,
        "{\n"
        "  \"count\": %d,\n"
        "  \"recordings\": [\n", count);

    for (int i = 0; i < count; i++) {
        // Add a comma between items (but not before the first item)
        if (i > 0) {
            pos += sprintf(debug_json + pos, ",\n");
        }

        char path_status[32] = "unknown";
        struct stat st;
        if (stat(recordings[i].file_path, &st) == 0) {
            strcpy(path_status, "exists");
        } else {
            strcpy(path_status, "missing");
        }

        pos += sprintf(debug_json + pos,
            "    {\n"
            "      \"id\": %llu,\n"
            "      \"stream\": \"%s\",\n"
            "      \"path\": \"%s\",\n"
            "      \"path_status\": \"%s\",\n"
            "      \"size\": %llu,\n"
            "      \"start_time\": %llu,\n"
            "      \"end_time\": %llu,\n"
            "      \"complete\": %s\n"
            "    }",
            (unsigned long long)recordings[i].id,
            recordings[i].stream_name,
            recordings[i].file_path,
            path_status,
            (unsigned long long)recordings[i].size_bytes,
            (unsigned long long)recordings[i].start_time,
            (unsigned long long)recordings[i].end_time,
            recordings[i].is_complete ? "true" : "false");
    }

    // Close JSON
    pos += sprintf(debug_json + pos, "\n  ]\n}");

    // Create response
    create_json_response(response, 200, debug_json);

    // Free resources
    free(debug_json);
}

/**
 * Register API handlers
 */
void register_api_handlers(void) {
    // Register settings API handlers
    register_request_handler("/api/settings", "GET", handle_get_settings);
    register_request_handler("/api/settings", "POST", handle_post_settings);

    // Register stream API handlers
    register_request_handler("/api/streams", "GET", handle_get_streams);
    register_request_handler("/api/streams", "POST", handle_post_stream);
    register_request_handler("/api/streams/test", "POST", handle_test_stream);

    // Register stream-specific API handlers (for individual streams)
    register_request_handler("/api/streams/", "GET", handle_get_stream);
    register_request_handler("/api/streams/", "PUT", handle_put_stream);
    register_request_handler("/api/streams/", "DELETE", handle_delete_stream);

    // Register system API handlers
    register_request_handler("/api/system/info", "GET", handle_get_system_info);
    register_request_handler("/api/system/logs", "GET", handle_get_system_logs);

    // Register recording API handlers
    register_request_handler("/api/recordings", "GET", handle_get_recordings);
    register_request_handler("/api/recordings/", "GET", handle_get_recording);
    register_request_handler("/api/recordings/", "DELETE", handle_delete_recording);
    register_request_handler("/api/debug/recordings", "GET", handle_get_debug_recordings);
    // Use a separate specific pattern for download endpoint
    register_request_handler("/api/recordings/download/*", "GET", handle_download_recording);

    log_info("API handlers registered");
}

void register_streaming_api_handlers(void) {
    // Register a single handler for HLS streaming at the parent path
    // This handler will parse the stream name and type from the path internally
    register_request_handler("/api/streaming/*", "GET", handle_streaming_request);

    log_info("Streaming API handlers registered");
}
