/* Real libuv worker/connection ownership regressions. All waits are finite. */
#define _POSIX_C_SOURCE 200809L
#include <arpa/inet.h>
#include <errno.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#include <sys/socket.h>
#include <time.h>
#include <unistd.h>
#include "unity.h"
#include "core/config.h"
#include "web/libuv_connection.h"
#include "web/libuv_server.h"

extern int libuv_server_register_handler(http_server_handle_t, const char *, const char *, request_handler_t);
extern int libuv_server_start(http_server_handle_t);
extern void libuv_server_stop(http_server_handle_t);
extern void libuv_server_destroy(http_server_handle_t);

static libuv_server_t *server;
static int client = -1;
static pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t changed = PTHREAD_COND_INITIALIZER;
static bool entered, released, finished, expired, server_state_valid;
static libuv_connection_t *connection;
static uv_async_t close_request;
static uv_prepare_t next_iteration;

static struct timespec deadline(unsigned seconds) {
    struct timespec value;
    clock_gettime(CLOCK_REALTIME, &value);
    value.tv_sec += seconds;
    return value;
}

static bool await_flag(bool *flag) {
    struct timespec end = deadline(5);
    pthread_mutex_lock(&mutex);
    while (!*flag) {
        if (pthread_cond_timedwait(&changed, &mutex, &end) == ETIMEDOUT) break;
    }
    bool value = *flag;
    pthread_mutex_unlock(&mutex);
    return value;
}

static void release_handler(void) {
    pthread_mutex_lock(&mutex);
    released = true;
    pthread_cond_broadcast(&changed);
    pthread_mutex_unlock(&mutex);
}

static void blocked_handler(const http_request_t *req, http_response_t *res) {
    struct timespec end = deadline(5);
    pthread_mutex_lock(&mutex);
    connection = req->user_data;
    entered = true;
    pthread_cond_broadcast(&changed);
    while (!released) {
        if (pthread_cond_timedwait(&changed, &mutex, &end) == ETIMEDOUT) {
            expired = true;
            break;
        }
    }
    pthread_mutex_unlock(&mutex);
    // The connection, server, request and response must all survive the gate.
    bool valid = strcmp(((libuv_connection_t *)req->user_data)->server->config.web_root,
                        "/nonexistent-lifecycle-fixture") == 0;
    http_response_set_json(res, 200, "{\"finished\":true}");
    pthread_mutex_lock(&mutex);
    server_state_valid = valid;
    finished = true;
    pthread_cond_broadcast(&changed);
    pthread_mutex_unlock(&mutex);
}

static void after_close_iteration(uv_prepare_t *handle) {
    // The async callback ran in the preceding poll phase. Its TCP close
    // callback has now run before this next iteration's prepare phase.
    uv_prepare_stop(handle);
    uv_close((uv_handle_t *)handle, NULL);
    release_handler();
}

static void close_on_loop(uv_async_t *handle) {
    pthread_mutex_lock(&mutex);
    libuv_connection_t *conn = connection;
    pthread_mutex_unlock(&mutex);
    libuv_connection_close(conn);
    uv_prepare_init(server->loop, &next_iteration);
    uv_prepare_start(&next_iteration, after_close_iteration);
    uv_close((uv_handle_t *)handle, NULL);
}

static void *release_after_four_seconds(void *unused) {
    (void)unused;
    struct timespec duration = {.tv_sec = 4};
    while (nanosleep(&duration, &duration) != 0 && errno == EINTR) {}
    release_handler();
    return NULL;
}

