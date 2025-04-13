#ifndef WEBSOCKET_HANDLER_H
#define WEBSOCKET_HANDLER_H

#include <stdbool.h>
#include <stddef.h>
#include "mongoose.h"

/**
 * @brief WebSocket handler function type
 * 
 * @param c Mongoose connection
 * @param data Message data
 * @param data_len Message data length
 * @param user_data User data (optional)
 */
typedef void (*websocket_handler_t)(struct mg_connection *c, const char *data, size_t data_len, void *user_data);

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

/**
 * @brief Get the number of registered WebSocket handlers
 * 
 * @return int Number of registered handlers
 */
int get_websocket_handler_count(void);

/**
 * @brief Get the topic of a WebSocket handler by index
 * 
 * @param index Handler index
 * @return const char* Topic name or NULL if not found
 */
const char *get_websocket_handler_topic(int index);

/**
 * @brief Debug function to print all registered WebSocket handlers
 */
void debug_print_websocket_handlers(void);

#endif /* WEBSOCKET_HANDLER_H */
