#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>
#include <errno.h>
#include <cjson/cJSON.h>

#include "web/api_handlers_recordings.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "web/audit_log.h"
#define LOG_COMPONENT "RecordingsAPI"
#include "core/logger.h"
#include "database/db_recordings.h"

static void audit_recording_file_delete(
    const http_request_t *req, const user_t *user,
    const fleet_camera_t *camera, uint64_t recording_id,
    const char *outcome, const char *reason, const char *file_state) {
    char recording_uuid[32];
    snprintf(recording_uuid, sizeof(recording_uuid), "%llu",
             (unsigned long long)recording_id);
    cJSON *details = cJSON_CreateObject();
    if (details) {
        cJSON_AddStringToObject(details, "camera_uuid", camera->camera_uuid);
        cJSON_AddStringToObject(details, "reason", reason);
        cJSON_AddStringToObject(details, "file_state", file_state);
    }
    audit_log_operation(req, user, "recording.delete", "recording",
                        recording_uuid, "delete_recording_file", outcome,
                        details);
    cJSON_Delete(details);
}

/**
 * @brief Handle GET /api/recordings/files/check
 * 
 * Checks if a recording file exists and returns its metadata.
 * Query parameter: path (URL-encoded file path)
 * 
 * Response:
 * {
 *   "exists": true/false,
 *   "size": <file size in bytes> (if exists),
 *   "mtime": <modification time as unix timestamp> (if exists)
 * }
 */
void handle_check_recording_file(const http_request_t *req, http_response_t *res) {
    log_info("Handling GET /api/recordings/files/check request");

    // Extract path from query parameter
    char path[MAX_PATH_LENGTH];
    if (http_request_get_query_param(req, "path", path, sizeof(path)) < 0) {
        log_error("Missing path parameter");
        http_response_set_json_error(res, 400, "Missing path parameter");
        return;
    }

    recording_metadata_t recording;
    if (get_recording_metadata_by_path(path, &recording) != 0) {
        http_response_set_json_error(res, 404, "Recording not found");
        return;
    }
    user_t user;
    fleet_camera_t camera;
    authorization_evaluation_t evaluation;
    if (!httpd_authorize_stream_action_with_context(
            req, res, AUTHZ_RECORDINGS_REPLAY, recording.stream_name, &user,
            &camera, &evaluation)) {
        return;
    }

    log_info("Checking file: %s", path);
    
    // Check if file exists
    struct stat st;
    bool exists = (stat(path, &st) == 0);
    
    // Create response JSON
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        log_error("Failed to create JSON response");
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    
    cJSON_AddBoolToObject(response, "exists", exists);
    if (exists) {
        cJSON_AddNumberToObject(response, "size", (double)st.st_size);
        cJSON_AddNumberToObject(response, "mtime", (double)st.st_mtime);
    }
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);
    
    if (!json_str) {
        log_error("Failed to convert response JSON to string");
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    
    // Send response
    http_response_set_json(res, 200, json_str);
    free(json_str);
    
    log_info("Successfully checked file: %s (exists: %d)", path, exists);
}

/**
 * @brief Handle DELETE /api/recordings/files
 * 
 * Deletes a recording file from the filesystem.
 * Query parameter: path (URL-encoded file path)
 * 
 * Response:
 * {
 *   "success": true,
 *   "existed": true/false (whether file existed before deletion)
 * }
 */
void handle_delete_recording_file(const http_request_t *req, http_response_t *res) {
    log_info("Handling DELETE /api/recordings/files request");

    user_t user;
    if (!httpd_check_action_access(req, &user)) {
        http_response_set_json_error(res, 401, "Unauthorized");
        return;
    }

    // Extract path from query parameter
    char path[MAX_PATH_LENGTH];
    if (http_request_get_query_param(req, "path", path, sizeof(path)) < 0) {
        log_error("Missing path parameter");
        http_response_set_json_error(res, 400, "Missing path parameter");
        return;
    }

    recording_metadata_t recording;
    if (get_recording_metadata_by_path(path, &recording) != 0) {
        http_response_set_json_error(res, 404, "Recording not found");
        return;
    }
    fleet_camera_t camera;
    authorization_evaluation_t evaluation;
    if (!httpd_authorize_stream_action_with_context(
            req, res, AUTHZ_RECORDING_DELETE, recording.stream_name, &user,
            &camera, &evaluation)) {
        return;
    }

    log_info("Deleting file: %s", path);

    // Attempt unlink directly instead of stat-then-unlink to avoid TOCTOU (#36).
    // Derive 'existed' from the result so the response JSON remains accurate.
    bool existed;
    if (unlink(path) == 0) {
        existed = true;
        log_info("Successfully deleted file: %s", path);
    } else if (errno == ENOENT) {
        existed = false;
        log_info("File doesn't exist, no need to delete: %s", path);
    } else {
        audit_recording_file_delete(req, &user, &camera, recording.id,
                                    "error", "filesystem_delete_failed",
                                    "unchanged");
        log_error("Failed to delete file: %s (error: %s)", path, strerror(errno));
        http_response_set_json_error(res, 500, "Failed to delete file");
        return;
    }

    audit_recording_file_delete(
        req, &user, &camera, recording.id, "success", "completed",
        existed ? "deleted" : "already_missing");

    // Create response JSON
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        log_error("Failed to create JSON response");
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }

    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddBoolToObject(response, "existed", existed);

    // Convert to string
    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    if (!json_str) {
        log_error("Failed to convert response JSON to string");
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }

    // Send response
    http_response_set_json(res, 200, json_str);
    free(json_str);
}
