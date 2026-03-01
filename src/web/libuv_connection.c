/**
 * @file libuv_connection.c
 * @brief Connection handling and llhttp parsing for libuv HTTP server
 */

#ifdef HTTP_BACKEND_LIBUV

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <sys/stat.h>
#include <llhttp.h>
#include <uv.h>

#include "utils/memory.h"
#include "web/libuv_server.h"
#include "web/libuv_connection.h"
#include "web/go2rtc_proxy_thread.h"
#include "core/logger.h"

// Forward declaration for MIME type helper defined in libuv_file_serve.c
extern const char *libuv_get_mime_type(const char *path);

// Forward declarations for llhttp callbacks
static int on_url(llhttp_t *parser, const char *at, size_t length);
static int on_header_field(llhttp_t *parser, const char *at, size_t length);
static int on_header_value(llhttp_t *parser, const char *at, size_t length);
static int on_headers_complete(llhttp_t *parser);
static int on_body(llhttp_t *parser, const char *at, size_t length);
static int on_message_complete(llhttp_t *parser);

/**
 * @brief Create a new connection
 */
libuv_connection_t *libuv_connection_create(libuv_server_t *server) {
    libuv_connection_t *conn = safe_calloc(1, sizeof(libuv_connection_t));
    if (!conn) {
        log_error("libuv_connection_create: Failed to allocate connection");
        return NULL;
    }
    
    // Initialize TCP handle
    if (uv_tcp_init(server->loop, &conn->handle) != 0) {
        log_error("libuv_connection_create: Failed to init TCP handle");
        safe_free(conn);
        return NULL;
    }

    // Store connection pointer in handle data
    conn->handle.data = conn;
    conn->server = server;

    // Allocate receive buffer
    conn->recv_buffer = safe_malloc(LIBUV_RECV_BUFFER_INITIAL);
    if (!conn->recv_buffer) {
        log_error("libuv_connection_create: Failed to allocate receive buffer");
        // Must use proper close callback to free connection
        uv_close((uv_handle_t *)&conn->handle, libuv_close_cb);
        return NULL;
    }
    conn->recv_buffer_size = LIBUV_RECV_BUFFER_INITIAL;
    conn->recv_buffer_used = 0;
    
    // Initialize llhttp parser
    llhttp_settings_init(&conn->settings);
    conn->settings.on_url = on_url;
    conn->settings.on_header_field = on_header_field;
    conn->settings.on_header_value = on_header_value;
    conn->settings.on_headers_complete = on_headers_complete;
    conn->settings.on_body = on_body;
    conn->settings.on_message_complete = on_message_complete;
    
    llhttp_init(&conn->parser, HTTP_REQUEST, &conn->settings);
    conn->parser.data = conn;
    
    // Initialize request/response
    http_request_init(&conn->request);
    http_response_init(&conn->response);
    
    conn->keep_alive = true;  // HTTP/1.1 default
    
    return conn;
}

/**
 * @brief Reset connection for keep-alive reuse
 */
void libuv_connection_reset(libuv_connection_t *conn) {
    if (!conn) return;

    // Free any allocated response body
    http_response_free(&conn->response);

    // Reset request/response
    http_request_init(&conn->request);
    http_response_init(&conn->response);

    // Reset parser state
    llhttp_reset(&conn->parser);
    conn->headers_complete = false;
    conn->message_complete = false;
    conn->current_header_field[0] = '\0';
    conn->current_header_field_len = 0;

    // Reset buffer usage (but keep the buffer)
    conn->recv_buffer_used = 0;

    // Reset thread pool offloading state
    conn->handler_on_worker = false;
    conn->deferred_file_serve = false;

    // Reset async response state — stale true values here silently drop the
    // response for the next request in handler_after_work_cb.
    conn->async_response_pending = false;
    conn->deferred_action = WRITE_ACTION_NONE;

    conn->requests_handled++;

    // Resume reading — reading was stopped before offloading the handler
    // to the thread pool. We must restart it for the next request.
    if (!uv_is_closing((uv_handle_t *)&conn->handle)) {
        uv_read_start((uv_stream_t *)&conn->handle, libuv_alloc_cb, libuv_read_cb);
    }
}

/**
 * @brief Close a connection gracefully
 */
