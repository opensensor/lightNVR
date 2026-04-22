/**
 * @file test_go2rtc_binary_detection.c
 * @brief Unit tests for T8 — go2rtc binary detection hardening.
 *
 * Exercises go2rtc_process_probe_version() directly by planting dummy shell
 * scripts in a temp directory:
 *   - good  : prints "go2rtc version 1.2.3 ..." and exits 0
 *   - wrong : prints some other banner and exits 0
 *   - nonzero: prints "go2rtc version ..." but exits 1
 *   - hang  : sleeps 10 seconds, must be killed by the 2s timeout
 *   - missing: path that doesn't exist at all
 *
 * All planted scripts are cleaned up in tearDown() — no scripts survive
 * across tests, and the probe helper is required to reap its own child so
 * no zombies outlive these tests.
 */

#define _POSIX_C_SOURCE 200809L

#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <time.h>
#include <unistd.h>

#include "unity.h"
#include "video/go2rtc/go2rtc_process.h"

static char s_tmpdir[PATH_MAX];

static void remove_tmpdir(void) {
    if (s_tmpdir[0] == '\0') return;

    /* Best-effort: rm every known test file, then rmdir. */
    static const char *const entries[] = {
        "good.sh", "wrong.sh", "nonzero.sh", "hang.sh", NULL
    };
    char entry_path[PATH_MAX + 16];
    for (int i = 0; entries[i]; i++) {
        snprintf(entry_path, sizeof(entry_path), "%s/%s", s_tmpdir, entries[i]);
        unlink(entry_path);
    }
    rmdir(s_tmpdir);
    s_tmpdir[0] = '\0';
}

static void write_script(const char *path, const char *body) {
    FILE *fp = fopen(path, "w");
    TEST_ASSERT_NOT_NULL_MESSAGE(fp, path);
    fputs("#!/bin/sh\n", fp);
    fputs(body, fp);
    fclose(fp);
    TEST_ASSERT_EQUAL_INT(0, chmod(path, 0700));
}

void setUp(void) {
    char tmpl[] = "/tmp/lightnvr-go2rtc-probe-XXXXXX";
    char *created = mkdtemp(tmpl);
    TEST_ASSERT_NOT_NULL(created);
    snprintf(s_tmpdir, sizeof(s_tmpdir), "%s", created);
}

void tearDown(void) {
    remove_tmpdir();
}

void test_probe_accepts_correct_signature(void) {
    char path[PATH_MAX];
    snprintf(path, sizeof(path), "%s/good.sh", s_tmpdir);
    write_script(path,
                 "echo 'go2rtc version 1.2.3 linux/amd64'\n"
                 "exit 0\n");

    char version[128] = {0};
    int rc = go2rtc_process_probe_version(path, version, sizeof(version));
    TEST_ASSERT_EQUAL_INT(1, rc);
    TEST_ASSERT_NOT_NULL(strstr(version, "go2rtc version "));
    TEST_ASSERT_NOT_NULL(strstr(version, "1.2.3"));
}

void test_probe_rejects_wrong_banner(void) {
    char path[PATH_MAX];
    snprintf(path, sizeof(path), "%s/wrong.sh", s_tmpdir);
    write_script(path,
                 "echo 'not-go2rtc build 0.1'\n"
                 "exit 0\n");

    int rc = go2rtc_process_probe_version(path, NULL, 0);
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_probe_rejects_nonzero_exit(void) {
    char path[PATH_MAX];
    snprintf(path, sizeof(path), "%s/nonzero.sh", s_tmpdir);
    write_script(path,
                 "echo 'go2rtc version 9.9.9'\n"
                 "exit 1\n");

    int rc = go2rtc_process_probe_version(path, NULL, 0);
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_probe_times_out_on_hang(void) {
    char path[PATH_MAX];
    snprintf(path, sizeof(path), "%s/hang.sh", s_tmpdir);
    write_script(path,
                 /* Sleep longer than the 2s probe timeout.  probe_go2rtc_version
                  * must SIGKILL + reap this child, so the test returns promptly
                  * (well under 10s even if the timeout logic is loose). */
                 "sleep 10\n"
                 "echo 'go2rtc version nope'\n");

    struct timespec t0, t1;
    clock_gettime(CLOCK_MONOTONIC, &t0);
    int rc = go2rtc_process_probe_version(path, NULL, 0);
    clock_gettime(CLOCK_MONOTONIC, &t1);

    TEST_ASSERT_EQUAL_INT(0, rc);

    /* Must finish well before the 10s sleep — sanity-bound at 5s so we catch
     * both "didn't kill" and "didn't reap" failure modes without being flaky. */
    long elapsed_ms = (t1.tv_sec - t0.tv_sec) * 1000 +
                      (t1.tv_nsec - t0.tv_nsec) / 1000000;
    TEST_ASSERT_LESS_THAN_INT(5000, (int)elapsed_ms);
}

void test_probe_rejects_missing_path(void) {
    char path[PATH_MAX];
    snprintf(path, sizeof(path), "%s/does-not-exist", s_tmpdir);
    int rc = go2rtc_process_probe_version(path, NULL, 0);
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_probe_rejects_null_and_empty(void) {
    TEST_ASSERT_EQUAL_INT(0, go2rtc_process_probe_version(NULL, NULL, 0));
    TEST_ASSERT_EQUAL_INT(0, go2rtc_process_probe_version("", NULL, 0));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_probe_accepts_correct_signature);
    RUN_TEST(test_probe_rejects_wrong_banner);
    RUN_TEST(test_probe_rejects_nonzero_exit);
    RUN_TEST(test_probe_times_out_on_hang);
    RUN_TEST(test_probe_rejects_missing_path);
    RUN_TEST(test_probe_rejects_null_and_empty);
    return UNITY_END();
}
