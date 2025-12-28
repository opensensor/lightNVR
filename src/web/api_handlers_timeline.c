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
#include <pthread.h>

#include "web/api_handlers_timeline.h"
#include "web/api_handlers.h"
#include "web/mongoose_adapter.h"
#include "core/logger.h"
#include "core/config.h"
#include "mongoose.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"

// Forward declarations for Mongoose API handlers
void mg_handle_get_timeline_segments(struct mg_connection *c, struct mg_http_message *hm);
void mg_handle_timeline_manifest(struct mg_connection *c, struct mg_http_message *hm);
void mg_handle_timeline_playback(struct mg_connection *c, struct mg_http_message *hm);

// Maximum number of segments to return in a single request
#define MAX_TIMELINE_SEGMENTS 1000

// Maximum number of segments in a manifest
#define MAX_MANIFEST_SEGMENTS 100

// Mutex for manifest creation
static pthread_mutex_t manifest_mutex = PTHREAD_MUTEX_INITIALIZER;

/**
 * Get timeline segments for a specific stream and time range
 */
int get_timeline_segments(const char *stream_name, time_t start_time, time_t end_time,
                         timeline_segment_t *segments, int max_segments) {
    if (!stream_name || !segments || max_segments <= 0) {
        log_error("Invalid parameters for get_timeline_segments");
        return -1;
    }
    
    // Allocate memory for recording metadata
    recording_metadata_t *recordings = (recording_metadata_t *)malloc(max_segments * sizeof(recording_metadata_t));
    if (!recordings) {
        log_error("Failed to allocate memory for recordings");
        return -1;
    }
    
    // Get recordings from database
    int count = get_recording_metadata_paginated(start_time, end_time, stream_name, 0,
                                              "start_time", "asc", recordings, max_segments, 0);
    
    if (count < 0) {
        log_error("Failed to get recordings from database");
        free(recordings);
        return -1;
    }
    
    // Convert recording metadata to timeline segments
    for (int i = 0; i < count; i++) {
        segments[i].id = recordings[i].id;
        strncpy(segments[i].stream_name, recordings[i].stream_name, sizeof(segments[i].stream_name) - 1);
        strncpy(segments[i].file_path, recordings[i].file_path, sizeof(segments[i].file_path) - 1);
        segments[i].start_time = recordings[i].start_time;
        segments[i].end_time = recordings[i].end_time;
        segments[i].size_bytes = recordings[i].size_bytes;
        segments[i].has_detection = false; // Default to false, could be updated with detection info
    }
    
    // Free recordings
    free(recordings);
    
    return count;
}

/**
 * @brief Handler for GET /api/timeline/segments
 */
