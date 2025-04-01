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

#include "web/api_handlers.h"
#include "core/logger.h"
#include "mongoose.h"
#include "database/db_recordings.h"

// Use MAX_PATH_LENGTH from config.h

// External function declarations from split files
extern void mg_handle_get_recordings(struct mg_connection *c, struct mg_http_message *hm);
extern void mg_handle_get_recording(struct mg_connection *c, struct mg_http_message *hm);
extern void mg_handle_delete_recording(struct mg_connection *c, struct mg_http_message *hm);
extern void mg_handle_batch_delete_recordings(struct mg_connection *c, struct mg_http_message *hm);
extern void mg_handle_play_recording(struct mg_connection *c, struct mg_http_message *hm);
extern void mg_handle_download_recording(struct mg_connection *c, struct mg_http_message *hm);
extern void mg_handle_check_recording_file(struct mg_connection *c, struct mg_http_message *hm);
extern void mg_handle_delete_recording_file(struct mg_connection *c, struct mg_http_message *hm);

// List of files to delete after they've been served
typedef struct temp_file_node {
    char *file_path;
    struct temp_file_node *next;
} temp_file_node_t;

// Global list of temporary files to delete
static temp_file_node_t *temp_files_list = NULL;
static pthread_mutex_t temp_files_mutex = PTHREAD_MUTEX_INITIALIZER;

// These functions are now imported from db_recordings.h
// to avoid duplicate definitions

/**
 * Serve an MP4 file with proper headers for download
 */
void serve_mp4_file(struct mg_connection *c, const char *file_path, const char *filename) {
    // Set proper headers for CORS and caching
    const char *headers = "Content-Type: application/json\r\n"
                         "Access-Control-Allow-Origin: *\r\n"
                         "Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\n"
                         "Access-Control-Allow-Headers: Content-Type, Authorization, X-Requested-With\r\n"
                         "Access-Control-Allow-Credentials: true\r\n"
                         "Cache-Control: no-cache, no-store, must-revalidate\r\n"
                         "Pragma: no-cache\r\n"
                         "Expires: 0\r\n";
                         
    // Check if file exists
    struct stat st;
    if (stat(file_path, &st) != 0) {
        mg_http_reply(c, 404, headers, "{\"error\": \"File not found\"}");
        return;
    }
    
    // Set headers for streaming playback
    mg_printf(c, "HTTP/1.1 200 OK\r\n");
    mg_printf(c, "Content-Type: video/mp4\r\n");
    mg_printf(c, "Content-Length: %ld\r\n", st.st_size);
    mg_printf(c, "Content-Disposition: attachment; filename=\"%s\"\r\n", filename);
    mg_printf(c, "Accept-Ranges: bytes\r\n");
    mg_printf(c, "Access-Control-Allow-Origin: *\r\n");
    mg_printf(c, "Cache-Control: max-age=3600\r\n");
    mg_printf(c, "\r\n");
    
    // Use mg_http_serve_file for efficient file serving
    struct mg_http_serve_opts opts = {
        .mime_types = "mp4=video/mp4",
        .extra_headers = "Content-Disposition: attachment; filename=\"%s\"\r\n"
    };
    
    mg_http_serve_file(c, NULL, file_path, &opts);
}

/**
 * Serve a file for download with proper headers to force browser download
 */
void serve_file_for_download(struct mg_connection *c, const char *file_path, const char *filename, off_t file_size) {
    // Set proper headers for CORS and caching
    const char *headers = "Content-Type: application/json\r\n"
                         "Access-Control-Allow-Origin: *\r\n"
                         "Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\n"
                         "Access-Control-Allow-Headers: Content-Type, Authorization, X-Requested-With\r\n"
                         "Access-Control-Allow-Credentials: true\r\n"
                         "Cache-Control: no-cache, no-store, must-revalidate\r\n"
                         "Pragma: no-cache\r\n"
                         "Expires: 0\r\n";
                         
    // Check if file exists
    struct stat st;
    if (stat(file_path, &st) != 0) {
        mg_http_reply(c, 404, headers, "{\"error\": \"File not found\"}");
        return;
    }
    
    // Set headers for download
    mg_printf(c, "HTTP/1.1 200 OK\r\n");
    mg_printf(c, "Content-Type: application/octet-stream\r\n");
    mg_printf(c, "Content-Length: %ld\r\n", st.st_size);
    mg_printf(c, "Content-Disposition: attachment; filename=\"%s\"\r\n", filename);
    mg_printf(c, "Accept-Ranges: bytes\r\n");
    mg_printf(c, "Access-Control-Allow-Origin: *\r\n");
    mg_printf(c, "Cache-Control: max-age=3600\r\n");
    mg_printf(c, "\r\n");
    
    // Use mg_http_serve_file for efficient file serving
    struct mg_http_serve_opts opts = {
        .mime_types = "mp4=application/octet-stream",
        .extra_headers = "Content-Disposition: attachment; filename=\"%s\"\r\n"
    };
    
    mg_http_serve_file(c, NULL, file_path, &opts);
}

/**
 * Serve the direct file download
 */
