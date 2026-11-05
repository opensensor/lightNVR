/**
 * Startup reconciliation of recordings interrupted by an unclean shutdown.
 *
 * Rows that cannot be pruned must not stop the scan: it used to re-read the same
 * oldest batch forever, which kept the storage controller from ever starting
 * retention. CTest's timeout on this binary is what catches a hang.
 */
#define _GNU_SOURCE

#include <errno.h>
#include <fcntl.h>
#include <limits.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

#include "unity.h"
#include "core/config.h"
#include "database/db_core.h"
#include "database/db_recordings.h"
#include "database/db_storage_targets.h"
#include "storage/storage_deletion.h"
#include "storage/storage_manager.h"
#include "utils/strings.h"

#define BATCH_SIZE 500

static char g_root[64];
static char g_db_path[PATH_MAX];
static char g_present_dir[PATH_MAX];
static char g_absent_dir[PATH_MAX];
static int g_next_file;
static const char *g_failed_stat_path;
static char g_target_uuid[LIGHTNVR_UUID_STRING_SIZE];

int __real_stat(const char *path, struct stat *st);
int __real_lstat(const char *path, struct stat *st);

int __wrap_stat(const char *path, struct stat *st) {
    if (g_failed_stat_path && strcmp(path, g_failed_stat_path) == 0) {
        errno = EIO;
        return -1;
    }
    return __real_stat(path, st);
}

int __wrap_lstat(const char *path, struct stat *st) {
    if (g_failed_stat_path && strcmp(path, g_failed_stat_path) == 0) {
        errno = EIO;
        return -1;
    }
    return __real_lstat(path, st);
}

static void exec_sql(const char *sql) {
    char *error = NULL;
    int rc = sqlite3_exec(get_db_handle(), sql, NULL, NULL, &error);
    TEST_ASSERT_EQUAL_STRING("", error ? error : "");
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, rc);
}

static long query_count(const char *sql) {
    sqlite3_stmt *statement = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK,
        sqlite3_prepare_v2(get_db_handle(), sql, -1, &statement, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(statement));
    long value = (long)sqlite3_column_int64(statement, 0);
    sqlite3_finalize(statement);
    return value;
}

static void add_row(const char *path, bool complete, bool protected_row, const char *target) {
    char *sql = sqlite3_mprintf(
        "INSERT INTO recordings(stream_name,file_path,start_time,end_time,size_bytes,"
        "is_complete,protected,storage_target_uuid) VALUES('cam','%q',%lld,0,0,%d,%d,%Q);",
        path, (long long)time(NULL) - 7200, complete, protected_row, target);
    TEST_ASSERT_NOT_NULL(sql);
    exec_sql(sql);
    sqlite3_free(sql);
}

static void next_path(char *out, size_t size, const char *directory) {
    int written = snprintf(out, size, "%s/recording_%d.mp4", directory, g_next_file++);
    TEST_ASSERT_TRUE(written > 0 && (size_t)written < size);
}

static void add_missing_rows(const char *directory, int count) {
    char path[PATH_MAX];
    exec_sql("BEGIN;");
    for (int i = 0; i < count; i++) {
        next_path(path, sizeof(path), directory);
        add_row(path, false, false, NULL);
    }
    exec_sql("COMMIT;");
}

static void create_file(const char *path, const char *contents) {
    int fd = open(path, O_CREAT | O_TRUNC | O_WRONLY, 0644);
    TEST_ASSERT_TRUE(fd >= 0);
    size_t length = strlen(contents);
    TEST_ASSERT_EQUAL_INT((int)length, (int)write(fd, contents, length));
    close(fd);
}

void setUp(void) {
    g_failed_stat_path = NULL;
    exec_sql("DELETE FROM recordings;");
}

void tearDown(void) {
    g_failed_stat_path = NULL;
}

static void test_failed_media_lookup_keeps_the_file_and_row(void) {
    char path[PATH_MAX];
    next_path(path, sizeof(path), g_present_dir);
    create_file(path, "footage");
    add_row(path, false, false, NULL);
    g_failed_stat_path = path;

    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings());
    TEST_ASSERT_EQUAL_INT(1, query_count(
        "SELECT COUNT(*) FROM recordings WHERE is_complete=0 AND deletion_pending=0;"));
    TEST_ASSERT_EQUAL_INT(0, access(path, F_OK));
    g_failed_stat_path = NULL;
    unlink(path);
}

static void test_nonregular_media_is_retained_without_scheduling_deletion(void) {
    char path[PATH_MAX];
    next_path(path, sizeof(path), g_present_dir);
    TEST_ASSERT_EQUAL_INT(0, symlink("missing.mp4", path));
    add_row(g_present_dir, false, false, NULL);
    add_row(path, false, false, NULL);

    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings());
    TEST_ASSERT_EQUAL_INT(2, query_count(
        "SELECT COUNT(*) FROM recordings WHERE deletion_pending=0;"));
    unlink(path);
}

