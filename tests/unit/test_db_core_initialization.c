#define _POSIX_C_SOURCE 200809L

#include <errno.h>
#include <dirent.h>
#include <fcntl.h>
#include <limits.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

#include "unity.h"

#include "core/config.h"
#include "database/db_core.h"

static char test_fixture_dir[] = "/tmp/lightnvr_db_core_initialization.XXXXXX";
static char test_db_path[512];
static char test_backup_dir[512];
static char overlong_parent_dir[512];
static char overlong_deepest_dir[8192];
static int test_paths_initialized = 0;
static unsigned char *wal_before_bytes;
static unsigned char *wal_after_bytes;
static sqlite3 *tracked_fixture_db = NULL;
static sqlite3_stmt *tracked_fixture_stmt = NULL;
static int sqlite3_open_v2_call_count = 0;
static char sqlite3_open_v2_last_path[8192];

static void reset_sqlite_open_instrumentation(void) {
    sqlite3_open_v2_call_count = 0;
    sqlite3_open_v2_last_path[0] = '\0';
}

int __real_sqlite3_open_v2(const char *filename, sqlite3 **ppDb, int flags, const char *zVfs);
int __wrap_sqlite3_open_v2(const char *filename, sqlite3 **ppDb, int flags, const char *zVfs) {
    sqlite3_open_v2_call_count++;
    if (filename != NULL) {
        (void)snprintf(sqlite3_open_v2_last_path, sizeof(sqlite3_open_v2_last_path), "%s", filename);
    } else {
        sqlite3_open_v2_last_path[0] = '\0';
    }
    return __real_sqlite3_open_v2(filename, ppDb, flags, zVfs);
}
#define TEST_DB_PATH test_db_path
#define TEST_BACKUP_DIR test_backup_dir

static const char *path_with_suffix(const char *base, const char *suffix) {
    static char path[1024];
    int written = snprintf(path, sizeof(path), "%s%s", base, suffix);
    TEST_ASSERT_TRUE(written >= 0 && (size_t)written < sizeof(path));
    return path;
}

static unsigned char *read_entire_file(const char *path, size_t *out_size) {
    FILE *f = fopen(path, "rb");
    TEST_ASSERT_NOT_NULL(f);

    if (fseek(f, 0, SEEK_END) != 0) {
        fclose(f);
        TEST_FAIL_MESSAGE("Could not seek fixture file");
    }
    long file_size = ftell(f);
    if (file_size < 0 || fseek(f, 0, SEEK_SET) != 0) {
        fclose(f);
        TEST_FAIL_MESSAGE("Could not size fixture file");
    }

    unsigned char *buffer = NULL;
    if (file_size > 0) {
        buffer = malloc((size_t)file_size);
        if (!buffer) {
            fclose(f);
            TEST_FAIL_MESSAGE("Could not allocate fixture snapshot");
        }
        size_t read_count = fread(buffer, 1, (size_t)file_size, f);
        if (read_count != (size_t)file_size) {
            free(buffer);
            fclose(f);
            TEST_FAIL_MESSAGE("Could not read fixture file");
        }
    }

    if (fclose(f) != 0) {
        free(buffer);
        TEST_FAIL_MESSAGE("Could not close fixture file");
    }
    *out_size = (size_t)file_size;
    return buffer;
}

static void initialize_test_paths(void) {
    if (test_paths_initialized) return;

    TEST_ASSERT_NOT_NULL(mkdtemp(test_fixture_dir));
    int written = snprintf(test_db_path, sizeof(test_db_path), "%s/db.sqlite", test_fixture_dir);
    TEST_ASSERT_TRUE(written > 0 && (size_t)written < sizeof(test_db_path));
    written = snprintf(test_backup_dir, sizeof(test_backup_dir), "%s.backups", test_db_path);
    TEST_ASSERT_TRUE(written > 0 && (size_t)written < sizeof(test_backup_dir));
    test_paths_initialized = 1;
}

static void remove_if_present(const char *path) {
    if (unlink(path) != 0 && errno != ENOENT) {
        TEST_FAIL_MESSAGE("Failed to remove test fixture");
    }
}

