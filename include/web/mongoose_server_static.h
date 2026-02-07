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
 * @brief Read file from disk and write a complete HTTP response to connection's send buffer.
 *
 * Unlike mg_http_serve_file(), this writes the ENTIRE response (headers + body)
 * at once, making it safe for use with fake connections in worker threads.
 *
 * @param c Mongoose connection (can be a real or fake connection)
 * @param path Path to the file on disk
 * @param content_type MIME content type (e.g., "text/html")
 * @param extra_headers Additional HTTP headers (each ending with \r\n), or NULL
 * @return true if file was served successfully, false otherwise
 */
bool serve_file_buffered(struct mg_connection *c, const char *path,
                         const char *content_type, const char *extra_headers);

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
