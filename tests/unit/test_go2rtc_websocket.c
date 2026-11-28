#define _POSIX_C_SOURCE 200809L
#include <arpa/inet.h>
#include <errno.h>
#include <pthread.h>
#include <stdatomic.h>
#include <poll.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/time.h>
#include <time.h>
#include <unistd.h>
#include <curl/curl.h>

#include "unity.h"
#include "core/config.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#include "video/go2rtc/go2rtc_api.h"
#include "utils/yaml_validate.h"
#include "web/api_handlers_go2rtc_proxy.h"
#include "web/go2rtc_websocket.h"
#include "web/libuv_server.h"

static http_server_handle_t server;
static int upstream = -1, web_port;
static char db_path[80];
static atomic_bool teardown_started = ATOMIC_VAR_INIT(false);
static const char upgrade_headers[] =
    "Upgrade: websocket\r\nConnection: keep-alive, Upgrade\r\n"
    "Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\nSec-WebSocket-Version: 13\r\n";
static const char upgrade_response[] =
    "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\n"
    "Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=\r\nX-LightNVR-Media-Policy: 1\r\n\r\n";

static void test_websocket_origin_authority_validation(void) {
    const struct { const char *host; const char *origin; bool allowed; bool tls;
                   const char *forwarded_proto; bool trusted_proxy; } cases[] = {
        {"camera", "http://CAMERA:80", true},
        {"camera:8080", "http://camera:8080", true},
        {"camera:443", "https://camera", true, true},
        {"camera:80", "https://camera:80", true, true},
        {"[::1]:8080", "http://[::1]:8080", true},
        {"camera:8080", "http://camera:8081", false},
        {"camera", "http://camera.evil", false},
        {"camera", "http://camera/path", false},
        {"camera", "http://user@camera", false},
        {"camera", "http://camera:0", false},
        {"[::1]:8080", "http://[::2]:8080", false},
        {"camera", "https://camera", false},
        {"camera", "http://camera", false, true},
        {"camera", "https://camera", false, false, "https"},
        {"camera", "https://camera", true, false, "https", true},
        {"camera", "http://camera", false, false, "https", true},
        {"camera", "http://camera", false, false, "invalid", true},
        {"camera", NULL, true},
    };
    for (size_t i = 0; i < sizeof(cases) / sizeof(cases[0]); i++) {
        http_request_t request = {0};
        request.is_tls = cases[i].tls;
        if (cases[i].trusted_proxy) {
            snprintf(request.client_ip, sizeof(request.client_ip), "127.0.0.1");
            snprintf(g_config.trusted_proxy_cidrs, sizeof(g_config.trusted_proxy_cidrs), "127.0.0.1/32");
        } else {
            g_config.trusted_proxy_cidrs[0] = '\0';
        }
        http_response_t response;
        http_response_init(&response);
        const char *names[] = {"Host", "Origin", "Upgrade", "Connection",
                               "Sec-WebSocket-Key", "Sec-WebSocket-Version"};
        const char *values[] = {cases[i].host, cases[i].origin, "websocket", "Upgrade",
                                "dGhlIHNhbXBsZSBub25jZQ==", "13"};
        for (size_t j = 0; j < sizeof(names) / sizeof(names[0]); j++) {
            if (!values[j]) continue;
            http_header_t *header = &request.headers[request.num_headers++];
            snprintf(header->name, sizeof(header->name), "%s", names[j]);
            snprintf(header->value, sizeof(header->value), "%s", values[j]);
        }
        if (cases[i].forwarded_proto) {
            http_header_t *header = &request.headers[request.num_headers++];
            snprintf(header->name, sizeof(header->name), "X-Forwarded-Proto");
            snprintf(header->value, sizeof(header->value), "%s", cases[i].forwarded_proto);
        }
        TEST_ASSERT_EQUAL_INT_MESSAGE(cases[i].allowed,
            go2rtc_websocket_validate(&request, &response),
            cases[i].origin ? cases[i].origin : "missing Origin");
        http_response_free(&response);
    }
    g_config.trusted_proxy_cidrs[0] = '\0';
}
/* A masked text frame containing {"type":"mse"}. */
static const unsigned char mse_request[] = {
    0x81, 0x8e, 0, 0, 0, 0, '{', '"', 't', 'y', 'p', 'e', '"', ':', '"', 'm', 's', 'e', '"', '}'
};
static const unsigned char media_response[] = {0x82, 0x08, 0, 0, 0, 8, 'f', 't', 'y', 'p'};

