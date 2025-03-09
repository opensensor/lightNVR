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

#endif /* API_HANDLERS_SYSTEM_H */
