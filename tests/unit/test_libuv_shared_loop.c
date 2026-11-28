#include "unity.h"
#include "core/config.h"
#include "web/libuv_server.h"

static uv_loop_t loop;
static uv_async_t foreign_async;
static uv_timer_t stop_timer;
static libuv_server_t *callback_server;
static bool stopped_in_callback;
static bool loop_initialized;
static bool foreign_async_initialized;
static bool stop_timer_initialized;
static bool foreign_async_closed;
static bool stop_timer_closed;

void setUp(void) {}
void tearDown(void) {
    if (loop_initialized) {
        if (foreign_async_initialized && !foreign_async_closed &&
            !uv_is_closing((uv_handle_t *)&foreign_async)) {
            uv_close((uv_handle_t *)&foreign_async, NULL);
            foreign_async_closed = true;
        }
        if (stop_timer_initialized && !stop_timer_closed &&
            !uv_is_closing((uv_handle_t *)&stop_timer)) {
            uv_close((uv_handle_t *)&stop_timer, NULL);
            stop_timer_closed = true;
        }
        if (callback_server) {
            http_server_stop((http_server_handle_t)callback_server);
            http_server_destroy((http_server_handle_t)callback_server);
            callback_server = NULL;
        }
        while (uv_loop_alive(&loop)) uv_run(&loop, UV_RUN_DEFAULT);
        TEST_ASSERT_EQUAL_INT(0, uv_loop_close(&loop));
    }

    loop_initialized = false;
    foreign_async_initialized = false;
    stop_timer_initialized = false;
    foreign_async_closed = false;
    stop_timer_closed = false;
    callback_server = NULL;
    stopped_in_callback = false;
}

static void foreign_cb(uv_async_t *handle) { (void)handle; }

static void test_stop_preserves_foreign_handle(void) {
    TEST_ASSERT_EQUAL_INT(0, uv_loop_init(&loop));
    loop_initialized = true;
    TEST_ASSERT_EQUAL_INT(0, uv_async_init(&loop, &foreign_async, foreign_cb));
    foreign_async_initialized = true;
    foreign_async_closed = false;
    load_default_config(&g_config);
    http_server_config_t config = {0};
    config.bind_ip = "127.0.0.1";
    config.web_root = "/nonexistent-shared-loop-fixture";
    config.max_connections = 8;
    callback_server = (libuv_server_t *)libuv_server_init_with_loop(&config, &loop);
    TEST_ASSERT_NOT_NULL(callback_server);
    TEST_ASSERT_EQUAL_INT(0, http_server_start((http_server_handle_t)callback_server));

    http_server_stop((http_server_handle_t)callback_server);
    TEST_ASSERT_FALSE(uv_is_closing((uv_handle_t *)&foreign_async));
    TEST_ASSERT_TRUE(uv_loop_alive(&loop));
    http_server_destroy((http_server_handle_t)callback_server);
    callback_server = NULL;
}

static void stop_in_callback(uv_timer_t *handle) {
    http_server_stop((http_server_handle_t)callback_server);
    http_server_destroy((http_server_handle_t)callback_server);
    callback_server = NULL;
    stopped_in_callback = true;
    uv_close((uv_handle_t *)handle, NULL);
    stop_timer_closed = true;
}

static void test_stop_and_destroy_from_shared_loop_callback(void) {
    TEST_ASSERT_EQUAL_INT(0, uv_loop_init(&loop));
    loop_initialized = true;
    TEST_ASSERT_EQUAL_INT(0, uv_async_init(&loop, &foreign_async, foreign_cb));
    foreign_async_initialized = true;
    foreign_async_closed = false;
    http_server_config_t config = {0};
    config.bind_ip = "127.0.0.1";
    config.web_root = "/nonexistent-shared-loop-fixture";
    config.max_connections = 8;
    callback_server = (libuv_server_t *)libuv_server_init_with_loop(&config, &loop);
    TEST_ASSERT_NOT_NULL(callback_server);
    TEST_ASSERT_EQUAL_INT(0, http_server_start((http_server_handle_t)callback_server));
    TEST_ASSERT_EQUAL_INT(0, uv_timer_init(&loop, &stop_timer));
    stop_timer_initialized = true;
    stop_timer_closed = false;
    TEST_ASSERT_EQUAL_INT(0, uv_timer_start(&stop_timer, stop_in_callback, 0, 0));
    stopped_in_callback = false;
    uv_run(&loop, UV_RUN_NOWAIT);
    TEST_ASSERT_TRUE(stopped_in_callback);
    TEST_ASSERT_FALSE(uv_is_closing((uv_handle_t *)&foreign_async));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_stop_preserves_foreign_handle);
    RUN_TEST(test_stop_and_destroy_from_shared_loop_callback);
    return UNITY_END();
}
