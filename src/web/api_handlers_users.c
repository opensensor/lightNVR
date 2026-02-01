#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <sqlite3.h>
#include <cjson/cJSON.h>

#include "web/api_handlers_users.h"
#include "web/api_handlers_common.h"
#include "web/api_handlers.h"
#include "web/mongoose_adapter.h"
#include "core/logger.h"
#include "database/db_auth.h"
#include "database/db_core.h"
#include "mongoose.h"
#include "web/mongoose_server_multithreading.h"

// Task structure for user API handlers
typedef struct {
    struct mg_connection *connection;
    struct mg_http_message *message;
    char *uri_copy;
    char *query_copy;
    char *body_copy;
} user_api_task_t;

// Forward declaration of task functions
static void users_create_task_function(void *arg);
static void users_update_task_function(void *arg);
static void users_delete_task_function(void *arg);
static void users_generate_api_key_task_function(void *arg);

// Forward declaration of handler functions
static void users_update_handler(struct mg_connection *c, struct mg_http_message *hm);
static void users_delete_handler(struct mg_connection *c, struct mg_http_message *hm);
static void users_generate_api_key_handler(struct mg_connection *c, struct mg_http_message *hm);

// Helper function to create a user API task
static user_api_task_t *create_user_api_task(struct mg_connection *c, struct mg_http_message *hm) {
    user_api_task_t *task = calloc(1, sizeof(user_api_task_t));
    if (!task) {
        log_error("Failed to allocate memory for user API task");
        return NULL;
    }

    task->connection = c;
    task->message = calloc(1, sizeof(struct mg_http_message));
    if (!task->message) {
        log_error("Failed to allocate memory for HTTP message");
        free(task);
        return NULL;
    }

    // Copy the HTTP message
    memcpy(task->message, hm, sizeof(struct mg_http_message));

    // Copy the URI
    if (hm->uri.len > 0) {
        task->uri_copy = calloc(1, hm->uri.len + 1);
        if (task->uri_copy) {
            memcpy(task->uri_copy, mg_str_get_ptr(&hm->uri), hm->uri.len);
            task->message->uri.buf = task->uri_copy;
        }
    }

    // Copy the query string
    if (hm->query.len > 0) {
        task->query_copy = calloc(1, hm->query.len + 1);
        if (task->query_copy) {
            memcpy(task->query_copy, mg_str_get_ptr(&hm->query), hm->query.len);
            task->message->query.buf = task->query_copy;
        }
    }

    // Copy the body
    if (hm->body.len > 0) {
        task->body_copy = calloc(1, hm->body.len + 1);
        if (task->body_copy) {
            memcpy(task->body_copy, mg_str_get_ptr(&hm->body), hm->body.len);
            task->message->body.buf = task->body_copy;
        }
    }

    return task;
}

// Helper function to free a user API task
static void free_user_api_task(user_api_task_t *task) {
    if (task) {
        if (task->uri_copy) free(task->uri_copy);
        if (task->query_copy) free(task->query_copy);
        if (task->body_copy) free(task->body_copy);
        if (task->message) free(task->message);
        free(task);
    }
}

/**
 * @brief Get the authenticated user from the request
 *
 * @param hm Mongoose HTTP message
 * @param user Pointer to store the user information
 * @return 1 if the user is authenticated, 0 otherwise
 */
