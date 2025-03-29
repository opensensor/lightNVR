#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#include "web/websocket_client.h"
#include "core/logger.h"
#include "mongoose.h"

// Maximum number of WebSocket clients
#define MAX_WS_CLIENTS 32

// Maximum number of subscriptions per client
#define MAX_SUBSCRIPTIONS 16

// Maximum topic name length
#define MAX_TOPIC_LENGTH 64

// WebSocket client structure
typedef struct {
    char id[64];                                // Client ID
    struct mg_connection *conn;                 // Mongoose connection
    char subscriptions[MAX_SUBSCRIPTIONS][MAX_TOPIC_LENGTH]; // Subscribed topics
    int subscription_count;                     // Number of subscriptions
    uint64_t last_activity;                     // Last activity timestamp
    bool active;                                // Whether the client is active
} ws_client_t;

// Global state
static ws_client_t s_clients[MAX_WS_CLIENTS];
static int s_client_count = 0;

/**
 * @brief Find a client by ID
 * 
 * @param client_id Client ID
 * @return int Client index or -1 if not found
 */
int websocket_client_find_by_id(const char *client_id) {
    if (!client_id) {
        return -1;
    }
    
    for (int i = 0; i < MAX_WS_CLIENTS; i++) {
        if (s_clients[i].active && strcmp(s_clients[i].id, client_id) == 0) {
            return i;
        }
    }
    
    return -1;
}

/**
 * @brief Find a client by connection
 * 
 * @param conn Mongoose connection
 * @return int Client index or -1 if not found
 */
int websocket_client_find_by_connection(const struct mg_connection *conn) {
    if (!conn) {
        return -1;
    }
    
    for (int i = 0; i < MAX_WS_CLIENTS; i++) {
        if (s_clients[i].active && s_clients[i].conn == conn) {
            return i;
        }
    }
    
    return -1;
}

/**
 * @brief Remove a client by connection
 * 
 * @param conn Mongoose connection
 * @return bool true if client was removed, false otherwise
 */
bool websocket_client_remove_by_connection(const struct mg_connection *conn) {
    int index = websocket_client_find_by_connection(conn);
    if (index < 0) {
        return false;
    }
    
    // Mark client as inactive
    s_clients[index].active = false;
    s_clients[index].conn = NULL;
    s_clients[index].subscription_count = 0;
    
    log_info("Removed WebSocket client: %s", s_clients[index].id);
    
    return true;
}

/**
 * @brief Check if a client is subscribed to a topic
 * 
 * @param client_id Client ID
 * @param topic Topic to check
 * @return bool true if subscribed, false otherwise
 */
bool websocket_client_is_subscribed(const char *client_id, const char *topic) {
    if (!client_id || !topic) {
        log_warn("Invalid client_id or topic in websocket_client_is_subscribed");
        return false;
    }
    
    // Check if client_id is empty
    if (client_id[0] == '\0') {
        log_warn("Empty client_id in websocket_client_is_subscribed");
        return false;
    }
    
    int index = websocket_client_find_by_id(client_id);
    if (index < 0) {
        // Client not found, but we'll auto-subscribe them
        log_info("Client %s not found, auto-subscribing to topic: %s", client_id, topic);
        websocket_client_subscribe(client_id, topic);
        return true;  // Return true to allow the operation to proceed
    }
    
    for (int i = 0; i < s_clients[index].subscription_count; i++) {
        if (strcmp(s_clients[index].subscriptions[i], topic) == 0) {
            return true;
        }
    }
    
    // Client found but not subscribed, auto-subscribe them
    log_info("Client %s found but not subscribed to topic: %s, auto-subscribing", client_id, topic);
    websocket_client_subscribe(client_id, topic);
    return true;  // Return true to allow the operation to proceed
}

/**
 * @brief Get all clients subscribed to a topic
 * 
 * @param topic Topic to check
 * @param client_ids Pointer to array of client IDs (will be allocated)
 * @return int Number of clients subscribed to the topic
 */
int websocket_client_get_subscribed(const char *topic, char ***client_ids) {
    if (!topic || !client_ids) {
        return 0;
    }
    
    // Count subscribed clients
    int count = 0;
    for (int i = 0; i < MAX_WS_CLIENTS; i++) {
        if (!s_clients[i].active) {
            continue;
        }
        
        for (int j = 0; j < s_clients[i].subscription_count; j++) {
            if (strcmp(s_clients[i].subscriptions[j], topic) == 0) {
                count++;
                break;
            }
        }
    }
    
    if (count == 0) {
        *client_ids = NULL;
        return 0;
    }
    
    // Allocate array for client IDs
    *client_ids = (char **)malloc(count * sizeof(char *));
    if (!*client_ids) {
        log_error("Failed to allocate memory for client IDs");
        return 0;
    }
    
    // Fill array with client IDs
    int index = 0;
    for (int i = 0; i < MAX_WS_CLIENTS; i++) {
        if (!s_clients[i].active) {
            continue;
        }
        
        for (int j = 0; j < s_clients[i].subscription_count; j++) {
            if (strcmp(s_clients[i].subscriptions[j], topic) == 0) {
                (*client_ids)[index] = strdup(s_clients[i].id);
                if (!(*client_ids)[index]) {
                    log_error("Failed to allocate memory for client ID");
                    // Free already allocated IDs
                    for (int k = 0; k < index; k++) {
                        free((*client_ids)[k]);
                    }
                    free(*client_ids);
                    *client_ids = NULL;
                    return 0;
                }
                
                index++;
                break;
            }
        }
    }
    
    return count;
}

