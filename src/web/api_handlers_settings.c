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
    
    // Don't include the password for security reasons
    cJSON_AddStringToObject(settings, "web_password", "********");
    
    cJSON_AddStringToObject(settings, "storage_path", g_config.storage_path);
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
    
    // Storage path
    cJSON *storage_path = cJSON_GetObjectItem(settings, "storage_path");
    if (storage_path && cJSON_IsString(storage_path)) {
        strncpy(g_config.storage_path, storage_path->valuestring, sizeof(g_config.storage_path) - 1);
        g_config.storage_path[sizeof(g_config.storage_path) - 1] = '\0';
        settings_changed = true;
        log_info("Updated storage_path: %s", g_config.storage_path);
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
    
    // Save settings if changed
    if (settings_changed) {
        // Use the loaded config path - save_config will handle this automatically
        log_info("Saving configuration to the loaded config file");
        if (save_config(&g_config, NULL) != 0) {
            log_error("Failed to save configuration");
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
