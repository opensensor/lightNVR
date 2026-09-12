#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <dirent.h>
#include <pthread.h>
#include <signal.h>
#include <stdatomic.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include <libavformat/avformat.h>

#include "unity.h"
#include "core/config.h"
#include "core/logger.h"
#include "core/shutdown_coordinator.h"
#include "video/hls_writer.h"
#include "video/hls/hls_unified_thread.h"
#include "video/stream_manager.h"
#include "video/stream_state.h"

static atomic_int alarm_calls;
static atomic_int disposition_changes;
static char test_dir[128];
static pthread_barrier_t close_barrier;
static atomic_bool pace_input;

int __real_av_read_frame(AVFormatContext *input, AVPacket *packet);

int __wrap_av_read_frame(AVFormatContext *input, AVPacket *packet) {
    // Keep the local media source open long enough to stop a running thread,
    // without requiring an external RTSP server in the unit-test suite.
    if (atomic_load(&pace_input) && input->url &&
        strcmp(input->url, TEST_RECORDING_PATH) == 0) usleep(10000);
    return __real_av_read_frame(input, packet);
}

int __real_sigaction(int sig, const struct sigaction *action,
                     struct sigaction *previous);

int __wrap_sigaction(int sig, const struct sigaction *action,
                     struct sigaction *previous) {
    if (action && (sig == SIGALRM || sig == SIGSEGV))
        atomic_fetch_add(&disposition_changes, 1);
    return __real_sigaction(sig, action, previous);
}

unsigned int __wrap_alarm(unsigned int seconds) {
    (void)seconds;
    atomic_fetch_add(&alarm_calls, 1);
    // Do not let a regression terminate the test runner with SIGALRM.
    return 0;
}

static void remove_test_files(const char *path) {
    DIR *dir = opendir(path);
    if (!dir) return;
    struct dirent *entry;
    while ((entry = readdir(dir))) {
        if (!strcmp(entry->d_name, ".") || !strcmp(entry->d_name, "..")) continue;
        char child[512];
        snprintf(child, sizeof(child), "%s/%s", path, entry->d_name);
        if (unlink(child) != 0) remove_test_files(child);
    }
    closedir(dir);
    rmdir(path);
}

void setUp(void) {
    strcpy(test_dir, "/tmp/lightnvr_hls_cleanup_XXXXXX");
    TEST_ASSERT_NOT_NULL(mkdtemp(test_dir));
    load_default_config(&g_config);
    snprintf(g_config.storage_path, sizeof(g_config.storage_path), "%s", test_dir);
    g_config.max_streams = 4;
    init_shutdown_coordinator();
    atomic_store(&alarm_calls, 0);
    atomic_store(&disposition_changes, 0);
}

void tearDown(void) {
    cleanup_all_hls_writers();
    remove_test_files(test_dir);
}

static void *close_writer(void *arg) {
    pthread_barrier_wait(&close_barrier);
    hls_writer_close(arg);
    return NULL;
}

void test_parallel_writer_teardown_and_recreation_preserves_signals(void) {
    enum { CAMERAS = 3, CYCLES = 3 };
    for (int cycle = 0; cycle < CYCLES; ++cycle) {
        hls_writer_t *writers[CAMERAS];
        pthread_t threads[CAMERAS];
        char names[CAMERAS][32];
        char playlists[CAMERAS][MAX_PATH_LENGTH + 32];
        for (int camera = 0; camera < CAMERAS; ++camera) {
            snprintf(names[camera], sizeof(names[camera]), "cleanup-%d", camera);
            writers[camera] = hls_writer_create(test_dir, names[camera], 2);
            TEST_ASSERT_NOT_NULL(writers[camera]);
            AVFormatContext *input = NULL;
            TEST_ASSERT_EQUAL_INT(0, avformat_open_input(
                &input, TEST_RECORDING_PATH, NULL, NULL));
            TEST_ASSERT_GREATER_OR_EQUAL(0, avformat_find_stream_info(input, NULL));
            TEST_ASSERT_EQUAL_INT(0, hls_writer_initialize(writers[camera], input->streams[0]));
            AVPacket *packet = av_packet_alloc();
            TEST_ASSERT_NOT_NULL(packet);
            for (int i = 0; i < 8; ++i) {
                TEST_ASSERT_EQUAL_INT(0, av_read_frame(input, packet));
                TEST_ASSERT_GREATER_OR_EQUAL(0, hls_writer_write_packet(
                    writers[camera], packet, input->streams[0]));
                av_packet_unref(packet);
            }
            av_packet_free(&packet);
            avformat_close_input(&input);
            snprintf(playlists[camera], sizeof(playlists[camera]), "%s/index.m3u8",
                     writers[camera]->output_dir);
        }
        TEST_ASSERT_EQUAL_INT(0, pthread_barrier_init(&close_barrier, NULL, CAMERAS));
        for (int camera = 0; camera < CAMERAS; ++camera)
            TEST_ASSERT_EQUAL_INT(0, pthread_create(&threads[camera], NULL,
                                                   close_writer, writers[camera]));
        for (int camera = 0; camera < CAMERAS; ++camera) {
            TEST_ASSERT_EQUAL_INT(0, pthread_join(threads[camera], NULL));
            TEST_ASSERT_NULL(find_hls_writer_by_stream_name(names[camera]));
            AVFormatContext *recorded = NULL;
            TEST_ASSERT_EQUAL_INT(0, avformat_open_input(
                &recorded, playlists[camera], NULL, NULL));
            AVPacket *packet = av_packet_alloc();
            TEST_ASSERT_NOT_NULL(packet);
            TEST_ASSERT_EQUAL_INT(0, av_read_frame(recorded, packet));
            av_packet_free(&packet);
            avformat_close_input(&recorded);
        }
        pthread_barrier_destroy(&close_barrier);
    }
    TEST_ASSERT_EQUAL_INT(0, atomic_load(&alarm_calls));
    TEST_ASSERT_EQUAL_INT(0, atomic_load(&disposition_changes));
}

