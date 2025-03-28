#ifndef WEBSOCKET_MANAGER_H
#define WEBSOCKET_MANAGER_H

#include <stdbool.h>
#include "mongoose.h"
#include "web/websocket_client.h"
#include "web/websocket_handler.h"
#include "web/websocket_message.h"

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

// Compatibility functions that delegate to the new API

/**
 * @brief Send a WebSocket message to a client (compatibility function)
 * 
 * @param client_id Client ID
 * @param message Message to send
 * @return bool true on success, false on error
 */
bool websocket_manager_send_to_client(const char *client_id, const websocket_message_t *message);

/**
 * @brief Send a WebSocket message to all clients subscribed to a topic (compatibility function)
 * 
 * @param topic Topic to send to
 * @param message Message to send
 * @return int Number of clients the message was sent to
 */
int websocket_manager_broadcast(const char *topic, const websocket_message_t *message);

/**
 * @brief Check if a client is subscribed to a topic (compatibility function)
 * 
 * @param client_id Client ID
 * @param topic Topic to check
 * @return bool true if subscribed, false otherwise
 */
bool websocket_manager_is_subscribed(const char *client_id, const char *topic);

/**
 * @brief Get all clients subscribed to a topic (compatibility function)
 * 
 * @param topic Topic to check
 * @param client_ids Pointer to array of client IDs (will be allocated)
 * @return int Number of clients subscribed to the topic
 */
int websocket_manager_get_subscribed_clients(const char *topic, char ***client_ids);

/**
 * @brief Register a WebSocket message handler (compatibility function)
 * 
 * @param topic Topic to handle
 * @param handler Handler function
 * @return int 0 on success, non-zero on error
 */
int websocket_manager_register_handler(const char *topic, void (*handler)(const char *client_id, const char *message));

#endif /* WEBSOCKET_MANAGER_H */
