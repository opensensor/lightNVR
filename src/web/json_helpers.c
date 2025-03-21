#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "web/json_helpers.h"
#include "core/logger.h"

/**
 * @brief Create a new JSON object
 */
cJSON* json_create_object(void) {
    return cJSON_CreateObject();
}

/**
 * @brief Create a new JSON array
 */
cJSON* json_create_array(void) {
    return cJSON_CreateArray();
}

/**
 * @brief Delete a JSON object
 */
void json_delete(cJSON *json) {
    cJSON_Delete(json);
}

/**
 * @brief Add a string to a JSON object
 */
int json_add_string(cJSON *object, const char *name, const char *value) {
    if (!object || !name || !value) {
        return -1;
    }
    
    cJSON *item = cJSON_AddStringToObject(object, name, value);
    return item ? 0 : -1;
}

/**
 * @brief Add a number to a JSON object
 */
int json_add_number(cJSON *object, const char *name, double value) {
    if (!object || !name) {
        return -1;
    }
    
    cJSON *item = cJSON_AddNumberToObject(object, name, value);
    return item ? 0 : -1;
}

/**
 * @brief Add an integer to a JSON object
 */
int json_add_integer(cJSON *object, const char *name, int64_t value) {
    if (!object || !name) {
        return -1;
    }
    
    // cJSON uses double internally, so we convert the int64_t to double
    cJSON *item = cJSON_AddNumberToObject(object, name, (double)value);
    return item ? 0 : -1;
}

/**
 * @brief Add a boolean to a JSON object
 */
int json_add_boolean(cJSON *object, const char *name, bool value) {
    if (!object || !name) {
        return -1;
    }
    
    cJSON *item = cJSON_AddBoolToObject(object, name, value ? 1 : 0);
    return item ? 0 : -1;
}

/**
 * @brief Add a null value to a JSON object
 */
int json_add_null(cJSON *object, const char *name) {
    if (!object || !name) {
        return -1;
    }
    
    cJSON *item = cJSON_AddNullToObject(object, name);
    return item ? 0 : -1;
}

/**
 * @brief Add an object to a JSON object
 */
int json_add_object(cJSON *object, const char *name, cJSON *value) {
    if (!object || !name || !value) {
        return -1;
    }
    
    // Add the value to the object
    int result = cJSON_AddItemToObject(object, name, value);
    return result ? 0 : -1;
}

/**
 * @brief Add an array to a JSON object
 */
int json_add_array(cJSON *object, const char *name, cJSON *value) {
    if (!object || !name || !value) {
        return -1;
    }
    
    // Add the value to the object
    int result = cJSON_AddItemToObject(object, name, value);
    return result ? 0 : -1;
}

/**
 * @brief Add an item to a JSON array
 */
int json_add_array_item(cJSON *array, cJSON *value) {
    if (!array || !value) {
        return -1;
    }
    
    cJSON_AddItemToArray(array, value);
    return 0;
}

/**
 * @brief Get a string from a JSON object
 */
const char* json_get_string(const cJSON *object, const char *name, const char *default_value) {
    if (!object || !name) {
        return default_value;
    }
    
    cJSON *item = cJSON_GetObjectItemCaseSensitive(object, name);
    if (!item || !cJSON_IsString(item)) {
        return default_value;
    }
    
    return item->valuestring;
}

/**
 * @brief Get a number from a JSON object
 */
double json_get_number(const cJSON *object, const char *name, double default_value) {
    if (!object || !name) {
        return default_value;
    }
    
    cJSON *item = cJSON_GetObjectItemCaseSensitive(object, name);
    if (!item || !cJSON_IsNumber(item)) {
        return default_value;
    }
    
    return item->valuedouble;
}

/**
 * @brief Get an integer from a JSON object
 */
int64_t json_get_integer(const cJSON *object, const char *name, int64_t default_value) {
    if (!object || !name) {
        return default_value;
    }
    
    cJSON *item = cJSON_GetObjectItemCaseSensitive(object, name);
    if (!item || !cJSON_IsNumber(item)) {
        return default_value;
    }
    
    return (int64_t)item->valuedouble;
}

/**
 * @brief Get a boolean from a JSON object
 */
