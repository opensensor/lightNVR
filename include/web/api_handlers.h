#ifndef API_HANDLERS_H
#define API_HANDLERS_H

#include "web_server.h"

/**
 * Handle GET request for settings
 */
void handle_get_settings(const http_request_t *request, http_response_t *response);

/**
 * Handle POST request for settings
 */
void handle_post_settings(const http_request_t *request, http_response_t *response);

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
 * Handle GET request for system information
 */
void handle_get_system_info(const http_request_t *request, http_response_t *response);

/**
 * Handle GET request for system logs
 */
void handle_get_system_logs(const http_request_t *request, http_response_t *response);

/**
 * Handle GET request for recordings
 */
void handle_get_recordings(const http_request_t *request, http_response_t *response);

/**
 * Handle GET request for a specific recording
 */
void handle_get_recording(const http_request_t *request, http_response_t *response);

/**
 * Handle DELETE request to remove a recording
 */
void handle_delete_recording(const http_request_t *request, http_response_t *response);

/**
 * Handle GET request to download a recording
 */
void handle_download_recording(const http_request_t *request, http_response_t *response);

/**
 * Register API handlers
 */
void register_api_handlers(void);

#endif /* API_HANDLERS_H */
