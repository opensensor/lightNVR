/**
 * @file mongoose_server_utils.h
 * @brief Utility functions for Mongoose HTTP server
 */

#ifndef MONGOOSE_SERVER_UTILS_H
#define MONGOOSE_SERVER_UTILS_H

#include "web/http_server.h"

// Forward declarations for Mongoose structures
struct mg_connection;
struct mg_http_message;

/**
 * @brief Check if a path matches a pattern
 * 
 * @param pattern Pattern to match
 * @param path Path to check
 * @return bool true if path matches pattern, false otherwise
 */
bool mongoose_server_path_matches(const char *pattern, const char *path);

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

#endif /* MONGOOSE_SERVER_UTILS_H */
