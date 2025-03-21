/**
 * @file mongoose_server_static.h
 * @brief Static file handling for Mongoose HTTP server
 */

#ifndef MONGOOSE_SERVER_STATIC_H
#define MONGOOSE_SERVER_STATIC_H

#include "web/http_server.h"

// Forward declarations for Mongoose structures
struct mg_connection;
struct mg_http_message;

/**
 * @brief Handle static file request
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 * @param server HTTP server
 */
void mongoose_server_handle_static_file(struct mg_connection *c, struct mg_http_message *hm, http_server_t *server);

/**
 * @brief Set maximum connections
 * 
 * @param server Server handle
 * @param max_connections Maximum number of connections
 * @return int 0 on success, non-zero on error
 */
int http_server_set_max_connections(http_server_handle_t server, int max_connections);

/**
 * @brief Set connection timeout
 * 
 * @param server Server handle
 * @param timeout_seconds Connection timeout in seconds
 * @return int 0 on success, non-zero on error
 */
int http_server_set_connection_timeout(http_server_handle_t server, int timeout_seconds);

#endif /* MONGOOSE_SERVER_STATIC_H */
