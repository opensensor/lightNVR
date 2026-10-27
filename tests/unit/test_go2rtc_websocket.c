#define _POSIX_C_SOURCE 200809L
#include <arpa/inet.h>
#include <errno.h>
#include <pthread.h>
#include <poll.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/time.h>
#include <unistd.h>
#include <curl/curl.h>

#include "unity.h"
#include "core/config.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#include "video/go2rtc/go2rtc_api.h"
#include "utils/yaml_validate.h"
#include "web/api_handlers_go2rtc_proxy.h"
#include "web/libuv_server.h"

static http_server_handle_t server;
static int upstream, web_port;
static char db_path[80];
static const char upgrade_headers[] =
    "Upgrade: websocket\r\nConnection: keep-alive, Upgrade\r\n"
    "Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\nSec-WebSocket-Version: 13\r\n";
static const char upgrade_response[] =
    "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\n"
    "Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=\r\n\r\n";
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
} peer_t;

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
    TEST_ASSERT_EQUAL_INT(0, connect(fd, (struct sockaddr *)&address, sizeof(address)));
    return fd;
}
static void *upstream_peer(void *arg) {
    peer_t *peer = arg;
    struct pollfd ready = {.fd = upstream, .events = POLLIN};
    if (poll(&ready, 1, 3000) <= 0) return NULL;
    peer->fd = accept(upstream, NULL, NULL);
    if (peer->fd < 0) return NULL;
    socket_timeout(peer->fd);
    if (!read_headers(peer->fd, peer->request, sizeof(peer->request))) goto done;
    if (peer->status != 101) {
        const char error[] = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";
        send_all(peer->fd, error, sizeof(error) - 1);
        goto done;
    }
    // Split the upstream HTTP response and coalesce its end with a frame.
    send_all(peer->fd, upgrade_response, 17);
    char combined[512];
    size_t remaining = strlen(upgrade_response) - 17;
    memcpy(combined, upgrade_response + 17, remaining);
    memcpy(combined + remaining, media_response, sizeof(media_response));
    send_all(peer->fd, combined, remaining + sizeof(media_response));
    unsigned char frame[sizeof(mse_request)];
    if (read_all(peer->fd, frame, sizeof(frame))) {
        peer->received_mse = memcmp(frame, mse_request, sizeof(frame)) == 0;
        send_all(peer->fd, media_response, sizeof(media_response));
    }
    // Keep the socket alive until its consumer or the server disconnects.
    char byte;
    recv(peer->fd, &byte, 1, 0);
done:
    close(peer->fd);
    return NULL;
}
void setUp(void) {
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
    if (server) { http_server_stop(server); http_server_destroy(server); server = NULL; }
    close(upstream);
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
    int clients[4];
    pthread_t threads[4];
    peer_t peers[4] = {0};
    char origin[64];
    snprintf(origin, sizeof(origin), "http://localhost:%d", web_port);
    for (int i = 0; i < 4; i++) {
        peers[i].status = 101;
        TEST_ASSERT_EQUAL_INT(0, pthread_create(&threads[i], NULL, upstream_peer, &peers[i]));
        clients[i] = client_open();
        send_upgrade(clients[i], i, origin, i == 0);
        char headers[1024];
        TEST_ASSERT_TRUE(read_headers(clients[i], headers, sizeof(headers)));
        TEST_ASSERT_EQUAL_STRING(upgrade_response, headers);
        unsigned char media[sizeof(media_response)];
        TEST_ASSERT_TRUE(read_all(clients[i], media, sizeof(media)));
        TEST_ASSERT_EQUAL_MEMORY(media_response, media, sizeof(media));
        TEST_ASSERT_TRUE(send_all(clients[i], mse_request, sizeof(mse_request)));
        TEST_ASSERT_TRUE(read_all(clients[i], media, sizeof(media)));
        TEST_ASSERT_EQUAL_MEMORY(media_response, media, sizeof(media));
    }
    // Two peer disconnects plus shutdown with two streams still active.
    close(clients[0]);
    close(clients[1]);
    http_server_stop(server);
    http_server_destroy(server);
    server = NULL;
    close(clients[2]);
    close(clients[3]);
    for (int i = 0; i < 4; i++) {
        pthread_join(threads[i], NULL);
        TEST_ASSERT_TRUE(peers[i].received_mse);
        char target[128];
        snprintf(target, sizeof(target), "GET %s/api/ws?src=Camera%%20%d_sub HTTP/1.1", go2rtc_api_get_base_path(), i);
        TEST_ASSERT_NOT_NULL(strstr(peers[i].request, target));
        TEST_ASSERT_NULL(strstr(peers[i].request, "Origin:"));
        TEST_ASSERT_NULL(strstr(peers[i].request, "Cookie:"));
        TEST_ASSERT_NULL(strstr(peers[i].request, "Authorization:"));
    }
}

void test_websocket_auth_origin_and_query_failures_never_reach_upstream(void) {
    for (int i = 0; i < 4; i++) {
        int client = client_open();
        char origin[64];
        snprintf(origin, sizeof(origin), "http://localhost:%d", web_port);
        if (i == 0) {
            g_config.web_auth_enabled = true;
            send_upgrade(client, 0, origin, false);
        } else if (i == 1) {
            g_config.web_auth_enabled = false;
            send_upgrade(client, 0, "https://attacker.invalid", false);
        } else {
            char request[1024];
            snprintf(request, sizeof(request), "GET /go2rtc/api/ws?src=Camera%%200%s HTTP/1.1\r\n"
                "Host: localhost:%d\r\nConnection: close\r\n\r\n",
                i == 2 ? "&src=other" : "", web_port);
            TEST_ASSERT_TRUE(send_all(client, request, strlen(request)));
        }
        char response[2048];
        TEST_ASSERT_TRUE(read_headers(client, response, sizeof(response)));
        int code;
        TEST_ASSERT_EQUAL_INT(1, sscanf(response, "HTTP/1.1 %d", &code));
        TEST_ASSERT_EQUAL_INT(i == 0 ? 401 : i == 1 ? 403 : 400, code);
        close(client);
    }
    struct pollfd ready = {.fd = upstream, .events = POLLIN};
    TEST_ASSERT_EQUAL_INT(0, poll(&ready, 1, 0));
}

void test_failed_upstream_handshake_returns_502(void) {
    peer_t peer = {.status = 404};
    pthread_t thread;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&thread, NULL, upstream_peer, &peer));
    int client = client_open();
    char origin[64], response[2048];
    snprintf(origin, sizeof(origin), "http://localhost:%d", web_port);
    send_upgrade(client, 0, origin, false);
    TEST_ASSERT_TRUE(read_headers(client, response, sizeof(response)));
    TEST_ASSERT_NOT_NULL(strstr(response, "502"));
    close(client);
    pthread_join(thread, NULL);
}
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_four_websockets_relay_media_on_the_web_port_and_close_cleanly);
    RUN_TEST(test_websocket_auth_origin_and_query_failures_never_reach_upstream);
    RUN_TEST(test_failed_upstream_handshake_returns_502);
    return UNITY_END();
}
