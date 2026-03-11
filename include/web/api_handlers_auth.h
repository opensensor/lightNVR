/**
 * @file api_handlers_auth.h
 * @brief Authentication API handlers
 */

#ifndef API_HANDLERS_AUTH_H
#define API_HANDLERS_AUTH_H

#include <stdbool.h>

#include "web/request_response.h"

/**
 * @brief Initialize the authentication system
 * This should be called during server startup
 * 
 * @return 0 on success, non-zero on failure
 */
int init_auth_system(void);

/**
 * @brief Clear the in-memory login rate limit entry for a username.
 *
 * @param username Username whose lockout/attempt window should be cleared.
 * @return true if an entry existed and was cleared, false otherwise.
 */
bool auth_clear_login_rate_limit_for_username(const char *username);

#endif /* API_HANDLERS_AUTH_H */
