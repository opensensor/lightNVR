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
#include "core/logger.h"
#include "core/config.h"
#include "mongoose.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"
#include "database/db_auth.h"
#include "web/mongoose_server_multithreading.h"

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
    
    uint64_t id = task->id;
    
    log_info("Processing DELETE /api/recordings/%llu task", (unsigned long long)id);

    // Get recording from database
    recording_metadata_t recording;
    memset(&recording, 0, sizeof(recording_metadata_t)); // Initialize to prevent undefined behavior

    if (get_recording_metadata_by_id(id, &recording) != 0) {
        log_error("Recording not found: %llu", (unsigned long long)id);
        // Don't send response here - already sent 202
        delete_recording_task_free(task);
        return;
    }

    // Save file path before deleting from database
    char file_path_copy[256];
    strncpy(file_path_copy, recording.file_path, sizeof(file_path_copy) - 1);
    file_path_copy[sizeof(file_path_copy) - 1] = '\0';

    // Delete from database FIRST
    if (delete_recording_metadata(id) != 0) {
        log_error("Failed to delete recording from database: %llu", (unsigned long long)id);
        // Don't send response here - already sent 202
        delete_recording_task_free(task);
        return;
    }

    log_info("Deleted recording from database: %llu", (unsigned long long)id);

    // Then delete the file from disk
    struct stat st;
    if (stat(file_path_copy, &st) == 0) {
        if (unlink(file_path_copy) != 0) {
            log_warn("Failed to delete recording file: %s (error: %s)",
                    file_path_copy, strerror(errno));
            // File deletion failed but DB entry is already removed
            // This is acceptable - orphaned files can be cleaned up later
        } else {
            log_info("Deleted recording file: %s", file_path_copy);
        }
    } else {
        log_warn("Recording file does not exist: %s (already deleted or never created)", file_path_copy);
        // This is acceptable - DB entry is removed
    }
    
    // Clean up
    delete_recording_task_free(task);
    
    log_info("Successfully deleted recording: %llu", (unsigned long long)id);
}

/**
 * @brief Handler function for delete recording
 * 
 * This function is called by the multithreading system.
 * 
 * @param c Mongoose connection
 * @param hm HTTP message
 */
void delete_recording_handler(struct mg_connection *c, struct mg_http_message *hm) {
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
    
    log_info("Handling DELETE /api/recordings/%llu request in worker thread", (unsigned long long)id);
    
    // Create task
    delete_recording_task_t *task = delete_recording_task_create(c, id);
    if (!task) {
        log_error("Failed to create delete recording task");
        mg_send_json_error(c, 500, "Failed to create delete recording task");
        return;
    }
    
    // Call the task function directly
    delete_recording_task_function(task);
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
                    if (db_auth_validate_session(session_token, &user_id) == 0) {
                        // Session is valid, check user role
                        if (db_auth_get_user_by_id(user_id, &user) == 0) {
                            // Only admin and regular users can delete recordings, viewers cannot
                            return (user.role == USER_ROLE_ADMIN || user.role == USER_ROLE_USER);
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
    data->handler_func = delete_recording_handler;
    
    // Start thread
    mg_start_thread(mg_thread_function, data);
    
    log_info("Delete recording task started in a worker thread");
}

// This function is now defined in api_handlers_recordings_batch.c
// to avoid duplicate definitions
