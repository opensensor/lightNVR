/**
 * @file test_api_handlers_investigation_bookmarks.c
 * @brief Durable investigation bookmark API tests.
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
#include "web/api_handlers_investigation_bookmarks.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_api_investigation_bookmarks.db"

static stream_config_t create_camera(const char *name) {
    stream_config_t stream;
    memset(&stream, 0, sizeof(stream));
    safe_strcpy(stream.name, name, sizeof(stream.name), 0);
    safe_strcpy(stream.url, "rtsp://camera.example/live",
                sizeof(stream.url), 0);
    safe_strcpy(stream.codec, "h264", sizeof(stream.codec), 0);
    stream.enabled = true;
    stream.streaming_enabled = true;
    stream.record = true;
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&stream));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name(name, &stream));
    return stream;
}

static cJSON *call(void (*handler)(const http_request_t *, http_response_t *),
                   http_method_t method, const char *path, const char *body,
                   int expected_status) {
    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    req.method = method;
    safe_strcpy(req.path, path, sizeof(req.path), 0);
    safe_strcpy(req.uri, path, sizeof(req.uri), 0);
    safe_strcpy(req.client_ip, "127.0.0.1", sizeof(req.client_ip), 0);
    if (body) {
        req.body = (void *)body;
        req.body_len = strlen(body);
    }
    handler(&req, &res);
    TEST_ASSERT_EQUAL_INT(expected_status, res.status_code);
    cJSON *json = res.body ? cJSON_Parse((const char *)res.body) : NULL;
    TEST_ASSERT_NOT_NULL(json);
    http_response_free(&res);
    return json;
}

static void bookmark_path(char *path, size_t size, const char *uuid) {
    snprintf(path, size, "/api/investigation-bookmarks/%s", uuid);
}

static void create_body(char *body, size_t size, const stream_config_t *first,
                        const stream_config_t *second, const char *extra) {
    snprintf(
        body, size,
        "{\"title\":\"Shift handoff\",\"note\":\"Review the loading bay\","
        "\"camera_uuids\":[\"%s\",\"%s\"],"
        "\"start_time\":1700000000,\"end_time\":1700000600,"
        "\"cursor_time\":1700000300,\"primary_camera_uuid\":\"%s\","
        "\"filters\":{\"event_type\":\"detection\","
        "\"min_confidence\":0.75,\"region\":{\"camera_uuid\":\"%s\","
        "\"x\":0.1,\"y\":0.2,\"width\":0.3,\"height\":0.4,"
        "\"match\":\"minimum_intersection\",\"min_intersection\":0.25}},"
        "\"representative_result\":{\"result_id\":\"detection:12\","
        "\"camera_uuid\":\"%s\",\"start_time\":1700000300}%s}",
        first->camera_uuid, second->camera_uuid, first->camera_uuid,
        first->camera_uuid, first->camera_uuid, extra ? extra : "");
}

void setUp(void) {
    sqlite3 *db = get_db_handle();
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
    sqlite3_exec(db, "DELETE FROM investigation_bookmarks;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM streams;", NULL, NULL, NULL);
}

void tearDown(void) {
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
}

void test_bookmark_crud_restores_state_without_retention_hold(void) {
    stream_config_t first = create_camera("Loading Bay");
    stream_config_t second = create_camera("North Door");
    char body[4096];
    create_body(body, sizeof(body), &first, &second, NULL);
    cJSON *json = call(handle_post_investigation_bookmark, HTTP_METHOD_POST,
                       "/api/investigation-bookmarks", body, 201);
    TEST_ASSERT_FALSE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        json, "holds_recordings")));
    TEST_ASSERT_EQUAL_INT(2, cJSON_GetArraySize(
        cJSON_GetObjectItemCaseSensitive(json, "camera_uuids")));
    TEST_ASSERT_EQUAL_STRING(
        "minimum_intersection",
        cJSON_GetObjectItemCaseSensitive(
            cJSON_GetObjectItemCaseSensitive(
                cJSON_GetObjectItemCaseSensitive(json, "filters"), "region"),
            "match")->valuestring);
    char uuid[CAMERA_UUID_STRING_SIZE];
    safe_strcpy(uuid,
        cJSON_GetObjectItemCaseSensitive(json, "uuid")->valuestring,
        sizeof(uuid), 0);
    int revision = cJSON_GetObjectItemCaseSensitive(json, "revision")->valueint;
    cJSON_Delete(json);

    json = call(handle_get_investigation_bookmarks, HTTP_METHOD_GET,
                "/api/investigation-bookmarks", NULL, 200);
    TEST_ASSERT_EQUAL_INT(1,
        cJSON_GetObjectItemCaseSensitive(json, "count")->valueint);
    cJSON_Delete(json);

    char path[MAX_PATH_LENGTH];
    bookmark_path(path, sizeof(path), uuid);
    json = call(handle_get_investigation_bookmark, HTTP_METHOD_GET,
                path, NULL, 200);
    TEST_ASSERT_EQUAL_STRING("Shift handoff",
        cJSON_GetObjectItemCaseSensitive(json, "title")->valuestring);
    cJSON_Delete(json);

    snprintf(body, sizeof(body),
             "{\"title\":\"Updated handoff\",\"note\":\"Ready\","
             "\"revision\":%d}", revision);
    json = call(handle_put_investigation_bookmark, HTTP_METHOD_PUT,
                path, body, 200);
    int updated_revision =
        cJSON_GetObjectItemCaseSensitive(json, "revision")->valueint;
    TEST_ASSERT_EQUAL_INT(revision + 1, updated_revision);
    TEST_ASSERT_EQUAL_STRING("Updated handoff",
        cJSON_GetObjectItemCaseSensitive(json, "title")->valuestring);
    cJSON_Delete(json);

    snprintf(body, sizeof(body), "{\"revision\":%d}", revision);
    json = call(handle_delete_investigation_bookmark, HTTP_METHOD_DELETE,
                path, body, 409);
    cJSON_Delete(json);
    snprintf(body, sizeof(body), "{\"revision\":%d}", updated_revision);
    json = call(handle_delete_investigation_bookmark, HTTP_METHOD_DELETE,
                path, body, 200);
    TEST_ASSERT_TRUE(cJSON_IsTrue(
        cJSON_GetObjectItemCaseSensitive(json, "success")));
    cJSON_Delete(json);
}

void test_bookmark_validation_and_current_camera_recheck(void) {
    stream_config_t first = create_camera("Gate");
    stream_config_t second = create_camera("Garage");
    char body[4096];
    create_body(body, sizeof(body), &first, &second,
                ",\"unexpected\":true");
    cJSON *json = call(handle_post_investigation_bookmark, HTTP_METHOD_POST,
                       "/api/investigation-bookmarks", body, 201);
    char uuid[CAMERA_UUID_STRING_SIZE];
    safe_strcpy(uuid,
        cJSON_GetObjectItemCaseSensitive(json, "uuid")->valuestring,
        sizeof(uuid), 0);
    cJSON_Delete(json);

    snprintf(body, sizeof(body),
             "{\"title\":\"Invalid\",\"camera_uuids\":[\"%s\"],"
             "\"start_time\":1700000000,\"end_time\":1700000600,"
             "\"cursor_time\":1700000300,\"primary_camera_uuid\":\"%s\","
             "\"filters\":{\"credential\":\"secret\"}}",
             first.camera_uuid, first.camera_uuid);
    json = call(handle_post_investigation_bookmark, HTTP_METHOD_POST,
                "/api/investigation-bookmarks", body, 400);
    cJSON_Delete(json);

    TEST_ASSERT_EQUAL_INT(0, delete_stream_config_internal(second.name, true));
    json = call(handle_get_investigation_bookmarks, HTTP_METHOD_GET,
                "/api/investigation-bookmarks", NULL, 200);
    TEST_ASSERT_EQUAL_INT(0,
        cJSON_GetObjectItemCaseSensitive(json, "count")->valueint);
    cJSON_Delete(json);
    char path[MAX_PATH_LENGTH];
    bookmark_path(path, sizeof(path), uuid);
    json = call(handle_get_investigation_bookmark, HTTP_METHOD_GET,
                path, NULL, 404);
    cJSON_Delete(json);
}

void test_demo_mode_has_no_persistent_bookmark_workspace(void) {
    stream_config_t first = create_camera("Demo One");
    stream_config_t second = create_camera("Demo Two");
    char body[4096];
    create_body(body, sizeof(body), &first, &second, NULL);
    g_config.web_auth_enabled = true;
    g_config.demo_mode = true;
    cJSON *json = call(handle_get_investigation_bookmarks, HTTP_METHOD_GET,
                       "/api/investigation-bookmarks", NULL, 200);
    TEST_ASSERT_EQUAL_INT(0,
        cJSON_GetObjectItemCaseSensitive(json, "count")->valueint);
    cJSON_Delete(json);
    json = call(handle_post_investigation_bookmark, HTTP_METHOD_POST,
                "/api/investigation-bookmarks", body, 403);
    cJSON_Delete(json);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_bookmark_crud_restores_state_without_retention_hold);
    RUN_TEST(test_bookmark_validation_and_current_camera_recheck);
    RUN_TEST(test_demo_mode_has_no_persistent_bookmark_workspace);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
