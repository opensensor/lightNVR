#define _POSIX_C_SOURCE 200809L

#include <arpa/inet.h>
#include <errno.h>
#include <poll.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/time.h>
#include <unistd.h>
#include <curl/curl.h>

#include "unity.h"
#include "core/config.h"
#include "web/api_handlers_go2rtc_proxy.h"
#include "web/libuv_server.h"

/* Exercise llhttp, route selection, the real authorizer and static-file
 * serving over raw HTTP. No proxy or response functions are wrapped. */
static http_server_handle_t server;
static int upstream_listener = -1;
static int web_port;
static char temporary_dir[64];
static char web_root[96];
static const char outside_marker[] = "PRIVATE-FILE-OUTSIDE-THE-WEB-ROOT";
static const char *fixture_directories[] = {
    "web", "web/go2rtc", "web/go2rtc/api", "web/go2rtc/api/hls",
    "web/go2rtc/api/streams"
};
static const char *fixture_files[] = {
    "private.txt", "web/index.html", "web/go2rtc-assets.txt"
};

static void fixture_path(char path[256], const char *relative) {
    snprintf(path, 256, "%s/%s", temporary_dir, relative);
}

static void write_fixture(const char *relative, const char *contents) {
    char path[256];
    fixture_path(path, relative);
    FILE *file = fopen(path, "w");
    TEST_ASSERT_NOT_NULL(file);
    int written = fputs(contents, file);
    int closed = fclose(file);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, written);
    TEST_ASSERT_EQUAL_INT(0, closed);
}

void setUp(void) {
    server = NULL;
    upstream_listener = -1;
    strcpy(temporary_dir, "/tmp/lightnvr-proxy-routing-XXXXXX");
    TEST_ASSERT_NOT_NULL(mkdtemp(temporary_dir));
    for (size_t i = 0; i < sizeof(fixture_directories) / sizeof(fixture_directories[0]); i++) {
        char path[256];
        fixture_path(path, fixture_directories[i]);
        TEST_ASSERT_EQUAL_INT(0, mkdir(path, 0700));
    }
    snprintf(web_root, sizeof(web_root), "%s/web", temporary_dir);
    write_fixture("private.txt", outside_marker);
    write_fixture("web/index.html", "public-index");
    write_fixture("web/go2rtc-assets.txt", "public-go2rtc-asset");

    load_default_config(&g_config);
    g_config.web_auth_enabled = true;
    g_config.demo_mode = false;
    g_config.web_thread_pool_size = 2;
    TEST_ASSERT_EQUAL_INT(CURLE_OK, curl_global_init(CURL_GLOBAL_ALL));

    upstream_listener = socket(AF_INET, SOCK_STREAM, 0);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, upstream_listener);
    struct sockaddr_in address = {
        .sin_family = AF_INET,
        .sin_addr.s_addr = htonl(INADDR_LOOPBACK),
    };
    TEST_ASSERT_EQUAL_INT(0, bind(upstream_listener, (struct sockaddr *)&address, sizeof(address)));
    TEST_ASSERT_EQUAL_INT(0, listen(upstream_listener, 4));
    socklen_t size = sizeof(address);
    TEST_ASSERT_EQUAL_INT(0, getsockname(upstream_listener, (struct sockaddr *)&address, &size));
    g_config.go2rtc_api_port = ntohs(address.sin_port);

    http_server_config_t config = {0};
    config.bind_ip = "127.0.0.1";
    config.web_root = web_root;
    config.max_connections = 16;
    config.connection_timeout = 5;
    server = libuv_server_init(&config);
    TEST_ASSERT_NOT_NULL(server);
    TEST_ASSERT_EQUAL_INT(0, http_server_register_handler(server,
        "/go2rtc/api/streams", "GET", handle_go2rtc_proxy));
    TEST_ASSERT_EQUAL_INT(0, http_server_register_handler(server,
        "/go2rtc/api/reload", "POST", handle_go2rtc_proxy));
    TEST_ASSERT_EQUAL_INT(0, http_server_register_handler(server,
        "/go2rtc/api/stream.m3u8", "GET", handle_go2rtc_proxy));
    TEST_ASSERT_EQUAL_INT(0, http_server_register_handler(server,
        "/go2rtc/api/hls/*", "GET", handle_go2rtc_proxy));
    TEST_ASSERT_EQUAL_INT(0, http_server_register_handler(server,
        "/go2rtc/api/frame.jpeg", "GET", handle_go2rtc_proxy));
    TEST_ASSERT_EQUAL_INT(0, http_server_start(server));
    int address_size = sizeof(address);
    TEST_ASSERT_EQUAL_INT(0, uv_tcp_getsockname(&((libuv_server_t *)server)->listener,
        (struct sockaddr *)&address, &address_size));
    web_port = ntohs(address.sin_port);
}

