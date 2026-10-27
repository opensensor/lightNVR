/* Authenticated, same-origin WebSocket transport for go2rtc playback.
 * Only a single authorized src reaches upstream. No credentials, arbitrary
 * request headers, publishing parameters or administrative routes are relayed.
 * After the upgrade, go2rtc's media handlers use this fixed query for playback.
 */
#ifdef HTTP_BACKEND_LIBUV
#include <ctype.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <strings.h>

#include "web/go2rtc_websocket.h"
#include "web/libuv_connection.h"

#define WS_BUFFER_SIZE (64 * 1024)
#define WS_HEADER_LIMIT 8192
#define WS_CONNECTION_LIMIT ((MAX_STREAMS < 32) ? 64 : MAX_STREAMS * 2)

typedef struct websocket_proxy {
    uv_tcp_t upstream;
    uv_timer_t timeout;
    uv_connect_t connect;
    libuv_connection_t *conn;
    struct websocket_proxy *next;
    unsigned refs;
    bool closing;
    bool upgraded;
    char handshake[4096];
    char headers[WS_HEADER_LIMIT];
    size_t header_size;
} websocket_proxy_t;

typedef struct {
    uv_write_t request;
    websocket_proxy_t *proxy;
    uv_stream_t *resume;
    char *data;
} websocket_write_t;

static websocket_proxy_t *proxies;
static unsigned proxy_count;
static void upstream_read(uv_stream_t *, ssize_t, const uv_buf_t *);
static void client_read(uv_stream_t *, ssize_t, const uv_buf_t *);

static const char *single_header(const http_request_t *req, const char *name) {
    const char *value = NULL;
    for (int i = 0; i < req->num_headers; i++) {
        if (strcasecmp(req->headers[i].name, name) != 0) continue;
        if (value) return NULL;
        value = req->headers[i].value;
    }
    return value;
}

static bool header_token(const char *value, const char *token) {
    if (!value) return false;
    while (*value) {
        while (*value == ' ' || *value == '\t' || *value == ',') value++;
        const char *end = strchr(value, ',');
        if (!end) end = value + strlen(value);
        const char *trimmed = end;
        while (trimmed > value && isspace((unsigned char)trimmed[-1])) trimmed--;
        if ((size_t)(trimmed - value) == strlen(token) &&
            strncasecmp(value, token, strlen(token)) == 0) return true;
        value = *end ? end + 1 : end;
    }
    return false;
}

bool go2rtc_websocket_validate(const http_request_t *req, http_response_t *res) {
    const char *key = single_header(req, "Sec-WebSocket-Key");
    const char *version = single_header(req, "Sec-WebSocket-Version");
    bool valid_key = key && strlen(key) == 24 && strcmp(key + 22, "==") == 0;
    for (int i = 0; valid_key && i < 22; i++)
        valid_key = isalnum((unsigned char)key[i]) || key[i] == '+' || key[i] == '/';
    if (!valid_key || !version || strcmp(version, "13") != 0 ||
        !header_token(single_header(req, "Upgrade"), "websocket") ||
        !header_token(single_header(req, "Connection"), "upgrade") ||
        req->content_length || req->body_len) {
        http_response_set_json_error(res, 400, "WebSocket upgrade required");
        return false;
    }
    // Browsers always supply Origin. Non-browser authenticated clients may
    // omit it. Reject duplicate Origin/Host headers and cross-site cookies.
    const char *origin = single_header(req, "Origin");
    const char *host = single_header(req, "Host");
    int origin_count = 0;
    for (int i = 0; i < req->num_headers; i++)
        if (strcasecmp(req->headers[i].name, "Origin") == 0) origin_count++;
    const char *authority = origin && strncmp(origin, "http://", 7) == 0 ? origin + 7 :
                            origin && strncmp(origin, "https://", 8) == 0 ? origin + 8 : NULL;
    if (!host || origin_count > 1 || (origin && (!authority || strcasecmp(authority, host) != 0))) {
        http_response_set_json_error(res, 403, "WebSocket origin denied");
        return false;
    }
    return true;
}

static void release_proxy(websocket_proxy_t *proxy) {
    if (--proxy->refs) return;
    websocket_proxy_t **link = &proxies;
    while (*link && *link != proxy) link = &(*link)->next;
    if (*link) *link = proxy->next;
    proxy_count--;
    free(proxy);
}

static void handle_closed(uv_handle_t *handle) {
    release_proxy(handle->data);
}

