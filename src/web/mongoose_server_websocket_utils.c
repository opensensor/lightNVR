#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "web/websocket_bridge.h"
#include "web/websocket_client.h"
#include "core/logger.h"
#include "mongoose.h"

/**
 * @brief Create a WebSocket message
 * 
 * @param type Message type
 * @param topic Message topic
 * @param payload Message payload
 * @return char* Allocated message string, must be freed with mg_websocket_message_free
 */
char *mg_websocket_message_create(const char *type, const char *topic, const char *payload) {
    if (!type || !topic || !payload) {
        log_error("Invalid parameters for mg_websocket_message_create");
        return NULL;
    }
    
    // Allocate memory for the message
    char *message = NULL;
    int len = asprintf(&message, "{\"type\":\"%s\",\"topic\":\"%s\",\"payload\":%s}", 
                      type, topic, payload);
    
    if (len < 0 || !message) {
        log_error("Failed to allocate memory for WebSocket message");
        return NULL;
    }
    
    return message;
}

/**
 * @brief Send a WebSocket message to a client
 * 
 * @param client_id Client ID
 * @param message Message to send
 * @return bool true on success, false on error
 */
bool mg_websocket_message_send_to_client(const char *client_id, const char *message) {
    if (!client_id || !message) {
        log_error("Invalid parameters for mg_websocket_message_send_to_client");
        return false;
    }
    
    // Get client connection directly from pointer value stored in client_id string
    struct mg_connection *conn = NULL;
    if (sscanf(client_id, "%p", &conn) != 1 || !conn) {
        log_error("Invalid client ID or connection not found: %s", client_id);
        return false;
    }
    
    // Send message directly using mongoose
    size_t len = strlen(message);
    mg_ws_send(conn, message, len, WEBSOCKET_OP_TEXT);
    
    return true;
}

/**
 * @brief Free a WebSocket message
 * 
 * @param message Message to free
 */
void mg_websocket_message_free(char *message) {
    free(message);
}
