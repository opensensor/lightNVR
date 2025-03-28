#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <time.h>
#include <sys/time.h>
#include <errno.h>

#include "web/websocket_client.h"
#include "web/websocket_manager.h"
#include "core/logger.h"
#include "video/onvif_discovery_messages.h"

// Maximum number of clients and topics
#define MAX_CLIENTS 100
#define MAX_TOPICS 20

// Client structure
typedef struct {
    char id[64];                    // Client ID
    struct mg_connection *conn;     // Mongoose connection
    char topics[MAX_TOPICS][64];    // Subscribed topics
    int topic_count;                // Number of subscribed topics
    bool active;                    // Whether the client is active
    time_t last_activity;           // Last activity timestamp
} websocket_client_t;

// Global state
static websocket_client_t s_clients[MAX_CLIENTS];
static pthread_mutex_t s_mutex;

// Forward declarations
extern bool websocket_manager_is_initialized(void);
extern int websocket_manager_init(void);

/**
 * @brief Generate a random client ID
 * 
 * @param buffer Buffer to store the ID
 * @param buffer_size Buffer size
 */
static void generate_client_id(char *buffer, size_t buffer_size) {
    // Use the generate_uuid function from onvif_discovery_messages.h
    generate_uuid(buffer, buffer_size);
}

/**
 * @brief Find a client by ID
 * 
 * @param client_id Client ID
 * @return int Client index or -1 if not found
 */
