#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>


#include "web/mongoose_server_websocket.h"
#include "web/websocket_manager.h"
#include "web/websocket_client.h"
#include "web/websocket_handler.h"
#include "core/logger.h"
#include "core/shutdown_coordinator.h"
#include "mongoose.h"

// Maximum topic name length
#define MAX_TOPIC_LENGTH 64


/**
 * @brief Initialize WebSocket subsystem
 */
void websocket_init(void) {
    // Initialize the WebSocket manager
    if (!websocket_manager_is_initialized()) {
        if (websocket_manager_init() != 0) {
            log_error("Failed to initialize WebSocket manager");
            return;
        }
    }
    
    log_info("WebSocket subsystem initialized");
}

/**
 * @brief Shutdown WebSocket subsystem
 */
void websocket_shutdown(void) {
    // Shutdown the WebSocket manager
    websocket_manager_shutdown();
    
    log_info("WebSocket subsystem shutdown");
}

/**
 * @brief Handle WebSocket upgrade request
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_websocket_upgrade(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling WebSocket upgrade request");
    
    // Initialize WebSocket subsystem if not already initialized
    if (!websocket_manager_is_initialized()) {
        log_info("Initializing WebSocket subsystem");
        websocket_init();
    }
    
    // Check for client_id in query parameters
    char client_id[64] = {0};
    
    // Check for client_id in query string
    if (hm->query.len > 0) {
        // Extract client_id from query string
        char query_buf[256] = {0};
        size_t query_len = hm->query.len < sizeof(query_buf) - 1 ? hm->query.len : sizeof(query_buf) - 1;
        memcpy(query_buf, hm->query.buf, query_len);
        query_buf[query_len] = '\0';
        
        // Parse query string manually
        char *param = strstr(query_buf, "client_id=");
        if (param) {
            param += 10; // Skip "client_id="
            char *end = strchr(param, '&');
            if (end) *end = '\0'; // Terminate at next parameter
            
            // Copy client_id
            strncpy(client_id, param, sizeof(client_id) - 1);
            client_id[sizeof(client_id) - 1] = '\0';
            log_info("Found client ID in query string: %s", client_id);
        }
    }
    
    // Check for auth cookie and log it (we can't store it in c->data as it's only 32 bytes)
    struct mg_str *cookie_header = mg_http_get_header(hm, "Cookie");
    if (cookie_header && cookie_header->len > 0) {
        log_info("Found Cookie header in WebSocket upgrade request");
        
        // Extract auth cookie
        char cookie_buf[1024] = {0};
        size_t cookie_len = cookie_header->len < sizeof(cookie_buf) - 1 ? cookie_header->len : sizeof(cookie_buf) - 1;
        memcpy(cookie_buf, cookie_header->buf, cookie_len);
        cookie_buf[cookie_len] = '\0';
        
        // Look for auth cookie
        char *auth_cookie = strstr(cookie_buf, "auth=");
        if (auth_cookie) {
            log_info("Found auth cookie in WebSocket upgrade request");
            
            // Skip "auth=" prefix
            auth_cookie += 5;
            
            // Find end of cookie value (semicolon or end of string)
            char *end = strchr(auth_cookie, ';');
            if (end) *end = '\0';
            
            // Log the auth cookie
            log_info("Auth cookie value: %s", auth_cookie);
            
            // We can't store the auth cookie in c->data as it's only 32 bytes
            // Instead, we'll set a flag in c->data to indicate that this connection has an auth cookie
            // c->data[0] is already used to mark this as a WebSocket client
            // We'll use c->data[1] to indicate that this connection has an auth cookie
            c->data[1] = 'A';  // 'A' for Auth
            
            // The actual auth cookie will be handled by the server's authentication system
        } else {
            log_info("No auth cookie found in WebSocket upgrade request");
        }
    }
    
    // Upgrade the connection to WebSocket
    mg_ws_upgrade(c, hm, NULL);
    
    // If no client ID was found, generate one based on connection pointer
    if (client_id[0] == '\0') {
        snprintf(client_id, sizeof(client_id), "%p", (void*)c);
        log_info("Generated client ID based on connection pointer: %s", client_id);
    }
    
    // Mark this connection as a WebSocket client
    c->data[0] = 'W';
    
    // Store client ID in connection data
    // We can use the remaining bytes in c->data to store the client ID
    // c->data[0] is already used to mark this as a WebSocket client
    size_t id_len = strlen(client_id);
    if (id_len < sizeof(c->data) - 1) {
        memcpy(&c->data[1], client_id, id_len);
        c->data[id_len + 1] = '\0';
        log_debug("Stored client ID in connection data: %s", client_id);
    } else {
        log_warn("Client ID too long to store in connection data: %s", client_id);
    }
    
    // Handle WebSocket open event
    websocket_manager_handle_open(c);
    
    // Send welcome message with client ID
    char welcome_message[256];
    snprintf(welcome_message, sizeof(welcome_message), 
             "{\"type\":\"welcome\",\"topic\":\"system\",\"payload\":{\"client_id\":\"%s\"}}", 
             client_id);
    
    mg_ws_send(c, welcome_message, strlen(welcome_message), WEBSOCKET_OP_TEXT);
    
    log_info("WebSocket connection upgraded, client ID: %s", client_id);
}

/**
 * @brief Handle WebSocket message
 * 
 * @param c Mongoose connection
 * @param wm Mongoose WebSocket message
 */
