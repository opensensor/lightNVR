#define _POSIX_C_SOURCE 200809L

#include <arpa/inet.h>
#include <pthread.h>
#include <poll.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <time.h>
#include <unistd.h>
#include <curl/curl.h>

#include "unity.h"
#include "video/go2rtc/go2rtc_api.h"
#include "utils/yaml_validate.h"
#include "core/config.h"
#include "web/go2rtc_proxy_thread.h"
#include "web/api_handlers_go2rtc_proxy.h"
#include "web/libuv_server.h"
#include "web/request_response.h"

static uv_loop_t loop;
static int completed_responses;
static bool hold_upstream;
static bool release_upstream;
static bool real_authorization;
static int upstream_requests;
static char upstream_request[2048];
static pthread_mutex_t upstream_mutex = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t upstream_ready = PTHREAD_COND_INITIALIZER;

bool __real_go2rtc_proxy_authorize_request(const http_request_t *request,
                                          http_response_t *response);
/* Slot/timeout tests bypass policy; security tests exercise real authorization. */
bool __wrap_go2rtc_proxy_authorize_request(const http_request_t *request,
                                          http_response_t *response) {
    if (real_authorization) {
        return __real_go2rtc_proxy_authorize_request(request, response);
    }
    return true;
}

int __wrap_libuv_send_response_ex(libuv_connection_t *connection,
                                  const http_response_t *response,
                                  write_complete_action_t action) {
    (void)connection;
    (void)response;
    (void)action;
    completed_responses++;
    return 0;
}

CURLcode __real_curl_easy_perform(CURL *curl);
CURLcode __wrap_curl_easy_perform(CURL *curl) {
    if (!hold_upstream) return __real_curl_easy_perform(curl);
    pthread_mutex_lock(&upstream_mutex);
    while (!release_upstream) pthread_cond_wait(&upstream_ready, &upstream_mutex);
    pthread_mutex_unlock(&upstream_mutex);
    return CURLE_COULDNT_CONNECT;
}

static long monotonic_ms(void) {
    struct timespec now;
    clock_gettime(CLOCK_MONOTONIC, &now);
    return now.tv_sec * 1000L + now.tv_nsec / 1000000L;
}

static void pump_responses(int expected, long timeout_ms) {
    long started = monotonic_ms();
    while (completed_responses < expected && monotonic_ms() - started < timeout_ms) {
        uv_run(&loop, UV_RUN_NOWAIT);
        struct timespec pause = { .tv_nsec = 10000000L };
        nanosleep(&pause, NULL);
    }
}

static void init_connection(libuv_connection_t *connection) {
    memset(connection, 0, sizeof(*connection));
    http_request_init(&connection->request);
    http_response_init(&connection->response);
    strcpy(connection->request.path, "/go2rtc/api/frame.jpeg");
    strcpy(connection->request.method_str, "GET");
    strcpy(connection->request.query_string, "src=cam_a");
}

void setUp(void) {
    load_default_config(&g_config);
    go2rtc_api_apply_base_path_override(NULL, 0);
    completed_responses = 0;
    hold_upstream = false;
    release_upstream = false;
    real_authorization = false;
    upstream_requests = 0;
    TEST_ASSERT_EQUAL_INT(CURLE_OK, curl_global_init(CURL_GLOBAL_ALL));
    TEST_ASSERT_EQUAL_INT(0, uv_loop_init(&loop));
    TEST_ASSERT_EQUAL_INT(0, go2rtc_proxy_thread_init(&loop));
}

void tearDown(void) {
    go2rtc_api_apply_base_path_override(NULL, 0);
    go2rtc_proxy_thread_shutdown();
    uv_run(&loop, UV_RUN_DEFAULT);
    TEST_ASSERT_EQUAL_INT(0, uv_loop_close(&loop));
    curl_global_cleanup();
}

