#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include "web/api_handlers_motion.h"
#include "web/api_handlers.h"
#include "web/mongoose_adapter.h"
#include "core/logger.h"
#include "database/db_motion_config.h"
#include "video/onvif_motion_recording.h"
#include "video/motion_storage_manager.h"
#include "mongoose.h"
#include <cjson/cJSON.h>

/**
 * Handler for GET /api/motion/config/:stream
 */
void mg_handle_get_motion_config(struct mg_connection *c, struct mg_http_message *hm) {
    char stream_name[256];

    // Extract stream name from URL
    if (mg_extract_path_param(hm, "/api/motion/config/", stream_name, sizeof(stream_name)) != 0) {
        mg_send_json_error(c, 400, "Invalid stream name");
        return;
    }

    log_info("GET /api/motion/config/%s", stream_name);

    // Load configuration from database
    motion_recording_config_t config;
    if (load_motion_config(stream_name, &config) != 0) {
        mg_send_json_error(c, 404, "Motion recording configuration not found");
        return;
    }

    // Create JSON response
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        mg_send_json_error(c, 500, "Failed to create JSON response");
        return;
    }

    cJSON_AddStringToObject(response, "stream_name", stream_name);
    cJSON_AddBoolToObject(response, "enabled", config.enabled);
    cJSON_AddNumberToObject(response, "pre_buffer_seconds", config.pre_buffer_seconds);
    cJSON_AddNumberToObject(response, "post_buffer_seconds", config.post_buffer_seconds);
    cJSON_AddNumberToObject(response, "max_file_duration", config.max_file_duration);
    cJSON_AddStringToObject(response, "codec", config.codec);
    cJSON_AddStringToObject(response, "quality", config.quality);
    cJSON_AddNumberToObject(response, "retention_days", config.retention_days);

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    if (json_str) {
        mg_send_json_response(c, 200, json_str);
        free(json_str);
    } else {
        mg_send_json_error(c, 500, "Failed to serialize JSON");
    }
}

/**
 * Handler for POST /api/motion/config/:stream
 */
void mg_handle_post_motion_config(struct mg_connection *c, struct mg_http_message *hm) {
    char stream_name[256];

    // Extract stream name from URL
    if (mg_extract_path_param(hm, "/api/motion/config/", stream_name, sizeof(stream_name)) != 0) {
        mg_send_json_error(c, 400, "Invalid stream name");
        return;
    }

    log_info("POST /api/motion/config/%s", stream_name);

    // Parse JSON body
    cJSON *json = mg_parse_json_body(hm);
    if (!json) {
        mg_send_json_error(c, 400, "Invalid JSON in request body");
        return;
    }

    // Build configuration from JSON
    motion_recording_config_t config = {0};

    cJSON *enabled = cJSON_GetObjectItem(json, "enabled");
    if (enabled && cJSON_IsBool(enabled)) {
        config.enabled = cJSON_IsTrue(enabled);
    } else {
        config.enabled = true;  // Default to enabled
    }

    cJSON *pre_buffer = cJSON_GetObjectItem(json, "pre_buffer_seconds");
    if (pre_buffer && cJSON_IsNumber(pre_buffer)) {
        config.pre_buffer_seconds = pre_buffer->valueint;
    } else {
        config.pre_buffer_seconds = 5;  // Default
    }

    cJSON *post_buffer = cJSON_GetObjectItem(json, "post_buffer_seconds");
    if (post_buffer && cJSON_IsNumber(post_buffer)) {
        config.post_buffer_seconds = post_buffer->valueint;
    } else {
        config.post_buffer_seconds = 10;  // Default
    }

    cJSON *max_duration = cJSON_GetObjectItem(json, "max_file_duration");
    if (max_duration && cJSON_IsNumber(max_duration)) {
        config.max_file_duration = max_duration->valueint;
    } else {
        config.max_file_duration = 300;  // Default: 5 minutes
    }

    cJSON *codec = cJSON_GetObjectItem(json, "codec");
    if (codec && cJSON_IsString(codec)) {
        strncpy(config.codec, codec->valuestring, sizeof(config.codec) - 1);
    } else {
        strncpy(config.codec, "h264", sizeof(config.codec) - 1);
    }

    cJSON *quality = cJSON_GetObjectItem(json, "quality");
    if (quality && cJSON_IsString(quality)) {
        strncpy(config.quality, quality->valuestring, sizeof(config.quality) - 1);
    } else {
        strncpy(config.quality, "medium", sizeof(config.quality) - 1);
    }

    cJSON *retention = cJSON_GetObjectItem(json, "retention_days");
    if (retention && cJSON_IsNumber(retention)) {
        config.retention_days = retention->valueint;
    } else {
        config.retention_days = 7;  // Default: 7 days
    }

    cJSON_Delete(json);

    // Enable motion recording with this configuration
    if (enable_motion_recording(stream_name, &config) != 0) {
        mg_send_json_error(c, 500, "Failed to enable motion recording");
        return;
    }

    // Return success
    cJSON *response = cJSON_CreateObject();
    cJSON_AddStringToObject(response, "message", "Motion recording configuration saved");
    cJSON_AddStringToObject(response, "stream_name", stream_name);

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    if (json_str) {
        mg_send_json_response(c, 200, json_str);
        free(json_str);
    } else {
        mg_send_json_error(c, 500, "Failed to serialize JSON");
    }
}

