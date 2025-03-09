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
 * Improved handler for stream API endpoints that correctly handles URL-encoded identifiers
 */
void handle_get_stream(const http_request_t *request, http_response_t *response) {
    // Extract stream ID from the URL
    // URL pattern can be:
    // 1. /api/streams/123 (numeric ID)
    // 2. /api/streams/Front%20Door (URL-encoded stream name)

    const char *path = request->path;
    const char *prefix = "/api/streams/";

    // Log the request
    log_debug("Stream request path: %s", path);

    // Verify path starts with expected prefix
    if (strncmp(path, prefix, strlen(prefix)) != 0) {
        log_error("Invalid request path: %s", path);
        create_json_response(response, 400, "{\"error\": \"Invalid request path\"}");
        return;
    }

    // Extract the stream identifier (everything after the prefix)
    const char *stream_id = path + strlen(prefix);

    // Skip any leading slashes in the ID part
    while (*stream_id == '/') {
        stream_id++;
    }

    // Find query string if present and truncate
    char *stream_id_copy = strdup(stream_id);
    if (!stream_id_copy) {
        log_error("Memory allocation failed for stream ID");
        create_json_response(response, 500, "{\"error\": \"Memory allocation failed\"}");
        return;
    }

    char *query_start = strchr(stream_id_copy, '?');
    if (query_start) {
        *query_start = '\0'; // Truncate at query string
    }

    // URL-decode the stream identifier
    char decoded_id[256];
    url_decode(stream_id_copy, decoded_id, sizeof(decoded_id));
    free(stream_id_copy);

    log_info("Looking up stream with identifier: %s", decoded_id);

    // First try to find the stream by ID (if it's a number)
    stream_handle_t stream = NULL;

    // try by name
    if (!stream) {
        stream = get_stream_by_name(decoded_id);
        log_debug("Tried to find stream by name '%s', result: %s",
                 decoded_id, stream ? "found" : "not found");
    }

    // If still not found, return error
    if (!stream) {
        log_error("Stream not found: %s", decoded_id);
        create_json_response(response, 404, "{\"error\": \"Stream not found\"}");
        return;
    }

    // Get stream configuration
    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get stream configuration for: %s", decoded_id);
        create_json_response(response, 500, "{\"error\": \"Failed to get stream configuration\"}");
        return;
    }

    // Create JSON response
    char *json = create_stream_json(&config);
    if (!json) {
        log_error("Failed to create stream JSON for: %s", decoded_id);
        create_json_response(response, 500, "{\"error\": \"Failed to create stream JSON\"}");
        return;
    }

    // Create response
    create_json_response(response, 200, json);

    // Free resources
    free(json);

    log_info("Successfully served stream details for: %s", decoded_id);
}

/**
 * Handle POST request to create a new stream with improved error handling
 * and duplicate prevention
 */
