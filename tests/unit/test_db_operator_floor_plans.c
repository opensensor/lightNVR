/**
 * @file test_db_operator_floor_plans.c
 * @brief Shared operator building-plan persistence tests.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include <sqlite3.h>

#include "unity.h"
#include "database/db_core.h"
#include "database/db_operator_floor_plans.h"
#include "database/db_streams.h"
#include "utils/strings.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_operator_floor_plans.db"

static stream_config_t create_camera(const char *name) {
    stream_config_t camera;
    memset(&camera, 0, sizeof(camera));
    safe_strcpy(camera.name, name, sizeof(camera.name), 0);
    safe_strcpy(camera.url, "rtsp://camera/plan", sizeof(camera.url), 0);
    safe_strcpy(camera.codec, "h264", sizeof(camera.codec), 0);
    camera.enabled = true;
    camera.streaming_enabled = true;
    camera.width = 1920;
    camera.height = 1080;
    camera.fps = 25;
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&camera));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name(name, &camera));
    return camera;
}

static operator_floor_plan_t make_plan(const char *name) {
    operator_floor_plan_t plan;
    memset(&plan, 0, sizeof(plan));
    safe_strcpy(plan.name, name, sizeof(plan.name), 0);
    plan.canvas_width = 1200;
    plan.canvas_height = 800;
    return plan;
}

void setUp(void) {
    sqlite3_exec(get_db_handle(), "DELETE FROM operator_floor_plans;",
                 NULL, NULL, NULL);
    sqlite3_exec(get_db_handle(), "DELETE FROM streams;", NULL, NULL, NULL);
}

void tearDown(void) {}

void test_create_update_and_delete_plan_camera_placements(void) {
    stream_config_t camera = create_camera("Front entrance");
    operator_floor_plan_t plan = make_plan("Main building");
    operator_floor_plan_camera_t marker = {0};
    safe_strcpy(marker.camera_uuid, camera.camera_uuid,
                sizeof(marker.camera_uuid), 0);
    marker.x = 0.5;
    marker.y = 0.9;
    marker.rotation = 180.0;
    marker.fov = 70.0;

    TEST_ASSERT_EQUAL_INT(
        DB_OPERATOR_FLOOR_PLAN_OK,
        db_operator_floor_plan_create(&plan, &marker, 1));
    TEST_ASSERT_EQUAL_INT64(1, plan.revision);

    operator_floor_plan_t plans[4];
    TEST_ASSERT_EQUAL_INT(1, db_operator_floor_plan_list(plans, 4));
    operator_floor_plan_camera_t loaded[4];
    TEST_ASSERT_EQUAL_INT(1, db_operator_floor_plan_camera_list(
        plan.uuid, loaded, 4));
    TEST_ASSERT_EQUAL_STRING(camera.camera_uuid, loaded[0].camera_uuid);
    TEST_ASSERT_EQUAL_INT(900, (int)(loaded[0].y * 1000.0 + 0.5));

    marker.x = 0.2;
    TEST_ASSERT_EQUAL_INT(
        DB_OPERATOR_FLOOR_PLAN_OK,
        db_operator_floor_plan_update(&plan, &marker, 1, plan.revision));
    TEST_ASSERT_EQUAL_INT64(2, plan.revision);
    TEST_ASSERT_EQUAL_INT(
        DB_OPERATOR_FLOOR_PLAN_STALE,
        db_operator_floor_plan_delete(plan.uuid, 1));
    TEST_ASSERT_EQUAL_INT(
        DB_OPERATOR_FLOOR_PLAN_OK,
        db_operator_floor_plan_delete(plan.uuid, plan.revision));
}

void test_rejects_duplicate_or_out_of_bounds_placements(void) {
    stream_config_t camera = create_camera("Hallway");
    operator_floor_plan_t plan = make_plan("Invalid plan");
    operator_floor_plan_camera_t markers[2] = {0};
    for (int index = 0; index < 2; index++) {
        safe_strcpy(markers[index].camera_uuid, camera.camera_uuid,
                    sizeof(markers[index].camera_uuid), 0);
        markers[index].x = 0.5;
        markers[index].y = 0.5;
        markers[index].fov = 65.0;
    }
    TEST_ASSERT_EQUAL_INT(
        DB_OPERATOR_FLOOR_PLAN_INVALID,
        db_operator_floor_plan_create(&plan, markers, 2));
    plan = make_plan("Outside canvas");
    markers[0].x = 1.1;
    TEST_ASSERT_EQUAL_INT(
        DB_OPERATOR_FLOOR_PLAN_INVALID,
        db_operator_floor_plan_create(&plan, markers, 1));
}

void test_set_and_clear_background_without_revision_bump(void) {
    operator_floor_plan_t plan = make_plan("Warehouse");
    TEST_ASSERT_EQUAL_INT(DB_OPERATOR_FLOOR_PLAN_OK,
        db_operator_floor_plan_create(&plan, NULL, 0));
    TEST_ASSERT_EQUAL_STRING("", plan.background_mime);

    TEST_ASSERT_EQUAL_INT(DB_OPERATOR_FLOOR_PLAN_OK,
        db_operator_floor_plan_set_background(plan.uuid, "image/png"));
    operator_floor_plan_t current;
    TEST_ASSERT_EQUAL_INT(DB_OPERATOR_FLOOR_PLAN_OK,
        db_operator_floor_plan_get(plan.uuid, &current));
    TEST_ASSERT_EQUAL_STRING("image/png", current.background_mime);
    TEST_ASSERT_EQUAL_INT64(1, current.revision);

    TEST_ASSERT_EQUAL_INT(DB_OPERATOR_FLOOR_PLAN_INVALID,
        db_operator_floor_plan_set_background(plan.uuid, "image/svg+xml"));
    TEST_ASSERT_EQUAL_INT(DB_OPERATOR_FLOOR_PLAN_NOT_FOUND,
        db_operator_floor_plan_set_background(
            "00000000-0000-4000-8000-000000000000", "image/jpeg"));

    TEST_ASSERT_EQUAL_INT(DB_OPERATOR_FLOOR_PLAN_OK,
        db_operator_floor_plan_set_background(plan.uuid, NULL));
    TEST_ASSERT_EQUAL_INT(DB_OPERATOR_FLOOR_PLAN_OK,
        db_operator_floor_plan_get(plan.uuid, &current));
    TEST_ASSERT_EQUAL_STRING("", current.background_mime);
    TEST_ASSERT_EQUAL_INT64(1, current.revision);

    TEST_ASSERT_EQUAL_INT(DB_OPERATOR_FLOOR_PLAN_OK,
        db_operator_floor_plan_set_background(plan.uuid, "image/jpeg"));
    TEST_ASSERT_EQUAL_INT(DB_OPERATOR_FLOOR_PLAN_OK,
        db_operator_floor_plan_set_background(plan.uuid, ""));
    TEST_ASSERT_EQUAL_INT(DB_OPERATOR_FLOOR_PLAN_OK,
        db_operator_floor_plan_get(plan.uuid, &current));
    TEST_ASSERT_EQUAL_STRING("", current.background_mime);
    TEST_ASSERT_EQUAL_INT64(1, current.revision);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_create_update_and_delete_plan_camera_placements);
    RUN_TEST(test_rejects_duplicate_or_out_of_bounds_placements);
    RUN_TEST(test_set_and_clear_background_without_revision_bump);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
