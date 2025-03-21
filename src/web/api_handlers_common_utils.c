#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include "cJSON.h"
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

/**
 * @brief Helper function to parse a JSON string value
 * 
 * @param json JSON string
 * @param key Key to look for
 * @return char* String value (must be freed by caller) or NULL if not found
 */
char* get_json_string_value(const char *json, const char *key) {
    if (!json || !key) {
        return NULL;
    }
    
    // Parse JSON
    cJSON *json_obj = cJSON_Parse(json);
    if (!json_obj) {
        log_error("Failed to parse JSON");
        return NULL;
    }
    
    // Get string value
    cJSON *value = cJSON_GetObjectItemCaseSensitive(json_obj, key);
    if (!value || !cJSON_IsString(value)) {
        cJSON_Delete(json_obj);
        return NULL;
    }
    
    // Duplicate string
    char *result = strdup(value->valuestring);
    
    // Clean up
    cJSON_Delete(json_obj);
    
    return result;
}

/**
 * @brief Helper function to get a boolean value from JSON
 * 
 * @param json JSON string
 * @param key Key to look for
 * @param default_value Default value if key is not found
 * @return int Boolean value (0 or 1)
 */
int get_json_boolean_value(const char *json, const char *key, int default_value) {
    if (!json || !key) {
        return default_value;
    }
    
    // Parse JSON
    cJSON *json_obj = cJSON_Parse(json);
    if (!json_obj) {
        log_error("Failed to parse JSON");
        return default_value;
    }
    
    // Get boolean value
    cJSON *value = cJSON_GetObjectItemCaseSensitive(json_obj, key);
    if (!value) {
        cJSON_Delete(json_obj);
        return default_value;
    }
    
    int result = default_value;
    
    if (cJSON_IsBool(value)) {
        result = cJSON_IsTrue(value) ? 1 : 0;
    } else if (cJSON_IsNumber(value)) {
        result = value->valueint != 0 ? 1 : 0;
    }
    
    // Clean up
    cJSON_Delete(json_obj);
    
    return result;
}

/**
 * @brief Helper function to check if a key exists in JSON
 * 
 * @param json JSON string
 * @param key Key to look for
 * @return int 1 if key exists, 0 otherwise
 */
int get_json_has_key(const char *json, const char *key) {
    if (!json || !key) {
        return 0;
    }
    
    // Parse JSON
    cJSON *json_obj = cJSON_Parse(json);
    if (!json_obj) {
        log_error("Failed to parse JSON");
        return 0;
    }
    
    // Check if key exists
    int result = cJSON_HasObjectItem(json_obj, key) ? 1 : 0;
    
    // Clean up
    cJSON_Delete(json_obj);
    
    return result;
}

/**
 * @brief Helper function to get an integer value from JSON
 * 
 * @param json JSON string
 * @param key Key to look for
 * @param default_value Default value if key is not found
 * @return long long Integer value
 */
long long get_json_integer_value(const char *json, const char *key, long long default_value) {
    if (!json || !key) {
        return default_value;
    }
    
    // Parse JSON
    cJSON *json_obj = cJSON_Parse(json);
    if (!json_obj) {
        log_error("Failed to parse JSON");
        return default_value;
    }
    
    // Get integer value
    cJSON *value = cJSON_GetObjectItemCaseSensitive(json_obj, key);
    if (!value || !cJSON_IsNumber(value)) {
        cJSON_Delete(json_obj);
        return default_value;
    }
    
    long long result = (long long)value->valuedouble;
    
    // Clean up
    cJSON_Delete(json_obj);
    
    return result;
}

/**
 * @brief Create a stream error response
 * 
 * @param response HTTP response
 * @param status_code HTTP status code
 * @param message Error message
 */
void create_stream_error_response(http_response_t *response, int status_code, const char *message) {
    if (!response || !message) {
        return;
    }
    
    // Create JSON object using cJSON
    cJSON *error = cJSON_CreateObject();
    if (!error) {
        log_error("Failed to create error JSON object");
        return;
    }
    
    // Add error message
    cJSON_AddStringToObject(error, "error", message);
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(error);
    if (!json_str) {
        log_error("Failed to convert error JSON to string");
        cJSON_Delete(error);
        return;
    }
    
    // Create response
    create_json_response(response, status_code, json_str);
    
    // Clean up
    free(json_str);
    
    // Clean up
    cJSON_Delete(error);
}
