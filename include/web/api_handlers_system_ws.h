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

/**
 * @brief Get system logs using tail command
 *
 * @param logs Pointer to array of log strings (will be allocated)
 * @param count Pointer to store number of logs
 * @param max_lines Maximum number of lines to return
 * @return int 0 on success, -1 on failure
 */
int get_system_logs_tail(char ***logs, int *count, int max_lines);


/**
 * @brief Get JSON logs using tail command
 *
 * @param min_level Minimum log level to include
 * @param last_timestamp Last timestamp received by client (for pagination)
 * @param logs Pointer to array of log entries (will be allocated)
 * @param count Pointer to store number of logs
 * @return int 0 on success, -1 on error
 */
int get_json_logs_tail(const char *min_level, const char *last_timestamp, char ***logs, int *count);


/**
 * @brief Get logs from the JSON log file with timestamp-based pagination
 *
 * @param min_level Minimum log level to include
 * @param last_timestamp Last timestamp received by client (for pagination)
 * @param logs Pointer to array of log entries (will be allocated)
 * @param count Pointer to store number of logs
 * @return int 0 on success, non-zero on error
 */
int get_json_logs(const char *min_level, const char *last_timestamp, char ***logs, int *count);



int log_level_meets_minimum(const char *log_level, const char *min_level);


#endif /* API_HANDLERS_SYSTEM_WS_H */
