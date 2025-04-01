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

#include "web/recordings_download_task.h"
#include "web/mongoose_adapter.h"
#include "web/mongoose_server_auth.h"
#include "web/http_server.h"
#include "web/api_handlers.h"
#include "core/logger.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"

/**
 * @brief Create a download recording task
 * 
 * @param c Mongoose connection
 * @param id Recording ID
 * @return download_recording_task_t* Pointer to the task or NULL on error
 */
download_recording_task_t *download_recording_task_create(struct mg_connection *c, uint64_t id) {
    download_recording_task_t *task = calloc(1, sizeof(download_recording_task_t));
    if (!task) {
        log_error("Failed to allocate memory for download recording task");
        return NULL;
    }
    
    task->connection = c;
    task->id = id;
    
    return task;
}

/**
 * @brief Free a download recording task
 * 
 * @param task Task to free
 */
void download_recording_task_free(download_recording_task_t *task) {
    if (task) {
        free(task);
        task = NULL; // Prevent use-after-free
    }
}

/**
 * @brief Direct handler for GET /api/recordings/download/:id
 */
void mg_handle_download_recording(struct mg_connection *c, struct mg_http_message *hm) {
    // Check authentication
    http_server_t *server = (http_server_t *)c->fn_data;
    if (server && server->config.auth_enabled) {
        // Check if the user is authenticated
        if (mongoose_server_basic_auth_check(hm, server) != 0) {
            log_error("Authentication failed for recording download request");
            mg_send_json_error(c, 401, "Unauthorized");
            return;
        }
    }
    
    // Extract recording ID from URL
    char id_str[32];
    if (mg_extract_path_param(hm, "/api/recordings/download/", id_str, sizeof(id_str)) != 0) {
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
    
    log_info("Handling GET /api/recordings/download/%llu request", (unsigned long long)id);
    
    // Get recording from database
    recording_metadata_t recording = {0};
    if (get_recording_metadata_by_id(id, &recording) != 0) {
        log_error("Recording not found: %llu", (unsigned long long)id);
        mg_send_json_error(c, 404, "Recording not found");
        return;
    }
    
    // Check if file exists
    struct stat st;
    if (stat(recording.file_path, &st) != 0) {
        log_error("Recording file not found: %s", recording.file_path);
        mg_send_json_error(c, 404, "Recording file not found");
        return;
    }
    
    // Extract filename from path
    const char *filename = strrchr(recording.file_path, '/');
    if (filename) {
        filename++; // Skip the slash
    } else {
        filename = recording.file_path;
    }
    
    // Check file extension to determine content type
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
    
    log_info("Using content type: %s for file: %s (download)", content_type, recording.file_path);
    
    // Set custom headers for the file download
    char headers[512];
    snprintf(headers, sizeof(headers),
             "Content-Type: %s\r\n"
             "Content-Disposition: attachment; filename=\"%s\"\r\n",
             content_type, filename);
    
    // Use Mongoose's built-in file serving capability
    struct mg_http_serve_opts opts = {
        .mime_types = "",  // We're setting Content-Type explicitly in extra_headers
        .extra_headers = headers
    };
    
    log_debug("Serving file directly using mg_http_serve_file: %s", recording.file_path);
    mg_http_serve_file(c, hm, recording.file_path, &opts);
    
    log_info("Successfully handled GET /api/recordings/download/%llu request", (unsigned long long)id);
}
