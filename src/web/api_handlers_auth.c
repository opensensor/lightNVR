#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <cjson/cJSON.h>

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
 * @brief Helper to send verify success response with user info
 */
static void send_verify_success(struct mg_connection *c, const char *username, user_role_t role) {
    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "authenticated", 1);
    cJSON_AddStringToObject(response, "username", username);
    cJSON_AddStringToObject(response, "role", db_auth_get_role_name(role));
    cJSON_AddNumberToObject(response, "role_id", (int)role);

    char *json_str = cJSON_PrintUnformatted(response);
    int json_len = (int)strlen(json_str);

    mg_printf(c, "HTTP/1.1 200 OK\r\n"
            "Content-Type: application/json\r\n"
            "Connection: close\r\n"
            "Content-Length: %d\r\n\r\n%s", json_len, json_str);

    free(json_str);
    cJSON_Delete(response);
}

/**
 * @brief Handle GET /api/auth/verify
 * Endpoint to verify authentication and return user info including role
 */
void mg_handle_auth_verify(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/auth/verify request");

    // First, check for session token in cookie
    struct mg_str *cookie = mg_http_get_header(hm, "Cookie");
    if (cookie) {
        // Parse cookie header manually (cookies use ';' separator, not '&')
        char cookie_str[1024] = {0};
        size_t cookie_len = cookie->len < sizeof(cookie_str) - 1 ? cookie->len : sizeof(cookie_str) - 1;
        memcpy(cookie_str, cookie->buf, cookie_len);
        cookie_str[cookie_len] = '\0';

        // Look for session cookie
        char *session_start = strstr(cookie_str, "session=");
        if (session_start) {
            session_start += 8; // Skip "session="
            char *session_end = strchr(session_start, ';');
            if (!session_end) {
                session_end = session_start + strlen(session_start);
            }

            char session_token[64] = {0};
            size_t token_len = session_end - session_start;
            if (token_len < sizeof(session_token)) {
                memcpy(session_token, session_start, token_len);
                session_token[token_len] = '\0';

                // Validate the session token
                int64_t user_id;
                int rc = db_auth_validate_session(session_token, &user_id);
                if (rc == 0) {
                    // Session is valid - get user info
                    user_t user;
                    if (db_auth_get_user_by_id(user_id, &user) == 0) {
                        log_info("Authentication successful with session token for user: %s", user.username);
                        send_verify_success(c, user.username, user.role);
                        return;
                    }
                    // Fallback if user lookup fails
                    log_info("Authentication successful with session token (user lookup failed)");
                    send_verify_success(c, "unknown", USER_ROLE_VIEWER);
                    return;
                }
                log_debug("Invalid session token, falling back to other auth methods");
            }
        }

        // If session token not found or invalid, check for auth cookie (backward compatibility)
        char *auth_start = strstr(cookie_str, "auth=");
        if (auth_start) {
            auth_start += 5; // Skip "auth="
            char *auth_end = strchr(auth_start, ';');
            if (!auth_end) {
                auth_end = auth_start + strlen(auth_start);
            }

            char auth_cookie[256] = {0};
            size_t auth_len = auth_end - auth_start;
            if (auth_len < sizeof(auth_cookie)) {
                memcpy(auth_cookie, auth_start, auth_len);
                auth_cookie[auth_len] = '\0';

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
                        // Authentication successful - get user info
                        user_t user;
                        if (db_auth_get_user_by_id(user_id, &user) == 0) {
                            log_info("Authentication successful with auth cookie for user: %s", user.username);
                            send_verify_success(c, user.username, user.role);
                            return;
                        }
                    }

                    // Fall back to config-based authentication
                    if (strcmp(username, g_config.web_username) == 0 &&
                        strcmp(password, g_config.web_password) == 0) {
                        // Authentication successful with config credentials - assume admin role
                        log_info("Authentication successful with config credentials (from cookie)");
                        send_verify_success(c, username, USER_ROLE_ADMIN);
                        return;
                    }
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
        // No credentials provided - return JSON error
        mg_send_json_error(c, 401, "Unauthorized");
        return;
    }

    // Authenticate the user
    int64_t user_id;
    int rc = db_auth_authenticate(username, password, &user_id);

    if (rc != 0) {
        // Fall back to config-based authentication for backward compatibility
        if (strcmp(username, g_config.web_username) == 0 &&
            strcmp(password, g_config.web_password) == 0) {
            // Authentication successful with config credentials - assume admin role
            log_info("Authentication successful with config credentials (from Basic Auth)");
            send_verify_success(c, username, USER_ROLE_ADMIN);
            return;
        }

        // Authentication failed
        log_warn("Authentication failed for user: %s", username);
        mg_send_json_error(c, 401, "Unauthorized");
        return;
    }

    // Authentication successful - get user info
    user_t user;
    if (db_auth_get_user_by_id(user_id, &user) == 0) {
        log_info("Authentication successful for user: %s (role: %s)", user.username, db_auth_get_role_name(user.role));
        send_verify_success(c, user.username, user.role);
    } else {
        // Fallback if user lookup fails
        log_info("Authentication successful for user: %s (user lookup failed)", username);
        send_verify_success(c, username, USER_ROLE_VIEWER);
    }
}

/**
 * @brief Direct handler for /api/auth/logout and /logout
 */
void mg_handle_auth_logout(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling logout request");

    // Check if this is an API request (POST method or has JSON content type or X-Requested-With header)
    bool is_post = mg_strcmp(hm->method, mg_str("POST")) == 0;
    struct mg_str *content_type = mg_http_get_header(hm, "Content-Type");
    struct mg_str *requested_with = mg_http_get_header(hm, "X-Requested-With");
    struct mg_str *accept = mg_http_get_header(hm, "Accept");

    // Check if Accept header wants JSON
    bool accepts_json = false;
    if (accept) {
        accepts_json = memmem(accept->buf, accept->len, "application/json", 16) != NULL;
    }

    // Check if content_type contains "application/json"
    bool is_json = false;
    if (content_type) {
        is_json = memmem(content_type->buf, content_type->len, "application/json", 16) != NULL;
    }

    bool is_api_request = is_post || is_json || requested_with != NULL || accepts_json;

    if (is_api_request) {
        // For API requests, return a JSON success response
        cJSON *response = cJSON_CreateObject();
        cJSON_AddBoolToObject(response, "success", 1);
        cJSON_AddStringToObject(response, "redirect", "/login.html?logout=true");

        char *json_response = cJSON_PrintUnformatted(response);
        int json_len = (int)strlen(json_response);

        // Send JSON response with cleared cookies
        mg_printf(c, "HTTP/1.1 200 OK\r\n"
                "Content-Type: application/json\r\n"
                "Set-Cookie: auth=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; SameSite=Lax\r\n"
                "Set-Cookie: session=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; SameSite=Lax\r\n"
                "Cache-Control: no-cache, no-store, must-revalidate\r\n"
                "Pragma: no-cache\r\n"
                "Connection: close\r\n"
                "Content-Length: %d\r\n\r\n%s", json_len, json_response);

        cJSON_Delete(response);
        free(json_response);

        log_info("Sent JSON logout response");
    } else {
        // For direct browser navigation (GET request), send a redirect response
        // Use 303 See Other for a cleaner redirect after logout
        mg_printf(c, "HTTP/1.1 303 See Other\r\n"
                "Location: /login.html?logout=true\r\n"
                "Set-Cookie: auth=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; SameSite=Lax\r\n"
                "Set-Cookie: session=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; SameSite=Lax\r\n"
                "Cache-Control: no-cache, no-store, must-revalidate\r\n"
                "Pragma: no-cache\r\n"
                "Content-Length: 0\r\n"
                "Connection: close\r\n\r\n");

        log_info("Sent redirect logout response");
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

            // Calculate session timeout from config
            int auth_timeout_seconds = g_config.auth_timeout_hours * 3600;

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

                // Send JSON response with auth cookie (using configured timeout)
                mg_printf(c, "HTTP/1.1 200 OK\r\n");
                mg_printf(c, "Content-Type: application/json\r\n");
                mg_printf(c, "Authorization: Basic %s\r\n", encoded_auth);
                mg_printf(c, "Set-Cookie: auth=%s; Path=/; Max-Age=%d; SameSite=Lax\r\n", encoded_auth, auth_timeout_seconds);
                mg_printf(c, "Cache-Control: no-cache, no-store, must-revalidate\r\n");
                mg_printf(c, "Pragma: no-cache\r\n");
                mg_printf(c, "Expires: 0\r\n");
                mg_printf(c, "Connection: close\r\n");

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
                // Set a cookie with the auth token to help maintain session across pages (using configured timeout)
                mg_printf(c, "Set-Cookie: auth=%s; Path=/; Max-Age=%d; SameSite=Lax\r\n", encoded_auth, auth_timeout_seconds);
                // Add Cache-Control headers to prevent caching
                mg_printf(c, "Cache-Control: no-cache, no-store, must-revalidate\r\n");
                mg_printf(c, "Pragma: no-cache\r\n");
                mg_printf(c, "Expires: 0\r\n");
                mg_printf(c, "Connection: close\r\n");
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
            mg_printf(c, "Connection: close\r\n");
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

    // Create a session token using configured timeout
    int session_timeout_seconds = g_config.auth_timeout_hours * 3600;
    char token[33];
    rc = db_auth_create_session(user_id, NULL, NULL, session_timeout_seconds, token, sizeof(token));
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
            
            // Send JSON response with auth cookie (using configured timeout)
            mg_printf(c, "HTTP/1.1 200 OK\r\n");
            mg_printf(c, "Content-Type: application/json\r\n");
            mg_printf(c, "Authorization: Basic %s\r\n", encoded_auth);
            mg_printf(c, "Set-Cookie: auth=%s; Path=/; Max-Age=%d; SameSite=Lax\r\n", encoded_auth, session_timeout_seconds);
            mg_printf(c, "Cache-Control: no-cache, no-store, must-revalidate\r\n");
            mg_printf(c, "Pragma: no-cache\r\n");
            mg_printf(c, "Expires: 0\r\n");
            mg_printf(c, "Connection: close\r\n");

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
            // Set a cookie with the auth token to help maintain session across pages (using configured timeout)
            mg_printf(c, "Set-Cookie: auth=%s; Path=/; Max-Age=%d; SameSite=Lax\r\n", encoded_auth, session_timeout_seconds);
            // Add Cache-Control headers to prevent caching
            mg_printf(c, "Cache-Control: no-cache, no-store, must-revalidate\r\n");
            mg_printf(c, "Pragma: no-cache\r\n");
            mg_printf(c, "Expires: 0\r\n");
            mg_printf(c, "Connection: close\r\n");
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

            // Send JSON response with session token cookie (using configured timeout)
            mg_printf(c, "HTTP/1.1 200 OK\r\n");
            mg_printf(c, "Content-Type: application/json\r\n");
            mg_printf(c, "Set-Cookie: session=%s; Path=/; Max-Age=%d; SameSite=Lax\r\n", token, session_timeout_seconds);
            mg_printf(c, "Cache-Control: no-cache, no-store, must-revalidate\r\n");
            mg_printf(c, "Pragma: no-cache\r\n");
            mg_printf(c, "Expires: 0\r\n");
            mg_printf(c, "Connection: close\r\n");

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
            // Set a cookie with the session token to maintain session across pages (using configured timeout)
            mg_printf(c, "Set-Cookie: session=%s; Path=/; Max-Age=%d; SameSite=Lax\r\n", token, session_timeout_seconds);
            // Add Cache-Control headers to prevent caching
            mg_printf(c, "Cache-Control: no-cache, no-store, must-revalidate\r\n");
            mg_printf(c, "Pragma: no-cache\r\n");
            mg_printf(c, "Expires: 0\r\n");
            mg_printf(c, "Connection: close\r\n");
            mg_printf(c, "Content-Length: 0\r\n");
            mg_printf(c, "\r\n");

            // Ensure the connection is closed properly
            c->is_draining = 1;
        }
    }
    
    log_info("Successfully handled POST /api/auth/login request");
}
