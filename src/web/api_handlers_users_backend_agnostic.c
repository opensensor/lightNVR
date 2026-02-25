#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <sqlite3.h>
#include <cjson/cJSON.h>

#include "web/api_handlers_users.h"
#include "web/httpd_utils.h"
#include "web/request_response.h"
#include "core/logger.h"
#include "database/db_auth.h"
#include "database/db_core.h"
#include "database/db_schema_cache.h"

/**
 * @brief Convert a user_t struct to a cJSON object
 *
 * @param user User struct
 * @param include_api_key Whether to include the API key in the response
 * @return cJSON* JSON object representing the user
 */
static cJSON *user_to_json(const user_t *user, int include_api_key) {
    cJSON *json = cJSON_CreateObject();

    cJSON_AddNumberToObject(json, "id", user->id);
    cJSON_AddStringToObject(json, "username", user->username);
    cJSON_AddStringToObject(json, "email", user->email);
    cJSON_AddNumberToObject(json, "role", user->role);
    cJSON_AddStringToObject(json, "role_name", db_auth_get_role_name(user->role));

    if (include_api_key && user->api_key[0] != '\0') {
        cJSON_AddStringToObject(json, "api_key", user->api_key);
    }

    cJSON_AddNumberToObject(json, "created_at", user->created_at);
    cJSON_AddNumberToObject(json, "updated_at", user->updated_at);
    cJSON_AddNumberToObject(json, "last_login", user->last_login);
    cJSON_AddBoolToObject(json, "is_active", user->is_active);
    cJSON_AddBoolToObject(json, "password_change_locked", user->password_change_locked);
    cJSON_AddBoolToObject(json, "totp_enabled", user->totp_enabled);

    // Tag-based RBAC: include allowed_tags (null when unrestricted, string when restricted)
    if (user->has_tag_restriction) {
        cJSON_AddStringToObject(json, "allowed_tags", user->allowed_tags);
    } else {
        cJSON_AddNullToObject(json, "allowed_tags");
    }

    return json;
}

/**
 * @brief Check if the user has permission to view users
 *
 * @param req HTTP request
 * @param res HTTP response (error will be set if not permitted)
 * @return 1 if the user has permission, 0 otherwise (error response already set)
 */
static int check_view_users_permission(const http_request_t *req, http_response_t *res) {
    user_t user;
    if (httpd_get_authenticated_user(req, &user)) {
        // Only admin and regular users can view users, viewers cannot
        if (user.role == USER_ROLE_ADMIN || user.role == USER_ROLE_USER) {
            return 1;
        }
        // User is authenticated but doesn't have permission
        log_warn("Access denied: User '%s' (role: %s) cannot view users",
                 user.username, db_auth_get_role_name(user.role));
        http_response_set_json_error(res, 403, "Forbidden: Insufficient privileges to view users");
        return 0;
    }
    // User is not authenticated
    log_warn("Access denied: Unauthenticated request attempted to view users");
    http_response_set_json_error(res, 401, "Unauthorized: Authentication required");
    return 0;
}

/**
 * @brief Check if the user has permission to generate API key
 *
 * @param req HTTP request
 * @param res HTTP response (error will be set if not permitted)
 * @param target_user_id ID of the user for whom the API key is being generated
 * @return 1 if the user has permission, 0 otherwise (error response already set)
 */
static int check_generate_api_key_permission(const http_request_t *req, http_response_t *res, int64_t target_user_id) {
    user_t user;
    if (httpd_get_authenticated_user(req, &user)) {
        // Admins can generate API keys for any user
        if (user.role == USER_ROLE_ADMIN) {
            return 1;
        }

        // Regular users can only generate API keys for themselves
        if (user.role == USER_ROLE_USER && user.id == target_user_id) {
            return 1;
        }

        // User doesn't have permission
        log_warn("Access denied: User '%s' (role: %s) cannot generate API key for user ID %lld",
                 user.username, db_auth_get_role_name(user.role), (long long)target_user_id);
        http_response_set_json_error(res, 403, "Forbidden: You can only generate API keys for yourself unless you are an admin");
        return 0;
    }
    // User is not authenticated
    log_warn("Access denied: Unauthenticated request attempted to generate API key");
    http_response_set_json_error(res, 401, "Unauthorized: Authentication required");
    return 0;
}

