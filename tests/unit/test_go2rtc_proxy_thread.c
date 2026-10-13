#define _POSIX_C_SOURCE 200809L

#include <arpa/inet.h>
#include <pthread.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <time.h>
#include <unistd.h>
#include <curl/curl.h>

#include "unity.h"
#include "core/config.h"
#include "web/go2rtc_proxy_thread.h"
#include "web/api_handlers_go2rtc_proxy.h"
#include "web/libuv_server.h"
#include "web/request_response.h"

static uv_loop_t loop;
static int completed_responses;
static bool hold_upstream;
static bool release_upstream;
static pthread_mutex_t upstream_mutex = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t upstream_ready = PTHREAD_COND_INITIALIZER;

/* Keep authorization independent of a real user/camera database. The actual
 * proxy threads, curl timeout, admission limit, and done queue remain real. */
bool __wrap_go2rtc_proxy_authorize_request(const http_request_t *request,
                                          http_response_t *response) {
    (void)request;
    (void)response;
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
    completed_responses = 0;
    hold_upstream = false;
    release_upstream = false;
    TEST_ASSERT_EQUAL_INT(CURLE_OK, curl_global_init(CURL_GLOBAL_ALL));
    TEST_ASSERT_EQUAL_INT(0, uv_loop_init(&loop));
    TEST_ASSERT_EQUAL_INT(0, go2rtc_proxy_thread_init(&loop));
}

void tearDown(void) {
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

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_proxy_rejects_overflow_and_recovers_slots_after_completion);
    RUN_TEST(test_slow_snapshot_releases_its_slot_after_ten_seconds);
    return UNITY_END();
}
