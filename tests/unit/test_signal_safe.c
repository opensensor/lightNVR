/**
 * @file test_signal_safe.c
 * @brief Layer 1 unit tests for the async-signal-safe stderr helper.
 */

#define _POSIX_C_SOURCE 200809L

#include <errno.h>
#include <fcntl.h>
#include <string.h>
#include <unistd.h>
#include "unity.h"
#include "core/signal_safe.h"

static int saved_stderr = -1;
static int pipe_fds[2] = {-1, -1};

void setUp(void) {
    TEST_ASSERT_EQUAL_INT(0, pipe(pipe_fds));
    saved_stderr = dup(STDERR_FILENO);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, saved_stderr);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, dup2(pipe_fds[1], STDERR_FILENO));
}

void tearDown(void) {
    dup2(saved_stderr, STDERR_FILENO);
    close(saved_stderr);
    if (pipe_fds[0] >= 0) close(pipe_fds[0]);
    if (pipe_fds[1] >= 0) close(pipe_fds[1]);
    pipe_fds[0] = pipe_fds[1] = -1;
}

static void read_captured(char *buffer, size_t size) {
    close(pipe_fds[1]);
    pipe_fds[1] = -1;
    dup2(saved_stderr, STDERR_FILENO);  /* drop the last write end */
    size_t used = 0;
    ssize_t n;
    while (used + 1 < size &&
           (n = read(pipe_fds[0], buffer + used, size - 1 - used)) > 0) {
        used += (size_t)n;
    }
    buffer[used] = '\0';
}

void test_writes_whole_message(void) {
    signal_safe_write("[SIGNAL] hello\n");
    char buffer[64];
    read_captured(buffer, sizeof(buffer));
    TEST_ASSERT_EQUAL_STRING("[SIGNAL] hello\n", buffer);
}

void test_null_and_empty_are_noops(void) {
    signal_safe_write(NULL);
    signal_safe_write("");
    char buffer[16];
    read_captured(buffer, sizeof(buffer));
    TEST_ASSERT_EQUAL_STRING("", buffer);
}

void test_preserves_errno_on_success(void) {
    errno = ENOENT;
    signal_safe_write("x\n");
    TEST_ASSERT_EQUAL_INT(ENOENT, errno);
}

void test_preserves_errno_when_stderr_is_closed(void) {
    dup2(saved_stderr, STDERR_FILENO);
    int closed_copy = dup(STDERR_FILENO);
    close(STDERR_FILENO);            /* write() will fail with EBADF */
    errno = EACCES;
    signal_safe_write("lost\n");
    int observed = errno;
    dup2(closed_copy, STDERR_FILENO);
    close(closed_copy);
    TEST_ASSERT_EQUAL_INT_MESSAGE(EACCES, observed,
                                  "A failed write must not clobber errno");
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_writes_whole_message);
    RUN_TEST(test_null_and_empty_are_noops);
    RUN_TEST(test_preserves_errno_on_success);
    RUN_TEST(test_preserves_errno_when_stderr_is_closed);
    return UNITY_END();
}