void mg_handle_websocket_message(struct mg_connection *c, struct mg_ws_message *wm) {
    if (!c || !wm) {
        log_error("Invalid parameters for mg_handle_websocket_message");
        return;
    }
    
    // Extract message data
    char *data = malloc(wm->data.len + 1);
    if (!data) {
        log_error("Failed to allocate memory for WebSocket message data");
        return;
    }
    
    memcpy(data, wm->data.buf, wm->data.len);
    data[wm->data.len] = '\0';
    
    // Generate client ID based on connection pointer
    char client_id[32];
    snprintf(client_id, sizeof(client_id), "%p", (void*)c);
    
    // Log the message for debugging
    log_debug("WebSocket message received from client %s: %s", client_id, data);
    
    // Extract topic from message
    char topic[MAX_TOPIC_LENGTH];
    char *topic_start = strstr(data, "\"topic\"");
    if (topic_start) {
        topic_start = strchr(topic_start + 7, '"');
        if (topic_start) {
            topic_start++; // Skip the opening quote
            char *topic_end = strchr(topic_start, '"');
            if (topic_end) {
                size_t topic_len = topic_end - topic_start;
                if (topic_len < sizeof(topic) - 1) {
                    memcpy(topic, topic_start, topic_len);
                    topic[topic_len] = '\0';
                    
                    // Call the handler directly
                    log_debug("Calling handler for topic: %s", topic);
                    if (websocket_handler_call(topic, client_id, data)) {
                        // Handler was called successfully
                        free(data);
                        return;
                    }
                }
            }
        }
    }
    
    // If we couldn't extract the topic or find a handler, fall back to the manager
    log_debug("Falling back to websocket_manager_handle_message");
    websocket_manager_handle_message(c, data, wm->data.len);
    
    // Free the allocated memory
    free(data);
}

/**
 * @brief Handle WebSocket close
 * 
 * @param c Mongoose connection
 */
void mg_handle_websocket_close(struct mg_connection *c) {
    if (!c) {
        log_error("Invalid connection in mg_handle_websocket_close");
        return;
    }
    
    // Generate client ID based on connection pointer
    char client_id[32];
    snprintf(client_id, sizeof(client_id), "%p", (void*)c);
    
    log_info("WebSocket connection closed, client ID: %s", client_id);
    
    // Remove client from WebSocket manager
    websocket_client_remove_by_connection(c);
    
    // Handle WebSocket close event
    websocket_manager_handle_close(c);
}
