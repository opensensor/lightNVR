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
#include "web/api_thread_pool.h"
#include "web/mongoose_adapter.h"
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
    }
}

/**
 * @brief Download recording task function
 * 
 * @param arg Task argument (download_recording_task_t*)
 */
void download_recording_task_function(void *arg) {
    download_recording_task_t *task = (download_recording_task_t *)arg;
    if (!task) {
        log_error("Invalid download recording task");
        return;
    }
    
    struct mg_connection *c = task->connection;
    if (!c) {
        log_error("Invalid Mongoose connection");
        download_recording_task_free(task);
        return;
    }
    
    // Release the thread pool when this task is done
    bool release_needed = true;
    
    uint64_t id = task->id;
    
    log_info("Handling GET /api/recordings/download/%llu request", (unsigned long long)id);
    
    // Get recording from database
    recording_metadata_t recording;
    if (get_recording_metadata_by_id(id, &recording) != 0) {
        log_error("Recording not found: %llu", (unsigned long long)id);
        mg_send_json_error(c, 404, "Recording not found");
        download_recording_task_free(task);
        if (release_needed) {
            api_thread_pool_release();
        }
        return;
    }
    
    // Check if file exists
    struct stat st;
    if (stat(recording.file_path, &st) != 0) {
        log_error("Recording file not found: %s", recording.file_path);
        mg_send_json_error(c, 404, "Recording file not found");
        download_recording_task_free(task);
        if (release_needed) {
            api_thread_pool_release();
        }
        return;
    }
    
    // Open file
    FILE *file = fopen(recording.file_path, "rb");
    if (!file) {
        log_error("Failed to open recording file: %s", recording.file_path);
        mg_send_json_error(c, 500, "Failed to open recording file");
        download_recording_task_free(task);
        if (release_needed) {
            api_thread_pool_release();
        }
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
    
    // Set headers
    char headers[512];
    snprintf(headers, sizeof(headers),
             "Content-Type: %s\r\n"
             "Content-Disposition: attachment; filename=\"%s\"\r\n"
             "Content-Length: %ld\r\n",
             content_type, filename, st.st_size);
    
    // Send headers
    mg_printf(c, "HTTP/1.1 200 OK\r\n%s\r\n", headers);
    
    // Send file content
    // Allocate buffer on heap instead of stack for embedded devices
    char *buffer = malloc(8192); // Smaller buffer size for embedded devices
    if (!buffer) {
        log_error("Failed to allocate memory for file buffer");
        if (fclose(file) != 0) {
            log_error("Error closing file: %s", strerror(errno));
        }
        file = NULL;
        download_recording_task_free(task);
        if (release_needed) {
            api_thread_pool_release();
        }
        return;
    }
    
    size_t bytes_read;
    
    // Add additional safety check for connection
    if (!c || c->is_closing) {
        log_error("Connection is invalid or closing, aborting download request");
        if (fclose(file) != 0) {
            log_error("Error closing file: %s", strerror(errno));
        }
        file = NULL;
        free(buffer); // Free the buffer
        download_recording_task_free(task);
        if (release_needed) {
            api_thread_pool_release();
        }
        return;
    }
    
    log_debug("Starting to send file data in chunks (download)");
    
    while (1) {
        // Check if connection is still valid
        if (!c || c->is_closing) {
            log_error("Connection closed during download request");
            break;
        }
        
        // Additional safety check for buffer
        if (!buffer) {
            log_error("Buffer became invalid during download");
            break;
        }
        
        // Additional safety check for file
        if (!file) {
            log_error("File pointer became invalid during download");
            break;
        }
        
        bytes_read = fread(buffer, 1, 8192, file);
        
        if (bytes_read <= 0) {
            if (feof(file)) {
                log_info("End of file reached for download serving");
                break;
            } else if (ferror(file)) {
                log_error("Error reading file during download serving: %s", strerror(errno));
                break;
            }
        }
        
        // Send the chunk to the client
        if (bytes_read > 0) {
            // Additional safety check before sending
            if (!c || c->is_closing) {
                log_error("Connection became invalid before sending chunk");
                break;
            }
            mg_send(c, buffer, bytes_read);
        }
    }
    
    // Free the buffer
    free(buffer);
    buffer = NULL;
    
    log_debug("Finished sending file data (download)");
    
    // Close file
    if (fclose(file) != 0) {
        log_error("Error closing file during download: %s", strerror(errno));
    }
    file = NULL; // Prevent use-after-free
    
    log_debug("Connection closed");
    
    download_recording_task_free(task);
    
    // Release the thread pool if needed
    if (release_needed) {
        api_thread_pool_release();
    }
    
    log_info("Successfully handled GET /api/recordings/download/%llu request", (unsigned long long)id);
}

/**
 * @brief Direct handler for GET /api/recordings/download/:id
 */
void mg_handle_download_recording(struct mg_connection *c, struct mg_http_message *hm) {
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
    
    // Acquire thread pool
    thread_pool_t *pool = api_thread_pool_acquire(4, 10);
    if (!pool) {
        log_error("Failed to acquire thread pool");
        mg_send_json_error(c, 500, "Failed to acquire thread pool");
        return;
    }
    
    // Create task
    download_recording_task_t *task = download_recording_task_create(c, id);
    if (!task) {
        log_error("Failed to create download recording task");
        api_thread_pool_release();
        mg_send_json_error(c, 500, "Failed to create download recording task");
        return;
    }
    
    // Add task to thread pool
    if (!thread_pool_add_task(pool, download_recording_task_function, task)) {
        log_error("Failed to add download recording task to thread pool");
        download_recording_task_free(task);
        api_thread_pool_release();
        mg_send_json_error(c, 500, "Failed to add download recording task to thread pool");
        return;
    }
    
    // Note: The task will release the thread pool when it's done
    log_info("Download recording task added to thread pool");
}
