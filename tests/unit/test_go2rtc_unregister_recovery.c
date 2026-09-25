/**
 * @file test_go2rtc_unregister_recovery.c
 * @brief Regression tests for the go2rtc stream unregister/recovery path (#620).
 *
 * The go2rtc API boundary is captured with --wrap so the ordering contract is
 * asserted without a real go2rtc: a preload consumer must be detached before
 * the stream is deleted (otherwise go2rtc keeps the replaced Stream object,
 * its producer reconnect loop and its RTSP session to the camera alive until
 * go2rtc restarts), and the reachability probe used before escalating to a
 * process restart must reflect real TCP state.
 */

#define _POSIX_C_SOURCE 200809L

#include <netinet/in.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

#include "unity.h"
#include "core/config.h"
#include "video/go2rtc/go2rtc_api.h"
#include "video/go2rtc/go2rtc_stream.h"

static char config_dir[128];
static int fake_port;
static int call_sequence;
static int delete_preload_order;
static int remove_stream_order;
static char delete_preload_name[MAX_STREAM_NAME + 8];
static char remove_stream_name[MAX_STREAM_NAME + 8];
static bool remove_succeeds;

bool __wrap_go2rtc_api_delete_preload(const char *stream_id) {
    delete_preload_order = ++call_sequence;
    snprintf(delete_preload_name, sizeof(delete_preload_name), "%s", stream_id);
    return true;
}

bool __wrap_go2rtc_api_remove_stream(const char *stream_id) {
    remove_stream_order = ++call_sequence;
    snprintf(remove_stream_name, sizeof(remove_stream_name), "%s", stream_id);
    return remove_succeeds;
}

static int reserve_loopback_port(void) {
    int fd = socket(AF_INET, SOCK_STREAM, 0);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, fd);
    struct sockaddr_in address = {0};
    address.sin_family = AF_INET;
    address.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    TEST_ASSERT_EQUAL_INT(0, bind(fd, (struct sockaddr *)&address, sizeof(address)));
    socklen_t size = sizeof(address);
    TEST_ASSERT_EQUAL_INT(0, getsockname(fd, (struct sockaddr *)&address, &size));
    int port = ntohs(address.sin_port);
    close(fd);
    return port;
}

void setUp(void) {
    call_sequence = 0;
    delete_preload_order = 0;
    remove_stream_order = 0;
    delete_preload_name[0] = '\0';
    remove_stream_name[0] = '\0';
    remove_succeeds = true;

    load_default_config(&g_config);
    g_config.go2rtc_enabled = true;
    g_config.go2rtc_webrtc_enabled = false;

    fake_port = reserve_loopback_port();

    char template[] = "/tmp/lightnvr-unregister-XXXXXX";
    char *dir = mkdtemp(template);
    TEST_ASSERT_NOT_NULL(dir);
    snprintf(config_dir, sizeof(config_dir), "%s", dir);
    snprintf(g_config.log_file, sizeof(g_config.log_file), "%s/lightnvr.log", dir);
    g_config.go2rtc_api_port = fake_port;

    char port_text[16];
    snprintf(port_text, sizeof(port_text), "%d", fake_port);
    setenv("FAKE_GO2RTC_PORT", port_text, 1);
    TEST_ASSERT_TRUE(go2rtc_stream_init(FAKE_GO2RTC_PATH, dir, fake_port));
    TEST_ASSERT_TRUE(go2rtc_stream_start_service());
}

void tearDown(void) {
    go2rtc_stream_cleanup();
    free(g_config.streams);
    g_config.streams = NULL;
    unsetenv("FAKE_GO2RTC_PORT");
    const char *files[] = {"go2rtc.yaml", "override.yaml", "go2rtc.log", "lightnvr.log"};
    for (size_t i = 0; i < sizeof(files) / sizeof(files[0]); i++) {
        char path[192];
        snprintf(path, sizeof(path), "%s/%s", config_dir, files[i]);
        unlink(path);
    }
    rmdir(config_dir);
}

void test_unregister_detaches_preload_before_deleting_stream(void) {
    TEST_ASSERT_TRUE(go2rtc_stream_unregister("Kamar"));

    TEST_ASSERT_EQUAL_STRING("Kamar", delete_preload_name);
    TEST_ASSERT_EQUAL_STRING("Kamar", remove_stream_name);
    TEST_ASSERT_EQUAL_INT(1, delete_preload_order);
    TEST_ASSERT_EQUAL_INT(2, remove_stream_order);
}

void test_unregister_reports_delete_failure_but_still_detaches_preload(void) {
    remove_succeeds = false;
    TEST_ASSERT_FALSE(go2rtc_stream_unregister("Kamar"));

    TEST_ASSERT_EQUAL_INT(1, delete_preload_order);
    TEST_ASSERT_EQUAL_INT(2, remove_stream_order);
}

void test_tcp_probe_reflects_listening_and_closed_ports(void) {
    /* The fake go2rtc listens on fake_port; a freshly reserved-and-released
     * port has nothing behind it. */
    TEST_ASSERT_TRUE(go2rtc_stream_tcp_port_open("127.0.0.1", fake_port, 1000));
    int closed_port = reserve_loopback_port();
    TEST_ASSERT_FALSE(go2rtc_stream_tcp_port_open("127.0.0.1", closed_port, 1000));
    TEST_ASSERT_FALSE(go2rtc_stream_tcp_port_open(NULL, fake_port, 1000));
    TEST_ASSERT_FALSE(go2rtc_stream_tcp_port_open("127.0.0.1", 0, 1000));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_unregister_detaches_preload_before_deleting_stream);
    RUN_TEST(test_unregister_reports_delete_failure_but_still_detaches_preload);
    RUN_TEST(test_tcp_probe_reflects_listening_and_closed_ports);
    return UNITY_END();
}