static void test_available_target_media_is_reconciled(void) {
    char missing[PATH_MAX], empty[PATH_MAX], survivor[PATH_MAX];
    next_path(missing, sizeof(missing), g_present_dir);
    next_path(empty, sizeof(empty), g_present_dir);
    next_path(survivor, sizeof(survivor), g_present_dir);
    create_file(empty, "");
    create_file(survivor, "footage");
    const char *paths[] = {missing, empty, survivor};
    for (size_t i = 0; i < sizeof(paths) / sizeof(paths[0]); i++) {
        add_row(paths[i], false, false, g_target_uuid);
        char *sql = sqlite3_mprintf(
            "UPDATE recordings SET object_key=%Q WHERE file_path=%Q;",
            strrchr(paths[i], '/') + 1, paths[i]);
        exec_sql(sql);
        sqlite3_free(sql);
    }

    TEST_ASSERT_EQUAL_INT(3, storage_reconcile_incomplete_recordings());
    TEST_ASSERT_EQUAL_INT(1, query_count("SELECT COUNT(*) FROM recordings;"));
    TEST_ASSERT_EQUAL_INT(7, query_count(
        "SELECT size_bytes FROM recordings WHERE is_complete=1;"));
    TEST_ASSERT_EQUAL_INT(-1, access(empty, F_OK));
    TEST_ASSERT_EQUAL_INT(0, access(survivor, F_OK));
    unlink(survivor);
}

static void test_inconsistent_target_identity_cannot_delete_other_footage(void) {
    char path[PATH_MAX], actual_path[PATH_MAX];
    next_path(path, sizeof(path), g_present_dir);
    next_path(actual_path, sizeof(actual_path), g_present_dir);
    create_file(path, "");
    create_file(actual_path, "footage");
    add_row(path, false, false, g_target_uuid);
    char *sql = sqlite3_mprintf("UPDATE recordings SET object_key=%Q;",
                                strrchr(actual_path, '/') + 1);
    exec_sql(sql);
    sqlite3_free(sql);

    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings());
    TEST_ASSERT_EQUAL_INT(1, query_count(
        "SELECT COUNT(*) FROM recordings WHERE deletion_pending=0;"));
    TEST_ASSERT_EQUAL_INT(0, access(actual_path, F_OK));
    unlink(path);
    unlink(actual_path);
}

static void test_empty_file_on_unmounted_target_is_retained(void) {
    char path[PATH_MAX];
    next_path(path, sizeof(path), g_present_dir);
    create_file(path, "");
    add_row(path, false, false, g_target_uuid);
    char *sql = sqlite3_mprintf(
        "UPDATE recordings SET object_key=%Q;"
        "UPDATE storage_targets SET mount_required=1,mount_guard_path=%Q WHERE uuid=%Q;",
        strrchr(path, '/') + 1, g_absent_dir, g_target_uuid);
    exec_sql(sql);
    sqlite3_free(sql);

    int reconciled = storage_reconcile_incomplete_recordings();
    exec_sql("UPDATE storage_targets SET mount_required=0;");
    TEST_ASSERT_EQUAL_INT(0, reconciled);
    TEST_ASSERT_EQUAL_INT(1, query_count(
        "SELECT COUNT(*) FROM recordings WHERE deletion_pending=0;"));
    TEST_ASSERT_EQUAL_INT(0, access(path, F_OK));
    unlink(path);
}

static void test_full_batches_of_phantom_rows_finish_and_reach_newer_footage(void) {
    add_missing_rows(g_present_dir, BATCH_SIZE * 2 + 1);
    char survivor[PATH_MAX];
    next_path(survivor, sizeof(survivor), g_present_dir);
    create_file(survivor, "footage");
    add_row(survivor, false, false, NULL);

    TEST_ASSERT_EQUAL_INT(BATCH_SIZE * 2 + 2, storage_reconcile_incomplete_recordings());

    TEST_ASSERT_EQUAL_INT(1, query_count("SELECT COUNT(*) FROM recordings;"));
    TEST_ASSERT_EQUAL_INT(7, query_count(
        "SELECT size_bytes FROM recordings WHERE is_complete=1;"));
    TEST_ASSERT_EQUAL_INT(0, access(survivor, F_OK));
    unlink(survivor);
}

static void test_rows_on_an_absent_volume_survive_without_stopping_the_scan(void) {
    add_missing_rows(g_absent_dir, BATCH_SIZE);
    char survivor[PATH_MAX];
    next_path(survivor, sizeof(survivor), g_present_dir);
    create_file(survivor, "footage");
    add_row(survivor, false, false, NULL);

    TEST_ASSERT_EQUAL_INT(1, storage_reconcile_incomplete_recordings());

    TEST_ASSERT_EQUAL_INT(BATCH_SIZE, query_count(
        "SELECT COUNT(*) FROM recordings WHERE is_complete=0 AND deletion_pending=0;"));
    TEST_ASSERT_EQUAL_INT(1, query_count("SELECT COUNT(*) FROM recordings WHERE is_complete=1;"));
    unlink(survivor);
}