typedef struct {
    int fd;
    int status;
    char request[4096];
    bool received_mse;
    bool observed_disconnect;
    bool observed_client_ws_data_after_reject;
    bool omit_media_ack;
    bool bad_accept;
} peer_t;

static int tracked_clients[5];
static pthread_t tracked_threads[4];
static bool tracked_thread_started[4];
static peer_t tracked_peers[4];

static void socket_timeout(int fd) {
    struct timeval timeout = {.tv_sec = 3};
    setsockopt(fd, SOL_SOCKET, SO_RCVTIMEO, &timeout, sizeof(timeout));
    setsockopt(fd, SOL_SOCKET, SO_SNDTIMEO, &timeout, sizeof(timeout));
}
static bool send_all(int fd, const void *bytes, size_t length) {
    const char *data = bytes;
    while (length) {
        ssize_t n = send(fd, data, length, MSG_NOSIGNAL);
        if (n < 0 && errno == EINTR) continue;
        if (n <= 0) return false;
        data += n;
        length -= (size_t)n;
    }
    return true;
}
static bool read_all(int fd, void *bytes, size_t length) {
    char *data = bytes;
    while (length) {
        ssize_t n = recv(fd, data, length, 0);
        if (n < 0 && errno == EINTR) continue;
        if (n <= 0) return false;
        data += n;
        length -= (size_t)n;
    }
    return true;
}
static bool read_headers(int fd, char *data, size_t capacity) {
    size_t used = 0;
    while (used + 1 < capacity) {
        if (!read_all(fd, data + used, 1)) return false;
        data[++used] = '\0';
        if (used >= 4 && memcmp(data + used - 4, "\r\n\r\n", 4) == 0) return true;
    }
    return false;
}
static int client_open(void) {
    int fd = socket(AF_INET, SOCK_STREAM, 0);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, fd);
    socket_timeout(fd);
    struct sockaddr_in address = {.sin_family = AF_INET,
        .sin_addr.s_addr = htonl(INADDR_LOOPBACK), .sin_port = htons(web_port)};
    int rc = connect(fd, (struct sockaddr *)&address, sizeof(address));
    if (rc != 0) close(fd);
    TEST_ASSERT_EQUAL_INT(0, rc);
    return fd;
}
static void *upstream_peer(void *arg) {
    peer_t *peer = arg;
    struct pollfd ready = {.fd = upstream, .events = POLLIN};
    if (poll(&ready, 1, 3000) <= 0 || !(ready.revents & POLLIN)) return NULL;
    peer->fd = accept(ready.fd, NULL, NULL);
    if (peer->fd < 0) return NULL;
    socket_timeout(peer->fd);
    if (!read_headers(peer->fd, peer->request, sizeof(peer->request))) goto done;
    if (peer->status != 101) {
        const char error[] = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";
        send_all(peer->fd, error, sizeof(error) - 1);
        struct pollfd ready = {.fd = peer->fd, .events = POLLIN};
        if (poll(&ready, 1, 200) > 0 && (ready.revents & POLLIN)) {
            unsigned char byte;
            peer->observed_client_ws_data_after_reject = recv(peer->fd, &byte, 1, MSG_DONTWAIT) > 0;
        }
        goto done;
    }
    if (peer->omit_media_ack) {
        const char unsupported[] = "HTTP/1.1 101 Switching Protocols\r\n"
            "Upgrade: websocket\r\nConnection: Upgrade\r\n"
            "Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=\r\n\r\n";
        send_all(peer->fd, unsupported, sizeof(unsupported) - 1);
        send_all(peer->fd, media_response, sizeof(media_response));
        struct pollfd ready = {.fd = peer->fd, .events = POLLIN};
        if (poll(&ready, 1, 200) > 0 && (ready.revents & POLLIN)) {
            unsigned char byte;
            peer->observed_client_ws_data_after_reject = recv(peer->fd, &byte, 1, MSG_DONTWAIT) > 0;
        }
        goto done;
    }
    if (peer->bad_accept) {
        const char invalid[] = "HTTP/1.1 101 Switching Protocols\r\n"
            "Upgrade: websocket\r\nConnection: Upgrade\r\n"
            "Sec-WebSocket-Accept: invalid\r\nX-LightNVR-Media-Policy: 1\r\n\r\n";
        send_all(peer->fd, invalid, sizeof(invalid) - 1);
        struct pollfd ready = {.fd = peer->fd, .events = POLLIN};
        if (poll(&ready, 1, 200) > 0 && (ready.revents & POLLIN)) {
            unsigned char byte;
            peer->observed_client_ws_data_after_reject = recv(peer->fd, &byte, 1, MSG_DONTWAIT) > 0;
        }
        goto done;
    }
    // Send the upgrade and first frame together; the client must handle
    // coalescing or splitting because TCP does not preserve write boundaries.
    unsigned char combined[sizeof(upgrade_response) - 1 + sizeof(media_response)];
    memcpy(combined, upgrade_response, sizeof(upgrade_response) - 1);
    memcpy(combined + sizeof(upgrade_response) - 1, media_response, sizeof(media_response));
    if (!send_all(peer->fd, combined, sizeof(combined))) goto done;
    unsigned char frame[sizeof(mse_request)];
    if (read_all(peer->fd, frame, sizeof(frame))) {
        peer->received_mse = memcmp(frame, mse_request, sizeof(frame)) == 0;
        send_all(peer->fd, media_response, sizeof(media_response));
    }
    // Keep the socket alive until its consumer or the server disconnects.
    // Start the bounded disconnect wait only once teardown begins.
    peer->observed_disconnect = false;
    char byte;
    struct timespec start, now;
    const long teardown_budget_ms = 5000;
    while (!atomic_load(&teardown_started)) {
        struct timespec pause = {.tv_sec = 0, .tv_nsec = 10 * 1000 * 1000}; // 10ms
        nanosleep(&pause, NULL);
    }
    if (clock_gettime(CLOCK_MONOTONIC, &start) != 0) goto done;
    for (;;) {
        if (clock_gettime(CLOCK_MONOTONIC, &now) != 0) break;
        long elapsed_ms = (now.tv_sec - start.tv_sec) * 1000L +
            (now.tv_nsec - start.tv_nsec) / 1000000L;
        long remaining_ms = teardown_budget_ms - elapsed_ms;
        if (remaining_ms <= 0) break;

        struct pollfd pfd = {.fd = peer->fd, .events = POLLIN};
        int pr = poll(&pfd, 1, (int)remaining_ms);
        if (pr == 0) break; // deadline reached without further events
        if (pr < 0) {
            if (errno == EINTR) continue;
            break;
        }
        if (!(pfd.revents & (POLLIN | POLLERR | POLLHUP | POLLNVAL))) continue;

        ssize_t n = recv(peer->fd, &byte, 1, 0);
        if (n == 0) {
            peer->observed_disconnect = true; // remote closed
            break;
        }
        if (n > 0) continue;
        if (errno == EINTR) continue;
        if (errno == EAGAIN || errno == EWOULDBLOCK) continue;
        if (errno == ECONNRESET || errno == ENOTCONN || errno == EPIPE) {
            peer->observed_disconnect = true;
        }
        break; // other socket error
    }
done:
    close(peer->fd);
    return NULL;
}
void setUp(void) {
    atomic_store(&teardown_started, false);
    for (int i = 0; i < 4; i++) {
        tracked_clients[i] = -1;
        tracked_thread_started[i] = false;
        memset(&tracked_peers[i], 0, sizeof(tracked_peers[i]));
    }
    tracked_clients[4] = -1;
    load_default_config(&g_config);
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
    strcpy(db_path, "/tmp/lightnvr-ws-test-XXXXXX");
    int fd = mkstemp(db_path);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, fd);
    close(fd);
    TEST_ASSERT_EQUAL_INT(0, init_database(db_path));
    for (int i = 0; i < 4; i++) {
        stream_config_t camera = {0};
        snprintf(camera.name, sizeof(camera.name), "Camera %d", i);
        strcpy(camera.url, "rtsp://test/live");
        camera.enabled = true;
        TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&camera));
    }
    go2rtc_api_apply_base_path_override(NULL, 0);
    if (yaml_validate_is_available()) {
        const char config[] = "api: {base_path: /private/media}";
        TEST_ASSERT_TRUE(go2rtc_api_apply_base_path_override(config, strlen(config)));
    }
    upstream = socket(AF_INET, SOCK_STREAM, 0);
    struct sockaddr_in address = {.sin_family = AF_INET, .sin_addr.s_addr = htonl(INADDR_LOOPBACK)};
    TEST_ASSERT_EQUAL_INT(0, bind(upstream, (struct sockaddr *)&address, sizeof(address)));
    TEST_ASSERT_EQUAL_INT(0, listen(upstream, 8));
    socklen_t size = sizeof(address);
    getsockname(upstream, (struct sockaddr *)&address, &size);
    g_config.go2rtc_api_port = ntohs(address.sin_port);
    curl_global_init(CURL_GLOBAL_ALL);
    http_server_config_t config = {0};
    config.bind_ip = "127.0.0.1";
    config.web_root = "/tmp";
    config.max_connections = 16;
    server = libuv_server_init(&config);
    TEST_ASSERT_NOT_NULL(server);
    TEST_ASSERT_EQUAL_INT(0, http_server_start(server));
    int size_int = sizeof(address);
    uv_tcp_getsockname(&((libuv_server_t *)server)->listener, (struct sockaddr *)&address, &size_int);
    web_port = ntohs(address.sin_port);
}
void tearDown(void) {
    atomic_store(&teardown_started, true);
    for (int i = 0; i < 5; i++) {
        if (tracked_clients[i] >= 0) {
            close(tracked_clients[i]);
            tracked_clients[i] = -1;
        }
    }
    if (server) { http_server_stop(server); http_server_destroy(server); server = NULL; }
    for (int i = 0; i < 4; i++) {
        if (tracked_thread_started[i]) {
            pthread_join(tracked_threads[i], NULL);
            tracked_thread_started[i] = false;
        }
    }
    if (upstream >= 0) {
        close(upstream);
        upstream = -1;
    }
    shutdown_database();
    unlink(db_path);
    go2rtc_api_apply_base_path_override(NULL, 0);
    curl_global_cleanup();
}
static void send_upgrade(int fd, int camera, const char *origin, bool fragmented) {
    char request[1024];
    snprintf(request, sizeof(request), "GET /go2rtc/api/ws?src=Camera%%20%d_sub HTTP/1.1\r\n"
        "Host: localhost:%d\r\nOrigin: %s\r\n%s\r\n", camera, web_port, origin, upgrade_headers);
    if (fragmented) {
        for (size_t i = 0; i < strlen(request); i += 7) {
            size_t length = strlen(request) - i;
            if (length > 7) length = 7;
            TEST_ASSERT_TRUE(send_all(fd, request + i, length));
            struct timespec delay = {.tv_nsec = 1000000L};
            nanosleep(&delay, NULL);
        }
    } else TEST_ASSERT_TRUE(send_all(fd, request, strlen(request)));
}

