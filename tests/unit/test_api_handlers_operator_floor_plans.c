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

static char test_storage_path[MAX_PATH_LENGTH];

static cJSON *call_raw(
    void (*handler)(const http_request_t *, http_response_t *),
    http_method_t method, const char *path, const void *body, size_t body_len,
    const char *content_type, int expected_status) {
    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    request.method = method;
    safe_strcpy(request.path, path, sizeof(request.path), 0);
    safe_strcpy(request.client_ip, "127.0.0.1",
                sizeof(request.client_ip), 0);
    request.body = (void *)body;
    request.body_len = body_len;
    if (content_type) {
        safe_strcpy(request.headers[0].name, "Content-Type",
                    sizeof(request.headers[0].name), 0);
        safe_strcpy(request.headers[0].value, content_type,
                    sizeof(request.headers[0].value), 0);
        request.num_headers = 1;
    }
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

static cJSON *call(void (*handler)(const http_request_t *, http_response_t *),
                   http_method_t method, const char *path, const char *body,
                   int expected_status) {
    return call_raw(handler, method, path, body, body ? strlen(body) : 0,
                    body ? "application/json" : NULL, expected_status);
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

void test_sketch_round_trips_canonically_and_rejects_bad_shapes(void) {
    const char *body =
        "{\"name\":\"Sketched\",\"sketch\":{\"version\":1,\"shapes\":["
        "{\"id\":\"room-1\",\"type\":\"rect\",\"x\":0.10004,\"y\":0.2,"
        "\"w\":0.3,\"h\":0.25,\"label\":\"Lobby\",\"tone\":\"blue\","
        "\"ignored\":\"dropped\"},"
        "{\"id\":\"wall-1\",\"type\":\"wall\",\"points\":[[0,0.5],[1,0.5]]},"
        "{\"id\":\"area-1\",\"type\":\"area\",\"label\":\"Parking\","
        "\"points\":[[0.6,0.6],[0.9,0.6],[0.9,0.9]]},"
        "{\"id\":\"label-1\",\"type\":\"label\",\"x\":0.5,\"y\":0.05,"
        "\"text\":\"North entrance\",\"size\":\"lg\"}]}}";
    cJSON *created = call(handle_post_operator_floor_plan,
                          HTTP_METHOD_POST, "/api/live/plans", body, 201);
    cJSON *sketch = cJSON_GetObjectItemCaseSensitive(created, "sketch");
    TEST_ASSERT_TRUE(cJSON_IsObject(sketch));
    cJSON *shapes = cJSON_GetObjectItemCaseSensitive(sketch, "shapes");
    TEST_ASSERT_EQUAL_INT(4, cJSON_GetArraySize(shapes));
    cJSON *room = cJSON_GetArrayItem(shapes, 0);
    TEST_ASSERT_NULL(cJSON_GetObjectItemCaseSensitive(room, "ignored"));
    TEST_ASSERT_EQUAL_STRING("Lobby", cJSON_GetObjectItemCaseSensitive(
        room, "label")->valuestring);
    TEST_ASSERT_EQUAL_STRING("blue", cJSON_GetObjectItemCaseSensitive(
        room, "tone")->valuestring);
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        room, "filled")));
    TEST_ASSERT_EQUAL_INT(1000, (int)(cJSON_GetObjectItemCaseSensitive(
        room, "x")->valuedouble * 10000.0 + 0.5));
    TEST_ASSERT_EQUAL_STRING("slate", cJSON_GetObjectItemCaseSensitive(
        cJSON_GetArrayItem(shapes, 1), "tone")->valuestring);
    TEST_ASSERT_EQUAL_STRING("lg", cJSON_GetObjectItemCaseSensitive(
        cJSON_GetArrayItem(shapes, 3), "size")->valuestring);
    char plan_uuid[CAMERA_UUID_STRING_SIZE];
    safe_strcpy(plan_uuid, cJSON_GetObjectItemCaseSensitive(
        created, "uuid")->valuestring, sizeof(plan_uuid), 0);
    cJSON_Delete(created);

    // The list endpoint carries the sketch too.
    cJSON *listed = call(handle_get_operator_floor_plans,
                         HTTP_METHOD_GET, "/api/live/plans", NULL, 200);
    cJSON *first = cJSON_GetArrayItem(
        cJSON_GetObjectItemCaseSensitive(listed, "plans"), 0);
    TEST_ASSERT_EQUAL_INT(4, cJSON_GetArraySize(
        cJSON_GetObjectItemCaseSensitive(
            cJSON_GetObjectItemCaseSensitive(first, "sketch"), "shapes")));
    cJSON_Delete(listed);

    char path[128];
    snprintf(path, sizeof(path), "/api/live/plans/%s", plan_uuid);

    // Updates without a sketch field keep the drawing.
    cJSON *renamed = call(handle_put_operator_floor_plan, HTTP_METHOD_PUT,
                          path, "{\"name\":\"Renamed\",\"revision\":1,"
                          "\"cameras\":[]}", 200);
    TEST_ASSERT_EQUAL_INT(4, cJSON_GetArraySize(
        cJSON_GetObjectItemCaseSensitive(
            cJSON_GetObjectItemCaseSensitive(renamed, "sketch"), "shapes")));
    cJSON_Delete(renamed);

    // Invalid shapes are rejected as a whole.
    static const char *const invalid[] = {
        "{\"id\":\"bad\",\"type\":\"rect\",\"x\":0.9,\"y\":0.1,"
        "\"w\":0.3,\"h\":0.2}",
        "{\"id\":\"bad\",\"type\":\"wall\",\"points\":[[0,0]]}",
        "{\"id\":\"bad\",\"type\":\"area\",\"points\":[[0,0],[1,1]]}",
        "{\"id\":\"bad\",\"type\":\"label\",\"x\":0.5,\"y\":0.5,"
        "\"text\":\"\"}",
        "{\"id\":\"bad\",\"type\":\"label\",\"x\":0.5,\"y\":0.5,"
        "\"text\":\"line\\nbreak\"}",
        "{\"id\":\"bad id\",\"type\":\"wall\",\"points\":[[0,0],[1,1]]}",
        "{\"id\":\"bad\",\"type\":\"rect\",\"x\":0.1,\"y\":0.1,"
        "\"w\":0.2,\"h\":0.2,\"tone\":\"#ff0000\"}",
        "{\"id\":\"bad\",\"type\":\"circle\",\"x\":0.1,\"y\":0.1}",
        "{\"id\":\"dup\",\"type\":\"wall\",\"points\":[[0,0],[1,1]]},"
        "{\"id\":\"dup\",\"type\":\"wall\",\"points\":[[0,1],[1,0]]}",
    };
    for (size_t index = 0; index < sizeof(invalid) / sizeof(invalid[0]);
         index++) {
        char update[1024];
        snprintf(update, sizeof(update),
                 "{\"name\":\"Renamed\",\"revision\":2,\"cameras\":[],"
                 "\"sketch\":{\"version\":1,\"shapes\":[%s]}}",
                 invalid[index]);
        cJSON *rejected = call(handle_put_operator_floor_plan,
                               HTTP_METHOD_PUT, path, update, 400);
        cJSON_Delete(rejected);
    }

    // Explicit null clears the sketch.
    cJSON *cleared = call(handle_put_operator_floor_plan, HTTP_METHOD_PUT,
                          path, "{\"name\":\"Renamed\",\"revision\":2,"
                          "\"cameras\":[],\"sketch\":null}", 200);
    TEST_ASSERT_TRUE(cJSON_IsNull(
        cJSON_GetObjectItemCaseSensitive(cleared, "sketch")));
    cJSON_Delete(cleared);
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

