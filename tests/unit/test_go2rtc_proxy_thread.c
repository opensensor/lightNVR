#define _POSIX_C_SOURCE 200809L

#include <arpa/inet.h>
#include <pthread.h>
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
#include "video/go2rtc/go2rtc_api.h"
#include "utils/yaml_validate.h"
#include "core/config.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#include "web/go2rtc_proxy_thread.h"
#include "web/api_handlers_go2rtc_proxy.h"
#include "web/libuv_server.h"
#include "web/request_response.h"

static uv_loop_t loop;
static int completed_responses;
static bool hold_upstream;
static bool release_upstream;
static bool worker_in_curl;
static bool real_authorization;
static bool fail_lease_uuid;
static bool acknowledge_media_policy;
static bool informational_media_ack;
static int upstream_requests;
static char upstream_request[2048];
static pthread_mutex_t upstream_mutex = PTHREAD_MUTEX_INITIALIZER;
static libuv_connection_t fixture_connection;
static bool fixture_connection_initialized;
static int fixture_upstream_listener = -1;
static pthread_t fixture_peer_thread;
static bool fixture_peer_started;
static pthread_cond_t upstream_ready = PTHREAD_COND_INITIALIZER;
static int slow_snapshot_server = -1;
static pthread_t slow_snapshot_thread;
static bool slow_snapshot_thread_started;
static libuv_connection_t slow_snapshot_connection;
static bool slow_snapshot_connection_initialized;
static libuv_connection_t threaded_connection;
static libuv_connection_t *snapshot_capacity_connections;
static int snapshot_capacity_count;

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

int __real_lightnvr_uuid_generate_v4(char output[37]);
int __wrap_lightnvr_uuid_generate_v4(char output[37]) {
    return fail_lease_uuid ? -1 : __real_lightnvr_uuid_generate_v4(output);
}
CURLcode __real_curl_easy_perform(CURL *curl);
CURLcode __wrap_curl_easy_perform(CURL *curl) {
    if (!hold_upstream) return __real_curl_easy_perform(curl);
    pthread_mutex_lock(&upstream_mutex);
    worker_in_curl = true;
    pthread_cond_broadcast(&upstream_ready);
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
    fixture_connection_initialized = false;
    fixture_upstream_listener = -1;
    fixture_peer_started = false;
    slow_snapshot_server = -1;
    slow_snapshot_thread_started = false;
    slow_snapshot_connection_initialized = false;
    snapshot_capacity_connections = NULL;
    snapshot_capacity_count = 0;
    load_default_config(&g_config);
    go2rtc_api_apply_base_path_override(NULL, 0);
    completed_responses = 0;
    hold_upstream = false;
    release_upstream = false;
    worker_in_curl = false;
    real_authorization = false;
    fail_lease_uuid = false;
    acknowledge_media_policy = true;
    informational_media_ack = false;
    upstream_requests = 0;
    TEST_ASSERT_EQUAL_INT(CURLE_OK, curl_global_init(CURL_GLOBAL_ALL));
    TEST_ASSERT_EQUAL_INT(0, uv_loop_init(&loop));
    TEST_ASSERT_EQUAL_INT(0, go2rtc_proxy_thread_init(&loop));
}