void test_unified_system_cleanup_preserves_signals(void) {
    cleanup_hls_unified_thread_system();
    TEST_ASSERT_EQUAL_INT(0, atomic_load(&alarm_calls));
    TEST_ASSERT_EQUAL_INT(0, atomic_load(&disposition_changes));
}

typedef struct {
    char name[32];
    bool restart;
    int result;
} lifecycle_task_t;

static void *change_stream_lifecycle(void *arg) {
    lifecycle_task_t *task = arg;
    pthread_barrier_wait(&close_barrier);
    task->result = task->restart ? restart_hls_unified_stream(task->name)
                                 : stop_hls_unified_stream(task->name);
    return NULL;
}

void test_parallel_unified_stream_shutdown_and_restart(void) {
    enum { CAMERAS = 3 };
    lifecycle_task_t tasks[CAMERAS] = {0};
    pthread_t threads[CAMERAS];
    TEST_ASSERT_EQUAL_INT(0, init_stream_state_manager(4));
    TEST_ASSERT_EQUAL_INT(0, init_stream_manager(4));
    atomic_store(&pace_input, true);
    for (int camera = 0; camera < CAMERAS; ++camera) {
        snprintf(tasks[camera].name, sizeof(tasks[camera].name), "lifecycle-%d", camera);
        stream_config_t config = {0};
        snprintf(config.name, sizeof(config.name), "%s", tasks[camera].name);
        snprintf(config.url, sizeof(config.url), "%s", TEST_RECORDING_PATH);
        config.enabled = true;
        config.streaming_enabled = true;
        config.segment_duration = 2;
        config.protocol = STREAM_PROTOCOL_TCP;
        TEST_ASSERT_NOT_NULL(add_stream(&config));
        TEST_ASSERT_EQUAL_INT(0, start_hls_unified_stream(config.name));
    }
    for (int cycle = 0; cycle < 3; ++cycle) {
        for (int camera = 0; camera < CAMERAS; ++camera) {
            int tries = 0;
            while (!is_hls_stream_active(tasks[camera].name) && tries++ < 100)
                usleep(50000);
            TEST_ASSERT_TRUE(is_hls_stream_active(tasks[camera].name));
            tasks[camera].restart = cycle < 2;
        }
        TEST_ASSERT_EQUAL_INT(0, pthread_barrier_init(&close_barrier, NULL, CAMERAS));
        for (int camera = 0; camera < CAMERAS; ++camera)
            TEST_ASSERT_EQUAL_INT(0, pthread_create(&threads[camera], NULL,
                change_stream_lifecycle, &tasks[camera]));
        for (int camera = 0; camera < CAMERAS; ++camera) {
            TEST_ASSERT_EQUAL_INT(0, pthread_join(threads[camera], NULL));
            TEST_ASSERT_EQUAL_INT(0, tasks[camera].result);
        }
        pthread_barrier_destroy(&close_barrier);
    }
    for (int camera = 0; camera < CAMERAS; ++camera) {
        TEST_ASSERT_FALSE(is_hls_stream_active(tasks[camera].name));
        TEST_ASSERT_NULL(find_hls_writer_by_stream_name(tasks[camera].name));
    }
    atomic_store(&pace_input, false);
    shutdown_stream_manager();
    shutdown_stream_state_manager();
    TEST_ASSERT_EQUAL_INT(0, atomic_load(&alarm_calls));
    TEST_ASSERT_EQUAL_INT(0, atomic_load(&disposition_changes));
}

int main(void) {
    init_logger();
    UNITY_BEGIN();
    RUN_TEST(test_parallel_writer_teardown_and_recreation_preserves_signals);
    RUN_TEST(test_parallel_unified_stream_shutdown_and_restart);
    RUN_TEST(test_unified_system_cleanup_preserves_signals);
    int result = UNITY_END();
    shutdown_logger();
    return result;
}
