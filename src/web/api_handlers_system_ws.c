#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <pthread.h>

#include "web/api_handlers_system_ws.h"
#include "web/websocket_bridge.h"
#include "web/api_handlers_system_ws.h"
#include "core/logger.h"
#include "cJSON.h"


// Map to store client log level preferences
// Key: client_id, Value: log level string (error, warning, info, debug)
typedef struct {
    char client_id[48]; // Reduced from 64 to save stack space
    char log_level[12]; // Reduced from 16 to save stack space
} client_log_level_t;

#define MAX_CLIENTS 100
static client_log_level_t client_log_levels[MAX_CLIENTS];
static int client_log_level_count = 0;

/**
 * @brief Set log level for a client
 *
 * @param client_id WebSocket client ID
 * @param log_level Log level string (error, warning, info, debug)
 */
static void set_client_log_level(const char *client_id, const char *log_level) {
    // Check if client already exists
    for (int i = 0; i < client_log_level_count; i++) {
        if (strcmp(client_log_levels[i].client_id, client_id) == 0) {
            // Update existing client
            strncpy(client_log_levels[i].log_level, log_level, sizeof(client_log_levels[i].log_level) - 1);
            client_log_levels[i].log_level[sizeof(client_log_levels[i].log_level) - 1] = '\0';
            log_debug("Updated log level for client %s: %s", client_id, log_level);
            return;
        }
    }

    // Add new client if space available
    if (client_log_level_count < MAX_CLIENTS) {
        strncpy(client_log_levels[client_log_level_count].client_id, client_id, sizeof(client_log_levels[client_log_level_count].client_id) - 1);
        client_log_levels[client_log_level_count].client_id[sizeof(client_log_levels[client_log_level_count].client_id) - 1] = '\0';

        strncpy(client_log_levels[client_log_level_count].log_level, log_level, sizeof(client_log_levels[client_log_level_count].log_level) - 1);
        client_log_levels[client_log_level_count].log_level[sizeof(client_log_levels[client_log_level_count].log_level) - 1] = '\0';

        client_log_level_count++;
        log_debug("Added log level for client %s: %s", client_id, log_level);
    } else {
        log_error("Maximum number of clients reached, cannot add log level for client %s", client_id);
    }
}

/**
 * @brief Get log level for a client
 *
 * @param client_id WebSocket client ID
 * @return const char* Log level string, or "info" if not found
 */
static const char *get_client_log_level(const char *client_id) {
    for (int i = 0; i < client_log_level_count; i++) {
        if (strcmp(client_log_levels[i].client_id, client_id) == 0) {
            return client_log_levels[i].log_level;
        }
    }

    // Default to info if not found
    return "info";
}

/**
 * @brief Remove log level for a client
 *
 * @param client_id WebSocket client ID
 */
void remove_client_log_level(const char *client_id) {
    for (int i = 0; i < client_log_level_count; i++) {
        if (strcmp(client_log_levels[i].client_id, client_id) == 0) {
            // Move last client to this position
            if (i < client_log_level_count - 1) {
                strncpy(client_log_levels[i].client_id, client_log_levels[client_log_level_count - 1].client_id, sizeof(client_log_levels[i].client_id));
                strncpy(client_log_levels[i].log_level, client_log_levels[client_log_level_count - 1].log_level, sizeof(client_log_levels[i].log_level));
            }

            client_log_level_count--;
            log_debug("Removed log level for client %s", client_id);
            return;
        }
    }
}

/**
 * @brief WebSocket handler for system logs
 *
 * @param client_id WebSocket client ID
 * @param message WebSocket message
 */
