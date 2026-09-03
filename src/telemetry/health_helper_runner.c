#define _POSIX_C_SOURCE 200809L

#include "telemetry/health_helper_runner.h"

#include <errno.h>
#include <fcntl.h>
#include <poll.h>
#include <pthread.h>
#include <signal.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>

static pthread_mutex_t runner_lock = PTHREAD_MUTEX_INITIALIZER;
static pthread_mutex_t abandoned_lock = PTHREAD_MUTEX_INITIALIZER;
static pid_t abandoned_pids[HEALTH_HELPER_ABANDONED_MAX];
static size_t abandoned_count;

static uint64_t monotonic_ms(void) {
    struct timespec value;
    if (clock_gettime(CLOCK_MONOTONIC, &value) != 0) return 0;
    return (uint64_t)value.tv_sec * 1000U + (uint64_t)value.tv_nsec / 1000000U;
}

static void remember_abandoned(pid_t pid) {
    pthread_mutex_lock(&abandoned_lock);
    if (abandoned_count < HEALTH_HELPER_ABANDONED_MAX) {
        abandoned_pids[abandoned_count++] = pid;
    }
    pthread_mutex_unlock(&abandoned_lock);
}

void health_helper_reap_abandoned(void) {
    pthread_mutex_lock(&abandoned_lock);
    size_t destination = 0U;
    for (size_t index = 0; index < abandoned_count; ++index) {
        pid_t pid = abandoned_pids[index];
        int status = 0;
        pid_t waited;
        do {
            waited = waitpid(pid, &status, WNOHANG);
        } while (waited < 0 && errno == EINTR);
        if (waited == 0) abandoned_pids[destination++] = pid;
    }
    abandoned_count = destination;
    pthread_mutex_unlock(&abandoned_lock);
}

uint32_t health_helper_abandoned_count(void) {
    health_helper_reap_abandoned();
    pthread_mutex_lock(&abandoned_lock);
    uint32_t count = (uint32_t)abandoned_count;
    pthread_mutex_unlock(&abandoned_lock);
    return count;
}

static void signal_helper(pid_t pid, int signal_number) {
    if (kill(-pid, signal_number) != 0) (void)kill(pid, signal_number);
}

static void drain_output(int fd, health_helper_result_t *result,
                         size_t limit) {
    char buffer[512];
    for (;;) {
        ssize_t count = read(fd, buffer, sizeof(buffer));
        if (count > 0) {
            size_t available = limit > result->output_length
                                   ? limit - result->output_length
                                   : 0U;
            size_t copy = (size_t)count < available ? (size_t)count : available;
            if (copy > 0) {
                memcpy(result->output + result->output_length, buffer, copy);
                result->output_length += copy;
                result->output[result->output_length] = '\0';
            }
            if (copy < (size_t)count) result->output_truncated = true;
            continue;
        }
        if (count < 0 && errno == EINTR) continue;
        break;
    }
}

static bool wait_until(pid_t pid, int *status, uint64_t deadline_ms,
                       int output_fd, health_helper_result_t *result,
                       size_t output_limit, bool *wait_error) {
    for (;;) {
        drain_output(output_fd, result, output_limit);
        pid_t waited = waitpid(pid, status, WNOHANG);
        if (waited == pid) {
            drain_output(output_fd, result, output_limit);
            return true;
        }
        if (waited < 0 && errno != EINTR) {
            if (wait_error) *wait_error = true;
            return false;
        }
        uint64_t now = monotonic_ms();
        if (now >= deadline_ms) return false;
        uint64_t remaining = deadline_ms - now;
        int wait_ms = remaining > 20U ? 20 : (int)remaining;
        struct pollfd poll_fd = {.fd = output_fd, .events = POLLIN | POLLHUP};
        (void)poll(&poll_fd, 1, wait_ms);
    }
}

