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
#include "core/logger.h"
#include "core/config.h"
#include "mongoose.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"

/**
 * @brief Direct handler for GET /api/recordings
 */
void mg_handle_get_recordings(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/recordings request");
    
    // Parse query parameters
    char query_string[512] = {0};
    if (hm->query.len > 0 && hm->query.len < sizeof(query_string)) {
        memcpy(query_string, mg_str_get_ptr(&hm->query), hm->query.len);
        query_string[hm->query.len] = '\0';
        log_info("Query string: %s", query_string);
    }
    
    // Extract parameters
    char stream_name[MAX_STREAM_NAME] = {0};
    char start_time_str[64] = {0};
    char end_time_str[64] = {0};
    int page = 1;
    int limit = 20;
    char sort_field[32] = "start_time";
    char sort_order[8] = "desc";
    int has_detection = 0;
    
    // Parse query string
    char *param = strtok(query_string, "&");
    while (param) {
        if (strncmp(param, "stream=", 7) == 0) {
            strncpy(stream_name, param + 7, sizeof(stream_name) - 1);
        } else if (strncmp(param, "start=", 6) == 0) {
            strncpy(start_time_str, param + 6, sizeof(start_time_str) - 1);
        } else if (strncmp(param, "end=", 4) == 0) {
            strncpy(end_time_str, param + 4, sizeof(end_time_str) - 1);
        } else if (strncmp(param, "page=", 5) == 0) {
            page = atoi(param + 5);
        } else if (strncmp(param, "limit=", 6) == 0) {
            limit = atoi(param + 6);
        } else if (strncmp(param, "sort=", 5) == 0) {
            strncpy(sort_field, param + 5, sizeof(sort_field) - 1);
        } else if (strncmp(param, "order=", 6) == 0) {
            strncpy(sort_order, param + 6, sizeof(sort_order) - 1);
        } else if (strncmp(param, "detection=", 10) == 0) {
            has_detection = atoi(param + 10);
        }
        param = strtok(NULL, "&");
    }
    
    // Validate parameters
    if (page <= 0) page = 1;
    if (limit <= 0) limit = 20;
    if (limit > 1000) limit = 1000;
    
    // Calculate offset from page and limit
    int offset = (page - 1) * limit;
    
    // Get recordings from database
    recording_metadata_t *recordings = NULL;
    int count = 0;
    int total_count = 0;
    
    // Allocate memory for recordings
    recordings = (recording_metadata_t *)malloc(limit * sizeof(recording_metadata_t));
    if (!recordings) {
        log_error("Failed to allocate memory for recordings");
        mg_send_json_error(c, 500, "Failed to allocate memory for recordings");
        return;
    }
    
    // Parse time strings to time_t
    time_t start_time = 0;
    time_t end_time = 0;
    
    if (start_time_str[0] != '\0') {
        // URL-decode the time string (replace %3A with :)
        char decoded_start_time[64] = {0};
        strncpy(decoded_start_time, start_time_str, sizeof(decoded_start_time) - 1);
        
        // Replace %3A with :
        char *pos = decoded_start_time;
        while ((pos = strstr(pos, "%3A")) != NULL) {
            *pos = ':';
            memmove(pos + 1, pos + 3, strlen(pos + 3) + 1);
        }
        
        log_info("Parsing start time string (decoded): %s", decoded_start_time);
        
        struct tm tm = {0};
        // Try different time formats
        if (strptime(decoded_start_time, "%Y-%m-%dT%H:%M:%S", &tm) != NULL ||
            strptime(decoded_start_time, "%Y-%m-%dT%H:%M:%S.000Z", &tm) != NULL ||
            strptime(decoded_start_time, "%Y-%m-%dT%H:%M:%S.000", &tm) != NULL ||
            strptime(decoded_start_time, "%Y-%m-%dT%H:%M:%SZ", &tm) != NULL) {
            
            // Set tm_isdst to -1 to let mktime determine if DST is in effect
            tm.tm_isdst = -1;
            start_time = mktime(&tm);
            log_info("Parsed start time: %ld", (long)start_time);
        } else {
            log_error("Failed to parse start time string: %s", decoded_start_time);
        }
    }
    
    if (end_time_str[0] != '\0') {
        // URL-decode the time string (replace %3A with :)
        char decoded_end_time[64] = {0};
        strncpy(decoded_end_time, end_time_str, sizeof(decoded_end_time) - 1);
        
        // Replace %3A with :
        char *pos = decoded_end_time;
        while ((pos = strstr(pos, "%3A")) != NULL) {
            *pos = ':';
            memmove(pos + 1, pos + 3, strlen(pos + 3) + 1);
        }
        
        log_info("Parsing end time string (decoded): %s", decoded_end_time);
        
        struct tm tm = {0};
        // Try different time formats
        if (strptime(decoded_end_time, "%Y-%m-%dT%H:%M:%S", &tm) != NULL ||
            strptime(decoded_end_time, "%Y-%m-%dT%H:%M:%S.000Z", &tm) != NULL ||
            strptime(decoded_end_time, "%Y-%m-%dT%H:%M:%S.000", &tm) != NULL ||
            strptime(decoded_end_time, "%Y-%m-%dT%H:%M:%SZ", &tm) != NULL) {
            
            // Set tm_isdst to -1 to let mktime determine if DST is in effect
            tm.tm_isdst = -1;
            end_time = mktime(&tm);
            log_info("Parsed end time: %ld", (long)end_time);
        } else {
            log_error("Failed to parse end time string: %s", decoded_end_time);
        }
    }
    
    // Get total count first (for pagination)
    total_count = get_recording_count(start_time, end_time, 
                                     stream_name[0] != '\0' ? stream_name : NULL,
                                     has_detection);
    
    if (total_count < 0) {
        log_error("Failed to get total recording count from database");
        free(recordings);
        mg_send_json_error(c, 500, "Failed to get recording count from database");
        return;
    }
    
    // Get recordings with pagination
    count = get_recording_metadata_paginated(start_time, end_time, 
                                           stream_name[0] != '\0' ? stream_name : NULL,
                                           has_detection, sort_field, sort_order,
                                           recordings, limit, offset);
    
    if (count < 0) {
        log_error("Failed to get recordings from database");
        mg_send_json_error(c, 500, "Failed to get recordings from database");
        return;
    }
    
    // Create response object with recordings array and pagination
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        log_error("Failed to create response JSON object");
        free(recordings);
        mg_send_json_error(c, 500, "Failed to create response JSON");
        return;
    }
    
    // Create recordings array
    cJSON *recordings_array = cJSON_CreateArray();
    if (!recordings_array) {
        log_error("Failed to create recordings JSON array");
        free(recordings);
        cJSON_Delete(response);
        mg_send_json_error(c, 500, "Failed to create recordings JSON");
        return;
    }
    
    // Add recordings array to response
    cJSON_AddItemToObject(response, "recordings", recordings_array);
    
    // Create pagination object
    cJSON *pagination = cJSON_CreateObject();
    if (!pagination) {
        log_error("Failed to create pagination JSON object");
        free(recordings);
        cJSON_Delete(response);
        mg_send_json_error(c, 500, "Failed to create pagination JSON");
        return;
    }
    
    // Add pagination info
    int total_pages = (total_count + limit - 1) / limit; // Ceiling division
    cJSON_AddNumberToObject(pagination, "page", page);
    cJSON_AddNumberToObject(pagination, "pages", total_pages);
    cJSON_AddNumberToObject(pagination, "total", total_count);
    cJSON_AddNumberToObject(pagination, "limit", limit);
    
    // Add pagination object to response
    cJSON_AddItemToObject(response, "pagination", pagination);
    
    // Add each recording to the array
    for (int i = 0; i < count; i++) {
        cJSON *recording = cJSON_CreateObject();
        if (!recording) {
            log_error("Failed to create recording JSON object");
            continue;
        }
        
        // Format timestamps
        char start_time_str[32] = {0};
        char end_time_str[32] = {0};
        struct tm *tm_info;
        
        tm_info = localtime(&recordings[i].start_time);
        if (tm_info) {
            strftime(start_time_str, sizeof(start_time_str), "%Y-%m-%d %H:%M:%S", tm_info);
        }
        
        tm_info = localtime(&recordings[i].end_time);
        if (tm_info) {
            strftime(end_time_str, sizeof(end_time_str), "%Y-%m-%d %H:%M:%S", tm_info);
        }
        
        // Calculate duration in seconds
        int duration = (int)difftime(recordings[i].end_time, recordings[i].start_time);
        
        // Format file size for display (e.g., "1.8 MB")
        char size_str[32] = {0};
        if (recordings[i].size_bytes < 1024) {
            snprintf(size_str, sizeof(size_str), "%ld B", recordings[i].size_bytes);
        } else if (recordings[i].size_bytes < 1024 * 1024) {
            snprintf(size_str, sizeof(size_str), "%.1f KB", recordings[i].size_bytes / 1024.0);
        } else if (recordings[i].size_bytes < 1024 * 1024 * 1024) {
            snprintf(size_str, sizeof(size_str), "%.1f MB", recordings[i].size_bytes / (1024.0 * 1024.0));
        } else {
            snprintf(size_str, sizeof(size_str), "%.1f GB", recordings[i].size_bytes / (1024.0 * 1024.0 * 1024.0));
        }
        
        cJSON_AddNumberToObject(recording, "id", recordings[i].id);
        cJSON_AddStringToObject(recording, "stream", recordings[i].stream_name);
        cJSON_AddStringToObject(recording, "file_path", recordings[i].file_path);
        cJSON_AddStringToObject(recording, "start_time", start_time_str);
        cJSON_AddStringToObject(recording, "end_time", end_time_str);
        cJSON_AddNumberToObject(recording, "duration", duration);
        cJSON_AddStringToObject(recording, "size", size_str);
        cJSON_AddBoolToObject(recording, "has_detection", false); // Default to false as it's not in metadata
        
        cJSON_AddItemToArray(recordings_array, recording);
    }
    
    // Free recordings
    free(recordings);
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(response);
    if (!json_str) {
        log_error("Failed to convert response JSON to string");
        cJSON_Delete(response);
        mg_send_json_error(c, 500, "Failed to convert response JSON to string");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    cJSON_Delete(response);
    
    log_info("Successfully handled GET /api/recordings request");
}