void mg_handle_get_timeline_segments(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/timeline/segments request");
    
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
    
    // Parse query string without modifying it
    // Extract stream parameter
    char stream_param[MAX_STREAM_NAME] = {0};
    mg_http_get_var(&hm->query, "stream", stream_param, sizeof(stream_param));
    if (stream_param[0] != '\0') {
        strncpy(stream_name, stream_param, sizeof(stream_name) - 1);
    }
    
    // Extract start parameter
    mg_http_get_var(&hm->query, "start", start_time_str, sizeof(start_time_str));
    
    // Extract end parameter
    mg_http_get_var(&hm->query, "end", end_time_str, sizeof(end_time_str));
    
    // Check required parameters
    if (stream_name[0] == '\0') {
        log_error("Missing required parameter: stream");
        mg_send_json_error(c, 400, "Missing required parameter: stream");
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

            // Convert to UTC timestamp - assume input is already in UTC
            tm.tm_isdst = 0; // No DST for UTC
            start_time = timegm(&tm);
            log_info("Parsed start time: %ld", (long)start_time);
        } else if (strptime(decoded_start_time, "%Y-%m-%d", &tm) != NULL) {
            // Handle date-only format (YYYY-MM-DD)
            // Set time to 00:00:00
            tm.tm_hour = 0;
            tm.tm_min = 0;
            tm.tm_sec = 0;
            tm.tm_isdst = 0; // No DST for UTC
            start_time = timegm(&tm);
            log_info("Parsed date-only start time: %ld", (long)start_time);
        } else {
            log_error("Failed to parse start time string: %s", decoded_start_time);
        }
    } else {
        // Default to 24 hours ago
        start_time = time(NULL) - (24 * 60 * 60);
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

            // Convert to UTC timestamp - assume input is already in UTC
            tm.tm_isdst = 0; // No DST for UTC
            end_time = timegm(&tm);
            log_info("Parsed end time: %ld", (long)end_time);
        } else if (strptime(decoded_end_time, "%Y-%m-%d", &tm) != NULL) {
            // Handle date-only format (YYYY-MM-DD)
            // Set time to 23:59:59 for the end of the day
            tm.tm_hour = 23;
            tm.tm_min = 59;
            tm.tm_sec = 59;
            tm.tm_isdst = 0; // No DST for UTC
            end_time = timegm(&tm);
            log_info("Parsed date-only end time: %ld", (long)end_time);
        } else {
            log_error("Failed to parse end time string: %s", decoded_end_time);
        }
    } else {
        // Default to now
        end_time = time(NULL);
    }
    
    // Get timeline segments
    timeline_segment_t *segments = (timeline_segment_t *)malloc(MAX_TIMELINE_SEGMENTS * sizeof(timeline_segment_t));
    if (!segments) {
        log_error("Failed to allocate memory for timeline segments");
        mg_send_json_error(c, 500, "Failed to allocate memory for timeline segments");
        return;
    }
    
    int count = get_timeline_segments(stream_name, start_time, end_time, segments, MAX_TIMELINE_SEGMENTS);
    
    if (count < 0) {
        log_error("Failed to get timeline segments");
        free(segments);
        mg_send_json_error(c, 500, "Failed to get timeline segments");
        return;
    }
    
    // Create response object
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        log_error("Failed to create response JSON object");
        free(segments);
        mg_send_json_error(c, 500, "Failed to create response JSON");
        return;
    }
    
    // Create segments array
    cJSON *segments_array = cJSON_CreateArray();
    if (!segments_array) {
        log_error("Failed to create segments JSON array");
        free(segments);
        cJSON_Delete(response);
        mg_send_json_error(c, 500, "Failed to create segments JSON");
        return;
    }
    
    // Add segments array to response
    cJSON_AddItemToObject(response, "segments", segments_array);
    
    // Add metadata
    cJSON_AddStringToObject(response, "stream", stream_name);
    
    // Format timestamps for display in local time
    char start_time_display[32] = {0};
    char end_time_display[32] = {0};
    struct tm *tm_info;
    
    tm_info = localtime(&start_time);
    if (tm_info) {
        strftime(start_time_display, sizeof(start_time_display), "%Y-%m-%d %H:%M:%S", tm_info);
    }
    
    tm_info = localtime(&end_time);
    if (tm_info) {
        strftime(end_time_display, sizeof(end_time_display), "%Y-%m-%d %H:%M:%S", tm_info);
    }
    
    cJSON_AddStringToObject(response, "start_time", start_time_display);
    cJSON_AddStringToObject(response, "end_time", end_time_display);
    cJSON_AddNumberToObject(response, "segment_count", count);
    
    // Add each segment to the array
    for (int i = 0; i < count; i++) {
        cJSON *segment = cJSON_CreateObject();
        if (!segment) {
            log_error("Failed to create segment JSON object");
            continue;
        }
        
        // Format timestamps in local time
        char segment_start_time[32] = {0};
        char segment_end_time[32] = {0};
        
        tm_info = localtime(&segments[i].start_time);
        if (tm_info) {
            strftime(segment_start_time, sizeof(segment_start_time), "%Y-%m-%d %H:%M:%S", tm_info);
        }
        
        tm_info = localtime(&segments[i].end_time);
        if (tm_info) {
            strftime(segment_end_time, sizeof(segment_end_time), "%Y-%m-%d %H:%M:%S", tm_info);
        }
        
        // Calculate duration in seconds
        int duration = (int)difftime(segments[i].end_time, segments[i].start_time);
        
        // Format file size for display (e.g., "1.8 MB")
        char size_str[32] = {0};
        if (segments[i].size_bytes < 1024) {
            snprintf(size_str, sizeof(size_str), "%ld B", segments[i].size_bytes);
        } else if (segments[i].size_bytes < 1024 * 1024) {
            snprintf(size_str, sizeof(size_str), "%.1f KB", segments[i].size_bytes / 1024.0);
        } else if (segments[i].size_bytes < 1024 * 1024 * 1024) {
            snprintf(size_str, sizeof(size_str), "%.1f MB", segments[i].size_bytes / (1024.0 * 1024.0));
        } else {
            snprintf(size_str, sizeof(size_str), "%.1f GB", segments[i].size_bytes / (1024.0 * 1024.0 * 1024.0));
        }
        
        cJSON_AddNumberToObject(segment, "id", segments[i].id);
        cJSON_AddStringToObject(segment, "stream", segments[i].stream_name);
        cJSON_AddStringToObject(segment, "start_time", segment_start_time);
        cJSON_AddStringToObject(segment, "end_time", segment_end_time);
        cJSON_AddNumberToObject(segment, "duration", duration);
        cJSON_AddStringToObject(segment, "size", size_str);
        cJSON_AddBoolToObject(segment, "has_detection", segments[i].has_detection);
        
        // Add Unix timestamps for easier frontend processing
        // Convert to local timezone by adding the timezone offset
        struct tm *tm_start = localtime(&segments[i].start_time);
        struct tm *tm_end = localtime(&segments[i].end_time);
        
        // Calculate timezone offset in seconds
        time_t timezone_offset = tm_start->tm_gmtoff;
        
        // Add timestamps adjusted for local timezone
        cJSON_AddNumberToObject(segment, "start_timestamp", (double)segments[i].start_time);
        cJSON_AddNumberToObject(segment, "end_timestamp", (double)segments[i].end_time);
        
        // Add local timestamps (without timezone adjustment - the browser will handle timezone display)
        cJSON_AddNumberToObject(segment, "local_start_timestamp", (double)segments[i].start_time);
        cJSON_AddNumberToObject(segment, "local_end_timestamp", (double)segments[i].end_time);
        
        cJSON_AddItemToArray(segments_array, segment);
    }
    
    // Free segments
    free(segments);
    
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
    
    free(json_str);
    cJSON_Delete(response);
    
    log_info("Successfully handled GET /api/timeline/segments request");
}


