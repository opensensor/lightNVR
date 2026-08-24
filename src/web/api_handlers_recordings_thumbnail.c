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
#include "utils/strings.h"

// Investigation drill-down caches one file per requested offset, and the
// offset comes straight off the URL. Quantizing to the grid the sample
// endpoint actually emits (whole seconds) keeps every legitimate UI request a
// cache hit while capping the key space at one frame per recording-second
// instead of one per millisecond.
#define INVESTIGATION_THUMBNAIL_OFFSET_GRANULARITY_MS 1000

// Hard ceiling on cached frames per recording. Exactly one file is written per
// cache miss, so evicting the single oldest entry once the directory is full
// pins it at the ceiling however many distinct offsets a client walks through.
#define INVESTIGATION_THUMBNAIL_CACHE_MAX_FILES 256

/**
 * @brief Parameters for serving (or lazily generating) one cached frame
 */
typedef struct {
    uint64_t recording_id;
    int64_t sample_key;
    const char *thumb_path;
    /** Directory to create/prune before generating, NULL for the fixed cache */
    const char *cache_directory;
    double seek_seconds;
    /** False for callers that may read the cache but must not spend ffmpeg */
    bool allow_generation;
} thumbnail_request_t;

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

static void investigation_thumbnail_dir_path(
    const char *storage_path, uint64_t recording_id, char *directory,
    size_t directory_size) {
    snprintf(directory, directory_size, "%s/thumbnails/investigation/%llu",
             storage_path, (unsigned long long)recording_id);
}

static bool is_investigation_thumbnail_name(const char *name) {
    const size_t length = strlen(name);
    if (length <= 4 || strcmp(name + length - 4, ".jpg") != 0) {
        return false;
    }
    for (size_t i = 0; i < length - 4; i++) {
        if (name[i] < '0' || name[i] > '9') return false;
    }
    return true;
}

/**
 * @brief Drop the oldest cached frame when a recording's cache is at capacity
 *
 * Called just before a cache miss adds a file, which keeps the directory
 * pinned at INVESTIGATION_THUMBNAIL_CACHE_MAX_FILES regardless of how many
 * distinct offsets are requested.
 */
static void evict_investigation_thumbnail_if_full(const char *directory) {
    DIR *dir = opendir(directory);
    if (!dir) return;

    int count = 0;
    char oldest_name[256] = {0};
    time_t oldest_mtime = 0;
    struct dirent *entry = NULL;
    while ((entry = readdir(dir)) != NULL) {
        if (!is_investigation_thumbnail_name(entry->d_name)) continue;
        count++;
        char candidate[MAX_PATH_LENGTH];
        snprintf(candidate, sizeof(candidate), "%s/%s", directory,
                 entry->d_name);
        struct stat st;
        if (stat(candidate, &st) != 0) continue;
        if (oldest_name[0] == '\0' || st.st_mtime < oldest_mtime) {
            oldest_mtime = st.st_mtime;
            safe_strcpy(oldest_name, entry->d_name, sizeof(oldest_name), 0);
        }
    }
    closedir(dir);

    if (count < INVESTIGATION_THUMBNAIL_CACHE_MAX_FILES ||
        oldest_name[0] == '\0') {
        return;
    }
    char victim[MAX_PATH_LENGTH];
    snprintf(victim, sizeof(victim), "%s/%s", directory, oldest_name);
    if (unlink(victim) == 0) {
        log_debug("Evicted cached investigation thumbnail: %s", victim);
    }
}

/**
 * @brief Create a recording's investigation cache directory and make room
 *
 * Deliberately deferred until a frame is actually about to be generated, so a
 * request that only reads the cache (or 404s on a missing recording file)
 * never leaves an empty directory behind.
 */
