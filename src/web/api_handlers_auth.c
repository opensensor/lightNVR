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
    
    // Check if this is an API request
    struct mg_str *content_type = mg_http_get_header(hm, "Content-Type");
    struct mg_str *requested_with = mg_http_get_header(hm, "X-Requested-With");
    
    // Check if content_type contains "application/json"
    bool is_json = false;
    if (content_type) {
        const char *json_str = "application/json";
        size_t json_len = strlen(json_str);
        
        if (content_type->len >= json_len) {
            for (size_t i = 0; i <= content_type->len - json_len; i++) {
                if (strncmp(content_type->buf + i, json_str, json_len) == 0) {
                    is_json = true;
                    break;
                }
            }
        }
    }
    
    bool is_api_request = is_json || requested_with != NULL;
    
    if (is_api_request) {
        // For API requests, return a JSON success response
        cJSON *response = cJSON_CreateObject();
        cJSON_AddBoolToObject(response, "success", 1);
        cJSON_AddStringToObject(response, "redirect", "/login.html");
        
        // Send JSON response with cleared auth cookie
        mg_printf(c, "HTTP/1.1 200 OK\r\n");
        mg_printf(c, "Content-Type: application/json\r\n");
        mg_printf(c, "Set-Cookie: auth=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; SameSite=Strict\r\n");
        mg_printf(c, "WWW-Authenticate: Basic realm=\"logout\"\r\n");
        mg_printf(c, "Cache-Control: no-cache, no-store, must-revalidate\r\n");
        mg_printf(c, "Pragma: no-cache\r\n");
        mg_printf(c, "Expires: 0\r\n");
        
        char *json_str = cJSON_PrintUnformatted(response);
        mg_printf(c, "Content-Length: %d\r\n\r\n", (int)strlen(json_str));
        mg_printf(c, "%s", json_str);
        
        cJSON_Delete(response);
        free(json_str);
        
        // Ensure the connection is closed properly
        c->is_draining = 1;
    } else {
        // For form submissions, send a redirect response
        mg_printf(c, "HTTP/1.1 302 Found\r\n"
                "Location: /login.html\r\n"
                "Set-Cookie: auth=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; SameSite=Strict\r\n"
                // Add WWW-Authenticate header with an invalid realm to clear browser's auth cache
                "WWW-Authenticate: Basic realm=\"logout\"\r\n"
                // Add Cache-Control headers to prevent caching
                "Cache-Control: no-cache, no-store, must-revalidate\r\n"
                "Pragma: no-cache\r\n"
                "Expires: 0\r\n"
                "Content-Length: 0\r\n\r\n");
        
        // Ensure the connection is closed properly
        c->is_draining = 1;
    }
    
    log_info("Successfully handled POST /api/auth/logout request");
}

/**
 * @brief Direct handler for POST /api/auth/login
 */
