/**
 * @file test_api_handlers_users.c
 * @brief User API tests: legacy API keys are returned only when generated.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include <cjson/cJSON.h>

#include "unity.h"
#include "core/config.h"
#include "database/db_auth.h"
#include "database/db_core.h"
#include "utils/strings.h"
#include "web/api_handlers_users.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_api_users.db"

static int64_t g_user_id;
static char g_api_key[64];

/* Run a handler and return its 200 response body, which the caller frees. */
static char *call(void (*handler)(const http_request_t *, http_response_t *),
                  http_method_t method, const char *path) {
    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    request.method = method;
    safe_strcpy(request.path, path, sizeof(request.path), 0);
    safe_strcpy(request.client_ip, "127.0.0.1", sizeof(request.client_ip), 0);

    handler(&request, &response);

    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    TEST_ASSERT_NOT_NULL(response.body);
    char *body = strdup((const char *)response.body);
    TEST_ASSERT_NOT_NULL(body);
    http_response_free(&response);
    return body;
}

static void assert_user_has_no_api_key(const cJSON *user) {
    TEST_ASSERT_NOT_NULL(user);
    TEST_ASSERT_NULL(cJSON_GetObjectItemCaseSensitive(user, "api_key"));
}

void setUp(void) {
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;

    char path[128];
    snprintf(path, sizeof(path), "/api/auth/users/%lld/api-key",
             (long long)g_user_id);
    char *body = call(handle_users_generate_api_key, HTTP_METHOD_POST, path);
    cJSON *json = cJSON_Parse(body);
    TEST_ASSERT_NOT_NULL(json);
    const cJSON *key = cJSON_GetObjectItemCaseSensitive(json, "api_key");
    TEST_ASSERT_TRUE(cJSON_IsString(key));
    safe_strcpy(g_api_key, key->valuestring, sizeof(g_api_key), 0);
    cJSON_Delete(json);
    free(body);
}

void tearDown(void) {}

void test_generated_key_is_returned_and_authenticates(void) {
    TEST_ASSERT_NOT_EQUAL(0, g_api_key[0]);

    user_t user;
    TEST_ASSERT_EQUAL_INT(0, db_auth_get_user_by_api_key(g_api_key, &user));
    TEST_ASSERT_EQUAL_INT64(g_user_id, user.id);
}

void test_list_does_not_return_api_keys(void) {
    char *body = call(handle_users_list, HTTP_METHOD_GET, "/api/auth/users");
    TEST_ASSERT_NULL(strstr(body, g_api_key));

    cJSON *json = cJSON_Parse(body);
    TEST_ASSERT_NOT_NULL(json);
    const cJSON *users = cJSON_GetObjectItemCaseSensitive(json, "users");
    TEST_ASSERT_TRUE(cJSON_IsArray(users));

    bool found = false;
    const cJSON *user = NULL;
    cJSON_ArrayForEach(user, users) {
        assert_user_has_no_api_key(user);
        const cJSON *id = cJSON_GetObjectItemCaseSensitive(user, "id");
        if (cJSON_IsNumber(id) && (int64_t)id->valuedouble == g_user_id) {
            found = true;
        }
    }
    TEST_ASSERT_TRUE(found);

    cJSON_Delete(json);
    free(body);
}

void test_get_does_not_return_api_key(void) {
    char path[64];
    snprintf(path, sizeof(path), "/api/auth/users/%lld", (long long)g_user_id);

    char *body = call(handle_users_get, HTTP_METHOD_GET, path);
    TEST_ASSERT_NULL(strstr(body, g_api_key));

    cJSON *json = cJSON_Parse(body);
    assert_user_has_no_api_key(json);
    TEST_ASSERT_EQUAL_INT64(
        g_user_id,
        (int64_t)cJSON_GetObjectItemCaseSensitive(json, "id")->valuedouble);

    cJSON_Delete(json);
    free(body);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    if (db_auth_init() != 0 ||
        db_auth_create_user("api-key-user", "correct-horse-battery-staple",
                            "api-key-user@example.com", USER_ROLE_API, true,
                            &g_user_id) != 0) {
        fprintf(stderr, "FATAL: test user setup failed\n");
        shutdown_database();
        unlink(TEST_DB_PATH);
        return 1;
    }

    UNITY_BEGIN();
    RUN_TEST(test_generated_key_is_returned_and_authenticates);
    RUN_TEST(test_list_does_not_return_api_keys);
    RUN_TEST(test_get_does_not_return_api_key);
    int result = UNITY_END();

    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
