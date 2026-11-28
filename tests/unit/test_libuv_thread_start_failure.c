#include <stdbool.h>
#include <arpa/inet.h>
#include <sys/socket.h>
#include <unistd.h>
#include "unity.h"
#include "core/config.h"
#include "web/libuv_server.h"

static bool fail_next_thread;
static http_server_handle_t fixture_server;
static int fixture_occupied_fd;

int __real_uv_thread_create(uv_thread_t *, void (*)(void *), void *);
int __wrap_uv_thread_create(uv_thread_t *thread, void (*entry)(void *), void *arg) {
    if (fail_next_thread) {
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
    fail_next_thread = false;
}

static void test_failed_thread_start_closes_handles_before_retry(void) {
    http_server_config_t config = {0};
    config.bind_ip = "127.0.0.1";
    config.web_root = "/nonexistent-thread-start-fixture";
    config.max_connections = 8;
    fixture_server = libuv_server_init(&config);
    TEST_ASSERT_NOT_NULL(fixture_server);

    fail_next_thread = true;
    TEST_ASSERT_EQUAL_INT(-1, http_server_start(fixture_server));
    TEST_ASSERT_FALSE(fail_next_thread);
    TEST_ASSERT_EQUAL_INT(0, http_server_start(fixture_server));
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
    fixture_server = libuv_server_init(&config);
    TEST_ASSERT_NOT_NULL(fixture_server);
    TEST_ASSERT_EQUAL_INT(-1, http_server_start(fixture_server));
    close(fixture_occupied_fd);
    fixture_occupied_fd = -1;
    TEST_ASSERT_EQUAL_INT(0, http_server_start(fixture_server));
}

static void test_duplicate_start_is_rejected_without_disrupting_server(void) {
    http_server_config_t config = {0};
    config.bind_ip = "127.0.0.1";
    config.web_root = "/nonexistent-thread-start-fixture";
    config.max_connections = 8;
    fixture_server = libuv_server_init(&config);
    TEST_ASSERT_NOT_NULL(fixture_server);
    TEST_ASSERT_EQUAL_INT(0, http_server_start(fixture_server));
    TEST_ASSERT_EQUAL_INT(-1, http_server_start(fixture_server));

    int client_fd = socket(AF_INET, SOCK_STREAM, 0);
    TEST_ASSERT_TRUE(client_fd >= 0);

    struct sockaddr_in server_addr = {0};
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(g_config.port);
    server_addr.sin_addr.s_addr = htonl(INADDR_LOOPBACK);

    TEST_ASSERT_EQUAL_INT(0, connect(client_fd, (struct sockaddr *)&server_addr, sizeof(server_addr)));

    const char request[] = "GET / HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n";
    ssize_t sent = send(client_fd, request, sizeof(request) - 1, 0);
    TEST_ASSERT_TRUE(sent > 0);

    char response[256];
    ssize_t received = recv(client_fd, response, sizeof(response), 0);
    TEST_ASSERT_TRUE(received > 0);

    close(client_fd);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_failed_thread_start_closes_handles_before_retry);
    RUN_TEST(test_failed_bind_can_retry_after_port_released);
    RUN_TEST(test_duplicate_start_is_rejected_without_disrupting_server);
    return UNITY_END();
}