int health_helper_run(const health_helper_request_t *request,
                      health_helper_result_t *result) {
    if (!request || !result || !request->program || request->program[0] != '/' ||
        !request->argv || !request->argv[0] || request->timeout_ms == 0U) {
        return -1;
    }
    memset(result, 0, sizeof(*result));
    result->exit_code = -1;
    size_t output_limit = request->output_limit;
    if (output_limit == 0U || output_limit > HEALTH_HELPER_OUTPUT_MAX)
        output_limit = HEALTH_HELPER_OUTPUT_MAX;

    health_helper_reap_abandoned();
    if (health_helper_abandoned_count() > 0U ||
        pthread_mutex_trylock(&runner_lock) != 0) {
        result->outcome = HEALTH_HELPER_BUSY;
        return 0;
    }

    int descriptors[2];
    if (pipe(descriptors) != 0) {
        result->outcome = HEALTH_HELPER_SYSTEM_ERROR;
        pthread_mutex_unlock(&runner_lock);
        return -1;
    }
    (void)fcntl(descriptors[0], F_SETFD, FD_CLOEXEC);
    (void)fcntl(descriptors[1], F_SETFD, FD_CLOEXEC);

    uint64_t started = monotonic_ms();
    pid_t pid = fork();
    if (pid == 0) {
        (void)setpgid(0, 0);
        close(descriptors[0]);
        int null_fd = open("/dev/null", O_RDONLY);
        if (null_fd < 0 || dup2(null_fd, STDIN_FILENO) < 0) _exit(126);
        if (null_fd != STDIN_FILENO) close(null_fd);
        if (dup2(descriptors[1], STDOUT_FILENO) < 0 ||
            dup2(descriptors[1], STDERR_FILENO) < 0) {
            _exit(126);
        }
        close(descriptors[1]);
        char *const clean_environment[] = {
            "PATH=/usr/sbin:/usr/bin:/sbin:/bin", "LANG=C", "LC_ALL=C", NULL};
        execve(request->program, request->argv, clean_environment);
        _exit(127);
    }
    close(descriptors[1]);
    if (pid < 0) {
        close(descriptors[0]);
        result->outcome = HEALTH_HELPER_SYSTEM_ERROR;
        pthread_mutex_unlock(&runner_lock);
        return -1;
    }
    (void)setpgid(pid, pid);
    int flags = fcntl(descriptors[0], F_GETFL, 0);
    if (flags >= 0) (void)fcntl(descriptors[0], F_SETFL, flags | O_NONBLOCK);

    int status = 0;
    bool wait_error = false;
    bool reaped = wait_until(pid, &status, started + request->timeout_ms,
                             descriptors[0], result, output_limit, &wait_error);
    if (wait_error) {
        result->outcome = HEALTH_HELPER_SYSTEM_ERROR;
        signal_helper(pid, SIGKILL);
        wait_error = false;
        reaped = wait_until(pid, &status, monotonic_ms() + 100U,
                            descriptors[0], result, output_limit, &wait_error);
        if (!reaped) {
            result->abandoned = true;
            remember_abandoned(pid);
        }
    } else if (!reaped) {
        result->outcome = HEALTH_HELPER_TIMED_OUT;
        signal_helper(pid, SIGTERM);
        uint32_t grace = request->terminate_grace_ms;
        if (grace > 1000U) grace = 1000U;
        reaped = wait_until(pid, &status, monotonic_ms() + grace,
                            descriptors[0], result, output_limit, &wait_error);
        if (!reaped) {
            signal_helper(pid, SIGKILL);
            wait_error = false;
            reaped = wait_until(pid, &status, monotonic_ms() + 100U,
                                descriptors[0], result, output_limit,
                                &wait_error);
        }
        if (!reaped) {
            result->abandoned = true;
            remember_abandoned(pid);
        }
    } else if (WIFEXITED(status)) {
        result->exit_code = WEXITSTATUS(status);
        result->outcome = result->exit_code == 0 ? HEALTH_HELPER_OK
                                                 : HEALTH_HELPER_EXITED;
        if (result->exit_code == 127) result->outcome = HEALTH_HELPER_EXEC_ERROR;
    } else if (WIFSIGNALED(status)) {
        result->term_signal = WTERMSIG(status);
        result->outcome = HEALTH_HELPER_EXITED;
    } else {
        result->outcome = HEALTH_HELPER_SYSTEM_ERROR;
    }

    result->latency_ms = (uint32_t)(monotonic_ms() - started);
    close(descriptors[0]);
    pthread_mutex_unlock(&runner_lock);
    return 0;
}
