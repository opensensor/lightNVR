#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <unistd.h>
#include <sys/stat.h>
#include <dirent.h>

#include "web/api_handlers.h"
#include "web/mongoose_adapter.h"
#include "web/mongoose_server_auth.h"
#include "web/http_server.h"
#include "web/api_thread_pool.h"
#include "core/logger.h"
#include "core/config.h"
#include "mongoose.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"
#include "database/db_auth.h"

// Forward declarations for batch delete functionality
typedef struct {
    struct mg_connection *connection;  // Mongoose connection
    char *json_str;                    // JSON request string (for parsing parameters)
} batch_delete_recordings_task_t;

batch_delete_recordings_task_t *batch_delete_recordings_task_create(struct mg_connection *c, const char *json_str);
void batch_delete_recordings_task_free(batch_delete_recordings_task_t *task);
void batch_delete_recordings_task_function(void *arg);

/**
 * @brief Structure for delete recording task
 */
typedef struct {
    struct mg_connection *connection;  // Mongoose connection
    uint64_t id;                       // Recording ID
} delete_recording_task_t;

/**
 * @brief Create a delete recording task
 * 
 * @param c Mongoose connection
 * @param id Recording ID
 * @return delete_recording_task_t* Pointer to the task or NULL on error
 */
delete_recording_task_t *delete_recording_task_create(struct mg_connection *c, uint64_t id) {
    delete_recording_task_t *task = calloc(1, sizeof(delete_recording_task_t));
    if (!task) {
        log_error("Failed to allocate memory for delete recording task");
        return NULL;
    }
    
    task->connection = c;
    task->id = id;
    
    return task;
}

/**
 * @brief Free a delete recording task
 * 
 * @param task Task to free
 */
void delete_recording_task_free(delete_recording_task_t *task) {
    if (task) {
        free(task);
    }
}

/**
 * @brief Delete recording task function
 * 
 * @param arg Task argument (delete_recording_task_t*)
 */
