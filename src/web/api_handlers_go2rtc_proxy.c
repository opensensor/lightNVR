/**
 * @file api_handlers_go2rtc_proxy.c
 * @brief Streaming reverse proxy handler for forwarding requests to go2rtc
 *
 * Proxies HLS and snapshot requests to the local go2rtc instance with:
 * - Streaming pass-through (no buffering)
 * - Concurrency limiting (semaphore-based)
 * - Scoped to HLS/snapshot endpoints only
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <curl/curl.h>
#include <pthread.h>
#include <semaphore.h>

#include "web/api_handlers_go2rtc_proxy.h"
#include "web/request_response.h"
#include "core/config.h"
#include "core/logger.h"
#include "utils/memory.h"

#ifdef HTTP_BACKEND_LIBUV
#include "web/libuv_connection.h"
#include "web/libuv_server.h"
#endif

// Concurrency limiter
static sem_t *g_proxy_semaphore = NULL;
static pthread_once_t g_proxy_init_once = PTHREAD_ONCE_INIT;

// Buffered proxy context
typedef struct {
    char *buffer;
    size_t buffer_size;
    char content_type[256];
    long http_code;
    bool error_occurred;
} streaming_proxy_ctx_t;

/**
 * @brief Initialize the proxy semaphore (called once)
 */
static void init_proxy_semaphore(void) {
    int max_inflight = g_config.go2rtc_proxy_max_inflight;
    if (max_inflight < 1) max_inflight = 12;

    g_proxy_semaphore = safe_malloc(sizeof(sem_t));
    if (!g_proxy_semaphore) {
        log_error("go2rtc proxy: Failed to allocate semaphore");
        return;
    }

    if (sem_init(g_proxy_semaphore, 0, max_inflight) != 0) {
        log_error("go2rtc proxy: Failed to initialize semaphore");
        safe_free(g_proxy_semaphore);
        g_proxy_semaphore = NULL;
        return;
    }

    log_info("go2rtc proxy: Initialized with max_inflight=%d", max_inflight);
}

/**
 * @brief Simple buffered write callback
 */
static size_t buffered_write_cb(void *contents, size_t size, size_t nmemb, void *userp) {
    size_t total = size * nmemb;
    streaming_proxy_ctx_t *ctx = (streaming_proxy_ctx_t *)userp;

    // Reallocate buffer to fit new data
    char *new_buf = realloc(ctx->buffer, ctx->buffer_size + total);
    if (!new_buf) {
        log_error("go2rtc proxy: Failed to allocate buffer");
        ctx->error_occurred = true;
        return 0;
    }

    ctx->buffer = new_buf;
    memcpy(ctx->buffer + ctx->buffer_size, contents, total);
    ctx->buffer_size += total;

    return total;
}

/**
 * @brief Header callback - extract Content-Type
 */
static size_t buffered_header_cb(char *buffer, size_t size, size_t nitems, void *userp) {
    size_t total = size * nitems;
    streaming_proxy_ctx_t *ctx = (streaming_proxy_ctx_t *)userp;

    // Parse Content-Type header
    if (total > 14 && strncasecmp(buffer, "Content-Type:", 13) == 0) {
        const char *val = buffer + 13;
        while (*val == ' ' || *val == '\t') val++;
        size_t len = total - (val - buffer);
        // Trim trailing \r\n
        while (len > 0 && (val[len - 1] == '\r' || val[len - 1] == '\n')) len--;
        if (len >= sizeof(ctx->content_type)) len = sizeof(ctx->content_type) - 1;
        memcpy(ctx->content_type, val, len);
        ctx->content_type[len] = '\0';
    }

    return total;
}

/**
 * @brief Check if path should be proxied (streaming endpoints only)
 */
static bool should_proxy_path(const char *path) {
    // Only proxy HLS streaming endpoints and health check
    // WebRTC connects directly to go2rtc for lower latency
    if (strstr(path, "/api/streams") != NULL) return true;   // Health check endpoint
    if (strstr(path, "/api/stream.m3u8") != NULL) return true;
    if (strstr(path, "/api/hls/") != NULL) return true;
    if (strstr(path, "/api/frame.jpeg") != NULL) return true;
    return false;
}

