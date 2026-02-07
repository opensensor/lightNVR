/**
 * @file api_handlers_auth.h
 * @brief Authentication API handlers
 */

#ifndef API_HANDLERS_AUTH_H
#define API_HANDLERS_AUTH_H

#include "web/request_response.h"

/**
 * @brief Initialize the authentication system
 * This should be called during server startup
 * 
 * @return 0 on success, non-zero on failure
 */
int init_auth_system(void);

#endif /* API_HANDLERS_AUTH_H */
