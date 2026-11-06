/* SPDX-License-Identifier: GPL-3.0-or-later */
#define _POSIX_C_SOURCE 200809L
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/wait.h>
#include <sys/stat.h>
#include <pthread.h>
#include <libavutil/error.h>
#include "unity.h"
#include "video/recording_file.h"
#include "video/recording_path.h"

static char directory[128], path[256];
static int write_error, close_error;
ssize_t __real_write(int fd, const void *buffer, size_t size);
int __real_close(int fd);
ssize_t __wrap_write(int fd, const void *buffer, size_t size) {
    if (write_error) { errno = write_error; return -1; }
    return __real_write(fd, buffer, size);
}
int __wrap_close(int fd) {
    int result = __real_close(fd);
    if (close_error) { errno = close_error; return -1; }
    return result;
}
void setUp(void) {
    strcpy(directory, "/tmp/lightnvr_recording_file_XXXXXX");
    TEST_ASSERT_NOT_NULL(mkdtemp(directory));
    snprintf(path, sizeof(path), "%s/recording.mp4", directory);
    write_error = close_error = 0;
}
void tearDown(void) {
    write_error = close_error = 0;
    unlink(path);
    rmdir(directory);
}
void test_permissions_are_private_even_with_permissive_umask(void) {
    AVIOContext *io = NULL;
    mode_t previous = umask(0);
    int result = recording_file_open(&io, path);
    umask(previous); // Restore process-wide state before assertions can abort.
    TEST_ASSERT_EQUAL_INT(0, result);
    struct stat st;
    TEST_ASSERT_EQUAL_INT(0, stat(path, &st));
    TEST_ASSERT_EQUAL_UINT32(0600, st.st_mode & 0777);
    TEST_ASSERT_EQUAL_INT(0, recording_file_close(&io));
}

