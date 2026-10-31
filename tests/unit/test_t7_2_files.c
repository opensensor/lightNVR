#define _GNU_SOURCE
#include <dirent.h>
#include <errno.h>
#include <fcntl.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>
#include <sqlite3.h>
#include "unity.h"
#include "core/config.h"
#include "core/daemon.h"
#include "core/path_utils.h"
#include "database/db_backup.h"
#include "database/db_core.h"
#include "storage/storage_manager.h"
#include "video/ffmpeg_utils.h"
#include "video/hls_writer.h"
#include "video/hls/hls_directory.h"
#include "video/hls/hls_context.h"

static char root[128], watched[512], moved[512], replacement[512];
static bool swap_on_chmod, swap_on_open, swap_on_stat, fail_chmod, hook_failed;
static int chmod_fd, owned_fd;
static mode_t saved_umask;
static config_t saved_config;
static hls_stream_ctx_t active;
static hls_stream_ctx_t *saved_context;
static hls_writer_t *writer;
static jpeg_encoder_cache_t *jpeg;
static bool block_worker_creation;
static int blocked_workers;

sqlite3 *__wrap_get_db_handle(void) { return NULL; }
const config_t *__wrap_get_streaming_config(void) { return &g_config; }
/* Keep the public legacy retention entry point synchronous, with no catalog
 * rows or background workers; its directory scan/stat/unlink path remains real. */
int __real_pthread_create(pthread_t *, const pthread_attr_t *, void *(*)(void *), void *);
int __wrap_pthread_create(pthread_t *thread, const pthread_attr_t *attributes,
                          void *(*entry)(void *), void *arg) {
    if (block_worker_creation) { blocked_workers++; errno = EAGAIN; return EAGAIN; }
    return __real_pthread_create(thread, attributes, entry, arg);
}
int __wrap_storage_migration_worker_start(void) { return 0; }
int __wrap_storage_source_worker_start(void) { return 0; }
bool __wrap_storage_target_health_register(const char *a, const char *b, bool probe) {
    (void)a; (void)b; (void)probe; return true;
}
int __wrap_get_all_stream_names_including_disabled(char names[][MAX_STREAM_NAME], int maximum) {
    (void)names; (void)maximum; return 0;
}
int __wrap_get_orphaned_recording_stream_names(char names[][MAX_STREAM_NAME], int maximum) {
    (void)names; (void)maximum; return 0;
}

