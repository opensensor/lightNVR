/**
 * @file test_api_handlers_observations.c
 * @brief INT_01 observation API handlers.
 *
 * C2 registers every observation route as an authenticated 501 stub
 * (contract §11 `not_implemented`). L8 replaces the GET stubs and L14 the PUT
 * stub; those tasks extend this file with per-status tests.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdbool.h>
#include <stdio.h>
#include <string.h>

#include <cjson/cJSON.h>

#include "unity.h"
#include "database/db_auth.h"
#include "utils/strings.h"
#include "web/api_handlers_observations.h"
#include "web/request_response.h"

static bool authenticated = true;

int __wrap_httpd_check_action_access(const http_request_t *req, user_t *user) {
    (void)req;
    if (user) memset(user, 0, sizeof(*user));
    return authenticated ? 1 : 0;
}

typedef void (*handler_fn)(const http_request_t *, http_response_t *);

static void expect(handler_fn handler, http_method_t method, const char *path,
                   int status, const char *code) {
    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    request.method = method;
    safe_strcpy(request.path, path, sizeof(request.path), 0);
    handler(&request, &response);
    TEST_ASSERT_EQUAL_INT(status, response.status_code);
    TEST_ASSERT_NOT_NULL(response.body);
    cJSON *json = cJSON_Parse((const char *)response.body);
    TEST_ASSERT_NOT_NULL(json);
    TEST_ASSERT_TRUE(cJSON_IsString(cJSON_GetObjectItemCaseSensitive(json, "error")));
    if (code) {
        cJSON *value = cJSON_GetObjectItemCaseSensitive(json, "code");
        TEST_ASSERT_TRUE(cJSON_IsString(value));
        TEST_ASSERT_EQUAL_STRING(code, value->valuestring);
    }
    cJSON_Delete(json);
    http_response_free(&response);
}

#define OBS "/api/observations/55555555-5555-4555-8555-555555555555"

static const struct {
    handler_fn handler;
    http_method_t method;
    const char *path;
} routes[] = {
    {handle_get_integration_capabilities, HTTP_METHOD_GET, "/api/integrations/capabilities"},
    {handle_get_observations, HTTP_METHOD_GET, "/api/observations"},
    {handle_get_observation, HTTP_METHOD_GET, OBS},
    {handle_get_observation_snapshot, HTTP_METHOD_GET, OBS "/snapshot"},
    {handle_put_observation_classification, HTTP_METHOD_PUT, OBS "/classifications/yawamf/species"},
};

void setUp(void) { authenticated = true; }
void tearDown(void) {}

static void test_stubs_return_not_implemented(void) {
    for (size_t i = 0; i < sizeof(routes) / sizeof(routes[0]); i++)
        expect(routes[i].handler, routes[i].method, routes[i].path, 501, "not_implemented");
}

static void test_stubs_require_authentication(void) {
    authenticated = false;
    for (size_t i = 0; i < sizeof(routes) / sizeof(routes[0]); i++)
        expect(routes[i].handler, routes[i].method, routes[i].path, 401, NULL);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_stubs_return_not_implemented);
    RUN_TEST(test_stubs_require_authentication);
    return UNITY_END();
}
