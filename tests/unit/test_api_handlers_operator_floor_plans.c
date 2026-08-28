/**
 * @file test_api_handlers_operator_floor_plans.c
 * @brief Operator building-plan API and authorization tests.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include <cjson/cJSON.h>
#include <sqlite3.h>

#include "unity.h"
#include "core/config.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#include "utils/strings.h"
#include "web/api_handlers_operator_floor_plans.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_api_operator_floor_plans.db"

static cJSON *call(void (*handler)(const http_request_t *, http_response_t *),
                   http_method_t method, const char *path, const char *body,
                   int expected_status) {
    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    request.method = method;
    safe_strcpy(request.path, path, sizeof(request.path), 0);
    safe_strcpy(request.client_ip, "127.0.0.1",
                sizeof(request.client_ip), 0);
    request.body = (void *)(body ? body : "");
    request.body_len = body ? strlen(body) : 0;
    handler(&request, &response);
    if (response.status_code != expected_status) {
        fprintf(stderr, "floor plan API expected %d, got %d: %s\n",
                expected_status, response.status_code,
                response.body ? (const char *)response.body : "(empty)");
    }
    TEST_ASSERT_EQUAL_INT(expected_status, response.status_code);
    cJSON *json = response.body
        ? cJSON_Parse((const char *)response.body) : NULL;
    TEST_ASSERT_NOT_NULL(json);
    http_response_free(&response);
    return json;
}

static stream_config_t create_camera(void) {
    stream_config_t stream;
    memset(&stream, 0, sizeof(stream));
    safe_strcpy(stream.name, "Rear door", sizeof(stream.name), 0);
    safe_strcpy(stream.url, "rtsp://camera/rear", sizeof(stream.url), 0);
    safe_strcpy(stream.codec, "h264", sizeof(stream.codec), 0);
    stream.enabled = true;
    stream.streaming_enabled = true;
    stream.width = 1920;
    stream.height = 1080;
    stream.fps = 25;
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&stream));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name(stream.name, &stream));
    return stream;
}

void setUp(void) {
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
    sqlite3_exec(get_db_handle(), "DELETE FROM operator_floor_plans;",
                 NULL, NULL, NULL);
    sqlite3_exec(get_db_handle(), "DELETE FROM streams;", NULL, NULL, NULL);
}

void tearDown(void) {}

void test_create_list_update_and_delete_authorized_plan(void) {
    stream_config_t camera = create_camera();
    char body[1024];
    snprintf(body, sizeof(body),
             "{\"name\":\"House\",\"canvas_width\":1000,"
             "\"canvas_height\":700,\"cameras\":[{"
             "\"camera_uuid\":\"%s\",\"x\":0.4,\"y\":0.8,"
             "\"rotation\":180,\"fov\":70}]}", camera.camera_uuid);
    cJSON *created = call(handle_post_operator_floor_plan,
                          HTTP_METHOD_POST, "/api/live/plans", body, 201);
    const char *uuid = cJSON_GetObjectItemCaseSensitive(
        created, "uuid")->valuestring;
    char plan_uuid[CAMERA_UUID_STRING_SIZE];
    safe_strcpy(plan_uuid, uuid, sizeof(plan_uuid), 0);
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(
        cJSON_GetObjectItemCaseSensitive(created, "cameras")));
    cJSON_Delete(created);

    cJSON *listed = call(handle_get_operator_floor_plans,
                         HTTP_METHOD_GET, "/api/live/plans", NULL, 200);
    TEST_ASSERT_TRUE(cJSON_IsTrue(
        cJSON_GetObjectItemCaseSensitive(listed, "can_modify")));
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(
        cJSON_GetObjectItemCaseSensitive(listed, "plans")));
    cJSON_Delete(listed);

    snprintf(body, sizeof(body),
             "{\"name\":\"House\",\"canvas_width\":1000,"
             "\"canvas_height\":700,\"revision\":1,\"cameras\":[{"
             "\"camera_uuid\":\"%s\",\"x\":0.2,\"y\":0.8,"
             "\"rotation\":180,\"fov\":70}]}", camera.camera_uuid);
    char path[128];
    snprintf(path, sizeof(path), "/api/live/plans/%s", plan_uuid);
    cJSON *updated = call(handle_put_operator_floor_plan,
                          HTTP_METHOD_PUT, path, body, 200);
    TEST_ASSERT_EQUAL_INT64(2, (int64_t)cJSON_GetObjectItemCaseSensitive(
        updated, "revision")->valuedouble);
    cJSON_Delete(updated);

    cJSON *deleted = call(handle_delete_operator_floor_plan,
                          HTTP_METHOD_DELETE, path, "{\"revision\":2}", 200);
    TEST_ASSERT_TRUE(cJSON_IsTrue(
        cJSON_GetObjectItemCaseSensitive(deleted, "deleted")));
    cJSON_Delete(deleted);
}

void test_rejects_unknown_camera_placement(void) {
    const char *body =
        "{\"name\":\"Unknown\",\"cameras\":[{"
        "\"camera_uuid\":\"11111111-1111-4111-8111-111111111111\","
        "\"x\":0.5,\"y\":0.5}]}";
    cJSON *json = call(handle_post_operator_floor_plan, HTTP_METHOD_POST,
                       "/api/live/plans", body, 403);
    cJSON_Delete(json);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_create_list_update_and_delete_authorized_plan);
    RUN_TEST(test_rejects_unknown_camera_placement);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
