/**
 * @file api_handlers_recordings_list.c
 * @brief Backend-agnostic handler for GET /api/recordings (list all recordings)
 */

#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <cjson/cJSON.h>

#include "web/api_handlers.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "core/logger.h"
#include "core/config.h"
#include "core/shutdown_coordinator.h"
#include "database/db_recordings.h"
#include "database/db_detections.h"
#include "database/db_auth.h"

/**
 * @brief Backend-agnostic handler for GET /api/recordings
 * 
 * Returns a paginated list of recordings with optional filtering by stream, time range, and detection status.
 * 
 * Query parameters:
 * - stream: Filter by stream name
 * - start: Start time (ISO 8601 format)
 * - end: End time (ISO 8601 format)
 * - page: Page number (default: 1)
 * - limit: Results per page (default: 20, max: 1000)
 * - sort: Sort field (default: "start_time")
 * - order: Sort order "asc" or "desc" (default: "desc")
 * - has_detection: Filter by detection status (0 or 1)
 */
void handle_get_recordings(const http_request_t *req, http_response_t *res) {
    // Check if shutdown is in progress
    if (is_shutdown_initiated()) {
        log_debug("Shutdown in progress, rejecting recordings request");
        http_response_set_json_error(res, 503, "Service shutting down");
        return;
    }

    log_debug("Processing GET /api/recordings request");

    // Check authentication if enabled
    if (g_config.web_auth_enabled) {
        user_t user;
        if (!httpd_get_authenticated_user(req, &user)) {
            log_error("Authentication failed for recordings request");
            http_response_set_json_error(res, 401, "Unauthorized");
            return;
        }
    }

    // Extract query parameters
    char stream_name[64] = {0};
    char start_time_str[64] = {0};
    char end_time_str[64] = {0};
    char page_str[16] = {0};
    char limit_str[16] = {0};
    char sort_field[32] = "start_time";
    char sort_order[8] = "desc";
    char has_detection_str[8] = {0};

    http_request_get_query_param(req, "stream", stream_name, sizeof(stream_name));
    http_request_get_query_param(req, "start", start_time_str, sizeof(start_time_str));
    http_request_get_query_param(req, "end", end_time_str, sizeof(end_time_str));
    http_request_get_query_param(req, "page", page_str, sizeof(page_str));
    http_request_get_query_param(req, "limit", limit_str, sizeof(limit_str));
    http_request_get_query_param(req, "sort", sort_field, sizeof(sort_field));
    http_request_get_query_param(req, "order", sort_order, sizeof(sort_order));
    http_request_get_query_param(req, "has_detection", has_detection_str, sizeof(has_detection_str));

    // Parse numeric parameters
    int page = page_str[0] ? atoi(page_str) : 1;
    int limit = limit_str[0] ? atoi(limit_str) : 20;
    int has_detection = has_detection_str[0] ? atoi(has_detection_str) : 0;

    // Validate parameters
    if (page <= 0) page = 1;
    if (limit <= 0) limit = 20;
    if (limit > 1000) limit = 1000;

    // Calculate offset from page and limit
    int offset = (page - 1) * limit;

    // Allocate memory for recordings
    recording_metadata_t *recordings = (recording_metadata_t *)malloc(limit * sizeof(recording_metadata_t));
    if (!recordings) {
        log_error("Failed to allocate memory for recordings");
        http_response_set_json_error(res, 500, "Failed to allocate memory for recordings");
        return;
    }

    // Parse time strings to time_t
    time_t start_time = 0;
    time_t end_time = 0;

    if (start_time_str[0] != '\0') {
        // URL-decode the time string (replace %3A with :)
        char decoded_start_time[64] = {0};
        url_decode(start_time_str, decoded_start_time, sizeof(decoded_start_time));

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
        url_decode(end_time_str, decoded_end_time, sizeof(decoded_end_time));

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
    int total_count = get_recording_count(start_time, end_time,
                                          stream_name[0] != '\0' ? stream_name : NULL,
                                          has_detection);

    if (total_count < 0) {
        log_error("Failed to get total recording count from database");
        free(recordings);
        http_response_set_json_error(res, 500, "Failed to get recording count from database");
        return;
    }

    // Get recordings with pagination
    int count = get_recording_metadata_paginated(start_time, end_time,
                                                 stream_name[0] != '\0' ? stream_name : NULL,
                                                 has_detection, sort_field, sort_order,
                                                 recordings, limit, offset);

    if (count < 0) {
        log_error("Failed to get recordings from database");
        free(recordings);
        http_response_set_json_error(res, 500, "Failed to get recordings from database");
        return;
    }

    // Create response object with recordings array and pagination
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        log_error("Failed to create response JSON object");
        free(recordings);
        http_response_set_json_error(res, 500, "Failed to create response JSON");
        return;
    }

    // Create recordings array
    cJSON *recordings_array = cJSON_CreateArray();
    if (!recordings_array) {
        log_error("Failed to create recordings JSON array");
        free(recordings);
        cJSON_Delete(response);
        http_response_set_json_error(res, 500, "Failed to create recordings JSON");
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
        http_response_set_json_error(res, 500, "Failed to create pagination JSON");
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
        char start_time_formatted[32] = {0};
        char end_time_formatted[32] = {0};
        struct tm *tm_info;

        tm_info = gmtime(&recordings[i].start_time);
        if (tm_info) {
            strftime(start_time_formatted, sizeof(start_time_formatted), "%Y-%m-%d %H:%M:%S UTC", tm_info);
        }

        tm_info = gmtime(&recordings[i].end_time);
        if (tm_info) {
            strftime(end_time_formatted, sizeof(end_time_formatted), "%Y-%m-%d %H:%M:%S UTC", tm_info);
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
        cJSON_AddStringToObject(recording, "start_time", start_time_formatted);
        cJSON_AddStringToObject(recording, "end_time", end_time_formatted);
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

    // Convert to JSON string
    char *json_str = cJSON_PrintUnformatted(response);
    if (!json_str) {
        log_error("Failed to convert response JSON to string");
        cJSON_Delete(response);
        http_response_set_json_error(res, 500, "Failed to convert response JSON to string");
        return;
    }

    // Send JSON response
    http_response_set_json(res, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(response);

    log_debug("Successfully handled GET /api/recordings request");
}