void test_four_websockets_relay_media_on_the_web_port_and_close_cleanly(void) {
    char origin[64];
    snprintf(origin, sizeof(origin), "http://localhost:%d", web_port);
    for (int i = 0; i < 4; i++) {
        tracked_peers[i].status = 101;
        TEST_ASSERT_EQUAL_INT(0, pthread_create(&tracked_threads[i], NULL, upstream_peer, &tracked_peers[i]));
        tracked_thread_started[i] = true;
        tracked_clients[i] = client_open();
        if (i == 0) {
            send_upgrade(tracked_clients[i], i, origin, true);
        } else {
            char req[1024];
            int n = snprintf(req, sizeof(req),
                             "GET /go2rtc/api/ws?src=Camera%%20%d_sub HTTP/1.1\r\n"
                             "Host: localhost:%d\r\n"
                             "Origin: %s\r\n"
                             "%s"
                             "Cookie: dummy_cookie=1\r\n"
                             "Authorization: Basic ZHVtbXk6ZHVtbXk=\r\n"
                             "\r\n",
                             i, web_port, origin, upgrade_headers);
            TEST_ASSERT_TRUE(n > 0 && (size_t)n < sizeof(req));
            TEST_ASSERT_TRUE(send_all(tracked_clients[i], req, (size_t)n));
        }
        char headers[1024];
        TEST_ASSERT_TRUE(read_headers(tracked_clients[i], headers, sizeof(headers)));
        TEST_ASSERT_EQUAL_STRING(upgrade_response, headers);
        unsigned char media[sizeof(media_response)];
        TEST_ASSERT_TRUE(read_all(tracked_clients[i], media, sizeof(media)));
        TEST_ASSERT_EQUAL_MEMORY(media_response, media, sizeof(media));
        TEST_ASSERT_TRUE(send_all(tracked_clients[i], mse_request, sizeof(mse_request)));
        TEST_ASSERT_TRUE(read_all(tracked_clients[i], media, sizeof(media)));
        TEST_ASSERT_EQUAL_MEMORY(media_response, media, sizeof(media));
    }
    // Two peer disconnects plus shutdown with two streams still active.
    atomic_store(&teardown_started, true);
    close(tracked_clients[0]); tracked_clients[0] = -1;
    close(tracked_clients[1]); tracked_clients[1] = -1;
    http_server_stop(server);
    http_server_destroy(server);
    server = NULL;
    // Verify shutdown disconnects still-active websocket clients.
    {
        unsigned char b;
        ssize_t n2 = recv(tracked_clients[2], &b, 1, 0);
        int err2 = (n2 < 0) ? errno : 0;
        ssize_t n3 = recv(tracked_clients[3], &b, 1, 0);
        int err3 = (n3 < 0) ? errno : 0;
        TEST_ASSERT_TRUE(n2 == 0 || (n2 < 0 && (err2 == ECONNRESET || err2 == ENOTCONN || err2 == EPIPE)));
        TEST_ASSERT_TRUE(n3 == 0 || (n3 < 0 && (err3 == ECONNRESET || err3 == ENOTCONN || err3 == EPIPE)));
    }
    close(tracked_clients[2]); tracked_clients[2] = -1;
    close(tracked_clients[3]); tracked_clients[3] = -1;
    for (int i = 0; i < 4; i++) {
        pthread_join(tracked_threads[i], NULL);
        tracked_thread_started[i] = false;
        TEST_ASSERT_TRUE(tracked_peers[i].received_mse);
        TEST_ASSERT_TRUE(tracked_peers[i].observed_disconnect);
        char target[128];
        snprintf(target, sizeof(target), "GET %s/api/ws?src=Camera%%20%d_sub HTTP/1.1", go2rtc_api_get_base_path(), i);
        TEST_ASSERT_NOT_NULL(strstr(tracked_peers[i].request, target));
        TEST_ASSERT_NULL(strstr(tracked_peers[i].request, "Origin:"));
        TEST_ASSERT_NULL(strstr(tracked_peers[i].request, "Cookie:"));
        TEST_ASSERT_NULL(strstr(tracked_peers[i].request, "Authorization:"));
    }
}

