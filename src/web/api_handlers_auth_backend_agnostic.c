/**
 * @file api_handlers_auth_backend_agnostic.c
 * @brief Backend-agnostic authentication handlers (login, logout, verify)
 */

#define _GNU_SOURCE

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <time.h>
#include <cjson/cJSON.h>

#include "web/api_handlers.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "web/api_handlers_totp.h"
#include "core/logger.h"
#include "core/config.h"
#include "database/db_auth.h"

/* ========== Login Rate Limiting ========== */

#define MAX_RATE_LIMIT_ENTRIES 256

typedef struct {
    char username[64];
    int attempt_count;
    time_t window_start;
} rate_limit_entry_t;

static rate_limit_entry_t rate_limit_table[MAX_RATE_LIMIT_ENTRIES];
static int rate_limit_count = 0;

/**
 * @brief Check if a login attempt is rate-limited
 * @param username The username being attempted
 * @return true if the attempt should be blocked, false if allowed
 */
static bool check_rate_limit(const char *username) {
    if (!g_config.login_rate_limit_enabled || !username) {
        return false;
    }

    time_t now = time(NULL);
    int max_attempts = g_config.login_rate_limit_max_attempts;
    int window = g_config.login_rate_limit_window_seconds;

    // Find existing entry for this username
    for (int i = 0; i < rate_limit_count; i++) {
        if (strcmp(rate_limit_table[i].username, username) == 0) {
            // Check if window has expired
            if (now - rate_limit_table[i].window_start > window) {
                // Reset window
                rate_limit_table[i].attempt_count = 0;
                rate_limit_table[i].window_start = now;
                return false;
            }
            // Check if over limit
            return rate_limit_table[i].attempt_count >= max_attempts;
        }
    }

    return false; // No entry found, not rate-limited
}

/**
 * @brief Record a failed login attempt for rate limiting
 * @param username The username that failed authentication
 */
static void record_failed_attempt(const char *username) {
    if (!g_config.login_rate_limit_enabled || !username) {
        return;
    }

    time_t now = time(NULL);
    int window = g_config.login_rate_limit_window_seconds;

    // Find existing entry
    for (int i = 0; i < rate_limit_count; i++) {
        if (strcmp(rate_limit_table[i].username, username) == 0) {
            // Reset window if expired
            if (now - rate_limit_table[i].window_start > window) {
                rate_limit_table[i].attempt_count = 1;
                rate_limit_table[i].window_start = now;
            } else {
                rate_limit_table[i].attempt_count++;
            }
            return;
        }
    }

    // Add new entry
    if (rate_limit_count < MAX_RATE_LIMIT_ENTRIES) {
        strncpy(rate_limit_table[rate_limit_count].username, username, 63);
        rate_limit_table[rate_limit_count].username[63] = '\0';
        rate_limit_table[rate_limit_count].attempt_count = 1;
        rate_limit_table[rate_limit_count].window_start = now;
        rate_limit_count++;
    } else {
        // Table full - evict oldest entry (simple strategy)
        int oldest_idx = 0;
        time_t oldest_time = rate_limit_table[0].window_start;
        for (int i = 1; i < MAX_RATE_LIMIT_ENTRIES; i++) {
            if (rate_limit_table[i].window_start < oldest_time) {
                oldest_time = rate_limit_table[i].window_start;
                oldest_idx = i;
            }
        }
        strncpy(rate_limit_table[oldest_idx].username, username, 63);
        rate_limit_table[oldest_idx].username[63] = '\0';
        rate_limit_table[oldest_idx].attempt_count = 1;
        rate_limit_table[oldest_idx].window_start = now;
    }
}

bool auth_clear_login_rate_limit_for_username(const char *username) {
    if (!username || username[0] == '\0') {
        return false;
    }

    for (int i = 0; i < rate_limit_count; i++) {
        if (strcmp(rate_limit_table[i].username, username) == 0) {
            rate_limit_table[i].attempt_count = 0;
            rate_limit_table[i].window_start = 0;
            return true;
        }
    }

    return false;
}