/**
 * @brief Check if the user has permission to delete a user
 *
 * @param req HTTP request
 * @param res HTTP response (error will be set if not permitted)
 * @param target_user_id ID of the user being deleted
 * @return 1 if the user has permission, 0 otherwise (error response already set)
 */
static int check_delete_user_permission(const http_request_t *req, http_response_t *res, int64_t target_user_id) {
    user_t user;
    if (httpd_get_authenticated_user(req, &user)) {
        // Only admins can delete users
        if (user.role == USER_ROLE_ADMIN) {
            // Admins cannot delete themselves
            if (user.id != target_user_id) {
                return 1;
            }
            log_warn("Access denied: Admin '%s' attempted to delete themselves", user.username);
            http_response_set_json_error(res, 403, "Forbidden: You cannot delete yourself");
            return 0;
        }
        // User doesn't have permission
        log_warn("Access denied: User '%s' (role: %s) cannot delete users",
                 user.username, db_auth_get_role_name(user.role));
        http_response_set_json_error(res, 403, "Forbidden: Only admins can delete users");
        return 0;
    }
    // User is not authenticated
    log_warn("Access denied: Unauthenticated request attempted to delete user");
    http_response_set_json_error(res, 401, "Unauthorized: Authentication required");
    return 0;
}

/**
 * @brief Backend-agnostic handler for GET /api/auth/users
 */
void handle_users_list(const http_request_t *req, http_response_t *res) {
    log_info("Handling GET /api/auth/users request");

    // Check if user has admin role
    if (!httpd_check_admin_privileges(req, res)) {
        return;  // Error response already set by httpd_check_admin_privileges
    }

    // Get database handle
    sqlite3 *db = get_db_handle();
    if (!db) {
        http_response_set_json_error(res, 500, "Database not initialized");
        return;
    }

    // Check if TOTP and allowed_tags columns exist for backward compatibility
    bool has_totp = cached_column_exists("users", "totp_enabled");
    bool has_allowed_tags = cached_column_exists("users", "allowed_tags");

    // Query all users
    sqlite3_stmt *stmt;
    int rc;
    if (has_totp && has_allowed_tags) {
        rc = sqlite3_prepare_v2(db,
                                "SELECT id, username, email, role, api_key, created_at, "
                                "updated_at, last_login, is_active, password_change_locked, totp_enabled, allowed_tags "
                                "FROM users ORDER BY id;",
                                -1, &stmt, NULL);
    } else if (has_totp) {
        rc = sqlite3_prepare_v2(db,
                                "SELECT id, username, email, role, api_key, created_at, "
                                "updated_at, last_login, is_active, password_change_locked, totp_enabled "
                                "FROM users ORDER BY id;",
                                -1, &stmt, NULL);
    } else {
        rc = sqlite3_prepare_v2(db,
                                "SELECT id, username, email, role, api_key, created_at, "
                                "updated_at, last_login, is_active, password_change_locked "
                                "FROM users ORDER BY id;",
                                -1, &stmt, NULL);
    }
    if (rc != SQLITE_OK) {
        http_response_set_json_error(res, 500, "Failed to prepare statement");
        return;
    }

    // Create JSON response
    cJSON *response = cJSON_CreateObject();
    cJSON *users_array = cJSON_CreateArray();

    // Iterate through the results
    while (sqlite3_step(stmt) == SQLITE_ROW) {
        user_t user;
        memset(&user, 0, sizeof(user));

        user.id = sqlite3_column_int64(stmt, 0);
        strncpy(user.username, (const char *)sqlite3_column_text(stmt, 1), sizeof(user.username) - 1);
        user.username[sizeof(user.username) - 1] = '\0';

        const char *email = (const char *)sqlite3_column_text(stmt, 2);
        if (email) {
            strncpy(user.email, email, sizeof(user.email) - 1);
            user.email[sizeof(user.email) - 1] = '\0';
        } else {
            user.email[0] = '\0';
        }

        user.role = (user_role_t)sqlite3_column_int(stmt, 3);

        const char *api_key = (const char *)sqlite3_column_text(stmt, 4);
        if (api_key) {
            strncpy(user.api_key, api_key, sizeof(user.api_key) - 1);
            user.api_key[sizeof(user.api_key) - 1] = '\0';
        } else {
            user.api_key[0] = '\0';
        }

        user.created_at = sqlite3_column_int64(stmt, 5);
        user.updated_at = sqlite3_column_int64(stmt, 6);
        user.last_login = sqlite3_column_int64(stmt, 7);
        user.is_active = sqlite3_column_int(stmt, 8) != 0;
        user.password_change_locked = sqlite3_column_int(stmt, 9) != 0;
        user.totp_enabled = has_totp ? (sqlite3_column_int(stmt, 10) != 0) : false;
        if (has_allowed_tags && has_totp) {
            const char *at = (const char *)sqlite3_column_text(stmt, 11);
            if (at && at[0] != '\0') {
                strncpy(user.allowed_tags, at, sizeof(user.allowed_tags) - 1);
                user.allowed_tags[sizeof(user.allowed_tags) - 1] = '\0';
                user.has_tag_restriction = true;
            } else {
                user.allowed_tags[0] = '\0';
                user.has_tag_restriction = false;
            }
        } else {
            user.allowed_tags[0] = '\0';
            user.has_tag_restriction = false;
        }

        // Add user to array
        cJSON_AddItemToArray(users_array, user_to_json(&user, 1));
    }

    sqlite3_finalize(stmt);

    cJSON_AddItemToObject(response, "users", users_array);

    // Send response
    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(response);

    log_info("Successfully handled GET /api/auth/users request");
}