void test_websocket_auth_origin_and_query_failures_never_reach_upstream(void) {
    for (int i = 0; i < 5; i++) {
        int client = client_open();
        tracked_clients[i] = client;
        char origin[64];
        snprintf(origin, sizeof(origin), "http://localhost:%d", web_port);
        if (i == 0) {
            g_config.web_auth_enabled = true;
            send_upgrade(client, 0, origin, false);
        } else if (i == 1) {
            g_config.web_auth_enabled = false;
            send_upgrade(client, 0, "https://attacker.invalid", false);
        } else if (i == 2) {
            g_config.web_auth_enabled = false;
            char request[1024];
            snprintf(request, sizeof(request), "GET /go2rtc/api/ws?src=Camera%%200&src=other HTTP/1.1\r\n"
                "Host: localhost:%d\r\nOrigin: %s\r\n%s\r\n", web_port, origin, upgrade_headers);
            TEST_ASSERT_TRUE(send_all(client, request, strlen(request)));
        } else if (i == 3) {
            char request[1024];
            snprintf(request, sizeof(request), "GET /go2rtc/api/ws?src=Camera%%200 HTTP/1.1\r\n"
                "Host: localhost:%d\r\nOrigin: %s\r\nOrigin: %s\r\n%s\r\n",
                web_port, origin, origin, upgrade_headers);
            TEST_ASSERT_TRUE(send_all(client, request, strlen(request)));
        } else {
            char request[1024];
            snprintf(request, sizeof(request), "GET /go2rtc/api/ws?src=Camera%%200 HTTP/1.1\r\n"
                "Host: localhost:%d\r\nConnection: close\r\n\r\n",
                web_port);
            TEST_ASSERT_TRUE(send_all(client, request, strlen(request)));
        }
        char response[2048];
        TEST_ASSERT_TRUE(read_headers(client, response, sizeof(response)));
        int code;
        TEST_ASSERT_EQUAL_INT(1, sscanf(response, "HTTP/1.1 %d", &code));
        TEST_ASSERT_EQUAL_INT(i == 0 ? 401 : (i == 1 || i == 3) ? 403 : 400, code);
        close(client);
        tracked_clients[i] = -1;
    }
    struct pollfd ready = {.fd = upstream, .events = POLLIN};
    TEST_ASSERT_EQUAL_INT(0, poll(&ready, 1, 0));
}