static int get_authenticated_user(struct mg_http_message *hm, user_t *user) {
    if (!user) {
        return 0;
    }

    // First, check for session token in cookie
    // Note: mg_http_get_var uses '&' separator which doesn't work for cookies (use ';')
    // So we manually parse the cookie header
    struct mg_str *cookie = mg_http_get_header(hm, "Cookie");
    if (cookie) {
        char cookie_str[1024] = {0};
        if (cookie->len < sizeof(cookie_str) - 1) {
            memcpy(cookie_str, cookie->buf, cookie->len);
            cookie_str[cookie->len] = '\0';

            // Look for session cookie
            char *session_start = strstr(cookie_str, "session=");
            if (session_start) {
                session_start += 8; // Skip "session="
                char *session_end = strchr(session_start, ';');
                if (!session_end) {
                    session_end = session_start + strlen(session_start);
                }

                // Extract session token
                size_t token_len = session_end - session_start;
                char session_token[64] = {0};
                if (token_len < sizeof(session_token) - 1) {
                    memcpy(session_token, session_start, token_len);
                    session_token[token_len] = '\0';

                    // Validate the session token
                    int64_t user_id;
                    int rc = db_auth_validate_session(session_token, &user_id);
                    if (rc == 0) {
                        // Session is valid, get user info
                        rc = db_auth_get_user_by_id(user_id, user);
                        if (rc == 0) {
                            return 1;
                        }
                    }
                }
            }
        }
    }

    // If no valid session, try HTTP Basic Auth
    char username[64] = {0};
    char password[64] = {0};

    mg_http_creds(hm, username, sizeof(username), password, sizeof(password));

    // Check if we have credentials
    if (username[0] != '\0' && password[0] != '\0') {
        // Authenticate the user
        int64_t user_id;
        int rc = db_auth_authenticate(username, password, &user_id);

        if (rc == 0) {
            // Authentication successful, get user info
            rc = db_auth_get_user_by_id(user_id, user);
            if (rc == 0) {
                return 1;
            }
        }
    }

    return 0;
}

/**
 * @brief Check if the user has admin privileges and send appropriate error response if not
 *
 * @param c Mongoose connection (used to send error response)
 * @param hm Mongoose HTTP message
 * @return 1 if the user is an admin, 0 otherwise (error response already sent)
 */
static int check_admin_privileges(struct mg_connection *c, struct mg_http_message *hm) {
    user_t user;
    if (get_authenticated_user(hm, &user)) {
        if (user.role == USER_ROLE_ADMIN) {
            return 1;
        }
        // User is authenticated but not admin
        log_warn("Access denied: User '%s' (role: %s) attempted admin action",
                 user.username, db_auth_get_role_name(user.role));
        mg_send_json_error(c, 403, "Forbidden: Admin privileges required");
        return 0;
    }
    // User is not authenticated
    log_warn("Access denied: Unauthenticated request attempted admin action");
    mg_send_json_error(c, 401, "Unauthorized: Authentication required");
    return 0;
}

/**
 * @brief Check if the user has permission to view users and send appropriate error response if not
 *
 * @param c Mongoose connection (used to send error response)
 * @param hm Mongoose HTTP message
 * @return 1 if the user has permission, 0 otherwise (error response already sent)
 */
static int check_view_users_permission(struct mg_connection *c, struct mg_http_message *hm) {
    user_t user;
    if (get_authenticated_user(hm, &user)) {
        // Only admin and regular users can view users, viewers cannot
        if (user.role == USER_ROLE_ADMIN || user.role == USER_ROLE_USER) {
            return 1;
        }
        // User is authenticated but doesn't have permission
        log_warn("Access denied: User '%s' (role: %s) cannot view users",
                 user.username, db_auth_get_role_name(user.role));
        mg_send_json_error(c, 403, "Forbidden: Insufficient privileges to view users");
        return 0;
    }
    // User is not authenticated
    log_warn("Access denied: Unauthenticated request attempted to view users");
    mg_send_json_error(c, 401, "Unauthorized: Authentication required");
    return 0;
}

/**
 * @brief Check if the user has permission to generate API key and send appropriate error response if not
 *
 * @param c Mongoose connection (used to send error response)
 * @param hm Mongoose HTTP message
 * @param target_user_id ID of the user for whom the API key is being generated
 * @return 1 if the user has permission, 0 otherwise (error response already sent)
 */
