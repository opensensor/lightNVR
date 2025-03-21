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
