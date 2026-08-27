/**
 * @file test_api_handlers_eptz_presets.c
 * @brief Browser ePTZ operator preset API tests.
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
#include "web/api_handlers_eptz_presets.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_api_eptz_presets.db"

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
        fprintf(stderr, "ePTZ preset API expected %d, got %d: %s\n",
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
    safe_strcpy(stream.name, "Dome", sizeof(stream.name), 0);
    safe_strcpy(stream.url, "rtsp://camera/dome", sizeof(stream.url), 0);
    safe_strcpy(stream.codec, "h264", sizeof(stream.codec), 0);
    stream.enabled = true;
    stream.streaming_enabled = true;
    stream.width = 2048;
    stream.height = 2048;
    stream.fps = 20;
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&stream));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name(stream.name, &stream));
    return stream;
}

static void preset_body(char *body, size_t size, const char *name,
                        const char *mode, int revision) {
    snprintf(body, size,
             "{\"name\":\"%s\",\"is_shared\":false,"
             "\"mode\":\"%s\",\"yaw\":15,\"tilt\":-45,"
             "\"view_fov\":70,\"secondary_yaw\":-165,"
             "\"secondary_tilt\":-40,\"secondary_view_fov\":65%s}",
             name, mode, revision > 0 ? ",\"revision\":1" : "");
}

void setUp(void) {
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
    sqlite3_exec(get_db_handle(), "DELETE FROM eptz_operator_presets;",
                 NULL, NULL, NULL);
    sqlite3_exec(get_db_handle(), "DELETE FROM streams;", NULL, NULL, NULL);
}

void tearDown(void) {}

void test_eptz_preset_crud_is_camera_scoped(void) {
    stream_config_t camera = create_camera();
    char collection_path[160];
    snprintf(collection_path, sizeof(collection_path),
             "/api/cameras/%s/eptz-presets", camera.camera_uuid);
    char body[1024];
    preset_body(body, sizeof(body), "Entrance", "dewarp", 0);
    cJSON *created = call(handle_post_eptz_preset, HTTP_METHOD_POST,
                          collection_path, body, 201);
    const char *preset_uuid = cJSON_GetObjectItemCaseSensitive(
        created, "uuid")->valuestring;
    char saved_uuid[CAMERA_UUID_STRING_SIZE];
    safe_strcpy(saved_uuid, preset_uuid, sizeof(saved_uuid), 0);
    cJSON_Delete(created);

    cJSON *listed = call(handle_get_eptz_presets, HTTP_METHOD_GET,
                         collection_path, NULL, 200);
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(
        cJSON_GetObjectItemCaseSensitive(listed, "presets")));
    cJSON_Delete(listed);

    char item_path[220];
    snprintf(item_path, sizeof(item_path), "%s/%s",
             collection_path, saved_uuid);
    preset_body(body, sizeof(body), "Entrance", "dual", 1);
    cJSON *updated = call(handle_put_eptz_preset, HTTP_METHOD_PUT,
                          item_path, body, 200);
    TEST_ASSERT_EQUAL_STRING(
        "dual", cJSON_GetObjectItemCaseSensitive(updated, "mode")->valuestring);
    TEST_ASSERT_EQUAL_INT64(
        2, (int64_t)cJSON_GetObjectItemCaseSensitive(
            updated, "revision")->valuedouble);
    cJSON_Delete(updated);

    cJSON *deleted = call(handle_delete_eptz_preset, HTTP_METHOD_DELETE,
                          item_path, "{\"revision\":2}", 200);
    TEST_ASSERT_TRUE(cJSON_IsTrue(
        cJSON_GetObjectItemCaseSensitive(deleted, "success")));
    cJSON_Delete(deleted);
}

void test_rejects_unknown_renderer_mode(void) {
    stream_config_t camera = create_camera();
    char path[160];
    snprintf(path, sizeof(path), "/api/cameras/%s/eptz-presets",
             camera.camera_uuid);
    char body[1024];
    preset_body(body, sizeof(body), "Bad", "cube", 0);
    cJSON *json = call(handle_post_eptz_preset, HTTP_METHOD_POST,
                       path, body, 400);
    cJSON_Delete(json);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_eptz_preset_crud_is_camera_scoped);
    RUN_TEST(test_rejects_unknown_renderer_mode);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
