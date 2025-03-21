#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include "cJSON.h"

#include "web/api_handlers.h"
#include "web/mongoose_adapter.h"
#include "core/logger.h"
#include "core/config.h"
#include "mongoose.h"

/**
 * @brief Direct handler for GET /api/settings
 */
void mg_handle_get_settings(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/settings request");
    
    // Get global configuration
    extern config_t global_config;
    
    // Create JSON object
    cJSON *settings = cJSON_CreateObject();
    if (!settings) {
        log_error("Failed to create settings JSON object");
        mg_send_json_error(c, 500, "Failed to create settings JSON");
        return;
    }
    
    // Add settings properties
    cJSON_AddNumberToObject(settings, "web_port", global_config.web_port);
    cJSON_AddStringToObject(settings, "web_root", global_config.web_root);
    cJSON_AddBoolToObject(settings, "web_auth_enabled", global_config.web_auth_enabled);
    cJSON_AddStringToObject(settings, "web_username", global_config.web_username);
    
    // Don't include the password for security reasons
    cJSON_AddStringToObject(settings, "web_password", "********");
    
    cJSON_AddStringToObject(settings, "storage_path", global_config.storage_path);
    cJSON_AddNumberToObject(settings, "max_storage_size", global_config.max_storage_size);
    cJSON_AddNumberToObject(settings, "max_streams", global_config.max_streams);
    cJSON_AddStringToObject(settings, "log_file", global_config.log_file);
    cJSON_AddNumberToObject(settings, "log_level", global_config.log_level);
    cJSON_AddStringToObject(settings, "pid_file", global_config.pid_file);
    cJSON_AddStringToObject(settings, "db_path", global_config.db_path);
    
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
    
    // Parse JSON from request body
    cJSON *settings = mg_parse_json_body(hm);
    if (!settings) {
        log_error("Failed to parse settings JSON from request body");
        mg_send_json_error(c, 400, "Invalid JSON in request body");
        return;
    }
    
    // Get global configuration
    extern config_t global_config;
    
    // Update settings
    bool settings_changed = false;
    
    // Web port
    cJSON *web_port = cJSON_GetObjectItem(settings, "web_port");
    if (web_port && cJSON_IsNumber(web_port)) {
        global_config.web_port = web_port->valueint;
        settings_changed = true;
        log_info("Updated web_port: %d", global_config.web_port);
    }
    
    // Web root
    cJSON *web_root = cJSON_GetObjectItem(settings, "web_root");
    if (web_root && cJSON_IsString(web_root)) {
        strncpy(global_config.web_root, web_root->valuestring, sizeof(global_config.web_root) - 1);
        global_config.web_root[sizeof(global_config.web_root) - 1] = '\0';
        settings_changed = true;
        log_info("Updated web_root: %s", global_config.web_root);
    }
    
    // Web auth enabled
    cJSON *web_auth_enabled = cJSON_GetObjectItem(settings, "web_auth_enabled");
    if (web_auth_enabled && cJSON_IsBool(web_auth_enabled)) {
        global_config.web_auth_enabled = cJSON_IsTrue(web_auth_enabled);
        settings_changed = true;
        log_info("Updated web_auth_enabled: %s", global_config.web_auth_enabled ? "true" : "false");
    }
    
    // Web username
    cJSON *web_username = cJSON_GetObjectItem(settings, "web_username");
    if (web_username && cJSON_IsString(web_username)) {
        strncpy(global_config.web_username, web_username->valuestring, sizeof(global_config.web_username) - 1);
        global_config.web_username[sizeof(global_config.web_username) - 1] = '\0';
        settings_changed = true;
        log_info("Updated web_username: %s", global_config.web_username);
    }
    
    // Web password
    cJSON *web_password = cJSON_GetObjectItem(settings, "web_password");
    if (web_password && cJSON_IsString(web_password) && strcmp(web_password->valuestring, "********") != 0) {
        strncpy(global_config.web_password, web_password->valuestring, sizeof(global_config.web_password) - 1);
        global_config.web_password[sizeof(global_config.web_password) - 1] = '\0';
        settings_changed = true;
        log_info("Updated web_password");
    }
    
    // Storage path
    cJSON *storage_path = cJSON_GetObjectItem(settings, "storage_path");
    if (storage_path && cJSON_IsString(storage_path)) {
        strncpy(global_config.storage_path, storage_path->valuestring, sizeof(global_config.storage_path) - 1);
        global_config.storage_path[sizeof(global_config.storage_path) - 1] = '\0';
        settings_changed = true;
        log_info("Updated storage_path: %s", global_config.storage_path);
    }
    
    // Max storage size
    cJSON *max_storage_size = cJSON_GetObjectItem(settings, "max_storage_size");
    if (max_storage_size && cJSON_IsNumber(max_storage_size)) {
        global_config.max_storage_size = max_storage_size->valueint;
        settings_changed = true;
        log_info("Updated max_storage_size: %d", global_config.max_storage_size);
    }
    
    // Max streams
    cJSON *max_streams = cJSON_GetObjectItem(settings, "max_streams");
    if (max_streams && cJSON_IsNumber(max_streams)) {
        global_config.max_streams = max_streams->valueint;
        settings_changed = true;
        log_info("Updated max_streams: %d", global_config.max_streams);
    }
    
    // Log file
    cJSON *log_file = cJSON_GetObjectItem(settings, "log_file");
    if (log_file && cJSON_IsString(log_file)) {
        strncpy(global_config.log_file, log_file->valuestring, sizeof(global_config.log_file) - 1);
        global_config.log_file[sizeof(global_config.log_file) - 1] = '\0';
        settings_changed = true;
        log_info("Updated log_file: %s", global_config.log_file);
    }
    
    // Log level
    cJSON *log_level = cJSON_GetObjectItem(settings, "log_level");
    if (log_level && cJSON_IsNumber(log_level)) {
        global_config.log_level = log_level->valueint;
        set_log_level(global_config.log_level);
        settings_changed = true;
        log_info("Updated log_level: %d", global_config.log_level);
    }
    
    // Save settings if changed
    if (settings_changed) {
        // Get the custom config path if set, otherwise use default paths
        const char *config_path = get_custom_config_path();
        if (!config_path) {
            // Try to use the system path first if it exists and is writable
            if (access("/etc/lightnvr", W_OK) == 0) {
                config_path = "/etc/lightnvr/lightnvr.ini";
            } else {
                // Fall back to current directory
                config_path = "./lightnvr.ini";
            }
        }
        
        log_info("Saving configuration to %s", config_path);
        if (save_config(&global_config, config_path) != 0) {
            log_error("Failed to save configuration to %s", config_path);
            cJSON_Delete(settings);
            mg_send_json_error(c, 500, "Failed to save configuration");
            return;
        }
        
        log_info("Configuration saved successfully to %s", config_path);
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
