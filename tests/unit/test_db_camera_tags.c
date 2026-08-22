/**
 * @file test_db_camera_tags.c
 * @brief Layer 2 tests for normalized camera tag dictionary and assignments.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <sqlite3.h>

#include "unity.h"
#include "database/db_camera_tags.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#include "utils/strings.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_camera_tags_test.db"

static camera_tag_t make_tag(const char *label) {
    camera_tag_t tag;
    memset(&tag, 0, sizeof(tag));
    safe_strcpy(tag.label, label, sizeof(tag.label), 0);
    return tag;
}

static stream_config_t make_stream(const char *name, const char *tags) {
    stream_config_t stream;
    memset(&stream, 0, sizeof(stream));
    safe_strcpy(stream.name, name, sizeof(stream.name), 0);
    safe_strcpy(stream.url, "rtsp://camera/stream", sizeof(stream.url), 0);
    safe_strcpy(stream.codec, "h264", sizeof(stream.codec), 0);
    if (tags) safe_strcpy(stream.tags, tags, sizeof(stream.tags), 0);
    stream.enabled = true;
    stream.streaming_enabled = true;
    stream.width = 1920;
    stream.height = 1080;
    stream.fps = 25;
    return stream;
}

static stream_config_t add_and_load_stream(const char *name, const char *tags) {
    stream_config_t stream = make_stream(name, tags);
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&stream));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name(name, &stream));
    return stream;
}

void setUp(void) {
    sqlite3 *db = get_db_handle();
    sqlite3_exec(db, "DELETE FROM streams;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM camera_tags;", NULL, NULL, NULL);
}

void tearDown(void) {}

void test_create_tag_round_trips_metadata_and_rejects_case_duplicate(void) {
    camera_tag_t tag = make_tag("Outdoor");
    safe_strcpy(tag.color, "#22aa44", sizeof(tag.color), 0);
    safe_strcpy(tag.description, "Exterior cameras", sizeof(tag.description), 0);
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_TAG_OK, db_camera_tag_create(&tag));
    TEST_ASSERT_EQUAL_UINT(CAMERA_UUID_STRING_SIZE - 1, strlen(tag.uuid));
    TEST_ASSERT_EQUAL_STRING("Outdoor", tag.label);
    TEST_ASSERT_EQUAL_STRING("#22aa44", tag.color);
    TEST_ASSERT_EQUAL_STRING("Exterior cameras", tag.description);
    TEST_ASSERT_EQUAL_INT(0, tag.camera_count);

    camera_tag_t duplicate = make_tag("outdoor");
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_TAG_CONFLICT,
                          db_camera_tag_create(&duplicate));
    camera_tag_t ambiguous = make_tag("north,exterior");
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_TAG_INVALID,
                          db_camera_tag_create(&ambiguous));
    TEST_ASSERT_EQUAL_INT(1, db_camera_tag_count());
}

void test_legacy_stream_tags_are_normalized_losslessly(void) {
    stream_config_t stream =
        add_and_load_stream("legacy", " outdoor,Critical,outdoor ");
    camera_tag_t tags[4];
    int count = db_camera_tag_list_for_camera(stream.camera_uuid, tags, 4);
    TEST_ASSERT_EQUAL_INT(2, count);
    TEST_ASSERT_EQUAL_STRING("Critical", tags[0].label);
    TEST_ASSERT_EQUAL_STRING("outdoor", tags[1].label);

    /* Compatibility writes do not rewrite callers' original serialized form. */
    TEST_ASSERT_EQUAL_STRING(" outdoor,Critical,outdoor ", stream.tags);
}

void test_startup_backfill_repairs_legacy_only_changes(void) {
    stream_config_t stream = add_and_load_stream("backfill", NULL);
    sqlite3 *db = get_db_handle();
    sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_prepare_v2(db,
                           "UPDATE streams SET tags = 'North, south ,NORTH' "
                           "WHERE camera_uuid = ?;",
                           -1, &stmt, NULL));
    sqlite3_bind_text(stmt, 1, stream.camera_uuid, -1, SQLITE_TRANSIENT);
    TEST_ASSERT_EQUAL_INT(SQLITE_DONE, sqlite3_step(stmt));
    sqlite3_finalize(stmt);

    TEST_ASSERT_EQUAL_INT(0, db_camera_tags_backfill_legacy());
    camera_tag_t tags[4];
    int count = db_camera_tag_list_for_camera(stream.camera_uuid, tags, 4);
    TEST_ASSERT_EQUAL_INT(2, count);
    TEST_ASSERT_EQUAL_STRING("North", tags[0].label);
    TEST_ASSERT_EQUAL_STRING("south", tags[1].label);
}

void test_uuid_assignments_rebuild_legacy_tags(void) {
    stream_config_t stream = add_and_load_stream("assign", NULL);
    camera_tag_t outdoor = make_tag("outdoor");
    camera_tag_t critical = make_tag("critical");
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_TAG_OK, db_camera_tag_create(&outdoor));
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_TAG_OK, db_camera_tag_create(&critical));
    const char *uuids[] = {outdoor.uuid, critical.uuid};
    TEST_ASSERT_EQUAL_INT(
        DB_CAMERA_TAG_OK,
        db_camera_tag_set_for_camera(stream.camera_uuid, uuids, 2));

    TEST_ASSERT_EQUAL_INT(0,
                          get_stream_config_by_uuid(stream.camera_uuid, &stream));
    TEST_ASSERT_EQUAL_STRING("critical,outdoor", stream.tags);

    camera_tag_t assigned[4];
    TEST_ASSERT_EQUAL_INT(
        2, db_camera_tag_list_for_camera(stream.camera_uuid, assigned, 4));
    TEST_ASSERT_EQUAL_INT(1, assigned[0].camera_count);
    TEST_ASSERT_EQUAL_INT(1, assigned[1].camera_count);

    TEST_ASSERT_EQUAL_INT(
        DB_CAMERA_TAG_OK,
        db_camera_tag_set_for_camera(stream.camera_uuid, NULL, 0));
    TEST_ASSERT_EQUAL_INT(0,
                          get_stream_config_by_uuid(stream.camera_uuid, &stream));
    TEST_ASSERT_EQUAL_STRING("", stream.tags);
}

