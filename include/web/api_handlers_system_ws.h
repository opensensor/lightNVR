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
 * @brief Fetch system logs with timestamp-based pagination
 * 
 * @param client_id WebSocket client ID
 * @param min_level Minimum log level to include
 * @param last_timestamp Last timestamp received by client (for pagination)
 * @return int Number of logs sent
 */
int fetch_system_logs(const char *client_id, const char *min_level, const char *last_timestamp);

/**
 * @brief Remove log level for a client
 * 
 * @param client_id WebSocket client ID
 */
void remove_client_log_level(const char *client_id);

#endif /* API_HANDLERS_SYSTEM_WS_H */
