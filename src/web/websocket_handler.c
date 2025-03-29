#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#include "web/websocket_handler.h"
#include "web/websocket_client.h"
#include "web/websocket_manager.h"
#include "core/logger.h"
#include "mongoose.h"

// Maximum number of WebSocket handlers
#define MAX_WS_HANDLERS 32

// Maximum topic name length
#define MAX_TOPIC_LENGTH 64

// WebSocket handler structure
typedef struct {
    char topic[MAX_TOPIC_LENGTH];
    void (*handler)(const char *client_id, const char *message);
    bool active;
} ws_handler_t;

// Global state
static ws_handler_t s_handlers[MAX_WS_HANDLERS];
static int s_handler_count = 0;

/**
 * @brief Register a WebSocket message handler
 * 
 * @param topic Topic to handle
 * @param handler Handler function
 * @return int 0 on success, non-zero on error
 */
int websocket_handler_register(const char *topic, void (*handler)(const char *client_id, const char *message)) {
    if (!topic || !handler) {
        log_error("Invalid parameters for websocket_handler_register");
        return -1;
    }
    
    // Initialize WebSocket subsystem if not already initialized
    extern void websocket_init(void);
    if (!websocket_manager_is_initialized()) {
        log_info("Initializing WebSocket subsystem from handler registration");
        websocket_init();
    }
    
    // Check if handler already exists
    int index = websocket_handler_find_by_topic(topic);
    if (index >= 0) {
        // Update existing handler
        s_handlers[index].handler = handler;
        log_info("Updated WebSocket handler for topic: %s", topic);
        return 0;
    }
    
    // Find free slot
    int free_index = -1;
    for (int i = 0; i < MAX_WS_HANDLERS; i++) {
        if (!s_handlers[i].active) {
            free_index = i;
            break;
        }
    }
    
    if (free_index < 0) {
        log_error("Maximum number of WebSocket handlers reached");
        return -1;
    }
    
    // Add handler
    strncpy(s_handlers[free_index].topic, topic, MAX_TOPIC_LENGTH - 1);
    s_handlers[free_index].topic[MAX_TOPIC_LENGTH - 1] = '\0';
    s_handlers[free_index].handler = handler;
    s_handlers[free_index].active = true;
    
    s_handler_count++;
    
    log_info("Registered WebSocket handler for topic: %s", topic);
    
    return 0;
}

/**
 * @brief Find a handler by topic
 * 
 * @param topic Topic to find
 * @return int Handler index or -1 if not found
 */
int websocket_handler_find_by_topic(const char *topic) {
    if (!topic) {
        return -1;
    }
    
    for (int i = 0; i < MAX_WS_HANDLERS; i++) {
        if (s_handlers[i].active && strcmp(s_handlers[i].topic, topic) == 0) {
            return i;
        }
    }
    
    return -1;
}

/**
 * @brief Call a handler for a topic
 * 
 * @param topic Topic to handle
 * @param client_id Client ID
 * @param message Message to handle
 * @return bool true if handler was found and called, false otherwise
 */
bool websocket_handler_call(const char *topic, const char *client_id, const char *message) {
    if (!topic || !client_id || !message) {
        log_error("Invalid parameters for websocket_handler_call");
        return false;
    }
    
    log_debug("Calling handler for topic: %s, client: %s", topic, client_id);
    
    // Initialize WebSocket subsystem if not already initialized
    extern void websocket_init(void);
    if (!websocket_manager_is_initialized()) {
        log_info("Initializing WebSocket subsystem before calling handler");
        websocket_init();
    }
    
    // Make sure handlers are registered
    extern void websocket_register_handlers(void);
    websocket_register_handlers();
    
    int index = websocket_handler_find_by_topic(topic);
    if (index < 0) {
        log_warn("No handler found for topic: %s", topic);
        return false;
    }
    
    // Call handler
    log_debug("Handler found for topic: %s, calling handler", topic);
    s_handlers[index].handler(client_id, message);
    
    return true;
}
