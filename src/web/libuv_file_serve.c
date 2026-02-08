/**
 * @file libuv_file_serve.c
 * @brief Async file serving for libuv HTTP server
 *
 * Uses libuv's async file I/O (uv_fs_*) for non-blocking file serving.
 * Supports Range requests for video seeking.
 */

#ifdef HTTP_BACKEND_LIBUV

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <llhttp.h>
#include <uv.h>

#include "utils/memory.h"
#include "web/libuv_server.h"
#include "web/libuv_connection.h"
#include "core/logger.h"

// Forward declarations
static void on_file_open(uv_fs_t *req);
static void on_file_stat(uv_fs_t *req);
static void on_file_read(uv_fs_t *req);
static void on_file_close(uv_fs_t *req);
static void on_chunk_write_complete(uv_write_t *req, int status);
static void file_serve_cleanup(file_serve_ctx_t *ctx);
static void send_file_chunk(file_serve_ctx_t *ctx);

/**
 * @brief Common MIME types
 */
const char *libuv_get_mime_type(const char *path) {
    const char *ext = strrchr(path, '.');
    if (!ext) return "application/octet-stream";
    
    ext++;  // Skip the dot
    
    // Video
    if (strcasecmp(ext, "mp4") == 0) return "video/mp4";
    if (strcasecmp(ext, "m4s") == 0) return "video/iso.segment";
    if (strcasecmp(ext, "ts") == 0) return "video/mp2t";
    if (strcasecmp(ext, "m3u8") == 0) return "application/vnd.apple.mpegurl";
    if (strcasecmp(ext, "webm") == 0) return "video/webm";
    if (strcasecmp(ext, "mkv") == 0) return "video/x-matroska";
    
    // Web
    if (strcasecmp(ext, "html") == 0) return "text/html; charset=utf-8";
    if (strcasecmp(ext, "css") == 0) return "text/css; charset=utf-8";
    if (strcasecmp(ext, "js") == 0) return "application/javascript; charset=utf-8";
    if (strcasecmp(ext, "json") == 0) return "application/json; charset=utf-8";
    if (strcasecmp(ext, "xml") == 0) return "application/xml; charset=utf-8";
    
    // Images
    if (strcasecmp(ext, "png") == 0) return "image/png";
    if (strcasecmp(ext, "jpg") == 0 || strcasecmp(ext, "jpeg") == 0) return "image/jpeg";
    if (strcasecmp(ext, "gif") == 0) return "image/gif";
    if (strcasecmp(ext, "svg") == 0) return "image/svg+xml";
    if (strcasecmp(ext, "ico") == 0) return "image/x-icon";
    if (strcasecmp(ext, "webp") == 0) return "image/webp";
    
    // Fonts
    if (strcasecmp(ext, "woff") == 0) return "font/woff";
    if (strcasecmp(ext, "woff2") == 0) return "font/woff2";
    if (strcasecmp(ext, "ttf") == 0) return "font/ttf";
    
    // Other
    if (strcasecmp(ext, "txt") == 0) return "text/plain; charset=utf-8";
    if (strcasecmp(ext, "pdf") == 0) return "application/pdf";
    
    return "application/octet-stream";
}

/**
 * @brief Parse Range header
 */
bool libuv_parse_range_header(const char *range_header, size_t file_size,
                               size_t *start, size_t *end) {
    if (!range_header || strncmp(range_header, "bytes=", 6) != 0) {
        return false;
    }
    
    const char *range = range_header + 6;
    
    if (*range == '-') {
        // Suffix range: bytes=-500 means last 500 bytes
        size_t suffix_len = strtoull(range + 1, NULL, 10);
        if (suffix_len > file_size) suffix_len = file_size;
        *start = file_size - suffix_len;
        *end = file_size - 1;
    } else {
        // Normal range: bytes=0-499 or bytes=500-
        *start = strtoull(range, NULL, 10);
        const char *dash = strchr(range, '-');
        if (dash && dash[1] != '\0') {
            *end = strtoull(dash + 1, NULL, 10);
        } else {
            *end = file_size - 1;
        }
    }
    
    // Validate range
    if (*start >= file_size) {
        return false;
    }
    if (*end >= file_size) {
        *end = file_size - 1;
    }
    if (*start > *end) {
        return false;
    }
    
    return true;
}

/**
 * @brief Serve a file asynchronously
 */
