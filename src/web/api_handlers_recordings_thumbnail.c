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
#include <sys/stat.h>
#include <unistd.h>
#include <errno.h>

#include "web/api_handlers_recordings_thumbnail.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "core/logger.h"
#include "core/config.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"

/**
 * Concurrency limiter for thumbnail generation.
 *
 * Each ffmpeg system() call blocks a libuv thread-pool worker for seconds.
 * If the grid view fires dozens of thumbnail requests at once, the pool is
 * starved and the health-check endpoint can't be served, triggering a
 * server restart.  Cap the number of in-flight ffmpeg processes so there
 * are always free workers for normal API traffic.
 */
#define MAX_CONCURRENT_THUMBNAIL_GENERATIONS 4
static volatile int g_active_thumbnail_generations = 0;

/**
 * @brief Ensure the thumbnails directory exists
 */
static int ensure_thumbnails_dir(const char *storage_path) {
    char dir_path[512];
    snprintf(dir_path, sizeof(dir_path), "%s/thumbnails", storage_path);

    struct stat st;
    if (stat(dir_path, &st) == 0 && S_ISDIR(st.st_mode)) {
        return 0; // Already exists
    }

    if (mkdir(dir_path, 0755) != 0 && errno != EEXIST) {
        log_error("Failed to create thumbnails directory: %s (error: %s)",
                  dir_path, strerror(errno));
        return -1;
    }

    return 0;
}

/**
 * @brief Generate a thumbnail using ffmpeg
 */
static int generate_thumbnail(const char *input_path, const char *output_path,
                              double seek_seconds) {
    char cmd[1024];

    // Clamp seek time to 0 minimum
    if (seek_seconds < 0) seek_seconds = 0;

    // Use -ss before -i for fast seeking (input seeking)
    // -frames:v 1 to grab a single frame
    // -vf scale=320:-1 to scale to 320px wide maintaining aspect ratio
    // -q:v 8 for reasonable JPEG quality (~5-10KB)
    // timeout 5s to prevent hanging on slow/corrupted files
    snprintf(cmd, sizeof(cmd),
             "timeout 5s ffmpeg -ss %.2f -i \"%s\" -frames:v 1 -vf scale=320:-1 "
             "-q:v 8 -y \"%s\" 2>/dev/null",
             seek_seconds, input_path, output_path);

    log_debug("Generating thumbnail: %s", cmd);

    int ret = system(cmd);
    if (ret != 0) {
        log_warn("ffmpeg thumbnail generation failed (exit code %d) for: %s",
                 ret, input_path);
        return -1;
    }

    // Verify the file was created
    struct stat st;
    if (stat(output_path, &st) != 0 || st.st_size == 0) {
        log_warn("Thumbnail file not created or empty: %s", output_path);
        unlink(output_path); // Clean up empty file
        return -1;
    }

    log_debug("Generated thumbnail: %s (%lld bytes)", output_path,
              (long long)st.st_size);
    return 0;
}

