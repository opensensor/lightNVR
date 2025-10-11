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
#include "core/logger.h"
#include "core/config.h"
#include "mongoose.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"
#include "web/mongoose_server_multithreading.h"

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

    // Check if we're deleting by IDs or by filter
    cJSON *ids_array = cJSON_GetObjectItem(json, "ids");
    cJSON *filter = cJSON_GetObjectItem(json, "filter");

    if (ids_array && cJSON_IsArray(ids_array)) {
        // Delete by IDs
        int array_size = cJSON_GetArraySize(ids_array);
        if (array_size == 0) {
            log_warn("Empty 'ids' array in batch delete request");
            cJSON_Delete(json);
            free(body);
            mg_send_json_error(c, 400, "Empty 'ids' array");
            return;
        }

        // Process each ID
        int success_count = 0;
        int error_count = 0;
        cJSON *results_array = cJSON_CreateArray();

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

                // Add result to array
                cJSON *result = cJSON_CreateObject();
                cJSON_AddNumberToObject(result, "id", id);
                cJSON_AddBoolToObject(result, "success", false);
                cJSON_AddStringToObject(result, "error", "Recording not found");
                cJSON_AddItemToArray(results_array, result);

                error_count++;
                continue;
            }

            // First delete from database to prevent any other operations on this recording
            if (delete_recording_metadata(id) != 0) {
                log_error("Failed to delete recording from database: %llu", (unsigned long long)id);

                // Add result to array
                cJSON *result = cJSON_CreateObject();
                cJSON_AddNumberToObject(result, "id", id);
                cJSON_AddBoolToObject(result, "success", false);
                cJSON_AddStringToObject(result, "error", "Failed to delete from database");
                cJSON_AddItemToArray(results_array, result);

                error_count++;
            } else {
                // Then delete the file after the database entry is gone
                bool file_deleted = true;
                // Make a copy of the file path before attempting to delete
                char file_path_copy[256];
                strncpy(file_path_copy, recording.file_path, sizeof(file_path_copy) - 1);
                file_path_copy[sizeof(file_path_copy) - 1] = '\0';

                // Check if file exists before attempting to delete
                struct stat st;
                if (stat(file_path_copy, &st) != 0) {
                    log_warn("Recording file does not exist: %s (errno: %d)", file_path_copy, errno);
                    file_deleted = false;
                } else if (unlink(file_path_copy) != 0) {
                    log_warn("Failed to delete recording file: %s (errno: %d)", file_path_copy, errno);
                    file_deleted = false;
                } else {
                    log_info("Deleted recording file: %s", file_path_copy);
                }
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
            free(body);
            mg_send_json_error(c, 500, "Failed to create response");
            return;
        }

        // Send response
        mg_send_json_response(c, 200, json_str);

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
        // Accept both 'stream' and 'stream_name' for backwards compatibility
        cJSON *stream = cJSON_GetObjectItem(filter, "stream_name");
        if (!stream) {
            stream = cJSON_GetObjectItem(filter, "stream");
        }
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
                free(body);
                mg_send_json_error(c, 500, "Failed to create response");
                return;
            }

            mg_send_json_response(c, 200, json_str);
            free(json_str);
            cJSON_Delete(json);
            cJSON_Delete(response);
            free(body);
            return;
        }

        // Allocate memory for recordings
        recordings = (recording_metadata_t *)malloc(total_count * sizeof(recording_metadata_t));
        if (!recordings) {
            log_error("Failed to allocate memory for recordings");
            cJSON_Delete(json);
            free(body);
            mg_send_json_error(c, 500, "Failed to allocate memory for recordings");
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
                free(body);
                mg_send_json_error(c, 500, "Failed to create response");
                return;
            }

            mg_send_json_response(c, 200, json_str);
            free(json_str);
            cJSON_Delete(json);
            cJSON_Delete(response);
            free(body);
            return;
        }

        // Process each recording
        int success_count = 0;
        int error_count = 0;
        cJSON *results_array = cJSON_CreateArray();

        for (int i = 0; i < count; i++) {
            uint64_t id = recordings[i].id;

            // First delete from database to prevent any other operations on this recording
            if (delete_recording_metadata(id) != 0) {
                log_error("Failed to delete recording from database: %llu", (unsigned long long)id);

                // Add result to array
                cJSON *result = cJSON_CreateObject();
                cJSON_AddNumberToObject(result, "id", id);
                cJSON_AddBoolToObject(result, "success", false);
                cJSON_AddStringToObject(result, "error", "Failed to delete from database");
                cJSON_AddItemToArray(results_array, result);

                error_count++;
            } else {
                // Then delete the file after the database entry is gone
                bool file_deleted = true;
                // Make a copy of the file path before attempting to delete
                char file_path_copy[256];
                strncpy(file_path_copy, recordings[i].file_path, sizeof(file_path_copy) - 1);
                file_path_copy[sizeof(file_path_copy) - 1] = '\0';

                // Check if file exists before attempting to delete
                struct stat st;
                if (stat(file_path_copy, &st) != 0) {
                    log_warn("Recording file does not exist: %s (errno: %d)", file_path_copy, errno);
                    file_deleted = false;
                } else if (unlink(file_path_copy) != 0) {
                    log_warn("Failed to delete recording file: %s (errno: %d)", file_path_copy, errno);
                    file_deleted = false;
                } else {
                    log_info("Deleted recording file: %s", file_path_copy);
                }

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
            free(body);
            mg_send_json_error(c, 500, "Failed to create response");
            return;
        }

        // Send response
        mg_send_json_response(c, 200, json_str);

        // Clean up
        free(json_str);
        cJSON_Delete(json);
        cJSON_Delete(response);

        log_info("Successfully handled batch delete by filter request: %d succeeded, %d failed",
                success_count, error_count);
    } else {
        log_error("Request must contain either 'ids' array or 'filter' object");
        cJSON_Delete(json);
        free(body);
        mg_send_json_error(c, 400, "Request must contain either 'ids' array or 'filter' object");
        return;
    }

    // Free the request body
    free(body);
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
