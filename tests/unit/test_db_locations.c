/**
 * @file test_db_locations.c
 * @brief Layer 2 integration tests for camera location hierarchy operations.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <sqlite3.h>

#include "unity.h"
#include "database/db_core.h"
#include "database/db_locations.h"
#include "database/db_streams.h"
#include "utils/strings.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_locations_test.db"

static camera_location_t make_location(const char *name, const char *type,
                                       const char *parent_uuid) {
    camera_location_t location;
    memset(&location, 0, sizeof(location));
    safe_strcpy(location.name, name, sizeof(location.name), 0);
    safe_strcpy(location.type, type, sizeof(location.type), 0);
    safe_strcpy(location.metadata_json, "{}", sizeof(location.metadata_json), 0);
    if (parent_uuid) {
        safe_strcpy(location.parent_uuid, parent_uuid,
                    sizeof(location.parent_uuid), 0);
    }
    return location;
}

static stream_config_t make_stream(const char *name) {
    stream_config_t stream;
    memset(&stream, 0, sizeof(stream));
    safe_strcpy(stream.name, name, sizeof(stream.name), 0);
    safe_strcpy(stream.url, "rtsp://camera/stream", sizeof(stream.url), 0);
    safe_strcpy(stream.codec, "h264", sizeof(stream.codec), 0);
    stream.enabled = true;
    stream.streaming_enabled = true;
    stream.width = 1920;
    stream.height = 1080;
    stream.fps = 25;
    return stream;
}

void setUp(void) {
    sqlite3 *db = get_db_handle();
    sqlite3_exec(db, "DELETE FROM streams;", NULL, NULL, NULL);
    do {
        sqlite3_exec(db,
                     "DELETE FROM camera_locations WHERE is_system = 0 "
                     "AND NOT EXISTS (SELECT 1 FROM camera_locations child "
                     "WHERE child.parent_uuid = camera_locations.uuid);",
                     NULL, NULL, NULL);
    } while (sqlite3_changes(db) > 0);
}

void tearDown(void) {}

void test_unassigned_root_is_seeded_and_immutable(void) {
    camera_location_t root;
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_get_unassigned(&root));
    TEST_ASSERT_EQUAL_STRING("Unassigned", root.name);
    TEST_ASSERT_EQUAL_STRING("system", root.type);
    TEST_ASSERT_TRUE(root.is_system);
    TEST_ASSERT_EQUAL_STRING("", root.parent_uuid);

    safe_strcpy(root.name, "Renamed", sizeof(root.name), 0);
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_INVALID, db_location_update(&root));
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_INVALID, db_location_delete(root.uuid));
}

void test_new_camera_defaults_to_unassigned(void) {
    camera_location_t root;
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_get_unassigned(&root));

    stream_config_t stream = make_stream("default_location");
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&stream));

    stream_config_t persisted;
    TEST_ASSERT_EQUAL_INT(0,
                          get_stream_config_by_name("default_location",
                                                    &persisted));
    TEST_ASSERT_EQUAL_STRING(root.uuid, persisted.location_uuid);

    camera_location_t camera_location;
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK,
                          db_location_get_for_camera(persisted.camera_uuid,
                                                     &camera_location));
    TEST_ASSERT_EQUAL_STRING(root.uuid, camera_location.uuid);
}

void test_create_nested_locations_and_report_direct_counts(void) {
    camera_location_t site = make_location("SJC", "site", NULL);
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_create(&site));

    camera_location_t building =
        make_location("Building C", "building", site.uuid);
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_create(&building));

    camera_location_t floor = make_location("Floor 2", "floor", building.uuid);
    floor.sort_order = 20;
    safe_strcpy(floor.metadata_json, "{\"map\":\"floor-2.svg\"}",
                sizeof(floor.metadata_json), 0);
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_create(&floor));

    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_get(site.uuid, &site));
    TEST_ASSERT_EQUAL_INT(1, site.direct_child_count);
    TEST_ASSERT_EQUAL_INT(0, site.direct_camera_count);
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK,
                          db_location_get(building.uuid, &building));
    TEST_ASSERT_EQUAL_INT(1, building.direct_child_count);
    TEST_ASSERT_EQUAL_STRING(site.uuid, building.parent_uuid);

    camera_location_t locations[8];
    int count = db_location_list(locations, 8);
    TEST_ASSERT_EQUAL_INT(4, count);
}

void test_sibling_names_are_case_insensitively_unique(void) {
    camera_location_t site = make_location("Campus", "site", NULL);
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_create(&site));

    camera_location_t first = make_location("North", "area", site.uuid);
    camera_location_t duplicate = make_location("north", "area", site.uuid);
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_create(&first));
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_CONFLICT,
                          db_location_create(&duplicate));

    camera_location_t other_root = make_location("north", "site", NULL);
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_create(&other_root));
}

void test_move_rejects_cycles_and_allows_subtree_move(void) {
    camera_location_t site = make_location("Site", "site", NULL);
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_create(&site));
    camera_location_t building =
        make_location("Building", "building", site.uuid);
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_create(&building));
    camera_location_t area = make_location("Area", "area", building.uuid);
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_create(&area));

    safe_strcpy(site.parent_uuid, area.uuid, sizeof(site.parent_uuid), 0);
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_CONFLICT, db_location_update(&site));

    camera_location_t second_site = make_location("Second Site", "site", NULL);
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_create(&second_site));
    safe_strcpy(building.parent_uuid, second_site.uuid,
                sizeof(building.parent_uuid), 0);
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_update(&building));
    TEST_ASSERT_EQUAL_STRING(second_site.uuid, building.parent_uuid);

    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_get(area.uuid, &area));
    TEST_ASSERT_EQUAL_STRING(building.uuid, area.parent_uuid);
}

void test_delete_requires_empty_location_and_camera_can_be_reassigned(void) {
    camera_location_t root;
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_get_unassigned(&root));
    camera_location_t site = make_location("Site", "site", NULL);
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_create(&site));
    camera_location_t area = make_location("Area", "area", site.uuid);
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_create(&area));

    TEST_ASSERT_EQUAL_INT(DB_LOCATION_CONFLICT, db_location_delete(site.uuid));

    stream_config_t stream = make_stream("assigned_camera");
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&stream));
    TEST_ASSERT_EQUAL_INT(0,
                          get_stream_config_by_name("assigned_camera", &stream));
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK,
                          db_location_assign_camera(stream.camera_uuid,
                                                    area.uuid));
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_CONFLICT, db_location_delete(area.uuid));

    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK,
                          db_location_assign_camera(stream.camera_uuid,
                                                    root.uuid));
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_delete(area.uuid));
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_delete(site.uuid));
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_NOT_FOUND,
                          db_location_get(area.uuid, &area));
}

void test_assignment_rejects_unknown_camera_or_location(void) {
    camera_location_t root;
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_get_unassigned(&root));
    TEST_ASSERT_EQUAL_INT(
        DB_LOCATION_NOT_FOUND,
        db_location_assign_camera("11111111-1111-4111-8111-111111111111",
                                  root.uuid));

    stream_config_t stream = make_stream("known_camera");
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&stream));
    TEST_ASSERT_EQUAL_INT(0,
                          get_stream_config_by_name("known_camera", &stream));
    TEST_ASSERT_EQUAL_INT(
        DB_LOCATION_NOT_FOUND,
        db_location_assign_camera(stream.camera_uuid,
                                  "22222222-2222-4222-8222-222222222222"));
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_unassigned_root_is_seeded_and_immutable);
    RUN_TEST(test_new_camera_defaults_to_unassigned);
    RUN_TEST(test_create_nested_locations_and_report_direct_counts);
    RUN_TEST(test_sibling_names_are_case_insensitively_unique);
    RUN_TEST(test_move_rejects_cycles_and_allows_subtree_move);
    RUN_TEST(test_delete_requires_empty_location_and_camera_can_be_reassigned);
    RUN_TEST(test_assignment_rejects_unknown_camera_or_location);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
