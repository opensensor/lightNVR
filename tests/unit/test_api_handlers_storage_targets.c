/**
 * @file test_api_handlers_storage_targets.c
 * @brief Storage target API CRUD, probe, RBAC, and safety tests.
 */

#define _POSIX_C_SOURCE 200809L

#include <cjson/cJSON.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "core/config.h"
#include "database/db_auth.h"
#include "database/db_core.h"
#include "database/db_storage_targets.h"
#include "unity.h"
#include "utils/strings.h"
#include "web/api_handlers_storage_targets.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_api_storage_targets.db"

static char default_root[] = "/tmp/lightnvr-api-target-default-XXXXXX";
static char second_root[] = "/tmp/lightnvr-api-target-second-XXXXXX";
static char default_uuid[LIGHTNVR_UUID_STRING_SIZE];

static const char *method_name(http_method_t method) {
    switch (method) {
        case HTTP_METHOD_GET: return "GET";
        case HTTP_METHOD_POST: return "POST";
        case HTTP_METHOD_PUT: return "PUT";
        case HTTP_METHOD_DELETE: return "DELETE";
        default: return "UNKNOWN";
    }
}

static cJSON *call(void (*handler)(const http_request_t *, http_response_t *),
                   http_method_t method, const char *path, const char *query,
                   const char *body, const char *api_key,
                   int expected_status) {
    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    request.method = method;
    safe_strcpy(request.method_str, method_name(method),
                sizeof(request.method_str), 0);
    safe_strcpy(request.path, path, sizeof(request.path), 0);
    safe_strcpy(request.uri, path, sizeof(request.uri), 0);
    safe_strcpy(request.query_string, query ? query : "",
                sizeof(request.query_string), 0);
    safe_strcpy(request.client_ip, "127.0.0.1",
                sizeof(request.client_ip), 0);
    if (body) {
        request.body = (void *)body;
        request.body_len = strlen(body);
    }
    if (api_key) {
        safe_strcpy(request.headers[0].name, "X-API-Key",
                    sizeof(request.headers[0].name), 0);
        safe_strcpy(request.headers[0].value, api_key,
                    sizeof(request.headers[0].value), 0);
        request.num_headers = 1;
    }
    handler(&request, &response);
    TEST_ASSERT_EQUAL_INT(expected_status, response.status_code);
    cJSON *json = response.body
        ? cJSON_Parse((const char *)response.body) : NULL;
    TEST_ASSERT_NOT_NULL(json);
    http_response_free(&response);
    return json;
}

static void remove_user(const char *username) {
    user_t user;
    if (db_auth_get_user_by_username(username, &user) == 0) {
        db_auth_delete_user(user.id);
    }
}