void delete_recording_task_function(void *arg) {
    delete_recording_task_t *task = (delete_recording_task_t *)arg;
    if (!task) {
        log_error("Invalid delete recording task");
        return;
    }
    
    struct mg_connection *c = task->connection;
    if (!c) {
        log_error("Invalid Mongoose connection");
        delete_recording_task_free(task);
        return;
    }
    
    // Release the thread pool when this task is done
    bool release_needed = true;
    
    uint64_t id = task->id;
    
    log_info("Handling DELETE /api/recordings/%llu request", (unsigned long long)id);
    
    // Get recording from database
    recording_metadata_t recording;
    if (get_recording_metadata_by_id(id, &recording) != 0) {
        log_error("Recording not found: %llu", (unsigned long long)id);
        mg_send_json_error(c, 404, "Recording not found");
        delete_recording_task_free(task);
        if (release_needed) {
            api_thread_pool_release();
        }
        return;
    }
    
    // Delete file
    if (unlink(recording.file_path) != 0) {
        log_warn("Failed to delete recording file: %s", recording.file_path);
        // Continue anyway, we'll remove from database
    } else {
        log_info("Deleted recording file: %s", recording.file_path);
    }
    
    // Delete from database
    if (delete_recording_metadata(id) != 0) {
        log_error("Failed to delete recording from database: %llu", (unsigned long long)id);
        mg_send_json_error(c, 500, "Failed to delete recording from database");
        delete_recording_task_free(task);
        if (release_needed) {
            api_thread_pool_release();
        }
        return;
    }
    
    // Create success response using cJSON
    cJSON *success = cJSON_CreateObject();
    if (!success) {
        log_error("Failed to create success JSON object");
        mg_send_json_error(c, 500, "Failed to create success JSON");
        delete_recording_task_free(task);
        if (release_needed) {
            api_thread_pool_release();
        }
        return;
    }
    
    cJSON_AddBoolToObject(success, "success", true);
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(success);
    if (!json_str) {
        log_error("Failed to convert success JSON to string");
        cJSON_Delete(success);
        mg_send_json_error(c, 500, "Failed to convert success JSON to string");
        delete_recording_task_free(task);
        if (release_needed) {
            api_thread_pool_release();
        }
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    cJSON_Delete(success);
    
    delete_recording_task_free(task);
    
    // Release the thread pool if needed
    if (release_needed) {
        api_thread_pool_release();
    }
    
    log_info("Successfully deleted recording: %llu", (unsigned long long)id);
}

/**
 * @brief Check if the user has permission to delete recordings
 * 
 * @param hm HTTP message
 * @param server HTTP server
 * @return 1 if the user has permission, 0 otherwise
 */
static int check_delete_permission(struct mg_http_message *hm, http_server_t *server) {
    // Get the authenticated user
    int64_t user_id;
    user_t user;
    
    // Check if authentication is enabled
    if (!server || !server->config.auth_enabled) {
        return 1; // Authentication is disabled, allow all
    }
    
    // First, check for session token in cookie
    struct mg_str *cookie = mg_http_get_header(hm, "Cookie");
    if (cookie) {
        char session_token[64] = {0};
        if (mg_http_get_var(cookie, "session", session_token, sizeof(session_token)) > 0) {
            // Validate the session token
            if (db_auth_validate_session(session_token, &user_id) == 0) {
                // Session is valid, check user role
                if (db_auth_get_user_by_id(user_id, &user) == 0) {
                    // Only admin and regular users can delete recordings, viewers cannot
                    return (user.role == USER_ROLE_ADMIN || user.role == USER_ROLE_USER);
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
        if (db_auth_authenticate(username, password, &user_id) == 0) {
            // Authentication successful, check user role
            if (db_auth_get_user_by_id(user_id, &user) == 0) {
                // Only admin and regular users can delete recordings, viewers cannot
                return (user.role == USER_ROLE_ADMIN || user.role == USER_ROLE_USER);
            }
        }
    }
    
    return 0; // No valid authentication or insufficient permissions
}

/**
 * @brief Direct handler for DELETE /api/recordings/:id
 */
void mg_handle_delete_recording(struct mg_connection *c, struct mg_http_message *hm) {
    // Check authentication and permissions
    http_server_t *server = (http_server_t *)c->fn_data;
    if (!check_delete_permission(hm, server)) {
        log_error("Permission denied for DELETE /api/recordings/:id");
        mg_send_json_error(c, 403, "Permission denied: Only admin and regular users can delete recordings");
        return;
    }
    
    // Extract recording ID from URL
    char id_str[32];
    if (mg_extract_path_param(hm, "/api/recordings/", id_str, sizeof(id_str)) != 0) {
        log_error("Failed to extract recording ID from URL");
        mg_send_json_error(c, 400, "Invalid request path");
        return;
    }
    
    // Convert ID to integer
    uint64_t id = strtoull(id_str, NULL, 10);
    if (id == 0) {
        log_error("Invalid recording ID: %s", id_str);
        mg_send_json_error(c, 400, "Invalid recording ID");
        return;
    }
    
    log_info("Handling DELETE /api/recordings/%llu request", (unsigned long long)id);
    
    // Acquire thread pool
    thread_pool_t *pool = api_thread_pool_acquire(api_thread_pool_get_size(), 10);
    if (!pool) {
        log_error("Failed to acquire thread pool");
        mg_send_json_error(c, 500, "Failed to acquire thread pool");
        return;
    }
    
    // Create task
    delete_recording_task_t *task = delete_recording_task_create(c, id);
    if (!task) {
        log_error("Failed to create delete recording task");
        api_thread_pool_release();
        mg_send_json_error(c, 500, "Failed to create delete recording task");
        return;
    }
    
    // Add task to thread pool
    if (!thread_pool_add_task(pool, delete_recording_task_function, task)) {
        log_error("Failed to add delete recording task to thread pool");
        delete_recording_task_free(task);
        api_thread_pool_release();
        mg_send_json_error(c, 500, "Failed to add delete recording task to thread pool");
        return;
    }
    
    // Note: The task will release the thread pool when it's done
    log_info("Delete recording task added to thread pool");
}

// This function is now defined in api_handlers_recordings_batch.c
// to avoid duplicate definitions
