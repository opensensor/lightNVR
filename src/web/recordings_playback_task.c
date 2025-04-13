#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <unistd.h>
#include <sys/stat.h>
#include <dirent.h>
#include <pthread.h>
#include <time.h>
#include <errno.h>

#include "web/recordings_playback_task.h"
#include "web/recordings_playback_state.h"
#include "web/mongoose_adapter.h"
#include "web/mongoose_server_auth.h"
#include "web/http_server.h"
#include "web/api_handlers.h"
#include "core/logger.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"
#include "web/mongoose_server_multithreading.h"

/**
 * @brief Create a playback recording task
 *
 * @param c Mongoose connection
 * @param id Recording ID
 * @param hm HTTP message
 * @return playback_recording_task_t* Pointer to the task or NULL on error
 */
playback_recording_task_t *playback_recording_task_create(struct mg_connection *c, uint64_t id, struct mg_http_message *hm) {
    playback_recording_task_t *task = calloc(1, sizeof(playback_recording_task_t));
    if (!task) {
        log_error("Failed to allocate memory for playback recording task");
        return NULL;
    }

    task->connection = c;
    task->id = id;
    task->hm = hm;

    // Check for Range header
    struct mg_str *range_header = mg_http_get_header(hm, "Range");
    if (range_header && range_header->len > 0) {
        task->range_header = malloc(range_header->len + 1);
        if (task->range_header) {
            memcpy(task->range_header, range_header->buf, range_header->len);
            task->range_header[range_header->len] = '\0';
        }
    }

    return task;
}

/**
 * @brief Free a playback recording task
 *
 * @param task Task to free
 * @param free_http_message Whether to free the HTTP message (ignored - HTTP message is never freed)
 */
void playback_recording_task_free(playback_recording_task_t *task, bool free_http_message) {
    if (task) {
        if (task->range_header) {
            free(task->range_header);
            task->range_header = NULL; // Prevent use-after-free
        }

        // IMPORTANT: We never free the HTTP message (hm) because it's allocated on the stack
        // by Mongoose and not dynamically allocated. Attempting to free it causes memory errors.
        // The free_http_message parameter is kept for backward compatibility.

        free(task);
    }
}
// Forward declarations for request tracking functions
static void init_active_requests(void);
static bool is_request_active(uint64_t id);
static bool mark_request_active(uint64_t id);
static void mark_request_inactive(uint64_t id);
/**
 * @brief Playback recording task function using Mongoose's built-in file serving
 *
 * @param arg Task argument (playback_recording_task_t*)
 */
