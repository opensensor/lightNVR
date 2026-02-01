#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <cjson/cJSON.h>

#include "web/api_handlers.h"
#include "web/api_handlers_common.h"
#include "web/mongoose_adapter.h"
#include "core/logger.h"
#include "core/config.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#include "database/db_auth.h"
#include "video/stream_manager.h"
#include "video/hls_streaming.h"
#include "mongoose.h"

/**
 * @brief Direct handler for GET /api/settings
 */
void mg_handle_get_settings(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/settings request");
    
    // Get global configuration
    // Create JSON object
    cJSON *settings = cJSON_CreateObject();
    if (!settings) {
        log_error("Failed to create settings JSON object");
        mg_send_json_error(c, 500, "Failed to create settings JSON");
        return;
    }
    
    // Add settings properties
    cJSON_AddNumberToObject(settings, "web_port", g_config.web_port);
    cJSON_AddStringToObject(settings, "web_root", g_config.web_root);
    cJSON_AddBoolToObject(settings, "web_auth_enabled", g_config.web_auth_enabled);
    cJSON_AddStringToObject(settings, "web_username", g_config.web_username);
    cJSON_AddBoolToObject(settings, "webrtc_disabled", g_config.webrtc_disabled);
    
    // Don't include the password for security reasons
    cJSON_AddStringToObject(settings, "web_password", "********");
    
    cJSON_AddStringToObject(settings, "storage_path", g_config.storage_path);
    cJSON_AddStringToObject(settings, "storage_path_hls", g_config.storage_path_hls);
    cJSON_AddNumberToObject(settings, "max_storage_size", g_config.max_storage_size);
    cJSON_AddNumberToObject(settings, "retention_days", g_config.retention_days);
    cJSON_AddBoolToObject(settings, "auto_delete_oldest", g_config.auto_delete_oldest);
    cJSON_AddNumberToObject(settings, "max_streams", g_config.max_streams);
    cJSON_AddStringToObject(settings, "log_file", g_config.log_file);
    cJSON_AddNumberToObject(settings, "log_level", g_config.log_level);
    cJSON_AddStringToObject(settings, "pid_file", g_config.pid_file);
    cJSON_AddStringToObject(settings, "db_path", g_config.db_path);
    cJSON_AddStringToObject(settings, "models_path", g_config.models_path);
    cJSON_AddNumberToObject(settings, "buffer_size", g_config.buffer_size);
    cJSON_AddBoolToObject(settings, "use_swap", g_config.use_swap);
    cJSON_AddNumberToObject(settings, "swap_size", g_config.swap_size / (1024 * 1024)); // Convert bytes to MB

    // Detection buffer defaults
    cJSON_AddNumberToObject(settings, "pre_detection_buffer", g_config.default_pre_detection_buffer);
    cJSON_AddNumberToObject(settings, "post_detection_buffer", g_config.default_post_detection_buffer);
    cJSON_AddStringToObject(settings, "buffer_strategy", g_config.default_buffer_strategy);

    // go2rtc settings (needed by frontend for WebRTC connections)
    cJSON_AddNumberToObject(settings, "go2rtc_api_port", g_config.go2rtc_api_port);

    // MQTT settings
    cJSON_AddBoolToObject(settings, "mqtt_enabled", g_config.mqtt_enabled);
    cJSON_AddStringToObject(settings, "mqtt_broker_host", g_config.mqtt_broker_host);
    cJSON_AddNumberToObject(settings, "mqtt_broker_port", g_config.mqtt_broker_port);
    cJSON_AddStringToObject(settings, "mqtt_username", g_config.mqtt_username);
    // Don't include the password for security reasons
    cJSON_AddStringToObject(settings, "mqtt_password", g_config.mqtt_password[0] ? "********" : "");
    cJSON_AddStringToObject(settings, "mqtt_client_id", g_config.mqtt_client_id);
    cJSON_AddStringToObject(settings, "mqtt_topic_prefix", g_config.mqtt_topic_prefix);
    cJSON_AddBoolToObject(settings, "mqtt_tls_enabled", g_config.mqtt_tls_enabled);
    cJSON_AddNumberToObject(settings, "mqtt_keepalive", g_config.mqtt_keepalive);
    cJSON_AddNumberToObject(settings, "mqtt_qos", g_config.mqtt_qos);
    cJSON_AddBoolToObject(settings, "mqtt_retain", g_config.mqtt_retain);

    // Convert to string
    char *json_str = cJSON_PrintUnformatted(settings);
    if (!json_str) {
        log_error("Failed to convert settings JSON to string");
        cJSON_Delete(settings);
        mg_send_json_error(c, 500, "Failed to convert settings JSON to string");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    cJSON_Delete(settings);
    
    log_info("Successfully handled GET /api/settings request");
}

/**
 * @brief Direct handler for POST /api/settings
 */
void mg_handle_post_settings(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/settings request");

    // Check if user has admin privileges to modify settings
    if (!mg_check_admin_privileges(c, hm)) {
        return;  // Error response already sent
    }

    // Parse JSON from request body
    cJSON *settings = mg_parse_json_body(hm);
    if (!settings) {
        log_error("Failed to parse settings JSON from request body");
        mg_send_json_error(c, 400, "Invalid JSON in request body");
        return;
    }

    // Update settings
    bool settings_changed = false;
    
    // Web port
    cJSON *web_port = cJSON_GetObjectItem(settings, "web_port");
    if (web_port && cJSON_IsNumber(web_port)) {
        g_config.web_port = web_port->valueint;
        settings_changed = true;
        log_info("Updated web_port: %d", g_config.web_port);
    }
    
    // Web root
    cJSON *web_root = cJSON_GetObjectItem(settings, "web_root");
    if (web_root && cJSON_IsString(web_root)) {
        strncpy(g_config.web_root, web_root->valuestring, sizeof(g_config.web_root) - 1);
        g_config.web_root[sizeof(g_config.web_root) - 1] = '\0';
        settings_changed = true;
        log_info("Updated web_root: %s", g_config.web_root);
    }
    
    // Web auth enabled
    cJSON *web_auth_enabled = cJSON_GetObjectItem(settings, "web_auth_enabled");
    if (web_auth_enabled && cJSON_IsBool(web_auth_enabled)) {
        g_config.web_auth_enabled = cJSON_IsTrue(web_auth_enabled);
        settings_changed = true;
        log_info("Updated web_auth_enabled: %s", g_config.web_auth_enabled ? "true" : "false");
    }
    
    // Web username
    cJSON *web_username = cJSON_GetObjectItem(settings, "web_username");
    if (web_username && cJSON_IsString(web_username)) {
        strncpy(g_config.web_username, web_username->valuestring, sizeof(g_config.web_username) - 1);
        g_config.web_username[sizeof(g_config.web_username) - 1] = '\0';
        settings_changed = true;
        log_info("Updated web_username: %s", g_config.web_username);
    }
    
    // Web password
    cJSON *web_password = cJSON_GetObjectItem(settings, "web_password");
    if (web_password && cJSON_IsString(web_password) && strcmp(web_password->valuestring, "********") != 0) {
        strncpy(g_config.web_password, web_password->valuestring, sizeof(g_config.web_password) - 1);
        g_config.web_password[sizeof(g_config.web_password) - 1] = '\0';
        settings_changed = true;
        log_info("Updated web_password");
    }
    
    // WebRTC disabled
    cJSON *webrtc_disabled = cJSON_GetObjectItem(settings, "webrtc_disabled");
    if (webrtc_disabled && cJSON_IsBool(webrtc_disabled)) {
        g_config.webrtc_disabled = cJSON_IsTrue(webrtc_disabled);
        settings_changed = true;
        log_info("Updated webrtc_disabled: %s", g_config.webrtc_disabled ? "true" : "false");
    }
    
    // Storage path
    cJSON *storage_path = cJSON_GetObjectItem(settings, "storage_path");
    if (storage_path && cJSON_IsString(storage_path)) {
        strncpy(g_config.storage_path, storage_path->valuestring, sizeof(g_config.storage_path) - 1);
        g_config.storage_path[sizeof(g_config.storage_path) - 1] = '\0';
        settings_changed = true;
        log_info("Updated storage_path: %s", g_config.storage_path);
    }
    
    // Storage path for HLS segments
    cJSON *storage_path_hls = cJSON_GetObjectItem(settings, "storage_path_hls");
    if (storage_path_hls && cJSON_IsString(storage_path_hls)) {
        strncpy(g_config.storage_path_hls, storage_path_hls->valuestring, sizeof(g_config.storage_path_hls) - 1);
        g_config.storage_path_hls[sizeof(g_config.storage_path_hls) - 1] = '\0';
        settings_changed = true;
        log_info("Updated storage_path_hls: %s", g_config.storage_path_hls);
    }
    
    // Max storage size
    cJSON *max_storage_size = cJSON_GetObjectItem(settings, "max_storage_size");
    if (max_storage_size && cJSON_IsNumber(max_storage_size)) {
        g_config.max_storage_size = max_storage_size->valueint;
        settings_changed = true;
        log_info("Updated max_storage_size: %d", g_config.max_storage_size);
    }
    
    // Retention days
    cJSON *retention_days = cJSON_GetObjectItem(settings, "retention_days");
    if (retention_days && cJSON_IsNumber(retention_days)) {
        g_config.retention_days = retention_days->valueint;
        settings_changed = true;
        log_info("Updated retention_days: %d", g_config.retention_days);
    }
    
    // Auto delete oldest
    cJSON *auto_delete_oldest = cJSON_GetObjectItem(settings, "auto_delete_oldest");
    if (auto_delete_oldest && cJSON_IsBool(auto_delete_oldest)) {
        g_config.auto_delete_oldest = cJSON_IsTrue(auto_delete_oldest);
        settings_changed = true;
        log_info("Updated auto_delete_oldest: %s", g_config.auto_delete_oldest ? "true" : "false");
    }
    
    // Models path
    cJSON *models_path = cJSON_GetObjectItem(settings, "models_path");
    if (models_path && cJSON_IsString(models_path)) {
        strncpy(g_config.models_path, models_path->valuestring, sizeof(g_config.models_path) - 1);
        g_config.models_path[sizeof(g_config.models_path) - 1] = '\0';
        settings_changed = true;
        log_info("Updated models_path: %s", g_config.models_path);
    }
    
    // Max streams
    cJSON *max_streams = cJSON_GetObjectItem(settings, "max_streams");
    if (max_streams && cJSON_IsNumber(max_streams)) {
        g_config.max_streams = max_streams->valueint;
        settings_changed = true;
        log_info("Updated max_streams: %d", g_config.max_streams);
    }
    
    // Log file
    cJSON *log_file = cJSON_GetObjectItem(settings, "log_file");
    if (log_file && cJSON_IsString(log_file)) {
        strncpy(g_config.log_file, log_file->valuestring, sizeof(g_config.log_file) - 1);
        g_config.log_file[sizeof(g_config.log_file) - 1] = '\0';
        settings_changed = true;
        log_info("Updated log_file: %s", g_config.log_file);
    }
    
    // Log level
    cJSON *log_level = cJSON_GetObjectItem(settings, "log_level");
    if (log_level && cJSON_IsNumber(log_level)) {
        g_config.log_level = log_level->valueint;
        set_log_level(g_config.log_level);
        settings_changed = true;
        log_info("Updated log_level: %d", g_config.log_level);
    }
    
    // Buffer size
    cJSON *buffer_size = cJSON_GetObjectItem(settings, "buffer_size");
    if (buffer_size && cJSON_IsNumber(buffer_size)) {
        g_config.buffer_size = buffer_size->valueint;
        settings_changed = true;
        log_info("Updated buffer_size: %d", g_config.buffer_size);
    }
    
    // Use swap
    cJSON *use_swap = cJSON_GetObjectItem(settings, "use_swap");
    if (use_swap && cJSON_IsBool(use_swap)) {
        g_config.use_swap = cJSON_IsTrue(use_swap);
        settings_changed = true;
        log_info("Updated use_swap: %s", g_config.use_swap ? "true" : "false");
    }
    
    // Swap size
    cJSON *swap_size = cJSON_GetObjectItem(settings, "swap_size");
    if (swap_size && cJSON_IsNumber(swap_size)) {
        g_config.swap_size = swap_size->valueint * 1024 * 1024; // Convert MB to bytes
        settings_changed = true;
        log_info("Updated swap_size: %llu bytes", (unsigned long long)g_config.swap_size);
    }

    // Pre-detection buffer (default for new streams)
    cJSON *pre_detection_buffer = cJSON_GetObjectItem(settings, "pre_detection_buffer");
    if (pre_detection_buffer && cJSON_IsNumber(pre_detection_buffer)) {
        int value = pre_detection_buffer->valueint;
        // Clamp to valid range
        if (value < 0) value = 0;
        if (value > 60) value = 60;
        g_config.default_pre_detection_buffer = value;
        settings_changed = true;
        log_info("Updated default_pre_detection_buffer: %d seconds", g_config.default_pre_detection_buffer);
    }

    // Post-detection buffer (default for new streams)
    cJSON *post_detection_buffer = cJSON_GetObjectItem(settings, "post_detection_buffer");
    if (post_detection_buffer && cJSON_IsNumber(post_detection_buffer)) {
        int value = post_detection_buffer->valueint;
        // Clamp to valid range
        if (value < 0) value = 0;
        if (value > 300) value = 300;
        g_config.default_post_detection_buffer = value;
        settings_changed = true;
        log_info("Updated default_post_detection_buffer: %d seconds", g_config.default_post_detection_buffer);
    }

    // Buffer strategy (default for new streams)
    cJSON *buffer_strategy = cJSON_GetObjectItem(settings, "buffer_strategy");
    if (buffer_strategy && cJSON_IsString(buffer_strategy)) {
        strncpy(g_config.default_buffer_strategy, buffer_strategy->valuestring, sizeof(g_config.default_buffer_strategy) - 1);
        g_config.default_buffer_strategy[sizeof(g_config.default_buffer_strategy) - 1] = '\0';
        settings_changed = true;
        log_info("Updated default_buffer_strategy: %s", g_config.default_buffer_strategy);
    }

    // MQTT enabled
    cJSON *mqtt_enabled = cJSON_GetObjectItem(settings, "mqtt_enabled");
    if (mqtt_enabled && cJSON_IsBool(mqtt_enabled)) {
        g_config.mqtt_enabled = cJSON_IsTrue(mqtt_enabled);
        settings_changed = true;
        log_info("Updated mqtt_enabled: %s", g_config.mqtt_enabled ? "true" : "false");
    }

    // MQTT broker host
    cJSON *mqtt_broker_host = cJSON_GetObjectItem(settings, "mqtt_broker_host");
    if (mqtt_broker_host && cJSON_IsString(mqtt_broker_host)) {
        strncpy(g_config.mqtt_broker_host, mqtt_broker_host->valuestring, sizeof(g_config.mqtt_broker_host) - 1);
        g_config.mqtt_broker_host[sizeof(g_config.mqtt_broker_host) - 1] = '\0';
        settings_changed = true;
        log_info("Updated mqtt_broker_host: %s", g_config.mqtt_broker_host);
    }

    // MQTT broker port
    cJSON *mqtt_broker_port = cJSON_GetObjectItem(settings, "mqtt_broker_port");
    if (mqtt_broker_port && cJSON_IsNumber(mqtt_broker_port)) {
        g_config.mqtt_broker_port = mqtt_broker_port->valueint;
        settings_changed = true;
        log_info("Updated mqtt_broker_port: %d", g_config.mqtt_broker_port);
    }

    // MQTT username
    cJSON *mqtt_username = cJSON_GetObjectItem(settings, "mqtt_username");
    if (mqtt_username && cJSON_IsString(mqtt_username)) {
        strncpy(g_config.mqtt_username, mqtt_username->valuestring, sizeof(g_config.mqtt_username) - 1);
        g_config.mqtt_username[sizeof(g_config.mqtt_username) - 1] = '\0';
        settings_changed = true;
        log_info("Updated mqtt_username: %s", g_config.mqtt_username);
    }

    // MQTT password (only update if not masked)
    cJSON *mqtt_password = cJSON_GetObjectItem(settings, "mqtt_password");
    if (mqtt_password && cJSON_IsString(mqtt_password) && strcmp(mqtt_password->valuestring, "********") != 0) {
        strncpy(g_config.mqtt_password, mqtt_password->valuestring, sizeof(g_config.mqtt_password) - 1);
        g_config.mqtt_password[sizeof(g_config.mqtt_password) - 1] = '\0';
        settings_changed = true;
        log_info("Updated mqtt_password");
    }

    // MQTT client ID
    cJSON *mqtt_client_id = cJSON_GetObjectItem(settings, "mqtt_client_id");
    if (mqtt_client_id && cJSON_IsString(mqtt_client_id)) {
        strncpy(g_config.mqtt_client_id, mqtt_client_id->valuestring, sizeof(g_config.mqtt_client_id) - 1);
        g_config.mqtt_client_id[sizeof(g_config.mqtt_client_id) - 1] = '\0';
        settings_changed = true;
        log_info("Updated mqtt_client_id: %s", g_config.mqtt_client_id);
    }

    // MQTT topic prefix
    cJSON *mqtt_topic_prefix = cJSON_GetObjectItem(settings, "mqtt_topic_prefix");
    if (mqtt_topic_prefix && cJSON_IsString(mqtt_topic_prefix)) {
        strncpy(g_config.mqtt_topic_prefix, mqtt_topic_prefix->valuestring, sizeof(g_config.mqtt_topic_prefix) - 1);
        g_config.mqtt_topic_prefix[sizeof(g_config.mqtt_topic_prefix) - 1] = '\0';
        settings_changed = true;
        log_info("Updated mqtt_topic_prefix: %s", g_config.mqtt_topic_prefix);
    }

    // MQTT TLS enabled
    cJSON *mqtt_tls_enabled = cJSON_GetObjectItem(settings, "mqtt_tls_enabled");
    if (mqtt_tls_enabled && cJSON_IsBool(mqtt_tls_enabled)) {
        g_config.mqtt_tls_enabled = cJSON_IsTrue(mqtt_tls_enabled);
        settings_changed = true;
        log_info("Updated mqtt_tls_enabled: %s", g_config.mqtt_tls_enabled ? "true" : "false");
    }

    // MQTT keepalive
    cJSON *mqtt_keepalive = cJSON_GetObjectItem(settings, "mqtt_keepalive");
    if (mqtt_keepalive && cJSON_IsNumber(mqtt_keepalive)) {
        g_config.mqtt_keepalive = mqtt_keepalive->valueint;
        settings_changed = true;
        log_info("Updated mqtt_keepalive: %d", g_config.mqtt_keepalive);
    }

    // MQTT QoS
    cJSON *mqtt_qos = cJSON_GetObjectItem(settings, "mqtt_qos");
    if (mqtt_qos && cJSON_IsNumber(mqtt_qos)) {
        int qos = mqtt_qos->valueint;
        if (qos < 0) qos = 0;
        if (qos > 2) qos = 2;
        g_config.mqtt_qos = qos;
        settings_changed = true;
        log_info("Updated mqtt_qos: %d", g_config.mqtt_qos);
    }

    // MQTT retain
    cJSON *mqtt_retain = cJSON_GetObjectItem(settings, "mqtt_retain");
    if (mqtt_retain && cJSON_IsBool(mqtt_retain)) {
        g_config.mqtt_retain = cJSON_IsTrue(mqtt_retain);
        settings_changed = true;
        log_info("Updated mqtt_retain: %s", g_config.mqtt_retain ? "true" : "false");
    }

    // Database path
    cJSON *db_path = cJSON_GetObjectItem(settings, "db_path");
    if (db_path && cJSON_IsString(db_path) && 
        strcmp(g_config.db_path, db_path->valuestring) != 0) {
        
        char old_db_path[MAX_PATH_LENGTH];
        strncpy(old_db_path, g_config.db_path, sizeof(old_db_path) - 1);
        old_db_path[sizeof(old_db_path) - 1] = '\0';
        
        // Update the config with the new path
        strncpy(g_config.db_path, db_path->valuestring, sizeof(g_config.db_path) - 1);
        g_config.db_path[sizeof(g_config.db_path) - 1] = '\0';
        settings_changed = true;
        log_info("Database path changed from %s to %s", old_db_path, g_config.db_path);
        
        // First, stop all HLS streams explicitly to ensure they're properly shut down
        log_info("Stopping all HLS streams before changing database path...");
        
        // Get a list of all active streams
        char active_streams[MAX_STREAMS][MAX_STREAM_NAME];
        int active_stream_count = 0;
        
        log_info("Scanning for active streams...");
        for (int i = 0; i < g_config.max_streams; i++) {
            log_info("Checking stream slot %d: name='%s', enabled=%d", 
                    i, g_config.streams[i].name, g_config.streams[i].enabled);
            
            if (g_config.streams[i].name[0] != '\0') {
                // Explicitly stop HLS streaming for all streams, even if they're not enabled
                log_info("Explicitly stopping HLS streaming for stream: %s", g_config.streams[i].name);
                stop_hls_stream(g_config.streams[i].name);
                
                // Only add enabled streams to the active list
                if (g_config.streams[i].enabled) {
                    // Copy the stream name for later use
                    strncpy(active_streams[active_stream_count], g_config.streams[i].name, MAX_STREAM_NAME - 1);
                    active_streams[active_stream_count][MAX_STREAM_NAME - 1] = '\0';
                    log_info("Added active stream %d: %s", active_stream_count, active_streams[active_stream_count]);
                    active_stream_count++;
                    
                    // Stop the stream
                    log_info("Stopping stream: %s", g_config.streams[i].name);
                    
                    // Get the stream handle
                    stream_handle_t stream = get_stream_by_name(g_config.streams[i].name);
                    if (stream) {
                        // Stop the stream
                        if (stop_stream(stream) == 0) {
                            log_info("Stream stopped: %s", g_config.streams[i].name);
                        } else {
                            log_warn("Failed to stop stream: %s", g_config.streams[i].name);
                        }
                    } else {
                        log_warn("Failed to get stream handle for: %s", g_config.streams[i].name);
                    }
                }
            }
        }
        
        log_info("Found %d active streams", active_stream_count);
        
        // Wait a bit to ensure all streams are fully stopped
        log_info("Waiting for streams to fully stop...");
        sleep(2);
        
        // We need to restart the database and stream manager
        log_info("Shutting down stream manager to change database path...");
        shutdown_stream_manager();
        
        log_info("Shutting down database...");
        shutdown_database();
        
        log_info("Initializing database with new path: %s", g_config.db_path);
        if (init_database(g_config.db_path) != 0) {
            log_error("Failed to initialize database with new path, reverting to old path");
            
            // Revert to the old path
            strncpy(g_config.db_path, old_db_path, sizeof(g_config.db_path) - 1);
            g_config.db_path[sizeof(g_config.db_path) - 1] = '\0';
            
            // Try to reinitialize with the old path
            if (init_database(g_config.db_path) != 0) {
                log_error("Failed to reinitialize database with old path, database may be unavailable");
            } else {
                log_info("Successfully reinitialized database with old path");
            }
            
            // Reinitialize stream manager
            if (init_stream_manager(g_config.max_streams) != 0) {
                log_error("Failed to reinitialize stream manager");
            } else {
                log_info("Successfully reinitialized stream manager");
            }
            
            // Send error response
            cJSON_Delete(settings);
            mg_send_json_error(c, 500, "Failed to initialize database with new path");
            return;
        }
        
        log_info("Reinitializing stream manager...");
        if (init_stream_manager(g_config.max_streams) != 0) {
            log_error("Failed to reinitialize stream manager");
            
            // Send error response
            cJSON_Delete(settings);
            mg_send_json_error(c, 500, "Failed to reinitialize stream manager");
            return;
        }
        
        // Restart streams from configuration
        log_info("Restarting streams from configuration...");
        
        // Restart all streams that were previously active
        log_info("Active stream count: %d", active_stream_count);
        for (int i = 0; i < active_stream_count; i++) {
            log_info("Processing active stream %d: %s", i, active_streams[i]);
            log_info("Restarting stream: %s", active_streams[i]);
            
            // Get the stream handle
            stream_handle_t stream = get_stream_by_name(active_streams[i]);
            if (stream) {
                // Get the stream configuration
                stream_config_t config;
                if (get_stream_config(stream, &config) != 0) {
                    log_error("Failed to get stream configuration for %s", active_streams[i]);
                    continue;
                }
                
                // Start the stream
                if (start_stream(stream) == 0) {
                    log_info("Stream restarted: %s", active_streams[i]);
                    
                    // Explicitly start HLS streaming if enabled
                    if (config.streaming_enabled) {
                        log_info("Starting HLS streaming for stream: %s", active_streams[i]);
                        
                        // Try multiple times to start HLS streaming
                        bool hls_started = false;
                        for (int retry = 0; retry < 3 && !hls_started; retry++) {
                            if (retry > 0) {
                                log_info("Retry %d starting HLS streaming for stream: %s", retry, active_streams[i]);
                                // Wait a bit before retrying
                                usleep(500000); // 500ms
                            }
                            
                            if (start_hls_stream(active_streams[i]) == 0) {
                                log_info("HLS streaming started for stream: %s", active_streams[i]);
                                hls_started = true;
                            } else {
                                log_warn("Failed to start HLS streaming for stream: %s (attempt %d/3)", 
                                        active_streams[i], retry + 1);
                            }
                        }
                        
                        if (!hls_started) {
                            log_error("Failed to start HLS streaming for stream: %s after multiple attempts", 
                                    active_streams[i]);
                        }
                    }
                } else {
                    log_warn("Failed to restart stream: %s", active_streams[i]);
                }
            } else {
                log_warn("Failed to get stream handle for: %s", active_streams[i]);
                
                // Try to find the stream configuration in the global config
                stream_config_t *config = NULL;
                for (int j = 0; j < g_config.max_streams; j++) {
                    if (strcmp(g_config.streams[j].name, active_streams[i]) == 0) {
                        config = &g_config.streams[j];
                        break;
                    }
                }
                
                if (config) {
                    // Try to add the stream first
                    stream = add_stream(config);
                    if (stream) {
                        log_info("Added stream: %s", active_streams[i]);
                        
                        // Start the stream
                        if (start_stream(stream) == 0) {
                            log_info("Stream started: %s", active_streams[i]);
                            
                            // Explicitly start HLS streaming if enabled
                            if (config->streaming_enabled) {
                                log_info("Starting HLS streaming for stream: %s", active_streams[i]);
                                
                                // Try multiple times to start HLS streaming
                                bool hls_started = false;
                                for (int retry = 0; retry < 3 && !hls_started; retry++) {
                                    if (retry > 0) {
                                        log_info("Retry %d starting HLS streaming for stream: %s", retry, active_streams[i]);
                                        // Wait a bit before retrying
                                        usleep(500000); // 500ms
                                    }
                                    
                                    if (start_hls_stream(active_streams[i]) == 0) {
                                        log_info("HLS streaming started for stream: %s", active_streams[i]);
                                        hls_started = true;
                                    } else {
                                        log_warn("Failed to start HLS streaming for stream: %s (attempt %d/3)", 
                                                active_streams[i], retry + 1);
                                    }
                                }
                                
                                if (!hls_started) {
                                    log_error("Failed to start HLS streaming for stream: %s after multiple attempts", 
                                            active_streams[i]);
                                }
                            }
                        } else {
                            log_warn("Failed to start stream: %s", active_streams[i]);
                        }
                    } else {
                        log_error("Failed to add stream: %s", active_streams[i]);
                    }
                } else {
                    log_error("Failed to find configuration for stream: %s", active_streams[i]);
                }
            }
        }
        
        // Wait a bit to ensure all streams have time to start
        log_info("Waiting for streams to fully start...");
        sleep(2);
        
        // Force restart all HLS streams to ensure they're properly started
        log_info("Force restarting all HLS streams to ensure they're properly started...");
        for (int i = 0; i < active_stream_count; i++) {
            log_info("Force restarting HLS for active stream %d: %s", i, active_streams[i]);
            
            // Get the stream handle
            stream_handle_t stream = get_stream_by_name(active_streams[i]);
            if (stream) {
                // Get the stream configuration
                stream_config_t config;
                if (get_stream_config(stream, &config) != 0) {
                    log_error("Failed to get stream configuration for %s", active_streams[i]);
                    continue;
                }
                
                // Explicitly restart HLS streaming if enabled
                if (config.streaming_enabled) {
                    log_info("Force restarting HLS streaming for stream: %s", active_streams[i]);
                    
                    // First stop the HLS stream
                    if (stop_hls_stream(active_streams[i]) != 0) {
                        log_warn("Failed to stop HLS stream for restart: %s", active_streams[i]);
                    }
                    
                    // Wait a bit to ensure the stream is fully stopped
                    usleep(500000); // 500ms
                    
                    // Start the HLS stream again
                    if (start_hls_stream(active_streams[i]) == 0) {
                        log_info("HLS streaming force restarted for stream: %s", active_streams[i]);
                    } else {
                        log_warn("Failed to force restart HLS streaming for stream: %s", active_streams[i]);
                    }
                } else {
                    log_warn("Streaming not enabled for stream: %s", active_streams[i]);
                }
            } else {
                log_warn("Failed to get stream handle for force restart: %s", active_streams[i]);
            }
        }
        
        // Always start all streams from the database after changing the database path
        log_info("Starting all streams from the database after changing database path...");
        
        // Get all stream configurations from the database
        stream_config_t db_streams[MAX_STREAMS];
        int count = get_all_stream_configs(db_streams, MAX_STREAMS);
        
        if (count > 0) {
            log_info("Found %d streams in the database", count);
            
            // Start each stream
            for (int i = 0; i < count; i++) {
                if (db_streams[i].name[0] != '\0' && db_streams[i].enabled) {
                    log_info("Starting stream from database: %s (streaming_enabled=%d)", 
                            db_streams[i].name, db_streams[i].streaming_enabled);
                    
                    // Add the stream
                    stream_handle_t stream = add_stream(&db_streams[i]);
                    if (stream) {
                        log_info("Added stream from database: %s", db_streams[i].name);
                        
                        // Start the stream
                        if (start_stream(stream) == 0) {
                            log_info("Started stream from database: %s", db_streams[i].name);
                            
                            // Explicitly start HLS streaming if enabled
                            if (db_streams[i].streaming_enabled) {
                                log_info("Starting HLS streaming for database stream: %s", db_streams[i].name);
                                
                                // Try multiple times to start HLS streaming
                                bool hls_started = false;
                                for (int retry = 0; retry < 3 && !hls_started; retry++) {
                                    if (retry > 0) {
                                        log_info("Retry %d starting HLS streaming for database stream: %s", 
                                                retry, db_streams[i].name);
                                        // Wait a bit before retrying
                                        usleep(500000); // 500ms
                                    }
                                    
                                    if (start_hls_stream(db_streams[i].name) == 0) {
                                        log_info("HLS streaming started for database stream: %s", db_streams[i].name);
                                        hls_started = true;
                                    } else {
                                        log_warn("Failed to start HLS streaming for database stream: %s (attempt %d/3)", 
                                                db_streams[i].name, retry + 1);
                                    }
                                }
                                
                                if (!hls_started) {
                                    log_error("Failed to start HLS streaming for database stream: %s after multiple attempts", 
                                            db_streams[i].name);
                                }
                            } else {
                                log_warn("HLS streaming not enabled for database stream: %s", db_streams[i].name);
                            }
                        } else {
                            log_warn("Failed to start stream from database: %s", db_streams[i].name);
                        }
                    } else {
                        log_error("Failed to add stream from database: %s", db_streams[i].name);
                    }
                }
            }
            
            // Wait a bit to ensure all streams have time to start
            log_info("Waiting for database streams to fully start...");
            sleep(2);
            
            // Force restart all HLS streams to ensure they're properly started
            log_info("Force restarting all HLS streams from database to ensure they're properly started...");
            for (int i = 0; i < count; i++) {
                if (db_streams[i].name[0] != '\0' && db_streams[i].enabled && db_streams[i].streaming_enabled) {
                    log_info("Force restarting HLS for database stream: %s", db_streams[i].name);
                    
                    // First stop the HLS stream
                    if (stop_hls_stream(db_streams[i].name) != 0) {
                        log_warn("Failed to stop HLS stream for restart: %s", db_streams[i].name);
                    }
                    
                    // Wait a bit to ensure the stream is fully stopped
                    usleep(500000); // 500ms
                    
                    // Start the HLS stream again
                    if (start_hls_stream(db_streams[i].name) == 0) {
                        log_info("HLS streaming force restarted for database stream: %s", db_streams[i].name);
                    } else {
                        log_warn("Failed to force restart HLS streaming for database stream: %s", db_streams[i].name);
                    }
                }
            }
        } else {
            log_warn("No streams found in the database");
        }
        
        log_info("Database path changed successfully");
    }
    
        // Save settings if changed
        if (settings_changed) {
            // Use the loaded config path - save_config will handle this automatically
            const char* config_path = get_loaded_config_path();
            log_info("Saving configuration to file: %s", config_path ? config_path : "default path");
            
            // Print the current database path to verify it's set correctly
            log_info("Current database path before saving: %s", g_config.db_path);
            
            // Save to the specific config file path if available
            int save_result;
            if (config_path) {
                save_result = save_config(&g_config, config_path);
            } else {
                save_result = save_config(&g_config, NULL);
            }
            
            if (save_result != 0) {
                log_error("Failed to save configuration, error code: %d", save_result);
                cJSON_Delete(settings);
                mg_send_json_error(c, 500, "Failed to save configuration");
                return;
            }
            
            log_info("Configuration saved successfully");
            
            // Reload the configuration to ensure changes are applied
            log_info("Reloading configuration after save");
            if (reload_config(&g_config) != 0) {
                log_warn("Failed to reload configuration after save, changes may not be applied until restart");
            } else {
                log_info("Configuration reloaded successfully");
                
                // Verify the database path after reload
                log_info("Database path after reload: %s", g_config.db_path);
            }
        } else {
            log_info("No settings changed");
        }
    
    // Clean up
    cJSON_Delete(settings);
    
    // Create success response using cJSON
    cJSON *success = cJSON_CreateObject();
    if (!success) {
        log_error("Failed to create success JSON object");
        mg_send_json_error(c, 500, "Failed to create success JSON");
        return;
    }
    
    cJSON_AddBoolToObject(success, "success", true);
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(success);
    if (!json_str) {
        log_error("Failed to convert success JSON to string");
        cJSON_Delete(success);
        mg_send_json_error(c, 500, "Failed to convert success JSON to string");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    cJSON_Delete(success);
    
    log_info("Successfully handled POST /api/settings request");
}
