/**
 * @file libuv_api_handlers.h
 * @brief API handler registration for libuv HTTP server
 */

#ifndef LIBUV_API_HANDLERS_H
#define LIBUV_API_HANDLERS_H

#include "web/http_server.h"

/**
 * @brief Register all API handlers with the libuv server
 * 
 * This function should be called after libuv_server_init() but before
 * http_server_start() to register all API routes.
 * 
 * @param server HTTP server handle
 * @return 0 on success, -1 on error
 */
int register_all_libuv_handlers(http_server_handle_t server);

/**
 * @brief Register static file handler for serving web assets
 * 
 * This should be called after registering API handlers to ensure
 * API routes take precedence over static file serving.
 * 
 * @param server HTTP server handle
 * @return 0 on success, -1 on error
 */
int register_static_file_handler(http_server_handle_t server);

#endif /* LIBUV_API_HANDLERS_H */

