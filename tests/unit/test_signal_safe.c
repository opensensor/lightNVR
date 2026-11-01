/**
 * @file test_signal_safe.c
 * @brief Layer 1 unit tests for the async-signal-safe stderr helper.
 */

#define _POSIX_C_SOURCE 200809L

#include <errno.h>
#include <stdbool.h>
#include <fcntl.h>
#include <string.h>
#include <unistd.h>
#include "unity.h"
#include "core/signal_safe.h"

static int saved_stderr = -1;
static int pipe_fds[2] = {-1, -1};

/* Fault injection via link-time wrapping (-Wl,--wrap=write, see
 * tests/unit/CMakeLists.txt), the convention used by the other unit tests.
 * signal_safe_write() is static inline, so its write() call is compiled into
 * this object and routed through __wrap_write. Only stderr writes are counted
 * or faulted, so unrelated output can never consume an injection. */
static bool inject_eintr;        /* one-shot: fail the next stderr write with EINTR */
static size_t short_write_len;   /* one-shot: if non-zero, write only this many bytes */
static int stderr_write_calls;

ssize_t __real_write(int fd, const void *buf, size_t count);
ssize_t __wrap_write(int fd, const void *buf, size_t count) {
    if (fd != STDERR_FILENO) {
        return __real_write(fd, buf, count);
    }
    stderr_write_calls++;
    if (inject_eintr) {
        inject_eintr = false;
        errno = EINTR;
        return -1;
    }
    if (short_write_len > 0 && count > short_write_len) {
        size_t len = short_write_len;
        short_write_len = 0;
        return __real_write(fd, buf, len);
    }
    return __real_write(fd, buf, count);
}

void setUp(void) {
    inject_eintr = false;
    short_write_len = 0;
    stderr_write_calls = 0;

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

void test_retries_on_eintr_and_preserves_errno(void) {
    inject_eintr = true;
    errno = EBUSY;
    signal_safe_write("retry-eintr\n");
    TEST_ASSERT_EQUAL_INT(EBUSY, errno);
    TEST_ASSERT_FALSE_MESSAGE(inject_eintr, "EINTR injection was not consumed by signal_safe_write");
    TEST_ASSERT_EQUAL_INT_MESSAGE(2, stderr_write_calls,
                                  "Expected one EINTR failure and one retry");

    char buffer[64];
    read_captured(buffer, sizeof(buffer));
    TEST_ASSERT_EQUAL_STRING("retry-eintr\n", buffer);
}

void test_retries_after_short_write_and_preserves_errno(void) {
    short_write_len = 3;
    errno = ENOTTY;
    signal_safe_write("short-write\n");
    TEST_ASSERT_EQUAL_INT(ENOTTY, errno);
    TEST_ASSERT_EQUAL_size_t_MESSAGE(0, short_write_len,
                                     "Short-write injection was not consumed by signal_safe_write");
    TEST_ASSERT_EQUAL_INT_MESSAGE(2, stderr_write_calls,
                                  "Expected one short write and one write for the remainder");

    char buffer[64];
    read_captured(buffer, sizeof(buffer));
    TEST_ASSERT_EQUAL_STRING("short-write\n", buffer);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_writes_whole_message);
    RUN_TEST(test_null_and_empty_are_noops);
    RUN_TEST(test_preserves_errno_on_success);
    RUN_TEST(test_preserves_errno_when_stderr_is_closed);
    RUN_TEST(test_retries_on_eintr_and_preserves_errno);
    RUN_TEST(test_retries_after_short_write_and_preserves_errno);
    return UNITY_END();
}
