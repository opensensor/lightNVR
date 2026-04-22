/**
 * @file test_go2rtc_two_config_merge.c
 * @brief T12 — runtime smoke test: does go2rtc actually merge two --config
 *        files for the streams: section, or does the second one replace?
 *
 * Risk R4 in go2rtc-override-plan.md flagged that gopkg.in/yaml.v3 (what
 * go2rtc uses internally) may treat container values as REPLACE rather than
 * deep-merge across repeated yaml.Unmarshal calls.  This test answers that
 * question with the actual binary so the docs and UI warnings can be
 * calibrated.
 *
 * Strategy:
 *   1. Generate a free TCP port.
 *   2. Write base.yaml   with { api: { listen: :PORT }, streams: { cam_a: ... } }
 *   3. Write override.yaml with                            { streams: { cam_b: ... } }
 *   4. Spawn go2rtc with `--config base.yaml --config override.yaml`.
 *   5. Poll /api/streams until 200 OK or 8 s timeout.
 *   6. Inspect the JSON for both cam_a and cam_b.
 *
 * If the binary is unavailable (no GO2RTC_TEST_BINARY env var, no go2rtc
 * in PATH), the test SKIPS with TEST_IGNORE so CI without go2rtc still
 * passes.  T15 wires the binary into CI via the env var.
 */

#define _POSIX_C_SOURCE 200809L

#include <arpa/inet.h>
#include <errno.h>
#include <fcntl.h>
#include <netinet/in.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>

#include <curl/curl.h>

#include "unity.h"

static char s_tmpdir[256];
static char s_binary_path[512];
static int  s_skip = 0;

static int find_free_port(void)
{
    int s = socket(AF_INET, SOCK_STREAM, 0);
    if (s < 0) return -1;
    struct sockaddr_in addr = { .sin_family = AF_INET,
                                .sin_addr.s_addr = htonl(INADDR_LOOPBACK),
                                .sin_port = 0 };
    if (bind(s, (struct sockaddr *)&addr, sizeof(addr)) != 0) {
        close(s); return -1;
    }
    socklen_t len = sizeof(addr);
    if (getsockname(s, (struct sockaddr *)&addr, &len) != 0) {
        close(s); return -1;
    }
    int port = ntohs(addr.sin_port);
    close(s);
    return port;
}

static void resolve_binary_path(void)
{
    s_binary_path[0] = '\0';
    const char *env = getenv("GO2RTC_TEST_BINARY");
    if (env && env[0]) {
        if (access(env, X_OK) == 0) {
            snprintf(s_binary_path, sizeof(s_binary_path), "%s", env);
            return;
        }
    }
    const char *candidates[] = {
        "/usr/local/bin/go2rtc", "/usr/bin/go2rtc", "/bin/go2rtc",
        NULL,
    };
    for (int i = 0; candidates[i]; i++) {
        if (access(candidates[i], X_OK) == 0) {
            snprintf(s_binary_path, sizeof(s_binary_path), "%s",
                     candidates[i]);
            return;
        }
    }
}

void setUp(void)
{
    s_skip = 0;
    s_tmpdir[0] = '\0';

    resolve_binary_path();
    if (s_binary_path[0] == '\0') {
        s_skip = 1;
        return;
    }

    char tmpl[] = "/tmp/lightnvr-go2rtc-merge-XXXXXX";
    char *created = mkdtemp(tmpl);
    if (!created) {
        s_skip = 1;
        return;
    }
    snprintf(s_tmpdir, sizeof(s_tmpdir), "%s", created);
}

void tearDown(void)
{
    if (s_tmpdir[0] != '\0') {
        char p[512];
        const char *files[] = { "base.yaml", "override.yaml", "go2rtc.log",
                                 NULL };
        for (int i = 0; files[i]; i++) {
            snprintf(p, sizeof(p), "%s/%s", s_tmpdir, files[i]);
            unlink(p);
        }
        rmdir(s_tmpdir);
        s_tmpdir[0] = '\0';
    }
}

static void write_file(const char *path, const char *body)
{
    FILE *fp = fopen(path, "w");
    TEST_ASSERT_NOT_NULL(fp);
    fputs(body, fp);
    fclose(fp);
}

struct curl_buf {
    char  *data;
    size_t len;
};

static size_t curl_collect(void *ptr, size_t size, size_t nmemb, void *userdata)
{
    struct curl_buf *b = (struct curl_buf *)userdata;
    size_t add = size * nmemb;
    char *p = realloc(b->data, b->len + add + 1);
    if (!p) return 0;
    b->data = p;
    memcpy(b->data + b->len, ptr, add);
    b->len += add;
    b->data[b->len] = '\0';
    return add;
}

/* Fetch URL into @p out (caller frees out->data).  Returns 1 on HTTP 200,
 * 0 otherwise. */