static int check_generate_api_key_permission(struct mg_connection *c, struct mg_http_message *hm, int64_t target_user_id) {
    user_t user;
    if (get_authenticated_user(hm, &user)) {
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
        mg_send_json_error(c, 403, "Forbidden: You can only generate API keys for yourself unless you are an admin");
        return 0;
    }
    // User is not authenticated
    log_warn("Access denied: Unauthenticated request attempted to generate API key");
    mg_send_json_error(c, 401, "Unauthorized: Authentication required");
    return 0;
}

/**
 * @brief Check if the user has permission to delete a user and send appropriate error response if not
 *
 * @param c Mongoose connection (used to send error response)
 * @param hm Mongoose HTTP message
 * @param target_user_id ID of the user being deleted
 * @return 1 if the user has permission, 0 otherwise (error response already sent)
 */
static int check_delete_user_permission(struct mg_connection *c, struct mg_http_message *hm, int64_t target_user_id) {
    user_t user;
    if (get_authenticated_user(hm, &user)) {
        // Only admins can delete users
        if (user.role == USER_ROLE_ADMIN) {
            // Admins cannot delete themselves
            if (user.id != target_user_id) {
                return 1;
            }
            log_warn("Access denied: Admin '%s' attempted to delete themselves", user.username);
            mg_send_json_error(c, 403, "Forbidden: You cannot delete yourself");
            return 0;
        }
        // User doesn't have permission
        log_warn("Access denied: User '%s' (role: %s) cannot delete users",
                 user.username, db_auth_get_role_name(user.role));
        mg_send_json_error(c, 403, "Forbidden: Only admins can delete users");
        return 0;
    }
    // User is not authenticated
    log_warn("Access denied: Unauthenticated request attempted to delete user");
    mg_send_json_error(c, 401, "Unauthorized: Authentication required");
    return 0;
}

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

    return json;
}

/**
 * @brief Handle GET /api/auth/users
 */
void mg_handle_users_list(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/auth/users request");

    // Check if user has admin role
    if (!check_admin_privileges(c, hm)) {
        return;  // Error response already sent by check_admin_privileges
    }

    // Get database handle
    sqlite3 *db = get_db_handle();
    if (!db) {
        mg_send_json_error(c, 500, "Database not initialized");
        return;
    }

    // Query all users
    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db,
                               "SELECT id, username, email, role, api_key, created_at, "
                               "updated_at, last_login, is_active "
                               "FROM users ORDER BY id;",
                               -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        mg_send_json_error(c, 500, "Failed to prepare statement");
        return;
    }

    // Create JSON response
    cJSON *response = cJSON_CreateObject();
    cJSON *users_array = cJSON_CreateArray();

    // Iterate through the results
    while (sqlite3_step(stmt) == SQLITE_ROW) {
        user_t user;

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

        // Add user to array
        cJSON_AddItemToArray(users_array, user_to_json(&user, 1));
    }

    sqlite3_finalize(stmt);

    cJSON_AddItemToObject(response, "users", users_array);

    // Send response
    char *json_str = cJSON_PrintUnformatted(response);
    mg_send_json_response(c, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(response);

    log_info("Successfully handled GET /api/auth/users request");
}

/**
 * @brief Handle GET /api/auth/users/:id
 */
