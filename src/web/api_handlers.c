#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include "web/api_handlers.h"
#include "web/mongoose_adapter.h"
#include "web/http_router.h"
#include "web/mongoose_server_handlers.h"
#include "core/logger.h"
#include "core/config.h"
#include "../external/mongoose/mongoose.h"

// We'll use the existing mg_handle_* functions directly

/**
 * @brief Register direct Mongoose API handlers
 */
void register_api_handlers(struct mg_mgr *mgr) {
    if (!mgr) {
        log_error("Invalid Mongoose manager for registering direct API handlers");
        return;
    }
    
    log_info("Registering direct Mongoose API handlers");
    
    // Note: The actual registration happens in mongoose_server.c in the s_api_routes table
    // This function is called during server initialization to perform any additional setup
    
    // Log the handlers that are being registered
    log_info("Registered handler: GET /api/streams");
    log_info("Registered handler: POST /api/streams");
    log_info("Registered handler: GET /api/streams/:id");
    log_info("Registered handler: PUT /api/streams/:id");
    log_info("Registered handler: DELETE /api/streams/:id");
    log_info("Registered handler: POST /api/streams/:id/toggle_streaming");
    log_info("Registered handler: GET /api/settings");
    log_info("Registered handler: POST /api/settings");
    log_info("Registered handler: GET /api/system/info");
    log_info("Registered handler: GET /api/system/logs");
    log_info("Registered handler: POST /api/system/restart");
    log_info("Registered handler: POST /api/system/shutdown");
    log_info("Registered handler: GET /api/recordings");
    log_info("Registered handler: GET /api/recordings/:id");
    log_info("Registered handler: DELETE /api/recordings/:id");
    log_info("Registered handler: GET /api/recordings/download/:id");
    log_info("Registered handler: GET /api/streaming/:stream/hls/index.m3u8");
    log_info("Registered handler: GET /api/streaming/:stream/hls/stream.m3u8");
    log_info("Registered handler: GET /api/streaming/:stream/hls/segment_:id.ts");
    log_info("Registered handler: GET /api/detection/results/:stream");
    log_info("Registered handler: POST /api/system/clear_logs");
    log_info("Registered handler: POST /api/system/backup");
    log_info("Registered handler: GET /api/system/status");
    log_info("Registered handler: POST /api/streaming/:stream/webrtc/offer");
    log_info("Registered handler: POST /api/streaming/:stream/webrtc/ice");
    
    log_info("Direct Mongoose API handlers registered");
}

/**
 * @brief Helper function to extract path parameter from URL
 */
int mg_extract_path_param(struct mg_http_message *hm, const char *prefix, char *param_buf, size_t buf_size) {
    if (!hm || !prefix || !param_buf || buf_size == 0) {
        return -1;
    }
    
    // Extract URI from HTTP message
    char uri[MAX_PATH_LENGTH];
    size_t uri_len = hm->uri.len < sizeof(uri) - 1 ? hm->uri.len : sizeof(uri) - 1;
    memcpy(uri, hm->uri.buf, uri_len);
    uri[uri_len] = '\0';
    
    // Check if URI starts with prefix
    size_t prefix_len = strlen(prefix);
    if (strncmp(uri, prefix, prefix_len) != 0) {
        return -1;
    }
    
    // Extract parameter (everything after the prefix)
    const char *param = uri + prefix_len;
    
    // Skip any leading slashes
    while (*param == '/') {
        param++;
    }
    
    // Find query string if present and truncate
    char *query = strchr(param, '?');
    if (query) {
        *query = '\0';
    }
    
    // Copy parameter to buffer
    size_t param_len = strlen(param);
    if (param_len >= buf_size) {
        return -1;
    }
    
    strcpy(param_buf, param);
    
    return 0;
}

/**
 * @brief Helper function to send a JSON response
 */
void mg_send_json_response(struct mg_connection *c, int status_code, const char *json_str) {
    if (!c || !json_str) {
        return;
    }
    
    mg_http_reply(c, status_code, "Content-Type: application/json\r\n", "%s", json_str);
}

/**
 * @brief Helper function to send a JSON error response
 */
void mg_send_json_error(struct mg_connection *c, int status_code, const char *error_message) {
    if (!c || !error_message) {
        return;
    }
    
    // Create error JSON
    cJSON *error = cJSON_CreateObject();
    if (!error) {
        mg_http_reply(c, 500, "Content-Type: application/json\r\n", 
                     "{\"error\": \"Failed to create error JSON\"}\n");
        return;
    }
    
    cJSON_AddStringToObject(error, "error", error_message);
    
    char *json_str = cJSON_PrintUnformatted(error);
    if (!json_str) {
        cJSON_Delete(error);
        mg_http_reply(c, 500, "Content-Type: application/json\r\n", 
                     "{\"error\": \"Failed to convert error JSON to string\"}\n");
        return;
    }
    
    mg_http_reply(c, status_code, "Content-Type: application/json\r\n", "%s", json_str);
    
    free(json_str);
    cJSON_Delete(error);
}

/**
 * @brief Helper function to parse JSON from request body
 */
cJSON* mg_parse_json_body(struct mg_http_message *hm) {
    if (!hm || !hm->body.len) {
        return NULL;
    }
    
    // Make a null-terminated copy of the request body
    char *body = malloc(hm->body.len + 1);
    if (!body) {
        log_error("Failed to allocate memory for request body");
        return NULL;
    }
    
    memcpy(body, hm->body.buf, hm->body.len);
    body[hm->body.len] = '\0';
    
    // Parse JSON
    cJSON *json = cJSON_Parse(body);
    free(body);
    
    if (!json) {
        log_error("Failed to parse JSON from request body");
        return NULL;
    }
    
    return json;
}
