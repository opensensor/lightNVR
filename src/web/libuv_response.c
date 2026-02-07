/**
 * @file libuv_response.c
 * @brief HTTP response serialization for libuv server
 */

#ifdef HTTP_BACKEND_LIBUV

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <llhttp.h>
#include <uv.h>

#include "utils/memory.h"
#include "web/libuv_server.h"
#include "web/libuv_connection.h"
#include "core/logger.h"

// HTTP status reason phrases
static const char *get_status_phrase(int status_code) {
    switch (status_code) {
        case 200: return "OK";
        case 201: return "Created";
        case 204: return "No Content";
        case 206: return "Partial Content";
        case 301: return "Moved Permanently";
        case 302: return "Found";
        case 304: return "Not Modified";
        case 400: return "Bad Request";
        case 401: return "Unauthorized";
        case 403: return "Forbidden";
        case 404: return "Not Found";
        case 405: return "Method Not Allowed";
        case 409: return "Conflict";
        case 413: return "Payload Too Large";
        case 415: return "Unsupported Media Type";
        case 500: return "Internal Server Error";
        case 501: return "Not Implemented";
        case 503: return "Service Unavailable";
        default: return "Unknown";
    }
}

/**
 * @brief Write complete callback
 */
void libuv_write_cb(uv_write_t *req, int status) {
    libuv_write_ctx_t *ctx = (libuv_write_ctx_t *)req;
    
    if (status < 0) {
        log_error("libuv_write_cb: Write error: %s", uv_strerror(status));
    }
    
    // Free the buffer if requested
    if (ctx->free_buffer && ctx->buf.base) {
        safe_free(ctx->buf.base);
    }
    
    safe_free(ctx);
}

/**
 * @brief Send raw data on connection
 */
int libuv_connection_send(libuv_connection_t *conn, char *data, size_t len,
                          bool free_after_send) {
    if (!conn || !data || len == 0) {
        if (free_after_send && data) {
            safe_free(data);
        }
        return -1;
    }

    // Check if connection is closing
    if (uv_is_closing((uv_handle_t *)&conn->handle)) {
        log_debug("libuv_connection_send: Connection is closing, discarding data");
        if (free_after_send) {
            safe_free(data);
        }
        return -1;
    }

    libuv_write_ctx_t *ctx = safe_malloc(sizeof(libuv_write_ctx_t));
    if (!ctx) {
        if (free_after_send) {
            safe_free(data);
        }
        return -1;
    }

    ctx->conn = conn;
    ctx->buf = uv_buf_init(data, len);
    ctx->free_buffer = free_after_send;

    int r = uv_write(&ctx->req, (uv_stream_t *)&conn->handle, &ctx->buf, 1, libuv_write_cb);
    if (r != 0) {
        log_error("libuv_connection_send: Write failed: %s", uv_strerror(r));
        if (free_after_send) {
            safe_free(data);
        }
        safe_free(ctx);
        return -1;
    }

    return 0;
}

/**
 * @brief Send an HTTP response
 */
int libuv_send_response(libuv_connection_t *conn, const http_response_t *response) {
    if (!conn || !response) {
        return -1;
    }
    
    // Calculate required buffer size
    size_t headers_size = 256;  // Base for status line
    for (int i = 0; i < response->num_headers; i++) {
        headers_size += strlen(response->headers[i].name) + 
                        strlen(response->headers[i].value) + 4;  // ": " + "\r\n"
    }
    if (response->content_type[0]) {
        headers_size += 64;  // Content-Type header
    }
    headers_size += 64;  // Content-Length header
    headers_size += 4;   // Final "\r\n"
    
    size_t total_size = headers_size + response->body_length;
    
    char *buffer = safe_malloc(total_size);
    if (!buffer) {
        log_error("libuv_send_response: Failed to allocate response buffer");
        return -1;
    }
    
    // Build response
    int offset = 0;
    
    // Status line
    offset += snprintf(buffer + offset, total_size - offset, 
                       "HTTP/1.1 %d %s\r\n",
                       response->status_code, 
                       get_status_phrase(response->status_code));
    
    // Content-Type
    if (response->content_type[0]) {
        offset += snprintf(buffer + offset, total_size - offset,
                           "Content-Type: %s\r\n", response->content_type);
    }
    
    // Content-Length
    offset += snprintf(buffer + offset, total_size - offset,
                       "Content-Length: %zu\r\n", response->body_length);
    
    // Custom headers
    for (int i = 0; i < response->num_headers; i++) {
        offset += snprintf(buffer + offset, total_size - offset,
                           "%s: %s\r\n",
                           response->headers[i].name,
                           response->headers[i].value);
    }
    
    // End of headers
    offset += snprintf(buffer + offset, total_size - offset, "\r\n");
    
    // Body
    if (response->body && response->body_length > 0) {
        memcpy(buffer + offset, response->body, response->body_length);
        offset += response->body_length;
    }
    
    log_debug("libuv_send_response: Sending %d %s (%zu bytes)",
              response->status_code, get_status_phrase(response->status_code),
              response->body_length);
    
    // Send the response (buffer will be freed after send)
    return libuv_connection_send(conn, buffer, offset, true);
}

#endif /* HTTP_BACKEND_LIBUV */

