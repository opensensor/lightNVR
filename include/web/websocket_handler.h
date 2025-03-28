#ifndef WEBSOCKET_HANDLER_H
#define WEBSOCKET_HANDLER_H

#include <stdbool.h>

/**
 * @brief Register a WebSocket message handler
 * 
 * @param topic Topic to handle
 * @param handler Handler function
 * @return int 0 on success, non-zero on error
 */
int websocket_handler_register(const char *topic, void (*handler)(const char *client_id, const char *message));

/**
 * @brief Find a handler by topic
 * 
 * @param topic Topic to find
 * @return int Handler index or -1 if not found
 */
int websocket_handler_find_by_topic(const char *topic);

/**
 * @brief Call a handler for a topic
 * 
 * @param topic Topic to handle
 * @param client_id Client ID
 * @param message Message to handle
 * @return bool true if handler was found and called, false otherwise
 */
bool websocket_handler_call(const char *topic, const char *client_id, const char *message);

#endif /* WEBSOCKET_HANDLER_H */
