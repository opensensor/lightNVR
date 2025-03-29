#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <stdbool.h>

#include "web/websocket_manager.h"
#include "web/websocket_handler.h"
#include "core/logger.h"
#include "core/shutdown_coordinator.h"

// Maximum number of WebSocket handlers
#define MAX_WS_HANDLERS 32

// Maximum topic name length
#define MAX_TOPIC_LENGTH 64

// WebSocket handler structure
typedef struct {
    char topic[MAX_TOPIC_LENGTH];
    websocket_handler_t handler;
    void *user_data;
    bool active;
} ws_handler_entry_t;

// Global state
static bool s_initialized = false;
static pthread_mutex_t s_init_mutex = PTHREAD_MUTEX_INITIALIZER;
static pthread_mutex_t s_handlers_mutex = PTHREAD_MUTEX_INITIALIZER;

// WebSocket handlers
static ws_handler_entry_t s_handlers[MAX_WS_HANDLERS];
static int s_handler_count = 0;

/**
 * @brief Find a WebSocket handler by topic
 * 
 * @param topic Topic name
 * @return int Index of handler or -1 if not found
 */
static int find_handler_by_topic(const char *topic) {
    for (int i = 0; i < MAX_WS_HANDLERS; i++) {
        if (s_handlers[i].active && strcmp(s_handlers[i].topic, topic) == 0) {
            return i;
        }
    }
    return -1;
}

/**
 * @brief Find a free slot in the WebSocket handlers array
 * 
 * @return int Index of free slot or -1 if none available
 */
static int find_free_handler_slot(void) {
    for (int i = 0; i < MAX_WS_HANDLERS; i++) {
        if (!s_handlers[i].active) {
            return i;
        }
    }
    return -1;
}

/**
 * @brief Extract topic from WebSocket message
 * 
 * @param data Message data
 * @param data_len Message data length
 * @param topic Output buffer for topic
 * @param topic_size Size of output buffer
 * @return int 0 on success, non-zero on error
 */