/**
 * @brief Direct handler for GET /api/recordings/:id
 */
void mg_handle_get_recording(struct mg_connection *c, struct mg_http_message *hm) {
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
    
    log_info("Handling GET /api/recordings/%llu request", (unsigned long long)id);
    
    // Get recording from database
    recording_metadata_t recording;
    if (get_recording_metadata_by_id(id, &recording) != 0) {
        log_error("Recording not found: %llu", (unsigned long long)id);
        mg_send_json_error(c, 404, "Recording not found");
        return;
    }
    
    // Create JSON object
    cJSON *recording_obj = cJSON_CreateObject();
    if (!recording_obj) {
        log_error("Failed to create recording JSON object");
        mg_send_json_error(c, 500, "Failed to create recording JSON");
        return;
    }
    
    // Format timestamps
    char start_time_str[32] = {0};
    char end_time_str[32] = {0};
    struct tm *tm_info;
    
    tm_info = localtime(&recording.start_time);
    if (tm_info) {
        strftime(start_time_str, sizeof(start_time_str), "%Y-%m-%d %H:%M:%S", tm_info);
    }
    
    tm_info = localtime(&recording.end_time);
    if (tm_info) {
        strftime(end_time_str, sizeof(end_time_str), "%Y-%m-%d %H:%M:%S", tm_info);
    }
    
    // Calculate duration in seconds
    int duration = (int)difftime(recording.end_time, recording.start_time);
    
    // Format file size for display (e.g., "1.8 MB")
    char size_str[32] = {0};
    if (recording.size_bytes < 1024) {
        snprintf(size_str, sizeof(size_str), "%ld B", recording.size_bytes);
    } else if (recording.size_bytes < 1024 * 1024) {
        snprintf(size_str, sizeof(size_str), "%.1f KB", recording.size_bytes / 1024.0);
    } else if (recording.size_bytes < 1024 * 1024 * 1024) {
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
    cJSON_AddNumberToObject(recording_obj, "duration", duration);
    cJSON_AddStringToObject(recording_obj, "size", size_str);
    cJSON_AddBoolToObject(recording_obj, "has_detection", false); // Default to false as it's not in metadata
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(recording_obj);
    if (!json_str) {
        log_error("Failed to convert recording JSON to string");
        cJSON_Delete(recording_obj);
        mg_send_json_error(c, 500, "Failed to convert recording JSON to string");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    cJSON_Delete(recording_obj);
    
    log_info("Successfully handled GET /api/recordings/%llu request", (unsigned long long)id);
}

/**
 * @brief Direct handler for batch delete recordings
 */
void mg_handle_batch_delete_recordings(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/recordings/batch-delete request");
    
    // Parse JSON body
    cJSON *json = mg_parse_json_body(hm);
    if (!json) {
        log_error("Failed to parse JSON body");
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
        char stream_name[MAX_STREAM_NAME] = {0};
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
                mg_send_json_error(c, 500, "Failed to create response");
                return;
            }
            
            mg_send_json_response(c, 200, json_str);
            free(json_str);
            cJSON_Delete(json);
            cJSON_Delete(response);
            return;
        }
        
        // Allocate memory for recordings
        recordings = (recording_metadata_t *)malloc(total_count * sizeof(recording_metadata_t));
        if (!recordings) {
            log_error("Failed to allocate memory for recordings");
            cJSON_Delete(json);
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
                mg_send_json_error(c, 500, "Failed to create response");
                return;
            }
            
            mg_send_json_response(c, 200, json_str);
            free(json_str);
            cJSON_Delete(json);
            cJSON_Delete(response);
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
        mg_send_json_error(c, 400, "Request must contain either 'ids' array or 'filter' object");
    }
}

