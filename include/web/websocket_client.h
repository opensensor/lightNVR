#ifndef WEBSOCKET_CLIENT_H
#define WEBSOCKET_CLIENT_H

#include <stdbool.h>
#include "mongoose.h"

/**
 * @brief Find a client by ID
 * 
 * @param client_id Client ID
 * @return int Client index or -1 if not found
 */
int websocket_client_find_by_id(const char *client_id);

/**
 * @brief Find a client by connection
 * 
 * @param conn Mongoose connection
 * @return int Client index or -1 if not found
 */
int websocket_client_find_by_connection(const struct mg_connection *conn);

/**
 * @brief Remove a client by connection
 * 
 * @param conn Mongoose connection
 * @return bool true if client was removed, false otherwise
 */
bool websocket_client_remove_by_connection(const struct mg_connection *conn);

/**
 * @brief Check if a client is subscribed to a topic
 * 
 * @param client_id Client ID
 * @param topic Topic to check
 * @return bool true if subscribed, false otherwise
 */
bool websocket_client_is_subscribed(const char *client_id, const char *topic);

/**
 * @brief Get all clients subscribed to a topic
 * 
 * @param topic Topic to check
 * @param client_ids Pointer to array of client IDs (will be allocated)
 * @return int Number of clients subscribed to the topic
 */
int websocket_client_get_subscribed(const char *topic, char ***client_ids);

/**
 * @brief Subscribe a client to a topic
 * 
 * @param client_id Client ID
 * @param topic Topic to subscribe to
 * @return bool true on success, false on error
 */
bool websocket_client_subscribe(const char *client_id, const char *topic);

/**
 * @brief Unsubscribe a client from a topic
 * 
 * @param client_id Client ID
 * @param topic Topic to unsubscribe from
 * @return bool true on success, false on error
 */
bool websocket_client_unsubscribe(const char *client_id, const char *topic);

/**
 * @brief Update client activity timestamp
 * 
 * @param client_id Client ID
 */
void websocket_client_update_activity(const char *client_id);

/**
 * @brief Clean up inactive clients
 */
void websocket_client_cleanup_inactive(void);

/**
 * @brief Get client connection by ID
 * 
 * @param client_id Client ID
 * @return struct mg_connection* Connection or NULL if not found
 */
struct mg_connection* websocket_client_get_connection(const char *client_id);

#endif /* WEBSOCKET_CLIENT_H */
