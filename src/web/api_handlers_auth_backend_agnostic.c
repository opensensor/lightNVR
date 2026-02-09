/**
 * @file api_handlers_auth_backend_agnostic.c
 * @brief Backend-agnostic authentication handlers (login, logout, verify)
 */

#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <cjson/cJSON.h>

#include "web/api_handlers.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "core/logger.h"
#include "core/config.h"
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
 * @brief Helper to parse form-encoded body (username=...&password=...)
 */
static int parse_form_credentials(const char *body, size_t body_len, char *username, size_t username_size, char *password, size_t password_size) {
    if (!body || body_len == 0) {
        return -1;
    }

    // Make a copy of the body to work with
    char *body_copy = malloc(body_len + 1);
    if (!body_copy) {
        return -1;
    }
    memcpy(body_copy, body, body_len);
    body_copy[body_len] = '\0';

    int result = -1;

    // Look for username=value
    char *username_start = strstr(body_copy, "username=");
    if (username_start) {
        username_start += 9; // Skip "username="
        char *username_end = strchr(username_start, '&');
        if (username_end) {
            *username_end = '\0';
        }
        
        // URL decode username
        url_decode(username_start, username, username_size);

        // Look for password=value
        char *password_start = strstr(username_end ? username_end + 1 : body_copy, "password=");
        if (password_start) {
            password_start += 9; // Skip "password="
            char *password_end = strchr(password_start, '&');
            if (password_end) {
                *password_end = '\0';
            }
            
            // URL decode password
            url_decode(password_start, password, password_size);
            result = 0;
        }
    }

    free(body_copy);
    return result;
}

/**
 * @brief Backend-agnostic handler for POST /api/auth/login
 */
void handle_auth_login(const http_request_t *req, http_response_t *res) {
    log_info("Handling POST /api/auth/login request");

    char username[64] = {0};
    char password[64] = {0};
    bool is_form = false;

    // Check Content-Type to determine if it's form data or JSON
    const char *content_type = http_request_get_header(req, "Content-Type");
    
    if (content_type && strstr(content_type, "application/x-www-form-urlencoded")) {
        // Parse form data
        if (parse_form_credentials(req->body, req->body_len, username, sizeof(username), password, sizeof(password)) == 0) {
            is_form = true;
            log_info("Extracted form data: username=%s", username);
        }
    }
    
    if (!is_form) {
        // Try to parse as JSON
        cJSON *login = httpd_parse_json_body(req);
        if (!login) {
            // Last attempt: try parsing as form data even without proper Content-Type
            if (req->body && req->body_len > 0) {
                if (parse_form_credentials(req->body, req->body_len, username, sizeof(username), password, sizeof(password)) == 0) {
                    is_form = true;
                }
            }
            
            if (!is_form) {
                log_error("Failed to parse login data from request body");
                http_response_set_json_error(res, 400, "Invalid login data");
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
                http_response_set_json_error(res, 400, "Missing or invalid username/password");
                return;
            }

            strncpy(username, username_json->valuestring, sizeof(username) - 1);
            strncpy(password, password_json->valuestring, sizeof(password) - 1);

            cJSON_Delete(login);
        }
    }

    // Check credentials using the database authentication system
    int64_t user_id;
    int rc = db_auth_authenticate(username, password, &user_id);

    if (rc != 0) {
        // Try config-based authentication fallback for backward compatibility
        if (strcmp(username, g_config.web_username) == 0 &&
            strcmp(password, g_config.web_password) == 0 &&
            g_config.web_password[0] != '\0') {
            // Config-based auth successful
            log_info("Login successful using config-based authentication for user: %s", username);

            // Try to get or create a database user for this config user
            // This allows session-based auth to work with config credentials
            user_t user;
            rc = db_auth_get_user_by_username(username, &user);
            if (rc != 0) {
                // User doesn't exist in database, create it with the config password
                log_info("Creating database user for config-based auth: %s", username);
                rc = db_auth_create_user(username, password, NULL, USER_ROLE_ADMIN, true, NULL);
                if (rc != 0) {
                    log_error("Failed to create database user for config auth");
                    http_response_set_json_error(res, 500, "Failed to create user session");
                    return;
                }
                // Get the newly created user
                rc = db_auth_get_user_by_username(username, &user);
                if (rc != 0) {
                    log_error("Failed to retrieve newly created user");
                    http_response_set_json_error(res, 500, "Failed to create user session");
                    return;
                }
            }
            user_id = user.id;
        } else {
            // Login failed
            log_warn("Login failed for user: %s", username);

            if (is_form) {
                // For form submissions, send redirect to login page with error
                http_response_add_header(res, "Location", "/login.html?error=1");
                res->status_code = 302;
                res->body = NULL;
                res->body_length = 0;
            } else {
                // For API requests, return JSON error
                http_response_set_json_error(res, 401, "Invalid username or password");
            }
            return;
        }
    }

    // Login successful
    log_info("Login successful for user: %s (ID: %lld)", username, (long long)user_id);

    // Check if user has TOTP enabled (only for API/JSON requests)
    if (!is_form) {
        char totp_secret[64] = {0};
        bool totp_enabled = false;
        if (db_auth_get_totp_info(user_id, totp_secret, sizeof(totp_secret), &totp_enabled) == 0 && totp_enabled) {
            // Create a short-lived pending MFA session (5 minutes)
            char totp_token[33];
            rc = db_auth_create_session(user_id, NULL, NULL, 300, totp_token, sizeof(totp_token));
            if (rc != 0) {
                log_error("Failed to create pending MFA session for user: %s", username);
                http_response_set_json_error(res, 500, "Failed to create MFA session");
                return;
            }

            // Return TOTP required response (NO Set-Cookie header)
            cJSON *response = cJSON_CreateObject();
            cJSON_AddBoolToObject(response, "totp_required", true);
            cJSON_AddStringToObject(response, "totp_token", totp_token);

            char *json_str = cJSON_PrintUnformatted(response);
            http_response_set_json(res, 200, json_str);
            free(json_str);
            cJSON_Delete(response);

            log_info("TOTP verification required for user: %s", username);
            return;
        }
    }

    // Create a session token using configured timeout
    int session_timeout_seconds = g_config.auth_timeout_hours * 3600;
    char token[33];
    rc = db_auth_create_session(user_id, NULL, NULL, session_timeout_seconds, token, sizeof(token));

    if (rc != 0) {
        log_error("Failed to create session for user: %s", username);
        http_response_set_json_error(res, 500, "Failed to create session");
        return;
    }

    // Set session cookie
    char cookie_header[256];
    snprintf(cookie_header, sizeof(cookie_header),
             "session=%s; Path=/; Max-Age=%d; HttpOnly; SameSite=Lax",
             token, session_timeout_seconds);
    http_response_add_header(res, "Set-Cookie", cookie_header);

    if (is_form) {
        // For form submissions, redirect to index.html
        http_response_add_header(res, "Location", "/index.html");
        res->status_code = 302;
        res->body = NULL;
        res->body_length = 0;
    } else {
        // For API requests, return JSON success
        cJSON *response = cJSON_CreateObject();
        cJSON_AddBoolToObject(response, "success", true);
        cJSON_AddStringToObject(response, "redirect", "/index.html");

        char *json_str = cJSON_PrintUnformatted(response);
        http_response_set_json(res, 200, json_str);
        free(json_str);
        cJSON_Delete(response);
    }

    log_info("Session created successfully for user: %s", username);
}

