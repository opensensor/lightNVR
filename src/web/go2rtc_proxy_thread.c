/**
 * @file go2rtc_proxy_thread.c
 * @brief Detached-pthread proxy for go2rtc reverse-proxy requests
 *
 * Bypasses the shared libuv thread pool (UV_THREADPOOL_SIZE=16) by spawning
 * one short-lived detached pthread per proxy request.  Completed responses are
 * delivered back to the event-loop thread via a uv_async_t handle so that
 * libuv_send_response_ex() is always called from the loop thread, as required.
 */

#ifdef HTTP_BACKEND_LIBUV

#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <unistd.h>
#include <strings.h>
#include <time.h>
#include <curl/curl.h>

#include "web/go2rtc_proxy_thread.h"
#include "web/api_handlers_go2rtc_proxy.h"
#include "web/httpd_utils.h"
#include "web/libuv_connection.h"
#include "web/request_response.h"
#include "core/config.h"
#define LOG_COMPONENT "go2rtcProxy"
#include "core/logger.h"
#include "utils/memory.h"
#include "utils/strings.h"

// Maximum concurrent proxy threads (independent of UV_THREADPOOL_SIZE)
#define MAX_CONCURRENT_PROXY_THREADS 32

// ============================================================================
// Internal types
// ============================================================================

/**
 * @brief Per-request context — allocated before the thread is spawned.
 *
 * All fields needed by the blocking curl call are captured here so the thread
 * never touches the (potentially freed) connection struct.  The connection
 * pointer is only accessed from the event-loop thread in proxy_async_cb().
 */
typedef struct proxy_thread_ctx {
    // Captured request data
    http_request_t request;      // inline copy; body points to body below
    char url[2048];
    char method[16];
    char *body;                 // malloc'd copy, NULL if no body
    size_t body_len;
    char content_type[256];
    bool capture_hls_session;
    char capture_stream_name[MAX_STREAM_NAME];
    char auth_fingerprint[65];
    write_complete_action_t action;

    // Connection for response delivery (safe only on event-loop thread)
    libuv_connection_t *conn;

    // Response populated by the worker thread
    char *response_buffer;      // malloc'd body, may be NULL
    size_t response_size;
    char response_content_type[256];
    long http_code;
    bool error;
    bool authorization_denied;
    http_response_t auth_response;

    // Intrusive linked-list node for the done queue
    struct proxy_thread_ctx *next;
} proxy_thread_ctx_t;

// ============================================================================
// Global state
// ============================================================================

static struct {
    uv_loop_t *loop;
    uv_async_t async_handle;
    pthread_mutex_t done_mutex;
    proxy_thread_ctx_t *done_head;
    proxy_thread_ctx_t *done_tail;
    volatile int active_count;
    volatile bool shutting_down;
    bool initialized;
} g_proxy_state = {0};

// ============================================================================
// curl write/header callbacks
// ============================================================================

static size_t proxy_write_cb(void *contents, size_t size, size_t nmemb, void *userp) {
    size_t total = size * nmemb;
    proxy_thread_ctx_t *ctx = (proxy_thread_ctx_t *)userp;

    char *new_buf = realloc(ctx->response_buffer, ctx->response_size + total);
    if (!new_buf) {
        log_error("go2rtc proxy thread: Response buffer realloc failed");
        ctx->error = true;
        return 0;
    }
    ctx->response_buffer = new_buf;
    memcpy(ctx->response_buffer + ctx->response_size, contents, total);
    ctx->response_size += total;
    return total;
}

static size_t proxy_header_cb(char *buffer, size_t size, size_t nitems, void *userp) {
    size_t total = size * nitems;
    proxy_thread_ctx_t *ctx = (proxy_thread_ctx_t *)userp;

    if (total > 14 && strncasecmp(buffer, "Content-Type:", 13) == 0) {
        const char *val = buffer + 13;
        while (*val == ' ' || *val == '\t') val++;
        size_t len = total - (size_t)(val - buffer);
        while (len > 0 && (val[len - 1] == '\r' || val[len - 1] == '\n')) len--;
        if (len >= sizeof(ctx->response_content_type))
            len = sizeof(ctx->response_content_type) - 1;
        memcpy(ctx->response_content_type, val, len);
        ctx->response_content_type[len] = '\0';
    }
    return total;
}

// ============================================================================
// Done-queue helpers (thread-safe)
// ============================================================================

static void enqueue_done(proxy_thread_ctx_t *ctx) {
    pthread_mutex_lock(&g_proxy_state.done_mutex);
    ctx->next = NULL;
    if (g_proxy_state.done_tail) {
        g_proxy_state.done_tail->next = ctx;
    } else {
        g_proxy_state.done_head = ctx;
    }
    g_proxy_state.done_tail = ctx;
    pthread_mutex_unlock(&g_proxy_state.done_mutex);
}

