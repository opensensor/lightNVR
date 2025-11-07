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
#include "database/db_auth.h"

/**
 * @brief Initialize the authentication system
 */
int init_auth_system(void) {
    log_info("Initializing authentication system");
    
    // Initialize the database authentication system
    int rc = db_auth_init();
    if (rc != 0) {
        log_error("Failed to initialize database authentication system");
        return -1;
    }
    
    log_info("Authentication system initialized successfully");
    return 0;
}

/**
 * @brief Handle GET /api/auth/verify
 * Lightweight endpoint to verify authentication without returning data
 */
void mg_handle_auth_verify(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/auth/verify request");
    
    // First, check for session token in cookie
    struct mg_str *cookie = mg_http_get_header(hm, "Cookie");
    if (cookie) {
        char session_token[64] = {0};
        if (mg_http_get_var(cookie, "session", session_token, sizeof(session_token)) > 0) {
            // Validate the session token
            int64_t user_id;
            int rc = db_auth_validate_session(session_token, &user_id);
            if (rc == 0) {
                // Session is valid
                log_info("Authentication successful with session token");
                mg_printf(c, "HTTP/1.1 200 OK\r\n");
                mg_printf(c, "Content-Length: 0\r\n\r\n");
                return;
            }
            log_debug("Invalid session token, falling back to other auth methods");
        }
        
        // If session token not found or invalid, check for auth cookie (backward compatibility)
        char auth_cookie[256] = {0};
        if (mg_http_get_var(cookie, "auth", auth_cookie, sizeof(auth_cookie)) > 0) {
            // Decode the auth cookie (base64 encoded username:password)
            char decoded[128] = {0};
            mg_base64_decode(auth_cookie, strlen(auth_cookie), decoded, sizeof(decoded));
            
            // Split the decoded string into username and password
            char *colon = strchr(decoded, ':');
            if (colon) {
                char username[64] = {0};
                char password[64] = {0};
                
                *colon = '\0';
                strncpy(username, decoded, sizeof(username) - 1);
                strncpy(password, colon + 1, sizeof(password) - 1);
                
                // Authenticate with username/password
                int64_t user_id;
                int rc = db_auth_authenticate(username, password, &user_id);
                
                if (rc == 0) {
                    // Authentication successful
                    log_info("Authentication successful with auth cookie");
                    mg_printf(c, "HTTP/1.1 200 OK\r\n");
                    mg_printf(c, "Content-Length: 0\r\n\r\n");
                    return;
                }
                
                // Fall back to config-based authentication
                if (strcmp(username, g_config.web_username) == 0 && 
                    strcmp(password, g_config.web_password) == 0) {
                    // Authentication successful with config credentials
                    log_info("Authentication successful with config credentials (from cookie)");
                    mg_printf(c, "HTTP/1.1 200 OK\r\n");
                    mg_printf(c, "Content-Length: 0\r\n\r\n");
                    return;
                }
            }
        }
    }
    
    // If no valid cookie auth, try HTTP Basic Auth
    char username[64] = {0};
    char password[64] = {0};
    
    mg_http_creds(hm, username, sizeof(username), password, sizeof(password));
    
    // Check if we have credentials
    if (username[0] == '\0' || password[0] == '\0') {
        // No credentials provided - return JSON error instead of WWW-Authenticate
        mg_printf(c, "HTTP/1.1 401 Unauthorized\r\n");
        mg_printf(c, "Content-Type: application/json\r\n");
        mg_printf(c, "Content-Length: 29\r\n\r\n");
        mg_printf(c, "{\"error\": \"Unauthorized\"}\n");
        return;
    }
    
    // Authenticate the user
    int64_t user_id;
    int rc = db_auth_authenticate(username, password, &user_id);
    
    if (rc != 0) {
        // Fall back to config-based authentication for backward compatibility
        if (strcmp(username, g_config.web_username) == 0 && 
            strcmp(password, g_config.web_password) == 0) {
            // Authentication successful with config credentials
            log_info("Authentication successful with config credentials (from Basic Auth)");
            mg_printf(c, "HTTP/1.1 200 OK\r\n");
            mg_printf(c, "Content-Length: 0\r\n\r\n");
            return;
        }
        
        // Authentication failed - return JSON error instead of WWW-Authenticate
        log_warn("Authentication failed for user: %s", username);
        mg_printf(c, "HTTP/1.1 401 Unauthorized\r\n");
        mg_printf(c, "Content-Type: application/json\r\n");
        mg_printf(c, "Content-Length: 29\r\n\r\n");
        mg_printf(c, "{\"error\": \"Unauthorized\"}\n");
        return;
    }
    
    // Authentication successful
    log_info("Authentication successful for user: %s (ID: %lld)", username, (long long)user_id);
    mg_printf(c, "HTTP/1.1 200 OK\r\n");
    mg_printf(c, "Content-Length: 0\r\n\r\n");
}