void websocket_handle_system_logs(const char *client_id, const char *message) {
    log_debug("Handling WebSocket message for system logs from client %s: %s", client_id, message);

    // Log the raw message for debugging
    log_debug("Raw WebSocket message: %s", message);

    // Parse message JSON
    cJSON *json = cJSON_Parse(message);
    if (json == NULL) {
        log_error("Failed to parse WebSocket message JSON: %s", message);

        // Create error message
        char error_message[256];
        snprintf(error_message, sizeof(error_message),
                "{\"type\":\"error\",\"topic\":\"system/logs\",\"payload\":{\"error\":\"Invalid JSON message\"}}");

        // Get client connection directly from pointer value stored in client_id string
        struct mg_connection *conn = NULL;
        if (sscanf(client_id, "%p", &conn) == 1 && conn) {
            // Send message directly using mongoose
            mg_ws_send(conn, error_message, strlen(error_message), WEBSOCKET_OP_TEXT);
        } else {
            log_error("Invalid client ID or connection not found: %s", client_id);
        }

        return;
    }

    // Check if this is a direct payload or a full message with type/topic/payload
    cJSON *type_obj = cJSON_GetObjectItem(json, "type");
    const char *type = NULL;
    cJSON *payload_obj = NULL;

    if (type_obj && cJSON_IsString(type_obj)) {
        // This is a full message with type/topic/payload
        type = type_obj->valuestring;
        payload_obj = cJSON_GetObjectItem(json, "payload");

        if (payload_obj && cJSON_IsObject(payload_obj)) {
            // Use the payload object for further processing
            log_debug("Found full message with type=%s and payload", type);
        } else {
            // No payload object, treat the whole message as the payload
            payload_obj = json;
            log_debug("No payload object found, treating whole message as payload");
        }
    } else {
        // This is a direct payload without type/topic
        // Determine the type based on the content
        log_debug("Message appears to be a direct payload without type field");

        // Check for common fields to determine the type
        if (cJSON_GetObjectItem(json, "level") != NULL) {
            type = "fetch";
            log_debug("Detected message type as 'fetch' based on payload content");
            payload_obj = json;
        } else if (cJSON_GetObjectItem(json, "client_id") != NULL) {
            type = "subscribe";
            log_debug("Detected message type as 'subscribe' based on payload content");
            payload_obj = json;
        } else {
            // Default to subscribe if we can't determine
            type = "subscribe";
            log_debug("Could not determine message type, defaulting to 'subscribe'");
            payload_obj = json;
        }
    }

    // If we still don't have a type, report an error
    if (!type) {
        log_error("WebSocket message missing type field and could not be determined: %s", message);

        // Create error message
        char error_message[256];
        snprintf(error_message, sizeof(error_message),
                "{\"type\":\"error\",\"topic\":\"system/logs\",\"payload\":{\"error\":\"Message missing type field\"}}");

        // Get client connection directly from pointer value stored in client_id string
        struct mg_connection *conn = NULL;
        if (sscanf(client_id, "%p", &conn) == 1 && conn) {
            // Send message directly using mongoose
            mg_ws_send(conn, error_message, strlen(error_message), WEBSOCKET_OP_TEXT);
        } else {
            log_error("Invalid client ID or connection not found: %s", client_id);
        }

        cJSON_Delete(json);
        return;
    }

    // Handle subscribe message
    if (strcmp(type, "subscribe") == 0) {
        // Extract log level from subscription parameters
        const char *log_level = "info"; // Default to info

        cJSON *params_obj = cJSON_GetObjectItem(json, "params");
        if (params_obj && cJSON_IsObject(params_obj)) {
            cJSON *level_obj = cJSON_GetObjectItem(params_obj, "level");
            if (level_obj && cJSON_IsString(level_obj)) {
                log_level = level_obj->valuestring;
                log_debug("Client %s subscribed to system logs with level: %s", client_id, log_level);
            }
        }

        // Store client log level preference
        set_client_log_level(client_id, log_level);

        log_info("Client %s subscribed to system logs with level: %s", client_id, log_level);

        // Fetch logs using the optimized approach
        fetch_system_logs(client_id, log_level, NULL);
    }
    // Handle unsubscribe message
    else if (strcmp(type, "unsubscribe") == 0) {
        log_info("Client %s unsubscribed from system logs", client_id);

        // Remove client log level preference
        remove_client_log_level(client_id);
    }
    // Handle fetch message for pagination
    else if (strcmp(type, "fetch") == 0) {
        // Extract log level and last timestamp from parameters
        const char *log_level = get_client_log_level(client_id); // Use stored preference
        const char *last_timestamp = NULL;

        // First check if we have a params object
        cJSON *params_obj = cJSON_GetObjectItem(json, "params");
        if (params_obj && cJSON_IsObject(params_obj)) {
            // Check if level is specified in the fetch request
            cJSON *level_obj = cJSON_GetObjectItem(params_obj, "level");
            if (level_obj && cJSON_IsString(level_obj)) {
                log_level = level_obj->valuestring;
                // Update stored preference
                set_client_log_level(client_id, log_level);
            }

            // Get last timestamp for pagination
            cJSON *timestamp_obj = cJSON_GetObjectItem(params_obj, "last_timestamp");
            if (timestamp_obj && cJSON_IsString(timestamp_obj)) {
                last_timestamp = timestamp_obj->valuestring;
            }
        }

        // If we didn't find params or timestamp in params, check the payload object directly
        // This handles the case where the frontend sends the timestamp in the payload directly
        if (!last_timestamp) {
            // Check if payload_obj is the same as json (for direct payload)
            if (payload_obj == json) {
                cJSON *timestamp_obj = cJSON_GetObjectItem(payload_obj, "last_timestamp");
                if (timestamp_obj && cJSON_IsString(timestamp_obj)) {
                    last_timestamp = timestamp_obj->valuestring;
                    log_debug("Found last_timestamp in direct payload: %s", last_timestamp);
                }
            }
        }

        log_info("Client %s fetching logs with level: %s, last_timestamp: %s",
                client_id, log_level, last_timestamp ? last_timestamp : "NULL");

        // Fetch logs using the optimized approach
        fetch_system_logs(client_id, log_level, last_timestamp);
    }
    // Handle unknown message type
    else {
        log_error("Unknown WebSocket message type: %s", type);

        // Create error message
        char error_message[256];
        snprintf(error_message, sizeof(error_message),
                "{\"type\":\"error\",\"topic\":\"system/logs\",\"payload\":{\"error\":\"Unknown message type\"}}");

        // Get client connection directly from pointer value stored in client_id string
        struct mg_connection *conn = NULL;
        if (sscanf(client_id, "%p", &conn) == 1 && conn) {
            // Send message directly using mongoose
            mg_ws_send(conn, error_message, strlen(error_message), WEBSOCKET_OP_TEXT);
        } else {
            log_error("Invalid client ID or connection not found: %s", client_id);
        }
    }

    cJSON_Delete(json);
}

