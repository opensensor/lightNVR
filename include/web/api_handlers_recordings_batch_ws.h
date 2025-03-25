#ifndef API_HANDLERS_RECORDINGS_BATCH_WS_H
#define API_HANDLERS_RECORDINGS_BATCH_WS_H

#include "mongoose.h"

/**
 * @brief WebSocket handler for batch delete recordings
 * 
 * @param client_id WebSocket client ID
 * @param message WebSocket message
 */
void websocket_handle_batch_delete_recordings(const char *client_id, const char *message);

/**
 * @brief HTTP handler for batch delete recordings with WebSocket support
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_batch_delete_recordings_ws(struct mg_connection *c, struct mg_http_message *hm);

#endif /* API_HANDLERS_RECORDINGS_BATCH_WS_H */