void handle_post_stream(const http_request_t *request, http_response_t *response) {
    // Copy current config settings from global config
    memcpy(&local_config, get_current_config(), sizeof(config_t));

    // Ensure we have a request body
    if (!request->body || request->content_length == 0) {
        log_error("Empty request body in stream creation");
        create_json_response(response, 400, "{\"error\": \"Empty request body\"}");
        return;
    }

    // Make a null-terminated copy of the request body
    char *json = malloc(request->content_length + 1);
    if (!json) {
        log_error("Memory allocation failed for request body");
        create_json_response(response, 500, "{\"error\": \"Memory allocation failed\"}");
        return;
    }

    memcpy(json, request->body, request->content_length);
    json[request->content_length] = '\0';

    // Parse JSON into stream configuration
    stream_config_t config;
    memset(&config, 0, sizeof(stream_config_t)); // Ensure complete initialization

    if (parse_stream_json(json, &config) != 0) {
        free(json);
        log_error("Invalid stream configuration");
        create_json_response(response, 400, "{\"error\": \"Invalid stream configuration\"}");
        return;
    }

    free(json);

    // Validate stream name and URL
    if (config.name[0] == '\0') {
        log_error("Stream name cannot be empty");
        create_json_response(response, 400, "{\"error\": \"Stream name cannot be empty\"}");
        return;
    }

    if (config.url[0] == '\0') {
        log_error("Stream URL cannot be empty");
        create_json_response(response, 400, "{\"error\": \"Stream URL cannot be empty\"}");
        return;
    }

    log_info("Attempting to add stream: name='%s', url='%s'", config.name, config.url);

    // Check if stream already exists - thorough check for duplicates
    stream_handle_t existing_stream = get_stream_by_name(config.name);
    if (existing_stream != NULL) {
        log_warn("Stream with name '%s' already exists", config.name);
        create_json_response(response, 409, "{\"error\": \"Stream with this name already exists\"}");
        return;
    }

    // Check if we've reached the maximum number of streams
    int stream_count = 0;
    for (int i = 0; i < local_config.max_streams; i++) {
        if (local_config.streams[i].name[0] != '\0') {
            stream_count++;
        }
    }

    if (stream_count >= local_config.max_streams) {
        log_error("Maximum number of streams reached (%d)", local_config.max_streams);
        create_json_response(response, 507, "{\"error\": \"Maximum number of streams reached\"}");
        return;
    }

    // Find an empty slot in the configuration
    int empty_slot = -1;
    for (int i = 0; i < local_config.max_streams; i++) {
        if (local_config.streams[i].name[0] == '\0') {
            empty_slot = i;
            break;
        }
    }

    if (empty_slot == -1) {
        // This shouldn't happen if we checked stream_count correctly above
        log_error("No empty slot found in configuration despite count check");
        create_json_response(response, 500, "{\"error\": \"Internal configuration error\"}");
        return;
    }

    // Add the stream
    stream_handle_t stream = add_stream(&config);
    if (!stream) {
        log_error("Failed to add stream: %s", config.name);
        create_json_response(response, 500, "{\"error\": \"Failed to add stream\"}");
        return;
    }

    log_info("Stream added successfully: %s (index: %d)", config.name, empty_slot);

    // Now update the configuration
    memcpy(&local_config.streams[empty_slot], &config, sizeof(stream_config_t));

    // Save configuration to ensure the new stream is persisted
    if (save_config(&local_config, "/etc/lightnvr/lightnvr.conf") != 0) {
        log_warn("Failed to save configuration after adding stream");
        // We won't fail the request, but log the warning
    } else {
        log_info("Configuration saved with new stream");
    }

    // Start the stream if enabled
    if (config.enabled) {
        log_info("Starting stream: %s", config.name);
        if (start_stream(stream) != 0) {
            log_warn("Failed to start stream: %s", config.name);
            // Continue anyway, the stream is added
        } else {
            log_info("Stream started: %s", config.name);

            // Start recording if record flag is set
            if (config.record) {
                log_info("Starting recording for stream: %s", config.name);
                if (start_hls_stream(config.name) == 0) {
                    log_info("Recording started for stream: %s", config.name);
                } else {
                    log_warn("Failed to start recording for stream: %s", config.name);
                }
            }
        }
    } else {
        log_info("Stream is disabled, not starting: %s", config.name);
    }

    // Create success response
    char response_json[256];
    snprintf(response_json, sizeof(response_json),
             "{\"success\": true, \"name\": \"%s\", \"index\": %d}",
             config.name, empty_slot);
    create_json_response(response, 201, response_json);

    log_info("Stream creation completed successfully: %s", config.name);
}


/**
 * Improved handler for updating a stream that correctly handles URL-encoded identifiers
 */