static proxy_thread_ctx_t *dequeue_all_done(void) {
    pthread_mutex_lock(&g_proxy_state.done_mutex);
    proxy_thread_ctx_t *head = g_proxy_state.done_head;
    g_proxy_state.done_head = NULL;
    g_proxy_state.done_tail = NULL;
    pthread_mutex_unlock(&g_proxy_state.done_mutex);
    return head;
}

static void free_proxy_context(proxy_thread_ctx_t *ctx) {
    if (!ctx) return;
    if (ctx->response_buffer) free(ctx->response_buffer);
    if (ctx->body) free(ctx->body);
    http_response_free(&ctx->auth_response);
    safe_free(ctx);
}



// ============================================================================
// Worker thread
// ============================================================================

static void *proxy_worker_thread(void *arg) {
    log_set_thread_context("go2rtcProxy", NULL);
    proxy_thread_ctx_t *ctx = (proxy_thread_ctx_t *)arg;

    // Scoped authorization may perform session/token lookups, camera policy
    // checks and audit writes.  It must not run in llhttp's message-complete
    // callback because that callback owns the sole libuv event-loop thread.
    struct timespec auth_started;
    struct timespec auth_finished;
    clock_gettime(CLOCK_MONOTONIC, &auth_started);
    if (!go2rtc_proxy_authorize_request(&ctx->request,
                                        &ctx->auth_response)) {
        ctx->authorization_denied = true;
    }
    clock_gettime(CLOCK_MONOTONIC, &auth_finished);
    long auth_elapsed_ms =
        (auth_finished.tv_sec - auth_started.tv_sec) * 1000L +
        (auth_finished.tv_nsec - auth_started.tv_nsec) / 1000000L;
    if (auth_elapsed_ms >= 250) {
        log_warn("go2rtc proxy authorization took %ld ms for %s",
                 auth_elapsed_ms, ctx->request.path);
    }
    if (ctx->authorization_denied) goto done;

    if (strstr(ctx->request.path, "/api/stream.m3u8") &&
        http_request_get_query_param(&ctx->request, "src",
                                     ctx->capture_stream_name,
                                     sizeof(ctx->capture_stream_name)) >= 0 &&
        httpd_request_auth_fingerprint(&ctx->request,
                                       ctx->auth_fingerprint) == 0) {
        ctx->capture_hls_session = true;
    }

    CURL *curl = curl_easy_init();
    if (!curl) {
        log_error("go2rtc proxy thread: curl_easy_init failed");
        ctx->error = true;
        goto done;
    }

    curl_easy_setopt(curl, CURLOPT_URL, ctx->url);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, proxy_write_cb);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, ctx);
    curl_easy_setopt(curl, CURLOPT_HEADERFUNCTION, proxy_header_cb);
    curl_easy_setopt(curl, CURLOPT_HEADERDATA, ctx);
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 30L);
    curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT, 5L);
    curl_easy_setopt(curl, CURLOPT_NOSIGNAL, 1L);
    curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, 1L);
    curl_easy_setopt(curl, CURLOPT_MAXREDIRS, 3L);

    struct curl_slist *headers = NULL;

    if (strcmp(ctx->method, "POST") == 0) {
        curl_easy_setopt(curl, CURLOPT_POST, 1L);
        if (ctx->body && ctx->body_len > 0) {
            curl_easy_setopt(curl, CURLOPT_POSTFIELDS, ctx->body);
            curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, (long)ctx->body_len);
        }
    } else if (strcmp(ctx->method, "PUT") == 0) {
        curl_easy_setopt(curl, CURLOPT_CUSTOMREQUEST, "PUT");
        if (ctx->body && ctx->body_len > 0) {
            curl_easy_setopt(curl, CURLOPT_POSTFIELDS, ctx->body);
            curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, (long)ctx->body_len);
        }
    } else if (strcmp(ctx->method, "DELETE") == 0) {
        curl_easy_setopt(curl, CURLOPT_CUSTOMREQUEST, "DELETE");
    }

    if (ctx->content_type[0] != '\0') {
        char ct_header[280];
        snprintf(ct_header, sizeof(ct_header), "Content-Type: %s", ctx->content_type);
        headers = curl_slist_append(headers, ct_header);
        curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
    }

    {
        CURLcode cres = curl_easy_perform(curl);
        curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &ctx->http_code);
        if (headers) curl_slist_free_all(headers);
        curl_easy_cleanup(curl);
        if (cres != CURLE_OK || ctx->error) {
            log_warn("go2rtc proxy thread: curl failed: %s", curl_easy_strerror(cres));
            ctx->error = true;
        } else {
            if (ctx->capture_hls_session) {
                go2rtc_proxy_capture_hls_sessions_for(
                    ctx->capture_stream_name, ctx->auth_fingerprint,
                    ctx->response_buffer, ctx->response_size);
            }
        }
    }

