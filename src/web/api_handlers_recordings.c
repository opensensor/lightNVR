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

#include "web/api_handlers_recordings.h"
#include "web/api_handlers_common.h"
#include "core/config.h"
#include "core/logger.h"
#include "database/database_manager.h"
#include "storage/storage_manager.h"
#include "web/request_response.h"

/**
 * Handle GET request for recordings with pagination
 */
void handle_get_recordings(const http_request_t *request, http_response_t *response) {
    // Get query parameters
    char date_str[32] = {0};
    char stream_name[MAX_STREAM_NAME] = {0};
    char page_str[16] = {0};
    char limit_str[16] = {0};
    time_t start_time = 0;
    time_t end_time = 0;
    int page = 1;
    int limit = 20;
    
    // Get date filter if provided
    if (get_query_param(request, "date", date_str, sizeof(date_str)) == 0) {
        // Parse date string (format: YYYY-MM-DD)
        struct tm tm = {0};
        if (strptime(date_str, "%Y-%m-%d", &tm) != NULL) {
            // Set start time to beginning of day
            tm.tm_hour = 0;
            tm.tm_min = 0;
            tm.tm_sec = 0;
            start_time = mktime(&tm);
            
            // Set end time to end of day
            tm.tm_hour = 23;
            tm.tm_min = 59;
            tm.tm_sec = 59;
            end_time = mktime(&tm);
        } else {
            log_warn("Invalid date format: %s", date_str);
        }
    }
    
    // Get stream filter if provided
    get_query_param(request, "stream", stream_name, sizeof(stream_name));
    
    // If no stream name provided or "all" specified, set to NULL for all streams
    if (stream_name[0] == '\0' || strcmp(stream_name, "all") == 0) {
        stream_name[0] = '\0';
    }
    
    log_debug("Filtering recordings by stream: %s", stream_name[0] ? stream_name : "all streams");
    
    // Get pagination parameters if provided
    if (get_query_param(request, "page", page_str, sizeof(page_str)) == 0) {
        int parsed_page = atoi(page_str);
        if (parsed_page > 0) {
            page = parsed_page;
        }
    }
    
    if (get_query_param(request, "limit", limit_str, sizeof(limit_str)) == 0) {
        int parsed_limit = atoi(limit_str);
        if (parsed_limit > 0 && parsed_limit <= 100) {
            limit = parsed_limit;
        }
    }
    
    log_debug("Fetching recordings with pagination: page=%d, limit=%d", page, limit);
    
    // First, get total count of recordings matching the filters
    // We'll use a larger buffer to get all recordings, then count them
    recording_metadata_t all_recordings[500]; // Temporary buffer for counting
    int total_count = get_recording_metadata(start_time, end_time, 
                                          stream_name[0] ? stream_name : NULL, 
                                          all_recordings, 500);
    
    if (total_count < 0) {
        create_json_response(response, 500, "{\"error\": \"Failed to get recordings count\"}");
        return;
    }
    
    // Calculate pagination values
    int total_pages = (total_count + limit - 1) / limit; // Ceiling division
    if (total_pages == 0) total_pages = 1;
    if (page > total_pages) page = total_pages;
    
    // Calculate offset
    int offset = (page - 1) * limit;
    
    // Get paginated recordings from database
    recording_metadata_t recordings[100]; // Limit to 100 recordings max
    int actual_limit = (offset + limit <= total_count) ? limit : (total_count - offset);
    if (actual_limit <= 0) actual_limit = 0;
    
    // Copy the relevant slice from our all_recordings buffer
    int count = 0;
    for (int i = offset; i < offset + actual_limit && i < total_count; i++) {
        memcpy(&recordings[count], &all_recordings[i], sizeof(recording_metadata_t));
        count++;
    }
    
    // Build JSON response with pagination metadata
    char *json = malloc(count * 512 + 256); // Allocate enough space for all recordings + pagination metadata
    if (!json) {
        create_json_response(response, 500, "{\"error\": \"Memory allocation failed\"}");
        return;
    }
    
    // Start with pagination metadata
    int pos = sprintf(json, 
                     "{"
                     "\"pagination\": {"
                     "\"total\": %d,"
                     "\"page\": %d,"
                     "\"limit\": %d,"
                     "\"pages\": %d"
                     "},"
                     "\"recordings\": [",
                     total_count, page, limit, total_pages);
    
    for (int i = 0; i < count; i++) {
        char recording_json[512];
        
        // Format start and end times
        char start_time_str[32];
        char end_time_str[32];
        strftime(start_time_str, sizeof(start_time_str), "%Y-%m-%d %H:%M:%S", localtime(&recordings[i].start_time));
        strftime(end_time_str, sizeof(end_time_str), "%Y-%m-%d %H:%M:%S", localtime(&recordings[i].end_time));
        
        // Calculate duration in seconds
        int duration_sec = recordings[i].end_time - recordings[i].start_time;
        
        // Format duration as HH:MM:SS
        char duration_str[16];
        int hours = duration_sec / 3600;
        int minutes = (duration_sec % 3600) / 60;
        int seconds = duration_sec % 60;
        snprintf(duration_str, sizeof(duration_str), "%02d:%02d:%02d", hours, minutes, seconds);
        
        // Format size in human-readable format
        char size_str[16];
        if (recordings[i].size_bytes < 1024) {
            snprintf(size_str, sizeof(size_str), "%llu B", (unsigned long long)recordings[i].size_bytes);
        } else if (recordings[i].size_bytes < 1024 * 1024) {
            snprintf(size_str, sizeof(size_str), "%.1f KB", (float)recordings[i].size_bytes / 1024);
        } else if (recordings[i].size_bytes < 1024 * 1024 * 1024) {
            snprintf(size_str, sizeof(size_str), "%.1f MB", (float)recordings[i].size_bytes / (1024 * 1024));
        } else {
            snprintf(size_str, sizeof(size_str), "%.1f GB", (float)recordings[i].size_bytes / (1024 * 1024 * 1024));
        }
        
        // Create JSON for this recording
        snprintf(recording_json, sizeof(recording_json),
                 "{"
                 "\"id\": %llu,"
                 "\"stream\": \"%s\","
                 "\"start_time\": \"%s\","
                 "\"end_time\": \"%s\","
                 "\"duration\": \"%s\","
                 "\"size\": \"%s\","
                 "\"path\": \"%s\","
                 "\"width\": %d,"
                 "\"height\": %d,"
                 "\"fps\": %d,"
                 "\"codec\": \"%s\","
                 "\"complete\": %s"
                 "}",
                 (unsigned long long)recordings[i].id,
                 recordings[i].stream_name,
                 start_time_str,
                 end_time_str,
                 duration_str,
                 size_str,
                 recordings[i].file_path,
                 recordings[i].width,
                 recordings[i].height,
                 recordings[i].fps,
                 recordings[i].codec,
                 recordings[i].is_complete ? "true" : "false");
        
        // Add comma if not first element
        if (i > 0) {
            json[pos++] = ',';
        }
        
        // Append to JSON array
        strcpy(json + pos, recording_json);
        pos += strlen(recording_json);
    }
    
    // Close array and object
    pos += sprintf(json + pos, "]}");
    
    // Create response
    create_json_response(response, 200, json);
    
    // Free resources
    free(json);
    
    log_info("Served recordings page %d of %d (limit: %d, total: %d)", 
             page, total_pages, limit, total_count);
}