void handle_put_stream(const http_request_t *request, http_response_t *response) {
    // Copy current config settings from global config
    memcpy(&local_config, get_current_config(), sizeof(config_t));

    // Extract stream identifier from the URL
    const char *path = request->path;
    const char *prefix = "/api/streams/";

    // Verify path starts with expected prefix
    if (strncmp(path, prefix, strlen(prefix)) != 0) {
        log_error("Invalid request path: %s", path);
        create_json_response(response, 400, "{\"error\": \"Invalid request path\"}");
        return;
    }

    // Extract the stream identifier (everything after the prefix)
    const char *stream_id = path + strlen(prefix);

    // Skip any leading slashes in the ID part
    while (*stream_id == '/') {
        stream_id++;
    }

    // Find query string if present and truncate
    char *stream_id_copy = strdup(stream_id);
    if (!stream_id_copy) {
        log_error("Memory allocation failed for stream ID");
        create_json_response(response, 500, "{\"error\": \"Memory allocation failed\"}");
        return;
    }

    char *query_start = strchr(stream_id_copy, '?');
    if (query_start) {
        *query_start = '\0'; // Truncate at query string
    }

    // URL-decode the stream identifier
    char decoded_id[256];
    url_decode(stream_id_copy, decoded_id, sizeof(decoded_id));
    free(stream_id_copy);

    log_info("Updating stream with identifier: %s", decoded_id);

    // Find the stream by ID or name
    stream_handle_t stream = NULL;

    // try by name
    if (!stream) {
        stream = get_stream_by_name(decoded_id);
        log_debug("Tried to find stream by name '%s', result: %s",
                 decoded_id, stream ? "found" : "not found");
    }

    // If still not found, return error
    if (!stream) {
        log_error("Stream not found: %s", decoded_id);
        create_json_response(response, 404, "{\"error\": \"Stream not found\"}");
        return;
    }

    // Ensure we have a request body
    if (!request->body || request->content_length == 0) {
        log_error("Empty request body for stream update");
        create_json_response(response, 400, "{\"error\": \"Empty request body\"}");
        return;
    }

    // Make a null-terminated copy of the request body
    char *json = malloc(request->content_length + 1);
    if (!json) {
        log_error("Memory allocation failed for request body");
        create_json_response(response, 500, "{\"error\": \"Memory allocation failed\"}");
        return;
    }

    memcpy(json, request->body, request->content_length);
    json[request->content_length] = '\0';

    // Parse JSON into stream configuration
    stream_config_t config;
    if (parse_stream_json(json, &config) != 0) {
        free(json);
        log_error("Invalid stream configuration in request body");
        create_json_response(response, 400, "{\"error\": \"Invalid stream configuration\"}");
        return;
    }

    free(json);

    // Get current stream config to check name
    stream_config_t current_config;
    if (get_stream_config(stream, &current_config) != 0) {
        log_error("Failed to get current stream configuration");
        create_json_response(response, 500, "{\"error\": \"Failed to get current stream configuration\"}");
        return;
    }

    // Special handling for name changes - log both names for clarity
    if (strcmp(current_config.name, config.name) != 0) {
        log_info("Stream name change detected: '%s' -> '%s'", current_config.name, config.name);
    }

    // Get current stream status
    stream_status_t current_status = get_stream_status(stream);

    // Stop the stream if it's running
    if (current_status == STREAM_STATUS_RUNNING || current_status == STREAM_STATUS_STARTING) {
        log_info("Stopping stream before update: %s", current_config.name);
        if (stop_stream(stream) != 0) {
            log_warn("Failed to stop stream: %s", current_config.name);
            // Continue anyway, we'll try to update
        }
    }

    // Update the stream configuration
    log_info("Updating stream configuration for: %s", current_config.name);
    if (update_stream_config(stream, &config) != 0) {
        log_error("Failed to update stream configuration");
        create_json_response(response, 500, "{\"error\": \"Failed to update stream configuration\"}");
        return;
    }

    // Start the stream if enabled
    if (config.enabled) {
        log_info("Stream is enabled, starting it: %s", config.name);
        if (start_stream(stream) != 0) {
            log_warn("Failed to start stream: %s", config.name);
            // Continue anyway, the configuration is updated
        } else {
            // Start recording if record flag is set
            if (config.record) {
                log_info("Starting recording for stream: %s", config.name);
                if (start_hls_stream(config.name) == 0) {
                    log_info("Recording started for stream: %s", config.name);
                } else {
                    log_warn("Failed to start recording for stream: %s", config.name);
                }
            }
        }
    } else {
        log_info("Stream is disabled, not starting it: %s", config.name);
    }

    // Update the stream in local_config.streams
    bool stream_updated_in_config = false;
    for (int i = 0; i < local_config.max_streams; i++) {
        if (strcmp(local_config.streams[i].name, current_config.name) == 0) {
            // Found the stream in config
            log_info("Updating stream '%s' in configuration at index %d", config.name, i);
            memcpy(&local_config.streams[i], &config, sizeof(stream_config_t));
            stream_updated_in_config = true;
            break;
        }
    }

    if (!stream_updated_in_config) {
        log_warn("Couldn't find stream '%s' in config, checking for empty slot", current_config.name);
        // If we didn't find the stream in config, try to add it
        for (int i = 0; i < local_config.max_streams; i++) {
            if (local_config.streams[i].name[0] == '\0') {
                log_info("Added missing stream '%s' to configuration at index %d", config.name, i);
                memcpy(&local_config.streams[i], &config, sizeof(stream_config_t));
                stream_updated_in_config = true;
                break;
            }
        }

        if (!stream_updated_in_config) {
            log_warn("Couldn't find stream '%s' or empty slot in configuration", current_config.name);
        }
    }

    // Save configuration to ensure the changes are persisted
    if (save_config(&local_config, "/etc/lightnvr/lightnvr.conf") != 0) {
        log_warn("Failed to save configuration after updating stream");
        // Continue anyway, the stream is updated in memory
    }

    // Create success response
    char response_json[256];
    snprintf(response_json, sizeof(response_json),
             "{\"success\": true, \"name\": \"%s\", \"id\": \"%s\"}",
             config.name, decoded_id);
    create_json_response(response, 200, response_json);

    log_info("Stream updated successfully: %s", config.name);
}

