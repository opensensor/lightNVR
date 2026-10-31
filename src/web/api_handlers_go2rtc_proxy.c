/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2025-2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/**
 * @file api_handlers_go2rtc_proxy.c
 * @brief Streaming reverse proxy handler for forwarding requests to go2rtc
 *
 * Proxies HLS, snapshots and WebRTC signaling to the local go2rtc instance with:
 * - Streaming pass-through (no buffering)
 * - Concurrency limiting (semaphore-based)
 * - Scoped to explicitly authorized endpoints only
 */

#include "video/go2rtc/go2rtc_api.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <curl/curl.h>
#include <pthread.h>
#include <semaphore.h>
#include <ctype.h>
#include <time.h>

#include "web/api_handlers_go2rtc_proxy.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "core/config.h"
#define LOG_COMPONENT "go2rtcAPI"
#include "core/logger.h"
#include "utils/memory.h"
#include "utils/strings.h"
#include "web/audit_log.h"

#ifdef HTTP_BACKEND_LIBUV
#include "web/libuv_connection.h"
#include "web/libuv_server.h"
#endif

// Concurrency limiter
static sem_t *g_proxy_semaphore = NULL;
static pthread_once_t g_proxy_init_once = PTHREAD_ONCE_INIT;

#define HLS_SESSION_CACHE_CAPACITY \
    ((MAX_STREAMS * 2 < 64) ? 64 : ((MAX_STREAMS * 2 > 1024) ? 1024 : MAX_STREAMS * 2))
#define HLS_SESSION_AUTH_TTL_SECONDS 30
#define HLS_SESSION_ID_MAX 128

typedef struct {
    bool occupied;
    char session_id[HLS_SESSION_ID_MAX];
    char auth_fingerprint[65];
    char stream_name[MAX_STREAM_NAME];
    time_t authorized_until;
    time_t last_used;
} hls_session_auth_t;

static hls_session_auth_t g_hls_sessions[HLS_SESSION_CACHE_CAPACITY];
static pthread_mutex_t g_hls_sessions_mutex = PTHREAD_MUTEX_INITIALIZER;

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
    if (max_inflight < 1) max_inflight = 16;

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

typedef enum {
    GO2RTC_PROXY_NONE,
    GO2RTC_PROXY_STREAMS,
    GO2RTC_PROXY_RELOAD,
    GO2RTC_PROXY_MANIFEST,
    GO2RTC_PROXY_SNAPSHOT,
    GO2RTC_PROXY_WEBRTC,
    GO2RTC_PROXY_WEBSOCKET,
    GO2RTC_PROXY_HLS
} go2rtc_proxy_endpoint_t;

static go2rtc_proxy_endpoint_t classify_proxy_path(const char *path) {
    if (!path) return GO2RTC_PROXY_NONE;
    if (strcmp(path, "/go2rtc/api/streams") == 0) return GO2RTC_PROXY_STREAMS;
    if (strcmp(path, "/go2rtc/api/reload") == 0) return GO2RTC_PROXY_RELOAD;
    if (strcmp(path, "/go2rtc/api/stream.m3u8") == 0) return GO2RTC_PROXY_MANIFEST;
    if (strcmp(path, "/go2rtc/api/frame.jpeg") == 0) return GO2RTC_PROXY_SNAPSHOT;
    if (strcmp(path, "/go2rtc/api/webrtc") == 0) return GO2RTC_PROXY_WEBRTC;
    if (strcmp(path, "/go2rtc/api/ws") == 0) return GO2RTC_PROXY_WEBSOCKET;
    // go2rtc uses fixed HLS filenames; session IDs belong in the query string.
    // Exact matches reject dot segments, encoded separators and path suffixes
    // before curl or go2rtc can interpret them as a different API endpoint.
    if (strcmp(path, "/go2rtc/api/hls/playlist.m3u8") == 0 ||
        strcmp(path, "/go2rtc/api/hls/init.mp4") == 0 ||
        strcmp(path, "/go2rtc/api/hls/segment.ts") == 0 ||
        strcmp(path, "/go2rtc/api/hls/segment.m4s") == 0) return GO2RTC_PROXY_HLS;
    return GO2RTC_PROXY_NONE;
}

bool go2rtc_proxy_path_matches(const char *path) {
    return classify_proxy_path(path) != GO2RTC_PROXY_NONE;
}

