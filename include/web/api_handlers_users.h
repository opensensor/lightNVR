/**
 * @file api_handlers_users.h
 * @brief User management API handlers
 */

#ifndef API_HANDLERS_USERS_H
#define API_HANDLERS_USERS_H

// Forward declarations for Mongoose structures
struct mg_connection;
struct mg_http_message;

/**
 * @brief Handle GET /api/auth/users
 * Get a list of all users
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_users_list(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handle GET /api/auth/users/:id
 * Get a specific user by ID
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_users_get(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handle POST /api/auth/users
 * Create a new user
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_users_create(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handle PUT /api/auth/users/:id
 * Update an existing user
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_users_update(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handle DELETE /api/auth/users/:id
 * Delete a user
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_users_delete(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handle POST /api/auth/users/:id/api-key
 * Generate a new API key for a user
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_users_generate_api_key(struct mg_connection *c, struct mg_http_message *hm);

#endif /* API_HANDLERS_USERS_H */