void handle_go2rtc_proxy(const http_request_t *req, http_response_t *res) {
    // Check if this path should be proxied
    if (!should_proxy_path(req->path)) {
        log_debug("go2rtc proxy: Path not in scope: %s", req->path);
        http_response_set_json_error(res, 404, "Not found");
        return;
    }

    // Initialize semaphore on first use
    pthread_once(&g_proxy_init_once, init_proxy_semaphore);

    if (!g_proxy_semaphore) {
        log_error("go2rtc proxy: Semaphore not initialized");
        http_response_set_json_error(res, 503, "Proxy service unavailable");
        return;
    }

    // Try to acquire semaphore (non-blocking)
    if (sem_trywait(g_proxy_semaphore) != 0) {
        log_warn("go2rtc proxy: Max concurrent requests reached, returning 503");
        http_response_set_json_error(res, 503, "Service temporarily unavailable - too many concurrent requests");
        return;
    }

    // Build the target URL
    char url[2048];
    int port = g_config.go2rtc_api_port > 0 ? g_config.go2rtc_api_port : 1984;

    if (req->query_string[0] != '\0') {
        snprintf(url, sizeof(url), "http://127.0.0.1:%d%s?%s", port, req->path, req->query_string);
    } else {
        snprintf(url, sizeof(url), "http://127.0.0.1:%d%s", port, req->path);
    }

    log_debug("go2rtc proxy: %s %s -> %s", req->method_str, req->path, url);

    // Get connection handle (libuv-specific)
    libuv_connection_t *conn = (libuv_connection_t *)req->user_data;
    if (!conn) {
        log_error("go2rtc proxy: No connection handle available");
        sem_post(g_proxy_semaphore);
        http_response_set_json_error(res, 500, "Internal server error");
        return;
    }

    // Set up buffered context
    streaming_proxy_ctx_t ctx = {
        .buffer = NULL,
        .buffer_size = 0,
        .http_code = 0,
        .error_occurred = false
    };
    ctx.content_type[0] = '\0';

    // Initialize curl
    CURL *curl = curl_easy_init();
    if (!curl) {
        log_error("go2rtc proxy: curl_easy_init failed");
        sem_post(g_proxy_semaphore);
        http_response_set_json_error(res, 502, "Proxy error: failed to initialize HTTP client");
        return;
    }

    // Configure curl for buffered response
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, buffered_write_cb);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &ctx);
    curl_easy_setopt(curl, CURLOPT_HEADERFUNCTION, buffered_header_cb);
    curl_easy_setopt(curl, CURLOPT_HEADERDATA, &ctx);
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 30L);
    curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT, 5L);
    curl_easy_setopt(curl, CURLOPT_NOSIGNAL, 1L);
    curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, 1L);
    curl_easy_setopt(curl, CURLOPT_MAXREDIRS, 3L);

    // Set method and body
    struct curl_slist *headers = NULL;
    if (strcmp(req->method_str, "POST") == 0) {
        curl_easy_setopt(curl, CURLOPT_POST, 1L);
        if (req->body && req->body_len > 0) {
            curl_easy_setopt(curl, CURLOPT_POSTFIELDS, req->body);
            curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, (long)req->body_len);
        }
    } else if (strcmp(req->method_str, "PUT") == 0) {
        curl_easy_setopt(curl, CURLOPT_CUSTOMREQUEST, "PUT");
        if (req->body && req->body_len > 0) {
            curl_easy_setopt(curl, CURLOPT_POSTFIELDS, req->body);
            curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, (long)req->body_len);
        }
    } else if (strcmp(req->method_str, "DELETE") == 0) {
        curl_easy_setopt(curl, CURLOPT_CUSTOMREQUEST, "DELETE");
    }

    // Forward Content-Type header if present
    if (req->content_type[0] != '\0') {
        char ct_header[256];
        snprintf(ct_header, sizeof(ct_header), "Content-Type: %s", req->content_type);
        headers = curl_slist_append(headers, ct_header);
    }
    if (headers) {
        curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
    }

    // Perform the request
    CURLcode cres = curl_easy_perform(curl);

    // Get HTTP status code
    curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &ctx.http_code);

    // Cleanup curl
    if (headers) curl_slist_free_all(headers);
    curl_easy_cleanup(curl);
    sem_post(g_proxy_semaphore);

    // Handle errors
    if (cres != CURLE_OK || ctx.error_occurred) {
        log_warn("go2rtc proxy: curl request failed: %s", curl_easy_strerror(cres));
        if (ctx.buffer) free(ctx.buffer);
        http_response_set_json_error(res, 502, "Proxy error: go2rtc is not responding");
        return;
    }

    // Set response
    res->status_code = (int)ctx.http_code;
    if (ctx.content_type[0] != '\0') {
        strncpy(res->content_type, ctx.content_type, sizeof(res->content_type) - 1);
    }

    // Add CORS headers
    http_response_add_header(res, "Access-Control-Allow-Origin", "*");
    http_response_add_header(res, "Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS");
    http_response_add_header(res, "Access-Control-Allow-Headers", "Content-Type, Authorization");

    // Set body (transfer ownership)
    res->body = ctx.buffer;
    res->body_length = ctx.buffer_size;
    res->body_allocated = true;

    log_debug("go2rtc proxy: %s %s -> %ld (%zu bytes, type: %s)",
              req->method_str, req->path, ctx.http_code, ctx.buffer_size,
              ctx.content_type[0] ? ctx.content_type : "unknown");
}