void handle_recordings_thumbnail(const http_request_t *req, http_response_t *res) {
    if (!req || !res) {
        log_error("Invalid parameters for handle_recordings_thumbnail");
        return;
    }

    // Check authentication if enabled
    if (g_config.web_auth_enabled) {
        user_t user;
        if (g_config.demo_mode) {
            if (!httpd_check_viewer_access(req, &user)) {
                http_response_set_json_error(res, 401, "Unauthorized");
                return;
            }
        } else {
            if (!httpd_get_authenticated_user(req, &user)) {
                http_response_set_json_error(res, 401, "Unauthorized");
                return;
            }
        }
    }

    // Check if thumbnails are enabled
    if (!g_config.generate_thumbnails) {
        http_response_set_json_error(res, 403, "Thumbnail generation is disabled");
        return;
    }

    // Extract path parameter: /api/recordings/thumbnail/{id}/{index}
    char param_buf[64];
    if (http_request_extract_path_param(req, "/api/recordings/thumbnail/",
                                         param_buf, sizeof(param_buf)) != 0) {
        http_response_set_json_error(res, 400, "Invalid request path");
        return;
    }

    // Parse id and index from "id/index"
    char *slash = strchr(param_buf, '/');
    if (!slash) {
        http_response_set_json_error(res, 400, "Missing thumbnail index");
        return;
    }
    *slash = '\0';
    const char *id_str = param_buf;
    const char *index_str = slash + 1;

    uint64_t id = strtoull(id_str, NULL, 10);
    int index = atoi(index_str);

    if (id == 0) {
        http_response_set_json_error(res, 400, "Invalid recording ID");
        return;
    }
    if (index < 0 || index > 2) {
        http_response_set_json_error(res, 400, "Invalid thumbnail index (must be 0, 1, or 2)");
        return;
    }

    // Build thumbnail path
    char thumb_path[512];
    snprintf(thumb_path, sizeof(thumb_path), "%s/thumbnails/%llu_%d.jpg",
             g_config.storage_path, (unsigned long long)id, index);

    // Check if thumbnail already exists (cached)
    struct stat st;
    if (stat(thumb_path, &st) == 0 && st.st_size > 0) {
        // Serve cached thumbnail
        log_debug("Serving cached thumbnail: %s", thumb_path);
        if (http_serve_file(req, res, thumb_path, "image/jpeg",
                            "Cache-Control: public, max-age=86400\r\n") != 0) {
            http_response_set_json_error(res, 500, "Failed to serve thumbnail");
        }
        return;
    }

    // Thumbnail doesn't exist - need to generate it
    // Get recording metadata
    recording_metadata_t recording = {0};
    if (get_recording_metadata_by_id(id, &recording) != 0) {
        http_response_set_json_error(res, 404, "Recording not found");
        return;
    }

    // Check recording file exists
    if (stat(recording.file_path, &st) != 0) {
        http_response_set_json_error(res, 404, "Recording file not found");
        return;
    }

    // Ensure thumbnails directory exists
    if (ensure_thumbnails_dir(g_config.storage_path) != 0) {
        http_response_set_json_error(res, 500, "Failed to create thumbnails directory");
        return;
    }

    // Calculate seek time based on index
    double duration = difftime(recording.end_time, recording.start_time);
    if (duration <= 0) {
        duration = 10.0; // Fallback if duration is unknown
    }

    double seek_seconds;
    switch (index) {
        case 0:
            seek_seconds = 1.0; // 1 second in (avoid black frames at start)
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

    // Enforce concurrency limit – avoid starving the libuv thread pool
    if (__sync_fetch_and_add(&g_active_thumbnail_generations, 0) >=
        MAX_CONCURRENT_THUMBNAIL_GENERATIONS) {
        // Tell the browser to retry shortly
        http_response_add_header(res, "Retry-After", "2");
        http_response_set_json_error(res, 503,
            "Thumbnail generation busy, try again later");
        return;
    }
    __sync_add_and_fetch(&g_active_thumbnail_generations, 1);

    // Generate thumbnail
    int gen_result = generate_thumbnail(recording.file_path, thumb_path,
                                        seek_seconds);
    __sync_sub_and_fetch(&g_active_thumbnail_generations, 1);

    if (gen_result != 0) {
        http_response_set_json_error(res, 500, "Failed to generate thumbnail");
        return;
    }

    // Serve the newly generated thumbnail
    if (http_serve_file(req, res, thumb_path, "image/jpeg",
                        "Cache-Control: public, max-age=86400\r\n") != 0) {
        http_response_set_json_error(res, 500, "Failed to serve thumbnail");
    }
}

void delete_recording_thumbnails(uint64_t recording_id) {
    for (int i = 0; i < 3; i++) {
        char thumb_path[512];
        snprintf(thumb_path, sizeof(thumb_path), "%s/thumbnails/%llu_%d.jpg",
                 g_config.storage_path, (unsigned long long)recording_id, i);
        if (unlink(thumb_path) == 0) {
            log_debug("Deleted thumbnail: %s", thumb_path);
        }
        // Silently ignore if thumbnail doesn't exist (ENOENT)
    }
}

