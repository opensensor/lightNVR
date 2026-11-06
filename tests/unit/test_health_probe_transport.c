/* The watchdog must follow the effective listener, including TLS fallback. */
#include "../../src/web/api_handlers_health.c"
#include "unity.h"

/* The application entry point owns this listener configuration. */
config_t config;
static bool listener_tls;
bool __wrap_httpd_secure_cookies(void) { return listener_tls; }
void setUp(void) { load_default_config(&config); listener_tls = false; }
void tearDown(void) {}

static void test_plain_listener_uses_credential_free_loopback_liveness(void) {
    config.web_port = 18080;
    init_health_check_system();
    TEST_ASSERT_FALSE(g_health_check_tls);
    TEST_ASSERT_EQUAL_STRING("http://127.0.0.1:18080/api/health?liveness=true", g_health_check_url);
}
static void test_tls_listener_uses_loopback_https(void) {
    listener_tls = true;
    config.web_port = 18443;
    init_health_check_system();
    TEST_ASSERT_TRUE(g_health_check_tls);
    TEST_ASSERT_EQUAL_STRING("https://127.0.0.1:18443/api/health?liveness=true", g_health_check_url);
}
static void test_configured_tls_does_not_override_plain_listener_fallback(void) {
    config.web_ssl_enabled = true;
    config.web_port = 18443;
    init_health_check_system();
    TEST_ASSERT_FALSE(g_health_check_tls);
    TEST_ASSERT_EQUAL_STRING("http://127.0.0.1:18443/api/health?liveness=true", g_health_check_url);
}
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_plain_listener_uses_credential_free_loopback_liveness);
    RUN_TEST(test_tls_listener_uses_loopback_https);
    RUN_TEST(test_configured_tls_does_not_override_plain_listener_fallback);
    return UNITY_END();
}
