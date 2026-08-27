/**
 * @file test_api_handlers_live_layouts.c
 * @brief Operator Live layout API and camera authorization tests.
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
#include "utils/uuid.h"
#include "web/api_handlers_live_layouts.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_api_live_layouts.db"

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
        fprintf(stderr, "live layout API expected %d, got %d: %s\n",
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
    safe_strcpy(stream.name, "Lobby", sizeof(stream.name), 0);
    safe_strcpy(stream.url, "rtsp://camera/lobby", sizeof(stream.url), 0);
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
    sqlite3_exec(get_db_handle(), "DELETE FROM live_saved_layouts;",
                 NULL, NULL, NULL);
    sqlite3_exec(get_db_handle(), "DELETE FROM streams;", NULL, NULL, NULL);
}

void tearDown(void) {}

void test_create_and_list_layout_with_authorized_camera(void) {
    stream_config_t camera = create_camera();
    char body[1024];
    snprintf(body, sizeof(body),
             "{\"name\":\"Main desk\",\"is_shared\":false,"
             "\"location_uuid\":null,\"availability\":\"live\","
             "\"columns\":2,\"rows\":1,\"camera_slots\":["
             "{\"camera_uuid\":\"%s\",\"eptz_mode\":\"dual\","
             "\"eptz_preset_uuid\":null,\"eptz_view\":{"
             "\"yaw\":15,\"tilt\":-45,\"fov\":70,"
             "\"secondary_yaw\":-165,\"secondary_tilt\":-40,"
             "\"secondary_view_fov\":65}}]}", camera.camera_uuid);
    cJSON *created = call(handle_post_live_layout, HTTP_METHOD_POST,
                          "/api/live/layouts", body, 201);
    cJSON *created_slot = cJSON_GetArrayItem(
        cJSON_GetObjectItemCaseSensitive(created, "camera_slots"), 0);
    TEST_ASSERT_EQUAL_STRING(camera.camera_uuid,
        cJSON_GetObjectItemCaseSensitive(
            created_slot, "camera_uuid")->valuestring);
    TEST_ASSERT_EQUAL_STRING("dual", cJSON_GetObjectItemCaseSensitive(
        created_slot, "eptz_mode")->valuestring);
    cJSON_Delete(created);

    cJSON *listed = call(handle_get_live_layouts, HTTP_METHOD_GET,
                         "/api/live/layouts", NULL, 200);
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(
        cJSON_GetObjectItemCaseSensitive(listed, "layouts")));
    TEST_ASSERT_TRUE(cJSON_IsTrue(
        cJSON_GetObjectItemCaseSensitive(listed, "can_modify")));
    cJSON_Delete(listed);
}

void test_rejects_layout_with_unknown_camera(void) {
    char unknown_uuid[CAMERA_UUID_STRING_SIZE];
    TEST_ASSERT_EQUAL_INT(0, lightnvr_uuid_generate_v4(unknown_uuid));
    char body[1024];
    snprintf(body, sizeof(body),
             "{\"name\":\"Invalid\",\"availability\":\"live\","
             "\"columns\":1,\"rows\":1,\"camera_slots\":["
             "{\"camera_uuid\":\"%s\"}]}", unknown_uuid);
    cJSON *json = call(handle_post_live_layout, HTTP_METHOD_POST,
                       "/api/live/layouts", body, 403);
    cJSON_Delete(json);
}

void test_rejects_invalid_eptz_layout_state(void) {
    stream_config_t camera = create_camera();
    char body[1024];
    snprintf(body, sizeof(body),
             "{\"name\":\"Invalid ePTZ\",\"availability\":\"live\","
             "\"columns\":1,\"rows\":1,\"camera_slots\":["
             "{\"camera_uuid\":\"%s\",\"eptz_mode\":\"cube\"}]}",
             camera.camera_uuid);
    cJSON *json = call(handle_post_live_layout, HTTP_METHOD_POST,
                       "/api/live/layouts", body, 400);
    cJSON_Delete(json);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_create_and_list_layout_with_authorized_camera);
    RUN_TEST(test_rejects_layout_with_unknown_camera);
    RUN_TEST(test_rejects_invalid_eptz_layout_state);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
