/**
 * @file test_api_handlers_camera_favorites.c
 * @brief Camera favorites API tests: idempotent writes, scope, read-only identities.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include <cjson/cJSON.h>
#include <sqlite3.h>

#include "unity.h"
#include "core/config.h"
#include "database/db_auth.h"
#include "database/db_authorization.h"
#include "database/db_camera_favorites.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#include "utils/strings.h"
#include "utils/uuid.h"
#include "web/api_handlers_camera_favorites.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_api_camera_favorites.db"
#define VIEWER_ROLE_UUID "00000000-0000-4000-8000-000000000003"

static char request_api_key[128];

static cJSON *call(void (*handler)(const http_request_t *, http_response_t *),
                   http_method_t method, const char *path, int expected_status) {
    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    request.method = method;
    safe_strcpy(request.path, path, sizeof(request.path), 0);
    safe_strcpy(request.client_ip, "127.0.0.1", sizeof(request.client_ip), 0);
    request.body = (void *)"";
    request.body_len = 0;
    if (request_api_key[0]) {
        safe_strcpy(request.headers[0].name, "X-API-Key",
                    sizeof(request.headers[0].name), 0);
        safe_strcpy(request.headers[0].value, request_api_key,
                    sizeof(request.headers[0].value), 0);
        request.num_headers = 1;
    }
    handler(&request, &response);
    if (response.status_code != expected_status) {
        fprintf(stderr, "camera favorites API expected %d, got %d: %s\n",
                expected_status, response.status_code,
                response.body ? (const char *)response.body : "(empty)");
    }
    TEST_ASSERT_EQUAL_INT(expected_status, response.status_code);
    cJSON *json = response.body ? cJSON_Parse((const char *)response.body) : NULL;
    TEST_ASSERT_NOT_NULL(json);
    http_response_free(&response);
    return json;
}

static stream_config_t create_camera(const char *name) {
    stream_config_t stream;
    memset(&stream, 0, sizeof(stream));
    safe_strcpy(stream.name, name, sizeof(stream.name), 0);
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

static void favorite_path(char *buffer, size_t size, const char *camera_uuid) {
    snprintf(buffer, size, "/api/camera-favorites/%s", camera_uuid);
}

static int favorites_count(void) {
    cJSON *listed = call(handle_get_camera_favorites, HTTP_METHOD_GET,
                         "/api/camera-favorites", 200);
    int count = cJSON_GetArraySize(cJSON_GetObjectItemCaseSensitive(listed, "favorites"));
    cJSON_Delete(listed);
    return count;
}

void setUp(void) {
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
    request_api_key[0] = '\0';
    sqlite3_exec(get_db_handle(), "DELETE FROM camera_favorites;", NULL, NULL, NULL);
    sqlite3_exec(get_db_handle(), "DELETE FROM streams;", NULL, NULL, NULL);
    sqlite3_exec(get_db_handle(), "DELETE FROM users WHERE username != 'admin';",
                 NULL, NULL, NULL);
}

void tearDown(void) {
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
}

void test_put_list_and_delete_favorite(void) {
    stream_config_t camera = create_camera("Lobby");
    char path[256];
    favorite_path(path, sizeof(path), camera.camera_uuid);

    cJSON *added = call(handle_put_camera_favorite, HTTP_METHOD_PUT, path, 200);
    TEST_ASSERT_EQUAL_STRING(camera.camera_uuid,
        cJSON_GetObjectItemCaseSensitive(added, "camera_uuid")->valuestring);
    double created_at = cJSON_GetObjectItemCaseSensitive(added, "created_at")->valuedouble;
    TEST_ASSERT_TRUE(created_at > 0);
    cJSON_Delete(added);

    cJSON *listed = call(handle_get_camera_favorites, HTTP_METHOD_GET,
                         "/api/camera-favorites", 200);
    cJSON *favorites = cJSON_GetObjectItemCaseSensitive(listed, "favorites");
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(favorites));
    TEST_ASSERT_EQUAL_STRING(camera.camera_uuid,
        cJSON_GetObjectItemCaseSensitive(cJSON_GetArrayItem(favorites, 0),
                                         "camera_uuid")->valuestring);
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(listed, "can_modify")));
    cJSON_Delete(listed);

    /* Repeating the PUT keeps the original record. */
    cJSON *repeat = call(handle_put_camera_favorite, HTTP_METHOD_PUT, path, 200);
    TEST_ASSERT_EQUAL_INT64((int64_t)created_at,
        (int64_t)cJSON_GetObjectItemCaseSensitive(repeat, "created_at")->valuedouble);
    cJSON_Delete(repeat);
    TEST_ASSERT_EQUAL_INT(1, favorites_count());

    cJSON *deleted = call(handle_delete_camera_favorite, HTTP_METHOD_DELETE, path, 200);
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(deleted, "removed")));
    cJSON_Delete(deleted);
    TEST_ASSERT_EQUAL_INT(0, favorites_count());

    /* Deleting again is a harmless no-op. */
    deleted = call(handle_delete_camera_favorite, HTTP_METHOD_DELETE, path, 200);
    TEST_ASSERT_FALSE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(deleted, "removed")));
    cJSON_Delete(deleted);
}

