/**
 * @file mongoose_server_auth.h
 * @brief Authentication functions for Mongoose HTTP server
 */

#ifndef MONGOOSE_SERVER_AUTH_H
#define MONGOOSE_SERVER_AUTH_H

#include "web/http_server.h"

// Forward declarations for Mongoose structures
struct mg_connection;
struct mg_http_message;

/**
 * @brief Check basic authentication
 * 
 * @param hm Mongoose HTTP message
 * @param server HTTP server
 * @return int 0 if authentication successful, non-zero otherwise
 */
int mongoose_server_basic_auth_check(struct mg_http_message *hm, http_server_t *server);

/**
 * @brief Add CORS headers to response
 * 
 * @param c Mongoose connection
 * @param server HTTP server
 */
void mongoose_server_add_cors_headers(struct mg_connection *c, http_server_t *server);

/**
 * @brief Handle CORS preflight request
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 * @param server HTTP server
 */
void mongoose_server_handle_cors_preflight(struct mg_connection *c, struct mg_http_message *hm, http_server_t *server);

/**
 * @brief Set authentication settings
 * 
 * @param server Server handle
 * @param enabled Authentication enabled
 * @param username Username
 * @param password Password
 * @return int 0 on success, non-zero on error
 */
int http_server_set_authentication(http_server_handle_t server, bool enabled, 
                                  const char *username, const char *password);

/**
 * @brief Set CORS settings
 * 
 * @param server Server handle
 * @param enabled CORS enabled
 * @param allowed_origins Allowed origins
 * @param allowed_methods Allowed methods
 * @param allowed_headers Allowed headers
 * @return int 0 on success, non-zero on error
 */
int http_server_set_cors(http_server_handle_t server, bool enabled, 
                        const char *allowed_origins, const char *allowed_methods, 
                        const char *allowed_headers);

/**
 * @brief Set SSL/TLS settings
 * 
 * @param server Server handle
 * @param enabled SSL/TLS enabled
 * @param cert_path Certificate path
 * @param key_path Key path
 * @return int 0 on success, non-zero on error
 */
int http_server_set_ssl(http_server_handle_t server, bool enabled, 
                       const char *cert_path, const char *key_path);

#endif /* MONGOOSE_SERVER_AUTH_H */
