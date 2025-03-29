#ifndef WEBSOCKET_MANAGER_H
#define WEBSOCKET_MANAGER_H

#include <stdbool.h>
#include <stddef.h>
#include "mongoose.h"
#include "web/websocket_handler.h"

/**
 * @brief Initialize WebSocket manager
 * 
 * @return int 0 on success, non-zero on error
 */
int websocket_manager_init(void);

/**
 * @brief Shutdown WebSocket manager
 */
void websocket_manager_shutdown(void);

/**
 * @brief Handle WebSocket connection open
 * 
 * @param c Mongoose connection
 */
void websocket_manager_handle_open(struct mg_connection *c);

/**
 * @brief Handle WebSocket message
 * 
 * @param c Mongoose connection
 * @param data Message data
 * @param data_len Message data length
 */
void websocket_manager_handle_message(struct mg_connection *c, const char *data, size_t data_len);

/**
 * @brief Handle WebSocket connection close
 * 
 * @param c Mongoose connection
 */
void websocket_manager_handle_close(struct mg_connection *c);

/**
 * @brief Check if the WebSocket manager is initialized
 * 
 * @return bool true if initialized, false otherwise
 */
bool websocket_manager_is_initialized(void);

/**
 * @brief Register a WebSocket handler for a specific topic
 * 
 * @param topic Topic name
 * @param handler Handler function
 * @param user_data User data (optional)
 * @return int 0 on success, non-zero on error
 */
int websocket_manager_register_handler(const char *topic, websocket_handler_t handler, void *user_data);

/**
 * @brief Broadcast a message to all WebSocket clients
 * 
 * @param mgr Mongoose manager
 * @param data Message data
 * @param data_len Message data length
 * @return int Number of clients the message was sent to
 */
int websocket_manager_broadcast(struct mg_mgr *mgr, const char *data, size_t data_len);

#endif // WEBSOCKET_MANAGER_H
