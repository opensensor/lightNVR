/** Saved-view listing must exhaust SQLite rows without crossing ownership. */
#define _POSIX_C_SOURCE 200809L
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <sqlite3.h>
#include "unity.h"
#include "database/db_core.h"
#include "database/db_fleet_saved_views.h"
#include "utils/strings.h"
#define TEST_DB_PATH "/tmp/lightnvr_unit_fleet_saved_views.db"
static fleet_saved_view_t create_view(const char *name, int64_t owner, bool shared) {
    fleet_saved_view_t view = {0};
    view.owner_user_id = owner; view.is_shared = shared;
    safe_strcpy(view.name, name, sizeof(view.name), 0);
    safe_strcpy(view.selector_json, "{\"version\":1,\"expression\":{\"op\":\"all\"}}", sizeof(view.selector_json), 0);
    safe_strcpy(view.columns_json, "[\"camera\"]", sizeof(view.columns_json), 0);
    safe_strcpy(view.sort_by, "name", sizeof(view.sort_by), 0);
    safe_strcpy(view.sort_order, "asc", sizeof(view.sort_order), 0);
    TEST_ASSERT_EQUAL_INT(DB_FLEET_SAVED_VIEW_OK, db_fleet_saved_view_create(&view));
    return view;
}
void setUp(void) { TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(get_db_handle(), "DELETE FROM fleet_saved_views;", NULL, NULL, NULL)); }
void tearDown(void) {}
static void test_multiple_owned_shared_and_bounded_rows(void) {
    fleet_saved_view_t mine_a = create_view("A private", 97001, false);
    create_view("B private", 97001, false);
    create_view("Shared", 97002, true);
    create_view("Other private", 97002, false);
    fleet_saved_view_t rows[4];
    TEST_ASSERT_EQUAL_INT(3, db_fleet_saved_view_list_visible(97001, rows, 4));
    TEST_ASSERT_TRUE(rows[0].is_shared);
    TEST_ASSERT_EQUAL_STRING(mine_a.uuid, rows[1].uuid);
    TEST_ASSERT_EQUAL_STRING("B private", rows[2].name);
    TEST_ASSERT_EQUAL_INT(2, db_fleet_saved_view_list_visible(97001, rows, 2));
    TEST_ASSERT_EQUAL_STRING(mine_a.uuid, rows[1].uuid);
    TEST_ASSERT_EQUAL_INT(2, db_fleet_saved_view_list_visible(97002, rows, 4));
    TEST_ASSERT_EQUAL_INT(1, db_fleet_saved_view_list_visible(0, rows, 4));
    TEST_ASSERT_EQUAL_INT(DB_FLEET_SAVED_VIEW_NOT_FOUND, db_fleet_saved_view_get_visible(97002, mine_a.uuid, &rows[0]));
}
static void test_empty_and_prepare_failure_are_distinct(void) {
    fleet_saved_view_t rows[2];
    TEST_ASSERT_EQUAL_INT(0, db_fleet_saved_view_list_visible(97001, rows, 2));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(get_db_handle(), "ALTER TABLE fleet_saved_views RENAME TO unavailable_views;", NULL, NULL, NULL));
    int result = db_fleet_saved_view_list_visible(97001, rows, 2);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(get_db_handle(), "ALTER TABLE unavailable_views RENAME TO fleet_saved_views;", NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(-1, result);
}
int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) return 1;
    if (sqlite3_exec(get_db_handle(), "INSERT INTO users(id,username,password_hash,salt,role) VALUES(97001,'fleet-owner-a','fixture','fixture',2),(97002,'fleet-owner-b','fixture','fixture',2);", NULL, NULL, NULL) != SQLITE_OK) {
        shutdown_database(); unlink(TEST_DB_PATH); return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_multiple_owned_shared_and_bounded_rows);
    RUN_TEST(test_empty_and_prepare_failure_are_distinct);
    int result = UNITY_END(); shutdown_database(); unlink(TEST_DB_PATH); return result;
}
