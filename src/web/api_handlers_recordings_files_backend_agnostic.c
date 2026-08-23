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
#define LOG_COMPONENT "RecordingsAPI"
#include "core/logger.h"
#include "database/db_recordings.h"

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
    authorization_evaluation_t evaluation;
    int authorization_result = httpd_evaluate_stream_action(
        &user, AUTHZ_RECORDING_DELETE, recording.stream_name, &evaluation);
    if (authorization_result < 0) {
        http_response_set_json_error(
            res, 500, "Authorization policy evaluation failed");
        return;
    }
    if (authorization_result > 0 ||
        evaluation.decision != AUTHZ_DECISION_ALLOW) {
        http_response_set_json_error(res, 403, "Forbidden");
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
        log_error("Failed to delete file: %s (error: %s)", path, strerror(errno));
        http_response_set_json_error(res, 500, "Failed to delete file");
        return;
    }

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