static void remove_database_files(void) {
    static const char *suffixes[] = {"", "-wal", "-shm", "-journal", ".bak", ".private-backup", ".private-backup.tmp"};
    char path[512];
    for (size_t i = 0; i < sizeof(suffixes) / sizeof(suffixes[0]); i++) {
        snprintf(path, sizeof(path), "%s%s", test_db_path, suffixes[i]);
        remove_if_present(path);
    }
    /* A new database creates an initial timestamped backup even when shutdown
     * backups are disabled. Remove only regular entries in this owned fixture. */
    DIR *directory = opendir(test_backup_dir);
    if (directory) {
        struct dirent *entry;
        while ((entry = readdir(directory))) {
            if (entry->d_name[0] == '.') continue;
            snprintf(path, sizeof(path), "%s/%s", test_backup_dir, entry->d_name);
            remove_if_present(path);
        }
        closedir(directory);
    }
}

static void remove_backup_fixture(const char *name) {
    char path[512];
    snprintf(path, sizeof(path), "%s/%s", test_backup_dir, name);
    remove_if_present(path);
}

static void create_file(const char *path) {
    int fd = open(path, O_WRONLY | O_CREAT | O_TRUNC, 0600);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, fd);
    TEST_ASSERT_EQUAL_INT(1, (int)write(fd, "x", 1));
    TEST_ASSERT_EQUAL_INT(0, close(fd));
}

void setUp(void) {
    wal_before_bytes = wal_after_bytes = NULL;
    initialize_test_paths();
    if (get_db_handle()) {
        shutdown_database();
    }
    remove_database_files();
    remove_backup_fixture("20260905T120000Z.sqlite3");
    remove_backup_fixture("20260905T120100Z.sqlite3.tmp");
    remove_backup_fixture("20260905T120100Z.sqlite3.tmp-journal");
    remove_backup_fixture("operator-notes.tmp");
    rmdir(test_backup_dir);
    load_default_config(&g_config);
}

void tearDown(void) {
    free(wal_before_bytes);
    free(wal_after_bytes);
    if (tracked_fixture_stmt) {
        sqlite3_finalize(tracked_fixture_stmt);
        tracked_fixture_stmt = NULL;
    }
    if (tracked_fixture_db) {
        sqlite3_close(tracked_fixture_db);
        tracked_fixture_db = NULL;
    }
    if (get_db_handle()) {
        shutdown_database();
    }
    chmod(TEST_DB_PATH, 0600);
    remove_database_files();
    remove_backup_fixture("20260905T120000Z.sqlite3");
    remove_backup_fixture("20260905T120100Z.sqlite3.tmp");
    remove_backup_fixture("20260905T120100Z.sqlite3.tmp-journal");
    remove_backup_fixture("operator-notes.tmp");
    rmdir(TEST_BACKUP_DIR);
    while (overlong_deepest_dir[0] != '\0' &&
           strcmp(overlong_deepest_dir, overlong_parent_dir) != 0) {
        rmdir(overlong_deepest_dir);
        char *slash = strrchr(overlong_deepest_dir, '/');
        if (!slash) break;
        *slash = '\0';
    }
    overlong_deepest_dir[0] = '\0';
    if (overlong_parent_dir[0] != '\0') {
        rmdir(overlong_parent_dir);
        overlong_parent_dir[0] = '\0';
    }
}

static void create_readable_database(void) {
    tracked_fixture_db = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_open(TEST_DB_PATH, &tracked_fixture_db));
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_exec(tracked_fixture_db,
                     "CREATE TABLE sample(id INTEGER PRIMARY KEY, value TEXT);"
                     "INSERT INTO sample(value) VALUES('unchanged');",
                     NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_close(tracked_fixture_db));
    tracked_fixture_db = NULL;
}

static void test_legacy_database_enables_incremental_auto_vacuum(void) {
    create_readable_database();
    TEST_ASSERT_EQUAL_INT(0, init_database_ex(TEST_DB_PATH, DB_INIT_NO_BACKUP));
    tracked_fixture_stmt = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(
        get_db_handle(), "PRAGMA auto_vacuum;", -1, &tracked_fixture_stmt, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(tracked_fixture_stmt));
    TEST_ASSERT_EQUAL_INT(2, sqlite3_column_int(tracked_fixture_stmt, 0));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_finalize(tracked_fixture_stmt));
    tracked_fixture_stmt = NULL;
}

