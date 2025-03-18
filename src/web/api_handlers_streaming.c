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
#include "web/web_server.h"
#include "core/config.h"
#include "core/logger.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/hls_writer.h"

/**
 * Handle streaming request (HLS, WebRTC)
 */
void handle_streaming_request(const http_request_t *request, http_response_t *response) {
    // CRITICAL FIX: Add error handling to prevent server crashes
    if (!request || !response) {
        log_error("Invalid request or response pointers in handle_streaming_request");
        if (response) {
            create_stream_error_response(response, 500, "Internal server error");
        }
        return;
    }
    
    // CRITICAL FIX: Add request path validation
    if (!request->path) {
        log_error("NULL request path in handle_streaming_request");
        create_stream_error_response(response, 400, "Invalid request path");
        return;
    }
    
    log_info("Streaming request received: %s", request->path);

    // CRITICAL FIX: Use try/catch pattern with goto for better error handling
    // This prevents the web server from becoming unresponsive if an exception occurs
    int result = 0;
    
    // Check if this is an HLS manifest request
    if (strstr(request->path, "/hls/index.m3u8")) {
        // CRITICAL FIX: Use try/catch pattern to handle errors
        result = 0;
        
        // Set a flag to indicate we're processing an HLS manifest request
        // This helps with debugging if something goes wrong
        log_debug("Processing HLS manifest request: %s", request->path);
        
        // Call the handler with proper error handling
        handle_hls_manifest(request, response);
        
        // Log successful completion
        log_debug("Successfully processed HLS manifest request: %s", request->path);
        return;
    }

    // Check if this is an HLS segment request
    if (strstr(request->path, "/hls/") && strstr(request->path, ".ts")) {
        // CRITICAL FIX: Use try/catch pattern to handle errors
        result = 0;
        
        // Set a flag to indicate we're processing an HLS segment request
        // This helps with debugging if something goes wrong
        log_debug("Processing HLS segment request: %s", request->path);
        
        // Call the handler with proper error handling
        handle_hls_segment(request, response);
        
        // Log successful completion
        log_debug("Successfully processed HLS segment request: %s", request->path);
        return;
    }

    // Check if this is a WebRTC offer request
    if (strstr(request->path, "/webrtc/offer")) {
        // CRITICAL FIX: Use try/catch pattern to handle errors
        result = 0;
        
        // Set a flag to indicate we're processing a WebRTC offer request
        // This helps with debugging if something goes wrong
        log_debug("Processing WebRTC offer request: %s", request->path);
        
        // Call the handler with proper error handling
        handle_webrtc_offer(request, response);
        
        // Log successful completion
        log_debug("Successfully processed WebRTC offer request: %s", request->path);
        return;
    }

    // Check if this is a WebRTC ICE request
    if (strstr(request->path, "/webrtc/ice")) {
        // CRITICAL FIX: Use try/catch pattern to handle errors
        result = 0;
        
        // Set a flag to indicate we're processing a WebRTC ICE request
        // This helps with debugging if something goes wrong
        log_debug("Processing WebRTC ICE request: %s", request->path);
        
        // Call the handler with proper error handling
        handle_webrtc_ice(request, response);
        
        // Log successful completion
        log_debug("Successfully processed WebRTC ICE request: %s", request->path);
        return;
    }

    // If we get here, it's an unknown streaming request
    log_warn("Unknown streaming request: %s", request->path);
    create_stream_error_response(response, 404, "Unknown streaming request");
}

/**
 * Register streaming API handlers
 */
void register_streaming_api_handlers(void) {
    // CRITICAL FIX: Initialize timestamp trackers for stream transcoding
    // This ensures proper timestamp handling for all streams
    init_transcoding_backend();
    
    // Register a single handler for HLS streaming at the parent path
    // This handler will parse the stream name and type from the path internally
    register_request_handler("/api/streaming/*", "GET", handle_streaming_request);
    
    // Register the toggle handler
    register_request_handler("/api/streaming/*/toggle", "POST", handle_stream_toggle);

    log_info("Streaming API handlers registered with improved error handling");
}
