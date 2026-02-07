/**
 * @file libuv_server.h
 * @brief HTTP server implementation using libuv + llhttp
 * 
 * This provides an alternative HTTP backend to Mongoose, using libuv for
 * async I/O and llhttp for HTTP parsing. It implements the same
 * http_server_handle_t interface for drop-in replacement.
 * 
 * Enable with: cmake -DHTTP_BACKEND=libuv
 */

#ifndef LIBUV_SERVER_H
#define LIBUV_SERVER_H

#ifdef HTTP_BACKEND_LIBUV

#include <llhttp.h>
#include <uv.h>
#include "web/http_server.h"
#include "web/request_response.h"

/**
 * @brief libuv server internal structure
 */
typedef struct libuv_server {
    uv_loop_t *loop;                    // Event loop (owned or shared)
    uv_tcp_t listener;                  // TCP listener handle
    http_server_config_t config;        // Server configuration (copied)
    bool running;                       // Server running flag
    bool owns_loop;                     // Whether we own the event loop
    bool shutting_down;                 // Server is shutting down

    // Handler registry (same structure as http_server_t)
    struct {
        char path[256];                 // Request path pattern
        char method[16];                // HTTP method or empty for any
        request_handler_t handler;      // Request handler function
    } *handlers;
    int handler_count;                  // Number of registered handlers
    int handler_capacity;               // Capacity of handlers array

    // TLS context (optional, NULL if TLS disabled)
    void *tls_ctx;

    // Server thread (for blocking start)
    uv_thread_t thread;
    bool thread_running;
} libuv_server_t;

/**
 * @brief Connection state for each client
 */
typedef struct libuv_connection {
    uv_tcp_t handle;                    // TCP handle (must be first for casting)
    uv_buf_t read_buf;                  // Current read buffer
    char *recv_buffer;                  // Accumulated receive buffer
    size_t recv_buffer_size;            // Size of receive buffer
    size_t recv_buffer_used;            // Bytes used in receive buffer
    
    llhttp_t parser;                    // HTTP parser instance
    llhttp_settings_t settings;         // Parser callbacks
    
    http_request_t request;             // Parsed request
    http_response_t response;           // Response being built
    
    libuv_server_t *server;             // Back-pointer to server
    
    // TLS session (optional, NULL if TLS disabled)
    void *tls_session;
    
    // Parser state
    bool headers_complete;              // Headers fully parsed
    bool message_complete;              // Full message received
    char current_header_field[128];     // Current header name being parsed
    size_t current_header_field_len;    // Length of current header name
    
    // Keep-alive support
    bool keep_alive;                    // Connection should be kept alive
    int requests_handled;               // Number of requests on this connection
} libuv_connection_t;

/**
 * @brief Initialize HTTP server using libuv + llhttp
 * 
 * @param config Server configuration
 * @return http_server_handle_t Server handle or NULL on error
 */
http_server_handle_t libuv_server_init(const http_server_config_t *config);

/**
 * @brief Initialize HTTP server with an existing event loop
 * 
 * Use this when you want to share the event loop with other subsystems
 * (e.g., RTSP handling, file I/O, timers).
 * 
 * @param config Server configuration
 * @param loop Existing uv_loop_t to use (not owned, not freed on destroy)
 * @return http_server_handle_t Server handle or NULL on error
 */
http_server_handle_t libuv_server_init_with_loop(const http_server_config_t *config, 
                                                  uv_loop_t *loop);

/**
 * @brief Get the event loop from a libuv server
 * 
 * Useful for integrating other libuv-based subsystems.
 * 
 * @param server Server handle
 * @return uv_loop_t* Event loop or NULL if not a libuv server
 */
uv_loop_t *libuv_server_get_loop(http_server_handle_t server);

/**
 * @brief Serve a file asynchronously
 * 
 * Uses libuv's async file I/O for non-blocking file serving.
 * Supports Range requests for video seeking.
 * 
 * @param conn Connection to send file on
 * @param path File path
 * @param content_type MIME type (or NULL for auto-detection)
 * @param extra_headers Additional headers to include (or NULL)
 * @return int 0 on success, -1 on error
 */
int libuv_serve_file(libuv_connection_t *conn, const char *path,
                     const char *content_type, const char *extra_headers);

/**
 * @brief Send an HTTP response on a libuv connection
 * 
 * Serializes http_response_t to wire format and writes to the connection.
 * 
 * @param conn Connection to send response on
 * @param response HTTP response to send
 * @return int 0 on success, -1 on error
 */
int libuv_send_response(libuv_connection_t *conn, const http_response_t *response);

/**
 * @brief Close a connection gracefully
 * 
 * @param conn Connection to close
 */
void libuv_connection_close(libuv_connection_t *conn);

#endif /* HTTP_BACKEND_LIBUV */

#endif /* LIBUV_SERVER_H */

