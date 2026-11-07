#define _GNU_SOURCE
#include "../../src/web/api_handlers_health.c"
#include "unity.h"

config_t config;
http_server_handle_t http_server;
static pthread_mutex_t probe_mutex = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t probe_cond = PTHREAD_COND_INITIALIZER;
static bool probe_entered, release_probe;
static bool fail_create, fast_stop;
static unsigned creates, joins;
static atomic_int server_restarts;
void __wrap_http_server_stop(http_server_handle_t server) { (void)server; server_restarts++; }
int __wrap_http_server_start(http_server_handle_t server) { (void)server; server_restarts++; return 0; }
static atomic_bool shutdown_requested;

bool __wrap_httpd_secure_cookies(void) { return false; }
bool __wrap_is_shutdown_initiated(void) { return shutdown_requested; }
void __wrap__log_message_ctx(log_level_t level, const char *component, const char *stream,
                            const char *format, ...) {
    (void)level; (void)component; (void)stream; (void)format;
}
int __real_pthread_create(pthread_t *, const pthread_attr_t *, void *(*)(void *), void *);
int __wrap_pthread_create(pthread_t *thread, const pthread_attr_t *attr,
                          void *(*entry)(void *), void *arg) {
    creates++;
    return fail_create ? EAGAIN : __real_pthread_create(thread, attr, entry, arg);
}
int __real_pthread_join(pthread_t thread, void **value);
int __wrap_pthread_join(pthread_t thread, void **value) {
    joins++;
    return __real_pthread_join(thread, value);
}
int __real_usleep(useconds_t duration);
int __wrap_usleep(useconds_t duration) { return fast_stop ? 0 : __real_usleep(duration); }
CURLcode __wrap_curl_easy_perform(CURL *curl) {
    (void)curl;
    pthread_mutex_lock(&probe_mutex);
    probe_entered = true;
    pthread_cond_broadcast(&probe_cond);
    while (!release_probe) pthread_cond_wait(&probe_cond, &probe_mutex);
    pthread_mutex_unlock(&probe_mutex);
    return CURLE_COULDNT_CONNECT;
}

static void wait_for_exit(void) {
    for (int i = 0; i < 5000 && !g_health_thread_exited; i++) __real_usleep(1000);
    TEST_ASSERT_TRUE(g_health_thread_exited);
}
void setUp(void) {
    fail_create = fast_stop = false;
    shutdown_requested = false;
    creates = joins = 0;
    server_restarts = 0;
    http_server = (http_server_handle_t)&config;
    probe_entered = release_probe = false;
    load_default_config(&config);
    curl_global_init(CURL_GLOBAL_DEFAULT);
    init_health_check_system();
    g_health_check_interval = 3600;
}
void tearDown(void) {
    pthread_mutex_lock(&probe_mutex);
    release_probe = true;
    pthread_cond_broadcast(&probe_cond);
    pthread_mutex_unlock(&probe_mutex);
    fast_stop = false;
    cleanup_health_check_system();
    curl_global_cleanup();
}

static void test_never_started_and_create_failure_do_not_join_invalid_thread(void) {
    stop_health_check_thread();
    cleanup_health_check_system();
    TEST_ASSERT_EQUAL_UINT(0, joins);
    fail_create = true;
    start_health_check_thread();
    TEST_ASSERT_FALSE(g_health_thread_created);
    TEST_ASSERT_FALSE(g_health_thread_running);
    stop_health_check_thread();
    TEST_ASSERT_EQUAL_UINT(0, joins);
}

static void test_stop_wakes_interval_sleep_and_repeated_stop_joins_once(void) {
    start_health_check_thread();
    start_health_check_thread();
    TEST_ASSERT_EQUAL_UINT(1, creates);
    stop_health_check_thread();
    TEST_ASSERT_FALSE(g_health_thread_created);
    stop_health_check_thread();
    cleanup_health_check_system();
    TEST_ASSERT_EQUAL_UINT(1, joins);
    start_health_check_thread();
    stop_health_check_thread();
    TEST_ASSERT_EQUAL_UINT(2, creates);
    TEST_ASSERT_EQUAL_UINT(2, joins);
}

static void test_timed_out_probe_retains_ownership_until_exit_then_can_restart(void) {
    g_health_check_interval = 0;
    start_health_check_thread();
    struct timespec deadline;
    clock_gettime(CLOCK_REALTIME, &deadline);
    deadline.tv_sec += 5;
    pthread_mutex_lock(&probe_mutex);
    int rc = 0;
    while (!probe_entered && rc == 0) rc = pthread_cond_timedwait(&probe_cond, &probe_mutex, &deadline);
    bool entered = probe_entered;
    pthread_mutex_unlock(&probe_mutex);
    TEST_ASSERT_TRUE(entered);
    g_failed_health_checks = 7;
    g_server_needs_restart = true;
    fast_stop = true; // Exercise the production timeout without a five-second delay.
    cleanup_health_check_system();
    TEST_ASSERT_TRUE(g_health_thread_created);
    TEST_ASSERT_TRUE(g_health_thread_stopping);
    TEST_ASSERT_FALSE(is_web_server_healthy());
    TEST_ASSERT_EQUAL_INT(7, get_failed_health_checks());
    reset_health_metrics();
    config.web_port++;
    char old_url[sizeof(g_health_check_url)];
    strcpy(old_url, g_health_check_url);
    init_health_check_system();
    TEST_ASSERT_EQUAL_STRING(old_url, g_health_check_url);
    TEST_ASSERT_EQUAL_INT(7, get_failed_health_checks());
    start_health_check_thread();
    stop_health_check_thread();
    TEST_ASSERT_EQUAL_UINT(1, creates);
    TEST_ASSERT_EQUAL_UINT(0, joins);
    pthread_mutex_lock(&probe_mutex);
    release_probe = true;
    pthread_cond_broadcast(&probe_cond);
    pthread_mutex_unlock(&probe_mutex);
    wait_for_exit();
    TEST_ASSERT_EQUAL_INT(0, server_restarts);
    fast_stop = false;
    cleanup_health_check_system();
    TEST_ASSERT_FALSE(g_health_thread_created);
    TEST_ASSERT_FALSE(g_health_thread_stopping);
    TEST_ASSERT_EQUAL_UINT(1, joins);
    TEST_ASSERT_EQUAL_INT(0, get_failed_health_checks());
    init_health_check_system();
    g_health_check_interval = 3600;
    start_health_check_thread();
    stop_health_check_thread();
    TEST_ASSERT_EQUAL_UINT(2, creates);
    TEST_ASSERT_EQUAL_UINT(2, joins);
}

static void test_worker_exit_during_shutdown_is_reaped(void) {
    shutdown_requested = true;
    start_health_check_thread();
    wait_for_exit();
    stop_health_check_thread();
    TEST_ASSERT_FALSE(g_health_thread_created);
    TEST_ASSERT_EQUAL_UINT(1, joins);
}
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_never_started_and_create_failure_do_not_join_invalid_thread);
    RUN_TEST(test_stop_wakes_interval_sleep_and_repeated_stop_joins_once);
    RUN_TEST(test_timed_out_probe_retains_ownership_until_exit_then_can_restart);
    RUN_TEST(test_worker_exit_during_shutdown_is_reaped);
    return UNITY_END();
}