/**
 * @brief Backend-agnostic handler for GET /api/auth/users/:id
 */
void handle_users_get(const http_request_t *req, http_response_t *res) {
    log_info("Handling GET /api/auth/users/:id request");

    // Check if user has admin role
    if (!httpd_check_admin_privileges(req, res)) {
        return;  // Error response already set by httpd_check_admin_privileges
    }

    // Extract user ID from URL
    char user_id_str[32];
    if (http_request_extract_path_param(req, "/api/auth/users/", user_id_str, sizeof(user_id_str)) != 0) {
        log_error("Failed to extract user ID from URL");
        http_response_set_json_error(res, 400, "Invalid request path");
        return;
    }

    // Convert user ID to integer
    int64_t user_id = strtoll(user_id_str, NULL, 10);
    if (user_id <= 0) {
        log_error("Invalid user ID: %s", user_id_str);
        http_response_set_json_error(res, 400, "Invalid user ID");
        return;
    }

    // Get database handle
    sqlite3 *db = get_db_handle();
    if (!db) {
        http_response_set_json_error(res, 500, "Database not initialized");
        return;
    }

    // Check if TOTP and allowed_tags columns exist for backward compatibility
    bool has_totp = cached_column_exists("users", "totp_enabled");
    bool has_allowed_tags = cached_column_exists("users", "allowed_tags");

    // Query user by ID
    sqlite3_stmt *stmt;
    int rc;
    if (has_totp && has_allowed_tags) {
        rc = sqlite3_prepare_v2(db,
                                "SELECT id, username, email, role, api_key, created_at, "
                                "updated_at, last_login, is_active, password_change_locked, totp_enabled, allowed_tags "
                                "FROM users WHERE id = ?;",
                                -1, &stmt, NULL);
    } else if (has_totp) {
        rc = sqlite3_prepare_v2(db,
                                "SELECT id, username, email, role, api_key, created_at, "
                                "updated_at, last_login, is_active, password_change_locked, totp_enabled "
                                "FROM users WHERE id = ?;",
                                -1, &stmt, NULL);
    } else {
        rc = sqlite3_prepare_v2(db,
                                "SELECT id, username, email, role, api_key, created_at, "
                                "updated_at, last_login, is_active, password_change_locked "
                                "FROM users WHERE id = ?;",
                                -1, &stmt, NULL);
    }
    if (rc != SQLITE_OK) {
        http_response_set_json_error(res, 500, "Failed to prepare statement");
        return;
    }

    sqlite3_bind_int64(stmt, 1, user_id);

    if (sqlite3_step(stmt) != SQLITE_ROW) {
        sqlite3_finalize(stmt);
        log_error("User not found: %lld", (long long)user_id);
        http_response_set_json_error(res, 404, "User not found");
        return;
    }

    // Create user object
    user_t user;
    memset(&user, 0, sizeof(user));
    user.id = sqlite3_column_int64(stmt, 0);
    strncpy(user.username, (const char *)sqlite3_column_text(stmt, 1), sizeof(user.username) - 1);
    user.username[sizeof(user.username) - 1] = '\0';

    const char *email = (const char *)sqlite3_column_text(stmt, 2);
    if (email) {
        strncpy(user.email, email, sizeof(user.email) - 1);
        user.email[sizeof(user.email) - 1] = '\0';
    } else {
        user.email[0] = '\0';
    }

    user.role = (user_role_t)sqlite3_column_int(stmt, 3);

    const char *api_key = (const char *)sqlite3_column_text(stmt, 4);
    if (api_key) {
        strncpy(user.api_key, api_key, sizeof(user.api_key) - 1);
        user.api_key[sizeof(user.api_key) - 1] = '\0';
    } else {
        user.api_key[0] = '\0';
    }

    user.created_at = sqlite3_column_int64(stmt, 5);
    user.updated_at = sqlite3_column_int64(stmt, 6);
    user.last_login = sqlite3_column_int64(stmt, 7);
    user.is_active = sqlite3_column_int(stmt, 8) != 0;
    user.password_change_locked = sqlite3_column_int(stmt, 9) != 0;
    user.totp_enabled = has_totp ? (sqlite3_column_int(stmt, 10) != 0) : false;
    if (has_allowed_tags && has_totp) {
        const char *at = (const char *)sqlite3_column_text(stmt, 11);
        if (at && at[0] != '\0') {
            strncpy(user.allowed_tags, at, sizeof(user.allowed_tags) - 1);
            user.allowed_tags[sizeof(user.allowed_tags) - 1] = '\0';
            user.has_tag_restriction = true;
        } else {
            user.allowed_tags[0] = '\0';
            user.has_tag_restriction = false;
        }
    } else {
        user.allowed_tags[0] = '\0';
        user.has_tag_restriction = false;
    }

    sqlite3_finalize(stmt);

    // Convert user to JSON
    cJSON *user_json = user_to_json(&user, 1);

    // Send response
    char *json_str = cJSON_PrintUnformatted(user_json);
    http_response_set_json(res, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(user_json);

    log_info("Successfully handled GET /api/auth/users/:id request");
}

/**
 * @brief Backend-agnostic handler for POST /api/auth/users
 */
void handle_users_create(const http_request_t *req, http_response_t *res) {
    log_info("Handling POST /api/auth/users request");

    // Check if user has admin role
    if (!httpd_check_admin_privileges(req, res)) {
        return;  // Error response already set by httpd_check_admin_privileges
    }

    // Parse JSON from request body
    cJSON *json_req = httpd_parse_json_body(req);
    if (!json_req) {
        log_error("Failed to parse user JSON from request body");
        http_response_set_json_error(res, 400, "Invalid JSON in request body");
        return;
    }

    // Extract fields
    cJSON *username_json = cJSON_GetObjectItem(json_req, "username");
    cJSON *password_json = cJSON_GetObjectItem(json_req, "password");
    cJSON *email_json = cJSON_GetObjectItem(json_req, "email");
    cJSON *role_json = cJSON_GetObjectItem(json_req, "role");
    cJSON *is_active_json = cJSON_GetObjectItem(json_req, "is_active");
    cJSON *allowed_tags_create_json = cJSON_GetObjectItem(json_req, "allowed_tags");

    // Validate required fields
    if (!username_json || !cJSON_IsString(username_json) ||
        !password_json || !cJSON_IsString(password_json)) {
        cJSON_Delete(json_req);
        http_response_set_json_error(res, 400, "Missing required fields: username and password");
        return;
    }

    // Make a copy of the username to use after the JSON object is freed
    char username_copy[64] = {0};
    const char *username = username_json->valuestring;
    strncpy(username_copy, username, sizeof(username_copy) - 1);

    const char *password = password_json->valuestring;
    const char *email = (email_json && cJSON_IsString(email_json)) ? email_json->valuestring : NULL;
    int role = (role_json && cJSON_IsNumber(role_json)) ? role_json->valueint : USER_ROLE_USER;
    int is_active = (is_active_json && cJSON_IsBool(is_active_json)) ? cJSON_IsTrue(is_active_json) : 1;

    // Validate username
    if (strlen(username) < 3 || strlen(username) > 32) {
        cJSON_Delete(json_req);
        http_response_set_json_error(res, 400, "Username must be between 3 and 32 characters");
        return;
    }

    // Validate password
    if (strlen(password) < 8) {
        cJSON_Delete(json_req);
        http_response_set_json_error(res, 400, "Password must be at least 8 characters");
        return;
    }

    // Validate role
    if (role < 0 || role > 3) {
        cJSON_Delete(json_req);
        http_response_set_json_error(res, 400, "Invalid role");
        return;
    }

    // Extract allowed_tags before freeing JSON
    char allowed_tags_buf[256] = {0};
    bool has_at_create = false;
    bool at_create_is_null = false;
    if (allowed_tags_create_json) {
        if (cJSON_IsNull(allowed_tags_create_json)) {
            has_at_create = true;
            at_create_is_null = true;
        } else if (cJSON_IsString(allowed_tags_create_json)) {
            strncpy(allowed_tags_buf, allowed_tags_create_json->valuestring, sizeof(allowed_tags_buf) - 1);
            has_at_create = true;
        }
    }

    // Create the user
    int64_t user_id;
    int rc = db_auth_create_user(username, password, email, role, is_active, &user_id);

    cJSON_Delete(json_req);

    if (rc != 0) {
        http_response_set_json_error(res, 500, "Failed to create user");
        return;
    }

    // Set allowed_tags if provided
    if (has_at_create) {
        db_auth_set_allowed_tags(user_id, at_create_is_null ? NULL : allowed_tags_buf);
    }

    // Get the created user
    user_t user;
    rc = db_auth_get_user_by_id(user_id, &user);

    if (rc != 0) {
        http_response_set_json_error(res, 500, "User created but failed to retrieve");
        return;
    }

    // Create JSON response
    cJSON *response = user_to_json(&user, 0);

    // Send response
    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(response);

    log_info("Successfully created user: %s", username_copy);
}

/**
 * @brief Backend-agnostic handler for PUT /api/auth/users/:id
 */
void handle_users_update(const http_request_t *req, http_response_t *res) {
    log_info("Handling PUT /api/auth/users/:id request");

    // Check if user has admin role
    if (!httpd_check_admin_privileges(req, res)) {
        return;  // Error response already set by httpd_check_admin_privileges
    }

    // Extract user ID from URL
    char id_str[16] = {0};
    if (http_request_extract_path_param(req, "/api/auth/users/", id_str, sizeof(id_str)) != 0) {
        log_error("Failed to extract user ID from URL");
        http_response_set_json_error(res, 400, "Invalid request path");
        return;
    }

    int64_t user_id = strtoll(id_str, NULL, 10);

    // Check if the user exists
    user_t user;
    int rc = db_auth_get_user_by_id(user_id, &user);

    if (rc != 0) {
        http_response_set_json_error(res, 404, "User not found");
        return;
    }

    // Parse JSON request
    cJSON *json_req = httpd_parse_json_body(req);
    if (!json_req) {
        http_response_set_json_error(res, 400, "Invalid JSON");
        return;
    }

    // Extract fields
    cJSON *password_json = cJSON_GetObjectItem(json_req, "password");
    cJSON *email_json = cJSON_GetObjectItem(json_req, "email");
    cJSON *role_json = cJSON_GetObjectItem(json_req, "role");
    cJSON *is_active_json = cJSON_GetObjectItem(json_req, "is_active");
    cJSON *allowed_tags_json = cJSON_GetObjectItem(json_req, "allowed_tags");

    // Update password if provided and not empty
    if (password_json && cJSON_IsString(password_json)) {
        const char *password = password_json->valuestring;

        // Only update if password is not empty
        if (strlen(password) > 0) {
            // Validate password length
            if (strlen(password) < 8) {
                cJSON_Delete(json_req);
                http_response_set_json_error(res, 400, "Password must be at least 8 characters");
                return;
            }

            rc = db_auth_change_password(user_id, password);
            if (rc == -2) {
                // Password changes are locked for this user
                cJSON_Delete(json_req);
                http_response_set_json_error(res, 403, "Password changes are locked for this user");
                return;
            } else if (rc != 0) {
                cJSON_Delete(json_req);
                http_response_set_json_error(res, 500, "Failed to update password");
                return;
            }
        }
    }

    // Update other fields
    const char *email = (email_json && cJSON_IsString(email_json)) ? email_json->valuestring : NULL;
    int role = (role_json && cJSON_IsNumber(role_json)) ? role_json->valueint : -1;
    int is_active = (is_active_json && cJSON_IsBool(is_active_json)) ? cJSON_IsTrue(is_active_json) : -1;

    // Validate role
    if (role >= 0 && (role < 0 || role > 3)) {
        cJSON_Delete(json_req);
        http_response_set_json_error(res, 400, "Invalid role");
        return;
    }

    rc = db_auth_update_user(user_id, email, role, is_active);

    if (rc == 0 && allowed_tags_json) {
        // allowed_tags: JSON null removes restriction; string sets it
        const char *at = NULL;
        bool set_tags = false;
        if (cJSON_IsNull(allowed_tags_json)) {
            at = NULL;    // Remove restriction
            set_tags = true;
        } else if (cJSON_IsString(allowed_tags_json)) {
            at = allowed_tags_json->valuestring;
            set_tags = true;
        }
        if (set_tags) {
            db_auth_set_allowed_tags(user_id, at);
        }
    }

    cJSON_Delete(json_req);

    if (rc != 0) {
        http_response_set_json_error(res, 500, "Failed to update user");
        return;
    }

    // Get the updated user
    rc = db_auth_get_user_by_id(user_id, &user);

    if (rc != 0) {
        http_response_set_json_error(res, 500, "User updated but failed to retrieve");
        return;
    }

    // Create JSON response
    cJSON *response = user_to_json(&user, 0);

    // Send response
    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(response);

    log_info("Successfully updated user: %s (ID: %lld)", user.username, (long long)user_id);
}

/**
 * @brief Backend-agnostic handler for DELETE /api/auth/users/:id
 */
void handle_users_delete(const http_request_t *req, http_response_t *res) {
    log_info("Handling DELETE /api/auth/users/:id request");

    // Extract user ID from URL
    char id_str[16] = {0};
    if (http_request_extract_path_param(req, "/api/auth/users/", id_str, sizeof(id_str)) != 0) {
        log_error("Failed to extract user ID from URL");
        http_response_set_json_error(res, 400, "Invalid request path");
        return;
    }

    int64_t user_id = strtoll(id_str, NULL, 10);

    // Check if the user has permission to delete this user (includes self-delete check)
    if (!check_delete_user_permission(req, res, user_id)) {
        return;  // Error response already set by check_delete_user_permission
    }

    // Check if the user exists
    user_t user;
    int rc = db_auth_get_user_by_id(user_id, &user);

    if (rc != 0) {
        http_response_set_json_error(res, 404, "User not found");
        return;
    }

    // Don't allow deleting the last admin user
    if (user.role == USER_ROLE_ADMIN) {
        sqlite3 *db = get_db_handle();
        if (!db) {
            http_response_set_json_error(res, 500, "Database not initialized");
            return;
        }

        // Count admin users
        sqlite3_stmt *stmt;
        rc = sqlite3_prepare_v2(db, "SELECT COUNT(*) FROM users WHERE role = 0;", -1, &stmt, NULL);
        if (rc != SQLITE_OK) {
            http_response_set_json_error(res, 500, "Failed to prepare statement");
            return;
        }

        if (sqlite3_step(stmt) == SQLITE_ROW) {
            int admin_count = sqlite3_column_int(stmt, 0);
            sqlite3_finalize(stmt);

            if (admin_count <= 1) {
                http_response_set_json_error(res, 400, "Cannot delete the last admin user");
                return;
            }
        } else {
            sqlite3_finalize(stmt);
            http_response_set_json_error(res, 500, "Failed to count admin users");
            return;
        }
    }

    // Delete the user
    rc = db_auth_delete_user(user_id);

    if (rc != 0) {
        http_response_set_json_error(res, 500, "Failed to delete user");
        return;
    }

    // Create JSON response
    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", 1);
    cJSON_AddStringToObject(response, "message", "User deleted successfully");

    // Send response
    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(response);

    log_info("Successfully deleted user ID: %lld", (long long)user_id);
}

/**
 * @brief Backend-agnostic handler for POST /api/auth/users/:id/api-key
 */
void handle_users_generate_api_key(const http_request_t *req, http_response_t *res) {
    log_info("Handling POST /api/auth/users/:id/api-key request");

    // Extract user ID from URL
    char id_str[16] = {0};
    if (http_request_extract_path_param(req, "/api/auth/users/", id_str, sizeof(id_str)) != 0) {
        log_error("Failed to extract user ID from URL");
        http_response_set_json_error(res, 400, "Invalid request path");
        return;
    }

    int64_t user_id = strtoll(id_str, NULL, 10);

    // Check if the user has permission to generate API key for this user
    if (!check_generate_api_key_permission(req, res, user_id)) {
        return;  // Error response already set by check_generate_api_key_permission
    }

    // Check if the user exists
    user_t user;
    int rc = db_auth_get_user_by_id(user_id, &user);

    if (rc != 0) {
        http_response_set_json_error(res, 404, "User not found");
        return;
    }

    // Generate a new API key
    char api_key[64] = {0};
    rc = db_auth_generate_api_key(user_id, api_key, sizeof(api_key));

    if (rc != 0) {
        http_response_set_json_error(res, 500, "Failed to generate API key");
        return;
    }

    // Create JSON response
    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", 1);
    cJSON_AddStringToObject(response, "api_key", api_key);

    // Send response
    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(response);

    log_info("Successfully generated API key for user ID: %lld", (long long)user_id);
}

/**
 * @brief Backend-agnostic handler for PUT /api/auth/users/:id/password
 */
void handle_users_change_password(const http_request_t *req, http_response_t *res) {
    log_info("Handling PUT /api/auth/users/:id/password request");

    // Get the current user from session
    user_t current_user;
    if (!httpd_get_authenticated_user(req, &current_user)) {
        http_response_set_json_error(res, 401, "Unauthorized");
        return;
    }

    // Extract user ID from URL
    char id_str[16] = {0};
    if (http_request_extract_path_param(req, "/api/auth/users/", id_str, sizeof(id_str)) != 0) {
        log_error("Failed to extract user ID from URL");
        http_response_set_json_error(res, 400, "Invalid request path");
        return;
    }

    int64_t target_user_id = strtoll(id_str, NULL, 10);

    // Check if the target user exists
    user_t target_user;
    int rc = db_auth_get_user_by_id(target_user_id, &target_user);
    if (rc != 0) {
        http_response_set_json_error(res, 404, "User not found");
        return;
    }

    // Check permissions:
    // - Admins can change any user's password
    // - Non-admins can only change their own password
    bool is_admin = (current_user.role == USER_ROLE_ADMIN);
    bool is_own_password = (current_user.id == target_user_id);

    if (!is_admin && !is_own_password) {
        http_response_set_json_error(res, 403, "You can only change your own password");
        return;
    }

    // Parse JSON request
    cJSON *json_req = httpd_parse_json_body(req);
    if (!json_req) {
        http_response_set_json_error(res, 400, "Invalid JSON");
        return;
    }

    // Extract fields
    cJSON *old_password_json = cJSON_GetObjectItem(json_req, "old_password");
    cJSON *new_password_json = cJSON_GetObjectItem(json_req, "new_password");

    if (!new_password_json || !cJSON_IsString(new_password_json)) {
        cJSON_Delete(json_req);
        http_response_set_json_error(res, 400, "New password is required");
        return;
    }

    const char *new_password = new_password_json->valuestring;

    // Validate new password length
    if (strlen(new_password) < 8) {
        cJSON_Delete(json_req);
        http_response_set_json_error(res, 400, "Password must be at least 8 characters");
        return;
    }

    // Non-admins must provide old password
    if (!is_admin) {
        if (!old_password_json || !cJSON_IsString(old_password_json)) {
            cJSON_Delete(json_req);
            http_response_set_json_error(res, 400, "Current password is required");
            return;
        }

        const char *old_password = old_password_json->valuestring;

        // Verify old password
        rc = db_auth_verify_password(target_user_id, old_password);
        if (rc != 0) {
            cJSON_Delete(json_req);
            http_response_set_json_error(res, 401, "Current password is incorrect");
            return;
        }
    }

    // Change the password
    rc = db_auth_change_password(target_user_id, new_password);
    if (rc == -2) {
        // Password changes are locked for this user
        cJSON_Delete(json_req);
        http_response_set_json_error(res, 403, "Password changes are locked for this user");
        return;
    } else if (rc != 0) {
        cJSON_Delete(json_req);
        http_response_set_json_error(res, 500, "Failed to change password");
        return;
    }

    cJSON_Delete(json_req);

    // Create JSON response
    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);

    // Send response
    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(response);

    log_info("Successfully changed password for user ID: %lld", (long long)target_user_id);
}

