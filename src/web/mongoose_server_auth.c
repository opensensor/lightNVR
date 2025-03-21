#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "web/mongoose_server_auth.h"
#include "web/mongoose_adapter.h"
#include "core/logger.h"

// Include Mongoose
#include "mongoose.h"

/**
 * @brief Check basic authentication
 */
int mongoose_server_basic_auth_check(struct mg_http_message *hm, http_server_t *server) {
    if (!server->config.auth_enabled) {
        return 0; // Authentication not required
    }

    // Get Authorization header
    struct mg_str *auth_header = mg_http_get_header(hm, "Authorization");
    if (auth_header == NULL) {
        log_debug("No Authorization header found");
        return -1;
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
            // Check credentials
            if (strcmp(user, server->config.username) == 0 && 
                strcmp(pass, server->config.password) == 0) {
                return 0; // Authentication successful
            }
        }
    }

    log_debug("Authentication failed");
    return -1;
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
    if (!server->config.cors_enabled) {
        mg_http_reply(c, 405, "", "Method Not Allowed\n");
        return;
    }

    // Send CORS preflight response
    mg_printf(c, "HTTP/1.1 200 OK\r\n");
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
