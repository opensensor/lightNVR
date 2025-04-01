/**
 * @file http_server.h
 * @brief HTTP server implementation using Mongoose
 */

#ifndef HTTP_SERVER_H
#define HTTP_SERVER_H

#include <stdint.h>
#include <stdbool.h>
#include <time.h>
#include "request_response.h"

// Forward declaration of Mongoose structures
struct mg_mgr;
struct mg_connection;
struct mg_http_message;

// Forward declarations

/**
 * @brief HTTP server configuration
 */
typedef struct {
    int port;                       // Server port
    const char *web_root;           // Web root directory
    bool auth_enabled;              // Authentication enabled
    char username[32];              // Authentication username
    char password[32];              // Authentication password
    bool cors_enabled;              // CORS enabled
    char allowed_origins[256];      // CORS allowed origins
    char allowed_methods[256];      // CORS allowed methods
    char allowed_headers[256];      // CORS allowed headers
    bool ssl_enabled;               // SSL/TLS enabled
    char cert_path[256];            // SSL/TLS certificate path
    char key_path[256];             // SSL/TLS key path
    int max_connections;            // Maximum number of connections
    int connection_timeout;         // Connection timeout in seconds
    bool daemon_mode;               // Daemon mode
    char pid_file[256];             // PID file path
} http_server_config_t;

/**
 * @brief HTTP server structure
 */
typedef struct http_server {
    struct mg_mgr *mgr;             // Mongoose event manager
    http_server_config_t config;    // Server configuration
    bool running;                   // Server running flag
    
    // Handler registry
    struct {
        char path[256];             // Request path
        char method[16];            // HTTP method
        request_handler_t handler;  // Request handler function
    } *handlers;                    // Array of handlers
    int handler_count;              // Number of handlers
    int handler_capacity;           // Capacity of handlers array
} http_server_t;

/**
 * @brief HTTP server handle
 */
typedef http_server_t* http_server_handle_t;

/**
 * @brief Initialize HTTP server
 * 
 * @param config Server configuration
 * @return http_server_handle_t Server handle or NULL on error
 */
http_server_handle_t http_server_init(const http_server_config_t *config);

/**
 * @brief Start HTTP server
 * 
 * @param server Server handle
 * @return int 0 on success, non-zero on error
 */
int http_server_start(http_server_handle_t server);

/**
 * @brief Stop HTTP server
 * 
 * @param server Server handle
 */
void http_server_stop(http_server_handle_t server);

/**
 * @brief Destroy HTTP server
 * 
 * @param server Server handle
 */
void http_server_destroy(http_server_handle_t server);

/**
 * @brief Register request handler
 * 
 * @param server Server handle
 * @param path Request path
 * @param method HTTP method or NULL for any method
 * @param handler Request handler function
 * @return int 0 on success, non-zero on error
 */
int http_server_register_handler(http_server_handle_t server, const char *path, 
                                const char *method, request_handler_t handler);

/**
 * @brief Get server statistics
 * 
 * Note: This function is kept for API compatibility but no longer tracks statistics
 * 
 * @param server Server handle
 * @param active_connections Number of active connections
 * @param requests_per_second Requests per second
 * @param bytes_sent Bytes sent
 * @param bytes_received Bytes received
 * @return int 0 on success, non-zero on error
 */
int http_server_get_stats(http_server_handle_t server, int *active_connections, 
                         double *requests_per_second, uint64_t *bytes_sent, 
                         uint64_t *bytes_received);

/**
 * @brief Set authentication settings
 * 
 * @param server Server handle
 * @param enabled Authentication enabled
 * @param username Username
 * @param password Password
 * @return int 0 on success, non-zero on error
 */
int http_server_set_authentication(http_server_handle_t server, bool enabled, 
                                  const char *username, const char *password);

/**
 * @brief Set CORS settings
 * 
 * @param server Server handle
 * @param enabled CORS enabled
 * @param allowed_origins Allowed origins
 * @param allowed_methods Allowed methods
 * @param allowed_headers Allowed headers
 * @return int 0 on success, non-zero on error
 */
int http_server_set_cors(http_server_handle_t server, bool enabled, 
                        const char *allowed_origins, const char *allowed_methods, 
                        const char *allowed_headers);

/**
 * @brief Set SSL/TLS settings
 * 
 * @param server Server handle
 * @param enabled SSL/TLS enabled
 * @param cert_path Certificate path
 * @param key_path Key path
 * @return int 0 on success, non-zero on error
 */
int http_server_set_ssl(http_server_handle_t server, bool enabled, 
                       const char *cert_path, const char *key_path);

/**
 * @brief Set maximum connections
 * 
 * @param server Server handle
 * @param max_connections Maximum number of connections
 * @return int 0 on success, non-zero on error
 */
int http_server_set_max_connections(http_server_handle_t server, int max_connections);

/**
 * @brief Set connection timeout
 * 
 * @param server Server handle
 * @param timeout_seconds Connection timeout in seconds
 * @return int 0 on success, non-zero on error
 */
int http_server_set_connection_timeout(http_server_handle_t server, int timeout_seconds);

/**
 * @brief Convert Mongoose HTTP message to HTTP request
 * 
 * @param conn Mongoose connection
 * @param msg Mongoose HTTP message
 * @param request HTTP request to fill
 * @return int 0 on success, non-zero on error
 */
int http_server_mg_to_request(struct mg_connection *conn, struct mg_http_message *msg, 
                             http_request_t *request);

/**
 * @brief Send HTTP response using Mongoose
 * 
 * @param conn Mongoose connection
 * @param response HTTP response
 * @return int 0 on success, non-zero on error
 */
int http_server_send_response(struct mg_connection *conn, const http_response_t *response);

#endif /* HTTP_SERVER_H */
