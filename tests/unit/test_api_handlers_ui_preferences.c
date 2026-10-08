/**
 * @file test_api_handlers_ui_preferences.c
 * @brief Проверки персональных настроек интерфейса и избранных камер.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include <cjson/cJSON.h>
#include <sqlite3.h>

#include "unity.h"
#include "core/authorization.h"
#include "core/config.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#include "database/db_ui_preferences.h"
#include "utils/strings.h"
#include "utils/uuid.h"
#include "web/api_handlers_ui_preferences.h"
#include "web/httpd_utils.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_api_ui_preferences.db"
#define TEST_USER_ID 44001

static int auth_enabled = 1;
static int inaccessible_cameras = 0;
static int64_t authenticated_user_id = TEST_USER_ID;

int __wrap_httpd_check_action_access(const http_request_t *req, user_t *user) {
    (void)req;
    memset(user, 0, sizeof(*user));
    if (!auth_enabled) return 0;
    user->id = authenticated_user_id;
    user->role = USER_ROLE_ADMIN;
    user->is_active = true;
    safe_strcpy(user->username, "api-test", sizeof(user->username), 0);
    safe_strcpy(user->authentication_method, "session",
                sizeof(user->authentication_method), 0);
    if (auth_enabled == 2) {
        user->id = 0;
        safe_strcpy(user->authentication_method, "demo",
                    sizeof(user->authentication_method), 0);
    } else if (auth_enabled == 3) {
        user->authenticated_via_scoped_token = true;
        safe_strcpy(user->authentication_method, "scoped_token",
                    sizeof(user->authentication_method), 0);
    }
    return 1;
}

int __real_authorization_filter_cameras(const user_t *user,
    authorization_action_t action, fleet_camera_t *cameras, int *count);
int __wrap_authorization_filter_cameras(const user_t *user,
    authorization_action_t action, fleet_camera_t *cameras, int *count) {
    if (inaccessible_cameras) {
        *count = 0;
        return 0;
    }
    return __real_authorization_filter_cameras(user, action, cameras, count);
}

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
    if (body) {
        request.body = (void *)body;
        request.body_len = strlen(body);
    }
    handler(&request, &response);
    if (response.status_code != expected_status) {
        fprintf(stderr, "UI preferences API expected %d, got %d: %s\n",
                expected_status, response.status_code,
                response.body ? (const char *)response.body : "(empty)");
    }
    TEST_ASSERT_EQUAL_INT(expected_status, response.status_code);
    cJSON *json = response.body ? cJSON_Parse((const char *)response.body) : NULL;
    TEST_ASSERT_NOT_NULL(json);
    http_response_free(&response);
    return json;
}

static void create_user(void) {
    sqlite3_exec(get_db_handle(),
        "INSERT OR IGNORE INTO users(id,username,password_hash,salt,role) "
        "VALUES(" "44001,'ui-test','x','x','admin');", NULL, NULL, NULL);
}

static stream_config_t create_camera(const char *name) {
    stream_config_t stream;
    memset(&stream, 0, sizeof(stream));
    safe_strcpy(stream.name, name, sizeof(stream.name), 0);
    safe_strcpy(stream.url, "rtsp://camera/test", sizeof(stream.url), 0);
    safe_strcpy(stream.codec, "h264", sizeof(stream.codec), 0);
    stream.enabled = true;
    stream.streaming_enabled = true;
    stream.width = 1280;
    stream.height = 720;
    stream.fps = 25;
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&stream));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name(name, &stream));
    return stream;
}

static int row_count(const char *sql) {
    sqlite3_stmt *stmt = NULL;
    int count = -1;
    if (sqlite3_prepare_v2(get_db_handle(), sql, -1, &stmt, NULL) == SQLITE_OK &&
        sqlite3_step(stmt) == SQLITE_ROW) {
        count = sqlite3_column_int(stmt, 0);
    }
    if (stmt) sqlite3_finalize(stmt);
    return count;
}

void setUp(void) {
    auth_enabled = 1;
    inaccessible_cameras = 0;
    authenticated_user_id = TEST_USER_ID;
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
    sqlite3_exec(get_db_handle(), "DELETE FROM user_favorites;", NULL, NULL, NULL);
    sqlite3_exec(get_db_handle(), "DELETE FROM user_preferences;", NULL, NULL, NULL);
    sqlite3_exec(get_db_handle(), "DELETE FROM users WHERE id IN (44001,44002);",
                 NULL, NULL, NULL);
    sqlite3_exec(get_db_handle(), "DELETE FROM streams;", NULL, NULL, NULL);
    create_user();
}

void tearDown(void) {}

void test_preferences_default_persistence_and_validation(void) {
    char mode[8] = "";
    TEST_ASSERT_EQUAL_INT(0, db_user_ui_mode_get(TEST_USER_ID, mode, sizeof(mode)));
    TEST_ASSERT_EQUAL_STRING("auto", mode);

    cJSON *json = call(handle_get_ui_preferences, HTTP_METHOD_GET,
                       "/api/ui/preferences", NULL, 200);
    TEST_ASSERT_EQUAL_STRING("auto", cJSON_GetObjectItemCaseSensitive(json,
        "ui_mode")->valuestring);
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(json,
        "configurable")));
    cJSON_Delete(json);

    json = call(handle_put_ui_preferences, HTTP_METHOD_PUT,
        "/api/ui/preferences", "{\"ui_mode\":\"desktop\"}", 200);
    TEST_ASSERT_EQUAL_STRING("desktop", cJSON_GetObjectItemCaseSensitive(json,
        "ui_mode")->valuestring);
    cJSON_Delete(json);
    memset(mode, 0, sizeof(mode));
    TEST_ASSERT_EQUAL_INT(0, db_user_ui_mode_get(TEST_USER_ID, mode, sizeof(mode)));
    TEST_ASSERT_EQUAL_STRING("desktop", mode);

    json = call(handle_put_ui_preferences, HTTP_METHOD_PUT,
        "/api/ui/preferences", "{\"ui_mode\":\"invalid\"}", 400);
    cJSON_Delete(json);
    json = call(handle_put_ui_preferences, HTTP_METHOD_PUT,
        "/api/ui/preferences", "{\"ui_mode\":\"mobile\",\"extra\":true}", 400);
    cJSON_Delete(json);
    json = call(handle_put_ui_preferences, HTTP_METHOD_PUT,
        "/api/ui/preferences", "{\"ui_mode\":\"mobile\",\"user_id\":44002}", 400);
    cJSON_Delete(json);
    json = call(handle_post_ui_favorite, HTTP_METHOD_POST,
        "/api/ui/favorites", "{\"camera_uuid\":\"bad\"}", 400);
    cJSON_Delete(json);
    json = call(handle_post_ui_favorite, HTTP_METHOD_POST,
        "/api/ui/favorites", "{\"camera_uuid\":\"00000000-0000-4000-8000-000000000000\",\"extra\":1}", 400);
    cJSON_Delete(json);
    json = call(handle_post_ui_favorite, HTTP_METHOD_POST,
        "/api/ui/favorites", "{\"camera_uuid\":\"00000000-0000-4000-8000-000000000000\"}", 404);
    cJSON_Delete(json);
    json = call(handle_post_ui_favorite, HTTP_METHOD_POST,
        "/api/ui/favorites", "{\"camera_uuid\":\"00000000-0000-4000-8000-000000000000\",\"user_id\":44002}", 400);
    cJSON_Delete(json);
}

void test_preferences_are_isolated_between_users(void) {
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(get_db_handle(),
        "INSERT INTO users(id,username,password_hash,salt,role) "
        "VALUES(44002,'ui-test-other','x','x','admin');", NULL, NULL, NULL));
    cJSON *json = call(handle_put_ui_preferences, HTTP_METHOD_PUT,
        "/api/ui/preferences", "{\"ui_mode\":\"mobile\"}", 200);
    cJSON_Delete(json);

    authenticated_user_id = TEST_USER_ID + 1;
    json = call(handle_get_ui_preferences, HTTP_METHOD_GET,
        "/api/ui/preferences", NULL, 200);
    TEST_ASSERT_EQUAL_STRING("auto", cJSON_GetObjectItemCaseSensitive(json,
        "ui_mode")->valuestring);
    cJSON_Delete(json);
    json = call(handle_put_ui_preferences, HTTP_METHOD_PUT,
        "/api/ui/preferences", "{\"ui_mode\":\"desktop\"}", 200);
    cJSON_Delete(json);

    authenticated_user_id = TEST_USER_ID;
    json = call(handle_get_ui_preferences, HTTP_METHOD_GET,
        "/api/ui/preferences", NULL, 200);
    TEST_ASSERT_EQUAL_STRING("mobile", cJSON_GetObjectItemCaseSensitive(json,
        "ui_mode")->valuestring);
    cJSON_Delete(json);
    authenticated_user_id = TEST_USER_ID + 1;
    json = call(handle_get_ui_preferences, HTTP_METHOD_GET,
        "/api/ui/preferences", NULL, 200);
    TEST_ASSERT_EQUAL_STRING("desktop", cJSON_GetObjectItemCaseSensitive(json,
        "ui_mode")->valuestring);
    cJSON_Delete(json);
}

void test_favorites_are_idempotent_isolated_and_access_filtered(void) {
    stream_config_t camera = create_camera("UI favorites camera");
    char body[128];
    snprintf(body, sizeof(body), "{\"camera_uuid\":\"%s\"}",
             camera.camera_uuid);
    cJSON *first = call(handle_post_ui_favorite, HTTP_METHOD_POST,
        "/api/ui/favorites", body, 200);
    int64_t created_at = (int64_t)cJSON_GetObjectItemCaseSensitive(
        first, "created_at")->valuedouble;
    cJSON *second = call(handle_post_ui_favorite, HTTP_METHOD_POST,
        "/api/ui/favorites", body, 200);
    TEST_ASSERT_EQUAL_INT64(created_at, (int64_t)cJSON_GetObjectItemCaseSensitive(
        second, "created_at")->valuedouble);
    cJSON_Delete(first);
    cJSON_Delete(second);

    user_favorite_t rows[4];
    TEST_ASSERT_EQUAL_INT(1, db_user_favorites_list(TEST_USER_ID, rows, 4));
    TEST_ASSERT_EQUAL_STRING(camera.camera_uuid, rows[0].camera_uuid);
    TEST_ASSERT_EQUAL_INT(0, db_user_favorites_list(TEST_USER_ID + 1, rows, 4));
    sqlite3_exec(get_db_handle(),
        "INSERT OR IGNORE INTO users(id,username,password_hash,salt,role) "
        "VALUES(44002,'ui-test-other','x','x','admin');", NULL, NULL, NULL);
    authenticated_user_id = TEST_USER_ID + 1;
    cJSON *other_user = call(handle_get_ui_favorites, HTTP_METHOD_GET,
        "/api/ui/favorites", NULL, 200);
    TEST_ASSERT_EQUAL_INT(0, cJSON_GetArraySize(cJSON_GetObjectItemCaseSensitive(
        other_user, "favorites")));
    cJSON_Delete(other_user);
    char delete_path[128];
    snprintf(delete_path, sizeof(delete_path), "/api/ui/favorites/%s",
             camera.camera_uuid);
    cJSON *deleted = call(handle_delete_ui_favorite, HTTP_METHOD_DELETE,
        delete_path, NULL, 200);
    cJSON_Delete(deleted);
    authenticated_user_id = TEST_USER_ID;
    TEST_ASSERT_EQUAL_INT(1, db_user_favorites_list(TEST_USER_ID, rows, 4));

    stream_config_t renamed = camera;
    safe_strcpy(renamed.name, "UI favorites renamed camera",
                sizeof(renamed.name), 0);
    TEST_ASSERT_EQUAL_INT(0, update_stream_config(camera.name, &renamed));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_uuid(camera.camera_uuid,
        &renamed));
    TEST_ASSERT_EQUAL_STRING("UI favorites renamed camera", renamed.name);
    cJSON *after_rename = call(handle_get_ui_favorites, HTTP_METHOD_GET,
        "/api/ui/favorites", NULL, 200);
    cJSON *renamed_items = cJSON_GetObjectItemCaseSensitive(after_rename,
        "favorites");
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(renamed_items));
    TEST_ASSERT_EQUAL_STRING(camera.camera_uuid,
        cJSON_GetObjectItemCaseSensitive(cJSON_GetArrayItem(renamed_items, 0),
            "camera_uuid")->valuestring);
    cJSON_Delete(after_rename);

    inaccessible_cameras = 1;
    cJSON *listed = call(handle_get_ui_favorites, HTTP_METHOD_GET,
        "/api/ui/favorites", NULL, 200);
    TEST_ASSERT_EQUAL_INT(0, cJSON_GetArraySize(cJSON_GetObjectItemCaseSensitive(
        listed, "favorites")));
    cJSON_Delete(listed);
    cJSON *denied = call(handle_post_ui_favorite, HTTP_METHOD_POST,
        "/api/ui/favorites", body, 404);
    cJSON_Delete(denied);
    inaccessible_cameras = 0;

    deleted = call(handle_delete_ui_favorite, HTTP_METHOD_DELETE,
        delete_path, NULL, 200);
    cJSON_Delete(deleted);
    deleted = call(handle_delete_ui_favorite, HTTP_METHOD_DELETE,
        delete_path, NULL, 200);
    cJSON_Delete(deleted);
    TEST_ASSERT_EQUAL_INT(0, db_user_favorites_list(TEST_USER_ID, rows, 4));
}

void test_authentication_and_non_user_write_restrictions(void) {
    auth_enabled = 0;
    cJSON *json = call(handle_get_ui_preferences, HTTP_METHOD_GET,
        "/api/ui/preferences", NULL, 401);
    cJSON_Delete(json);

    auth_enabled = 2;
    json = call(handle_get_ui_preferences, HTTP_METHOD_GET,
        "/api/ui/preferences", NULL, 200);
    TEST_ASSERT_FALSE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(json,
        "configurable")));
    cJSON_Delete(json);
    json = call(handle_put_ui_preferences, HTTP_METHOD_PUT,
        "/api/ui/preferences", "{\"ui_mode\":\"mobile\"}", 403);
    cJSON_Delete(json);
    json = call(handle_post_ui_favorite, HTTP_METHOD_POST,
        "/api/ui/favorites", "{\"camera_uuid\":\"bad\"}", 403);
    cJSON_Delete(json);

    auth_enabled = 3;
    json = call(handle_get_ui_favorites, HTTP_METHOD_GET,
        "/api/ui/favorites", NULL, 200);
    TEST_ASSERT_FALSE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(json,
        "configurable")));
    cJSON_Delete(json);
    json = call(handle_delete_ui_favorite, HTTP_METHOD_DELETE,
        "/api/ui/favorites/00000000-0000-4000-8000-000000000000", NULL, 403);
    cJSON_Delete(json);
}

void test_foreign_key_cascades_on_user_and_camera_deletion(void) {
    stream_config_t camera = create_camera("Cascade camera");
    int64_t ignored_time = 0;
    TEST_ASSERT_EQUAL_INT(0, db_user_favorite_add(TEST_USER_ID,
        camera.camera_uuid, &ignored_time));
    TEST_ASSERT_EQUAL_INT(0, db_user_ui_mode_set(TEST_USER_ID, "desktop"));
    TEST_ASSERT_EQUAL_INT(1, row_count(
        "SELECT count(*) FROM user_preferences WHERE user_id=44001;"));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(get_db_handle(),
        "DELETE FROM users WHERE id=44001;", NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(0, row_count(
        "SELECT count(*) FROM user_favorites WHERE user_id=44001;"));
    TEST_ASSERT_EQUAL_INT(0, row_count(
        "SELECT count(*) FROM user_preferences WHERE user_id=44001;"));

    create_user();
    TEST_ASSERT_EQUAL_INT(0, db_user_favorite_add(TEST_USER_ID,
        camera.camera_uuid, &ignored_time));
    TEST_ASSERT_EQUAL_INT(0, delete_stream_config_internal(camera.name, true));
    user_favorite_t rows[2];
    TEST_ASSERT_EQUAL_INT(0, db_user_favorites_list(TEST_USER_ID, rows, 2));
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_preferences_default_persistence_and_validation);
    RUN_TEST(test_preferences_are_isolated_between_users);
    RUN_TEST(test_favorites_are_idempotent_isolated_and_access_filtered);
    RUN_TEST(test_authentication_and_non_user_write_restrictions);
    RUN_TEST(test_foreign_key_cascades_on_user_and_camera_deletion);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
