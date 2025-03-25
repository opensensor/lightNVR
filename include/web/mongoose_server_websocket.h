/**
 * @file mongoose_server_websocket.h
 * @brief Mongoose WebSocket server implementation
 */

#ifndef MONGOOSE_SERVER_WEBSOCKET_H
#define MONGOOSE_SERVER_WEBSOCKET_H

// Forward declarations for Mongoose structures
struct mg_connection;
struct mg_http_message;

/**
 * @brief Handle WebSocket upgrade request
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_websocket_upgrade(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Register WebSocket handlers
 */
void websocket_register_handlers(void);

#endif /* MONGOOSE_SERVER_WEBSOCKET_H */
