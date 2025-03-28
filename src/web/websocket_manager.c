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
#include <unistd.h>

#include "web/websocket_manager.h"
#include "web/websocket_client.h"
#include "web/websocket_handler.h"
#include "web/websocket_message.h"
#include "core/logger.h"
#include "core/shutdown_coordinator.h"
#include "../external/cjson/cJSON.h"
#include "web/register_websocket_handlers.h"
#include "video/onvif_discovery_messages.h"

// Maximum number of clients and handlers
#define MAX_CLIENTS 100
#define MAX_HANDLERS 20
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

// Handler structure
typedef struct {
    char topic[64];                                     // Topic to handle
    void (*handler)(const char *client_id, const char *message);  // Handler function
    bool active;                                        // Whether the handler is active
} websocket_handler_t;

// Global state
static websocket_client_t s_clients[MAX_CLIENTS];
static websocket_handler_t s_handlers[MAX_HANDLERS];
static pthread_mutex_t s_mutex;
static bool s_initialized = false;

// Global mutex to protect initialization
static pthread_mutex_t s_init_mutex = PTHREAD_MUTEX_INITIALIZER;

// Forward declaration for cleanup function
static void websocket_manager_cleanup_inactive_clients(void);

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
static int find_client_by_id(const char *client_id) {
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
static int find_client_by_connection(const struct mg_connection *conn) {
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
 * @brief Find a handler by topic
 * 
 * @param topic Topic to find
 * @return int Handler index or -1 if not found
 */
static int find_handler_by_topic(const char *topic) {
    for (int i = 0; i < MAX_HANDLERS; i++) {
        if (s_handlers[i].active && strcmp(s_handlers[i].topic, topic) == 0) {
            return i;
        }
    }
    return -1;
}

/**
 * @brief Find a free handler slot
 * 
 * @return int Handler index or -1 if no free slots
 */
static int find_free_handler_slot(void) {
    for (int i = 0; i < MAX_HANDLERS; i++) {
        if (!s_handlers[i].active) {
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
static bool remove_client_by_connection(const struct mg_connection *conn) {
    pthread_mutex_lock(&s_mutex);
    
    int client_index = find_client_by_connection(conn);
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
 * @brief Clean up inactive clients
 * 
 * This function removes clients that have been inactive for too long
 * or have invalid connections.
 */
static void websocket_manager_cleanup_inactive_clients(void) {
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
 * @brief Initialize WebSocket manager
 * 
 * @return int 0 on success, non-zero on error
 */
int websocket_manager_init(void) {
    // Use the initialization mutex to ensure thread safety
    pthread_mutex_lock(&s_init_mutex);
    
    if (s_initialized) {
        log_warn("WebSocket manager already initialized");
        pthread_mutex_unlock(&s_init_mutex);
        return 0;
    }
    
    // Initialize mutex
    if (pthread_mutex_init(&s_mutex, NULL) != 0) {
        log_error("Failed to initialize WebSocket manager mutex");
        pthread_mutex_unlock(&s_init_mutex);
        return -1;
    }
    
    // Initialize clients and handlers
    memset(s_clients, 0, sizeof(s_clients));
    memset(s_handlers, 0, sizeof(s_handlers));
    
    s_initialized = true;
    log_info("WebSocket manager initialized");
    
    pthread_mutex_unlock(&s_init_mutex);
    return 0;
}

/**
 * @brief Shutdown WebSocket manager
 */
void websocket_manager_shutdown(void) {
    // Use the initialization mutex to ensure thread safety
    if (pthread_mutex_trylock(&s_init_mutex) != 0) {
        log_warn("WebSocket manager shutdown already in progress");
        return;
    }
    
    if (!s_initialized) {
        pthread_mutex_unlock(&s_init_mutex);
        return;
    }
    
    log_info("WebSocket manager shutting down...");
    
    // Set initialized to false first to prevent new operations
    s_initialized = false;
    
    // Register with shutdown coordinator if available
    shutdown_coordinator_t *coordinator = NULL;
    int websocket_component_id = -1;
    
    // Get the shutdown coordinator if it's available
    coordinator = get_shutdown_coordinator();
    if (coordinator != NULL) {
        // Register the websocket manager as a component
        websocket_component_id = register_component("websocket_manager", COMPONENT_OTHER, NULL, 10);
        if (websocket_component_id >= 0) {
            log_info("Registered WebSocket manager with shutdown coordinator, component ID: %d", 
                     websocket_component_id);
            // Update state to stopping
            update_component_state(websocket_component_id, COMPONENT_STOPPING);
        }
    }
    
    // First, make a copy of all active client connections to ensure we can close them
    // even if we can't acquire the mutex
    struct mg_connection *active_connections[MAX_CLIENTS] = {NULL};
    char client_ids[MAX_CLIENTS][64] = {{0}};
    int active_count = 0;
    
    // Try to acquire the mutex with a short timeout
    struct timespec timeout;
    clock_gettime(CLOCK_REALTIME, &timeout);
    timeout.tv_sec += 1; // Reduced to 1 second for faster shutdown
    
    int mutex_result = pthread_mutex_timedlock(&s_mutex, &timeout);
    if (mutex_result == 0) {
        // We successfully acquired the mutex
        
        // First, make a copy of all active connections
        for (int i = 0; i < MAX_CLIENTS; i++) {
            if (s_clients[i].active && s_clients[i].conn != NULL) {
                active_connections[active_count] = s_clients[i].conn;
                strncpy(client_ids[active_count], s_clients[i].id, sizeof(client_ids[0]) - 1);
                active_count++;
                
                // Mark as inactive to prevent further operations
                s_clients[i].active = false;
            }
        }
        
        // Clear handler state safely
        for (int i = 0; i < MAX_HANDLERS; i++) {
            if (s_handlers[i].active) {
                s_handlers[i].active = false;
                s_handlers[i].handler = NULL;
                memset(s_handlers[i].topic, 0, sizeof(s_handlers[i].topic));
            }
        }
        
        // Clear all client data
        for (int i = 0; i < MAX_CLIENTS; i++) {
            s_clients[i].active = false;
            s_clients[i].conn = NULL;
            s_clients[i].topic_count = 0;
            memset(s_clients[i].topics, 0, sizeof(s_clients[i].topics));
        }
        
        // Release the mutex
        pthread_mutex_unlock(&s_mutex);
    } else {
        log_warn("Could not acquire WebSocket mutex within timeout, proceeding with shutdown");
        
        // Even without the mutex, try to find active connections by scanning the client array
        // This is a best-effort approach when the mutex is unavailable
        for (int i = 0; i < MAX_CLIENTS; i++) {
            if (s_clients[i].conn != NULL) {
                active_connections[active_count] = s_clients[i].conn;
                strncpy(client_ids[active_count], s_clients[i].id, sizeof(client_ids[0]) - 1);
                active_count++;
                
                // Mark as inactive to prevent further operations
                s_clients[i].active = false;
                s_clients[i].conn = NULL;
            }
        }
    }
    
    // Now close all the connections we found
    log_info("Closing %d active WebSocket connections", active_count);
    int closed_count = 0;
    
    for (int i = 0; i < active_count; i++) {
        struct mg_connection *conn = active_connections[i];
        if (conn != NULL) {
            // Send a proper WebSocket close frame before marking for closing
            if (conn->is_websocket && !conn->is_closing) {
                // Send close frame with normal closure code (1000)
                uint16_t close_code = 1000; // Normal closure
                char close_frame[2];
                close_frame[0] = (close_code >> 8) & 0xFF;
                close_frame[1] = close_code & 0xFF;
                mg_ws_send(conn, close_frame, 2, WEBSOCKET_OP_CLOSE);
                
                // Mark connection for closing
                conn->is_closing = 1;
                
                // Explicitly close the socket to ensure it's released
                if (conn->fd != NULL) {
                    int socket_fd = (int)(size_t)conn->fd;
                    log_debug("Closing WebSocket socket: %d for client %s", socket_fd, client_ids[i]);
                    
                    // Set SO_LINGER to force immediate socket closure
                    struct linger so_linger;
                    so_linger.l_onoff = 1;
                    so_linger.l_linger = 0;
                    setsockopt(socket_fd, SOL_SOCKET, SO_LINGER, &so_linger, sizeof(so_linger));
                    
                    // Set socket to non-blocking mode to avoid hang on close
                    int flags = fcntl(socket_fd, F_GETFL, 0);
                    fcntl(socket_fd, F_SETFL, flags | O_NONBLOCK);
                    
                    // Now close the socket
                    close(socket_fd);
                    conn->fd = NULL;  // Mark as closed
                }
                
                closed_count++;
            }
        }
    }
    
    log_info("Closed %d WebSocket connections", closed_count);
    
    // Update shutdown coordinator if we registered
    if (coordinator != NULL && websocket_component_id >= 0) {
        update_component_state(websocket_component_id, COMPONENT_STOPPED);
        log_info("Updated WebSocket manager state to STOPPED in shutdown coordinator");
    }
    
    // Destroy mutex with proper error handling
    // Only attempt to destroy if we're not in a forced shutdown situation
    if (mutex_result == 0) {
        int destroy_result = pthread_mutex_destroy(&s_mutex);
        if (destroy_result != 0) {
            log_warn("Failed to destroy WebSocket mutex: %d (%s)", destroy_result, strerror(destroy_result));
        }
    }
    
    log_info("WebSocket manager shutdown complete");
    
    pthread_mutex_unlock(&s_init_mutex);
}

/**
 * @brief Handle WebSocket connection open
 * 
 * @param c Mongoose connection
 */
void websocket_manager_handle_open(struct mg_connection *c) {
    log_info("websocket_manager_handle_open called");
    
    // Check if WebSocket manager is initialized
    if (!websocket_manager_is_initialized()) {
        log_error("WebSocket manager not initialized");
        // Try to initialize it
        if (websocket_manager_init() != 0) {
            log_error("Failed to initialize WebSocket manager");
            return;
        }
        log_info("WebSocket manager initialized on demand during connection open");
    }
    
    if (!c) {
        log_error("Invalid connection pointer in websocket_manager_handle_open");
        return;
    }
    
    pthread_mutex_lock(&s_mutex);
    
    // Clean up inactive clients before adding a new one
    websocket_manager_cleanup_inactive_clients();
    
    // Find a free client slot
    int slot = find_free_client_slot();
    if (slot < 0) {
        log_error("No free client slots");
        pthread_mutex_unlock(&s_mutex);
        return;
    }
    
    // Initialize client
    s_clients[slot].active = true;
    s_clients[slot].conn = c;
    s_clients[slot].topic_count = 0;
    s_clients[slot].last_activity = time(NULL);
    
    // Generate client ID
    generate_client_id(s_clients[slot].id, sizeof(s_clients[slot].id));
    
    log_info("WebSocket client connected: %s", s_clients[slot].id);
    
    // Send welcome message with client ID - use a simpler approach for older systems
    char payload[256];
    snprintf(payload, sizeof(payload), "{\"client_id\":\"%s\"}", s_clients[slot].id);
    
    log_info("Preparing welcome message with payload: %s", payload);
    
    // Create a simple JSON string directly without using cJSON for better compatibility
    char welcome_message[512];
    snprintf(welcome_message, sizeof(welcome_message), 
             "{\"type\":\"welcome\",\"topic\":\"system\",\"payload\":%s}", payload);
    
    // Send the welcome message
    log_info("Sending welcome message: %s", welcome_message);
    
    // Use a try-catch style approach with error handling
    int send_result = -1;
    
    // Try to send the message
    if (c && c->is_websocket) {
        send_result = mg_ws_send(c, welcome_message, strlen(welcome_message), WEBSOCKET_OP_TEXT);
        if (send_result > 0) {
            log_info("Welcome message sent successfully (%d bytes)", send_result);
        } else {
            log_error("Failed to send welcome message, error code: %d", send_result);
        }
    } else {
        log_error("Cannot send welcome message - connection is not a valid WebSocket");
    }
    
    // If sending failed, try a simpler message format
    if (send_result <= 0) {
        log_info("Trying simplified welcome message format");
        const char *simple_message = "{\"type\":\"welcome\",\"topic\":\"system\",\"payload\":{\"client_id\":\"";
        const char *simple_message_end = "\"}}";
        
        char simple_welcome[384];
        snprintf(simple_welcome, sizeof(simple_welcome), "%s%s%s", 
                 simple_message, s_clients[slot].id, simple_message_end);
        
        if (c && c->is_websocket) {
            send_result = mg_ws_send(c, simple_welcome, strlen(simple_welcome), WEBSOCKET_OP_TEXT);
            if (send_result > 0) {
                log_info("Simplified welcome message sent successfully (%d bytes)", send_result);
            } else {
                log_error("Failed to send simplified welcome message, error code: %d", send_result);
            }
        }
    }
    
    pthread_mutex_unlock(&s_mutex);
    log_info("websocket_manager_handle_open completed");
}

/**
 * @brief Handle WebSocket message
 * 
 * @param c Mongoose connection
 * @param data Message data
 * @param data_len Message data length
 */
void websocket_manager_handle_message(struct mg_connection *c, const char *data, size_t data_len) {
    // Check if WebSocket manager is initialized
    if (!websocket_manager_is_initialized()) {
        log_error("WebSocket manager not initialized");
        // Try to initialize it
        if (websocket_manager_init() != 0) {
            log_error("Failed to initialize WebSocket manager");
            return;
        }
        log_info("WebSocket manager initialized on demand during message handling");
    }
    
    // Copy data to null-terminated string
    char *message = malloc(data_len + 1);
    if (!message) {
        log_error("Failed to allocate memory for WebSocket message");
        return;
    }
    
    memcpy(message, data, data_len);
    message[data_len] = '\0';
    
    log_debug("Received WebSocket message: %s", message);
    
    // Parse JSON message
    cJSON *json = cJSON_Parse(message);
    if (!json) {
        log_error("Failed to parse WebSocket message as JSON");
        free(message);  // Free message buffer on error path
        return;
    }
    
    // Extract message type, topic, and payload
    cJSON *type_json = cJSON_GetObjectItem(json, "type");
    cJSON *topic_json = cJSON_GetObjectItem(json, "topic");
    cJSON *payload_json = cJSON_GetObjectItem(json, "payload");
    
    if (!type_json || !cJSON_IsString(type_json) ||
        !topic_json || !cJSON_IsString(topic_json)) {
        log_error("Invalid WebSocket message format - missing type or topic");
        cJSON_Delete(json);
        free(message);  // Free message buffer on error path
        return;
    }
    
    // Payload can be empty or missing for subscribe/unsubscribe messages
    if (!payload_json) {
        log_warn("Message has no payload, using empty object");
        payload_json = cJSON_CreateObject();
        cJSON_AddItemToObject(json, "payload", payload_json);
    } else if (!cJSON_IsObject(payload_json)) {
        log_error("Invalid payload format - not an object");
        cJSON_Delete(json);
        free(message);  // Free message buffer on error path
        return;
    }
    
    const char *type = type_json->valuestring;
    const char *topic = topic_json->valuestring;
    
    // Convert payload to string
    char *payload = cJSON_PrintUnformatted(payload_json);
    if (!payload) {
        log_error("Failed to convert payload to string");
        cJSON_Delete(json);
        free(message);  // Fix: Free message buffer on error path
        return;
    }
    
    // Find client - USING MUTEX LOCK to prevent race condition
    pthread_mutex_lock(&s_mutex);
    int client_index = find_client_by_connection(c);
    if (client_index < 0) {
        log_error("Client not found for connection");
        pthread_mutex_unlock(&s_mutex);
        free(payload);
        cJSON_Delete(json);
        free(message);
        return;
    }
    
    // Update last activity timestamp
    s_clients[client_index].last_activity = time(NULL);
    
    // Make a copy of the client ID to use after releasing the mutex
    char client_id[64];
    strncpy(client_id, s_clients[client_index].id, sizeof(client_id) - 1);
    client_id[sizeof(client_id) - 1] = '\0';
    pthread_mutex_unlock(&s_mutex);
    
    // Handle message based on type
    if (strcmp(type, "subscribe") == 0) {
        // Subscribe to topic
        pthread_mutex_lock(&s_mutex);
        client_index = find_client_by_id(client_id);
        if (client_index < 0) {
            log_error("Client not found: %s", client_id);
            pthread_mutex_unlock(&s_mutex);
            free(payload);
            cJSON_Delete(json);
            free(message);
            return;
        }
        
        if (s_clients[client_index].topic_count >= MAX_TOPICS) {
            log_error("Client %s has too many subscriptions", client_id);
            pthread_mutex_unlock(&s_mutex);
            free(payload);  // Free payload on error path
            cJSON_Delete(json);
            free(message);
            return;  // Return to prevent memory leak
        } else {
            // Check if already subscribed
            bool already_subscribed = false;
            for (int i = 0; i < s_clients[client_index].topic_count; i++) {
                if (strcmp(s_clients[client_index].topics[i], topic) == 0) {
                    already_subscribed = true;
                    break;
                }
            }
            
            if (!already_subscribed) {
                // Add subscription
                strncpy(s_clients[client_index].topics[s_clients[client_index].topic_count],
                       topic, sizeof(s_clients[client_index].topics[0]) - 1);
                s_clients[client_index].topics[s_clients[client_index].topic_count][sizeof(s_clients[client_index].topics[0]) - 1] = '\0';
                s_clients[client_index].topic_count++;
                
                log_info("Client %s subscribed to topic %s", client_id, topic);
                
                // Check if payload contains client_id (for verification)
                cJSON *payload_client_id = cJSON_GetObjectItem(payload_json, "client_id");
                if (payload_client_id && cJSON_IsString(payload_client_id)) {
                    log_info("Subscription payload contains client_id: %s", payload_client_id->valuestring);
                }
                
                pthread_mutex_unlock(&s_mutex);
                
                // Send acknowledgment
                websocket_message_t *ack = websocket_message_create(
                    "ack", "system", "{\"message\":\"Subscribed\"}");
                
                if (ack) {
                    websocket_message_send_to_client(client_id, ack);
                    websocket_message_free(ack);
                }
                
                // Make sure handlers are registered
                register_websocket_handlers();
            } else {
                pthread_mutex_unlock(&s_mutex);
            }
        }
    } else if (strcmp(type, "unsubscribe") == 0) {
        // Unsubscribe from topic
        pthread_mutex_lock(&s_mutex);
        client_index = find_client_by_id(client_id);
        if (client_index < 0) {
            log_error("Client not found: %s", client_id);
            pthread_mutex_unlock(&s_mutex);
            free(payload);
            cJSON_Delete(json);
            free(message);
            return;
        }
        
        bool found = false;
        for (int i = 0; i < s_clients[client_index].topic_count; i++) {
            if (strcmp(s_clients[client_index].topics[i], topic) == 0) {
                // Remove subscription by shifting remaining topics
                for (int j = i; j < s_clients[client_index].topic_count - 1; j++) {
                    strcpy(s_clients[client_index].topics[j], s_clients[client_index].topics[j + 1]);
                }
                
                s_clients[client_index].topic_count--;
                log_info("Client %s unsubscribed from topic %s", client_id, topic);
                
                pthread_mutex_unlock(&s_mutex);
                
                // Send acknowledgment
                websocket_message_t *ack = websocket_message_create(
                    "ack", "system", "{\"message\":\"Unsubscribed\"}");
                
                if (ack) {
                    websocket_message_send_to_client(client_id, ack);
                    websocket_message_free(ack);
                }
                
                found = true;
                break;
            }
        }
        
        if (!found) {
            pthread_mutex_unlock(&s_mutex);
        }
    } else {
        // Handle message with registered handler
        int handler_index = find_handler_by_topic(topic);
        if (handler_index >= 0 && s_handlers[handler_index].handler) {
            // Call handler directly without mutex locks
            log_info("Found handler for topic %s, calling it", topic);
            s_handlers[handler_index].handler(client_id, payload);
        } else {
            log_warn("No handler registered for topic %s, attempting to register handlers", topic);
            // Try to register handlers again
            register_websocket_handlers();
            
            // Check if handler is now registered
            handler_index = find_handler_by_topic(topic);
            if (handler_index >= 0 && s_handlers[handler_index].handler) {
                log_info("Handler registered successfully, calling it now");
                s_handlers[handler_index].handler(client_id, payload);
            } else {
                log_error("Still no handler registered for topic %s after registration attempt", topic);
            }
        }
    }
    
    // Clean up
    free(payload);
    cJSON_Delete(json);
    free(message);
}

/**
 * @brief Handle WebSocket connection close
 * 
 * @param c Mongoose connection
 */
void websocket_manager_handle_close(struct mg_connection *c) {
    // Check if WebSocket manager is initialized
    if (!websocket_manager_is_initialized()) {
        log_error("WebSocket manager not initialized during connection close");
        return;
    }
    
    if (!c) {
        log_error("Invalid connection pointer in websocket_manager_handle_close");
        return;
    }
    
    log_info("WebSocket connection closed, cleaning up resources");
    
    // Remove client by connection
    if (remove_client_by_connection(c)) {
        log_info("WebSocket client removed successfully");
    } else {
        log_warn("WebSocket client not found for connection during close");
    }
}

/**
 * @brief Check if the WebSocket manager is initialized
 * 
 * @return bool true if initialized, false otherwise
 */
bool websocket_manager_is_initialized(void) {
    // Use the initialization mutex to ensure thread safety
    pthread_mutex_lock(&s_init_mutex);
    bool initialized = s_initialized;
    pthread_mutex_unlock(&s_init_mutex);
    return initialized;
}