void test_proxy_rejects_overflow_and_recovers_slots_after_completion(void) {
    enum { LIMIT = 32 };
    libuv_connection_t *connections = calloc(LIMIT + 1, sizeof(*connections));
    TEST_ASSERT_NOT_NULL(connections);
    hold_upstream = true;
    for (int i = 0; i < LIMIT + 1; i++) init_connection(&connections[i]);
    int accepted = 0;
    for (int i = 0; i < LIMIT; i++) {
        if (go2rtc_proxy_thread_submit(&connections[i], WRITE_ACTION_CLOSE) == 0) accepted++;
    }
    int overflow_result = go2rtc_proxy_thread_submit(&connections[LIMIT], WRITE_ACTION_CLOSE);
    pthread_mutex_lock(&upstream_mutex);
    release_upstream = true;
    pthread_cond_broadcast(&upstream_ready);
    pthread_mutex_unlock(&upstream_mutex);
    pump_responses(accepted, 3000);

    TEST_ASSERT_EQUAL_INT(LIMIT, accepted);
    TEST_ASSERT_EQUAL_INT(-1, overflow_result);
    TEST_ASSERT_EQUAL_INT(LIMIT, completed_responses);
    TEST_ASSERT_EQUAL_INT(0, go2rtc_proxy_thread_submit(&connections[LIMIT], WRITE_ACTION_CLOSE));
    pump_responses(LIMIT + 1, 3000);
    TEST_ASSERT_EQUAL_INT(LIMIT + 1, completed_responses);
    for (int i = 0; i < LIMIT + 1; i++) http_response_free(&connections[i].response);
    free(connections);
}

static void *slow_snapshot_peer(void *arg) {
    int client = accept(*(int *)arg, NULL, NULL);
    if (client < 0) return NULL;
    char request[2048];
    (void)read(client, request, sizeof(request));
    struct timespec delay = { .tv_sec = 11 };
    nanosleep(&delay, NULL);
    const char response[] = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
    (void)send(client, response, sizeof(response) - 1, MSG_NOSIGNAL);
    close(client);
    return NULL;
}

void test_slow_snapshot_releases_its_slot_after_ten_seconds(void) {
    int server = socket(AF_INET, SOCK_STREAM, 0);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, server);
    struct sockaddr_in address = {
        .sin_family = AF_INET,
        .sin_addr.s_addr = htonl(INADDR_LOOPBACK),
    };
    TEST_ASSERT_EQUAL_INT(0, bind(server, (struct sockaddr *)&address, sizeof(address)));
    TEST_ASSERT_EQUAL_INT(0, listen(server, 1));
    socklen_t size = sizeof(address);
    TEST_ASSERT_EQUAL_INT(0, getsockname(server, (struct sockaddr *)&address, &size));
    g_config.go2rtc_api_port = ntohs(address.sin_port);
    pthread_t peer;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&peer, NULL, slow_snapshot_peer, &server));
    libuv_connection_t connection;
    init_connection(&connection);
    long started = monotonic_ms();
    TEST_ASSERT_EQUAL_INT(0, go2rtc_proxy_thread_submit(&connection, WRITE_ACTION_CLOSE));
    pump_responses(1, 12000);
    long elapsed = monotonic_ms() - started;
    int status = connection.response.status_code;
    pthread_join(peer, NULL);
    close(server);
    http_response_free(&connection.response);

    TEST_ASSERT_EQUAL_INT(1, completed_responses);
    TEST_ASSERT_EQUAL_INT(502, status);
    TEST_ASSERT_GREATER_OR_EQUAL_INT64(9000, elapsed);
    TEST_ASSERT_LESS_THAN_INT64(11000, elapsed);
}

static int open_upstream_listener(void) {
    int server = socket(AF_INET, SOCK_STREAM, 0);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, server);
    struct sockaddr_in address = {
        .sin_family = AF_INET,
        .sin_addr.s_addr = htonl(INADDR_LOOPBACK),
    };
    TEST_ASSERT_EQUAL_INT(0, bind(server, (struct sockaddr *)&address, sizeof(address)));
    TEST_ASSERT_EQUAL_INT(0, listen(server, 4));
    socklen_t size = sizeof(address);
    TEST_ASSERT_EQUAL_INT(0, getsockname(server, (struct sockaddr *)&address, &size));
    g_config.go2rtc_api_port = ntohs(address.sin_port);
    return server;
}

static void *redirect_peer(void *arg) {
    int server = *(int *)arg;
    struct pollfd ready = { .fd = server, .events = POLLIN };
    while (poll(&ready, 1, 1000) > 0) {
        int client = accept(server, NULL, NULL);
        if (client < 0) break;
        char request[2048];
        ssize_t received = read(client, request, sizeof(request));
        if (received > 0) {
            size_t n = (size_t)received < sizeof(upstream_request) - 1
                ? (size_t)received : sizeof(upstream_request) - 1;
            memcpy(upstream_request, request, n);
            upstream_request[n] = '\0';
            upstream_requests++;
            const char *response = upstream_requests == 1
                ? "HTTP/1.1 302 Found\r\nLocation: /go2rtc/api/config\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                : "HTTP/1.1 200 OK\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
            (void)send(client, response, strlen(response), MSG_NOSIGNAL);
        }
        close(client);
    }
    return NULL;
}