void tearDown(void) {
    go2rtc_api_apply_base_path_override(NULL, 0);
    pthread_mutex_lock(&upstream_mutex);
    release_upstream = true;
    pthread_cond_broadcast(&upstream_ready);
    pthread_mutex_unlock(&upstream_mutex);
    if (fixture_peer_started) {
        pthread_join(fixture_peer_thread, NULL);
        fixture_peer_started = false;
    }
    if (fixture_upstream_listener >= 0) {
        close(fixture_upstream_listener);
        fixture_upstream_listener = -1;
    }
    go2rtc_proxy_thread_shutdown();
    uv_run(&loop, UV_RUN_DEFAULT);
    if (fixture_connection_initialized) {
        http_response_free(&fixture_connection.response);
        fixture_connection_initialized = false;
    }
    http_response_free(&threaded_connection.response);
    if (slow_snapshot_thread_started) {
        pthread_join(slow_snapshot_thread, NULL);
        slow_snapshot_thread_started = false;
    }
    if (slow_snapshot_server >= 0) {
        close(slow_snapshot_server);
        slow_snapshot_server = -1;
    }
    if (slow_snapshot_connection_initialized) {
        http_response_free(&slow_snapshot_connection.response);
        slow_snapshot_connection_initialized = false;
    }
    for (int i = 0; i < snapshot_capacity_count; i++)
        http_response_free(&snapshot_capacity_connections[i].response);
    free(snapshot_capacity_connections);
    snapshot_capacity_connections = NULL;
    snapshot_capacity_count = 0;
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
    struct pollfd pfd = {
        .fd = *(int *)arg,
        .events = POLLIN
    };
    if (poll(&pfd, 1, 3000) <= 0 || !(pfd.revents & POLLIN)) return NULL;

    int client = accept(*(int *)arg, NULL, NULL);
    if (client < 0) return NULL;

    struct timeval rcv_timeout = {
        .tv_sec = 3,
        .tv_usec = 0
    };
    (void)setsockopt(client, SOL_SOCKET, SO_RCVTIMEO, &rcv_timeout, sizeof(rcv_timeout));

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
    slow_snapshot_server = socket(AF_INET, SOCK_STREAM, 0);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, slow_snapshot_server);
    struct sockaddr_in address = {
        .sin_family = AF_INET,
        .sin_addr.s_addr = htonl(INADDR_LOOPBACK),
    };
    TEST_ASSERT_EQUAL_INT(0, bind(slow_snapshot_server, (struct sockaddr *)&address, sizeof(address)));
    TEST_ASSERT_EQUAL_INT(0, listen(slow_snapshot_server, 1));
    socklen_t size = sizeof(address);
    TEST_ASSERT_EQUAL_INT(0, getsockname(slow_snapshot_server, (struct sockaddr *)&address, &size));
    g_config.go2rtc_api_port = ntohs(address.sin_port);
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&slow_snapshot_thread, NULL, slow_snapshot_peer, &slow_snapshot_server));
    slow_snapshot_thread_started = true;
    init_connection(&slow_snapshot_connection);
    slow_snapshot_connection_initialized = true;
    long started = monotonic_ms();
    TEST_ASSERT_EQUAL_INT(0, go2rtc_proxy_thread_submit(&slow_snapshot_connection, WRITE_ACTION_CLOSE));
    pump_responses(1, 12000);
    long elapsed = monotonic_ms() - started;
    int status = slow_snapshot_connection.response.status_code;
    pthread_join(slow_snapshot_thread, NULL);
    slow_snapshot_thread_started = false;
    close(slow_snapshot_server);
    slow_snapshot_server = -1;

    TEST_ASSERT_EQUAL_INT(1, completed_responses);
    TEST_ASSERT_EQUAL_INT(502, status);
    TEST_ASSERT_GREATER_OR_EQUAL_INT64(9000, elapsed);
    TEST_ASSERT_LESS_THAN_INT64(11000, elapsed);

    http_response_free(&slow_snapshot_connection.response);
    slow_snapshot_connection_initialized = false;

    /* A timed-out request must release its slot so all 32 slots can be used. */
    enum { LIMIT = 32 };
    snapshot_capacity_connections = calloc(LIMIT, sizeof(*snapshot_capacity_connections));
    TEST_ASSERT_NOT_NULL(snapshot_capacity_connections);
    snapshot_capacity_count = LIMIT;
    hold_upstream = true;
    release_upstream = false;
    int accepted = 0;
    for (int i = 0; i < LIMIT; i++) {
        init_connection(&snapshot_capacity_connections[i]);
        if (go2rtc_proxy_thread_submit(&snapshot_capacity_connections[i], WRITE_ACTION_CLOSE) == 0)
            accepted++;
    }
    pthread_mutex_lock(&upstream_mutex);
    release_upstream = true;
    pthread_cond_broadcast(&upstream_ready);
    pthread_mutex_unlock(&upstream_mutex);
    pump_responses(1 + accepted, 5000);
    hold_upstream = false;
    TEST_ASSERT_EQUAL_INT(LIMIT, accepted);
    TEST_ASSERT_EQUAL_INT(1 + LIMIT, completed_responses);
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
        size_t received_total = 0;
        struct pollfd client_ready = { .fd = client, .events = POLLIN };
        while (received_total < sizeof(request) - 1) {
            int pr = poll(&client_ready, 1, 1000);
            if (pr <= 0) break;
            ssize_t chunk = read(client, request + received_total, sizeof(request) - 1 - received_total);
            if (chunk <= 0) break;
            received_total += (size_t)chunk;
            request[received_total] = '\0';
            if (strstr(request, "\r\n\r\n") != NULL) break;
        }
        if (received_total > 0) {
            size_t n = received_total < sizeof(upstream_request) - 1
                ? received_total : sizeof(upstream_request) - 1;
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
        TEST_ASSERT_EQUAL_INT(502, status);
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
                init_connection(&fixture_connection);
                fixture_connection_initialized = true;
                strcpy(fixture_connection.request.path, paths[i]);
                strcpy(fixture_connection.request.method_str, methods[j]);
                fixture_connection.request.user_data = &fixture_connection;
                if (threaded) {
                    int expected = completed_responses + 1;
                    TEST_ASSERT_EQUAL_INT(0, go2rtc_proxy_thread_submit(&fixture_connection, WRITE_ACTION_CLOSE));
                    pump_responses(expected, 1000);
                    TEST_ASSERT_EQUAL_INT(expected, completed_responses);
                } else {
                    handle_go2rtc_proxy(&fixture_connection.request, &fixture_connection.response);
                }
                int status = fixture_connection.response.status_code;
                http_response_free(&fixture_connection.response);
                fixture_connection_initialized = false;
                TEST_ASSERT_EQUAL_INT(i < 3 ? 404 : 405, status);
            }
        }
    }
    struct pollfd ready = { .fd = server, .events = POLLIN };
    int contacted = poll(&ready, 1, 0);
    close(server);
    TEST_ASSERT_EQUAL_INT(0, contacted);
}

