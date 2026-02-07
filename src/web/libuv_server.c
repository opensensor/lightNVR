/**
 * @file libuv_server.c
 * @brief HTTP server implementation using libuv + llhttp
 */

#ifdef HTTP_BACKEND_LIBUV

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <llhttp.h>
#include <uv.h>

#include "utils/memory.h"
#include "web/libuv_server.h"
#include "web/libuv_connection.h"
#include "core/logger.h"

// Initial handler capacity
#define INITIAL_HANDLER_CAPACITY 32

// Forward declarations
static void on_connection(uv_stream_t *server, int status);
static void server_thread_func(void *arg);

/**
 * @brief Initialize libuv server with configuration
 */
static http_server_handle_t libuv_server_init_internal(const http_server_config_t *config,
                                                        uv_loop_t *existing_loop) {
    if (!config) {
        log_error("libuv_server_init: NULL config");
        return NULL;
    }
    
    libuv_server_t *server = safe_calloc(1, sizeof(libuv_server_t));
    if (!server) {
        log_error("libuv_server_init: Failed to allocate server");
        return NULL;
    }
    
    // Copy configuration
    memcpy(&server->config, config, sizeof(http_server_config_t));
    
    // Set up event loop
    if (existing_loop) {
        server->loop = existing_loop;
        server->owns_loop = false;
    } else {
        server->loop = safe_malloc(sizeof(uv_loop_t));
        if (!server->loop) {
            log_error("libuv_server_init: Failed to allocate event loop");
            safe_free(server);
            return NULL;
        }
        if (uv_loop_init(server->loop) != 0) {
            log_error("libuv_server_init: Failed to initialize event loop");
            safe_free(server->loop);
            safe_free(server);
            return NULL;
        }
        server->owns_loop = true;
    }
    
    // Initialize TCP listener
    if (uv_tcp_init(server->loop, &server->listener) != 0) {
        log_error("libuv_server_init: Failed to initialize TCP handle");
        if (server->owns_loop) {
            uv_loop_close(server->loop);
            safe_free(server->loop);
        }
        safe_free(server);
        return NULL;
    }
    
    // Store server pointer in handle data for callback access
    server->listener.data = server;
    
    // Initialize handler registry
    server->handlers = safe_calloc(INITIAL_HANDLER_CAPACITY, sizeof(*server->handlers));
    if (!server->handlers) {
        log_error("libuv_server_init: Failed to allocate handler registry");
        uv_close((uv_handle_t *)&server->listener, NULL);
        if (server->owns_loop) {
            uv_loop_close(server->loop);
            safe_free(server->loop);
        }
        safe_free(server);
        return NULL;
    }
    server->handler_capacity = INITIAL_HANDLER_CAPACITY;
    server->handler_count = 0;
    
    // TLS initialization (if enabled)
    if (config->ssl_enabled) {
        // TODO: Initialize TLS context with config->cert_path, config->key_path
        log_info("libuv_server_init: TLS support requested (not yet implemented)");
        server->tls_ctx = NULL;
    }
    
    log_info("libuv_server_init: Server initialized on port %d", config->port);
    
    // Cast to generic handle type (http_server_t* is compatible pointer)
    return (http_server_handle_t)server;
}

http_server_handle_t libuv_server_init(const http_server_config_t *config) {
    return libuv_server_init_internal(config, NULL);
}

http_server_handle_t libuv_server_init_with_loop(const http_server_config_t *config,
                                                  uv_loop_t *loop) {
    return libuv_server_init_internal(config, loop);
}

uv_loop_t *libuv_server_get_loop(http_server_handle_t handle) {
    libuv_server_t *server = (libuv_server_t *)handle;
    return server ? server->loop : NULL;
}

/**
 * @brief Start the HTTP server
 */
