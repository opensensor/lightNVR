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
    libuv_connection_t *conn = ctx->conn;
    write_complete_action_t action = ctx->action;

    if (status < 0) {
        log_error("libuv_write_cb: Write error: %s", uv_strerror(status));
        // On write error, always close the connection
        action = WRITE_ACTION_CLOSE;
    }

    // Free the buffer if requested
    if (ctx->free_buffer && ctx->buf.base) {
        safe_free(ctx->buf.base);
    }

    safe_free(ctx);

    // Perform post-write action
    if (conn) {
        switch (action) {
            case WRITE_ACTION_KEEP_ALIVE:
                libuv_connection_reset(conn);
                break;
            case WRITE_ACTION_CLOSE:
                libuv_connection_close(conn);
                break;
            case WRITE_ACTION_NONE:
            default:
                break;
        }
    }
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
    ctx->action = WRITE_ACTION_NONE;

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
 * @brief Send raw data on connection with post-write action
 */
int libuv_connection_send_ex(libuv_connection_t *conn, char *data, size_t len,
                             bool free_after_send, write_complete_action_t action) {
    if (!conn || !data || len == 0) {
        if (free_after_send && data) {
            safe_free(data);
        }
        return -1;
    }

    // Check if connection is closing
    if (uv_is_closing((uv_handle_t *)&conn->handle)) {
        log_debug("libuv_connection_send_ex: Connection is closing, discarding data");
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
    ctx->action = action;

    int r = uv_write(&ctx->req, (uv_stream_t *)&conn->handle, &ctx->buf, 1, libuv_write_cb);
    if (r != 0) {
        log_error("libuv_connection_send_ex: Write failed: %s", uv_strerror(r));
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
    
    // Build response with proper overflow checking
    size_t offset = 0;

    // Status line
    {
        size_t remaining = total_size - offset;
        int written = snprintf(buffer + offset, remaining,
                               "HTTP/1.1 %d %s\r\n",
                               response->status_code,
                               get_status_phrase(response->status_code));
        if (written < 0 || (size_t)written >= remaining) {
            log_error("libuv_send_response: Failed to write status line");
            safe_free(buffer);
            return -1;
        }
        offset += (size_t)written;
    }

    // Content-Type
    if (response->content_type[0]) {
        size_t remaining = total_size - offset;
        int written = snprintf(buffer + offset, remaining,
                               "Content-Type: %s\r\n", response->content_type);
        if (written < 0 || (size_t)written >= remaining) {
            log_error("libuv_send_response: Failed to write Content-Type header");
            safe_free(buffer);
            return -1;
        }
        offset += (size_t)written;
    }

    // Content-Length
    {
        size_t remaining = total_size - offset;
        int written = snprintf(buffer + offset, remaining,
                               "Content-Length: %zu\r\n", response->body_length);
        if (written < 0 || (size_t)written >= remaining) {
            log_error("libuv_send_response: Failed to write Content-Length header");
            safe_free(buffer);
            return -1;
        }
        offset += (size_t)written;
    }

    // Custom headers
    for (int i = 0; i < response->num_headers; i++) {
        size_t remaining = total_size - offset;
        int written = snprintf(buffer + offset, remaining,
                               "%s: %s\r\n",
                               response->headers[i].name,
                               response->headers[i].value);
        if (written < 0 || (size_t)written >= remaining) {
            log_error("libuv_send_response: Failed to write custom header");
            safe_free(buffer);
            return -1;
        }
        offset += (size_t)written;
    }

    // End of headers
    {
        size_t remaining = total_size - offset;
        int written = snprintf(buffer + offset, remaining, "\r\n");
        if (written < 0 || (size_t)written >= remaining) {
            log_error("libuv_send_response: Failed to write end-of-headers");
            safe_free(buffer);
            return -1;
        }
        offset += (size_t)written;
    }

    // Body
    if (response->body && response->body_length > 0) {
        if (response->body_length > total_size - offset) {
            log_error("libuv_send_response: Body does not fit into allocated buffer");
            safe_free(buffer);
            return -1;
        }
        memcpy(buffer + offset, response->body, response->body_length);
        offset += response->body_length;
    }
    
    log_debug("libuv_send_response: Sending %d %s (%zu bytes)",
              response->status_code, get_status_phrase(response->status_code),
              response->body_length);
    
    // Send the response (buffer will be freed after send)
    return libuv_connection_send(conn, buffer, offset, true);
}

/**
 * @brief Send an HTTP response with a post-write action
 */
int libuv_send_response_ex(libuv_connection_t *conn, const http_response_t *response,
                           write_complete_action_t action) {
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
        log_error("libuv_send_response_ex: Failed to allocate response buffer");
        return -1;
    }

    // Build response with proper overflow checking
    size_t offset = 0;

    // Status line
    {
        size_t remaining = total_size - offset;
        int written = snprintf(buffer + offset, remaining,
                               "HTTP/1.1 %d %s\r\n",
                               response->status_code,
                               get_status_phrase(response->status_code));
        if (written < 0 || (size_t)written >= remaining) {
            log_error("libuv_send_response_ex: Failed to write status line");
            safe_free(buffer);
            return -1;
        }
        offset += (size_t)written;
    }

    // Content-Type
    if (response->content_type[0]) {
        size_t remaining = total_size - offset;
        int written = snprintf(buffer + offset, remaining,
                               "Content-Type: %s\r\n", response->content_type);
        if (written < 0 || (size_t)written >= remaining) {
            log_error("libuv_send_response_ex: Failed to write Content-Type header");
            safe_free(buffer);
            return -1;
        }
        offset += (size_t)written;
    }

    // Content-Length
    {
        size_t remaining = total_size - offset;
        int written = snprintf(buffer + offset, remaining,
                               "Content-Length: %zu\r\n", response->body_length);
        if (written < 0 || (size_t)written >= remaining) {
            log_error("libuv_send_response_ex: Failed to write Content-Length header");
            safe_free(buffer);
            return -1;
        }
        offset += (size_t)written;
    }

    // Custom headers
    for (int i = 0; i < response->num_headers; i++) {
        size_t remaining = total_size - offset;
        int written = snprintf(buffer + offset, remaining,
                               "%s: %s\r\n",
                               response->headers[i].name,
                               response->headers[i].value);
        if (written < 0 || (size_t)written >= remaining) {
            log_error("libuv_send_response_ex: Failed to write custom header");
            safe_free(buffer);
            return -1;
        }
        offset += (size_t)written;
    }

    // End of headers
    {
        size_t remaining = total_size - offset;
        int written = snprintf(buffer + offset, remaining, "\r\n");
        if (written < 0 || (size_t)written >= remaining) {
            log_error("libuv_send_response_ex: Failed to write end-of-headers");
            safe_free(buffer);
            return -1;
        }
        offset += (size_t)written;
    }

    // Body
    if (response->body && response->body_length > 0) {
        if (response->body_length > total_size - offset) {
            log_error("libuv_send_response_ex: Body does not fit into allocated buffer");
            safe_free(buffer);
            return -1;
        }
        memcpy(buffer + offset, response->body, response->body_length);
        offset += response->body_length;
    }

    log_debug("libuv_send_response_ex: Sending %d %s (%zu bytes, action=%d)",
              response->status_code, get_status_phrase(response->status_code),
              response->body_length, action);

    // Send the response with the specified post-write action
    return libuv_connection_send_ex(conn, buffer, offset, true, action);
}

#endif /* HTTP_BACKEND_LIBUV */

