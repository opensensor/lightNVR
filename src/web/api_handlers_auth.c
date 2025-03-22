#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include "cJSON.h"

#include "web/api_handlers.h"
#include "web/mongoose_adapter.h"
#include "core/logger.h"
#include "core/config.h"
#include "mongoose.h"

/**
 * @brief Direct handler for POST /api/auth/logout
 */
void mg_handle_auth_logout(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/auth/logout request");
    
    // Send a 401 Unauthorized response with WWW-Authenticate header
    // This will clear the browser's basic auth cache
    mg_printf(c, "HTTP/1.1 401 Unauthorized\r\n"
              "WWW-Authenticate: Basic realm=\"LightNVR\", stale=true\r\n"
              "Content-Type: application/json\r\n"
              "Content-Length: 29\r\n\r\n"
              "{\"success\":true,\"logged_out\":true}");
    
    log_info("Successfully handled POST /api/auth/logout request");
}

/**
 * @brief Direct handler for POST /api/auth/login
 */
void mg_handle_auth_login(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/auth/login request");
    
    // Parse JSON from request body
    cJSON *login = mg_parse_json_body(hm);
    if (!login) {
        log_error("Failed to parse login JSON from request body");
        mg_send_json_error(c, 400, "Invalid JSON in request body");
        return;
    }
    
    // Extract username and password
    cJSON *username = cJSON_GetObjectItem(login, "username");
    cJSON *password = cJSON_GetObjectItem(login, "password");
    
    if (!username || !cJSON_IsString(username) || !password || !cJSON_IsString(password)) {
        log_error("Missing or invalid username/password in login request");
        cJSON_Delete(login);
        mg_send_json_error(c, 400, "Missing or invalid username/password");
        return;
    }
    
    // Check credentials
    if (strcmp(username->valuestring, g_config.web_username) == 0 && 
        strcmp(password->valuestring, g_config.web_password) == 0) {
        // Login successful
        log_info("Login successful for user: %s", username->valuestring);
        
        // Create success response
        cJSON *success = cJSON_CreateObject();
        if (!success) {
            log_error("Failed to create success JSON object");
            cJSON_Delete(login);
            mg_send_json_error(c, 500, "Failed to create success JSON");
            return;
        }
        
        cJSON_AddBoolToObject(success, "success", true);
        cJSON_AddStringToObject(success, "message", "Login successful");
        
        // Convert to string
        char *json_str = cJSON_PrintUnformatted(success);
        if (!json_str) {
            log_error("Failed to convert success JSON to string");
            cJSON_Delete(success);
            cJSON_Delete(login);
            mg_send_json_error(c, 500, "Failed to convert success JSON to string");
            return;
        }
        
        // Send response
        mg_send_json_response(c, 200, json_str);
        
        // Clean up
        free(json_str);
        cJSON_Delete(success);
    } else {
        // Login failed
        log_warn("Login failed for user: %s", username->valuestring);
        
        // Create error response
        cJSON *error = cJSON_CreateObject();
        if (!error) {
            log_error("Failed to create error JSON object");
            cJSON_Delete(login);
            mg_send_json_error(c, 500, "Failed to create error JSON");
            return;
        }
        
        cJSON_AddBoolToObject(error, "success", false);
        cJSON_AddStringToObject(error, "message", "Invalid username or password");
        
        // Convert to string
        char *json_str = cJSON_PrintUnformatted(error);
        if (!json_str) {
            log_error("Failed to convert error JSON to string");
            cJSON_Delete(error);
            cJSON_Delete(login);
            mg_send_json_error(c, 500, "Failed to convert error JSON to string");
            return;
        }
        
        // Send response
        mg_send_json_response(c, 401, json_str);
        
        // Clean up
        free(json_str);
        cJSON_Delete(error);
    }
    
    // Clean up
    cJSON_Delete(login);
    
    log_info("Successfully handled POST /api/auth/login request");
}