void libuv_connection_close(libuv_connection_t *conn) {
    if (!conn) return;

    if (!uv_is_closing((uv_handle_t *)&conn->handle)) {
        // Stop reading to prevent callbacks after close starts
        uv_read_stop((uv_stream_t *)&conn->handle);
        uv_close((uv_handle_t *)&conn->handle, libuv_close_cb);
    }
}

/**
 * @brief Destroy a connection (called after close completes)
 */
void libuv_connection_destroy(libuv_connection_t *conn) {
    if (!conn) return;
    
    http_response_free(&conn->response);
    
    if (conn->recv_buffer) {
        safe_free(conn->recv_buffer);
    }
    
    safe_free(conn);
}

/**
 * @brief Handle close callback
 */
void libuv_close_cb(uv_handle_t *handle) {
    libuv_connection_t *conn = (libuv_connection_t *)handle->data;
    if (conn) {
        log_debug("libuv_close_cb: Connection closed after %d requests", 
                  conn->requests_handled);
        libuv_connection_destroy(conn);
    }
}

/**
 * @brief Allocate buffer for reading
 */
void libuv_alloc_cb(uv_handle_t *handle, size_t suggested_size, uv_buf_t *buf) {
    libuv_connection_t *conn = (libuv_connection_t *)handle->data;

    // Safety check - connection might be closing
    if (!conn || !conn->recv_buffer) {
        buf->base = NULL;
        buf->len = 0;
        return;
    }

    // Ensure we have space in the receive buffer
    size_t available = conn->recv_buffer_size - conn->recv_buffer_used;
    if (available < 1024) {
        // Need to expand buffer
        size_t new_size = conn->recv_buffer_size * 2;
        if (new_size > LIBUV_RECV_BUFFER_MAX) {
            new_size = LIBUV_RECV_BUFFER_MAX;
        }
        if (new_size > conn->recv_buffer_size) {
            char *new_buf = safe_realloc(conn->recv_buffer, new_size);
            if (new_buf) {
                conn->recv_buffer = new_buf;
                conn->recv_buffer_size = new_size;
                available = new_size - conn->recv_buffer_used;
            }
        }
    }
    
    buf->base = conn->recv_buffer + conn->recv_buffer_used;
    buf->len = available;
}

/**
 * @brief Read callback - parse incoming HTTP data
 */
void libuv_read_cb(uv_stream_t *stream, ssize_t nread, const uv_buf_t *buf) {
    libuv_connection_t *conn = (libuv_connection_t *)stream->data;

    // Safety check - connection might be closing
    if (!conn) {
        return;
    }

    if (nread < 0) {
        if (nread != UV_EOF) {
            log_debug("libuv_read_cb: Read error: %s", uv_strerror((int)nread));
        }
        libuv_connection_close(conn);
        return;
    }

    if (nread == 0) {
        return;  // EAGAIN, try again later
    }

    conn->recv_buffer_used += nread;

    // Parse the received data
    enum llhttp_errno err = llhttp_execute(&conn->parser, buf->base, nread);

    if (err != HPE_OK) {
        // HPE_PAUSED is expected when we offload a handler to the thread pool.
        // on_message_complete returns HPE_PAUSED to prevent llhttp from parsing
        // pipelined data while the handler runs concurrently on a worker thread.
        // The parser will be resumed when the connection is reset for keep-alive.
        if (err == HPE_PAUSED) {
            return;
        }

        // HPE_CLOSED_CONNECTION is expected when client sends data after Connection: close
        // Just close the connection without sending an error response
        if (err == HPE_CLOSED_CONNECTION) {
            log_debug("libuv_read_cb: Connection closed by parser (expected after Connection: close)");
            libuv_connection_close(conn);
            return;
        }

        log_error("libuv_read_cb: Parse error: %s %s",
                  llhttp_errno_name(err), llhttp_get_error_reason(&conn->parser));

        // Send 400 Bad Request for actual parse errors, then close
        conn->response.status_code = 400;
        http_response_set_json_error(&conn->response, 400, "Bad Request");
        libuv_send_response_ex(conn, &conn->response, WRITE_ACTION_CLOSE);
        return;
    }

    // If message is complete, it's handled in on_message_complete callback
}

// ============================================================================
// llhttp Parser Callbacks
// ============================================================================

