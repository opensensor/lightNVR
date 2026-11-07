/* Owned-process regression: exercise the actual managed launch and go2rtc's
 * config parser, not a second implementation of its merge semantics. */
#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#include <arpa/inet.h>
#include <fcntl.h>
#include <ftw.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/time.h>
#include <sys/wait.h>
#include <unistd.h>
#include <curl/curl.h>
#include <cjson/cJSON.h>
#include "unity.h"
#include "core/config.h"
#include "database/db_core.h"
#include "database/db_system_settings.h"
#include "video/go2rtc/go2rtc_api.h"
#include "video/go2rtc/go2rtc_process.h"
#include "web/api_handlers_system.h"
#include "utils/yaml_validate.h"

static char directory[128], database[160], unix_path[160];
static const char *binary;
static int ports[5];
static bool database_ready, manager_ready, api_ready;

static int remove_fixture(const char *path, const struct stat *st, int kind, struct FTW *walk) {
    (void)st; (void)kind; (void)walk;
    return remove(path);
}

void setUp(void) {
    binary = getenv("GO2RTC_TEST_BINARY");
    if (!binary || access(binary, X_OK))
        TEST_IGNORE_MESSAGE("Set GO2RTC_TEST_BINARY to the pinned fork binary");
    strcpy(directory, "/tmp/lightnvr-managed-listeners-XXXXXX");
    TEST_ASSERT_NOT_NULL(mkdtemp(directory));
    snprintf(database, sizeof(database), "%s/test.db", directory);
    snprintf(unix_path, sizeof(unix_path), "%s/api.sock", directory);
    TEST_ASSERT_EQUAL_INT(0, init_database(database));
    database_ready = true;
    load_default_config(&g_config);
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
    g_config.go2rtc_enabled = true;
    g_config.go2rtc_webrtc_enabled = true;
    g_config.go2rtc_stun_enabled = false;
    snprintf(g_config.log_file, sizeof(g_config.log_file), "%s/lightnvr.log", directory);

    // Hold all sockets while reserving, so the ports are distinct.
    int sockets[5];
    for (int i = 0; i < 5; i++) {
        sockets[i] = socket(AF_INET, SOCK_STREAM, 0);
        TEST_ASSERT_GREATER_OR_EQUAL_INT(0, sockets[i]);
        struct sockaddr_in addr = {.sin_family = AF_INET,
            .sin_addr.s_addr = htonl(INADDR_LOOPBACK)};
        TEST_ASSERT_EQUAL_INT(0, bind(sockets[i], (struct sockaddr *)&addr, sizeof(addr)));
        socklen_t size = sizeof(addr);
        TEST_ASSERT_EQUAL_INT(0, getsockname(sockets[i], (struct sockaddr *)&addr, &size));
        ports[i] = ntohs(addr.sin_port);
    }
    for (int i = 0; i < 5; i++) close(sockets[i]);
    g_config.go2rtc_api_port = ports[0];
    g_config.go2rtc_rtsp_port = ports[1];
    g_config.go2rtc_webrtc_listen_port = ports[4];
    curl_global_init(CURL_GLOBAL_ALL);
}

void tearDown(void) {
    if (manager_ready) go2rtc_process_cleanup();
    if (api_ready) go2rtc_api_cleanup();
    if (database_ready) shutdown_database();
    manager_ready = api_ready = database_ready = false;
    free(g_config.streams);
    g_config.streams = NULL;
    if (directory[0]) {
        nftw(directory, remove_fixture, 16, FTW_DEPTH | FTW_PHYS);
        directory[0] = '\0';
    }
    curl_global_cleanup();
}

static int connect_to(const char *host, int port) {
    int family = strchr(host, ':') ? AF_INET6 : AF_INET;
    int fd = socket(family, SOCK_STREAM, 0);
    if (fd < 0) return -1;
    struct timeval timeout = {.tv_sec = 2};
    setsockopt(fd, SOL_SOCKET, SO_SNDTIMEO, &timeout, sizeof(timeout));
    setsockopt(fd, SOL_SOCKET, SO_RCVTIMEO, &timeout, sizeof(timeout));
    int rc;
    if (family == AF_INET) {
        struct sockaddr_in addr = {.sin_family = AF_INET, .sin_port = htons(port)};
        inet_pton(AF_INET, host, &addr.sin_addr);
        rc = connect(fd, (struct sockaddr *)&addr, sizeof(addr));
    } else {
        struct sockaddr_in6 addr = {.sin6_family = AF_INET6, .sin6_port = htons(port)};
        inet_pton(AF_INET6, host, &addr.sin6_addr);
        rc = connect(fd, (struct sockaddr *)&addr, sizeof(addr));
    }
    if (rc != 0) { close(fd); return -1; }
    return fd;
}