int libuv_serve_file(libuv_connection_t *conn, const char *path,
                     const char *content_type, const char *extra_headers) {
    if (!conn || !path) {
        return -1;
    }

    // Mark that async response is pending
    conn->async_response_pending = true;

    // Allocate context
    file_serve_ctx_t *ctx = safe_calloc(1, sizeof(file_serve_ctx_t));
    if (!ctx) {
        log_error("libuv_serve_file: Failed to allocate context");
        conn->async_response_pending = false;
        return -1;
    }

    ctx->conn = conn;
    ctx->fd = -1;
    
    // Set content type
    if (content_type) {
        strncpy(ctx->content_type, content_type, sizeof(ctx->content_type) - 1);
    } else {
        strncpy(ctx->content_type, libuv_get_mime_type(path), 
                sizeof(ctx->content_type) - 1);
    }
    
    // Check for Range header
    const char *range_header = http_request_get_header(&conn->request, "Range");
    if (range_header) {
        ctx->has_range = true;
        // Range will be parsed after we know the file size
    }
    
    // Allocate read buffer
    ctx->buffer = safe_malloc(LIBUV_FILE_BUFFER_SIZE);
    if (!ctx->buffer) {
        log_error("libuv_serve_file: Failed to allocate buffer");
        safe_free(ctx);
        conn->async_response_pending = false;
        return -1;
    }
    ctx->buffer_size = LIBUV_FILE_BUFFER_SIZE;
    
    // Store context in request data for callbacks
    ctx->open_req.data = ctx;
    ctx->stat_req.data = ctx;
    ctx->read_req.data = ctx;
    ctx->close_req.data = ctx;
    
    // Open file asynchronously
    int r = uv_fs_open(conn->server->loop, &ctx->open_req, path,
                       UV_FS_O_RDONLY, 0, on_file_open);
    if (r != 0) {
        log_error("libuv_serve_file: Failed to start open: %s", uv_strerror(r));
        file_serve_cleanup(ctx);
        return -1;
    }

    return 0;
}

/**
 * @brief Cleanup file serve context
 */
static void file_serve_cleanup(file_serve_ctx_t *ctx) {
    if (!ctx) return;

    if (ctx->buffer) {
        safe_free(ctx->buffer);
    }

    // Clean up uv_fs_t requests
    uv_fs_req_cleanup(&ctx->open_req);
    uv_fs_req_cleanup(&ctx->stat_req);
    uv_fs_req_cleanup(&ctx->read_req);
    uv_fs_req_cleanup(&ctx->close_req);

    safe_free(ctx);
}

/**
 * @brief File open callback
 */
static void on_file_open(uv_fs_t *req) {
    file_serve_ctx_t *ctx = (file_serve_ctx_t *)req->data;
    libuv_connection_t *conn = ctx->conn;

    if (req->result < 0) {
        log_error("on_file_open: Failed to open file: %s", uv_strerror(req->result));

        // Send 404 response
        http_response_set_json_error(&conn->response, 404, "File Not Found");

        // Determine post-response action based on keep-alive
        write_complete_action_t action =
            (conn->keep_alive && llhttp_should_keep_alive(&conn->parser))
                ? WRITE_ACTION_KEEP_ALIVE
                : WRITE_ACTION_CLOSE;

        extern int libuv_send_response_ex(libuv_connection_t *conn, const http_response_t *response,
                                          write_complete_action_t action);
        libuv_send_response_ex(conn, &conn->response, action);
        file_serve_cleanup(ctx);

        // Clear async response flag
        conn->async_response_pending = false;
        return;
    }

    ctx->fd = req->result;
    uv_fs_req_cleanup(req);

    // Get file stats
    int r = uv_fs_fstat(ctx->conn->server->loop, &ctx->stat_req, ctx->fd, on_file_stat);
    if (r != 0) {
        log_error("on_file_open: Failed to start stat: %s", uv_strerror(r));
        uv_fs_close(ctx->conn->server->loop, &ctx->close_req, ctx->fd, on_file_close);
        return;
    }
}

/**
 * @brief File stat callback
 */
