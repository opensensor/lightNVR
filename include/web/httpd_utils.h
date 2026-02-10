/**
 * @file httpd_utils.h
 * @brief Backend-agnostic HTTP utility functions for API handlers
 *
 * These functions work with http_request_t/http_response_t and provide
 * application-level helpers (JSON parsing, auth, etc.) without depending
 * on any specific HTTP server backend.
 */

#ifndef HTTPD_UTILS_H
#define HTTPD_UTILS_H

#include <cjson/cJSON.h>
#include "web/request_response.h"
#include "database/db_auth.h"

/**
 * @brief Parse JSON body from an HTTP request
 * @param req HTTP request
 * @return cJSON* Parsed JSON object or NULL on error. Caller must free with cJSON_Delete.
 */
cJSON* httpd_parse_json_body(const http_request_t *req);

/**
 * @brief Extract HTTP Basic Auth credentials from request
 * @param req HTTP request
 * @param username Buffer to store username
 * @param username_size Size of username buffer
 * @param password Buffer to store password
 * @param password_size Size of password buffer
 * @return 0 if credentials found, -1 if not
 */
int httpd_get_basic_auth_credentials(const http_request_t *req,
                                      char *username, size_t username_size,
                                      char *password, size_t password_size);

/**
 * @brief Get the authenticated user from the HTTP request
 *
 * Checks session cookie first, then falls back to HTTP Basic Auth.
 *
 * @param req HTTP request
 * @param user Pointer to store the user information
 * @return 1 if user is authenticated, 0 otherwise
 */
int httpd_get_authenticated_user(const http_request_t *req, user_t *user);

/**
 * @brief Check if the authenticated user has admin privileges
 *
 * If not authenticated or not admin, sets an appropriate error response.
 *
 * @param req HTTP request
 * @param res HTTP response (error will be set if not admin)
 * @return 1 if user is admin, 0 otherwise (error response already set)
 */
int httpd_check_admin_privileges(const http_request_t *req, http_response_t *res);

/**
 * @brief Extract session token from Cookie header
 * @param req HTTP request
 * @param token Buffer to store the session token
 * @param token_size Size of token buffer
 * @return 0 if session token found, -1 if not
 */
int httpd_get_session_token(const http_request_t *req, char *token, size_t token_size);

/**
 * @brief Check if the request has viewer-level access
 *
 * In demo mode, unauthenticated users are granted viewer-level access.
 * Otherwise, requires authentication with any role.
 *
 * @param req HTTP request
 * @param user Pointer to store the user information (may be a demo user)
 * @return 1 if user has viewer access, 0 otherwise
 */
int httpd_check_viewer_access(const http_request_t *req, user_t *user);

/**
 * @brief Check if demo mode is enabled
 * @return 1 if demo mode is enabled, 0 otherwise
 */
int httpd_is_demo_mode(void);

#endif /* HTTPD_UTILS_H */

