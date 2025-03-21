/**
 * @file mongoose_api_handlers.h
 * @brief API handlers for Mongoose HTTP server
 */

#ifndef MONGOOSE_API_HANDLERS_H
#define MONGOOSE_API_HANDLERS_H

#include "web/http_server.h"

/**
 * @brief Set the HTTP server handle for API handlers
 * 
 * @param server HTTP server handle
 */
void set_http_server_handle(http_server_handle_t server);

/**
 * @brief Register API handlers with the Mongoose HTTP server
 * 
 * @param server HTTP server handle
 */
void register_mongoose_api_handlers(http_server_handle_t server);

#endif /* MONGOOSE_API_HANDLERS_H */
