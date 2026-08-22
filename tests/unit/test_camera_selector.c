/**
 * @file test_camera_selector.c
 * @brief Typed, bounded fleet selector parser and evaluator tests.
 */

#include <stdio.h>
#include <string.h>
#include <cjson/cJSON.h>

#include "unity.h"
#include "core/camera_selector.h"
#include "utils/strings.h"

static const char *CAMERA_UUID = "11111111-1111-4111-8111-111111111111";
static const char *SITE_UUID = "22222222-2222-4222-8222-222222222222";
static const char *AREA_UUID = "33333333-3333-4333-8333-333333333333";
static const char *OUTDOOR_UUID = "44444444-4444-4444-8444-444444444444";
static const char *CRITICAL_UUID = "55555555-5555-4555-8555-555555555555";

static fleet_camera_t make_camera(void) {
    fleet_camera_t camera;
    memset(&camera, 0, sizeof(camera));
    safe_strcpy(camera.camera_uuid, CAMERA_UUID, sizeof(camera.camera_uuid), 0);
    safe_strcpy(camera.name, "North Entrance", sizeof(camera.name), 0);
    safe_strcpy(camera.location_uuid, AREA_UUID,
                sizeof(camera.location_uuid), 0);
    safe_strcpy(camera.location_ancestor_uuids[0], SITE_UUID,
                CAMERA_UUID_STRING_SIZE, 0);
    safe_strcpy(camera.location_ancestor_uuids[1], AREA_UUID,
                CAMERA_UUID_STRING_SIZE, 0);
    camera.location_depth = 2;
    safe_strcpy(camera.tags[0].uuid, OUTDOOR_UUID,
                sizeof(camera.tags[0].uuid), 0);
    safe_strcpy(camera.tags[0].label, "Outdoor",
                sizeof(camera.tags[0].label), 0);
    safe_strcpy(camera.tags[1].uuid, CRITICAL_UUID,
                sizeof(camera.tags[1].uuid), 0);
    safe_strcpy(camera.tags[1].label, "Critical",
                sizeof(camera.tags[1].label), 0);
    camera.tag_count = 2;
    safe_strcpy(camera.manufacturer, "Axis", sizeof(camera.manufacturer), 0);
    safe_strcpy(camera.model, "P3265-LV", sizeof(camera.model), 0);
    camera.enabled = true;
    camera.record = true;
    camera.detection_based_recording = true;
    camera.is_onvif = true;
    camera.ptz_enabled = false;
    camera.backchannel_enabled = true;
    camera.health = FLEET_HEALTH_DOWN;
    return camera;
}

static fleet_selector_t *parse(const char *json, char *error) {
    cJSON *root = cJSON_Parse(json);
    TEST_ASSERT_NOT_NULL(root);
    fleet_selector_t *selector =
        fleet_selector_parse(root, error, FLEET_SELECTOR_ERROR_MAX);
    cJSON_Delete(root);
    return selector;
}

void setUp(void) {}
void tearDown(void) {}

void test_composes_location_tags_health_and_explains_match(void) {
    char json[2048];
    snprintf(json, sizeof(json),
             "{\"version\":1,\"expression\":{\"op\":\"and\",\"children\":["
             "{\"op\":\"location_subtree\",\"uuid\":\"%s\"},"
             "{\"op\":\"tag_any\",\"uuids\":[\"%s\"]},"
             "{\"op\":\"health\",\"values\":[\"down\"]}]}}",
             SITE_UUID, OUTDOOR_UUID);
    char error[FLEET_SELECTOR_ERROR_MAX] = {0};
    fleet_selector_t *selector = parse(json, error);
    TEST_ASSERT_NOT_NULL_MESSAGE(selector, error);
    fleet_camera_t camera = make_camera();
    fleet_selector_explanation_t explanation;
    TEST_ASSERT_TRUE(fleet_selector_matches(selector, &camera, &explanation));
    TEST_ASSERT_EQUAL_INT(3, explanation.clause_count);
    TEST_ASSERT_NOT_NULL(strstr(explanation.clauses[0], "location_subtree"));
    TEST_ASSERT_NOT_NULL(strstr(explanation.clauses[1], "tag_any"));
    TEST_ASSERT_EQUAL_STRING("health=down", explanation.clauses[2]);
    fleet_selector_free(selector);
}

void test_tag_all_none_or_and_not_semantics(void) {
    char json[3072];
    snprintf(json, sizeof(json),
             "{\"version\":1,\"expression\":{\"op\":\"and\",\"children\":["
             "{\"op\":\"tag_all\",\"uuids\":[\"%s\",\"%s\"]},"
             "{\"op\":\"tag_none\",\"uuids\":[\"66666666-6666-4666-8666-666666666666\"]},"
             "{\"op\":\"not\",\"child\":{\"op\":\"enabled\",\"value\":false}},"
             "{\"op\":\"or\",\"children\":["
             "{\"op\":\"health\",\"values\":[\"up\"]},"
             "{\"op\":\"recording_mode\",\"values\":[\"detection\"]}]}]}}",
             OUTDOOR_UUID, CRITICAL_UUID);
    char error[FLEET_SELECTOR_ERROR_MAX] = {0};
    fleet_selector_t *selector = parse(json, error);
    TEST_ASSERT_NOT_NULL_MESSAGE(selector, error);
    fleet_camera_t camera = make_camera();
    TEST_ASSERT_TRUE(fleet_selector_matches(selector, &camera, NULL));
    camera.tags[1].uuid[0] = '\0';
    camera.tag_count = 1;
    TEST_ASSERT_FALSE(fleet_selector_matches(selector, &camera, NULL));
    fleet_selector_free(selector);
}

