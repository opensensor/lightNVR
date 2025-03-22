#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <unistd.h>

#include "web/api_handlers.h"
#include "web/mongoose_adapter.h"
#include "core/logger.h"
#include "core/config.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/stream_state.h"
#include "mongoose.h"
#include "video/detection_stream.h"
#include "database/database_manager.h"
#include "video/hls/hls_directory.h"
#include "video/hls/hls_api.h"

/**
 * @brief Direct handler for POST /api/streams
 */
void mg_handle_post_stream(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/streams request");
    
    // Parse JSON from request body
    cJSON *stream_json = mg_parse_json_body(hm);
    if (!stream_json) {
        log_error("Failed to parse stream JSON from request body");
        mg_send_json_error(c, 400, "Invalid JSON in request body");
        return;
    }
    
    // Extract stream configuration
    stream_config_t config;
    memset(&config, 0, sizeof(config));
    
    // Required fields
    cJSON *name = cJSON_GetObjectItem(stream_json, "name");
    cJSON *url = cJSON_GetObjectItem(stream_json, "url");
    
    if (!name || !cJSON_IsString(name) || !url || !cJSON_IsString(url)) {
        log_error("Missing required fields in stream configuration");
        cJSON_Delete(stream_json);
        mg_send_json_error(c, 400, "Missing required fields (name, url)");
        return;
    }
    
    // Copy name and URL
    strncpy(config.name, name->valuestring, sizeof(config.name) - 1);
    strncpy(config.url, url->valuestring, sizeof(config.url) - 1);
    
    // Optional fields with defaults
    config.enabled = true;
    config.streaming_enabled = true;
    config.width = 1280;
    config.height = 720;
    config.fps = 30;
    strncpy(config.codec, "h264", sizeof(config.codec) - 1);
    config.priority = 5;
    config.record = true;
    config.segment_duration = 60;
    config.detection_based_recording = false;
    config.detection_interval = 10;
    config.detection_threshold = 0.5f;
    config.pre_detection_buffer = 5;
    config.post_detection_buffer = 5;
    config.protocol = STREAM_PROTOCOL_TCP;
    
    // Override with provided values
    cJSON *enabled = cJSON_GetObjectItem(stream_json, "enabled");
    if (enabled && cJSON_IsBool(enabled)) {
        config.enabled = cJSON_IsTrue(enabled);
    }
    
    cJSON *streaming_enabled = cJSON_GetObjectItem(stream_json, "streaming_enabled");
    if (streaming_enabled && cJSON_IsBool(streaming_enabled)) {
        config.streaming_enabled = cJSON_IsTrue(streaming_enabled);
    }
    
    cJSON *width = cJSON_GetObjectItem(stream_json, "width");
    if (width && cJSON_IsNumber(width)) {
        config.width = width->valueint;
    }
    
    cJSON *height = cJSON_GetObjectItem(stream_json, "height");
    if (height && cJSON_IsNumber(height)) {
        config.height = height->valueint;
    }
    
    cJSON *fps = cJSON_GetObjectItem(stream_json, "fps");
    if (fps && cJSON_IsNumber(fps)) {
        config.fps = fps->valueint;
    }
    
    cJSON *codec = cJSON_GetObjectItem(stream_json, "codec");
    if (codec && cJSON_IsString(codec)) {
        strncpy(config.codec, codec->valuestring, sizeof(config.codec) - 1);
    }
    
    cJSON *priority = cJSON_GetObjectItem(stream_json, "priority");
    if (priority && cJSON_IsNumber(priority)) {
        config.priority = priority->valueint;
    }
    
    cJSON *record = cJSON_GetObjectItem(stream_json, "record");
    if (record && cJSON_IsBool(record)) {
        config.record = cJSON_IsTrue(record);
    }
    
    cJSON *segment_duration = cJSON_GetObjectItem(stream_json, "segment_duration");
    if (segment_duration && cJSON_IsNumber(segment_duration)) {
        config.segment_duration = segment_duration->valueint;
    }
    
    cJSON *detection_based_recording = cJSON_GetObjectItem(stream_json, "detection_based_recording");
    if (detection_based_recording && cJSON_IsBool(detection_based_recording)) {
        config.detection_based_recording = cJSON_IsTrue(detection_based_recording);
    }
    
    cJSON *detection_model = cJSON_GetObjectItem(stream_json, "detection_model");
    if (detection_model && cJSON_IsString(detection_model)) {
        strncpy(config.detection_model, detection_model->valuestring, sizeof(config.detection_model) - 1);
    }
    
    cJSON *detection_threshold = cJSON_GetObjectItem(stream_json, "detection_threshold");
    if (detection_threshold && cJSON_IsNumber(detection_threshold)) {
        // Convert from percentage (0-100) to float (0.0-1.0)
        config.detection_threshold = detection_threshold->valuedouble / 100.0f;
    }
    
    cJSON *detection_interval = cJSON_GetObjectItem(stream_json, "detection_interval");
    if (detection_interval && cJSON_IsNumber(detection_interval)) {
        config.detection_interval = detection_interval->valueint;
    }
    
    cJSON *pre_detection_buffer = cJSON_GetObjectItem(stream_json, "pre_detection_buffer");
    if (pre_detection_buffer && cJSON_IsNumber(pre_detection_buffer)) {
        config.pre_detection_buffer = pre_detection_buffer->valueint;
    }
    
    cJSON *post_detection_buffer = cJSON_GetObjectItem(stream_json, "post_detection_buffer");
    if (post_detection_buffer && cJSON_IsNumber(post_detection_buffer)) {
        config.post_detection_buffer = post_detection_buffer->valueint;
    }
    
    cJSON *protocol = cJSON_GetObjectItem(stream_json, "protocol");
    if (protocol && cJSON_IsNumber(protocol)) {
        config.protocol = (stream_protocol_t)protocol->valueint;
    }
    
    // Clean up JSON
    cJSON_Delete(stream_json);
    
    // Check if stream already exists
    stream_handle_t existing_stream = get_stream_by_name(config.name);
    if (existing_stream) {
        log_error("Stream already exists: %s", config.name);
        mg_send_json_error(c, 409, "Stream already exists");
        return;
    }
    
    // Add stream to database
    uint64_t stream_id = add_stream_config(&config);
    if (stream_id == 0) {
        log_error("Failed to add stream configuration to database");
        mg_send_json_error(c, 500, "Failed to add stream configuration");
        return;
    }
    
    // Create stream in memory from the database configuration
    stream_handle_t stream = add_stream(&config);
    if (!stream) {
        log_error("Failed to create stream: %s", config.name);
        // Delete from database since we couldn't create it in memory
        delete_stream_config(config.name);
        mg_send_json_error(c, 500, "Failed to create stream");
        return;
    }
    
    // Start stream if enabled
    if (config.enabled) {
        if (start_stream(stream) != 0) {
            log_error("Failed to start stream: %s", config.name);
            // Continue anyway, stream is created
        }
    }
    
    // Create success response using cJSON
    cJSON *success = cJSON_CreateObject();
    if (!success) {
        log_error("Failed to create success JSON object");
        mg_send_json_error(c, 500, "Failed to create success JSON");
        return;
    }
    
    cJSON_AddBoolToObject(success, "success", true);
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(success);
    if (!json_str) {
        log_error("Failed to convert success JSON to string");
        cJSON_Delete(success);
        mg_send_json_error(c, 500, "Failed to convert success JSON to string");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 201, json_str);
    
    // Clean up
    free(json_str);
    cJSON_Delete(success);
    
    log_info("Successfully created stream: %s", config.name);
}