void setUp(void) {
    sqlite3 *db = get_db_handle();
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(
        db, "DELETE FROM detections;DELETE FROM recordings;"
            "DELETE FROM storage_targets;DELETE FROM audit_events;",
        NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(
        0, db_storage_target_bootstrap_default(default_root, default_uuid));
    storage_target_t default_target;
    TEST_ASSERT_EQUAL_INT(
        DB_STORAGE_TARGET_OK,
        db_storage_target_probe(default_uuid, true, &default_target));
    remove_user("storageviewer");
}

void tearDown(void) {
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
}

void test_storage_target_api_creates_lists_probes_and_deletes(void) {
    cJSON *json = call(handle_get_storage_targets, HTTP_METHOD_GET,
                       "/api/storage-targets", NULL, NULL, NULL, 200);
    TEST_ASSERT_EQUAL_INT(1,
        cJSON_GetObjectItemCaseSensitive(json, "count")->valueint);
    cJSON_Delete(json);

    char body[1024];
    snprintf(body, sizeof(body),
             "{\"name\":\"NAS hot 01\",\"root_path\":\"%s\","
             "\"storage_class\":\"hot\",\"enabled\":true,"
             "\"reserve_bytes\":1073741824,"
             "\"low_watermark_pct\":80,\"high_watermark_pct\":90}",
             second_root);
    json = call(handle_post_storage_target, HTTP_METHOD_POST,
                "/api/storage-targets", NULL, body, NULL, 201);
    const char *uuid = cJSON_GetObjectItemCaseSensitive(
        json, "uuid")->valuestring;
    char created_uuid[LIGHTNVR_UUID_STRING_SIZE];
    safe_strcpy(created_uuid, uuid, sizeof(created_uuid), 0);
    int revision = cJSON_GetObjectItemCaseSensitive(
        json, "revision")->valueint;
    TEST_ASSERT_EQUAL_STRING(
        "healthy", cJSON_GetObjectItemCaseSensitive(
            cJSON_GetObjectItemCaseSensitive(json, "health"),
            "status")->valuestring);
    cJSON_Delete(json);

    json = call(handle_get_storage_targets, HTTP_METHOD_GET,
                "/api/storage-targets", NULL, NULL, NULL, 200);
    TEST_ASSERT_EQUAL_INT(2,
        cJSON_GetObjectItemCaseSensitive(json, "count")->valueint);
    cJSON *targets = cJSON_GetObjectItemCaseSensitive(json, "targets");
    TEST_ASSERT_TRUE(cJSON_IsArray(targets));
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        cJSON_GetObjectItemCaseSensitive(cJSON_GetArrayItem(targets, 0),
                                         "health"),
        "duplicate_filesystem")));
    cJSON_Delete(json);

    char path[160];
    snprintf(path, sizeof(path), "/api/storage-targets/%s/probe",
             created_uuid);
    json = call(handle_post_storage_target_probe, HTTP_METHOD_POST, path,
                NULL, NULL, NULL, 200);
    cJSON_Delete(json);

    snprintf(path, sizeof(path), "/api/storage-targets/%s", created_uuid);
    char query[64];
    snprintf(query, sizeof(query), "revision=%d", revision);
    json = call(handle_delete_storage_target, HTTP_METHOD_DELETE, path, query,
                NULL, NULL, 200);
    cJSON_Delete(json);

    snprintf(path, sizeof(path), "/api/storage-targets/%s", default_uuid);
    json = call(handle_delete_storage_target, HTTP_METHOD_DELETE, path,
                "revision=1", NULL, NULL, 409);
    cJSON_Delete(json);
}

void test_unavailable_target_must_be_staged_disabled(void) {
    const char *missing = "/tmp/lightnvr-api-target-not-mounted";
    char body[768];
    snprintf(body, sizeof(body),
             "{\"name\":\"Future NAS\",\"root_path\":\"%s\","
             "\"enabled\":true}", missing);
    cJSON *json = call(handle_post_storage_target, HTTP_METHOD_POST,
                       "/api/storage-targets", NULL, body, NULL, 422);
    cJSON_Delete(json);
    snprintf(body, sizeof(body),
             "{\"name\":\"Future NAS\",\"root_path\":\"%s\","
             "\"enabled\":false}", missing);
    json = call(handle_post_storage_target, HTTP_METHOD_POST,
                "/api/storage-targets", NULL, body, NULL, 201);
    TEST_ASSERT_FALSE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        json, "enabled")));
    TEST_ASSERT_EQUAL_STRING(
        "disabled", cJSON_GetObjectItemCaseSensitive(
            cJSON_GetObjectItemCaseSensitive(json, "health"),
            "status")->valuestring);
    cJSON_Delete(json);
}

void test_viewer_cannot_read_storage_target_configuration(void) {
    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("storageviewer", "password123", NULL,
                               USER_ROLE_VIEWER, true, &user_id));
    char api_key[128] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_generate_api_key(user_id, api_key, sizeof(api_key)));
    g_config.web_auth_enabled = true;
    cJSON *json = call(handle_get_storage_targets, HTTP_METHOD_GET,
                       "/api/storage-targets", NULL, NULL, api_key, 403);
    cJSON_Delete(json);
}

int main(void) {
    TEST_ASSERT_NOT_NULL(mkdtemp(default_root));
    TEST_ASSERT_NOT_NULL(mkdtemp(second_root));
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0 || db_auth_init() != 0) {
        fprintf(stderr, "FATAL: failed to initialize storage target API test\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_storage_target_api_creates_lists_probes_and_deletes);
    RUN_TEST(test_unavailable_target_must_be_staged_disabled);
    RUN_TEST(test_viewer_cannot_read_storage_target_configuration);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    rmdir(default_root);
    rmdir(second_root);
    return result;
}
