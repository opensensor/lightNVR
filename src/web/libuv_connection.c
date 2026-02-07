/**
 * @file libuv_connection.c
 * @brief Connection handling and llhttp parsing for libuv HTTP server
 */

#ifdef HTTP_BACKEND_LIBUV

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <uv.h>
#include <llhttp.h>

#include "web/libuv_server.h"
#include "web/libuv_connection.h"
#include "core/logger.h"
#include "utils/memory.h"

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
        uv_close((uv_handle_t *)&conn->handle, NULL);
        safe_free(conn);
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
    
    conn->requests_handled++;
}

/**
 * @brief Close a connection gracefully
 */
void libuv_connection_close(libuv_connection_t *conn) {
    if (!conn) return;
    
    if (!uv_is_closing((uv_handle_t *)&conn->handle)) {
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

    if (nread < 0) {
        if (nread != UV_EOF) {
            log_debug("libuv_read_cb: Read error: %s", uv_strerror(nread));
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
        log_error("libuv_read_cb: Parse error: %s %s",
                  llhttp_errno_name(err), llhttp_get_error_reason(&conn->parser));

        // Send 400 Bad Request
        conn->response.status_code = 400;
        http_response_set_json_error(&conn->response, 400, "Bad Request");
        libuv_send_response(conn, &conn->response);
        libuv_connection_close(conn);
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

    // Map to enum
    if (strcmp(method, "GET") == 0) conn->request.method = HTTP_GET;
    else if (strcmp(method, "POST") == 0) conn->request.method = HTTP_POST;
    else if (strcmp(method, "PUT") == 0) conn->request.method = HTTP_PUT;
    else if (strcmp(method, "DELETE") == 0) conn->request.method = HTTP_DELETE;
    else if (strcmp(method, "OPTIONS") == 0) conn->request.method = HTTP_OPTIONS;
    else if (strcmp(method, "HEAD") == 0) conn->request.method = HTTP_HEAD;

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

        // Check path match (simple prefix match for now)
        // TODO: Implement proper pattern matching with wildcards
        if (strncmp(conn->request.path, server->handlers[i].path,
                    strlen(server->handlers[i].path)) == 0) {
            handler = server->handlers[i].handler;
            break;
        }
    }

    // Set user_data to point to server
    conn->request.user_data = server;

    if (handler) {
        // Call handler
        handler(&conn->request, &conn->response);
    } else {
        // 404 Not Found
        http_response_set_json_error(&conn->response, 404, "Not Found");
    }

    // Send response
    libuv_send_response(conn, &conn->response);

    // Handle keep-alive
    if (conn->keep_alive && llhttp_should_keep_alive(parser)) {
        libuv_connection_reset(conn);
    } else {
        libuv_connection_close(conn);
    }

    return 0;
}

#endif /* HTTP_BACKEND_LIBUV */

