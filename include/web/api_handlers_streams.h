#ifndef API_HANDLERS_STREAMS_H
#define API_HANDLERS_STREAMS_H

#include "web/web_server.h"

/**
 * Handle GET request for streams
 */
void handle_get_streams(const http_request_t *request, http_response_t *response);

/**
 * Handle GET request for a specific stream
 */
void handle_get_stream(const http_request_t *request, http_response_t *response);

/**
 * Handle POST request to create a new stream
 */
void handle_post_stream(const http_request_t *request, http_response_t *response);

/**
 * Handle PUT request to update a stream
 */
void handle_put_stream(const http_request_t *request, http_response_t *response);

/**
 * Handle DELETE request to remove a stream
 */
void handle_delete_stream(const http_request_t *request, http_response_t *response);

/**
 * Handle POST request to test a stream connection
 */
void handle_test_stream(const http_request_t *request, http_response_t *response);

/**
 * Handle POST request to toggle streaming for a stream
 */
void handle_toggle_streaming(const http_request_t *request, http_response_t *response);

#endif /* API_HANDLERS_STREAMS_H */
