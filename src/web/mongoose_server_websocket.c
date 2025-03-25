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