void tearDown(void) {
    if (server) {
        http_server_stop(server);
        http_server_destroy(server);
        server = NULL;
    }
    if (upstream_listener >= 0) close(upstream_listener);
    curl_global_cleanup();
    for (size_t i = 0; i < sizeof(fixture_files) / sizeof(fixture_files[0]); i++) {
        char path[256];
        fixture_path(path, fixture_files[i]);
        (void)unlink(path);
    }
    for (size_t i = sizeof(fixture_directories) / sizeof(fixture_directories[0]); i > 0; i--) {
        char path[256];
        fixture_path(path, fixture_directories[i - 1]);
        (void)rmdir(path);
    }
    (void)rmdir(temporary_dir);
}

static int raw_request(const char *method, const char *target, char response[4096]) {
    int client = socket(AF_INET, SOCK_STREAM, 0);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, client);
    struct timeval timeout = { .tv_sec = 3 };
    TEST_ASSERT_EQUAL_INT(0, setsockopt(client, SOL_SOCKET, SO_RCVTIMEO, &timeout, sizeof(timeout)));
    TEST_ASSERT_EQUAL_INT(0, setsockopt(client, SOL_SOCKET, SO_SNDTIMEO, &timeout, sizeof(timeout)));
    struct sockaddr_in address = {
        .sin_family = AF_INET,
        .sin_addr.s_addr = htonl(INADDR_LOOPBACK),
        .sin_port = htons(web_port),
    };
    TEST_ASSERT_EQUAL_INT(0, connect(client, (struct sockaddr *)&address, sizeof(address)));
    char request[1024];
    int length = snprintf(request, sizeof(request),
        "%s %s HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        method, target);
    TEST_ASSERT_GREATER_THAN_INT(0, length);
    TEST_ASSERT_LESS_THAN_INT((int)sizeof(request), length);
    size_t sent = 0;
    while (sent < (size_t)length) {
        ssize_t count = send(client, request + sent, (size_t)length - sent, MSG_NOSIGNAL);
        if (count < 0 && errno == EINTR) continue;
        if (count <= 0) {
            close(client);
            TEST_FAIL_MESSAGE("Failed to send raw HTTP request");
        }
        sent += (size_t)count;
    }
    size_t used = 0;
    while (used < 4095) {
        ssize_t count = recv(client, response + used, 4095 - used, 0);
        if (count < 0 && errno == EINTR) continue;
        if (count < 0) {
            close(client);
            TEST_FAIL_MESSAGE("Raw HTTP response timed out or failed");
        }
        if (count == 0) break;
        used += (size_t)count;
    }
    close(client);
    response[used] = '\0';
    int status = 0;
    TEST_ASSERT_EQUAL_INT(1, sscanf(response, "HTTP/1.1 %d", &status));
    return status;
}

static void assert_no_upstream_contact(void) {
    struct pollfd ready = { .fd = upstream_listener, .events = POLLIN };
    TEST_ASSERT_EQUAL_INT(0, poll(&ready, 1, 0));
}

