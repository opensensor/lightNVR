#define _POSIX_C_SOURCE 200809L
#include "unity.h"
#include "core/config.h"
#include "database/db_core.h"
#include <dirent.h>
#include <errno.h>
#include <fcntl.h>
#include <sqlite3.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <unistd.h>

static char root[128], source[256], destination[256];
static bool fail_rename, fail_sync, fail_old_sync, fail_capture, published, sidecars_at_publish;
static int crash_at_publish, publish_attempts;
typedef enum {
    RECOVERY_SYNC_NONE,
    RECOVERY_SYNC_DATABASE,
    RECOVERY_SYNC_WAL,
    RECOVERY_SYNC_MARKER,
    RECOVERY_SYNC_DIRECTORY,
    RECOVERY_SYNC_PARENT
} recovery_sync_point_t;
static recovery_sync_point_t fail_recovery_sync;
static int recovery_sync_failures;
static bool recovery_directory_synced;

int __real_rename(const char *, const char *);
int __real_fsync(int);
ssize_t __real_write(int, const void *, size_t);

ssize_t __wrap_write(int fd, const void *data, size_t size) {
    if (fail_capture) {
        char link[64], path[512];
        snprintf(link, sizeof(link), "/proc/self/fd/%d", fd);
        ssize_t length = readlink(link, path, sizeof(path) - 1);
        if (length >= 0) {
            path[length] = '\0';
            if (strstr(path, root) && strstr(path, ".recovery-")) { errno = ENOSPC; return -1; }
        }
    }
    return __real_write(fd, data, size);
}

static bool sidecars_exist(void) {
    const char *suffixes[] = {"-wal", "-shm", "-journal"};
    char path[300];
    for (size_t i = 0; i < sizeof(suffixes) / sizeof(suffixes[0]); ++i) {
        snprintf(path, sizeof(path), "%s%s", destination, suffixes[i]);
        if (access(path, F_OK) == 0) return true;
    }
    return false;
}

int __wrap_rename(const char *from, const char *to) {
    if (strcmp(to, destination)) return __real_rename(from, to);
    ++publish_attempts;
    sidecars_at_publish = sidecars_exist();
    if (crash_at_publish == 1) _exit(71);
    if (fail_rename) { errno = EIO; return -1; }
    int result = __real_rename(from, to);
    if (!result) published = true;
    if (!result && crash_at_publish == 2) _exit(72);
    return result;
}

/* Identify the actual recovery descriptor, never the original database or
 * staging replacement. The parent fault is armed only after the capture
 * directory has been synced, so an earlier parent sync cannot satisfy it. */
static recovery_sync_point_t recovery_sync_point(int fd) {
    char link[64], path[512], prefix[300];
    snprintf(link, sizeof(link), "/proc/self/fd/%d", fd);
    ssize_t length = readlink(link, path, sizeof(path) - 1);
    if (length < 0) return RECOVERY_SYNC_NONE;
    path[length] = '\0';
    if (recovery_directory_synced && !strcmp(path, root)) return RECOVERY_SYNC_PARENT;
    snprintf(prefix, sizeof(prefix), "%s.recovery-", destination);
    if (strncmp(path, prefix, strlen(prefix))) return RECOVERY_SYNC_NONE;
    const char *entry = strchr(path + strlen(prefix), '/');
    if (!entry) return RECOVERY_SYNC_DIRECTORY;
    if (!strcmp(entry, "/database.sqlite")) return RECOVERY_SYNC_DATABASE;
    if (!strcmp(entry, "/database.sqlite-wal")) return RECOVERY_SYNC_WAL;
    if (!strcmp(entry, "/COMPLETE")) return RECOVERY_SYNC_MARKER;
    return RECOVERY_SYNC_NONE;
}

