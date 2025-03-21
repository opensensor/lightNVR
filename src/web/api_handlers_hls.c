#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <ctype.h>
#include <stdbool.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <dirent.h>
#include <errno.h>

#include "cJSON.h"
#include "web/api_handlers_streaming.h"
#include "web/api_handlers_common.h"
#include "web/request_response.h"
#include "core/config.h"
#include "core/logger.h"
#include "video/stream_manager.h"
#include "video/stream_state.h"
#include "video/streams.h"
#include "video/hls_writer.h"

/**
 * Handle request for HLS manifest
 */
void handle_hls_manifest(const http_request_t *request, http_response_t *response) {
    // Extract stream name from URL
    // URL format: /api/streaming/{stream_name}/hls/index.m3u8

    const char *path = request->path;
    const char *streams_pos = strstr(path, "/streaming/");

    if (!streams_pos) {
        create_stream_error_response(response, 400, "Invalid request path");
        return;
    }

    const char *stream_name_start = streams_pos + 11;  // Skip "/streaming/"
    const char *hls_pos = strstr(stream_name_start, "/hls/");

    if (!hls_pos) {
        create_stream_error_response(response, 400, "Invalid HLS request path");
        return;
    }

    // Extract stream name
    char stream_name[MAX_STREAM_NAME];
    size_t name_len = hls_pos - stream_name_start;

    if (name_len >= MAX_STREAM_NAME) {
        create_stream_error_response(response, 400, "Stream name too long");
        return;
    }

    memcpy(stream_name, stream_name_start, name_len);
    stream_name[name_len] = '\0';

    // URL decode the stream name
    char decoded_stream[MAX_STREAM_NAME];
    url_decode(stream_name, decoded_stream, sizeof(decoded_stream));
    strncpy(stream_name, decoded_stream, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Check if stream exists
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        create_stream_error_response(response, 404, "Stream not found");
        return;
    }
    
    // Get stream configuration to check if streaming is enabled
    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        create_stream_error_response(response, 500, "Failed to get stream configuration");
        return;
    }
    
    // Check if streaming is enabled for this stream
    // If streaming_enabled is false, don't start streaming
    if (config.streaming_enabled == false) {
        // Streaming is disabled for this stream
        log_info("Streaming is disabled for stream %s", stream_name);
        create_stream_error_response(response, 403, "Streaming is disabled for this stream");
        return;
    }
    
    // Get the stream state to check if it's in the process of stopping and create it if it doesn't exist
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (state && is_stream_state_stopping(state)) {
        log_warn("Cannot start HLS stream %s while it is in the process of being stopped", stream_name);
        create_stream_error_response(response, 503, "Stream is in the process of stopping, please try again later");
        return;
    }
    if (!state) {
        log_warn("Stream state not found for %s, creating one", stream_name);
        
        // Get the stream configuration
        stream_config_t config;
        if (get_stream_config(stream, &config) != 0) {
            log_error("Failed to get stream configuration for %s", stream_name);
            create_stream_error_response(response, 500, "Failed to get stream configuration");
            return;
        }
        
        // Create a new stream state
        state = create_stream_state(&config);
        if (!state) {
            log_error("Failed to create stream state for %s", stream_name);
            create_stream_error_response(response, 500, "Failed to create stream state");
            return;
        }
        
        log_info("Created new stream state for %s", stream_name);
    }
    
    // Start HLS if not already running - this only starts streaming, not recording
    int hls_result = start_hls_stream(stream_name);
    if (hls_result != 0) {
        log_error("Failed to start HLS stream %s (error code: %d)", stream_name, hls_result);
        create_stream_error_response(response, 500, "Failed to start HLS stream");
        return;
    }
    
    log_info("Successfully started or confirmed HLS stream for %s", stream_name);

    // Get the manifest file path
    config_t *global_config = get_streaming_config();
    
    // Log the storage path for debugging
    log_info("API looking for HLS manifest in storage path: %s", global_config->storage_path);
    
    // CRITICAL FIX: Use the correct path for HLS manifests
    // The storage path is already set to /var/lib/lightnvr/recordings in the config
    // So we need to use /var/lib/lightnvr/recordings/hls/
    char manifest_path[MAX_PATH_LENGTH];
    snprintf(manifest_path, MAX_PATH_LENGTH, "%s/hls/%s/index.m3u8",
             global_config->storage_path, stream_name);
    
    // Log the full manifest path
    log_info("Full manifest path: %s", manifest_path);

    // Wait for the manifest file to be created with a longer timeout for low-powered devices
    // Try up to 50 times with 100ms between attempts (5 seconds total)
    bool manifest_exists = false;
    for (int i = 0; i < 50; i++) {
        // Check for the final manifest file
        if (access(manifest_path, F_OK) == 0) {
            manifest_exists = true;
            break;
        }

        // Also check for the temporary file
        char temp_manifest_path[MAX_PATH_LENGTH];
        snprintf(temp_manifest_path, MAX_PATH_LENGTH, "%s.tmp", manifest_path);
        if (access(temp_manifest_path, F_OK) == 0) {
            log_debug("Found temporary manifest file, waiting for it to be finalized");
            // Continue waiting for the final file
        }

        log_debug("Waiting for manifest file to be created (attempt %d/50)", i+1);
        usleep(100000);  // 100ms
    }

    if (!manifest_exists) {
        log_error("Manifest file was not created in time: %s", manifest_path);
        
        // Check if the directory exists
        char dir_path[MAX_PATH_LENGTH];
        snprintf(dir_path, sizeof(dir_path), "%s/hls/%s", global_config->storage_path, stream_name);
        
        struct stat st;
        if (stat(dir_path, &st) != 0 || !S_ISDIR(st.st_mode)) {
            log_error("HLS directory does not exist: %s", dir_path);
            
            // Try to create it
            char mkdir_cmd[MAX_PATH_LENGTH * 2];
            snprintf(mkdir_cmd, sizeof(mkdir_cmd), "mkdir -p %s && chmod -R 777 %s", 
                    dir_path, dir_path);
            system(mkdir_cmd);
            
            log_info("Created HLS directory: %s", dir_path);
        }
        
        // Try to restart the HLS stream
        stop_hls_stream(stream_name);
        
        // Wait a short time to ensure the stream is fully stopped
        usleep(500000); // 500ms
        
        // Check if the stream is still in the process of stopping
        stream_state_manager_t *state = get_stream_state_by_name(stream_name);
        if (state && is_stream_state_stopping(state)) {
            log_warn("Stream %s is still in the process of stopping, cannot restart yet", stream_name);
            create_stream_error_response(response, 503, "Stream is still stopping, please try again later");
            return;
        }
        
        if (start_hls_stream(stream_name) != 0) {
            log_error("Failed to restart HLS stream for %s", stream_name);
            create_stream_error_response(response, 500, "Failed to start HLS stream");
            return;
        }
        
        log_info("Restarted HLS stream for %s, but manifest file still not available", stream_name);
        create_stream_error_response(response, 404, "Manifest file not found, please try again");
        return;
    }

    // Read the manifest file
    FILE *fp = fopen(manifest_path, "r");
    if (!fp) {
        create_stream_error_response(response, 500, "Failed to open manifest file");
        return;
    }

    // Get file size
    fseek(fp, 0, SEEK_END);
    long file_size = ftell(fp);
    fseek(fp, 0, SEEK_SET);

    // Read file content
    char *content = malloc(file_size + 1);
    if (!content) {
        fclose(fp);
        create_stream_error_response(response, 500, "Memory allocation failed");
        return;
    }

    size_t bytes_read = fread(content, 1, file_size, fp);
    if (bytes_read != (size_t)file_size) {
        free(content);
        fclose(fp);
        create_stream_error_response(response, 500, "Failed to read manifest file");
        return;
    }

    content[file_size] = '\0';
    fclose(fp);
    
    // Check if the manifest file is empty or doesn't contain the EXTM3U delimiter
    if (file_size == 0 || strstr(content, "#EXTM3U") == NULL) {
        log_error("Manifest file is empty or missing EXTM3U delimiter: %s", manifest_path);
        
        // If the manifest file is empty or invalid, restart the stream to let FFmpeg create a proper one
        // FFmpeg is now configured to create valid manifest files with segments immediately
        log_info("Found empty or invalid manifest file for %s, restarting stream", stream_name);
        
        // If creating a minimal manifest failed or the file had invalid content, restart the stream
    log_info("Restarting HLS stream for %s to generate a fresh manifest", stream_name);
    
    // Stop the stream
    stop_hls_stream(stream_name);
    
    // Wait a moment to ensure the stream is fully stopped
    usleep(500000); // 500ms
    
    // Start the stream again
    if (start_hls_stream(stream_name) != 0) {
        log_error("Failed to restart HLS stream for %s", stream_name);
        free(content);
        create_stream_error_response(response, 500, "Failed to restart HLS stream");
        return;
    }
    
    // Wait for the manifest file to be created
    for (int i = 0; i < 30; i++) {
        // Check if the file exists and has content
        FILE *check = fopen(manifest_path, "r");
        if (check) {
            fseek(check, 0, SEEK_END);
            long size = ftell(check);
            fseek(check, 0, SEEK_SET);
            
            if (size > 0) {
                // Read the new content
                free(content);
                content = malloc(size + 1);
                if (!content) {
                    fclose(check);
                    create_stream_error_response(response, 500, "Memory allocation failed");
                    return;
                }
                
                size_t bytes_read = fread(content, 1, size, check);
                if (bytes_read == (size_t)size) {
                    content[size] = '\0';
                    file_size = size;
                    fclose(check);
                    
                    if (strstr(content, "#EXTM3U") != NULL) {
                        log_info("Successfully regenerated manifest file for %s", stream_name);
                        break;
                    }
                }
                fclose(check);
            } else {
                fclose(check);
            }
        }
        
        // If we've tried 30 times and still don't have a valid manifest, give up
        if (i == 29) {
            log_error("Failed to generate valid manifest file for %s after 30 attempts", stream_name);
            free(content);
            create_stream_error_response(response, 500, "Failed to generate valid manifest file");
            return;
        }
        
        usleep(100000); // 100ms
    }
        
        // Wait for the manifest file to be properly created
        bool manifest_valid = false;
        for (int i = 0; i < 30; i++) {
            // Re-read the manifest file
            FILE *fp_retry = fopen(manifest_path, "r");
            if (fp_retry) {
                fseek(fp_retry, 0, SEEK_END);
                long new_size = ftell(fp_retry);
                fseek(fp_retry, 0, SEEK_SET);
                
                log_debug("Manifest file check (attempt %d/30): size=%ld, path=%s", 
                         i+1, new_size, manifest_path);
                
                if (new_size > 0) {
                    // Free old content and allocate new buffer
                    free(content);
                    content = malloc(new_size + 1);
                    if (!content) {
                        fclose(fp_retry);
                        create_stream_error_response(response, 500, "Memory allocation failed");
                        return;
                    }
                    
                    size_t new_bytes_read = fread(content, 1, new_size, fp_retry);
                    log_debug("Read %zu bytes from manifest file (attempt %d/30)", new_bytes_read, i+1);
                    
                    if (new_bytes_read == (size_t)new_size) {
                        content[new_size] = '\0';
                        if (strstr(content, "#EXTM3U") != NULL) {
                            file_size = new_size;
                            manifest_valid = true;
                            log_info("Successfully regenerated valid manifest file for %s", stream_name);
                            fclose(fp_retry);
                            break;
                        } else {
                            log_debug("Manifest file does not contain #EXTM3U tag (attempt %d/30)", i+1);
                        }
                    } else {
                        log_debug("Failed to read complete manifest file: read %zu of %ld bytes (attempt %d/30)", 
                                 new_bytes_read, new_size, i+1);
                    }
                } else {
                    log_debug("Manifest file is empty (attempt %d/30)", i+1);
                }
                fclose(fp_retry);
            } else {
                log_debug("Failed to open manifest file: %s (error: %s) (attempt %d/30)", 
                         manifest_path, strerror(errno), i+1);
            }
            
            log_debug("Waiting for valid manifest file (attempt %d/30)", i+1);
            usleep(100000);  // 100ms
        }
        
        if (!manifest_valid) {
            log_error("Failed to generate valid manifest file for %s", stream_name);
            log_error("Manifest path: %s", manifest_path);
            
            // Check if the directory exists and list its contents
            char dir_path[MAX_PATH_LENGTH];
            snprintf(dir_path, sizeof(dir_path), "%s/hls/%s", global_config->storage_path, stream_name);
            
            struct stat st;
            if (stat(dir_path, &st) == 0 && S_ISDIR(st.st_mode)) {
                log_error("HLS directory exists: %s", dir_path);
                
                // List directory contents
                DIR *dir = opendir(dir_path);
                if (dir) {
                    struct dirent *entry;
                    log_error("HLS directory contents:");
                    while ((entry = readdir(dir)) != NULL) {
                        log_error("  %s", entry->d_name);
                    }
                    closedir(dir);
                } else {
                    log_error("Failed to open HLS directory: %s (error: %s)", 
                             dir_path, strerror(errno));
                }
            } else {
                log_error("HLS directory does not exist: %s (error: %s)", 
                         dir_path, strerror(errno));
            }
            
            free(content);
            create_stream_error_response(response, 500, "Failed to generate valid manifest file");
            return;
        }
    }

    // Create response
    response->status_code = 200;
    strncpy(response->content_type, "application/vnd.apple.mpegurl", sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';
    
    // Add strict cache control headers to prevent caching of HLS manifests
    set_response_header(response, "Cache-Control", "no-cache, no-store, must-revalidate, max-age=0");
    set_response_header(response, "Pragma", "no-cache");
    set_response_header(response, "Expires", "0");
    
    // Add timestamp header to help client identify the freshness of the manifest
    char timestamp_str[32];
    time_t now = time(NULL);
    snprintf(timestamp_str, sizeof(timestamp_str), "%ld", now);
    set_response_header(response, "X-Timestamp", timestamp_str);
    
    response->body = content;
    response->body_length = file_size;
}

