/**
 * @file api_handlers_recordings_thumbnail.c
 * @brief Backend-agnostic handler for recording thumbnail generation and serving
 *
 * Implements lazy thumbnail generation: thumbnails are generated on first request
 * using ffmpeg, then cached to disk for subsequent requests.
 */

#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dirent.h>
#include <sys/stat.h>
#include <unistd.h>
#include <time.h>
#include <errno.h>

#include "web/api_handlers_recordings_thumbnail.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "web/thumbnail_thread.h"
#include "web/libuv_server.h"
#define LOG_COMPONENT "RecordingsAPI"
#include "core/logger.h"
#include "core/config.h"
#include "core/path_utils.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"



/**
 * @brief Ensure the thumbnails directory exists
 */
static int ensure_thumbnails_dir(const char *storage_path) {
    char dir_path[MAX_PATH_LENGTH];
    snprintf(dir_path, sizeof(dir_path), "%s/thumbnails", storage_path);

    struct stat st;
    if (stat(dir_path, &st) == 0 && S_ISDIR(st.st_mode)) {
        return 0; // Already exists
    }

    if (ensure_dir(dir_path)) {
        log_error("Failed to create thumbnails directory: %s (error: %s)",
                  dir_path, strerror(errno));
        return -1;
    }

    return 0;
}

static int ensure_investigation_thumbnail_dir(
    const char *storage_path, uint64_t recording_id, char *directory,
    size_t directory_size) {
    if (ensure_thumbnails_dir(storage_path) != 0) return -1;
    char root[MAX_PATH_LENGTH];
    snprintf(root, sizeof(root), "%s/thumbnails/investigation", storage_path);
    if (ensure_dir(root)) {
        log_error("Failed to create investigation thumbnail directory: %s", root);
        return -1;
    }
    snprintf(directory, directory_size, "%s/%llu", root,
             (unsigned long long)recording_id);
    if (ensure_dir(directory)) {
        log_error("Failed to create recording thumbnail directory: %s",
                  directory);
        return -1;
    }
    return 0;
}

/**
 * @brief Callback invoked when thumbnail generation completes
 *
 * This runs on the event loop thread via uv_async.
 */
static void thumbnail_complete_callback(deferred_action_handle_t handle,
                                       const char *output_path, int result) {
    libuv_connection_t *conn = (libuv_connection_t *)handle;

    if (result == 0 && output_path) {
        // Success - serve the generated thumbnail
        log_debug("Serving generated thumbnail: %s", output_path);
        if (libuv_serve_file(conn, output_path, "image/jpeg",
                            "Cache-Control: public, max-age=86400\r\n") != 0) {
            http_response_set_json_error(&conn->response, 500, "Failed to serve thumbnail");
            libuv_send_response_ex(conn, &conn->response, conn->deferred_action);
        }
        // libuv_serve_file handles the response and connection lifecycle
    } else {
        // Failure - send error response
        http_response_set_json_error(&conn->response, 500, "Failed to generate thumbnail");
        libuv_send_response_ex(conn, &conn->response, conn->deferred_action);
    }
}

static void serve_or_generate_thumbnail(
    const http_request_t *req, http_response_t *res,
    const recording_metadata_t *recording, uint64_t recording_id,
    int64_t sample_key, const char *thumb_path, double seek_seconds) {
    struct stat st;
    if (stat(thumb_path, &st) == 0 && st.st_size > 0) {
        log_debug("Serving cached thumbnail: %s", thumb_path);
        if (http_serve_file(req, res, thumb_path, "image/jpeg",
                            "Cache-Control: public, max-age=86400\r\n") != 0) {
            http_response_set_json_error(res, 500, "Failed to serve thumbnail");
        }
        return;
    }

    if (stat(recording->file_path, &st) != 0) {
        http_response_set_json_error(res, 404, "Recording file not found");
        return;
    }
    if (ensure_thumbnails_dir(g_config.storage_path) != 0) {
        http_response_set_json_error(res, 500,
                                     "Failed to create thumbnails directory");
        return;
    }

    libuv_connection_t *conn = (libuv_connection_t *)req->user_data;
    if (!conn) {
        log_error("Thumbnail request has no connection in request user_data");
        http_response_set_json_error(res, 500, "Internal server error");
        return;
    }
    if (thumbnail_thread_submit(
            recording_id, sample_key, recording->file_path, thumb_path,
            seek_seconds, (deferred_action_handle_t)conn,
            thumbnail_complete_callback) != 0) {
        http_response_add_header(res, "Retry-After", "2");
        http_response_set_json_error(
            res, 503, "Thumbnail generation busy, try again later");
        return;
    }

    conn->async_response_pending = true;
    log_debug("Submitted thumbnail generation for recording %llu sample %lld",
              (unsigned long long)recording_id, (long long)sample_key);
}