int __wrap_fsync(int fd) {
    recovery_sync_point_t point = recovery_sync_point(fd);
    if (fail_recovery_sync != RECOVERY_SYNC_NONE && point == fail_recovery_sync) {
        ++recovery_sync_failures;
        errno = EIO;
        return -1;
    }
    if (fail_sync && published) { errno = EIO; return -1; }
    struct stat opened, old;
    if (fail_old_sync && fstat(fd, &opened) == 0 && stat(destination, &old) == 0 &&
        opened.st_dev == old.st_dev && opened.st_ino == old.st_ino) {
        errno = EIO; return -1;
    }
    int result = __real_fsync(fd);
    if (result == 0 && point == RECOVERY_SYNC_DIRECTORY) recovery_directory_synced = true;
    return result;
}

static void clear_fixture(void) {
    DIR *dir = opendir(root);
    TEST_ASSERT_NOT_NULL(dir);
    struct dirent *entry;
    while ((entry = readdir(dir))) {
        if (!strcmp(entry->d_name, ".") || !strcmp(entry->d_name, "..")) continue;
        char path[512]; snprintf(path, sizeof(path), "%s/%s", root, entry->d_name);
        struct stat info; TEST_ASSERT_EQUAL_INT(0, lstat(path, &info));
        if (S_ISDIR(info.st_mode)) {
            DIR *capture = opendir(path); TEST_ASSERT_NOT_NULL(capture);
            struct dirent *file;
            while ((file = readdir(capture))) {
                if (!strcmp(file->d_name, ".") || !strcmp(file->d_name, "..")) continue;
                char child[1024]; snprintf(child, sizeof(child), "%s/%s", path, file->d_name);
                TEST_ASSERT_EQUAL_INT(0, unlink(child));
            }
            closedir(capture);
            TEST_ASSERT_EQUAL_INT(0, rmdir(path));
            continue;
        }
        TEST_ASSERT_EQUAL_INT(0, unlink(path));
    }
    closedir(dir);
}

static void create_database(const char *path, const char *value) {
    sqlite3 *db = NULL; sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_open(path, &db));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(db, "CREATE TABLE state(value TEXT)", NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(db, "INSERT INTO state VALUES(?)", -1, &stmt, NULL));
    sqlite3_bind_text(stmt, 1, value, -1, SQLITE_STATIC);
    TEST_ASSERT_EQUAL_INT(SQLITE_DONE, sqlite3_step(stmt));
    sqlite3_finalize(stmt);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_close(db));
}

static void assert_value(const char *expected) {
    sqlite3 *db = NULL; sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_open_v2(destination, &db, SQLITE_OPEN_READWRITE, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(db, "SELECT value FROM state", -1, &stmt, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(stmt));
    TEST_ASSERT_EQUAL_STRING(expected, sqlite3_column_text(stmt, 0));
    sqlite3_finalize(stmt);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(db, "PRAGMA integrity_check", -1, &stmt, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(stmt));
    TEST_ASSERT_EQUAL_STRING("ok", sqlite3_column_text(stmt, 0));
    sqlite3_finalize(stmt);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_close(db));
}

static void wait_child(pid_t child, int code) {
    int status;
    TEST_ASSERT_EQUAL_INT(child, waitpid(child, &status, 0));
    TEST_ASSERT_TRUE(WIFEXITED(status));
    TEST_ASSERT_EQUAL_INT(code, WEXITSTATUS(status));
}

static void prepare_fixture(bool hot_journal) {
    clear_fixture();
    fail_rename = fail_sync = fail_old_sync = fail_capture = published = sidecars_at_publish = false;
    crash_at_publish = publish_attempts = 0;
    fail_recovery_sync = RECOVERY_SYNC_NONE;
    recovery_sync_failures = 0;
    recovery_directory_synced = false;
    create_database(source, "replacement");
    create_database(destination, "old-main");
    fflush(NULL);
    pid_t child = fork(); TEST_ASSERT_GREATER_OR_EQUAL_INT(0, child);
    if (child == 0) {
        sqlite3 *db = NULL;
        if (sqlite3_open(destination, &db) != SQLITE_OK) _exit(10);
        const char *sql = hot_journal
            ? "PRAGMA journal_mode=DELETE; BEGIN EXCLUSIVE; UPDATE state SET value='uncommitted';"
            : "PRAGMA journal_mode=WAL; PRAGMA wal_autocheckpoint=0; UPDATE state SET value='old-wal';";
        if (sqlite3_exec(db, sql, NULL, NULL, NULL) != SQLITE_OK) _exit(11);
        if (hot_journal && sqlite3_db_cacheflush(db) != SQLITE_OK) _exit(12);
        /* No sqlite3_close: retain real recovery state, not dummy sidecars. */
        _exit(0);
    }
    wait_child(child, 0);
    TEST_ASSERT_TRUE(sidecars_exist());
}