void mg_handle_users_get(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/auth/users/:id request");

    // Check if user has admin role
    if (!check_admin_privileges(c, hm)) {
        return;  // Error response already sent by check_admin_privileges
    }

    // Extract user ID from URL
    char user_id_str[32];
    if (mg_extract_path_param(hm, "/api/auth/users/", user_id_str, sizeof(user_id_str)) != 0) {
        log_error("Failed to extract user ID from URL");
        mg_send_json_error(c, 400, "Invalid request path");
        return;
    }

    // Convert user ID to integer
    int64_t user_id = strtoll(user_id_str, NULL, 10);
    if (user_id <= 0) {
        log_error("Invalid user ID: %s", user_id_str);
        mg_send_json_error(c, 400, "Invalid user ID");
        return;
    }

    // Get database handle
    sqlite3 *db = get_db_handle();
    if (!db) {
        mg_send_json_error(c, 500, "Database not initialized");
        return;
    }

    // Query user by ID
    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db,
                               "SELECT id, username, email, role, api_key, created_at, "
                               "updated_at, last_login, is_active "
                               "FROM users WHERE id = ?;",
                               -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        mg_send_json_error(c, 500, "Failed to prepare statement");
        return;
    }

    sqlite3_bind_int64(stmt, 1, user_id);

    if (sqlite3_step(stmt) != SQLITE_ROW) {
        sqlite3_finalize(stmt);
        log_error("User not found: %lld", (long long)user_id);
        mg_send_json_error(c, 404, "User not found");
        return;
    }

    // Create user object
    user_t user;
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

    sqlite3_finalize(stmt);

    // Convert user to JSON
    cJSON *user_json = user_to_json(&user, 1);

    // Send response
    char *json_str = cJSON_PrintUnformatted(user_json);
    mg_send_json_response(c, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(user_json);

    log_info("Successfully handled GET /api/auth/users/%lld request", (long long)user_id);
}

/**
 * @brief Task function for handling POST /api/auth/users
 * This function is executed in a worker thread
 */
static void users_create_task_function(void *arg) {
    user_api_task_t *task = (user_api_task_t *)arg;
    if (!task) {
        log_error("Invalid user API task");
        return;
    }

    struct mg_connection *c = task->connection;
    struct mg_http_message *hm = task->message;
    cJSON *req = NULL;

    log_info("Processing POST /api/auth/users request in worker thread");

    // Parse JSON request
    req = mg_parse_json_body(hm);
    if (!req) {
        mg_send_json_error(c, 400, "Invalid JSON");
        goto cleanup;
    }

    // Extract fields
    cJSON *username_json = cJSON_GetObjectItem(req, "username");
    cJSON *password_json = cJSON_GetObjectItem(req, "password");
    cJSON *email_json = cJSON_GetObjectItem(req, "email");
    cJSON *role_json = cJSON_GetObjectItem(req, "role");
    cJSON *is_active_json = cJSON_GetObjectItem(req, "is_active");

    // Validate required fields
    if (!username_json || !cJSON_IsString(username_json) ||
        !password_json || !cJSON_IsString(password_json)) {
        mg_send_json_error(c, 400, "Missing required fields: username and password");
        goto cleanup;
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
        mg_send_json_error(c, 400, "Username must be between 3 and 32 characters");
        goto cleanup;
    }

    // Validate password
    if (strlen(password) < 8) {
        mg_send_json_error(c, 400, "Password must be at least 8 characters");
        goto cleanup;
    }

    // Validate role
    if (role < 0 || role > 3) {
        mg_send_json_error(c, 400, "Invalid role");
        goto cleanup;
    }

    // Create the user
    int64_t user_id;
    int rc = db_auth_create_user(username, password, email, role, is_active, &user_id);

    if (rc != 0) {
        mg_send_json_error(c, 500, "Failed to create user");
        goto cleanup;
    }

    // Get the created user
    user_t user;
    rc = db_auth_get_user_by_id(user_id, &user);

    if (rc != 0) {
        mg_send_json_error(c, 500, "User created but failed to retrieve");
        goto cleanup;
    }

    // Create JSON response
    cJSON *response = user_to_json(&user, 0);

    // Send response
    char *json_str = cJSON_PrintUnformatted(response);
    mg_send_json_response(c, 200, json_str);
    free(json_str);
    cJSON_Delete(response);

    log_info("Successfully created user: %s (ID: %lld) in worker thread", username_copy, (long long)user_id);

cleanup:
    if (req) {
        cJSON_Delete(req);
    }

    free_user_api_task(task);
}

/**
 * @brief Handle POST /api/auth/users
 */