void serve_direct_download(struct mg_connection *c, uint64_t id, recording_metadata_t *metadata) {
    // Set proper headers for CORS and caching
    const char *headers = "Content-Type: application/json\r\n"
                         "Access-Control-Allow-Origin: *\r\n"
                         "Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\n"
                         "Access-Control-Allow-Headers: Content-Type, Authorization, X-Requested-With\r\n"
                         "Access-Control-Allow-Credentials: true\r\n"
                         "Cache-Control: no-cache, no-store, must-revalidate\r\n"
                         "Pragma: no-cache\r\n"
                         "Expires: 0\r\n";
                         
    // Check if file exists
    struct stat st;
    if (stat(metadata->file_path, &st) != 0) {
        mg_http_reply(c, 404, headers, "{\"error\": \"Recording file not found\"}");
        return;
    }
    
    // Extract filename from path
    const char *filename = strrchr(metadata->file_path, '/');
    if (filename) {
        filename++; // Skip the slash
    } else {
        filename = metadata->file_path;
    }
    
    // Serve file for download
    serve_file_for_download(c, metadata->file_path, filename, st.st_size);
}

/**
 * Serve a file for download with proper headers
 */
void serve_download_file(struct mg_connection *c, const char *file_path, const char *content_type,
                       const char *stream_name, time_t timestamp) {
    // Set proper headers for CORS and caching
    const char *headers = "Content-Type: application/json\r\n"
                         "Access-Control-Allow-Origin: *\r\n"
                         "Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\n"
                         "Access-Control-Allow-Headers: Content-Type, Authorization, X-Requested-With\r\n"
                         "Access-Control-Allow-Credentials: true\r\n"
                         "Cache-Control: no-cache, no-store, must-revalidate\r\n"
                         "Pragma: no-cache\r\n"
                         "Expires: 0\r\n";
                         
    // Check if file exists
    struct stat st;
    if (stat(file_path, &st) != 0) {
        mg_http_reply(c, 404, headers, "{\"error\": \"File not found\"}");
        return;
    }
    
    // Format timestamp for filename in UTC
    char timestamp_str[32] = {0};
    struct tm *tm_info = gmtime(&timestamp);
    if (tm_info) {
        strftime(timestamp_str, sizeof(timestamp_str), "%Y%m%d_%H%M%S", tm_info);
    }
    
    // Create filename
    char filename[256];
    if (stream_name && timestamp_str[0] != '\0') {
        snprintf(filename, sizeof(filename), "%s_%s.mp4", stream_name, timestamp_str);
    } else {
        // Extract filename from path
        const char *path_filename = strrchr(file_path, '/');
        if (path_filename) {
            path_filename++; // Skip the slash
            strncpy(filename, path_filename, sizeof(filename) - 1);
            filename[sizeof(filename) - 1] = '\0';
        } else {
            strncpy(filename, file_path, sizeof(filename) - 1);
            filename[sizeof(filename) - 1] = '\0';
        }
    }
    
    // Set headers for download
    mg_printf(c, "HTTP/1.1 200 OK\r\n");
    mg_printf(c, "Content-Type: %s\r\n", content_type);
    mg_printf(c, "Content-Length: %ld\r\n", st.st_size);
    mg_printf(c, "Content-Disposition: attachment; filename=\"%s\"\r\n", filename);
    mg_printf(c, "Accept-Ranges: bytes\r\n");
    mg_printf(c, "Access-Control-Allow-Origin: *\r\n");
    mg_printf(c, "Cache-Control: max-age=3600\r\n");
    mg_printf(c, "\r\n");
    
    // Use mg_http_serve_file for efficient file serving
    struct mg_http_serve_opts opts = {
        .mime_types = "mp4=video/mp4",
        .extra_headers = "Content-Disposition: attachment; filename=\"%s\"\r\n"
    };
    
    mg_http_serve_file(c, NULL, file_path, &opts);
}

/**
 * Schedule a file for deletion after it has been served
 */
void schedule_file_deletion(const char *file_path) {
    if (!file_path) {
        return;
    }
    
    // Allocate memory for file path
    char *path_copy = strdup(file_path);
    if (!path_copy) {
        log_error("Failed to allocate memory for file path");
        return;
    }
    
    // Create new node
    temp_file_node_t *node = malloc(sizeof(temp_file_node_t));
    if (!node) {
        log_error("Failed to allocate memory for temp file node");
        free(path_copy);
        return;
    }
    
    node->file_path = path_copy;
    
    // Add to list
    pthread_mutex_lock(&temp_files_mutex);
    
    node->next = temp_files_list;
    temp_files_list = node;
    
    pthread_mutex_unlock(&temp_files_mutex);
    
    log_info("Scheduled file for deletion: %s", file_path);
}

/**
 * Callback to remove temporary files after they've been sent
 */
void remove_temp_file_callback(void *data) {
    // Process all files in the list
    pthread_mutex_lock(&temp_files_mutex);
    
    temp_file_node_t *node = temp_files_list;
    temp_file_node_t *prev = NULL;
    
    while (node) {
        // Try to delete the file
        if (unlink(node->file_path) != 0) {
            log_warn("Failed to delete temporary file: %s", node->file_path);
            
            // Keep in list and try again later
            prev = node;
            node = node->next;
        } else {
            log_info("Deleted temporary file: %s", node->file_path);
            
            // Remove from list
            temp_file_node_t *to_delete = node;
            
            if (prev) {
                prev->next = node->next;
                node = node->next;
            } else {
                temp_files_list = node->next;
                node = node->next;
            }
            
            // Free memory
            free(to_delete->file_path);
            free(to_delete);
        }
    }
    
    pthread_mutex_unlock(&temp_files_mutex);
}
