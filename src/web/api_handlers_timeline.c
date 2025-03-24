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

// Implementation of the API handlers defined in the header
void handle_get_timeline_segments(const http_request_t *request, http_response_t *response) {
    // This is a wrapper function that would convert the request/response to Mongoose format
    // In a real implementation, this would call the Mongoose handler after conversion
    // For now, we'll just return a simple response
    response->status_code = 200;
    strcpy(response->content_type, "application/json");
    response->body = strdup("{\"message\": \"Timeline segments API not implemented in HTTP handler format\"}");
    response->body_length = strlen(response->body);
}

void handle_timeline_manifest(const http_request_t *request, http_response_t *response) {
    // This is a wrapper function that would convert the request/response to Mongoose format
    // In a real implementation, this would call the Mongoose handler after conversion
    // For now, we'll just return a simple response
    response->status_code = 200;
    strcpy(response->content_type, "application/json");
    response->body = strdup("{\"message\": \"Timeline manifest API not implemented in HTTP handler format\"}");
    response->body_length = strlen(response->body);
}

void handle_timeline_playback(const http_request_t *request, http_response_t *response) {
    // This is a wrapper function that would convert the request/response to Mongoose format
    // In a real implementation, this would call the Mongoose handler after conversion
    // For now, we'll just return a simple response
    response->status_code = 200;
    strcpy(response->content_type, "application/json");
    response->body = strdup("{\"message\": \"Timeline playback API not implemented in HTTP handler format\"}");
    response->body_length = strlen(response->body);
}

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
 * Direct handler for GET /api/timeline/segments
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
    
    // Parse query string
    char *param = strtok(query_string, "&");
    while (param) {
        if (strncmp(param, "stream=", 7) == 0) {
            strncpy(stream_name, param + 7, sizeof(stream_name) - 1);
        } else if (strncmp(param, "start=", 6) == 0) {
            strncpy(start_time_str, param + 6, sizeof(start_time_str) - 1);
        } else if (strncmp(param, "end=", 4) == 0) {
            strncpy(end_time_str, param + 4, sizeof(end_time_str) - 1);
        }
        param = strtok(NULL, "&");
    }
    
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
            
            // Set tm_isdst to -1 to let mktime determine if DST is in effect
            tm.tm_isdst = -1;
            start_time = mktime(&tm);
            log_info("Parsed start time: %ld", (long)start_time);
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
            
            // Set tm_isdst to -1 to let mktime determine if DST is in effect
            tm.tm_isdst = -1;
            end_time = mktime(&tm);
            log_info("Parsed end time: %ld", (long)end_time);
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
    
    // Format timestamps for display
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
        
        // Format timestamps
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
        cJSON_AddNumberToObject(segment, "start_timestamp", (double)segments[i].start_time);
        cJSON_AddNumberToObject(segment, "end_timestamp", (double)segments[i].end_time);
        
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
    
    // Clean up
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
    
    // Write segments to manifest
    for (int i = start_segment_index; i < segment_count; i++) {
        // Calculate duration
        double duration = difftime(segments[i].end_time, segments[i].start_time);
        
        // Check if the file exists
        struct stat file_st;
        if (stat(segments[i].file_path, &file_st) != 0) {
            log_warn("Recording file not found: %s, skipping in manifest", segments[i].file_path);
            continue;
        }
        
        // Write segment info with more precise duration
        fprintf(manifest, "#EXTINF:%.6f,\n", duration);
        
        // Add additional HLS tags to help with MP4 playback
        fprintf(manifest, "#EXT-X-BYTERANGE:%lld@0\n", (long long)file_st.st_size);
        
        // Use the direct file path for the recording
        fprintf(manifest, "/api/recordings/play/%llu\n", (unsigned long long)segments[i].id);
    }
    
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
 * Direct handler for GET /api/timeline/manifest
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
    
    // Parse query string
    char *param = strtok(query_string, "&");
    while (param) {
        if (strncmp(param, "stream=", 7) == 0) {
            strncpy(stream_name, param + 7, sizeof(stream_name) - 1);
        } else if (strncmp(param, "start=", 6) == 0) {
            strncpy(start_time_str, param + 6, sizeof(start_time_str) - 1);
        } else if (strncmp(param, "end=", 4) == 0) {
            strncpy(end_time_str, param + 4, sizeof(end_time_str) - 1);
        }
        param = strtok(NULL, "&");
    }
    
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
            
            // Set tm_isdst to -1 to let mktime determine if DST is in effect
            tm.tm_isdst = -1;
            start_time = mktime(&tm);
            log_info("Parsed start time: %ld", (long)start_time);
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
            
            // Set tm_isdst to -1 to let mktime determine if DST is in effect
            tm.tm_isdst = -1;
            end_time = mktime(&tm);
            log_info("Parsed end time: %ld", (long)end_time);
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
    
    // Open manifest file
    FILE *manifest_file = fopen(manifest_path, "r");
    if (!manifest_file) {
        log_error("Failed to open manifest file: %s", manifest_path);
        mg_send_json_error(c, 500, "Failed to open manifest file");
        return;
    }
    
    // Get file size
    fseek(manifest_file, 0, SEEK_END);
    long file_size = ftell(manifest_file);
    fseek(manifest_file, 0, SEEK_SET);
    
    // Read file content
    char *manifest_content = (char *)malloc(file_size + 1);
    if (!manifest_content) {
        log_error("Failed to allocate memory for manifest content");
        fclose(manifest_file);
        mg_send_json_error(c, 500, "Failed to allocate memory for manifest content");
        return;
    }
    
    size_t bytes_read = fread(manifest_content, 1, file_size, manifest_file);
    manifest_content[bytes_read] = '\0';
    
    // Close file
    fclose(manifest_file);
    
    // Send response
    mg_printf(c, "HTTP/1.1 200 OK\r\n");
    mg_printf(c, "Content-Type: application/vnd.apple.mpegurl\r\n");
    mg_printf(c, "Content-Length: %ld\r\n", bytes_read);
    mg_printf(c, "Cache-Control: no-cache\r\n");
    mg_printf(c, "\r\n");
    mg_send(c, manifest_content, bytes_read);
    
    // Clean up
    free(manifest_content);
    
    // Schedule manifest file for deletion after a while
    // In a real implementation, you might want to keep it around for a bit
    // and implement a cleanup mechanism
    
    log_info("Successfully handled GET /api/timeline/manifest request");
}

/**
 * Direct handler for GET /api/timeline/play
 */
void mg_handle_timeline_playback(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/timeline/play request");
    
    // This endpoint redirects to the manifest endpoint
    // It's a convenience endpoint for the frontend
    
    // Send redirect response
    mg_printf(c, "HTTP/1.1 302 Found\r\n");
    mg_printf(c, "Location: /api/timeline/manifest%.*s\r\n", (int)hm->query.len, mg_str_get_ptr(&hm->query));
    mg_printf(c, "\r\n");
    
    log_info("Redirected to /api/timeline/manifest");
}