void test_upload_replace_and_remove_background(void) {
    static const unsigned char png[] = {
        0x89, 'P', 'N', 'G', 0x0D, 0x0A, 0x1A, 0x0A};
    static const unsigned char jpeg[] = {0xFF, 0xD8, 0xFF, 0xE0};
    cJSON *created = call(handle_post_operator_floor_plan, HTTP_METHOD_POST,
                          "/api/live/plans", "{\"name\":\"Background\"}",
                          201);
    char uuid[CAMERA_UUID_STRING_SIZE];
    safe_strcpy(uuid, cJSON_GetObjectItemCaseSensitive(
        created, "uuid")->valuestring, sizeof(uuid), 0);
    cJSON_Delete(created);

    char endpoint[128];
    snprintf(endpoint, sizeof(endpoint), "/api/live/plans/%s/background",
             uuid);
    cJSON *json = call_raw(handle_put_operator_floor_plan_background,
                           HTTP_METHOD_PUT, endpoint, jpeg, sizeof(jpeg),
                           "image/png", 400);
    cJSON_Delete(json);
    json = call_raw(handle_put_operator_floor_plan_background,
                    HTTP_METHOD_PUT, endpoint, "<svg", 4,
                    "image/svg+xml", 415);
    cJSON_Delete(json);
    json = call_raw(handle_put_operator_floor_plan_background,
                    HTTP_METHOD_PUT, endpoint, png, sizeof(png),
                    "image/png garbage", 415);
    cJSON_Delete(json);

    json = call_raw(handle_put_operator_floor_plan_background,
                    HTTP_METHOD_PUT, endpoint, png, sizeof(png),
                    "image/png", 200);
    TEST_ASSERT_EQUAL_STRING("image/png",
        cJSON_GetObjectItemCaseSensitive(json, "background_mime")->valuestring);
    TEST_ASSERT_EQUAL_INT64(1, (int64_t)cJSON_GetObjectItemCaseSensitive(
        json, "revision")->valuedouble);
    cJSON_Delete(json);

    char png_path[MAX_PATH_LENGTH + 128];
    char jpeg_path[MAX_PATH_LENGTH + 128];
    snprintf(png_path, sizeof(png_path), "%s/floor_plans/%s.png",
             test_storage_path, uuid);
    snprintf(jpeg_path, sizeof(jpeg_path), "%s/floor_plans/%s.jpg",
             test_storage_path, uuid);
    TEST_ASSERT_EQUAL_INT(0, access(png_path, F_OK));

    json = call_raw(handle_put_operator_floor_plan_background,
                    HTTP_METHOD_PUT, endpoint, jpeg, sizeof(jpeg),
                    "image/jpeg; charset=binary", 200);
    TEST_ASSERT_EQUAL_STRING("image/jpeg",
        cJSON_GetObjectItemCaseSensitive(json, "background_mime")->valuestring);
    TEST_ASSERT_EQUAL_INT64(1, (int64_t)cJSON_GetObjectItemCaseSensitive(
        json, "revision")->valuedouble);
    cJSON_Delete(json);
    TEST_ASSERT_NOT_EQUAL(0, access(png_path, F_OK));
    TEST_ASSERT_EQUAL_INT(0, access(jpeg_path, F_OK));

    json = call(handle_delete_operator_floor_plan_background,
                HTTP_METHOD_DELETE, endpoint, NULL, 200);
    TEST_ASSERT_TRUE(cJSON_IsNull(
        cJSON_GetObjectItemCaseSensitive(json, "background_mime")));
    TEST_ASSERT_EQUAL_INT64(1, (int64_t)cJSON_GetObjectItemCaseSensitive(
        json, "revision")->valuedouble);
    cJSON_Delete(json);
    TEST_ASSERT_NOT_EQUAL(0, access(jpeg_path, F_OK));

    json = call_raw(handle_put_operator_floor_plan_background,
                    HTTP_METHOD_PUT,
                    "/api/live/plans/00000000-0000-4000-8000-000000000000/background",
                    png, sizeof(png), "image/png", 404);
    cJSON_Delete(json);

    json = call_raw(handle_put_operator_floor_plan_background,
                    HTTP_METHOD_PUT, endpoint, png, sizeof(png),
                    "image/png", 200);
    cJSON_Delete(json);
    TEST_ASSERT_EQUAL_INT(0, access(png_path, F_OK));

    char plan_endpoint[128];
    snprintf(plan_endpoint, sizeof(plan_endpoint), "/api/live/plans/%s", uuid);
    json = call(handle_delete_operator_floor_plan, HTTP_METHOD_DELETE,
                plan_endpoint, "{\"revision\":1}", 200);
    cJSON_Delete(json);
    TEST_ASSERT_NOT_EQUAL(0, access(png_path, F_OK));
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    snprintf(test_storage_path, sizeof(test_storage_path),
             "/tmp/lightnvr_unit_api_operator_floor_plans_storage_%ld",
             (long)getpid());
    safe_strcpy(g_config.storage_path, test_storage_path,
                sizeof(g_config.storage_path), 0);
    UNITY_BEGIN();
    RUN_TEST(test_create_list_update_and_delete_authorized_plan);
    RUN_TEST(test_rejects_unknown_camera_placement);
    RUN_TEST(test_sketch_round_trips_canonically_and_rejects_bad_shapes);
    RUN_TEST(test_upload_replace_and_remove_background);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    char background_directory[MAX_PATH_LENGTH + 32];
    snprintf(background_directory, sizeof(background_directory),
             "%s/floor_plans", test_storage_path);
    rmdir(background_directory);
    rmdir(test_storage_path);
    return result;
}
