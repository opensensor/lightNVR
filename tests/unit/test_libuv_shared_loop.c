#include "unity.h"
#include "core/config.h"
#include "web/libuv_server.h"

static uv_loop_t loop;
static uv_async_t foreign_async;

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

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_stop_preserves_foreign_handle);
    return UNITY_END();
}