/**
 * Handle request for HLS segment
 */
void handle_hls_segment(const http_request_t *request, http_response_t *response) {
    // Extract stream name and segment from URL
    // URL format: /api/streaming/{stream_name}/hls/segment_{number}.ts or /api/streaming/{stream_name}/hls/index{number}.ts

    const char *path = request->path;
    const char *streams_pos = strstr(path, "/streaming/");

    if (!streams_pos) {
        create_stream_error_response(response, 400, "Invalid request path");
        return;
    }

    const char *stream_name_start = streams_pos + 11;  // Skip "/streaming/"
    const char *hls_pos = strstr(stream_name_start, "/hls/");

    if (!hls_pos) {
        create_stream_error_response(response, 400, "Invalid HLS request path");
        return;
    }

    // Extract stream name
    char stream_name[MAX_STREAM_NAME];
    size_t name_len = hls_pos - stream_name_start;

    if (name_len >= MAX_STREAM_NAME) {
        create_stream_error_response(response, 400, "Stream name too long");
        return;
    }

    memcpy(stream_name, stream_name_start, name_len);
    stream_name[name_len] = '\0';

    // URL decode the stream name
    char decoded_stream[MAX_STREAM_NAME];
    url_decode(stream_name, decoded_stream, sizeof(decoded_stream));
    strncpy(stream_name, decoded_stream, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Extract segment filename
    const char *segment_filename = hls_pos + 5;  // Skip "/hls/"

    log_info("Segment requested: %s", segment_filename);

    // Get the segment file path
    config_t *global_config = get_streaming_config();
    
    // CRITICAL FIX: Use a consistent path for HLS segments
    // Always use the /hls/ directory structure without /recordings/
    char segment_path[MAX_PATH_LENGTH];
    snprintf(segment_path, MAX_PATH_LENGTH, "%s/hls/%s/%s",
             global_config->storage_path, stream_name, segment_filename);

    log_info("Looking for segment at path: %s", segment_path);

    // Check if segment file exists
    if (access(segment_path, F_OK) != 0) {
        log_debug("Segment file not found on first attempt: %s (%s)", segment_path, strerror(errno));

        // Wait for it to be created with a longer timeout for low-powered devices
        bool segment_exists = false;
        for (int i = 0; i < 40; i++) {  // Try for 4 seconds total (increased from 2 seconds)
            if (access(segment_path, F_OK) == 0) {
                log_info("Segment file found after waiting: %s (attempt %d)", segment_path, i+1);
                segment_exists = true;
                break;
            }

            log_debug("Waiting for segment file to be created: %s (attempt %d/40)", segment_path, i+1);
            usleep(100000);  // 100ms
        }

        if (!segment_exists) {
            log_error("Segment file not found after waiting: %s", segment_path);
            create_stream_error_response(response, 404, "Segment file not found");
            return;
        }
    }

    // Read the segment file
    FILE *fp = fopen(segment_path, "rb");
    if (!fp) {
        log_error("Failed to open segment file: %s (%s)", segment_path, strerror(errno));
        create_stream_error_response(response, 500, "Failed to open segment file");
        return;
    }

    // Get file size
    fseek(fp, 0, SEEK_END);
    long file_size = ftell(fp);
    fseek(fp, 0, SEEK_SET);

    log_info("Successfully opened segment file, size: %ld bytes", file_size);

    // Read file content
    char *content = malloc(file_size);
    if (!content) {
        log_error("Memory allocation failed for file content, size: %ld", file_size);
        fclose(fp);
        create_stream_error_response(response, 500, "Memory allocation failed");
        return;
    }

    size_t bytes_read = fread(content, 1, file_size, fp);
    if (bytes_read != (size_t)file_size) {
        log_error("Failed to read full file content: read %zu of %ld bytes", bytes_read, file_size);
        free(content);
        fclose(fp);
        create_stream_error_response(response, 500, "Failed to read segment file");
        return;
    }

    fclose(fp);

    // Create response
    response->status_code = 200;
    strncpy(response->content_type, "video/mp2t", sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';
    
    // Add strict cache control headers to prevent caching of HLS segments
    set_response_header(response, "Cache-Control", "no-cache, no-store, must-revalidate, max-age=0");
    set_response_header(response, "Pragma", "no-cache");
    set_response_header(response, "Expires", "0");
    
    // Add timestamp header to help client identify the freshness of the segment
    char timestamp_str[32];
    time_t now = time(NULL);
    snprintf(timestamp_str, sizeof(timestamp_str), "%ld", now);
    set_response_header(response, "X-Timestamp", timestamp_str);
    
    response->body = content;
    response->body_length = file_size;

    log_info("Successfully served segment: %s", segment_filename);
}

/**
 * Handle stream toggle request
 */
void handle_stream_toggle(const http_request_t *request, http_response_t *response) {
    // Extract stream name from URL
    // URL format: /api/streaming/{stream_name}/toggle
    
    const char *path = request->path;
    const char *streams_pos = strstr(path, "/streaming/");
    
    if (!streams_pos) {
        create_stream_error_response(response, 400, "Invalid request path");
        return;
    }
    
    const char *stream_name_start = streams_pos + 11;  // Skip "/streaming/"
    const char *toggle_pos = strstr(stream_name_start, "/toggle");
    
    if (!toggle_pos) {
        create_stream_error_response(response, 400, "Invalid toggle request path");
        return;
    }
    
    // Extract stream name
    char stream_name[MAX_STREAM_NAME];
    size_t name_len = toggle_pos - stream_name_start;
    
    if (name_len >= MAX_STREAM_NAME) {
        create_stream_error_response(response, 400, "Stream name too long");
        return;
    }
    
    memcpy(stream_name, stream_name_start, name_len);
    stream_name[name_len] = '\0';
    
    // URL decode the stream name
    char decoded_stream[MAX_STREAM_NAME];
    url_decode(stream_name, decoded_stream, sizeof(decoded_stream));
    strncpy(stream_name, decoded_stream, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';
    
    // Check if stream exists
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        create_stream_error_response(response, 404, "Stream not found");
        return;
    }
    
    // Get current stream configuration to check recording state
    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        create_stream_error_response(response, 500, "Failed to get stream configuration");
        return;
    }
    
    // Store the current recording state
    bool recording_enabled = config.record;
    log_info("Current recording state for stream %s: %s", 
             stream_name, recording_enabled ? "enabled" : "disabled");
    
    // Parse request body to get enabled flag
    if (!request->body || request->content_length == 0) {
        create_stream_error_response(response, 400, "Empty request body");
        return;
    }
    
    // Make a null-terminated copy of the request body
    char *json = malloc(request->content_length + 1);
    if (!json) {
        create_stream_error_response(response, 500, "Memory allocation failed");
        return;
    }
    
    memcpy(json, request->body, request->content_length);
    json[request->content_length] = '\0';
    
    // Parse the enabled flag from the JSON using cJSON
    cJSON *json_obj = cJSON_Parse(json);
    if (!json_obj) {
        log_error("Failed to parse JSON: %s", json);
        free(json);
        create_stream_error_response(response, 400, "Invalid JSON format");
        return;
    }
    
    // Get the enabled flag
    cJSON *enabled_item = cJSON_GetObjectItemCaseSensitive(json_obj, "enabled");
    bool enabled = true; // Default value
    
    if (cJSON_IsBool(enabled_item)) {
        enabled = cJSON_IsTrue(enabled_item);
    }
    
    // Clean up
    cJSON_Delete(json_obj);
    free(json);
    
    log_info("Toggle streaming request for stream %s: enabled=%s", stream_name, enabled ? "true" : "false");
    
    // Set the streaming_enabled flag in the stream configuration
    set_stream_streaming_enabled(stream, enabled);
    
    // Toggle the stream
    if (enabled) {
        // Check if the stream is in the process of stopping
        stream_state_manager_t *state = get_stream_state_by_name(stream_name);
        if (state && is_stream_state_stopping(state)) {
            log_warn("Cannot start HLS stream %s while it is in the process of being stopped", stream_name);
            create_stream_error_response(response, 503, "Stream is in the process of stopping, please try again later");
            return;
        }
        
        // Start HLS stream if not already running
        if (start_hls_stream(stream_name) != 0) {
            create_stream_error_response(response, 500, "Failed to start HLS stream");
            return;
        }
        log_info("Started HLS stream for %s", stream_name);
    } else {
        // Stop HLS stream if running
        if (stop_hls_stream(stream_name) != 0) {
            create_stream_error_response(response, 500, "Failed to stop HLS stream");
            return;
        }
        log_info("Stopped HLS stream for %s", stream_name);
    }
    
    // If recording is enabled, ensure it's running regardless of streaming state
    if (recording_enabled) {
        // Check current recording state
        int recording_state = get_recording_state(stream_name);
        
        if (recording_state == 0) {
            // Recording is not active, start it
            log_info("Ensuring recording is active for stream %s (independent of streaming)", stream_name);
            if (start_mp4_recording(stream_name) != 0) {
                log_warn("Failed to start recording for stream %s", stream_name);
                // Continue anyway, this is not a fatal error for streaming
            } else {
                log_info("Successfully started recording for stream %s", stream_name);
            }
        } else if (recording_state == 1) {
            log_info("Recording is already active for stream %s", stream_name);
        } else {
            log_warn("Could not determine recording state for stream %s", stream_name);
        }
    }
    
    // Create success response using cJSON
    cJSON *success_response = cJSON_CreateObject();
    if (!success_response) {
        log_error("Failed to create success response JSON object");
        create_stream_error_response(response, 500, "Failed to create response");
        return;
    }
    
    cJSON_AddBoolToObject(success_response, "success", true);
    cJSON_AddStringToObject(success_response, "name", stream_name);
    cJSON_AddBoolToObject(success_response, "streaming_enabled", enabled);
    cJSON_AddBoolToObject(success_response, "recording_enabled", recording_enabled);
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(success_response);
    if (!json_str) {
        log_error("Failed to convert success response JSON to string");
        cJSON_Delete(success_response);
        create_stream_error_response(response, 500, "Failed to create response");
        return;
    }
    
    // Create response
    create_json_response(response, 200, json_str);
    
    // Clean up
    free(json_str);
    cJSON_Delete(success_response);
}