static void test_overlong_database_path_is_rejected_before_initialization(void) {
    char rejected_path[8192];
    struct stat parent;
    TEST_ASSERT_EQUAL_INT(0, stat(test_fixture_dir, &parent));
    TEST_ASSERT_TRUE(S_ISDIR(parent.st_mode));

    int written = snprintf(overlong_parent_dir, sizeof(overlong_parent_dir),
                           "%s/existing_parent", test_fixture_dir);
    TEST_ASSERT_TRUE(written > 0 && (size_t)written < sizeof(overlong_parent_dir));
    written = snprintf(rejected_path, sizeof(rejected_path), "%s", overlong_parent_dir);
    TEST_ASSERT_TRUE(written > 0 && (size_t)written < sizeof(rejected_path));
    if (mkdir(overlong_parent_dir, 0700) != 0) {
        TEST_ASSERT_EQUAL_INT(EEXIST, errno);
    }

    size_t used = strlen(rejected_path);
    while (used + 1 + 50 < PATH_MAX) {
        const size_t component_len = 50; /* Keep each component below NAME_MAX. */
        TEST_ASSERT_TRUE(used + component_len + 2 < sizeof(rejected_path));
        rejected_path[used++] = '/';
        memset(rejected_path + used, 'a', component_len);
        used += component_len;
        rejected_path[used] = '\0';
        TEST_ASSERT_EQUAL_INT(0, mkdir(rejected_path, 0700));
        snprintf(overlong_deepest_dir, sizeof(overlong_deepest_dir),
                 "%s", rejected_path);
    }

    {
        size_t leaf_len = (size_t)PATH_MAX - used + 1;
        TEST_ASSERT_TRUE(leaf_len <= NAME_MAX);
        TEST_ASSERT_TRUE(used + leaf_len + 2 < sizeof(rejected_path));
        rejected_path[used++] = '/';
        memset(rejected_path + used, 'b', leaf_len);
        rejected_path[used + leaf_len] = '\0';
    }

    TEST_ASSERT_TRUE(strlen(rejected_path) > PATH_MAX);

    reset_sqlite_open_instrumentation();
    TEST_ASSERT_EQUAL_INT(-1, init_database_ex(rejected_path, DB_INIT_NO_BACKUP));
    TEST_ASSERT_EQUAL_INT(0, sqlite3_open_v2_call_count);
    TEST_ASSERT_NULL(get_db_handle());
    DIR *parent_dir = opendir(overlong_deepest_dir);
    TEST_ASSERT_NOT_NULL(parent_dir);
    struct dirent *entry;
    bool unexpected_entry = false;
    while ((entry = readdir(parent_dir))) {
        if (strcmp(entry->d_name, ".") != 0 &&
            strcmp(entry->d_name, "..") != 0) unexpected_entry = true;
    }
    TEST_ASSERT_EQUAL_INT(0, closedir(parent_dir));
    TEST_ASSERT_FALSE(unexpected_entry);
}

static void create_readable_wal_database(void) {
    TEST_ASSERT_NULL(tracked_fixture_db);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_open(TEST_DB_PATH, &tracked_fixture_db));
    TEST_ASSERT_NOT_NULL(tracked_fixture_db);
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_exec(tracked_fixture_db, "PRAGMA journal_mode=WAL;", NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_exec(tracked_fixture_db,
                     "CREATE TABLE IF NOT EXISTS sample("
                     "id INTEGER PRIMARY KEY, value TEXT NOT NULL);",
                     NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_exec(tracked_fixture_db,
                     "DELETE FROM sample; "
                     "INSERT INTO sample(id, value) VALUES(1, 'unchanged');",
                     NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_exec(tracked_fixture_db, "PRAGMA wal_checkpoint(TRUNCATE);", NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_close(tracked_fixture_db));
    tracked_fixture_db = NULL;

    /* Re-open in WAL mode and write once so sidecars are present in fixture. */
    TEST_ASSERT_NULL(tracked_fixture_db);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_open(TEST_DB_PATH, &tracked_fixture_db));
    TEST_ASSERT_NOT_NULL(tracked_fixture_db);
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_exec(tracked_fixture_db, "PRAGMA journal_mode=WAL;", NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_exec(tracked_fixture_db, "PRAGMA wal_autocheckpoint=0;", NULL, NULL, NULL));
    int no_ckpt_on_close = 1;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK,
                          sqlite3_db_config(tracked_fixture_db, SQLITE_DBCONFIG_NO_CKPT_ON_CLOSE,
                                            no_ckpt_on_close, NULL));
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_exec(tracked_fixture_db,
                     "INSERT OR REPLACE INTO sample(id, value) "
                     "VALUES(1, 'wal_only');",
                     NULL, NULL, NULL));
    int persist_wal = 1;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK,
                          sqlite3_file_control(tracked_fixture_db, "main", SQLITE_FCNTL_PERSIST_WAL, &persist_wal));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_close(tracked_fixture_db));
    tracked_fixture_db = NULL;

    TEST_ASSERT_EQUAL_INT(0, access(path_with_suffix(TEST_DB_PATH, "-wal"), F_OK));
    TEST_ASSERT_EQUAL_INT(0, access(path_with_suffix(TEST_DB_PATH, "-shm"), F_OK));
}