static const char sdp_offer[] = "v=0\r\ns=browser-offer\r\n";
static const char sdp_answer[] = "v=0\r\ns=go2rtc-answer\r\n";

static void *webrtc_peer(void *arg) {
    struct pollfd ready = { .fd = *(int *)arg, .events = POLLIN };
    if (poll(&ready, 1, 2000) <= 0 || !(ready.revents & POLLIN)) return NULL;
    int client = accept(ready.fd, NULL, NULL);
    if (client < 0) return NULL;
    struct timeval timeout = { .tv_sec = 2 };
    (void)setsockopt(client, SOL_SOCKET, SO_RCVTIMEO, &timeout, sizeof(timeout));
    size_t used = 0;
    while (used < sizeof(upstream_request) - 1) {
        ssize_t count = recv(client, upstream_request + used,
                             sizeof(upstream_request) - 1 - used, 0);
        if (count <= 0) break;
        used += (size_t)count;
        upstream_request[used] = '\0';
        char *body = strstr(upstream_request, "\r\n\r\n");
        if (body && strlen(body + 4) >= strlen(sdp_offer)) break;
    }
    upstream_requests++;
    if (informational_media_ack) {
        const char interim[] = "HTTP/1.1 100 Continue\r\n"
            "X-LightNVR-Media-Policy: 1\r\n\r\n";
        (void)send(client, interim, sizeof(interim) - 1, MSG_NOSIGNAL);
    }
    char response[512];
    int length = snprintf(response, sizeof(response),
        "HTTP/1.1 201 Created\r\n%sContent-Type: application/sdp\r\n"
        "Content-Length: %zu\r\nConnection: close\r\n\r\n%s",
        acknowledge_media_policy ? "X-LightNVR-Media-Policy: 1\r\n" : "",
        strlen(sdp_answer), sdp_answer);
    (void)send(client, response, (size_t)length, MSG_NOSIGNAL);
    close(client);
    return NULL;
}