void test_failed_upstream_handshake_returns_502(void) {
    /* Use tracked peer/thread/client resources so tearDown cleanup runs even if assertions fail. */
    tracked_peers[0] = (peer_t){.status = 404};
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&tracked_threads[0], NULL, upstream_peer, &tracked_peers[0]));
    tracked_thread_started[0] = true;
    int client = client_open();
    tracked_clients[0] = client;
    char origin[64], response[2048];
    int status = 0;
    snprintf(origin, sizeof(origin), "http://localhost:%d", web_port);
    send_upgrade(client, 0, origin, false);
    /* Send a masked client WebSocket frame so forwarding-after-reject regressions are detectable. */
    const unsigned char frame[] = {0x81, 0x81, 0x12, 0x34, 0x56, 0x78, (unsigned char)('x' ^ 0x12)};
    (void)send_all(client, frame, sizeof(frame));
    TEST_ASSERT_TRUE(read_headers(client, response, sizeof(response)));
    TEST_ASSERT_EQUAL_INT(1, sscanf(response, "HTTP/%*d.%*d %d", &status));
    TEST_ASSERT_EQUAL_INT(502, status);
    close(client);
    tracked_clients[0] = -1;
    pthread_join(tracked_threads[0], NULL);
    tracked_thread_started[0] = false;
    TEST_ASSERT_FALSE(tracked_peers[0].observed_client_ws_data_after_reject);
}
void test_upgrade_without_media_policy_ack_is_rejected(void) {
    tracked_peers[0] = (peer_t){.status = 101, .omit_media_ack = true};
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&tracked_threads[0], NULL, upstream_peer, &tracked_peers[0]));
    tracked_thread_started[0] = true;
    int client = client_open();
    tracked_clients[0] = client;
    char origin[64], response[2048];
    int status = 0;
    static const unsigned char masked_frame[] = {
        0x81, 0x82, 0x01, 0x02, 0x03, 0x04, (unsigned char)('h' ^ 0x01), (unsigned char)('i' ^ 0x02)
    };
    unsigned char request_with_frame[4096];
    int header_len;
    snprintf(origin, sizeof(origin), "http://localhost:%d", web_port);
    header_len = snprintf((char *)request_with_frame, sizeof(request_with_frame),
                          "GET /go2rtc/api/ws?src=Camera%%200_sub HTTP/1.1\r\n"
                          "Host: localhost:%d\r\n"
                          "Origin: %s\r\n"
                          "%s"
                          "Sec-WebSocket-Extensions: permessage-deflate\r\n"
                          "\r\n",
                          web_port, origin, upgrade_headers);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, header_len);
    TEST_ASSERT_LESS_THAN_INT((int)sizeof(request_with_frame) - (int)sizeof(masked_frame), header_len);
    memcpy(request_with_frame + header_len, masked_frame, sizeof(masked_frame));
    TEST_ASSERT_TRUE(send_all(client, request_with_frame, (size_t)header_len + sizeof(masked_frame)));
    TEST_ASSERT_TRUE(read_headers(client, response, sizeof(response)));
    TEST_ASSERT_EQUAL_INT(1, sscanf(response, "HTTP/%*d.%*d %d", &status));
    TEST_ASSERT_EQUAL_INT(502, status);
    const char *length_header = strstr(response, "Content-Length: ");
    TEST_ASSERT_NOT_NULL(length_header);
    long body_length = strtol(length_header + strlen("Content-Length: "), NULL, 10);
    TEST_ASSERT_GREATER_THAN_INT(0, body_length);
    TEST_ASSERT_LESS_THAN_INT((int)sizeof(response), body_length);
    TEST_ASSERT_TRUE(read_all(client, response, (size_t)body_length));
    response[body_length] = '\0';
    TEST_ASSERT_NOT_NULL(strstr(response, "compatible OpenSensor go2rtc fork"));
    close(client);
    tracked_clients[0] = -1;
    pthread_join(tracked_threads[0], NULL);
    tracked_thread_started[0] = false;
    TEST_ASSERT_FALSE(tracked_peers[0].observed_client_ws_data_after_reject);
}
void test_upgrade_with_wrong_accept_is_rejected(void) {
    tracked_peers[0] = (peer_t){.status = 101, .bad_accept = true};
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&tracked_threads[0], NULL, upstream_peer, &tracked_peers[0]));
    tracked_thread_started[0] = true;
    int client = client_open();
    tracked_clients[0] = client;
    char origin[64], response[2048], request[1024];
    unsigned char payload[1200];
    int status = 0;
    static const unsigned char masked_frame[] = {
        0x81, 0x82, 0x01, 0x02, 0x03, 0x04, (unsigned char)('h' ^ 0x01), (unsigned char)('i' ^ 0x02)
    };
    snprintf(origin, sizeof(origin), "http://localhost:%d", web_port);
    int req_len = snprintf(
        request, sizeof(request),
        "GET /go2rtc/api/ws?src=Camera%%200_sub HTTP/1.1\r\n"
        "Host: localhost:%d\r\n"
        "Origin: %s\r\n"
        "%s"
        "\r\n",
        web_port, origin, upgrade_headers
    );
    TEST_ASSERT_TRUE(req_len > 0);
    TEST_ASSERT_TRUE((size_t)req_len < sizeof(request));
    TEST_ASSERT_TRUE((size_t)req_len + sizeof(masked_frame) <= sizeof(payload));
    memcpy(payload, request, (size_t)req_len);
    memcpy(payload + req_len, masked_frame, sizeof(masked_frame));
    (void)send_all(client, payload, (size_t)req_len + sizeof(masked_frame));
    TEST_ASSERT_TRUE(read_headers(client, response, sizeof(response)));
    TEST_ASSERT_EQUAL_INT(1, sscanf(response, "HTTP/%*d.%*d %d", &status));
    TEST_ASSERT_EQUAL_INT(502, status);
    close(client);
    tracked_clients[0] = -1;
    pthread_join(tracked_threads[0], NULL);
    tracked_thread_started[0] = false;
    TEST_ASSERT_TRUE(tracked_peers[0].observed_upgrade_request);
    TEST_ASSERT_TRUE(tracked_peers[0].sent_configured_response);
    TEST_ASSERT_FALSE(tracked_peers[0].observed_client_ws_data_after_reject);
}
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_websocket_origin_authority_validation);
    RUN_TEST(test_four_websockets_relay_media_on_the_web_port_and_close_cleanly);
    RUN_TEST(test_websocket_auth_origin_and_query_failures_never_reach_upstream);
    RUN_TEST(test_failed_upstream_handshake_returns_502);
    RUN_TEST(test_upgrade_without_media_policy_ack_is_rejected);
    RUN_TEST(test_upgrade_with_wrong_accept_is_rejected);
    return UNITY_END();
}