void playback_recording_task_function(void *arg) {
    playback_recording_task_t *task = (playback_recording_task_t *)arg;
    if (!task) {
        log_error("Invalid playback recording task");
        return;
    }

    // Extract ID early for error handling
    uint64_t id = task->id;

    struct mg_connection *c = task->connection;
    if (!c) {
        log_error("Invalid Mongoose connection");
        mark_request_inactive(id);  // Mark request as inactive
        playback_recording_task_free(task, true);
        return;
    }

    // Check if connection is still valid
    if (c->is_closing) {
        log_error("Connection is closing, aborting playback task");
        mark_request_inactive(id);  // Mark request as inactive
        playback_recording_task_free(task, true);
        return;
    }

    // Initialize playback sessions if not already done
    init_playback_sessions();

    // Clean up inactive sessions
    cleanup_inactive_playback_sessions();

    log_info("Handling GET /api/recordings/play/%llu request", (unsigned long long)id);

    // Get recording from database
    recording_metadata_t recording;
    memset(&recording, 0, sizeof(recording_metadata_t));

    // Set proper headers for CORS and caching
    const char *headers = "Content-Type: application/json\r\n"
                         "Access-Control-Allow-Origin: *\r\n"
                         "Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\n"
                         "Access-Control-Allow-Headers: Content-Type, Authorization, X-Requested-With\r\n"
                         "Access-Control-Allow-Credentials: true\r\n"
                         "Cache-Control: no-cache, no-store, must-revalidate\r\n"
                         "Pragma: no-cache\r\n"
                         "Expires: 0\r\n";

    if (get_recording_metadata_by_id(id, &recording) != 0) {
        log_error("Recording not found: %llu", (unsigned long long)id);
        mg_http_reply(c, 404, headers, "{\"error\":\"Recording not found\"}");
        mark_request_inactive(id);  // Mark request as inactive
        playback_recording_task_free(task, true);
        return;
    }

    // Validate file path
    if (recording.file_path[0] == '\0') {
        log_error("Recording has empty file path: %llu", (unsigned long long)id);
        mg_http_reply(c, 500, headers, "{\"error\":\"Recording has invalid file path\"}");
        mark_request_inactive(id);  // Mark request as inactive
        playback_recording_task_free(task, true);
        return;
    }

    // Check if file exists
    struct stat st;
    if (stat(recording.file_path, &st) != 0) {
        log_error("Recording file not found: %s (error: %s)", recording.file_path, strerror(errno));
        mg_http_reply(c, 404, headers, "{\"error\":\"Recording file not found\"}");
        mark_request_inactive(id);  // Mark request as inactive
        playback_recording_task_free(task, true);
        return;
    }

    log_info("Using Mongoose file serving for file: %s (%ld bytes)", recording.file_path, st.st_size);

    // Determine content type based on file extension
    const char *content_type = "video/mp4"; // Default content type
    const char *file_ext = strrchr(recording.file_path, '.');
    if (file_ext) {
        if (strcasecmp(file_ext, ".mp4") == 0) {
            content_type = "video/mp4";
        } else if (strcasecmp(file_ext, ".webm") == 0) {
            content_type = "video/webm";
        } else if (strcasecmp(file_ext, ".mkv") == 0) {
            content_type = "video/x-matroska";
        } else if (strcasecmp(file_ext, ".avi") == 0) {
            content_type = "video/x-msvideo";
        } else if (strcasecmp(file_ext, ".mov") == 0) {
            content_type = "video/quicktime";
        }
    }

    log_info("Using content type: %s for file: %s", content_type, recording.file_path);

    // Create struct mg_http_serve_opts for file serving options
    struct mg_http_serve_opts opts = {0};
    opts.mime_types = content_type; // Override MIME type
    opts.extra_headers = "Accept-Ranges: bytes\r\n"
                         "Access-Control-Allow-Origin: *\r\n"
                         "Access-Control-Allow-Methods: GET, OPTIONS\r\n"
                         "Access-Control-Allow-Headers: Range, Origin, Content-Type, Accept\r\n"
                         "Cache-Control: max-age=3600\r\n";

    // Log if this is a range request
    if (task->range_header) {
        log_info("Range request: %s", task->range_header);
    }

    // Let Mongoose handle the file serving, including range requests
    log_info("Serving file using Mongoose's built-in file server");
    mg_http_serve_file(c, task->hm, recording.file_path, &opts);

    log_info("File serving initiated");

    // Mark the request as inactive
    mark_request_inactive(id);

    // Free task resources but DO NOT free the HTTP message
    // Mongoose's mg_http_serve_file needs the HTTP message to remain valid
    // until it completes serving the file asynchronously
    playback_recording_task_free(task, false);

    log_info("Successfully handled GET /api/recordings/play/%llu request", (unsigned long long)id);
}

// Global mutex to protect against duplicate processing
static pthread_mutex_t g_playback_mutex = PTHREAD_MUTEX_INITIALIZER;

// Hash table to track active requests (simple implementation)
#define MAX_ACTIVE_REQUESTS 32
static struct {
    uint64_t id;
    bool active;
} g_active_requests[MAX_ACTIVE_REQUESTS];

// Initialize the active requests table
static void init_active_requests(void) {
    static bool initialized = false;

    if (!initialized) {
        pthread_mutex_lock(&g_playback_mutex);

        if (!initialized) {
            memset(g_active_requests, 0, sizeof(g_active_requests));
            initialized = true;
        }

        pthread_mutex_unlock(&g_playback_mutex);
    }
}

// Check if a request is already being processed
static bool is_request_active(uint64_t id) {
    bool active = false;

    pthread_mutex_lock(&g_playback_mutex);

    for (int i = 0; i < MAX_ACTIVE_REQUESTS; i++) {
        if (g_active_requests[i].id == id && g_active_requests[i].active) {
            active = true;
            break;
        }
    }

    pthread_mutex_unlock(&g_playback_mutex);

    return active;
}

// Mark a request as active
static bool mark_request_active(uint64_t id) {
    bool success = false;

    pthread_mutex_lock(&g_playback_mutex);

    // First check if already active
    for (int i = 0; i < MAX_ACTIVE_REQUESTS; i++) {
        if (g_active_requests[i].id == id && g_active_requests[i].active) {
            // Already active
            pthread_mutex_unlock(&g_playback_mutex);
            return false;
        }
    }

    // Find a free slot
    for (int i = 0; i < MAX_ACTIVE_REQUESTS; i++) {
        if (!g_active_requests[i].active) {
            g_active_requests[i].id = id;
            g_active_requests[i].active = true;
            success = true;
            break;
        }
    }

    pthread_mutex_unlock(&g_playback_mutex);

    return success;
}