int libuv_server_start(http_server_handle_t handle) {
    libuv_server_t *server = (libuv_server_t *)handle;
    if (!server) {
        return -1;
    }
    
    // Bind to address
    struct sockaddr_in addr;
    uv_ip4_addr("0.0.0.0", server->config.port, &addr);
    
    int r = uv_tcp_bind(&server->listener, (const struct sockaddr *)&addr, 0);
    if (r != 0) {
        log_error("libuv_server_start: Bind failed: %s", uv_strerror(r));
        return -1;
    }
    
    // Start listening
    r = uv_listen((uv_stream_t *)&server->listener, 128, on_connection);
    if (r != 0) {
        log_error("libuv_server_start: Listen failed: %s", uv_strerror(r));
        return -1;
    }
    
    server->running = true;
    log_info("libuv_server_start: Listening on port %d", server->config.port);
    
    // Start event loop in separate thread if we own it
    if (server->owns_loop) {
        r = uv_thread_create(&server->thread, server_thread_func, server);
        if (r != 0) {
            log_error("libuv_server_start: Failed to create server thread");
            return -1;
        }
        server->thread_running = true;
    }

    return 0;
}

/**
 * @brief Server thread function - runs the event loop
 */
static void server_thread_func(void *arg) {
    libuv_server_t *server = (libuv_server_t *)arg;

    log_info("libuv_server: Event loop thread started");

    // Run the event loop until stopped
    while (server->running) {
        uv_run(server->loop, UV_RUN_DEFAULT);
    }

    log_info("libuv_server: Event loop thread exiting");
}

/**
 * @brief Walk callback to close all handles
 */
static void close_walk_cb(uv_handle_t *handle, void *arg) {
    (void)arg;
    if (!uv_is_closing(handle)) {
        log_debug("close_walk_cb: Closing handle type %d", handle->type);
        // Use proper close callback for connection handles
        if (handle->type == UV_TCP && handle->data) {
            uv_close(handle, libuv_close_cb);
        } else {
            uv_close(handle, NULL);
        }
    }
}

/**
 * @brief Stop the HTTP server
 */
void libuv_server_stop(http_server_handle_t handle) {
    libuv_server_t *server = (libuv_server_t *)handle;
    if (!server || !server->running) {
        return;
    }

    log_info("libuv_server_stop: Stopping server");
    server->running = false;
    server->shutting_down = true;  // Signal shutdown to all callbacks

    // Close the listener to stop accepting new connections
    if (!uv_is_closing((uv_handle_t *)&server->listener)) {
        uv_close((uv_handle_t *)&server->listener, NULL);
    }

    // Give pending operations a chance to complete
    log_info("libuv_server_stop: Waiting for pending operations");
    for (int i = 0; i < 10; i++) {
        uv_run(server->loop, UV_RUN_NOWAIT);
        usleep(50000);  // 50ms
    }

    // Walk all handles and close them
    log_info("libuv_server_stop: Closing all active handles");
    uv_walk(server->loop, close_walk_cb, NULL);

    // Run the loop to process all close callbacks
    log_info("libuv_server_stop: Processing close callbacks");
    uv_run(server->loop, UV_RUN_DEFAULT);

    // Stop the event loop
    uv_stop(server->loop);

    // Wait for thread to finish if we started one
    if (server->thread_running) {
        uv_thread_join(&server->thread);
        server->thread_running = false;
    }

    log_info("libuv_server_stop: Server stopped");
}

/**
 * @brief Destroy the HTTP server and free resources
 */
void libuv_server_destroy(http_server_handle_t handle) {
    libuv_server_t *server = (libuv_server_t *)handle;
    if (!server) {
        return;
    }

    // Ensure server is stopped
    if (server->running) {
        libuv_server_stop(handle);
    }

    // Free handler registry
    if (server->handlers) {
        safe_free(server->handlers);
    }

    // Free TLS context if allocated
    if (server->tls_ctx) {
        // TODO: Free TLS context based on SSL library
    }

    // Close and free event loop if we own it
    if (server->owns_loop && server->loop) {
        uv_loop_close(server->loop);
        safe_free(server->loop);
    }

    safe_free(server);
    log_info("libuv_server_destroy: Server destroyed");
}

