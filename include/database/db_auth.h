/**
 * @file db_auth.h
 * @brief Database authentication functions
 */

#ifndef LIGHTNVR_DB_AUTH_H
#define LIGHTNVR_DB_AUTH_H

#include <stdbool.h>
#include <stdint.h>
#include <stddef.h>

#define USER_ALLOWED_TAGS_MAX 256
#define USER_ALLOWED_LOGIN_CIDRS_MAX 1024

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
    bool password_change_locked; /**< Whether password changes are locked (for demo accounts) */
    bool totp_enabled;       /**< Whether TOTP MFA is enabled */
    char allowed_tags[USER_ALLOWED_TAGS_MAX];  /**< Comma-separated tag whitelist for RBAC (empty = no restriction) */
    bool has_tag_restriction; /**< Whether allowed_tags is set (true) or NULL/unrestricted (false) */
    char allowed_login_cidrs[USER_ALLOWED_LOGIN_CIDRS_MAX]; /**< Newline-separated CIDR whitelist for login/auth IPs */
    bool has_login_cidr_restriction; /**< Whether allowed_login_cidrs is set (true) or NULL/unrestricted (false) */
} user_t;

/**
 * @brief Session structure
 */
typedef struct {
    int64_t id;              /**< Session ID */
    int64_t user_id;         /**< User ID */
    char token[128];         /**< Session token */
    int64_t created_at;      /**< Creation timestamp */
    int64_t last_activity_at;/**< Last authenticated activity timestamp */
    int64_t idle_expires_at; /**< Idle timeout expiry timestamp */
    int64_t expires_at;      /**< Expiration timestamp */
    char ip_address[46];     /**< IP address of the client */
    char user_agent[256];    /**< User agent of the client */
} session_t;

/**
 * @brief Trusted device structure
 */
typedef struct {
    int64_t id;              /**< Trusted device ID */
    int64_t user_id;         /**< User ID */
    char token[128];         /**< Trusted-device token */
    int64_t created_at;      /**< Creation timestamp */
    int64_t last_used_at;    /**< Last successful use */
    int64_t expires_at;      /**< Expiration timestamp */
    char ip_address[46];     /**< Last known IP address */
    char user_agent[256];    /**< Last known user agent */
} trusted_device_t;

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
 * @param username New username (optional, can be NULL to leave unchanged)
 * @param email New email address (optional, can be NULL to leave unchanged)
 * @param role New user role (optional, can be -1 to leave unchanged)
 * @param is_active New active status (optional, can be -1 to leave unchanged)
 * @return 0 on success, non-zero on failure
 */
int db_auth_update_user(int64_t user_id, const char *username, const char *email, int role, int is_active);

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
 * @brief Verify a password for a specific user ID
 *
 * @param user_id User ID
 * @param password Password to verify
 * @return 0 on success, non-zero on failure
 */
int db_auth_verify_password(int64_t user_id, const char *password);

/**
 * @brief Set the password change lock status for a user
 *
 * @param user_id User ID
 * @param locked Whether password changes should be locked
 * @return 0 on success, non-zero on failure
 */
int db_auth_set_password_lock(int64_t user_id, bool locked);

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
 * @param ip_address Current client IP address for session tracking (optional, can be NULL)
 * @param user_agent Current client user agent for session tracking (optional, can be NULL)
 * @return 0 on success, non-zero on failure
 */
int db_auth_validate_session_with_context(const char *token, int64_t *user_id,
                                          const char *ip_address, const char *user_agent);

/**
 * @brief Validate a session token without request context
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
 * @brief List sessions for a user, newest activity first.
 */
int db_auth_list_user_sessions(int64_t user_id, session_t *sessions, int max_count);

/**
 * @brief Delete a single session by ID for a user.
 */
int db_auth_delete_session_by_id(int64_t user_id, int64_t session_id);

/**
 * @brief Create a trusted device token for a user.
 */
