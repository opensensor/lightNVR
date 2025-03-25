#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <time.h>
#include <sys/time.h>
#include <errno.h>

#include "web/websocket_manager.h"
#include "core/logger.h"
#include "../external/cjson/cJSON.h"
#include "video/onvif_discovery_messages.h"
#include "web/register_websocket_handlers.h"

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
    
    // Try to lock the mutex with a non-blocking approach to avoid deadlocks
    if (pthread_mutex_trylock(&s_mutex) != 0) {
        log_warn("Could not acquire WebSocket mutex within timeout, forcing shutdown");
        // Continue with shutdown anyway
    } else {
        // Close all connections
        int closed_count = 0;
        for (int i = 0; i < MAX_CLIENTS; i++) {
            if (s_clients[i].active && s_clients[i].conn) {
                // Mark connection for closing
                s_clients[i].conn->is_closing = 1;
                closed_count++;
            }
        }
        
        log_info("Marked %d WebSocket connections for closing", closed_count);
        
        // IMPORTANT: Don't reset state immediately - give connections time to close
        pthread_mutex_unlock(&s_mutex);
        
        // Wait a short time for connections to start closing
        usleep(100000);  // 100ms
    }
    
    // Wait a bit longer for connections to finish closing
    usleep(500000);  // 500ms
    
    // Now it's safer to clean up the state
    if (pthread_mutex_trylock(&s_mutex) == 0) {
        // Reset state after waiting for connections to close
        memset(s_clients, 0, sizeof(s_clients));
        memset(s_handlers, 0, sizeof(s_handlers));
        pthread_mutex_unlock(&s_mutex);
    }
    
    // Destroy mutex
    pthread_mutex_destroy(&s_mutex);
    
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
        free(message);
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
        free(message);
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
        free(message);
        return;
    }
    
    const char *type = type_json->valuestring;
    const char *topic = topic_json->valuestring;
    
    // Convert payload to string
    char *payload = cJSON_PrintUnformatted(payload_json);
    if (!payload) {
        log_error("Failed to convert payload to string");
        cJSON_Delete(json);
        free(message);
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
    
    // Make a copy of the client ID to use after releasing the mutex
    char client_id[64];
    strncpy(client_id, s_clients[client_index].id, sizeof(client_id) - 1);
    client_id[sizeof(client_id) - 1] = '\0';
    pthread_mutex_unlock(&s_mutex);
    
    // Handle message based on type
    if (strcmp(type, "subscribe") == 0) {
        // Subscribe to topic
        if (s_clients[client_index].topic_count >= MAX_TOPICS) {
            log_error("Client %s has too many subscriptions", client_id);
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
                
                // Send acknowledgment
                websocket_message_t *ack = websocket_message_create(
                    "ack", "system", "{\"message\":\"Subscribed\"}");
                
                if (ack) {
                    websocket_manager_send_to_client(client_id, ack);
                    websocket_message_free(ack);
                }
                
                // Make sure handlers are registered
                register_websocket_handlers();
            }
        }
    } else if (strcmp(type, "unsubscribe") == 0) {
        // Unsubscribe from topic
        for (int i = 0; i < s_clients[client_index].topic_count; i++) {
            if (strcmp(s_clients[client_index].topics[i], topic) == 0) {
                // Remove subscription by shifting remaining topics
                for (int j = i; j < s_clients[client_index].topic_count - 1; j++) {
                    strcpy(s_clients[client_index].topics[j], s_clients[client_index].topics[j + 1]);
                }
                
                s_clients[client_index].topic_count--;
                log_info("Client %s unsubscribed from topic %s", client_id, topic);
                
                // Send acknowledgment
                websocket_message_t *ack = websocket_message_create(
                    "ack", "system", "{\"message\":\"Unsubscribed\"}");
                
                if (ack) {
                    websocket_manager_send_to_client(client_id, ack);
                    websocket_message_free(ack);
                }
                
                break;
            }
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
 * @brief Create a WebSocket message
 * 
 * @param type Message type
 * @param topic Message topic
 * @param payload Message payload
 * @return websocket_message_t* Pointer to the message or NULL on error
 */
websocket_message_t *websocket_message_create(const char *type, const char *topic, const char *payload) {
    if (!type || !topic || !payload) {
        log_error("Invalid parameters for websocket_message_create");
        return NULL;
    }
    
    websocket_message_t *message = calloc(1, sizeof(websocket_message_t));
    if (!message) {
        log_error("Failed to allocate memory for WebSocket message");
        return NULL;
    }
    
    message->type = strdup(type);
    message->topic = strdup(topic);
    message->payload = strdup(payload);
    
    if (!message->type || !message->topic || !message->payload) {
        log_error("Failed to allocate memory for WebSocket message fields");
        websocket_message_free(message);
        return NULL;
    }
    
    return message;
}

/**
 * @brief Free a WebSocket message
 * 
 * @param message Message to free
 */
void websocket_message_free(websocket_message_t *message) {
    if (message) {
        if (message->type) {
            free(message->type);
        }
        
        if (message->topic) {
            free(message->topic);
        }
        
        if (message->payload) {
            free(message->payload);
        }
        
        free(message);
    }
}

/**
 * @brief Send a WebSocket message to a client
 * 
 * @param client_id Client ID
 * @param message Message to send
 * @return bool true on success, false on error
 */
bool websocket_manager_send_to_client(const char *client_id, const websocket_message_t *message) {
    // Check if WebSocket manager is initialized
    if (!websocket_manager_is_initialized()) {
        log_error("WebSocket manager not initialized");
        // Try to initialize it
        if (websocket_manager_init() != 0) {
            log_error("Failed to initialize WebSocket manager");
            return false;
        }
        log_info("WebSocket manager initialized on demand during send to client");
    }
    
    if (!client_id || !message) {
        log_error("Invalid parameters for websocket_manager_send_to_client");
        return false;
    }
    
    // Find client - USING MUTEX LOCK to prevent race condition
    pthread_mutex_lock(&s_mutex);
    int client_index = find_client_by_id(client_id);
    if (client_index < 0) {
        log_error("Client not found: %s", client_id);
        pthread_mutex_unlock(&s_mutex);
        return false;
    }
    
    // Get a copy of the connection pointer while holding the mutex
    struct mg_connection *conn = s_clients[client_index].conn;
    if (!conn) {
        log_error("Client %s has no connection", client_id);
        pthread_mutex_unlock(&s_mutex);
        return false;
    }
    pthread_mutex_unlock(&s_mutex);
    
    // Log the message details for debugging
    log_info("Sending message to client %s: type=%s, topic=%s",
             client_id, message->type, message->topic);
    
    // Convert message to JSON
    cJSON *json = cJSON_CreateObject();
    cJSON_AddStringToObject(json, "type", message->type);
    cJSON_AddStringToObject(json, "topic", message->topic);
    
    // Always parse payload as JSON
    cJSON *payload_json = cJSON_Parse(message->payload);
    if (payload_json) {
        // If payload is valid JSON, add it as an object
        cJSON_AddItemToObject(json, "payload", payload_json);
        log_debug("Added payload as JSON object");
    } else {
        // If parsing failed, log an error and create a new JSON object
        log_error("Failed to parse payload as JSON: %s", message->payload);
        
        // For progress and result messages, try to add the payload as a string
        if (strcmp(message->type, "progress") == 0 || strcmp(message->type, "result") == 0) {
            log_info("Adding %s payload as string for client %s", message->type, client_id);
            cJSON_AddStringToObject(json, "payload", message->payload);
        } else {
            // Create a new JSON object for the payload
            cJSON *new_payload = cJSON_CreateObject();
            cJSON_AddStringToObject(new_payload, "error", "Failed to parse payload");
            cJSON_AddStringToObject(new_payload, "raw_payload", message->payload);
            
            // Add the new payload object
            cJSON_AddItemToObject(json, "payload", new_payload);
            log_debug("Added error payload object");
        }
    }
    
    char *json_str = cJSON_PrintUnformatted(json);
    if (!json_str) {
        log_error("Failed to convert message to JSON");
        cJSON_Delete(json);
        return false;
    }
    
    // Send message using the connection pointer we saved earlier
    mg_ws_send(conn, json_str, strlen(json_str), WEBSOCKET_OP_TEXT);
    log_debug("Sent WebSocket message to client %s", client_id);
    
    free(json_str);
    cJSON_Delete(json);
    
    return true;
}

/**
 * @brief Send a WebSocket message to all clients subscribed to a topic
 * 
 * @param topic Topic to send to
 * @param message Message to send
 * @return int Number of clients the message was sent to
 */
int websocket_manager_broadcast(const char *topic, const websocket_message_t *message) {
    // Check if WebSocket manager is initialized
    if (!websocket_manager_is_initialized()) {
        log_error("WebSocket manager not initialized");
        // Try to initialize it
        if (websocket_manager_init() != 0) {
            log_error("Failed to initialize WebSocket manager");
            return 0;
        }
        log_info("WebSocket manager initialized on demand during broadcast");
    }
    
    if (!topic || !message) {
        log_error("Invalid parameters for websocket_manager_broadcast");
        return 0;
    }
    
    pthread_mutex_lock(&s_mutex);
    
    // Convert message to JSON
    cJSON *json = cJSON_CreateObject();
    cJSON_AddStringToObject(json, "type", message->type);
    cJSON_AddStringToObject(json, "topic", message->topic);
    
    // Always parse payload as JSON
    cJSON *payload_json = cJSON_Parse(message->payload);
    if (payload_json) {
        // If payload is valid JSON, add it as an object
        cJSON_AddItemToObject(json, "payload", payload_json);
        log_debug("Added payload as JSON object for broadcast");
    } else {
        // If parsing failed, log an error and create a new JSON object
        log_error("Failed to parse payload as JSON for broadcast: %s", message->payload);
        
        // Create a new JSON object for the payload
        cJSON *new_payload = cJSON_CreateObject();
        cJSON_AddStringToObject(new_payload, "error", "Failed to parse payload");
        cJSON_AddStringToObject(new_payload, "raw_payload", message->payload);
        
        // Add the new payload object
        cJSON_AddItemToObject(json, "payload", new_payload);
        log_debug("Added error payload object for broadcast");
    }
    
    char *json_str = cJSON_PrintUnformatted(json);
    if (!json_str) {
        log_error("Failed to convert message to JSON");
        cJSON_Delete(json);
        pthread_mutex_unlock(&s_mutex);
        return 0;
    }
    
    // Create a temporary array to store connection pointers
    struct mg_connection *connections[MAX_CLIENTS];
    int connection_count = 0;
    
    // Find all subscribed clients and store their connection pointers
    for (int i = 0; i < MAX_CLIENTS; i++) {
        if (s_clients[i].active && s_clients[i].conn) {
            // Check if client is subscribed to topic
            for (int j = 0; j < s_clients[i].topic_count; j++) {
                if (strcmp(s_clients[i].topics[j], topic) == 0) {
                    // Store connection pointer
                    connections[connection_count++] = s_clients[i].conn;
                    break;
                }
            }
        }
    }
    
    // Release the mutex before sending messages
    pthread_mutex_unlock(&s_mutex);
    
    // Send message to all stored connections
    for (int i = 0; i < connection_count; i++) {
        mg_ws_send(connections[i], json_str, strlen(json_str), WEBSOCKET_OP_TEXT);
    }
    
    // Clean up
    free(json_str);
    cJSON_Delete(json);
    
    return connection_count;
}

/**
 * @brief Check if a client is subscribed to a topic
 * 
 * @param client_id Client ID
 * @param topic Topic to check
 * @return bool true if subscribed, false otherwise
 */
bool websocket_manager_is_subscribed(const char *client_id, const char *topic) {
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
        log_error("Invalid parameters for websocket_manager_is_subscribed");
        return false;
    }
    
    // Find client - USING MUTEX LOCK to prevent race condition
    pthread_mutex_lock(&s_mutex);
    int client_index = find_client_by_id(client_id);
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
int websocket_manager_get_subscribed_clients(const char *topic, char ***client_ids) {
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
        log_error("Invalid parameters for websocket_manager_get_subscribed_clients");
        return 0;
    }
    
    // Initialize client_ids to NULL
    *client_ids = NULL;
    
    pthread_mutex_lock(&s_mutex);
    
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
                            free((*client_ids)[k]);
                        }
                        free(*client_ids);
                        *client_ids = NULL;
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

/**
 * @brief Register a WebSocket message handler
 * 
 * @param topic Topic to handle
 * @param handler Handler function
 * @return int 0 on success, non-zero on error
 */
int websocket_manager_register_handler(const char *topic, void (*handler)(const char *client_id, const char *message)) {
    // Check if WebSocket manager is initialized
    if (!websocket_manager_is_initialized()) {
        log_error("WebSocket manager not initialized");
        // Try to initialize it
        if (websocket_manager_init() != 0) {
            log_error("Failed to initialize WebSocket manager");
            return -1;
        }
        log_info("WebSocket manager initialized on demand during handler registration");
    }
    
    if (!topic || !handler) {
        log_error("Invalid parameters for websocket_manager_register_handler");
        return -1;
    }
    
    pthread_mutex_lock(&s_mutex);
    
    // Check if handler already exists
    int handler_index = find_handler_by_topic(topic);
    if (handler_index >= 0) {
        // Update handler
        s_handlers[handler_index].handler = handler;
        pthread_mutex_unlock(&s_mutex);
        return 0;
    }
    
    // Find a free handler slot
    int slot = find_free_handler_slot();
    if (slot < 0) {
        log_error("No free handler slots");
        pthread_mutex_unlock(&s_mutex);
        return -1;
    }
    
    // Register handler
    strncpy(s_handlers[slot].topic, topic, sizeof(s_handlers[slot].topic) - 1);
    s_handlers[slot].topic[sizeof(s_handlers[slot].topic) - 1] = '\0';
    s_handlers[slot].handler = handler;
    s_handlers[slot].active = true;
    
    log_info("Registered WebSocket handler for topic %s", topic);
    
    pthread_mutex_unlock(&s_mutex);
    return 0;
}