void mg_handle_users_create(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/auth/users request");

    // Check if user has admin role
    if (!check_admin_privileges(c, hm)) {
        return;  // Error response already sent by check_admin_privileges
    }

    // Parse JSON from request body
    cJSON *req = mg_parse_json_body(hm);
    if (!req) {
        log_error("Failed to parse user JSON from request body");
        mg_send_json_error(c, 400, "Invalid JSON in request body");
        return;
    }

    // Extract fields
    cJSON *username_json = cJSON_GetObjectItem(req, "username");
    cJSON *password_json = cJSON_GetObjectItem(req, "password");
    cJSON *email_json = cJSON_GetObjectItem(req, "email");
    cJSON *role_json = cJSON_GetObjectItem(req, "role");
    cJSON *is_active_json = cJSON_GetObjectItem(req, "is_active");

    // Validate required fields
    if (!username_json || !cJSON_IsString(username_json) ||
        !password_json || !cJSON_IsString(password_json)) {
        cJSON_Delete(req);
        mg_send_json_error(c, 400, "Missing required fields: username and password");
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
        cJSON_Delete(req);
        mg_send_json_error(c, 400, "Username must be between 3 and 32 characters");
        return;
    }

    // Validate password
    if (strlen(password) < 8) {
        cJSON_Delete(req);
        mg_send_json_error(c, 400, "Password must be at least 8 characters");
        return;
    }

    // Validate role
    if (role < 0 || role > 3) {
        cJSON_Delete(req);
        mg_send_json_error(c, 400, "Invalid role");
        return;
    }

    // Create the user
    int64_t user_id;
    int rc = db_auth_create_user(username, password, email, role, is_active, &user_id);

    cJSON_Delete(req);

    if (rc != 0) {
        mg_send_json_error(c, 500, "Failed to create user");
        return;
    }

    // Get the created user
    user_t user;
    rc = db_auth_get_user_by_id(user_id, &user);

    if (rc != 0) {
        mg_send_json_error(c, 500, "User created but failed to retrieve");
        return;
    }

    // Create JSON response
    cJSON *response = user_to_json(&user, 0);

    // Send response
    char *json_str = cJSON_PrintUnformatted(response);
    mg_send_json_response(c, 200, json_str);
    free(json_str);
    cJSON_Delete(response);

    log_info("Successfully created user: %s (ID: %lld)", username_copy, (long long)user_id);
}

/**
 * @brief Task function for handling PUT /api/auth/users/:id
 * This function is executed in a worker thread
 */
static void users_update_task_function(void *arg) {
    user_api_task_t *task = (user_api_task_t *)arg;
    if (!task) {
        log_error("Invalid user API task");
        return;
    }

    struct mg_connection *c = task->connection;
    struct mg_http_message *hm = task->message;
    cJSON *req = NULL;

    log_info("Processing PUT /api/auth/users/:id request in worker thread");

    // Extract user ID from URL
    char id_str[16] = {0};
    if (mg_extract_path_param(hm, "/api/auth/users/", id_str, sizeof(id_str)) != 0) {
        log_error("Failed to extract user ID from URL");
        mg_send_json_error(c, 400, "Invalid request path");
        goto cleanup;
    }

    int64_t user_id = strtoll(id_str, NULL, 10);

    // Check if the user exists
    user_t user;
    int rc = db_auth_get_user_by_id(user_id, &user);

    if (rc != 0) {
        mg_send_json_error(c, 404, "User not found");
        goto cleanup;
    }

    // Parse JSON request
    req = mg_parse_json_body(hm);
    if (!req) {
        mg_send_json_error(c, 400, "Invalid JSON");
        goto cleanup;
    }

    // Extract fields
    cJSON *password_json = cJSON_GetObjectItem(req, "password");
    cJSON *email_json = cJSON_GetObjectItem(req, "email");
    cJSON *role_json = cJSON_GetObjectItem(req, "role");
    cJSON *is_active_json = cJSON_GetObjectItem(req, "is_active");

    // Update password if provided and not empty
    if (password_json && cJSON_IsString(password_json)) {
        const char *password = password_json->valuestring;

        // Only update if password is not empty
        if (strlen(password) > 0) {
            // Validate password length
            if (strlen(password) < 8) {
                mg_send_json_error(c, 400, "Password must be at least 8 characters");
                goto cleanup;
            }

            rc = db_auth_change_password(user_id, password);
            if (rc != 0) {
                mg_send_json_error(c, 500, "Failed to update password");
                goto cleanup;
            }
        }
    }

    // Update other fields
    const char *email = (email_json && cJSON_IsString(email_json)) ? email_json->valuestring : NULL;
    int role = (role_json && cJSON_IsNumber(role_json)) ? role_json->valueint : -1;
    int is_active = (is_active_json && cJSON_IsBool(is_active_json)) ? cJSON_IsTrue(is_active_json) : -1;

    // Validate role
    if (role >= 0 && (role < 0 || role > 3)) {
        mg_send_json_error(c, 400, "Invalid role");
        goto cleanup;
    }

    rc = db_auth_update_user(user_id, email, role, is_active);

    if (rc != 0) {
        mg_send_json_error(c, 500, "Failed to update user");
        goto cleanup;
    }

    // Get the updated user
    rc = db_auth_get_user_by_id(user_id, &user);

    if (rc != 0) {
        mg_send_json_error(c, 500, "User updated but failed to retrieve");
        goto cleanup;
    }

    // Create JSON response
    cJSON *response = user_to_json(&user, 0);

    // Send response
    char *json_str = cJSON_PrintUnformatted(response);
    mg_send_json_response(c, 200, json_str);
    free(json_str);
    cJSON_Delete(response);

    log_info("Successfully updated user: %s (ID: %lld) in worker thread", user.username, (long long)user_id);

cleanup:
    if (req) {
        cJSON_Delete(req);
    }

    free_user_api_task(task);
}