int websocket_client_find_by_id(const char *client_id) {
    for (int i = 0; i < MAX_CLIENTS; i++) {
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
    for (int i = 0; i < MAX_CLIENTS; i++) {
        if (s_clients[i].active && s_clients[i].conn == conn) {
            return i;
        }
    }
    return -1;
}

/**
 * @brief Find a free client slot
 * 
 * @return int Client index or -1 if no free slots
 */
static int find_free_client_slot(void) {
    for (int i = 0; i < MAX_CLIENTS; i++) {
        if (!s_clients[i].active) {
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
    pthread_mutex_lock(&s_mutex);
    
    int client_index = websocket_client_find_by_connection(conn);
    if (client_index < 0) {
        pthread_mutex_unlock(&s_mutex);
        return false;
    }
    
    // Log client removal
    log_info("Removing WebSocket client: %s", s_clients[client_index].id);
    
    // Mark client as inactive
    s_clients[client_index].active = false;
    s_clients[client_index].conn = NULL;
    s_clients[client_index].topic_count = 0;
    
    pthread_mutex_unlock(&s_mutex);
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
    // Check if WebSocket manager is initialized
    if (!websocket_manager_is_initialized()) {
        log_error("WebSocket manager not initialized");
        // Try to initialize it
        if (websocket_manager_init() != 0) {
            log_error("Failed to initialize WebSocket manager");
            return false;
        }
        log_info("WebSocket manager initialized on demand during subscription check");
    }
    
    if (!client_id || !topic) {
        log_error("Invalid parameters for websocket_client_is_subscribed");
        return false;
    }
    
    // Find client - USING MUTEX LOCK to prevent race condition
    pthread_mutex_lock(&s_mutex);
    int client_index = websocket_client_find_by_id(client_id);
    if (client_index < 0) {
        log_error("Client not found: %s", client_id);
        pthread_mutex_unlock(&s_mutex);
        return false;
    }
    
    // Check if client is subscribed to topic
    bool is_subscribed = false;
    for (int i = 0; i < s_clients[client_index].topic_count; i++) {
        if (strcmp(s_clients[client_index].topics[i], topic) == 0) {
            is_subscribed = true;
            break;
        }
    }
    
    pthread_mutex_unlock(&s_mutex);
    return is_subscribed;
}

/**
 * @brief Get all clients subscribed to a topic
 * 
 * @param topic Topic to check
 * @param client_ids Pointer to array of client IDs (will be allocated)
 * @return int Number of clients subscribed to the topic
 */
int websocket_client_get_subscribed(const char *topic, char ***client_ids) {
    // Check if WebSocket manager is initialized
    if (!websocket_manager_is_initialized()) {
        log_error("WebSocket manager not initialized");
        // Try to initialize it
        if (websocket_manager_init() != 0) {
            log_error("Failed to initialize WebSocket manager");
            return 0;
        }
        log_info("WebSocket manager initialized on demand during get subscribed clients");
    }
    
    if (!topic || !client_ids) {
        log_error("Invalid parameters for websocket_client_get_subscribed");
        return 0;
    }
    
    // Initialize client_ids to NULL
    *client_ids = NULL;
    
    pthread_mutex_lock(&s_mutex);
    
    // Clean up inactive clients first
    websocket_client_cleanup_inactive();
    
    // Count subscribed clients
    int count = 0;
    for (int i = 0; i < MAX_CLIENTS; i++) {
        if (s_clients[i].active) {
            for (int j = 0; j < s_clients[i].topic_count; j++) {
                if (strcmp(s_clients[i].topics[j], topic) == 0) {
                    count++;
                    break;
                }
            }
        }
    }
    
    if (count == 0) {
        pthread_mutex_unlock(&s_mutex);
        return 0;
    }
    
    // Allocate memory for client IDs
    *client_ids = (char **)malloc(count * sizeof(char *));
    if (!*client_ids) {
        log_error("Failed to allocate memory for client IDs");
        pthread_mutex_unlock(&s_mutex);
        return 0;
    }
    
    // Initialize all pointers to NULL to handle errors safely
    for (int i = 0; i < count; i++) {
        (*client_ids)[i] = NULL;
    }
    
    // Fill client IDs
    int index = 0;
    for (int i = 0; i < MAX_CLIENTS; i++) {
        if (s_clients[i].active) {
            for (int j = 0; j < s_clients[i].topic_count; j++) {
                if (strcmp(s_clients[i].topics[j], topic) == 0) {
                    (*client_ids)[index] = strdup(s_clients[i].id);
                    if (!(*client_ids)[index]) {
                        log_error("Failed to allocate memory for client ID");
                        // Free already allocated client IDs
                        for (int k = 0; k < index; k++) {
                            if ((*client_ids)[k]) {
                                free((*client_ids)[k]);
                                (*client_ids)[k] = NULL;  // Set to NULL after freeing
                            }
                        }
                        free(*client_ids);
                        *client_ids = NULL;  // Set to NULL after freeing
                        pthread_mutex_unlock(&s_mutex);
                        return 0;
                    }
                    index++;
                    break;
                }
            }
        }
    }
    
    pthread_mutex_unlock(&s_mutex);
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
    // Check if WebSocket manager is initialized
    if (!websocket_manager_is_initialized()) {
        log_error("WebSocket manager not initialized");
        // Try to initialize it
        if (websocket_manager_init() != 0) {
            log_error("Failed to initialize WebSocket manager");
            return false;
        }
        log_info("WebSocket manager initialized on demand during subscription");
    }
    
    if (!client_id || !topic) {
        log_error("Invalid parameters for websocket_client_subscribe");
        return false;
    }
    
    pthread_mutex_lock(&s_mutex);
    int client_index = websocket_client_find_by_id(client_id);
    if (client_index < 0) {
        log_error("Client not found: %s", client_id);
        pthread_mutex_unlock(&s_mutex);
        return false;
    }
    
    if (s_clients[client_index].topic_count >= MAX_TOPICS) {
        log_error("Client %s has too many subscriptions", client_id);
        pthread_mutex_unlock(&s_mutex);
        return false;
    }
    
    // Check if already subscribed
    for (int i = 0; i < s_clients[client_index].topic_count; i++) {
        if (strcmp(s_clients[client_index].topics[i], topic) == 0) {
            // Already subscribed
            pthread_mutex_unlock(&s_mutex);
            return true;
        }
    }
    
    // Add subscription
    strncpy(s_clients[client_index].topics[s_clients[client_index].topic_count],
           topic, sizeof(s_clients[client_index].topics[0]) - 1);
    s_clients[client_index].topics[s_clients[client_index].topic_count][sizeof(s_clients[client_index].topics[0]) - 1] = '\0';
    s_clients[client_index].topic_count++;
    
    log_info("Client %s subscribed to topic %s", client_id, topic);
    
    pthread_mutex_unlock(&s_mutex);
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
    // Check if WebSocket manager is initialized
    if (!websocket_manager_is_initialized()) {
        log_error("WebSocket manager not initialized");
        // Try to initialize it
        if (websocket_manager_init() != 0) {
            log_error("Failed to initialize WebSocket manager");
            return false;
        }
        log_info("WebSocket manager initialized on demand during unsubscription");
    }
    
    if (!client_id || !topic) {
        log_error("Invalid parameters for websocket_client_unsubscribe");
        return false;
    }
    
    pthread_mutex_lock(&s_mutex);
    int client_index = websocket_client_find_by_id(client_id);
    if (client_index < 0) {
        log_error("Client not found: %s", client_id);
        pthread_mutex_unlock(&s_mutex);
        return false;
    }
    
    // Find topic subscription
    bool found = false;
    for (int i = 0; i < s_clients[client_index].topic_count; i++) {
        if (strcmp(s_clients[client_index].topics[i], topic) == 0) {
            // Remove subscription by shifting remaining topics
            for (int j = i; j < s_clients[client_index].topic_count - 1; j++) {
                strcpy(s_clients[client_index].topics[j], s_clients[client_index].topics[j + 1]);
            }
            
            s_clients[client_index].topic_count--;
            log_info("Client %s unsubscribed from topic %s", client_id, topic);
            found = true;
            break;
        }
    }
    
    pthread_mutex_unlock(&s_mutex);
    return found;
}

/**
 * @brief Update client activity timestamp
 * 
 * @param client_id Client ID
 */
void websocket_client_update_activity(const char *client_id) {
    if (!client_id) {
        log_error("Invalid parameters for websocket_client_update_activity");
        return;
    }
    
    pthread_mutex_lock(&s_mutex);
    int client_index = websocket_client_find_by_id(client_id);
    if (client_index >= 0) {
        s_clients[client_index].last_activity = time(NULL);
    }
    pthread_mutex_unlock(&s_mutex);
}

/**
 * @brief Clean up inactive clients
 */
void websocket_client_cleanup_inactive(void) {
    // This function should be called with the mutex already locked
    
    time_t now = time(NULL);
    const time_t timeout = 3600; // 1 hour timeout
    
    int cleaned_count = 0;
    
    for (int i = 0; i < MAX_CLIENTS; i++) {
        if (s_clients[i].active) {
            // Check if connection is valid
            if (!s_clients[i].conn || s_clients[i].conn->is_closing) {
                log_info("Cleaning up client %s with invalid connection", s_clients[i].id);
                s_clients[i].active = false;
                s_clients[i].conn = NULL;
                s_clients[i].topic_count = 0;
                cleaned_count++;
                continue;
            }
            
            // Check if client has been inactive for too long
            if (now - s_clients[i].last_activity > timeout) {
                log_info("Cleaning up inactive client %s (inactive for %ld seconds)",
                        s_clients[i].id, now - s_clients[i].last_activity);
                
                // Send a close frame if possible
                if (s_clients[i].conn && s_clients[i].conn->is_websocket && !s_clients[i].conn->is_closing) {
                    mg_ws_send(s_clients[i].conn, "", 0, WEBSOCKET_OP_CLOSE);
                    s_clients[i].conn->is_closing = 1;
                }
                
                s_clients[i].active = false;
                s_clients[i].conn = NULL;
                s_clients[i].topic_count = 0;
                cleaned_count++;
            }
        }
    }
    
    if (cleaned_count > 0) {
        log_info("Cleaned up %d inactive WebSocket clients", cleaned_count);
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
        log_error("Invalid parameters for websocket_client_get_connection");
        return NULL;
    }
    
    pthread_mutex_lock(&s_mutex);
    int client_index = websocket_client_find_by_id(client_id);
    struct mg_connection *conn = NULL;
    
    if (client_index >= 0) {
        conn = s_clients[client_index].conn;
    }
    
    pthread_mutex_unlock(&s_mutex);
    return conn;
}

// Functions for websocket_manager.c to use

/**
 * @brief Initialize client array
 */
void websocket_client_init(void) {
    memset(s_clients, 0, sizeof(s_clients));
}

/**
 * @brief Add a new client
 * 
 * @param conn Mongoose connection
 * @return char* Client ID or NULL on error
 */
char* websocket_client_add(struct mg_connection *conn) {
    if (!conn) {
        log_error("Invalid connection pointer in websocket_client_add");
        return NULL;
    }
    
    // Find a free client slot
    int slot = find_free_client_slot();
    if (slot < 0) {
        log_error("No free client slots");
        return NULL;
    }
    
    // Initialize client
    s_clients[slot].active = true;
    s_clients[slot].conn = conn;
    s_clients[slot].topic_count = 0;
    s_clients[slot].last_activity = time(NULL);
    
    // Generate client ID
    generate_client_id(s_clients[slot].id, sizeof(s_clients[slot].id));
    
    log_info("WebSocket client connected: %s", s_clients[slot].id);
    
    return s_clients[slot].id;
}

/**
 * @brief Get mutex for client operations
 * 
 * @return pthread_mutex_t* Mutex
 */
pthread_mutex_t* websocket_client_get_mutex(void) {
    return &s_mutex;
}
