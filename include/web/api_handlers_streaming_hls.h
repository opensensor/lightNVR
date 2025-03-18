#ifndef API_HANDLERS_STREAMING_HLS_H
#define API_HANDLERS_STREAMING_HLS_H

#include "web/web_server.h"

/**
 * Handle HLS manifest request
 */
void handle_hls_manifest(const http_request_t *request, http_response_t *response);

/**
 * Handle HLS segment request
 */
void handle_hls_segment(const http_request_t *request, http_response_t *response);

#endif /* API_HANDLERS_STREAMING_HLS_H */