/**
 * @brief Handler function for updating users
 *
 * This function is called by the multithreading system.
 *
 * @param c Mongoose connection
 * @param hm HTTP message
 */
static void users_update_handler(struct mg_connection *c, struct mg_http_message *hm) {
    // Create task
    user_api_task_t *task = create_user_api_task(c, hm);
    if (!task) {
        log_error("Failed to create user API task");
        mg_send_json_error(c, 500, "Failed to create user API task");
        return;
    }

    // Call the task function directly
    users_update_task_function(task);
}

/**
 * @brief Handle PUT /api/auth/users/:id
 */
void mg_handle_users_update(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling PUT /api/auth/users/:id request");

    // Check if user has admin role
    if (!check_admin_privileges(c, hm)) {
        return;  // Error response already sent by check_admin_privileges
    }

    // Send an immediate response to the client before processing the request
    mg_send_json_response(c, 202, "{\"success\":true,\"message\":\"Processing request\"}");

    // Create a thread data structure
    struct mg_thread_data *data = calloc(1, sizeof(struct mg_thread_data));
    if (!data) {
        log_error("Failed to allocate memory for thread data");
        mg_http_reply(c, 500, "", "Internal Server Error\n");
        return;
    }

    // Copy the HTTP message
    data->message = mg_strdup(hm->message);
    if (data->message.len == 0) {
        log_error("Failed to duplicate HTTP message");
        free(data);
        return;
    }

    // Set connection ID, manager, and handler function
    data->conn_id = c->id;
    data->mgr = c->mgr;
    data->handler_func = users_update_handler;

    // Start thread
    mg_start_thread(mg_thread_function, data);

    log_info("User update task started in a worker thread");
}

/**
 * @brief Task function for handling DELETE /api/auth/users/:id
 * This function is executed in a worker thread
 */
