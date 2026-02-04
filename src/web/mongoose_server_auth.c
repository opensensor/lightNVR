#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "web/mongoose_server_auth.h"
#include "web/mongoose_adapter.h"
#include "core/logger.h"
#include "database/db_auth.h"

// Include Mongoose
#include "mongoose.h"

/**
 * @brief Check basic authentication
 */
int mongoose_server_basic_auth_check(struct mg_http_message *hm, http_server_t *server) {
    if (!server->config.auth_enabled) {
        return 0; // Authentication not required
    }
    
    // Allow health check endpoint without authentication
    if (mg_http_match_uri(hm, "/api/health")) {
        return 0; // No authentication required for health check
    }
    
    // Extract URI to check for login endpoint
    char uri[256];
    size_t uri_len = hm->uri.len < sizeof(uri) - 1 ? hm->uri.len : sizeof(uri) - 1;
    memcpy(uri, hm->uri.buf, uri_len);
    uri[uri_len] = '\0';
    
    // Skip authentication for login endpoints and login page
    if (strcmp(uri, "/api/auth/login") == 0 || 
        strcmp(uri, "/login") == 0 || 
        strcmp(uri, "/login.html") == 0) {
        log_debug("Skipping authentication for login endpoint or page: %s", uri);
        return 0;
    }

    // Get Authorization header
    struct mg_str *auth_header = mg_http_get_header(hm, "Authorization");
    
    // If no Authorization header, check for session or auth cookie
    if (auth_header == NULL) {
        log_debug("No Authorization header found for URI: %s, checking for cookies", uri);
        struct mg_str *cookie_header = mg_http_get_header(hm, "Cookie");
        if (cookie_header != NULL) {
            log_debug("Cookie header found: %.*s", (int)cookie_header->len, cookie_header->buf);
            
            // Parse the cookie header manually
            char cookie_str[1024] = {0};
            if (cookie_header->len < sizeof(cookie_str) - 1) {
                memcpy(cookie_str, cookie_header->buf, cookie_header->len);
                cookie_str[cookie_header->len] = '\0';
                
                // First, check for session cookie (new auth system)
                char *session_cookie_start = strstr(cookie_str, "session=");
                if (session_cookie_start) {
                    session_cookie_start += 8; // Skip "session="
                    char *session_cookie_end = strchr(session_cookie_start, ';');
                    if (!session_cookie_end) {
                        session_cookie_end = session_cookie_start + strlen(session_cookie_start);
                    }
                    
                    // Extract session cookie value
                    size_t session_cookie_len = session_cookie_end - session_cookie_start;
                    char session_cookie_value[512] = {0};
                    if (session_cookie_len < sizeof(session_cookie_value) - 1) {
                        memcpy(session_cookie_value, session_cookie_start, session_cookie_len);
                        session_cookie_value[session_cookie_len] = '\0';
                        
                        log_debug("Found session cookie value: %s", session_cookie_value);

                        // Validate the session token
                        int64_t user_id;
                        if (db_auth_validate_session(session_cookie_value, &user_id) == 0) {
                            log_debug("Session token validated successfully for user ID: %lld", (long long)user_id);
                            return 0; // Authentication successful
                        } else {
                            log_debug("Invalid session token: %s", session_cookie_value);
                        }
                    }
                }
                
                // If no valid session cookie, check for auth cookie (legacy auth system)
                char *auth_cookie_start = strstr(cookie_str, "auth=");
                if (auth_cookie_start) {
                    auth_cookie_start += 5; // Skip "auth="
                    char *auth_cookie_end = strchr(auth_cookie_start, ';');
                    if (!auth_cookie_end) {
                        auth_cookie_end = auth_cookie_start + strlen(auth_cookie_start);
                    }
                    
                    // Extract auth cookie value
                    size_t auth_cookie_len = auth_cookie_end - auth_cookie_start;
                    char auth_cookie_value[512] = {0};
                    if (auth_cookie_len < sizeof(auth_cookie_value) - 1) {
                        memcpy(auth_cookie_value, auth_cookie_start, auth_cookie_len);
                        auth_cookie_value[auth_cookie_len] = '\0';
                        
                        log_debug("Found auth cookie value: %s", auth_cookie_value);
                        
                        // Create a temporary buffer for the Authorization header value
                        char auth_value[512];
                        snprintf(auth_value, sizeof(auth_value), "Basic %s", auth_cookie_value);
                        
                        // Create a static buffer for the header
                        static char auth_header_buf[512];
                        strncpy(auth_header_buf, auth_value, sizeof(auth_header_buf) - 1);
                        auth_header_buf[sizeof(auth_header_buf) - 1] = '\0';
                        
                        // Create a static mg_str for the header
                        static struct mg_str static_auth_header;
                        static_auth_header.buf = auth_header_buf;
                        static_auth_header.len = strlen(auth_header_buf);
                        
                        // Use the cookie value as the Authorization header
                        auth_header = &static_auth_header;
                        log_debug("Using auth cookie for authentication: %s", auth_header_buf);
                    }
                } else {
                    log_debug("No auth cookie found in cookie string: %s", cookie_str);
                }
            } else {
                log_debug("Cookie header too long to process");
            }
        } else {
            log_debug("No Cookie header found");
        }
        
        // If still no auth header, authentication fails
        if (auth_header == NULL) {
            log_debug("No Authorization header or valid cookies found");
            return -1;
        }
    }

    // Check if it's Basic authentication
    const char *auth_str = auth_header->buf;
    if (auth_header->len > 6 && strncmp(auth_str, "Basic ", 6) == 0) {
        // Extract credentials
        char user[64] = {0}, pass[64] = {0};
        char decoded[128] = {0};
        
        // Skip "Basic " prefix and decode base64
        const char *b64 = auth_str + 6;
        size_t b64_len = auth_header->len - 6;
        mg_base64_decode(b64, b64_len, decoded, sizeof(decoded));
        
        // Find the colon separator
        char *colon = strchr(decoded, ':');
        if (colon != NULL) {
            size_t user_len = colon - decoded;
            if (user_len < sizeof(user)) {
                strncpy(user, decoded, user_len);
                user[user_len] = '\0';
                
                // Get password (everything after the colon)
                strncpy(pass, colon + 1, sizeof(pass) - 1);
                pass[sizeof(pass) - 1] = '\0';
            }
        }
        
        if (user[0] != '\0') {
            // First try to authenticate against the database
            int64_t user_id;
            if (db_auth_authenticate(user, pass, &user_id) == 0) {
                // Get the user to check if they're active
                user_t db_user;
                if (db_auth_get_user_by_id(user_id, &db_user) == 0 && db_user.is_active) {
                    log_debug("Authentication successful with database credentials for user: %s (ID: %lld)",
                            user, (long long)user_id);
                    return 0; // Authentication successful
                }
            }
            
            // If database authentication fails, check against server config (legacy)
            if (strcmp(user, server->config.username) == 0 && 
                strcmp(pass, server->config.password) == 0) {
                return 0; // Authentication successful with legacy credentials
            }
            
            // Also check against global config (for API login compatibility)
            extern config_t g_config;
            if (strcmp(user, g_config.web_username) == 0 && 
                strcmp(pass, g_config.web_password) == 0) {
                return 0; // Authentication successful with global config credentials
            }
        }
    }

    log_debug("Authentication failed");
    return -1; // Authentication failed, caller will handle redirection to login page
}

