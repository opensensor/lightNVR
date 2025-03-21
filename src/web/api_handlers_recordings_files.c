#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <ctype.h>
#include <time.h>
#include <sys/stat.h>
#include <errno.h>
#include <sys/types.h>
#include <fcntl.h>
#include <glob.h>
#include <dirent.h>

#include "web/api_handlers_recordings.h"
#include "web/api_handlers_common.h"
#include "core/config.h"
#include "core/logger.h"
#include "database/database_manager.h"
#include "storage/storage_manager.h"
#include "web/request_response.h"
#include "cJSON.h"

/**
 * @brief Create a JSON error response for recordings files
 * 
 * @param response HTTP response
 * @param status_code HTTP status code
 * @param error_message Error message
 */
static void create_recordings_error_response(http_response_t *response, int status_code, const char *error_message) {
    if (!response || !error_message) {
        return;
    }
    
    // Create JSON object using cJSON
    cJSON *error = cJSON_CreateObject();
    if (!error) {
        log_error("Failed to create error JSON object");
        return;
    }
    
    // Add error message
    cJSON_AddStringToObject(error, "error", error_message);
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(error);
    if (!json_str) {
        log_error("Failed to convert error JSON to string");
        cJSON_Delete(error);
        return;
    }
    
    // Create response
    create_json_response(response, status_code, json_str);
    
    // Clean up
    free(json_str);
    cJSON_Delete(error);
}

/**
 * Serve an MP4 file with proper headers for download
 */
void serve_mp4_file(http_response_t *response, const char *file_path, const char *filename) {
    // Verify file exists and is readable
    struct stat st;
    if (stat(file_path, &st) != 0 || access(file_path, R_OK) != 0) {
        log_error("MP4 file not accessible: %s (error: %s)", file_path, strerror(errno));
        create_recordings_error_response(response, 404, "Recording file not found");
        return;
    }

    // Check file size
    if (st.st_size == 0) {
        log_error("MP4 file is empty: %s", file_path);
        create_recordings_error_response(response, 500, "Recording file is empty");
        return;
    }

    log_info("Serving MP4 file: %s, size: %lld bytes", file_path, (long long)st.st_size);

    // Set content type header
    set_response_header(response, "Content-Type", "video/mp4");

    // Set content length header
    char content_length[32];
    snprintf(content_length, sizeof(content_length), "%lld", (long long)st.st_size);
    set_response_header(response, "Content-Length", content_length);

    // Set disposition header for download
    char disposition[256];
    snprintf(disposition, sizeof(disposition), "attachment; filename=\"%s\"", filename);
    set_response_header(response, "Content-Disposition", disposition);

    // Set status code
    response->status_code = 200;

    // Open file for reading
    int fd = open(file_path, O_RDONLY);
    if (fd < 0) {
        log_error("Failed to open MP4 file: %s (error: %s)", file_path, strerror(errno));
        create_recordings_error_response(response, 500, "Failed to read recording file");
        return;
    }

    // Allocate response body
    response->body = malloc(st.st_size);
    if (!response->body) {
        log_error("Failed to allocate memory for response body");
        close(fd);
        create_recordings_error_response(response, 500, "{\"error\": \"Server memory allocation failed\"}");
        return;
    }

    // Read file content into response body
    ssize_t bytes_read = read(fd, response->body, st.st_size);
    close(fd);

    if (bytes_read != st.st_size) {
        log_error("Failed to read complete file: %s (read %zd of %lld bytes)",
                file_path, bytes_read, (long long)st.st_size);
        free(response->body);
        create_recordings_error_response(response, 500, "{\"error\": \"Failed to read complete recording file\"}");
        return;
    }

    // Set response body length
    response->body_length = st.st_size;

    log_info("Successfully read MP4 file into response: %s (%lld bytes)",
            file_path, (long long)st.st_size);
}

/**
 * Serve a file for download with proper headers to force browser download
 */