/**
 * Handle GET request for a specific recording
 */
void handle_get_recording(const http_request_t *request, http_response_t *response) {
    // Extract recording ID from the URL
    // URL format: /api/recordings/{id}
    const char *id_str = strrchr(request->path, '/');
    if (!id_str || strlen(id_str) <= 1) {
        create_json_response(response, 400, "{\"error\": \"Invalid recording ID\"}");
        return;
    }
    
    // Skip the '/'
    id_str++;
    
    // Convert ID to integer
    uint64_t id = strtoull(id_str, NULL, 10);
    if (id == 0) {
        create_json_response(response, 400, "{\"error\": \"Invalid recording ID\"}");
        return;
    }
    
    // Get recording metadata from database
    recording_metadata_t metadata;
    int result = get_recording_metadata_by_id(id, &metadata);
    
    if (result != 0) {
        create_json_response(response, 404, "{\"error\": \"Recording not found\"}");
        return;
    }
    
    // Format start and end times
    char start_time_str[32];
    char end_time_str[32];
    strftime(start_time_str, sizeof(start_time_str), "%Y-%m-%d %H:%M:%S", localtime(&metadata.start_time));
    strftime(end_time_str, sizeof(end_time_str), "%Y-%m-%d %H:%M:%S", localtime(&metadata.end_time));
    
    // Calculate duration in seconds
    int duration_sec = metadata.end_time - metadata.start_time;
    
    // Format duration as HH:MM:SS
    char duration_str[16];
    int hours = duration_sec / 3600;
    int minutes = (duration_sec % 3600) / 60;
    int seconds = duration_sec % 60;
    snprintf(duration_str, sizeof(duration_str), "%02d:%02d:%02d", hours, minutes, seconds);
    
    // Format size in human-readable format
    char size_str[16];
    if (metadata.size_bytes < 1024) {
        snprintf(size_str, sizeof(size_str), "%llu B", (unsigned long long)metadata.size_bytes);
    } else if (metadata.size_bytes < 1024 * 1024) {
        snprintf(size_str, sizeof(size_str), "%.1f KB", (float)metadata.size_bytes / 1024);
    } else if (metadata.size_bytes < 1024 * 1024 * 1024) {
        snprintf(size_str, sizeof(size_str), "%.1f MB", (float)metadata.size_bytes / (1024 * 1024));
    } else {
        snprintf(size_str, sizeof(size_str), "%.1f GB", (float)metadata.size_bytes / (1024 * 1024 * 1024));
    }
    
    // Create JSON response
    char json[1024];
    snprintf(json, sizeof(json),
             "{"
             "\"id\": %llu,"
             "\"stream\": \"%s\","
             "\"start_time\": \"%s\","
             "\"end_time\": \"%s\","
             "\"duration\": \"%s\","
             "\"size\": \"%s\","
             "\"path\": \"%s\","
             "\"width\": %d,"
             "\"height\": %d,"
             "\"fps\": %d,"
             "\"codec\": \"%s\","
             "\"complete\": %s,"
             "\"url\": \"/api/recordings/%llu/download\""
             "}",
             (unsigned long long)metadata.id,
             metadata.stream_name,
             start_time_str,
             end_time_str,
             duration_str,
             size_str,
             metadata.file_path,
             metadata.width,
             metadata.height,
             metadata.fps,
             metadata.codec,
             metadata.is_complete ? "true" : "false",
             (unsigned long long)metadata.id);
    
    // Create response
    create_json_response(response, 200, json);
}

