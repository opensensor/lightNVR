#ifndef API_HANDLERS_SYSTEM_H
#define API_HANDLERS_SYSTEM_H

#include "web/web_server.h"

/**
 * Handle GET request for system information
 */
void handle_get_system_info(const http_request_t *request, http_response_t *response);

/**
 * Handle GET request for system logs
 */
void handle_get_system_logs(const http_request_t *request, http_response_t *response);

/**
 * Handle POST request to restart the service
 */
void handle_post_system_restart(const http_request_t *request, http_response_t *response);

/**
 * Handle POST request to shutdown the service
 */
void handle_post_system_shutdown(const http_request_t *request, http_response_t *response);

/**
 * Handle POST request to clear system logs
 */
void handle_post_system_clear_logs(const http_request_t *request, http_response_t *response);

/**
 * Handle POST request to backup configuration
 */
void handle_post_system_backup(const http_request_t *request, http_response_t *response);

/**
 * Handle GET request for system status
 */
void handle_get_system_status(const http_request_t *request, http_response_t *response);

#endif /* API_HANDLERS_SYSTEM_H */