/**
 * @brief Direct handler for /api/auth/logout and /logout
 */
void mg_handle_auth_logout(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling logout request");
    
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
        cJSON_AddStringToObject(response, "redirect", "/login.html?auth_required=true&logout=true");
        
        // Send JSON response with cleared cookies
        mg_printf(c, "HTTP/1.1 401 Unauthorized\r\n");  // Use 401 to help clear browser auth cache
        mg_printf(c, "Content-Type: application/json\r\n");
        mg_printf(c, "Set-Cookie: auth=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; SameSite=Strict\r\n");
        mg_printf(c, "Set-Cookie: session=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; SameSite=Strict\r\n");
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
                "Location: /login.html?auth_required=true&logout=true\r\n"
                "Set-Cookie: auth=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; SameSite=Strict\r\n"
                "Set-Cookie: session=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; SameSite=Strict\r\n"
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
    
    log_info("Successfully handled logout request");
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
    
    // Check credentials using the database authentication system
    int64_t user_id;
    int rc = db_auth_authenticate(username, password, &user_id);
    
    if (rc != 0) {
        // Fall back to config-based authentication for backward compatibility
        if (strcmp(username, g_config.web_username) == 0 && 
            strcmp(password, g_config.web_password) == 0) {
            // Login successful with config credentials
            log_info("Login successful for user: %s (using config credentials)", username);
            
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
            
            return;
        }
        
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
        
        return;
    }
    
    // Login successful with database credentials
    log_info("Login successful for user: %s (ID: %lld)", username, (long long)user_id);

    // Create a session token (7 days = 604800 seconds)
    char token[33];
    rc = db_auth_create_session(user_id, NULL, NULL, 604800, token, sizeof(token));
    if (rc != 0) {
        log_error("Failed to create session for user: %s", username);
        
        // Fall back to Basic Auth
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
        // Login successful with session token
        log_info("Session created successfully for user: %s", username);
        
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
            cJSON_AddStringToObject(response, "token", token);
            
            // Send JSON response with session token cookie (7 days)
            mg_printf(c, "HTTP/1.1 200 OK\r\n");
            mg_printf(c, "Content-Type: application/json\r\n");
            mg_printf(c, "Set-Cookie: session=%s; Path=/; Max-Age=604800; SameSite=Lax\r\n", token);
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
            // Set a cookie with the session token to maintain session across pages (7 days)
            mg_printf(c, "Set-Cookie: session=%s; Path=/; Max-Age=604800; SameSite=Lax\r\n", token);
            // Add Cache-Control headers to prevent caching
            mg_printf(c, "Cache-Control: no-cache, no-store, must-revalidate\r\n");
            mg_printf(c, "Pragma: no-cache\r\n");
            mg_printf(c, "Expires: 0\r\n");
            mg_printf(c, "Content-Length: 0\r\n");
            mg_printf(c, "\r\n");
            
            // Ensure the connection is closed properly
            c->is_draining = 1;
        }
    }
    
    log_info("Successfully handled POST /api/auth/login request");
}