/**
 * Create a playback manifest for a sequence of recordings
 */
int create_timeline_manifest(const timeline_segment_t *segments, int segment_count,
                            time_t start_time, char *manifest_path) {
    if (!segments || segment_count <= 0 || !manifest_path) {
        log_error("Invalid parameters for create_timeline_manifest");
        return -1;
    }
    
    // Limit the number of segments
    if (segment_count > MAX_MANIFEST_SEGMENTS) {
        log_warn("Limiting manifest to %d segments (requested %d)", MAX_MANIFEST_SEGMENTS, segment_count);
        segment_count = MAX_MANIFEST_SEGMENTS;
    }
    
    // Create a temporary directory for the manifest
    char temp_dir[MAX_PATH_LENGTH];
    snprintf(temp_dir, sizeof(temp_dir), "%s/timeline_manifests", g_config.storage_path);
    
    // Create directory if it doesn't exist
    struct stat st = {0};
    if (stat(temp_dir, &st) == -1) {
        mkdir(temp_dir, 0755);
    }
    
    // Generate a unique manifest filename
    char manifest_filename[MAX_PATH_LENGTH];
    snprintf(manifest_filename, sizeof(manifest_filename), "%s/manifest_%ld_%s_%ld.m3u8",
            temp_dir, (long)time(NULL), segments[0].stream_name, (long)start_time);
    
    // Lock mutex for manifest creation
    pthread_mutex_lock(&manifest_mutex);
    
    // Create manifest file
    FILE *manifest = fopen(manifest_filename, "w");
    if (!manifest) {
        log_error("Failed to create manifest file: %s", manifest_filename);
        pthread_mutex_unlock(&manifest_mutex);
        return -1;
    }
    
    // Write manifest header
    fprintf(manifest, "#EXTM3U\n");
    fprintf(manifest, "#EXT-X-VERSION:3\n");
    fprintf(manifest, "#EXT-X-MEDIA-SEQUENCE:0\n");
    fprintf(manifest, "#EXT-X-ALLOW-CACHE:YES\n");
    
    // Find the maximum segment duration for EXT-X-TARGETDURATION
    double max_duration = 0;
    for (int i = 0; i < segment_count; i++) {
        double duration = difftime(segments[i].end_time, segments[i].start_time);
        if (duration > max_duration) {
            max_duration = duration;
        }
    }
    // Round up to the nearest integer and add a small buffer
    int target_duration = (int)max_duration + 1;
    fprintf(manifest, "#EXT-X-TARGETDURATION:%d\n", target_duration);
    
    // Find the segment that contains the start time
    int start_segment_index = -1;
    for (int i = 0; i < segment_count; i++) {
        if (start_time >= segments[i].start_time && start_time <= segments[i].end_time) {
            start_segment_index = i;
            break;
        }
    }
    
    // If no segment contains the start time, use the first segment after the start time
    if (start_segment_index == -1) {
        for (int i = 0; i < segment_count; i++) {
            if (start_time < segments[i].start_time) {
                start_segment_index = i;
                break;
            }
        }
    }
    
    // If still no segment found, use the first segment
    if (start_segment_index == -1 && segment_count > 0) {
        start_segment_index = 0;
    }
    
    // Create a single segment for the entire timeline
    // This simplifies playback and avoids issues with segment transitions
    fprintf(manifest, "#EXTINF:%.6f,\n", max_duration);
    fprintf(manifest, "/api/timeline/play?stream=%s&start=%ld\n", 
            segments[0].stream_name, (long)start_time);
    
    // Write manifest end
    fprintf(manifest, "#EXT-X-ENDLIST\n");
    
    // Close manifest file
    fclose(manifest);
    
    // Copy manifest path to output
    strncpy(manifest_path, manifest_filename, MAX_PATH_LENGTH - 1);
    manifest_path[MAX_PATH_LENGTH - 1] = '\0';
    
    pthread_mutex_unlock(&manifest_mutex);
    
    log_info("Created timeline manifest: %s", manifest_filename);
    
    return 0;
}