static int prepare_investigation_thumbnail_dir(const char *storage_path,
                                               const char *directory) {
    char root[MAX_PATH_LENGTH];
    snprintf(root, sizeof(root), "%s/thumbnails/investigation", storage_path);
    if (ensure_dir(root)) {
        log_error("Failed to create investigation thumbnail directory: %s", root);
        return -1;
    }
    if (ensure_dir(directory)) {
        log_error("Failed to create recording thumbnail directory: %s",
                  directory);
        return -1;
    }
    evict_investigation_thumbnail_if_full(directory);
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
    const recording_metadata_t *recording,
    const thumbnail_request_t *request) {
    struct stat st;
    if (stat(request->thumb_path, &st) == 0 && st.st_size > 0) {
        log_debug("Serving cached thumbnail: %s", request->thumb_path);
        if (http_serve_file(req, res, request->thumb_path, "image/jpeg",
                            "Cache-Control: public, max-age=86400\r\n") != 0) {
            http_response_set_json_error(res, 500, "Failed to serve thumbnail");
        }
        return;
    }

    if (!request->allow_generation) {
        http_response_set_json_error(
            res, 403, "Thumbnail generation is not permitted for this session");
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
    if (request->cache_directory &&
        prepare_investigation_thumbnail_dir(g_config.storage_path,
                                            request->cache_directory) != 0) {
        http_response_set_json_error(
            res, 500, "Failed to create thumbnail cache directory");
        return;
    }

    libuv_connection_t *conn = (libuv_connection_t *)req->user_data;
    if (!conn) {
        log_error("Thumbnail request has no connection in request user_data");
        http_response_set_json_error(res, 500, "Internal server error");
        return;
    }
    if (thumbnail_thread_submit(
            request->recording_id, request->sample_key, recording->file_path,
            request->thumb_path, request->seek_seconds,
            (deferred_action_handle_t)conn,
            thumbnail_complete_callback) != 0) {
        http_response_add_header(res, "Retry-After", "2");
        http_response_set_json_error(
            res, 503, "Thumbnail generation busy, try again later");
        return;
    }

    conn->async_response_pending = true;
    log_debug("Submitted thumbnail generation for recording %llu sample %lld",
              (unsigned long long)request->recording_id,
              (long long)request->sample_key);
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

    thumbnail_request_t request = {
        .recording_id = id,
        .sample_key = index,
        .thumb_path = thumb_path,
        .cache_directory = NULL,
        .seek_seconds = seek_seconds,
        .allow_generation = true,
    };
    serve_or_generate_thumbnail(req, res, &recording, &request);
}

void handle_investigation_thumbnail(const http_request_t *req,
                                    http_response_t *res) {
    if (!req || !res) return;
    if (!g_config.generate_thumbnails) {
        http_response_set_json_error(res, 403,
                                     "Thumbnail generation is disabled");
        return;
    }
    // CPU-save mode (#364) exists to keep slow devices off the ffmpeg path.
    // Drill-down decodes an arbitrary offset, so it is strictly more expensive
    // than the hover frames that mode already rejects.
    if (g_config.thumbnails_per_recording <= 1) {
        http_response_set_json_error(
            res, 403,
            "Thumbnail drill-down disabled (thumbnails_per_recording=1)");
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
    // Snap to the cache grid before it is used as a filename or a seek target,
    // so a client walking millisecond offsets keeps hitting the same files.
    offset_ms = (offset_ms / INVESTIGATION_THUMBNAIL_OFFSET_GRANULARITY_MS) *
                INVESTIGATION_THUMBNAIL_OFFSET_GRANULARITY_MS;
    double seek_seconds = (double)offset_ms / 1000.0;
    if (seek_seconds >= duration) {
        seek_seconds = duration > 1.0 ? duration - 1.0 : 0.0;
    }

    char thumb_directory[MAX_PATH_LENGTH];
    investigation_thumbnail_dir_path(g_config.storage_path, id, thumb_directory,
                                     sizeof(thumb_directory));
    char thumb_path[MAX_PATH_LENGTH];
    snprintf(thumb_path, sizeof(thumb_path), "%s/%lld.jpg",
             thumb_directory, (long long)offset_ms);

    thumbnail_request_t request = {
        .recording_id = id,
        .sample_key = offset_ms,
        .thumb_path = thumb_path,
        .cache_directory = thumb_directory,
        .seek_seconds = seek_seconds,
        // Demo visitors are unauthenticated. Let them read frames the cache
        // already holds, but never spend ffmpeg time or disk on their behalf.
        .allow_generation = strcmp(user.authentication_method, "demo") != 0,
    };
    serve_or_generate_thumbnail(req, res, &recording, &request);
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
    investigation_thumbnail_dir_path(g_config.storage_path, recording_id,
                                     directory, sizeof(directory));
    DIR *dir = opendir(directory);
    if (!dir) return;
    struct dirent *entry = NULL;
    while ((entry = readdir(dir)) != NULL) {
        if (!is_investigation_thumbnail_name(entry->d_name)) continue;
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
