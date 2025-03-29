/**
 * @file websocket_bridge.h
 * @brief Bridge between mongoose and libwebsockets for WebSocket handling
 */

#ifndef WEBSOCKET_BRIDGE_H
#define WEBSOCKET_BRIDGE_H

#include <stdbool.h>
#include <stddef.h>

// Forward declarations
struct mg_connection;

/**
 * @brief Initialize WebSocket bridge
 * 
 * @return int 0 on success, non-zero on error
 */
int websocket_bridge_init(void);

/**
 * @brief Shutdown WebSocket bridge
 */
void websocket_bridge_shutdown(void);

/**
 * @brief Check if WebSocket bridge is initialized
 * 
 * @return bool true if initialized, false otherwise
 */
bool websocket_bridge_is_initialized(void);

/**
 * @brief Handle WebSocket connection open
 * 
 * @param mg_conn Mongoose connection
 */
void websocket_bridge_handle_open(struct mg_connection *mg_conn);

/**
 * @brief Handle WebSocket message
 * 
 * @param mg_conn Mongoose connection
 * @param data Message data
 * @param len Message data length
 */
void websocket_bridge_handle_message(struct mg_connection *mg_conn, const char *data, size_t len);

/**
 * @brief Handle WebSocket connection close
 * 
 * @param mg_conn Mongoose connection
 */
void websocket_bridge_handle_close(struct mg_connection *mg_conn);

/**
 * @brief Register a WebSocket handler for a specific topic
 * 
 * @param topic Topic name
 * @param handler Handler function
 * @param user_data User data (optional)
 * @return int 0 on success, non-zero on error
 */
int websocket_bridge_register_handler(const char *topic, 
                                     void (*handler)(struct mg_connection *mg_conn, const char *data, size_t len, void *user_data),
                                     void *user_data);

/**
 * @brief Send a WebSocket message to a specific connection
 * 
 * @param mg_conn Mongoose connection
 * @param data Message data
 * @param len Message data length
 * @return int 0 on success, non-zero on error
 */
int websocket_bridge_send(struct mg_connection *mg_conn, const char *data, size_t len);

/**
 * @brief Broadcast a WebSocket message to all connections with a specific topic
 * 
 * @param topic Topic name
 * @param data Message data
 * @param len Message data length
 * @return int Number of connections the message was sent to
 */
int websocket_bridge_broadcast(const char *topic, const char *data, size_t len);

#endif // WEBSOCKET_BRIDGE_H
