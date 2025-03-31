#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include "web/api_handlers.h"
#include "web/mongoose_adapter.h"
#include "web/mongoose_server_auth.h"
#include "web/http_server.h"
#include "core/logger.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "mongoose.h"

/**
 * @brief Direct handler for POST /api/streaming/:stream/webrtc/offer
 * 
 * This is a placeholder implementation for WebRTC functionality.
 * In a real implementation, this would handle WebRTC offer SDP messages
 * and establish a WebRTC connection for streaming.
 */
void mg_handle_webrtc_offer(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/streaming/*/webrtc/offer request");
    
    // Check authentication
    http_server_t *server = (http_server_t *)c->fn_data;
    if (server && server->config.auth_enabled) {
        // Check if the user is authenticated
        if (mongoose_server_basic_auth_check(hm, server) != 0) {
            log_error("Authentication failed for WebRTC offer request");
            mg_send_json_error(c, 401, "Unauthorized");
            return;
        }
    }
    
    // Extract stream name from URL
    char stream_name[MAX_STREAM_NAME];
    
    // Extract the stream name from the URL path
    // URL format: /api/streaming/{stream_name}/webrtc/offer
    struct mg_str uri = hm->uri;
    const char *uri_ptr = mg_str_get_ptr(&uri);
    
    // Find the start of the stream name
    const char *stream_start = strstr(uri_ptr, "/streaming/");
    if (!stream_start) {
        log_error("Invalid request path: %.*s", (int)uri.len, uri_ptr);
        mg_send_json_error(c, 400, "Invalid request path");
        return;
    }
    
    stream_start += 11; // Skip "/streaming/"
    
    // Find the end of the stream name
    const char *stream_end = strstr(stream_start, "/webrtc/");
    if (!stream_end) {
        log_error("Invalid WebRTC request path: %.*s", (int)uri.len, uri_ptr);
        mg_send_json_error(c, 400, "Invalid WebRTC request path");
        return;
    }
    
    // Extract the stream name
    size_t name_len = stream_end - stream_start;
    if (name_len >= sizeof(stream_name)) {
        log_error("Stream name too long");
        mg_send_json_error(c, 400, "Stream name too long");
        return;
    }
    
    memcpy(stream_name, stream_start, name_len);
    stream_name[name_len] = '\0';
    
    // URL decode the stream name
    char decoded_name[MAX_STREAM_NAME];
    mg_url_decode(stream_name, strlen(stream_name), decoded_name, sizeof(decoded_name), 0);
    
    log_info("WebRTC offer request for stream: %s", decoded_name);
    
    // Check if stream exists
    stream_handle_t stream = get_stream_by_name(decoded_name);
    if (!stream) {
        log_error("Stream not found: %s", decoded_name);
        mg_send_json_error(c, 404, "Stream not found");
        return;
    }
    
    // In a real implementation, this would:
    // 1. Parse the WebRTC offer from the request body
    // 2. Use the libWebRTC or similar library to create an answer
    // 3. Send the answer back to the client
    
    // Create response using cJSON
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        log_error("Failed to create response JSON object");
        mg_send_json_error(c, 500, "Failed to create response JSON");
        return;
    }
    
    cJSON_AddStringToObject(response, "status", "acknowledged");
    cJSON_AddStringToObject(response, "message", "WebRTC not yet implemented");
    
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
    
    log_info("Successfully handled WebRTC offer request for stream: %s", decoded_name);
}

/**
 * @brief Direct handler for POST /api/streaming/:stream/webrtc/ice
 * 
 * This is a placeholder implementation for WebRTC ICE functionality.
 * In a real implementation, this would handle WebRTC ICE candidates
 * for establishing peer-to-peer connections.
 */
void mg_handle_webrtc_ice(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/streaming/*/webrtc/ice request");
    
    // Check authentication
    http_server_t *server = (http_server_t *)c->fn_data;
    if (server && server->config.auth_enabled) {
        // Check if the user is authenticated
        if (mongoose_server_basic_auth_check(hm, server) != 0) {
            log_error("Authentication failed for WebRTC ICE request");
            mg_send_json_error(c, 401, "Unauthorized");
            return;
        }
    }
    
    // Extract stream name from URL
    char stream_name[MAX_STREAM_NAME];
    
    // Extract the stream name from the URL path
    // URL format: /api/streaming/{stream_name}/webrtc/ice
    struct mg_str uri = hm->uri;
    const char *uri_ptr = mg_str_get_ptr(&uri);
    
    // Find the start of the stream name
    const char *stream_start = strstr(uri_ptr, "/streaming/");
    if (!stream_start) {
        log_error("Invalid request path: %.*s", (int)uri.len, uri_ptr);
        mg_send_json_error(c, 400, "Invalid request path");
        return;
    }
    
    stream_start += 11; // Skip "/streaming/"
    
    // Find the end of the stream name
    const char *stream_end = strstr(stream_start, "/webrtc/");
    if (!stream_end) {
        log_error("Invalid WebRTC request path: %.*s", (int)uri.len, uri_ptr);
        mg_send_json_error(c, 400, "Invalid WebRTC request path");
        return;
    }
    
    // Extract the stream name
    size_t name_len = stream_end - stream_start;
    if (name_len >= sizeof(stream_name)) {
        log_error("Stream name too long");
        mg_send_json_error(c, 400, "Stream name too long");
        return;
    }
    
    memcpy(stream_name, stream_start, name_len);
    stream_name[name_len] = '\0';
    
    // URL decode the stream name
    char decoded_name[MAX_STREAM_NAME];
    mg_url_decode(stream_name, strlen(stream_name), decoded_name, sizeof(decoded_name), 0);
    
    log_info("WebRTC ICE request for stream: %s", decoded_name);
    
    // In a real implementation, this would:
    // 1. Parse the ICE candidate from the request body
    // 2. Add the ICE candidate to the WebRTC connection
    // 3. Send a success response
    
    // For now, just return a 501 Not Implemented error
    mg_send_json_error(c, 501, "WebRTC ICE handling not implemented");
    
    log_info("Successfully handled WebRTC ICE request for stream: %s", decoded_name);
}