static void close_proxy(websocket_proxy_t *proxy) {
    if (proxy->closing) return;
    proxy->closing = true;
    if (proxy->conn) {
        libuv_connection_t *conn = proxy->conn;
        proxy->conn = NULL;
        conn->go2rtc_websocket = NULL;
        libuv_connection_close(conn);
    }
    uv_read_stop((uv_stream_t *)&proxy->upstream);
    uv_timer_stop(&proxy->timeout);
    uv_close((uv_handle_t *)&proxy->upstream, handle_closed);
    uv_close((uv_handle_t *)&proxy->timeout, handle_closed);
}

static void fail_proxy(websocket_proxy_t *proxy) {
    if (!proxy->upgraded && proxy->conn && !proxy->closing) {
        libuv_connection_t *conn = proxy->conn;
        proxy->conn = NULL;
        conn->go2rtc_websocket = NULL;
        conn->async_response_pending = false;
        http_response_set_json_error(&conn->response, 502, "go2rtc WebSocket unavailable");
        libuv_send_response_ex(conn, &conn->response, WRITE_ACTION_CLOSE);
    }
    close_proxy(proxy);
}

static void allocate_read(uv_handle_t *handle, size_t suggested, uv_buf_t *buf) {
    (void)handle; (void)suggested;
    buf->base = malloc(WS_BUFFER_SIZE);
    buf->len = buf->base ? WS_BUFFER_SIZE : 0;
}

static void write_done(uv_write_t *request, int status) {
    websocket_write_t *write = (websocket_write_t *)request;
    websocket_proxy_t *proxy = write->proxy;
    if (!proxy->closing) {
        if (status < 0) fail_proxy(proxy);
        else if (write->resume) {
            uv_read_cb callback = write->resume == (uv_stream_t *)&proxy->upstream
                ? upstream_read : client_read;
            if (uv_read_start(write->resume, allocate_read, callback) < 0) fail_proxy(proxy);
        }
    }
    free(write->data);
    free(write);
    release_proxy(proxy);
}

/* One pending read buffer per direction bounds memory for slow clients. */
static bool relay(websocket_proxy_t *proxy, uv_stream_t *target,
                  uv_stream_t *resume, const char *data, size_t length) {
    websocket_write_t *write = calloc(1, sizeof(*write));
    if (!write) return false;
    write->data = malloc(length);
    if (!write->data) { free(write); return false; }
    memcpy(write->data, data, length);
    write->proxy = proxy;
    write->resume = resume;
    uv_buf_t buffer = uv_buf_init(write->data, (unsigned int)length);
    if (resume) uv_read_stop(resume);
    if (uv_write(&write->request, target, &buffer, 1, write_done) < 0) {
        free(write->data);
        free(write);
        return false;
    }
    proxy->refs++;
    return true;
}

static void client_read(uv_stream_t *stream, ssize_t count, const uv_buf_t *buf) {
    libuv_connection_t *conn = stream->data;
    websocket_proxy_t *proxy = conn->go2rtc_websocket;
    if (proxy && !proxy->closing) {
        if (count < 0 || (count > 0 && !relay(proxy, (uv_stream_t *)&proxy->upstream,
                                             stream, buf->base, (size_t)count))) close_proxy(proxy);
    }
    free(buf->base);
}

static void upstream_read(uv_stream_t *stream, ssize_t count, const uv_buf_t *buf) {
    websocket_proxy_t *proxy = stream->data;
    if (proxy->closing || count <= 0) {
        if (!proxy->closing && count < 0) fail_proxy(proxy);
        free(buf->base);
        return;
    }
    const char *data = buf->base;
    size_t length = (size_t)count;
    if (!proxy->upgraded) {
        // Accumulate just the HTTP headers, retaining any coalesced frames.
        size_t consumed = 0;
        while (consumed < length && proxy->header_size + 1 < sizeof(proxy->headers)) {
            proxy->headers[proxy->header_size++] = data[consumed++];
            proxy->headers[proxy->header_size] = '\0';
            if (proxy->header_size >= 4 &&
                memcmp(proxy->headers + proxy->header_size - 4, "\r\n\r\n", 4) == 0) break;
        }
        if (!strstr(proxy->headers, "\r\n\r\n")) {
            if (proxy->header_size + 1 == sizeof(proxy->headers)) fail_proxy(proxy);
            free(buf->base);
            return;
        }
        if (strncmp(proxy->headers, "HTTP/1.1 101 ", 13) != 0) {
            fail_proxy(proxy);
            free(buf->base);
            return;
        }
        proxy->upgraded = true;
        uv_timer_stop(&proxy->timeout);
        if (!relay(proxy, (uv_stream_t *)&proxy->conn->handle, NULL,
                   proxy->headers, proxy->header_size) ||
            uv_read_start((uv_stream_t *)&proxy->conn->handle, allocate_read, client_read) < 0) {
            close_proxy(proxy);
            free(buf->base);
            return;
        }
        data += consumed;
        length -= consumed;
    }
    if (length && !relay(proxy, (uv_stream_t *)&proxy->conn->handle, stream, data, length))
        close_proxy(proxy);
    free(buf->base);
}

