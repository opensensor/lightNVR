/**
 * @file api_handlers_recordings_list.c
 * @brief Backend-agnostic handler for GET /api/recordings (list all recordings)
 */

#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <time.h>
#include <cjson/cJSON.h>

#include "web/api_handlers.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#define LOG_COMPONENT "RecordingsAPI"
#include "core/logger.h"
#include "core/config.h"
#include "core/camera_collection_filter.h"
#include "core/shutdown_coordinator.h"
#include "utils/strings.h"
#include "database/db_recordings.h"
#include "database/db_detections.h"
#include "database/db_auth.h"
#include "database/db_streams.h"
#include "database/db_recording_tags.h"

#define MAX_SELECTED_STREAM_FILTERS 32
#define MAX_SELECTED_STREAM_NAME_LEN 64
#define MAX_BATCHED_DETECTION_RECORDINGS 1000
#define MAX_BATCHED_TAG_RECORDINGS 100

static int parse_selected_streams(const char *csv,
                                  char values[][MAX_SELECTED_STREAM_NAME_LEN],
                                  int max_values) {
    if (!csv || !*csv || max_values <= 0) {
        return 0;
    }

    char buffer[MAX_SELECTED_STREAM_FILTERS * MAX_SELECTED_STREAM_NAME_LEN];
    safe_strcpy(buffer, csv, sizeof(buffer), 0);

    int count = 0;
    char *saveptr = NULL;
    char *token = strtok_r(buffer, ",", &saveptr);
    while (token && count < max_values) {
        if (copy_trimmed_value(values[count], MAX_SELECTED_STREAM_NAME_LEN, token, 0)) {
            count++;
        }
        token = strtok_r(NULL, ",", &saveptr);
    }

    return count;
}

static bool valid_uuid(const char *value) {
    if (!value || strlen(value) != CAMERA_UUID_STRING_SIZE - 1) return false;
    for (int i = 0; i < CAMERA_UUID_STRING_SIZE - 1; i++) {
        char c = value[i];
        if (i == 8 || i == 13 || i == 18 || i == 23) {
            if (c != '-') return false;
        } else if (!isxdigit((unsigned char)c)) {
            return false;
        }
    }
    return true;
}

static void respond_empty_recordings(http_response_t *res, int page, int limit) {
    cJSON *root = cJSON_CreateObject();
    cJSON *recordings = cJSON_CreateArray();
    cJSON *pagination = cJSON_CreateObject();
    if (!root || !recordings || !pagination) {
        cJSON_Delete(root);
        cJSON_Delete(recordings);
        cJSON_Delete(pagination);
        http_response_set_json_error(res, 500, "Failed to create response JSON");
        return;
    }
    cJSON_AddItemToObject(root, "recordings", recordings);
    cJSON_AddNumberToObject(pagination, "page", page);
    cJSON_AddNumberToObject(pagination, "pages", 0);
    cJSON_AddNumberToObject(pagination, "total", 0);
    cJSON_AddNumberToObject(pagination, "limit", limit);
    cJSON_AddItemToObject(root, "pagination", pagination);
    char *json = cJSON_PrintUnformatted(root);
    http_response_set_json(res, 200, json ? json : "{}");
    free(json);
    cJSON_Delete(root);
}