static bool parse_thumbnail_path(
    const http_request_t *req, const char *prefix, uint64_t *recording_id,
    int64_t *sample_key) {
    char params[96];
    if (http_request_extract_path_param(
            req, prefix, params, sizeof(params)) != 0) {
        return false;
    }
    char *slash = strchr(params, '/');
    if (!slash || slash == params || slash[1] == '\0' ||
        strchr(slash + 1, '/')) {
        return false;
    }
    *slash = '\0';

    for (const char *cursor = params; *cursor; cursor++) {
        if (*cursor < '0' || *cursor > '9') return false;
    }
    for (const char *cursor = slash + 1; *cursor; cursor++) {
        if (*cursor < '0' || *cursor > '9') return false;
    }

    char *id_end = NULL;
    char *key_end = NULL;
    errno = 0;
    unsigned long long parsed_id = strtoull(params, &id_end, 10);
    if (errno != 0 || !id_end || *id_end != '\0' || parsed_id == 0) {
        return false;
    }
    errno = 0;
    long long parsed_key = strtoll(slash + 1, &key_end, 10);
    if (errno != 0 || !key_end || *key_end != '\0' || parsed_key < 0) {
        return false;
    }
    *recording_id = (uint64_t)parsed_id;
    *sample_key = (int64_t)parsed_key;
    return true;
}

void handle_recordings_thumbnail(const http_request_t *req, http_response_t *res) {
    if (!req || !res) {
        log_error("Invalid parameters for handle_recordings_thumbnail");
        return;
    }

    // Check if thumbnails are enabled
    if (!g_config.generate_thumbnails) {
        http_response_set_json_error(res, 403, "Thumbnail generation is disabled");
        return;
    }

    uint64_t id = 0;
    int64_t parsed_index = -1;
    if (!parse_thumbnail_path(req, "/api/recordings/thumbnail/", &id,
                              &parsed_index)) {
        http_response_set_json_error(res, 400, "Invalid thumbnail path");
        return;
    }
    if (parsed_index > 2) {
        http_response_set_json_error(res, 400, "Invalid thumbnail index (must be 0, 1, or 2)");
        return;
    }
    int index = (int)parsed_index;
    // When thumbnails_per_recording is 1 (CPU-save mode, #364) the client
    // shouldn't be requesting hover frames at all. Reject cleanly so a
    // stale frontend can't force extra ffmpeg work.
    if (index > 0 && g_config.thumbnails_per_recording <= 1) {
        http_response_set_json_error(res, 403,
            "Hover frames disabled (thumbnails_per_recording=1)");
        return;
    }

    recording_metadata_t recording = {0};
    if (get_recording_metadata_by_id(id, &recording) != 0) {
        http_response_set_json_error(res, 404, "Recording not found");
        return;
    }
    user_t user;
    fleet_camera_t camera;
    authorization_evaluation_t evaluation;
    if (!httpd_authorize_camera_identity_action_with_context(
            req, res, AUTHZ_RECORDINGS_REPLAY, recording.camera_uuid,
            recording.stream_name, &user, &camera, &evaluation)) {
        return;
    }

    // Build thumbnail path
    char thumb_path[MAX_PATH_LENGTH];
    snprintf(thumb_path, sizeof(thumb_path), "%s/thumbnails/%llu_%d.jpg",
             g_config.storage_path, (unsigned long long)id, index);

    // Calculate seek time based on index
    double duration = difftime(recording.end_time, recording.start_time);
    if (duration <= 0) {
        duration = 10.0; // Fallback if duration is unknown
    }

    double seek_seconds;
    switch (index) {
        case 0:
            // First frame — no seek at all. On slow CPUs (#364) the seek
            // dominates the cost, and the earlier "1s in to dodge black
            // frames" heuristic isn't worth it: modern IP cameras don't
            // emit black intros, and the mount-time thumbnail is the one
            // users see by default, so it must be the cheapest path.
            seek_seconds = 0.0;
            break;
        case 1:
            seek_seconds = duration / 2.0;
            break;
        case 2:
            seek_seconds = duration > 2.0 ? duration - 1.0 : duration * 0.9;
            break;
        default:
            seek_seconds = 0;
            break;
    }

    // Clamp seek time to recording duration
    if (seek_seconds >= duration) {
        seek_seconds = duration > 1.0 ? duration - 1.0 : 0;
    }

    serve_or_generate_thumbnail(req, res, &recording, id, index, thumb_path,
                                seek_seconds);
}

