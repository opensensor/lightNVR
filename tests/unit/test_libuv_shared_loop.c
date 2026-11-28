#include "unity.h"
#include "core/config.h"
#include "web/libuv_server.h"

static uv_loop_t loop;
static uv_async_t foreign_async;
static uv_timer_t stop_timer;
static libuv_server_t *callback_server;
static bool stopped_in_callback;

void setUp(void) {}
void tearDown(void) {}

static void foreign_cb(uv_async_t *handle) { (void)handle; }

static void test_stop_preserves_foreign_handle(void) {
    TEST_ASSERT_EQUAL_INT(0, uv_loop_init(&loop));
    TEST_ASSERT_EQUAL_INT(0, uv_async_init(&loop, &foreign_async, foreign_cb));
    load_default_config(&g_config);
    http_server_config_t config = {0};
    config.bind_ip = "127.0.0.1";
    config.web_root = "/nonexistent-shared-loop-fixture";
    config.max_connections = 8;
    libuv_server_t *server = (libuv_server_t *)libuv_server_init_with_loop(&config, &loop);
    TEST_ASSERT_NOT_NULL(server);
    TEST_ASSERT_EQUAL_INT(0, http_server_start((http_server_handle_t)server));

    http_server_stop((http_server_handle_t)server);
    TEST_ASSERT_FALSE(uv_is_closing((uv_handle_t *)&foreign_async));
    TEST_ASSERT_TRUE(uv_loop_alive(&loop));
    http_server_destroy((http_server_handle_t)server);

    uv_close((uv_handle_t *)&foreign_async, NULL);
    while (uv_loop_alive(&loop)) uv_run(&loop, UV_RUN_DEFAULT);
    TEST_ASSERT_EQUAL_INT(0, uv_loop_close(&loop));
}

static void stop_in_callback(uv_timer_t *handle) {
    http_server_stop((http_server_handle_t)callback_server);
    http_server_destroy((http_server_handle_t)callback_server);
    stopped_in_callback = true;
    uv_close((uv_handle_t *)handle, NULL);
}

static void test_stop_and_destroy_from_shared_loop_callback(void) {
    TEST_ASSERT_EQUAL_INT(0, uv_loop_init(&loop));
    TEST_ASSERT_EQUAL_INT(0, uv_async_init(&loop, &foreign_async, foreign_cb));
    http_server_config_t config = {0};
    config.bind_ip = "127.0.0.1";
    config.web_root = "/nonexistent-shared-loop-fixture";
    config.max_connections = 8;
    callback_server = (libuv_server_t *)libuv_server_init_with_loop(&config, &loop);
    TEST_ASSERT_NOT_NULL(callback_server);
    TEST_ASSERT_EQUAL_INT(0, http_server_start((http_server_handle_t)callback_server));
    TEST_ASSERT_EQUAL_INT(0, uv_timer_init(&loop, &stop_timer));
    TEST_ASSERT_EQUAL_INT(0, uv_timer_start(&stop_timer, stop_in_callback, 0, 0));
    stopped_in_callback = false;
    uv_run(&loop, UV_RUN_NOWAIT);
    TEST_ASSERT_TRUE(stopped_in_callback);
    TEST_ASSERT_FALSE(uv_is_closing((uv_handle_t *)&foreign_async));
    uv_close((uv_handle_t *)&foreign_async, NULL);
    while (uv_loop_alive(&loop)) uv_run(&loop, UV_RUN_DEFAULT);
    TEST_ASSERT_EQUAL_INT(0, uv_loop_close(&loop));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_stop_preserves_foreign_handle);
    RUN_TEST(test_stop_and_destroy_from_shared_loop_callback);
    return UNITY_END();
}