static int replace(bool restore) {
    return restore ? restore_database_from_backup(source, destination)
                   : backup_database(source, destination, false);
}

void setUp(void) {
    snprintf(root, sizeof(root), "/tmp/lightnvr-publication-XXXXXX");
    TEST_ASSERT_NOT_NULL(mkdtemp(root));
    snprintf(source, sizeof(source), "%s/source.sqlite", root);
    snprintf(destination, sizeof(destination), "%s/destination.sqlite", root);
    load_default_config(&g_config);
    fail_rename = fail_sync = fail_old_sync = fail_capture = published = sidecars_at_publish = false;
    crash_at_publish = publish_attempts = 0;
    fail_recovery_sync = RECOVERY_SYNC_NONE;
    recovery_sync_failures = 0;
    recovery_directory_synced = false;
}
void tearDown(void) { clear_fixture(); TEST_ASSERT_EQUAL_INT(0, rmdir(root)); }

static void test_failed_publish_preserves_committed_wal_and_can_retry(void) {
    for (int restore = 0; restore < 2; ++restore) {
        prepare_fixture(false);
        fail_rename = true;
        TEST_ASSERT_EQUAL_INT(-1, replace(restore));
        TEST_ASSERT_EQUAL_INT(1, publish_attempts);
        assert_value("old-wal");
        fail_rename = false;
        TEST_ASSERT_EQUAL_INT(0, replace(restore));
        assert_value("replacement");
    }
}

static void test_publish_never_pairs_replacement_with_old_sidecars(void) {
    for (int restore = 0; restore < 2; ++restore) {
        prepare_fixture(false);
        TEST_ASSERT_EQUAL_INT(0, replace(restore));
        TEST_ASSERT_FALSE(sidecars_at_publish);
        TEST_ASSERT_FALSE(sidecars_exist());
        assert_value("replacement");
    }
}

static void crash_test(int point) {
    for (int restore = 0; restore < 2; ++restore) {
        prepare_fixture(false);
        crash_at_publish = point;
        fflush(NULL);
        pid_t child = fork(); TEST_ASSERT_GREATER_OR_EQUAL_INT(0, child);
        if (child == 0) { replace(restore); _exit(20); }
        wait_child(child, 70 + point);
        TEST_ASSERT_FALSE(sidecars_exist());
        assert_value(point == 1 ? "old-wal" : "replacement");
    }
}
static void test_process_exit_before_rename_preserves_old_data(void) { crash_test(1); }
static void test_process_exit_after_rename_leaves_self_contained_replacement(void) { crash_test(2); }

static void test_post_publish_sync_failure_has_no_old_sidecars(void) {
    for (int restore = 0; restore < 2; ++restore) {
        prepare_fixture(false);
        fail_sync = true;
        TEST_ASSERT_EQUAL_INT(-1, replace(restore));
        TEST_ASSERT_TRUE(published);
        TEST_ASSERT_FALSE(sidecars_exist());
        fail_sync = false;
        assert_value("replacement");
    }
}

static void test_hot_journal_is_recovered_before_failed_publish(void) {
    for (int restore = 0; restore < 2; ++restore) {
        prepare_fixture(true);
        fail_rename = true;
        TEST_ASSERT_EQUAL_INT(-1, replace(restore));
        assert_value("old-main");
    }
}

static void test_busy_destination_is_not_replaced(void) {
    for (int restore = 0; restore < 2; ++restore) {
        prepare_fixture(false);
        sqlite3 *reader = NULL;
        TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_open(destination, &reader));
        TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(reader, "BEGIN; SELECT * FROM state", NULL, NULL, NULL));
        int result = replace(restore);
        sqlite3_exec(reader, "ROLLBACK", NULL, NULL, NULL);
        sqlite3_close(reader);
        TEST_ASSERT_EQUAL_INT(-1, result);
        TEST_ASSERT_EQUAL_INT(0, publish_attempts);
        assert_value("old-wal");
    }
}

