/**
 * @file mongoose_server_websocket.h
 * @brief Mongoose WebSocket server implementation
 */

#ifndef MONGOOSE_SERVER_WEBSOCKET_H
#define MONGOOSE_SERVER_WEBSOCKET_H

// Forward declarations for Mongoose structures
struct mg_connection;
struct mg_http_message;
struct mg_ws_message;

/**
 * @brief Handle WebSocket upgrade request
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_websocket_upgrade(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handle WebSocket message
 * 
 * @param c Mongoose connection
 * @param wm Mongoose WebSocket message
 */
void mg_handle_websocket_message(struct mg_connection *c, struct mg_ws_message *wm);

/**
 * @brief Handle WebSocket close
 * 
 * @param c Mongoose connection
 */
void mg_handle_websocket_close(struct mg_connection *c);

/**
 * @brief Register WebSocket handlers
 */
void websocket_register_handlers(void);

#endif /* MONGOOSE_SERVER_WEBSOCKET_H */
