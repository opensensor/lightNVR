#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>

#include "web/websocket_handler.h"
#include "web/websocket_manager.h"
#include "core/logger.h"

// Maximum number of handlers
#define MAX_HANDLERS 20

// Handler structure
typedef struct {
    char topic[64];                                     // Topic to handle
    void (*handler)(const char *client_id, const char *message);  // Handler function
    bool active;                                        // Whether the handler is active
} websocket_handler_t;

// Global state
static websocket_handler_t s_handlers[MAX_HANDLERS];
static pthread_mutex_t s_mutex;

// Forward declarations
extern bool websocket_manager_is_initialized(void);
extern int websocket_manager_init(void);

/**
 * @brief Find a handler by topic
 * 
 * @param topic Topic to find
 * @return int Handler index or -1 if not found
 */
int websocket_handler_find_by_topic(const char *topic) {
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
 * @brief Register a WebSocket message handler
 * 
 * @param topic Topic to handle
 * @param handler Handler function
 * @return int 0 on success, non-zero on error
 */
int websocket_handler_register(const char *topic, void (*handler)(const char *client_id, const char *message)) {
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
        log_error("Invalid parameters for websocket_handler_register");
        return -1;
    }
    
    pthread_mutex_lock(&s_mutex);
    
    // Check if handler already exists
    int handler_index = websocket_handler_find_by_topic(topic);
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
    
    pthread_mutex_lock(&s_mutex);
    int handler_index = websocket_handler_find_by_topic(topic);
    
    if (handler_index < 0 || !s_handlers[handler_index].handler) {
        pthread_mutex_unlock(&s_mutex);
        log_warn("No handler registered for topic %s", topic);
        return false;
    }
    
    // Get a copy of the handler function pointer
    void (*handler_func)(const char *, const char *) = s_handlers[handler_index].handler;
    pthread_mutex_unlock(&s_mutex);
    
    // Call handler without holding the mutex
    log_info("Calling handler for topic %s", topic);
    handler_func(client_id, message);
    
    return true;
}

// Functions for websocket_manager.c to use

/**
 * @brief Initialize handler array
 */
void websocket_handler_init(void) {
    memset(s_handlers, 0, sizeof(s_handlers));
}

/**
 * @brief Get mutex for handler operations
 * 
 * @return pthread_mutex_t* Mutex
 */
pthread_mutex_t* websocket_handler_get_mutex(void) {
    return &s_mutex;
}