/**
 * @brief Register a request handler
 */
int libuv_server_register_handler(http_server_handle_t handle, const char *path,
                                   const char *method, request_handler_t handler) {
    libuv_server_t *server = (libuv_server_t *)handle;
    if (!server || !path || !handler) {
        return -1;
    }

    // Expand handler array if needed
    if (server->handler_count >= server->handler_capacity) {
        int new_capacity = server->handler_capacity * 2;
        void *new_handlers = safe_realloc(server->handlers,
                                          new_capacity * sizeof(*server->handlers));
        if (!new_handlers) {
            log_error("libuv_server_register_handler: Failed to expand handler registry");
            return -1;
        }
        server->handlers = new_handlers;
        server->handler_capacity = new_capacity;
    }

    // Add new handler
    int idx = server->handler_count;
    strncpy(server->handlers[idx].path, path, sizeof(server->handlers[idx].path) - 1);
    if (method) {
        strncpy(server->handlers[idx].method, method, sizeof(server->handlers[idx].method) - 1);
    } else {
        server->handlers[idx].method[0] = '\0';  // Match any method
    }
    server->handlers[idx].handler = handler;
    server->handler_count++;

    log_debug("libuv_server_register_handler: Registered %s %s",
              method ? method : "*", path);
    return 0;
}

/**
 * @brief Connection callback - called when a new client connects
 */
static void on_connection(uv_stream_t *listener, int status) {
    if (status < 0) {
        log_error("on_connection: Error: %s", uv_strerror(status));
        return;
    }

    libuv_server_t *server = (libuv_server_t *)listener->data;

    // Create new connection
    libuv_connection_t *conn = libuv_connection_create(server);
    if (!conn) {
        log_error("on_connection: Failed to create connection");
        return;
    }

    // Accept the connection
    int r = uv_accept(listener, (uv_stream_t *)&conn->handle);
    if (r != 0) {
        log_error("on_connection: Accept failed: %s", uv_strerror(r));
        // Must close handle before destroying connection
        libuv_connection_close(conn);
        return;
    }

    // Get client address for logging
    struct sockaddr_storage addr;
    int addr_len = sizeof(addr);
    uv_tcp_getpeername(&conn->handle, (struct sockaddr *)&addr, &addr_len);

    char ip[64] = {0};
    if (addr.ss_family == AF_INET) {
        uv_ip4_name((struct sockaddr_in *)&addr, ip, sizeof(ip));
    } else if (addr.ss_family == AF_INET6) {
        uv_ip6_name((struct sockaddr_in6 *)&addr, ip, sizeof(ip));
    }
    strncpy(conn->request.client_ip, ip, sizeof(conn->request.client_ip) - 1);

    log_debug("on_connection: New connection from %s", ip);

    // Start reading from the connection
    r = uv_read_start((uv_stream_t *)&conn->handle, libuv_alloc_cb, libuv_read_cb);
    if (r != 0) {
        log_error("on_connection: Failed to start reading: %s", uv_strerror(r));
        libuv_connection_close(conn);
    }
}

/**
 * @brief Generic wrapper functions for API compatibility
 *
 * These provide the generic http_server_* API that the rest of the codebase expects,
 * mapping to the libuv-specific implementations.
 */

int http_server_start(http_server_handle_t server) {
    return libuv_server_start(server);
}

void http_server_stop(http_server_handle_t server) {
    libuv_server_stop(server);
}

void http_server_destroy(http_server_handle_t server) {
    libuv_server_destroy(server);
}

int http_server_register_handler(http_server_handle_t server, const char *path,
                                 const char *method, request_handler_t handler) {
    return libuv_server_register_handler(server, path, method, handler);
}

#endif /* HTTP_BACKEND_LIBUV */