void setUp(void) {
    load_default_config(&g_config);
    g_config.web_auth_enabled = false;
    g_config.web_compression_enabled = false;
    g_config.web_thread_pool_size = 2;
    entered = released = finished = expired = server_state_valid = false;
    connection = NULL;
    http_server_config_t config = {0};
    config.bind_ip = "127.0.0.1";
    config.web_root = "/nonexistent-lifecycle-fixture";
    config.max_connections = 8;
    server = (libuv_server_t *)libuv_server_init(&config);
    TEST_ASSERT_NOT_NULL(server);
    TEST_ASSERT_EQUAL_INT(0, libuv_server_register_handler((http_server_handle_t)server,
        "/api/lifecycle-test", "GET", blocked_handler));
    TEST_ASSERT_EQUAL_INT(0, uv_async_init(server->loop, &close_request, close_on_loop));
    TEST_ASSERT_EQUAL_INT(0, libuv_server_start((http_server_handle_t)server));
    struct sockaddr_in address;
    int size = sizeof(address);
    TEST_ASSERT_EQUAL_INT(0, uv_tcp_getsockname(&server->listener, (struct sockaddr *)&address, &size));
    client = socket(AF_INET, SOCK_STREAM, 0);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, client);
    TEST_ASSERT_EQUAL_INT(0, connect(client, (struct sockaddr *)&address, sizeof(address)));
    const char request[] = "GET /api/lifecycle-test HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    TEST_ASSERT_EQUAL_INT(sizeof(request) - 1, send(client, request, sizeof(request) - 1, 0));
    TEST_ASSERT_TRUE(await_flag(&entered));
}

void tearDown(void) {
    release_handler();
    if (client >= 0) close(client);
    client = -1;
    if (server) {
        libuv_server_stop((http_server_handle_t)server);
        libuv_server_destroy((http_server_handle_t)server);
        server = NULL;
    }
}

static void test_disconnect_close_keeps_worker_response_alive(void) {
    shutdown(client, SHUT_RDWR);
    close(client);
    client = -1;
    // Reads are paused during worker execution, so force the same loop-side
    // close path after peer disconnect rather than wait for a read callback.
    TEST_ASSERT_EQUAL_INT(0, uv_async_send(&close_request));
    TEST_ASSERT_TRUE(await_flag(&finished));
    TEST_ASSERT_FALSE(expired);
    TEST_ASSERT_TRUE(server_state_valid);
}

static void test_shutdown_waits_for_handler_beyond_three_second_warning(void) {
    pthread_t release_thread;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&release_thread, NULL, release_after_four_seconds, NULL));
    struct timespec before, after;
    clock_gettime(CLOCK_MONOTONIC, &before);
    libuv_server_stop((http_server_handle_t)server);
    clock_gettime(CLOCK_MONOTONIC, &after);
    double elapsed = after.tv_sec - before.tv_sec + (after.tv_nsec - before.tv_nsec) / 1e9;
    pthread_mutex_lock(&mutex);
    bool done_at_stop = finished;
    pthread_mutex_unlock(&mutex);
    pthread_join(release_thread, NULL);
    TEST_ASSERT_TRUE(done_at_stop);
    TEST_ASSERT_TRUE(server_state_valid);
    TEST_ASSERT_TRUE(elapsed >= 3.5 && elapsed < 6.0);
    TEST_ASSERT_FALSE(expired);
    TEST_ASSERT_EQUAL_INT(0, uv_loop_alive(server->loop));
    libuv_server_destroy((http_server_handle_t)server);
    server = NULL;
}

static void test_destroy_initialized_server_without_start(void) {
    tearDown();
    http_server_config_t config = {0};
    config.bind_ip = "127.0.0.1";
    config.web_root = "/nonexistent-lifecycle-fixture";
    config.max_connections = 8;
    server = (libuv_server_t *)libuv_server_init(&config);
    TEST_ASSERT_NOT_NULL(server);
    TEST_ASSERT_EQUAL_INT(0, libuv_server_register_handler((http_server_handle_t)server,
        "/unused", "GET", blocked_handler));
    libuv_server_destroy((http_server_handle_t)server);
    server = NULL; /* ASan/LSan verifies registry and initialized handles freed. */
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_disconnect_close_keeps_worker_response_alive);
    RUN_TEST(test_shutdown_waits_for_handler_beyond_three_second_warning);
    RUN_TEST(test_destroy_initialized_server_without_start);
    return UNITY_END();
}
