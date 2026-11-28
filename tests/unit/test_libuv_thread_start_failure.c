#include <stdbool.h>
#include <errno.h>
#include <string.h>
#include <arpa/inet.h>
#include <sys/socket.h>
#include <sys/time.h>
#include <unistd.h>
#include "unity.h"
#include "core/config.h"
#include "web/libuv_server.h"

static bool fail_next_thread;
static http_server_handle_t fixture_server;
static int fixture_occupied_fd;
static int fixture_client_fd;
static uint16_t failed_listener_port;

static void probe_handler(const http_request_t *request, http_response_t *response) {
    (void)request;
    http_response_set_json(response, 200, "{\"ok\":true}");
}

int __real_uv_thread_create(uv_thread_t *, void (*)(void *), void *);
int __wrap_uv_thread_create(uv_thread_t *thread, void (*entry)(void *), void *arg) {
    if (fail_next_thread) {
        struct sockaddr_in bound = {0};
        int length = sizeof(bound);
        if (fixture_server && uv_tcp_getsockname(
                &((libuv_server_t *)fixture_server)->listener,
                (struct sockaddr *)&bound, &length) == 0)
            failed_listener_port = ntohs(bound.sin_port);
        fail_next_thread = false;
        return UV_EAGAIN;
    }
    return __real_uv_thread_create(thread, entry, arg);
}

void setUp(void) {
    load_default_config(&g_config);
    fail_next_thread = false;
    fixture_server = NULL;
    fixture_occupied_fd = -1;
    fixture_client_fd = -1;
    failed_listener_port = 0;
}

void tearDown(void) {
    if (fixture_server != NULL) {
        http_server_stop(fixture_server);
        http_server_destroy(fixture_server);
        fixture_server = NULL;
    }
    if (fixture_occupied_fd >= 0) {
        close(fixture_occupied_fd);
        fixture_occupied_fd = -1;
    }
    if (fixture_client_fd >= 0) {
        close(fixture_client_fd);
        fixture_client_fd = -1;
    }
    fail_next_thread = false;
}

static void test_failed_thread_start_closes_handles_before_retry(void) {
    http_server_config_t config = {0};
    config.bind_ip = "127.0.0.1";
    config.web_root = "/nonexistent-thread-start-fixture";
    config.max_connections = 8;
    config.request_handler = probe_handler;
    fixture_server = libuv_server_init(&config);
    TEST_ASSERT_NOT_NULL(fixture_server);

    fail_next_thread = true;
    TEST_ASSERT_EQUAL_INT(-1, http_server_start(fixture_server));
    TEST_ASSERT_FALSE(fail_next_thread);
    TEST_ASSERT_TRUE(failed_listener_port > 0);
    fixture_occupied_fd = socket(AF_INET, SOCK_STREAM, 0);
    TEST_ASSERT_TRUE(fixture_occupied_fd >= 0);
    struct sockaddr_in released = {.sin_family = AF_INET,
        .sin_addr.s_addr = htonl(INADDR_LOOPBACK),
        .sin_port = htons(failed_listener_port)};
    TEST_ASSERT_EQUAL_INT(0, bind(fixture_occupied_fd,
        (struct sockaddr *)&released, sizeof(released)));
    TEST_ASSERT_EQUAL_INT(0, close(fixture_occupied_fd));
    fixture_occupied_fd = -1;
    TEST_ASSERT_EQUAL_INT(0, http_server_start(fixture_server));
    struct sockaddr_in assigned = {0};
    int assigned_length = sizeof(assigned);
    TEST_ASSERT_EQUAL_INT(0, uv_tcp_getsockname(
        &((libuv_server_t *)fixture_server)->listener,
        (struct sockaddr *)&assigned, &assigned_length));
    TEST_ASSERT_TRUE(ntohs(assigned.sin_port) > 0);
    assert_http_probe_ok("127.0.0.1", ntohs(assigned.sin_port));
}

