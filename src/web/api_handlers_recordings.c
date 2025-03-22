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
        struct tm tm = {0};
        if (strptime(start_time_str, "%Y-%m-%dT%H:%M:%S", &tm) != NULL) {
            start_time = mktime(&tm);
        }
    }
    
    if (end_time_str[0] != '\0') {
        struct tm tm = {0};
        if (strptime(end_time_str, "%Y-%m-%dT%H:%M:%S", &tm) != NULL) {
            end_time = mktime(&tm);
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

/**
 * @brief Direct handler for GET /api/recordings/play/:id
 */
void mg_handle_play_recording(struct mg_connection *c, struct mg_http_message *hm) {
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
    
    // Set headers for streaming playback (not download)
    char headers[512];
    snprintf(headers, sizeof(headers),
             "Content-Type: video/mp4\r\n"
             "Content-Length: %ld\r\n"
             "Accept-Ranges: bytes\r\n"
             "Cache-Control: max-age=3600\r\n",
             st.st_size);
    
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