static bool request_has_valid_trusted_device(const http_request_t *req, int64_t user_id) {
    char trusted_token[128] = {0};
    if (httpd_trusted_device_lifetime_seconds() <= 0) {
        return false;
    }
    if (httpd_get_cookie_value(req, "trusted_device", trusted_token, sizeof(trusted_token)) != 0) {
        return false;
    }
    return db_auth_validate_trusted_device(user_id, trusted_token) == 0;
}

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
    char totp_code[8] = {0};  // Optional TOTP code for force-MFA mode
    char effective_client_ip[64] = {0};
    bool is_form = false;
    bool remember_device = false;
    bool trusted_device_used = false;
    bool totp_verified = false;

    if (httpd_get_effective_client_ip(req, effective_client_ip, sizeof(effective_client_ip)) != 0) {
        strncpy(effective_client_ip, req->client_ip, sizeof(effective_client_ip) - 1);
        effective_client_ip[sizeof(effective_client_ip) - 1] = '\0';
    }

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

            // Extract optional TOTP code (used in force-MFA mode)
            cJSON *totp_code_json = cJSON_GetObjectItem(login, "totp_code");
            if (totp_code_json && cJSON_IsString(totp_code_json)) {
                strncpy(totp_code, totp_code_json->valuestring, sizeof(totp_code) - 1);
            }

            cJSON *remember_device_json = cJSON_GetObjectItem(login, "remember_device");
            if (remember_device_json && cJSON_IsBool(remember_device_json)) {
                remember_device = cJSON_IsTrue(remember_device_json);
            }

            cJSON_Delete(login);
        }
    }

    // Check rate limiting before processing credentials
    if (check_rate_limit(username)) {
        log_warn("Login rate-limited for user: %s", username);

        if (is_form) {
            http_response_add_header(res, "Location", "/login.html?error=rate_limited");
            res->status_code = 302;
            res->body = NULL;
            res->body_length = 0;
        } else {
            http_response_set_json_error(res, 429, "Too many login attempts. Please try again later.");
        }
        return;
    }

    // Check credentials using the database authentication system
    int64_t user_id;
    int rc = db_auth_authenticate(username, password, &user_id);

    if (rc != 0) {
        // Login failed - record attempt for rate limiting
        record_failed_attempt(username);
        log_warn("Login failed for user: %s", username);

        if (is_form) {
            // For form submissions, send redirect to login page with error
            http_response_add_header(res, "Location", "/login.html?error=1");
            res->status_code = 302;
            res->body = NULL;
            res->body_length = 0;
        } else {
            // Use generic error message (same for password-only or password+TOTP failure)
            http_response_set_json_error(res, 401, "Invalid credentials");
        }
        return;
    }

    // Login successful (password verified)
    log_info("Password verified for user: %s (ID: %lld)", username, (long long)user_id);

    user_t authenticated_user;
    if (db_auth_get_user_by_id(user_id, &authenticated_user) != 0) {
        log_error("Failed to load authenticated user record for %s", username);
        http_response_set_json_error(res, 500, "Failed to load user");
        return;
    }

    if (!db_auth_ip_allowed_for_user(&authenticated_user, effective_client_ip)) {
        record_failed_attempt(username);
        log_warn("Login blocked by allowed_login_cidrs for user '%s' from IP %s",
                 username, effective_client_ip[0] != '\0' ? effective_client_ip : "(unknown)");

        if (is_form) {
            http_response_add_header(res, "Location", "/login.html?error=1");
            res->status_code = 302;
            res->body = NULL;
            res->body_length = 0;
        } else {
            http_response_set_json_error(res, 401, "Invalid credentials");
        }
        return;
    }

    // Check if user has TOTP enabled (only for API/JSON requests)
    if (!is_form) {
        char totp_secret[64] = {0};
        bool totp_enabled = false;
        if (db_auth_get_totp_info(user_id, totp_secret, sizeof(totp_secret), &totp_enabled) == 0 && totp_enabled) {
            bool trusted_device_valid = request_has_valid_trusted_device(req, user_id);

            // Force MFA mode: verify TOTP code in the same request
            if (g_config.force_mfa_on_login) {
                if (totp_code[0] == '\0') {
                    if (trusted_device_valid) {
                        trusted_device_used = true;
                        log_info("Trusted device accepted for user: %s", username);
                    } else {
                        // No TOTP code provided - return generic error
                        // Don't reveal that password was correct
                        record_failed_attempt(username);
                        log_warn("Force MFA: no TOTP code provided for user: %s", username);
                        http_response_set_json_error(res, 401, "Invalid credentials");
                        return;
                    }
                }

                if (!trusted_device_used) {
                    // Verify the TOTP code
                    if (totp_verify(totp_secret, totp_code) != 0) {
                        record_failed_attempt(username);
                        log_warn("Force MFA: invalid TOTP code for user: %s", username);
                        http_response_set_json_error(res, 401, "Invalid credentials");
                        return;
                    }
                    totp_verified = true;
                    log_info("Force MFA: TOTP verified for user: %s", username);
                }
                // Fall through to create session
            } else {
                if (totp_code[0] != '\0') {
                    if (totp_verify(totp_secret, totp_code) != 0) {
                        record_failed_attempt(username);
                        log_warn("Invalid inline TOTP code for user: %s", username);
                        http_response_set_json_error(res, 401, "Invalid credentials");
                        return;
                    }
                    totp_verified = true;
                } else if (trusted_device_valid) {
                    trusted_device_used = true;
                    log_info("Trusted device accepted for user: %s", username);
                } else {
                // Standard two-step MFA flow
                // Create a short-lived pending MFA session (5 minutes)
                char totp_token[33];
                rc = db_auth_create_session(user_id, effective_client_ip, req->user_agent, 300, totp_token, sizeof(totp_token));
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
        } else if (g_config.force_mfa_on_login && totp_code[0] != '\0') {
            // Force MFA is on, user provided a TOTP code but doesn't have TOTP enabled
            // Just ignore the code and proceed (user hasn't set up MFA yet)
            log_info("Force MFA: user %s has no TOTP configured, allowing login", username);
        }
    }

    // Clear rate limit on successful login
    (void)auth_clear_login_rate_limit_for_username(username);

    // Create a session token using the configured absolute session lifetime.
    char token[33];
    rc = db_auth_create_session(user_id, effective_client_ip, req->user_agent,
                                0, token, sizeof(token));

    if (rc != 0) {
        log_error("Failed to create session for user: %s", username);
        http_response_set_json_error(res, 500, "Failed to create session");
        return;
    }

    httpd_add_session_cookie(res, token);

    if (totp_verified && remember_device && !trusted_device_used && httpd_trusted_device_lifetime_seconds() > 0) {
        char trusted_token[33];
        if (db_auth_create_trusted_device(user_id, effective_client_ip, req->user_agent,
                                          httpd_trusted_device_lifetime_seconds(),
                                          trusted_token, sizeof(trusted_token)) == 0) {
            httpd_add_trusted_device_cookie(res, trusted_token);
        } else {
            log_warn("Failed to create trusted device for user: %s", username);
        }
    }

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

    char session_token[64] = {0};
    if (httpd_get_session_token(req, session_token, sizeof(session_token)) == 0) {
        db_auth_delete_session(session_token);
        log_info("Session deleted for logout request");
    }

    httpd_clear_session_cookie(res);

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

    // If authentication is disabled, return success immediately
    if (!g_config.web_auth_enabled) {
        log_info("Authentication is disabled, returning success for verify request");
        cJSON *response = cJSON_CreateObject();
        cJSON_AddBoolToObject(response, "authenticated", true);
        cJSON_AddStringToObject(response, "username", "admin");
        cJSON_AddStringToObject(response, "role", "admin");
        cJSON_AddBoolToObject(response, "auth_enabled", false);
        cJSON_AddNumberToObject(response, "auth_timeout_hours", g_config.auth_timeout_hours);
        cJSON_AddNumberToObject(response, "auth_absolute_timeout_hours", g_config.auth_absolute_timeout_hours);
        cJSON_AddNumberToObject(response, "trusted_device_days", g_config.trusted_device_days);

        char *json_str = cJSON_PrintUnformatted(response);
        http_response_set_json(res, 200, json_str);
        free(json_str);
        cJSON_Delete(response);
        return;
    }

    user_t user;
    if (httpd_get_authenticated_user(req, &user)) {
        log_info("Authentication successful for user: %s (role: %s)", user.username, db_auth_get_role_name(user.role));

        // Send success response with user info
        cJSON *response = cJSON_CreateObject();
        cJSON_AddBoolToObject(response, "authenticated", true);
        cJSON_AddNumberToObject(response, "id", (double)user.id);
        cJSON_AddStringToObject(response, "username", user.username);
        cJSON_AddStringToObject(response, "email", user.email);
        cJSON_AddStringToObject(response, "role", db_auth_get_role_name(user.role));
        cJSON_AddNumberToObject(response, "role_id", user.role);
        cJSON_AddBoolToObject(response, "is_active", user.is_active);
        cJSON_AddBoolToObject(response, "password_change_locked", user.password_change_locked);
        cJSON_AddBoolToObject(response, "auth_enabled", true);
        cJSON_AddNumberToObject(response, "auth_timeout_hours", g_config.auth_timeout_hours);
        cJSON_AddNumberToObject(response, "auth_absolute_timeout_hours", g_config.auth_absolute_timeout_hours);
        cJSON_AddNumberToObject(response, "trusted_device_days", g_config.trusted_device_days);

        char *json_str = cJSON_PrintUnformatted(response);
        http_response_set_json(res, 200, json_str);
        free(json_str);
        cJSON_Delete(response);
        return;
    }

    // If demo mode is enabled, return success with demo viewer role
    if (g_config.demo_mode) {
        log_info("Demo mode: returning viewer access for unauthenticated user");

        cJSON *response = cJSON_CreateObject();
        cJSON_AddBoolToObject(response, "authenticated", false);
        cJSON_AddBoolToObject(response, "demo_mode", true);
        cJSON_AddStringToObject(response, "username", "demo");
        cJSON_AddStringToObject(response, "role", "viewer");

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

/**
 * @brief Handler for GET /api/auth/login/config
 * Returns public login configuration (no auth required).
 * The frontend uses this to determine if it should show the TOTP field on login.
 */
void handle_auth_login_config(const http_request_t *req, http_response_t *res) {
    (void)req; // Unused

    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "force_mfa_on_login", g_config.force_mfa_on_login);
    cJSON_AddBoolToObject(response, "remember_device_enabled", g_config.trusted_device_days > 0);
    cJSON_AddNumberToObject(response, "auth_timeout_hours", g_config.auth_timeout_hours);
    cJSON_AddNumberToObject(response, "auth_absolute_timeout_hours", g_config.auth_absolute_timeout_hours);
    cJSON_AddNumberToObject(response, "trusted_device_days", g_config.trusted_device_days);

    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);
    free(json_str);
    cJSON_Delete(response);
}

void handle_auth_sessions_list(const http_request_t *req, http_response_t *res) {
    user_t user;
    if (!httpd_get_authenticated_user(req, &user)) {
        http_response_set_json_error(res, 401, "Unauthorized");
        return;
    }

    session_t sessions[32];
    int count = db_auth_list_user_sessions(user.id, sessions, 32);
    if (count < 0) {
        http_response_set_json_error(res, 500, "Failed to list sessions");
        return;
    }

    char current_token[128] = {0};
    httpd_get_session_token(req, current_token, sizeof(current_token));

    cJSON *response = cJSON_CreateObject();
    cJSON *items = cJSON_AddArrayToObject(response, "sessions");
    for (int i = 0; i < count; i++) {
        cJSON *item = cJSON_CreateObject();
        cJSON_AddNumberToObject(item, "id", (double)sessions[i].id);
        cJSON_AddNumberToObject(item, "created_at", (double)sessions[i].created_at);
        cJSON_AddNumberToObject(item, "last_activity_at", (double)sessions[i].last_activity_at);
        cJSON_AddNumberToObject(item, "idle_expires_at", (double)sessions[i].idle_expires_at);
        cJSON_AddNumberToObject(item, "expires_at", (double)sessions[i].expires_at);
        cJSON_AddStringToObject(item, "ip_address", sessions[i].ip_address);
        cJSON_AddStringToObject(item, "user_agent", sessions[i].user_agent);
        cJSON_AddBoolToObject(item, "current", current_token[0] != '\0' && strcmp(current_token, sessions[i].token) == 0);
        cJSON_AddItemToArray(items, item);
    }

    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);
    free(json_str);
    cJSON_Delete(response);
}

