/**
 * @file mongoose_server_websocket_utils.h
 * @brief Utility functions for Mongoose WebSocket server
 */

#ifndef MONGOOSE_SERVER_WEBSOCKET_UTILS_H
#define MONGOOSE_SERVER_WEBSOCKET_UTILS_H

#include <stdbool.h>

/**
 * @brief Create a WebSocket message
 * 
 * @param type Message type
 * @param topic Message topic
 * @param payload Message payload
 * @return char* Allocated message string, must be freed with mg_websocket_message_free
 */
char *mg_websocket_message_create(const char *type, const char *topic, const char *payload);

/**
 * @brief Send a WebSocket message to a client
 * 
 * @param client_id Client ID
 * @param message Message to send
 * @return bool true on success, false on error
 */
bool mg_websocket_message_send_to_client(const char *client_id, const char *message);

/**
 * @brief Free a WebSocket message
 * 
 * @param message Message to free
 */
void mg_websocket_message_free(char *message);

#endif /* MONGOOSE_SERVER_WEBSOCKET_UTILS_H */
