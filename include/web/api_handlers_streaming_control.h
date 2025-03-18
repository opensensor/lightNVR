#ifndef API_HANDLERS_STREAMING_CONTROL_H
#define API_HANDLERS_STREAMING_CONTROL_H

#include "web/web_server.h"

/**
 * Handle stream toggle request
 * Enables or disables streaming for a specific stream
 */
void handle_stream_toggle(const http_request_t *request, http_response_t *response);

#endif /* API_HANDLERS_STREAMING_CONTROL_H */