/**
 * Handler for DELETE /api/motion/config/:stream
 */
void mg_handle_delete_motion_config(struct mg_connection *c, struct mg_http_message *hm) {
    char stream_name[256];

    // Extract stream name from URL
    if (mg_extract_path_param(hm, "/api/motion/config/", stream_name, sizeof(stream_name)) != 0) {
        mg_send_json_error(c, 400, "Invalid stream name");
        return;
    }

    log_info("DELETE /api/motion/config/%s", stream_name);

    // Disable motion recording
    if (disable_motion_recording(stream_name) != 0) {
        mg_send_json_error(c, 500, "Failed to disable motion recording");
        return;
    }

    // Delete from database
    if (delete_motion_config(stream_name) != 0) {
        log_warn("Failed to delete motion config from database for stream: %s", stream_name);
    }

    // Return success
    cJSON *response = cJSON_CreateObject();
    cJSON_AddStringToObject(response, "message", "Motion recording disabled");
    cJSON_AddStringToObject(response, "stream_name", stream_name);

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    if (json_str) {
        mg_send_json_response(c, 200, json_str);
        free(json_str);
    } else {
        mg_send_json_error(c, 500, "Failed to serialize JSON");
    }
}

/**
 * Handler for GET /api/motion/stats/:stream
 */
void mg_handle_get_motion_stats(struct mg_connection *c, struct mg_http_message *hm) {
    char stream_name[256];

    // Extract stream name from URL
    if (mg_extract_path_param(hm, "/api/motion/stats/", stream_name, sizeof(stream_name)) != 0) {
        mg_send_json_error(c, 400, "Invalid stream name");
        return;
    }

    log_info("GET /api/motion/stats/%s", stream_name);

    // Get statistics from database
    uint64_t total_recordings = 0;
    uint64_t total_size_bytes = 0;
    time_t oldest_recording = 0;
    time_t newest_recording = 0;

    if (get_motion_recording_db_stats(stream_name, &total_recordings, &total_size_bytes,
                                      &oldest_recording, &newest_recording) != 0) {
        mg_send_json_error(c, 500, "Failed to get motion recording statistics");
        return;
    }

    // Get runtime statistics
    uint64_t total_events = 0;
    uint64_t runtime_recordings = 0;
    get_motion_recording_stats(stream_name, &runtime_recordings, &total_events);

    // Create JSON response
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        mg_send_json_error(c, 500, "Failed to create JSON response");
        return;
    }

    cJSON_AddStringToObject(response, "stream_name", stream_name);
    cJSON_AddNumberToObject(response, "total_recordings", (double)total_recordings);
    cJSON_AddNumberToObject(response, "total_size_bytes", (double)total_size_bytes);
    cJSON_AddNumberToObject(response, "total_size_mb", (double)(total_size_bytes / 1024.0 / 1024.0));
    cJSON_AddNumberToObject(response, "oldest_recording", (double)oldest_recording);
    cJSON_AddNumberToObject(response, "newest_recording", (double)newest_recording);
    cJSON_AddNumberToObject(response, "total_events", (double)total_events);
    cJSON_AddBoolToObject(response, "enabled", is_motion_recording_enabled(stream_name));

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    if (json_str) {
        mg_send_json_response(c, 200, json_str);
        free(json_str);
    } else {
        mg_send_json_error(c, 500, "Failed to serialize JSON");
    }
}

