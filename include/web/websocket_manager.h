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


#endif // WEBSOCKET_MANAGER_H