void test_rename_updates_all_legacy_assignments_atomically(void) {
    stream_config_t first = add_and_load_stream("rename_one", "old");
    stream_config_t second = add_and_load_stream("rename_two", "old,other");
    camera_tag_t tags[4];
    int count = db_camera_tag_list_for_camera(first.camera_uuid, tags, 4);
    TEST_ASSERT_EQUAL_INT(1, count);
    camera_tag_t old = tags[0];

    safe_strcpy(old.label, "renamed", sizeof(old.label), 0);
    safe_strcpy(old.description, "Updated centrally", sizeof(old.description), 0);
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_TAG_OK, db_camera_tag_update(&old));

    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_uuid(first.camera_uuid, &first));
    TEST_ASSERT_EQUAL_STRING("renamed", first.tags);
    TEST_ASSERT_EQUAL_INT(0,
                          get_stream_config_by_uuid(second.camera_uuid, &second));
    TEST_ASSERT_EQUAL_STRING("other,renamed", second.tags);

    camera_tag_t other;
    count = db_camera_tag_list_for_camera(second.camera_uuid, tags, 4);
    TEST_ASSERT_EQUAL_INT(2, count);
    other = tags[0];
    safe_strcpy(old.label, other.label, sizeof(old.label), 0);
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_TAG_CONFLICT, db_camera_tag_update(&old));
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_TAG_OK, db_camera_tag_get(old.uuid, &old));
    TEST_ASSERT_EQUAL_STRING("renamed", old.label);
}

void test_merge_deduplicates_assignments_and_removes_source(void) {
    stream_config_t stream = add_and_load_stream("merge", "entrance,entry");
    camera_tag_t tags[4];
    TEST_ASSERT_EQUAL_INT(
        2, db_camera_tag_list_for_camera(stream.camera_uuid, tags, 4));
    camera_tag_t entrance = tags[0];
    camera_tag_t entry = tags[1];

    TEST_ASSERT_EQUAL_INT(DB_CAMERA_TAG_OK,
                          db_camera_tag_merge(entry.uuid, entrance.uuid));
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_TAG_NOT_FOUND,
                          db_camera_tag_get(entry.uuid, &entry));
    TEST_ASSERT_EQUAL_INT(
        1, db_camera_tag_list_for_camera(stream.camera_uuid, tags, 4));
    TEST_ASSERT_EQUAL_STRING(entrance.uuid, tags[0].uuid);
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_uuid(stream.camera_uuid, &stream));
    TEST_ASSERT_EQUAL_STRING("entrance", stream.tags);
}

void test_delete_tag_cascades_assignments_and_legacy_value(void) {
    stream_config_t stream = add_and_load_stream("delete_tag", "temporary");
    camera_tag_t tags[2];
    TEST_ASSERT_EQUAL_INT(
        1, db_camera_tag_list_for_camera(stream.camera_uuid, tags, 2));
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_TAG_OK, db_camera_tag_delete(tags[0].uuid));
    TEST_ASSERT_EQUAL_INT(
        0, db_camera_tag_list_for_camera(stream.camera_uuid, tags, 2));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_uuid(stream.camera_uuid, &stream));
    TEST_ASSERT_EQUAL_STRING("", stream.tags);
}

void test_assignment_rolls_back_when_legacy_limit_would_be_exceeded(void) {
    stream_config_t stream = add_and_load_stream("limit", NULL);
    char first_label[201];
    char second_label[101];
    memset(first_label, 'a', sizeof(first_label) - 1);
    first_label[sizeof(first_label) - 1] = '\0';
    memset(second_label, 'b', sizeof(second_label) - 1);
    second_label[sizeof(second_label) - 1] = '\0';
    camera_tag_t first = make_tag(first_label);
    camera_tag_t second = make_tag(second_label);
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_TAG_OK, db_camera_tag_create(&first));
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_TAG_OK, db_camera_tag_create(&second));
    const char *uuids[] = {first.uuid, second.uuid};
    TEST_ASSERT_EQUAL_INT(
        DB_CAMERA_TAG_LIMIT,
        db_camera_tag_set_for_camera(stream.camera_uuid, uuids, 2));

    camera_tag_t assigned[2];
    TEST_ASSERT_EQUAL_INT(
        0, db_camera_tag_list_for_camera(stream.camera_uuid, assigned, 2));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_uuid(stream.camera_uuid, &stream));
    TEST_ASSERT_EQUAL_STRING("", stream.tags);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_create_tag_round_trips_metadata_and_rejects_case_duplicate);
    RUN_TEST(test_legacy_stream_tags_are_normalized_losslessly);
    RUN_TEST(test_startup_backfill_repairs_legacy_only_changes);
    RUN_TEST(test_uuid_assignments_rebuild_legacy_tags);
    RUN_TEST(test_rename_updates_all_legacy_assignments_atomically);
    RUN_TEST(test_merge_deduplicates_assignments_and_removes_source);
    RUN_TEST(test_delete_tag_cascades_assignments_and_legacy_value);
    RUN_TEST(test_assignment_rolls_back_when_legacy_limit_would_be_exceeded);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
