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

#include "web/api_handlers_streaming_control.h"
#include "web/api_handlers_common.h"
#include "web/request_response.h"
#include "web/web_server.h"
#include "core/config.h"
#include "core/logger.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/hls_writer.h"

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
        // Start the stream if not already running
        if (get_stream_status(stream) != STREAM_STATUS_RUNNING) {
            if (start_stream(stream) != 0) {
                log_error("Failed to start stream %s", stream_name);
                create_stream_error_response(response, 500, "Failed to start stream");
                return;
            }
            log_info("Started stream %s", stream_name);
        } else {
            log_info("Stream %s is already running", stream_name);
        }
        
        // Get the stream processor
        stream_processor_t processor = get_stream_processor(stream);
        if (!processor) {
            log_error("Failed to get stream processor for %s", stream_name);
            
            // CRITICAL FIX: Reset timestamp tracker before restarting
            reset_timestamp_tracker(stream_name);
            
            // Try to restart the stream
            log_info("Attempting to restart stream %s", stream_name);
            stop_stream(stream);
            
            // Wait a moment for the stream to stop
            usleep(200000);  // 200ms
            
            if (start_stream(stream) != 0) {
                log_error("Failed to restart stream %s", stream_name);
                create_stream_error_response(response, 500, "Failed to restart stream");
                return;
            }
            
            // Wait a moment for the stream to start
            usleep(800000);  // 800ms
            
            // Try to get the processor again
            processor = get_stream_processor(stream);
            if (!processor) {
                log_error("Still failed to get stream processor for %s after restart", stream_name);
                create_stream_error_response(response, 500, "Failed to get stream processor after restart");
                return;
            }
        }
        
        // Check if HLS writer exists
        hls_writer_t *hls_writer = stream_processor_get_hls_writer(processor);
        if (!hls_writer) {
            log_info("HLS writer not found in stream processor for %s, adding HLS output", stream_name);
            
            // CRITICAL FIX: Set UDP flag for timestamp tracker based on stream URL
            bool is_udp = false;
            if (strncmp(config.url, "udp://", 6) == 0 || strncmp(config.url, "rtp://", 6) == 0) {
                is_udp = true;
                log_info("Setting UDP flag for stream %s based on URL: %s", stream_name, config.url);
                set_timestamp_tracker_udp_flag(stream_name, true);
            }
            
            // First stop the processor with proper cleanup
            if (stream_processor_stop(processor) != 0) {
                log_error("Failed to stop stream processor for %s", stream_name);
                create_stream_error_response(response, 500, "Failed to stop stream processor");
                return;
            }
            
            // Wait a moment for the processor to fully stop
            usleep(100000);  // 100ms
            
            // Add HLS output to the processor
            output_config_t output_config;
            memset(&output_config, 0, sizeof(output_config));
            output_config.type = OUTPUT_TYPE_HLS;
            
            // Get HLS output path from global config
            config_t *global_config = get_streaming_config();
            snprintf(output_config.hls.output_path, MAX_PATH_LENGTH, "%s/hls/%s",
                    global_config->storage_path, stream_name);
            
            // Use segment duration from stream config or default to 4 seconds
            output_config.hls.segment_duration = config.segment_duration > 0 ?
                                               config.segment_duration : 4;
            
            // Create HLS directory if it doesn't exist
            char dir_cmd[MAX_PATH_LENGTH * 2];
            snprintf(dir_cmd, sizeof(dir_cmd), "mkdir -p %s", output_config.hls.output_path);
            int ret = system(dir_cmd);
            if (ret != 0) {
                log_error("Failed to create HLS directory: %s (return code: %d)", 
                         output_config.hls.output_path, ret);
                create_stream_error_response(response, 500, "Failed to create HLS directory");
                return;
            }
            
            // Set full permissions to ensure FFmpeg can write files
            snprintf(dir_cmd, sizeof(dir_cmd), "chmod -R 777 %s", output_config.hls.output_path);
            system(dir_cmd);
            
            if (stream_processor_add_output(processor, &output_config) != 0) {
                log_error("Failed to add HLS output to stream processor for %s", stream_name);
                create_stream_error_response(response, 500, "Failed to add HLS output to stream processor");
                return;
            }
            
            // CRITICAL FIX: Create a minimal valid manifest file before restarting
            char manifest_path[MAX_PATH_LENGTH];
            snprintf(manifest_path, MAX_PATH_LENGTH, "%s/index.m3u8", output_config.hls.output_path);
            FILE *manifest_init = fopen(manifest_path, "w");
            if (manifest_init) {
                // Write a minimal valid HLS manifest
                fprintf(manifest_init, "#EXTM3U\n");
                fprintf(manifest_init, "#EXT-X-VERSION:3\n");
                fprintf(manifest_init, "#EXT-X-TARGETDURATION:%d\n", output_config.hls.segment_duration);
                fprintf(manifest_init, "#EXT-X-MEDIA-SEQUENCE:0\n");
                fclose(manifest_init);
                
                log_info("Created initial valid HLS manifest file: %s", manifest_path);
            }
            
            // Restart the processor
            if (stream_processor_start(processor) != 0) {
                log_error("Failed to restart stream processor for %s", stream_name);
                create_stream_error_response(response, 500, "Failed to restart stream processor");
                return;
            }
            
            // Wait a moment for the HLS stream to initialize
            usleep(800000);  // 800ms - increased from 500ms for better initialization
            
            log_info("Successfully added HLS output to stream processor for %s", stream_name);
        } else {
            log_info("HLS writer already exists for stream %s", stream_name);
        }
    } else {
        // Stop the stream if running
        if (get_stream_status(stream) == STREAM_STATUS_RUNNING) {
            // Get the stream processor
            stream_processor_t processor = get_stream_processor(stream);
            if (processor) {
                // CRITICAL FIX: Reset timestamp tracker before removing output
                reset_timestamp_tracker(stream_name);
                
                // Remove HLS output from the processor
                if (stream_processor_remove_output(processor, OUTPUT_TYPE_HLS) != 0) {
                    log_warn("Failed to remove HLS output from stream processor for %s", stream_name);
                    // Continue anyway
                } else {
                    log_info("Removed HLS output from stream processor for %s", stream_name);
                }
                
                // Wait a moment for the output to be fully removed
                usleep(100000);  // 100ms
            }
            
            // If no other outputs are needed, stop the stream
            if (!config.record && !config.detection_based_recording) {
                if (stop_stream(stream) != 0) {
                    log_error("Failed to stop stream %s", stream_name);
                    create_stream_error_response(response, 500, "Failed to stop stream");
                    return;
                }
                log_info("Stopped stream %s", stream_name);
            } else {
                log_info("Keeping stream %s running for recording or detection", stream_name);
            }
        } else {
            log_info("Stream %s is already stopped", stream_name);
        }
    }
    
    // Create success response
    char response_json[256];
    snprintf(response_json, sizeof(response_json),
             "{\"success\": true, \"name\": \"%s\", \"streaming_enabled\": %s, \"recording_enabled\": %s}",
             stream_name, enabled ? "true" : "false", recording_enabled ? "true" : "false");
    
    create_json_response(response, 200, response_json);
}
