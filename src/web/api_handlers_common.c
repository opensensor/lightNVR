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
 * @brief Create a JSON string from a config structure
 * 
 * @param config Configuration structure
 * @return char* JSON string (must be freed by caller)
 */
char* mg_create_config_json(const config_t *config) {
    if (!config) {
        return NULL;
    }
    
    // Create JSON object
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
 * @brief Create a JSON error response
 * 
 * @param c Mongoose connection
 * @param status_code HTTP status code
 * @param message Error message
 */
void mg_create_error_response(struct mg_connection *c, int status_code, const char *message) {
    // Set proper headers for CORS and caching
    const char *headers = "Content-Type: application/json\r\n"
                         "Access-Control-Allow-Origin: *\r\n"
                         "Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\n"
                         "Access-Control-Allow-Headers: Content-Type, Authorization, X-Requested-With\r\n"
                         "Access-Control-Allow-Credentials: true\r\n"
                         "Cache-Control: no-cache, no-store, must-revalidate\r\n"
                         "Pragma: no-cache\r\n"
                         "Expires: 0\r\n";
    
    // Create JSON object
    cJSON *error = cJSON_CreateObject();
    if (!error) {
        mg_http_reply(c, 500, headers, "{\"error\": \"Failed to create error JSON\"}");
        return;
    }
    
    // Add error message
    cJSON_AddStringToObject(error, "error", message);
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(error);
    if (!json_str) {
        cJSON_Delete(error);
        mg_http_reply(c, 500, headers, "{\"error\": \"Failed to convert error JSON to string\"}");
        return;
    }
    
    // Send response
    mg_http_reply(c, status_code, headers, "%s", json_str);
    
    // Clean up
    free(json_str);
    cJSON_Delete(error);
}

/**
 * @brief URL decode a string
 * 
 * @param src Source string
 * @param dst Destination buffer
 * @param dst_size Destination buffer size
 */
void mg_url_decode_string(const char *src, char *dst, size_t dst_size) {
    if (!src || !dst || dst_size == 0) {
        return;
    }
    
    // Use Mongoose's URL decode function
    mg_url_decode(src, strlen(src), dst, dst_size, 0);
}

/**
 * @brief Extract a parameter from a URL path
 * 
 * @param path URL path
 * @param prefix Path prefix to skip
 * @param param_buf Buffer to store the parameter
 * @param buf_size Buffer size
 * @return int 0 on success, non-zero on error
 */
int mg_extract_path_parameter(const char *path, const char *prefix, char *param_buf, size_t buf_size) {
    if (!path || !prefix || !param_buf || buf_size == 0) {
        return -1;
    }
    
    // Check if path starts with prefix
    size_t prefix_len = strlen(prefix);
    if (strncmp(path, prefix, prefix_len) != 0) {
        return -1;
    }
    
    // Extract parameter (everything after the prefix)
    const char *param = path + prefix_len;
    
    // Skip any leading slashes
    while (*param == '/') {
        param++;
    }
    
    // Find query string if present and truncate
    const char *query = strchr(param, '?');
    if (query) {
        size_t param_len = query - param;
        if (param_len >= buf_size) {
            return -1;
        }
        
        strncpy(param_buf, param, param_len);
        param_buf[param_len] = '\0';
    } else {
        // No query string
        if (strlen(param) >= buf_size) {
            return -1;
        }
        
        strcpy(param_buf, param);
    }
    
    return 0;
}

/**
 * @brief Parse a JSON boolean value
 * 
 * @param json JSON object
 * @param key Key to look for
 * @param default_value Default value if key is not found
 * @return int Boolean value (0 or 1)
 */
int mg_parse_json_boolean(const cJSON *json, const char *key, int default_value) {
    if (!json || !key) {
        return default_value;
    }
    
    cJSON *value = cJSON_GetObjectItem(json, key);
    if (!value) {
        return default_value;
    }
    
    if (cJSON_IsBool(value)) {
        return cJSON_IsTrue(value) ? 1 : 0;
    }
    
    return default_value;
}

/**
 * @brief Parse a JSON integer value
 * 
 * @param json JSON object
 * @param key Key to look for
 * @param default_value Default value if key is not found
 * @return long long Integer value
 */
long long mg_parse_json_integer(const cJSON *json, const char *key, long long default_value) {
    if (!json || !key) {
        return default_value;
    }
    
    cJSON *value = cJSON_GetObjectItem(json, key);
    if (!value) {
        return default_value;
    }
    
    if (cJSON_IsNumber(value)) {
        return (long long)value->valuedouble;
    }
    
    return default_value;
}

/**
 * @brief Parse a JSON string value
 * 
 * @param json JSON object
 * @param key Key to look for
 * @return char* String value (must be freed by caller) or NULL if not found
 */
char* mg_parse_json_string(const cJSON *json, const char *key) {
    if (!json || !key) {
        return NULL;
    }
    
    cJSON *value = cJSON_GetObjectItem(json, key);
    if (!value || !cJSON_IsString(value)) {
        return NULL;
    }
    
    return strdup(value->valuestring);
}

/**
 * @brief Check if a JSON object has a key
 * 
 * @param json JSON object
 * @param key Key to look for
 * @return int 1 if key exists, 0 otherwise
 */
int mg_json_has_key(const cJSON *json, const char *key) {
    if (!json || !key) {
        return 0;
    }
    
    return cJSON_HasObjectItem(json, key) ? 1 : 0;
}
