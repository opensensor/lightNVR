/**
 * @file test_go2rtc_process_detection.c
 * @brief Regression tests for go2rtc process identification.
 */

#define _POSIX_C_SOURCE 200809L

#include <errno.h>
#include <limits.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>

#include "unity.h"
#include "video/go2rtc/go2rtc_process.h"

void setUp(void) {}
void tearDown(void) {}

static void sleep_millis(long millis) {
    struct timespec ts;
    ts.tv_sec = millis / 1000;
    ts.tv_nsec = (millis % 1000) * 1000000L;
    nanosleep(&ts, NULL);
}

static int wait_for_pid_running(pid_t pid) {
    for (int i = 0; i < 100; i++) {
        if (kill(pid, 0) == 0) {
            return 1;
        }
        sleep_millis(10);
    }
    return 0;
}

static void terminate_child(pid_t pid) {
    if (pid <= 0) return;

    kill(pid, SIGTERM);
    for (int i = 0; i < 50; i++) {
        int status = 0;
        pid_t rc = waitpid(pid, &status, WNOHANG);
        if (rc == pid || (rc == -1 && errno == ECHILD)) {
            return;
        }
        sleep_millis(10);
    }

    kill(pid, SIGKILL);
    waitpid(pid, NULL, 0);
}

static pid_t spawn_argument_false_positive(void) {
    pid_t pid = fork();
    if (pid == 0) {
        execl("/bin/sh", "sh", "-c", "while :; do sleep 1; done", "go2rtc", (char *)NULL);
        _exit(127);
    }

    if (pid <= 0 || !wait_for_pid_running(pid)) {
        terminate_child(pid);
        return -1;
    }

    return pid;
}

static pid_t spawn_path_false_positive(char *dir_out, size_t dir_out_size) {
    char dir_template[] = "/tmp/lightnvr-go2rtc-test-XXXXXX";
    char *dir = mkdtemp(dir_template);
    if (!dir) {
        return -1;
    }

    strncpy(dir_out, dir, dir_out_size - 1);
    dir_out[dir_out_size - 1] = '\0';

    char script_path[PATH_MAX];
    snprintf(script_path, sizeof(script_path), "%s/hold.sh", dir_out);

    FILE *fp = fopen(script_path, "w");
    if (!fp) {
        rmdir(dir_out);
        dir_out[0] = '\0';
        return -1;
    }

    fputs("#!/bin/sh\nwhile :; do sleep 1; done\n", fp);
    fclose(fp);

    if (chmod(script_path, 0700) != 0) {
        unlink(script_path);
        rmdir(dir_out);
        dir_out[0] = '\0';
        return -1;
    }

    pid_t pid = fork();
    if (pid == 0) {
        execl(script_path, script_path, (char *)NULL);
        _exit(127);
    }

    if (pid <= 0 || !wait_for_pid_running(pid)) {
        terminate_child(pid);
        unlink(script_path);
        rmdir(dir_out);
        dir_out[0] = '\0';
        return -1;
    }

    return pid;
}

void test_get_pid_ignores_go2rtc_in_non_argv0_argument(void) {
    pid_t fake_pid = spawn_argument_false_positive();
    TEST_ASSERT_GREATER_THAN_INT(0, fake_pid);

    int detected_pid = go2rtc_process_get_pid();
    terminate_child(fake_pid);

    TEST_ASSERT_NOT_EQUAL(fake_pid, detected_pid);
}

void test_get_pid_ignores_go2rtc_in_directory_name(void) {
    char fake_dir[PATH_MAX] = {0};
    pid_t fake_pid = spawn_path_false_positive(fake_dir, sizeof(fake_dir));
    TEST_ASSERT_GREATER_THAN_INT(0, fake_pid);

    int detected_pid = go2rtc_process_get_pid();

    char script_path[PATH_MAX];
    snprintf(script_path, sizeof(script_path), "%s/hold.sh", fake_dir);
    terminate_child(fake_pid);
    unlink(script_path);
    rmdir(fake_dir);

    TEST_ASSERT_NOT_EQUAL(fake_pid, detected_pid);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_get_pid_ignores_go2rtc_in_non_argv0_argument);
    RUN_TEST(test_get_pid_ignores_go2rtc_in_directory_name);
    return UNITY_END();
}