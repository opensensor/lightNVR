/**
 * @file api_handlers_recordings_playback.c
 * @brief Backend-agnostic handler for recording playback
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <errno.h>
#include <pthread.h>

#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "core/logger.h"
#include "core/config.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"

// Active request tracking to prevent duplicate requests
#define MAX_ACTIVE_REQUESTS 100

typedef struct {
    uint64_t id;
    bool active;
} active_request_t;

static active_request_t active_requests[MAX_ACTIVE_REQUESTS];
static pthread_mutex_t active_requests_mutex = PTHREAD_MUTEX_INITIALIZER;
static bool active_requests_initialized = false;

static void init_active_requests(void) {
    pthread_mutex_lock(&active_requests_mutex);
    if (!active_requests_initialized) {
        memset(active_requests, 0, sizeof(active_requests));
        active_requests_initialized = true;
    }
    pthread_mutex_unlock(&active_requests_mutex);
}

static bool is_request_active(uint64_t id) {
    pthread_mutex_lock(&active_requests_mutex);
    for (int i = 0; i < MAX_ACTIVE_REQUESTS; i++) {
        if (active_requests[i].active && active_requests[i].id == id) {
            pthread_mutex_unlock(&active_requests_mutex);
            return true;
        }
    }
    pthread_mutex_unlock(&active_requests_mutex);
    return false;
}

static bool mark_request_active(uint64_t id) {
    pthread_mutex_lock(&active_requests_mutex);
    for (int i = 0; i < MAX_ACTIVE_REQUESTS; i++) {
        if (!active_requests[i].active) {
            active_requests[i].id = id;
            active_requests[i].active = true;
            pthread_mutex_unlock(&active_requests_mutex);
            return true;
        }
    }
    pthread_mutex_unlock(&active_requests_mutex);
    return false;
}

static void mark_request_inactive(uint64_t id) {
    pthread_mutex_lock(&active_requests_mutex);
    for (int i = 0; i < MAX_ACTIVE_REQUESTS; i++) {
        if (active_requests[i].active && active_requests[i].id == id) {
            active_requests[i].active = false;
            break;
        }
    }
    pthread_mutex_unlock(&active_requests_mutex);
}

/**
 * @brief Backend-agnostic handler for GET /api/recordings/play/:id
 * 
 * Serves a recording file for playback with range request support for seeking.
 */
void handle_recordings_playback(const http_request_t *req, http_response_t *res) {
    if (!req || !res) {
        log_error("Invalid parameters for handle_recordings_playback");
        return;
    }

    // Initialize active requests tracking
    init_active_requests();

    // Check authentication if enabled
    if (g_config.web_auth_enabled) {
        user_t user;
        if (!httpd_get_authenticated_user(req, &user)) {
            log_error("Authentication failed for recording playback request");
            http_response_set_json_error(res, 401, "Unauthorized");
            return;
        }
    }

    // Extract recording ID from URL
    char id_str[32];
    if (http_request_extract_path_param(req, "/api/recordings/play/", id_str, sizeof(id_str)) != 0) {
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

    log_info("Handling GET /api/recordings/play/%llu request", (unsigned long long)id);

    // Get recording from database
    recording_metadata_t recording = {0};
    if (get_recording_metadata_by_id(id, &recording) != 0) {
        log_error("Recording not found: %llu", (unsigned long long)id);
        http_response_set_json_error(res, 404, "Recording not found");
        return;
    }

    // Validate file path
    if (recording.file_path[0] == '\0') {
        log_error("Recording has empty file path: %llu", (unsigned long long)id);
        http_response_set_json_error(res, 500, "Recording has invalid file path");
        return;
    }

    // Check if file exists
    struct stat st;
    if (stat(recording.file_path, &st) != 0) {
        log_error("Recording file not found: %s (error: %s)", recording.file_path, strerror(errno));
        http_response_set_json_error(res, 404, "Recording file not found");
        return;
    }

    log_info("Serving file for playback: %s (%ld bytes)", recording.file_path, st.st_size);

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

    log_info("Using content type: %s for file: %s", content_type, recording.file_path);

    // Build headers with CORS and range support
    const char *headers = "Accept-Ranges: bytes\r\n"
                         "Access-Control-Allow-Origin: *\r\n"
                         "Access-Control-Allow-Methods: GET, OPTIONS\r\n"
                         "Access-Control-Allow-Headers: Range, Origin, Content-Type, Accept\r\n";

    // Check for Range header
    const char *range_header = http_request_get_header(req, "Range");
    if (range_header) {
        log_info("Range request: %s", range_header);
    }

    // Serve the file using backend-agnostic function
    // Note: This is async and will complete in background callbacks
    log_info("Serving file for playback using backend-agnostic file server");
    if (http_serve_file(req, res, recording.file_path, content_type, headers) != 0) {
        log_error("Failed to serve file: %s", recording.file_path);
        http_response_set_json_error(res, 500, "Failed to serve file");
        return;
    }

    log_info("File serving initiated for GET /api/recordings/play/%llu", (unsigned long long)id);
}