/**
 * Handle DELETE request to remove a recording
 */
void handle_delete_recording(const http_request_t *request, http_response_t *response) {
    // Extract recording ID from the URL
    // URL format: /api/recordings/{id}
    const char *id_str = strrchr(request->path, '/');
    if (!id_str || strlen(id_str) <= 1) {
        create_json_response(response, 400, "{\"error\": \"Invalid recording ID\"}");
        return;
    }

    // Skip the '/'
    id_str++;

    // Convert ID to integer
    uint64_t id = strtoull(id_str, NULL, 10);
    if (id == 0) {
        create_json_response(response, 400, "{\"error\": \"Invalid recording ID\"}");
        return;
    }

    log_info("Attempting to delete recording with ID: %llu", (unsigned long long)id);

    // Get recording metadata from database
    recording_metadata_t metadata;
    int result = get_recording_metadata_by_id(id, &metadata);

    if (result != 0) {
        log_error("Recording with ID %llu not found in database", (unsigned long long)id);
        create_json_response(response, 404, "{\"error\": \"Recording not found\"}");
        return;
    }

    log_info("Found recording in database: ID=%llu, Path=%s", (unsigned long long)id, metadata.file_path);

    // Determine directory where recording segments are stored
    char dir_path[MAX_PATH_LENGTH];
    const char *last_slash = strrchr(metadata.file_path, '/');
    if (last_slash) {
        size_t dir_len = last_slash - metadata.file_path + 1; // Include the slash
        strncpy(dir_path, metadata.file_path, dir_len);
        dir_path[dir_len] = '\0';
        log_info("Recording directory: %s", dir_path);

        // Delete all TS segment files in this directory
        char delete_cmd[MAX_PATH_LENGTH + 50];
        snprintf(delete_cmd, sizeof(delete_cmd), "rm -f %s*.ts %s*.mp4 %s*.m3u8",
                dir_path, dir_path, dir_path);
        log_info("Executing cleanup command: %s", delete_cmd);
        system(delete_cmd); // Ignore result - we'll continue with metadata deletion anyway
    }

    // Explicitly try to delete the main file
    if (access(metadata.file_path, F_OK) == 0) {
        if (unlink(metadata.file_path) != 0) {
            log_warn("Failed to delete recording file: %s (error: %s)",
                    metadata.file_path, strerror(errno));
            // Continue anyway - we'll still delete the metadata
        } else {
            log_info("Successfully deleted recording file: %s", metadata.file_path);
        }
    } else {
        log_warn("Recording file not found on disk: %s", metadata.file_path);
    }

    // Delete MP4 recordings if they exist
    char mp4_path[MAX_PATH_LENGTH];
    snprintf(mp4_path, sizeof(mp4_path), "%srecording*.mp4", dir_path);

    // Use glob to find MP4 files - this requires including glob.h
    glob_t glob_result;
    if (glob(mp4_path, GLOB_NOSORT, NULL, &glob_result) == 0) {
        for (size_t i = 0; i < glob_result.gl_pathc; i++) {
            log_info("Deleting MP4 file: %s", glob_result.gl_pathv[i]);
            if (unlink(glob_result.gl_pathv[i]) != 0) {
                log_warn("Failed to delete MP4 file: %s (error: %s)",
                       glob_result.gl_pathv[i], strerror(errno));
            }
        }
        globfree(&glob_result);
    }

    // Delete the recording metadata from database
    if (delete_recording_metadata(id) != 0) {
        log_error("Failed to delete recording metadata for ID: %llu", (unsigned long long)id);
        create_json_response(response, 500, "{\"error\": \"Failed to delete recording metadata\"}");
        return;
    }
    
    // Create success response
    char json[256];
    snprintf(json, sizeof(json), 
             "{\"success\": true, \"id\": %llu, \"message\": \"Recording deleted successfully\"}", 
             (unsigned long long)id);
    
    create_json_response(response, 200, json);
    
    log_info("Recording deleted successfully: ID=%llu, Path=%s", (unsigned long long)id, metadata.file_path);
}

