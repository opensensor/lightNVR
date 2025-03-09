#ifndef API_HANDLERS_COMMON_H
#define API_HANDLERS_COMMON_H

#include "web/web_server.h"
#include "core/config.h"

/**
 * Helper function to create a simple JSON string from config
 */
char* create_json_string(const config_t *config);

/**
 * Helper function to parse a JSON string value
 */
char* get_json_string_value(const char *json, const char *key);

/**
 * Helper function to get a boolean value from JSON
 */
int get_json_boolean_value(const char *json, const char *key, int default_value);

/**
 * Helper function to get an integer value from JSON
 */
long long get_json_integer_value(const char *json, const char *key, long long default_value);

/**
 * Create a stream error response
 */
void create_stream_error_response(http_response_t *response, int status_code, const char *message);

#endif /* API_HANDLERS_COMMON_H */
