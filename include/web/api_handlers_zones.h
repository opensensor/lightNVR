#ifndef API_HANDLERS_ZONES_H
#define API_HANDLERS_ZONES_H

#include "mongoose.h"

/**
 * Handler for GET /api/streams/:stream_name/zones
 * Returns all detection zones for a stream
 */
void mg_handle_get_zones(struct mg_connection *c, struct mg_http_message *hm);

/**
 * Handler for POST /api/streams/:stream_name/zones
 * Saves detection zones for a stream
 */
void mg_handle_post_zones(struct mg_connection *c, struct mg_http_message *hm);

/**
 * Handler for DELETE /api/streams/:stream_name/zones
 * Deletes all detection zones for a stream
 */
void mg_handle_delete_zones(struct mg_connection *c, struct mg_http_message *hm);

#endif /* API_HANDLERS_ZONES_H */