/**
 * @brief Handler for GET /api/timeline/manifest
 */
void mg_handle_timeline_manifest(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/timeline/manifest request");
    
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
    
    // Parse query string without modifying it
    // Extract stream parameter
    char stream_param[MAX_STREAM_NAME] = {0};
    mg_http_get_var(&hm->query, "stream", stream_param, sizeof(stream_param));
    if (stream_param[0] != '\0') {
        strncpy(stream_name, stream_param, sizeof(stream_name) - 1);
    }
    
    // Extract start parameter
    mg_http_get_var(&hm->query, "start", start_time_str, sizeof(start_time_str));
    
    // Extract end parameter
    mg_http_get_var(&hm->query, "end", end_time_str, sizeof(end_time_str));
    
    // Check required parameters
    if (stream_name[0] == '\0') {
        log_error("Missing required parameter: stream");
        mg_send_json_error(c, 400, "Missing required parameter: stream");
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

            // Convert to UTC timestamp - assume input is already in UTC
            tm.tm_isdst = 0; // No DST for UTC
            start_time = timegm(&tm);
            log_info("Parsed start time: %ld", (long)start_time);
        } else if (strptime(decoded_start_time, "%Y-%m-%d", &tm) != NULL) {
            // Handle date-only format (YYYY-MM-DD)
            // Set time to 00:00:00
            tm.tm_hour = 0;
            tm.tm_min = 0;
            tm.tm_sec = 0;
            tm.tm_isdst = 0; // No DST for UTC
            start_time = timegm(&tm);
            log_info("Parsed date-only start time: %ld", (long)start_time);
        } else {
            log_error("Failed to parse start time string: %s", decoded_start_time);
        }
    } else {
        // Default to 24 hours ago
        start_time = time(NULL) - (24 * 60 * 60);
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

            // Convert to UTC timestamp - assume input is already in UTC
            tm.tm_isdst = 0; // No DST for UTC
            end_time = timegm(&tm);
            log_info("Parsed end time: %ld", (long)end_time);
        } else if (strptime(decoded_end_time, "%Y-%m-%d", &tm) != NULL) {
            // Handle date-only format (YYYY-MM-DD)
            // Set time to 23:59:59 for the end of the day
            tm.tm_hour = 23;
            tm.tm_min = 59;
            tm.tm_sec = 59;
            tm.tm_isdst = 0; // No DST for UTC
            end_time = timegm(&tm);
            log_info("Parsed date-only end time: %ld", (long)end_time);
        } else {
            log_error("Failed to parse end time string: %s", decoded_end_time);
        }
    } else {
        // Default to now
        end_time = time(NULL);
    }
    
    // Get timeline segments
    timeline_segment_t *segments = (timeline_segment_t *)malloc(MAX_TIMELINE_SEGMENTS * sizeof(timeline_segment_t));
    if (!segments) {
        log_error("Failed to allocate memory for timeline segments");
        mg_send_json_error(c, 500, "Failed to allocate memory for timeline segments");
        return;
    }
    
    int count = get_timeline_segments(stream_name, start_time, end_time, segments, MAX_TIMELINE_SEGMENTS);
    
    if (count <= 0) {
        log_error("No timeline segments found for stream %s", stream_name);
        free(segments);
        mg_send_json_error(c, 404, "No recordings found for the specified time range");
        return;
    }
    
    // Create manifest
    char manifest_path[MAX_PATH_LENGTH];
    if (create_timeline_manifest(segments, count, start_time, manifest_path) != 0) {
        log_error("Failed to create timeline manifest");
        free(segments);
        mg_send_json_error(c, 500, "Failed to create timeline manifest");
        return;
    }
    
    // Free segments
    free(segments);
    
    // Use Mongoose's built-in file serving capabilities
    // This is more stable and handles all the HTTP headers properly
    struct mg_http_serve_opts opts = {
        .mime_types = "m3u8=application/vnd.apple.mpegurl",
        .extra_headers = "Connection: close\r\nCache-Control: no-cache\r\n"
    };
    
    log_info("Serving manifest file using mg_http_serve_file: %s", manifest_path);
    mg_http_serve_file(c, hm, manifest_path, &opts);
    
    // Schedule manifest file for deletion after a while
    // In a real implementation, you might want to keep it around for a bit
    // and implement a cleanup mechanism
    
    log_info("Successfully handled GET /api/timeline/manifest request");
}