static void users_delete_task_function(void *arg) {
    user_api_task_t *task = (user_api_task_t *)arg;
    if (!task) {
        log_error("Invalid user API task");
        return;
    }

    struct mg_connection *c = task->connection;
    struct mg_http_message *hm = task->message;

    log_info("Processing DELETE /api/auth/users/:id request in worker thread");

    // Extract user ID from URL
    char id_str[16] = {0};
    if (mg_extract_path_param(hm, "/api/auth/users/", id_str, sizeof(id_str)) != 0) {
        log_error("Failed to extract user ID from URL");
        mg_send_json_error(c, 400, "Invalid request path");
        goto cleanup;
    }

    int64_t user_id = strtoll(id_str, NULL, 10);

    // Check if the user exists
    user_t user;
    int rc = db_auth_get_user_by_id(user_id, &user);

    if (rc != 0) {
        mg_send_json_error(c, 404, "User not found");
        goto cleanup;
    }

    // Don't allow deleting the last admin user
    if (user.role == USER_ROLE_ADMIN) {
        sqlite3 *db = get_db_handle();
        if (!db) {
            mg_send_json_error(c, 500, "Database not initialized");
            goto cleanup;
        }

        // Count admin users
        sqlite3_stmt *stmt;
        rc = sqlite3_prepare_v2(db, "SELECT COUNT(*) FROM users WHERE role = 0;", -1, &stmt, NULL);
        if (rc != SQLITE_OK) {
            mg_send_json_error(c, 500, "Failed to prepare statement");
            goto cleanup;
        }

        if (sqlite3_step(stmt) == SQLITE_ROW) {
            int admin_count = sqlite3_column_int(stmt, 0);
            sqlite3_finalize(stmt);

            if (admin_count <= 1) {
                mg_send_json_error(c, 400, "Cannot delete the last admin user");
                goto cleanup;
            }
        } else {
            sqlite3_finalize(stmt);
            mg_send_json_error(c, 500, "Failed to count admin users");
            goto cleanup;
        }
    }

    // Delete the user
    rc = db_auth_delete_user(user_id);

    if (rc != 0) {
        mg_send_json_error(c, 500, "Failed to delete user");
        goto cleanup;
    }

    // Create JSON response
    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", 1);
    cJSON_AddStringToObject(response, "message", "User deleted successfully");

    // Send response
    char *json_str = cJSON_PrintUnformatted(response);
    mg_send_json_response(c, 200, json_str);
    free(json_str);
    cJSON_Delete(response);

    log_info("Successfully deleted user: %s (ID: %lld) in worker thread", user.username, (long long)user_id);

cleanup:
    free_user_api_task(task);
}

/**
 * @brief Handler function for deleting users
 *
 * This function is called by the multithreading system.
 *
 * @param c Mongoose connection
 * @param hm HTTP message
 */
static void users_delete_handler(struct mg_connection *c, struct mg_http_message *hm) {
    // Create task
    user_api_task_t *task = create_user_api_task(c, hm);
    if (!task) {
        log_error("Failed to create user API task");
        mg_send_json_error(c, 500, "Failed to create user API task");
        return;
    }

    // Call the task function directly
    users_delete_task_function(task);
}

/**
 * @brief Handle DELETE /api/auth/users/:id
 */
void mg_handle_users_delete(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling DELETE /api/auth/users/:id request");

    // Check if user has admin role
    if (!check_admin_privileges(c, hm)) {
        return;  // Error response already sent by check_admin_privileges
    }

    // Extract user ID for permission check
    char id_str[16] = {0};
    if (mg_extract_path_param(hm, "/api/auth/users/", id_str, sizeof(id_str)) != 0) {
        log_error("Failed to extract user ID from URL");
        mg_send_json_error(c, 400, "Invalid request path");
        return;
    }

    int64_t user_id = strtoll(id_str, NULL, 10);

    // Check if the user has permission to delete this user (now includes self-delete check)
    if (!check_delete_user_permission(c, hm, user_id)) {
        return;  // Error response already sent by check_delete_user_permission
    }

    // Send an immediate response to the client before processing the request
    mg_send_json_response(c, 202, "{\"success\":true,\"message\":\"Processing request\"}");

    // Create a thread data structure
    struct mg_thread_data *data = calloc(1, sizeof(struct mg_thread_data));
    if (!data) {
        log_error("Failed to allocate memory for thread data");
        mg_http_reply(c, 500, "", "Internal Server Error\n");
        return;
    }

    // Copy the HTTP message
    data->message = mg_strdup(hm->message);
    if (data->message.len == 0) {
        log_error("Failed to duplicate HTTP message");
        free(data);
        return;
    }

    // Set connection ID, manager, and handler function
    data->conn_id = c->id;
    data->mgr = c->mgr;
    data->handler_func = users_delete_handler;

    // Start thread
    mg_start_thread(mg_thread_function, data);

    log_info("User delete task started in a worker thread");
}