void test_exclusive_open_preserves_existing_file(void) {
    AVIOContext *first = NULL, *second = NULL;
    TEST_ASSERT_EQUAL_INT(0, recording_file_open(&first, path));
    avio_write(first, (const unsigned char *)"protected", 9);
    TEST_ASSERT_EQUAL_INT(AVERROR(EEXIST), recording_file_open(&second, path));
    TEST_ASSERT_NULL(second);
    TEST_ASSERT_EQUAL_INT(0, recording_file_close(&first));
    TEST_ASSERT_NULL(first);
    TEST_ASSERT_EQUAL_INT(AVERROR(EEXIST), recording_file_open(&second, path));
    char data[16] = {0};
    FILE *f = fopen(path, "rb");
    TEST_ASSERT_NOT_NULL(f);
    TEST_ASSERT_EQUAL_UINT(9, fread(data, 1, sizeof(data), f));
    fclose(f);
    TEST_ASSERT_EQUAL_STRING("protected", data);
}
void test_symlink_is_not_followed(void) {
    TEST_ASSERT_EQUAL_INT(0, symlink("missing-target", path));
    AVIOContext *io = NULL;
    TEST_ASSERT_EQUAL_INT(AVERROR(EEXIST), recording_file_open(&io, path));
    TEST_ASSERT_NULL(io);
}
static void *competing_open(void *arg) {
    AVIOContext *io = NULL;
    int *result = arg;
    *result = recording_file_open(&io, path);
    if (*result == 0) recording_file_close(&io);
    return NULL;
}
void test_concurrent_open_has_one_owner(void) {
    pthread_t threads[16];
    int results[16];
    for (int i = 0; i < 16; ++i)
        TEST_ASSERT_EQUAL_INT(0, pthread_create(&threads[i], NULL, competing_open, &results[i]));
    int owners = 0;
    for (int i = 0; i < 16; ++i) {
        pthread_join(threads[i], NULL);
        if (results[i] == 0) ++owners;
        else TEST_ASSERT_EQUAL_INT(AVERROR(EEXIST), results[i]);
    }
    TEST_ASSERT_EQUAL_INT(1, owners);
}
void test_flush_and_close_failures_are_returned(void) {
    AVIOContext *io = NULL;
    TEST_ASSERT_EQUAL_INT(0, recording_file_open(&io, path));
    avio_write(io, (const unsigned char *)"data", 4);
    write_error = ENOSPC;
    close_error = EIO;
    int result = recording_file_close(&io);
    write_error = close_error = 0;
    TEST_ASSERT_EQUAL_INT(AVERROR(ENOSPC), result);
    TEST_ASSERT_NULL(io);
    unlink(path);
    TEST_ASSERT_EQUAL_INT(0, recording_file_open(&io, path));
    close_error = EIO;
    result = recording_file_close(&io);
    close_error = 0;
    TEST_ASSERT_EQUAL_INT(AVERROR(EIO), result);
    TEST_ASSERT_NULL(io);
}
void test_timestamp_reuse_rollback_and_process_restart_get_fresh_paths(void) {
    config_t config = {0};
    config.record_mp4_directly = true;
    snprintf(config.mp4_storage_path, sizeof(config.mp4_storage_path), "%s", directory);
    strcpy(config.mp4_directory_format, MP4_DIRECTORY_FORMAT_FLAT);
    char paths[3][MAX_PATH_LENGTH];
    for (int i = 0; i < 3; ++i) {
        TEST_ASSERT_EQUAL_INT(0, prepare_mp4_recording_path(&config, "camera",
            i == 2 ? 1699999900 : 1700000000, paths[i], sizeof(paths[i])));
        AVIOContext *io = NULL;
        TEST_ASSERT_EQUAL_INT(0, recording_file_open(&io, paths[i]));
        avio_write(io, (const unsigned char *)"kept", 4);
        TEST_ASSERT_EQUAL_INT(0, recording_file_close(&io));
    }
    TEST_ASSERT_NOT_EQUAL(0, strcmp(paths[0], paths[1]));
    int pipes[2];
    TEST_ASSERT_EQUAL_INT(0, pipe(pipes));
    pid_t child = fork();
    TEST_ASSERT_GREATER_OR_EQUAL(0, child);
    if (child == 0) {
        close(pipes[0]);
        char child_path[MAX_PATH_LENGTH] = {0};
        int result = prepare_mp4_recording_path(&config, "camera", 1700000000,
                                                 child_path, sizeof(child_path));
        if (result == 0 && write(pipes[1], child_path, sizeof(child_path)) != sizeof(child_path)) result = -1;
        close(pipes[1]);
        _exit(result == 0 ? 0 : 1);
    }
    close(pipes[1]);
    char child_path[MAX_PATH_LENGTH] = {0};
    TEST_ASSERT_EQUAL_INT(sizeof(child_path), read(pipes[0], child_path, sizeof(child_path)));
    close(pipes[0]);
    int status;
    TEST_ASSERT_EQUAL_INT(child, waitpid(child, &status, 0));
    TEST_ASSERT_TRUE(WIFEXITED(status));
    TEST_ASSERT_EQUAL_INT(0, WEXITSTATUS(status));
    for (int i = 0; i < 3; ++i) {
        TEST_ASSERT_NOT_EQUAL(0, strcmp(paths[i], child_path));
        FILE *f = fopen(paths[i], "rb");
        char data[5] = {0};
        TEST_ASSERT_NOT_NULL(f);
        TEST_ASSERT_EQUAL_UINT(4, fread(data, 1, 4, f));
        fclose(f);
        TEST_ASSERT_EQUAL_STRING("kept", data);
        unlink(paths[i]);
    }
    char camera_dir[256];
    snprintf(camera_dir, sizeof(camera_dir), "%s/camera", directory);
    rmdir(camera_dir);
}
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_permissions_are_private_even_with_permissive_umask);
    RUN_TEST(test_exclusive_open_preserves_existing_file);
    RUN_TEST(test_symlink_is_not_followed);
    RUN_TEST(test_concurrent_open_has_one_owner);
    RUN_TEST(test_flush_and_close_failures_are_returned);
    RUN_TEST(test_timestamp_reuse_rollback_and_process_restart_get_fresh_paths);
    return UNITY_END();
}