/**
 * @brief Handler for GET /api/timeline/play
 */
void mg_handle_timeline_playback(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/timeline/play request");
    
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
    
    // Parse query string without modifying it
    // Extract stream parameter
    char stream_param[MAX_STREAM_NAME] = {0};
    mg_http_get_var(&hm->query, "stream", stream_param, sizeof(stream_param));
    if (stream_param[0] != '\0') {
        strncpy(stream_name, stream_param, sizeof(stream_name) - 1);
    }
    
    // Extract start parameter
    mg_http_get_var(&hm->query, "start", start_time_str, sizeof(start_time_str));
    
    // Check required parameters
    if (stream_name[0] == '\0') {
        log_error("Missing required parameter: stream");
        mg_send_json_error(c, 400, "Missing required parameter: stream");
        return;
    }
    
    // Parse start time
    time_t start_time = 0;
    if (start_time_str[0] != '\0') {
        // Try parsing as a timestamp first
        char *endptr;
        start_time = strtol(start_time_str, &endptr, 10);
        
        // If not a valid number, try parsing as a date string
        if (*endptr != '\0') {
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

                // Convert to UTC timestamp - assume input is already in UTC
                tm.tm_isdst = 0; // No DST for UTC
                start_time = timegm(&tm);
                log_info("Parsed start time: %ld", (long)start_time);
            } else if (strptime(decoded_start_time, "%Y-%m-%d", &tm) != NULL) {
                // Handle date-only format (YYYY-MM-DD)
                // Set time to 00:00:00
                tm.tm_hour = 0;
                tm.tm_min = 0;
                tm.tm_sec = 0;
                tm.tm_isdst = 0; // No DST for UTC
                start_time = timegm(&tm);
                log_info("Parsed date-only start time: %ld", (long)start_time);
            } else {
                log_error("Failed to parse start time string: %s", decoded_start_time);
                mg_send_json_error(c, 400, "Invalid start time format");
                return;
            }
        }
    } else {
        // Default to 24 hours ago
        start_time = time(NULL) - (24 * 60 * 60);
    }
    
    // Get timeline segments
    timeline_segment_t *segments = (timeline_segment_t *)malloc(MAX_TIMELINE_SEGMENTS * sizeof(timeline_segment_t));
    if (!segments) {
        log_error("Failed to allocate memory for timeline segments");
        mg_send_json_error(c, 500, "Failed to allocate memory for timeline segments");
        return;
    }
    
    // Get segments for the next 24 hours from start time
    time_t end_time = start_time + (24 * 60 * 60);
    int count = get_timeline_segments(stream_name, start_time, end_time, segments, MAX_TIMELINE_SEGMENTS);
    
    if (count <= 0) {
        log_error("No timeline segments found for stream %s", stream_name);
        free(segments);
        mg_send_json_error(c, 404, "No recordings found for the specified time range");
        return;
    }
    
    // Find the segment that contains the start time
    int start_segment_index = -1;
    for (int i = 0; i < count; i++) {
        if (start_time >= segments[i].start_time && start_time <= segments[i].end_time) {
            start_segment_index = i;
            break;
        }
    }
    
    // If no segment contains the start time, use the first segment after the start time
    if (start_segment_index == -1) {
        for (int i = 0; i < count; i++) {
            if (start_time < segments[i].start_time) {
                start_segment_index = i;
                break;
            }
        }
    }
    
    // If still no segment found, use the first segment
    if (start_segment_index == -1 && count > 0) {
        start_segment_index = 0;
    }
    
    // Get the recording ID for the segment
    uint64_t recording_id = segments[start_segment_index].id;
    
    // Free segments
    free(segments);
    
    // Redirect to the recording playback endpoint
    char redirect_url[256];
    snprintf(redirect_url, sizeof(redirect_url), "/api/recordings/play/%llu", (unsigned long long)recording_id);
    
    log_info("Redirecting to recording playback: %s", redirect_url);
    
    // Send redirect response
    mg_printf(c, "HTTP/1.1 302 Found\r\n");
    mg_printf(c, "Connection: close\r\n");
    mg_printf(c, "Location: %s\r\n", redirect_url);
    mg_printf(c, "Content-Length: 0\r\n");
    mg_printf(c, "\r\n");
}
