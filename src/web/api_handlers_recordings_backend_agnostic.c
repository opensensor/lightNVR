/**
 * @file api_handlers_recordings_backend_agnostic.c
 * @brief Backend-agnostic handlers for recording operations (get, delete, batch delete)
 */

#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <errno.h>
#include <pthread.h>
#include <cjson/cJSON.h>

#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "web/batch_delete_progress.h"
#include "core/logger.h"
#include "core/config.h"
#include "core/shutdown_coordinator.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"
#include "database/db_detections.h"
#include "database/db_auth.h"
#include "web/api_handlers_recordings_thumbnail.h"

/**
 * @brief Backend-agnostic handler for GET /api/recordings/:id
 * 
 * Gets detailed information about a single recording.
 */
void handle_get_recording(const http_request_t *req, http_response_t *res) {
    // Check if shutdown is in progress
    if (is_shutdown_initiated()) {
        log_debug("Shutdown in progress, rejecting recording detail request");
        http_response_set_json_error(res, 503, "Service shutting down");
        return;
    }

    // Extract recording ID from URL
    char id_str[32];
    if (http_request_extract_path_param(req, "/api/recordings/", id_str, sizeof(id_str)) != 0) {
        log_error("Failed to extract recording ID from URL");
        http_response_set_json_error(res, 400, "Invalid request path");
        return;
    }
    
    // Convert ID to integer
    uint64_t id = strtoull(id_str, NULL, 10);
    if (id == 0) {
        log_error("Invalid recording ID: %s", id_str);
        http_response_set_json_error(res, 400, "Invalid recording ID");
        return;
    }
    
    log_debug("Handling GET /api/recordings/%llu request", (unsigned long long)id);
    
    // Get recording from database
    recording_metadata_t recording;
    if (get_recording_metadata_by_id(id, &recording) != 0) {
        log_error("Recording not found: %llu", (unsigned long long)id);
        http_response_set_json_error(res, 404, "Recording not found");
        return;
    }
    
    // Create JSON object
    cJSON *recording_obj = cJSON_CreateObject();
    if (!recording_obj) {
        log_error("Failed to create recording JSON object");
        http_response_set_json_error(res, 500, "Failed to create recording JSON");
        return;
    }
    
    // Format timestamps as ISO 8601 UTC (compatible with all browsers including Safari)
    char start_time_str[32] = {0};
    char end_time_str[32] = {0};
    struct tm *tm_info;

    tm_info = gmtime(&recording.start_time);
    if (tm_info) {
        strftime(start_time_str, sizeof(start_time_str), "%Y-%m-%dT%H:%M:%SZ", tm_info);
    }

    tm_info = gmtime(&recording.end_time);
    if (tm_info) {
        strftime(end_time_str, sizeof(end_time_str), "%Y-%m-%dT%H:%M:%SZ", tm_info);
    }
    
    // Calculate duration in seconds
    int duration = (int)difftime(recording.end_time, recording.start_time);
    
    // Format file size for display
    char size_str[32] = {0};
    if (recording.size_bytes < 1024) {
        snprintf(size_str, sizeof(size_str), "%ld B", recording.size_bytes);
    } else if (recording.size_bytes < (uint64_t)1024 * 1024) {
        snprintf(size_str, sizeof(size_str), "%.1f KB", recording.size_bytes / 1024.0);
    } else if (recording.size_bytes < (uint64_t)1024 * 1024 * 1024) {
        snprintf(size_str, sizeof(size_str), "%.1f MB", recording.size_bytes / (1024.0 * 1024.0));
    } else {
        snprintf(size_str, sizeof(size_str), "%.1f GB", recording.size_bytes / (1024.0 * 1024.0 * 1024.0));
    }
    
    // Add recording properties
    cJSON_AddNumberToObject(recording_obj, "id", recording.id);
    cJSON_AddStringToObject(recording_obj, "stream", recording.stream_name);
    cJSON_AddStringToObject(recording_obj, "file_path", recording.file_path);
    cJSON_AddStringToObject(recording_obj, "start_time", start_time_str);
    cJSON_AddStringToObject(recording_obj, "end_time", end_time_str);
    cJSON_AddNumberToObject(recording_obj, "start_time_unix", (double)recording.start_time);
    cJSON_AddNumberToObject(recording_obj, "end_time_unix", (double)recording.end_time);
    cJSON_AddNumberToObject(recording_obj, "duration", duration);
    cJSON_AddStringToObject(recording_obj, "size", size_str);

    // Check if recording has detections and get detection labels summary
    bool has_detection_flag = (strcmp(recording.trigger_type, "detection") == 0);
    detection_label_summary_t labels[MAX_DETECTION_LABELS];
    int label_count = 0;

    if (recording.start_time > 0 && recording.end_time > 0) {
        // Get detection labels summary for this recording's time range
        label_count = get_detection_labels_summary(recording.stream_name,
                                                   recording.start_time,
                                                   recording.end_time,
                                                   labels, MAX_DETECTION_LABELS);
        if (label_count > 0) {
            has_detection_flag = true;
        } else if (!has_detection_flag) {
            // Fall back to simple check if get_detection_labels_summary returned 0
            int det_result = has_detections_in_time_range(recording.stream_name,
                                                          recording.start_time,
                                                          recording.end_time);
            if (det_result > 0) {
                has_detection_flag = true;
            }
        }
    }
    cJSON_AddBoolToObject(recording_obj, "has_detection", has_detection_flag);

    // Add detection labels array if there are any detections
    if (label_count > 0) {
        cJSON *labels_array = cJSON_CreateArray();
        if (labels_array) {
            for (int j = 0; j < label_count; j++) {
                cJSON *label_obj = cJSON_CreateObject();
                if (label_obj) {
                    cJSON_AddStringToObject(label_obj, "label", labels[j].label);
                    cJSON_AddNumberToObject(label_obj, "count", labels[j].count);
                    cJSON_AddItemToArray(labels_array, label_obj);
                }
            }
            cJSON_AddItemToObject(recording_obj, "detection_labels", labels_array);
        }
    }

    // Convert to string and send response
    char *json_str = cJSON_PrintUnformatted(recording_obj);
    if (!json_str) {
        log_error("Failed to convert recording JSON to string");
        cJSON_Delete(recording_obj);
        http_response_set_json_error(res, 500, "Failed to convert recording JSON to string");
        return;
    }

    http_response_set_json(res, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(recording_obj);

    log_debug("Successfully handled GET /api/recordings/%llu request", (unsigned long long)id);
}

/**
 * @brief Check if the user has permission to delete recordings
 */
static int check_delete_permission(const http_request_t *req) {
    user_t user;

    // Get the authenticated user
    if (!httpd_get_authenticated_user(req, &user)) {
        return 0; // Not authenticated
    }

    // Only admin and regular users can delete recordings, viewers cannot
    return (user.role == USER_ROLE_ADMIN || user.role == USER_ROLE_USER);
}

/**
 * @brief Backend-agnostic handler for DELETE /api/recordings/:id
 *
 * Deletes a single recording from the database and filesystem.
 */
void handle_delete_recording(const http_request_t *req, http_response_t *res) {
    // Check authentication and permissions
    if (!check_delete_permission(req)) {
        log_error("Permission denied for DELETE /api/recordings/:id");
        http_response_set_json_error(res, 403, "Permission denied: Only admin and regular users can delete recordings");
        return;
    }

    // Extract recording ID from URL
    char id_str[32];
    if (http_request_extract_path_param(req, "/api/recordings/", id_str, sizeof(id_str)) != 0) {
        log_error("Failed to extract recording ID from URL");
        http_response_set_json_error(res, 400, "Invalid request path");
        return;
    }

    // Convert ID to integer
    uint64_t id = strtoull(id_str, NULL, 10);
    if (id == 0) {
        log_error("Invalid recording ID: %s", id_str);
        http_response_set_json_error(res, 400, "Invalid recording ID");
        return;
    }

    log_info("Handling DELETE /api/recordings/%llu request", (unsigned long long)id);

    // Get recording from database
    recording_metadata_t recording;
    memset(&recording, 0, sizeof(recording_metadata_t));

    if (get_recording_metadata_by_id(id, &recording) != 0) {
        log_error("Recording not found: %llu", (unsigned long long)id);
        http_response_set_json_error(res, 404, "Recording not found");
        return;
    }

    // Save file path before deleting from database
    char file_path_copy[256];
    strncpy(file_path_copy, recording.file_path, sizeof(file_path_copy) - 1);
    file_path_copy[sizeof(file_path_copy) - 1] = '\0';

    // Delete from database FIRST
    if (delete_recording_metadata(id) != 0) {
        log_error("Failed to delete recording from database: %llu", (unsigned long long)id);
        http_response_set_json_error(res, 500, "Failed to delete recording from database");
        return;
    }

    log_info("Deleted recording from database: %llu", (unsigned long long)id);

    // Then delete the file from disk.
    // Attempt unlink directly instead of stat-then-unlink to avoid TOCTOU (#38).
    if (unlink(file_path_copy) != 0) {
        if (errno == ENOENT) {
            log_warn("Recording file does not exist: %s (already deleted or never created)", file_path_copy);
            // This is acceptable - DB entry is removed
        } else {
            log_warn("Failed to delete recording file: %s (error: %s)",
                    file_path_copy, strerror(errno));
            // File deletion failed but DB entry is already removed
            // This is acceptable - orphaned files can be cleaned up later
        }
    } else {
        log_info("Deleted recording file: %s", file_path_copy);
    }

    // Delete associated thumbnails
    delete_recording_thumbnails(id);

    // Send success response
    http_response_set_json(res, 200, "{\"success\":true,\"message\":\"Recording deleted successfully\"}");

    log_info("Successfully deleted recording: %llu", (unsigned long long)id);
}

/**
 * @brief Data structure for batch delete thread
 */
typedef struct {
    char job_id[64];
    cJSON *json;  // Parsed JSON request (will be freed by thread)
} batch_delete_thread_data_t;

/**
 * @brief Thread function to perform batch delete with progress updates
 */
static void *batch_delete_worker_thread(void *arg) {
    batch_delete_thread_data_t *data = (batch_delete_thread_data_t *)arg;
    if (!data) {
        log_error("Invalid thread data");
        return NULL;
    }

    char *job_id = data->job_id;
    cJSON *json = data->json;

    log_info("Batch delete worker thread started for job: %s", job_id);

    // Check if we're deleting by IDs or by filter
    cJSON *ids_array = cJSON_GetObjectItem(json, "ids");
    cJSON *filter = cJSON_GetObjectItem(json, "filter");

    if (ids_array && cJSON_IsArray(ids_array)) {
        // Delete by IDs
        int array_size = cJSON_GetArraySize(ids_array);

        // Update progress to running
        batch_delete_progress_update(job_id, 0, 0, 0, "Starting batch delete operation...");

        // Process each ID
        int success_count = 0;
        int error_count = 0;

        for (int i = 0; i < array_size; i++) {
            cJSON *id_item = cJSON_GetArrayItem(ids_array, i);
            if (!id_item || !cJSON_IsNumber(id_item)) {
                log_warn("Invalid ID at index %d", i);
                error_count++;
                continue;
            }

            uint64_t id = (uint64_t)id_item->valuedouble;

            // Get recording from database
            recording_metadata_t recording;
            if (get_recording_metadata_by_id(id, &recording) != 0) {
                log_warn("Recording not found: %llu", (unsigned long long)id);
                error_count++;
            } else {
                // Save file path before deleting from database
                char file_path_copy[256];
                strncpy(file_path_copy, recording.file_path, sizeof(file_path_copy) - 1);
                file_path_copy[sizeof(file_path_copy) - 1] = '\0';

                // Delete from database FIRST
                if (delete_recording_metadata(id) != 0) {
                    log_error("Failed to delete recording from database: %llu", (unsigned long long)id);
                    error_count++;
                } else {
                    // Then delete the file from disk.
                    // Attempt unlink directly instead of stat-then-unlink to avoid TOCTOU (#35).
                    if (unlink(file_path_copy) != 0) {
                        if (errno == ENOENT) {
                            log_warn("Recording file does not exist: %s (already deleted or never created)",
                                    file_path_copy);
                        } else {
                            log_warn("Failed to delete recording file: %s (error: %s)",
                                    file_path_copy, strerror(errno));
                        }
                    } else {
                        log_info("Deleted recording file: %s", file_path_copy);
                    }

                    // Delete associated thumbnails
                    delete_recording_thumbnails(id);

                    success_count++;
                    log_info("Successfully deleted recording: %llu", (unsigned long long)id);
                }
            }

            // Update progress every 10 recordings or on last recording
            if ((i + 1) % 10 == 0 || (i + 1) == array_size) {
                char status_msg[256];
                snprintf(status_msg, sizeof(status_msg), "Deleting recordings... %d/%d", i + 1, array_size);
                batch_delete_progress_update(job_id, i + 1, success_count, error_count, status_msg);
            }
        }

        // Mark as complete
        batch_delete_progress_complete(job_id, success_count, error_count);
        log_info("Batch delete job completed: %s (succeeded: %d, failed: %d)", job_id, success_count, error_count);

    } else if (filter && cJSON_IsObject(filter)) {
        // Delete by filter
        time_t start_time = 0;
        time_t end_time = 0;
        char stream_name[64] = {0};
        int has_detection = 0;

        // Extract filter parameters
        cJSON *start = cJSON_GetObjectItem(filter, "start");
        cJSON *end = cJSON_GetObjectItem(filter, "end");
        cJSON *stream = cJSON_GetObjectItem(filter, "stream_name");
        if (!stream) {
            stream = cJSON_GetObjectItem(filter, "stream");
        }
        cJSON *detection = cJSON_GetObjectItem(filter, "detection");

        if (start && cJSON_IsString(start)) {
            struct tm tm = {0};
            if (strptime(start->valuestring, "%Y-%m-%dT%H:%M:%S", &tm) != NULL ||
                strptime(start->valuestring, "%Y-%m-%dT%H:%M:%S.000Z", &tm) != NULL ||
                strptime(start->valuestring, "%Y-%m-%dT%H:%M:%S.000", &tm) != NULL ||
                strptime(start->valuestring, "%Y-%m-%dT%H:%M:%SZ", &tm) != NULL) {
                tm.tm_isdst = 0;
                start_time = timegm(&tm);
            }
        }

        if (end && cJSON_IsString(end)) {
            struct tm tm = {0};
            if (strptime(end->valuestring, "%Y-%m-%dT%H:%M:%S", &tm) != NULL ||
                strptime(end->valuestring, "%Y-%m-%dT%H:%M:%S.000Z", &tm) != NULL ||
                strptime(end->valuestring, "%Y-%m-%dT%H:%M:%S.000", &tm) != NULL ||
                strptime(end->valuestring, "%Y-%m-%dT%H:%M:%SZ", &tm) != NULL) {
                tm.tm_isdst = 0;
                end_time = timegm(&tm);
            }
        }

        if (stream && cJSON_IsString(stream)) {
            strncpy(stream_name, stream->valuestring, sizeof(stream_name) - 1);
        }

        if (detection && cJSON_IsNumber(detection)) {
            has_detection = detection->valueint;
        }

        // Get total count
        int total_count = get_recording_count(start_time, end_time,
                                            stream_name[0] != '\0' ? stream_name : NULL,
                                            has_detection, NULL);

        if (total_count <= 0) {
            batch_delete_progress_complete(job_id, 0, 0);
            cJSON_Delete(json);
            free(data);
            return NULL;
        }

        // Update progress
        batch_delete_progress_update(job_id, 0, 0, 0, "Loading recordings to delete...");

        // Allocate memory for recordings
        recording_metadata_t *recordings = (recording_metadata_t *)malloc(total_count * sizeof(recording_metadata_t));
        if (!recordings) {
            log_error("Failed to allocate memory for recordings");
            batch_delete_progress_error(job_id, "Failed to allocate memory");
            cJSON_Delete(json);
            free(data);
            return NULL;
        }

        // Get all recordings
        int count = get_recording_metadata_paginated(start_time, end_time,
                                                  stream_name[0] != '\0' ? stream_name : NULL,
                                                  has_detection, NULL, "id", "asc",
                                                  recordings, total_count, 0);

        if (count <= 0) {
            free(recordings);
            batch_delete_progress_complete(job_id, 0, 0);
            cJSON_Delete(json);
            free(data);
            return NULL;
        }

        // Process each recording
        int success_count = 0;
        int error_count = 0;

        for (int i = 0; i < count; i++) {
            uint64_t id = recordings[i].id;

            // Save file path before deleting from database
            char file_path_copy[256];
            strncpy(file_path_copy, recordings[i].file_path, sizeof(file_path_copy) - 1);
            file_path_copy[sizeof(file_path_copy) - 1] = '\0';

            // Delete from database FIRST
            if (delete_recording_metadata(id) != 0) {
                log_error("Failed to delete recording from database: %llu", (unsigned long long)id);
                error_count++;
            } else {
                // Then delete the file from disk.
                // Attempt unlink directly instead of stat-then-unlink to avoid TOCTOU (#37).
                if (unlink(file_path_copy) != 0) {
                    if (errno == ENOENT) {
                        log_warn("Recording file does not exist: %s (already deleted or never created)",
                                file_path_copy);
                    } else {
                        log_warn("Failed to delete recording file: %s (error: %s)",
                                file_path_copy, strerror(errno));
                    }
                } else {
                    log_info("Deleted recording file: %s", file_path_copy);
                }

                // Delete associated thumbnails
                delete_recording_thumbnails(id);

                success_count++;
                log_info("Successfully deleted recording: %llu", (unsigned long long)id);
            }

            // Update progress every 10 recordings or on last recording
            if ((i + 1) % 10 == 0 || (i + 1) == count) {
                char status_msg[256];
                snprintf(status_msg, sizeof(status_msg), "Deleting recordings... %d/%d", i + 1, count);
                batch_delete_progress_update(job_id, i + 1, success_count, error_count, status_msg);
            }
        }

        free(recordings);
        batch_delete_progress_complete(job_id, success_count, error_count);
        log_info("Batch delete job completed: %s (succeeded: %d, failed: %d)", job_id, success_count, error_count);
    } else {
        log_error("Invalid request format");
        batch_delete_progress_error(job_id, "Invalid request format");
    }

    // Cleanup
    cJSON_Delete(json);
    free(data);

    return NULL;
}

/**
 * @brief Backend-agnostic handler for POST /api/recordings/batch-delete
 *
 * Initiates a batch delete operation and returns a job ID for progress tracking.
 */
void handle_batch_delete_recordings(const http_request_t *req, http_response_t *res) {
    log_info("Handling POST /api/recordings/batch-delete request");

    // Check authentication and permissions
    if (!check_delete_permission(req)) {
        log_error("Permission denied for batch delete");
        http_response_set_json_error(res, 403, "Permission denied: Only admin and regular users can delete recordings");
        return;
    }

    // Parse JSON request
    cJSON *json = httpd_parse_json_body(req);
    if (!json) {
        log_error("Failed to parse JSON body");
        http_response_set_json_error(res, 400, "Invalid JSON body");
        return;
    }

    // Check if we're deleting by IDs or by filter
    cJSON *ids_array = cJSON_GetObjectItem(json, "ids");
    cJSON *filter = cJSON_GetObjectItem(json, "filter");

    // Determine total count for job creation
    int total_count = 0;

    if (ids_array && cJSON_IsArray(ids_array)) {
        // Delete by IDs
        total_count = cJSON_GetArraySize(ids_array);
        if (total_count == 0) {
            log_warn("Empty 'ids' array in batch delete request");
            cJSON_Delete(json);
            http_response_set_json_error(res, 400, "Empty 'ids' array");
            return;
        }
    } else if (filter && cJSON_IsObject(filter)) {
        // Delete by filter - total count will be determined by worker thread
        total_count = 0;
    } else {
        log_error("Request must contain either 'ids' array or 'filter' object");
        cJSON_Delete(json);
        http_response_set_json_error(res, 400, "Request must contain either 'ids' array or 'filter' object");
        return;
    }

    // Create a batch delete job
    char job_id[64];
    if (batch_delete_progress_create_job(total_count, job_id) != 0) {
        log_error("Failed to create batch delete job");
        cJSON_Delete(json);
        http_response_set_json_error(res, 500, "Failed to create batch delete job");
        return;
    }

    log_info("Created batch delete job: %s (total: %d)", job_id, total_count);

    // Prepare thread data
    batch_delete_thread_data_t *thread_data = (batch_delete_thread_data_t *)malloc(sizeof(batch_delete_thread_data_t));
    if (!thread_data) {
        log_error("Failed to allocate memory for thread data");
        batch_delete_progress_error(job_id, "Failed to allocate memory");
        cJSON_Delete(json);
        http_response_set_json_error(res, 500, "Failed to allocate memory");
        return;
    }

    strncpy(thread_data->job_id, job_id, sizeof(thread_data->job_id) - 1);
    thread_data->job_id[sizeof(thread_data->job_id) - 1] = '\0';
    thread_data->json = json;  // Transfer ownership to thread

    // Spawn worker thread
    pthread_t thread;
    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);

    if (pthread_create(&thread, &attr, batch_delete_worker_thread, thread_data) != 0) {
        log_error("Failed to create worker thread");
        batch_delete_progress_error(job_id, "Failed to create worker thread");
        cJSON_Delete(json);
        free(thread_data);
        pthread_attr_destroy(&attr);
        http_response_set_json_error(res, 500, "Failed to create worker thread");
        return;
    }

    pthread_attr_destroy(&attr);

    // Send immediate response with job_id
    cJSON *response = cJSON_CreateObject();
    cJSON_AddStringToObject(response, "job_id", job_id);
    cJSON_AddStringToObject(response, "status", "started");

    char *json_str = cJSON_PrintUnformatted(response);
    if (!json_str) {
        log_error("Failed to convert response JSON to string");
        cJSON_Delete(response);
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }

    http_response_set_json(res, 202, json_str);  // 202 Accepted

    free(json_str);
    cJSON_Delete(response);

    log_info("Batch delete job started: %s", job_id);
}