/**
 * @brief URL callback - extract path and query string
 */
static int on_url(llhttp_t *parser, const char *at, size_t length) {
    libuv_connection_t *conn = (libuv_connection_t *)parser->data;

    // Copy full URI
    size_t uri_len = length < sizeof(conn->request.uri) - 1 ?
                     length : sizeof(conn->request.uri) - 1;
    memcpy(conn->request.uri, at, uri_len);
    conn->request.uri[uri_len] = '\0';

    // Split path and query string
    char *query = strchr(conn->request.uri, '?');
    if (query) {
        size_t path_len = query - conn->request.uri;
        if (path_len < sizeof(conn->request.path)) {
            memcpy(conn->request.path, conn->request.uri, path_len);
            conn->request.path[path_len] = '\0';
        }
        strncpy(conn->request.query_string, query + 1,
                sizeof(conn->request.query_string) - 1);
    } else {
        strncpy(conn->request.path, conn->request.uri,
                sizeof(conn->request.path) - 1);
        conn->request.query_string[0] = '\0';
    }

    // Get method from parser
    const char *method = llhttp_method_name(llhttp_get_method(parser));
    strncpy(conn->request.method_str, method, sizeof(conn->request.method_str) - 1);

    // Map to enum (using HTTP_METHOD_ prefix to avoid llhttp conflicts)
    if (strcmp(method, "GET") == 0) conn->request.method = HTTP_METHOD_GET;
    else if (strcmp(method, "POST") == 0) conn->request.method = HTTP_METHOD_POST;
    else if (strcmp(method, "PUT") == 0) conn->request.method = HTTP_METHOD_PUT;
    else if (strcmp(method, "DELETE") == 0) conn->request.method = HTTP_METHOD_DELETE;
    else if (strcmp(method, "OPTIONS") == 0) conn->request.method = HTTP_METHOD_OPTIONS;
    else if (strcmp(method, "HEAD") == 0) conn->request.method = HTTP_METHOD_HEAD;
    else if (strcmp(method, "PATCH") == 0) conn->request.method = HTTP_METHOD_PATCH;
    else conn->request.method = HTTP_METHOD_UNKNOWN;

    return 0;
}

/**
 * @brief Header field callback - store current header name
 */
static int on_header_field(llhttp_t *parser, const char *at, size_t length) {
    libuv_connection_t *conn = (libuv_connection_t *)parser->data;

    size_t len = length < sizeof(conn->current_header_field) - 1 ?
                 length : sizeof(conn->current_header_field) - 1;
    memcpy(conn->current_header_field, at, len);
    conn->current_header_field[len] = '\0';
    conn->current_header_field_len = len;

    return 0;
}

/**
 * @brief Header value callback - store header in request
 */
static int on_header_value(llhttp_t *parser, const char *at, size_t length) {
    libuv_connection_t *conn = (libuv_connection_t *)parser->data;

    if (conn->request.num_headers >= MAX_HEADERS) {
        return 0;  // Silently ignore extra headers
    }

    // Add to headers array
    int idx = conn->request.num_headers;
    strncpy(conn->request.headers[idx].name, conn->current_header_field,
            sizeof(conn->request.headers[idx].name) - 1);

    size_t val_len = length < sizeof(conn->request.headers[idx].value) - 1 ?
                     length : sizeof(conn->request.headers[idx].value) - 1;
    memcpy(conn->request.headers[idx].value, at, val_len);
    conn->request.headers[idx].value[val_len] = '\0';
    conn->request.num_headers++;

    // Handle special headers
    if (strcasecmp(conn->current_header_field, "Content-Type") == 0) {
        strncpy(conn->request.content_type, conn->request.headers[idx].value,
                sizeof(conn->request.content_type) - 1);
    } else if (strcasecmp(conn->current_header_field, "Content-Length") == 0) {
        conn->request.content_length = strtoull(conn->request.headers[idx].value, NULL, 10);
    } else if (strcasecmp(conn->current_header_field, "User-Agent") == 0) {
        strncpy(conn->request.user_agent, conn->request.headers[idx].value,
                sizeof(conn->request.user_agent) - 1);
    } else if (strcasecmp(conn->current_header_field, "Connection") == 0) {
        conn->keep_alive = (strcasecmp(conn->request.headers[idx].value, "close") != 0);
    }

    return 0;
}