/**
 * @brief Add CORS headers to response
 */
void mongoose_server_add_cors_headers(struct mg_connection *c, http_server_t *server) {

    if (!server->config.cors_enabled) {
        return;
    }

    // Add CORS headers
    mg_printf(c, "Access-Control-Allow-Origin: %s\r\n", 
             server->config.allowed_origins[0] ? server->config.allowed_origins : "*");
    mg_printf(c, "Access-Control-Allow-Methods: %s\r\n", 
             server->config.allowed_methods[0] ? server->config.allowed_methods : "GET, POST, PUT, DELETE, OPTIONS");
    mg_printf(c, "Access-Control-Allow-Headers: %s\r\n", 
             server->config.allowed_headers[0] ? server->config.allowed_headers : "Content-Type, Authorization");
    mg_printf(c, "Access-Control-Max-Age: 86400\r\n");
}

/**
 * @brief Handle CORS preflight request
 */
void mongoose_server_handle_cors_preflight(struct mg_connection *c, struct mg_http_message *hm, http_server_t *server) {
    // Note: The mutex is already locked in the calling function (mongoose_event_handler)
    
    if (!server->config.cors_enabled) {
        mg_http_reply(c, 405, "", "Method Not Allowed\n");
        return;
    }

    // Send CORS preflight response
    mg_printf(c, "HTTP/1.1 200 OK\r\n");
    mg_printf(c, "Connection: close\r\n");
    mongoose_server_add_cors_headers(c, server);
    mg_printf(c, "Content-Length: 0\r\n");
    mg_printf(c, "\r\n");
}