static int substitute_path(void) {
    if (rename(watched, moved) != 0 || symlink(replacement, watched) != 0) {
        hook_failed = true;
        return -1;
    }
    return 0;
}
int __real_fchmod(int, mode_t);
int __wrap_fchmod(int fd, mode_t mode) {
    chmod_fd = fd;
    if (swap_on_chmod) {
        swap_on_chmod = false;
        if (substitute_path() != 0) return -1;
    }
    if (fail_chmod) { errno = EPERM; return -1; }
    return __real_fchmod(fd, mode);
}
int __real_open(const char *, int, ...);
int __wrap_open(const char *path, int flags, ...) {
    mode_t mode = 0;
    if (flags & O_CREAT) {
        va_list args;
        va_start(args, flags);
        mode = (mode_t)va_arg(args, int);
        va_end(args);
    }
    int fd = __real_open(path, flags, mode);
    if (fd >= 0 && swap_on_open && strcmp(path, watched) == 0) {
        swap_on_open = false;
        if (substitute_path() != 0) { close(fd); return -1; }
    }
    return fd;
}
int __real_fstatat(int, const char *, struct stat *, int);
int __wrap_fstatat(int fd, const char *name, struct stat *st, int flags) {
    int result = __real_fstatat(fd, name, st, flags);
    if (!result && swap_on_stat && strcmp(name, "old.ts") == 0) {
        swap_on_stat = false;
        if (rename(watched, moved) != 0 || mkdir(watched, 0700) != 0) hook_failed = true;
        int replacement_fd = open(replacement, O_WRONLY | O_CREAT | O_EXCL, 0600);
        if (replacement_fd < 0) hook_failed = true;
        else { if (write(replacement_fd, "replacement", 11) != 11) hook_failed = true; close(replacement_fd); }
    }
    return result;
}
static void path(char *out, size_t size, const char *name) {
    int length = snprintf(out, size, "%s/%s", root, name);
    TEST_ASSERT_TRUE(length > 0 && (size_t)length < size);
}
static void make_file(const char *file, const char *bytes, mode_t mode) {
    int fd = open(file, O_WRONLY | O_CREAT | O_TRUNC, mode);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, fd);
    ssize_t count = write(fd, bytes, strlen(bytes));
    close(fd);
    TEST_ASSERT_EQUAL_INT(strlen(bytes), count);
}
static mode_t permissions(const char *file) {
    struct stat st;
    TEST_ASSERT_EQUAL_INT(0, stat(file, &st));
    return st.st_mode & 0777;
}
static bool exists(const char *file) { struct stat st; return lstat(file, &st) == 0; }
static void remove_tree(const char *directory) {
    DIR *dir = opendir(directory);
    if (!dir) return;
    struct dirent *entry;
    while ((entry = readdir(dir))) {
        if (!strcmp(entry->d_name, ".") || !strcmp(entry->d_name, "..")) continue;
        char file[1024];
        snprintf(file, sizeof(file), "%s/%s", directory, entry->d_name);
        struct stat st;
        if (!lstat(file, &st) && S_ISDIR(st.st_mode)) remove_tree(file);
        else unlink(file);
    }
    closedir(dir);
    rmdir(directory);
}
void setUp(void) {
    strcpy(root, "/tmp/lightnvr_t7_files_XXXXXX");
    TEST_ASSERT_NOT_NULL(mkdtemp(root));
    saved_umask = umask(0);
    saved_config = g_config;
    saved_context = streaming_contexts[0];
    memset(&active, 0, sizeof(active));
    strcpy(active.config.name, "camera");
    active.running = true;
    streaming_contexts[0] = &active;
    g_config.max_streams = 1;
    strcpy(g_config.storage_path, root);
    g_config.storage_path_hls[0] = '\0';
    swap_on_chmod = swap_on_open = swap_on_stat = fail_chmod = hook_failed = false;
    owned_fd = chmod_fd = -1;
    writer = NULL;
    jpeg = NULL;
    block_worker_creation = false;
    blocked_workers = 0;
}
void tearDown(void) {
    swap_on_chmod = swap_on_open = swap_on_stat = fail_chmod = false;
    if (owned_fd >= 0) close(owned_fd);
    if (writer) hls_writer_close(writer);
    if (jpeg) jpeg_encoder_cache_destroy(jpeg);
    streaming_contexts[0] = saved_context;
    g_config = saved_config;
    umask(saved_umask);
    remove_tree(root);
}
static void arm_substitution(void) {
    path(watched, sizeof(watched), "owned");
    path(moved, sizeof(moved), "original-inode");
    path(replacement, sizeof(replacement), "outside-sentinel");
    make_file(watched, "original", 0600);
    make_file(replacement, "untouched", 0600);
}
static void test_pid_permissions_follow_owned_descriptor_after_path_substitution(void) {
    arm_substitution();
    swap_on_chmod = true;
    owned_fd = write_pid_file(watched);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, owned_fd);
    TEST_ASSERT_FALSE(hook_failed);
    TEST_ASSERT_FALSE(swap_on_chmod);
    TEST_ASSERT_EQUAL_INT(0644, permissions(moved));
    TEST_ASSERT_EQUAL_INT(0600, permissions(replacement));
    struct stat fd_info, original_info;
    TEST_ASSERT_EQUAL_INT(0, fstat(owned_fd, &fd_info));
    TEST_ASSERT_EQUAL_INT(0, stat(moved, &original_info));
    TEST_ASSERT_EQUAL_UINT64(original_info.st_ino, fd_info.st_ino);
}
static void test_chmod_path_file_directory_and_symlink_controls(void) {
    char file[512], directory[512], link[512];
    path(file, sizeof(file), "file"); path(directory, sizeof(directory), "directory");
    path(link, sizeof(link), "link");
    make_file(file, "payload", 0600);
    TEST_ASSERT_EQUAL_INT(0, mkdir(directory, 0700));
    TEST_ASSERT_EQUAL_INT(0, chmod_path(file, 0640));
    TEST_ASSERT_EQUAL_INT(0, chmod_path(directory, 0750));
    TEST_ASSERT_EQUAL_INT(0640, permissions(file));
    TEST_ASSERT_EQUAL_INT(0750, permissions(directory));
    TEST_ASSERT_EQUAL_INT(0, symlink(file, link));
    TEST_ASSERT_EQUAL_INT(0, chmod_path(link, 0777));
    TEST_ASSERT_EQUAL_INT(0640, permissions(file));
}
static void test_chmod_path_substitution_keeps_descriptor_identity(void) {
    arm_substitution(); swap_on_chmod = true;
    TEST_ASSERT_EQUAL_INT(0, chmod_path(watched, 0640));
    TEST_ASSERT_FALSE(hook_failed);
    TEST_ASSERT_EQUAL_INT(0640, permissions(moved));
    TEST_ASSERT_EQUAL_INT(0600, permissions(replacement));
    TEST_ASSERT_EQUAL_INT(-1, fcntl(chmod_fd, F_GETFD));
    TEST_ASSERT_EQUAL_INT(EBADF, errno);
}
static void test_chmod_failure_releases_open_descriptor(void) {
    arm_substitution(); fail_chmod = true;
    TEST_ASSERT_EQUAL_INT(-1, chmod_path(watched, 0640));
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, chmod_fd);
    TEST_ASSERT_EQUAL_INT(-1, fcntl(chmod_fd, F_GETFD));
    TEST_ASSERT_EQUAL_INT(EBADF, errno);
}
static void test_recursive_chmod_normal_tree(void) {
    char directory[512], file[512];
    path(directory, sizeof(directory), "tree"); path(file, sizeof(file), "tree/file");
    TEST_ASSERT_EQUAL_INT(0, mkdir(directory, 0700));
    make_file(file, "payload", 0600);
    TEST_ASSERT_EQUAL_INT(0, chmod_recursive(directory, 0750));
    TEST_ASSERT_EQUAL_INT(0750, permissions(directory));
    TEST_ASSERT_EQUAL_INT(0750, permissions(file));
}
static void test_recursive_symlink_skip_preserves_unowned_descriptor(void) {
    char sentinel[512], link[512];
    path(sentinel, sizeof(sentinel), "sentinel"); path(link, sizeof(link), "symlink");
    make_file(sentinel, "private", 0600);
    TEST_ASSERT_EQUAL_INT(0, symlink(sentinel, link));
    fflush(NULL);
    pid_t pid = fork();
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, pid);
    if (pid == 0) {
        /* This child's fd0 belongs exclusively to the disposable sentinel.
         * Test-only zero initialization makes the missing fd_out assignment
         * deterministic without affecting the parent's descriptors. */
        int fd = open(sentinel, O_RDONLY);
        if (fd < 0 || dup2(fd, 0) < 0) _exit(20);
        if (fd != 0) close(fd);
        int status = chmod_recursive(link, 0777);
        bool sentinel_open = fcntl(0, F_GETFD) >= 0;
        if (!sentinel_open) _exit(21);
        _exit(status == 0 ? 0 : 22);
    }
    int status = 0;
    TEST_ASSERT_EQUAL_INT(pid, waitpid(pid, &status, 0));
    TEST_ASSERT_TRUE(WIFEXITED(status));
    TEST_ASSERT_EQUAL_INT_MESSAGE(0, WEXITSTATUS(status), "symlink skip consumed an unowned descriptor (21) or failed (22)");
    TEST_ASSERT_EQUAL_INT(0600, permissions(sentinel));
}
static void create_database(const char *file, int value) {
    sqlite3 *db = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_open(file, &db));
    char sql[128];
    snprintf(sql, sizeof(sql), "CREATE TABLE marker(value INTEGER); INSERT INTO marker VALUES(%d);", value);
    int result = sqlite3_exec(db, sql, NULL, NULL, NULL);
    sqlite3_close(db);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, result);
}
static int database_value(const char *file) {
    sqlite3 *db = NULL; sqlite3_stmt *stmt = NULL;
    int value = -1;
    if (sqlite3_open_v2(file, &db, SQLITE_OPEN_READONLY, NULL) == SQLITE_OK &&
        sqlite3_prepare_v2(db, "SELECT value FROM marker", -1, &stmt, NULL) == SQLITE_OK &&
        sqlite3_step(stmt) == SQLITE_ROW) value = sqlite3_column_int(stmt, 0);
    sqlite3_finalize(stmt); sqlite3_close(db);
    return value;
}
static void test_restore_reads_open_source_descriptor_after_path_substitution(void) {
    char destination[512];
    path(watched, sizeof(watched), "backup.sqlite"); path(moved, sizeof(moved), "held.sqlite");
    path(replacement, sizeof(replacement), "replacement.sqlite"); path(destination, sizeof(destination), "destination.sqlite");
    create_database(watched, 11); create_database(replacement, 99); create_database(destination, 22);
    swap_on_open = true;
    TEST_ASSERT_EQUAL_INT(0, restore_database_from_backup(watched, destination));
    TEST_ASSERT_FALSE(hook_failed); TEST_ASSERT_FALSE(swap_on_open);
    TEST_ASSERT_EQUAL_INT(11, database_value(destination));
    TEST_ASSERT_EQUAL_INT(99, database_value(replacement));
}
static void test_missing_corrupt_restore_preserves_existing_database(void) {
    char source[512], destination[512];
    path(source, sizeof(source), "backup.sqlite"); path(destination, sizeof(destination), "destination.sqlite");
    create_database(destination, 22);
    TEST_ASSERT_EQUAL_INT(-1, restore_database_from_backup(source, destination));
    TEST_ASSERT_EQUAL_INT(22, database_value(destination));
    make_file(source, "not a sqlite database", 0600);
    TEST_ASSERT_EQUAL_INT(-1, restore_database_from_backup(source, destination));
    TEST_ASSERT_EQUAL_INT(22, database_value(destination));
}
static void test_delete_recording_unlinks_symlink_without_touching_target(void) {
    char file[512], link[512];
    path(file, sizeof(file), "recording"); path(link, sizeof(link), "link.mp4");
    make_file(file, "payload", 0600); TEST_ASSERT_EQUAL_INT(0, symlink(file, link));
    TEST_ASSERT_EQUAL_INT(0, delete_recording(link));
    TEST_ASSERT_TRUE(exists(file)); TEST_ASSERT_FALSE(exists(link));
    TEST_ASSERT_EQUAL_INT(0, delete_recording(file));
    TEST_ASSERT_FALSE(exists(file));
    TEST_ASSERT_EQUAL_INT(-1, delete_recording(file));
}
static void test_jpeg_new_files_have_restricted_creation_permissions(void) {
    char direct[512], cached[512];
    unsigned char pixels[32 * 32 * 3] = {0};
    path(direct, sizeof(direct), "direct.jpg"); path(cached, sizeof(cached), "cached.jpg");
    TEST_ASSERT_EQUAL_INT(0, ffmpeg_encode_jpeg(pixels, 32, 32, 3, 80, direct));
    jpeg = jpeg_encoder_cache_create(32, 32, 3, 80);
    TEST_ASSERT_NOT_NULL(jpeg);
    TEST_ASSERT_EQUAL_INT(0, jpeg_encoder_cache_encode(jpeg, pixels, cached));
    TEST_ASSERT_EQUAL_INT(0640, permissions(direct));
    TEST_ASSERT_EQUAL_INT(0640, permissions(cached));
}
static void test_hls_directory_and_writer_use_common_permission_helper(void) {
    char directory[512];
    TEST_ASSERT_EQUAL_INT(0, ensure_hls_directory(directory, sizeof(directory), "camera"));
    TEST_ASSERT_EQUAL_INT(0755, permissions(directory));
    writer = hls_writer_create(directory, "camera", 2);
    TEST_ASSERT_NOT_NULL(writer);
    TEST_ASSERT_EQUAL_STRING(directory, writer->output_dir);
    TEST_ASSERT_EQUAL_INT(0755, permissions(writer->output_dir));
}
static void old_file(const char *file) {
    make_file(file, "segment", 0600);
    struct timespec times[2] = {{time(NULL) - 600, 0}, {time(NULL) - 600, 0}};
    TEST_ASSERT_EQUAL_INT(0, utimensat(AT_FDCWD, file, times, 0));
}
static void test_active_hls_removes_old_regular_ts_m4s_but_preserves_symlinks(void) {
    char directory[512], ts[512], m4s[512], link[512], victim[512], recent[512];
    TEST_ASSERT_EQUAL_INT(0, ensure_hls_directory(directory, sizeof(directory), "camera"));
    path(ts, sizeof(ts), "hls/camera/old.ts"); path(m4s, sizeof(m4s), "hls/camera/old.m4s");
    path(link, sizeof(link), "hls/camera/link.ts"); path(victim, sizeof(victim), "sentinel");
    path(recent, sizeof(recent), "hls/camera/recent.ts");
    old_file(ts); old_file(m4s); old_file(victim); make_file(recent, "recent", 0600);
    TEST_ASSERT_EQUAL_INT(0, symlink(victim, link));
    cleanup_hls_directories();
    TEST_ASSERT_FALSE(exists(ts)); TEST_ASSERT_FALSE(exists(m4s));
    TEST_ASSERT_TRUE(exists(link)); TEST_ASSERT_TRUE(exists(victim)); TEST_ASSERT_TRUE(exists(recent));
}
static void test_hls_unlinkat_stays_on_open_directory_after_parent_rename(void) {
    char original_segment[512];
    TEST_ASSERT_EQUAL_INT(0, ensure_hls_directory(watched, sizeof(watched), "camera"));
    path(moved, sizeof(moved), "moved-camera"); path(replacement, sizeof(replacement), "hls/camera/old.ts");
    path(original_segment, sizeof(original_segment), "moved-camera/old.ts");
    old_file(replacement); swap_on_stat = true;
    cleanup_hls_directories();
    TEST_ASSERT_FALSE(hook_failed); TEST_ASSERT_FALSE(swap_on_stat);
    TEST_ASSERT_FALSE(exists(original_segment)); TEST_ASSERT_TRUE(exists(replacement));
}
static void test_legacy_retention_removes_old_regular_file_but_skips_symlink(void) {
    char directory[512], old[512], link[512], target[512], recent[512];
    path(directory, sizeof(directory), "legacy");
    path(old, sizeof(old), "legacy/old.mp4"); path(link, sizeof(link), "legacy/link.mp4");
    path(target, sizeof(target), "outside-sentinel"); path(recent, sizeof(recent), "legacy/recent.mp4");
    TEST_ASSERT_EQUAL_INT(0, mkdir(directory, 0700));
    make_file(old, "old recording", 0600); make_file(target, "untouched", 0600);
    make_file(recent, "recent recording", 0600);
    struct timespec times[2] = {{time(NULL) - 172800, 0}, {time(NULL) - 172800, 0}};
    TEST_ASSERT_EQUAL_INT(0, utimensat(AT_FDCWD, old, times, 0));
    TEST_ASSERT_EQUAL_INT(0, utimensat(AT_FDCWD, target, times, 0));
    TEST_ASSERT_EQUAL_INT(0, symlink(target, link));
    block_worker_creation = true;
    int initialized = init_storage_manager(root, 0);
    block_worker_creation = false;
    TEST_ASSERT_EQUAL_INT(0, initialized);
    TEST_ASSERT_EQUAL_INT(1, blocked_workers);
    TEST_ASSERT_EQUAL_INT(0, set_retention_days(1));
    TEST_ASSERT_EQUAL_INT(1, apply_retention_policy());
    TEST_ASSERT_FALSE(exists(old)); TEST_ASSERT_TRUE(exists(link));
    TEST_ASSERT_TRUE(exists(target)); TEST_ASSERT_TRUE(exists(recent));
    TEST_ASSERT_EQUAL_INT(0, set_retention_days(0));
}
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_pid_permissions_follow_owned_descriptor_after_path_substitution);
    RUN_TEST(test_chmod_path_file_directory_and_symlink_controls);
    RUN_TEST(test_chmod_path_substitution_keeps_descriptor_identity);
    RUN_TEST(test_chmod_failure_releases_open_descriptor);
    RUN_TEST(test_recursive_chmod_normal_tree);
    RUN_TEST(test_recursive_symlink_skip_preserves_unowned_descriptor);
    RUN_TEST(test_restore_reads_open_source_descriptor_after_path_substitution);
    RUN_TEST(test_missing_corrupt_restore_preserves_existing_database);
    RUN_TEST(test_delete_recording_unlinks_symlink_without_touching_target);
    RUN_TEST(test_jpeg_new_files_have_restricted_creation_permissions);
    RUN_TEST(test_hls_directory_and_writer_use_common_permission_helper);
    RUN_TEST(test_active_hls_removes_old_regular_ts_m4s_but_preserves_symlinks);
    RUN_TEST(test_hls_unlinkat_stays_on_open_directory_after_parent_rename);
    RUN_TEST(test_legacy_retention_removes_old_regular_file_but_skips_symlink);
    return UNITY_END();
}
