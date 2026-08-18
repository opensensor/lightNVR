/**
 * @file test_go2rtc_process_lifecycle.c
 * @brief Process-level race, PID ownership, and child-reaping regressions.
 */

#define _POSIX_C_SOURCE 200809L

#include <errno.h>
#include <netinet/in.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

#include "unity.h"
#include "core/config.h"
#include "core/logger.h"
#include "video/go2rtc/go2rtc_api.h"
#include "video/go2rtc/go2rtc_process.h"

#ifndef FAKE_GO2RTC_PATH
#error "FAKE_GO2RTC_PATH must name the native test server"
#endif

static char config_dir[128];
static char start_log[160];
static int api_port;
static bool process_initialized;
static bool api_initialized;

static int reserve_local_port(void) {
    int fd = socket(AF_INET, SOCK_STREAM, 0);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, fd);

    struct sockaddr_in address;
    memset(&address, 0, sizeof(address));
    address.sin_family = AF_INET;
    address.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    address.sin_port = 0;
    TEST_ASSERT_EQUAL_INT(0, bind(fd, (struct sockaddr *)&address, sizeof(address)));

    socklen_t length = sizeof(address);
    TEST_ASSERT_EQUAL_INT(0, getsockname(fd, (struct sockaddr *)&address, &length));
    int port = ntohs(address.sin_port);
    close(fd);
    return port;
}

static void initialize_manager(void) {
    load_default_config(&g_config);
    g_config.go2rtc_enabled = true;
    g_config.go2rtc_api_port = api_port;
    g_config.go2rtc_rtsp_port = 18554;
    g_config.go2rtc_webrtc_enabled = false;
    snprintf(g_config.log_file, sizeof(g_config.log_file), "%s/lightnvr.log", config_dir);

    TEST_ASSERT_TRUE(go2rtc_process_init(FAKE_GO2RTC_PATH, config_dir, api_port));
    process_initialized = true;
    TEST_ASSERT_TRUE(go2rtc_api_init("localhost", api_port));
    api_initialized = true;
}

void setUp(void) {
    process_initialized = false;
    api_initialized = false;
    api_port = reserve_local_port();

    char template[] = "/tmp/lightnvr-go2rtc-lifecycle-XXXXXX";
    char *dir = mkdtemp(template);
    TEST_ASSERT_NOT_NULL(dir);
    snprintf(config_dir, sizeof(config_dir), "%s", dir);
    snprintf(start_log, sizeof(start_log), "%s/starts.log", config_dir);

    char port[16];
    snprintf(port, sizeof(port), "%d", api_port);
    setenv("FAKE_GO2RTC_PORT", port, 1);
    setenv("FAKE_GO2RTC_START_LOG", start_log, 1);
    unsetenv("FAKE_GO2RTC_FAIL_START");
}

void tearDown(void) {
    if (process_initialized) {
        go2rtc_process_cleanup();
    }
    if (api_initialized) {
        go2rtc_api_cleanup();
    }
    free(g_config.streams);
    g_config.streams = NULL;
    unsetenv("FAKE_GO2RTC_FAIL_START");
    unsetenv("FAKE_GO2RTC_PORT");
    unsetenv("FAKE_GO2RTC_START_LOG");

    char path[160];
    snprintf(path, sizeof(path), "%s/go2rtc.yaml", config_dir);
    unlink(path);
    snprintf(path, sizeof(path), "%s/override.yaml", config_dir);
    unlink(path);
    snprintf(path, sizeof(path), "%s/go2rtc.log", config_dir);
    unlink(path);
    unlink(start_log);
    rmdir(config_dir);
}

static int count_starts(pid_t *last_pid) {
    FILE *fp = fopen(start_log, "r");
    if (!fp) return 0;
    int count = 0;
    int pid = -1;
    while (fscanf(fp, "%d", &pid) == 1) {
        count++;
        if (last_pid) *last_pid = (pid_t)pid;
    }
    fclose(fp);
    return count;
}

static void *start_worker(void *arg) {
    bool *result = arg;
    *result = go2rtc_process_start(api_port);
    return NULL;
}

void test_concurrent_starts_launch_one_verified_serving_child(void) {
    initialize_manager();

    enum { WORKERS = 6 };
    pthread_t threads[WORKERS];
    bool results[WORKERS] = {false};
    for (int i = 0; i < WORKERS; i++) {
        TEST_ASSERT_EQUAL_INT(0, pthread_create(&threads[i], NULL,
                                                start_worker, &results[i]));
    }
    for (int i = 0; i < WORKERS; i++) {
        pthread_join(threads[i], NULL);
        TEST_ASSERT_TRUE(results[i]);
    }

    pid_t serving_pid = (pid_t)go2rtc_process_get_pid();
    TEST_ASSERT_GREATER_THAN_INT(0, serving_pid);
    TEST_ASSERT_EQUAL_INT(1, count_starts(NULL));

    TEST_ASSERT_TRUE(go2rtc_process_stop());
    errno = 0;
    TEST_ASSERT_EQUAL_INT(-1, waitpid(serving_pid, NULL, WNOHANG));
    TEST_ASSERT_EQUAL_INT(ECHILD, errno);
    TEST_ASSERT_EQUAL_INT(-1, go2rtc_process_get_pid());
}

void test_failed_candidate_is_reaped_and_never_tracked(void) {
    initialize_manager();
    setenv("FAKE_GO2RTC_FAIL_START", "1", 1);

    TEST_ASSERT_FALSE(go2rtc_process_start(api_port));
    pid_t failed_pid = -1;
    TEST_ASSERT_EQUAL_INT(1, count_starts(&failed_pid));
    TEST_ASSERT_GREATER_THAN_INT(0, failed_pid);
    TEST_ASSERT_EQUAL_INT(-1, go2rtc_process_get_pid());

    errno = 0;
    TEST_ASSERT_EQUAL_INT(-1, waitpid(failed_pid, NULL, WNOHANG));
    TEST_ASSERT_EQUAL_INT(ECHILD, errno);
}

int main(void) {
    init_logger();
    UNITY_BEGIN();
    RUN_TEST(test_concurrent_starts_launch_one_verified_serving_child);
    RUN_TEST(test_failed_candidate_is_reaped_and_never_tracked);
    int result = UNITY_END();
    shutdown_logger();
    return result;
}