void handle_auth_sessions_delete(const http_request_t *req, http_response_t *res) {
    user_t user;
    if (!httpd_get_authenticated_user(req, &user)) {
        http_response_set_json_error(res, 401, "Unauthorized");
        return;
    }

    char id_str[64] = {0};
    if (http_request_extract_path_param(req, "/api/auth/sessions/", id_str, sizeof(id_str)) != 0) {
        http_response_set_json_error(res, 400, "Missing session ID");
        return;
    }

    int64_t session_id = strtoll(id_str, NULL, 10);
    char current_token[128] = {0};
    bool deleted_current_session = false;
    session_t sessions[32];
    int count = db_auth_list_user_sessions(user.id, sessions, 32);
    httpd_get_session_token(req, current_token, sizeof(current_token));
    for (int i = 0; i < count; i++) {
        if (sessions[i].id == session_id && current_token[0] != '\0' && strcmp(current_token, sessions[i].token) == 0) {
            deleted_current_session = true;
            break;
        }
    }

    if (session_id <= 0 || db_auth_delete_session_by_id(user.id, session_id) != 0) {
        http_response_set_json_error(res, 404, "Session not found");
        return;
    }

    if (deleted_current_session) {
        httpd_clear_session_cookie(res);
    }

    http_response_set_json(res, 200, "{\"success\":true}");
}

