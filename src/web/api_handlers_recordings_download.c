/**
 * @file api_handlers_recordings_download.c
 * @brief Backend-agnostic handler for recording downloads
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <errno.h>

#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "web/audit_log.h"
#define LOG_COMPONENT "RecordingsAPI"
#include "core/logger.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"
#include "utils/strings.h"

static void audit_recording_export(
    const http_request_t *req, const user_t *user,
    const fleet_camera_t *camera, uint64_t recording_id,
    const char *outcome, const char *reason, uint64_t size_bytes,
    const char *content_type) {
    char recording_uuid[32];
    snprintf(recording_uuid, sizeof(recording_uuid), "%llu",
             (unsigned long long)recording_id);
    cJSON *details = cJSON_CreateObject();
    if (details) {
        cJSON_AddStringToObject(details, "camera_uuid", camera->camera_uuid);
        cJSON_AddStringToObject(details, "reason", reason);
        if (size_bytes > 0) {
            cJSON_AddNumberToObject(details, "size_bytes", (double)size_bytes);
        }
        if (content_type) {
            cJSON_AddStringToObject(details, "content_type", content_type);
        }
    }
    audit_log_operation(req, user, "recordings.export", "recording",
                        recording_uuid, "download_recording", outcome,
                        details);
    cJSON_Delete(details);
}

/**
 * @brief Backend-agnostic handler for GET /api/recordings/download/:id
 * 
 * Serves a recording file for download with proper Content-Disposition header.
 */
void handle_recordings_download(const http_request_t *req, http_response_t *res) {
    if (!req || !res) {
        log_error("Invalid parameters for handle_recordings_download");
        return;
    }

    user_t user;
    if (!httpd_check_action_access(req, &user)) {
        log_error("Authentication failed for GET /api/recordings/download request");
        http_response_set_json_error(res, 401, "Unauthorized");
        return;
    }

    // Extract recording ID from URL
    char id_str[32];
    if (http_request_extract_path_param(req, "/api/recordings/download/", id_str, sizeof(id_str)) != 0) {
        log_error("Failed to extract recording ID from URL");
        http_response_set_json_error(res, 400, "Invalid request path");
        return;
    }

    // Convert ID to integer
    uint64_t id = strtoull(id_str, NULL, 10);
    if (id == 0) {
        log_error("Invalid recording ID: %s", id_str);
        http_response_set_json_error(res, 400, "Invalid recording ID");
        return;
    }

    log_info("Handling GET /api/recordings/download/%llu request", (unsigned long long)id);

    // Get recording from database
    recording_metadata_t recording = {0};
    if (get_recording_metadata_by_id(id, &recording) != 0) {
        log_error("Recording not found: %llu", (unsigned long long)id);
        http_response_set_json_error(res, 404, "Recording not found");
        return;
    }
    fleet_camera_t camera;
    authorization_evaluation_t evaluation;
    if (!httpd_authorize_camera_identity_action_with_context(
            req, res, AUTHZ_RECORDINGS_EXPORT, recording.camera_uuid,
            recording.stream_name, &user, &camera, &evaluation)) {
        return;
    }

    // Check if file exists
    struct stat st;
    if (stat(recording.file_path, &st) != 0) {
        audit_recording_export(req, &user, &camera, id, "failure",
                               "recording_file_missing", 0, NULL);
        log_error("Recording file not found: %s", recording.file_path);
        http_response_set_json_error(res, 404, "Recording file not found");
        return;
    }

    // Extract filename from path
    char filename[192];
    httpd_sanitize_attachment_filename(recording.file_path, filename,
                                       sizeof(filename));
    if (filename[0] == '\0') safe_strcpy(filename, "recording.mp4",
                                          sizeof(filename), 0);

    // Determine content type based on file extension
    const char *content_type = "video/mp4"; // Default
    const char *file_ext = strrchr(recording.file_path, '.');
    if (file_ext) {
        file_ext++; // Skip the '.'
        if (strcasecmp(file_ext, "mp4") == 0) {
            content_type = "video/mp4";
        } else if (strcasecmp(file_ext, "mkv") == 0) {
            content_type = "video/x-matroska";
        } else if (strcasecmp(file_ext, "webm") == 0) {
            content_type = "video/webm";
        } else if (strcasecmp(file_ext, "avi") == 0) {
            content_type = "video/x-msvideo";
        }
    }

    // Build headers with Content-Disposition for download
    char headers[512];
    snprintf(headers, sizeof(headers),
             "Content-Disposition: attachment; filename=\"%s\"\r\n",
             filename);

    // Serve the file using backend-agnostic function
    log_debug("Serving file for download: %s", recording.file_path);
    if (http_serve_file(req, res, recording.file_path, content_type, headers) != 0) {
        audit_recording_export(req, &user, &camera, id, "error",
                               "response_stream_failed",
                               (uint64_t)st.st_size, content_type);
        log_error("Failed to serve file: %s", recording.file_path);
        http_response_set_json_error(res, 500, "Failed to serve file");
        return;
    }

    audit_recording_export(req, &user, &camera, id, "success", "completed",
                           (uint64_t)st.st_size, content_type);

    log_info("Successfully handled GET /api/recordings/download/%llu request", (unsigned long long)id);
}