/**
 * @brief Headers complete callback
 */
static int on_headers_complete(llhttp_t *parser) {
    libuv_connection_t *conn = (libuv_connection_t *)parser->data;
    conn->headers_complete = true;
    return 0;
}

/**
 * @brief Body callback - accumulate request body
 */
static int on_body(llhttp_t *parser, const char *at, size_t length) {
    libuv_connection_t *conn = (libuv_connection_t *)parser->data;

    // For now, just point to the body in the receive buffer
    // (works because we keep the buffer until request is processed)
    if (!conn->request.body) {
        conn->request.body = (void *)at;
        conn->request.body_len = length;
    } else {
        // Continuation of body - update length
        conn->request.body_len += length;
    }

    return 0;
}

/**
 * @brief Message complete callback - dispatch to handler
 */
/**
 * @brief Match a path against a pattern with wildcard support
 *
 * Supports '#' as a single-segment wildcard (matches anything except '/')
 * Examples:
 *   - "/api/streams" matches "/api/streams" exactly
 *   - "/api/streams/#" matches "/api/streams/foo" but not "/api/streams/foo/bar"
 *   - "/api/streams/#/zones" matches "/api/streams/foo/zones"
 *   - "/go2rtc/*" matches "/go2rtc/api/streams", "/go2rtc/api/hls/segment.m4s", etc.
 *
 * @param path The request path to match
 * @param pattern The pattern to match against
 * @return true if path matches pattern, false otherwise
 */
static bool path_matches_pattern(const char *path, const char *pattern) {
    const char *p = path;
    const char *pat = pattern;

    while (*pat) {
        if (*pat == '*') {
            // Glob wildcard - match everything remaining (including slashes)
            return true;
        } else if (*pat == '#') {
            // Wildcard - match any characters except '/'
            pat++; // Move past '#'

            // Skip characters in path until we hit '/' or end
            while (*p && *p != '/') {
                p++;
            }

            // If pattern expects more after wildcard, continue matching
            // Otherwise, we're done
            if (!*pat) {
                // Pattern ends with '#', path should also end here
                return *p == '\0';
            }

            // Pattern continues, path must also continue
            if (!*p) {
                return false;
            }
        } else if (*pat == *p) {
            // Exact character match
            pat++;
            p++;
        } else {
            // Mismatch
            return false;
        }
    }

    // Both should be at end for exact match
    return *p == '\0' && *pat == '\0';
}

// ============================================================================
// Thread pool handler offloading (uv_queue_work)
// ============================================================================

/**
 * @brief Work context for offloading handler execution to libuv's thread pool
 */
typedef struct {
    uv_work_t work;                     // libuv work request (must be first)
    libuv_connection_t *conn;           // Connection being handled
    request_handler_t handler;          // Handler function to call
    write_complete_action_t action;     // Post-response action (keep-alive/close)
} handler_work_t;

/**
 * @brief Worker callback - runs on libuv thread pool
 *
 * Executes the HTTP handler function. All blocking I/O (database queries,
 * network calls, filesystem scans) happens here instead of on the event loop.
 */
static void handler_work_cb(uv_work_t *req) {
    handler_work_t *hw = (handler_work_t *)req->data;
    hw->handler(&hw->conn->request, &hw->conn->response);
}

/**
 * @brief After-work callback - runs on the event loop thread
 *
 * Sends the response back to the client. If the handler requested deferred
 * file serving (because libuv_serve_file must be called from the loop thread),
 * it initiates async file serving here.
 */