static void assert_refused(const char *host, int port) {
    int fd = connect_to(host, port);
    if (fd >= 0) close(fd);
    TEST_ASSERT_EQUAL_INT_MESSAGE(-1, fd, "Unexpected unmanaged listener reachable");
}

static void make_certificate(void) {
    char cert[192], key[192];
    snprintf(cert, sizeof(cert), "%s/cert.pem", directory);
    snprintf(key, sizeof(key), "%s/key.pem", directory);
    pid_t pid = fork();
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, pid);
    if (pid == 0) {
        alarm(15);
        int quiet = open("/dev/null", O_WRONLY);
        if (quiet >= 0) { dup2(quiet, STDOUT_FILENO); dup2(quiet, STDERR_FILENO); close(quiet); }
        execlp("openssl", "openssl", "req", "-x509", "-newkey", "rsa:2048", "-nodes",
            "-keyout", key, "-out", cert, "-days", "1", "-subj", "/CN=localhost", (char *)NULL);
        _exit(127);
    }
    int status;
    TEST_ASSERT_EQUAL_INT(pid, waitpid(pid, &status, 0));
    TEST_ASSERT_TRUE_MESSAGE(WIFEXITED(status) && WEXITSTATUS(status) == 0,
                             "openssl is required to exercise a valid alternate HTTPS listener");
}

static void assert_local_video_decodes(void) {
    char url[128], output[192];
    snprintf(url, sizeof(url), "rtsp://127.0.0.1:%d/fixture", ports[1]);
    snprintf(output, sizeof(output), "%s/frames.rgb", directory);
    pid_t pid = fork();
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, pid);
    if (pid == 0) {
        alarm(20);
        execlp("ffmpeg", "ffmpeg", "-nostdin", "-v", "error", "-xerror", "-y",
            "-rtsp_transport", "tcp", "-i", url, "-frames:v", "5", "-an",
            "-threads", "1", "-pix_fmt", "rgb24", "-f", "rawvideo", output, (char *)NULL);
        _exit(127);
    }
    int status;
    TEST_ASSERT_EQUAL_INT(pid, waitpid(pid, &status, 0));
    TEST_ASSERT_TRUE_MESSAGE(WIFEXITED(status) && WEXITSTATUS(status) == 0,
                             "Local RTSP video must decode with FFmpeg");
    struct stat frames;
    TEST_ASSERT_EQUAL_INT(0, stat(output, &frames));
    TEST_ASSERT_EQUAL_INT(160 * 90 * 3 * 5, frames.st_size);
}

static void assert_runtime_boundary(void) {
    int rtsp = connect_to("127.0.0.1", ports[1]);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, rtsp);
    const char options[] = "OPTIONS rtsp://127.0.0.1/fixture RTSP/1.0\r\nCSeq: 1\r\n\r\n";
    ssize_t sent = send(rtsp, options, sizeof(options) - 1, MSG_NOSIGNAL);
    char response[1024] = {0};
    ssize_t received = recv(rtsp, response, sizeof(response) - 1, 0);
    close(rtsp);
    TEST_ASSERT_EQUAL_INT(sizeof(options) - 1, sent);
    TEST_ASSERT_GREATER_THAN_INT(0, received);
    TEST_ASSERT_NOT_NULL(strstr(response, "RTSP/1.0 200"));
    // 127.0.0.2 distinguishes a wildcard IPv4 bind from the required exact bind.
    for (int i = 0; i < 2; i++) {
        assert_refused("127.0.0.2", ports[i]);
        assert_refused("::1", ports[i]);
    }
    for (int i = 2; i < 4; i++) {
        assert_refused("127.0.0.1", ports[i]);
        assert_refused("::1", ports[i]);
    }
    TEST_ASSERT_NOT_EQUAL(0, access(unix_path, F_OK));
    // ICE transport is intentionally still available beyond the exact loopback bind.
    int ice = connect_to("127.0.0.2", ports[4]);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, ice);
    close(ice);

    http_request_t request;
    http_response_t result;
    http_request_init(&request);
    http_response_init(&result);
    handle_get_system_go2rtc_effective_config(&request, &result);
    cJSON *json = cJSON_Parse(result.body);
    TEST_ASSERT_NOT_NULL(json);
    const cJSON *policy = cJSON_GetObjectItemCaseSensitive(json, "managed_listener_policy");
    TEST_ASSERT_TRUE(cJSON_IsString(policy));
    TEST_ASSERT_NOT_NULL(strstr(policy->valuestring, "127.0.0.1"));
    const cJSON *order = cJSON_GetObjectItemCaseSensitive(json, "merged_source_order");
    TEST_ASSERT_EQUAL_STRING("managed_listener_policy",
        cJSON_GetArrayItem(order, cJSON_GetArraySize(order) - 1)->valuestring);
    cJSON_Delete(json);
    http_response_free(&result);
}