static void test_failed_bind_can_retry_after_port_released(void) {
    fixture_occupied_fd = socket(AF_INET, SOCK_STREAM, 0);
    TEST_ASSERT_TRUE(fixture_occupied_fd >= 0);
    struct sockaddr_in address = {.sin_family = AF_INET,
        .sin_addr.s_addr = htonl(INADDR_LOOPBACK)};
    TEST_ASSERT_EQUAL_INT(0, bind(fixture_occupied_fd, (struct sockaddr *)&address, sizeof(address)));
    TEST_ASSERT_EQUAL_INT(0, listen(fixture_occupied_fd, 1));
    socklen_t length = sizeof(address);
    TEST_ASSERT_EQUAL_INT(0, getsockname(fixture_occupied_fd, (struct sockaddr *)&address, &length));

    http_server_config_t config = {0};
    config.bind_ip = "127.0.0.1";
    config.port = ntohs(address.sin_port);
    config.web_root = "/nonexistent-thread-start-fixture";
    config.max_connections = 8;
    config.request_handler = probe_handler;
    fixture_server = libuv_server_init(&config);
    TEST_ASSERT_NOT_NULL(fixture_server);
    TEST_ASSERT_EQUAL_INT(-1, http_server_start(fixture_server));
    close(fixture_occupied_fd);
    fixture_occupied_fd = -1;
    TEST_ASSERT_EQUAL_INT(0, http_server_start(fixture_server));
    assert_http_probe_ok("127.0.0.1", ntohs(address.sin_port));
}

static void test_duplicate_start_is_rejected_without_disrupting_server(void) {
    http_server_config_t config = {0};
    config.bind_ip = "127.0.0.1";
    config.web_root = "/nonexistent-thread-start-fixture";
    config.max_connections = 8;
    fixture_server = libuv_server_init(&config);
    TEST_ASSERT_NOT_NULL(fixture_server);
    TEST_ASSERT_EQUAL_INT(0, http_server_register_handler(
        fixture_server, "/probe", "GET", probe_handler));
    TEST_ASSERT_EQUAL_INT(0, http_server_start(fixture_server));
    TEST_ASSERT_EQUAL_INT(-1, http_server_start(fixture_server));

    fixture_client_fd = socket(AF_INET, SOCK_STREAM, 0);
    TEST_ASSERT_TRUE(fixture_client_fd >= 0);

    struct sockaddr_in server_addr = {0};
    int address_length = sizeof(server_addr);
    TEST_ASSERT_EQUAL_INT(0, uv_tcp_getsockname(
        &((libuv_server_t *)fixture_server)->listener,
        (struct sockaddr *)&server_addr, &address_length));
    struct timeval timeout = {.tv_sec = 2};
    TEST_ASSERT_EQUAL_INT(0, setsockopt(fixture_client_fd, SOL_SOCKET,
        SO_RCVTIMEO, &timeout, sizeof(timeout)));

    TEST_ASSERT_EQUAL_INT(0, connect(fixture_client_fd,
        (struct sockaddr *)&server_addr, sizeof(server_addr)));

    const char request[] = "GET /probe HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n";
    size_t request_len = sizeof(request) - 1;
    size_t total_sent = 0;
    while (total_sent < request_len) {
        ssize_t sent = send(fixture_client_fd, request + total_sent,
            request_len - total_sent, 0);
        if (sent < 0 && errno == EINTR) continue;
        TEST_ASSERT_TRUE(sent > 0);
        total_sent += (size_t)sent;
    }

    char response[256];
    size_t total_received = 0;
    bool status_line_complete = false;
    while (total_received < sizeof(response) - 1 && !status_line_complete) {
        ssize_t chunk = recv(fixture_client_fd, response + total_received,
            (sizeof(response) - 1) - total_received, 0);
        if (chunk < 0 && errno == EINTR) continue;
        if (chunk <= 0) {
            break;
        }
        total_received += (size_t)chunk;
        response[total_received] = '\0';
        if (strstr(response, "\r\n") != NULL) {
            status_line_complete = true;
        }
    }
    TEST_ASSERT_TRUE(total_received > 0);
    TEST_ASSERT_TRUE(status_line_complete);
    TEST_ASSERT_NOT_NULL(strstr(response, "HTTP/1.1 200"));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_failed_thread_start_closes_handles_before_retry);
    RUN_TEST(test_failed_bind_can_retry_after_port_released);
    RUN_TEST(test_duplicate_start_is_rejected_without_disrupting_server);
    return UNITY_END();
}