/**
 * @brief Task function for handling POST /api/auth/users/:id/api-key
 * This function is executed in a worker thread
 */
static void users_generate_api_key_task_function(void *arg) {
    user_api_task_t *task = (user_api_task_t *)arg;
    if (!task) {
        log_error("Invalid user API task");
        return;
    }

    struct mg_connection *c = task->connection;
    struct mg_http_message *hm = task->message;

    log_info("Processing POST /api/auth/users/:id/api-key request in worker thread");

    // Extract user ID from URL
    char id_str[16] = {0};
    if (mg_extract_path_param(hm, "/api/auth/users/", id_str, sizeof(id_str)) != 0) {
        log_error("Failed to extract user ID from URL");
        mg_send_json_error(c, 400, "Invalid request path");
        goto cleanup;
    }

    int64_t user_id = strtoll(id_str, NULL, 10);

    // Check if the user exists
    user_t user;
    int rc = db_auth_get_user_by_id(user_id, &user);

    if (rc != 0) {
        mg_send_json_error(c, 404, "User not found");
        goto cleanup;
    }

    // Generate a new API key
    char api_key[64] = {0};
    rc = db_auth_generate_api_key(user_id, api_key, sizeof(api_key));

    if (rc != 0) {
        mg_send_json_error(c, 500, "Failed to generate API key");
        goto cleanup;
    }

    // Create JSON response
    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", 1);
    cJSON_AddStringToObject(response, "api_key", api_key);

    // Send response
    char *json_str = cJSON_PrintUnformatted(response);
    mg_send_json_response(c, 200, json_str);
    free(json_str);
    cJSON_Delete(response);

    log_info("Successfully generated API key for user: %s (ID: %lld) in worker thread", user.username, (long long)user_id);

cleanup:
    free_user_api_task(task);
}

/**
 * @brief Handler function for generating API keys
 *
 * This function is called by the multithreading system.
 *
 * @param c Mongoose connection
 * @param hm HTTP message
 */
static void users_generate_api_key_handler(struct mg_connection *c, struct mg_http_message *hm) {
    // Create task
    user_api_task_t *task = create_user_api_task(c, hm);
    if (!task) {
        log_error("Failed to create user API task");
        mg_send_json_error(c, 500, "Failed to create user API task");
        return;
    }

    // Call the task function directly
    users_generate_api_key_task_function(task);
}

/**
 * @brief Handle POST /api/auth/users/:id/api-key
 */
void mg_handle_users_generate_api_key(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/auth/users/:id/api-key request");

    // Extract user ID from URL
    char id_str[16] = {0};
    if (mg_extract_path_param(hm, "/api/auth/users/", id_str, sizeof(id_str)) != 0) {
        log_error("Failed to extract user ID from URL");
        mg_send_json_error(c, 400, "Invalid request path");
        return;
    }

    int64_t user_id = strtoll(id_str, NULL, 10);

    // Check if the user has permission to generate API key for this user
    if (!check_generate_api_key_permission(c, hm, user_id)) {
        return;  // Error response already sent by check_generate_api_key_permission
    }

    // Create a user API task and process it directly
    user_api_task_t *task = create_user_api_task(c, hm);
    if (!task) {
        log_error("Failed to create user API task");
        mg_send_json_error(c, 500, "Failed to create user API task");
        return;
    }

    // Process the task directly instead of in a worker thread
    users_generate_api_key_task_function(task);

    log_info("User generate API key task completed");
}
