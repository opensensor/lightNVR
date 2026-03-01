/**
 * @file thumbnail_thread.c
 * @brief Detached pthread-based thumbnail generation with async completion
 */

#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <unistd.h>
#include <fcntl.h>
#include <time.h>
#include <errno.h>

#include "web/thumbnail_thread.h"
#include "core/logger.h"
#include "utils/memory.h"

// Maximum concurrent thumbnail generations
#define MAX_CONCURRENT_THUMBNAILS 4

/**
 * @brief Work item for thumbnail generation
 */
typedef struct thumbnail_work {
    uint64_t recording_id;
    int index;
    char input_path[512];
    char output_path[512];
    double seek_seconds;
    deferred_action_handle_t deferred_action;
    deferred_response_callback_t callback;
    int result;  // 0 = success, -1 = failure
    struct thumbnail_work *next;
} thumbnail_work_t;

/**
 * @brief Global state for the thumbnail thread subsystem
 */
static struct {
    uv_loop_t *loop;
    uv_async_t async_handle;
    pthread_mutex_t done_mutex;
    thumbnail_work_t *done_queue_head;
    thumbnail_work_t *done_queue_tail;
    volatile int active_count;
    volatile bool shutting_down;
} g_thumbnail_state = {0};

/**
 * @brief Generate a thumbnail using ffmpeg
 */
static int generate_thumbnail_internal(const char *input_path, const char *output_path,
                                       double seek_seconds) {
    // Clamp seek time to 0 minimum
    if (seek_seconds < 0) seek_seconds = 0;

    char seek_str[32];
    snprintf(seek_str, sizeof(seek_str), "%.2f", seek_seconds);

    log_debug("Generating thumbnail: ffmpeg -ss %s -i \"%s\" -> \"%s\"",
              seek_str, input_path, output_path);

    // Use fork/execvp instead of system() to avoid spawning a shell
    pid_t pid = fork();
    if (pid < 0) {
        log_warn("fork() failed for thumbnail generation: %s", strerror(errno));
        return -1;
    }

    if (pid == 0) {
        // Child process: redirect stderr to /dev/null, then exec ffmpeg
        int devnull = open("/dev/null", O_WRONLY);
        if (devnull >= 0) {
            dup2(devnull, STDERR_FILENO);
            close(devnull);
        }
        const char *args[] = {
            "ffmpeg", "-ss", seek_str,
            "-i", input_path,
            "-frames:v", "1",
            "-vf", "scale=320:-1",
            "-q:v", "8",
            "-y", output_path,
            NULL
        };
        execvp("ffmpeg", (char *const *)args);
        // execvp only returns on failure
        _exit(127);
    }

    // Parent: wait up to 5 seconds for child to complete
    int ret = -1;
    time_t deadline = time(NULL) + 5;
    while (time(NULL) < deadline) {
        int status = 0;
        pid_t result = waitpid(pid, &status, WNOHANG);
        if (result == pid) {
            if (WIFEXITED(status) && WEXITSTATUS(status) == 0) {
                ret = 0;
            }
            pid = -1;
            break;
        }
        if (result < 0) {
            pid = -1;
            break;
        }
        usleep(50000); // 50 ms polling interval
    }
    if (pid > 0) {
        // Timeout — kill the child
        kill(pid, SIGKILL);
        waitpid(pid, NULL, 0);
    }

    if (ret != 0) {
        log_warn("ffmpeg thumbnail generation failed for: %s", input_path);
        return -1;
    }

    // Verify the file was created
    struct stat st;
    if (stat(output_path, &st) != 0 || st.st_size == 0) {
        log_warn("Thumbnail file not created or empty: %s", output_path);
        unlink(output_path); // Clean up empty file
        return -1;
    }

    log_debug("Generated thumbnail: %s (%lld bytes)", output_path,
              (long long)st.st_size);
    return 0;
}

/**
 * @brief Add a work item to the done queue (thread-safe)
 */
static void enqueue_done(thumbnail_work_t *work) {
    pthread_mutex_lock(&g_thumbnail_state.done_mutex);
    work->next = NULL;
    if (g_thumbnail_state.done_queue_tail) {
        g_thumbnail_state.done_queue_tail->next = work;
    } else {
        g_thumbnail_state.done_queue_head = work;
    }
    g_thumbnail_state.done_queue_tail = work;
    pthread_mutex_unlock(&g_thumbnail_state.done_mutex);
}

/**
 * @brief Dequeue all completed work items (thread-safe)
 */
static thumbnail_work_t *dequeue_all_done(void) {
    pthread_mutex_lock(&g_thumbnail_state.done_mutex);
    thumbnail_work_t *head = g_thumbnail_state.done_queue_head;
    g_thumbnail_state.done_queue_head = NULL;
    g_thumbnail_state.done_queue_tail = NULL;
    pthread_mutex_unlock(&g_thumbnail_state.done_mutex);
    return head;
}

/**
 * @brief Detached pthread worker function
 */
static void *thumbnail_worker_thread(void *arg) {
    thumbnail_work_t *work = (thumbnail_work_t *)arg;

    // Generate the thumbnail
    work->result = generate_thumbnail_internal(work->input_path, work->output_path,
                                               work->seek_seconds);

    // Add to done queue
    enqueue_done(work);

    // Signal the event loop via uv_async
    uv_async_send(&g_thumbnail_state.async_handle);

    // Decrement active count
    __sync_sub_and_fetch(&g_thumbnail_state.active_count, 1);

    return NULL;
}