/**
 * Handler for GET /api/motion/recordings/:stream
 */
void mg_handle_get_motion_recordings(struct mg_connection *c, struct mg_http_message *hm) {
    char stream_name[256];

    // Extract stream name from URL
    if (mg_extract_path_param(hm, "/api/motion/recordings/", stream_name, sizeof(stream_name)) != 0) {
        mg_send_json_error(c, 400, "Invalid stream name");
        return;
    }

    log_info("GET /api/motion/recordings/%s", stream_name);

    // Get list of recordings
    char paths[100][512];
    time_t timestamps[100];
    uint64_t sizes[100];

    int count = get_motion_recordings_list(stream_name, 0, 0, paths, timestamps, sizes, 100);
    if (count < 0) {
        mg_send_json_error(c, 500, "Failed to get motion recordings list");
        return;
    }

    // Create JSON array
    cJSON *recordings = cJSON_CreateArray();
    if (!recordings) {
        mg_send_json_error(c, 500, "Failed to create JSON array");
        return;
    }

    for (int i = 0; i < count; i++) {
        cJSON *recording = cJSON_CreateObject();
        if (!recording) continue;

        cJSON_AddStringToObject(recording, "file_path", paths[i]);
        cJSON_AddNumberToObject(recording, "timestamp", (double)timestamps[i]);
        cJSON_AddNumberToObject(recording, "size_bytes", (double)sizes[i]);
        cJSON_AddNumberToObject(recording, "size_mb", (double)(sizes[i] / 1024.0 / 1024.0));

        cJSON_AddItemToArray(recordings, recording);
    }

    // Create response
    cJSON *response = cJSON_CreateObject();
    cJSON_AddStringToObject(response, "stream_name", stream_name);
    cJSON_AddNumberToObject(response, "count", count);
    cJSON_AddItemToObject(response, "recordings", recordings);

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    if (json_str) {
        mg_send_json_response(c, 200, json_str);
        free(json_str);
    } else {
        mg_send_json_error(c, 500, "Failed to serialize JSON");
    }
}

/**
 * Handler for DELETE /api/motion/recordings/:id
 */
void mg_handle_delete_motion_recording(struct mg_connection *c, struct mg_http_message *hm) {
    char file_path[512];

    // Extract file path from URL parameter
    if (mg_extract_path_param(hm, "/api/motion/recordings/", file_path, sizeof(file_path)) != 0) {
        mg_send_json_error(c, 400, "Invalid file path");
        return;
    }

    log_info("DELETE /api/motion/recordings (path: %s)", file_path);

    // Delete the recording
    if (delete_motion_recording(file_path) != 0) {
        mg_send_json_error(c, 500, "Failed to delete motion recording");
        return;
    }

    // Return success
    cJSON *response = cJSON_CreateObject();
    cJSON_AddStringToObject(response, "message", "Motion recording deleted");
    cJSON_AddStringToObject(response, "file_path", file_path);

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    if (json_str) {
        mg_send_json_response(c, 200, json_str);
        free(json_str);
    } else {
        mg_send_json_error(c, 500, "Failed to serialize JSON");
    }
}

/**
 * Handler for POST /api/motion/cleanup
 */