static int http_get(const char *url, struct curl_buf *out)
{
    CURL *c = curl_easy_init();
    if (!c) return 0;
    out->data = NULL; out->len = 0;
    curl_easy_setopt(c, CURLOPT_URL, url);
    curl_easy_setopt(c, CURLOPT_WRITEFUNCTION, curl_collect);
    curl_easy_setopt(c, CURLOPT_WRITEDATA, out);
    curl_easy_setopt(c, CURLOPT_TIMEOUT, 2L);
    curl_easy_setopt(c, CURLOPT_NOSIGNAL, 1L);
    CURLcode rc = curl_easy_perform(c);
    long status = 0;
    if (rc == CURLE_OK) {
        curl_easy_getinfo(c, CURLINFO_RESPONSE_CODE, &status);
    }
    curl_easy_cleanup(c);
    return (rc == CURLE_OK && status == 200) ? 1 : 0;
}

void test_two_config_streams_merge(void)
{
    if (s_skip) {
        TEST_IGNORE_MESSAGE("go2rtc binary not found (set GO2RTC_TEST_BINARY)");
        return;
    }

    int port = find_free_port();
    TEST_ASSERT_GREATER_THAN_INT(0, port);

    char base_path[512], override_path[512], api_url[256];
    snprintf(base_path, sizeof(base_path), "%s/base.yaml", s_tmpdir);
    snprintf(override_path, sizeof(override_path), "%s/override.yaml", s_tmpdir);
    snprintf(api_url, sizeof(api_url),
             "http://127.0.0.1:%d/api/streams", port);

    char base_body[512];
    snprintf(base_body, sizeof(base_body),
             "api:\n"
             "  listen: \"127.0.0.1:%d\"\n"
             "rtsp:\n"
             "  listen: \"\"\n"
             "webrtc:\n"
             "  listen: \"\"\n"
             "streams:\n"
             "  cam_a: rtsp://example.invalid/a\n",
             port);
    write_file(base_path, base_body);

    write_file(override_path,
               "log:\n"
               "  level: trace\n"
               "streams:\n"
               "  cam_b: rtsp://example.invalid/b\n");

    pid_t pid = fork();
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, pid);
    if (pid == 0) {
        /* child: silence stdio so the test runner output stays clean. */
        int devnull = open("/dev/null", O_RDWR);
        if (devnull >= 0) {
            dup2(devnull, STDIN_FILENO);
            dup2(devnull, STDOUT_FILENO);
            dup2(devnull, STDERR_FILENO);
            close(devnull);
        }
        execl(s_binary_path, "go2rtc",
              "--config", base_path,
              "--config", override_path,
              (char *)NULL);
        _exit(127);
    }

    /* Parent: poll up to 8 s for the API to come up. */
    int api_up = 0;
    struct curl_buf body = {0};
    for (int i = 0; i < 16; i++) {
        struct timespec ts = { .tv_sec = 0, .tv_nsec = 500 * 1000 * 1000 };
        nanosleep(&ts, NULL);
        free(body.data); body.data = NULL; body.len = 0;
        if (http_get(api_url, &body)) { api_up = 1; break; }
    }

    /* Whatever happens next, kill the child first so we don't leak it. */
    int saw_a = 0, saw_b = 0;
    if (api_up && body.data) {
        saw_a = (strstr(body.data, "cam_a") != NULL) ? 1 : 0;
        saw_b = (strstr(body.data, "cam_b") != NULL) ? 1 : 0;
    }
    free(body.data);

    kill(pid, SIGTERM);
    int status;
    /* Give it 1 s to exit gracefully, then SIGKILL. */
    for (int i = 0; i < 10; i++) {
        struct timespec ts = { .tv_sec = 0, .tv_nsec = 100 * 1000 * 1000 };
        nanosleep(&ts, NULL);
        if (waitpid(pid, &status, WNOHANG) == pid) { pid = -1; break; }
    }
    if (pid > 0) {
        kill(pid, SIGKILL);
        waitpid(pid, &status, 0);
    }

    TEST_ASSERT_TRUE_MESSAGE(api_up,
        "go2rtc API never came up — check binary version compatibility");
    /* The actual answer to risk R4: do BOTH streams survive the merge? */
    TEST_ASSERT_TRUE_MESSAGE(saw_a,
        "cam_a (from base.yaml streams:) missing — second --config "
        "REPLACED the streams map instead of merging it. Doc this in T13.");
    TEST_ASSERT_TRUE_MESSAGE(saw_b,
        "cam_b (from override.yaml streams:) missing — override didn't "
        "take effect at all");
}

int main(void)
{
    UNITY_BEGIN();
    RUN_TEST(test_two_config_streams_merge);
    return UNITY_END();
}
