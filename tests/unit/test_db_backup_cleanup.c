#define _GNU_SOURCE
#define _POSIX_C_SOURCE 200809L
/* Actual private cleanup helper, with public DB definitions linked from here. */
#include "../../src/database/db_core.c"
#include <sys/syscall.h>
#include "unity.h"

static char root[128], backup_dir[256], moved_dir[256], artifact[320], sentinel[256];
static bool armed, replaced, leaf_swap, regular_swap, fixture_config_loaded;
static sqlite3 *fixture_db;
static config_t saved_config;
static struct stat original_identity;
static bool held_inode_verified, fail_candidate_open, fail_private_open, fail_mkdir;
static bool fail_move, fail_delete, restore_collision, change_after_move;
static int restore_error, move_error, private_stat_fault, private_fd;
static unsigned restore_calls, mkdir_calls;
static bool fail_uuid, collide_mkdir, repeat_restored, repeated, substitute_private_directory;
struct dirent *__real_readdir(DIR *);
struct dirent *__wrap_readdir(DIR *dir) {
    if (repeat_restored && restore_calls && !repeated) {
        static struct dirent duplicate;
        memset(&duplicate, 0, sizeof(duplicate));
        strcpy(duplicate.d_name, "20261031T010000Z.sqlite3.tmp");
        repeated = true;
        return &duplicate;
    }
    return __real_readdir(dir);
}
int __real_lightnvr_uuid_generate_v4(char *);
int __wrap_lightnvr_uuid_generate_v4(char *uuid) {
    return fail_uuid ? -1 : __real_lightnvr_uuid_generate_v4(uuid);
}
static int owned_fds[32];
static unsigned owned_fd_count;
static const char artifact_name[] = "20261031T010000Z.sqlite3.tmp";
static void put_file(const char *path, const char *bytes) {
    FILE *file = fopen(path, "w"); TEST_ASSERT_NOT_NULL(file);
    TEST_ASSERT_EQUAL_UINT(strlen(bytes), fwrite(bytes, 1, strlen(bytes), file));
    TEST_ASSERT_EQUAL_INT(0, fclose(file));
}
static void replace_after_check(void) {
    armed = false; replaced = true;
    if (leaf_swap) {
        TEST_ASSERT_EQUAL_INT(0, unlink(artifact));
        if (regular_swap) put_file(artifact, "new regular file must survive");
        else TEST_ASSERT_EQUAL_INT(0, symlink(sentinel, artifact));
    } else {
        TEST_ASSERT_EQUAL_INT(0, rename(backup_dir, moved_dir));
        TEST_ASSERT_EQUAL_INT(0, mkdir(backup_dir, 0700));
        put_file(artifact, "replacement must survive");
    }
}
int __real_openat(int, const char *, int, ...);
int __wrap_openat(int fd, const char *name, int flags, ...) {
    mode_t mode = 0;
    if (flags & O_CREAT) { va_list args; va_start(args, flags); mode = va_arg(args, int); va_end(args); }
    bool candidate = strcmp(name, artifact_name) == 0;
    bool private = strncmp(name, ".cleanup-", 9) == 0;
    if ((candidate && fail_candidate_open) || (private && fail_private_open)) { errno = EACCES; return -1; }
    int result = __real_openat(fd, name, flags, mode);
    if (result >= 0) {
        TEST_ASSERT_LESS_THAN_UINT(32, owned_fd_count);
        owned_fds[owned_fd_count++] = result;
        if (private) private_fd = result;
    }
    return result;
}
int __real_close(int);
int __wrap_close(int fd) {
    int result = __real_close(fd);
    for (unsigned i = 0; i < owned_fd_count; i++) if (owned_fds[i] == fd) {
        owned_fds[i] = owned_fds[--owned_fd_count]; break;
    }
    return result;
}
int __real_fstat(int, struct stat *);
int __wrap_fstat(int fd, struct stat *info) {
    int result = __real_fstat(fd, info);
    if (result == 0 && armed && info->st_dev == original_identity.st_dev &&
            info->st_ino == original_identity.st_ino && S_ISREG(info->st_mode)) {
        held_inode_verified = true;
        replace_after_check();
        struct stat pinned;
        TEST_ASSERT_EQUAL_INT(0, __real_fstat(fd, &pinned));
        TEST_ASSERT_EQUAL_UINT64(original_identity.st_ino, pinned.st_ino);
    }
    if (result == 0 && fd == private_fd && S_ISDIR(info->st_mode)) {
        if (private_stat_fault == 1) info->st_uid = geteuid() + 1;
        if (private_stat_fault == 2) info->st_mode |= 0070;
        if (private_stat_fault == 3) info->st_ino++;
    }
    return result;
}
int __real_mkdirat(int, const char *, mode_t);
int __wrap_mkdirat(int fd, const char *name, mode_t mode) {
    if (strncmp(name, ".cleanup-", 9) == 0) {
        mkdir_calls++;
        if (fail_mkdir || collide_mkdir) { errno = collide_mkdir ? EEXIST : EACCES; return -1; }
    }
    int result = __real_mkdirat(fd, name, mode);
    if (result == 0 && substitute_private_directory && strncmp(name, ".cleanup-", 9) == 0) {
        char prepared[256]; snprintf(prepared, sizeof(prepared), "%s/prepared-private", root);
        TEST_ASSERT_EQUAL_INT(0, renameat(fd, name, fd, "saved-created-directory"));
        TEST_ASSERT_EQUAL_INT(0, renameat(AT_FDCWD, prepared, fd, name));
    }
    return result;
}
long __real_syscall(long, ...);
long __wrap_syscall(long number, ...) {
    if (number != SYS_renameat2) {
        TEST_FAIL_MESSAGE("Unexpected syscall in bounded renameat2 fault seam");
        errno = ENOSYS; return -1;
    }
    va_list args; va_start(args, number);
    int oldfd = va_arg(args, int);
    const char *oldname = va_arg(args, const char *);
    int newfd = va_arg(args, int);
    const char *newname = va_arg(args, const char *);
    unsigned int flags = va_arg(args, unsigned int);
    va_end(args);
    TEST_ASSERT_EQUAL_UINT(RENAME_NOREPLACE, flags);
    if (newfd == private_fd) {
        if (fail_move || move_error) { errno = move_error ? move_error : EIO; return -1; }
        long result = __real_syscall(number, oldfd, oldname, newfd, newname, flags);
        if (result == 0) {
            if (restore_collision) put_file(artifact, "concurrent destination must survive");
            if (change_after_move) {
                int fd = __real_openat(newfd, newname, O_WRONLY | O_APPEND);
                TEST_ASSERT_GREATER_OR_EQUAL_INT(0, fd);
                TEST_ASSERT_EQUAL_INT(8, write(fd, " changed", 8));
                TEST_ASSERT_EQUAL_INT(0, close(fd));
            }
        }
        return result;
    }
    restore_calls++;
    if (restore_error) { errno = restore_error; return -1; }
    return __real_syscall(number, oldfd, oldname, newfd, newname, flags);
}
int __real_unlinkat(int, const char *, int);
int __wrap_unlinkat(int fd, const char *name, int flags) {
    if (fail_delete && flags == 0 && strcmp(name, artifact_name) == 0) { errno = EACCES; return -1; }
    return __real_unlinkat(fd, name, flags);
}
int __real_lstat(const char *, struct stat *);
int __wrap_lstat(const char *path, struct stat *info) {
    int result = __real_lstat(path, info);
    if (armed && result == 0 && strcmp(path, artifact) == 0) replace_after_check();
    return result;
}
int __real_fstatat(int, const char *, struct stat *, int);
int __wrap_fstatat(int fd, const char *name, struct stat *info, int flags) {
    int result = __real_fstatat(fd, name, info, flags);
    if (armed && result == 0 && strcmp(name, artifact_name) == 0) replace_after_check();
    return result;
}
static void assert_bytes(const char *path, const char *expected) {
    FILE *file = fopen(path, "r"); TEST_ASSERT_NOT_NULL(file);
    char actual[128] = {0}; size_t bytes = fread(actual, 1, sizeof(actual)-1, file);
    TEST_ASSERT_EQUAL_INT(0, fclose(file));
    TEST_ASSERT_EQUAL_UINT(strlen(expected), bytes); TEST_ASSERT_EQUAL_STRING(expected, actual);
}
static void remove_owned_tree(const char *directory) {
    DIR *dir = opendir(directory); if (!dir) return;
    struct dirent *entry;
    while ((entry = readdir(dir))) {
        if (!strcmp(entry->d_name, ".") || !strcmp(entry->d_name, "..")) continue;
        char path[512]; snprintf(path, sizeof(path), "%s/%s", directory, entry->d_name);
        struct stat info;
        if (lstat(path, &info) == 0 && S_ISDIR(info.st_mode)) remove_owned_tree(path);
        else unlink(path);
    }
    closedir(dir); rmdir(directory);
}
void setUp(void) {
    saved_config = g_config; fixture_db = NULL; fixture_config_loaded = false;
    strcpy(root, "/tmp/lightnvr-db-backup-cleanup-XXXXXX"); TEST_ASSERT_NOT_NULL(mkdtemp(root));
    snprintf(backup_dir, sizeof(backup_dir), "%s/backup", root);
    snprintf(moved_dir, sizeof(moved_dir), "%s/inspected", root);
    snprintf(artifact, sizeof(artifact), "%s/%s", backup_dir, artifact_name);
    snprintf(sentinel, sizeof(sentinel), "%s/sentinel", root);
    TEST_ASSERT_EQUAL_INT(0, mkdir(backup_dir, 0700));
    put_file(artifact, "stale partial backup"); put_file(sentinel, "sentinel unchanged");
    armed = replaced = leaf_swap = regular_swap = false;
    held_inode_verified = fail_candidate_open = fail_private_open = fail_mkdir = false;
    fail_move = fail_delete = restore_collision = change_after_move = false;
    restore_error = move_error = private_stat_fault = 0; private_fd = -1;
    restore_calls = owned_fd_count = mkdir_calls = 0;
    fail_uuid = collide_mkdir = repeat_restored = repeated = substitute_private_directory = false;
    TEST_ASSERT_EQUAL_INT(0, stat(artifact, &original_identity));
}
void tearDown(void) {
    armed = repeat_restored = false;
    if (get_db_handle()) shutdown_database();
    if (fixture_db) { sqlite3_close_v2(fixture_db); fixture_db = NULL; }
    if (fixture_config_loaded) free(g_config.streams);
    g_config = saved_config;
    remove_owned_tree(root);
    TEST_ASSERT_EQUAL_INT(-1, access(root, F_OK));
    TEST_ASSERT_EQUAL_UINT(0, owned_fd_count);
}
static void test_normal_stale_cleanup_preserves_completed_unrelated_and_symlinks(void) {
    char complete[320], other[320], link[320], directory[320];
    snprintf(complete, sizeof(complete), "%s/complete.sqlite3", backup_dir);
    snprintf(other, sizeof(other), "%s/operator-notes.tmp", backup_dir);
    snprintf(link, sizeof(link), "%s/link.sqlite3.tmp", backup_dir);
    snprintf(directory, sizeof(directory), "%s/directory.sqlite3.tmp", backup_dir);
    put_file(complete, "complete"); put_file(other, "notes");
    TEST_ASSERT_EQUAL_INT(0, symlink(sentinel, link));
    TEST_ASSERT_EQUAL_INT(0, mkdir(directory, 0700));
    TEST_ASSERT_EQUAL_INT(0, cleanup_stale_backup_artifacts(backup_dir));
    TEST_ASSERT_EQUAL_INT(-1, access(artifact, F_OK));
    assert_bytes(complete, "complete"); assert_bytes(other, "notes");
    struct stat info; TEST_ASSERT_EQUAL_INT(0, lstat(link, &info)); TEST_ASSERT_TRUE(S_ISLNK(info.st_mode));
    TEST_ASSERT_EQUAL_INT(0, lstat(directory, &info)); TEST_ASSERT_TRUE(S_ISDIR(info.st_mode));
    assert_bytes(sentinel, "sentinel unchanged");
}
static void test_parent_replacement_cannot_redirect_stale_cleanup(void) {
    armed = true;
    TEST_ASSERT_EQUAL_INT(0, cleanup_stale_backup_artifacts(backup_dir));
    TEST_ASSERT_TRUE(replaced);
    assert_bytes(artifact, "replacement must survive");
    char old_artifact[320]; snprintf(old_artifact, sizeof(old_artifact), "%s/%s", moved_dir, artifact_name);
    TEST_ASSERT_EQUAL_INT(-1, access(old_artifact, F_OK));
    assert_bytes(sentinel, "sentinel unchanged");
}
static void test_leaf_symlink_replacement_does_not_remove_unvalidated_entry(void) {
    armed = leaf_swap = true;
    (void)cleanup_stale_backup_artifacts(backup_dir);
    TEST_ASSERT_TRUE(replaced);
    assert_bytes(sentinel, "sentinel unchanged");
    struct stat info;
    TEST_ASSERT_EQUAL_INT(0, lstat(artifact, &info));
    TEST_ASSERT_TRUE(S_ISLNK(info.st_mode));
}
static void test_missing_directory_is_noop(void) {
    char missing[256]; snprintf(missing, sizeof(missing), "%s/missing", root);
    TEST_ASSERT_EQUAL_INT(0, cleanup_stale_backup_artifacts(missing));
}
static void test_leaf_regular_replacement_preserves_new_contents(void) {
    armed = leaf_swap = regular_swap = true;
    (void)cleanup_stale_backup_artifacts(backup_dir);
    TEST_ASSERT_TRUE(replaced);
    assert_bytes(artifact, "new regular file must survive");
}
static void test_non_directory_returns_error(void) {
    TEST_ASSERT_EQUAL_INT(-1, cleanup_stale_backup_artifacts(sentinel));
    assert_bytes(sentinel, "sentinel unchanged");
}
static void test_readonly_open_backup_restore_preserve_database_contents(void) {
    char source[256], copy[256], restored[256], associated_backups[300];
    snprintf(source, sizeof(source), "%s/database.sqlite", root);
    snprintf(copy, sizeof(copy), "%s/copy.sqlite", root);
    snprintf(restored, sizeof(restored), "%s/restored.sqlite", root);
    snprintf(associated_backups, sizeof(associated_backups), "%s.backups", source);
    TEST_ASSERT_EQUAL_INT(0, rename(backup_dir, associated_backups));
    safe_strcpy(backup_dir, associated_backups, sizeof(backup_dir), 0);
    snprintf(artifact, sizeof(artifact), "%s/%s", backup_dir, artifact_name);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_open(source, &fixture_db));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(fixture_db,
        "CREATE TABLE sample(value TEXT); INSERT INTO sample VALUES('preserved');", NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_close(fixture_db)); fixture_db = NULL;
    memset(&g_config, 0, sizeof(g_config));
    fixture_config_loaded = true;
    load_default_config(&g_config);
    TEST_ASSERT_EQUAL_INT(0, init_database_ex(source, DB_INIT_READ_ONLY));
    TEST_ASSERT_EQUAL_INT(1, sqlite3_db_readonly(get_db_handle(), "main"));
    assert_bytes(artifact, "stale partial backup");
    shutdown_database();
    TEST_ASSERT_EQUAL_INT(0, backup_database(source, copy, false));
    TEST_ASSERT_EQUAL_INT(0, restore_database_from_backup(copy, restored));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_open_v2(restored, &fixture_db, SQLITE_OPEN_READONLY, NULL));
    sqlite3_stmt *statement = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(fixture_db, "SELECT value FROM sample", -1, &statement, NULL));
    int stepped = sqlite3_step(statement);
    bool preserved = stepped == SQLITE_ROW && strcmp((const char *)sqlite3_column_text(statement, 0), "preserved") == 0;
    sqlite3_finalize(statement);
    TEST_ASSERT_TRUE(preserved);
}
static unsigned quarantine_count(char *retained, size_t length) {
    DIR *dir = opendir(backup_dir); TEST_ASSERT_NOT_NULL(dir);
    unsigned count = 0; struct dirent *entry;
    while ((entry = readdir(dir))) if (strncmp(entry->d_name, ".cleanup-", 9) == 0) {
        count++;
        if (retained) snprintf(retained, length, "%s/%s/%s", backup_dir, entry->d_name, artifact_name);
    }
    closedir(dir); return count;
}
static void assert_clean_failure(void) {
    TEST_ASSERT_EQUAL_INT(-1, cleanup_stale_backup_artifacts(backup_dir));
    assert_bytes(artifact, "stale partial backup");
    TEST_ASSERT_EQUAL_UINT(0, quarantine_count(NULL, 0));
}
static void test_candidate_open_failure_preserves_artifact(void) { fail_candidate_open = true; assert_clean_failure(); }
static void test_private_directory_open_failure_preserves_artifact(void) { fail_private_open = true; assert_clean_failure(); }
static void test_private_directory_create_failure_preserves_artifact(void) { fail_mkdir = true; assert_clean_failure(); }
static void test_move_failure_preserves_artifact(void) { fail_move = true; assert_clean_failure(); }
static void test_unlink_failure_restores_without_overwrite(void) {
    fail_delete = true; assert_clean_failure(); TEST_ASSERT_EQUAL_UINT(1, restore_calls);
}
static void test_private_directory_wrong_owner_is_rejected(void) { private_stat_fault = 1; assert_clean_failure(); }
static void test_private_directory_wrong_mode_is_rejected(void) { private_stat_fault = 2; assert_clean_failure(); }
static void test_private_directory_wrong_inode_is_rejected(void) { private_stat_fault = 3; assert_clean_failure(); }
static void retained_restore(int error, bool collision) {
    armed = leaf_swap = regular_swap = true;
    restore_error = error; restore_collision = collision;
    TEST_ASSERT_EQUAL_INT(-1, cleanup_stale_backup_artifacts(backup_dir));
    TEST_ASSERT_TRUE(held_inode_verified);
    TEST_ASSERT_EQUAL_UINT(1, restore_calls);
    char retained[640]; TEST_ASSERT_EQUAL_UINT(1, quarantine_count(retained, sizeof(retained)));
    assert_bytes(retained, "new regular file must survive");
    if (collision) assert_bytes(artifact, "concurrent destination must survive");
    else TEST_ASSERT_EQUAL_INT(-1, access(artifact, F_OK));
}
static void test_restore_collision_preserves_both_objects(void) { retained_restore(0, true); }
static void test_unsupported_restore_retains_quarantine(void) { retained_restore(ENOSYS, false); }
static void test_restore_error_retains_quarantine(void) { retained_restore(EIO, false); }
static void test_changed_size_or_mtime_is_restored(void) {
    change_after_move = true;
    TEST_ASSERT_EQUAL_INT(-1, cleanup_stale_backup_artifacts(backup_dir));
    assert_bytes(artifact, "stale partial backup changed");
    TEST_ASSERT_EQUAL_UINT(1, restore_calls); TEST_ASSERT_EQUAL_UINT(0, quarantine_count(NULL, 0));
}
static void test_crash_leftover_is_not_recursively_deleted(void) {
    char private[320], retained[400];
    snprintf(private, sizeof(private), "%s/.cleanup-leftover", backup_dir);
    snprintf(retained, sizeof(retained), "%s/%s", private, artifact_name);
    TEST_ASSERT_EQUAL_INT(0, mkdir(private, 0700)); put_file(retained, "retained crash artifact");
    TEST_ASSERT_EQUAL_INT(0, cleanup_stale_backup_artifacts(backup_dir));
    assert_bytes(retained, "retained crash artifact"); TEST_ASSERT_EQUAL_UINT(1, quarantine_count(NULL, 0));
}
static void test_fifo_artifact_is_skipped_without_blocking(void) {
    char fifo[320]; snprintf(fifo, sizeof(fifo), "%s/fifo.sqlite3.tmp", backup_dir);
    TEST_ASSERT_EQUAL_INT(0, mkfifo(fifo, 0600));
    TEST_ASSERT_EQUAL_INT(0, cleanup_stale_backup_artifacts(backup_dir));
    struct stat info; TEST_ASSERT_EQUAL_INT(0, lstat(fifo, &info)); TEST_ASSERT_TRUE(S_ISFIFO(info.st_mode));
}
static void test_candidate_descriptor_pins_inode_until_replacement_is_preserved(void) {
    armed = leaf_swap = regular_swap = true;
    (void)cleanup_stale_backup_artifacts(backup_dir);
    TEST_ASSERT_TRUE(held_inode_verified);
    assert_bytes(artifact, "new regular file must survive");
}
static void test_restored_changed_entry_is_not_revisited_in_same_pass(void) {
    armed = leaf_swap = regular_swap = repeat_restored = true;
    int result = cleanup_stale_backup_artifacts(backup_dir);
    repeat_restored = false;
    TEST_ASSERT_EQUAL_INT(-1, result);
    assert_bytes(artifact, "new regular file must survive");
    TEST_ASSERT_EQUAL_UINT(1, restore_calls);
}
static void test_uuid_failure_preserves_artifact(void) {
    fail_uuid = true; assert_clean_failure(); TEST_ASSERT_EQUAL_UINT(0, mkdir_calls);
}
static void test_directory_name_collisions_have_bounded_retries(void) {
    collide_mkdir = true; assert_clean_failure(); TEST_ASSERT_EQUAL_UINT(4, mkdir_calls);
}
static void test_unsupported_filesystem_restore_retains_quarantine(void) { retained_restore(EINVAL, false); }
static void test_setgid_parent_still_allows_private_cleanup(void) {
    TEST_ASSERT_EQUAL_INT(0, chmod(backup_dir, 02700));
    TEST_ASSERT_EQUAL_INT(0, cleanup_stale_backup_artifacts(backup_dir));
    TEST_ASSERT_EQUAL_INT(-1, access(artifact, F_OK));
    TEST_ASSERT_EQUAL_UINT(0, quarantine_count(NULL, 0));
}
static void test_populated_private_directory_substitution_never_overwrites_data(void) {
    char prepared[256], unrelated[320], retained[640];
    snprintf(prepared, sizeof(prepared), "%s/prepared-private", root);
    snprintf(unrelated, sizeof(unrelated), "%s/%s", prepared, artifact_name);
    TEST_ASSERT_EQUAL_INT(0, mkdir(prepared, 0700));
    put_file(unrelated, "unrelated private data");
    substitute_private_directory = true;
    TEST_ASSERT_EQUAL_INT(-1, cleanup_stale_backup_artifacts(backup_dir));
    assert_bytes(artifact, "stale partial backup");
    TEST_ASSERT_EQUAL_UINT(1, quarantine_count(retained, sizeof(retained)));
    assert_bytes(retained, "unrelated private data");
}
static void test_unsupported_initial_move_preserves_original_and_cleans_empty_directory(void) {
    move_error = ENOSYS; assert_clean_failure(); TEST_ASSERT_EQUAL_UINT(0, restore_calls);
}
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_normal_stale_cleanup_preserves_completed_unrelated_and_symlinks);
    RUN_TEST(test_parent_replacement_cannot_redirect_stale_cleanup);
    RUN_TEST(test_leaf_symlink_replacement_does_not_remove_unvalidated_entry);
    RUN_TEST(test_leaf_regular_replacement_preserves_new_contents);
    RUN_TEST(test_missing_directory_is_noop);
    RUN_TEST(test_non_directory_returns_error);
    RUN_TEST(test_readonly_open_backup_restore_preserve_database_contents);
    RUN_TEST(test_candidate_open_failure_preserves_artifact);
    RUN_TEST(test_private_directory_open_failure_preserves_artifact);
    RUN_TEST(test_private_directory_create_failure_preserves_artifact);
    RUN_TEST(test_move_failure_preserves_artifact);
    RUN_TEST(test_unlink_failure_restores_without_overwrite);
    RUN_TEST(test_private_directory_wrong_owner_is_rejected);
    RUN_TEST(test_private_directory_wrong_mode_is_rejected);
    RUN_TEST(test_private_directory_wrong_inode_is_rejected);
    RUN_TEST(test_restore_collision_preserves_both_objects);
    RUN_TEST(test_unsupported_restore_retains_quarantine);
    RUN_TEST(test_restore_error_retains_quarantine);
    RUN_TEST(test_changed_size_or_mtime_is_restored);
    RUN_TEST(test_crash_leftover_is_not_recursively_deleted);
    RUN_TEST(test_fifo_artifact_is_skipped_without_blocking);
    RUN_TEST(test_candidate_descriptor_pins_inode_until_replacement_is_preserved);
    RUN_TEST(test_restored_changed_entry_is_not_revisited_in_same_pass);
    RUN_TEST(test_uuid_failure_preserves_artifact);
    RUN_TEST(test_directory_name_collisions_have_bounded_retries);
    RUN_TEST(test_unsupported_filesystem_restore_retains_quarantine);
    RUN_TEST(test_setgid_parent_still_allows_private_cleanup);
    RUN_TEST(test_populated_private_directory_substitution_never_overwrites_data);
    RUN_TEST(test_unsupported_initial_move_preserves_original_and_cleans_empty_directory);
    return UNITY_END();
}
