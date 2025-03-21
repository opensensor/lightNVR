/**
 * @file json_helpers.h
 * @brief JSON helper functions using cJSON
 */

#ifndef JSON_HELPERS_H
#define JSON_HELPERS_H

#include <stdbool.h>
#include <stdint.h>
#include "request_response.h"

// Include cJSON directly
#include "../external/cjson/cJSON.h"

/**
 * @brief Create a new JSON object
 * 
 * @return cJSON* New JSON object or NULL on error
 */
cJSON* json_create_object(void);

/**
 * @brief Create a new JSON array
 * 
 * @return cJSON* New JSON array or NULL on error
 */
cJSON* json_create_array(void);

/**
 * @brief Delete a JSON object
 * 
 * @param json JSON object to delete
 */
void json_delete(cJSON *json);

/**
 * @brief Add a string to a JSON object
 * 
 * @param object JSON object
 * @param name Property name
 * @param value String value
 * @return int 0 on success, non-zero on error
 */
int json_add_string(cJSON *object, const char *name, const char *value);

/**
 * @brief Add a number to a JSON object
 * 
 * @param object JSON object
 * @param name Property name
 * @param value Number value
 * @return int 0 on success, non-zero on error
 */
int json_add_number(cJSON *object, const char *name, double value);

/**
 * @brief Add an integer to a JSON object
 * 
 * @param object JSON object
 * @param name Property name
 * @param value Integer value
 * @return int 0 on success, non-zero on error
 */
int json_add_integer(cJSON *object, const char *name, int64_t value);

/**
 * @brief Add a boolean to a JSON object
 * 
 * @param object JSON object
 * @param name Property name
 * @param value Boolean value
 * @return int 0 on success, non-zero on error
 */
int json_add_boolean(cJSON *object, const char *name, bool value);

/**
 * @brief Add a null value to a JSON object
 * 
 * @param object JSON object
 * @param name Property name
 * @return int 0 on success, non-zero on error
 */
int json_add_null(cJSON *object, const char *name);

/**
 * @brief Add an object to a JSON object
 * 
 * @param object JSON object
 * @param name Property name
 * @param value Object value
 * @return int 0 on success, non-zero on error
 */
int json_add_object(cJSON *object, const char *name, cJSON *value);

/**
 * @brief Add an array to a JSON object
 * 
 * @param object JSON object
 * @param name Property name
 * @param value Array value
 * @return int 0 on success, non-zero on error
 */
int json_add_array(cJSON *object, const char *name, cJSON *value);

/**
 * @brief Add an item to a JSON array
 * 
 * @param array JSON array
 * @param value Item value
 * @return int 0 on success, non-zero on error
 */
int json_add_array_item(cJSON *array, cJSON *value);

/**
 * @brief Get a string from a JSON object
 * 
 * @param object JSON object
 * @param name Property name
 * @param default_value Default value if property not found
 * @return const char* String value or default_value if not found
 */
const char* json_get_string(const cJSON *object, const char *name, const char *default_value);

/**
 * @brief Get a number from a JSON object
 * 
 * @param object JSON object
 * @param name Property name
 * @param default_value Default value if property not found
 * @return double Number value or default_value if not found
 */
double json_get_number(const cJSON *object, const char *name, double default_value);

/**
 * @brief Get an integer from a JSON object
 * 
 * @param object JSON object
 * @param name Property name
 * @param default_value Default value if property not found
 * @return int64_t Integer value or default_value if not found
 */
int64_t json_get_integer(const cJSON *object, const char *name, int64_t default_value);

/**
 * @brief Get a boolean from a JSON object
 * 
 * @param object JSON object
 * @param name Property name
 * @param default_value Default value if property not found
 * @return bool Boolean value or default_value if not found
 */
bool json_get_boolean(const cJSON *object, const char *name, bool default_value);

/**
 * @brief Get an object from a JSON object
 * 
 * @param object JSON object
 * @param name Property name
 * @return cJSON* Object value or NULL if not found
 */
cJSON* json_get_object(const cJSON *object, const char *name);

/**
 * @brief Get an array from a JSON object
 * 
 * @param object JSON object
 * @param name Property name
 * @return cJSON* Array value or NULL if not found
 */
cJSON* json_get_array(const cJSON *object, const char *name);

/**
 * @brief Check if a property exists in a JSON object
 * 
 * @param object JSON object
 * @param name Property name
 * @return bool true if property exists, false otherwise
 */
bool json_has_property(const cJSON *object, const char *name);

/**
 * @brief Get the size of a JSON array
 * 
 * @param array JSON array
 * @return int Array size or 0 if not an array
 */
int json_array_size(const cJSON *array);

/**
 * @brief Get an item from a JSON array
 * 
 * @param array JSON array
 * @param index Item index
 * @return cJSON* Item value or NULL if not found
 */
cJSON* json_array_get(const cJSON *array, int index);

/**
 * @brief Parse a JSON string
 * 
 * @param string JSON string
 * @return cJSON* Parsed JSON object or NULL on error
 */
cJSON* json_parse(const char *string);

/**
 * @brief Convert a JSON object to a string
 * 
 * @param object JSON object
 * @param formatted Whether to format the string with indentation
 * @return char* JSON string (must be freed by the caller) or NULL on error
 */
char* json_to_string(const cJSON *object, bool formatted);

/**
 * @brief Create a JSON response
 * 
 * @param response HTTP response
 * @param status_code HTTP status code
 * @param json JSON object
 * @return int 0 on success, non-zero on error
 */
int json_create_response(http_response_t *response, int status_code, const cJSON *json);

/**
 * @brief Create a JSON error response
 * 
 * @param response HTTP response
 * @param status_code HTTP status code
 * @param error_message Error message
 * @return int 0 on success, non-zero on error
 */
int json_create_error_response(http_response_t *response, int status_code, const char *error_message);

/**
 * @brief Parse JSON from an HTTP request
 * 
 * @param request HTTP request
 * @return cJSON* Parsed JSON object or NULL on error
 */
cJSON* json_parse_request(const http_request_t *request);

#endif /* JSON_HELPERS_H */