static void connect_done(uv_connect_t *request, int status) {
    websocket_proxy_t *proxy = request->data;
    if (proxy->closing) return;
    if (status < 0 || !relay(proxy, (uv_stream_t *)&proxy->upstream, NULL,
                             proxy->handshake, strlen(proxy->handshake)) ||
        uv_read_start((uv_stream_t *)&proxy->upstream, allocate_read, upstream_read) < 0)
        fail_proxy(proxy);
}

static void handshake_timeout(uv_timer_t *timer) {
    fail_proxy(timer->data);
}

void go2rtc_websocket_start(libuv_connection_t *conn, const char *url) {
    websocket_proxy_t *proxy = NULL;
    if (proxy_count >= WS_CONNECTION_LIMIT || !(proxy = calloc(1, sizeof(*proxy)))) {
        http_response_set_json_error(&conn->response, 503, "WebSocket capacity reached");
        conn->async_response_pending = false;
        libuv_send_response_ex(conn, &conn->response, WRITE_ACTION_CLOSE);
        return;
    }
    const char *target = strchr(url + strlen("http://"), '/');
    int port = 0;
    (void)sscanf(url, "http://127.0.0.1:%d/", &port);
    const char *key = single_header(&conn->request, "Sec-WebSocket-Key");
    int length = snprintf(proxy->handshake, sizeof(proxy->handshake),
        "GET %s HTTP/1.1\r\nHost: 127.0.0.1:%d\r\nUpgrade: websocket\r\n"
        "Connection: Upgrade\r\nSec-WebSocket-Key: %s\r\nSec-WebSocket-Version: 13\r\n\r\n",
        target ? target : "", port, key ? key : "");
    if (!target || port <= 0 || port > 65535 || length < 0 ||
        (size_t)length >= sizeof(proxy->handshake) ||
        uv_tcp_init(conn->server->loop, &proxy->upstream) < 0) {
        free(proxy);
        http_response_set_json_error(&conn->response, 502, "Invalid WebSocket upstream");
        conn->async_response_pending = false;
        libuv_send_response_ex(conn, &conn->response, WRITE_ACTION_CLOSE);
        return;
    }
    proxy->upstream.data = proxy;
    proxy->refs = 1;
    proxy->next = proxies;
    proxies = proxy;
    proxy_count++;
    if (uv_timer_init(conn->server->loop, &proxy->timeout) < 0) {
        proxy->closing = true;
        uv_close((uv_handle_t *)&proxy->upstream, handle_closed);
        libuv_connection_close(conn);
        return;
    }
    proxy->refs++;
    proxy->timeout.data = proxy;
    proxy->connect.data = proxy;
    proxy->conn = conn;
    conn->go2rtc_websocket = proxy;
    struct sockaddr_in address;
    uv_ip4_addr("127.0.0.1", port, &address);
    uv_timer_start(&proxy->timeout, handshake_timeout, 5000, 0);
    if (uv_tcp_connect(&proxy->connect, &proxy->upstream,
                       (const struct sockaddr *)&address, connect_done) < 0) fail_proxy(proxy);
}

void go2rtc_websocket_disconnected(libuv_connection_t *conn) {
    websocket_proxy_t *proxy = conn->go2rtc_websocket;
    if (!proxy) return;
    conn->go2rtc_websocket = NULL;
    proxy->conn = NULL;
    close_proxy(proxy);
}

void go2rtc_websocket_shutdown(uv_loop_t *loop) {
    for (websocket_proxy_t *proxy = proxies; proxy; proxy = proxy->next)
        if (proxy->upstream.loop == loop) close_proxy(proxy);
}
#endif
