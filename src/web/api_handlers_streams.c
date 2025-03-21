#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

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

/**
 * @brief Direct handler for GET /api/streams
 */
void mg_handle_get_streams(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/streams request");
    
    // Get all stream configurations from database
    stream_config_t db_streams[MAX_STREAMS];
    int count = get_all_stream_configs(db_streams, MAX_STREAMS);
    
    if (count < 0) {
        log_error("Failed to get stream configurations from database");
        mg_send_json_error(c, 500, "Failed to get stream configurations");
        return;
    }
    
    // Create JSON array
    cJSON *streams_array = cJSON_CreateArray();
    if (!streams_array) {
        log_error("Failed to create streams JSON array");
        mg_send_json_error(c, 500, "Failed to create streams JSON");
        return;
    }
    
    // Add each stream to the array
    for (int i = 0; i < count; i++) {
        cJSON *stream_obj = cJSON_CreateObject();
        if (!stream_obj) {
            log_error("Failed to create stream JSON object");
            cJSON_Delete(streams_array);
            mg_send_json_error(c, 500, "Failed to create stream JSON");
            return;
        }
        
        // Add stream properties
        cJSON_AddStringToObject(stream_obj, "name", db_streams[i].name);
        cJSON_AddStringToObject(stream_obj, "url", db_streams[i].url);
        cJSON_AddBoolToObject(stream_obj, "enabled", db_streams[i].enabled);
        cJSON_AddBoolToObject(stream_obj, "streaming_enabled", db_streams[i].streaming_enabled);
        cJSON_AddNumberToObject(stream_obj, "width", db_streams[i].width);
        cJSON_AddNumberToObject(stream_obj, "height", db_streams[i].height);
        cJSON_AddNumberToObject(stream_obj, "fps", db_streams[i].fps);
        cJSON_AddStringToObject(stream_obj, "codec", db_streams[i].codec);
        cJSON_AddNumberToObject(stream_obj, "priority", db_streams[i].priority);
        cJSON_AddBoolToObject(stream_obj, "record", db_streams[i].record);
        cJSON_AddNumberToObject(stream_obj, "segment_duration", db_streams[i].segment_duration);
        
        // Add detection settings
        cJSON_AddBoolToObject(stream_obj, "detection_based_recording", db_streams[i].detection_based_recording);
        cJSON_AddStringToObject(stream_obj, "detection_model", db_streams[i].detection_model);
        
        // Convert threshold from 0.0-1.0 to percentage (0-100)
        int threshold_percent = (int)(db_streams[i].detection_threshold * 100.0f);
        cJSON_AddNumberToObject(stream_obj, "detection_threshold", threshold_percent);
        
        cJSON_AddNumberToObject(stream_obj, "detection_interval", db_streams[i].detection_interval);
        cJSON_AddNumberToObject(stream_obj, "pre_detection_buffer", db_streams[i].pre_detection_buffer);
        cJSON_AddNumberToObject(stream_obj, "post_detection_buffer", db_streams[i].post_detection_buffer);
        cJSON_AddNumberToObject(stream_obj, "protocol", (int)db_streams[i].protocol);
        
        // Get stream status
        stream_handle_t stream = get_stream_by_name(db_streams[i].name);
        const char *status = "Unknown";
        if (stream) {
            stream_status_t stream_status = get_stream_status(stream);
            switch (stream_status) {
                case STREAM_STATUS_STOPPED:
                    status = "Stopped";
                    break;
                case STREAM_STATUS_STARTING:
                    status = "Starting";
                    break;
                case STREAM_STATUS_RUNNING:
                    status = "Running";
                    break;
                case STREAM_STATUS_STOPPING:
                    status = "Stopping";
                    break;
                case STREAM_STATUS_ERROR:
                    status = "Error";
                    break;
                default:
                    status = "Unknown";
                    break;
            }
        }
        cJSON_AddStringToObject(stream_obj, "status", status);
        
        // Add stream to array
        cJSON_AddItemToArray(streams_array, stream_obj);
    }
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(streams_array);
    if (!json_str) {
        log_error("Failed to convert streams JSON to string");
        cJSON_Delete(streams_array);
        mg_send_json_error(c, 500, "Failed to convert streams JSON to string");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    cJSON_Delete(streams_array);
    
    log_info("Successfully handled GET /api/streams request");
}

/**
 * @brief Direct handler for GET /api/streams/:id
 */
void mg_handle_get_stream(struct mg_connection *c, struct mg_http_message *hm) {
    // Extract stream ID from URL
    char stream_id[MAX_STREAM_NAME];
    if (mg_extract_path_param(hm, "/api/streams/", stream_id, sizeof(stream_id)) != 0) {
        log_error("Failed to extract stream ID from URL");
        mg_send_json_error(c, 400, "Invalid request path");
        return;
    }
    
    log_info("Handling GET /api/streams/%s request", stream_id);
    
    // URL-decode the stream identifier
    char decoded_id[MAX_STREAM_NAME];
    mg_url_decode(stream_id, strlen(stream_id), decoded_id, sizeof(decoded_id), 0);
    
    // Find the stream by name
    stream_handle_t stream = get_stream_by_name(decoded_id);
    if (!stream) {
        log_error("Stream not found: %s", decoded_id);
        mg_send_json_error(c, 404, "Stream not found");
        return;
    }
    
    // Get stream configuration
    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get stream configuration for: %s", decoded_id);
        mg_send_json_error(c, 500, "Failed to get stream configuration");
        return;
    }
    
    // Create JSON object
    cJSON *stream_obj = cJSON_CreateObject();
    if (!stream_obj) {
        log_error("Failed to create stream JSON object");
        mg_send_json_error(c, 500, "Failed to create stream JSON");
        return;
    }
    
    // Add stream properties
    cJSON_AddStringToObject(stream_obj, "name", config.name);
    cJSON_AddStringToObject(stream_obj, "url", config.url);
    cJSON_AddBoolToObject(stream_obj, "enabled", config.enabled);
    cJSON_AddBoolToObject(stream_obj, "streaming_enabled", config.streaming_enabled);
    cJSON_AddNumberToObject(stream_obj, "width", config.width);
    cJSON_AddNumberToObject(stream_obj, "height", config.height);
    cJSON_AddNumberToObject(stream_obj, "fps", config.fps);
    cJSON_AddStringToObject(stream_obj, "codec", config.codec);
    cJSON_AddNumberToObject(stream_obj, "priority", config.priority);
    cJSON_AddBoolToObject(stream_obj, "record", config.record);
    cJSON_AddNumberToObject(stream_obj, "segment_duration", config.segment_duration);
    
    // Add detection settings
    cJSON_AddBoolToObject(stream_obj, "detection_based_recording", config.detection_based_recording);
    cJSON_AddStringToObject(stream_obj, "detection_model", config.detection_model);
    
    // Convert threshold from 0.0-1.0 to percentage (0-100)
    int threshold_percent = (int)(config.detection_threshold * 100.0f);
    cJSON_AddNumberToObject(stream_obj, "detection_threshold", threshold_percent);
    
    cJSON_AddNumberToObject(stream_obj, "detection_interval", config.detection_interval);
    cJSON_AddNumberToObject(stream_obj, "pre_detection_buffer", config.pre_detection_buffer);
    cJSON_AddNumberToObject(stream_obj, "post_detection_buffer", config.post_detection_buffer);
    cJSON_AddNumberToObject(stream_obj, "protocol", (int)config.protocol);
    
    // Get stream status
    stream_status_t stream_status = get_stream_status(stream);
    const char *status = "Unknown";
    switch (stream_status) {
        case STREAM_STATUS_STOPPED:
            status = "Stopped";
            break;
        case STREAM_STATUS_STARTING:
            status = "Starting";
            break;
        case STREAM_STATUS_RUNNING:
            status = "Running";
            break;
        case STREAM_STATUS_STOPPING:
            status = "Stopping";
            break;
        case STREAM_STATUS_ERROR:
            status = "Error";
            break;
        default:
            status = "Unknown";
            break;
    }
    cJSON_AddStringToObject(stream_obj, "status", status);
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(stream_obj);
    if (!json_str) {
        log_error("Failed to convert stream JSON to string");
        cJSON_Delete(stream_obj);
        mg_send_json_error(c, 500, "Failed to convert stream JSON to string");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    cJSON_Delete(stream_obj);
    
    log_info("Successfully handled GET /api/streams/%s request", decoded_id);
}

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
        config.detection_threshold = detection_threshold->valueint / 100.0f;
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
    if (add_stream_config(&config) == 0) {
        log_error("Failed to add stream configuration to database");
        mg_send_json_error(c, 500, "Failed to add stream configuration");
        return;
    }
    
    // Create stream
    stream_handle_t stream = add_stream(&config);
    if (!stream) {
        log_error("Failed to create stream: %s", config.name);
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
    
    cJSON *url = cJSON_GetObjectItem(stream_json, "url");
    if (url && cJSON_IsString(url)) {
        strncpy(config.url, url->valuestring, sizeof(config.url) - 1);
        config_changed = true;
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
        config.detection_threshold = detection_threshold->valueint / 100.0f;
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
        config.protocol = (stream_protocol_t)protocol->valueint;
        config_changed = true;
    }
    
    // Clean up JSON
    cJSON_Delete(stream_json);
    
    // If configuration changed, update and restart stream
    if (config_changed) {
        // Update stream configuration
        if (update_stream_config(config.name, &config) != 0) {
            log_error("Failed to update stream configuration in database");
            mg_send_json_error(c, 500, "Failed to update stream configuration");
            return;
        }
        
        // Restart stream if it's running
        stream_status_t status = get_stream_status(stream);
        if (status == STREAM_STATUS_RUNNING || status == STREAM_STATUS_STARTING) {
            // Stop stream
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
            
            // Start stream if enabled
            if (config.enabled) {
                if (start_stream(stream) != 0) {
                    log_error("Failed to restart stream: %s", decoded_id);
                    // Continue anyway
                }
            }
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

/**
 * @brief Direct handler for POST /api/streams/:id/toggle_streaming
 */
void mg_handle_toggle_streaming(struct mg_connection *c, struct mg_http_message *hm) {
    // Extract stream ID from URL
    char stream_id[MAX_STREAM_NAME];
    char *toggle_path = "/api/streams/";
    char *toggle_suffix = "/toggle_streaming";
    
    // Extract stream ID from URL (between prefix and suffix)
    struct mg_str uri = hm->uri;
    const char *uri_ptr = mg_str_get_ptr(&uri);
    size_t uri_len = mg_str_get_len(&uri);
    
    size_t prefix_len = strlen(toggle_path);
    size_t suffix_len = strlen(toggle_suffix);
    
    if (uri_len <= prefix_len + suffix_len || 
        strncmp(uri_ptr, toggle_path, prefix_len) != 0 ||
        strncmp(uri_ptr + uri_len - suffix_len, toggle_suffix, suffix_len) != 0) {
        log_error("Invalid toggle_streaming URL format");
        mg_send_json_error(c, 400, "Invalid request path");
        return;
    }
    
    // Extract stream ID
    size_t id_len = uri_len - prefix_len - suffix_len;
    if (id_len >= sizeof(stream_id)) {
        log_error("Stream ID too long");
        mg_send_json_error(c, 400, "Stream ID too long");
        return;
    }
    
    memcpy(stream_id, uri_ptr + prefix_len, id_len);
    stream_id[id_len] = '\0';
    
    // URL-decode the stream identifier
    char decoded_id[MAX_STREAM_NAME];
    mg_url_decode(stream_id, strlen(stream_id), decoded_id, sizeof(decoded_id), 0);
    
    log_info("Handling POST /api/streams/%s/toggle_streaming request", decoded_id);
    
    // Find the stream by name
    stream_handle_t stream = get_stream_by_name(decoded_id);
    if (!stream) {
        log_error("Stream not found: %s", decoded_id);
        mg_send_json_error(c, 404, "Stream not found");
        return;
    }
    
    // Get stream configuration
    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get stream configuration for: %s", decoded_id);
        mg_send_json_error(c, 500, "Failed to get stream configuration");
        return;
    }
    
    // Toggle streaming_enabled flag
    config.streaming_enabled = !config.streaming_enabled;
    
    // Update stream configuration
    if (update_stream_config(config.name, &config) != 0) {
        log_error("Failed to update stream configuration in database");
        mg_send_json_error(c, 500, "Failed to update stream configuration");
        return;
    }
    
    // Apply changes to stream
    if (set_stream_streaming_enabled(stream, config.streaming_enabled) != 0) {
        log_error("Failed to %s streaming for stream: %s", 
                 config.streaming_enabled ? "enable" : "disable", decoded_id);
        mg_send_json_error(c, 500, "Failed to toggle streaming");
        return;
    }
    
    // Create response using cJSON
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        log_error("Failed to create response JSON object");
        mg_send_json_error(c, 500, "Failed to create response JSON");
        return;
    }
    
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddBoolToObject(response, "streaming_enabled", config.streaming_enabled);
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(response);
    if (!json_str) {
        log_error("Failed to convert response JSON to string");
        cJSON_Delete(response);
        mg_send_json_error(c, 500, "Failed to convert response JSON to string");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    cJSON_Delete(response);
    
    log_info("Successfully %s streaming for stream: %s", 
            config.streaming_enabled ? "enabled" : "disabled", decoded_id);
}