/**
 * @brief uv_async callback - runs on the event loop thread
 *
 * Processes all completed thumbnail generations and sends responses.
 */
static void thumbnail_async_cb(uv_async_t *handle) {
    (void)handle;

    thumbnail_work_t *work = dequeue_all_done();
    while (work) {
        thumbnail_work_t *next = work->next;

        log_debug("Thumbnail generation completed for recording %llu index %d: %s",
                  (unsigned long long)work->recording_id, work->index,
                  work->result == 0 ? "success" : "failure");

        // Invoke the callback to send the response
        if (work->callback) {
            const char *output_path = work->result == 0 ? work->output_path : NULL;
            work->callback(work->deferred_action, output_path, work->result);
        }

        safe_free(work);
        work = next;
    }
}

// ============================================================================
// Public API
// ============================================================================

int thumbnail_thread_init(uv_loop_t *loop) {
    if (!loop) {
        log_error("thumbnail_thread_init: NULL loop");
        return -1;
    }

    memset(&g_thumbnail_state, 0, sizeof(g_thumbnail_state));
    g_thumbnail_state.loop = loop;

    // Initialize mutex
    if (pthread_mutex_init(&g_thumbnail_state.done_mutex, NULL) != 0) {
        log_error("thumbnail_thread_init: Failed to initialize mutex");
        return -1;
    }

    // Initialize uv_async handle
    if (uv_async_init(loop, &g_thumbnail_state.async_handle, thumbnail_async_cb) != 0) {
        log_error("thumbnail_thread_init: Failed to initialize uv_async");
        pthread_mutex_destroy(&g_thumbnail_state.done_mutex);
        return -1;
    }

    log_info("thumbnail_thread_init: Thumbnail thread subsystem initialized");
    return 0;
}

void thumbnail_thread_shutdown(void) {
    log_info("thumbnail_thread_shutdown: Shutting down thumbnail thread subsystem");
    g_thumbnail_state.shutting_down = true;

    // Wait for all active threads to complete
    int wait_count = 0;
    while (__sync_fetch_and_add(&g_thumbnail_state.active_count, 0) > 0 && wait_count < 100) {
        usleep(100000); // 100ms
        wait_count++;
    }

    if (g_thumbnail_state.active_count > 0) {
        log_warn("thumbnail_thread_shutdown: %d threads still active after timeout",
                 g_thumbnail_state.active_count);
    }

    // Close uv_async handle
    if (!uv_is_closing((uv_handle_t *)&g_thumbnail_state.async_handle)) {
        uv_close((uv_handle_t *)&g_thumbnail_state.async_handle, NULL);
    }

    // Destroy mutex
    pthread_mutex_destroy(&g_thumbnail_state.done_mutex);

    // Free any remaining work items in the done queue
    thumbnail_work_t *work = g_thumbnail_state.done_queue_head;
    while (work) {
        thumbnail_work_t *next = work->next;
        safe_free(work);
        work = next;
    }

    log_info("thumbnail_thread_shutdown: Shutdown complete");
}

int thumbnail_thread_submit(uint64_t recording_id, int index,
                            const char *input_path, const char *output_path,
                            double seek_seconds, deferred_action_handle_t deferred_action,
                            deferred_response_callback_t callback) {
    if (g_thumbnail_state.shutting_down) {
        log_warn("thumbnail_thread_submit: Rejecting request during shutdown");
        return -1;
    }

    // Enforce concurrency limit
    if (__sync_fetch_and_add(&g_thumbnail_state.active_count, 0) >= MAX_CONCURRENT_THUMBNAILS) {
        log_debug("thumbnail_thread_submit: Max concurrent thumbnails reached");
        return -1;
    }

    // Allocate work item
    thumbnail_work_t *work = safe_calloc(1, sizeof(thumbnail_work_t));
    if (!work) {
        log_error("thumbnail_thread_submit: Failed to allocate work item");
        return -1;
    }

    work->recording_id = recording_id;
    work->index = index;
    snprintf(work->input_path, sizeof(work->input_path), "%s", input_path);
    snprintf(work->output_path, sizeof(work->output_path), "%s", output_path);
    work->seek_seconds = seek_seconds;
    work->deferred_action = deferred_action;
    work->callback = callback;

    // Increment active count
    __sync_add_and_fetch(&g_thumbnail_state.active_count, 1);

    // Spawn detached thread
    pthread_t thread;
    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);

    if (pthread_create(&thread, &attr, thumbnail_worker_thread, work) != 0) {
        log_error("thumbnail_thread_submit: Failed to create thread");
        pthread_attr_destroy(&attr);
        __sync_sub_and_fetch(&g_thumbnail_state.active_count, 1);
        safe_free(work);
        return -1;
    }

    pthread_attr_destroy(&attr);
    log_debug("thumbnail_thread_submit: Submitted thumbnail generation for recording %llu index %d",
              (unsigned long long)recording_id, index);
    return 0;
}

int thumbnail_thread_get_active_count(void) {
    return __sync_fetch_and_add(&g_thumbnail_state.active_count, 0);
}

