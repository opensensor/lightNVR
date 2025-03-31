/**
 * @file db_auth.h
 * @brief Database authentication functions
 */

#ifndef LIGHTNVR_DB_AUTH_H
#define LIGHTNVR_DB_AUTH_H

#include <stdbool.h>
#include <stdint.h>
#include <stddef.h>

/**
 * @brief User roles
 */
typedef enum {
    USER_ROLE_ADMIN = 0,  /**< Administrator role with full access */
    USER_ROLE_USER = 1,   /**< Regular user with limited access */
    USER_ROLE_VIEWER = 2, /**< Viewer with read-only access */
    USER_ROLE_API = 3     /**< API user for programmatic access */
} user_role_t;

/**
 * @brief User structure
 */
typedef struct {
    int64_t id;              /**< User ID */
    char username[64];       /**< Username */
    char email[128];         /**< Email address (optional) */
    user_role_t role;        /**< User role */
    char api_key[64];        /**< API key for programmatic access */
    int64_t created_at;      /**< Creation timestamp */
    int64_t updated_at;      /**< Last update timestamp */
    int64_t last_login;      /**< Last login timestamp */
    bool is_active;          /**< Whether the user is active */
} user_t;

/**
 * @brief Session structure
 */
typedef struct {
    int64_t id;              /**< Session ID */
    int64_t user_id;         /**< User ID */
    char token[128];         /**< Session token */
    int64_t created_at;      /**< Creation timestamp */
    int64_t expires_at;      /**< Expiration timestamp */
    char ip_address[46];     /**< IP address of the client */
    char user_agent[256];    /**< User agent of the client */
} session_t;

/**
 * @brief Initialize the authentication system
 * 
 * This function initializes the authentication system and creates the default admin user
 * if it doesn't exist.
 * 
 * @return 0 on success, non-zero on failure
 */
int db_auth_init(void);

/**
 * @brief Create a new user
 * 
 * @param username Username
 * @param password Password (will be hashed)
 * @param email Email address (optional, can be NULL)
 * @param role User role
 * @param is_active Whether the user is active
 * @param user_id Pointer to store the user ID (optional, can be NULL)
 * @return 0 on success, non-zero on failure
 */
int db_auth_create_user(const char *username, const char *password, const char *email, 
                        user_role_t role, bool is_active, int64_t *user_id);

/**
 * @brief Update a user
 * 
 * @param user_id User ID
 * @param email New email address (optional, can be NULL to leave unchanged)
 * @param role New user role (optional, can be -1 to leave unchanged)
 * @param is_active New active status (optional, can be -1 to leave unchanged)
 * @return 0 on success, non-zero on failure
 */
int db_auth_update_user(int64_t user_id, const char *email, int role, int is_active);

/**
 * @brief Change a user's password
 * 
 * @param user_id User ID
 * @param new_password New password (will be hashed)
 * @return 0 on success, non-zero on failure
 */
int db_auth_change_password(int64_t user_id, const char *new_password);

/**
 * @brief Delete a user
 * 
 * @param user_id User ID
 * @return 0 on success, non-zero on failure
 */
int db_auth_delete_user(int64_t user_id);

/**
 * @brief Get a user by ID
 * 
 * @param user_id User ID
 * @param user Pointer to store the user information
 * @return 0 on success, non-zero on failure
 */
int db_auth_get_user_by_id(int64_t user_id, user_t *user);

/**
 * @brief Get a user by username
 * 
 * @param username Username
 * @param user Pointer to store the user information
 * @return 0 on success, non-zero on failure
 */
int db_auth_get_user_by_username(const char *username, user_t *user);

/**
 * @brief Get a user by API key
 * 
 * @param api_key API key
 * @param user Pointer to store the user information
 * @return 0 on success, non-zero on failure
 */
int db_auth_get_user_by_api_key(const char *api_key, user_t *user);

/**
 * @brief Generate a new API key for a user
 * 
 * @param user_id User ID
 * @param api_key Buffer to store the generated API key
 * @param api_key_size Size of the API key buffer
 * @return 0 on success, non-zero on failure
 */
int db_auth_generate_api_key(int64_t user_id, char *api_key, size_t api_key_size);

/**
 * @brief Authenticate a user with username and password
 * 
 * @param username Username
 * @param password Password
 * @param user_id Pointer to store the user ID (optional, can be NULL)
 * @return 0 on success, non-zero on failure
 */
int db_auth_authenticate(const char *username, const char *password, int64_t *user_id);

/**
 * @brief Create a new session for a user
 * 
 * @param user_id User ID
 * @param ip_address IP address of the client (optional, can be NULL)
 * @param user_agent User agent of the client (optional, can be NULL)
 * @param expiry_seconds Session expiry time in seconds
 * @param token Buffer to store the generated session token
 * @param token_size Size of the token buffer
 * @return 0 on success, non-zero on failure
 */
int db_auth_create_session(int64_t user_id, const char *ip_address, const char *user_agent,
                          int expiry_seconds, char *token, size_t token_size);

/**
 * @brief Validate a session token
 * 
 * @param token Session token
 * @param user_id Pointer to store the user ID (optional, can be NULL)
 * @return 0 on success, non-zero on failure
 */
int db_auth_validate_session(const char *token, int64_t *user_id);

/**
 * @brief Delete a session
 * 
 * @param token Session token
 * @return 0 on success, non-zero on failure
 */
int db_auth_delete_session(const char *token);

/**
 * @brief Delete all sessions for a user
 * 
 * @param user_id User ID
 * @return 0 on success, non-zero on failure
 */
int db_auth_delete_user_sessions(int64_t user_id);

/**
 * @brief Clean up expired sessions
 * 
 * @return Number of sessions deleted, or negative on failure
 */
int db_auth_cleanup_sessions(void);

/**
 * @brief Get the role name for a role ID
 * 
 * @param role Role ID
 * @return Role name string
 */
const char *db_auth_get_role_name(user_role_t role);

/**
 * @brief Get the role ID for a role name
 * 
 * @param role_name Role name
 * @return Role ID, or -1 if not found
 */
int db_auth_get_role_id(const char *role_name);

#endif /* LIGHTNVR_DB_AUTH_H */
