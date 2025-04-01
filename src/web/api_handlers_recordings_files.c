#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <unistd.h>
#include <sys/stat.h>
#include <dirent.h>
#include <time.h>

#include "web/api_handlers.h"
#include "web/mongoose_adapter.h"
#include "core/logger.h"
#include "core/config.h"
#include "mongoose.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"
#include "web/mongoose_server_multithreading.h"

/**
 * @brief Structure for file operations task
 */
typedef struct {
    struct mg_connection *connection;  // Mongoose connection
    char *path;                        // File path
    char *operation;                   // Operation type (e.g., "check", "delete")
} file_operation_task_t;

/**
 * @brief Create a file operation task
 * 
 * @param c Mongoose connection
 * @param path File path
 * @param operation Operation type
 * @return file_operation_task_t* Pointer to the task or NULL on error
 */
file_operation_task_t *file_operation_task_create(struct mg_connection *c, const char *path, const char *operation) {
    file_operation_task_t *task = calloc(1, sizeof(file_operation_task_t));
    if (!task) {
        log_error("Failed to allocate memory for file operation task");
        return NULL;
    }
    
    task->connection = c;
    
    if (path) {
        task->path = strdup(path);
        if (!task->path) {
            log_error("Failed to allocate memory for file path");
            free(task);
            return NULL;
        }
    } else {
        task->path = NULL;
    }
    
    if (operation) {
        task->operation = strdup(operation);
        if (!task->operation) {
            log_error("Failed to allocate memory for operation");
            if (task->path) free(task->path);
            free(task);
            return NULL;
        }
    } else {
        task->operation = NULL;
    }
    
    return task;
}

/**
 * @brief Free a file operation task
 * 
 * @param task Task to free
 */
void file_operation_task_free(file_operation_task_t *task) {
    if (task) {
        if (task->path) {
            free(task->path);
        }
        if (task->operation) {
            free(task->operation);
        }
        free(task);
    }
}

/**
 * @brief File operation task function
 * 
 * @param arg Task argument (file_operation_task_t*)
 */
void file_operation_task_function(void *arg) {
    file_operation_task_t *task = (file_operation_task_t *)arg;
    if (!task) {
        log_error("Invalid file operation task");
        return;
    }
    
    struct mg_connection *c = task->connection;
    if (!c) {
        log_error("Invalid Mongoose connection");
        file_operation_task_free(task);
        return;
    }
    
    // Check operation type
    if (strcmp(task->operation, "check") == 0) {
        // Check if file exists
        struct stat st;
        bool exists = (stat(task->path, &st) == 0);
        
        // Create response
        cJSON *response = cJSON_CreateObject();
        cJSON_AddBoolToObject(response, "exists", exists);
        if (exists) {
            cJSON_AddNumberToObject(response, "size", st.st_size);
            cJSON_AddNumberToObject(response, "mtime", st.st_mtime);
        }
        
        // Convert to string
        char *json_str = cJSON_PrintUnformatted(response);
        if (!json_str) {
            log_error("Failed to convert response JSON to string");
            cJSON_Delete(response);
            mg_send_json_error(c, 500, "Failed to create response");
            file_operation_task_free(task);
            return;
        }
        
        // Send response
        mg_send_json_response(c, 200, json_str);
        
        // Clean up
        free(json_str);
        cJSON_Delete(response);
        
        log_info("Successfully checked file: %s (exists: %d)", task->path, exists);
    } else if (strcmp(task->operation, "delete") == 0) {
        // Check if file exists
        struct stat st;
        bool exists = (stat(task->path, &st) == 0);
        
        if (!exists) {
            // File doesn't exist, return success
            cJSON *response = cJSON_CreateObject();
            cJSON_AddBoolToObject(response, "success", true);
            cJSON_AddBoolToObject(response, "existed", false);
            
            // Convert to string
            char *json_str = cJSON_PrintUnformatted(response);
            if (!json_str) {
                log_error("Failed to convert response JSON to string");
                cJSON_Delete(response);
                mg_send_json_error(c, 500, "Failed to create response");
                file_operation_task_free(task);
                return;
            }
            
            // Send response
            mg_send_json_response(c, 200, json_str);
            
            // Clean up
            free(json_str);
            cJSON_Delete(response);
            
            log_info("File doesn't exist, no need to delete: %s", task->path);
        } else {
            // Delete file
            if (unlink(task->path) != 0) {
                log_error("Failed to delete file: %s", task->path);
                mg_send_json_error(c, 500, "Failed to delete file");
                file_operation_task_free(task);
                return;
            }
            
            // Create response
            cJSON *response = cJSON_CreateObject();
            cJSON_AddBoolToObject(response, "success", true);
            cJSON_AddBoolToObject(response, "existed", true);
            
            // Convert to string
            char *json_str = cJSON_PrintUnformatted(response);
            if (!json_str) {
                log_error("Failed to convert response JSON to string");
                cJSON_Delete(response);
                mg_send_json_error(c, 500, "Failed to create response");
                file_operation_task_free(task);
                return;
            }
            
            // Send response
            mg_send_json_response(c, 200, json_str);
            
            // Clean up
            free(json_str);
            cJSON_Delete(response);
            
            log_info("Successfully deleted file: %s", task->path);
        }
    } else {
        log_error("Unknown operation: %s", task->operation);
        mg_send_json_error(c, 400, "Unknown operation");
    }
    
    file_operation_task_free(task);
}

