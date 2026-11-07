#define _GNU_SOURCE
#include <stdarg.h>
#include <arpa/inet.h>
#include <poll.h>
#include <sys/socket.h>
/* The watchdog must follow the effective listener, including TLS fallback. */
#include "../../src/web/api_handlers_health.c"
#include "unity.h"

/* The application entry point owns this listener configuration. */
config_t config;
static bool listener_tls;
static bool fail_initial_allocation;
static unsigned curl_initializations;
static char last_log[1024];
void __wrap__log_message_ctx(log_level_t level, const char *component, const char *stream,
                            const char *format, ...) {
    (void)level; (void)component; (void)stream;
    va_list args;
    va_start(args, format);
    vsnprintf(last_log, sizeof(last_log), format, args);
    va_end(args);
}
void *__real_malloc(size_t size);
void *__wrap_malloc(size_t size) {
    if (fail_initial_allocation && size == 1) {
        fail_initial_allocation = false;
        return NULL;
    }
    return __real_malloc(size);
}
CURL *__real_curl_easy_init(void);
CURL *__wrap_curl_easy_init(void) {
    curl_initializations++;
    return __real_curl_easy_init();
}
bool __wrap_httpd_secure_cookies(void) { return listener_tls; }
void setUp(void) {
    load_default_config(&config);
    listener_tls = false;
    fail_initial_allocation = false;
    curl_initializations = 0;
    last_log[0] = '\0';
    TEST_ASSERT_EQUAL_INT(CURLE_OK, curl_global_init(CURL_GLOBAL_DEFAULT));
}
void tearDown(void) { curl_global_cleanup(); }

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
static void test_initial_response_allocation_failure_stops_before_curl(void) {
    fail_initial_allocation = true;
    TEST_ASSERT_FALSE(perform_health_check());
    TEST_ASSERT_FALSE(fail_initial_allocation);
    TEST_ASSERT_EQUAL_UINT(0, curl_initializations);
}

static void *empty_health_response(void *arg) {
    int listener = *(int *)arg;
    struct pollfd ready = {.fd = listener, .events = POLLIN};
    if (poll(&ready, 1, 3000) <= 0) return NULL;
    int client = accept(listener, NULL, NULL);
    if (client < 0) return NULL;
    const char response[] = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
    char request[2048];
    ssize_t received = recv(client, request, sizeof(request), 0);
    if (received > 0) (void)send(client, response, sizeof(response) - 1, MSG_NOSIGNAL);
    close(client);
    return NULL;
}

static void test_empty_success_response_is_a_valid_terminated_log_string(void) {
    int listener = socket(AF_INET, SOCK_STREAM, 0);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, listener);
    struct sockaddr_in address = {.sin_family = AF_INET, .sin_addr.s_addr = htonl(INADDR_LOOPBACK)};
    TEST_ASSERT_EQUAL_INT(0, bind(listener, (struct sockaddr *)&address, sizeof(address)));
    TEST_ASSERT_EQUAL_INT(0, listen(listener, 1));
    socklen_t size = sizeof(address);
    TEST_ASSERT_EQUAL_INT(0, getsockname(listener, (struct sockaddr *)&address, &size));
    snprintf(g_health_check_url, sizeof(g_health_check_url), "http://127.0.0.1:%d/health", ntohs(address.sin_port));
    g_health_check_tls = false;
    g_failed_health_checks = 2;
    pthread_t thread;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&thread, NULL, empty_health_response, &listener));
    bool success = perform_health_check();
    pthread_join(thread, NULL);
    close(listener);
    TEST_ASSERT_TRUE(success);
    TEST_ASSERT_TRUE(g_is_healthy);
    TEST_ASSERT_EQUAL_INT(0, g_failed_health_checks);
    TEST_ASSERT_EQUAL_STRING("Health check succeeded with response: ", last_log);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_plain_listener_uses_credential_free_loopback_liveness);
    RUN_TEST(test_tls_listener_uses_loopback_https);
    RUN_TEST(test_configured_tls_does_not_override_plain_listener_fallback);
    RUN_TEST(test_initial_response_allocation_failure_stops_before_curl);
    RUN_TEST(test_empty_success_response_is_a_valid_terminated_log_string);
    return UNITY_END();
}