static void handler_after_work_cb(uv_work_t *req, int status) {
    handler_work_t *hw = (handler_work_t *)req->data;
    libuv_connection_t *conn = hw->conn;
    write_complete_action_t action = hw->action;
    safe_free(hw);

    conn->handler_on_worker = false;

    // If cancelled (e.g., server shutting down), close connection
    if (status == UV_ECANCELED) {
        log_debug("handler_after_work_cb: Work cancelled, closing connection");
        libuv_connection_close(conn);
        return;
    }

    // Check if handler requested deferred file serving
    // (http_serve_file was called from worker thread and deferred the actual
    //  libuv_serve_file call to here, since it must run on the loop thread)
    if (conn->deferred_file_serve) {
        conn->deferred_file_serve = false;
        const char *ct = conn->deferred_content_type[0] ? conn->deferred_content_type : NULL;
        const char *eh = conn->deferred_extra_headers[0] ? conn->deferred_extra_headers : NULL;

        if (libuv_serve_file(conn, conn->deferred_file_path, ct, eh) == 0) {
            // File serving started — it manages its own response and connection lifecycle
            return;
        }
        // File serving failed — fall through to send error response
        log_error("handler_after_work_cb: Deferred file serve failed for %s",
                  conn->deferred_file_path);
        http_response_set_json_error(&conn->response, 500, "Failed to serve file");
    }

    // Check if handler initiated async response directly (shouldn't happen
    // from worker thread, but handle it defensively)
    if (conn->async_response_pending) {
        log_debug("handler_after_work_cb: Async response pending, skipping response send");
        return;
    }

    // Send the response — write callback handles keep-alive/close
    libuv_send_response_ex(conn, &conn->response, action);
}

/**
 * @brief Static file resolution handler — runs on the libuv thread pool.
 *
 * Performs blocking stat() calls to resolve the requested static file.
 * Sets conn->deferred_file_serve so handler_after_work_cb can call
 * libuv_serve_file() from the event-loop thread (required by libuv async I/O).
 */
static void static_file_resolve_handler(const http_request_t *req, http_response_t *res) {
    libuv_connection_t *conn = (libuv_connection_t *)req->user_data;
    libuv_server_t *server = conn->server;

    char file_path[1024];
    snprintf(file_path, sizeof(file_path), "%s%s", server->config.web_root, req->path);

    struct stat st;
    if (stat(file_path, &st) == 0) {
        if (S_ISDIR(st.st_mode)) {
            snprintf(file_path, sizeof(file_path), "%s%s/index.html",
                     server->config.web_root, req->path);
            if (stat(file_path, &st) != 0) {
                log_debug("Directory index not found: %s", file_path);
                http_response_set_json_error(res, 404, "Directory index not found");
                return;
            }
        }
        log_debug("Serving static file: %s", file_path);
        strncpy(conn->deferred_file_path, file_path, sizeof(conn->deferred_file_path) - 1);
        conn->deferred_file_path[sizeof(conn->deferred_file_path) - 1] = '\0';
        conn->deferred_content_type[0] = '\0';
        conn->deferred_extra_headers[0] = '\0';
        conn->deferred_file_serve = true;
    } else {
        char gz_path[1080];
        snprintf(gz_path, sizeof(gz_path), "%s.gz", file_path);
        if (stat(gz_path, &st) == 0 && S_ISREG(st.st_mode)) {
            log_debug("Serving gzip static file: %s", gz_path);
            const char *mime_type = libuv_get_mime_type(file_path);
            strncpy(conn->deferred_file_path, gz_path, sizeof(conn->deferred_file_path) - 1);
            conn->deferred_file_path[sizeof(conn->deferred_file_path) - 1] = '\0';
            strncpy(conn->deferred_content_type, mime_type,
                    sizeof(conn->deferred_content_type) - 1);
            conn->deferred_content_type[sizeof(conn->deferred_content_type) - 1] = '\0';
            strncpy(conn->deferred_extra_headers, "Content-Encoding: gzip\r\n",
                    sizeof(conn->deferred_extra_headers) - 1);
            conn->deferred_extra_headers[sizeof(conn->deferred_extra_headers) - 1] = '\0';
            conn->deferred_file_serve = true;
        } else {
            log_debug("Static file not found: %s", file_path);
            http_response_set_json_error(res, 404, "Not Found");
        }
    }
}

