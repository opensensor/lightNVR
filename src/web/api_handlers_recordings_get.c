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
#include "web/mongoose_server_auth.h"
#include "web/http_server.h"
#include "core/logger.h"
#include "core/config.h"
#include "core/shutdown_coordinator.h"
#include "mongoose.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"
#include "database/db_detections.h"
#include "web/mongoose_server_multithreading.h"

/**
 * @brief Worker function for GET /api/recordings
 *
 * This function is called by the multithreading system to handle recordings requests.
 */
void mg_handle_get_recordings_worker(struct mg_connection *c, struct mg_http_message *hm) {
    // Check if shutdown is in progress - skip expensive database queries
    if (is_shutdown_initiated()) {
        log_debug("Shutdown in progress, rejecting recordings request");
        mg_send_json_error(c, 503, "Service shutting down");
        return;
    }

    log_debug("Processing GET /api/recordings request in worker thread");

    // Extract URI for logging
    char uri_buf[MAX_PATH_LENGTH] = {0};
    size_t uri_len = hm->uri.len < sizeof(uri_buf) - 1 ? hm->uri.len : sizeof(uri_buf) - 1;
    memcpy(uri_buf, hm->uri.buf, uri_len);
    uri_buf[uri_len] = '\0';

    // Log all headers for debugging
    log_debug("Request headers for %s:", uri_buf);
    for (int i = 0; i < MG_MAX_HTTP_HEADERS; i++) {
        if (hm->headers[i].name.len == 0) break;
        log_debug("  %.*s: %.*s",
                (int)hm->headers[i].name.len, hm->headers[i].name.buf,
                (int)hm->headers[i].value.len, hm->headers[i].value.buf);
    }

    // Check authentication
    http_server_t *server = (http_server_t *)c->fn_data;
    if (server && server->config.auth_enabled) {
        // Check if the user is authenticated
        if (mongoose_server_basic_auth_check(hm, server) != 0) {
            log_error("Authentication failed for recordings request");
            mg_send_json_error(c, 401, "Unauthorized");
            return;
        }
    }
    
    // Parse query parameters
    char query_string[512] = {0};
    if (hm->query.len > 0 && hm->query.len < sizeof(query_string)) {
        memcpy(query_string, mg_str_get_ptr(&hm->query), hm->query.len);
        query_string[hm->query.len] = '\0';
        log_debug("Query string: %s", query_string);
    }
    
    // Extract parameters
    char stream_name[64] = {0};
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
        } else if (strncmp(param, "has_detection=", 14) == 0) {
            has_detection = atoi(param + 14);
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
        
        log_debug("Parsing start time string (decoded): %s", decoded_start_time);

        struct tm tm = {0};
        // Try different time formats
        if (strptime(decoded_start_time, "%Y-%m-%dT%H:%M:%S", &tm) != NULL ||
            strptime(decoded_start_time, "%Y-%m-%dT%H:%M:%S.000Z", &tm) != NULL ||
            strptime(decoded_start_time, "%Y-%m-%dT%H:%M:%S.000", &tm) != NULL ||
            strptime(decoded_start_time, "%Y-%m-%dT%H:%M:%SZ", &tm) != NULL) {

            // Convert to UTC timestamp - assume input is already in UTC
            tm.tm_isdst = 0; // No DST for UTC
            start_time = timegm(&tm);
            log_debug("Parsed start time: %ld", (long)start_time);
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
        
        log_debug("Parsing end time string (decoded): %s", decoded_end_time);

        struct tm tm = {0};
        // Try different time formats
        if (strptime(decoded_end_time, "%Y-%m-%dT%H:%M:%S", &tm) != NULL ||
            strptime(decoded_end_time, "%Y-%m-%dT%H:%M:%S.000Z", &tm) != NULL ||
            strptime(decoded_end_time, "%Y-%m-%dT%H:%M:%S.000", &tm) != NULL ||
            strptime(decoded_end_time, "%Y-%m-%dT%H:%M:%SZ", &tm) != NULL) {

            // Convert to UTC timestamp - assume input is already in UTC
            tm.tm_isdst = 0; // No DST for UTC
            end_time = timegm(&tm);
            log_debug("Parsed end time: %ld", (long)end_time);
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
        free(recordings);
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
        
        // Format timestamps in UTC
        char start_time_str[32] = {0};
        char end_time_str[32] = {0};
        struct tm *tm_info;
        
        tm_info = gmtime(&recordings[i].start_time);
        if (tm_info) {
            strftime(start_time_str, sizeof(start_time_str), "%Y-%m-%d %H:%M:%S UTC", tm_info);
        }
        
        tm_info = gmtime(&recordings[i].end_time);
        if (tm_info) {
            strftime(end_time_str, sizeof(end_time_str), "%Y-%m-%d %H:%M:%S UTC", tm_info);
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

        // Check if recording has detections and get detection labels summary
        bool has_detection_flag = (strcmp(recordings[i].trigger_type, "detection") == 0);
        detection_label_summary_t labels[MAX_DETECTION_LABELS];
        int label_count = 0;

        if (recordings[i].start_time > 0 && recordings[i].end_time > 0) {
            // Get detection labels summary for this recording's time range
            label_count = get_detection_labels_summary(recordings[i].stream_name,
                                                       recordings[i].start_time,
                                                       recordings[i].end_time,
                                                       labels, MAX_DETECTION_LABELS);
            if (label_count > 0) {
                has_detection_flag = true;
            } else if (!has_detection_flag) {
                // Fall back to simple check if get_detection_labels_summary returned 0
                int det_result = has_detections_in_time_range(recordings[i].stream_name,
                                                              recordings[i].start_time,
                                                              recordings[i].end_time);
                if (det_result > 0) {
                    has_detection_flag = true;
                }
            }
        }
        cJSON_AddBoolToObject(recording, "has_detection", has_detection_flag);

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
                cJSON_AddItemToObject(recording, "detection_labels", labels_array);
            }
        }

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
    
    // Send response directly
    log_debug("Sending JSON response for GET /api/recordings request");
    mg_send_json_response(c, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(response);

    log_debug("Successfully handled GET /api/recordings request");
}

/**
 * @brief Handler for GET /api/recordings
 * 
 * This handler processes the request directly in the current thread.
 * For large datasets, this approach ensures the client receives the complete response.
 */
void mg_handle_get_recordings(struct mg_connection *c, struct mg_http_message *hm) {
    log_debug("Processing GET /api/recordings request");

    // Process the request directly
    mg_handle_get_recordings_worker(c, hm);

    log_debug("Completed GET /api/recordings request");
}

/**
 * @brief Worker function for GET /api/recordings/:id
 *
 * This function is called by the multithreading system to handle recording detail requests.
 */
void mg_handle_get_recording_worker(struct mg_connection *c, struct mg_http_message *hm) {
    // Check if shutdown is in progress
    if (is_shutdown_initiated()) {
        log_debug("Shutdown in progress, rejecting recording detail request");
        mg_send_json_error(c, 503, "Service shutting down");
        return;
    }

    log_debug("Processing GET /api/recordings/:id request in worker thread");

    // Check authentication
    http_server_t *server = (http_server_t *)c->fn_data;
    if (server && server->config.auth_enabled) {
        // Check if the user is authenticated
        if (mongoose_server_basic_auth_check(hm, server) != 0) {
            log_error("Authentication failed for recording detail request");
            mg_send_json_error(c, 401, "Unauthorized");
            return;
        }
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
    
    log_debug("Handling GET /api/recordings/%llu request", (unsigned long long)id);
    
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
    
    // Format timestamps in UTC
    char start_time_str[32] = {0};
    char end_time_str[32] = {0};
    struct tm *tm_info;
    
    tm_info = gmtime(&recording.start_time);
    if (tm_info) {
        strftime(start_time_str, sizeof(start_time_str), "%Y-%m-%d %H:%M:%S UTC", tm_info);
    }
    
    tm_info = gmtime(&recording.end_time);
    if (tm_info) {
        strftime(end_time_str, sizeof(end_time_str), "%Y-%m-%d %H:%M:%S UTC", tm_info);
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

    // Convert to string
    char *json_str = cJSON_PrintUnformatted(recording_obj);
    if (!json_str) {
        log_error("Failed to convert recording JSON to string");
        cJSON_Delete(recording_obj);
        mg_send_json_error(c, 500, "Failed to convert recording JSON to string");
        return;
    }
    
    // Send response directly
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    cJSON_Delete(recording_obj);
    
    log_debug("Successfully handled GET /api/recordings/%llu request", (unsigned long long)id);
}

/**
 * @brief Handler for GET /api/recordings/:id
 * 
 * This handler processes the request directly in the current thread.
 * This approach ensures the client receives the complete response.
 */
void mg_handle_get_recording(struct mg_connection *c, struct mg_http_message *hm) {
    log_debug("Processing GET /api/recordings/:id request");

    // Process the request directly
    mg_handle_get_recording_worker(c, hm);

    log_debug("Completed GET /api/recordings/:id request");
}
