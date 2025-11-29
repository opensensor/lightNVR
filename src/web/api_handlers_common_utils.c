#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include <cjson/cJSON.h>
#include "web/api_handlers_common.h"
#include "core/logger.h"
#include "web/request_response.h"

/**
 * @brief Create a JSON string from a config structure
 * 
 * @param config Configuration structure
 * @return char* JSON string (must be freed by caller)
 */
char* create_json_string(const config_t *config) {
    if (!config) {
        return NULL;
    }
    
    // Create JSON object using cJSON
    cJSON *json = cJSON_CreateObject();
    if (!json) {
        log_error("Failed to create config JSON object");
        return NULL;
    }
    
    // Add config properties
    cJSON_AddNumberToObject(json, "log_level", config->log_level);
    cJSON_AddStringToObject(json, "storage_path", config->storage_path);
    cJSON_AddNumberToObject(json, "max_storage", config->max_storage_size / (1024 * 1024 * 1024)); // Convert to GB
    cJSON_AddNumberToObject(json, "retention", config->retention_days);
    cJSON_AddBoolToObject(json, "auto_delete", config->auto_delete_oldest);
    cJSON_AddNumberToObject(json, "web_port", config->web_port);
    cJSON_AddBoolToObject(json, "auth_enabled", config->web_auth_enabled);
    cJSON_AddStringToObject(json, "username", config->web_username);
    cJSON_AddStringToObject(json, "password", "********"); // Don't include actual password
    cJSON_AddNumberToObject(json, "buffer_size", config->buffer_size);
    cJSON_AddBoolToObject(json, "use_swap", config->use_swap);
    cJSON_AddNumberToObject(json, "swap_size", config->swap_size / (1024 * 1024)); // Convert to MB
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(json);
    
    // Clean up
    cJSON_Delete(json);
    
    return json_str;
}
