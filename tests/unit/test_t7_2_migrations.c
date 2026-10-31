#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>
#include <sqlite3.h>
#include "unity.h"
#include "database/sqlite_migrate.h"

static char root[256], directory[320], path[384], outside[384];
static sqlite3 *db;
static sqlite_migrate_t *migration;

static const char valid_sql[] =
    "-- migrate:up\nCREATE TABLE evidence(value TEXT);\n"
    "INSERT INTO evidence VALUES('preserved');\n"
    "-- migrate:down\nDROP TABLE evidence;\n";

static void write_fixture(const char *name, const char *sql) {
    FILE *file = fopen(name, "w");
    TEST_ASSERT_NOT_NULL(file);
    TEST_ASSERT_EQUAL(strlen(sql), fwrite(sql, 1, strlen(sql), file));
    TEST_ASSERT_EQUAL_INT(0, fclose(file));
    TEST_ASSERT_EQUAL_INT(0, chmod(name, 0600));
}

static int scalar(const char *sql) {
    sqlite3_stmt *statement = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(db, sql, -1, &statement, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(statement));
    int value = sqlite3_column_int(statement, 0);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_finalize(statement));
    return value;
}

static void open_migrations(void) {
    migrate_config_t config = { .migrations_dir = directory };
    migration = migrate_init(db, &config);
    TEST_ASSERT_NOT_NULL(migration);
}

void setUp(void) {
    strcpy(root, "/tmp/lightnvr-migration-boundary-XXXXXX");
    TEST_ASSERT_NOT_NULL(mkdtemp(root));
    snprintf(directory, sizeof(directory), "%s/migrations", root);
    snprintf(path, sizeof(path), "%s/0001_evidence.sql", directory);
    snprintf(outside, sizeof(outside), "%s/migrations-outside.sql", root);
    TEST_ASSERT_EQUAL_INT(0, mkdir(directory, 0700));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_open(":memory:", &db));
    migration = NULL;
}

void tearDown(void) {
    migrate_free(migration);
    sqlite3_close(db);
    unlink(path);
    unlink(outside);
    rmdir(directory);
    rmdir(root);
}

static void assert_rejected_without_schema_effect(void) {
    migrate_stats_t stats;
    TEST_ASSERT_EQUAL_INT(-1, migrate_up(migration, &stats));
    TEST_ASSERT_EQUAL_INT(1, stats.failed);
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM sqlite_master WHERE name='evidence'"));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM schema_migrations"));
}

void test_regular_private_migration_applies_and_rolls_back(void) {
    write_fixture(path, valid_sql);
    open_migrations();
    migrate_stats_t stats;
    TEST_ASSERT_EQUAL_INT(0, migrate_up(migration, &stats));
    TEST_ASSERT_EQUAL_INT(1, stats.applied);
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM evidence WHERE value='preserved'"));
    TEST_ASSERT_EQUAL_INT(1, migrate_down(migration));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM sqlite_master WHERE name='evidence'"));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM schema_migrations"));
}

void test_outside_symlink_is_not_executed(void) {
    write_fixture(outside, valid_sql);
    TEST_ASSERT_EQUAL_INT(0, symlink(outside, path));
    open_migrations();
    migrate_stats_t stats;
    int result = migrate_up(migration, &stats);
    /* readdir may identify a symlink immediately or report DT_UNKNOWN. */
    if (stats.total == 0) TEST_ASSERT_EQUAL_INT(0, result);
    else { TEST_ASSERT_EQUAL_INT(-1, result); TEST_ASSERT_EQUAL_INT(1, stats.failed); }
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM sqlite_master WHERE name='evidence'"));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM schema_migrations"));
    TEST_ASSERT_EQUAL_INT(0, access(outside, F_OK));
}

void test_world_writable_migration_is_not_executed(void) {
    write_fixture(path, valid_sql);
    TEST_ASSERT_EQUAL_INT(0, chmod(path, 0666));
    open_migrations();
    assert_rejected_without_schema_effect();
}

void test_file_replaced_after_discovery_is_rechecked_before_execution(void) {
    write_fixture(path, valid_sql);
    write_fixture(outside, valid_sql);
    open_migrations();
    TEST_ASSERT_EQUAL_INT(0, unlink(path));
    char traversal[512];
    snprintf(traversal, sizeof(traversal), "%s/../migrations-outside.sql", directory);
    TEST_ASSERT_EQUAL_INT(0, symlink(traversal, path));
    assert_rejected_without_schema_effect();
}

void test_disallowed_attach_rejects_entire_migration_before_effects(void) {
    char sql[1024];
    snprintf(sql, sizeof(sql), "-- migrate:up\nCREATE TABLE evidence(value TEXT);\n"
        "ATTACH DATABASE '%s' AS outside_db;\n-- migrate:down\nDROP TABLE evidence;\n", outside);
    write_fixture(path, sql);
    open_migrations();
    assert_rejected_without_schema_effect();
    TEST_ASSERT_EQUAL_INT(-1, access(outside, F_OK));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM pragma_database_list"));
}

void test_rollback_rechecks_replaced_file_and_preserves_applied_state(void) {
    write_fixture(path, valid_sql);
    open_migrations();
    TEST_ASSERT_EQUAL_INT(0, migrate_up(migration, NULL));
    write_fixture(outside, valid_sql);
    TEST_ASSERT_EQUAL_INT(0, unlink(path));
    TEST_ASSERT_EQUAL_INT(0, symlink(outside, path));
    TEST_ASSERT_EQUAL_INT(-1, migrate_down(migration));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM evidence WHERE value='preserved'"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM schema_migrations"));
}

void test_rollback_rechecks_world_writable_mode(void) {
    write_fixture(path, valid_sql);
    open_migrations();
    TEST_ASSERT_EQUAL_INT(0, migrate_up(migration, NULL));
    TEST_ASSERT_EQUAL_INT(0, chmod(path, 0666));
    TEST_ASSERT_EQUAL_INT(-1, migrate_down(migration));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM evidence"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM schema_migrations"));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_regular_private_migration_applies_and_rolls_back);
    RUN_TEST(test_outside_symlink_is_not_executed);
    RUN_TEST(test_world_writable_migration_is_not_executed);
    RUN_TEST(test_file_replaced_after_discovery_is_rechecked_before_execution);
    RUN_TEST(test_disallowed_attach_rejects_entire_migration_before_effects);
    RUN_TEST(test_rollback_rechecks_replaced_file_and_preserves_applied_state);
    RUN_TEST(test_rollback_rechecks_world_writable_mode);
    return UNITY_END();
}
