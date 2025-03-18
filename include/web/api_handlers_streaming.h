#ifndef API_HANDLERS_STREAMING_H
#define API_HANDLERS_STREAMING_H

#include "web/web_server.h"

/**
 * Handle streaming request (HLS, WebRTC)
 */
void handle_streaming_request(const http_request_t *request, http_response_t *response);

/**
 * Handle HLS manifest request
 */
void handle_hls_manifest(const http_request_t *request, http_response_t *response);

/**
 * Handle HLS segment request
 */
void handle_hls_segment(const http_request_t *request, http_response_t *response);

/**
 * Handle WebRTC offer request
 */
void handle_webrtc_offer(const http_request_t *request, http_response_t *response);

/**
 * Handle WebRTC ICE request
 */
void handle_webrtc_ice(const http_request_t *request, http_response_t *response);

/**
 * Handle stream toggle request
 */
void handle_stream_toggle(const http_request_t *request, http_response_t *response);

/**
 * Register streaming API handlers
 */
void register_streaming_api_handlers(void);

#endif /* API_HANDLERS_STREAMING_H */