done:
    enqueue_done(ctx);
    uv_async_send(&g_proxy_state.async_handle);
    __sync_sub_and_fetch(&g_proxy_state.active_count, 1);
    return NULL;
}

// ============================================================================
// uv_async callback — runs on the event-loop thread
// ============================================================================

static void proxy_async_cb(uv_async_t *handle) {
    (void)handle;

    proxy_thread_ctx_t *ctx = dequeue_all_done();
    while (ctx) {
        proxy_thread_ctx_t *next = ctx->next;
        libuv_connection_t *conn = ctx->conn;

        if (g_proxy_state.shutting_down) {
            // Server shutting down — connection may already be torn down
            free_proxy_context(ctx);
            ctx = next;
            continue;
        }

        if (ctx->authorization_denied) {
            // Transfer the worker-owned response, including an allocated JSON
            // error body, to the connection before sending it on the loop.
            http_response_free(&conn->response);
            conn->response = ctx->auth_response;
            ctx->auth_response.body = NULL;
            ctx->auth_response.body_length = 0;
            ctx->auth_response.body_allocated = false;
        } else if (ctx->error || ctx->http_code == 0) {
            http_response_set_json_error(&conn->response, 502,
                                         "Proxy error: go2rtc is not responding");
        } else {
            conn->response.status_code = (int)ctx->http_code;
            if (ctx->response_content_type[0] != '\0') {
                safe_strcpy(conn->response.content_type, ctx->response_content_type,
                        sizeof(conn->response.content_type), 0);
            }
            http_response_add_header(&conn->response, "Access-Control-Allow-Origin", "*");
            http_response_add_header(&conn->response, "Access-Control-Allow-Methods",
                                     "GET, POST, PUT, DELETE, OPTIONS");
            http_response_add_header(&conn->response, "Access-Control-Allow-Headers",
                                     "Content-Type, Authorization");
            // Transfer buffer ownership to the response (freed by http_response_free)
            conn->response.body = ctx->response_buffer;
            conn->response.body_length = ctx->response_size;
            conn->response.body_allocated = true;
            ctx->response_buffer = NULL;  // ownership transferred
            log_debug("go2rtc proxy thread: %s -> %ld (%zu bytes)",
                      ctx->url, ctx->http_code, ctx->response_size);
        }

        conn->async_response_pending = false;
        libuv_send_response_ex(conn, &conn->response, ctx->action);

        free_proxy_context(ctx);
        ctx = next;
    }
}

// ============================================================================
// Public API
// ============================================================================

bool go2rtc_proxy_path_matches(const char *path) {
    if (!path) return false;
    // Only intercept the HTTP endpoints registered by libuv_api_handlers.
    // WebRTC SDP and MSE WebSockets connect directly to go2rtc and must never
    // enter this extra libcurl proxy hop.
    return strcmp(path, "/go2rtc/api/streams") == 0 ||
           strcmp(path, "/go2rtc/api/reload") == 0 ||
           strcmp(path, "/go2rtc/api/stream.m3u8") == 0 ||
           strncmp(path, "/go2rtc/api/hls/", 16) == 0 ||
           strcmp(path, "/go2rtc/api/frame.jpeg") == 0;
}

int go2rtc_proxy_thread_init(uv_loop_t *loop) {
    if (!loop) {
        log_error("go2rtc_proxy_thread_init: NULL loop");
        return -1;
    }
    if (g_proxy_state.initialized) {
        log_warn("go2rtc_proxy_thread_init: Already initialized");
        return 0;
    }

    memset(&g_proxy_state, 0, sizeof(g_proxy_state));
    g_proxy_state.loop = loop;

    if (pthread_mutex_init(&g_proxy_state.done_mutex, NULL) != 0) {
        log_error("go2rtc_proxy_thread_init: Failed to initialize mutex");
        return -1;
    }

    if (uv_async_init(loop, &g_proxy_state.async_handle, proxy_async_cb) != 0) {
        log_error("go2rtc_proxy_thread_init: Failed to initialize uv_async");
        pthread_mutex_destroy(&g_proxy_state.done_mutex);
        return -1;
    }

    g_proxy_state.initialized = true;
    log_info("go2rtc_proxy_thread_init: Initialized (max %d concurrent proxy threads)",
             MAX_CONCURRENT_PROXY_THREADS);
    return 0;
}