// Mark a request as inactive
static void mark_request_inactive(uint64_t id) {
    pthread_mutex_lock(&g_playback_mutex);

    for (int i = 0; i < MAX_ACTIVE_REQUESTS; i++) {
        if (g_active_requests[i].id == id && g_active_requests[i].active) {
            g_active_requests[i].active = false;
            break;
        }
    }

    pthread_mutex_unlock(&g_playback_mutex);
}

/**
 * @brief Handler function for playback recording
 *
 * This function is called by the multithreading system.
 *
 * @param c Mongoose connection
 * @param hm HTTP message
 */
void playback_recording_handler(struct mg_connection *c, struct mg_http_message *hm) {
    // Extract recording ID from URL
    char id_str[32];
    if (mg_extract_path_param(hm, "/api/recordings/play/", id_str, sizeof(id_str)) != 0) {
        log_error("Failed to extract recording ID from URL");
        mg_send_json_error(c, 400, "Invalid request path");
        return;
    }

    // Convert ID to integer
    uint64_t id = strtoull(id_str, NULL, 10);
    if (id == 0) {
        log_error("Invalid recording ID: %s", id_str);
        mg_send_json_error(c, 400, "Invalid recording ID");
        return;
    }

    // Check if this request is already being processed
    if (is_request_active(id)) {
        log_warn("Request for recording %llu already being processed, skipping duplicate",
                (unsigned long long)id);
        // Instead of just returning, send an error to the client
        mg_send_json_error(c, 429, "This recording is already being processed");
        return;
    }

    // Mark this request as active
    if (!mark_request_active(id)) {
        log_error("Failed to mark request as active, too many concurrent requests");
        mg_send_json_error(c, 503, "Too many concurrent requests");
        return;
    }

    log_info("Handling GET /api/recordings/play/%llu request in worker thread", (unsigned long long)id);

    // Create task
    playback_recording_task_t *task = playback_recording_task_create(c, id, hm);
    if (!task) {
        log_error("Failed to create playback recording task");
        mark_request_inactive(id);
        mg_send_json_error(c, 500, "Failed to create playback recording task");
        return;
    }

    // Call the task function directly
    playback_recording_task_function(task);
}

/**
 * @brief Direct handler for GET /api/recordings/play/:id
 */
void mg_handle_play_recording(struct mg_connection *c, struct mg_http_message *hm) {
    // Check authentication
    http_server_t *server = (http_server_t *)c->fn_data;
    if (server && server->config.auth_enabled) {
        // Check if the user is authenticated
        if (mongoose_server_basic_auth_check(hm, server) != 0) {
            log_error("Authentication failed for recording playback request");
            mg_send_json_error(c, 401, "Unauthorized");
            return;
        }
    }

    // Initialize active requests tracking
    init_active_requests();

    // Extract recording ID from URL
    char id_str[32];
    if (mg_extract_path_param(hm, "/api/recordings/play/", id_str, sizeof(id_str)) != 0) {
        log_error("Failed to extract recording ID from URL");
        mg_send_json_error(c, 400, "Invalid request path");
        return;
    }

    // Convert ID to integer
    uint64_t id = strtoull(id_str, NULL, 10);
    if (id == 0) {
        log_error("Invalid recording ID: %s", id_str);
        mg_send_json_error(c, 400, "Invalid recording ID");
        return;
    }

    log_info("Handling GET /api/recordings/play/%llu request", (unsigned long long)id);

    // Check if this request is already being processed
    if (is_request_active(id)) {
        log_warn("Request for recording %llu already being processed, skipping duplicate",
                (unsigned long long)id);
        // Instead of just returning, send an error to the client
        mg_send_json_error(c, 429, "This recording is already being processed");
        return;
    }

    // Mark this request as active
    if (!mark_request_active(id)) {
        log_error("Failed to mark request as active, too many concurrent requests");
        mg_send_json_error(c, 503, "Too many concurrent requests");
        return;
    }

    // Create task directly
    playback_recording_task_t *task = playback_recording_task_create(c, id, hm);
    if (!task) {
        log_error("Failed to create playback recording task");
        mark_request_inactive(id);
        mg_send_json_error(c, 500, "Failed to create playback recording task");
        return;
    }

    // Call the task function directly without creating a worker thread
    playback_recording_task_function(task);

    log_info("Playback recording task completed");
}