static void check_both_webrtc_proxies(bool acknowledged, bool fail_issuance) {
    acknowledge_media_policy = acknowledged;
    char db_path[] = "/tmp/lightnvr-webrtc-proxy-XXXXXX";
    int db_file = mkstemp(db_path);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, db_file);
    close(db_file);
    TEST_ASSERT_EQUAL_INT(0, init_database(db_path));
    stream_config_t stream = {0};
    strcpy(stream.name, "Front Gate");
    strcpy(stream.url, "rtsp://camera/live");
    stream.enabled = true;
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&stream));
    g_config.web_auth_enabled = false;
    real_authorization = true;
    if (yaml_validate_is_available()) {
        const char *override = "api: {base_path: /custom}";
        TEST_ASSERT_TRUE(go2rtc_api_apply_base_path_override(override, strlen(override)));
    }
    for (int threaded = 0; threaded < 2; threaded++) {
        upstream_request[0] = '\0';
        fixture_upstream_listener = open_upstream_listener();
        fixture_peer_started = false;
        if (!fail_issuance) {
            TEST_ASSERT_EQUAL_INT(0, pthread_create(&fixture_peer_thread, NULL, webrtc_peer, &fixture_upstream_listener));
            fixture_peer_started = true;
        }
        init_connection(&threaded_connection);
        strcpy(threaded_connection.request.path, "/go2rtc/api/webrtc");
        strcpy(threaded_connection.request.method_str, "POST");
        strcpy(threaded_connection.request.query_string, "src=Front%20Gate");
        strcpy(threaded_connection.request.content_type, "application/sdp");
        threaded_connection.request.body = (char *)sdp_offer;
        threaded_connection.request.body_len = strlen(sdp_offer);
        threaded_connection.request.user_data = &threaded_connection;
        fail_lease_uuid = fail_issuance;
        if (threaded) {
            TEST_ASSERT_EQUAL_INT(0, go2rtc_proxy_thread_submit(&threaded_connection, WRITE_ACTION_CLOSE));
            pump_responses(1, 3000);
            TEST_ASSERT_EQUAL_INT(1, completed_responses);
        } else {
            handle_go2rtc_proxy(&threaded_connection.request, &threaded_connection.response);
        }
        fail_lease_uuid = false;
        if (fail_issuance) {
            struct pollfd ready = {.fd = fixture_upstream_listener, .events = POLLIN};
            TEST_ASSERT_EQUAL_INT(0, poll(&ready, 1, 0));
        }
        if (fixture_peer_started) {
            pthread_join(fixture_peer_thread, NULL);
            fixture_peer_started = false;
        }
        close(fixture_upstream_listener);
        fixture_upstream_listener = -1;
        if (!acknowledged || fail_issuance) {
            TEST_ASSERT_EQUAL_INT(fail_issuance ? 503 : 502, threaded_connection.response.status_code);
            TEST_ASSERT_NULL(strstr((char *)threaded_connection.response.body, sdp_answer));
            if (!acknowledged && !fail_issuance) {
                TEST_ASSERT_NOT_NULL(strstr((char *)threaded_connection.response.body,
                                            "compatible OpenSensor go2rtc fork"));
            }
            http_response_free(&threaded_connection.response);
            continue;
        }
        TEST_ASSERT_EQUAL_INT(201, threaded_connection.response.status_code);
        TEST_ASSERT_EQUAL_STRING("application/sdp", threaded_connection.response.content_type);
        TEST_ASSERT_EQUAL_UINT(strlen(sdp_answer), threaded_connection.response.body_length);
        TEST_ASSERT_EQUAL_MEMORY(sdp_answer, threaded_connection.response.body, strlen(sdp_answer));
        TEST_ASSERT_NOT_NULL(strstr(upstream_request, yaml_validate_is_available()
            ? "POST /custom/api/webrtc?src=Front%20Gate HTTP/1.1"
            : "POST /go2rtc/api/webrtc?src=Front%20Gate HTTP/1.1"));
        TEST_ASSERT_NOT_NULL(strstr(upstream_request, "Content-Type: application/sdp\r\n"));
        TEST_ASSERT_NOT_NULL(strstr(upstream_request, sdp_offer));
        http_response_free(&threaded_connection.response);
    }
    shutdown_database();
    unlink(db_path);
}

