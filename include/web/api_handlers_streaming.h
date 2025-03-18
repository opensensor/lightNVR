#ifndef API_HANDLERS_STREAMING_H
#define API_HANDLERS_STREAMING_H

#include "web/web_server.h"
#include "web/api_handlers_streaming_hls.h"
#include "web/api_handlers_streaming_webrtc.h"
#include "web/api_handlers_streaming_control.h"

/**
 * Handle streaming request (HLS, WebRTC)
 * Main entry point for all streaming requests
 */
void handle_streaming_request(const http_request_t *request, http_response_t *response);

/**
 * Register streaming API handlers
 */
void register_streaming_api_handlers(void);

#endif /* API_HANDLERS_STREAMING_H */