static void audit_hls_session_denial(const http_request_t *req) {
    user_t user;
    user_t *principal = NULL;
    memset(&user, 0, sizeof(user));
    if (httpd_check_action_access(req, &user)) principal = &user;
    cJSON *details = cJSON_CreateObject();
    if (details) {
        cJSON_AddStringToObject(details, "event_type",
                                "authorization.media_session_denied");
        cJSON_AddStringToObject(details, "reason",
                                "missing, expired, or credential-mismatched HLS session");
    }
    audit_log_append(req, principal, "live.view", "camera", NULL,
                     "denied", details);
    cJSON_Delete(details);
}

/* Decode each value once without truncating encoded bytes. Only playback
 * options are accepted, and every key is unique. Re-encode the parsed values
 * for forwarding so authorization, session capture and go2rtc agree. */
typedef struct {
    char source[MAX_STREAM_NAME];
    char session[HLS_SESSION_ID_MAX];
    char canonical[2048];
} proxy_query_t;

static bool decode_query_value(const char *begin, const char *end,
                               char *value, size_t capacity) {
    size_t length = 0;
    for (const unsigned char *p = (const unsigned char *)begin;
         p < (const unsigned char *)end; p++) {
        unsigned char ch = *p;
        if (ch == ';' || ch == '#' || isspace(ch)) return false;
        if (ch == '%') {
            if ((const char *)p + 2 >= end || !isxdigit(p[1]) || !isxdigit(p[2])) return false;
            unsigned int decoded;
            if (sscanf((const char *)p + 1, "%2x", &decoded) != 1) return false;
            ch = (unsigned char)decoded;
            p += 2;
        } else if (ch == '+') {
            ch = ' ';
        }
        if (ch < 0x20 || ch == 0x7f || length + 1 >= capacity) return false;
        value[length++] = (char)ch;
    }
    value[length] = '\0';
    return true;
}