void test_both_proxy_handlers_never_follow_unauthorized_redirects(void) {
    if (yaml_validate_is_available()) {
        const char *override = "api: {base_path: /foo}";
        TEST_ASSERT_TRUE(go2rtc_api_apply_base_path_override(override, strlen(override)));
    }
    g_config.web_auth_enabled = false;
    real_authorization = true;
    for (int threaded = 0; threaded < 2; threaded++) {
        upstream_requests = 0;
        int server = open_upstream_listener();
        pthread_t peer;
        TEST_ASSERT_EQUAL_INT(0, pthread_create(&peer, NULL, redirect_peer, &server));
        libuv_connection_t connection;
        init_connection(&connection);
        strcpy(connection.request.path, "/go2rtc/api/streams");
        connection.request.query_string[0] = '\0';
        connection.request.user_data = &connection;
        if (threaded) {
            TEST_ASSERT_EQUAL_INT(0, go2rtc_proxy_thread_submit(&connection, WRITE_ACTION_CLOSE));
            pump_responses(1, 3000);
            TEST_ASSERT_EQUAL_INT(1, completed_responses);
        } else {
            handle_go2rtc_proxy(&connection.request, &connection.response);
        }
        pthread_join(peer, NULL);
        close(server);
        int status = connection.response.status_code;
        http_response_free(&connection.response);
        TEST_ASSERT_EQUAL_INT(1, upstream_requests);
        TEST_ASSERT_EQUAL_INT(302, status);
        TEST_ASSERT_NOT_NULL(strstr(upstream_request, yaml_validate_is_available()
            ? "GET /foo/api/streams HTTP/1.1" : "GET /go2rtc/api/streams HTTP/1.1"));
    }
}

void test_proxy_rejects_unsafe_requests_before_contacting_upstream(void) {
    g_config.web_auth_enabled = false;
    real_authorization = true;
    int server = open_upstream_listener();
    const char *paths[] = {
        "/go2rtc/api/hls/../streams", "/go2rtc/api/hls/../config",
        "/go2rtc/api/hls/%2e%2e/streams", "/go2rtc/api/streams",
        "/go2rtc/api/hls/segment.ts"
    };
    const char *methods[] = {"GET", "POST", "PUT", "DELETE"};
    for (int threaded = 0; threaded < 2; threaded++) {
        for (size_t i = 0; i < sizeof(paths) / sizeof(paths[0]); i++) {
            for (size_t j = 0; j < sizeof(methods) / sizeof(methods[0]); j++) {
                // Canonical GETs need valid credentials/session or reach upstream.
                if (i >= 3 && j == 0) continue;
                libuv_connection_t connection;
                init_connection(&connection);
                strcpy(connection.request.path, paths[i]);
                strcpy(connection.request.method_str, methods[j]);
                connection.request.user_data = &connection;
                if (threaded) {
                    int expected = completed_responses + 1;
                    TEST_ASSERT_EQUAL_INT(0, go2rtc_proxy_thread_submit(&connection, WRITE_ACTION_CLOSE));
                    pump_responses(expected, 1000);
                    TEST_ASSERT_EQUAL_INT(expected, completed_responses);
                } else {
                    handle_go2rtc_proxy(&connection.request, &connection.response);
                }
                int status = connection.response.status_code;
                http_response_free(&connection.response);
                TEST_ASSERT_EQUAL_INT(i < 3 ? 404 : 405, status);
            }
        }
    }
    struct pollfd ready = { .fd = server, .events = POLLIN };
    int contacted = poll(&ready, 1, 0);
    close(server);
    TEST_ASSERT_EQUAL_INT(0, contacted);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_proxy_rejects_overflow_and_recovers_slots_after_completion);
    RUN_TEST(test_slow_snapshot_releases_its_slot_after_ten_seconds);
    RUN_TEST(test_both_proxy_handlers_never_follow_unauthorized_redirects);
    RUN_TEST(test_proxy_rejects_unsafe_requests_before_contacting_upstream);
    return UNITY_END();
}