/**
 * @brief Handler function for file operations
 * 
 * This function is called by the multithreading system.
 * 
 * @param c Mongoose connection
 * @param hm HTTP message
 */
void file_operation_handler(struct mg_connection *c, struct mg_http_message *hm) {
    // Parse query parameters
    char query_string[512] = {0};
    if (hm->query.len > 0 && hm->query.len < sizeof(query_string)) {
        memcpy(query_string, hm->query.buf, hm->query.len);
        query_string[hm->query.len] = '\0';
        log_info("Query string: %s", query_string);
    }
    
    // Extract path parameter
    char path[256] = {0};
    char *param = strtok(query_string, "&");
    while (param) {
        if (strncmp(param, "path=", 5) == 0) {
            // URL-decode the path
            char *decoded_path = param + 5;
            // Simple URL decoding for %20 (space)
            char *src = decoded_path;
            char *dst = path;
            while (*src && dst < path + sizeof(path) - 1) {
                if (src[0] == '%' && src[1] == '2' && src[2] == '0') {
                    *dst++ = ' ';
                    src += 3;
                } else {
                    *dst++ = *src++;
                }
            }
            *dst = '\0';
            break;
        }
        param = strtok(NULL, "&");
    }
    
    if (path[0] == '\0') {
        log_error("Missing path parameter");
        mg_send_json_error(c, 400, "Missing path parameter");
        return;
    }
    
    // Determine operation type based on HTTP method
    const char *operation = NULL;
    if (mg_http_match_uri(hm, "/api/recordings/files/check")) {
        operation = "check";
    } else if (mg_http_match_uri(hm, "/api/recordings/files")) {
        operation = "delete";
    } else {
        log_error("Unknown URI");
        mg_send_json_error(c, 404, "Unknown URI");
        return;
    }
    
    // Create task
    file_operation_task_t *task = file_operation_task_create(c, path, operation);
    if (!task) {
        log_error("Failed to create file operation task");
        mg_send_json_error(c, 500, "Failed to create file operation task");
        return;
    }
    
    // Call the task function directly
    file_operation_task_function(task);
}

/**
 * @brief Direct handler for GET /api/recordings/files/check
 */
void mg_handle_check_recording_file(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/recordings/files/check request");
    
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
    data->handler_func = file_operation_handler;
    
    // Start thread
    mg_start_thread(mg_thread_function, data);
    
    log_info("File operation task started in a worker thread");
}

/**
 * @brief Direct handler for DELETE /api/recordings/files
 */
void mg_handle_delete_recording_file(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling DELETE /api/recordings/files request");
    
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
    data->handler_func = file_operation_handler;
    
    // Start thread
    mg_start_thread(mg_thread_function, data);
    
    log_info("File operation task started in a worker thread");
}