void handle_auth_trusted_devices_list(const http_request_t *req, http_response_t *res) {
    user_t user;
    if (!httpd_get_authenticated_user(req, &user)) {
        http_response_set_json_error(res, 401, "Unauthorized");
        return;
    }

    trusted_device_t devices[32];
    int count = db_auth_list_trusted_devices(user.id, devices, 32);
    if (count < 0) {
        http_response_set_json_error(res, 500, "Failed to list trusted devices");
        return;
    }

    char current_token[128] = {0};
    int64_t current_device_id = 0;
    if (httpd_get_cookie_value(req, "trusted_device", current_token, sizeof(current_token)) == 0) {
        db_auth_get_trusted_device_id(user.id, current_token, &current_device_id);
    }

    cJSON *response = cJSON_CreateObject();
    cJSON *items = cJSON_AddArrayToObject(response, "trusted_devices");
    for (int i = 0; i < count; i++) {
        cJSON *item = cJSON_CreateObject();
        cJSON_AddNumberToObject(item, "id", (double)devices[i].id);
        cJSON_AddNumberToObject(item, "created_at", (double)devices[i].created_at);
        cJSON_AddNumberToObject(item, "last_used_at", (double)devices[i].last_used_at);
        cJSON_AddNumberToObject(item, "expires_at", (double)devices[i].expires_at);
        cJSON_AddStringToObject(item, "ip_address", devices[i].ip_address);
        cJSON_AddStringToObject(item, "user_agent", devices[i].user_agent);
        cJSON_AddBoolToObject(item, "current", current_device_id > 0 && current_device_id == devices[i].id);
        cJSON_AddItemToArray(items, item);
    }

    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);
    free(json_str);
    cJSON_Delete(response);
}