/**
 * Serve an MP4 file with proper headers for download
 */
void serve_mp4_file(http_response_t *response, const char *file_path, const char *filename) {
    // Verify file exists and is readable
    struct stat st;
    if (stat(file_path, &st) != 0 || access(file_path, R_OK) != 0) {
        log_error("MP4 file not accessible: %s (error: %s)", file_path, strerror(errno));
        create_json_response(response, 404, "{\"error\": \"Recording file not found\"}");
        return;
    }

    // Check file size
    if (st.st_size == 0) {
        log_error("MP4 file is empty: %s", file_path);
        create_json_response(response, 500, "{\"error\": \"Recording file is empty\"}");
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
        create_json_response(response, 500, "{\"error\": \"Failed to read recording file\"}");
        return;
    }

    // Allocate response body
    response->body = malloc(st.st_size);
    if (!response->body) {
        log_error("Failed to allocate memory for response body");
        close(fd);
        create_json_response(response, 500, "{\"error\": \"Server memory allocation failed\"}");
        return;
    }

    // Read file content into response body
    ssize_t bytes_read = read(fd, response->body, st.st_size);
    close(fd);

    if (bytes_read != st.st_size) {
        log_error("Failed to read complete file: %s (read %zd of %lld bytes)",
                file_path, bytes_read, (long long)st.st_size);
        free(response->body);
        create_json_response(response, 500, "{\"error\": \"Failed to read complete recording file\"}");
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
        create_json_response(response, 500, "{\"error\": \"Failed to read file\"}");
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
        create_json_response(response, 500, "{\"error\": \"Server memory allocation failed\"}");
        return;
    }

    // Read the file content
    ssize_t bytes_read = read(fd, response->body, file_size);
    close(fd);

    if (bytes_read != file_size) {
        log_error("Failed to read complete file: %s (read %zd of %lld bytes)",
                file_path, bytes_read, (long long)file_size);
        free(response->body);
        create_json_response(response, 500, "{\"error\": \"Failed to read complete file\"}");
        return;
    }

    // Set response body length
    response->body_length = file_size;

    log_info("File prepared for download: %s (%lld bytes)", file_path, (long long)file_size);
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
                create_json_response(response, 500, "{\"error\": \"Failed to serve recording file\"}");
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
                create_json_response(response, 500, "{\"error\": \"Failed to convert recording\"}");
                return;
            }
        }

        // Verify the output file was created and has content
        struct stat st;
        if (stat(output_path, &st) != 0 || st.st_size == 0) {
            log_error("Converted MP4 file not found or empty: %s", output_path);
            create_json_response(response, 500, "{\"error\": \"Failed to convert recording\"}");
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
            create_json_response(response, 500, "{\"error\": \"Failed to serve converted MP4 file\"}");
            return;
        }

        log_info("Converted MP4 download started: ID=%llu, Path=%s, Filename=%s",
               (unsigned long long)id, output_path, filename);
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
            create_json_response(response, 500, "{\"error\": \"Failed to access recording file\"}");
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
            create_json_response(response, 500, "{\"error\": \"Failed to serve recording file\"}");
            return;
        }

        log_info("Original file download started: ID=%llu, Path=%s, Filename=%s",
               (unsigned long long)id, metadata->file_path, filename);
    }
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
        create_json_response(response, 404, "{\"error\": \"Recording file not found\"}");
        return;
    }

    if (access(file_path, R_OK) != 0) {
        log_error("File not readable: %s", file_path);
        create_json_response(response, 403, "{\"error\": \"Recording file not readable\"}");
        return;
    }

    if (st.st_size == 0) {
        log_error("File is empty: %s", file_path);
        create_json_response(response, 500, "{\"error\": \"Recording file is empty\"}");
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
        create_json_response(response, 500, "{\"error\": \"Failed to serve recording file\"}");
        return;
    }

    log_info("Download started: Path=%s, Filename=%s", file_path, filename);
}

