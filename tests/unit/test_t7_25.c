#define _GNU_SOURCE
#define _POSIX_C_SOURCE 200809L
/* Actual private cleanup helper, with public DB definitions linked from here. */
#include "../../src/database/db_core.c"
#include "unity.h"

static char root[128], backup_dir[256], moved_dir[256], artifact[320], sentinel[256];
static bool armed, replaced, leaf_swap, regular_swap, fixture_config_loaded;
static sqlite3 *fixture_db;
static config_t saved_config;
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
    strcpy(root, "/tmp/lightnvr-t7-25-XXXXXX"); TEST_ASSERT_NOT_NULL(mkdtemp(root));
    snprintf(backup_dir, sizeof(backup_dir), "%s/backup", root);
    snprintf(moved_dir, sizeof(moved_dir), "%s/inspected", root);
    snprintf(artifact, sizeof(artifact), "%s/%s", backup_dir, artifact_name);
    snprintf(sentinel, sizeof(sentinel), "%s/sentinel", root);
    TEST_ASSERT_EQUAL_INT(0, mkdir(backup_dir, 0700));
    put_file(artifact, "stale partial backup"); put_file(sentinel, "sentinel unchanged");
    armed = replaced = leaf_swap = regular_swap = false;
}
void tearDown(void) {
    armed = false;
    if (get_db_handle()) shutdown_database();
    if (fixture_db) { sqlite3_close_v2(fixture_db); fixture_db = NULL; }
    if (fixture_config_loaded) free(g_config.streams);
    g_config = saved_config;
    remove_owned_tree(root);
    TEST_ASSERT_EQUAL_INT(-1, access(root, F_OK));
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
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_normal_stale_cleanup_preserves_completed_unrelated_and_symlinks);
    RUN_TEST(test_parent_replacement_cannot_redirect_stale_cleanup);
    RUN_TEST(test_leaf_symlink_replacement_does_not_remove_unvalidated_entry);
    RUN_TEST(test_leaf_regular_replacement_preserves_new_contents);
    RUN_TEST(test_missing_directory_is_noop);
    RUN_TEST(test_non_directory_returns_error);
    RUN_TEST(test_readonly_open_backup_restore_preserve_database_contents);
    return UNITY_END();
}
