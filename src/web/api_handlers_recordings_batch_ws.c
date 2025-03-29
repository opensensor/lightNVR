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

#include "../external/cjson/cJSON.h"
#include "web/api_handlers.h"
#include "web/mongoose_adapter.h"
#include "web/api_thread_pool.h"
#include "web/websocket_bridge.h"
#include "web/websocket_client.h"
#include "web/websocket_handler.h"
#include "web/api_handlers_recordings_batch_ws.h"
#include "web/mongoose_server_websocket_utils.h"
#include "core/logger.h"
#include "core/config.h"
#include "core/shutdown_coordinator.h"
#include "mongoose.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"

/**
 * @brief Structure for batch delete recordings task with WebSocket support
 */
typedef struct {
    char client_id[64];                // WebSocket client ID
    char *json_str;                    // JSON request string (for parsing parameters)
    bool use_websocket;                // Whether to use WebSocket for progress updates
    struct mg_connection *conn;        // Mongoose connection for WebSocket updates
} batch_delete_recordings_ws_task_t;

/**
 * @brief Create a batch delete recordings task with WebSocket support
 * 
 * @param client_id WebSocket client ID
 * @param json_str JSON request string (for parsing parameters)
 * @param use_websocket Whether to use WebSocket for progress updates
 * @param conn Mongoose connection for WebSocket updates
 * @return batch_delete_recordings_ws_task_t* Pointer to the task or NULL on error
 */
batch_delete_recordings_ws_task_t *batch_delete_recordings_ws_task_create(
    const char *client_id, const char *json_str, bool use_websocket, struct mg_connection *conn) {
    
    batch_delete_recordings_ws_task_t *task = calloc(1, sizeof(batch_delete_recordings_ws_task_t));
    if (!task) {
        log_error("Failed to allocate memory for batch delete recordings task");
        return NULL;
    }
    
    if (client_id) {
        strncpy(task->client_id, client_id, sizeof(task->client_id) - 1);
        task->client_id[sizeof(task->client_id) - 1] = '\0';
    } else {
        task->client_id[0] = '\0';
    }
    
    task->use_websocket = use_websocket;
    task->conn = conn;
    
    if (json_str) {
        task->json_str = strdup(json_str);
        if (!task->json_str) {
            log_error("Failed to allocate memory for JSON string");
            free(task);
            return NULL;
        }
    } else {
        task->json_str = NULL;
    }
    
    return task;
}

/**
 * @brief Free a batch delete recordings task with WebSocket support
 * 
 * @param task Task to free
 */
void batch_delete_recordings_ws_task_free(batch_delete_recordings_ws_task_t *task) {
    if (task) {
        if (task->json_str) {
            free(task->json_str);
        }
        free(task);
    }
}

/**
 * @brief Send a progress update via WebSocket
 * 
 * @param conn Mongoose connection
 * @param current Current progress
 * @param total Total items
 * @param success_count Number of successful deletions
 * @param error_count Number of failed deletions
 * @param status Status message
 * @param is_complete Whether the operation is complete
 */
void send_progress_update(struct mg_connection *conn, int current, int total, 
                         int success_count, int error_count, 
                         const char *status, bool is_complete) {
    
    if (!conn) {
        log_error("Invalid connection in send_progress_update");
        return;
    }
    
    // Create progress update message
    char message[1024];
    int len = snprintf(message, sizeof(message), 
            "{\"type\":\"progress\",\"topic\":\"recordings/batch-delete\",\"payload\":{\"current\":%d,\"total\":%d,\"succeeded\":%d,\"failed\":%d,\"status\":\"%s\",\"complete\":%s}}",
            current, total, success_count, error_count, status ? status : "", 
            is_complete ? "true" : "false");
    
    if (len < 0 || len >= (int)sizeof(message)) {
        log_error("Failed to format progress update message (buffer too small)");
        return;
    }
    
    log_info("Sending progress update: %s", message);
    
    // Send message directly using mongoose
    mg_ws_send(conn, message, len, WEBSOCKET_OP_TEXT);
}

/**
 * @brief Send a final result via WebSocket
 * 
 * @param client_id Client ID
 * @param success Whether the operation was successful
 * @param total Total items
 * @param success_count Number of successful deletions
 * @param error_count Number of failed deletions
 * @param results_json JSON array of results
 */
void send_final_result(const char *client_id, bool success, int total, 
                      int success_count, int error_count, const char *results_json) {
    
    if (!client_id) {
        log_error("Invalid client_id in send_final_result");
        return;
    }
    
    // Create message
    char *message = NULL;
    int len = 0;
    
    if (results_json) {
        // Use asprintf to allocate memory for the complete message
        len = asprintf(&message, 
                   "{\"type\":\"result\",\"topic\":\"recordings/batch-delete\",\"payload\":{\"success\":%s,\"total\":%d,\"succeeded\":%d,\"failed\":%d,\"results\":%s}}",
                   success ? "true" : "false", total, success_count, error_count, results_json);
    } else {
        // Use asprintf to allocate memory for the complete message without results
        len = asprintf(&message, 
                   "{\"type\":\"result\",\"topic\":\"recordings/batch-delete\",\"payload\":{\"success\":%s,\"total\":%d,\"succeeded\":%d,\"failed\":%d,\"results\":[]}}",
                   success ? "true" : "false", total, success_count, error_count);
    }
    
    if (len < 0 || !message) {
        log_error("Failed to allocate memory for WebSocket message");
        return;
    }
    
    // Get client connection directly from pointer value stored in client_id string
    struct mg_connection *conn = NULL;
    if (sscanf(client_id, "%p", &conn) == 1 && conn) {
        // Send message directly using mongoose
        log_info("Sending final result to client %s", client_id);
        mg_ws_send(conn, message, strlen(message), WEBSOCKET_OP_TEXT);
    } else {
        log_error("Invalid client ID or connection not found: %s", client_id);
    }
    
    // Free message
    free(message);
}