/**
 * @brief Direct handler for DELETE /api/recordings/:id
 */
void mg_handle_delete_recording(struct mg_connection *c, struct mg_http_message *hm) {
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
    
    // Get recording from database
    recording_metadata_t recording;
    if (get_recording_metadata_by_id(id, &recording) != 0) {
        log_error("Recording not found: %llu", (unsigned long long)id);
        mg_send_json_error(c, 404, "Recording not found");
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
        return;
    }
    
    // Create success response using cJSON
    cJSON *success = cJSON_CreateObject();
    if (!success) {
        log_error("Failed to create success JSON object");
        mg_send_json_error(c, 500, "Failed to create success JSON");
        return;
    }
    
    cJSON_AddBoolToObject(success, "success", true);
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(success);
    if (!json_str) {
        log_error("Failed to convert success JSON to string");
        cJSON_Delete(success);
        mg_send_json_error(c, 500, "Failed to convert success JSON to string");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    cJSON_Delete(success);
    
    log_info("Successfully deleted recording: %llu", (unsigned long long)id);
}

// Structure to hold recording playback state
typedef struct {
    FILE *file;                  // File handle
    char file_path[MAX_PATH_LENGTH]; // File path
    size_t file_size;            // Total file size
    size_t bytes_sent;           // Bytes sent so far
    uint64_t recording_id;       // Recording ID
    time_t last_activity;        // Last activity timestamp
} recording_playback_state_t;

// Maximum number of concurrent playback sessions
#define MAX_CONCURRENT_PLAYBACKS 32

// Array of active playback sessions
static recording_playback_state_t playback_sessions[MAX_CONCURRENT_PLAYBACKS];

// Mutex to protect access to playback sessions
static pthread_mutex_t playback_mutex = PTHREAD_MUTEX_INITIALIZER;

// Initialize playback sessions
static void init_playback_sessions(void) {
    static bool initialized = false;
    
    if (!initialized) {
        pthread_mutex_lock(&playback_mutex);
        
        if (!initialized) {
            memset(playback_sessions, 0, sizeof(playback_sessions));
            initialized = true;
            log_info("Initialized recording playback session manager");
        }
        
        pthread_mutex_unlock(&playback_mutex);
    }
}

// Find a free playback session slot
static int find_free_playback_slot(void) {
    for (int i = 0; i < MAX_CONCURRENT_PLAYBACKS; i++) {
        if (playback_sessions[i].file == NULL) {
            return i;
        }
    }
    return -1;
}

// Clean up inactive playback sessions (called periodically)
static void cleanup_inactive_playback_sessions(void) {
    time_t now = time(NULL);
    
    pthread_mutex_lock(&playback_mutex);
    
    for (int i = 0; i < MAX_CONCURRENT_PLAYBACKS; i++) {
        if (playback_sessions[i].file != NULL) {
            // If no activity for 30 seconds, close the session
            if (difftime(now, playback_sessions[i].last_activity) > 30) {
                log_info("Closing inactive playback session for recording %llu", 
                        (unsigned long long)playback_sessions[i].recording_id);
                
                fclose(playback_sessions[i].file);
                playback_sessions[i].file = NULL;
                playback_sessions[i].recording_id = 0;
                playback_sessions[i].bytes_sent = 0;
            }
        }
    }
    
    pthread_mutex_unlock(&playback_mutex);
}

/**
 * @brief Direct handler for GET /api/recordings/play/:id
 */
void mg_handle_play_recording(struct mg_connection *c, struct mg_http_message *hm) {
    // Initialize playback sessions if not already done
    init_playback_sessions();
    
    // Clean up inactive sessions
    cleanup_inactive_playback_sessions();
    
    // Extract recording ID from URL
    char id_str[32];
    if (mg_extract_path_param(hm, "/api/recordings/play/", id_str, sizeof(id_str)) != 0) {
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
    
    log_info("Handling GET /api/recordings/play/%llu request", (unsigned long long)id);
    
    // Check for Range header (for seeking)
    struct mg_str *range_header = mg_http_get_header(hm, "Range");
    bool is_range_request = (range_header != NULL && range_header->len > 0);
    size_t range_start = 0;
    size_t range_end = 0;
    
    if (is_range_request) {
        // Parse Range header (e.g., "bytes=0-1023")
        char range_str[64] = {0};
        size_t range_len = range_header->len < sizeof(range_str) - 1 ? range_header->len : sizeof(range_str) - 1;
        memcpy(range_str, range_header->buf, range_len);
        range_str[range_len] = '\0';
        
        log_info("Range request: %s", range_str);
        
        // Parse range values
        if (sscanf(range_str, "bytes=%zu-%zu", &range_start, &range_end) < 1) {
            log_error("Invalid Range header format: %s", range_str);
            is_range_request = false;
        }
    }
    
    // Get recording from database
    recording_metadata_t recording;
    if (get_recording_metadata_by_id(id, &recording) != 0) {
        log_error("Recording not found: %llu", (unsigned long long)id);
        mg_send_json_error(c, 404, "Recording not found");
        return;
    }
    
    // Check if file exists
    struct stat st;
    if (stat(recording.file_path, &st) != 0) {
        log_error("Recording file not found: %s", recording.file_path);
        mg_send_json_error(c, 404, "Recording file not found");
        return;
    }
    
    // Allocate a playback session
    int slot = -1;
    FILE *file = NULL;
    
    pthread_mutex_lock(&playback_mutex);
    
    // Find a free slot
    slot = find_free_playback_slot();
    if (slot < 0) {
        // No free slots, try to clean up inactive sessions
        cleanup_inactive_playback_sessions();
        
        // Try again
        slot = find_free_playback_slot();
        if (slot < 0) {
            pthread_mutex_unlock(&playback_mutex);
            log_error("No free playback slots available");
            mg_send_json_error(c, 503, "Server busy, too many concurrent playback sessions");
            return;
        }
    }
    
    // Open file
    file = fopen(recording.file_path, "rb");
    if (!file) {
        pthread_mutex_unlock(&playback_mutex);
        log_error("Failed to open recording file: %s", recording.file_path);
        mg_send_json_error(c, 500, "Failed to open recording file");
        return;
    }
    
    // Initialize playback session
    playback_sessions[slot].file = file;
    strncpy(playback_sessions[slot].file_path, recording.file_path, sizeof(playback_sessions[slot].file_path) - 1);
    playback_sessions[slot].file_size = st.st_size;
    playback_sessions[slot].bytes_sent = 0;
    playback_sessions[slot].recording_id = id;
    playback_sessions[slot].last_activity = time(NULL);
    
    pthread_mutex_unlock(&playback_mutex);
    
    // Handle range request if present
    if (is_range_request) {
        // Adjust range_end if not specified or beyond file size
        if (range_end == 0 || range_end >= st.st_size) {
            range_end = st.st_size - 1;
        }
        
        // Validate range
        if (range_start > range_end || range_start >= st.st_size) {
            log_error("Invalid range: %zu-%zu for file size %zu", range_start, range_end, (size_t)st.st_size);
            mg_send_json_error(c, 416, "Range Not Satisfiable");
            
            // Clean up
            pthread_mutex_lock(&playback_mutex);
            fclose(playback_sessions[slot].file);
            playback_sessions[slot].file = NULL;
            pthread_mutex_unlock(&playback_mutex);
            
            return;
        }
        
        // Seek to range start
        if (fseek(file, range_start, SEEK_SET) != 0) {
            log_error("Failed to seek to position %zu in file", range_start);
            mg_send_json_error(c, 500, "Failed to seek in file");
            
            // Clean up
            pthread_mutex_lock(&playback_mutex);
            fclose(playback_sessions[slot].file);
            playback_sessions[slot].file = NULL;
            pthread_mutex_unlock(&playback_mutex);
            
            return;
        }
        
        // Calculate content length for range
        size_t content_length = range_end - range_start + 1;
        
        // Send 206 Partial Content response
        mg_printf(c, "HTTP/1.1 206 Partial Content\r\n");
        mg_printf(c, "Content-Type: video/mp4\r\n");
        mg_printf(c, "Content-Length: %zu\r\n", content_length);
        mg_printf(c, "Content-Range: bytes %zu-%zu/%zu\r\n", range_start, range_end, (size_t)st.st_size);
        mg_printf(c, "Accept-Ranges: bytes\r\n");
        mg_printf(c, "Cache-Control: max-age=3600\r\n");
        mg_printf(c, "\r\n");
        
        // Update bytes sent
        pthread_mutex_lock(&playback_mutex);
        playback_sessions[slot].bytes_sent = range_start;
        pthread_mutex_unlock(&playback_mutex);
        
        // Send range of file
        char buffer[8192];
        size_t bytes_to_send = content_length;
        size_t bytes_read;
        
        while (bytes_to_send > 0 && 
               (bytes_read = fread(buffer, 1, bytes_to_send > sizeof(buffer) ? sizeof(buffer) : bytes_to_send, file)) > 0) {
            mg_send(c, buffer, bytes_read);
            bytes_to_send -= bytes_read;
            
            // Update bytes sent and last activity
            pthread_mutex_lock(&playback_mutex);
            playback_sessions[slot].bytes_sent += bytes_read;
            playback_sessions[slot].last_activity = time(NULL);
            pthread_mutex_unlock(&playback_mutex);
        }
    } else {
        // Regular request (not range)
        // Extract filename from path
        const char *filename = strrchr(recording.file_path, '/');
        if (filename) {
            filename++; // Skip the slash
        } else {
            filename = recording.file_path;
        }
        
        // Set headers for streaming playback (not download)
        mg_printf(c, "HTTP/1.1 200 OK\r\n");
        mg_printf(c, "Content-Type: video/mp4\r\n");
        mg_printf(c, "Content-Length: %ld\r\n", st.st_size);
        mg_printf(c, "Accept-Ranges: bytes\r\n");
        mg_printf(c, "Cache-Control: max-age=3600\r\n");
        mg_printf(c, "\r\n");
        
        // Send file content in chunks
        char buffer[8192];
        size_t bytes_read;
        
        while ((bytes_read = fread(buffer, 1, sizeof(buffer), file)) > 0) {
            mg_send(c, buffer, bytes_read);
            
            // Update bytes sent and last activity
            pthread_mutex_lock(&playback_mutex);
            playback_sessions[slot].bytes_sent += bytes_read;
            playback_sessions[slot].last_activity = time(NULL);
            pthread_mutex_unlock(&playback_mutex);
        }
    }
    
    // Clean up - close file and free slot
    pthread_mutex_lock(&playback_mutex);
    fclose(playback_sessions[slot].file);
    playback_sessions[slot].file = NULL;
    pthread_mutex_unlock(&playback_mutex);
    
    log_info("Successfully handled GET /api/recordings/play/%llu request", (unsigned long long)id);
}

/**
 * @brief Direct handler for GET /api/recordings/download/:id
 */
void mg_handle_download_recording(struct mg_connection *c, struct mg_http_message *hm) {
    // Extract recording ID from URL
    char id_str[32];
    if (mg_extract_path_param(hm, "/api/recordings/download/", id_str, sizeof(id_str)) != 0) {
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
    
    log_info("Handling GET /api/recordings/download/%llu request", (unsigned long long)id);
    
    // Get recording from database
    recording_metadata_t recording;
    if (get_recording_metadata_by_id(id, &recording) != 0) {
        log_error("Recording not found: %llu", (unsigned long long)id);
        mg_send_json_error(c, 404, "Recording not found");
        return;
    }
    
    // Check if file exists
    struct stat st;
    if (stat(recording.file_path, &st) != 0) {
        log_error("Recording file not found: %s", recording.file_path);
        mg_send_json_error(c, 404, "Recording file not found");
        return;
    }
    
    // Open file
    FILE *file = fopen(recording.file_path, "rb");
    if (!file) {
        log_error("Failed to open recording file: %s", recording.file_path);
        mg_send_json_error(c, 500, "Failed to open recording file");
        return;
    }
    
    // Extract filename from path
    const char *filename = strrchr(recording.file_path, '/');
    if (filename) {
        filename++; // Skip the slash
    } else {
        filename = recording.file_path;
    }
    
    // Set headers
    char headers[512];
    snprintf(headers, sizeof(headers),
             "Content-Type: video/mp4\r\n"
             "Content-Disposition: attachment; filename=\"%s\"\r\n"
             "Content-Length: %ld\r\n",
             filename, st.st_size);
    
    // Send headers
    mg_printf(c, "HTTP/1.1 200 OK\r\n%s\r\n", headers);
    
    // Send file content
    char buffer[8192];
    size_t bytes_read;
    while ((bytes_read = fread(buffer, 1, sizeof(buffer), file)) > 0) {
        mg_send(c, buffer, bytes_read);
    }
    
    // Close file
    fclose(file);
    
    log_info("Successfully handled GET /api/recordings/download/%llu request", (unsigned long long)id);
}
