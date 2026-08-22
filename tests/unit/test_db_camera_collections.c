/**
 * @file test_db_camera_collections.c
 * @brief Static and smart camera collection persistence tests.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <sqlite3.h>

#include "unity.h"
#include "database/db_camera_collections.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#include "utils/strings.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_camera_collections_test.db"

static stream_config_t create_camera(const char *name) {
    stream_config_t stream;
    memset(&stream, 0, sizeof(stream));
    safe_strcpy(stream.name, name, sizeof(stream.name), 0);
    safe_strcpy(stream.url, "rtsp://camera/live", sizeof(stream.url), 0);
    safe_strcpy(stream.codec, "h264", sizeof(stream.codec), 0);
    stream.enabled = true;
    stream.streaming_enabled = true;
    stream.record = true;
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&stream));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name(name, &stream));
    return stream;
}

static camera_collection_t make_collection(const char *name, const char *type) {
    camera_collection_t collection;
    memset(&collection, 0, sizeof(collection));
    safe_strcpy(collection.name, name, sizeof(collection.name), 0);
    safe_strcpy(collection.description, "Operator view",
                sizeof(collection.description), 0);
    safe_strcpy(collection.collection_type, type,
                sizeof(collection.collection_type), 0);
    collection.is_shared = true;
    if (strcmp(type, "smart") == 0) {
        safe_strcpy(collection.selector_json,
                    "{\"version\":1,\"expression\":{\"op\":\"enabled\",\"value\":true}}",
                    sizeof(collection.selector_json), 0);
    }
    return collection;
}

void setUp(void) {
    sqlite3 *db = get_db_handle();
    sqlite3_exec(db, "DELETE FROM camera_collections;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM streams;", NULL, NULL, NULL);
}

void tearDown(void) {}

void test_create_list_update_and_case_insensitive_conflict(void) {
    camera_collection_t collection = make_collection("Guard Tour A", "static");
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_OK,
                          db_camera_collection_create(&collection));
    TEST_ASSERT_TRUE(strlen(collection.uuid) == 36);
    TEST_ASSERT_EQUAL_INT(1, db_camera_collection_count());

    camera_collection_t duplicate = make_collection("guard tour a", "static");
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_CONFLICT,
                          db_camera_collection_create(&duplicate));

    safe_strcpy(collection.name, "Guard Tour North",
                sizeof(collection.name), 0);
    collection.is_shared = false;
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_OK,
                          db_camera_collection_update(&collection));
    TEST_ASSERT_EQUAL_STRING("Guard Tour North", collection.name);
    TEST_ASSERT_FALSE(collection.is_shared);

    camera_collection_t listed[2];
    TEST_ASSERT_EQUAL_INT(1, db_camera_collection_list(listed, 2));
    TEST_ASSERT_EQUAL_STRING(collection.uuid, listed[0].uuid);
}

void test_static_members_replace_deduplicate_and_cascade_camera_delete(void) {
    stream_config_t first = create_camera("First");
    stream_config_t second = create_camera("Second");
    camera_collection_t collection = make_collection("Static", "static");
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_OK,
                          db_camera_collection_create(&collection));
    const char *members[] = {
        first.camera_uuid, second.camera_uuid, first.camera_uuid
    };
    TEST_ASSERT_EQUAL_INT(
        DB_CAMERA_COLLECTION_OK,
        db_camera_collection_set_members(collection.uuid, members, 3));
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_OK,
                          db_camera_collection_get(collection.uuid, &collection));
    TEST_ASSERT_EQUAL_INT(2, collection.member_count);
    char listed[4][CAMERA_UUID_STRING_SIZE];
    TEST_ASSERT_EQUAL_INT(
        2, db_camera_collection_list_members(collection.uuid, listed, 4));

    sqlite3 *db = get_db_handle();
    sqlite3_stmt *delete_stmt = NULL;
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_prepare_v2(db, "DELETE FROM streams WHERE camera_uuid = ?;",
                           -1, &delete_stmt, NULL));
    sqlite3_bind_text(delete_stmt, 1, first.camera_uuid, -1, SQLITE_TRANSIENT);
    TEST_ASSERT_EQUAL_INT(SQLITE_DONE, sqlite3_step(delete_stmt));
    sqlite3_finalize(delete_stmt);
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_OK,
                          db_camera_collection_get(collection.uuid, &collection));
    TEST_ASSERT_EQUAL_INT(1, collection.member_count);
}

void test_switching_to_smart_clears_static_members(void) {
    stream_config_t camera = create_camera("Member");
    camera_collection_t collection = make_collection("Switchable", "static");
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_OK,
                          db_camera_collection_create(&collection));
    const char *members[] = {camera.camera_uuid};
    TEST_ASSERT_EQUAL_INT(
        DB_CAMERA_COLLECTION_OK,
        db_camera_collection_set_members(collection.uuid, members, 1));

    safe_strcpy(collection.collection_type, "smart",
                sizeof(collection.collection_type), 0);
    safe_strcpy(collection.selector_json,
                "{\"version\":1,\"expression\":{\"op\":\"all\"}}",
                sizeof(collection.selector_json), 0);
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_OK,
                          db_camera_collection_update(&collection));
    TEST_ASSERT_EQUAL_INT(0, collection.member_count);
    TEST_ASSERT_EQUAL_INT(
        DB_CAMERA_COLLECTION_WRONG_TYPE,
        db_camera_collection_set_members(collection.uuid, members, 1));
    char listed[2][CAMERA_UUID_STRING_SIZE];
    TEST_ASSERT_EQUAL_INT(
        -3, db_camera_collection_list_members(collection.uuid, listed, 2));
}

void test_rejects_invalid_smart_selector_and_unknown_member(void) {
    camera_collection_t invalid = make_collection("Invalid", "smart");
    safe_strcpy(invalid.selector_json,
                "{\"version\":1,\"expression\":{\"op\":\"sql\"}}",
                sizeof(invalid.selector_json), 0);
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_INVALID,
                          db_camera_collection_create(&invalid));

    camera_collection_t collection = make_collection("Known", "static");
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_OK,
                          db_camera_collection_create(&collection));
    const char *unknown[] = {
        "99999999-9999-4999-8999-999999999999"
    };
    TEST_ASSERT_EQUAL_INT(
        DB_CAMERA_COLLECTION_NOT_FOUND,
        db_camera_collection_set_members(collection.uuid, unknown, 1));
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_OK,
                          db_camera_collection_get(collection.uuid, &collection));
    TEST_ASSERT_EQUAL_INT(0, collection.member_count);
}

void test_delete_cascades_members_and_reports_not_found(void) {
    stream_config_t camera = create_camera("Delete Member");
    camera_collection_t collection = make_collection("Delete Me", "static");
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_OK,
                          db_camera_collection_create(&collection));
    const char *members[] = {camera.camera_uuid};
    TEST_ASSERT_EQUAL_INT(
        DB_CAMERA_COLLECTION_OK,
        db_camera_collection_set_members(collection.uuid, members, 1));
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_OK,
                          db_camera_collection_delete(collection.uuid));
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_NOT_FOUND,
                          db_camera_collection_get(collection.uuid, &collection));
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_NOT_FOUND,
                          db_camera_collection_delete(collection.uuid));
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_create_list_update_and_case_insensitive_conflict);
    RUN_TEST(test_static_members_replace_deduplicate_and_cascade_camera_delete);
    RUN_TEST(test_switching_to_smart_clears_static_members);
    RUN_TEST(test_rejects_invalid_smart_selector_and_unknown_member);
    RUN_TEST(test_delete_cascades_members_and_reports_not_found);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