/**
 * @brief Batch delete recordings task function with WebSocket support
 * 
 * @param arg Task argument (batch_delete_recordings_ws_task_t*)
 */
void batch_delete_recordings_ws_task_function(void *arg) {
    batch_delete_recordings_ws_task_t *task = (batch_delete_recordings_ws_task_t *)arg;
    if (!task) {
        log_error("Invalid batch delete recordings task");
        return;
    }
    
    //  Check for shutdown before starting work
    if (is_shutdown_initiated()) {
        log_info("Skipping batch delete task due to system shutdown");
        batch_delete_recordings_ws_task_free(task);
        api_thread_pool_release();
        return;
    }
    
    // Release the thread pool when this task is done
    bool release_needed = true;
    
    // Parse JSON request
    cJSON *json = cJSON_Parse(task->json_str);
    if (!json) {
        log_error("Failed to parse JSON body");
        
        if (task->use_websocket && task->conn) {
            // Convert connection pointer to client_id string
            char client_id[32];
            snprintf(client_id, sizeof(client_id), "%p", (void*)task->conn);
            send_final_result(client_id, false, 0, 0, 0, NULL);
        }
        
        batch_delete_recordings_ws_task_free(task);
        if (release_needed) {
            api_thread_pool_release();
        }
        return;
    }
    
    // Check if we're deleting by IDs or by filter
    cJSON *ids_array = cJSON_GetObjectItem(json, "ids");
    cJSON *filter = cJSON_GetObjectItem(json, "filter");
    
    if (ids_array && cJSON_IsArray(ids_array)) {
        // Delete by IDs
        int array_size = cJSON_GetArraySize(ids_array);
        if (array_size == 0) {
            log_warn("Empty 'ids' array in batch delete request");
            cJSON_Delete(json);
            
            if (task->use_websocket && task->conn) {
                // Convert connection pointer to client_id string
                char client_id[32];
                snprintf(client_id, sizeof(client_id), "%p", (void*)task->conn);
                send_final_result(client_id, false, 0, 0, 0, NULL);
            }
            
            batch_delete_recordings_ws_task_free(task);
            if (release_needed) {
                api_thread_pool_release();
            }
            return;
        }
        
        // Process each ID
        int success_count = 0;
        int error_count = 0;
        cJSON *results_array = cJSON_CreateArray();
        
        // Send initial progress update
        if (task->use_websocket && task->conn) {
            send_progress_update(task->conn, 0, array_size, 0, 0, 
                               "Starting batch delete operation", false);
        }
        
        for (int i = 0; i < array_size; i++) {
            cJSON *id_item = cJSON_GetArrayItem(ids_array, i);
            if (!id_item || !cJSON_IsNumber(id_item)) {
                log_warn("Invalid ID at index %d", i);
                error_count++;
                
                // Send progress update
                if (task->use_websocket && task->conn) {
                    char status[128];
                    snprintf(status, sizeof(status), "Invalid ID at index %d", i);
                    send_progress_update(task->conn, i + 1, array_size, 
                                       success_count, error_count, status, false);
                }
                
                continue;
            }
            
            uint64_t id = (uint64_t)id_item->valuedouble;
            
            // Get recording from database
            recording_metadata_t recording;
            if (get_recording_metadata_by_id(id, &recording) != 0) {
                log_warn("Recording not found: %llu", (unsigned long long)id);
                
                // Add result to array
                cJSON *result = cJSON_CreateObject();
                cJSON_AddNumberToObject(result, "id", id);
                cJSON_AddBoolToObject(result, "success", false);
                cJSON_AddStringToObject(result, "error", "Recording not found");
                cJSON_AddItemToArray(results_array, result);
                
                error_count++;
                
                // Send progress update
                if (task->use_websocket && task->conn) {
                    char status[128];
                    snprintf(status, sizeof(status), "Recording not found: %llu", (unsigned long long)id);
                    send_progress_update(task->conn, i + 1, array_size, 
                                       success_count, error_count, status, false);
                }
                
                continue;
            }
            
            // Delete file
            bool file_deleted = true;
            if (unlink(recording.file_path) != 0) {
                log_warn("Failed to delete recording file: %s", recording.file_path);
                file_deleted = false;
            } else {
                log_info("Deleted recording file: %s", recording.file_path);
            }
            
            // Delete from database
            if (delete_recording_metadata(id) != 0) {
                log_error("Failed to delete recording from database: %llu", (unsigned long long)id);
                
                // Add result to array
                cJSON *result = cJSON_CreateObject();
                cJSON_AddNumberToObject(result, "id", id);
                cJSON_AddBoolToObject(result, "success", false);
                cJSON_AddStringToObject(result, "error", "Failed to delete from database");
                cJSON_AddItemToArray(results_array, result);
                
                error_count++;
                
                // Send progress update
                if (task->use_websocket && task->conn) {
                    char status[128];
                    snprintf(status, sizeof(status), "Failed to delete recording %llu from database", 
                           (unsigned long long)id);
                    send_progress_update(task->conn, i + 1, array_size, 
                                       success_count, error_count, status, false);
                }
            } else {
                // Add success result to array
                cJSON *result = cJSON_CreateObject();
                cJSON_AddNumberToObject(result, "id", id);
                cJSON_AddBoolToObject(result, "success", true);
                if (!file_deleted) {
                    cJSON_AddStringToObject(result, "warning", "File not deleted but removed from database");
                }
                cJSON_AddItemToArray(results_array, result);
                
                success_count++;
                log_info("Successfully deleted recording: %llu", (unsigned long long)id);
                
            // Send progress update
            if (task->use_websocket && task->conn) {
                char status[128];
                snprintf(status, sizeof(status), "Deleted recording %llu", (unsigned long long)id);
                send_progress_update(task->conn, i + 1, array_size, 
                                   success_count, error_count, status, false);
                
                // Add a small delay to ensure the WebSocket message is sent
                usleep(5000);  // 5ms delay
            }
            }
            
            // Add a small delay to avoid overwhelming the WebSocket connection
            if (task->use_websocket && task->conn) {
                usleep(10000);  // 10ms delay
            }
        }
        
        // Create response
        cJSON *response = cJSON_CreateObject();
        cJSON_AddBoolToObject(response, "success", error_count == 0);
        cJSON_AddNumberToObject(response, "total", array_size);
        cJSON_AddNumberToObject(response, "succeeded", success_count);
        cJSON_AddNumberToObject(response, "failed", error_count);
        cJSON_AddItemToObject(response, "results", results_array);
        
        // Convert to string
        char *json_str = cJSON_PrintUnformatted(response);
        if (!json_str) {
            log_error("Failed to convert response JSON to string");
            cJSON_Delete(json);
            cJSON_Delete(response);
            
            if (task->use_websocket && task->conn) {
                // Convert connection pointer to client_id string
                char client_id[32];
                snprintf(client_id, sizeof(client_id), "%p", (void*)task->conn);
                send_final_result(client_id, false, array_size, success_count, error_count, NULL);
            }
            
            batch_delete_recordings_ws_task_free(task);
            if (release_needed) {
                api_thread_pool_release();
            }
            return;
        }
        
    // Send final result via WebSocket
    if (task->use_websocket && task->conn && !is_shutdown_initiated()) {
        //  Check for shutdown before sending final result
        // Extract results array as string
        char *results_str = cJSON_PrintUnformatted(results_array);
        // Convert connection pointer to client_id string
        char client_id[32];
        snprintf(client_id, sizeof(client_id), "%p", (void*)task->conn);
        if (results_str) {
            send_final_result(client_id, error_count == 0, array_size, 
                            success_count, error_count, results_str);
            free(results_str);
        } else {
            send_final_result(client_id, error_count == 0, array_size, 
                            success_count, error_count, "[]");
        }
    }
        
        // Clean up
        free(json_str);
        cJSON_Delete(json);
        cJSON_Delete(response);
        
        log_info("Successfully handled batch delete request: %d succeeded, %d failed", 
                success_count, error_count);
    } else if (filter && cJSON_IsObject(filter)) {
        // Delete by filter
        time_t start_time = 0;
        time_t end_time = 0;
        char stream_name[64] = {0};
        int has_detection = 0;
        
        // Extract filter parameters
        cJSON *start = cJSON_GetObjectItem(filter, "start");
        cJSON *end = cJSON_GetObjectItem(filter, "end");
        cJSON *stream = cJSON_GetObjectItem(filter, "stream");
        cJSON *detection = cJSON_GetObjectItem(filter, "detection");
        
        if (start && cJSON_IsString(start)) {
            // URL-decode the time string (replace %3A with :)
            char decoded_start_time[64] = {0};
            strncpy(decoded_start_time, start->valuestring, sizeof(decoded_start_time) - 1);
            
            // Replace %3A with :
            char *pos = decoded_start_time;
            while ((pos = strstr(pos, "%3A")) != NULL) {
                *pos = ':';
                memmove(pos + 1, pos + 3, strlen(pos + 3) + 1);
            }
            
            log_info("Filter start time (decoded): %s", decoded_start_time);
            
            struct tm tm = {0};
            // Try different time formats
            if (strptime(decoded_start_time, "%Y-%m-%dT%H:%M:%S", &tm) != NULL ||
                strptime(decoded_start_time, "%Y-%m-%dT%H:%M:%S.000Z", &tm) != NULL ||
                strptime(decoded_start_time, "%Y-%m-%dT%H:%M:%S.000", &tm) != NULL ||
                strptime(decoded_start_time, "%Y-%m-%dT%H:%M:%SZ", &tm) != NULL) {
                
                // Set tm_isdst to -1 to let mktime determine if DST is in effect
                tm.tm_isdst = -1;
                start_time = mktime(&tm);
                log_info("Filter start time parsed: %s -> %ld", decoded_start_time, (long)start_time);
            } else {
                log_error("Failed to parse start time: %s", decoded_start_time);
            }
        }
        
        if (end && cJSON_IsString(end)) {
            // URL-decode the time string (replace %3A with :)
            char decoded_end_time[64] = {0};
            strncpy(decoded_end_time, end->valuestring, sizeof(decoded_end_time) - 1);
            
            // Replace %3A with :
            char *pos = decoded_end_time;
            while ((pos = strstr(pos, "%3A")) != NULL) {
                *pos = ':';
                memmove(pos + 1, pos + 3, strlen(pos + 3) + 1);
            }
            
            log_info("Filter end time (decoded): %s", decoded_end_time);
            
            struct tm tm = {0};
            // Try different time formats
            if (strptime(decoded_end_time, "%Y-%m-%dT%H:%M:%S", &tm) != NULL ||
                strptime(decoded_end_time, "%Y-%m-%dT%H:%M:%S.000Z", &tm) != NULL ||
                strptime(decoded_end_time, "%Y-%m-%dT%H:%M:%S.000", &tm) != NULL ||
                strptime(decoded_end_time, "%Y-%m-%dT%H:%M:%SZ", &tm) != NULL) {
                
                // Set tm_isdst to -1 to let mktime determine if DST is in effect
                tm.tm_isdst = -1;
                end_time = mktime(&tm);
                log_info("Filter end time parsed: %s -> %ld", decoded_end_time, (long)end_time);
            } else {
                log_error("Failed to parse end time: %s", decoded_end_time);
            }
        }
        
        if (stream && cJSON_IsString(stream)) {
            strncpy(stream_name, stream->valuestring, sizeof(stream_name) - 1);
        }
        
        if (detection && cJSON_IsNumber(detection)) {
            has_detection = detection->valueint;
        }
        
        // Get recordings matching filter
        recording_metadata_t *recordings = NULL;
        int count = 0;
        int total_count = 0;
        
        // Get total count first
        total_count = get_recording_count(start_time, end_time, 
                                        stream_name[0] != '\0' ? stream_name : NULL,
                                        has_detection);
        
        if (total_count <= 0) {
            log_info("No recordings match the filter");
            cJSON *response = cJSON_CreateObject();
            cJSON_AddBoolToObject(response, "success", true);
            cJSON_AddNumberToObject(response, "total", 0);
            cJSON_AddNumberToObject(response, "succeeded", 0);
            cJSON_AddNumberToObject(response, "failed", 0);
            cJSON_AddItemToObject(response, "results", cJSON_CreateArray());
            
            char *json_str = cJSON_PrintUnformatted(response);
            if (!json_str) {
                log_error("Failed to convert response JSON to string");
                cJSON_Delete(json);
                cJSON_Delete(response);
                
                if (task->use_websocket && task->conn) {
                    // Convert connection pointer to client_id string
                    char client_id[32];
                    snprintf(client_id, sizeof(client_id), "%p", (void*)task->conn);
                    send_final_result(client_id, true, 0, 0, 0, "[]");
                }
                
                batch_delete_recordings_ws_task_free(task);
                if (release_needed) {
                    api_thread_pool_release();
                }
                return;
            }
            
            // Send final result via WebSocket
            if (task->use_websocket && task->conn) {
                // Convert connection pointer to client_id string
                char client_id[32];
                snprintf(client_id, sizeof(client_id), "%p", (void*)task->conn);
                send_final_result(client_id, true, 0, 0, 0, "[]");
            }
            
            free(json_str);
            cJSON_Delete(json);
            cJSON_Delete(response);
            batch_delete_recordings_ws_task_free(task);
            if (release_needed) {
                api_thread_pool_release();
            }
            return;
        }
        
        // Send initial progress update
        if (task->use_websocket && task->conn) {
            char status[128];
            snprintf(status, sizeof(status), "Found %d recordings matching filter", total_count);
            send_progress_update(task->conn, 0, total_count, 0, 0, status, false);
        }
        
        // Allocate memory for recordings
        recordings = (recording_metadata_t *)malloc(total_count * sizeof(recording_metadata_t));
        if (!recordings) {
            log_error("Failed to allocate memory for recordings");
            cJSON_Delete(json);
            
            if (task->use_websocket && task->conn) {
                // Convert connection pointer to client_id string
                char client_id[32];
                snprintf(client_id, sizeof(client_id), "%p", (void*)task->conn);
                send_final_result(client_id, false, total_count, 0, 0, NULL);
            }
            
            batch_delete_recordings_ws_task_free(task);
            if (release_needed) {
                api_thread_pool_release();
            }
            return;
        }
        
        // Get all recordings matching filter
        count = get_recording_metadata_paginated(start_time, end_time, 
                                              stream_name[0] != '\0' ? stream_name : NULL,
                                              has_detection, "id", "asc",
                                              recordings, total_count, 0);
        
        if (count <= 0) {
            log_info("No recordings match the filter");
            free(recordings);
            cJSON *response = cJSON_CreateObject();
            cJSON_AddBoolToObject(response, "success", true);
            cJSON_AddNumberToObject(response, "total", 0);
            cJSON_AddNumberToObject(response, "succeeded", 0);
            cJSON_AddNumberToObject(response, "failed", 0);
            cJSON_AddItemToObject(response, "results", cJSON_CreateArray());
            
            char *json_str = cJSON_PrintUnformatted(response);
            if (!json_str) {
                log_error("Failed to convert response JSON to string");
                cJSON_Delete(json);
                cJSON_Delete(response);
                
                if (task->use_websocket && task->conn) {
                    // Convert connection pointer to client_id string
                    char client_id[32];
                    snprintf(client_id, sizeof(client_id), "%p", (void*)task->conn);
                    send_final_result(client_id, true, 0, 0, 0, "[]");
                }
                
                batch_delete_recordings_ws_task_free(task);
                if (release_needed) {
                    api_thread_pool_release();
                }
                return;
            }
            
            // Send final result via WebSocket
            if (task->use_websocket && task->conn) {
                // Convert connection pointer to client_id string
                char client_id[32];
                snprintf(client_id, sizeof(client_id), "%p", (void*)task->conn);
                send_final_result(client_id, true, 0, 0, 0, "[]");
            }
            
            free(json_str);
            cJSON_Delete(json);
            cJSON_Delete(response);
            batch_delete_recordings_ws_task_free(task);
            if (release_needed) {
                api_thread_pool_release();
            }
            return;
        }
        
        // Process each recording
        int success_count = 0;
        int error_count = 0;
        cJSON *results_array = cJSON_CreateArray();
        
        for (int i = 0; i < count; i++) {
            uint64_t id = recordings[i].id;
            
            // Delete file
            bool file_deleted = true;
            if (unlink(recordings[i].file_path) != 0) {
                log_warn("Failed to delete recording file: %s", recordings[i].file_path);
                file_deleted = false;
            } else {
                log_info("Deleted recording file: %s", recordings[i].file_path);
            }
            
            // Delete from database
            if (delete_recording_metadata(id) != 0) {
                log_error("Failed to delete recording from database: %llu", (unsigned long long)id);
                
                // Add result to array
                cJSON *result = cJSON_CreateObject();
                cJSON_AddNumberToObject(result, "id", id);
                cJSON_AddBoolToObject(result, "success", false);
                cJSON_AddStringToObject(result, "error", "Failed to delete from database");
                cJSON_AddItemToArray(results_array, result);
                
                error_count++;
                
                // Send progress update
                if (task->use_websocket && task->conn) {
                    char status[128];
                    snprintf(status, sizeof(status), "Failed to delete recording %llu from database", 
                           (unsigned long long)id);
                    send_progress_update(task->conn, i + 1, count, 
                                       success_count, error_count, status, false);
                }
            } else {
                // Add success result to array
                cJSON *result = cJSON_CreateObject();
                cJSON_AddNumberToObject(result, "id", id);
                cJSON_AddBoolToObject(result, "success", true);
                if (!file_deleted) {
                    cJSON_AddStringToObject(result, "warning", "File not deleted but removed from database");
                }
                cJSON_AddItemToArray(results_array, result);
                
                success_count++;
                log_info("Successfully deleted recording: %llu", (unsigned long long)id);
                
                // Send progress update
                if (task->use_websocket && task->conn) {
                    char status[128];
                    snprintf(status, sizeof(status), "Deleted recording %llu", (unsigned long long)id);
                    send_progress_update(task->conn, i + 1, count, 
                                       success_count, error_count, status, false);
                }
            }
            
            // Add a small delay to avoid overwhelming the WebSocket connection
            if (task->use_websocket && task->conn) {
                usleep(10000);  // 10ms delay
            }
        }
        
        // Free recordings
        free(recordings);
        
        // Create response
        cJSON *response = cJSON_CreateObject();
        cJSON_AddBoolToObject(response, "success", error_count == 0);
        cJSON_AddNumberToObject(response, "total", count);
        cJSON_AddNumberToObject(response, "succeeded", success_count);
        cJSON_AddNumberToObject(response, "failed", error_count);
        cJSON_AddItemToObject(response, "results", results_array);
        
        // Convert to string
        char *json_str = cJSON_PrintUnformatted(response);
        if (!json_str) {
            log_error("Failed to convert response JSON to string");
            cJSON_Delete(json);
            cJSON_Delete(response);
            
            if (task->use_websocket && task->conn) {
                // Convert connection pointer to client_id string
                char client_id[32];
                snprintf(client_id, sizeof(client_id), "%p", (void*)task->conn);
                send_final_result(client_id, error_count == 0, count, 
                                success_count, error_count, NULL);
            }
            
            batch_delete_recordings_ws_task_free(task);
            if (release_needed) {
                api_thread_pool_release();
            }
            return;
        }
        
        // Send final result via WebSocket
        if (task->use_websocket && task->conn) {
            // Extract results array as string
            char *results_str = cJSON_PrintUnformatted(results_array);
            // Convert connection pointer to client_id string
            char client_id[32];
            snprintf(client_id, sizeof(client_id), "%p", (void*)task->conn);
            if (results_str) {
                send_final_result(client_id, error_count == 0, count, 
                                success_count, error_count, results_str);
                free(results_str);
            } else {
                send_final_result(client_id, error_count == 0, count, 
                                success_count, error_count, "[]");
            }
        }
        
        // Clean up
        free(json_str);
        cJSON_Delete(json);
        cJSON_Delete(response);
        
        log_info("Successfully handled batch delete by filter request: %d succeeded, %d failed", 
                success_count, error_count);
    } else {
        log_error("Request must contain either 'ids' array or 'filter' object");
        cJSON_Delete(json);
        
        if (task->use_websocket && task->conn) {
            // Convert connection pointer to client_id string
            char client_id[32];
            snprintf(client_id, sizeof(client_id), "%p", (void*)task->conn);
            send_final_result(client_id, false, 0, 0, 0, NULL);
        }
        
        batch_delete_recordings_ws_task_free(task);
        if (release_needed) {
            api_thread_pool_release();
        }
        return;
    }
    
    // Send completion update
    if (task->use_websocket && task->conn && !is_shutdown_initiated()) {
        //  Check for shutdown before sending completion update
        // Send a final progress update with the complete flag set to true
        send_progress_update(task->conn, 0, 0, 0, 0, "Operation complete", true);
        
        // Add a small delay to ensure the final message is sent
        usleep(10000);  // 10ms delay
        
        // Note: We no longer send a duplicate final result message as it can confuse the frontend
        // The frontend should have already received the actual result message with the full results
    }
    
    batch_delete_recordings_ws_task_free(task);
    if (release_needed) {
        api_thread_pool_release();
    }
}

