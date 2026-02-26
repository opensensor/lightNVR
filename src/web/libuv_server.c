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

#include <pthread.h>

#include "utils/memory.h"
#include "web/libuv_server.h"
#include "web/libuv_connection.h"
#include "web/thumbnail_thread.h"
#include "web/go2rtc_proxy_thread.h"
#include "web/api_handlers_health.h"
#include "core/logger.h"

// Initial handler capacity
#define INITIAL_HANDLER_CAPACITY 32

// Forward declarations
static void on_connection(uv_stream_t *server, int status);
static void server_thread_func(void *arg);
static void stop_async_cb(uv_async_t *handle);
static void count_handles_cb(uv_handle_t *handle, void *arg);

/**
 * @brief Async callback to wake up the event loop for shutdown
 * This is called when uv_async_send() is invoked from another thread
 */
static void stop_async_cb(uv_async_t *handle) {
    libuv_server_t *server = (libuv_server_t *)handle->data;
    if (server) {
        log_debug("stop_async_cb: Received stop signal, closing listener and stopping event loop");

        // BUGFIX: Close the listener handle to allow uv_run() to exit quickly
        // Without this, uv_run(UV_RUN_DEFAULT) will keep running until all handles are closed,
        // which causes the 5-second timeout delay during shutdown
        if (!uv_is_closing((uv_handle_t *)&server->listener)) {
            uv_close((uv_handle_t *)&server->listener, NULL);
        }

        // Stop the event loop - this will cause uv_run() to return after handles are closed
        uv_stop(server->loop);
    }
}

/**
 * @brief Initialize libuv server with configuration
 */
