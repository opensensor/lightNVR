/**
 * @file api_handlers_users.h
 * @brief User management API handlers
 */

#ifndef API_HANDLERS_USERS_H
#define API_HANDLERS_USERS_H

#include "web/request_response.h"

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

/**
 * @brief Backend-agnostic handler for PUT /api/auth/users/:id/password
 * Change a user's password (requires old password for non-admins)
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_users_change_password(const http_request_t *req, http_response_t *res);

/**
 * @brief Backend-agnostic handler for PUT /api/auth/users/:id/password-lock
 * Lock or unlock password changes for a user (admin only)
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_users_password_lock(const http_request_t *req, http_response_t *res);

#endif /* API_HANDLERS_USERS_H */