static void test_old_database_sync_failure_prevents_publication(void) {
    for (int restore = 0; restore < 2; ++restore) {
        prepare_fixture(false);
        fail_old_sync = true;
        TEST_ASSERT_EQUAL_INT(-1, replace(restore));
        TEST_ASSERT_EQUAL_INT(0, publish_attempts);
        fail_old_sync = false;
        assert_value("old-wal");
    }
}

static void write_bytes(const char *path, const char *bytes) {
    int fd = open(path, O_WRONLY | O_CREAT | O_TRUNC, 0600);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, fd);
    TEST_ASSERT_EQUAL_INT(strlen(bytes), write(fd, bytes, strlen(bytes)));
    TEST_ASSERT_EQUAL_INT(0, close(fd));
}

static void assert_bytes(const char *path, const char *bytes) {
    char content[128] = {0};
    int fd = open(path, O_RDONLY); TEST_ASSERT_GREATER_OR_EQUAL_INT(0, fd);
    ssize_t length = read(fd, content, sizeof(content) - 1);
    close(fd);
    TEST_ASSERT_EQUAL_INT(strlen(bytes), length);
    TEST_ASSERT_EQUAL_STRING(bytes, content);
}

static void assert_recovery_capture(bool complete) {
    DIR *dir = opendir(root); TEST_ASSERT_NOT_NULL(dir);
    struct dirent *entry;
    char capture[512] = {0}; int count = 0;
    while ((entry = readdir(dir))) {
        if (!strncmp(entry->d_name, "destination.sqlite.recovery-", 28)) {
            snprintf(capture, sizeof(capture), "%s/%s", root, entry->d_name);
            ++count;
        }
    }
    closedir(dir);
    TEST_ASSERT_EQUAL_INT(1, count);
    struct stat info;
    TEST_ASSERT_EQUAL_INT(0, stat(capture, &info));
    TEST_ASSERT_EQUAL_UINT(0700, info.st_mode & 0777);
    char path[600]; snprintf(path, sizeof(path), "%s/COMPLETE", capture);
    TEST_ASSERT_EQUAL_INT(complete ? 0 : -1, access(path, F_OK));
    if (!complete) return;
    snprintf(path, sizeof(path), "%s/database.sqlite", capture);
    assert_bytes(path, "not-a-sqlite-database");
    TEST_ASSERT_EQUAL_INT(0, stat(path, &info));
    TEST_ASSERT_EQUAL_UINT(0600, info.st_mode & 0777);
    snprintf(path, sizeof(path), "%s/database.sqlite-wal", capture);
    assert_bytes(path, "operator-recovery-state");
    TEST_ASSERT_EQUAL_INT(0, stat(path, &info));
    TEST_ASSERT_EQUAL_UINT(0600, info.st_mode & 0777);
}

static void corrupt_fixture(void) {
    clear_fixture();
    published = sidecars_at_publish = false; publish_attempts = 0;
    fail_recovery_sync = RECOVERY_SYNC_NONE;
    recovery_sync_failures = 0;
    recovery_directory_synced = false;
    create_database(source, "replacement");
    write_bytes(destination, "not-a-sqlite-database");
    char wal[300]; snprintf(wal, sizeof(wal), "%s-wal", destination);
    write_bytes(wal, "operator-recovery-state");
}

static void test_corrupt_database_recovery_files_are_retained_before_replacement(void) {
    for (int restore = 0; restore < 2; ++restore) {
        corrupt_fixture();
        TEST_ASSERT_EQUAL_INT(0, replace(restore));
        TEST_ASSERT_FALSE(sidecars_at_publish);
        assert_value("replacement");
        assert_recovery_capture(true);
    }
}