/**
 * @brief Set authentication settings
 */
int http_server_set_authentication(http_server_handle_t server, bool enabled, 
                                  const char *username, const char *password) {
    if (!server) {
        return -1;
    }

    if (enabled && (!username || !password)) {
        log_error("Username and password are required when authentication is enabled");
        return -1;
    }

    server->config.auth_enabled = enabled;

    if (enabled) {
        strncpy(server->config.username, username, sizeof(server->config.username) - 1);
        server->config.username[sizeof(server->config.username) - 1] = '\0';

        strncpy(server->config.password, password, sizeof(server->config.password) - 1);
        server->config.password[sizeof(server->config.password) - 1] = '\0';
    }

    log_info("Authentication %s", enabled ? "enabled" : "disabled");
    return 0;
}

/**
 * @brief Set CORS settings
 */
int http_server_set_cors(http_server_handle_t server, bool enabled, 
                        const char *allowed_origins, const char *allowed_methods, 
                        const char *allowed_headers) {
    if (!server) {
        return -1;
    }

    server->config.cors_enabled = enabled;

    if (enabled) {
        if (allowed_origins) {
            strncpy(server->config.allowed_origins, allowed_origins, sizeof(server->config.allowed_origins) - 1);
            server->config.allowed_origins[sizeof(server->config.allowed_origins) - 1] = '\0';
        }

        if (allowed_methods) {
            strncpy(server->config.allowed_methods, allowed_methods, sizeof(server->config.allowed_methods) - 1);
            server->config.allowed_methods[sizeof(server->config.allowed_methods) - 1] = '\0';
        }

        if (allowed_headers) {
            strncpy(server->config.allowed_headers, allowed_headers, sizeof(server->config.allowed_headers) - 1);
            server->config.allowed_headers[sizeof(server->config.allowed_headers) - 1] = '\0';
        }
    }

    log_info("CORS %s", enabled ? "enabled" : "disabled");
    return 0;
}

/**
 * @brief Set SSL/TLS settings
 */
int http_server_set_ssl(http_server_handle_t server, bool enabled, 
                       const char *cert_path, const char *key_path) {
    if (!server) {
        return -1;
    }

    if (enabled && (!cert_path || !key_path)) {
        log_error("Certificate and key paths are required when SSL/TLS is enabled");
        return -1;
    }

    server->config.ssl_enabled = enabled;

    if (enabled) {
        strncpy(server->config.cert_path, cert_path, sizeof(server->config.cert_path) - 1);
        server->config.cert_path[sizeof(server->config.cert_path) - 1] = '\0';

        strncpy(server->config.key_path, key_path, sizeof(server->config.key_path) - 1);
        server->config.key_path[sizeof(server->config.key_path) - 1] = '\0';
    }

    log_info("SSL/TLS %s", enabled ? "enabled" : "disabled");
    return 0;
}
