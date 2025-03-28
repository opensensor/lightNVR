#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>

#include "web/websocket_message.h"
#include "web/websocket_client.h"
#include "web/websocket_manager.h"
#include "core/logger.h"
#include "../external/cjson/cJSON.h"

// Forward declarations
extern bool websocket_manager_is_initialized(void);
extern int websocket_manager_init(void);

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
bool websocket_message_send_to_client(const char *client_id, const websocket_message_t *message) {
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
        log_error("Invalid parameters for websocket_message_send_to_client");
        return false;
    }
    
    // Get client connection
    struct mg_connection *conn = websocket_client_get_connection(client_id);
    if (!conn) {
        log_error("Client %s has no connection", client_id);
        return false;
    }
    
    // Update client activity timestamp
    websocket_client_update_activity(client_id);
    
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
    
    // Send message
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
int websocket_message_broadcast(const char *topic, const websocket_message_t *message) {
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
        log_error("Invalid parameters for websocket_message_broadcast");
        return 0;
    }
    
    // Get all clients subscribed to the topic
    char **client_ids = NULL;
    int client_count = websocket_client_get_subscribed(topic, &client_ids);
    
    if (client_count == 0 || !client_ids) {
        log_info("No clients subscribed to topic %s", topic);
        return 0;
    }
    
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
        
        // Free client IDs
        for (int i = 0; i < client_count; i++) {
            free(client_ids[i]);
        }
        free(client_ids);
        
        return 0;
    }
    
    // Send message to all clients
    int success_count = 0;
    for (int i = 0; i < client_count; i++) {
        struct mg_connection *conn = websocket_client_get_connection(client_ids[i]);
        if (conn && !conn->is_closing) {
            int result = mg_ws_send(conn, json_str, strlen(json_str), WEBSOCKET_OP_TEXT);
            if (result > 0) {
                success_count++;
                log_debug("Broadcast message sent to client %s", client_ids[i]);
                
                // Update client activity timestamp
                websocket_client_update_activity(client_ids[i]);
            } else {
                log_error("Failed to send broadcast message to client %s", client_ids[i]);
            }
        }
    }
    
    // Clean up
    free(json_str);
    cJSON_Delete(json);
    
    // Free client IDs
    for (int i = 0; i < client_count; i++) {
        free(client_ids[i]);
    }
    free(client_ids);
    
    return success_count;
}
