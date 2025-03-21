/*
 * Main streams interface file that coordinates between different modules
 */
#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <ctype.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <fcntl.h>
#include <signal.h>
#include <errno.h>
#include <time.h>
#include <dirent.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/time.h>

#include "cJSON.h"
#include "web/web_server.h"
#include "core/logger.h"
#include "core/config.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/hls_writer.h"
#include "video/mp4_writer.h"
#include "database/database_manager.h"

#define LIGHTNVR_VERSION_STRING "0.2.0"

// Define a local config variable to work with
static config_t local_config;

// Global configuration - to be accessed from other modules if needed
config_t global_config;

/**
 * Get current global configuration
 */
config_t* get_streaming_config(void) {
    // For now, just use our global config
    return &global_config;
}

/**
 * Serve a video file with support for HTTP range requests (essential for video streaming)
 */
void serve_video_file(http_response_t *response, const char *file_path, const char *content_type,
                     const char *filename, const http_request_t *request) {
    // Verify file exists and is readable
    struct stat st;
    if (stat(file_path, &st) != 0) {
        log_error("Video file not found: %s (error: %s)", file_path, strerror(errno));
        
        // Create error response using cJSON
        cJSON *error = cJSON_CreateObject();
        if (!error) {
            log_error("Failed to create error JSON object");
            return;
        }
        
        cJSON_AddStringToObject(error, "error", "Video file not found");
        
        // Convert to string
        char *json_str = cJSON_PrintUnformatted(error);
        if (!json_str) {
            log_error("Failed to convert error JSON to string");
            cJSON_Delete(error);
            return;
        }
        
        // Create response
        create_json_response(response, 404, json_str);
        
        // Clean up
        free(json_str);
        cJSON_Delete(error);
        return;
    }

    if (access(file_path, R_OK) != 0) {
        log_error("Video file not readable: %s (error: %s)", file_path, strerror(errno));
        
        // Create error response using cJSON
        cJSON *error = cJSON_CreateObject();
        if (!error) {
            log_error("Failed to create error JSON object");
            return;
        }
        
        cJSON_AddStringToObject(error, "error", "Video file not accessible");
        
        // Convert to string
        char *json_str = cJSON_PrintUnformatted(error);
        if (!json_str) {
            log_error("Failed to convert error JSON to string");
            cJSON_Delete(error);
            return;
        }
        
        // Create response
        create_json_response(response, 403, json_str);
        
        // Clean up
        free(json_str);
        cJSON_Delete(error);
        return;
    }

    // Check file size
    if (st.st_size == 0) {
        log_error("Video file is empty: %s", file_path);
        
        // Create error response using cJSON
        cJSON *error = cJSON_CreateObject();
        if (!error) {
            log_error("Failed to create error JSON object");
            return;
        }
        
        cJSON_AddStringToObject(error, "error", "Video file is empty");
        
        // Convert to string
        char *json_str = cJSON_PrintUnformatted(error);
        if (!json_str) {
            log_error("Failed to convert error JSON to string");
            cJSON_Delete(error);
            return;
        }
        
        // Create response
        create_json_response(response, 500, json_str);
        
        // Clean up
        free(json_str);
        cJSON_Delete(error);
        return;
    }

    log_info("Serving video file: %s, size: %lld bytes", file_path, (long long)st.st_size);

    // Open file
    int fd = open(file_path, O_RDONLY);
    if (fd < 0) {
        log_error("Failed to open video file: %s (error: %s)", file_path, strerror(errno));
        
        // Create error response using cJSON
        cJSON *error = cJSON_CreateObject();
        if (!error) {
            log_error("Failed to create error JSON object");
            return;
        }
        
        cJSON_AddStringToObject(error, "error", "Failed to open video file");
        
        // Convert to string
        char *json_str = cJSON_PrintUnformatted(error);
        if (!json_str) {
            log_error("Failed to convert error JSON to string");
            cJSON_Delete(error);
            return;
        }
        
        // Create response
        create_json_response(response, 500, json_str);
        
        // Clean up
        free(json_str);
        cJSON_Delete(error);
        return;
    }

    // Parse Range header if present
    const char *range_header = get_request_header(request, "Range");
    off_t start_pos = 0;
    off_t end_pos = st.st_size - 1;
    bool is_range_request = false;

    if (range_header && strncmp(range_header, "bytes=", 6) == 0) {
        is_range_request = true;
        // Parse range header - format is typically "bytes=start-end"
        char range_value[256];
        strncpy(range_value, range_header + 6, sizeof(range_value) - 1);
        range_value[sizeof(range_value) - 1] = '\0';

        char *dash = strchr(range_value, '-');
        if (dash) {
            *dash = '\0'; // Split the string at dash

            // Parse start position
            if (range_value[0] != '\0') {
                start_pos = atoll(range_value);
                // Ensure start_pos is within file bounds
                if (start_pos >= st.st_size) {
                    start_pos = 0;
                }
            }

            // Parse end position if provided
            if (dash[1] != '\0') {
                end_pos = atoll(dash + 1);
                // Ensure end_pos is within file bounds
                if (end_pos >= st.st_size) {
                    end_pos = st.st_size - 1;
                }
            }

            // Sanity check
            if (start_pos > end_pos) {
                start_pos = 0;
                end_pos = st.st_size - 1;
                is_range_request = false;
            }
        }
    }

    // Calculate content length
    size_t content_length = end_pos - start_pos + 1;

    // Set content type header
    strncpy(response->content_type, content_type, sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';

    // Set content length header
    char content_length_str[32];
    snprintf(content_length_str, sizeof(content_length_str), "%zu", content_length);
    set_response_header(response, "Content-Length", content_length_str);

    // Set Accept-Ranges header to indicate range support
    set_response_header(response, "Accept-Ranges", "bytes");

    // Set disposition header for download if requested
    if (filename) {
        char disposition[256];
        snprintf(disposition, sizeof(disposition), "attachment; filename=\"%s\"", filename);
        set_response_header(response, "Content-Disposition", disposition);
    }

    // For range requests, set status code and Content-Range header
    if (is_range_request) {
        response->status_code = 206; // Partial Content

        char content_range[64];
        snprintf(content_range, sizeof(content_range), "bytes %lld-%lld/%lld",
                 (long long)start_pos, (long long)end_pos, (long long)st.st_size);
        set_response_header(response, "Content-Range", content_range);

        log_info("Serving range request: %s", content_range);
    } else {
        response->status_code = 200; // OK
    }

    // Seek to start position
    if (lseek(fd, start_pos, SEEK_SET) == -1) {
        log_error("Failed to seek in file: %s (error: %s)", file_path, strerror(errno));
        close(fd);
        
        // Create error response using cJSON
        cJSON *error = cJSON_CreateObject();
        if (!error) {
            log_error("Failed to create error JSON object");
            return;
        }
        
        cJSON_AddStringToObject(error, "error", "Failed to read from video file");
        
        // Convert to string
        char *json_str = cJSON_PrintUnformatted(error);
        if (!json_str) {
            log_error("Failed to convert error JSON to string");
            cJSON_Delete(error);
            return;
        }
        
        // Create response
        create_json_response(response, 500, json_str);
        
        // Clean up
        free(json_str);
        cJSON_Delete(error);
        return;
    }

    // Allocate response body to exact content length
    response->body = malloc(content_length);
    if (!response->body) {
        log_error("Failed to allocate response body of size %zu bytes", content_length);
        close(fd);
        
        // Create error response using cJSON
        cJSON *error = cJSON_CreateObject();
        if (!error) {
            log_error("Failed to create error JSON object");
            return;
        }
        
        cJSON_AddStringToObject(error, "error", "Server memory allocation failed");
        
        // Convert to string
        char *json_str = cJSON_PrintUnformatted(error);
        if (!json_str) {
            log_error("Failed to convert error JSON to string");
            cJSON_Delete(error);
            return;
        }
        
        // Create response
        create_json_response(response, 500, json_str);
        
        // Clean up
        free(json_str);
        cJSON_Delete(error);
        return;
    }

    // Read the file content
    ssize_t bytes_read = read(fd, response->body, content_length);
    close(fd);

    if (bytes_read != (ssize_t)content_length) {
        log_error("Failed to read complete file: %s (read %zd of %zu bytes)",
                file_path, bytes_read, content_length);
        free(response->body);
        
        // Create error response using cJSON
        cJSON *error = cJSON_CreateObject();
        if (!error) {
            log_error("Failed to create error JSON object");
            return;
        }
        
        cJSON_AddStringToObject(error, "error", "Failed to read complete video file");
        
        // Convert to string
        char *json_str = cJSON_PrintUnformatted(error);
        if (!json_str) {
            log_error("Failed to convert error JSON to string");
            cJSON_Delete(error);
            return;
        }
        
        // Create response
        create_json_response(response, 500, json_str);
        
        // Clean up
        free(json_str);
        cJSON_Delete(error);
        return;
    }

    // Set response body length
    response->body_length = content_length;

    log_info("Successfully prepared video file for response: %s (%zu bytes)",
            file_path, content_length);
}

// These functions have been moved to src/web/api_handlers_common.c and
// src/web/api_handlers_streaming.c to avoid duplicate definitions

/**
 * Update to stop_transcode_stream to handle decoupled MP4 recording
 */
int stop_transcode_stream(const char *stream_name) {
    if (!stream_name) {
        return -1;
    }

    // First stop the HLS stream
    int result = stop_hls_stream(stream_name);
    if (result != 0) {
        log_warn("Failed to stop HLS stream: %s", stream_name);
        // Continue anyway
    }

    // Also stop any separate MP4 recording for this stream
    unregister_mp4_writer_for_stream(stream_name);

    return result;
}