static void on_file_stat(uv_fs_t *req) {
    file_serve_ctx_t *ctx = (file_serve_ctx_t *)req->data;
    libuv_connection_t *conn = ctx->conn;

    if (req->result < 0) {
        log_error("on_file_stat: Failed to stat file: %s", uv_strerror(req->result));
        http_response_set_json_error(&conn->response, 500, "Failed to stat file");

        // Determine post-response action based on keep-alive
        write_complete_action_t action =
            (conn->keep_alive && llhttp_should_keep_alive(&conn->parser))
                ? WRITE_ACTION_KEEP_ALIVE
                : WRITE_ACTION_CLOSE;

        extern int libuv_send_response_ex(libuv_connection_t *conn, const http_response_t *response,
                                          write_complete_action_t action);
        libuv_send_response_ex(conn, &conn->response, action);

        // Clear async flag before closing file
        conn->async_response_pending = false;

        // Close the file descriptor
        uv_fs_close(conn->server->loop, &ctx->close_req, ctx->fd, on_file_close);
        return;
    }

    ctx->file_size = req->statbuf.st_size;
    uv_fs_req_cleanup(req);

    // Handle Range header
    if (ctx->has_range) {
        const char *range_header = http_request_get_header(&conn->request, "Range");
        if (!libuv_parse_range_header(range_header, ctx->file_size,
                                       &ctx->range_start, &ctx->range_end)) {
            // Invalid range - send 416
            http_response_set_json_error(&conn->response, 416,
                                         "Requested Range Not Satisfiable");

            // Determine post-response action based on keep-alive
            write_complete_action_t action =
                (conn->keep_alive && llhttp_should_keep_alive(&conn->parser))
                    ? WRITE_ACTION_KEEP_ALIVE
                    : WRITE_ACTION_CLOSE;

            extern int libuv_send_response_ex(libuv_connection_t *conn, const http_response_t *response,
                                              write_complete_action_t action);
            libuv_send_response_ex(conn, &conn->response, action);

            // Clear async flag before closing file
            conn->async_response_pending = false;

            // Close the file descriptor
            uv_fs_close(conn->server->loop, &ctx->close_req, ctx->fd, on_file_close);
            return;
        }
        ctx->offset = ctx->range_start;
        ctx->remaining = ctx->range_end - ctx->range_start + 1;
    } else {
        ctx->offset = 0;
        ctx->remaining = ctx->file_size;
        ctx->range_start = 0;
        ctx->range_end = ctx->file_size - 1;
    }

    // Send headers
    // Note: We don't send Connection: close as it causes issues with HTTP/2 proxies
    // The connection will be closed after all data is sent based on keep-alive settings
    char headers[1024];
    int len;

    if (ctx->has_range) {
        len = snprintf(headers, sizeof(headers),
            "HTTP/1.1 206 Partial Content\r\n"
            "Content-Type: %s\r\n"
            "Content-Length: %zu\r\n"
            "Content-Range: bytes %zu-%zu/%zu\r\n"
            "Accept-Ranges: bytes\r\n"
            "\r\n",
            ctx->content_type, ctx->remaining,
            ctx->range_start, ctx->range_end, ctx->file_size);
    } else {
        len = snprintf(headers, sizeof(headers),
            "HTTP/1.1 200 OK\r\n"
            "Content-Type: %s\r\n"
            "Content-Length: %zu\r\n"
            "Accept-Ranges: bytes\r\n"
            "\r\n",
            ctx->content_type, ctx->file_size);
    }

    char *header_buf = safe_malloc(len);
    if (!header_buf) {
        uv_fs_close(ctx->conn->server->loop, &ctx->close_req, ctx->fd, on_file_close);
        return;
    }
    memcpy(header_buf, headers, len);

    libuv_connection_send(ctx->conn, header_buf, len, true);
    ctx->headers_sent = true;

    // Start sending file content
    send_file_chunk(ctx);
}

/**
 * @brief Send next chunk of file
 */
static void send_file_chunk(file_serve_ctx_t *ctx) {
    if (ctx->remaining == 0) {
        // Done sending file
        uv_fs_close(ctx->conn->server->loop, &ctx->close_req, ctx->fd, on_file_close);
        return;
    }

    size_t to_read = ctx->remaining < ctx->buffer_size ? ctx->remaining : ctx->buffer_size;

    uv_buf_t buf = uv_buf_init(ctx->buffer, to_read);
    int r = uv_fs_read(ctx->conn->server->loop, &ctx->read_req, ctx->fd,
                       &buf, 1, ctx->offset, on_file_read);
    if (r != 0) {
        log_error("send_file_chunk: Failed to start read: %s", uv_strerror(r));
        uv_fs_close(ctx->conn->server->loop, &ctx->close_req, ctx->fd, on_file_close);
    }
}

/**
 * @brief Context for chunked file write
 */
typedef struct {
    uv_write_t req;           // Write request (must be first)
    uv_buf_t buf;             // Buffer being written
    file_serve_ctx_t *ctx;    // Parent file serve context
    bool free_buffer;         // Whether to free buffer on completion
} file_chunk_write_ctx_t;

/**
 * @brief Callback when a file chunk write completes
 *
 * This is the key fix for ERR_HTTP2_PROTOCOL_ERROR - we must wait for
 * each write to complete before starting the next read/write cycle.
 */