void handle_auth_trusted_devices_delete(const http_request_t *req, http_response_t *res) {
    user_t user;
    if (!httpd_get_authenticated_user(req, &user)) {
        http_response_set_json_error(res, 401, "Unauthorized");
        return;
    }

    char id_str[64] = {0};
    if (http_request_extract_path_param(req, "/api/auth/trusted-devices/", id_str, sizeof(id_str)) != 0) {
        http_response_set_json_error(res, 400, "Missing trusted device ID");
        return;
    }

    int64_t device_id = strtoll(id_str, NULL, 10);
    char current_token[128] = {0};
    int64_t current_device_id = 0;
    bool deleted_current_device = false;
    trusted_device_t devices[32];
    int count = db_auth_list_trusted_devices(user.id, devices, 32);
    if (httpd_get_cookie_value(req, "trusted_device", current_token, sizeof(current_token)) == 0) {
        db_auth_get_trusted_device_id(user.id, current_token, &current_device_id);
    }
    for (int i = 0; i < count; i++) {
        if (devices[i].id == device_id && current_device_id > 0 && current_device_id == devices[i].id) {
            deleted_current_device = true;
            break;
        }
    }

    if (device_id <= 0 || db_auth_delete_trusted_device_by_id(user.id, device_id) != 0) {
        http_response_set_json_error(res, 404, "Trusted device not found");
        return;
    }

    if (deleted_current_device) {
        httpd_clear_trusted_device_cookie(res);
    }

    http_response_set_json(res, 200, "{\"success\":true}");
}

