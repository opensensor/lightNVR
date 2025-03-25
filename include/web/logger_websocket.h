/**
 * @file logger_websocket.h
 * @brief WebSocket integration for the logger
 */

#ifndef LOGGER_WEBSOCKET_H
#define LOGGER_WEBSOCKET_H

/**
 * @brief Initialize logger WebSocket integration
 */
void init_logger_websocket(void);

/**
 * @brief Shutdown logger WebSocket integration
 */
void shutdown_logger_websocket(void);

/**
 * @brief Broadcast system logs to WebSocket clients
 * 
 * This function is called by the logger after a log message is written.
 * It broadcasts the logs to all WebSocket clients subscribed to the system/logs topic.
 */
void broadcast_logs_to_websocket(void);

#endif /* LOGGER_WEBSOCKET_H */