/**
 * @brief Direct handler for PUT /api/streams/:id
 */
void mg_handle_put_stream(struct mg_connection *c, struct mg_http_message *hm) {
    // Extract stream ID from URL
    char stream_id[MAX_STREAM_NAME];
    if (mg_extract_path_param(hm, "/api/streams/", stream_id, sizeof(stream_id)) != 0) {
        log_error("Failed to extract stream ID from URL");
        mg_send_json_error(c, 400, "Invalid request path");
        return;
    }
    
    // URL-decode the stream identifier
    char decoded_id[MAX_STREAM_NAME];
    mg_url_decode(stream_id, strlen(stream_id), decoded_id, sizeof(decoded_id), 0);
    
    log_info("Handling PUT /api/streams/%s request", decoded_id);
    
    // Find the stream by name
    stream_handle_t stream = get_stream_by_name(decoded_id);
    if (!stream) {
        log_error("Stream not found: %s", decoded_id);
        mg_send_json_error(c, 404, "Stream not found");
        return;
    }
    
    // Get current stream configuration
    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get stream configuration for: %s", decoded_id);
        mg_send_json_error(c, 500, "Failed to get stream configuration");
        return;
    }
    
    // Parse JSON from request body
    cJSON *stream_json = mg_parse_json_body(hm);
    if (!stream_json) {
        log_error("Failed to parse stream JSON from request body");
        mg_send_json_error(c, 400, "Invalid JSON in request body");
        return;
    }
    
    // Update configuration with provided values
    bool config_changed = false;
    bool requires_restart = false;  // Flag for changes that require stream restart
    
    // Save original values for comparison
    char original_url[MAX_URL_LENGTH];
    strncpy(original_url, config.url, MAX_URL_LENGTH - 1);
    original_url[MAX_URL_LENGTH - 1] = '\0';
    
    stream_protocol_t original_protocol = config.protocol;
    
    cJSON *url = cJSON_GetObjectItem(stream_json, "url");
    if (url && cJSON_IsString(url)) {
        if (strcmp(config.url, url->valuestring) != 0) {
            strncpy(config.url, url->valuestring, sizeof(config.url) - 1);
            config_changed = true;
            requires_restart = true;  // URL changes always require restart
            log_info("URL changed from '%s' to '%s' - restart required", original_url, config.url);
        }
    }
    
    cJSON *enabled = cJSON_GetObjectItem(stream_json, "enabled");
    if (enabled && cJSON_IsBool(enabled)) {
        config.enabled = cJSON_IsTrue(enabled);
        config_changed = true;
    }
    
    cJSON *streaming_enabled = cJSON_GetObjectItem(stream_json, "streaming_enabled");
    if (streaming_enabled && cJSON_IsBool(streaming_enabled)) {
        config.streaming_enabled = cJSON_IsTrue(streaming_enabled);
        config_changed = true;
    }
    
    cJSON *width = cJSON_GetObjectItem(stream_json, "width");
    if (width && cJSON_IsNumber(width)) {
        config.width = width->valueint;
        config_changed = true;
    }
    
    cJSON *height = cJSON_GetObjectItem(stream_json, "height");
    if (height && cJSON_IsNumber(height)) {
        config.height = height->valueint;
        config_changed = true;
    }
    
    cJSON *fps = cJSON_GetObjectItem(stream_json, "fps");
    if (fps && cJSON_IsNumber(fps)) {
        config.fps = fps->valueint;
        config_changed = true;
    }
    
    cJSON *codec = cJSON_GetObjectItem(stream_json, "codec");
    if (codec && cJSON_IsString(codec)) {
        strncpy(config.codec, codec->valuestring, sizeof(config.codec) - 1);
        config_changed = true;
    }
    
    cJSON *priority = cJSON_GetObjectItem(stream_json, "priority");
    if (priority && cJSON_IsNumber(priority)) {
        config.priority = priority->valueint;
        config_changed = true;
    }
    
    cJSON *record = cJSON_GetObjectItem(stream_json, "record");
    if (record && cJSON_IsBool(record)) {
        config.record = cJSON_IsTrue(record);
        config_changed = true;
    }
    
    cJSON *segment_duration = cJSON_GetObjectItem(stream_json, "segment_duration");
    if (segment_duration && cJSON_IsNumber(segment_duration)) {
        config.segment_duration = segment_duration->valueint;
        config_changed = true;
    }
    
    cJSON *detection_based_recording = cJSON_GetObjectItem(stream_json, "detection_based_recording");
    if (detection_based_recording && cJSON_IsBool(detection_based_recording)) {
        config.detection_based_recording = cJSON_IsTrue(detection_based_recording);
        config_changed = true;
    }
    
    cJSON *detection_model = cJSON_GetObjectItem(stream_json, "detection_model");
    if (detection_model && cJSON_IsString(detection_model)) {
        strncpy(config.detection_model, detection_model->valuestring, sizeof(config.detection_model) - 1);
        config_changed = true;
    }
    
    cJSON *detection_threshold = cJSON_GetObjectItem(stream_json, "detection_threshold");
    if (detection_threshold && cJSON_IsNumber(detection_threshold)) {
        // Convert from percentage (0-100) to float (0.0-1.0)
        config.detection_threshold = detection_threshold->valuedouble / 100.0f;
        config_changed = true;
    }
    
    cJSON *detection_interval = cJSON_GetObjectItem(stream_json, "detection_interval");
    if (detection_interval && cJSON_IsNumber(detection_interval)) {
        config.detection_interval = detection_interval->valueint;
        config_changed = true;
    }
    
    cJSON *pre_detection_buffer = cJSON_GetObjectItem(stream_json, "pre_detection_buffer");
    if (pre_detection_buffer && cJSON_IsNumber(pre_detection_buffer)) {
        config.pre_detection_buffer = pre_detection_buffer->valueint;
        config_changed = true;
    }
    
    cJSON *post_detection_buffer = cJSON_GetObjectItem(stream_json, "post_detection_buffer");
    if (post_detection_buffer && cJSON_IsNumber(post_detection_buffer)) {
        config.post_detection_buffer = post_detection_buffer->valueint;
        config_changed = true;
    }
    
    cJSON *protocol = cJSON_GetObjectItem(stream_json, "protocol");
    if (protocol && cJSON_IsNumber(protocol)) {
        stream_protocol_t new_protocol = (stream_protocol_t)protocol->valueint;
        if (config.protocol != new_protocol) {
            config.protocol = new_protocol;
            config_changed = true;
            requires_restart = true;  // Protocol changes always require restart
            log_info("Protocol changed from %d to %d - restart required", 
                    original_protocol, config.protocol);
        }
    }
    
    // Clean up JSON
    cJSON_Delete(stream_json);
    
    // Always update stream configuration in database, even if no changes detected
    // This ensures the database and memory state are in sync
    log_info("Detection settings before update - Model: %s, Threshold: %.2f, Interval: %d, Pre-buffer: %d, Post-buffer: %d",
             config.detection_model, config.detection_threshold, config.detection_interval,
             config.pre_detection_buffer, config.post_detection_buffer);
    
    // Update stream configuration in database first
    if (update_stream_config(decoded_id, &config) != 0) {
        log_error("Failed to update stream configuration in database");
        mg_send_json_error(c, 500, "Failed to update stream configuration");
        return;
    }
    
    // Force update of stream configuration in memory to ensure it matches the database
    // This ensures the stream handle has the latest configuration
    
    // IMPORTANT: Ensure the in-memory configuration is fully updated from the database
    // This is critical for URL changes to take effect
    stream_config_t updated_config;
    if (get_stream_config(stream, &updated_config) != 0) {
        log_error("Failed to refresh stream configuration from database for stream %s", config.name);
        mg_send_json_error(c, 500, "Failed to refresh stream configuration");
        return;
    }
    
    if (set_stream_detection_params(stream, 
                                   config.detection_interval, 
                                   config.detection_threshold, 
                                   config.pre_detection_buffer, 
                                   config.post_detection_buffer) != 0) {
        log_warn("Failed to update detection parameters for stream %s", config.name);
    }
    
    if (set_stream_detection_recording(stream, 
                                      config.detection_based_recording, 
                                      config.detection_model) != 0) {
        log_warn("Failed to update detection recording for stream %s", config.name);
    }
    
    // Update other stream properties in memory
    if (set_stream_recording(stream, config.record) != 0) {
        log_warn("Failed to update recording setting for stream %s", config.name);
    }
    
    if (set_stream_streaming_enabled(stream, config.streaming_enabled) != 0) {
        log_warn("Failed to update streaming setting for stream %s", config.name);
    }
    
    log_info("Updated stream configuration in memory for stream %s", config.name);
    
    // Verify the update by reading back the configuration
    if (get_stream_config(stream, &updated_config) == 0) {
        log_info("Detection settings after update - Model: %s, Threshold: %.2f, Interval: %d, Pre-buffer: %d, Post-buffer: %d",
                 updated_config.detection_model, updated_config.detection_threshold, updated_config.detection_interval,
                 updated_config.pre_detection_buffer, updated_config.post_detection_buffer);
    }
    
    // Restart stream if configuration changed and either:
    // 1. Critical parameters requiring restart were changed (URL, protocol)
    // 2. The stream is currently running
    stream_status_t status = get_stream_status(stream);
    bool is_running = (status == STREAM_STATUS_RUNNING || status == STREAM_STATUS_STARTING);
    
    if (config_changed && (requires_restart || is_running)) {
        log_info("Restarting stream %s (requires_restart=%s, is_running=%s)", 
                config.name, 
                requires_restart ? "true" : "false",
                is_running ? "true" : "false");
        
        // If URL or protocol changed, we need to force restart the HLS stream thread
        bool url_changed = strcmp(original_url, config.url) != 0;
        bool protocol_changed = original_protocol != config.protocol;
        
        // First clear HLS segments if URL changed
        if (url_changed) {
            log_info("URL changed for stream %s, clearing HLS segments", config.name);
            if (clear_stream_hls_segments(config.name) != 0) {
                log_warn("Failed to clear HLS segments for stream %s", config.name);
                // Continue anyway
            }
        }
        
        // Stop stream if it's running
        if (is_running) {
            log_info("Stopping stream %s for restart", config.name);
            
            // Stop stream
            if (stop_stream(stream) != 0) {
                log_error("Failed to stop stream: %s", decoded_id);
                // Continue anyway
            }
            
            // Wait for stream to stop with increased timeout for critical parameter changes
            int timeout = requires_restart ? 50 : 30; // 5 seconds for critical changes, 3 seconds otherwise
            while (get_stream_status(stream) != STREAM_STATUS_STOPPED && timeout > 0) {
                usleep(100000); // 100ms
                timeout--;
            }
            
            if (timeout == 0) {
                log_warn("Timeout waiting for stream %s to stop, continuing anyway", config.name);
            }
        }
        
        // Start stream if enabled
        if (config.enabled) {
            log_info("Starting stream %s after configuration update", config.name);
            if (start_stream(stream) != 0) {
                log_error("Failed to restart stream: %s", decoded_id);
                // Continue anyway
            }
            
            // If URL or protocol changed, force restart the HLS stream thread
            if ((url_changed || protocol_changed) && config.streaming_enabled) {
                log_info("URL or protocol changed for stream %s, force restarting HLS stream thread", config.name);
                if (restart_hls_stream(config.name) != 0) {
                    log_warn("Failed to force restart HLS stream for %s", config.name);
                    // Continue anyway
                } else {
                    log_info("Successfully force restarted HLS stream for %s", config.name);
                }
            }
        }
    } else if (config_changed) {
        log_info("Configuration changed for stream %s but restart not required", config.name);
    }
    
    // Create success response using cJSON
    cJSON *success = cJSON_CreateObject();
    if (!success) {
        log_error("Failed to create success JSON object");
        mg_send_json_error(c, 500, "Failed to create success JSON");
        return;
    }
    
    cJSON_AddBoolToObject(success, "success", true);
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(success);
    if (!json_str) {
        log_error("Failed to convert success JSON to string");
        cJSON_Delete(success);
        mg_send_json_error(c, 500, "Failed to convert success JSON to string");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    cJSON_Delete(success);
    
    log_info("Successfully updated stream: %s", decoded_id);
}