int go2rtc_proxy_thread_submit(libuv_connection_t *conn, write_complete_action_t action) {
    if (!g_proxy_state.initialized) {
        log_error("go2rtc_proxy_thread_submit: Not initialized");
        return -1;
    }
    if (g_proxy_state.shutting_down) {
        log_warn("go2rtc_proxy_thread_submit: Rejecting request during shutdown");
        return -1;
    }
    if (__sync_fetch_and_add(&g_proxy_state.active_count, 0) >= MAX_CONCURRENT_PROXY_THREADS) {
        log_warn("go2rtc_proxy_thread_submit: Max concurrent proxy threads (%d) reached",
                 MAX_CONCURRENT_PROXY_THREADS);
        return -1;
    }

    const http_request_t *req = &conn->request;
    int port = g_config.go2rtc_api_port > 0 ? g_config.go2rtc_api_port : 1984;

    proxy_thread_ctx_t *ctx = safe_calloc(1, sizeof(proxy_thread_ctx_t));
    if (!ctx) {
        log_error("go2rtc_proxy_thread_submit: Failed to allocate context");
        return -1;
    }
    ctx->request = *req;
    ctx->request.user_data = NULL;
    ctx->request.body = NULL;
    ctx->request.body_len = 0;
    http_response_init(&ctx->auth_response);

    // Forward the full path as-is. go2rtc is configured with base_path: /go2rtc
    // (see GO2RTC_BASE_PATH), so it serves /go2rtc/api/streams directly.
    if (req->query_string[0] != '\0') {
        snprintf(ctx->url, sizeof(ctx->url), "http://127.0.0.1:%d%s?%s",
                 port, req->path, req->query_string);
    } else {
        snprintf(ctx->url, sizeof(ctx->url), "http://127.0.0.1:%d%s",
                 port, req->path);
    }

    safe_strcpy(ctx->method, req->method_str, sizeof(ctx->method), 0);
    safe_strcpy(ctx->content_type, req->content_type, sizeof(ctx->content_type), 0);
    ctx->action = action;
    ctx->conn   = conn;

    if (req->body && req->body_len > 0) {
        ctx->body = malloc(req->body_len);
        if (!ctx->body) {
            log_error("go2rtc_proxy_thread_submit: Failed to copy request body");
            free_proxy_context(ctx);
            return -1;
        }
        memcpy(ctx->body, req->body, req->body_len);
        ctx->body_len = req->body_len;
        ctx->request.body = ctx->body;
        ctx->request.body_len = ctx->body_len;
    }

    conn->async_response_pending = true;
    __sync_add_and_fetch(&g_proxy_state.active_count, 1);

    pthread_t thread;
    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);

    if (pthread_create(&thread, &attr, proxy_worker_thread, ctx) != 0) {
        log_error("go2rtc_proxy_thread_submit: pthread_create failed");
        pthread_attr_destroy(&attr);
        __sync_sub_and_fetch(&g_proxy_state.active_count, 1);
        conn->async_response_pending = false;
        free_proxy_context(ctx);
        return -1;
    }

    pthread_attr_destroy(&attr);
    log_debug("go2rtc_proxy_thread_submit: Submitted %s %s", ctx->method, ctx->url);
    return 0;
}

void go2rtc_proxy_thread_shutdown(void) {
    if (!g_proxy_state.initialized) return;

    log_info("go2rtc_proxy_thread_shutdown: Shutting down");
    g_proxy_state.shutting_down = true;

    // Wait up to 10 s for in-flight threads to finish
    int wait_count = 0;
    while (__sync_fetch_and_add(&g_proxy_state.active_count, 0) > 0 && wait_count < 100) {
        usleep(100000);  // 100 ms
        wait_count++;
    }
    if (g_proxy_state.active_count > 0) {
        log_warn("go2rtc_proxy_thread_shutdown: %d threads still active after timeout",
                 g_proxy_state.active_count);
    }

    if (!uv_is_closing((uv_handle_t *)&g_proxy_state.async_handle)) {
        uv_close((uv_handle_t *)&g_proxy_state.async_handle, NULL);
    }
    pthread_mutex_destroy(&g_proxy_state.done_mutex);

    // Drain any remaining done-queue items
    proxy_thread_ctx_t *ctx = g_proxy_state.done_head;
    while (ctx) {
        proxy_thread_ctx_t *next = ctx->next;
        free_proxy_context(ctx);
        ctx = next;
    }

    g_proxy_state.initialized = false;
    log_info("go2rtc_proxy_thread_shutdown: Shutdown complete");
}

#endif /* HTTP_BACKEND_LIBUV */