void serve_file_for_download(http_response_t *response, const char *file_path, const char *filename, off_t file_size) {
    // Open the file for reading
    int fd = open(file_path, O_RDONLY);
    if (fd < 0) {
        log_error("Failed to open file for download: %s (error: %s)",
                file_path, strerror(errno));
        create_recordings_error_response(response, 500, "{\"error\": \"Failed to read file\"}");
        return;
    }

    // Set response status
    response->status_code = 200;

    // Set headers to force download
    set_response_header(response, "Content-Type", "application/octet-stream");

    // Set content length
    char content_length[32];
    snprintf(content_length, sizeof(content_length), "%lld", (long long)file_size);
    set_response_header(response, "Content-Length", content_length);

    // Force download with Content-Disposition
    char disposition[256];
    snprintf(disposition, sizeof(disposition), "attachment; filename=\"%s\"", filename);
    set_response_header(response, "Content-Disposition", disposition);

    // Prevent caching
    set_response_header(response, "Cache-Control", "no-cache, no-store, must-revalidate");
    set_response_header(response, "Pragma", "no-cache");
    set_response_header(response, "Expires", "0");

    log_info("Serving file for download: %s, size: %lld bytes", file_path, (long long)file_size);

    // Allocate memory for the file content
    response->body = malloc(file_size);
    if (!response->body) {
        log_error("Failed to allocate memory for file: %s (size: %lld bytes)",
                file_path, (long long)file_size);
        close(fd);
        create_recordings_error_response(response, 500, "{\"error\": \"Server memory allocation failed\"}");
        return;
    }

    // Read the file content
    ssize_t bytes_read = read(fd, response->body, file_size);
    close(fd);

    if (bytes_read != file_size) {
        log_error("Failed to read complete file: %s (read %zd of %lld bytes)",
                file_path, bytes_read, (long long)file_size);
        free(response->body);
        create_recordings_error_response(response, 500, "{\"error\": \"Failed to read complete file\"}");
        return;
    }

    // Set response body length
    response->body_length = file_size;

    log_info("File prepared for download: %s (%lld bytes)", file_path, (long long)file_size);
}

/**
 * Callback to remove temporary files after they've been sent
 */
void remove_temp_file_callback(void *data) {
    if (!data) return;

    char *temp_file_path = (char *)data;
    log_info("Removing temporary file: %s", temp_file_path);

    // Sync to ensure data is written to disk
    sync();

    if (unlink(temp_file_path) != 0) {
        log_warn("Failed to remove temporary file: %s (error: %s)",
               temp_file_path, strerror(errno));
    } else {
        log_info("Successfully removed temporary file: %s", temp_file_path);
    }

    free(temp_file_path);
}

/**
 * Schedule a file for deletion after it has been served
 */
void schedule_file_deletion(const char *file_path) {
    if (!file_path) {
        log_error("Invalid file path for scheduled deletion");
        return;
    }
    
    // Create a copy of the file path since we'll need it later
    char *path_copy = strdup(file_path);
    if (!path_copy) {
        log_error("Failed to allocate memory for file path");
        return;
    }
    
    // Register the file for cleanup using the callback mechanism
    remove_temp_file_callback(path_copy);
    
    log_info("Registered temporary file for deletion: %s", file_path);
}

/**
 * Serve a file for download with proper headers
 */
void serve_download_file(http_response_t *response, const char *file_path, const char *content_type,
                       const char *stream_name, time_t timestamp) {
    // Verify file exists and is readable
    struct stat st;
    if (stat(file_path, &st) != 0) {
        log_error("File not found: %s", file_path);
        create_recordings_error_response(response, 404, "{\"error\": \"Recording file not found\"}");
        return;
    }

    if (access(file_path, R_OK) != 0) {
        log_error("File not readable: %s", file_path);
        create_recordings_error_response(response, 403, "{\"error\": \"Recording file not readable\"}");
        return;
    }

    if (st.st_size == 0) {
        log_error("File is empty: %s", file_path);
        create_recordings_error_response(response, 500, "{\"error\": \"Recording file is empty\"}");
        return;
    }

    // Generate filename for download
    char filename[128];
    char *file_ext = strrchr(file_path, '.');
    if (!file_ext) {
        // Default to .mp4 if no extension
        file_ext = ".mp4";
    }

    snprintf(filename, sizeof(filename), "%s_%lld%s",
           stream_name, (long long)timestamp, file_ext);

    // Set response headers for download
    set_response_header(response, "Content-Type", "application/octet-stream");

    // Set Content-Disposition to force download
    char disposition[256];
    snprintf(disposition, sizeof(disposition), "attachment; filename=\"%s\"", filename);
    set_response_header(response, "Content-Disposition", disposition);

    // Set Cache-Control
    set_response_header(response, "Cache-Control", "no-cache, no-store, must-revalidate");
    set_response_header(response, "Pragma", "no-cache");
    set_response_header(response, "Expires", "0");

    // Log the download attempt
    log_info("Serving file for download: %s, size: %lld bytes, type: %s",
           file_path, (long long)st.st_size, content_type);

    // Serve the file - use your existing file serving function
    int result = create_file_response(response, 200, file_path, "application/octet-stream");
    if (result != 0) {
        log_error("Failed to serve file: %s", file_path);
        create_recordings_error_response(response, 500, "{\"error\": \"Failed to serve recording file\"}");
        return;
    }

    log_info("Download started: Path=%s, Filename=%s", file_path, filename);
}