static void on_chunk_write_complete(uv_write_t *req, int status) {
    file_chunk_write_ctx_t *write_ctx = (file_chunk_write_ctx_t *)req;
    file_serve_ctx_t *ctx = write_ctx->ctx;

    // Free the send buffer
    if (write_ctx->free_buffer && write_ctx->buf.base) {
        safe_free(write_ctx->buf.base);
    }
    safe_free(write_ctx);

    if (status < 0) {
        log_error("on_chunk_write_complete: Write error: %s", uv_strerror(status));
        uv_fs_close(ctx->conn->server->loop, &ctx->close_req, ctx->fd, on_file_close);
        return;
    }

    // Now that the write is complete, continue with next chunk
    send_file_chunk(ctx);
}

/**
 * @brief File read callback
 */
static void on_file_read(uv_fs_t *req) {
    file_serve_ctx_t *ctx = (file_serve_ctx_t *)req->data;

    if (req->result < 0) {
        log_error("on_file_read: Read error: %s", uv_strerror(req->result));
        uv_fs_close(ctx->conn->server->loop, &ctx->close_req, ctx->fd, on_file_close);
        return;
    }

    if (req->result == 0) {
        // EOF
        uv_fs_close(ctx->conn->server->loop, &ctx->close_req, ctx->fd, on_file_close);
        return;
    }

    size_t bytes_read = req->result;
    uv_fs_req_cleanup(req);

    // Copy data to a new buffer for sending (async write needs its own buffer)
    char *send_buf = safe_malloc(bytes_read);
    if (!send_buf) {
        uv_fs_close(ctx->conn->server->loop, &ctx->close_req, ctx->fd, on_file_close);
        return;
    }
    memcpy(send_buf, ctx->buffer, bytes_read);

    // Update position tracking BEFORE sending
    ctx->offset += bytes_read;
    ctx->remaining -= bytes_read;

    // Allocate write context
    file_chunk_write_ctx_t *write_ctx = safe_malloc(sizeof(file_chunk_write_ctx_t));
    if (!write_ctx) {
        safe_free(send_buf);
        uv_fs_close(ctx->conn->server->loop, &ctx->close_req, ctx->fd, on_file_close);
        return;
    }

    write_ctx->ctx = ctx;
    write_ctx->buf = uv_buf_init(send_buf, bytes_read);
    write_ctx->free_buffer = true;

    // Check if connection is closing
    if (uv_is_closing((uv_handle_t *)&ctx->conn->handle)) {
        log_debug("on_file_read: Connection is closing, aborting file send");
        safe_free(send_buf);
        safe_free(write_ctx);
        uv_fs_close(ctx->conn->server->loop, &ctx->close_req, ctx->fd, on_file_close);
        return;
    }

    // Send this chunk and wait for completion before sending next
    int r = uv_write(&write_ctx->req, (uv_stream_t *)&ctx->conn->handle,
                     &write_ctx->buf, 1, on_chunk_write_complete);
    if (r != 0) {
        log_error("on_file_read: Write failed: %s", uv_strerror(r));
        safe_free(send_buf);
        safe_free(write_ctx);
        uv_fs_close(ctx->conn->server->loop, &ctx->close_req, ctx->fd, on_file_close);
    }
    // Note: on_chunk_write_complete will call send_file_chunk() when write is done
}

/**
 * @brief File close callback
 */
static void on_file_close(uv_fs_t *req) {
    file_serve_ctx_t *ctx = (file_serve_ctx_t *)req->data;
    libuv_connection_t *conn = ctx->conn;

    // Check if this is a successful file serve or an error case
    // If async_response_pending is false, the error handler already managed the connection
    bool should_manage_connection = conn->async_response_pending;

    uv_fs_req_cleanup(req);
    file_serve_cleanup(ctx);

    // Clear async response flag
    conn->async_response_pending = false;

    // If error handler already managed connection, don't do it again
    if (!should_manage_connection) {
        log_debug("on_file_close: Connection already managed by error handler");
        return;
    }

    // Check if server is shutting down
    if (conn->server->shutting_down) {
        log_debug("on_file_close: Server shutting down, skipping connection management");
        return;
    }

    // Respect keep-alive: reset connection for reuse or close it
    extern void libuv_connection_close(libuv_connection_t *conn);
    extern void libuv_connection_reset(libuv_connection_t *conn);

    if (!uv_is_closing((uv_handle_t *)&conn->handle)) {
        if (conn->keep_alive && llhttp_should_keep_alive(&conn->parser)) {
            // Keep connection alive for next request
            log_debug("on_file_close: Keeping connection alive for reuse");
            libuv_connection_reset(conn);
        } else {
            // Close connection
            log_debug("on_file_close: Closing connection (keep_alive=%d)", conn->keep_alive);
            libuv_connection_close(conn);
        }
    }
}

#endif /* HTTP_BACKEND_LIBUV */