/**
 * Schedule a file for deletion after it has been served
 * This function should be customized based on your application's architecture
 */
void schedule_file_deletion(const char *file_path) {
    // Simple implementation: create a background task to delete the file after a delay
    char delete_cmd[512];

    // Wait 5 minutes before deleting to ensure the file has been fully downloaded
    snprintf(delete_cmd, sizeof(delete_cmd),
            "(sleep 300 && rm -f %s) > /dev/null 2>&1 &",
            file_path);

    system(delete_cmd);
    log_info("Scheduled temporary file for deletion: %s", file_path);
}

/**
 * Callback to remove temporary files after they've been sent
 */
void remove_temp_file_callback(void *data) {
    if (!data) return;

    char *temp_file_path = (char *)data;
    log_info("Removing temporary file: %s", temp_file_path);

    if (remove(temp_file_path) != 0) {
        log_warn("Failed to remove temporary file: %s (error: %s)",
               temp_file_path, strerror(errno));
    }

    free(temp_file_path);
}

/**
 * Handle GET request for debug database info
 */
void handle_get_debug_recordings(const http_request_t *request, http_response_t *response) {
    // Get recordings from database with no filters
    recording_metadata_t recordings[100]; // Limit to 100 recordings
    int count = get_recording_metadata(0, 0, NULL, recordings, 100);

    if (count < 0) {
        log_error("DEBUG: Failed to get recordings from database");
        create_json_response(response, 500, "{\"error\": \"Failed to get recordings\", \"count\": -1}");
        return;
    }

    // Create a detailed debug response
    char *debug_json = malloc(10000); // Allocate a large buffer
    if (!debug_json) {
        create_json_response(response, 500, "{\"error\": \"Memory allocation failed\"}");
        return;
    }

    // Start building JSON
    int pos = sprintf(debug_json,
        "{\n"
        "  \"count\": %d,\n"
        "  \"recordings\": [\n", count);

    for (int i = 0; i < count; i++) {
        // Add a comma between items (but not before the first item)
        if (i > 0) {
            pos += sprintf(debug_json + pos, ",\n");
        }

        char path_status[32] = "unknown";
        struct stat st;
        if (stat(recordings[i].file_path, &st) == 0) {
            strcpy(path_status, "exists");
        } else {
            strcpy(path_status, "missing");
        }

        pos += sprintf(debug_json + pos,
            "    {\n"
            "      \"id\": %llu,\n"
            "      \"stream\": \"%s\",\n"
            "      \"path\": \"%s\",\n"
            "      \"path_status\": \"%s\",\n"
            "      \"size\": %llu,\n"
            "      \"start_time\": %llu,\n"
            "      \"end_time\": %llu,\n"
            "      \"complete\": %s\n"
            "    }",
            (unsigned long long)recordings[i].id,
            recordings[i].stream_name,
            recordings[i].file_path,
            path_status,
            (unsigned long long)recordings[i].size_bytes,
            (unsigned long long)recordings[i].start_time,
            (unsigned long long)recordings[i].end_time,
            recordings[i].is_complete ? "true" : "false");
    }

    // Close JSON
    pos += sprintf(debug_json + pos, "\n  ]\n}");

    // Create response
    create_json_response(response, 200, debug_json);

    // Free resources
    free(debug_json);
}

/**
 * Handle GET request to download a recording - With fixed query parameter handling
 */
