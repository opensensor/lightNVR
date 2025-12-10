#ifndef API_HANDLERS_PTZ_H
#define API_HANDLERS_PTZ_H

#include "../../external/mongoose/mongoose.h"

/**
 * @brief Handle POST request for PTZ continuous move
 * POST /api/streams/{name}/ptz/move
 * Body: { "pan": 0.5, "tilt": 0.0, "zoom": 0.0 }
 */
void mg_handle_ptz_move(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handle POST request for PTZ stop
 * POST /api/streams/{name}/ptz/stop
 */
void mg_handle_ptz_stop(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handle POST request for PTZ absolute move
 * POST /api/streams/{name}/ptz/absolute
 * Body: { "pan": 0.5, "tilt": 0.3, "zoom": 0.0 }
 */
void mg_handle_ptz_absolute(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handle POST request for PTZ relative move
 * POST /api/streams/{name}/ptz/relative
 * Body: { "pan": 0.1, "tilt": -0.1, "zoom": 0.0 }
 */
void mg_handle_ptz_relative(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handle POST request for PTZ go to home position
 * POST /api/streams/{name}/ptz/home
 */
void mg_handle_ptz_home(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handle POST request for PTZ set home position
 * POST /api/streams/{name}/ptz/sethome
 */
void mg_handle_ptz_set_home(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handle GET request for PTZ presets
 * GET /api/streams/{name}/ptz/presets
 */
void mg_handle_ptz_get_presets(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handle POST request for PTZ go to preset
 * POST /api/streams/{name}/ptz/preset
 * Body: { "token": "preset_token" }
 */
void mg_handle_ptz_goto_preset(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handle PUT request for PTZ set preset
 * PUT /api/streams/{name}/ptz/preset
 * Body: { "name": "Preset 1" }
 */
void mg_handle_ptz_set_preset(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handle GET request for PTZ capabilities
 * GET /api/streams/{name}/ptz/capabilities
 */
void mg_handle_ptz_capabilities(struct mg_connection *c, struct mg_http_message *hm);

#endif /* API_HANDLERS_PTZ_H */