/**
 * @brief Backend-agnostic handler for PUT /api/auth/users/:id/password-lock
 */
void handle_users_password_lock(const http_request_t *req, http_response_t *res) {
    log_info("Handling PUT /api/auth/users/:id/password-lock request");

    // Check if user has admin role
    if (!httpd_check_admin_privileges(req, res)) {
        return;  // Error response already set by httpd_check_admin_privileges
    }

    // Extract user ID from URL
    char id_str[16] = {0};
    if (http_request_extract_path_param(req, "/api/auth/users/", id_str, sizeof(id_str)) != 0) {
        log_error("Failed to extract user ID from URL");
        http_response_set_json_error(res, 400, "Invalid request path");
        return;
    }

    int64_t user_id = strtoll(id_str, NULL, 10);

    // Check if the user exists
    user_t user;
    int rc = db_auth_get_user_by_id(user_id, &user);
    if (rc != 0) {
        http_response_set_json_error(res, 404, "User not found");
        return;
    }

    // Parse JSON request
    cJSON *json_req = httpd_parse_json_body(req);
    if (!json_req) {
        http_response_set_json_error(res, 400, "Invalid JSON");
        return;
    }

    // Extract locked field
    cJSON *locked_json = cJSON_GetObjectItem(json_req, "locked");
    if (!locked_json || !cJSON_IsBool(locked_json)) {
        cJSON_Delete(json_req);
        http_response_set_json_error(res, 400, "Locked field is required and must be a boolean");
        return;
    }

    bool locked = cJSON_IsTrue(locked_json);

    // Set the password lock status
    rc = db_auth_set_password_lock(user_id, locked);

    cJSON_Delete(json_req);

    if (rc != 0) {
        http_response_set_json_error(res, 500, "Failed to update password lock status");
        return;
    }

    // Get the updated user
    rc = db_auth_get_user_by_id(user_id, &user);
    if (rc != 0) {
        http_response_set_json_error(res, 500, "Password lock updated but failed to retrieve user");
        return;
    }

    // Create JSON response
    cJSON *response = user_to_json(&user, 0);

    // Send response
    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(response);

    log_info("Successfully updated password lock status for user: %s (ID: %lld, locked: %d)",
             user.username, (long long)user_id, locked);
}

