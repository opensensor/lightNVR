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
#include "video/detection_stream_thread.h"
#include "database/database_manager.h"
#include "video/hls/hls_directory.h"
#include "video/hls/hls_api.h"
#include "video/onvif_device_management.h"

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
    config.record_audio = true; // Default to true for new streams
    
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
    
    cJSON *record_audio = cJSON_GetObjectItem(stream_json, "record_audio");
    if (record_audio && cJSON_IsBool(record_audio)) {
        config.record_audio = cJSON_IsTrue(record_audio);
        log_info("Audio recording %s for stream %s", 
                config.record_audio ? "enabled" : "disabled", config.name);
    }
    
    // Check if isOnvif flag is set in the request
    cJSON *is_onvif = cJSON_GetObjectItem(stream_json, "isOnvif");
    if (is_onvif && cJSON_IsBool(is_onvif)) {
        config.is_onvif = cJSON_IsTrue(is_onvif);
    } else {
        // Fall back to URL-based detection if not explicitly set
        config.is_onvif = (strstr(config.url, "onvif") != NULL);
    }
    
    log_info("ONVIF flag for stream %s: %s", config.name, config.is_onvif ? "true" : "false");
    
    // If ONVIF flag is set, test the connection
    bool onvif_test_success = true;
    bool onvif_test_performed = false;
    if (config.is_onvif) {
        onvif_test_performed = true;
        log_info("Testing ONVIF capabilities for stream %s", config.name);
        
        // Extract username and password if provided
        cJSON *onvif_username = cJSON_GetObjectItem(stream_json, "onvif_username");
        cJSON *onvif_password = cJSON_GetObjectItem(stream_json, "onvif_password");
        
        if (onvif_username && cJSON_IsString(onvif_username)) {
            strncpy(config.onvif_username, onvif_username->valuestring, sizeof(config.onvif_username) - 1);
            config.onvif_username[sizeof(config.onvif_username) - 1] = '\0';
        }
        
        if (onvif_password && cJSON_IsString(onvif_password)) {
            strncpy(config.onvif_password, onvif_password->valuestring, sizeof(config.onvif_password) - 1);
            config.onvif_password[sizeof(config.onvif_password) - 1] = '\0';
        }
        
        // Test ONVIF connection
        int result = test_onvif_connection(config.url, 
                                          config.onvif_username[0] ? config.onvif_username : NULL, 
                                          config.onvif_password[0] ? config.onvif_password : NULL);
        
        onvif_test_success = (result == 0);
        
        // If ONVIF test fails, don't save as ONVIF
        if (!onvif_test_success) {
            log_warn("ONVIF test failed for stream %s, disabling ONVIF flag", config.name);
            config.is_onvif = false;
        }
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
        
        // Start detection thread if detection is enabled and we have a model
        if (config.detection_based_recording && config.detection_model[0] != '\0') {
            log_info("Detection enabled for new stream %s, starting detection thread with model %s", 
                    config.name, config.detection_model);
            
            // Construct HLS directory path
            char hls_dir[MAX_PATH_LENGTH];
            snprintf(hls_dir, MAX_PATH_LENGTH, "/var/lib/lightnvr/hls/%s", config.name);
            
            // Start detection thread
            if (start_stream_detection_thread(config.name, config.detection_model, 
                                             config.detection_threshold, 
                                             config.detection_interval, hls_dir) != 0) {
                log_warn("Failed to start detection thread for new stream %s", config.name);
            } else {
                log_info("Successfully started detection thread for new stream %s", config.name);
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
    
    // Add ONVIF detection result if applicable
    if (onvif_test_performed) {
        if (onvif_test_success) {
            cJSON_AddStringToObject(success, "onvif_status", "success");
            cJSON_AddStringToObject(success, "onvif_message", "ONVIF capabilities detected successfully");
        } else {
            cJSON_AddStringToObject(success, "onvif_status", "error");
            cJSON_AddStringToObject(success, "onvif_message", "Failed to detect ONVIF capabilities");
        }
    }
    
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
    
    cJSON *record_audio = cJSON_GetObjectItem(stream_json, "record_audio");
    if (record_audio && cJSON_IsBool(record_audio)) {
        bool original_record_audio = config.record_audio;
        config.record_audio = cJSON_IsTrue(record_audio);
        if (original_record_audio != config.record_audio) {
            config_changed = true;
            requires_restart = true;  // Audio recording changes require restart
            log_info("Audio recording changed from %s to %s - restart required", 
                    original_record_audio ? "enabled" : "disabled", 
                    config.record_audio ? "enabled" : "disabled");
        }
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
    
    // Update is_onvif flag based on request or URL
    bool original_is_onvif = config.is_onvif;
    
    // Check if isOnvif flag is set in the request
    cJSON *is_onvif = cJSON_GetObjectItem(stream_json, "isOnvif");
    if (is_onvif && cJSON_IsBool(is_onvif)) {
        config.is_onvif = cJSON_IsTrue(is_onvif);
    } else {
        // Fall back to URL-based detection if not explicitly set
        config.is_onvif = (strstr(config.url, "onvif") != NULL);
    }
    
    if (original_is_onvif != config.is_onvif) {
        log_info("ONVIF flag changed from %s to %s", 
                original_is_onvif ? "true" : "false", 
                config.is_onvif ? "true" : "false");
        config_changed = true;
    }
    
    // If ONVIF flag is set, test the connection
    bool onvif_test_success = true;
    bool onvif_test_performed = false;
    
    if (config.is_onvif) {
        log_info("Testing ONVIF capabilities for stream %s", config.name);
        onvif_test_performed = true;
        
        // Extract username and password if provided
        cJSON *onvif_username = cJSON_GetObjectItem(stream_json, "onvif_username");
        cJSON *onvif_password = cJSON_GetObjectItem(stream_json, "onvif_password");
        
        if (onvif_username && cJSON_IsString(onvif_username)) {
            strncpy(config.onvif_username, onvif_username->valuestring, sizeof(config.onvif_username) - 1);
            config.onvif_username[sizeof(config.onvif_username) - 1] = '\0';
        }
        
        if (onvif_password && cJSON_IsString(onvif_password)) {
            strncpy(config.onvif_password, onvif_password->valuestring, sizeof(config.onvif_password) - 1);
            config.onvif_password[sizeof(config.onvif_password) - 1] = '\0';
        }
        
        // Test ONVIF connection
        int result = test_onvif_connection(config.url, 
                                          config.onvif_username[0] ? config.onvif_username : NULL, 
                                          config.onvif_password[0] ? config.onvif_password : NULL);
        
        onvif_test_success = (result == 0);
        // If ONVIF test fails, don't save as ONVIF
        if (!onvif_test_success) {
            log_warn("ONVIF test failed for stream %s, disabling ONVIF flag", config.name);
            config.is_onvif = false;
        }
    }
    
    // Check if there's a request to undelete the stream
    cJSON *is_deleted = cJSON_GetObjectItem(stream_json, "is_deleted");
    bool undelete_requested = false;
    if (is_deleted && cJSON_IsBool(is_deleted) && !cJSON_IsTrue(is_deleted)) {
        // Request to set is_deleted to false (undelete)
        undelete_requested = true;
        log_info("Undelete requested for stream %s", decoded_id);
        
        // Check if the stream is currently soft-deleted
        bool currently_deleted = false;
        sqlite3 *db = get_db_handle();
        if (db) {
            sqlite3_stmt *stmt;
            const char *sql = "SELECT is_deleted FROM streams WHERE name = ?;";
            if (sqlite3_prepare_v2(db, sql, -1, &stmt, NULL) == SQLITE_OK) {
                sqlite3_bind_text(stmt, 1, decoded_id, -1, SQLITE_STATIC);
                if (sqlite3_step(stmt) == SQLITE_ROW && sqlite3_column_type(stmt, 0) != SQLITE_NULL) {
                    currently_deleted = sqlite3_column_int(stmt, 0) != 0;
                }
                sqlite3_finalize(stmt);
            }
        }
        
        if (currently_deleted) {
            // Undelete the stream by setting is_deleted to 0
            sqlite3 *db = get_db_handle();
            if (db) {
                sqlite3_stmt *stmt;
                const char *sql = "UPDATE streams SET is_deleted = 0 WHERE name = ?;";
                if (sqlite3_prepare_v2(db, sql, -1, &stmt, NULL) == SQLITE_OK) {
                    sqlite3_bind_text(stmt, 1, decoded_id, -1, SQLITE_STATIC);
                    if (sqlite3_step(stmt) != SQLITE_DONE) {
                        log_error("Failed to undelete stream %s: %s", decoded_id, sqlite3_errmsg(db));
                    } else {
                        log_info("Successfully undeleted stream %s", decoded_id);
                    }
                    sqlite3_finalize(stmt);
                }
            }
        }
    }
    
    // Clean up JSON
    cJSON_Delete(stream_json);
    
    // Always update stream configuration in database, even if no changes detected
    // This ensures the database and memory state are in sync
    log_info("Detection settings before update - Model: %s, Threshold: %.2f, Interval: %d, Pre-buffer: %d, Post-buffer: %d",
             config.detection_model, config.detection_threshold, config.detection_interval,
             config.pre_detection_buffer, config.post_detection_buffer);
    
    // Check if stream is running - we'll need this information for detection settings changes
    stream_status_t status = get_stream_status(stream);
    bool is_running = (status == STREAM_STATUS_RUNNING || status == STREAM_STATUS_STARTING);
    
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
    
    // If detection settings were changed and the stream is running,
    // we need to restart the stream to apply the new detection settings
    if (config_changed && 
        (detection_based_recording != NULL || detection_model != NULL || 
         detection_threshold != NULL || detection_interval != NULL) && 
        is_running && !requires_restart) {
        log_info("Detection settings changed for stream %s, marking for restart to apply changes", config.name);
        requires_restart = true;
    }
    
    // Update other stream properties in memory
    if (set_stream_recording(stream, config.record) != 0) {
        log_warn("Failed to update recording setting for stream %s", config.name);
    }
    
    if (set_stream_streaming_enabled(stream, config.streaming_enabled) != 0) {
        log_warn("Failed to update streaming setting for stream %s", config.name);
    }
    
    // Handle detection thread management based on detection_based_recording setting
    bool detection_enabled_changed = false;
    bool detection_was_enabled = false;
    bool detection_now_enabled = false;
    
    // Check if detection_based_recording was changed in this request
    if (detection_based_recording != NULL) {
        detection_enabled_changed = true;
        detection_was_enabled = !cJSON_IsTrue(detection_based_recording); // Previous state was opposite
        detection_now_enabled = cJSON_IsTrue(detection_based_recording);  // New state
    } else {
        // If not explicitly changed in this request, use the current config value
        detection_now_enabled = config.detection_based_recording;
        
        // We need to check if a detection thread is already running to determine previous state
        detection_was_enabled = is_stream_detection_thread_running(config.name);
    }
    
    // If detection was enabled and now disabled, stop the detection thread
    if (detection_was_enabled && !detection_now_enabled) {
        log_info("Detection disabled for stream %s, stopping detection thread", config.name);
        if (stop_stream_detection_thread(config.name) != 0) {
            log_warn("Failed to stop detection thread for stream %s", config.name);
        } else {
            log_info("Successfully stopped detection thread for stream %s", config.name);
        }
    }
    // If detection was disabled and now enabled, start the detection thread
    else if (!detection_was_enabled && detection_now_enabled) {
        // Only start if we have a valid model and the stream is enabled
        if (config.detection_model[0] != '\0' && config.enabled) {
            log_info("Detection enabled for stream %s, starting detection thread with model %s", 
                    config.name, config.detection_model);
            
            // Construct HLS directory path
            char hls_dir[MAX_PATH_LENGTH];
            snprintf(hls_dir, MAX_PATH_LENGTH, "/var/lib/lightnvr/hls/%s", config.name);
            
            // Start detection thread
            if (start_stream_detection_thread(config.name, config.detection_model, 
                                             config.detection_threshold, 
                                             config.detection_interval, hls_dir) != 0) {
                log_warn("Failed to start detection thread for stream %s", config.name);
            } else {
                log_info("Successfully started detection thread for stream %s", config.name);
            }
        } else {
            log_warn("Detection enabled for stream %s but no model specified or stream disabled", config.name);
        }
    }
    // If detection settings changed but detection was already enabled, restart the thread with new settings
    else if (detection_now_enabled && (detection_model != NULL || detection_threshold != NULL || detection_interval != NULL)) {
        log_info("Detection settings changed for stream %s, restarting detection thread", config.name);
        
        // Stop existing thread
        if (stop_stream_detection_thread(config.name) != 0) {
            log_warn("Failed to stop existing detection thread for stream %s", config.name);
        }
        
        // Start new thread with updated settings
        if (config.detection_model[0] != '\0' && config.enabled) {
            // Construct HLS directory path
            char hls_dir[MAX_PATH_LENGTH];
            snprintf(hls_dir, MAX_PATH_LENGTH, "/var/lib/lightnvr/hls/%s", config.name);
            
            // Start detection thread
            if (start_stream_detection_thread(config.name, config.detection_model, 
                                             config.detection_threshold, 
                                             config.detection_interval, hls_dir) != 0) {
                log_warn("Failed to restart detection thread for stream %s", config.name);
            } else {
                log_info("Successfully restarted detection thread for stream %s", config.name);
            }
        }
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
    // Note: We already checked the stream status earlier
    
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
    
    // Add ONVIF detection result if applicable
    if (onvif_test_performed) {
        if (onvif_test_success) {
            cJSON_AddStringToObject(success, "onvif_status", "success");
            cJSON_AddStringToObject(success, "onvif_message", "ONVIF capabilities detected successfully");
        } else {
            cJSON_AddStringToObject(success, "onvif_status", "error");
            cJSON_AddStringToObject(success, "onvif_message", "Failed to detect ONVIF capabilities");
        }
    }
    
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
    
    // Stop any detection thread for this stream
    if (is_stream_detection_thread_running(decoded_id)) {
        log_info("Stopping detection thread for stream %s", decoded_id);
        if (stop_stream_detection_thread(decoded_id) != 0) {
            log_warn("Failed to stop detection thread for stream %s", decoded_id);
            // Continue anyway
        } else {
            log_info("Successfully stopped detection thread for stream %s", decoded_id);
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