void test_inventory_and_capability_predicates(void) {
    char json[3072];
    snprintf(json, sizeof(json),
             "{\"version\":1,\"expression\":{\"op\":\"and\",\"children\":["
             "{\"op\":\"camera_uuid\",\"values\":[\"%s\"]},"
             "{\"op\":\"vendor\",\"values\":[\"axis\"]},"
             "{\"op\":\"model\",\"values\":[\"P3265-LV\"]},"
             "{\"op\":\"capability_all\",\"values\":[\"onvif\",\"backchannel\"]},"
             "{\"op\":\"capability_any\",\"values\":[\"ptz\",\"onvif\"]}]}}",
             CAMERA_UUID);
    char error[FLEET_SELECTOR_ERROR_MAX] = {0};
    fleet_selector_t *selector = parse(json, error);
    TEST_ASSERT_NOT_NULL_MESSAGE(selector, error);
    fleet_camera_t camera = make_camera();
    TEST_ASSERT_TRUE(fleet_selector_matches(selector, &camera, NULL));
    camera.is_onvif = false;
    TEST_ASSERT_FALSE(fleet_selector_matches(selector, &camera, NULL));
    fleet_selector_free(selector);
}

void test_rejects_unknown_version_operation_and_invalid_values(void) {
    const char *cases[] = {
        "{\"version\":2,\"expression\":{\"op\":\"all\"}}",
        "{\"version\":1,\"expression\":{\"op\":\"sql\",\"value\":\"1=1\"}}",
        "{\"version\":1,\"expression\":{\"op\":\"tag_any\",\"uuids\":[\"bad\"]}}",
        "{\"version\":1,\"expression\":{\"op\":\"health\",\"values\":[\"broken\"]}}",
        "{\"version\":1,\"expression\":{\"op\":\"enabled\",\"value\":\"true\"}}"
    };
    for (size_t i = 0; i < sizeof(cases) / sizeof(cases[0]); i++) {
        char error[FLEET_SELECTOR_ERROR_MAX] = {0};
        fleet_selector_t *selector = parse(cases[i], error);
        TEST_ASSERT_NULL(selector);
        TEST_ASSERT_TRUE(strlen(error) > 0);
    }
}

void test_rejects_excessive_depth(void) {
    cJSON *root = cJSON_CreateObject();
    cJSON_AddNumberToObject(root, "version", 1);
    cJSON *expression = cJSON_CreateObject();
    cJSON_AddItemToObject(root, "expression", expression);
    for (int i = 0; i < FLEET_SELECTOR_MAX_DEPTH; i++) {
        cJSON_AddStringToObject(expression, "op", "not");
        cJSON *child = cJSON_CreateObject();
        cJSON_AddItemToObject(expression, "child", child);
        expression = child;
    }
    cJSON_AddStringToObject(expression, "op", "all");
    char error[FLEET_SELECTOR_ERROR_MAX] = {0};
    fleet_selector_t *selector =
        fleet_selector_parse(root, error, sizeof(error));
    cJSON_Delete(root);
    TEST_ASSERT_NULL(selector);
    TEST_ASSERT_NOT_NULL(strstr(error, "depth"));
}

void test_rejects_excessive_node_count(void) {
    cJSON *root = cJSON_CreateObject();
    cJSON_AddNumberToObject(root, "version", 1);
    cJSON *expression = cJSON_CreateObject();
    cJSON_AddStringToObject(expression, "op", "and");
    cJSON *children = cJSON_CreateArray();
    cJSON_AddItemToObject(expression, "children", children);
    cJSON_AddItemToObject(root, "expression", expression);
    for (int i = 0; i < FLEET_SELECTOR_MAX_NODES; i++) {
        cJSON *child = cJSON_CreateObject();
        cJSON_AddStringToObject(child, "op", "all");
        cJSON_AddItemToArray(children, child);
    }
    char error[FLEET_SELECTOR_ERROR_MAX] = {0};
    fleet_selector_t *selector =
        fleet_selector_parse(root, error, sizeof(error));
    cJSON_Delete(root);
    TEST_ASSERT_NULL(selector);
    TEST_ASSERT_NOT_NULL(strstr(error, "node count"));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_composes_location_tags_health_and_explains_match);
    RUN_TEST(test_tag_all_none_or_and_not_semantics);
    RUN_TEST(test_inventory_and_capability_predicates);
    RUN_TEST(test_rejects_unknown_version_operation_and_invalid_values);
    RUN_TEST(test_rejects_excessive_depth);
    RUN_TEST(test_rejects_excessive_node_count);
    return UNITY_END();
}
