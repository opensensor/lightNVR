/**
 * @file api_handlers_ice_servers.h
 * @brief API handlers for ICE server configuration (WebRTC TURN/STUN)
 */

#ifndef API_HANDLERS_ICE_SERVERS_H
#define API_HANDLERS_ICE_SERVERS_H

#include "web/request_response.h"

/**
 * @brief Handler for GET /api/ice-servers
 * Returns ICE server configuration for WebRTC clients (browser)
 */
void handle_get_ice_servers(const http_request_t *req, http_response_t *res);

#endif /* API_HANDLERS_ICE_SERVERS_H */