/**
 * @brief Subscribe a client to a topic
 * 
 * @param client_id Client ID
 * @param topic Topic to subscribe to
 * @return bool true on success, false on error
 */
bool websocket_client_subscribe(const char *client_id, const char *topic) {
    if (!client_id || !topic) {
        log_error("Invalid parameters for websocket_client_subscribe");
        return false;
    }
    
    log_debug("Subscribing client %s to topic: %s", client_id, topic);
    
    int index = websocket_client_find_by_id(client_id);
    if (index < 0) {
        // Client not found, create a new one
        for (int i = 0; i < MAX_WS_CLIENTS; i++) {
            if (!s_clients[i].active) {
                // Found an empty slot
                s_clients[i].active = true;
                strncpy(s_clients[i].id, client_id, sizeof(s_clients[i].id) - 1);
                s_clients[i].id[sizeof(s_clients[i].id) - 1] = '\0';
                s_clients[i].subscription_count = 0;
                s_clients[i].last_activity = mg_millis();
                
                // Get connection from client ID (assuming it's a pointer)
                struct mg_connection *conn = NULL;
                if (sscanf(client_id, "%p", &conn) == 1 && conn != NULL) {
                    s_clients[i].conn = conn;
                    log_debug("Successfully parsed connection pointer from client ID: %s", client_id);
                } else {
                    s_clients[i].conn = NULL;
                    log_warn("Failed to parse connection pointer from client ID: %s", client_id);
                }
                
                index = i;
                s_client_count++;
                log_info("Created new WebSocket client: %s", client_id);
                break;
            }
        }
        
        if (index < 0) {
            log_error("Maximum number of WebSocket clients reached");
            return false;
        }
    }
    
    // Check if client is already subscribed to this topic
    for (int i = 0; i < s_clients[index].subscription_count; i++) {
        if (strcmp(s_clients[index].subscriptions[i], topic) == 0) {
            // Already subscribed
            return true;
        }
    }
    
    // Check if client has reached maximum number of subscriptions
    if (s_clients[index].subscription_count >= MAX_SUBSCRIPTIONS) {
        log_error("Maximum number of subscriptions reached for client: %s", client_id);
        return false;
    }
    
    // Add subscription
    strncpy(s_clients[index].subscriptions[s_clients[index].subscription_count], 
           topic, MAX_TOPIC_LENGTH - 1);
    s_clients[index].subscriptions[s_clients[index].subscription_count][MAX_TOPIC_LENGTH - 1] = '\0';
    s_clients[index].subscription_count++;
    
    // Update activity timestamp
    s_clients[index].last_activity = mg_millis();
    
    log_info("Client %s subscribed to topic: %s", client_id, topic);
    
    return true;
}

/**
 * @brief Unsubscribe a client from a topic
 * 
 * @param client_id Client ID
 * @param topic Topic to unsubscribe from
 * @return bool true on success, false on error
 */
bool websocket_client_unsubscribe(const char *client_id, const char *topic) {
    if (!client_id || !topic) {
        return false;
    }
    
    int index = websocket_client_find_by_id(client_id);
    if (index < 0) {
        return false;
    }
    
    // Find subscription
    int sub_index = -1;
    for (int i = 0; i < s_clients[index].subscription_count; i++) {
        if (strcmp(s_clients[index].subscriptions[i], topic) == 0) {
            sub_index = i;
            break;
        }
    }
    
    if (sub_index < 0) {
        // Not subscribed
        return false;
    }
    
    // Remove subscription by shifting remaining subscriptions
    for (int i = sub_index; i < s_clients[index].subscription_count - 1; i++) {
        strcpy(s_clients[index].subscriptions[i], s_clients[index].subscriptions[i + 1]);
    }
    
    s_clients[index].subscription_count--;
    
    // Update activity timestamp
    s_clients[index].last_activity = mg_millis();
    
    log_info("Client %s unsubscribed from topic: %s", client_id, topic);
    
    return true;
}

/**
 * @brief Update client activity timestamp
 * 
 * @param client_id Client ID
 */
void websocket_client_update_activity(const char *client_id) {
    if (!client_id) {
        return;
    }
    
    int index = websocket_client_find_by_id(client_id);
    if (index < 0) {
        return;
    }
    
    s_clients[index].last_activity = mg_millis();
}

/**
 * @brief Clean up inactive clients
 */
void websocket_client_cleanup_inactive(void) {
    uint64_t now = mg_millis();
    uint64_t timeout = 300000; // 5 minutes
    
    for (int i = 0; i < MAX_WS_CLIENTS; i++) {
        if (s_clients[i].active && (now - s_clients[i].last_activity) > timeout) {
            log_info("Removing inactive WebSocket client: %s", s_clients[i].id);
            s_clients[i].active = false;
            s_clients[i].conn = NULL;
            s_clients[i].subscription_count = 0;
            s_client_count--;
        }
    }
}

/**
 * @brief Get client connection by ID
 * 
 * @param client_id Client ID
 * @return struct mg_connection* Connection or NULL if not found
 */
struct mg_connection* websocket_client_get_connection(const char *client_id) {
    if (!client_id) {
        return NULL;
    }
    
    int index = websocket_client_find_by_id(client_id);
    if (index < 0) {
        return NULL;
    }
    
    return s_clients[index].conn;
}