void mg_handle_post_motion_cleanup(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("POST /api/motion/cleanup");

    // Parse JSON body
    cJSON *json = mg_parse_json_body(hm);
    if (!json) {
        mg_send_json_error(c, 400, "Invalid JSON in request body");
        return;
    }

    const char *stream_name = NULL;
    int retention_days = 7;

    cJSON *stream = cJSON_GetObjectItem(json, "stream_name");
    if (stream && cJSON_IsString(stream)) {
        stream_name = stream->valuestring;
    }

    cJSON *retention = cJSON_GetObjectItem(json, "retention_days");
    if (retention && cJSON_IsNumber(retention)) {
        retention_days = retention->valueint;
    }

    cJSON_Delete(json);

    // Perform cleanup
    int deleted = cleanup_old_recordings(stream_name, retention_days);
    if (deleted < 0) {
        mg_send_json_error(c, 500, "Failed to cleanup old recordings");
        return;
    }

    // Return success
    cJSON *response = cJSON_CreateObject();
    cJSON_AddStringToObject(response, "message", "Cleanup completed");
    cJSON_AddNumberToObject(response, "deleted_count", deleted);
    cJSON_AddNumberToObject(response, "retention_days", retention_days);
    if (stream_name) {
        cJSON_AddStringToObject(response, "stream_name", stream_name);
    }

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    if (json_str) {
        mg_send_json_response(c, 200, json_str);
        free(json_str);
    } else {
        mg_send_json_error(c, 500, "Failed to serialize JSON");
    }
}

/**
 * Handler for GET /api/motion/storage
 */
void mg_handle_get_motion_storage(struct mg_connection *c, struct mg_http_message *hm) {
    (void)hm;  // Unused

    log_info("GET /api/motion/storage");

    // Get storage statistics for all streams
    motion_storage_stats_t stats;
    if (get_motion_storage_stats(NULL, &stats) != 0) {
        mg_send_json_error(c, 500, "Failed to get storage statistics");
        return;
    }

    // Create JSON response
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        mg_send_json_error(c, 500, "Failed to create JSON response");
        return;
    }

    cJSON_AddNumberToObject(response, "total_recordings", (double)stats.total_recordings);
    cJSON_AddNumberToObject(response, "total_size_bytes", (double)stats.total_size_bytes);
    cJSON_AddNumberToObject(response, "total_size_mb", (double)(stats.total_size_bytes / 1024.0 / 1024.0));
    cJSON_AddNumberToObject(response, "total_size_gb", (double)(stats.total_size_bytes / 1024.0 / 1024.0 / 1024.0));
    cJSON_AddNumberToObject(response, "oldest_recording", (double)stats.oldest_recording);
    cJSON_AddNumberToObject(response, "newest_recording", (double)stats.newest_recording);
    cJSON_AddNumberToObject(response, "disk_space_total", (double)stats.disk_space_total);
    cJSON_AddNumberToObject(response, "disk_space_available", (double)stats.disk_space_available);
    cJSON_AddNumberToObject(response, "disk_space_used_percent",
                           (double)((stats.disk_space_total - stats.disk_space_available) * 100.0 / stats.disk_space_total));

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    if (json_str) {
        mg_send_json_response(c, 200, json_str);
        free(json_str);
    } else {
        mg_send_json_error(c, 500, "Failed to serialize JSON");
    }
}


/**
 * Handler for POST /api/motion/test/:stream
 * Simulates a motion event for testing purposes
 */
void mg_handle_test_motion_event(struct mg_connection *c, struct mg_http_message *hm) {
    char stream_name[256] = {0};

    if (mg_extract_path_param(hm, "/api/motion/test/", stream_name, sizeof(stream_name)) != 0) {
        mg_send_json_error(c, 400, "Invalid stream name");
        return;
    }

    // URL-decode in case the name contains special characters
    char decoded_name[256] = {0};
    mg_url_decode(stream_name, strlen(stream_name), decoded_name, sizeof(decoded_name), 0);

    log_info("POST /api/motion/test/%s", decoded_name);

    // Check if motion recording is enabled for this stream
    if (!is_motion_recording_enabled(decoded_name)) {
        mg_send_json_error(c, 400, "Motion recording not enabled for this stream");
        return;
    }

    // Trigger a motion event
    time_t now = time(NULL);
    int rc = process_motion_event(decoded_name, true, now);

    // Build response
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        mg_send_json_error(c, 500, "Failed to create JSON response");
        return;
    }
    cJSON_AddStringToObject(response, "stream_name", decoded_name);
    cJSON_AddBoolToObject(response, "success", rc == 0);
    cJSON_AddStringToObject(response, "message", rc == 0 ? "Test motion event triggered" : "Failed to trigger test motion event");

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);
    if (json_str) {
        mg_send_json_response(c, 200, json_str);
        free(json_str);
    } else {
        mg_send_json_error(c, 500, "Failed to serialize JSON");
    }
}