static void test_empty_leftover_is_removed_with_its_row(void) {
    char path[PATH_MAX];
    next_path(path, sizeof(path), g_present_dir);
    create_file(path, "");
    add_row(path, false, false, NULL);

    TEST_ASSERT_EQUAL_INT(1, storage_reconcile_incomplete_recordings());

    TEST_ASSERT_EQUAL_INT(0, query_count("SELECT COUNT(*) FROM recordings;"));
    TEST_ASSERT_EQUAL_INT(-1, access(path, F_OK));
}

static void test_protected_and_unresolvable_target_rows_are_retained(void) {
    char protected_path[PATH_MAX], target_path[PATH_MAX];
    next_path(protected_path, sizeof(protected_path), g_present_dir);
    next_path(target_path, sizeof(target_path), g_present_dir);
    add_row(protected_path, false, true, NULL);
    exec_sql("PRAGMA foreign_keys = OFF;");
    add_row(target_path, false, false, "00000000-0000-4000-8000-000000000000");
    exec_sql("PRAGMA foreign_keys = ON;");

    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings());

    TEST_ASSERT_EQUAL_INT(2, query_count(
        "SELECT COUNT(*) FROM recordings WHERE deletion_pending=0;"));
}

static void test_complete_rows_are_out_of_scope(void) {
    char path[PATH_MAX];
    next_path(path, sizeof(path), g_present_dir);
    add_row(path, true, false, NULL);
    uint64_t id = (uint64_t)query_count("SELECT MAX(id) FROM recordings;");

    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings());
    TEST_ASSERT_EQUAL_INT(-2, storage_recording_delete_stale_incomplete(id));

    TEST_ASSERT_EQUAL_INT(1, query_count("SELECT COUNT(*) FROM recordings;"));
}

static void test_ordinary_deletion_still_refuses_incomplete_rows(void) {
    char path[PATH_MAX];
    next_path(path, sizeof(path), g_present_dir);
    add_row(path, false, false, NULL);
    uint64_t id = (uint64_t)query_count("SELECT MAX(id) FROM recordings;");

    TEST_ASSERT_EQUAL_INT(-2, storage_recording_delete(id, "manual deletion", NULL));
    TEST_ASSERT_EQUAL_INT(-2, delete_recording_metadata(id));

    TEST_ASSERT_EQUAL_INT(1, query_count("SELECT COUNT(*) FROM recordings;"));
}

int main(void) {
    strcpy(g_root, "/tmp/lightnvr_reconcile_XXXXXX");
    if (!mkdtemp(g_root)) return 1;
    snprintf(g_db_path, sizeof(g_db_path), "%s/test.db", g_root);
    snprintf(g_present_dir, sizeof(g_present_dir), "%s/present", g_root);
    snprintf(g_absent_dir, sizeof(g_absent_dir), "%s/unmounted/cam", g_root);
    if (mkdir(g_present_dir, 0755) != 0) return 1;
    safe_strcpy(g_config.storage_path, g_root, sizeof(g_config.storage_path), 0);
    if (init_database(g_db_path) != 0) return 1;
    storage_target_t target = {0};
    safe_strcpy(target.name, "Reconcile test", sizeof(target.name), 0);
    safe_strcpy(target.root_path, g_present_dir, sizeof(target.root_path), 0);
    strcpy(target.target_type, "filesystem");
    strcpy(target.storage_class, "hot");
    target.enabled = true;
    target.high_watermark_pct = 90;
    target.low_watermark_pct = 80;
    if (db_storage_target_create(&target) != DB_STORAGE_TARGET_OK) return 1;
    safe_strcpy(g_target_uuid, target.uuid, sizeof(g_target_uuid), 0);

    UNITY_BEGIN();
    RUN_TEST(test_full_batches_of_phantom_rows_finish_and_reach_newer_footage);
    RUN_TEST(test_rows_on_an_absent_volume_survive_without_stopping_the_scan);
    RUN_TEST(test_empty_leftover_is_removed_with_its_row);
    RUN_TEST(test_protected_and_unresolvable_target_rows_are_retained);
    RUN_TEST(test_complete_rows_are_out_of_scope);
    RUN_TEST(test_ordinary_deletion_still_refuses_incomplete_rows);
    RUN_TEST(test_failed_media_lookup_keeps_the_file_and_row);
    RUN_TEST(test_nonregular_media_is_retained_without_scheduling_deletion);
    RUN_TEST(test_available_target_media_is_reconciled);
    RUN_TEST(test_inconsistent_target_identity_cannot_delete_other_footage);
    RUN_TEST(test_empty_file_on_unmounted_target_is_retained);
    int result = UNITY_END();

    shutdown_database();
    char command[PATH_MAX + 16];
    snprintf(command, sizeof(command), "rm -rf %s", g_root);
    if (system(command) != 0) result = 1;
    return result;
}
