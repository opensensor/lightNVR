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
#include <errno.h>

#include "web/api_handlers.h"
#include "web/mongoose_adapter.h"
#include "web/batch_delete_progress.h"
#include "core/logger.h"
#include "core/config.h"
#include "mongoose.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"
#include "web/mongoose_server_multithreading.h"
#include <pthread.h>

/**
 * @brief Data structure for batch delete thread
 */
typedef struct {
    char job_id[64];
    cJSON *json;  // Parsed JSON request (will be freed by thread)
} batch_delete_thread_data_t;

/**
 * @brief Thread function to perform batch delete with progress updates
 *
 * @param arg Pointer to batch_delete_thread_data_t
 * @return NULL
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
                    // Then delete the file from disk
                    struct stat st;
                    if (stat(file_path_copy, &st) == 0) {
                        if (unlink(file_path_copy) != 0) {
                            log_warn("Failed to delete recording file: %s (error: %s)",
                                    file_path_copy, strerror(errno));
                            // File deletion failed but DB entry is already removed
                        } else {
                            log_info("Deleted recording file: %s", file_path_copy);
                        }
                    } else {
                        log_warn("Recording file does not exist: %s (already deleted or never created)",
                                file_path_copy);
                    }

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

        // Extract filter parameters (same as before)
        cJSON *start = cJSON_GetObjectItem(filter, "start");
        cJSON *end = cJSON_GetObjectItem(filter, "end");
        cJSON *stream = cJSON_GetObjectItem(filter, "stream_name");
        if (!stream) {
            stream = cJSON_GetObjectItem(filter, "stream");
        }
        cJSON *detection = cJSON_GetObjectItem(filter, "detection");

        if (start && cJSON_IsString(start)) {
            char decoded_start_time[64] = {0};
            strncpy(decoded_start_time, start->valuestring, sizeof(decoded_start_time) - 1);
            char *pos = decoded_start_time;
            while ((pos = strstr(pos, "%3A")) != NULL) {
                *pos = ':';
                memmove(pos + 1, pos + 3, strlen(pos + 3) + 1);
            }
            struct tm tm = {0};
            if (strptime(decoded_start_time, "%Y-%m-%dT%H:%M:%S", &tm) != NULL ||
                strptime(decoded_start_time, "%Y-%m-%dT%H:%M:%S.000Z", &tm) != NULL ||
                strptime(decoded_start_time, "%Y-%m-%dT%H:%M:%S.000", &tm) != NULL ||
                strptime(decoded_start_time, "%Y-%m-%dT%H:%M:%SZ", &tm) != NULL) {
                tm.tm_isdst = -1;
                start_time = mktime(&tm);
            }
        }

        if (end && cJSON_IsString(end)) {
            char decoded_end_time[64] = {0};
            strncpy(decoded_end_time, end->valuestring, sizeof(decoded_end_time) - 1);
            char *pos = decoded_end_time;
            while ((pos = strstr(pos, "%3A")) != NULL) {
                *pos = ':';
                memmove(pos + 1, pos + 3, strlen(pos + 3) + 1);
            }
            struct tm tm = {0};
            if (strptime(decoded_end_time, "%Y-%m-%dT%H:%M:%S", &tm) != NULL ||
                strptime(decoded_end_time, "%Y-%m-%dT%H:%M:%S.000Z", &tm) != NULL ||
                strptime(decoded_end_time, "%Y-%m-%dT%H:%M:%S.000", &tm) != NULL ||
                strptime(decoded_end_time, "%Y-%m-%dT%H:%M:%SZ", &tm) != NULL) {
                tm.tm_isdst = -1;
                end_time = mktime(&tm);
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
                                            has_detection);

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
                                                  has_detection, "id", "asc",
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
                // Then delete the file from disk
                struct stat st;
                if (stat(file_path_copy, &st) == 0) {
                    if (unlink(file_path_copy) != 0) {
                        log_warn("Failed to delete recording file: %s (error: %s)",
                                file_path_copy, strerror(errno));
                        // File deletion failed but DB entry is already removed
                    } else {
                        log_info("Deleted recording file: %s", file_path_copy);
                    }
                } else {
                    log_warn("Recording file does not exist: %s (already deleted or never created)",
                            file_path_copy);
                }

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
 * @brief Batch delete recordings task function
 *
 * This function is called by the multithreading system to handle batch delete recordings requests.
 *
 * @param c Mongoose connection
 * @param hm HTTP message
 */
