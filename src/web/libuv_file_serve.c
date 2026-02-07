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
#include <uv.h>

#include "web/libuv_server.h"
#include "web/libuv_connection.h"
#include "core/logger.h"
#include "utils/memory.h"

// Forward declarations
static void on_file_open(uv_fs_t *req);
static void on_file_stat(uv_fs_t *req);
static void on_file_read(uv_fs_t *req);
static void on_file_close(uv_fs_t *req);
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
    
    // Allocate context
    file_serve_ctx_t *ctx = safe_calloc(1, sizeof(file_serve_ctx_t));
    if (!ctx) {
        log_error("libuv_serve_file: Failed to allocate context");
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