/**
 * @brief Send system logs to a specific client with filtering
 *
 * @param client_id The client ID to send logs to
 * @param min_level The minimum log level to include
 * @return int 1 if logs were sent, 0 otherwise
 */
static int send_filtered_logs_to_client(const char *client_id, const char *min_level) {
    log_info("send_filtered_logs_to_client called for client %s with level %s", client_id, min_level);

    char **logs = NULL;
    int count = 0;

    // Use the optimized tail-based log retrieval
    // Request 1000 lines to ensure we have enough after filtering
    int result = get_system_logs_tail(&logs, &count, 1000);

    if (result != 0 || logs == NULL || count == 0) {
        return 0;
    }

    log_info("Processing %d logs from tail", count);

    // Create JSON array of logs
    cJSON *logs_array = cJSON_CreateArray();

    for (int i = 0; i < count; i++) {
        if (logs[i] != NULL) {
            // Parse log line (format: [TIMESTAMP] [LEVEL] MESSAGE)
            // Use smaller buffers to reduce stack usage
            char timestamp[24] = "";
            char log_level[12] = "";
            char *message = NULL;

            // First, check if this is a standard format log
            if (logs[i][0] == '[') {
                char *timestamp_end = strchr(logs[i] + 1, ']');
                if (timestamp_end) {
                    size_t timestamp_len = timestamp_end - (logs[i] + 1);
                    if (timestamp_len < sizeof(timestamp)) {
                        memcpy(timestamp, logs[i] + 1, timestamp_len);
                        timestamp[timestamp_len] = '\0';

                        // Look for level
                        char *level_start = strchr(timestamp_end + 1, '[');
                        if (level_start) {
                            char *level_end = strchr(level_start + 1, ']');
                            if (level_end) {
                                size_t level_len = level_end - (level_start + 1);
                                if (level_len < sizeof(log_level)) {
                                    memcpy(log_level, level_start + 1, level_len);
                                    log_level[level_len] = '\0';

                                    // Convert to lowercase for consistency
                                    for (int j = 0; log_level[j]; j++) {
                                        log_level[j] = tolower(log_level[j]);
                                    }

                                    // Message starts after level
                                    message = level_end + 1;
                                    if (message && *message) {
                                        while (*message == ' ') message++; // Skip leading spaces
                                    } else {
                                        message = ""; // Set to empty string if NULL or empty
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // Check for alternative format: TIMESTAMP LEVEL MESSAGE
            else if (isdigit(logs[i][0])) {
                // Try to parse YYYY-MM-DD HH:MM:SS format
                if (strlen(logs[i]) > 19 && logs[i][4] == '-' && logs[i][7] == '-' && logs[i][10] == ' ' && logs[i][13] == ':' && logs[i][16] == ':') {
                    strncpy(timestamp, logs[i], 19);
                    timestamp[19] = '\0';

                    // Extract level - assume it follows timestamp immediately
                    char *level_start = logs[i] + 19;
                    while (*level_start == ' ') level_start++; // Skip spaces

                    // Find end of level (next space)
                    char *level_end = strchr(level_start, ' ');
                    if (level_end) {
                        size_t level_len = level_end - level_start;
                        if (level_len < sizeof(log_level)) {
                            memcpy(log_level, level_start, level_len);
                            log_level[level_len] = '\0';

                            // Convert to lowercase for consistency
                            for (int j = 0; log_level[j]; j++) {
                                log_level[j] = tolower(log_level[j]);
                            }

                            // Message starts after level
                            message = level_end + 1;
                            if (message && *message) {
                                while (*message == ' ') message++; // Skip leading spaces
                            } else {
                                message = ""; // Set to empty string if NULL or empty
                            }
                        }
                    }
                }
            }

            // If parsing failed, use defaults
            if (!message) {
                message = logs[i];

                // Check if the message starts with "Unknown"
                if (strncmp(message, "Unknown", 7) == 0) {
                    strcpy(log_level, "info");

                    // Skip "Unknown" prefix in the message
                    if (strlen(message) > 7) {
                        message += 7;
                        while (*message == ' ') message++; // Skip spaces
                    }
                } else {
                    strcpy(log_level, "info");
                }
            }

            // Normalize log level
            if (strcmp(log_level, "warn") == 0) {
                strcpy(log_level, "warning");
            }

            // Only include logs that meet the minimum level
            if (log_level_meets_minimum(log_level, min_level)) {
                // Create log entry object
                cJSON *log_entry = cJSON_CreateObject();
                if (log_entry) {
                    cJSON_AddStringToObject(log_entry, "timestamp", timestamp[0] ? timestamp : "Unknown");
                    cJSON_AddStringToObject(log_entry, "level", log_level);
                    cJSON_AddStringToObject(log_entry, "message", message);

                    cJSON_AddItemToArray(logs_array, log_entry);
                }
            }
        }
    }

    // Create JSON payload
    cJSON *payload = cJSON_CreateObject();
    cJSON_AddItemToObject(payload, "logs", logs_array);
    // Include level at the top level for frontend filtering purposes
    cJSON_AddStringToObject(payload, "level", min_level);

    // Convert payload to string
    char *payload_str = cJSON_PrintUnformatted(payload);

    // Create WebSocket message
    char *logs_message = NULL;
    int len = asprintf(&logs_message, "{\"type\":\"update\",\"topic\":\"system/logs\",\"payload\":%s}", payload_str);

    result = 0;

    if (len > 0 && logs_message != NULL) {
        // Get client connection directly from pointer value stored in client_id string
        struct mg_connection *conn = NULL;
        if (sscanf(client_id, "%p", &conn) == 1 && conn) {
            // Send message directly using mongoose
            mg_ws_send(conn, logs_message, strlen(logs_message), WEBSOCKET_OP_TEXT);
            result = 1;
        } else {
            log_error("Invalid client ID or connection not found: %s", client_id);
        }

        // Free the message
        free(logs_message);
    }

    cJSON_Delete(payload);
    free(payload_str);

    // Free logs
    if (logs) {
        for (int i = 0; i < count; i++) {
            if (logs[i]) {
                free(logs[i]);
            }
        }
        free(logs);
    }

    return result;
}

/**
 * @brief Fetch system logs with timestamp-based pagination
 *
 * @param client_id WebSocket client ID
 * @param min_level Minimum log level to include
 * @param last_timestamp Last timestamp received by client (for pagination)
 * @return int Number of logs sent
 */
int fetch_system_logs(const char *client_id, const char *min_level, const char *last_timestamp) {
    log_info("fetch_system_logs called for client %s with level %s, last_timestamp %s",
             client_id, min_level, last_timestamp ? last_timestamp : "NULL");

    // Get logs using the optimized tail-based approach if available
    char **logs = NULL;
    int count = 250;

    // Try to use the optimized tail-based JSON logs function first
    const int result = get_json_logs_tail(min_level, last_timestamp, &logs, &count);
    if (result != 0 || logs == NULL || count == 0) {
        log_error("Failed to get JSON logs using tail command");
        return 0;
    }

    // Create JSON array of logs
    cJSON *logs_array = cJSON_CreateArray();

    // Track the latest timestamp for pagination
    char latest_timestamp[32] = "";

    // Process each log entry (already in JSON format)
    for (int i = 0; i < count; i++) {
        if (logs[i] != NULL) {
            // Parse JSON log entry
            cJSON *log_entry = cJSON_Parse(logs[i]);
            if (log_entry) {
                // Add to logs array
                cJSON_AddItemToArray(logs_array, log_entry);

                // Update latest timestamp for pagination
                cJSON *timestamp = cJSON_GetObjectItem(log_entry, "timestamp");
                if (timestamp && cJSON_IsString(timestamp) && timestamp->valuestring) {
                    strncpy(latest_timestamp, timestamp->valuestring, sizeof(latest_timestamp) - 1);
                    latest_timestamp[sizeof(latest_timestamp) - 1] = '\0';
                }
            }
        }
    }

    // Create JSON payload
    cJSON *payload = cJSON_CreateObject();
    cJSON_AddItemToObject(payload, "logs", logs_array);
    // Include level at the top level for frontend filtering purposes
    cJSON_AddStringToObject(payload, "level", min_level);
    // Include latest timestamp for pagination
    if (latest_timestamp[0] != '\0') {
        cJSON_AddStringToObject(payload, "latest_timestamp", latest_timestamp);
    }
    // Indicate if there might be more logs
    cJSON_AddBoolToObject(payload, "more", count >= 50); // Assume more if we hit the limit

    // Convert payload to string
    char *payload_str = cJSON_PrintUnformatted(payload);

    // Create WebSocket message
    char *logs_message = NULL;
    int len = asprintf(&logs_message, "{\"type\":\"update\",\"topic\":\"system/logs\",\"payload\":%s}", payload_str);

    int sent = 0;

    if (len > 0 && logs_message != NULL) {
        // Get client connection directly from pointer value stored in client_id string
        struct mg_connection *conn = NULL;
        if (sscanf(client_id, "%p", &conn) == 1 && conn) {
            // Send message directly using mongoose
            mg_ws_send(conn, logs_message, strlen(logs_message), WEBSOCKET_OP_TEXT);
            sent = count;
        } else {
            log_error("Invalid client ID or connection not found: %s", client_id);
        }

        // Free the message
        free(logs_message);
    }

    cJSON_Delete(payload);
    free(payload_str);

    // Free logs
    if (logs) {
        for (int i = 0; i < count; i++) {
            if (logs[i]) {
                free(logs[i]);
            }
        }
        free(logs);
    }

    return sent ? count : 0;
}
