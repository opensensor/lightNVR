#ifndef WEBSOCKET_MANAGER_H
#define WEBSOCKET_MANAGER_H

#include <stdbool.h>
#include "mongoose.h"

// WebSocket message structure
typedef struct {
    char *type;
    char *topic;
    char *payload;
} websocket_message_t;

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
 * @brief Create a WebSocket message
 * 
 * @param type Message type
 * @param topic Message topic
 * @param payload Message payload
 * @return websocket_message_t* Pointer to the message or NULL on error
 */
websocket_message_t *websocket_message_create(const char *type, const char *topic, const char *payload);

/**
 * @brief Free a WebSocket message
 * 
 * @param message Message to free
 */
void websocket_message_free(websocket_message_t *message);

/**
 * @brief Send a WebSocket message to a client
 * 
 * @param client_id Client ID
 * @param message Message to send
 * @return bool true on success, false on error
 */
bool websocket_manager_send_to_client(const char *client_id, const websocket_message_t *message);

/**
 * @brief Send a WebSocket message to all clients subscribed to a topic
 * 
 * @param topic Topic to send to
 * @param message Message to send
 * @return int Number of clients the message was sent to
 */
int websocket_manager_broadcast(const char *topic, const websocket_message_t *message);

/**
 * @brief Check if a client is subscribed to a topic
 * 
 * @param client_id Client ID
 * @param topic Topic to check
 * @return bool true if subscribed, false otherwise
 */
bool websocket_manager_is_subscribed(const char *client_id, const char *topic);

/**
 * @brief Get all clients subscribed to a topic
 * 
 * @param topic Topic to check
 * @param client_ids Pointer to array of client IDs (will be allocated)
 * @return int Number of clients subscribed to the topic
 */
int websocket_manager_get_subscribed_clients(const char *topic, char ***client_ids);

/**
 * @brief Check if the WebSocket manager is initialized
 * 
 * @return bool true if initialized, false otherwise
 */
bool websocket_manager_is_initialized(void);

/**
 * @brief Register a WebSocket message handler
 * 
 * @param topic Topic to handle
 * @param handler Handler function
 * @return int 0 on success, non-zero on error
 */
int websocket_manager_register_handler(const char *topic, void (*handler)(const char *client_id, const char *message));

#endif // WEBSOCKET_MANAGER_H
