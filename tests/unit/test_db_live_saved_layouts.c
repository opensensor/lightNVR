/**
 * @file test_db_live_saved_layouts.c
 * @brief Server-backed operator Live layout persistence tests.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include <sqlite3.h>

#include "unity.h"
#include "database/db_core.h"
#include "database/db_live_saved_layouts.h"
#include "utils/strings.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_live_layouts.db"

static live_saved_layout_t make_layout(const char *name, int columns,
                                       int rows) {
    live_saved_layout_t layout;
    memset(&layout, 0, sizeof(layout));
    safe_strcpy(layout.name, name, sizeof(layout.name), 0);
    safe_strcpy(layout.availability, "live",
                sizeof(layout.availability), 0);
    safe_strcpy(layout.camera_slots_json, "[]",
                sizeof(layout.camera_slots_json), 0);
    layout.columns = columns;
    layout.rows = rows;
    return layout;
}

void setUp(void) {
    sqlite3_exec(get_db_handle(), "DELETE FROM live_saved_layouts;",
                 NULL, NULL, NULL);
}

void tearDown(void) {}

void test_create_list_update_and_delete_installation_layouts(void) {
    live_saved_layout_t first = make_layout("Main desk", 3, 2);
    live_saved_layout_t second = make_layout("Overnight", 2, 2);
    TEST_ASSERT_EQUAL_INT(DB_LIVE_LAYOUT_OK,
                          db_live_layout_create(&first));
    TEST_ASSERT_EQUAL_INT(DB_LIVE_LAYOUT_OK,
                          db_live_layout_create(&second));
    TEST_ASSERT_EQUAL_INT64(1, first.revision);

    live_saved_layout_t visible[4];
    TEST_ASSERT_EQUAL_INT(2, db_live_layout_list_visible(0, visible, 4));

    first.columns = 4;
    first.rows = 2;
    TEST_ASSERT_EQUAL_INT(DB_LIVE_LAYOUT_OK,
                          db_live_layout_update(&first, first.revision));
    TEST_ASSERT_EQUAL_INT64(2, first.revision);
    TEST_ASSERT_EQUAL_INT(DB_LIVE_LAYOUT_STALE,
                          db_live_layout_delete(0, first.uuid, 1));
    TEST_ASSERT_EQUAL_INT(DB_LIVE_LAYOUT_OK,
                          db_live_layout_delete(0, first.uuid,
                                                first.revision));
    TEST_ASSERT_EQUAL_INT(1, db_live_layout_list_visible(0, visible, 4));
}

void test_rejects_invalid_capacity_and_availability(void) {
    live_saved_layout_t layout = make_layout("Too large", 9, 9);
    TEST_ASSERT_EQUAL_INT(DB_LIVE_LAYOUT_INVALID,
                          db_live_layout_create(&layout));
    layout = make_layout("Bad availability", 2, 2);
    safe_strcpy(layout.availability, "sometimes",
                sizeof(layout.availability), 0);
    TEST_ASSERT_EQUAL_INT(DB_LIVE_LAYOUT_INVALID,
                          db_live_layout_create(&layout));
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_create_list_update_and_delete_installation_layouts);
    RUN_TEST(test_rejects_invalid_capacity_and_availability);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
