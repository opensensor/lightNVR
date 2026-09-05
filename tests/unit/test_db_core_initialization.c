#define _POSIX_C_SOURCE 200809L

#include <errno.h>
#include <fcntl.h>
#include <sqlite3.h>
#include <stdio.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

#include "unity.h"

#include "core/config.h"
#include "database/db_core.h"

#define TEST_DB_PATH "/tmp/lightnvr_db_core_initialization.sqlite"
#define TEST_BACKUP_DIR TEST_DB_PATH ".backups"

static void remove_if_present(const char *path) {
    if (unlink(path) != 0 && errno != ENOENT) {
        TEST_FAIL_MESSAGE("Failed to remove test fixture");
    }
}

static void remove_database_files(void) {
    static const char *suffixes[] = {"", "-wal", "-shm", "-journal"};
    char path[512];
    for (size_t i = 0; i < sizeof(suffixes) / sizeof(suffixes[0]); i++) {
        snprintf(path, sizeof(path), "%s%s", TEST_DB_PATH, suffixes[i]);
        remove_if_present(path);
    }
}

static void remove_backup_fixture(const char *name) {
    char path[512];
    snprintf(path, sizeof(path), "%s/%s", TEST_BACKUP_DIR, name);
    remove_if_present(path);
}

static void create_file(const char *path) {
    int fd = open(path, O_WRONLY | O_CREAT | O_TRUNC, 0600);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, fd);
    TEST_ASSERT_EQUAL_INT(1, (int)write(fd, "x", 1));
    TEST_ASSERT_EQUAL_INT(0, close(fd));
}

void setUp(void) {
    if (get_db_handle()) {
        shutdown_database();
    }
    remove_database_files();
    remove_backup_fixture("20260905T120000Z.sqlite3");
    remove_backup_fixture("20260905T120100Z.sqlite3.tmp");
    remove_backup_fixture("20260905T120100Z.sqlite3.tmp-journal");
    remove_backup_fixture("operator-notes.tmp");
    rmdir(TEST_BACKUP_DIR);
    load_default_config(&g_config);
}

void tearDown(void) {
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
}

static void create_readable_database(void) {
    sqlite3 *fixture = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_open(TEST_DB_PATH, &fixture));
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_exec(fixture,
                     "CREATE TABLE sample(id INTEGER PRIMARY KEY, value TEXT);"
                     "INSERT INTO sample(value) VALUES('unchanged');",
                     NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_close(fixture));
}

void test_read_only_initialization_cannot_write(void) {
    struct stat before;
    struct stat after;
    create_readable_database();
    TEST_ASSERT_EQUAL_INT(0, chmod(TEST_DB_PATH, 0400));
    TEST_ASSERT_EQUAL_INT(0, stat(TEST_DB_PATH, &before));

    TEST_ASSERT_EQUAL_INT(
        0, init_database_ex(TEST_DB_PATH, DB_INIT_READ_ONLY));
    sqlite3 *handle = get_db_handle();
    TEST_ASSERT_NOT_NULL(handle);
    TEST_ASSERT_EQUAL_INT(1, sqlite3_db_readonly(handle, "main"));

    sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_prepare_v2(handle, "SELECT value FROM sample WHERE id=1;",
                           -1, &stmt, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(stmt));
    TEST_ASSERT_EQUAL_STRING("unchanged", sqlite3_column_text(stmt, 0));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_finalize(stmt));

    TEST_ASSERT_EQUAL_INT(
        SQLITE_READONLY,
        sqlite3_exec(handle, "INSERT INTO sample(value) VALUES('changed');",
                     NULL, NULL, NULL));

    shutdown_database();
    TEST_ASSERT_EQUAL_INT(0, stat(TEST_DB_PATH, &after));
    TEST_ASSERT_EQUAL_INT64(before.st_size, after.st_size);
    TEST_ASSERT_EQUAL_INT64(before.st_mtim.tv_sec, after.st_mtim.tv_sec);
    TEST_ASSERT_EQUAL_INT64(before.st_mtim.tv_nsec, after.st_mtim.tv_nsec);
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

    sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_prepare_v2(
            get_db_handle(),
            "EXPLAIN QUERY PLAN "
            "UPDATE detections SET event_end_time=timestamp "
            "WHERE source='external_motion' AND event_end_time IS NULL;",
            -1, &stmt, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(stmt));
    const char *plan = (const char *)sqlite3_column_text(stmt, 3);
    TEST_ASSERT_NOT_NULL(plan);
    TEST_ASSERT_NOT_NULL(strstr(plan, "idx_detections_open_external_motion"));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_finalize(stmt));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_read_only_initialization_cannot_write);
    RUN_TEST(test_writable_initialization_removes_only_stale_backup_artifacts);
    return UNITY_END();
}