/**
 * @brief Direct handler for DELETE /api/streams/:id
 */
void mg_handle_delete_stream(struct mg_connection *c, struct mg_http_message *hm) {
    // Extract stream ID from URL
    char stream_id[MAX_STREAM_NAME];
    if (mg_extract_path_param(hm, "/api/streams/", stream_id, sizeof(stream_id)) != 0) {
        log_error("Failed to extract stream ID from URL");
        mg_send_json_error(c, 400, "Invalid request path");
        return;
    }
    
    // URL-decode the stream identifier
    char decoded_id[MAX_STREAM_NAME];
    mg_url_decode(stream_id, strlen(stream_id), decoded_id, sizeof(decoded_id), 0);
    
    log_info("Handling DELETE /api/streams/%s request", decoded_id);
    
    // Find the stream by name
    stream_handle_t stream = get_stream_by_name(decoded_id);
    if (!stream) {
        log_error("Stream not found: %s", decoded_id);
        mg_send_json_error(c, 404, "Stream not found");
        return;
    }
    
    // Stop stream if it's running
    stream_status_t status = get_stream_status(stream);
    if (status == STREAM_STATUS_RUNNING || status == STREAM_STATUS_STARTING) {
        if (stop_stream(stream) != 0) {
            log_error("Failed to stop stream: %s", decoded_id);
            // Continue anyway
        }
        
        // Wait for stream to stop
        int timeout = 30; // 3 seconds
        while (get_stream_status(stream) != STREAM_STATUS_STOPPED && timeout > 0) {
            usleep(100000); // 100ms
            timeout--;
        }
    }
    
    // Delete stream
    if (remove_stream(stream) != 0) {
        log_error("Failed to delete stream: %s", decoded_id);
        mg_send_json_error(c, 500, "Failed to delete stream");
        return;
    }
    
    // Delete stream configuration from database
    if (delete_stream_config(decoded_id) != 0) {
        log_error("Failed to delete stream configuration from database");
        mg_send_json_error(c, 500, "Failed to delete stream configuration");
        return;
    }
    
    // Create success response using cJSON
    cJSON *success = cJSON_CreateObject();
    if (!success) {
        log_error("Failed to create success JSON object");
        mg_send_json_error(c, 500, "Failed to create success JSON");
        return;
    }
    
    cJSON_AddBoolToObject(success, "success", true);
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(success);
    if (!json_str) {
        log_error("Failed to convert success JSON to string");
        cJSON_Delete(success);
        mg_send_json_error(c, 500, "Failed to convert success JSON to string");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    cJSON_Delete(success);
    
    log_info("Successfully deleted stream: %s", decoded_id);
}