static void assert_no_policy_preview(void) {
    http_request_t request;
    http_response_t result;
    http_request_init(&request);
    http_response_init(&result);
    handle_get_system_go2rtc_effective_config(&request, &result);
    cJSON *json = cJSON_Parse(result.body);
    TEST_ASSERT_NOT_NULL(json);
    const cJSON *policy = cJSON_GetObjectItemCaseSensitive(json, "managed_listener_policy");
    TEST_ASSERT_TRUE(cJSON_IsString(policy));
    TEST_ASSERT_EQUAL_STRING("", policy->valuestring);
    const cJSON *entry;
    cJSON_ArrayForEach(entry, cJSON_GetObjectItemCaseSensitive(json, "merged_source_order")) {
        TEST_ASSERT_NOT_EQUAL(0, strcmp("managed_listener_policy", entry->valuestring));
    }
    cJSON_Delete(json);
    http_response_free(&result);
}

void test_managed_launch_and_restart_pin_listeners_after_overrides(void) {
    make_certificate();
    TEST_ASSERT_TRUE(go2rtc_process_init(binary, directory, ports[0]));
    manager_ready = true;
    TEST_ASSERT_TRUE(go2rtc_api_init("127.0.0.1", ports[0]));
    api_ready = true;
    char policy[512];
    TEST_ASSERT_FALSE(go2rtc_process_get_listener_policy(policy, sizeof(policy)));
    assert_no_policy_preview();
    for (int variant = 0; variant < 3; variant++) {
        char override[2048] = {0};
        if (variant) {
            // Includes a valid certificate: TLS must be disabled by policy,
            // not merely fail to start because a fixture certificate is missing.
            snprintf(override, sizeof(override),
                "api:\n  listen: '%s:%d'\n  tls_listen: ':%d'\n"
                "  tls_cert: '%s/cert.pem'\n  tls_key: '%s/key.pem'\n"
                "  unix_listen: '%s'\n  base_path: %s\n"
                "rtsp:\n  listen: '%s:%d'\n"
                "streams:\n  fixture: 'ffmpeg:virtual?video=testsrc2&size=160x90&rate=5#video=h264'\n",
                variant == 1 ? "0.0.0.0" : "[::]", ports[2], ports[3],
                directory, directory, unix_path,
                yaml_validate_is_available() ? "/private/media" : GO2RTC_BASE_PATH,
                variant == 1 ? "0.0.0.0" : "[::]", ports[2]);
        }
        TEST_ASSERT_EQUAL_INT(0, db_set_system_setting("go2rtc_config_override", override));
        TEST_ASSERT_TRUE(go2rtc_process_start(ports[0]));
        int actual_rtsp_port = 0;
        TEST_ASSERT_TRUE(go2rtc_api_get_server_info(&actual_rtsp_port));
        TEST_ASSERT_EQUAL_INT(ports[1], actual_rtsp_port);
        if (variant) TEST_ASSERT_TRUE(go2rtc_api_stream_exists("fixture"));
        assert_runtime_boundary();
        if (variant) assert_local_video_decodes();
        TEST_ASSERT_TRUE(go2rtc_process_stop());
        TEST_ASSERT_FALSE(go2rtc_process_get_listener_policy(policy, sizeof(policy)));
        assert_no_policy_preview();
    }
    g_config.go2rtc_rtsp_port = 65536;
    TEST_ASSERT_FALSE(go2rtc_process_start(ports[0]));
    TEST_ASSERT_EQUAL_INT(-1, go2rtc_process_get_pid());
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_managed_launch_and_restart_pin_listeners_after_overrides);
    return UNITY_END();
}
