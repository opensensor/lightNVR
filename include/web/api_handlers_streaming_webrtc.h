#ifndef API_HANDLERS_STREAMING_WEBRTC_H
#define API_HANDLERS_STREAMING_WEBRTC_H

#include "web/web_server.h"

/**
 * Handle WebRTC offer request
 */
void handle_webrtc_offer(const http_request_t *request, http_response_t *response);

/**
 * Handle WebRTC ICE request
 */
void handle_webrtc_ice(const http_request_t *request, http_response_t *response);

#endif /* API_HANDLERS_STREAMING_WEBRTC_H */
