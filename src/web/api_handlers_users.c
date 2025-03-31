#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <sqlite3.h>
#include "cJSON.h"

#include "web/api_handlers_users.h"
#include "web/api_handlers_common.h"
#include "web/api_handlers.h"
#include "web/mongoose_adapter.h"
#include "core/logger.h"
#include "database/db_auth.h"
#include "database/db_core.h"
#include "mongoose.h"

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
    struct mg_str *cookie = mg_http_get_header(hm, "Cookie");
    if (cookie) {
        char session_token[64] = {0};
        if (mg_http_get_var(cookie, "session", session_token, sizeof(session_token)) > 0) {
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
 * @brief Check if the user has admin privileges
 * 
 * @param hm Mongoose HTTP message
 * @return 1 if the user is an admin, 0 otherwise
 */
static int check_admin_privileges(struct mg_http_message *hm) {
    user_t user;
    if (get_authenticated_user(hm, &user)) {
        return (user.role == USER_ROLE_ADMIN);
    }
    return 0;
}

/**
 * @brief Check if the user has permission to view users
 * 
 * @param hm Mongoose HTTP message
 * @return 1 if the user has permission, 0 otherwise
 */
static int check_view_users_permission(struct mg_http_message *hm) {
    user_t user;
    if (get_authenticated_user(hm, &user)) {
        // Only admin and regular users can view users, viewers cannot
        return (user.role == USER_ROLE_ADMIN || user.role == USER_ROLE_USER);
    }
    return 0;
}

/**
 * @brief Check if the user has permission to create users
 * 
 * @param hm Mongoose HTTP message
 * @return 1 if the user has permission, 0 otherwise
 */
static int check_create_user_permission(struct mg_http_message *hm) {
    // Only admins can create users
    return check_admin_privileges(hm);
}

/**
 * @brief Check if the user has permission to generate API key
 * 
 * @param hm Mongoose HTTP message
 * @param target_user_id ID of the user for whom the API key is being generated
 * @return 1 if the user has permission, 0 otherwise
 */
static int check_generate_api_key_permission(struct mg_http_message *hm, int64_t target_user_id) {
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
    }
    return 0;
}

/**
 * @brief Check if the user has permission to delete a user
 * 
 * @param hm Mongoose HTTP message
 * @param target_user_id ID of the user being deleted
 * @return 1 if the user has permission, 0 otherwise
 */
static int check_delete_user_permission(struct mg_http_message *hm, int64_t target_user_id) {
    user_t user;
    if (get_authenticated_user(hm, &user)) {
        // Only admins can delete users
        if (user.role == USER_ROLE_ADMIN) {
            // Admins cannot delete themselves
            return (user.id != target_user_id);
        }
    }
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
 * Get a list of all users
 */
void mg_handle_users_list(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/auth/users request");
    
    // Check if the user has permission to view users
    if (!check_view_users_permission(hm)) {
        mg_send_json_error(c, 403, "Forbidden: Only admin and regular users can view users");
        return;
    }
    
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
    mg_printf(c, "HTTP/1.1 200 OK\r\n");
    mg_printf(c, "Content-Type: application/json\r\n");
    mg_printf(c, "Content-Length: %d\r\n\r\n", (int)strlen(json_str));
    mg_printf(c, "%s", json_str);
    free(json_str);
    cJSON_Delete(response);
    
    log_info("Successfully handled GET /api/auth/users request");
}

/**
 * @brief Handle GET /api/auth/users/:id
 * Get a specific user by ID
 */
void mg_handle_users_get(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/auth/users/:id request");
    
    // Check if the user has permission to view users
    if (!check_view_users_permission(hm)) {
        mg_send_json_error(c, 403, "Forbidden: Only admin and regular users can view users");
        return;
    }
    
    // Extract user ID from URL
    char id_str[16] = {0};
    mg_http_get_var(&hm->query, "id", id_str, sizeof(id_str));
    
    if (id_str[0] == '\0') {
        // Try to extract from the URL path
        struct mg_str uri = hm->uri;
        const char *users_path = "/api/auth/users/";
        size_t users_path_len = strlen(users_path);
        
        if (mg_str_get_len(&uri) > users_path_len) {
            const char *id_start = mg_str_get_ptr(&uri) + users_path_len;
            size_t id_len = 0;
            
            // Find the end of the ID (either end of string or next slash)
            while (id_len < mg_str_get_len(&uri) - users_path_len && id_start[id_len] != '/' && id_start[id_len] != '?') {
                id_len++;
            }
            
            if (id_len > 0 && id_len < sizeof(id_str)) {
                strncpy(id_str, id_start, id_len);
                id_str[id_len] = '\0';
            }
        }
    }
    
    if (id_str[0] == '\0') {
        mg_send_json_error(c, 400, "Missing user ID");
        return;
    }
    
    int64_t user_id = strtoll(id_str, NULL, 10);
    
    // Check if the user has permission to generate API key for this user
    if (!check_generate_api_key_permission(hm, user_id)) {
        mg_send_json_error(c, 403, "Forbidden: You can only generate API keys for yourself unless you are an admin");
        return;
    }
    
    // Get the user
    user_t user;
    int rc = db_auth_get_user_by_id(user_id, &user);
    
    if (rc != 0) {
        mg_send_json_error(c, 404, "User not found");
        return;
    }
    
    // Create JSON response
    cJSON *response = user_to_json(&user, 1);
    
    // Send response
    char *json_str = cJSON_PrintUnformatted(response);
    mg_send_json_response(c, 200, json_str);
    free(json_str);
    cJSON_Delete(response);
    
    log_info("Successfully handled GET /api/auth/users/:id request");
}

/**
 * @brief Handle POST /api/auth/users
 * Create a new user
 */
void mg_handle_users_create(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/auth/users request");
    
    // Check if the user has admin privileges
    if (!check_admin_privileges(hm)) {
        mg_send_json_error(c, 403, "Forbidden: Admin privileges required");
        return;
    }
    
    // Parse JSON request
    cJSON *req = mg_parse_json_body(hm);
    if (!req) {
        mg_send_json_error(c, 400, "Invalid JSON");
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
    
    const char *username = username_json->valuestring;
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
    
    log_info("Successfully created user: %s (ID: %lld)", username, (long long)user_id);
}

/**
 * @brief Handle PUT /api/auth/users/:id
 * Update an existing user
 */
void mg_handle_users_update(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling PUT /api/auth/users/:id request");
    
    // Check if the user has admin privileges
    if (!check_admin_privileges(hm)) {
        mg_send_json_error(c, 403, "Forbidden: Admin privileges required");
        return;
    }
    
    // Extract user ID from URL
    char id_str[16] = {0};
    mg_http_get_var(&hm->query, "id", id_str, sizeof(id_str));
    
    if (id_str[0] == '\0') {
        // Try to extract from the URL path
        struct mg_str uri = hm->uri;
        const char *users_path = "/api/auth/users/";
        size_t users_path_len = strlen(users_path);
        
        if (mg_str_get_len(&uri) > users_path_len) {
            const char *id_start = mg_str_get_ptr(&uri) + users_path_len;
            size_t id_len = 0;
            
            // Find the end of the ID (either end of string or next slash)
            while (id_len < mg_str_get_len(&uri) - users_path_len && id_start[id_len] != '/' && id_start[id_len] != '?') {
                id_len++;
            }
            
            if (id_len > 0 && id_len < sizeof(id_str)) {
                strncpy(id_str, id_start, id_len);
                id_str[id_len] = '\0';
            }
        }
    }
    
    if (id_str[0] == '\0') {
        mg_send_json_error(c, 400, "Missing user ID");
        return;
    }
    
    int64_t user_id = strtoll(id_str, NULL, 10);
    
    // Check if the user exists
    user_t user;
    int rc = db_auth_get_user_by_id(user_id, &user);
    
    if (rc != 0) {
        mg_send_json_error(c, 404, "User not found");
        return;
    }
    
    // Parse JSON request
    cJSON *req = mg_parse_json_body(hm);
    if (!req) {
        mg_send_json_error(c, 400, "Invalid JSON");
        return;
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
                cJSON_Delete(req);
                mg_send_json_error(c, 400, "Password must be at least 8 characters");
                return;
            }
            
            rc = db_auth_change_password(user_id, password);
            if (rc != 0) {
                cJSON_Delete(req);
                mg_send_json_error(c, 500, "Failed to update password");
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
        cJSON_Delete(req);
        mg_send_json_error(c, 400, "Invalid role");
        return;
    }
    
    rc = db_auth_update_user(user_id, email, role, is_active);
    
    cJSON_Delete(req);
    
    if (rc != 0) {
        mg_send_json_error(c, 500, "Failed to update user");
        return;
    }
    
    // Get the updated user
    rc = db_auth_get_user_by_id(user_id, &user);
    
    if (rc != 0) {
        mg_send_json_error(c, 500, "User updated but failed to retrieve");
        return;
    }
    
    // Create JSON response
    cJSON *response = user_to_json(&user, 0);
    
    // Send response
    char *json_str = cJSON_PrintUnformatted(response);
    mg_send_json_response(c, 200, json_str);
    free(json_str);
    cJSON_Delete(response);
    
    log_info("Successfully updated user: %s (ID: %lld)", user.username, (long long)user_id);
}

/**
 * @brief Handle DELETE /api/auth/users/:id
 * Delete a user
 */
void mg_handle_users_delete(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling DELETE /api/auth/users/:id request");
    
    // Check if the user has admin privileges
    if (!check_admin_privileges(hm)) {
        mg_send_json_error(c, 403, "Forbidden: Admin privileges required");
        return;
    }
    
    // Extract user ID from URL
    char id_str[16] = {0};
    mg_http_get_var(&hm->query, "id", id_str, sizeof(id_str));
    
    if (id_str[0] == '\0') {
        // Try to extract from the URL path
        struct mg_str uri = hm->uri;
        const char *users_path = "/api/auth/users/";
        size_t users_path_len = strlen(users_path);
        
        if (mg_str_get_len(&uri) > users_path_len) {
            const char *id_start = mg_str_get_ptr(&uri) + users_path_len;
            size_t id_len = 0;
            
            // Find the end of the ID (either end of string or next slash)
            while (id_len < mg_str_get_len(&uri) - users_path_len && id_start[id_len] != '/' && id_start[id_len] != '?') {
                id_len++;
            }
            
            if (id_len > 0 && id_len < sizeof(id_str)) {
                strncpy(id_str, id_start, id_len);
                id_str[id_len] = '\0';
            }
        }
    }
    
    if (id_str[0] == '\0') {
        mg_send_json_error(c, 400, "Missing user ID");
        return;
    }
    
    int64_t user_id = strtoll(id_str, NULL, 10);
    
    // Check if the user exists
    user_t user;
    int rc = db_auth_get_user_by_id(user_id, &user);
    
    if (rc != 0) {
        mg_send_json_error(c, 404, "User not found");
        return;
    }
    
    // Don't allow deleting the last admin user
    if (user.role == USER_ROLE_ADMIN) {
        sqlite3 *db = get_db_handle();
        if (!db) {
            mg_send_json_error(c, 500, "Database not initialized");
            return;
        }
        
        // Count admin users
        sqlite3_stmt *stmt;
        rc = sqlite3_prepare_v2(db, "SELECT COUNT(*) FROM users WHERE role = 0;", -1, &stmt, NULL);
        if (rc != SQLITE_OK) {
            mg_send_json_error(c, 500, "Failed to prepare statement");
            return;
        }
        
        if (sqlite3_step(stmt) == SQLITE_ROW) {
            int admin_count = sqlite3_column_int(stmt, 0);
            sqlite3_finalize(stmt);
            
            if (admin_count <= 1) {
                mg_send_json_error(c, 400, "Cannot delete the last admin user");
                return;
            }
        } else {
            sqlite3_finalize(stmt);
            mg_send_json_error(c, 500, "Failed to count admin users");
            return;
        }
    }
    
    // Delete the user
    rc = db_auth_delete_user(user_id);
    
    if (rc != 0) {
        mg_send_json_error(c, 500, "Failed to delete user");
        return;
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
    
    log_info("Successfully deleted user: %s (ID: %lld)", user.username, (long long)user_id);
}

/**
 * @brief Handle POST /api/auth/users/:id/api-key
 * Generate a new API key for a user
 */
void mg_handle_users_generate_api_key(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/auth/users/:id/api-key request");
    
    // Extract user ID from URL
    char id_str[16] = {0};
    mg_http_get_var(&hm->query, "id", id_str, sizeof(id_str));
    
    if (id_str[0] == '\0') {
        // Try to extract from the URL path
        struct mg_str uri = hm->uri;
        const char *users_path = "/api/auth/users/";
        size_t users_path_len = strlen(users_path);
        
        if (mg_str_get_len(&uri) > users_path_len) {
            const char *id_start = mg_str_get_ptr(&uri) + users_path_len;
            size_t id_len = 0;
            
            // Find the end of the ID (either end of string or next slash)
            while (id_len < mg_str_get_len(&uri) - users_path_len && id_start[id_len] != '/' && id_start[id_len] != '?') {
                id_len++;
            }
            
            if (id_len > 0 && id_len < sizeof(id_str)) {
                strncpy(id_str, id_start, id_len);
                id_str[id_len] = '\0';
            }
        }
    }
    
    if (id_str[0] == '\0') {
        mg_send_json_error(c, 400, "Missing user ID");
        return;
    }
    
    int64_t user_id = strtoll(id_str, NULL, 10);
    
    // Check if the user exists
    user_t user;
    int rc = db_auth_get_user_by_id(user_id, &user);
    
    if (rc != 0) {
        mg_send_json_error(c, 404, "User not found");
        return;
    }
    
    // Generate a new API key
    char api_key[64] = {0};
    rc = db_auth_generate_api_key(user_id, api_key, sizeof(api_key));
    
    if (rc != 0) {
        mg_send_json_error(c, 500, "Failed to generate API key");
        return;
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
    
    log_info("Successfully generated API key for user: %s (ID: %lld)", user.username, (long long)user_id);
}