void test_normal_static_files_still_work_outside_the_go2rtc_namespace(void) {
    char response[4096];
    TEST_ASSERT_EQUAL_INT(200, raw_request("GET", "/index.html", response));
    TEST_ASSERT_NOT_NULL(strstr(response, "public-index"));
    TEST_ASSERT_EQUAL_INT(200, raw_request("GET", "/go2rtc-assets.txt", response));
    TEST_ASSERT_NOT_NULL(strstr(response, "public-go2rtc-asset"));
    assert_no_upstream_contact();
}

void test_raw_go2rtc_traversal_never_falls_through_to_static_files(void) {
    // Dot segments (literal or percent-encoded) are rejected in on_url()
    // before routing, authorization, curl, or static-file resolution ever
    // see the path, so no consumer can interpret it a second, different way.
    const char *paths[] = {
        "/go2rtc/api/hls/../../../../private.txt",
        "/go2rtc/api/hls/../../../../private.txt?id=unused",
        "/go2rtc/api/streams/../../../../private.txt",
        "/go2rtc/../../private.txt",
        "/go2rtc/api/hls/%2e%2e/%2e%2e/%2e%2e/%2e%2e/private.txt"
    };
    const char *methods[] = {"GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"};
    for (size_t i = 0; i < sizeof(paths) / sizeof(paths[0]); i++) {
        for (size_t j = 0; j < sizeof(methods) / sizeof(methods[0]); j++) {
            char response[4096];
            int status = raw_request(methods[j], paths[i], response);
            TEST_ASSERT_EQUAL_INT_MESSAGE(400, status, paths[i]);
            TEST_ASSERT_NULL(strstr(response, outside_marker));
        }
    }
    assert_no_upstream_contact();
}

void test_raw_static_file_traversal_is_rejected_outside_go2rtc(void) {
    // The same dot-segment rejection closes the general static-file
    // traversal: a path that would otherwise concatenate past web_root is
    // refused before static_file_resolve_handler can resolve it.
    const char *paths[] = {
        "/../../private.txt",
        "/assets/../../../private.txt",
        "/././private.txt",
        "/..%2f..%2f..%2fprivate.txt"
    };
    const char *methods[] = {"GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"};
    for (size_t i = 0; i < sizeof(paths) / sizeof(paths[0]); i++) {
        for (size_t j = 0; j < sizeof(methods) / sizeof(methods[0]); j++) {
            char response[4096];
            int status = raw_request(methods[j], paths[i], response);
            TEST_ASSERT_EQUAL_INT_MESSAGE(400, status, paths[i]);
            TEST_ASSERT_NULL(strstr(response, outside_marker));
        }
    }
    assert_no_upstream_contact();
}

void test_raw_go2rtc_requests_enforce_methods_before_static_fallback(void) {
    const char *paths[] = {
        "/go2rtc/api/streams", "/go2rtc/api/reload",
        "/go2rtc/api/stream.m3u8", "/go2rtc/api/frame.jpeg",
        "/go2rtc/api/hls/segment.ts"
    };
    const char *methods[] = {"GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"};
    for (size_t i = 0; i < sizeof(paths) / sizeof(paths[0]); i++) {
        const char *allowed = i == 1 ? "POST" : "GET";
        for (size_t j = 0; j < sizeof(methods) / sizeof(methods[0]); j++) {
            if (strcmp(methods[j], allowed) == 0) continue;
            char response[4096], allow_header[32];
            snprintf(allow_header, sizeof(allow_header), "Allow: %s\r\n", allowed);
            TEST_ASSERT_EQUAL_INT_MESSAGE(405, raw_request(methods[j], paths[i], response), paths[i]);
            TEST_ASSERT_NOT_NULL(strstr(response, allow_header));
        }
    }
    assert_no_upstream_contact();
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_normal_static_files_still_work_outside_the_go2rtc_namespace);
    RUN_TEST(test_raw_go2rtc_traversal_never_falls_through_to_static_files);
    RUN_TEST(test_raw_static_file_traversal_is_rejected_outside_go2rtc);
    RUN_TEST(test_raw_go2rtc_requests_enforce_methods_before_static_fallback);
    return UNITY_END();
}
