#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "unity.h"
#include "database/db_core.h"
#include "database/db_investigation_bookmarks.h"
#include "utils/strings.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_investigation_bookmarks.db"

static const char TEST_CAMERAS[][CAMERA_UUID_STRING_SIZE] = {
    "11111111-1111-4111-8111-111111111111",
    "22222222-2222-4222-8222-222222222222",
};

void setUp(void) {
    sqlite3_exec(get_db_handle(), "DELETE FROM investigation_bookmarks;",
                 NULL, NULL, NULL);
}

void tearDown(void) {}

static investigation_bookmark_t valid_bookmark(void) {
    investigation_bookmark_t bookmark;
    memset(&bookmark, 0, sizeof(bookmark));
    safe_strcpy(bookmark.title, "Loading dock review",
                sizeof(bookmark.title), 0);
    safe_strcpy(bookmark.note, "Check the delivery window.",
                sizeof(bookmark.note), 0);
    bookmark.start_time = 1700000000;
    bookmark.end_time = 1700000600;
    bookmark.cursor_time = 1700000120;
    safe_strcpy(bookmark.primary_camera_uuid, TEST_CAMERAS[1],
                sizeof(bookmark.primary_camera_uuid), 0);
    safe_strcpy(bookmark.filters_json, "{\"label\":\"person\"}",
                sizeof(bookmark.filters_json), 0);
    safe_strcpy(
        bookmark.representative_result_json,
        "{\"result_id\":\"detection:9\",\"camera_uuid\":"
        "\"22222222-2222-4222-8222-222222222222\"}",
        sizeof(bookmark.representative_result_json), 0);
    return bookmark;
}

void test_bookmark_crud_preserves_ordered_cameras_and_revision(void) {
    investigation_bookmark_t bookmark = valid_bookmark();
    TEST_ASSERT_EQUAL_INT(
        DB_INVESTIGATION_BOOKMARK_OK,
        db_investigation_bookmark_create(&bookmark, TEST_CAMERAS, 2));
    TEST_ASSERT_EQUAL_INT(1, bookmark.revision);
    TEST_ASSERT_EQUAL_INT(2, bookmark.camera_count);
    TEST_ASSERT_EQUAL_INT(1, db_investigation_bookmark_count(0));

    investigation_bookmark_t listed[2];
    TEST_ASSERT_EQUAL_INT(
        1, db_investigation_bookmark_list(0, listed, 2));
    TEST_ASSERT_EQUAL_STRING(bookmark.uuid, listed[0].uuid);

    char cameras[INVESTIGATION_BOOKMARK_MAX_CAMERAS]
                [CAMERA_UUID_STRING_SIZE] = {{0}};
    TEST_ASSERT_EQUAL_INT(
        2, db_investigation_bookmark_list_cameras(
               bookmark.uuid, cameras, INVESTIGATION_BOOKMARK_MAX_CAMERAS));
    TEST_ASSERT_EQUAL_STRING(TEST_CAMERAS[0], cameras[0]);
    TEST_ASSERT_EQUAL_STRING(TEST_CAMERAS[1], cameras[1]);

    safe_strcpy(bookmark.title, "Loading dock follow-up",
                sizeof(bookmark.title), 0);
    safe_strcpy(bookmark.note, "Reviewed once.", sizeof(bookmark.note), 0);
    TEST_ASSERT_EQUAL_INT(
        DB_INVESTIGATION_BOOKMARK_OK,
        db_investigation_bookmark_update_metadata(&bookmark, 1));
    TEST_ASSERT_EQUAL_INT(2, bookmark.revision);
    TEST_ASSERT_EQUAL_STRING("Loading dock follow-up", bookmark.title);
    TEST_ASSERT_EQUAL_INT(
        DB_INVESTIGATION_BOOKMARK_STALE,
        db_investigation_bookmark_update_metadata(&bookmark, 1));

    TEST_ASSERT_EQUAL_INT(
        DB_INVESTIGATION_BOOKMARK_STALE,
        db_investigation_bookmark_delete(0, bookmark.uuid, 1));
    TEST_ASSERT_EQUAL_INT(
        DB_INVESTIGATION_BOOKMARK_OK,
        db_investigation_bookmark_delete(0, bookmark.uuid, 2));
    TEST_ASSERT_EQUAL_INT(0, db_investigation_bookmark_count(0));
    TEST_ASSERT_EQUAL_INT(
        0, db_investigation_bookmark_list_cameras(
               bookmark.uuid, cameras, INVESTIGATION_BOOKMARK_MAX_CAMERAS));
}

void test_bookmark_rejects_duplicate_cameras_and_out_of_range_cursor(void) {
    investigation_bookmark_t bookmark = valid_bookmark();
    char duplicates[2][CAMERA_UUID_STRING_SIZE];
    safe_strcpy(duplicates[0], TEST_CAMERAS[0], sizeof(duplicates[0]), 0);
    safe_strcpy(duplicates[1], TEST_CAMERAS[0], sizeof(duplicates[1]), 0);
    safe_strcpy(bookmark.primary_camera_uuid, duplicates[0],
                sizeof(bookmark.primary_camera_uuid), 0);
    TEST_ASSERT_EQUAL_INT(
        DB_INVESTIGATION_BOOKMARK_INVALID,
        db_investigation_bookmark_create(&bookmark, duplicates, 2));

    bookmark = valid_bookmark();
    bookmark.cursor_time = bookmark.end_time + 1;
    TEST_ASSERT_EQUAL_INT(
        DB_INVESTIGATION_BOOKMARK_INVALID,
        db_investigation_bookmark_create(&bookmark, TEST_CAMERAS, 2));
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_bookmark_crud_preserves_ordered_cameras_and_revision);
    RUN_TEST(test_bookmark_rejects_duplicate_cameras_and_out_of_range_cursor);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
