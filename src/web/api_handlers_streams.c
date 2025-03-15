#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <ctype.h>

#include "core/config.h"
#include "core/logger.h"
#include "database/database_manager.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "web/request_response.h"
#include "web/api_handlers_streams.h"
#include "web/api_handlers_common.h"

// Forward declarations of helper functions

// Forward declarations of helper functions
static char* create_stream_json(const stream_config_t *stream);
static char* create_streams_json_array();
static int parse_stream_json(const char *json, stream_config_t *stream);

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
    // No need to copy config settings anymore, using database

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
    int stream_count = count_stream_configs();
    if (stream_count < 0) {
        log_error("Failed to count stream configurations");
        create_json_response(response, 500, "{\"error\": \"Failed to count stream configurations\"}");
        return;
    }
    
    // Get max streams from global config
    extern config_t global_config;
    int max_streams = global_config.max_streams;
    
    if (stream_count >= max_streams) {
        log_error("Maximum number of streams reached (%d)", max_streams);
        create_json_response(response, 507, "{\"error\": \"Maximum number of streams reached\"}");
        return;
    }

    // Add the stream
    stream_handle_t stream = add_stream(&config);
    if (!stream) {
        log_error("Failed to add stream: %s", config.name);
        create_json_response(response, 500, "{\"error\": \"Failed to add stream\"}");
        return;
    }

    log_info("Stream added successfully: %s", config.name);

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
             "{\"success\": true, \"name\": \"%s\"}",
             config.name);
    create_json_response(response, 201, response_json);

    log_info("Stream creation completed successfully: %s", config.name);
}

/**
 * Improved handler for updating a stream that correctly handles URL-encoded identifiers
 */
void handle_put_stream(const http_request_t *request, http_response_t *response) {
    // No need to copy config settings anymore, using database

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
    if (update_stream_config(current_config.name, &config) != 0) {
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

    // Stream configuration is updated in the database by update_stream_config

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
    // No need to copy config settings anymore, using database

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

    // Stream configuration is removed from the database by remove_stream

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
 * Create a JSON string for stream configuration
 */
static char* create_stream_json(const stream_config_t *stream) {
    if (!stream) return NULL;
    
    // Allocate more space for the JSON to accommodate detection settings
    char *json = malloc(2048);
    if (!json) return NULL;
    
    // Start with the basic stream configuration
    int pos = snprintf(json, 2048,
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
             "\"status\": \"%s\"",
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
    
    // Add detection-based recording settings
    pos += snprintf(json + pos, 2048 - pos,
             ","
             "\"detection_based_recording\": %s",
             stream->detection_based_recording ? "true" : "false");
    
    // Only include detection model and parameters if detection-based recording is enabled
    if (stream->detection_based_recording) {
        // Convert threshold from 0.0-1.0 to percentage (0-100)
        int threshold_percent = (int)(stream->detection_threshold * 100.0f);
        
        pos += snprintf(json + pos, 2048 - pos,
                 ","
                 "\"detection_model\": \"%s\","
                 "\"detection_threshold\": %d,"
                 "\"detection_interval\": %d,"
                 "\"pre_detection_buffer\": %d,"
                 "\"post_detection_buffer\": %d",
                 stream->detection_model,
                 threshold_percent,
                 stream->detection_interval,
                 stream->pre_detection_buffer,
                 stream->post_detection_buffer);
    }
    
    // Close the JSON object
    pos += snprintf(json + pos, 2048 - pos, "}");
    
    return json;
}

/**
 * Create a JSON array of all streams
 */
static char* create_streams_json_array() {
    // Get all stream configurations from database
    stream_config_t db_streams[MAX_STREAMS];
    int count = get_all_stream_configs(db_streams, MAX_STREAMS);
    
    if (count <= 0) {
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
    for (int i = 0; i < count; i++) {
        // Add comma if not first element
        if (pos > 1) {
            json[pos++] = ',';
        }
        
        // Create stream JSON
        char *stream_json = create_stream_json(&db_streams[i]);
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

    // Parse detection-based recording options
    stream->detection_based_recording = get_json_boolean_value(json, "detection_based_recording", false);
    
    if (stream->detection_based_recording) {
        // Only parse detection options if detection-based recording is enabled
        char *detection_model = get_json_string_value(json, "detection_model");
        if (detection_model) {
            strncpy(stream->detection_model, detection_model, MAX_PATH_LENGTH - 1);
            stream->detection_model[MAX_PATH_LENGTH - 1] = '\0';
            free(detection_model);
        }
        
        // Parse detection threshold (convert from percentage to 0.0-1.0 range)
        int threshold_percent = get_json_integer_value(json, "detection_threshold", 50);
        stream->detection_threshold = (float)threshold_percent / 100.0f;
        
        // Parse detection interval
        stream->detection_interval = get_json_integer_value(json, "detection_interval", 10);
        
        // Parse pre/post detection buffers
        stream->pre_detection_buffer = get_json_integer_value(json, "pre_detection_buffer", 5);
        stream->post_detection_buffer = get_json_integer_value(json, "post_detection_buffer", 10);
        
        log_info("Detection options parsed: model=%s, threshold=%.2f, interval=%d, pre_buffer=%d, post_buffer=%d",
                stream->detection_model, stream->detection_threshold, stream->detection_interval,
                stream->pre_detection_buffer, stream->post_detection_buffer);
    }

    return 0;
}
