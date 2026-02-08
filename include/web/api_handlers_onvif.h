#ifndef API_HANDLERS_ONVIF_H
#define API_HANDLERS_ONVIF_H

#include "web/request_response.h"

// Backend-agnostic handlers (for libuv)
/**
 * @brief Backend-agnostic handler for GET /api/onvif/discovery/status
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_get_onvif_discovery_status(const http_request_t *req, http_response_t *res);

/**
 * @brief Backend-agnostic handler for GET /api/onvif/devices
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_get_discovered_onvif_devices(const http_request_t *req, http_response_t *res);

/**
 * @brief Backend-agnostic handler for POST /api/onvif/discovery/discover
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_post_discover_onvif_devices(const http_request_t *req, http_response_t *res);

/**
 * @brief Backend-agnostic handler for GET /api/onvif/device/profiles
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_get_onvif_device_profiles(const http_request_t *req, http_response_t *res);

/**
 * @brief Backend-agnostic handler for POST /api/onvif/device/add
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_post_add_onvif_device_as_stream(const http_request_t *req, http_response_t *res);

/**
 * @brief Backend-agnostic handler for POST /api/onvif/device/test
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_post_test_onvif_connection(const http_request_t *req, http_response_t *res);

#endif /* API_HANDLERS_ONVIF_H */