static int extract_topic_from_message(const char *data, size_t data_len, char *topic, size_t topic_size) {
    // Default topic
    strncpy(topic, "default", topic_size - 1);
    topic[topic_size - 1] = '\0';
    
    // Check if data is valid
    if (!data || data_len == 0) {
        log_warn("Invalid data for topic extraction");
        return -1;
    }
    
    // Log the raw message for debugging
    char *debug_data = strndup(data, data_len < 200 ? data_len : 200);
    if (debug_data) {
        log_debug("Extracting topic from message: %s%s", debug_data, data_len > 200 ? "..." : "");
        free(debug_data);
    }
    
    // Try to find topic in the message
    // Format is expected to be JSON with a "topic" field
    // Example: {"topic":"recordings/batch-delete","type":"request","payload":{...}}
    
    // Simple JSON parsing to extract topic
    const char *topic_start = strstr(data, "\"topic\"");
    if (topic_start) {
        topic_start = strchr(topic_start + 7, '"');
        if (topic_start) {
            topic_start++; // Skip the opening quote
            const char *topic_end = strchr(topic_start, '"');
            if (topic_end) {
                size_t topic_len = topic_end - topic_start;
                if (topic_len < topic_size - 1) {
                    memcpy(topic, topic_start, topic_len);
                    topic[topic_len] = '\0';
                    log_debug("Successfully extracted topic: %s", topic);
                    return 0;
                }
            }
        }
    }
    
    // If we couldn't extract the topic, check for alternative format
    // Some clients might send the topic as part of the URL or in a different format
    
    // Check for system/logs topic pattern
    if (strstr(data, "system/logs") || strstr(data, "\"level\"") || strstr(data, "\"last_timestamp\"")) {
        strncpy(topic, "system/logs", topic_size - 1);
        topic[topic_size - 1] = '\0';
        log_debug("Detected system/logs topic from message content");
        return 0;
    }
    
    // Check for recordings/batch-delete topic pattern
    if (strstr(data, "recordings/batch-delete") || strstr(data, "\"recording_ids\"")) {
        strncpy(topic, "recordings/batch-delete", topic_size - 1);
        topic[topic_size - 1] = '\0';
        log_debug("Detected recordings/batch-delete topic from message content");
        return 0;
    }
    
    log_warn("Could not extract topic from message, using default");
    return 0;
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
    
    // Initialize handlers
    pthread_mutex_lock(&s_handlers_mutex);
    memset(s_handlers, 0, sizeof(s_handlers));
    s_handler_count = 0;
    pthread_mutex_unlock(&s_handlers_mutex);
    
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
    
    // Clear handlers
    pthread_mutex_lock(&s_handlers_mutex);
    memset(s_handlers, 0, sizeof(s_handlers));
    s_handler_count = 0;
    pthread_mutex_unlock(&s_handlers_mutex);
    
    log_info("WebSocket manager shutdown complete");
    
    pthread_mutex_unlock(&s_init_mutex);
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
 * @brief Register a WebSocket handler for a specific topic
 * 
 * @param topic Topic name
 * @param handler Handler function
 * @param user_data User data (optional)
 * @return int 0 on success, non-zero on error
 */
int websocket_manager_register_handler(const char *topic, websocket_handler_t handler, void *user_data) {
    // Check if WebSocket manager is initialized
    if (!websocket_manager_is_initialized()) {
        log_error("WebSocket manager not initialized");
        return -1;
    }
    
    if (!topic || !handler) {
        log_error("Invalid parameters for register_handler");
        return -1;
    }
    
    // Lock handlers mutex
    pthread_mutex_lock(&s_handlers_mutex);
    
    // Check if handler already exists
    int handler_idx = find_handler_by_topic(topic);
    if (handler_idx >= 0) {
        // Update existing handler
        s_handlers[handler_idx].handler = handler;
        s_handlers[handler_idx].user_data = user_data;
        pthread_mutex_unlock(&s_handlers_mutex);
        log_info("Updated WebSocket handler for topic: %s", topic);
        return 0;
    }
    
    // Find free slot
    int slot = find_free_handler_slot();
    if (slot < 0) {
        log_error("No free slots in WebSocket handlers array");
        pthread_mutex_unlock(&s_handlers_mutex);
        return -1;
    }
    
    // Add handler
    strncpy(s_handlers[slot].topic, topic, sizeof(s_handlers[slot].topic) - 1);
    s_handlers[slot].topic[sizeof(s_handlers[slot].topic) - 1] = '\0';
    s_handlers[slot].handler = handler;
    s_handlers[slot].user_data = user_data;
    s_handlers[slot].active = true;
    
    s_handler_count++;
    
    pthread_mutex_unlock(&s_handlers_mutex);
    
    log_info("Registered WebSocket handler for topic: %s", topic);
    
    return 0;
}

/**
 * @brief Handle WebSocket connection open
 * 
 * @param c Mongoose connection
 */
void websocket_manager_handle_open(struct mg_connection *c) {
    // Check if WebSocket manager is initialized
    if (!websocket_manager_is_initialized()) {
        log_error("WebSocket manager not initialized");
        return;
    }
    
    if (!c) {
        log_error("Invalid mongoose connection");
        return;
    }
    
    // Mark this connection as a WebSocket client
    c->data[0] = 'W';
    
    log_info("WebSocket connection opened");
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
        return;
    }
    
    if (!c || !data) {
        log_error("Invalid parameters for WebSocket message");
        return;
    }
    
    // Extract topic from message
    char topic[MAX_TOPIC_LENGTH];
    if (extract_topic_from_message(data, data_len, topic, sizeof(topic)) != 0) {
        log_warn("Failed to extract topic from WebSocket message");
        // Continue with default topic
    }
    
    // Find handler for topic
    pthread_mutex_lock(&s_handlers_mutex);
    int handler_idx = find_handler_by_topic(topic);
    if (handler_idx >= 0) {
        // Call handler
        websocket_handler_t handler = s_handlers[handler_idx].handler;
        void *user_data = s_handlers[handler_idx].user_data;
        pthread_mutex_unlock(&s_handlers_mutex);
        
        // Call handler outside of mutex
        handler(c, data, data_len, user_data);
    } else {
        pthread_mutex_unlock(&s_handlers_mutex);
        log_warn("No handler found for topic: %s", topic);
    }
}

/**
 * @brief Handle WebSocket connection close
 * 
 * @param c Mongoose connection
 */
void websocket_manager_handle_close(struct mg_connection *c) {
    // Check if WebSocket manager is initialized
    if (!websocket_manager_is_initialized()) {
        log_error("WebSocket manager not initialized");
        return;
    }
    
    if (!c) {
        log_error("Invalid mongoose connection");
        return;
    }
    
    log_info("WebSocket connection closed");
}

/**
 * @brief Broadcast a message to all WebSocket clients
 * 
 * @param mgr Mongoose manager
 * @param data Message data
 * @param data_len Message data length
 * @return int Number of clients the message was sent to
 */
int websocket_manager_broadcast(struct mg_mgr *mgr, const char *data, size_t data_len) {
    if (!mgr || !data) {
        log_error("Invalid parameters for WebSocket broadcast");
        return 0;
    }
    
    int count = 0;
    
    // Traverse all connections and send to WebSocket clients
    for (struct mg_connection *c = mgr->conns; c != NULL; c = c->next) {
        // Check if this is a WebSocket client
        if (c->data[0] == 'W') {
            // Send message
            mg_ws_send(c, data, data_len, WEBSOCKET_OP_TEXT);
            count++;
        }
    }
    
    return count;
}