void test_rejects_invalid_and_unknown_cameras(void) {
    char unknown[CAMERA_UUID_STRING_SIZE];
    char path[256];
    TEST_ASSERT_EQUAL_INT(0, lightnvr_uuid_generate_v4(unknown));

    cJSON *json = call(handle_put_camera_favorite, HTTP_METHOD_PUT,
                       "/api/camera-favorites/not-a-uuid", 400);
    cJSON_Delete(json);
    json = call(handle_delete_camera_favorite, HTTP_METHOD_DELETE,
                "/api/camera-favorites/not-a-uuid", 400);
    cJSON_Delete(json);

    favorite_path(path, sizeof(path), unknown);
    json = call(handle_put_camera_favorite, HTTP_METHOD_PUT, path, 404);
    cJSON_Delete(json);
}

void test_demo_identity_is_read_only(void) {
    stream_config_t camera = create_camera("Lobby");
    char path[256];
    favorite_path(path, sizeof(path), camera.camera_uuid);

    /* Unauthenticated requests become the demo viewer when demo mode is on. */
    g_config.web_auth_enabled = true;
    g_config.demo_mode = true;

    cJSON *listed = call(handle_get_camera_favorites, HTTP_METHOD_GET,
                         "/api/camera-favorites", 200);
    TEST_ASSERT_EQUAL_INT(0, cJSON_GetArraySize(
        cJSON_GetObjectItemCaseSensitive(listed, "favorites")));
    TEST_ASSERT_FALSE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(listed, "can_modify")));
    cJSON_Delete(listed);

    cJSON *json = call(handle_put_camera_favorite, HTTP_METHOD_PUT, path, 403);
    cJSON_Delete(json);
    json = call(handle_delete_camera_favorite, HTTP_METHOD_DELETE, path, 403);
    cJSON_Delete(json);
}

void test_policy_authenticated_favorites_respect_live_view_scope(void) {
    stream_config_t allowed = create_camera("Allowed");
    stream_config_t denied = create_camera("Denied");
    stream_config_t other = create_camera("Other");
    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_user("scopedviewer", "password123",
        NULL, USER_ROLE_VIEWER, true, &user_id));
    TEST_ASSERT_EQUAL_INT(0, db_authorization_set_user_mode(user_id, "policy"));
    char selector[512];
    snprintf(selector, sizeof(selector),
        "{\"version\":1,\"expression\":{\"op\":\"camera_uuid\","
        "\"values\":[\"%s\"]}}", allowed.camera_uuid);
    char grant_uuid[CAMERA_UUID_STRING_SIZE];
    TEST_ASSERT_EQUAL_INT(0, db_authorization_create_user_grant(user_id,
        VIEWER_ROLE_UUID, "selector", selector, NULL, grant_uuid));
    TEST_ASSERT_EQUAL_INT(0, db_auth_generate_api_key(user_id,
        request_api_key, sizeof(request_api_key)));

    /* A saved favorite can leave the user's scope after a policy change. */
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_FAVORITE_OK,
        db_camera_favorite_add(user_id, allowed.camera_uuid, NULL));
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_FAVORITE_OK,
        db_camera_favorite_add(user_id, denied.camera_uuid, NULL));
    /* The installation's set is separate from the authenticated user's set. */
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_FAVORITE_OK,
        db_camera_favorite_add(0, other.camera_uuid, NULL));
    g_config.web_auth_enabled = true;

    cJSON *listed = call(handle_get_camera_favorites, HTTP_METHOD_GET,
                         "/api/camera-favorites", 200);
    cJSON *favorites = cJSON_GetObjectItemCaseSensitive(listed, "favorites");
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(favorites));
    TEST_ASSERT_EQUAL_STRING(allowed.camera_uuid,
        cJSON_GetObjectItemCaseSensitive(cJSON_GetArrayItem(favorites, 0),
                                         "camera_uuid")->valuestring);
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(listed, "can_modify")));
    cJSON_Delete(listed);

    char path[256];
    favorite_path(path, sizeof(path), allowed.camera_uuid);
    cJSON *json = call(handle_put_camera_favorite, HTTP_METHOD_PUT, path, 200);
    cJSON_Delete(json);

    favorite_path(path, sizeof(path), denied.camera_uuid);
    cJSON *denied_response = call(handle_put_camera_favorite, HTTP_METHOD_PUT, path, 404);
    favorite_path(path, sizeof(path), other.camera_uuid);
    json = call(handle_put_camera_favorite, HTTP_METHOD_PUT, path, 404);
    cJSON_Delete(json);

    char unknown[CAMERA_UUID_STRING_SIZE];
    TEST_ASSERT_EQUAL_INT(0, lightnvr_uuid_generate_v4(unknown));
    favorite_path(path, sizeof(path), unknown);
    json = call(handle_put_camera_favorite, HTTP_METHOD_PUT, path, 404);
    TEST_ASSERT_TRUE(cJSON_Compare(denied_response, json, true));
    cJSON_Delete(denied_response);
    cJSON_Delete(json);

    camera_favorite_t stored[4];
    TEST_ASSERT_EQUAL_INT(2, db_camera_favorites_list(user_id, stored, 4));
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_put_list_and_delete_favorite);
    RUN_TEST(test_rejects_invalid_and_unknown_cameras);
    RUN_TEST(test_demo_identity_is_read_only);
    RUN_TEST(test_policy_authenticated_favorites_respect_live_view_scope);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
