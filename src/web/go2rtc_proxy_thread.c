/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

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

#include "video/go2rtc/go2rtc_api.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <unistd.h>
#include <strings.h>
#include <time.h>
#include <stdatomic.h>
#include <curl/curl.h>

#include "web/go2rtc_proxy_thread.h"
#include "web/go2rtc_websocket.h"
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
    bool websocket_request;
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
    pthread_cond_t workers_done;
    proxy_thread_ctx_t *done_head;
    proxy_thread_ctx_t *done_tail;
    atomic_int active_count;
    atomic_bool shutting_down;
    bool close_pending;
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

static int proxy_progress_cb(void *unused, curl_off_t download_total,
                             curl_off_t downloaded, curl_off_t upload_total,
                             curl_off_t uploaded) {
    (void)unused; (void)download_total; (void)downloaded;
    (void)upload_total; (void)uploaded;
    return atomic_load(&g_proxy_state.shutting_down) ? 1 : 0;
}

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
    if (ctx->authorization_denied || atomic_load(&g_proxy_state.shutting_down)) goto done;
    if (!go2rtc_proxy_build_url(&ctx->request, ctx->url, sizeof(ctx->url))) {
        http_response_set_json_error(&ctx->auth_response, 400, "Invalid proxy query");
        ctx->authorization_denied = true;
        goto done;
    }
    if (strcmp(ctx->request.path, "/go2rtc/api/ws") == 0) {
        ctx->websocket_request = go2rtc_websocket_validate(&ctx->request, &ctx->auth_response);
        ctx->authorization_denied = !ctx->websocket_request;
        goto done;
    }

    if (strcmp(ctx->request.path, "/go2rtc/api/stream.m3u8") == 0 &&
        go2rtc_proxy_parse_source(&ctx->request, ctx->capture_stream_name,
                                   sizeof(ctx->capture_stream_name)) &&
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

    // PATH_AS_IS preserves the exact, already-validated path, but it is NOT
    // sufficient on its own: even with the path preserved, go2rtc's traversal
    // redirects would re-open the bypass by pointing curl at an endpoint that
    // was never authorized. Disabling redirect following below is what
    // actually closes it.
    curl_easy_setopt(curl, CURLOPT_URL, ctx->url);
    curl_easy_setopt(curl, CURLOPT_PATH_AS_IS, 1L);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, proxy_write_cb);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, ctx);
    curl_easy_setopt(curl, CURLOPT_HEADERFUNCTION, proxy_header_cb);
    curl_easy_setopt(curl, CURLOPT_HEADERDATA, ctx);
    /* Snapshot previews should release proxy slots promptly when a source
     * cannot produce a frame. HLS retains its longer streaming timeout. */
    bool snapshot_request = strcmp(ctx->request.path, "/go2rtc/api/frame.jpeg") == 0;
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, snapshot_request ? 10L : 30L);
    curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT, snapshot_request ? 2L : 5L);
    curl_easy_setopt(curl, CURLOPT_NOSIGNAL, 1L);
    curl_easy_setopt(curl, CURLOPT_NOPROGRESS, 0L);
    curl_easy_setopt(curl, CURLOPT_XFERINFOFUNCTION, proxy_progress_cb);
    // A redirect target has not been authorized. Never follow it internally —
    // this (not PATH_AS_IS) is what closes the traversal-redirect bypass.
    curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, 0L);

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
            if (ctx->capture_hls_session && ctx->http_code == 200) {
                go2rtc_proxy_capture_hls_sessions_for(
                    ctx->capture_stream_name, ctx->auth_fingerprint,
                    ctx->response_buffer, ctx->response_size);
            }
        }
    }