static bool query_unreserved(unsigned char ch) {
    return (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') ||
           (ch >= '0' && ch <= '9') || ch == '-' || ch == '_' || ch == '.' || ch == '~';
}

static bool parse_proxy_query(const http_request_t *req, proxy_query_t *parsed) {
    static const char *keys[] = {
        "src", "id", "mp4", "video", "audio", "cache", "t", "n",
        "width", "height", "w", "h", "rotate"
    };
    go2rtc_proxy_endpoint_t endpoint = classify_proxy_path(req->path);
    memset(parsed, 0, sizeof(*parsed));
    if (endpoint == GO2RTC_PROXY_NONE) return false;
    if (endpoint == GO2RTC_PROXY_RELOAD) return req->query_string[0] == '\0';
    if (endpoint == GO2RTC_PROXY_STREAMS && req->query_string[0] == '\0') return true;
    unsigned int seen = 0;
    size_t used = 0;
    for (const char *p = req->query_string; *p;) {
        const char *end = strchr(p, '&');
        if (!end) end = p + strlen(p);
        const char *eq = memchr(p, '=', (size_t)(end - p));
        const char *key_end = eq ? eq : end;
        int key = -1;
        for (size_t i = 0; i < sizeof(keys) / sizeof(keys[0]); i++) {
            if (strlen(keys[i]) == (size_t)(key_end - p) &&
                memcmp(p, keys[i], (size_t)(key_end - p)) == 0) key = (int)i;
        }
        if (key < 0 || (seen & (1U << key))) return false;
        bool allowed = false;
        if (endpoint == GO2RTC_PROXY_HLS) {
            allowed = key == 1 || key == 7; // id, segment sequence
        } else {
            allowed = key == 0;
            if (endpoint == GO2RTC_PROXY_MANIFEST) allowed |= key >= 2 && key <= 4;
            if (endpoint == GO2RTC_PROXY_SNAPSHOT) allowed |= key == 5 || key == 6 || key >= 8;
        }
        if (!allowed) return false;
        seen |= 1U << key;
        char value[MAX_STREAM_NAME];
        size_t capacity = key == 1 ? sizeof(parsed->session) : sizeof(value);
        if (!decode_query_value(eq ? eq + 1 : end, end, value, capacity)) return false;
        if (key == 0) {
            if (!*value) return false;
            strcpy(parsed->source, value);
        } else if (key == 1) {
            if (!*value) return false;
            for (const unsigned char *v = (unsigned char *)value; *v; v++)
                if (!query_unreserved(*v)) return false;
            strcpy(parsed->session, value);
        }
        size_t key_length = strlen(keys[key]);
        if (used + key_length + 2 >= sizeof(parsed->canonical)) return false;
        if (used) parsed->canonical[used++] = '&';
        memcpy(parsed->canonical + used, keys[key], key_length);
        used += key_length;
        parsed->canonical[used++] = '=';
        for (const unsigned char *v = (unsigned char *)value; *v; v++) {
            size_t needed = query_unreserved(*v) ? 1 : 3;
            if (used + needed >= sizeof(parsed->canonical)) return false;
            if (needed == 1) parsed->canonical[used++] = (char)*v;
            else {
                snprintf(parsed->canonical + used, 4, "%%%02X", *v);
                used += 3;
            }
        }
        parsed->canonical[used] = '\0';
        if (!*end) break;
        p = end + 1;
        if (!*p) return false;
    }
    return endpoint == GO2RTC_PROXY_HLS ? parsed->session[0] != '\0' : parsed->source[0] != '\0';
}

bool go2rtc_proxy_parse_source(const http_request_t *req, char *name, size_t capacity) {
    proxy_query_t query;
    if (!req || !name || !parse_proxy_query(req, &query) || !query.source[0] ||
        strlen(query.source) >= capacity) return false;
    strcpy(name, query.source);
    return true;
}

bool go2rtc_proxy_build_url(const http_request_t *req, char *url, size_t capacity) {
    proxy_query_t query;
    if (!req || !url || !parse_proxy_query(req, &query)) return false;
    int port = g_config.go2rtc_api_port > 0 ? g_config.go2rtc_api_port : 1984;
    int length = snprintf(url, capacity, "http://127.0.0.1:%d%s%s%s%s", port,
        go2rtc_api_get_base_path(), req->path + strlen(GO2RTC_BASE_PATH),
        query.canonical[0] ? "?" : "", query.canonical);
    return length >= 0 && (size_t)length < capacity;
}

static bool authorize_source(const http_request_t *req, http_response_t *res,
                             authorization_action_t action, const char *source) {
    if (httpd_authorize_media_stream_action(req, res, action, source)) return true;
    // Managed substreams inherit the parent camera's policy only when there
    // is no actual camera with that name. Never turn a denial into a fallback.
    size_t length = strlen(source);
    if (res->status_code != 404 || length <= 4 || strcmp(source + length - 4, "_sub") != 0)
        return false;
    char parent[MAX_STREAM_NAME];
    memcpy(parent, source, length - 4);
    parent[length - 4] = '\0';
    http_response_free(res);
    http_response_init(res);
    return httpd_authorize_media_stream_action(req, res, action, parent) != 0;
}

static bool authorize_hls_session_request(const http_request_t *req,
                                          http_response_t *res) {
    char session_id[HLS_SESSION_ID_MAX] = {0};
    char fingerprint[65] = {0};
    proxy_query_t query;
    if (!parse_proxy_query(req, &query)) {
        http_response_set_json_error(res, 400, "Invalid HLS query");
        return false;
    }
    strcpy(session_id, query.session);
    if (httpd_request_auth_fingerprint(req, fingerprint) != 0) {
        audit_hls_session_denial(req);
        http_response_set_json_error(res, 401, "Unauthorized HLS session");
        return false;
    }

    time_t now = time(NULL);
    char stream_name[MAX_STREAM_NAME] = {0};
    bool found = false;
    bool current = false;
    pthread_mutex_lock(&g_hls_sessions_mutex);
    for (int i = 0; i < HLS_SESSION_CACHE_CAPACITY; i++) {
        hls_session_auth_t *entry = &g_hls_sessions[i];
        if (!entry->occupied || strcmp(entry->session_id, session_id) != 0 ||
            strcmp(entry->auth_fingerprint, fingerprint) != 0) continue;
        found = true;
        current = entry->authorized_until >= now;
        entry->last_used = now;
        safe_strcpy(stream_name, entry->stream_name,
                    sizeof(stream_name), 0);
        break;
    }
    pthread_mutex_unlock(&g_hls_sessions_mutex);

    if (!found) {
        audit_hls_session_denial(req);
        http_response_set_json_error(res, 403, "Forbidden HLS session");
        return false;
    }
    if (current) return true;

    // Re-check policy at a bounded interval. This retains long-running HLS
    // sessions while limiting role, selector, token, and session revocation
    // staleness to the media authorization window.
    if (!authorize_source(req, res, AUTHZ_LIVE_VIEW, stream_name)) return false;
    pthread_mutex_lock(&g_hls_sessions_mutex);
    for (int i = 0; i < HLS_SESSION_CACHE_CAPACITY; i++) {
        hls_session_auth_t *entry = &g_hls_sessions[i];
        if (entry->occupied && strcmp(entry->session_id, session_id) == 0 &&
            strcmp(entry->auth_fingerprint, fingerprint) == 0) {
            entry->authorized_until = now + HLS_SESSION_AUTH_TTL_SECONDS;
            entry->last_used = now;
            break;
        }
    }
    pthread_mutex_unlock(&g_hls_sessions_mutex);
    return true;
}


bool go2rtc_proxy_authorize_request(const http_request_t *req,
                                    http_response_t *res) {
    if (!req || !res) return false;
    go2rtc_proxy_endpoint_t endpoint = classify_proxy_path(req->path);
    if (endpoint == GO2RTC_PROXY_NONE) {
        http_response_set_json_error(res, 404, "Not found");
        return false;
    }
    const char *allowed_method =
        endpoint == GO2RTC_PROXY_RELOAD || endpoint == GO2RTC_PROXY_WEBRTC ? "POST" : "GET";
    if (strcmp(req->method_str, allowed_method) != 0) {
        http_response_set_json_error(res, 405, "Method not allowed");
        http_response_add_header(res, "Allow", allowed_method);
        return false;
    }
    if (endpoint == GO2RTC_PROXY_STREAMS || endpoint == GO2RTC_PROXY_RELOAD) {
        return httpd_authorize_global_action(req, res, AUTHZ_SYSTEM_ADMIN) != 0;
    }
    if (endpoint == GO2RTC_PROXY_HLS) return authorize_hls_session_request(req, res);
    char stream_name[MAX_STREAM_NAME] = {0};
    if (!go2rtc_proxy_parse_source(req, stream_name, sizeof(stream_name))) {
        http_response_set_json_error(res, 400, "Invalid playback query: expected one src camera");
        return false;
    }
    authorization_action_t action = endpoint == GO2RTC_PROXY_SNAPSHOT
        ? AUTHZ_SNAPSHOT_CREATE : AUTHZ_LIVE_VIEW;
    return authorize_source(req, res, action, stream_name);
}

static void cache_hls_session(const char *session_id,
                              const char *fingerprint,
                              const char *stream_name) {
    if (!session_id || !fingerprint || !stream_name ||
        session_id[0] == '\0' || stream_name[0] == '\0' ||
        strlen(session_id) >= HLS_SESSION_ID_MAX ||
        strlen(fingerprint) >= sizeof(g_hls_sessions[0].auth_fingerprint) || strlen(stream_name) >= MAX_STREAM_NAME) return;
    time_t now = time(NULL);
    pthread_mutex_lock(&g_hls_sessions_mutex);
    int selected = -1;
    int first_free = -1;
    int least_recent = 0;
    time_t oldest = g_hls_sessions[0].last_used;
    for (int i = 0; i < HLS_SESSION_CACHE_CAPACITY; i++) {
        hls_session_auth_t *entry = &g_hls_sessions[i];
        if (entry->occupied &&
            strcmp(entry->session_id, session_id) == 0 &&
            strcmp(entry->auth_fingerprint, fingerprint) == 0) {
            selected = i;
            break;
        }
        if (!entry->occupied) {
            if (first_free < 0) first_free = i;
            continue;
        }
        if (entry->last_used < oldest) {
            least_recent = i;
            oldest = entry->last_used;
        }
    }
    if (selected < 0) selected = first_free >= 0 ? first_free : least_recent;
    hls_session_auth_t *entry = &g_hls_sessions[selected];
    memset(entry, 0, sizeof(*entry));
    entry->occupied = true;
    safe_strcpy(entry->session_id, session_id, sizeof(entry->session_id), 0);
    safe_strcpy(entry->auth_fingerprint, fingerprint,
                sizeof(entry->auth_fingerprint), 0);
    safe_strcpy(entry->stream_name, stream_name,
                sizeof(entry->stream_name), 0);
    entry->authorized_until = now + HLS_SESSION_AUTH_TTL_SECONDS;
    entry->last_used = now;
    pthread_mutex_unlock(&g_hls_sessions_mutex);
}

void go2rtc_proxy_capture_hls_sessions(const http_request_t *req,
                                       const void *manifest,
                                       size_t manifest_size) {
    if (!req || !manifest || manifest_size == 0 ||
        classify_proxy_path(req->path) != GO2RTC_PROXY_MANIFEST) return;
    char stream_name[MAX_STREAM_NAME] = {0};
    char fingerprint[65] = {0};
    if (!go2rtc_proxy_parse_source(req, stream_name, sizeof(stream_name)) ||
        httpd_request_auth_fingerprint(req, fingerprint) != 0) return;

    go2rtc_proxy_capture_hls_sessions_for(stream_name, fingerprint, manifest,
                                           manifest_size);
}

void go2rtc_proxy_capture_hls_sessions_for(const char *stream_name,
                                           const char *auth_fingerprint,
                                           const void *manifest,
                                           size_t manifest_size) {
    if (!stream_name || !auth_fingerprint || !manifest ||
        manifest_size == 0) return;
    const unsigned char *bytes = manifest;
    for (size_t i = 0; i + 3 < manifest_size; i++) {
        if (i == 0 || (bytes[i - 1] != '?' && bytes[i - 1] != '&') ||
            bytes[i] != 'i' || bytes[i + 1] != 'd' || bytes[i + 2] != '=') {
            continue;
        }
        char session_id[HLS_SESSION_ID_MAX] = {0};
        size_t written = 0;
        size_t j = i + 3;
        while (j < manifest_size && query_unreserved(bytes[j])) {
            if (written + 1 < sizeof(session_id)) session_id[written++] = (char)bytes[j];
            j++;
        }
        // Never bind a prefix of an overlong upstream session identifier.
        if (j - (i + 3) >= sizeof(session_id)) continue;
        if (written > 0) {
            cache_hls_session(session_id, auth_fingerprint, stream_name);
        }
    }
}

void handle_go2rtc_proxy(const http_request_t *req, http_response_t *res) {
    if (!go2rtc_proxy_authorize_request(req, res)) return;

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
    if (!go2rtc_proxy_build_url(req, url, sizeof(url))) {
        sem_post(g_proxy_semaphore);
        http_response_set_json_error(res, 400, "Invalid proxy query");
        return;
    }
    // WebSocket upgrades are handled on the event loop, never by buffered curl.
    if (strcmp(req->path, "/go2rtc/api/ws") == 0) {
        sem_post(g_proxy_semaphore);
        http_response_set_json_error(res, 400, "WebSocket upgrade required");
        return;
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

    // Configure curl for buffered response.
    // PATH_AS_IS preserves the exact, already-validated path, but it is NOT
    // sufficient on its own: even with the path preserved, go2rtc's traversal
    // redirects would re-open the bypass by pointing curl at an endpoint that
    // was never authorized. Disabling redirect following below is what
    // actually closes it.
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_PATH_AS_IS, 1L);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, buffered_write_cb);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &ctx);
    curl_easy_setopt(curl, CURLOPT_HEADERFUNCTION, buffered_header_cb);
    curl_easy_setopt(curl, CURLOPT_HEADERDATA, &ctx);
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 30L);
    curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT, 5L);
    curl_easy_setopt(curl, CURLOPT_NOSIGNAL, 1L);
    // A redirect target has not been authorized. Never follow it internally —
    // this (not PATH_AS_IS) is what closes the traversal-redirect bypass.
    curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, 0L);

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
        safe_strcpy(res->content_type, ctx.content_type, sizeof(res->content_type), 0);
    }

    // Add CORS headers
    http_response_add_header(res, "Access-Control-Allow-Origin", "*");
    http_response_add_header(res, "Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS");
    http_response_add_header(res, "Access-Control-Allow-Headers", "Content-Type, Authorization");

    // Set body (transfer ownership)
    res->body = ctx.buffer;
    res->body_length = ctx.buffer_size;
    res->body_allocated = true;

    if (ctx.http_code == 200)
        go2rtc_proxy_capture_hls_sessions(req, ctx.buffer, ctx.buffer_size);

    log_debug("go2rtc proxy: %s %s -> %ld (%zu bytes, type: %s)",
              req->method_str, req->path, ctx.http_code, ctx.buffer_size,
              ctx.content_type[0] ? ctx.content_type : "unknown");
}
