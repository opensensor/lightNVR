#include <stdbool.h>
#include <arpa/inet.h>
#include <sys/socket.h>
#include <unistd.h>
#include "unity.h"
#include "core/config.h"
#include "web/libuv_server.h"

static bool fail_next_thread;

int __real_uv_thread_create(uv_thread_t *, void (*)(void *), void *);
int __wrap_uv_thread_create(uv_thread_t *thread, void (*entry)(void *), void *arg) {
    if (fail_next_thread) {
        fail_next_thread = false;
        return UV_EAGAIN;
    }
    return __real_uv_thread_create(thread, entry, arg);
}

void setUp(void) { load_default_config(&g_config); }
void tearDown(void) {}

static void test_failed_thread_start_closes_handles_before_retry(void) {
    http_server_config_t config = {0};
    config.bind_ip = "127.0.0.1";
    config.web_root = "/nonexistent-thread-start-fixture";
    config.max_connections = 8;
    http_server_handle_t server = libuv_server_init(&config);
    TEST_ASSERT_NOT_NULL(server);

    fail_next_thread = true;
    TEST_ASSERT_EQUAL_INT(-1, http_server_start(server));
    TEST_ASSERT_FALSE(fail_next_thread);
    TEST_ASSERT_EQUAL_INT(0, http_server_start(server));
    http_server_stop(server);
    http_server_destroy(server);
}

static void test_failed_bind_can_retry_after_port_released(void) {
    int occupied = socket(AF_INET, SOCK_STREAM, 0);
    TEST_ASSERT_TRUE(occupied >= 0);
    struct sockaddr_in address = {.sin_family = AF_INET,
        .sin_addr.s_addr = htonl(INADDR_LOOPBACK)};
    TEST_ASSERT_EQUAL_INT(0, bind(occupied, (struct sockaddr *)&address, sizeof(address)));
    TEST_ASSERT_EQUAL_INT(0, listen(occupied, 1));
    socklen_t length = sizeof(address);
    TEST_ASSERT_EQUAL_INT(0, getsockname(occupied, (struct sockaddr *)&address, &length));

    http_server_config_t config = {0};
    config.bind_ip = "127.0.0.1";
    config.port = ntohs(address.sin_port);
    config.web_root = "/nonexistent-thread-start-fixture";
    config.max_connections = 8;
    http_server_handle_t server = libuv_server_init(&config);
    TEST_ASSERT_NOT_NULL(server);
    TEST_ASSERT_EQUAL_INT(-1, http_server_start(server));
    close(occupied);
    TEST_ASSERT_EQUAL_INT(0, http_server_start(server));
    http_server_stop(server);
    http_server_destroy(server);
}

static void test_duplicate_start_is_rejected_without_disrupting_server(void) {
    http_server_config_t config = {0};
    config.bind_ip = "127.0.0.1";
    config.web_root = "/nonexistent-thread-start-fixture";
    config.max_connections = 8;
    http_server_handle_t server = libuv_server_init(&config);
    TEST_ASSERT_NOT_NULL(server);
    TEST_ASSERT_EQUAL_INT(0, http_server_start(server));
    TEST_ASSERT_EQUAL_INT(-1, http_server_start(server));
    http_server_stop(server);
    http_server_destroy(server);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_failed_thread_start_closes_handles_before_retry);
    RUN_TEST(test_failed_bind_can_retry_after_port_released);
    RUN_TEST(test_duplicate_start_is_rejected_without_disrupting_server);
    return UNITY_END();
}
