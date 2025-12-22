#ifndef API_HANDLERS_COMMON_H
#define API_HANDLERS_COMMON_H

#include "web/web_server.h"
#include "core/config.h"
#include "database/db_auth.h"

// Forward declaration for mongoose types
struct mg_http_message;
struct mg_connection;

/**
 * Get the authenticated user from the HTTP request
 * @param hm Mongoose HTTP message
 * @param user Pointer to store the user information
 * @return 1 if user is authenticated, 0 otherwise
 */
int mg_get_authenticated_user(struct mg_http_message *hm, user_t *user);

/**
 * Check if the user has admin privileges
 * @param c Mongoose connection (used to send error response)
 * @param hm Mongoose HTTP message
 * @return 1 if user is admin, 0 otherwise (error response already sent)
 */
int mg_check_admin_privileges(struct mg_connection *c, struct mg_http_message *hm);

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
 * Helper function to check if a key exists in JSON
 */
int get_json_has_key(const char *json, const char *key);

/**
 * Helper function to get an integer value from JSON
 */
long long get_json_integer_value(const char *json, const char *key, long long default_value);

/**
 * Create a stream error response
 */
void create_stream_error_response(http_response_t *response, int status_code, const char *message);

#endif /* API_HANDLERS_COMMON_H */