void test_read_only_initialization_cannot_write(void) {
    struct stat before;
    struct stat after;
    struct stat wal_before;
    struct stat wal_after;
    struct stat shm_before;
    struct stat shm_after;
    size_t wal_before_size = 0;
    size_t wal_after_size = 0;
    char wal_path[1024];
    char shm_path[1024];

    create_readable_wal_database();
    TEST_ASSERT_EQUAL_INT(0, chmod(TEST_DB_PATH, 0400));
    TEST_ASSERT_EQUAL_INT(0, stat(TEST_DB_PATH, &before));
    int path_len = snprintf(wal_path, sizeof(wal_path), "%s-wal", TEST_DB_PATH);
    TEST_ASSERT_TRUE(path_len > 0 && (size_t)path_len < sizeof(wal_path));
    TEST_ASSERT_EQUAL_INT(0, stat(wal_path, &wal_before));
    wal_before_bytes = read_entire_file(wal_path, &wal_before_size);
    path_len = snprintf(shm_path, sizeof(shm_path), "%s-shm", TEST_DB_PATH);
    TEST_ASSERT_TRUE(path_len > 0 && (size_t)path_len < sizeof(shm_path));
    TEST_ASSERT_EQUAL_INT(0, stat(shm_path, &shm_before));

    TEST_ASSERT_EQUAL_INT(
        0, init_database_ex(TEST_DB_PATH, DB_INIT_READ_ONLY));
    sqlite3 *handle = get_db_handle();
    TEST_ASSERT_NOT_NULL(handle);
    TEST_ASSERT_EQUAL_INT(1, sqlite3_db_readonly(handle, "main"));

    tracked_fixture_stmt = NULL;
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_prepare_v2(handle, "SELECT value FROM sample WHERE id=1;",
                           -1, &tracked_fixture_stmt, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(tracked_fixture_stmt));
    TEST_ASSERT_EQUAL_STRING("wal_only",
                             sqlite3_column_text(tracked_fixture_stmt, 0));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_finalize(tracked_fixture_stmt));
    tracked_fixture_stmt = NULL;

    TEST_ASSERT_EQUAL_INT(
        SQLITE_READONLY,
        sqlite3_exec(handle, "INSERT INTO sample(value) VALUES('changed');",
                     NULL, NULL, NULL));

    shutdown_database();
    TEST_ASSERT_EQUAL_INT(0, stat(TEST_DB_PATH, &after));
    TEST_ASSERT_EQUAL_INT(0, stat(wal_path, &wal_after));
    wal_after_bytes = read_entire_file(wal_path, &wal_after_size);
    TEST_ASSERT_EQUAL_INT(0, stat(shm_path, &shm_after));
    TEST_ASSERT_EQUAL_INT64(before.st_size, after.st_size);
    TEST_ASSERT_EQUAL_UINT(before.st_mode, after.st_mode);
    TEST_ASSERT_EQUAL_INT64(before.st_mtim.tv_sec, after.st_mtim.tv_sec);
    TEST_ASSERT_EQUAL_INT64(before.st_mtim.tv_nsec, after.st_mtim.tv_nsec);
    TEST_ASSERT_EQUAL_UINT((unsigned int)(wal_before.st_mode & 07777),
                           (unsigned int)(wal_after.st_mode & 07777));
    TEST_ASSERT_EQUAL_UINT((unsigned int)(shm_before.st_mode & 07777),
                           (unsigned int)(shm_after.st_mode & 07777));
    TEST_ASSERT_EQUAL_UINT64((uint64_t)wal_before_size, (uint64_t)wal_after_size);
    TEST_ASSERT_EQUAL_INT64(shm_before.st_size, shm_after.st_size);
    if (wal_before_size > 0) {
        TEST_ASSERT_EQUAL_INT(0, memcmp(wal_before_bytes, wal_after_bytes, wal_before_size));
    }

}