static void test_recovery_copy_failure_preserves_original_and_stops_publish(void) {
    for (int restore = 0; restore < 2; ++restore) {
        corrupt_fixture();
        fail_capture = true;
        int result = replace(restore);
        fail_capture = false;
        TEST_ASSERT_EQUAL_INT(-1, result);
        TEST_ASSERT_EQUAL_INT(0, publish_attempts);
        assert_bytes(destination, "not-a-sqlite-database");
        char wal[300]; snprintf(wal, sizeof(wal), "%s-wal", destination);
        assert_bytes(wal, "operator-recovery-state");
        assert_recovery_capture(false);
    }
}

static void assert_recovery_sync_failure(recovery_sync_point_t point) {
    for (int restore = 0; restore < 2; ++restore) {
        corrupt_fixture();
        fail_recovery_sync = point;
        int result = replace(restore);
        fail_recovery_sync = RECOVERY_SYNC_NONE;
        TEST_ASSERT_EQUAL_INT(1, recovery_sync_failures);
        TEST_ASSERT_EQUAL_INT(-1, result);
        TEST_ASSERT_EQUAL_INT(0, publish_attempts);
        TEST_ASSERT_FALSE(published);
        assert_bytes(destination, "not-a-sqlite-database");
        char wal[300]; snprintf(wal, sizeof(wal), "%s-wal", destination);
        assert_bytes(wal, "operator-recovery-state");
        /* Marker/directory fsync fails after COMPLETE has been created. Its
         * presence alone does not prove durability or permit publication. */
        assert_recovery_capture(point >= RECOVERY_SYNC_MARKER);
    }
}

static void test_recovery_copy_fsync_failure_preserves_original_and_stops_publish(void) {
    assert_recovery_sync_failure(RECOVERY_SYNC_DATABASE);
    assert_recovery_sync_failure(RECOVERY_SYNC_WAL);
    assert_recovery_sync_failure(RECOVERY_SYNC_MARKER);
}

static void test_recovery_copy_directory_fsync_failure_preserves_original_and_stops_publish(void) {
    assert_recovery_sync_failure(RECOVERY_SYNC_DIRECTORY);
    assert_recovery_sync_failure(RECOVERY_SYNC_PARENT);
}

static void test_corrupt_recovery_copy_survives_failed_rename(void) {
    for (int restore = 0; restore < 2; ++restore) {
        corrupt_fixture();
        fail_rename = true;
        TEST_ASSERT_EQUAL_INT(-1, replace(restore));
        fail_rename = false;
        TEST_ASSERT_EQUAL_INT(1, publish_attempts);
        assert_bytes(destination, "not-a-sqlite-database");
        assert_recovery_capture(true);
    }
}

static void test_corrupt_destination_without_sidecars_can_be_replaced(void) {
    for (int restore = 0; restore < 2; ++restore) {
        clear_fixture();
        create_database(source, "replacement");
        write_bytes(destination, "not-a-sqlite-database");
        TEST_ASSERT_EQUAL_INT(0, replace(restore));
        assert_value("replacement");
    }
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_failed_publish_preserves_committed_wal_and_can_retry);
    RUN_TEST(test_publish_never_pairs_replacement_with_old_sidecars);
    RUN_TEST(test_process_exit_before_rename_preserves_old_data);
    RUN_TEST(test_process_exit_after_rename_leaves_self_contained_replacement);
    RUN_TEST(test_post_publish_sync_failure_has_no_old_sidecars);
    RUN_TEST(test_hot_journal_is_recovered_before_failed_publish);
    RUN_TEST(test_busy_destination_is_not_replaced);
    RUN_TEST(test_old_database_sync_failure_prevents_publication);
    RUN_TEST(test_corrupt_database_recovery_files_are_retained_before_replacement);
    RUN_TEST(test_recovery_copy_failure_preserves_original_and_stops_publish);
    RUN_TEST(test_recovery_copy_fsync_failure_preserves_original_and_stops_publish);
    RUN_TEST(test_recovery_copy_directory_fsync_failure_preserves_original_and_stops_publish);
    RUN_TEST(test_corrupt_recovery_copy_survives_failed_rename);
    RUN_TEST(test_corrupt_destination_without_sidecars_can_be_replaced);
    return UNITY_END();
}
