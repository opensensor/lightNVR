/**
 * @file api_handlers_ice_servers.c
 * @brief API handlers for ICE server configuration (WebRTC TURN/STUN)
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <cjson/cJSON.h>

#include "web/api_handlers.h"
#include "web/request_response.h"
#include "core/config.h"
#include "core/logger.h"

/**
 * @brief Handler for GET /api/ice-servers
 * Returns ICE server configuration for WebRTC clients (browser)
 * This allows the TURN server to be configured per-deployment
 */
void handle_get_ice_servers(const http_request_t *req, http_response_t *res) {
    log_info("Handling GET /api/ice-servers request");
    
    // Create response object
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        log_error("Failed to create ice-servers JSON object");
        http_response_set_json_error(res, 500, "Failed to create JSON response");
        return;
    }
    
    // Create ice_servers array
    cJSON *ice_servers = cJSON_CreateArray();
    if (!ice_servers) {
        log_error("Failed to create ice_servers JSON array");
        cJSON_Delete(response);
        http_response_set_json_error(res, 500, "Failed to create JSON response");
        return;
    }
    
    // Always add STUN server if enabled
    if (g_config.go2rtc_stun_enabled) {
        cJSON *stun = cJSON_CreateObject();
        if (stun) {
            cJSON *urls = cJSON_CreateArray();
            if (urls) {
                // Use configured STUN server or default
                const char *stun_server = g_config.go2rtc_stun_server[0] ? 
                    g_config.go2rtc_stun_server : "stun:stun.l.google.com:19302";
                cJSON_AddItemToArray(urls, cJSON_CreateString(stun_server));
                cJSON_AddItemToObject(stun, "urls", urls);
                cJSON_AddItemToArray(ice_servers, stun);
            } else {
                cJSON_Delete(stun);
            }
        }
    }
    
    // Add TURN server if enabled and configured
    if (g_config.turn_enabled && g_config.turn_server_url[0]) {
        cJSON *turn = cJSON_CreateObject();
        if (turn) {
            cJSON *urls = cJSON_CreateArray();
            if (urls) {
                cJSON_AddItemToArray(urls, cJSON_CreateString(g_config.turn_server_url));
                cJSON_AddItemToObject(turn, "urls", urls);
                
                // Add credentials if configured
                if (g_config.turn_username[0]) {
                    cJSON_AddStringToObject(turn, "username", g_config.turn_username);
                }
                if (g_config.turn_password[0]) {
                    cJSON_AddStringToObject(turn, "credential", g_config.turn_password);
                }
                
                cJSON_AddItemToArray(ice_servers, turn);
            } else {
                cJSON_Delete(turn);
            }
        }
    }
    
    cJSON_AddItemToObject(response, "ice_servers", ice_servers);
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(response);
    if (!json_str) {
        log_error("Failed to serialize ice-servers JSON");
        cJSON_Delete(response);
        http_response_set_json_error(res, 500, "Failed to serialize JSON response");
        return;
    }
    
    // Send response
    http_response_set_json(res, 200, json_str);
    
    // Cleanup
    free(json_str);
    cJSON_Delete(response);
    
    log_info("Successfully returned ICE servers configuration");
}

