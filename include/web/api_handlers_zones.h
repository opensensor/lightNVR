#ifndef API_HANDLERS_ZONES_H
#define API_HANDLERS_ZONES_H

#include "web/request_response.h"

/**
 * Handler for GET /api/streams/:stream_name/zones
 * Returns all detection zones for a stream
 */
void handle_get_zones(const http_request_t *req, http_response_t *res);

/**
 * Handler for POST /api/streams/:stream_name/zones
 * Saves detection zones for a stream
 */
void handle_post_zones(const http_request_t *req, http_response_t *res);

/**
 * Handler for DELETE /api/streams/:stream_name/zones
 * Deletes all detection zones for a stream
 */
void handle_delete_zones(const http_request_t *req, http_response_t *res);

#endif /* API_HANDLERS_ZONES_H */