void test_writable_initialization_removes_only_stale_backup_artifacts(void) {
    char completed_path[512];
    char temporary_path[512];
    char journal_path[512];
    char unrelated_path[512];

    create_readable_database();
    TEST_ASSERT_EQUAL_INT(0, mkdir(TEST_BACKUP_DIR, 0700));
    snprintf(completed_path, sizeof(completed_path),
             "%s/20260905T120000Z.sqlite3", TEST_BACKUP_DIR);
    snprintf(temporary_path, sizeof(temporary_path),
             "%s/20260905T120100Z.sqlite3.tmp", TEST_BACKUP_DIR);
    snprintf(journal_path, sizeof(journal_path),
             "%s/20260905T120100Z.sqlite3.tmp-journal", TEST_BACKUP_DIR);
    snprintf(unrelated_path, sizeof(unrelated_path),
             "%s/operator-notes.tmp", TEST_BACKUP_DIR);
    create_file(completed_path);
    create_file(temporary_path);
    create_file(journal_path);
    create_file(unrelated_path);

    TEST_ASSERT_EQUAL_INT(
        0, init_database_ex(TEST_DB_PATH, DB_INIT_NO_BACKUP));

    TEST_ASSERT_EQUAL_INT(0, access(completed_path, F_OK));
    TEST_ASSERT_EQUAL_INT(-1, access(temporary_path, F_OK));
    TEST_ASSERT_EQUAL_INT(ENOENT, errno);
    TEST_ASSERT_EQUAL_INT(-1, access(journal_path, F_OK));
    TEST_ASSERT_EQUAL_INT(ENOENT, errno);
    TEST_ASSERT_EQUAL_INT(0, access(unrelated_path, F_OK));

    tracked_fixture_stmt = NULL;
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_prepare_v2(
            get_db_handle(),
            "EXPLAIN QUERY PLAN "
            "UPDATE detections SET event_end_time=timestamp "
            "WHERE source='external_motion' AND event_end_time IS NULL;",
            -1, &tracked_fixture_stmt, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(tracked_fixture_stmt));
    const char *plan = (const char *)sqlite3_column_text(tracked_fixture_stmt, 3);
    TEST_ASSERT_NOT_NULL(plan);
    TEST_ASSERT_NOT_NULL(strstr(plan, "idx_detections_open_external_motion"));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_finalize(tracked_fixture_stmt));
    tracked_fixture_stmt = NULL;
}

static void assert_database_private(void) {
    static const char *suffixes[] = {"", "-wal", "-shm"};
    char path[512];
    for (size_t i = 0; i < sizeof(suffixes)/sizeof(suffixes[0]); ++i) {
        snprintf(path, sizeof(path), "%s%s", TEST_DB_PATH, suffixes[i]);
        struct stat info;
        TEST_ASSERT_EQUAL_INT(0, stat(path, &info));
        TEST_ASSERT_EQUAL_UINT(0600, info.st_mode & 0777);
    }
}

static void test_new_database_and_sidecars_are_private_with_permissive_umask(void) {
    mode_t previous = umask(0);
    int result = init_database_ex(TEST_DB_PATH, DB_INIT_NO_BACKUP);
    umask(previous);
    TEST_ASSERT_EQUAL_INT(0, result);
    assert_database_private();
}

static void test_writable_startup_tightens_legacy_database_and_wal_permissions(void) {
    tracked_fixture_db = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_open(TEST_DB_PATH, &tracked_fixture_db));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(tracked_fixture_db,
        "PRAGMA journal_mode=WAL; CREATE TABLE sample(value TEXT);"
        "INSERT INTO sample VALUES('preserved');", NULL, NULL, NULL));
    int persist = 1;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_file_control(tracked_fixture_db, "main", SQLITE_FCNTL_PERSIST_WAL, &persist));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_close(tracked_fixture_db));
    tracked_fixture_db = NULL;
    static const char *suffixes[] = {"", "-wal", "-shm"};
    char path[512];
    for (size_t i = 0; i < sizeof(suffixes)/sizeof(suffixes[0]); ++i) {
        snprintf(path, sizeof(path), "%s%s", TEST_DB_PATH, suffixes[i]);
        TEST_ASSERT_EQUAL_INT(0, chmod(path, 0666));
    }
    TEST_ASSERT_EQUAL_INT(0, init_database_ex(TEST_DB_PATH, DB_INIT_NO_BACKUP));
    assert_database_private();
    tracked_fixture_stmt = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(get_db_handle(), "SELECT value FROM sample", -1, &tracked_fixture_stmt, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(tracked_fixture_stmt));
    TEST_ASSERT_EQUAL_STRING("preserved", sqlite3_column_text(tracked_fixture_stmt, 0));
    sqlite3_finalize(tracked_fixture_stmt);
    tracked_fixture_stmt = NULL;
}

