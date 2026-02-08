#ifndef API_HANDLERS_PTZ_H
#define API_HANDLERS_PTZ_H

#include "web/request_response.h"

/**
 * @brief Handle POST request for PTZ continuous move
 * POST /api/streams/{name}/ptz/move
 * Body: { "pan": 0.5, "tilt": 0.0, "zoom": 0.0 }
 */
void handle_ptz_move(const http_request_t *req, http_response_t *res);

/**
 * @brief Handle POST request for PTZ stop
 * POST /api/streams/{name}/ptz/stop
 */
void handle_ptz_stop(const http_request_t *req, http_response_t *res);

/**
 * @brief Handle POST request for PTZ absolute move
 * POST /api/streams/{name}/ptz/absolute
 * Body: { "pan": 0.5, "tilt": 0.3, "zoom": 0.0 }
 */
void handle_ptz_absolute(const http_request_t *req, http_response_t *res);

/**
 * @brief Handle POST request for PTZ relative move
 * POST /api/streams/{name}/ptz/relative
 * Body: { "pan": 0.1, "tilt": -0.1, "zoom": 0.0 }
 */
void handle_ptz_relative(const http_request_t *req, http_response_t *res);

/**
 * @brief Handle POST request for PTZ go to home position
 * POST /api/streams/{name}/ptz/home
 */
void handle_ptz_home(const http_request_t *req, http_response_t *res);

/**
 * @brief Handle POST request for PTZ set home position
 * POST /api/streams/{name}/ptz/sethome
 */
void handle_ptz_set_home(const http_request_t *req, http_response_t *res);

/**
 * @brief Handle GET request for PTZ presets
 * GET /api/streams/{name}/ptz/presets
 */
void handle_ptz_get_presets(const http_request_t *req, http_response_t *res);

/**
 * @brief Handle POST request for PTZ go to preset
 * POST /api/streams/{name}/ptz/preset
 * Body: { "token": "preset_token" }
 */
void handle_ptz_goto_preset(const http_request_t *req, http_response_t *res);

/**
 * @brief Handle PUT request for PTZ set preset
 * PUT /api/streams/{name}/ptz/preset
 * Body: { "name": "Preset 1" }
 */
void handle_ptz_set_preset(const http_request_t *req, http_response_t *res);

/**
 * @brief Handle GET request for PTZ capabilities
 * GET /api/streams/{name}/ptz/capabilities
 */
void handle_ptz_capabilities(const http_request_t *req, http_response_t *res);

#endif /* API_HANDLERS_PTZ_H */