void test_both_proxy_handlers_forward_authorized_webrtc_sdp(void) {
    check_both_webrtc_proxies(true, false);
}
void test_both_proxy_handlers_reject_missing_media_ack(void) {
    check_both_webrtc_proxies(false, false);
}
void test_interim_ack_does_not_authorize_final_response(void) {
    informational_media_ack = true;
    check_both_webrtc_proxies(false, false);
}
void test_both_proxy_handlers_reject_lease_failure_without_upstream_contact(void) {
    check_both_webrtc_proxies(true, true);
}

static void *release_delayed_worker(void *unused) {
    (void)unused;
    struct timespec delay = {.tv_sec = 11};
    nanosleep(&delay, NULL);
    pthread_mutex_lock(&upstream_mutex);
    release_upstream = true;
    pthread_cond_broadcast(&upstream_ready);
    pthread_mutex_unlock(&upstream_mutex);
    return NULL;
}

void test_shutdown_waits_for_a_worker_beyond_the_old_ten_second_timeout(void) {
    libuv_connection_t connection;
    init_connection(&connection);
    // Shutdown ownership is independent of media policy; use the snapshot worker.
    strcpy(connection.request.path, "/go2rtc/api/frame.jpeg");
    hold_upstream = true;
    TEST_ASSERT_EQUAL_INT(0, go2rtc_proxy_thread_submit(&connection, WRITE_ACTION_CLOSE));
    pthread_mutex_lock(&upstream_mutex);
    struct timespec deadline;
    TEST_ASSERT_EQUAL_INT(0, clock_gettime(CLOCK_REALTIME, &deadline));
    deadline.tv_sec += 5;
    int wait_rc = 0;
    while (!worker_in_curl && wait_rc == 0) {
        wait_rc = pthread_cond_timedwait(&upstream_ready, &upstream_mutex, &deadline);
    }
    pthread_mutex_unlock(&upstream_mutex);
    TEST_ASSERT_TRUE_MESSAGE(worker_in_curl, "Timed out waiting for worker to enter curl");
    pthread_t releaser;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&releaser, NULL, release_delayed_worker, NULL));
    long started = monotonic_ms();
    go2rtc_proxy_thread_shutdown();
    long elapsed = monotonic_ms() - started;
    pthread_join(releaser, NULL);
    TEST_ASSERT_GREATER_OR_EQUAL_INT64(10500, elapsed);
    TEST_ASSERT_EQUAL_INT(0, completed_responses);
    TEST_ASSERT_EQUAL_INT(-1, go2rtc_proxy_thread_submit(&connection, WRITE_ACTION_CLOSE));
    uv_run(&loop, UV_RUN_DEFAULT);
    TEST_ASSERT_EQUAL_INT(0, go2rtc_proxy_thread_init(&loop));
    http_response_free(&connection.response);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_shutdown_waits_for_a_worker_beyond_the_old_ten_second_timeout);
    RUN_TEST(test_both_proxy_handlers_forward_authorized_webrtc_sdp);
    RUN_TEST(test_both_proxy_handlers_reject_missing_media_ack);
    RUN_TEST(test_interim_ack_does_not_authorize_final_response);
    RUN_TEST(test_both_proxy_handlers_reject_lease_failure_without_upstream_contact);
    RUN_TEST(test_proxy_rejects_overflow_and_recovers_slots_after_completion);
    RUN_TEST(test_slow_snapshot_releases_its_slot_after_ten_seconds);
    RUN_TEST(test_both_proxy_handlers_never_follow_unauthorized_redirects);
    RUN_TEST(test_proxy_rejects_unsafe_requests_before_contacting_upstream);
    return UNITY_END();
}