static void test_backup_copy_is_private_without_changing_source_permissions(void) {
    create_readable_database();
    TEST_ASSERT_EQUAL_INT(0, chmod(TEST_DB_PATH, 0644));
    mode_t previous = umask(0);
    int result = backup_database(TEST_DB_PATH,
                                 path_with_suffix(TEST_DB_PATH, ".private-backup"), false);
    umask(previous);
    TEST_ASSERT_EQUAL_INT(0, result);
    struct stat info;
    TEST_ASSERT_EQUAL_INT(0, stat(path_with_suffix(TEST_DB_PATH, ".private-backup"), &info));
    TEST_ASSERT_EQUAL_UINT(0600, info.st_mode & 0777);
    TEST_ASSERT_EQUAL_INT(0, stat(TEST_DB_PATH, &info));
    TEST_ASSERT_EQUAL_UINT(0644, info.st_mode & 0777);
}

static void test_private_backup_creation_failure_is_reported_and_can_retry(void) {
    create_readable_database();
    TEST_ASSERT_EQUAL_INT(-1, access(TEST_BACKUP_DIR, F_OK));
    TEST_ASSERT_EQUAL_INT(ENOENT, errno);
    /* A missing parent allows stale-file cleanup but prevents creation of the
     * private staging inode, independently of root/non-root test privileges. */
    TEST_ASSERT_EQUAL_INT(-1, backup_database(TEST_DB_PATH,
                                            path_with_suffix(TEST_BACKUP_DIR, "/missing.sqlite"), false));
    TEST_ASSERT_EQUAL_INT(-1, access(path_with_suffix(TEST_BACKUP_DIR, "/missing.sqlite"), F_OK));
    TEST_ASSERT_EQUAL_INT(0, mkdir(TEST_BACKUP_DIR, 0700));
    TEST_ASSERT_EQUAL_INT(0, backup_database(TEST_DB_PATH,
                                           path_with_suffix(TEST_BACKUP_DIR, "/missing.sqlite"), false));
    TEST_ASSERT_EQUAL_INT(-1, access(path_with_suffix(TEST_BACKUP_DIR, "/missing.sqlite.tmp"), F_OK));
    TEST_ASSERT_EQUAL_INT(ENOENT, errno);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_open_v2(path_with_suffix(TEST_BACKUP_DIR, "/missing.sqlite"),
                                                  &tracked_fixture_db, SQLITE_OPEN_READONLY, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(tracked_fixture_db, "SELECT value FROM sample",
                                                     -1, &tracked_fixture_stmt, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(tracked_fixture_stmt));
    TEST_ASSERT_EQUAL_STRING("unchanged", sqlite3_column_text(tracked_fixture_stmt, 0));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_finalize(tracked_fixture_stmt));
    tracked_fixture_stmt = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_close(tracked_fixture_db));
    tracked_fixture_db = NULL;
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_legacy_database_enables_incremental_auto_vacuum);
    RUN_TEST(test_overlong_database_path_is_rejected_before_initialization);
    RUN_TEST(test_new_database_and_sidecars_are_private_with_permissive_umask);
    RUN_TEST(test_writable_startup_tightens_legacy_database_and_wal_permissions);
    RUN_TEST(test_backup_copy_is_private_without_changing_source_permissions);
    RUN_TEST(test_private_backup_creation_failure_is_reported_and_can_retry);
    RUN_TEST(test_read_only_initialization_cannot_write);
    RUN_TEST(test_writable_initialization_removes_only_stale_backup_artifacts);
    int result = UNITY_END();
    if (test_paths_initialized && rmdir(test_fixture_dir) != 0) return 1;
    return result;
}
