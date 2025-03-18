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

#include "web/api_handlers_streaming.h"
#include "web/api_handlers_common.h"
#include "web/request_response.h"
#include "core/config.h"
#include "core/logger.h"
#include "video/stream_manager.h"
#include "video/stream_state.h"
#include "video/streams.h"
#include "video/hls_writer.h"
#include <stdbool.h>

/**
 * Handle streaming request (HLS, WebRTC)
 */
void handle_streaming_request(const http_request_t *request, http_response_t *response) {
    log_info("Streaming request received: %s", request->path);

    // Check if this is an HLS manifest request
    if (strstr(request->path, "/hls/index.m3u8")) {
        handle_hls_manifest(request, response);
        return;
    }

    // Check if this is an HLS segment request
    if (strstr(request->path, "/hls/index") && strstr(request->path, ".ts")) {
        handle_hls_segment(request, response);
        return;
    }

    // Check if this is a WebRTC offer request
    if (strstr(request->path, "/webrtc/offer")) {
        handle_webrtc_offer(request, response);
        return;
    }

    // Check if this is a WebRTC ICE request
    if (strstr(request->path, "/webrtc/ice")) {
        handle_webrtc_ice(request, response);
        return;
    }

    // If we get here, it's an unknown streaming request
    create_stream_error_response(response, 404, "Unknown streaming request");
}

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
    
    // Get the stream state to check if it's in the process of stopping
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (state && is_stream_state_stopping(state)) {
        log_warn("Cannot start HLS stream %s while it is in the process of being stopped", stream_name);
        create_stream_error_response(response, 503, "Stream is in the process of stopping, please try again later");
        return;
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
        
        // Try to restart the HLS stream
        stop_hls_stream(stream_name);
        
        // Wait a short time to ensure the stream is fully stopped
        usleep(500000); // 500ms
        
        // Check if the stream is still in the process of stopping
        stream_state_manager_t *state = get_stream_state_by_name(stream_name);
        if (state && is_stream_state_stopping(state)) {
            log_warn("Stream %s is still in the process of stopping, cannot restart yet", stream_name);
            free(content);
            create_stream_error_response(response, 503, "Stream is still stopping, please try again later");
            return;
        }
        
        if (start_hls_stream(stream_name) != 0) {
            log_error("Failed to restart HLS stream for %s", stream_name);
            free(content);
            create_stream_error_response(response, 500, "Failed to restart HLS stream");
            return;
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
                    if (new_bytes_read == (size_t)new_size) {
                        content[new_size] = '\0';
                        if (strstr(content, "#EXTM3U") != NULL) {
                            file_size = new_size;
                            manifest_valid = true;
                            log_info("Successfully regenerated valid manifest file for %s", stream_name);
                            fclose(fp_retry);
                            break;
                        }
                    }
                }
                fclose(fp_retry);
            }
            
            log_debug("Waiting for valid manifest file (attempt %d/30)", i+1);
            usleep(100000);  // 100ms
        }
        
        if (!manifest_valid) {
            log_error("Failed to generate valid manifest file for %s", stream_name);
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
 * URL decode function
 */
static void url_decode_stream(char *str) {
    char *src = str;
    char *dst = str;
    char a, b;

    while (*src) {
        if ((*src == '%') && ((a = src[1]) && (b = src[2])) &&
            (isxdigit(a) && isxdigit(b))) {
            if (a >= 'a') a -= 'a' - 'A';
            if (a >= 'A') a -= ('A' - 10);
            else a -= '0';

            if (b >= 'a') b -= 'a' - 'A';
            if (b >= 'A') b -= ('A' - 10);
            else b -= '0';

            *dst++ = 16 * a + b;
            src += 3;
            } else if (*src == '+') {
                *dst++ = ' ';
                src++;
            } else {
                *dst++ = *src++;
            }
    }
    *dst = '\0';
}

/**
 * Handle WebRTC offer request - simple placeholder implementation
 */
void handle_webrtc_offer(const http_request_t *request, http_response_t *response) {
    // Extract stream name from URL
    const char *path = request->path;
    const char *streams_pos = strstr(path, "/streaming/");

    if (!streams_pos) {
        create_stream_error_response(response, 400, "Invalid request path");
        return;
    }

    const char *stream_name_start = streams_pos + 11;  // Skip "/streaming/"
    const char *webrtc_pos = strstr(stream_name_start, "/webrtc/");

    if (!webrtc_pos) {
        create_stream_error_response(response, 400, "Invalid WebRTC request path");
        return;
    }

    // Extract stream name
    char stream_name[MAX_STREAM_NAME];
    size_t name_len = webrtc_pos - stream_name_start;

    if (name_len >= MAX_STREAM_NAME) {
        create_stream_error_response(response, 400, "Stream name too long");
        return;
    }

    memcpy(stream_name, stream_name_start, name_len);
    stream_name[name_len] = '\0';

    // URL decode the stream name
    char decoded_stream[MAX_STREAM_NAME];
    url_decode_stream(decoded_stream);
    strncpy(stream_name, decoded_stream, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Check if stream exists
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        create_stream_error_response(response, 404, "Stream not found");
        return;
    }

    // In a real implementation, this would:
    // 1. Parse the WebRTC offer from the request body
    // 2. Use the libWebRTC or similar library to create an answer
    // 3. Send the answer back to the client

    // For now, just acknowledge the request with a placeholder

    log_info("Received WebRTC offer for stream %s", stream_name);

    // Create placeholder response
    const char *response_json = "{\"status\": \"acknowledged\", \"message\": \"WebRTC not yet implemented\"}";

    response->status_code = 200;
    strncpy(response->content_type, "application/json", sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';
    response->body = strdup(response_json);

    // Indicate that body should be freed (if needed)
    // response->needs_free = 1;
}

/**
 * Handle WebRTC ICE request
 */
void handle_webrtc_ice(const http_request_t *request, http_response_t *response) {
    // This is a placeholder implementation
    // In a real implementation, we would process the WebRTC ICE candidates
    
    log_info("WebRTC ICE request: %s", request->path);
    
    // For now, just return a 501 error (not implemented)
    create_stream_error_response(response, 501, "WebRTC not implemented");
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
    
    // Parse the enabled flag from the JSON
    bool enabled = get_json_boolean_value(json, "enabled", true);
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
    
    // Create success response
    char response_json[256];
    snprintf(response_json, sizeof(response_json),
             "{\"success\": true, \"name\": \"%s\", \"streaming_enabled\": %s, \"recording_enabled\": %s}",
             stream_name, enabled ? "true" : "false", recording_enabled ? "true" : "false");
    
    create_json_response(response, 200, response_json);
}

/**
 * Register streaming API handlers
 */
void register_streaming_api_handlers(void) {
    // Register a single handler for HLS streaming at the parent path
    // This handler will parse the stream name and type from the path internally
    register_request_handler("/api/streaming/*", "GET", handle_streaming_request);

    log_info("Streaming API handlers registered");
}