/**
 * @brief WebSocket handler for batch delete recordings
 * 
 * @param client_id WebSocket client ID
 * @param message WebSocket message
 */
void websocket_handle_batch_delete_recordings(const char *client_id, const char *message) {
    if (!client_id || !message) {
        log_error("Invalid parameters for websocket_handle_batch_delete_recordings");
        return;
    }
    
    log_info("Handling WebSocket batch delete recordings request from client: %s", client_id);
    log_info("Raw message: %s", message);
    
    // Parse JSON message
    cJSON *json = cJSON_Parse(message);
    if (!json) {
        log_error("Failed to parse WebSocket message as JSON");
        
        // Send error response
        char *response = mg_websocket_message_create(
            "error", "recordings/batch-delete", 
            "{\"error\":\"Invalid JSON message\"}");
        
        if (response) {
            if (mg_websocket_message_send_to_client(client_id, response)) {
                log_info("Error response sent successfully to client %s", client_id);
            } else {
                log_error("Failed to send error response to client %s", client_id);
            }
            mg_websocket_message_free(response);
        }
        
        return;
    }
    
    // Check if this is a request message with a payload
    cJSON *type = cJSON_GetObjectItem(json, "type");
    cJSON *topic = cJSON_GetObjectItem(json, "topic");
    cJSON *payload = cJSON_GetObjectItem(json, "payload");
    
    if (type && cJSON_IsString(type) && 
        topic && cJSON_IsString(topic)) {
        
        // This is a properly formatted message with type and topic
        log_info("Received WebSocket message: type=%s, topic=%s", 
                type->valuestring, topic->valuestring);
        
        // Check if this is a request for batch delete
        if (strcmp(type->valuestring, "request") == 0 && 
            strcmp(topic->valuestring, "recordings/batch-delete") == 0) {
            
            // Ensure payload is an object
            if (!payload || !cJSON_IsObject(payload)) {
                log_warn("Request payload is missing or not an object, creating empty object");
                if (payload) {
                    // Remove invalid payload
                    cJSON_DeleteItemFromObject(json, "payload");
                }
                // Create empty payload object
                payload = cJSON_CreateObject();
                cJSON_AddItemToObject(json, "payload", payload);
            }
            
        // Log client_id from payload if present (for debugging)
        cJSON *payload_client_id = cJSON_GetObjectItem(payload, "client_id");
        if (payload_client_id && cJSON_IsString(payload_client_id)) {
            log_info("Client ID in payload: %s (using connection ID: %s for responses)", 
                    payload_client_id->valuestring, client_id);
        }
            
            // Check for ids or filter in the payload
            cJSON *ids = cJSON_GetObjectItem(payload, "ids");
            cJSON *filter = cJSON_GetObjectItem(payload, "filter");
            
            if (ids || filter) {
                log_info("Found batch delete parameters: ids=%s, filter=%s", 
                        ids ? "present" : "not present", 
                        filter ? "present" : "not present");
            } else {
                log_error("No ids or filter found in payload");
                
                // Send error response
                char *response = mg_websocket_message_create(
                    "error", "recordings/batch-delete", 
                    "{\"error\":\"Missing ids or filter in request\"}");
                
                if (response) {
                    if (mg_websocket_message_send_to_client(client_id, response)) {
                        log_info("Error response sent successfully to client %s", client_id);
                    } else {
                        log_error("Failed to send error response to client %s", client_id);
                    }
                    mg_websocket_message_free(response);
                }
                
                cJSON_Delete(json);
                return;
            }
            
            // Use the payload as the parameters for batch delete
            cJSON *params = cJSON_Duplicate(payload, 1);
            
            // Remove client_id from params if present
            cJSON_DeleteItemFromObject(params, "client_id");
            
            // Free the original JSON
            cJSON_Delete(json);
            
            // Use the params for batch delete
            json = params;
            
            // Log the parameters
            char *params_str = cJSON_PrintUnformatted(json);
            if (params_str) {
                log_info("Batch delete parameters: %s", params_str);
                free(params_str);
            }
        } else if (strcmp(type->valuestring, "subscribe") == 0 && 
                  strcmp(topic->valuestring, "recordings/batch-delete") == 0) {
            // Handle subscribe message
            log_info("Received subscribe message for recordings/batch-delete");
            
            // Send acknowledgment for subscribe message
            char *ack = mg_websocket_message_create(
                "ack", "recordings/batch-delete", 
                "{\"message\":\"Subscribed to recordings/batch-delete\"}");
            
            if (ack) {
                if (mg_websocket_message_send_to_client(client_id, ack)) {
                    log_info("Acknowledgment sent successfully to client %s", client_id);
                } else {
                    log_error("Failed to send acknowledgment to client %s", client_id);
                }
                mg_websocket_message_free(ack);
            }
            
            // Add subscription
            websocket_client_subscribe(client_id, "recordings/batch-delete");
            
            // Free resources
            cJSON_Delete(json);
            return;
        } else {
            log_warn("Unexpected WebSocket message type or topic: %s/%s - ignoring", 
                    type->valuestring, topic->valuestring);
            
            // Don't send error response for unexpected message types
            // This avoids confusing the client with error messages for messages it might not expect a response to
            
            cJSON_Delete(json);
            return;
        }
    } else {
        log_info("Message doesn't have type/topic structure, treating as direct parameters");
    }
    
    // Check if client is subscribed to the topic
    // But don't fail if not subscribed - the client might be in the process of subscribing
    if (!websocket_client_is_subscribed(client_id, "recordings/batch-delete")) {
        log_warn("Client %s is not yet subscribed to recordings/batch-delete topic, but continuing anyway", client_id);
        
        // If this is a subscribe message, we'll handle it normally
        if (type && cJSON_IsString(type) && strcmp(type->valuestring, "subscribe") == 0) {
            log_info("This is a subscribe message, so we'll process it normally");
            
            // Send acknowledgment for subscribe message
            char *ack = mg_websocket_message_create(
                "ack", "recordings/batch-delete", 
                "{\"message\":\"Subscribed to recordings/batch-delete\"}");
            
            if (ack) {
                mg_websocket_message_send_to_client(client_id, ack);
                mg_websocket_message_free(ack);
            }
            
            // Add subscription
            websocket_handler_register("recordings/batch-delete", websocket_handle_batch_delete_recordings);
            
            // Free resources
            cJSON_Delete(json);
            return;
        }
        
        // If this is any other type of message (not a request), don't process it as a batch delete
        if (type && cJSON_IsString(type) && strcmp(type->valuestring, "request") != 0) {
            log_info("Message type is not 'request', ignoring: %s", type->valuestring);
            cJSON_Delete(json);
            return;
        }
        
        // Auto-subscribe the client to the topic
        log_info("Auto-subscribing client %s to recordings/batch-delete topic", client_id);
        websocket_client_subscribe(client_id, "recordings/batch-delete");
    }
    
    // Create a copy of the JSON string for the task
    char *json_str = cJSON_PrintUnformatted(json);
    if (!json_str) {
        log_error("Failed to convert JSON to string");
        cJSON_Delete(json);
        
        // Send error response
        char *response = mg_websocket_message_create(
            "error", "recordings/batch-delete", 
            "{\"error\":\"Failed to process request\"}");
        
        if (response) {
            if (mg_websocket_message_send_to_client(client_id, response)) {
                log_info("Error response sent successfully to client %s", client_id);
            } else {
                log_error("Failed to send error response to client %s", client_id);
            }
            mg_websocket_message_free(response);
        }
        
        return;
    }
    
    // Free the original JSON as we now have a string copy
    cJSON_Delete(json);
    
    // Send acknowledgment before starting the task
    char *ack_response = mg_websocket_message_create(
        "ack", "recordings/batch-delete", 
        "{\"message\":\"Batch delete operation started\"}");
    
    if (ack_response) {
                if (mg_websocket_message_send_to_client(client_id, ack_response)) {
            log_info("Acknowledgment sent successfully to client %s", client_id);
        } else {
            log_error("Failed to send acknowledgment to client %s", client_id);
        }
        mg_websocket_message_free(ack_response);
    }
    
    // Get connection from client_id
    struct mg_connection *conn = NULL;
    if (sscanf(client_id, "%p", &conn) != 1 || !conn) {
        log_error("Invalid client ID or connection not found: %s", client_id);
        free(json_str);
        
        // Send error response
        char *response = mg_websocket_message_create(
            "error", "recordings/batch-delete", 
            "{\"error\":\"Invalid client ID or connection not found\"}");
        
        if (response) {
            if (mg_websocket_message_send_to_client(client_id, response)) {
                log_info("Error response sent successfully to client %s", client_id);
            } else {
                log_error("Failed to send error response to client %s", client_id);
            }
            mg_websocket_message_free(response);
        }
        
        return;
    }
    
    // Create a task for the batch delete operation
    batch_delete_recordings_ws_task_t *task = batch_delete_recordings_ws_task_create(
        client_id, json_str, true, conn);
    
    if (!task) {
        log_error("Failed to create batch delete recordings task");
        free(json_str);
        
        // Send error response
        char *response = mg_websocket_message_create(
            "error", "recordings/batch-delete", 
            "{\"error\":\"Failed to create task\"}");
        
        if (response) {
            if (mg_websocket_message_send_to_client(client_id, response)) {
                log_info("Error response sent successfully to client %s", client_id);
            } else {
                log_error("Failed to send error response to client %s", client_id);
            }
            mg_websocket_message_free(response);
        }
        
        return;
    }
    
    // Create a thread to handle the batch delete operation
    pthread_t thread_id;
    if (pthread_create(&thread_id, NULL, (void *(*)(void *))batch_delete_recordings_ws_task_function, task) != 0) {
        log_error("Failed to create thread for batch delete recordings task");
        batch_delete_recordings_ws_task_free(task);
        
        // Send error response
        char *response = mg_websocket_message_create(
            "error", "recordings/batch-delete", 
            "{\"error\":\"Failed to create thread for task\"}");
        
        if (response) {
            if (mg_websocket_message_send_to_client(client_id, response)) {
                log_info("Error response sent successfully to client %s", client_id);
            } else {
                log_error("Failed to send error response to client %s", client_id);
            }
            mg_websocket_message_free(response);
        }
        
        return;
    }
    
    // Detach the thread so it can clean up itself when done
    pthread_detach(thread_id);
    
    log_info("Batch delete recordings task started in a separate thread");
}