static http_server_handle_t libuv_server_init_internal(const http_server_config_t *config,
                                                        uv_loop_t *existing_loop) {
    if (!config) {
        log_error("libuv_server_init: NULL config");
        return NULL;
    }

    // Set libuv's thread pool size for handler offloading.
    // All HTTP handlers run on the thread pool via uv_queue_work, so the
    // default of 4 threads is too small — slow handlers (ONVIF discovery,
    // recording sync) would starve fast handlers (config reads, stream CRUD).
    // Must be set before the first uv_loop_init / uv_queue_work call.
    // 8 threads is sufficient for typical workloads (was 16, reduced to cut
    // per-thread stack RSS, especially important in memory-constrained pods).
    if (!getenv("UV_THREADPOOL_SIZE")) {
        setenv("UV_THREADPOOL_SIZE", "8", 1);
        log_info("libuv_server_init: Set UV_THREADPOOL_SIZE=8 for handler offloading");
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

    // Initialize async handle for cross-thread stop signaling
    if (uv_async_init(server->loop, &server->stop_async, stop_async_cb) != 0) {
        log_error("libuv_server_init: Failed to initialize async handle");
        uv_close((uv_handle_t *)&server->listener, NULL);
        if (server->owns_loop) {
            uv_loop_close(server->loop);
            safe_free(server->loop);
        }
        safe_free(server);
        return NULL;
    }
    server->stop_async.data = server;

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

    // Initialize thumbnail thread subsystem
    if (thumbnail_thread_init(server->loop) != 0) {
        log_error("libuv_server_init: Failed to initialize thumbnail thread subsystem");
        // Continue anyway - thumbnails will just fail
    }

    // Initialize go2rtc proxy thread subsystem
    if (go2rtc_proxy_thread_init(server->loop) != 0) {
        log_error("libuv_server_init: Failed to initialize go2rtc proxy thread subsystem");
        // Continue anyway - proxy requests will return 503
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
 * @brief Create a fresh event loop, replacing the old dirty one
 *
 * When the event loop thread dies unexpectedly and handles are stuck in a
 * closing state, the only reliable recovery is to abandon the old loop and
 * create a fresh one.
 *
 * @return 0 on success, -1 on failure
 */
static int libuv_server_reset_loop(libuv_server_t *server) {
    log_warn("libuv_server_reset_loop: Abandoning dirty event loop, creating fresh one");

    // Try to close the old loop (may fail if handles are stuck, that's OK)
    int close_result = uv_loop_close(server->loop);
    if (close_result != 0) {
        log_warn("libuv_server_reset_loop: Old loop close returned %d (%s) - "
                 "forcing new allocation", close_result, uv_strerror(close_result));
        // Allocate a new loop structure since we can't cleanly close the old one
        server->loop = safe_malloc(sizeof(uv_loop_t));
        if (!server->loop) {
            log_error("libuv_server_reset_loop: Failed to allocate new event loop");
            return -1;
        }
    }

    // Initialize the fresh loop
    if (uv_loop_init(server->loop) != 0) {
        log_error("libuv_server_reset_loop: Failed to initialize new event loop");
        return -1;
    }

    log_info("libuv_server_reset_loop: Fresh event loop created successfully");
    return 0;
}

/**
 * @brief Start the HTTP server
 */
int libuv_server_start(http_server_handle_t handle) {
    libuv_server_t *server = (libuv_server_t *)handle;
    if (!server) {
        return -1;
    }

    // CRITICAL: Reset shutdown flag so callbacks work properly after restart
    server->shutting_down = false;

    // Check if handles are closed (happens after restart)
    // If they're closed, we need to reinitialize them
    bool need_reinit = uv_is_closing((uv_handle_t *)&server->listener) ||
                       uv_is_closing((uv_handle_t *)&server->stop_async);

    if (need_reinit) {
        log_info("libuv_server_start: Handles are closed, reinitializing");

        // Wait for all closes to complete
        int wait_count = 0;
        while ((uv_is_closing((uv_handle_t *)&server->listener) ||
                uv_is_closing((uv_handle_t *)&server->stop_async)) &&
               wait_count < 100) {
            uv_run(server->loop, UV_RUN_NOWAIT);
            usleep(10000); // 10ms
            wait_count++;
        }

        if (uv_is_closing((uv_handle_t *)&server->listener) ||
            uv_is_closing((uv_handle_t *)&server->stop_async)) {
            // Handles stuck closing - the event loop is dirty.
            // Create a fresh event loop as fallback.
            log_warn("libuv_server_start: Handles still closing after timeout, "
                     "creating fresh event loop");

            if (server->owns_loop) {
                if (libuv_server_reset_loop(server) != 0) {
                    log_error("libuv_server_start: Failed to create fresh event loop");
                    return -1;
                }
            } else {
                log_error("libuv_server_start: Cannot reset shared event loop");
                return -1;
            }
        }

        // Reinitialize the TCP listener on the (possibly fresh) loop
        if (uv_tcp_init(server->loop, &server->listener) != 0) {
            log_error("libuv_server_start: Failed to reinitialize TCP handle");
            return -1;
        }
        server->listener.data = server;

        // Reinitialize the async handle for stop signaling
        if (uv_async_init(server->loop, &server->stop_async, stop_async_cb) != 0) {
            log_error("libuv_server_start: Failed to reinitialize async handle");
            return -1;
        }
        server->stop_async.data = server;

        log_info("libuv_server_start: Handles reinitialized successfully");
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

    // Register this thread's ID with the health check system so it can
    // detect if this thread dies unexpectedly. On Linux, uv_thread_t is
    // pthread_t, so we use pthread_self() for portability.
    set_web_server_thread_id(pthread_self());

    // Run the event loop until stopped.
    // Use UV_RUN_DEFAULT which blocks until there are no more active handles
    // or until uv_stop() is called.
    //
    // If uv_run() returns unexpectedly while server->running is still true,
    // it means handles were unexpectedly closed (e.g., resource exhaustion,
    // handle leak). Rather than letting the thread die (which leaves the
    // server completely unresponsive for 90+ seconds until the health check
    // thread detects the failure and restarts), we attempt to keep the event
    // loop alive by running it again after a brief delay.
    const int MAX_UNEXPECTED_RESTARTS = 5;
    const int RESTART_DELAY_SEC = 2;
    int unexpected_restarts = 0;

    while (server->running) {
        int run_result = uv_run(server->loop, UV_RUN_DEFAULT);

        if (!server->running) {
            log_info("libuv_server: Event loop thread exiting normally (shutdown requested)");
            break;
        }

        // uv_run returned but we didn't request shutdown — unexpected exit
        unexpected_restarts++;
        int active_handles = 0;
        uv_walk(server->loop, count_handles_cb, &active_handles);
        log_error("libuv_server: Event loop exited UNEXPECTEDLY (running=true, "
                  "uv_run returned %d, active_handles=%d, restart_attempt=%d/%d)",
                  run_result, active_handles, unexpected_restarts, MAX_UNEXPECTED_RESTARTS);

        if (unexpected_restarts > MAX_UNEXPECTED_RESTARTS) {
            log_error("libuv_server: Too many unexpected restarts (%d), giving up — "
                      "health check thread will handle recovery",
                      unexpected_restarts);
            break;
        }

        // Brief delay before retrying to avoid a tight spin loop
        log_warn("libuv_server: Retrying uv_run() in %d seconds...", RESTART_DELAY_SEC);
        sleep(RESTART_DELAY_SEC);
    }

    // CRITICAL: Mark thread as no longer running so restart logic works correctly.
    // Without this, libuv_server_stop would try to join a thread that already exited
    // and the restart mechanism would be stuck.
    server->thread_running = false;
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
 * @brief Count active handles callback
 */
static void count_handles_cb(uv_handle_t *handle, void *arg) {
    int *count = (int *)arg;
    if (!uv_is_closing(handle)) {
        (*count)++;
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

    // CRITICAL FIX: We must wait for the event loop thread to exit FIRST
    // before trying to manipulate the loop from the main thread.
    // Running uv_run() from multiple threads is not safe.
    if (server->thread_running) {
        log_info("libuv_server_stop: Signaling event loop thread to stop");

        // Send async signal to wake up the event loop thread
        // This is critical because uv_run(UV_RUN_ONCE) can block indefinitely
        // waiting for I/O, and we need to wake it up so it can check server->running
        uv_async_send(&server->stop_async);

        // Also stop the loop to ensure uv_run returns
        uv_stop(server->loop);

        // Wait for thread with timeout - keep sending signals periodically
        log_info("libuv_server_stop: Waiting for server thread to exit");
        const int max_wait_ms = 5000;  // 5 second maximum wait
        const int check_interval_ms = 100;  // Check every 100ms
        int elapsed_ms = 0;

        while (elapsed_ms < max_wait_ms && server->thread_running) {
            // Keep sending async signals and stop requests
            uv_async_send(&server->stop_async);
            uv_stop(server->loop);

            usleep(check_interval_ms * 1000);
            elapsed_ms += check_interval_ms;
        }

        // Now join the thread - hopefully it has exited
        uv_thread_join(&server->thread);
        server->thread_running = false;
        log_info("libuv_server_stop: Server thread exited after %d ms", elapsed_ms);
    }

    // Now that the event loop thread has stopped, we can safely manipulate the loop

    // Close the listener to stop accepting new connections
    if (!uv_is_closing((uv_handle_t *)&server->listener)) {
        uv_close((uv_handle_t *)&server->listener, NULL);
    }

    // Close the async handle since we're done with it
    if (!uv_is_closing((uv_handle_t *)&server->stop_async)) {
        uv_close((uv_handle_t *)&server->stop_async, NULL);
    }

    // Walk all handles and close them
    log_info("libuv_server_stop: Closing all active handles");
    uv_walk(server->loop, close_walk_cb, NULL);

    // Run the loop to process close callbacks with a timeout
    log_info("libuv_server_stop: Processing close callbacks (with timeout)");

    const int max_wait_ms = 3000;  // 3 second maximum wait for close callbacks
    const int check_interval_ms = 50;  // Check every 50ms
    int elapsed_ms = 0;

    while (elapsed_ms < max_wait_ms) {
        // Run one iteration of the event loop
        int active = uv_run(server->loop, UV_RUN_NOWAIT);

        // If no more active handles, we're done
        if (active == 0) {
            log_info("libuv_server_stop: All handles closed after %d ms", elapsed_ms);
            break;
        }

        // Count remaining active handles for logging
        if (elapsed_ms > 0 && elapsed_ms % 1000 == 0) {
            int handle_count = 0;
            uv_walk(server->loop, count_handles_cb, &handle_count);
            log_info("libuv_server_stop: Still waiting for %d handles after %d ms",
                     handle_count, elapsed_ms);
        }

        usleep(check_interval_ms * 1000);
        elapsed_ms += check_interval_ms;
    }

    if (elapsed_ms >= max_wait_ms) {
        int handle_count = 0;
        uv_walk(server->loop, count_handles_cb, &handle_count);
        log_warn("libuv_server_stop: Timeout waiting for handles to close, %d handles still active",
                 handle_count);
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

    // Shutdown thumbnail thread subsystem
    thumbnail_thread_shutdown();

    // Shutdown go2rtc proxy thread subsystem
    go2rtc_proxy_thread_shutdown();

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