void batch_delete_recordings_task_function(struct mg_connection *c, struct mg_http_message *hm) {
    // Get request body
    char *body = NULL;
    if (hm->body.len > 0) {
        body = malloc(hm->body.len + 1);
        if (!body) {
            log_error("Failed to allocate memory for request body");
            mg_send_json_error(c, 500, "Failed to allocate memory for request body");
            return;
        }

        memcpy(body, hm->body.buf, hm->body.len);
        body[hm->body.len] = '\0';
    } else {
        log_error("Empty request body");
        mg_send_json_error(c, 400, "Empty request body");
        return;
    }

    // Parse JSON request
    cJSON *json = cJSON_Parse(body);
    if (!json) {
        log_error("Failed to parse JSON body");
        free(body);
        mg_send_json_error(c, 400, "Invalid JSON body");
        return;
    }

    // Free the body string as we've parsed it
    free(body);

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
            mg_send_json_error(c, 400, "Empty 'ids' array");
            return;
        }
    } else if (filter && cJSON_IsObject(filter)) {
        // Delete by filter - just get the count for now
        // The worker thread will do the actual parsing and deletion
        total_count = 0;  // Will be determined by worker thread
    } else {
        log_error("Request must contain either 'ids' array or 'filter' object");
        cJSON_Delete(json);
        mg_send_json_error(c, 400, "Request must contain either 'ids' array or 'filter' object");
        return;
    }

    // Create a batch delete job
    char job_id[64];
    if (batch_delete_progress_create_job(total_count, job_id) != 0) {
        log_error("Failed to create batch delete job");
        cJSON_Delete(json);
        mg_send_json_error(c, 500, "Failed to create batch delete job");
        return;
    }

    log_info("Created batch delete job: %s (total: %d)", job_id, total_count);

    // Prepare thread data
    batch_delete_thread_data_t *thread_data = (batch_delete_thread_data_t *)malloc(sizeof(batch_delete_thread_data_t));
    if (!thread_data) {
        log_error("Failed to allocate memory for thread data");
        batch_delete_progress_error(job_id, "Failed to allocate memory");
        cJSON_Delete(json);
        mg_send_json_error(c, 500, "Failed to allocate memory");
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
        mg_send_json_error(c, 500, "Failed to create worker thread");
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
        mg_send_json_error(c, 500, "Failed to create response");
        return;
    }

    mg_send_json_response(c, 202, json_str);  // 202 Accepted

    free(json_str);
    cJSON_Delete(response);

    log_info("Batch delete job started: %s", job_id);
}

/**
 * @brief Direct handler for POST /api/recordings/batch-delete
 */
void mg_handle_batch_delete_recordings(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/recordings/batch-delete request");

    // Process the deletion synchronously and send response when complete
    // This ensures the client receives the actual results
    batch_delete_recordings_task_function(c, hm);
}

/**
 * @brief Direct handler for GET /api/recordings/batch-delete/progress/:job_id
 */
void mg_handle_batch_delete_progress(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/recordings/batch-delete/progress request");

    // Extract job ID from URL
    // URL format: /api/recordings/batch-delete/progress/:job_id
    const char *prefix = "/api/recordings/batch-delete/progress/";
    size_t prefix_len = strlen(prefix);

    if (hm->uri.len <= prefix_len) {
        log_error("Missing job ID in URL");
        mg_send_json_error(c, 400, "Missing job ID");
        return;
    }

    // Extract job ID
    char job_id[64] = {0};
    size_t job_id_len = hm->uri.len - prefix_len;
    if (job_id_len >= sizeof(job_id)) {
        log_error("Job ID too long");
        mg_send_json_error(c, 400, "Invalid job ID");
        return;
    }

    memcpy(job_id, hm->uri.buf + prefix_len, job_id_len);
    job_id[job_id_len] = '\0';

    log_info("Getting progress for job: %s", job_id);

    // Get progress information
    batch_delete_progress_t progress;
    if (batch_delete_progress_get(job_id, &progress) != 0) {
        log_error("Job not found: %s", job_id);
        mg_send_json_error(c, 404, "Job not found");
        return;
    }

    // Create JSON response
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        log_error("Failed to create JSON response");
        mg_send_json_error(c, 500, "Failed to create response");
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
        mg_send_json_error(c, 500, "Failed to create response");
        return;
    }

    // Send response
    mg_send_json_response(c, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(response);
}
