#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "web/mongoose_server.h"
#include "web/mongoose_server_websocket.h"
#include "web/websocket_manager.h"
#include "web/api_handlers_recordings_batch_ws.h"
#include "web/api_handlers_system_ws.h"
#include "web/register_websocket_handlers.h"
#include "core/logger.h"
#include "mongoose.h"

/**
 * @brief Handle WebSocket upgrade request
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_websocket_upgrade(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling WebSocket upgrade request");
    
    // Make sure WebSocket manager is initialized
    if (!websocket_manager_is_initialized()) {
        log_error("WebSocket manager not initialized");
        // Try to initialize it
        if (websocket_manager_init() != 0) {
            log_error("Failed to initialize WebSocket manager");
            mg_http_reply(c, 500, "", "{\"error\":\"WebSocket manager not initialized\"}");
            return;
        }
        log_info("WebSocket manager initialized on demand");
        
        // Register WebSocket handlers
        websocket_register_handlers();
    }
    
    // Extract URI
    char uri[256];
    size_t uri_len = hm->uri.len < sizeof(uri) - 1 ? hm->uri.len : sizeof(uri) - 1;
    memcpy(uri, hm->uri.buf, uri_len);
    uri[uri_len] = '\0';
    
    // Extract query parameters
    char query[256] = {0};
    if (hm->query.len > 0) {
        size_t query_len = hm->query.len < sizeof(query) - 1 ? hm->query.len : sizeof(query) - 1;
        memcpy(query, hm->query.buf, query_len);
        query[query_len] = '\0';
    }
    
    // Check if topic is specified
    char topic[64] = {0};
    if (strstr(query, "topic=") != NULL) {
        const char *topic_start = strstr(query, "topic=") + 6;
        const char *topic_end = strchr(topic_start, '&');
        if (topic_end == NULL) {
            topic_end = topic_start + strlen(topic_start);
        }
        
        size_t topic_len = topic_end - topic_start;
        if (topic_len < sizeof(topic) - 1) {
            memcpy(topic, topic_start, topic_len);
            topic[topic_len] = '\0';
        }
    }
    
    // If no topic specified, use default
    if (topic[0] == '\0') {
        strcpy(topic, "default");
    }
    
    log_info("WebSocket upgrade request for topic: %s", topic);
    
    // Check for auth header or cookie
    struct mg_str *auth_header = mg_http_get_header(hm, "Authorization");
    if (auth_header == NULL) {
        log_info("No Authorization header found for URI: %s, checking for cookie", uri);
        
        // Check for auth cookie
        struct mg_str *cookie_header = mg_http_get_header(hm, "Cookie");
        if (cookie_header == NULL) {
            log_info("No Cookie header found");
            
            // For compatibility with older systems, proceed with WebSocket upgrade
            // even without authentication - we'll handle auth at the application level
            log_info("Proceeding with WebSocket upgrade without authentication for compatibility");
        } else {
            log_info("Cookie header found, proceeding with WebSocket upgrade");
        }
    } else {
        log_info("Authorization header found, proceeding with WebSocket upgrade");
    }
    
    // Set a smaller buffer size for WebSocket frames to improve compatibility with older systems
    c->recv.size = 4096;  // Use a smaller receive buffer (default is usually 8192)
    
    // Register this connection with the shutdown coordinator
    // This ensures the connection will be properly tracked during shutdown
    char conn_name[64];
    snprintf(conn_name, sizeof(conn_name), "websocket_%p", (void*)c);
    
    // Include the shutdown coordinator header
    #include "core/shutdown_coordinator.h"
    
    // Only register if shutdown coordinator is initialized
    if (get_shutdown_coordinator() != NULL) {
        // Register as a server component with high priority (5)
        // Higher priority components are stopped first during shutdown
        int component_id = register_component(conn_name, COMPONENT_SERVER_THREAD, c, 5);
        if (component_id >= 0) {
            // Store the component ID in a custom field in the connection's user data
            // We can't modify the connection struct directly, but we can use the existing fn_data
            // which is already set to the server instance
            log_debug("Registered WebSocket connection %s with shutdown coordinator, ID: %d", 
                     conn_name, component_id);
        } else {
            log_warn("Failed to register WebSocket connection with shutdown coordinator");
        }
    }
    
    // Upgrade connection to WebSocket
    // The fn_data is already set in mongoose_server.c when the connection is created
    mg_ws_upgrade(c, hm, NULL);
    
    // Log upgrade success
    log_info("WebSocket connection upgraded successfully");
    
    // Note: The websocket_manager_handle_open(c) will be called when the MG_EV_WS_OPEN event is triggered
}

/**
 * @brief Register WebSocket handlers
 */
void websocket_register_handlers(void) {
    // Call our new function to register all WebSocket handlers
    register_websocket_handlers();
}

/**
 * @brief Forward declaration for batch delete recordings HTTP handler with WebSocket support
 */
extern void mg_handle_batch_delete_recordings_ws(struct mg_connection *c, struct mg_http_message *hm);