void mg_handle_auth_login(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/auth/login request");
    
    // Always try to extract form data first
    char username[64] = {0};
    char password[64] = {0};
    bool is_form = false;
    
    // Log the request body for debugging
    char body_str[256] = {0};
    if (hm->body.len < sizeof(body_str) - 1) {
        memcpy(body_str, hm->body.buf, hm->body.len);
        body_str[hm->body.len] = '\0';
        log_info("Login request body: %s", body_str);
    }
    
    // Try to extract form data
    int username_len = mg_http_get_var(&hm->body, "username", username, sizeof(username));
    int password_len = mg_http_get_var(&hm->body, "password", password, sizeof(password));
    
    if (username_len > 0 && password_len > 0) {
        // Successfully extracted form data
        is_form = true;
        log_info("Extracted form data: username=%s", username);
    } else {
        // Try to parse as JSON
        cJSON *login = mg_parse_json_body(hm);
        if (!login) {
            // If we can't parse as JSON and didn't get form data, try one more approach
            // Some browsers might send form data without proper Content-Type
            if (hm->body.len > 0) {
                // Try to manually parse the form data
                char *body_copy = malloc(hm->body.len + 1);
                if (body_copy) {
                    memcpy(body_copy, hm->body.buf, hm->body.len);
                    body_copy[hm->body.len] = '\0';
                    
                    // Look for username=value
                    char *username_start = strstr(body_copy, "username=");
                    if (username_start) {
                        username_start += 9; // Skip "username="
                        char *username_end = strchr(username_start, '&');
                        if (username_end) {
                            *username_end = '\0';
                            strncpy(username, username_start, sizeof(username) - 1);
                            
                            // Look for password=value
                            char *password_start = strstr(username_end + 1, "password=");
                            if (password_start) {
                                password_start += 9; // Skip "password="
                                char *password_end = strchr(password_start, '&');
                                if (password_end) {
                                    *password_end = '\0';
                                }
                                strncpy(password, password_start, sizeof(password) - 1);
                                is_form = true;
                            }
                        }
                    }
                    
                    free(body_copy);
                }
            }
            
            if (!is_form) {
                log_error("Failed to parse login data from request body");
                mg_send_json_error(c, 400, "Invalid login data");
                return;
            }
        } else {
            // Extract username and password from JSON
            cJSON *username_json = cJSON_GetObjectItem(login, "username");
            cJSON *password_json = cJSON_GetObjectItem(login, "password");
            
            if (!username_json || !cJSON_IsString(username_json) || 
                !password_json || !cJSON_IsString(password_json)) {
                log_error("Missing or invalid username/password in login request");
                cJSON_Delete(login);
                mg_send_json_error(c, 400, "Missing or invalid username/password");
                return;
            }
            
            strncpy(username, username_json->valuestring, sizeof(username) - 1);
            strncpy(password, password_json->valuestring, sizeof(password) - 1);
            
            // Clean up
            cJSON_Delete(login);
        }
    }
    
    // Check credentials
    if (strcmp(username, g_config.web_username) == 0 && 
        strcmp(password, g_config.web_password) == 0) {
        // Login successful
        log_info("Login successful for user: %s", username);
        
        // Create Basic Auth header value
        char auth_credentials[128];
        snprintf(auth_credentials, sizeof(auth_credentials), "%s:%s", username, password);
        
        // Base64 encode the credentials
        char encoded_auth[256];
        mg_base64_encode((unsigned char *)auth_credentials, strlen(auth_credentials), encoded_auth, sizeof(encoded_auth));
        
        // Default redirect to index.html
        const char *redirect_url = "/index.html";
        
        // Check if this is a form submission or an API request
        struct mg_str *content_type = mg_http_get_header(hm, "Content-Type");
        struct mg_str *requested_with = mg_http_get_header(hm, "X-Requested-With");
        
        // Check if content_type contains "application/json"
        bool is_json = false;
        if (content_type) {
            const char *json_str = "application/json";
            size_t json_len = strlen(json_str);
            
            if (content_type->len >= json_len) {
                for (size_t i = 0; i <= content_type->len - json_len; i++) {
                    if (strncmp(content_type->buf + i, json_str, json_len) == 0) {
                        is_json = true;
                        break;
                    }
                }
            }
        }
        
        bool is_api_request = is_json || requested_with != NULL;
        
        if (is_api_request) {
            // For API requests, return a JSON success response
            cJSON *response = cJSON_CreateObject();
            cJSON_AddBoolToObject(response, "success", 1);
            cJSON_AddStringToObject(response, "redirect", redirect_url);
            
            // Send JSON response with auth cookie
            mg_printf(c, "HTTP/1.1 200 OK\r\n");
            mg_printf(c, "Content-Type: application/json\r\n");
            mg_printf(c, "Authorization: Basic %s\r\n", encoded_auth);
            mg_printf(c, "Set-Cookie: auth=%s; Path=/; Max-Age=86400; SameSite=Lax\r\n", encoded_auth);
            mg_printf(c, "Cache-Control: no-cache, no-store, must-revalidate\r\n");
            mg_printf(c, "Pragma: no-cache\r\n");
            mg_printf(c, "Expires: 0\r\n");
            
            char *json_str = cJSON_PrintUnformatted(response);
            mg_printf(c, "Content-Length: %d\r\n\r\n", (int)strlen(json_str));
            mg_printf(c, "%s", json_str);
            
            cJSON_Delete(response);
            free(json_str);
            
            // Ensure the connection is closed properly
            c->is_draining = 1;
        } else {
            // For form submissions, send a redirect response
            mg_printf(c, "HTTP/1.1 302 Found\r\n");
            mg_printf(c, "Location: %s\r\n", redirect_url);
            mg_printf(c, "Authorization: Basic %s\r\n", encoded_auth);
            // Set a cookie with the auth token to help maintain session across pages
            mg_printf(c, "Set-Cookie: auth=%s; Path=/; Max-Age=86400; SameSite=Lax\r\n", encoded_auth);
            // Add Cache-Control headers to prevent caching
            mg_printf(c, "Cache-Control: no-cache, no-store, must-revalidate\r\n");
            mg_printf(c, "Pragma: no-cache\r\n");
            mg_printf(c, "Expires: 0\r\n");
            mg_printf(c, "Content-Length: 0\r\n");
            mg_printf(c, "\r\n");
            
            // Ensure the connection is closed properly
            c->is_draining = 1;
        }
    } else {
        // Login failed
        log_warn("Login failed for user: %s", username);
        
        if (is_form) {
            // For form submissions, redirect back to login page with error
            mg_printf(c, "HTTP/1.1 302 Found\r\n");
            mg_printf(c, "Location: /login.html?error=1\r\n");
            mg_printf(c, "Content-Length: 0\r\n");
            mg_printf(c, "\r\n");
            
            // Ensure the connection is closed properly
            c->is_draining = 1;
        } else {
            // For API requests, return JSON error
            mg_send_json_error(c, 401, "Invalid username or password");
            
            // Ensure the connection is closed properly
            c->is_draining = 1;
        }
    }
    
    log_info("Successfully handled POST /api/auth/login request");
}