/**
 * @brief Backend-agnostic handler for GET /api/recordings/batch-delete/progress/:job_id
 *
 * Returns the progress status of a batch delete operation.
 */
void handle_batch_delete_progress(const http_request_t *req, http_response_t *res) {
    log_info("Handling GET /api/recordings/batch-delete/progress request");

    // Extract job ID from URL
    // URL format: /api/recordings/batch-delete/progress/:job_id
    char job_id[64] = {0};
    if (http_request_extract_path_param(req, "/api/recordings/batch-delete/progress/", job_id, sizeof(job_id)) != 0) {
        log_error("Failed to extract job ID from URL");
        http_response_set_json_error(res, 400, "Missing or invalid job ID");
        return;
    }

    log_info("Getting progress for job: %s", job_id);

    // Get progress information
    batch_delete_progress_t progress;
    if (batch_delete_progress_get(job_id, &progress) != 0) {
        log_error("Job not found: %s", job_id);
        http_response_set_json_error(res, 404, "Job not found");
        return;
    }

    // Create JSON response
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        log_error("Failed to create JSON response");
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }

    cJSON_AddStringToObject(response, "job_id", progress.job_id);

    // Add status as string
    const char *status_str = "unknown";
    switch (progress.status) {
        case BATCH_DELETE_STATUS_PENDING:
            status_str = "pending";
            break;
        case BATCH_DELETE_STATUS_RUNNING:
            status_str = "running";
            break;
        case BATCH_DELETE_STATUS_COMPLETE:
            status_str = "complete";
            break;
        case BATCH_DELETE_STATUS_ERROR:
            status_str = "error";
            break;
    }
    cJSON_AddStringToObject(response, "status", status_str);

    cJSON_AddNumberToObject(response, "total", progress.total);
    cJSON_AddNumberToObject(response, "current", progress.current);
    cJSON_AddNumberToObject(response, "succeeded", progress.succeeded);
    cJSON_AddNumberToObject(response, "failed", progress.failed);
    cJSON_AddStringToObject(response, "status_message", progress.status_message);

    if (progress.error_message[0] != '\0') {
        cJSON_AddStringToObject(response, "error_message", progress.error_message);
    }

    cJSON_AddBoolToObject(response, "complete",
                         progress.status == BATCH_DELETE_STATUS_COMPLETE ||
                         progress.status == BATCH_DELETE_STATUS_ERROR);

    // Convert to string
    char *json_str = cJSON_PrintUnformatted(response);
    if (!json_str) {
        log_error("Failed to convert response JSON to string");
        cJSON_Delete(response);
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }

    // Send response
    http_response_set_json(res, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(response);
}

