#ifndef STREAMS_H
#define STREAMS_H

#include "../web/web_server.h"

// Initialize FFmpeg streaming backend
void init_streaming_backend(void);

// Clean up FFmpeg resources
void cleanup_streaming_backend(void);

// Register streaming API handlers
void register_streaming_api_handlers(void);

// Start HLS transcoding for a stream
int start_hls_stream(const char *stream_name);

// Stop HLS transcoding for a stream
int stop_hls_stream(const char *stream_name);

// Handle request for HLS manifest
void handle_hls_manifest(const http_request_t *request, http_response_t *response);

// Handle request for HLS segment
void handle_hls_segment(const http_request_t *request, http_response_t *response);

// Handle WebRTC offer request
void handle_webrtc_offer(const http_request_t *request, http_response_t *response);

// Handle WebRTC ICE candidate request
void handle_webrtc_ice(const http_request_t *request, http_response_t *response);

// create_stream_error_response
void create_stream_error_response(http_response_t *response, int status_code, const char *message);

#endif /* STREAMS_H */