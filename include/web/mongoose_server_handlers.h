/**
 * @file mongoose_server_handlers.h
 * @brief Request handlers for Mongoose HTTP server
 */

#ifndef MONGOOSE_SERVER_HANDLERS_H
#define MONGOOSE_SERVER_HANDLERS_H

#include "web/http_server.h"
#include "web/http_router.h"

// Forward declarations for Mongoose structures
struct mg_connection;
struct mg_http_message;

/**
 * @brief Initialize the router
 * 
 * @return int 0 on success, non-zero on error
 */
int mongoose_server_init_router(void);

/**
 * @brief Get the router handle
 * 
 * @return http_router_handle_t Router handle or NULL if not initialized
 */
http_router_handle_t mongoose_server_get_router(void);

/**
 * @brief Register request handler
 * 
 * @param server Server handle
 * @param path Request path
 * @param method HTTP method or NULL for any method
 * @param handler Request handler function
 * @return int 0 on success, non-zero on error
 */
int http_server_register_handler(http_server_handle_t server, const char *path, 
                                const char *method, request_handler_t handler);

/**
 * @brief Get server statistics
 * 
 * @param server Server handle
 * @param active_connections Number of active connections
 * @param requests_per_second Requests per second
 * @param bytes_sent Bytes sent
 * @param bytes_received Bytes received
 * @return int 0 on success, non-zero on error
 */
int http_server_get_stats(http_server_handle_t server, int *active_connections, 
                         double *requests_per_second, uint64_t *bytes_sent, 
                         uint64_t *bytes_received);

#endif /* MONGOOSE_SERVER_HANDLERS_H */
