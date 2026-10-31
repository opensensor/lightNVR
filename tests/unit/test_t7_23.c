#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>
#include <sqlite3.h>
#include "unity.h"
#include "database/sqlite_migrate.h"

static char root[128], directory[192], path[256], outside[256];
static sqlite3 *db;
static sqlite_migrate_t *migration;
static char *allocated_sql;
static const char ordinary_sql[] =
    "-- migrate:up\nCREATE TABLE evidence(value TEXT); INSERT INTO evidence VALUES('kept');\n"
    "-- migrate:down\nDROP TABLE evidence;\n";

static void write_fixture(const char *sql) {
    FILE *file = fopen(path, "w"); TEST_ASSERT_NOT_NULL(file);
    size_t count = fwrite(sql, 1, strlen(sql), file); int closed = fclose(file);
    TEST_ASSERT_EQUAL_UINT(strlen(sql), count); TEST_ASSERT_EQUAL_INT(0, closed);
    TEST_ASSERT_EQUAL_INT(0, chmod(path, 0600));
}
static int scalar(const char *sql) {
    sqlite3_stmt *statement = NULL;
    int prepared = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    int stepped = prepared == SQLITE_OK ? sqlite3_step(statement) : prepared;
    int value = stepped == SQLITE_ROW ? sqlite3_column_int(statement, 0) : -1;
    if (statement) sqlite3_finalize(statement);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, prepared); TEST_ASSERT_EQUAL_INT(SQLITE_ROW, stepped);
    return value;
}
static bool table_exists(const char *name) {
    sqlite3_stmt *statement = NULL;
    int prepared = sqlite3_prepare_v2(db, "SELECT count(*) FROM sqlite_master WHERE type='table' AND name=?", -1, &statement, NULL);
    int bound = prepared == SQLITE_OK ? sqlite3_bind_text(statement, 1, name, -1, SQLITE_TRANSIENT) : prepared;
    int stepped = bound == SQLITE_OK ? sqlite3_step(statement) : bound;
    int count = stepped == SQLITE_ROW ? sqlite3_column_int(statement, 0) : -1;
    if (statement) sqlite3_finalize(statement);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, prepared); TEST_ASSERT_EQUAL_INT(SQLITE_OK, bound);
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, stepped); return count == 1;
}
static void assert_sentinel(void) {
    TEST_ASSERT_TRUE(table_exists("sentinel"));
    TEST_ASSERT_EQUAL_INT(73, scalar("SELECT value FROM sentinel"));
}
static void open_migrations(const char *tracking_table) {
    migrate_config_t config = {.migrations_dir = directory, .migrations_table = tracking_table};
    migration = migrate_init(db, &config); TEST_ASSERT_NOT_NULL(migration);
}
void setUp(void) {
    migration = NULL; db = NULL; allocated_sql = NULL;
    strcpy(root, "/tmp/lightnvr-t7-23-XXXXXX"); TEST_ASSERT_NOT_NULL(mkdtemp(root));
    snprintf(directory, sizeof(directory), "%s/migrations", root);
    snprintf(path, sizeof(path), "%s/0001_identifiers.sql", directory);
    snprintf(outside, sizeof(outside), "%s/forbidden.sqlite", root);
    TEST_ASSERT_EQUAL_INT(0, mkdir(directory, 0700));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_open(":memory:", &db));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(db, "CREATE TABLE sentinel(value INTEGER); INSERT INTO sentinel VALUES(73);", NULL, NULL, NULL));
}
void tearDown(void) {
    migrate_free(migration); migration = NULL;
    sqlite3_free(allocated_sql); allocated_sql = NULL;
    if (db) sqlite3_close(db);
    db = NULL;
    unlink(path); unlink(outside); rmdir(directory); rmdir(root);
}
static void test_valid_tracking_name_applies_reloads_and_rolls_back(void) {
    write_fixture(ordinary_sql); open_migrations("custom_tracking");
    TEST_ASSERT_EQUAL_INT(0, migrate_up(migration, NULL));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM custom_tracking"));
    migrate_free(migration); migration = NULL; open_migrations("custom_tracking");
    char version[MIGRATE_VERSION_LEN]; TEST_ASSERT_EQUAL_INT(0, migrate_get_version(migration, version, sizeof(version)));
    TEST_ASSERT_EQUAL_STRING("0001", version);
    TEST_ASSERT_EQUAL_INT(1, migrate_down(migration));
    TEST_ASSERT_FALSE(table_exists("evidence"));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM custom_tracking")); assert_sentinel();
}
static void quoted_identifier_round_trip(const char *name) {
    allocated_sql = sqlite3_mprintf("-- migrate:up\nCREATE TABLE \"%w\"(value TEXT);\n"
        "-- migrate:down\nDROP TABLE \"%w\";\n", name, name);
    TEST_ASSERT_NOT_NULL(allocated_sql);
    /* SQLite itself accepts this quoted identifier; this is not malformed SQL. */
    char *control = sqlite3_mprintf("CREATE TABLE \"%w\"(value TEXT);", name);
    TEST_ASSERT_NOT_NULL(control);
    sqlite3_stmt *statement = NULL;
    int prepared = sqlite3_prepare_v2(db, control, -1, &statement, NULL);
    if (statement) sqlite3_finalize(statement);
    sqlite3_free(control);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, prepared);
    write_fixture(allocated_sql); open_migrations(NULL);
    int applied = migrate_up(migration, NULL); assert_sentinel();
    TEST_ASSERT_EQUAL_INT(0, applied); TEST_ASSERT_TRUE(table_exists(name));
    TEST_ASSERT_EQUAL_INT(1, migrate_down(migration));
    TEST_ASSERT_FALSE(table_exists(name)); assert_sentinel();
}
static void test_double_quote_in_identifier_round_trips(void) { quoted_identifier_round_trip("quoted\"identifier"); }
static void test_apostrophe_in_quoted_identifier_round_trips(void) { quoted_identifier_round_trip("quoted'identifier"); }
static void test_semicolon_and_comment_text_in_quoted_identifier_round_trip(void) {
    quoted_identifier_round_trip("semi;--/*identifier*/");
}
static void statement_round_trip(const char *up, const char *down) {
    allocated_sql = sqlite3_mprintf("-- migrate:up\n%s\n-- migrate:down\n%s\n", up, down);
    TEST_ASSERT_NOT_NULL(allocated_sql);
    write_fixture(allocated_sql); open_migrations(NULL);
    TEST_ASSERT_EQUAL_INT(0, migrate_up(migration, NULL)); assert_sentinel();
    TEST_ASSERT_EQUAL_INT(1, migrate_down(migration)); assert_sentinel();
}
static void test_backtick_identifier_and_escaped_delimiter_round_trip(void) {
    statement_round_trip("CREATE TABLE `semi;quo``te'name`(value TEXT);",
                         "DROP TABLE `semi;quo``te'name`;");
}
static void test_bracket_identifier_round_trip(void) {
    statement_round_trip("CREATE TABLE [semi;quote'name](value TEXT);",
                         "DROP TABLE [semi;quote'name];");
}
static void test_inline_comments_do_not_change_statement_boundaries(void) {
    statement_round_trip("CREATE TABLE evidence /* '; END */ (value TEXT);\n"
                         "INSERT INTO evidence VALUES('it''s; kept'); -- '; BEGIN\n"
                         "UPDATE evidence SET value='changed';",
                         "DROP TABLE evidence /* '; BEGIN */;");
}
static void test_trigger_quoted_keywords_and_nested_case_round_trip(void) {
    statement_round_trip("CREATE TABLE evidence(value TEXT);"
                         "CREATE TRIGGER \"BEGIN;END\" AFTER INSERT ON evidence BEGIN "
                         "UPDATE evidence SET value=CASE WHEN new.value='it''s;' THEN 'END' ELSE 'BEGIN' END; END;"
                         "INSERT INTO evidence VALUES('it''s;');",
                         "DROP TRIGGER \"BEGIN;END\"; DROP TABLE evidence;");
}
static void trigger_following_statement_is_validated(const char *name) {
    allocated_sql = sqlite3_mprintf("-- migrate:up\nCREATE TABLE evidence(value TEXT);"
        "CREATE TRIGGER %s AFTER INSERT ON evidence BEGIN SELECT 1; END;"
        "ATTACH DATABASE '%q' AS escaped;\n-- migrate:down\nDROP TABLE evidence;\n", name, outside);
    TEST_ASSERT_NOT_NULL(allocated_sql); write_fixture(allocated_sql); open_migrations(NULL);
    int result = migrate_up(migration, NULL);
    assert_sentinel(); TEST_ASSERT_EQUAL_INT(-1, access(outside, F_OK));
    TEST_ASSERT_EQUAL_INT(-1, result); TEST_ASSERT_FALSE(table_exists("evidence"));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM schema_migrations"));
}
static void test_trigger_named_begin_does_not_hide_following_statement(void) {
    trigger_following_statement_is_validated("BEGIN");
}
static void test_trigger_dollar_identifier_does_not_hide_following_statement(void) {
    trigger_following_statement_is_validated("BEGIN$name");
}
static void test_trigger_unicode_identifier_does_not_hide_following_statement(void) {
    trigger_following_statement_is_validated("BEGIN\xc3\xa9");
}
static void test_trigger_keyword_name_and_terminator_comments_round_trip(void) {
    statement_round_trip("CREATE TABLE evidence(value TEXT);"
                         "CREATE TRIGGER BEGIN AFTER INSERT ON evidence BEGIN "
                         "SELECT CASE WHEN 1 THEN 'END;' ELSE ';END;' END; /* ; END */ END /* ; */;"
                         "INSERT INTO evidence VALUES('kept');",
                         "DROP TRIGGER BEGIN; DROP TABLE evidence;");
}
static void test_up_identifier_quote_cannot_hide_disallowed_attach(void) {
    char sql[1024];
    int length = snprintf(sql, sizeof(sql), "-- migrate:up\n"
        "CREATE TABLE \"quoted'name\"(value TEXT); ATTACH DATABASE '%s' AS escaped;\n"
        "-- migrate:down\nDROP TABLE \"quoted'name\";\n", outside);
    TEST_ASSERT_TRUE(length > 0 && (size_t)length < sizeof(sql));
    write_fixture(sql); open_migrations(NULL);
    int applied = migrate_up(migration, NULL);
    assert_sentinel(); TEST_ASSERT_EQUAL_INT(-1, access(outside, F_OK));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM pragma_database_list"));
    TEST_ASSERT_EQUAL_INT(-1, applied); TEST_ASSERT_FALSE(table_exists("quoted'name"));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM schema_migrations"));
}
static void test_down_identifier_quote_cannot_hide_disallowed_attach(void) {
    char sql[1024];
    int length = snprintf(sql, sizeof(sql), "-- migrate:up\nCREATE TABLE \"quoted'name\"(value TEXT);\n"
        "-- migrate:down\nDROP TABLE \"quoted'name\"; ATTACH DATABASE '%s' AS escaped;\n", outside);
    TEST_ASSERT_TRUE(length > 0 && (size_t)length < sizeof(sql));
    write_fixture(sql); open_migrations(NULL);
    TEST_ASSERT_EQUAL_INT(0, migrate_up(migration, NULL));
    int rolled_back = migrate_down(migration);
    assert_sentinel(); TEST_ASSERT_EQUAL_INT(-1, access(outside, F_OK));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM pragma_database_list"));
    TEST_ASSERT_EQUAL_INT(-1, rolled_back); TEST_ASSERT_TRUE(table_exists("quoted'name"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM schema_migrations"));
}
static void test_tracking_identifier_cannot_execute_second_statement(void) {
    migrate_config_t config = {.migrations_table = "injected(version TEXT); DROP TABLE sentinel; --"};
    migration = migrate_init(db, &config);
    /* Either literal treatment or rejection is safe; execution of fragments is not. */
    assert_sentinel(); TEST_ASSERT_FALSE(table_exists("injected"));
}
static void test_tracking_quote_and_comment_names_cannot_change_unrelated_schema(void) {
    const char *names[] = {"quote\"tracking", "tracking;--comment", "tracking/*comment*/"};
    for (size_t index = 0; index < sizeof(names)/sizeof(names[0]); ++index) {
        migrate_config_t config = {.migrations_table = names[index]};
        migration = migrate_init(db, &config); assert_sentinel();
        migrate_free(migration); migration = NULL;
    }
}
static void tracking_round_trip(const char *name) {
    write_fixture(ordinary_sql); open_migrations(name);
    TEST_ASSERT_TRUE(table_exists(name));
    TEST_ASSERT_EQUAL_INT(0, migrate_up(migration, NULL)); assert_sentinel();
    migrate_free(migration); migration = NULL; open_migrations(name);
    char version[MIGRATE_VERSION_LEN];
    TEST_ASSERT_EQUAL_INT(0, migrate_get_version(migration, version, sizeof(version)));
    TEST_ASSERT_EQUAL_STRING("0001", version);
    TEST_ASSERT_EQUAL_INT(1, migrate_down(migration)); assert_sentinel();
    TEST_ASSERT_FALSE(table_exists("evidence"));
}
static void test_quoted_tracking_name_round_trips_as_one_identifier(void) {
    tracking_round_trip("track\"ing; -- /*name*/ 'quoted'");
}
static void test_long_tracking_name_round_trips_without_truncation(void) {
    char name[1025]; memset(name, 'x', sizeof(name) - 1); name[sizeof(name) - 1] = '\0';
    tracking_round_trip(name);
}
static void test_migration_version_is_bound_as_data(void) {
    migration_t embedded = {.version = "v'; DROP TABLE sentinel;--", .description = "bound value",
        .sql_up = "CREATE TABLE evidence(value TEXT);", .sql_down = "DROP TABLE evidence;"};
    migrate_config_t config = {.embedded_migrations = &embedded, .embedded_count = 1};
    migration = migrate_init(db, &config); TEST_ASSERT_NOT_NULL(migration);
    TEST_ASSERT_EQUAL_INT(0, migrate_up(migration, NULL)); assert_sentinel();
    TEST_ASSERT_EQUAL_INT(1, migrate_down(migration)); assert_sentinel();
}
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_valid_tracking_name_applies_reloads_and_rolls_back);
    RUN_TEST(test_double_quote_in_identifier_round_trips);
    RUN_TEST(test_apostrophe_in_quoted_identifier_round_trips);
    RUN_TEST(test_semicolon_and_comment_text_in_quoted_identifier_round_trip);
    RUN_TEST(test_backtick_identifier_and_escaped_delimiter_round_trip);
    RUN_TEST(test_bracket_identifier_round_trip);
    RUN_TEST(test_inline_comments_do_not_change_statement_boundaries);
    RUN_TEST(test_trigger_quoted_keywords_and_nested_case_round_trip);
    RUN_TEST(test_trigger_named_begin_does_not_hide_following_statement);
    RUN_TEST(test_trigger_dollar_identifier_does_not_hide_following_statement);
    RUN_TEST(test_trigger_unicode_identifier_does_not_hide_following_statement);
    RUN_TEST(test_trigger_keyword_name_and_terminator_comments_round_trip);
    RUN_TEST(test_up_identifier_quote_cannot_hide_disallowed_attach);
    RUN_TEST(test_down_identifier_quote_cannot_hide_disallowed_attach);
    RUN_TEST(test_tracking_identifier_cannot_execute_second_statement);
    RUN_TEST(test_tracking_quote_and_comment_names_cannot_change_unrelated_schema);
    RUN_TEST(test_quoted_tracking_name_round_trips_as_one_identifier);
    RUN_TEST(test_long_tracking_name_round_trips_without_truncation);
    RUN_TEST(test_migration_version_is_bound_as_data);
    return UNITY_END();
}
