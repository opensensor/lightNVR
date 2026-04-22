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

/* Fetch URL into @p out (caller frees out->data).  Returns HTTP status code
 * (e.g. 200, 404), or 0 on transport failure. */
static long http_get_status(const char *url, struct curl_buf *out)
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
    return (rc == CURLE_OK) ? status : 0;
}

/* 1 on HTTP 200, 0 otherwise.  Kept for back-compat with the streams test. */
static int http_get(const char *url, struct curl_buf *out)
{
    return http_get_status(url, out) == 200 ? 1 : 0;
}

/* Spawn go2rtc with two --config files, wait for the API at @p api_url to
 * respond, and return the child PID in @p out_pid.  The api_url is polled
 * for up to 8 s.  Returns 1 if the API came up, 0 otherwise.  Caller must
 * always reap the child via reap_child() even on a 0 return — the process
 * may still be running (just not serving on that URL yet). */
static int spawn_and_wait(const char *base_path, const char *override_path,
                          const char *api_url, pid_t *out_pid)
{
    pid_t pid = fork();
    if (pid < 0) return 0;
    if (pid == 0) {
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

    *out_pid = pid;

    int api_up = 0;
    struct curl_buf body = {0};
    for (int i = 0; i < 16; i++) {
        struct timespec ts = { .tv_sec = 0, .tv_nsec = 500 * 1000 * 1000 };
        nanosleep(&ts, NULL);
        free(body.data); body.data = NULL; body.len = 0;
        if (http_get(api_url, &body)) { api_up = 1; break; }
    }
    free(body.data);
    return api_up;
}

/* Reap the spawned go2rtc cleanly: SIGTERM, then SIGKILL after 1 s.  Safe
 * to call with pid <= 0 (no-op). */
static void reap_child(pid_t pid)
{
    if (pid <= 0) return;
    kill(pid, SIGTERM);
    int status;
    for (int i = 0; i < 10; i++) {
        struct timespec ts = { .tv_sec = 0, .tv_nsec = 100 * 1000 * 1000 };
        nanosleep(&ts, NULL);
        if (waitpid(pid, &status, WNOHANG) == pid) return;
    }
    kill(pid, SIGKILL);
    waitpid(pid, &status, 0);
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

    pid_t pid = -1;
    int api_up = spawn_and_wait(base_path, override_path, api_url, &pid);

    int saw_a = 0, saw_b = 0;
    if (api_up) {
        struct curl_buf body = {0};
        if (http_get(api_url, &body) && body.data) {
            saw_a = (strstr(body.data, "cam_a") != NULL) ? 1 : 0;
            saw_b = (strstr(body.data, "cam_b") != NULL) ? 1 : 0;
        }
        free(body.data);
    }
    reap_child(pid);

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

/* ------------------------------------------------------------------ *
 * Issue #394: the user's override YAML (ffmpeg.h264, log.level,
 * webrtc.candidates) appeared to have no effect.  The tests below
 * exercise the three YAML value shapes the user was touching, using
 * go2rtc's own observable endpoints:
 *
 *   - SCALAR override           via api.base_path
 *   - MAP-OF-SCALARS override   via log.level  (same yaml.v3 code path
 *                                               as ffmpeg's
 *                                               `Mod map[string]string`)
 *   - NESTED MAP KEY override   via streams.<name>
 *
 * If go2rtc silently dropped overrides on the second --config, one of
 * these three assertions would fail — and the issue would have a
 * reproducer.
 * ------------------------------------------------------------------ */

/* Scalar override: a string value in a struct field (api.base_path).
 * yaml.v3 semantics for a struct scalar across two Unmarshal calls is
 * "second wins".  We verify by moving the API mount point with the
 * override and asserting the base path no longer serves. */
void test_two_config_scalar_override(void)
{
    if (s_skip) {
        TEST_IGNORE_MESSAGE("go2rtc binary not found (set GO2RTC_TEST_BINARY)");
        return;
    }

    int port = find_free_port();
    TEST_ASSERT_GREATER_THAN_INT(0, port);

    char base_path[512], override_path[512];
    char base_url[256], override_url[256];
    snprintf(base_path, sizeof(base_path), "%s/base.yaml", s_tmpdir);
    snprintf(override_path, sizeof(override_path), "%s/override.yaml", s_tmpdir);
    /* go2rtc mounts api below the base_path; so with base_path=/override,
     * /override/api/streams is the live endpoint. */
    snprintf(base_url,     sizeof(base_url),
             "http://127.0.0.1:%d/base/api/streams", port);
    snprintf(override_url, sizeof(override_url),
             "http://127.0.0.1:%d/override/api/streams", port);

    char base_body[512];
    snprintf(base_body, sizeof(base_body),
             "api:\n"
             "  listen: \"127.0.0.1:%d\"\n"
             "  base_path: /base\n"
             "rtsp:\n"
             "  listen: \"\"\n"
             "webrtc:\n"
             "  listen: \"\"\n"
             "streams:\n"
             "  cam_x: rtsp://example.invalid/x\n",
             port);
    write_file(base_path, base_body);

    write_file(override_path,
               "api:\n"
               "  base_path: /override\n");

    pid_t pid = -1;
    /* Wait using the override URL so we block until the new mount point
     * actually serves.  If the override were ignored, /override/api would
     * 404 indefinitely and the test would detect it via api_up=false. */
    int api_up = spawn_and_wait(base_path, override_path, override_url, &pid);

    long base_status = 0, override_status = 0;
    if (api_up) {
        struct curl_buf b1 = {0}, b2 = {0};
        override_status = http_get_status(override_url, &b1);
        base_status     = http_get_status(base_url, &b2);
        free(b1.data);
        free(b2.data);
    }
    reap_child(pid);

    TEST_ASSERT_TRUE_MESSAGE(api_up,
        "/override/api/streams never came up — override scalar (api.base_path) "
        "was likely ignored and go2rtc stayed mounted at /base");
    TEST_ASSERT_EQUAL_INT_MESSAGE(200, (int)override_status,
        "override base_path (/override) did not serve the API");
    TEST_ASSERT_NOT_EQUAL_MESSAGE(200, (int)base_status,
        "base base_path (/base) still served the API — override did NOT "
        "replace the scalar");
}

/* Map-of-scalars override: the exact code path used by the ffmpeg module
 * (`Mod map[string]string yaml:"ffmpeg"`).  The log module uses the same
 * shape (`Mod map[string]string yaml:"log"`) and is observable via the
 * /api/log endpoint — go2rtc's MemoryLog stores the raw zerolog JSON
 * events regardless of the console `format:` setting, so we grep for
 * the `"level":"trace"` marker.
 *
 * base sets log.level=error+format=text; override bumps level to trace.
 * If the merge works, /api/log contains `"level":"trace"` entries; if
 * the override were dropped, we'd see only level=error or above. */
void test_two_config_log_map_override(void)
{
    if (s_skip) {
        TEST_IGNORE_MESSAGE("go2rtc binary not found (set GO2RTC_TEST_BINARY)");
        return;
    }

    int port = find_free_port();
    TEST_ASSERT_GREATER_THAN_INT(0, port);

    char base_path[512], override_path[512];
    char api_url[256], log_url[256];
    snprintf(base_path,     sizeof(base_path),     "%s/base.yaml",     s_tmpdir);
    snprintf(override_path, sizeof(override_path), "%s/override.yaml", s_tmpdir);
    snprintf(api_url, sizeof(api_url),
             "http://127.0.0.1:%d/api/streams", port);
    snprintf(log_url, sizeof(log_url),
             "http://127.0.0.1:%d/api/log", port);

    char base_body[512];
    snprintf(base_body, sizeof(base_body),
             "api:\n"
             "  listen: \"127.0.0.1:%d\"\n"
             "rtsp:\n"
             "  listen: \"\"\n"
             "webrtc:\n"
             "  listen: \"\"\n"
             "log:\n"
             "  level: error\n"
             "  format: text\n"
             "streams:\n"
             "  cam_x: rtsp://example.invalid/x\n",
             port);
    write_file(base_path, base_body);

    write_file(override_path,
               "log:\n"
               "  level: trace\n");

    pid_t pid = -1;
    int api_up = spawn_and_wait(base_path, override_path, api_url, &pid);

    int saw_trace = 0;
    if (api_up) {
        /* Give zerolog a moment to flush startup traces into the memory
         * buffer before we read it. */
        struct timespec ts = { .tv_sec = 0, .tv_nsec = 300 * 1000 * 1000 };
        nanosleep(&ts, NULL);

        struct curl_buf body = {0};
        if (http_get(log_url, &body) && body.data) {
            /* MemoryLog gets raw zerolog JSON events — look for the
             * trace level tag in that form. */
            saw_trace = (strstr(body.data, "\"level\":\"trace\"") != NULL)
                        ? 1 : 0;
        }
        free(body.data);
    }
    reap_child(pid);

    TEST_ASSERT_TRUE_MESSAGE(api_up,
        "go2rtc API never came up for log-map override test");
    TEST_ASSERT_TRUE_MESSAGE(saw_trace,
        "log.level=trace from override.yaml did NOT take effect — "
        "/api/log contained no trace-level entries. The same yaml.v3 "
        "merge path is used by ffmpeg.h264/h265 overrides (#394).");
}

/* Nested map-key override: `streams` is `map[string]any`.  When both
 * configs define cam_keep and only override redefines cam_change, we
 * expect:
 *   - cam_keep → base source survives (map merge, not replace)
 *   - cam_change → override wins (second Unmarshal sets the key)
 *   - cam_new → override-only key appears
 * This is the shape of the user's complaint: per-key overrides inside
 * a top-level map should take effect without wiping siblings. */
void test_two_config_nested_key_override(void)
{
    if (s_skip) {
        TEST_IGNORE_MESSAGE("go2rtc binary not found (set GO2RTC_TEST_BINARY)");
        return;
    }

    int port = find_free_port();
    TEST_ASSERT_GREATER_THAN_INT(0, port);

    char base_path[512], override_path[512];
    char streams_url[256], stream_change_url[256];
    snprintf(base_path,     sizeof(base_path),     "%s/base.yaml",     s_tmpdir);
    snprintf(override_path, sizeof(override_path), "%s/override.yaml", s_tmpdir);
    snprintf(streams_url, sizeof(streams_url),
             "http://127.0.0.1:%d/api/streams", port);
    snprintf(stream_change_url, sizeof(stream_change_url),
             "http://127.0.0.1:%d/api/streams?src=cam_change", port);

    char base_body[512];
    snprintf(base_body, sizeof(base_body),
             "api:\n"
             "  listen: \"127.0.0.1:%d\"\n"
             "rtsp:\n"
             "  listen: \"\"\n"
             "webrtc:\n"
             "  listen: \"\"\n"
             "streams:\n"
             "  cam_keep: rtsp://example.invalid/KEEP\n"
             "  cam_change: rtsp://example.invalid/OLD\n",
             port);
    write_file(base_path, base_body);

    write_file(override_path,
               "streams:\n"
               "  cam_change: rtsp://example.invalid/NEW\n"
               "  cam_new: rtsp://example.invalid/NEW2\n");

    pid_t pid = -1;
    int api_up = spawn_and_wait(base_path, override_path, streams_url, &pid);

    int saw_keep = 0, saw_change = 0, saw_new = 0;
    int saw_new_source = 0, saw_old_source = 0;
    if (api_up) {
        struct curl_buf list = {0};
        if (http_get(streams_url, &list) && list.data) {
            saw_keep   = (strstr(list.data, "cam_keep")   != NULL) ? 1 : 0;
            saw_change = (strstr(list.data, "cam_change") != NULL) ? 1 : 0;
            saw_new    = (strstr(list.data, "cam_new")    != NULL) ? 1 : 0;
        }
        free(list.data);

        /* Query the individual stream to see its source string.
         * /api/streams?src=<name> returns the producer's info; the
         * configured URL (/NEW vs /OLD) appears in the "source" field. */
        struct curl_buf one = {0};
        if (http_get(stream_change_url, &one) && one.data) {
            saw_new_source = (strstr(one.data, "/NEW") != NULL) ? 1 : 0;
            saw_old_source = (strstr(one.data, "/OLD") != NULL) ? 1 : 0;
        }
        free(one.data);
    }
    reap_child(pid);

    TEST_ASSERT_TRUE_MESSAGE(api_up,
        "go2rtc API never came up for nested-key override test");
    TEST_ASSERT_TRUE_MESSAGE(saw_keep,
        "cam_keep (base only) missing — override WIPED the streams map "
        "instead of merging. This would break real deployments.");
    TEST_ASSERT_TRUE_MESSAGE(saw_change,
        "cam_change missing from listing after merge");
    TEST_ASSERT_TRUE_MESSAGE(saw_new,
        "cam_new (override only) missing — override key not added");
    TEST_ASSERT_TRUE_MESSAGE(saw_new_source,
        "cam_change source did NOT switch to /NEW — the nested-key "
        "override value was dropped (would explain #394 symptoms where "
        "ffmpeg.h264 override stays on the base value)");
    TEST_ASSERT_FALSE_MESSAGE(saw_old_source,
        "cam_change still reports the /OLD base source — the override "
        "key did NOT replace the base key's value");
}

int main(void)
{
    UNITY_BEGIN();
    RUN_TEST(test_two_config_streams_merge);
    RUN_TEST(test_two_config_scalar_override);
    RUN_TEST(test_two_config_log_map_override);
    RUN_TEST(test_two_config_nested_key_override);
    return UNITY_END();
}