/**
 * Improved handler for deleting a stream that correctly handles URL-encoded identifiers
 */
void handle_delete_stream(const http_request_t *request, http_response_t *response) {
    // Copy current config settings from global config
    memcpy(&local_config, get_current_config(), sizeof(config_t));

    // Extract stream identifier from the URL
    const char *path = request->path;
    const char *prefix = "/api/streams/";

    // Verify path starts with expected prefix
    if (strncmp(path, prefix, strlen(prefix)) != 0) {
        log_error("Invalid request path: %s", path);
        create_json_response(response, 400, "{\"error\": \"Invalid request path\"}");
        return;
    }

    // Extract the stream identifier (everything after the prefix)
    const char *stream_id = path + strlen(prefix);

    // Skip any leading slashes in the ID part
    while (*stream_id == '/') {
        stream_id++;
    }

    // Find query string if present and truncate
    char *stream_id_copy = strdup(stream_id);
    if (!stream_id_copy) {
        log_error("Memory allocation failed for stream ID");
        create_json_response(response, 500, "{\"error\": \"Memory allocation failed\"}");
        return;
    }

    char *query_start = strchr(stream_id_copy, '?');
    if (query_start) {
        *query_start = '\0'; // Truncate at query string
    }

    // URL-decode the stream identifier
    char decoded_id[256];
    url_decode(stream_id_copy, decoded_id, sizeof(decoded_id));
    free(stream_id_copy);

    log_info("Deleting stream with identifier: %s", decoded_id);

    // Find the stream by ID or name
    stream_handle_t stream = NULL;

    // try by name
    if (!stream) {
        stream = get_stream_by_name(decoded_id);
        log_debug("Tried to find stream by name '%s', result: %s",
                 decoded_id, stream ? "found" : "not found");
    }

    // If still not found, return error
    if (!stream) {
        log_error("Stream not found: %s", decoded_id);
        create_json_response(response, 404, "{\"error\": \"Stream not found\"}");
        return;
    }

    // Get stream name for logging and config updates
    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get stream configuration for: %s", decoded_id);
        create_json_response(response, 500, "{\"error\": \"Failed to get stream configuration\"}");
        return;
    }

    // Save stream name before removal
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, config.name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    log_info("Found stream to delete: %s (name: %s)", decoded_id, stream_name);

    // Stop and remove the stream
    log_info("Stopping stream: %s", stream_name);
    if (stop_stream(stream) != 0) {
        log_warn("Failed to stop stream: %s", stream_name);
        // Continue anyway, we'll try to remove it
    }

    log_info("Removing stream: %s", stream_name);
    if (remove_stream(stream) != 0) {
        log_error("Failed to remove stream: %s", stream_name);
        create_json_response(response, 500, "{\"error\": \"Failed to remove stream\"}");
        return;
    }

    // Remove the stream from local_config.streams
    bool stream_removed_from_config = false;
    for (int i = 0; i < local_config.max_streams; i++) {
        if (strcmp(local_config.streams[i].name, stream_name) == 0) {
            // Found the stream in config, clear it
            log_info("Removing stream '%s' from configuration at index %d", stream_name, i);
            memset(&local_config.streams[i], 0, sizeof(stream_config_t));
            stream_removed_from_config = true;
            break;
        }
    }

    if (!stream_removed_from_config) {
        log_warn("Couldn't find stream '%s' in configuration to remove", stream_name);
    }

    // Save configuration to ensure the changes are persisted
    if (save_config(&local_config, "/etc/lightnvr/lightnvr.conf") != 0) {
        log_warn("Failed to save configuration after removing stream");
        // Continue anyway, the stream is removed from memory
    }

    // Create success response
    char response_json[256];
    snprintf(response_json, sizeof(response_json),
             "{\"success\": true, \"name\": \"%s\", \"id\": \"%s\"}",
             stream_name, decoded_id);
    create_json_response(response, 200, response_json);

    log_info("Stream removed successfully: %s", stream_name);
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
        // Ensure we don't overflow the codec buffer (which is 16 bytes)
        size_t codec_size = sizeof(stream->codec);
        strncpy(stream->codec, codec, codec_size - 1);
        stream->codec[codec_size - 1] = '\0';
        free(codec);
    } else {
        // Use safe string copy for default codec
        strncpy(stream->codec, "h264", sizeof(stream->codec) - 1);
        stream->codec[sizeof(stream->codec) - 1] = '\0';
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
 * Handle GET request for recordings with pagination
 */
void handle_get_recordings(const http_request_t *request, http_response_t *response) {
    // Get query parameters
    char date_str[32] = {0};
    char stream_name[MAX_STREAM_NAME] = {0};
    char page_str[16] = {0};
    char limit_str[16] = {0};
    time_t start_time = 0;
    time_t end_time = 0;
    int page = 1;
    int limit = 20;
    
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
    
    log_debug("Filtering recordings by stream: %s", stream_name[0] ? stream_name : "all streams");
    
    // Get pagination parameters if provided
    if (get_query_param(request, "page", page_str, sizeof(page_str)) == 0) {
        int parsed_page = atoi(page_str);
        if (parsed_page > 0) {
            page = parsed_page;
        }
    }
    
    if (get_query_param(request, "limit", limit_str, sizeof(limit_str)) == 0) {
        int parsed_limit = atoi(limit_str);
        if (parsed_limit > 0 && parsed_limit <= 100) {
            limit = parsed_limit;
        }
    }
    
    log_debug("Fetching recordings with pagination: page=%d, limit=%d", page, limit);
    
    // First, get total count of recordings matching the filters
    // We'll use a larger buffer to get all recordings, then count them
    recording_metadata_t all_recordings[500]; // Temporary buffer for counting
    int total_count = get_recording_metadata(start_time, end_time, 
                                          stream_name[0] ? stream_name : NULL, 
                                          all_recordings, 500);
    
    if (total_count < 0) {
        create_json_response(response, 500, "{\"error\": \"Failed to get recordings count\"}");
        return;
    }
    
    // Calculate pagination values
    int total_pages = (total_count + limit - 1) / limit; // Ceiling division
    if (total_pages == 0) total_pages = 1;
    if (page > total_pages) page = total_pages;
    
    // Calculate offset
    int offset = (page - 1) * limit;
    
    // Get paginated recordings from database
    recording_metadata_t recordings[100]; // Limit to 100 recordings max
    int actual_limit = (offset + limit <= total_count) ? limit : (total_count - offset);
    if (actual_limit <= 0) actual_limit = 0;
    
    // Copy the relevant slice from our all_recordings buffer
    int count = 0;
    for (int i = offset; i < offset + actual_limit && i < total_count; i++) {
        memcpy(&recordings[count], &all_recordings[i], sizeof(recording_metadata_t));
        count++;
    }
    
    // Build JSON response with pagination metadata
    char *json = malloc(count * 512 + 256); // Allocate enough space for all recordings + pagination metadata
    if (!json) {
        create_json_response(response, 500, "{\"error\": \"Memory allocation failed\"}");
        return;
    }
    
    // Start with pagination metadata
    int pos = sprintf(json, 
                     "{"
                     "\"pagination\": {"
                     "\"total\": %d,"
                     "\"page\": %d,"
                     "\"limit\": %d,"
                     "\"pages\": %d"
                     "},"
                     "\"recordings\": [",
                     total_count, page, limit, total_pages);
    
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
    
    // Close array and object
    pos += sprintf(json + pos, "]}");
    
    // Create response
    create_json_response(response, 200, json);
    
    // Free resources
    free(json);
    
    log_info("Served recordings page %d of %d (limit: %d, total: %d)", 
             page, total_pages, limit, total_count);
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

    // Determine directory where recording segments are stored
    char dir_path[MAX_PATH_LENGTH];
    const char *last_slash = strrchr(metadata.file_path, '/');
    if (last_slash) {
        size_t dir_len = last_slash - metadata.file_path + 1; // Include the slash
        strncpy(dir_path, metadata.file_path, dir_len);
        dir_path[dir_len] = '\0';
        log_info("Recording directory: %s", dir_path);

        // Delete all TS segment files in this directory
        char delete_cmd[MAX_PATH_LENGTH + 50];
        snprintf(delete_cmd, sizeof(delete_cmd), "rm -f %s*.ts %s*.mp4 %s*.m3u8",
                dir_path, dir_path, dir_path);
        log_info("Executing cleanup command: %s", delete_cmd);
        system(delete_cmd); // Ignore result - we'll continue with metadata deletion anyway
    }

    // Explicitly try to delete the main file
    if (access(metadata.file_path, F_OK) == 0) {
        if (unlink(metadata.file_path) != 0) {
            log_warn("Failed to delete recording file: %s (error: %s)",
                    metadata.file_path, strerror(errno));
            // Continue anyway - we'll still delete the metadata
        } else {
            log_info("Successfully deleted recording file: %s", metadata.file_path);
        }
    } else {
        log_warn("Recording file not found on disk: %s", metadata.file_path);
    }

    // Delete MP4 recordings if they exist
    char mp4_path[MAX_PATH_LENGTH];
    snprintf(mp4_path, sizeof(mp4_path), "%srecording*.mp4", dir_path);

    // Use glob to find MP4 files - this requires including glob.h
    glob_t glob_result;
    if (glob(mp4_path, GLOB_NOSORT, NULL, &glob_result) == 0) {
        for (size_t i = 0; i < glob_result.gl_pathc; i++) {
            log_info("Deleting MP4 file: %s", glob_result.gl_pathv[i]);
            if (unlink(glob_result.gl_pathv[i]) != 0) {
                log_warn("Failed to delete MP4 file: %s (error: %s)",
                       glob_result.gl_pathv[i], strerror(errno));
            }
        }
        globfree(&glob_result);
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
 * Register fixed API handlers to ensure proper URL handling
 */
void register_api_handlers(void) {
    // Register settings API handlers (unchanged)
    register_request_handler("/api/settings", "GET", handle_get_settings);
    register_request_handler("/api/settings", "POST", handle_post_settings);

    // Register stream API handlers (unchanged)
    register_request_handler("/api/streams", "GET", handle_get_streams);
    register_request_handler("/api/streams", "POST", handle_post_stream);
    register_request_handler("/api/streams/test", "POST", handle_test_stream);

    // Register improved stream-specific API handlers for individual streams
    // Use a more specific pattern that matches the exact pattern of IDs
    register_request_handler("/api/streams/*", "GET", handle_get_stream);
    register_request_handler("/api/streams/*", "PUT", handle_put_stream);
    register_request_handler("/api/streams/*", "DELETE", handle_delete_stream);

    // Register system API handlers (unchanged)
    register_request_handler("/api/system/info", "GET", handle_get_system_info);
    register_request_handler("/api/system/logs", "GET", handle_get_system_logs);

    // Register recording API handlers
    // IMPORTANT: Register more specific routes first to avoid conflicts
    register_request_handler("/api/recordings/download/*", "GET", handle_download_recording);
    register_request_handler("/api/debug/recordings", "GET", handle_get_debug_recordings);
    register_request_handler("/api/recordings", "GET", handle_get_recordings);
    
    // These must come last as they're more general patterns
    register_request_handler("/api/recordings/*", "GET", handle_get_recording);
    register_request_handler("/api/recordings/*", "DELETE", handle_delete_recording);

    log_info("API handlers registered with improved URL handling and route priority");
}

/**
 * Handle GET request to download a recording - With fixed query parameter handling
 */
void handle_download_recording(const http_request_t *request, http_response_t *response) {
    // Extract recording ID from the URL
    const char *path = request->path;
    const char *prefix = "/api/recordings/download/";

    // Verify path starts with expected prefix
    if (strncmp(path, prefix, strlen(prefix)) != 0) {
        log_error("Invalid request path: %s", path);
        create_json_response(response, 400, "{\"error\": \"Invalid request path\"}");
        return;
    }

    // Extract the recording ID (everything after the prefix)
    const char *id_str = path + strlen(prefix);

    // Skip any leading slashes in the ID part
    while (*id_str == '/') {
        id_str++;
    }

    // Find query string if present and truncate
    char *id_str_copy = strdup(id_str);
    if (!id_str_copy) {
        log_error("Memory allocation failed for recording ID");
        create_json_response(response, 500, "{\"error\": \"Memory allocation failed\"}");
        return;
    }

    char *query_start = strchr(id_str_copy, '?');
    if (query_start) {
        *query_start = '\0'; // Truncate at query string
    }

    // Convert ID to integer
    uint64_t id = strtoull(id_str_copy, NULL, 10);
    if (id == 0) {
        log_error("Invalid recording ID: %s", id_str_copy);
        free(id_str_copy);
        create_json_response(response, 400, "{\"error\": \"Invalid recording ID\"}");
        return;
    }

    free(id_str_copy); // Free this as we don't need it anymore

    // Check for force download parameter - use the request's query params directly
    bool force_download = false;

    // Get 'download' parameter directly from request's query params
    char download_param[10] = {0};
    if (get_query_param(request, "download", download_param, sizeof(download_param)) == 0) {
        // Check if the parameter is set to "1" or "true"
        if (strcmp(download_param, "1") == 0 || strcmp(download_param, "true") == 0) {
            force_download = true;
            log_info("Force download requested for recording ID %llu (via query param)", (unsigned long long)id);
        }
    }

    // Get recording metadata from database
    recording_metadata_t metadata;
    int result = get_recording_metadata_by_id(id, &metadata);

    if (result != 0) {
        log_error("Recording with ID %llu not found in database", (unsigned long long)id);
        create_json_response(response, 404, "{\"error\": \"Recording not found\"}");
        return;
    }

    log_info("Found recording in database: ID=%llu, Path=%s, Download=%s",
             (unsigned long long)id, metadata.file_path, force_download ? "true" : "false");

    // Check if the file exists
    struct stat st;
    if (stat(metadata.file_path, &st) != 0) {
        log_error("Recording file not found on disk: %s (error: %s)",
                 metadata.file_path, strerror(errno));
        create_json_response(response, 404, "{\"error\": \"Recording file not found\"}");
        return;
    }

    // Determine if this is an MP4 file
    const char *ext = strrchr(metadata.file_path, '.');
    bool is_mp4 = (ext && strcasecmp(ext, ".mp4") == 0);

    // Generate a filename for download
    char filename[128];
    if (is_mp4) {
        snprintf(filename, sizeof(filename), "%s_%lld.mp4",
                metadata.stream_name, (long long)metadata.start_time);
    } else {
        // Use whatever extension the file has, or default to .mp4
        if (ext) {
            snprintf(filename, sizeof(filename), "%s_%lld%s",
                    metadata.stream_name, (long long)metadata.start_time, ext);
        } else {
            snprintf(filename, sizeof(filename), "%s_%lld.mp4",
                    metadata.stream_name, (long long)metadata.start_time);
        }
    }

    if (is_mp4 && !force_download) {
        // For MP4 files, serve with video/mp4 content type for playback
        log_info("Serving MP4 file with video/mp4 content type for playback: %s", metadata.file_path);

        // Set content type to video/mp4 for playback
        set_response_header(response, "Content-Type", "video/mp4");

        // Set content length
        char content_length[32];
        snprintf(content_length, sizeof(content_length), "%lld", (long long)st.st_size);
        set_response_header(response, "Content-Length", content_length);

        // Create file response with video/mp4 content type
        int result = create_file_response(response, 200, metadata.file_path, "video/mp4");
        if (result != 0) {
            log_error("Failed to create file response: %s", metadata.file_path);
            create_json_response(response, 500, "{\"error\": \"Failed to serve recording file\"}");
        }
    } else if (is_mp4) {
        // For MP4 files with forced download, use the serve_mp4_file function
        log_info("Serving MP4 file with attachment disposition for download: %s", metadata.file_path);
        serve_mp4_file(response, metadata.file_path, filename);
    } else {
        // For non-MP4 files, use the direct download approach
        serve_direct_download(response, id, &metadata);
    }
}

void register_streaming_api_handlers(void) {
    // Register a single handler for HLS streaming at the parent path
    // This handler will parse the stream name and type from the path internally
    register_request_handler("/api/streaming/*", "GET", handle_streaming_request);

    log_info("Streaming API handlers registered");
}
