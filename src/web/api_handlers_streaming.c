#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <ctype.h>
#include <stdbool.h>

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
 * Register streaming API handlers
 */
void register_streaming_api_handlers(void) {
    // Register a single handler for HLS streaming at the parent path
    // This handler will parse the stream name and type from the path internally
    register_request_handler("/api/streaming/*", "GET", handle_streaming_request);

    log_info("Streaming API handlers registered");
}