done:
    enqueue_done(ctx);
    uv_async_send(&g_proxy_state.async_handle);
    pthread_mutex_lock(&g_proxy_state.done_mutex);
    atomic_fetch_sub(&g_proxy_state.active_count, 1);
    pthread_cond_signal(&g_proxy_state.workers_done);
    pthread_mutex_unlock(&g_proxy_state.done_mutex);
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

        if (ctx->websocket_request) {
            go2rtc_websocket_start(conn, ctx->url);
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

int go2rtc_proxy_thread_init(uv_loop_t *loop) {
    if (!loop) {
        log_error("go2rtc_proxy_thread_init: NULL loop");
        return -1;
    }
    if (g_proxy_state.close_pending) return -1;
    if (g_proxy_state.initialized) {
        log_warn("go2rtc_proxy_thread_init: Already initialized");
        return 0;
    }

    memset(&g_proxy_state, 0, sizeof(g_proxy_state));
    g_proxy_state.loop = loop;
    atomic_init(&g_proxy_state.active_count, 0);
    atomic_init(&g_proxy_state.shutting_down, false);

    if (pthread_mutex_init(&g_proxy_state.done_mutex, NULL) != 0) {
        log_error("go2rtc_proxy_thread_init: Failed to initialize mutex");
        return -1;
    }

    if (pthread_cond_init(&g_proxy_state.workers_done, NULL) != 0) {
        pthread_mutex_destroy(&g_proxy_state.done_mutex);
        return -1;
    }

    if (uv_async_init(loop, &g_proxy_state.async_handle, proxy_async_cb) != 0) {
        log_error("go2rtc_proxy_thread_init: Failed to initialize uv_async");
        pthread_mutex_destroy(&g_proxy_state.done_mutex);
        pthread_cond_destroy(&g_proxy_state.workers_done);
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
    if (atomic_load(&g_proxy_state.active_count) >= MAX_CONCURRENT_PROXY_THREADS) {
        log_warn("go2rtc_proxy_thread_submit: Max concurrent proxy threads (%d) reached",
                 MAX_CONCURRENT_PROXY_THREADS);
        return -1;
    }

    const http_request_t *req = &conn->request;

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
    atomic_fetch_add(&g_proxy_state.active_count, 1);

    pthread_t thread;
    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);

    if (pthread_create(&thread, &attr, proxy_worker_thread, ctx) != 0) {
        log_error("go2rtc_proxy_thread_submit: pthread_create failed");
        pthread_attr_destroy(&attr);
        atomic_fetch_sub(&g_proxy_state.active_count, 1);
        conn->async_response_pending = false;
        free_proxy_context(ctx);
        return -1;
    }

    pthread_attr_destroy(&attr);
    log_debug("go2rtc_proxy_thread_submit: Submitted %s %s", ctx->method, ctx->request.path);
    return 0;
}

static void proxy_async_closed(uv_handle_t *handle) {
    (void)handle;
    g_proxy_state.close_pending = false;
}

void go2rtc_proxy_thread_shutdown(void) {
    if (!g_proxy_state.initialized) return;

    log_info("go2rtc_proxy_thread_shutdown: Shutting down");
    g_proxy_state.shutting_down = true;

    // Curl progress callbacks cancel transfers promptly. Authorization may
    // still be running: wait for every worker's final async send before
    // destroying its queue/handle, even when that takes longer than 10 s.
    pthread_mutex_lock(&g_proxy_state.done_mutex);
    while (atomic_load(&g_proxy_state.active_count) > 0)
        pthread_cond_wait(&g_proxy_state.workers_done, &g_proxy_state.done_mutex);
    pthread_mutex_unlock(&g_proxy_state.done_mutex);

    g_proxy_state.close_pending = true;
    uv_close((uv_handle_t *)&g_proxy_state.async_handle, proxy_async_closed);
    pthread_cond_destroy(&g_proxy_state.workers_done);
    pthread_mutex_destroy(&g_proxy_state.done_mutex);

    // Drain any remaining done-queue items
    proxy_thread_ctx_t *ctx = g_proxy_state.done_head;
    while (ctx) {
        proxy_thread_ctx_t *next = ctx->next;
        free_proxy_context(ctx);
        ctx = next;
    }

    g_proxy_state.done_head = g_proxy_state.done_tail = NULL;
    g_proxy_state.initialized = false;
    log_info("go2rtc_proxy_thread_shutdown: Shutdown complete");
}

#endif /* HTTP_BACKEND_LIBUV */