void handle_download_recording(const http_request_t *request, http_response_t *response) {
    // Extract recording ID from the URL
    const char *path = request->path;
    const char *prefix = "/api/recordings/download/";

    // Verify path starts with expected prefix
    if (strncmp(path, prefix, strlen(prefix)) != 0) {
        log_error("Invalid request path: %s", path);
        create_json_response(response, 400, "{\"error\": \"Invalid request path\"}");
        return;
    }

    // Extract the recording ID (everything after the prefix)
    const char *id_str = path + strlen(prefix);

    // Skip any leading slashes in the ID part
    while (*id_str == '/') {
        id_str++;
    }

    // Find query string if present and truncate
    char *id_str_copy = strdup(id_str);
    if (!id_str_copy) {
        log_error("Memory allocation failed for recording ID");
        create_json_response(response, 500, "{\"error\": \"Memory allocation failed\"}");
        return;
    }

    char *query_start = strchr(id_str_copy, '?');
    if (query_start) {
        *query_start = '\0'; // Truncate at query string
    }

    // Convert ID to integer
    uint64_t id = strtoull(id_str_copy, NULL, 10);
    if (id == 0) {
        log_error("Invalid recording ID: %s", id_str_copy);
        free(id_str_copy);
        create_json_response(response, 400, "{\"error\": \"Invalid recording ID\"}");
        return;
    }

    free(id_str_copy); // Free this as we don't need it anymore

    // Check for force download parameter - use the request's query params directly
    bool force_download = false;

    // Get 'download' parameter directly from request's query params
    char download_param[10] = {0};
    if (get_query_param(request, "download", download_param, sizeof(download_param)) == 0) {
        // Check if the parameter is set to "1" or "true"
        if (strcmp(download_param, "1") == 0 || strcmp(download_param, "true") == 0) {
            force_download = true;
            log_info("Force download requested for recording ID %llu (via query param)", (unsigned long long)id);
        }
    }

    // Get recording metadata from database
    recording_metadata_t metadata;
    int result = get_recording_metadata_by_id(id, &metadata);

    if (result != 0) {
        log_error("Recording with ID %llu not found in database", (unsigned long long)id);
        create_json_response(response, 404, "{\"error\": \"Recording not found\"}");
        return;
    }

    log_info("Found recording in database: ID=%llu, Path=%s, Download=%s",
             (unsigned long long)id, metadata.file_path, force_download ? "true" : "false");

    // Check if the file exists
    struct stat st;
    if (stat(metadata.file_path, &st) != 0) {
        log_error("Recording file not found on disk: %s (error: %s)",
                 metadata.file_path, strerror(errno));
        create_json_response(response, 404, "{\"error\": \"Recording file not found\"}");
        return;
    }

    // Determine if this is an MP4 file
    const char *ext = strrchr(metadata.file_path, '.');
    bool is_mp4 = (ext && strcasecmp(ext, ".mp4") == 0);

    // Generate a filename for download
    char filename[128];
    if (is_mp4) {
        snprintf(filename, sizeof(filename), "%s_%lld.mp4",
                metadata.stream_name, (long long)metadata.start_time);
    } else {
        // Use whatever extension the file has, or default to .mp4
        if (ext) {
            snprintf(filename, sizeof(filename), "%s_%lld%s",
                    metadata.stream_name, (long long)metadata.start_time, ext);
        } else {
            snprintf(filename, sizeof(filename), "%s_%lld.mp4",
                    metadata.stream_name, (long long)metadata.start_time);
        }
    }

    if (is_mp4 && !force_download) {
        // For MP4 files, serve with video/mp4 content type for playback
        log_info("Serving MP4 file with video/mp4 content type for playback: %s", metadata.file_path);

        // Set content type to video/mp4 for playback
        set_response_header(response, "Content-Type", "video/mp4");

        // Set content length
        char content_length[32];
        snprintf(content_length, sizeof(content_length), "%lld", (long long)st.st_size);
        set_response_header(response, "Content-Length", content_length);

        // Create file response with video/mp4 content type
        int result = create_file_response(response, 200, metadata.file_path, "video/mp4");
        if (result != 0) {
            log_error("Failed to create file response: %s", metadata.file_path);
            create_json_response(response, 500, "{\"error\": \"Failed to serve recording file\"}");
        }
    } else if (is_mp4) {
        // For MP4 files with forced download, use the serve_mp4_file function
        log_info("Serving MP4 file with attachment disposition for download: %s", metadata.file_path);
        serve_mp4_file(response, metadata.file_path, filename);
    } else {
        // For non-MP4 files, use the direct download approach
        serve_direct_download(response, id, &metadata);
    }
}
