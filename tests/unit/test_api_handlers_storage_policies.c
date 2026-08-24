/**
 * @file test_api_handlers_storage_policies.c
 * @brief Storage placement policy API tests.
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
#include "web/api_handlers_storage_policies.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_api_storage_policies.db"

static char default_root[] = "/tmp/lightnvr-api-policy-default-XXXXXX";
static char primary_root[] = "/tmp/lightnvr-api-policy-primary-XXXXXX";
static char default_uuid[LIGHTNVR_UUID_STRING_SIZE];
static storage_target_t primary_target;

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
                   const char *body, int expected_status) {
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
    handler(&request, &response);
    TEST_ASSERT_EQUAL_INT(expected_status, response.status_code);
    cJSON *json = response.body
        ? cJSON_Parse((const char *)response.body) : NULL;
    TEST_ASSERT_NOT_NULL(json);
    http_response_free(&response);
    return json;
}

void setUp(void) {
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(
        get_db_handle(),
        "DELETE FROM recordings;DELETE FROM storage_policies;"
        "DELETE FROM storage_targets;DELETE FROM audit_events;",
        NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(
        0, db_storage_target_bootstrap_default(default_root, default_uuid));
    memset(&primary_target, 0, sizeof(primary_target));
    safe_strcpy(primary_target.name, "Primary", sizeof(primary_target.name), 0);
    safe_strcpy(primary_target.target_type, "filesystem",
                sizeof(primary_target.target_type), 0);
    safe_strcpy(primary_target.root_path, primary_root,
                sizeof(primary_target.root_path), 0);
    safe_strcpy(primary_target.storage_class, "hot",
                sizeof(primary_target.storage_class), 0);
    primary_target.enabled = true;
    primary_target.high_watermark_pct = 99;
    primary_target.low_watermark_pct = 95;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK,
                          db_storage_target_create(&primary_target));
}

void tearDown(void) {}

void test_policy_api_crud_and_revision_guard(void) {
    char body[1200];
    snprintf(body, sizeof(body),
             "{\"name\":\"Lobby cameras\",\"enabled\":true,"
             "\"priority\":200,\"selector\":{\"version\":1,"
             "\"expression\":{\"op\":\"all\"}},"
             "\"primary_target_uuid\":\"%s\","
             "\"fallback_mode\":\"default\","
             "\"fallback_target_uuid\":null}", primary_target.uuid);
    cJSON *json = call(handle_post_storage_policy, HTTP_METHOD_POST,
                       "/api/storage-policies", NULL, body, 201);
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    safe_strcpy(uuid, cJSON_GetObjectItemCaseSensitive(
                        json, "uuid")->valuestring, sizeof(uuid), 0);
    TEST_ASSERT_EQUAL_INT(200, cJSON_GetObjectItemCaseSensitive(
                                   json, "priority")->valueint);
    cJSON_Delete(json);

    json = call(handle_get_storage_policies, HTTP_METHOD_GET,
                "/api/storage-policies", NULL, NULL, 200);
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetObjectItemCaseSensitive(
                                 json, "count")->valueint);
    cJSON_Delete(json);

    char path[160];
    snprintf(path, sizeof(path), "/api/storage-policies/%s", uuid);
    snprintf(body, sizeof(body),
             "{\"name\":\"Lobby cameras\",\"enabled\":true,"
             "\"priority\":200,\"selector\":{\"version\":1,"
             "\"expression\":{\"op\":\"all\"}},"
             "\"primary_target_uuid\":\"%s\","
             "\"fallback_mode\":\"pause\","
             "\"fallback_target_uuid\":null,\"revision\":1}",
             primary_target.uuid);
    json = call(handle_put_storage_policy, HTTP_METHOD_PUT, path, NULL,
                body, 200);
    TEST_ASSERT_EQUAL_INT(2, cJSON_GetObjectItemCaseSensitive(
                                 json, "revision")->valueint);
    cJSON_Delete(json);
    json = call(handle_put_storage_policy, HTTP_METHOD_PUT, path, NULL,
                body, 409);
    cJSON_Delete(json);

    json = call(handle_delete_storage_policy, HTTP_METHOD_DELETE, path,
                "revision=2", NULL, 200);
    cJSON_Delete(json);
}

void test_policy_api_rejects_primary_as_named_fallback(void) {
    char body[1200];
    snprintf(body, sizeof(body),
             "{\"name\":\"Bad fallback\",\"selector\":{\"version\":1,"
             "\"expression\":{\"op\":\"all\"}},"
             "\"primary_target_uuid\":\"%s\","
             "\"fallback_mode\":\"target\","
             "\"fallback_target_uuid\":\"%s\"}",
             primary_target.uuid, primary_target.uuid);
    cJSON *json = call(handle_post_storage_policy, HTTP_METHOD_POST,
                       "/api/storage-policies", NULL, body, 400);
    cJSON_Delete(json);
}

int main(void) {
    TEST_ASSERT_NOT_NULL(mkdtemp(default_root));
    TEST_ASSERT_NOT_NULL(mkdtemp(primary_root));
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0 || db_auth_init() != 0) {
        fprintf(stderr, "FATAL: failed to initialize storage policy API test\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_policy_api_crud_and_revision_guard);
    RUN_TEST(test_policy_api_rejects_primary_as_named_fallback);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    rmdir(default_root);
    rmdir(primary_root);
    return result;
}
