/**
 * @file test_api_handlers_workspaces.c
 * @brief Per-user workspace visibility API tests.
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
#include "utils/strings.h"
#include "web/api_handlers_workspaces.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_api_workspaces.db"

static cJSON *call(void (*handler)(const http_request_t *, http_response_t *),
                   http_method_t method, const char *body,
                   int expected_status) {
    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    request.method = method;
    safe_strcpy(request.path, "/api/ui/workspaces", sizeof(request.path), 0);
    safe_strcpy(request.client_ip, "127.0.0.1",
                sizeof(request.client_ip), 0);
    if (body) {
        request.body = (void *)body;
        request.body_len = strlen(body);
    }
    handler(&request, &response);
    if (response.status_code != expected_status) {
        fprintf(stderr, "workspace API expected %d, got %d: %s\n",
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

static cJSON *find_workspace(cJSON *response, const char *key) {
    cJSON *workspace = NULL;
    cJSON_ArrayForEach(
        workspace,
        cJSON_GetObjectItemCaseSensitive(response, "workspaces")) {
        cJSON *candidate =
            cJSON_GetObjectItemCaseSensitive(workspace, "key");
        if (cJSON_IsString(candidate) &&
            strcmp(candidate->valuestring, key) == 0) return workspace;
    }
    return NULL;
}

void setUp(void) {
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
    sqlite3_exec(get_db_handle(), "DELETE FROM user_workspace_preferences;",
                 NULL, NULL, NULL);
}

void tearDown(void) {
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
}

void test_defaults_and_partial_updates_are_installation_scoped(void) {
    cJSON *json = call(handle_get_ui_workspaces, HTTP_METHOD_GET, NULL, 200);
    TEST_ASSERT_EQUAL_STRING(
        "installation",
        cJSON_GetObjectItemCaseSensitive(json, "scope")->valuestring);
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        find_workspace(json, "live.navigator"), "visible")));
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        find_workspace(json, "investigation"), "visible")));
    cJSON_Delete(json);

    json = call(
        handle_put_ui_workspaces, HTTP_METHOD_PUT,
        "{\"workspaces\":{\"investigation\":false}}", 200);
    TEST_ASSERT_FALSE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        find_workspace(json, "investigation"), "visible")));
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        find_workspace(json, "live.navigator"), "visible")));
    cJSON_Delete(json);

    json = call(handle_get_ui_workspaces, HTTP_METHOD_GET, NULL, 200);
    TEST_ASSERT_FALSE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        find_workspace(json, "investigation"), "visible")));
    cJSON_Delete(json);
}

void test_updates_reject_unknown_or_non_boolean_workspaces(void) {
    cJSON *json = call(
        handle_put_ui_workspaces, HTTP_METHOD_PUT,
        "{\"workspaces\":{\"unknown\":true}}", 400);
    cJSON_Delete(json);
    json = call(
        handle_put_ui_workspaces, HTTP_METHOD_PUT,
        "{\"workspaces\":{\"investigation\":\"yes\"}}", 400);
    cJSON_Delete(json);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_defaults_and_partial_updates_are_installation_scoped);
    RUN_TEST(test_updates_reject_unknown_or_non_boolean_workspaces);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
