/**
 * @file api_handlers_totp.h
 * @brief TOTP MFA API handlers
 */

#ifndef API_HANDLERS_TOTP_H
#define API_HANDLERS_TOTP_H

#include "web/request_response.h"

/**
 * @brief Handler for POST /api/auth/users/:id/totp/setup
 * Generate a TOTP secret and return the otpauth:// URI for QR code
 * Requires admin privileges or own user ID
 */
void handle_totp_setup(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for POST /api/auth/users/:id/totp/verify
 * Verify a TOTP code and enable TOTP for the user
 * Requires admin privileges or own user ID
 */
void handle_totp_verify(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for POST /api/auth/users/:id/totp/disable
 * Disable TOTP for a user
 * Requires admin privileges or own user ID
 */
void handle_totp_disable(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for GET /api/auth/users/:id/totp/status
 * Get TOTP enabled status for a user
 */
void handle_totp_status(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for POST /api/auth/login/totp
 * Complete login by verifying TOTP code after password authentication
 * Accepts { "totp_token": "...", "code": "123456" }
 */
void handle_auth_login_totp(const http_request_t *req, http_response_t *res);

/**
 * @brief Verify a TOTP code against a base32-encoded secret
 * @param secret_b32 Base32-encoded TOTP secret
 * @param code_str 6-digit code string to verify
 * @return 0 if valid, -1 if invalid
 */
int totp_verify(const char *secret_b32, const char *code_str);

#endif /* API_HANDLERS_TOTP_H */

