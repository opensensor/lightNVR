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