/**
 * Serve the direct file download
 */
void serve_direct_download(http_response_t *response, uint64_t id, recording_metadata_t *metadata) {
    // Determine if this is an HLS stream (m3u8)
    const char *ext = strrchr(metadata->file_path, '.');
    bool is_hls = (ext && strcasecmp(ext, ".m3u8") == 0);

    if (is_hls) {
        // Check if a direct MP4 recording already exists in the same directory
        char dir_path[256];
        const char *last_slash = strrchr(metadata->file_path, '/');
        if (last_slash) {
            size_t dir_len = last_slash - metadata->file_path;
            strncpy(dir_path, metadata->file_path, dir_len);
            dir_path[dir_len] = '\0';
        } else {
            // If no slash, use the current directory
            strcpy(dir_path, ".");
        }

        // Check for an existing MP4 file
        char mp4_path[256];
        snprintf(mp4_path, sizeof(mp4_path), "%s/recording.mp4", dir_path);

        struct stat mp4_stat;
        if (stat(mp4_path, &mp4_stat) == 0 && mp4_stat.st_size > 0) {
            // Direct MP4 exists, serve it
            log_info("Found direct MP4 recording: %s (%lld bytes)",
                   mp4_path, (long long)mp4_stat.st_size);

            // Create filename for download
            char filename[128];
            snprintf(filename, sizeof(filename), "%s_%lld.mp4",
                   metadata->stream_name, (long long)metadata->start_time);

            // Set necessary headers
            set_response_header(response, "Content-Type", "application/octet-stream");
            char content_length[32];
            snprintf(content_length, sizeof(content_length), "%lld", (long long)mp4_stat.st_size);
            set_response_header(response, "Content-Length", content_length);
            char disposition[256];
            snprintf(disposition, sizeof(disposition), "attachment; filename=\"%s\"", filename);
            set_response_header(response, "Content-Disposition", disposition);

            log_info("Serving direct MP4 recording for download: %s", mp4_path);

            // Use existing file serving mechanism
            int result = create_file_response(response, 200, mp4_path, "application/octet-stream");
            if (result != 0) {
                log_error("Failed to create file response: %s", mp4_path);
                create_recordings_error_response(response, 500, "{\"error\": \"Failed to serve recording file\"}");
                return;
            }

            log_info("Direct MP4 recording download started: ID=%llu, Path=%s, Filename=%s",
                   (unsigned long long)id, mp4_path, filename);
            return;
        }

        // No direct MP4 found, create one
        char output_path[256];
        snprintf(output_path, sizeof(output_path), "%s/download_%llu.mp4",
                dir_path, (unsigned long long)id);

        log_info("Converting HLS stream to MP4: %s -> %s", metadata->file_path, output_path);

        // Create a more robust FFmpeg command
        char ffmpeg_cmd[512];
        snprintf(ffmpeg_cmd, sizeof(ffmpeg_cmd),
                "ffmpeg -y -i %s -c copy -bsf:a aac_adtstoasc -movflags +faststart %s 2>/dev/null",
                metadata->file_path, output_path);

        log_info("Running FFmpeg command: %s", ffmpeg_cmd);

        // Execute FFmpeg command
        int cmd_result = system(ffmpeg_cmd);
        if (cmd_result != 0) {
            log_error("FFmpeg command failed with status %d", cmd_result);

            // Try alternative approach with TS files directly
            log_info("Trying alternative conversion method with TS files");
            snprintf(ffmpeg_cmd, sizeof(ffmpeg_cmd),
                    "cd %s && ffmpeg -y -pattern_type glob -i \"*.ts\" -c copy -bsf:a aac_adtstoasc -movflags +faststart %s 2>/dev/null",
                    dir_path, output_path);

            log_info("Running alternative FFmpeg command: %s", ffmpeg_cmd);

            cmd_result = system(ffmpeg_cmd);
            if (cmd_result != 0) {
                log_error("Alternative FFmpeg command failed with status %d", cmd_result);
                create_recordings_error_response(response, 500, "{\"error\": \"Failed to convert recording\"}");
                return;
            }
        }

        // Verify the output file was created and has content
        struct stat st;
        if (stat(output_path, &st) != 0 || st.st_size == 0) {
            log_error("Converted MP4 file not found or empty: %s", output_path);
            create_recordings_error_response(response, 500, "{\"error\": \"Failed to convert recording\"}");
            return;
        }

        log_info("Successfully converted HLS to MP4: %s (%lld bytes)",
                output_path, (long long)st.st_size);

        // Create filename for download
        char filename[128];
        snprintf(filename, sizeof(filename), "%s_%lld.mp4",
               metadata->stream_name, (long long)metadata->start_time);

        // Set necessary headers
        set_response_header(response, "Content-Type", "application/octet-stream");
        char content_length[32];
        snprintf(content_length, sizeof(content_length), "%lld", (long long)st.st_size);
        set_response_header(response, "Content-Length", content_length);
        char disposition[256];
        snprintf(disposition, sizeof(disposition), "attachment; filename=\"%s\"", filename);
        set_response_header(response, "Content-Disposition", disposition);

        // Serve the converted file
        int result = create_file_response(response, 200, output_path, "application/octet-stream");
        if (result != 0) {
            log_error("Failed to create file response: %s", output_path);
            create_recordings_error_response(response, 500, "{\"error\": \"Failed to serve converted MP4 file\"}");
            return;
        }

        log_info("Converted MP4 download started: ID=%llu, Path=%s, Filename=%s",
               (unsigned long long)id, output_path, filename);
        
        // Register the temporary file for cleanup after serving
        // We need to make a copy of the path since it's on the stack
        char *temp_file_path = strdup(output_path);
        if (temp_file_path) {
            // Schedule the file for deletion after a delay to ensure it's fully downloaded
            schedule_file_deletion(temp_file_path);
        } else {
            log_warn("Failed to allocate memory for temporary file path, cleanup may not occur");
        }
    } else {
        // For non-HLS files, serve directly
        // Create filename for download
        char filename[128];
        snprintf(filename, sizeof(filename), "%s_%lld%s",
               metadata->stream_name, (long long)metadata->start_time,
               ext ? ext : ".mp4");

        // Get file size
        struct stat st;
        if (stat(metadata->file_path, &st) != 0) {
            log_error("Failed to stat file: %s", metadata->file_path);
            create_recordings_error_response(response, 500, "{\"error\": \"Failed to access recording file\"}");
            return;
        }

        // Set necessary headers
        set_response_header(response, "Content-Type", "application/octet-stream");
        char content_length[32];
        snprintf(content_length, sizeof(content_length), "%lld", (long long)st.st_size);
        set_response_header(response, "Content-Length", content_length);
        char disposition[256];
        snprintf(disposition, sizeof(disposition), "attachment; filename=\"%s\"", filename);
        set_response_header(response, "Content-Disposition", disposition);

        // Serve the file
        int result = create_file_response(response, 200, metadata->file_path, "application/octet-stream");
        if (result != 0) {
            log_error("Failed to create file response: %s", metadata->file_path);
            create_recordings_error_response(response, 500, "{\"error\": \"Failed to serve recording file\"}");
            return;
        }

        log_info("Original file download started: ID=%llu, Path=%s, Filename=%s",
               (unsigned long long)id, metadata->file_path, filename);
    }
}