bool json_get_boolean(const cJSON *object, const char *name, bool default_value) {
    if (!object || !name) {
        return default_value;
    }
    
    cJSON *item = cJSON_GetObjectItemCaseSensitive(object, name);
    if (!item || (!cJSON_IsBool(item) && !cJSON_IsNumber(item))) {
        return default_value;
    }
    
    if (cJSON_IsBool(item)) {
        return cJSON_IsTrue(item);
    } else {
        // Handle numeric values as booleans (0 = false, non-0 = true)
        return item->valuedouble != 0;
    }
}

/**
 * @brief Get an object from a JSON object
 */
cJSON* json_get_object(const cJSON *object, const char *name) {
    if (!object || !name) {
        return NULL;
    }
    
    cJSON *item = cJSON_GetObjectItemCaseSensitive(object, name);
    if (!item || !cJSON_IsObject(item)) {
        return NULL;
    }
    
    return item;
}

/**
 * @brief Get an array from a JSON object
 */
cJSON* json_get_array(const cJSON *object, const char *name) {
    if (!object || !name) {
        return NULL;
    }
    
    cJSON *item = cJSON_GetObjectItemCaseSensitive(object, name);
    if (!item || !cJSON_IsArray(item)) {
        return NULL;
    }
    
    return item;
}

/**
 * @brief Check if a property exists in a JSON object
 */
bool json_has_property(const cJSON *object, const char *name) {
    if (!object || !name) {
        return false;
    }
    
    return cJSON_GetObjectItemCaseSensitive(object, name) != NULL;
}

/**
 * @brief Get the size of a JSON array
 */
int json_array_size(const cJSON *array) {
    if (!array || !cJSON_IsArray(array)) {
        return 0;
    }
    
    return cJSON_GetArraySize(array);
}

/**
 * @brief Get an item from a JSON array
 */
cJSON* json_array_get(const cJSON *array, int index) {
    if (!array || !cJSON_IsArray(array) || index < 0) {
        return NULL;
    }
    
    return cJSON_GetArrayItem(array, index);
}

/**
 * @brief Parse a JSON string
 */
cJSON* json_parse(const char *string) {
    if (!string) {
        return NULL;
    }
    
    return cJSON_Parse(string);
}

/**
 * @brief Convert a JSON object to a string
 */
char* json_to_string(const cJSON *object, bool formatted) {
    if (!object) {
        return NULL;
    }
    
    return formatted ? cJSON_Print(object) : cJSON_PrintUnformatted(object);
}

/**
 * @brief Create a JSON response
 */
int json_create_response(http_response_t *response, int status_code, const cJSON *json) {
    if (!response || !json) {
        return -1;
    }
    
    char *json_string = cJSON_PrintUnformatted(json);
    if (!json_string) {
        log_error("Failed to convert JSON to string");
        return -1;
    }
    
    response->status_code = status_code;
    strncpy(response->content_type, "application/json", sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';
    
    // Free any existing response body
    if (response->body) {
        free(response->body);
    }
    
    response->body = json_string;
    response->body_length = strlen(json_string);
    
    return 0;
}

/**
 * @brief Create a JSON error response
 */
int json_create_error_response(http_response_t *response, int status_code, const char *error_message) {
    if (!response || !error_message) {
        return -1;
    }
    
    cJSON *error = cJSON_CreateObject();
    if (!error) {
        log_error("Failed to create error JSON object");
        return -1;
    }
    
    if (cJSON_AddStringToObject(error, "error", error_message) == NULL) {
        log_error("Failed to add error message to JSON object");
        cJSON_Delete(error);
        return -1;
    }
    
    int result = json_create_response(response, status_code, error);
    cJSON_Delete(error);
    
    return result;
}

/**
 * @brief Parse JSON from an HTTP request
 */
cJSON* json_parse_request(const http_request_t *request) {
    if (!request || !request->body || request->content_length == 0) {
        return NULL;
    }
    
    // Check if content type is JSON
    if (strncmp(request->content_type, "application/json", 16) != 0) {
        log_warn("Request content type is not application/json: %s", request->content_type);
        // Continue anyway, try to parse as JSON
    }
    
    // Parse JSON
    cJSON *json = cJSON_ParseWithLength(request->body, request->content_length);
    if (!json) {
        const char *error_ptr = cJSON_GetErrorPtr();
        if (error_ptr) {
            log_error("JSON parse error near: %s", error_ptr);
        } else {
            log_error("JSON parse error");
        }
        return NULL;
    }
    
    return json;
}
