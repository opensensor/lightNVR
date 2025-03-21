/**
 * @file mongoose_server.h
 * @brief Mongoose HTTP server implementation
 */

#ifndef MONGOOSE_SERVER_H
#define MONGOOSE_SERVER_H

#include "web/http_server.h"

// Forward declarations for Mongoose structures
struct mg_connection;
struct mg_http_message;

/**
 * @brief Initialize HTTP server using Mongoose
 * 
 * @param config Server configuration
 * @return http_server_handle_t Server handle or NULL on error
 */
http_server_handle_t mongoose_server_init(const http_server_config_t *config);

/**
 * @brief Convert Mongoose HTTP message to HTTP request
 * 
 * @param conn Mongoose connection
 * @param msg Mongoose HTTP message
 * @param request HTTP request to fill
 * @return int 0 on success, non-zero on error
 */
int mongoose_server_mg_to_request(struct mg_connection *conn, struct mg_http_message *msg, 
                                 http_request_t *request);

/**
 * @brief Send HTTP response using Mongoose
 * 
 * @param conn Mongoose connection
 * @param response HTTP response
 * @return int 0 on success, non-zero on error
 */
int mongoose_server_send_response(struct mg_connection *conn, const http_response_t *response);

#endif /* MONGOOSE_SERVER_H */
