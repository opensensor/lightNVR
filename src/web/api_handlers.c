#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <ctype.h>
#include <time.h>

#include "web/web_server.h"
#include "web/api_handlers.h"
#include "core/config.h"
#include "core/logger.h"
#include "video/stream_manager.h"
#include "video/streams.h"

#define LIGHTNVR_VERSION_STRING "0.1.0"

// Define a local config variable to work with
static config_t local_config;

// Forward declaration of helper function to get current configuration
static config_t* get_current_config(void);

// Helper function to get current configuration
static config_t* get_current_config(void) {
    // For now, just initialize a default configuration
    // In a real implementation, this would get the actual global configuration
    load_default_config(&local_config);
    return &local_config;
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
        }
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

    log_info("API handlers registered");
}

void register_streaming_api_handlers(void) {
    // Register a single handler for HLS streaming at the parent path
    // This handler will parse the stream name and type from the path internally
    register_request_handler("/api/streaming/*", "GET", handle_streaming_request);

    log_info("Streaming API handlers registered");
}
