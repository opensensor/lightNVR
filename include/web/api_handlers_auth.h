/**
 * @file api_handlers_auth.h
 * @brief Authentication API handlers
 */

#ifndef API_HANDLERS_AUTH_H
#define API_HANDLERS_AUTH_H

// Forward declarations for Mongoose structures
struct mg_connection;
struct mg_http_message;

/**
 * @brief Handle POST /api/auth/login
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_auth_login(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handle POST /api/auth/logout
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_auth_logout(struct mg_connection *c, struct mg_http_message *hm);

#endif /* API_HANDLERS_AUTH_H */