/**
 * @brief Backend-agnostic handler for GET /api/recordings
 * 
 * Returns a paginated list of recordings with optional filtering by stream, time range, and detection status.
 * 
 * Query parameters:
 * - stream: Filter by stream name or comma-separated stream names
 * - collection_uuid: Filter by one authorized camera collection; cannot be
 *   combined with stream
 * - start: Start time (ISO 8601 format)
 * - end: End time (ISO 8601 format)
 * - page: Page number (default: 1)
 * - limit: Results per page (default: 20, max: 1000)
 * - sort: Sort field (default: "start_time")
 * - order: Sort order "asc" or "desc" (default: "desc")
 * - has_detection: Filter by detection status (0 or 1)
 * - detection_label: Filter by specific detection object label(s), comma-separated
 * - tag: Filter by recording tag(s), comma-separated
 * - capture_method: Filter by capture method(s), comma-separated
 *   (continuous, scheduled, detection, motion, manual)
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
    // In demo mode, allow unauthenticated viewer access to read recordings
    user_t auth_user;
    memset(&auth_user, 0, sizeof(auth_user));
    bool have_auth_user = false;
    if (g_config.web_auth_enabled) {
        if (g_config.demo_mode) {
            if (!httpd_check_viewer_access(req, &auth_user)) {
                log_error("Authentication failed for GET /api/recordings request");
                http_response_set_json_error(res, 401, "Unauthorized");
                return;
            }
            have_auth_user = true;
        } else {
            if (!httpd_get_authenticated_user(req, &auth_user)) {
                log_error("Authentication failed for GET /api/recordings request");
                http_response_set_json_error(res, 401, "Unauthorized");
                return;
            }
            have_auth_user = true;
        }
    }

    // Extract query parameters
    char stream_name[256] = {0};
    char start_time_str[64] = {0};
    char end_time_str[64] = {0};
    char page_str[16] = {0};
    char limit_str[16] = {0};
    char sort_field[32] = "start_time";
    char sort_order[8] = "desc";
    char has_detection_str[8] = {0};
    char detection_label[256] = {0};
    char protected_str[8] = {0};
    char tag_filter_str[512] = {0};
    char capture_method_str[128] = {0};
    char collection_uuid[CAMERA_UUID_STRING_SIZE] = {0};

    http_request_get_query_param(req, "stream", stream_name, sizeof(stream_name));
    http_request_get_query_param(req, "start", start_time_str, sizeof(start_time_str));
    http_request_get_query_param(req, "end", end_time_str, sizeof(end_time_str));
    http_request_get_query_param(req, "page", page_str, sizeof(page_str));
    http_request_get_query_param(req, "limit", limit_str, sizeof(limit_str));
    http_request_get_query_param(req, "sort", sort_field, sizeof(sort_field));
    http_request_get_query_param(req, "order", sort_order, sizeof(sort_order));
    http_request_get_query_param(req, "has_detection", has_detection_str, sizeof(has_detection_str));
    http_request_get_query_param(req, "detection_label", detection_label, sizeof(detection_label));
    http_request_get_query_param(req, "protected", protected_str, sizeof(protected_str));
    http_request_get_query_param(req, "tag", tag_filter_str, sizeof(tag_filter_str));
    http_request_get_query_param(req, "capture_method", capture_method_str, sizeof(capture_method_str));
    http_request_get_query_param(req, "collection_uuid", collection_uuid, sizeof(collection_uuid));

    if (collection_uuid[0] && !valid_uuid(collection_uuid)) {
        http_response_set_json_error(res, 400, "Invalid collection_uuid");
        return;
    }
    if (collection_uuid[0] && stream_name[0]) {
        http_response_set_json_error(
            res, 400, "collection_uuid cannot be combined with stream");
        return;
    }

    // Parse numeric parameters
    int page = page_str[0] ? (int)strtol(page_str, NULL, 10) : 1;
    int all_limit_requested = (limit_str[0] != '\0' && strcasecmp(limit_str, "all") == 0);
    int limit = all_limit_requested ? 20 : (limit_str[0] ? (int)strtol(limit_str, NULL, 10) : 20);
    recording_metadata_t *recordings = NULL;
    // has_detection: 0=all, 1=detection events only, -1=no detection events only
    int has_detection = has_detection_str[0] ? (int)strtol(has_detection_str, NULL, 10) : 0;
    if (has_detection < -1) has_detection = -1;
    else if (has_detection > 1) has_detection = 1;
    // protected_filter: -1=all (default), 0=not protected, 1=protected
    int protected_filter = -1;
    if (protected_str[0] == '0') protected_filter = 0;
    else if (protected_str[0] == '1') protected_filter = 1;

    // Validate parameters
    if (page <= 0) page = 1;
    if (limit <= 0) limit = 20;
    if (limit > 1000) limit = 1000;

    // Calculate offset from page and limit
    int offset = (page - 1) * limit;

    // Parse time strings to time_t
    time_t start_time = 0;
    time_t end_time = 0;

    if (start_time_str[0] != '\0') {
        // URL-decode the time string (replace %3A with :)
        log_debug("Parsing start time string (decoded): %s", start_time_str);

        struct tm tm = {0};
        // Try different time formats
        if (strptime(start_time_str, "%Y-%m-%dT%H:%M:%S", &tm) != NULL ||
            strptime(start_time_str, "%Y-%m-%dT%H:%M:%S.000Z", &tm) != NULL ||
            strptime(start_time_str, "%Y-%m-%dT%H:%M:%S.000", &tm) != NULL ||
            strptime(start_time_str, "%Y-%m-%dT%H:%M:%SZ", &tm) != NULL) {

            // Convert to UTC timestamp - assume input is already in UTC
            tm.tm_isdst = 0; // No DST for UTC
            start_time = timegm(&tm);
            log_debug("Parsed start time: %ld", (long)start_time);
        } else {
            log_error("Failed to parse start time string: %s", start_time_str);
        }
    }

    if (end_time_str[0] != '\0') {
        log_debug("Parsing end time string (decoded): %s", end_time_str);

        struct tm tm = {0};
        // Try different time formats
        if (strptime(end_time_str, "%Y-%m-%dT%H:%M:%S", &tm) != NULL ||
            strptime(end_time_str, "%Y-%m-%dT%H:%M:%S.000Z", &tm) != NULL ||
            strptime(end_time_str, "%Y-%m-%dT%H:%M:%S.000", &tm) != NULL ||
            strptime(end_time_str, "%Y-%m-%dT%H:%M:%SZ", &tm) != NULL) {

            // Convert to UTC timestamp - assume input is already in UTC
            tm.tm_isdst = 0; // No DST for UTC
            end_time = timegm(&tm);
            log_debug("Parsed end time: %ld", (long)end_time);
        } else {
            log_error("Failed to parse end time string: %s", end_time_str);
        }
    }

    // If detection_label is specified, also force has_detection filter
    const char *label_filter = detection_label[0] != '\0' ? detection_label : NULL;
    if (label_filter) {
        has_detection = 1;  // Searching by label implies detection filter
    }

    // --- Tag-based RBAC: build list of allowed streams for this user ---
    stream_config_t *all_stream_cfgs = NULL;
    const char *allowed_streams[MAX_STREAMS];
    int allowed_streams_count = 0;
    bool tag_restricted = have_auth_user && auth_user.has_tag_restriction;

    if (tag_restricted) {
        all_stream_cfgs = calloc(g_config.max_streams, sizeof(stream_config_t));
        if (all_stream_cfgs) {
            int sc = get_all_stream_configs(all_stream_cfgs, g_config.max_streams);
            for (int i = 0; i < sc; i++) {
                if (db_auth_stream_allowed_for_user(&auth_user, all_stream_cfgs[i].tags)) {
                    allowed_streams[allowed_streams_count++] = all_stream_cfgs[i].name;
                }
            }
        }

        // If the caller requested specific streams, validate each one is in the allowed list
        if (stream_name[0] != '\0') {
            char requested_streams[MAX_SELECTED_STREAM_FILTERS][MAX_SELECTED_STREAM_NAME_LEN] = {{0}};
            int requested_stream_count = parse_selected_streams(stream_name, requested_streams, MAX_SELECTED_STREAM_FILTERS);
            bool permitted = requested_stream_count > 0;

            for (int requested = 0; requested < requested_stream_count && permitted; requested++) {
                bool found = false;
                for (int i = 0; i < allowed_streams_count; i++) {
                    if (strcmp(requested_streams[requested], allowed_streams[i]) == 0) {
                        found = true;
                        break;
                    }
                }
                if (!found) {
                    permitted = false;
                }
            }

            if (!permitted) {
                log_warn("User '%s' attempted to access restricted stream '%s' via recordings API",
                         auth_user.username, stream_name);
                // Return an empty result set rather than an error to avoid leaking stream existence
                if (all_stream_cfgs) free(all_stream_cfgs);
                respond_empty_recordings(res, page, limit);
                return;
            }
            // The specific stream is permitted — no need for the IN clause; use stream_name filter
            allowed_streams_count = 0;
        } else if (allowed_streams_count == 0) {
            // User has tag restriction but no accessible streams at all
            if (all_stream_cfgs) free(all_stream_cfgs);
            respond_empty_recordings(res, page, limit);
            return;
        }
    }

    // Resolve a collection on the server so smart rules stay private and large
    // fleets do not expand into hundreds of stream names in a query string.
    char **collection_streams = NULL;
    int collection_stream_count = 0;
    if (collection_uuid[0]) {
        camera_collection_filter_result_t collection_result =
            camera_collection_filter_resolve_stream_names(
                collection_uuid, &auth_user, &collection_streams,
                &collection_stream_count);
        if (collection_result != CAMERA_COLLECTION_FILTER_OK) {
            if (all_stream_cfgs) free(all_stream_cfgs);
            if (collection_result == CAMERA_COLLECTION_FILTER_NOT_FOUND) {
                http_response_set_json_error(res, 404, "Collection not found");
            } else if (collection_result == CAMERA_COLLECTION_FILTER_OUT_OF_MEMORY) {
                http_response_set_json_error(res, 500, "Out of memory");
            } else if (collection_result == CAMERA_COLLECTION_FILTER_INVALID_SELECTOR) {
                http_response_set_json_error(res, 500, "Collection selector is invalid");
            } else {
                http_response_set_json_error(res, 500,
                                             "Failed to load collection members");
            }
            return;
        }

        if (collection_stream_count == 0) {
            camera_collection_filter_free_stream_names(
                collection_streams, collection_stream_count);
            if (all_stream_cfgs) free(all_stream_cfgs);
            respond_empty_recordings(res, page, limit);
            return;
        }
    }

    // Get total count first (for pagination)
    const char * const *streams_filter = collection_uuid[0]
        ? (const char * const *)collection_streams
        : ((tag_restricted && allowed_streams_count > 0) ? allowed_streams : NULL);
    int streams_filter_count = collection_uuid[0]
        ? collection_stream_count
        : ((tag_restricted && allowed_streams_count > 0) ? allowed_streams_count : 0);

    const char *tag_filt = tag_filter_str[0] != '\0' ? tag_filter_str : NULL;

    int total_count = get_recording_count(start_time, end_time,
                                          stream_name[0] != '\0' ? stream_name : NULL,
                                          has_detection, label_filter, protected_filter,
                                          streams_filter, streams_filter_count,
                                          tag_filt,
                                          capture_method_str[0] != '\0' ? capture_method_str : NULL);

    if (total_count < 0) {
        log_error("Failed to get total recording count from database");
        camera_collection_filter_free_stream_names(
            collection_streams, collection_stream_count);
        if (all_stream_cfgs) free(all_stream_cfgs);
        http_response_set_json_error(res, 500, "Failed to get recording count from database");
        return;
    }

    if (all_limit_requested) {
        page = 1;
        limit = total_count > 0 ? total_count : 1;
        offset = 0;
    }

    // Allocate memory for recordings
    recordings = (recording_metadata_t *)malloc(limit * sizeof(recording_metadata_t));
    if (!recordings) {
        log_error("Failed to allocate memory for recordings");
        camera_collection_filter_free_stream_names(
            collection_streams, collection_stream_count);
        if (all_stream_cfgs) free(all_stream_cfgs);
        http_response_set_json_error(res, 500, "Failed to allocate memory for recordings");
        return;
    }

    // Get recordings with pagination
    int count = get_recording_metadata_paginated(start_time, end_time,
                                                 stream_name[0] != '\0' ? stream_name : NULL,
                                                 has_detection, label_filter, protected_filter,
                                                 sort_field, sort_order,
                                                 recordings, limit, offset,
                                                 streams_filter, streams_filter_count,
                                                 tag_filt,
                                                 capture_method_str[0] != '\0' ? capture_method_str : NULL);

    if (count < 0) {
        log_error("Failed to get recordings from database");
        free(recordings);
        camera_collection_filter_free_stream_names(
            collection_streams, collection_stream_count);
        if (all_stream_cfgs) free(all_stream_cfgs);
        http_response_set_json_error(res, 500, "Failed to get recordings from database");
        return;
    }
    camera_collection_filter_free_stream_names(
        collection_streams, collection_stream_count);

    // Create response object with recordings array and pagination
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        log_error("Failed to create response JSON object");
        free(recordings);
        if (all_stream_cfgs) free(all_stream_cfgs);
        http_response_set_json_error(res, 500, "Failed to create response JSON");
        return;
    }

    // Create recordings array
    cJSON *recordings_array = cJSON_CreateArray();
    if (!recordings_array) {
        log_error("Failed to create recordings JSON array");
        free(recordings);
        if (all_stream_cfgs) free(all_stream_cfgs);
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
        if (all_stream_cfgs) free(all_stream_cfgs);
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

    // Enrich the whole page in bounded database batches. Historically this
    // loop issued one detection aggregation and one tag query per recording,
    // making the endpoint increasingly sensitive to SQLite mutex contention.
    recording_detection_summary_t *detection_summaries = NULL;
    recording_tag_list_t *recording_tag_lists = NULL;

    if (count > 0 && count <= MAX_BATCHED_DETECTION_RECORDINGS) {
        detection_summaries = calloc((size_t)count, sizeof(*detection_summaries));
        if (!detection_summaries ||
            get_recording_detection_summaries(recordings, count,
                                              detection_summaries) != 0) {
            log_warn("Falling back to per-recording detection summary queries");
            free(detection_summaries);
            detection_summaries = NULL;
        }
    }

    if (count > 0 && count <= MAX_BATCHED_TAG_RECORDINGS) {
        recording_tag_lists = calloc((size_t)count, sizeof(*recording_tag_lists));
        uint64_t *recording_ids = malloc((size_t)count * sizeof(*recording_ids));
        if (recording_tag_lists && recording_ids) {
            for (int i = 0; i < count; i++) recording_ids[i] = recordings[i].id;
            if (db_recording_tag_get_batch(recording_ids, count,
                                           recording_tag_lists) < 0) {
                log_warn("Falling back to per-recording tag queries");
                free(recording_tag_lists);
                recording_tag_lists = NULL;
            }
        } else {
            free(recording_tag_lists);
            recording_tag_lists = NULL;
        }
        free(recording_ids);
    }

    // Add each recording to the array
    for (int i = 0; i < count; i++) {
        cJSON *recording = cJSON_CreateObject();
        if (!recording) {
            log_error("Failed to create recording JSON object");
            continue;
        }

        // Format timestamps as ISO 8601 UTC (compatible with all browsers including Safari)
        char start_time_formatted[32] = {0};
        char end_time_formatted[32] = {0};
        struct tm tm_info_buf;
        const struct tm *tm_info;

        tm_info = gmtime_r(&recordings[i].start_time, &tm_info_buf);
        if (tm_info) {
            strftime(start_time_formatted, sizeof(start_time_formatted), "%Y-%m-%dT%H:%M:%SZ", tm_info);
        }

        tm_info = gmtime_r(&recordings[i].end_time, &tm_info_buf);
        if (tm_info) {
            strftime(end_time_formatted, sizeof(end_time_formatted), "%Y-%m-%dT%H:%M:%SZ", tm_info);
        }

        // Calculate duration in seconds
        int duration = (int)difftime(recordings[i].end_time, recordings[i].start_time);

        // Format file size for display (e.g., "1.8 MB")
        char size_str[32] = {0};
        if (recordings[i].size_bytes < 1024) {
            snprintf(size_str, sizeof(size_str), "%lu B", (unsigned long)recordings[i].size_bytes);
        } else if (recordings[i].size_bytes < (uint64_t)1024 * 1024) {
            snprintf(size_str, sizeof(size_str), "%.1f KB", (double)recordings[i].size_bytes / 1024.0);
        } else if (recordings[i].size_bytes < (uint64_t)1024 * 1024 * 1024) {
            snprintf(size_str, sizeof(size_str), "%.1f MB", (double)recordings[i].size_bytes / (1024.0 * 1024.0));
        } else {
            snprintf(size_str, sizeof(size_str), "%.1f GB", (double)recordings[i].size_bytes / (1024.0 * 1024.0 * 1024.0));
        }

        cJSON_AddNumberToObject(recording, "id", (double)recordings[i].id);
        cJSON_AddStringToObject(recording, "stream", recordings[i].stream_name);
        if (recordings[i].camera_uuid[0] != '\0') {
            cJSON_AddStringToObject(recording, "camera_uuid",
                                    recordings[i].camera_uuid);
        } else {
            cJSON_AddNullToObject(recording, "camera_uuid");
        }
        cJSON_AddStringToObject(recording, "file_path", recordings[i].file_path);
        cJSON_AddStringToObject(recording, "start_time", start_time_formatted);
        cJSON_AddStringToObject(recording, "end_time", end_time_formatted);
        cJSON_AddNumberToObject(recording, "start_time_unix", (double)recordings[i].start_time);
        cJSON_AddNumberToObject(recording, "end_time_unix", (double)recordings[i].end_time);
        cJSON_AddNumberToObject(recording, "duration", duration);
        cJSON_AddStringToObject(recording, "size", size_str);
        cJSON_AddStringToObject(recording, "capture_method",
                                recording_capture_method(&recordings[i]));
        if (recordings[i].schedule_restricted < 0) {
            cJSON_AddNullToObject(recording, "schedule_restricted");
        } else {
            cJSON_AddBoolToObject(recording, "schedule_restricted",
                                  recordings[i].schedule_restricted != 0);
        }

        // Check if recording has detections and get detection labels summary
        bool has_detection_flag = (strcmp(recordings[i].trigger_type, "detection") == 0);
        detection_label_summary_t fallback_labels[MAX_DETECTION_LABELS];
        detection_label_summary_t *labels = fallback_labels;
        int label_count = 0;

        if (detection_summaries) {
            labels = detection_summaries[i].labels;
            label_count = detection_summaries[i].label_count;
            if (detection_summaries[i].has_detection) {
                has_detection_flag = true;
            }
        } else if (recordings[i].start_time > 0 && recordings[i].end_time > 0) {
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
        cJSON_AddBoolToObject(recording, "protected", recordings[i].protected);

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

        // Add recording tags
        recording_tag_list_t fallback_tag_list = {0};
        recording_tag_list_t *tag_list = recording_tag_lists
            ? &recording_tag_lists[i] : &fallback_tag_list;
        if (!recording_tag_lists) {
            tag_list->count = db_recording_tag_get(
                recordings[i].id, tag_list->tags, MAX_RECORDING_TAGS);
        }
        int tag_count_val = tag_list->count;
        if (tag_count_val > 0) {
            cJSON *tags_array = cJSON_CreateArray();
            if (tags_array) {
                for (int j = 0; j < tag_count_val; j++) {
                    cJSON_AddItemToArray(tags_array,
                                         cJSON_CreateString(tag_list->tags[j]));
                }
                cJSON_AddItemToObject(recording, "tags", tags_array);
            }
        } else {
            cJSON_AddItemToObject(recording, "tags", cJSON_CreateArray());
        }

        cJSON_AddItemToArray(recordings_array, recording);
    }

    free(detection_summaries);
    free(recording_tag_lists);

    // Free recordings and stream config buffer (if allocated for tag-based RBAC)
    free(recordings);
    if (all_stream_cfgs) free(all_stream_cfgs);

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