void handle_investigation_thumbnail(const http_request_t *req,
                                    http_response_t *res) {
    if (!req || !res) return;
    if (!g_config.generate_thumbnails) {
        http_response_set_json_error(res, 403,
                                     "Thumbnail generation is disabled");
        return;
    }

    uint64_t id = 0;
    int64_t offset_ms = -1;
    if (!parse_thumbnail_path(req, "/api/investigations/thumbnail/", &id,
                              &offset_ms)) {
        http_response_set_json_error(res, 400, "Invalid thumbnail path");
        return;
    }

    recording_metadata_t recording = {0};
    if (get_recording_metadata_by_id(id, &recording) != 0) {
        http_response_set_json_error(res, 404, "Recording not found");
        return;
    }
    user_t user;
    fleet_camera_t camera;
    authorization_evaluation_t evaluation;
    if (!httpd_authorize_camera_identity_action_with_context(
            req, res, AUTHZ_RECORDINGS_REPLAY, recording.camera_uuid,
            recording.stream_name, &user, &camera, &evaluation)) {
        return;
    }

    double duration = difftime(recording.end_time, recording.start_time);
    int64_t duration_ms = duration > 0.0
        ? (int64_t)(duration * 1000.0) : 0;
    if (duration_ms <= 0 || offset_ms > duration_ms) {
        http_response_set_json_error(
            res, 400, "Thumbnail offset is outside the recording");
        return;
    }
    double seek_seconds = (double)offset_ms / 1000.0;
    if (seek_seconds >= duration) {
        seek_seconds = duration > 1.0 ? duration - 1.0 : 0.0;
    }

    char thumb_directory[MAX_PATH_LENGTH];
    if (ensure_investigation_thumbnail_dir(
            g_config.storage_path, id, thumb_directory,
            sizeof(thumb_directory)) != 0) {
        http_response_set_json_error(
            res, 500, "Failed to create thumbnail cache directory");
        return;
    }
    char thumb_path[MAX_PATH_LENGTH];
    snprintf(thumb_path, sizeof(thumb_path), "%s/%lld.jpg",
             thumb_directory, (long long)offset_ms);
    serve_or_generate_thumbnail(req, res, &recording, id, offset_ms,
                                thumb_path, seek_seconds);
}

void delete_recording_thumbnails(uint64_t recording_id) {
    for (int i = 0; i < 3; i++) {
        char thumb_path[MAX_PATH_LENGTH];
        snprintf(thumb_path, sizeof(thumb_path), "%s/thumbnails/%llu_%d.jpg",
                 g_config.storage_path, (unsigned long long)recording_id, i);
        if (unlink(thumb_path) == 0) {
            log_debug("Deleted thumbnail: %s", thumb_path);
        }
        // Silently ignore if thumbnail doesn't exist (ENOENT)
    }

    char directory[MAX_PATH_LENGTH];
    snprintf(directory, sizeof(directory),
             "%s/thumbnails/investigation/%llu", g_config.storage_path,
             (unsigned long long)recording_id);
    DIR *dir = opendir(directory);
    if (!dir) return;
    struct dirent *entry = NULL;
    while ((entry = readdir(dir)) != NULL) {
        const size_t name_length = strlen(entry->d_name);
        if (name_length <= 4 ||
            strcmp(entry->d_name + name_length - 4, ".jpg") != 0) {
            continue;
        }
        bool numeric_offset = true;
        for (size_t i = 0; i < name_length - 4; i++) {
            if (entry->d_name[i] < '0' || entry->d_name[i] > '9') {
                numeric_offset = false;
                break;
            }
        }
        if (!numeric_offset) continue;
        char thumb_path[MAX_PATH_LENGTH];
        snprintf(thumb_path, sizeof(thumb_path), "%s/%s", directory,
                 entry->d_name);
        if (unlink(thumb_path) == 0) {
            log_debug("Deleted thumbnail: %s", thumb_path);
        }
    }
    closedir(dir);
    if (rmdir(directory) == 0) {
        log_debug("Deleted thumbnail directory: %s", directory);
    }
}
