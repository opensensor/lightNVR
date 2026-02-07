/**
 * @file api_handlers_users.h
 * @brief User management API handlers
 */

#ifndef API_HANDLERS_USERS_H
#define API_HANDLERS_USERS_H

#include "web/request_response.h"

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

/* ========================================================================
 * Backend-Agnostic User Management Handlers
 * ======================================================================== */

/**
 * @brief Backend-agnostic handler for GET /api/auth/users
 * Get a list of all users
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_users_list(const http_request_t *req, http_response_t *res);

/**
 * @brief Backend-agnostic handler for GET /api/auth/users/:id
 * Get a specific user by ID
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_users_get(const http_request_t *req, http_response_t *res);

/**
 * @brief Backend-agnostic handler for POST /api/auth/users
 * Create a new user
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_users_create(const http_request_t *req, http_response_t *res);

/**
 * @brief Backend-agnostic handler for PUT /api/auth/users/:id
 * Update an existing user
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_users_update(const http_request_t *req, http_response_t *res);

/**
 * @brief Backend-agnostic handler for DELETE /api/auth/users/:id
 * Delete a user
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_users_delete(const http_request_t *req, http_response_t *res);

/**
 * @brief Backend-agnostic handler for POST /api/auth/users/:id/api-key
 * Generate a new API key for a user
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_users_generate_api_key(const http_request_t *req, http_response_t *res);

#endif /* API_HANDLERS_USERS_H */
