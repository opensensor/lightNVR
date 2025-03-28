#ifndef WEBSOCKET_MESSAGE_H
#define WEBSOCKET_MESSAGE_H

#include <stdbool.h>

/**
 * @brief WebSocket message structure
 */
typedef struct {
    char *type;         // Message type (e.g., "progress", "result", "error")
    char *topic;        // Message topic (e.g., "recordings/batch-delete")
    char *payload;      // Message payload (JSON string)
} websocket_message_t;

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
bool websocket_message_send_to_client(const char *client_id, const websocket_message_t *message);

/**
 * @brief Send a WebSocket message to all clients subscribed to a topic
 * 
 * @param topic Topic to send to
 * @param message Message to send
 * @return int Number of clients the message was sent to
 */
int websocket_message_broadcast(const char *topic, const websocket_message_t *message);

#endif /* WEBSOCKET_MESSAGE_H */