static int on_message_complete(llhttp_t *parser) {
    libuv_connection_t *conn = (libuv_connection_t *)parser->data;
    conn->message_complete = true;

    // Find matching handler
    libuv_server_t *server = conn->server;
    request_handler_t handler = NULL;

    for (int i = 0; i < server->handler_count; i++) {
        // Check method match (empty = any)
        if (server->handlers[i].method[0] != '\0') {
            if (strcmp(server->handlers[i].method, conn->request.method_str) != 0) {
                continue;
            }
        }

        // Check path match with wildcard support
        if (path_matches_pattern(conn->request.path, server->handlers[i].path)) {
            handler = server->handlers[i].handler;
            break;
        }
    }

    // Determine post-response action: keep-alive reset or close.
    // This decision is deferred to the write completion callback to avoid
    // closing/resetting the connection before the async write finishes.
    write_complete_action_t action =
        (conn->keep_alive && llhttp_should_keep_alive(parser))
            ? WRITE_ACTION_KEEP_ALIVE
            : WRITE_ACTION_CLOSE;

    // Set user_data to point to connection (needed for file serving and proxy)
    conn->request.user_data = conn;

    // Go2rtc proxy paths use dedicated detached threads to avoid starving the
    // shared libuv thread pool with 30-second blocking curl calls.
    if (go2rtc_proxy_path_matches(conn->request.path)) {
        uv_read_stop((uv_stream_t *)&conn->handle);
        if (go2rtc_proxy_thread_submit(conn, action) == 0) {
            return HPE_PAUSED;
        }
        // Submit failed — fall back to a 503 error response
        log_error("on_message_complete: go2rtc proxy thread submit failed for %s",
                  conn->request.path);
        http_response_set_json_error(&conn->response, 503, "Service temporarily unavailable");
        libuv_send_response_ex(conn, &conn->response, action);
        return HPE_OK;
    }

    if (handler) {
        // Stop reading to prevent new data from arriving while the
        // handler runs on the thread pool
        uv_read_stop((uv_stream_t *)&conn->handle);

        // Offload handler execution to libuv's thread pool
        handler_work_t *hw = safe_malloc(sizeof(handler_work_t));
        if (!hw) {
            log_error("on_message_complete: Failed to allocate handler work context");
            http_response_set_json_error(&conn->response, 500, "Internal Server Error");
            libuv_send_response_ex(conn, &conn->response, action);
            return HPE_OK;
        }

        hw->work.data = hw;
        hw->conn = conn;
        hw->handler = handler;
        hw->action = action;
        conn->handler_on_worker = true;
        conn->deferred_action = action;  // Store action for async responses

        int r = uv_queue_work(server->loop, &hw->work,
                              handler_work_cb, handler_after_work_cb);
        if (r != 0) {
            log_error("on_message_complete: uv_queue_work failed: %s", uv_strerror(r));
            conn->handler_on_worker = false;
            safe_free(hw);
            http_response_set_json_error(&conn->response, 500, "Internal Server Error");
            libuv_send_response_ex(conn, &conn->response, action);
            return HPE_OK;
        }

        // Pause the parser to prevent llhttp from continuing to parse
        // any pipelined data in the receive buffer. This is critical:
        // on_message_complete is called *during* llhttp_execute(), and
        // without pausing, llhttp would continue parsing the next request
        // and overwrite conn->request while the worker thread reads it.
        // The parser will be resumed in handler_after_work_cb via
        // libuv_connection_reset (which calls llhttp_reset).
        return HPE_PAUSED;
    } else {
        // No handler matched — offload static file resolution to the thread pool
        // to avoid blocking stat() calls on the event-loop thread.
        uv_read_stop((uv_stream_t *)&conn->handle);

        handler_work_t *hw = safe_malloc(sizeof(handler_work_t));
        if (!hw) {
            log_error("on_message_complete: Failed to allocate static file work context");
            http_response_set_json_error(&conn->response, 500, "Internal Server Error");
            libuv_send_response_ex(conn, &conn->response, action);
            return HPE_OK;
        }

        hw->work.data = hw;
        hw->conn = conn;
        hw->handler = static_file_resolve_handler;
        hw->action = action;
        conn->handler_on_worker = true;
        conn->deferred_action = action;

        int r = uv_queue_work(server->loop, &hw->work,
                              handler_work_cb, handler_after_work_cb);
        if (r != 0) {
            log_error("on_message_complete: uv_queue_work failed for static file: %s",
                      uv_strerror(r));
            conn->handler_on_worker = false;
            safe_free(hw);
            http_response_set_json_error(&conn->response, 500, "Internal Server Error");
            libuv_send_response_ex(conn, &conn->response, action);
            return HPE_OK;
        }

        return HPE_PAUSED;
    }
}

#endif /* HTTP_BACKEND_LIBUV */

