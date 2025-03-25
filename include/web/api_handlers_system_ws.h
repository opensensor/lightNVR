/**
 * @file api_handlers_system_ws.h
 * @brief WebSocket handlers for system operations
 */

#ifndef API_HANDLERS_SYSTEM_WS_H
#define API_HANDLERS_SYSTEM_WS_H

#include "mongoose.h"

/**
 * @brief WebSocket handler for system logs
 * 
 * @param client_id WebSocket client ID
 * @param message WebSocket message
 */
void websocket_handle_system_logs(const char *client_id, const char *message);

/**
 * @brief Send system logs to all subscribed clients
 * 
 * @return int Number of clients the message was sent to
 */
int websocket_broadcast_system_logs(void);

#endif /* API_HANDLERS_SYSTEM_WS_H */