/**
 * @brief HTTP handler for batch delete recordings with WebSocket support
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_batch_delete_recordings_ws(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/recordings/batch-delete-ws request");
    
    // Get request body
    char *body = NULL;
    if (hm->body.len > 0) {
        body = malloc(hm->body.len + 1);
        if (!body) {
            log_error("Failed to allocate memory for request body");
            mg_send_json_error(c, 500, "Failed to allocate memory for request body");
            return;
        }
        
        memcpy(body, mg_str_get_ptr(&hm->body), hm->body.len);
        body[hm->body.len] = '\0';
    } else {
        log_error("Empty request body");
        mg_send_json_error(c, 400, "Empty request body");
        return;
    }
    
    // Parse JSON to extract WebSocket client ID
    cJSON *json = cJSON_Parse(body);
    if (!json) {
        log_error("Failed to parse JSON body");
        free(body);
        mg_send_json_error(c, 400, "Invalid JSON body");
        return;
    }
    
    // Extract client ID
    cJSON *client_id_json = cJSON_GetObjectItem(json, "client_id");
    if (!client_id_json || !cJSON_IsString(client_id_json)) {
        log_error("Missing or invalid client_id in request");
        cJSON_Delete(json);
        free(body);
        mg_send_json_error(c, 400, "Missing or invalid client_id in request");
        return;
    }
    
    const char *client_id = client_id_json->valuestring;
    
    // Check if client is connected
    if (!websocket_client_is_subscribed(client_id, "recordings/batch-delete")) {
        log_error("WebSocket client not subscribed: %s", client_id);
        cJSON_Delete(json);
        free(body);
        mg_send_json_error(c, 400, "WebSocket client not subscribed to recordings/batch-delete topic");
        return;
    }
    
    // Acquire thread pool
    thread_pool_t *pool = api_thread_pool_acquire(4, 10);
    if (!pool) {
        log_error("Failed to acquire thread pool");
        cJSON_Delete(json);
        free(body);
        mg_send_json_error(c, 500, "Failed to acquire thread pool");
        return;
    }
    
    // Get connection from client_id
    struct mg_connection *conn = websocket_client_get_connection(client_id);
    if (!conn) {
        log_error("WebSocket connection not found for client: %s", client_id);
        cJSON_Delete(json);
        free(body);
        api_thread_pool_release();
        mg_send_json_error(c, 400, "WebSocket connection not found for client");
        return;
    }
    
    // Create task
    batch_delete_recordings_ws_task_t *task = batch_delete_recordings_ws_task_create(
        client_id, body, true, conn);
    
    if (!task) {
        log_error("Failed to create batch delete recordings task");
        cJSON_Delete(json);
        free(body);
        api_thread_pool_release();
        mg_send_json_error(c, 500, "Failed to create batch delete recordings task");
        return;
    }
    
    // Add task to thread pool
    if (!thread_pool_add_task(pool, batch_delete_recordings_ws_task_function, task)) {
        log_error("Failed to add batch delete recordings task to thread pool");
        batch_delete_recordings_ws_task_free(task);
        cJSON_Delete(json);
        free(body);
        api_thread_pool_release();
        mg_send_json_error(c, 500, "Failed to add batch delete recordings task to thread pool");
        return;
    }
    
    // Free resources
    cJSON_Delete(json);
    free(body);
    
    // Send success response
    mg_send_json_response(c, 200, "{\"success\":true,\"message\":\"Batch delete operation started\"}");
    
    log_info("Batch delete recordings task added to thread pool");
}
