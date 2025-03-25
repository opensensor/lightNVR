#ifndef API_HANDLERS_RECORDINGS_PLAYBACK_H
#define API_HANDLERS_RECORDINGS_PLAYBACK_H

#include "mongoose.h"

/**
 * @brief Direct handler for GET /api/recordings/play/:id
 * 
 * @param c Mongoose connection
 * @param hm HTTP message
 */
void mg_handle_play_recording(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for GET /api/recordings/download/:id
 * 
 * @param c Mongoose connection
 * @param hm HTTP message
 */
void mg_handle_download_recording(struct mg_connection *c, struct mg_http_message *hm);

#endif // API_HANDLERS_RECORDINGS_PLAYBACK_H