int db_auth_create_trusted_device(int64_t user_id, const char *ip_address, const char *user_agent,
                                  int expiry_seconds, char *token, size_t token_size);

/**
 * @brief Validate a trusted device token for a user.
 */
int db_auth_validate_trusted_device(int64_t user_id, const char *token);

/**
 * @brief Resolve the trusted-device ID for a presented token if still valid.
 */
int db_auth_get_trusted_device_id(int64_t user_id, const char *token, int64_t *trusted_device_id);

/**
 * @brief List trusted devices for a user.
 */
int db_auth_list_trusted_devices(int64_t user_id, trusted_device_t *devices, int max_count);

/**
 * @brief Delete a single trusted device by ID for a user.
 */
int db_auth_delete_trusted_device_by_id(int64_t user_id, int64_t trusted_device_id);

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

/**
 * @brief Get TOTP info for a user
 *
 * @param user_id User ID
 * @param secret Buffer to store the base32-encoded TOTP secret
 * @param secret_size Size of the secret buffer
 * @param enabled Pointer to store whether TOTP is enabled
 * @return 0 on success (secret exists), -1 if no secret configured or error
 */
int db_auth_get_totp_info(int64_t user_id, char *secret, size_t secret_size, bool *enabled);

/**
 * @brief Set or clear the TOTP secret for a user
 *
 * @param user_id User ID
 * @param secret Base32-encoded TOTP secret (NULL to clear)
 * @return 0 on success, non-zero on failure
 */
int db_auth_set_totp_secret(int64_t user_id, const char *secret);

/**
 * @brief Enable or disable TOTP for a user
 *
 * @param user_id User ID
 * @param enabled Whether to enable or disable TOTP
 * @return 0 on success, non-zero on failure
 */
int db_auth_enable_totp(int64_t user_id, bool enabled);

/**
 * @brief Set the allowed_tags restriction for a user
 *
 * @param user_id User ID
 * @param allowed_tags Comma-separated tag list, or NULL to remove restriction
 * @return 0 on success, non-zero on failure
 */
int db_auth_set_allowed_tags(int64_t user_id, const char *allowed_tags);

/**
 * @brief Validate a per-user allowed_login_cidrs list.
 *
 * Accepts IPv4/IPv6 CIDR entries or single IP literals separated by commas and/or
 * newlines. Bare IPv4 addresses are normalized to /32, and bare IPv6 addresses to
 * /128. NULL or blank input is treated as unrestricted and therefore valid.
 *
 * @param allowed_login_cidrs CIDR list to validate
 * @return 0 when valid, non-zero when invalid
 */
int db_auth_validate_allowed_login_cidrs(const char *allowed_login_cidrs);

/**
 * @brief Set the allowed_login_cidrs restriction for a user.
 *
 * @param user_id User ID
 * @param allowed_login_cidrs CIDR list, or NULL to remove restriction
 * @return 0 on success, non-zero on failure
 */
int db_auth_set_allowed_login_cidrs(int64_t user_id, const char *allowed_login_cidrs);

/**
 * @brief Check whether a client's IP is allowed for a user.
 *
 * Returns true when the user has no login CIDR restriction, or when the
 * supplied client_ip matches at least one configured CIDR.
 *
 * @param user Pointer to the authenticated user
 * @param client_ip Client IP address string
 * @return true if authentication is permitted from client_ip
 */
bool db_auth_ip_allowed_for_user(const user_t *user, const char *client_ip);

/**
 * @brief Check whether a stream's tags satisfy a user's allowed_tags restriction
 *
 * Returns true when:
 *   - The user has no tag restriction (has_tag_restriction == false), OR
 *   - The stream has at least one tag that appears in the user's allowed_tags list
 *
 * @param user Pointer to the authenticated user
 * @param stream_tags Comma-separated tags from the stream (may be empty)
 * @return true if access is permitted, false otherwise
 */
bool db_auth_stream_allowed_for_user(const user_t *user, const char *stream_tags);

#endif /* LIGHTNVR_DB_AUTH_H */