/**
 * @brief Backend-agnostic handler for POST /api/auth/logout and GET /logout
 */
void handle_auth_logout(const http_request_t *req, http_response_t *res) {
    log_info("Handling logout request");

    // Check for session cookie and invalidate it
    const char *cookie_header = http_request_get_header(req, "Cookie");
    if (cookie_header) {
        // Look for session cookie
        const char *session_start = strstr(cookie_header, "session=");
        if (session_start) {
            session_start += 8; // Skip "session="
            const char *session_end = strchr(session_start, ';');
            if (!session_end) {
                session_end = session_start + strlen(session_start);
            }

            char session_token[64] = {0};
            size_t token_len = session_end - session_start;
            if (token_len < sizeof(session_token)) {
                memcpy(session_token, session_start, token_len);
                session_token[token_len] = '\0';

                // Invalidate the session
                db_auth_delete_session(session_token);
                log_info("Session deleted: %s", session_token);
            }
        }
    }

    // Clear the session cookie
    http_response_add_header(res, "Set-Cookie", "session=; Path=/; Max-Age=0; HttpOnly");

    // Check if this is an API request or browser request
    const char *accept = http_request_get_header(req, "Accept");
    const char *requested_with = http_request_get_header(req, "X-Requested-With");
    bool is_api_request = (accept && strstr(accept, "application/json")) || requested_with;

    if (is_api_request) {
        // For API requests, return JSON success
        cJSON *response = cJSON_CreateObject();
        cJSON_AddBoolToObject(response, "success", true);
        cJSON_AddStringToObject(response, "redirect", "/login.html?logout=true");

        char *json_str = cJSON_PrintUnformatted(response);
        http_response_set_json(res, 200, json_str);
        free(json_str);
        cJSON_Delete(response);
    } else {
        // For browser requests, redirect to login page
        http_response_add_header(res, "Location", "/login.html?logout=true");
        res->status_code = 302;
        res->body = NULL;
        res->body_length = 0;
    }

    log_info("Logout successful");
}

/**
 * @brief Backend-agnostic handler for GET /api/auth/verify
 */
void handle_auth_verify(const http_request_t *req, http_response_t *res) {
    log_info("Handling GET /api/auth/verify request");

    // First, check for session token in cookie
    const char *cookie_header = http_request_get_header(req, "Cookie");
    if (cookie_header) {
        // Look for session cookie
        const char *session_start = strstr(cookie_header, "session=");
        if (session_start) {
            session_start += 8; // Skip "session="
            const char *session_end = strchr(session_start, ';');
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

                        // Send success response with user info
                        cJSON *response = cJSON_CreateObject();
                        cJSON_AddBoolToObject(response, "authenticated", true);
                        cJSON_AddStringToObject(response, "username", user.username);
                        cJSON_AddStringToObject(response, "role", db_auth_get_role_name(user.role));

                        char *json_str = cJSON_PrintUnformatted(response);
                        http_response_set_json(res, 200, json_str);
                        free(json_str);
                        cJSON_Delete(response);
                        return;
                    }
                }
            }
        }
    }

    // If no valid session, try HTTP Basic Auth
    user_t user;
    if (httpd_get_authenticated_user(req, &user)) {
        log_info("Authentication successful for user: %s (role: %s)", user.username, db_auth_get_role_name(user.role));

        // Send success response with user info
        cJSON *response = cJSON_CreateObject();
        cJSON_AddBoolToObject(response, "authenticated", true);
        cJSON_AddStringToObject(response, "username", user.username);
        cJSON_AddStringToObject(response, "role", db_auth_get_role_name(user.role));

        char *json_str = cJSON_PrintUnformatted(response);
        http_response_set_json(res, 200, json_str);
        free(json_str);
        cJSON_Delete(response);
        return;
    }

    // No valid authentication
    log_debug("Authentication verification failed");
    http_response_set_json_error(res, 401, "Unauthorized");
}

