/**
 * @file test_httpd_utils.c
 * @brief Layer 2 Unity tests for web/httpd_utils.c
 *
 * Tests:
 *   httpd_parse_json_body()             - JSON body parsing
 *   httpd_get_basic_auth_credentials()  - Base64 decode of Authorization header
 *   httpd_get_session_token()           - Cookie session= extraction
 *   httpd_is_demo_mode()               - g_config.demo_mode wrapper
 *   httpd_get_authenticated_user()     - auth-disabled path (no DB needed)
 *   httpd_check_admin_privileges()     - auth-disabled path (no DB needed)
 *
 * All tests use synthetic http_request_t structs; no network or browser needed.
 * "admin:password" in Base64 is "YWRtaW46cGFzc3dvcmQ=".
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include <cjson/cJSON.h>

#include "unity.h"
#include "web/httpd_utils.h"
#include "web/request_response.h"
#include "core/config.h"
#include "database/db_auth.h"

/* ---- external globals from lightnvr_lib ---- */
extern config_t g_config;

/* ---- header helper ---- */
static void add_header(http_request_t *req, const char *name, const char *value) {
    if (req->num_headers >= MAX_HEADERS) return;
    strncpy(req->headers[req->num_headers].name,  name,  127);
    req->headers[req->num_headers].name[127]  = '\0';
    strncpy(req->headers[req->num_headers].value, value, 1023);
    req->headers[req->num_headers].value[1023] = '\0';
    req->num_headers++;
}

/* ---- Unity boilerplate ---- */
void setUp(void) {
    /* Ensure auth is enabled by default so we control path in each test */
    g_config.web_auth_enabled = true;
    g_config.demo_mode = false;
}
void tearDown(void) {}

/* ================================================================
 * httpd_parse_json_body
 * ================================================================ */

void test_parse_json_valid_body(void) {
    const char *js = "{\"foo\":42}";
    http_request_t req;
    http_request_init(&req);
    req.body     = (void *)js;
    req.body_len = strlen(js);

    cJSON *json = httpd_parse_json_body(&req);
    TEST_ASSERT_NOT_NULL(json);

    cJSON *foo = cJSON_GetObjectItem(json, "foo");
    TEST_ASSERT_NOT_NULL(foo);
    TEST_ASSERT_EQUAL_INT(42, (int)cJSON_GetNumberValue(foo));
    cJSON_Delete(json);
}

void test_parse_json_invalid_body_returns_null(void) {
    const char *bad = "not json at all {{{";
    http_request_t req;
    http_request_init(&req);
    req.body     = (void *)bad;
    req.body_len = strlen(bad);

    cJSON *json = httpd_parse_json_body(&req);
    TEST_ASSERT_NULL(json);
}

void test_parse_json_null_request_returns_null(void) {
    cJSON *json = httpd_parse_json_body(NULL);
    TEST_ASSERT_NULL(json);
}

void test_parse_json_empty_body_returns_null(void) {
    http_request_t req;
    http_request_init(&req);
    req.body     = NULL;
    req.body_len = 0;

    cJSON *json = httpd_parse_json_body(&req);
    TEST_ASSERT_NULL(json);
}

/* ================================================================
 * httpd_get_basic_auth_credentials
 * ================================================================ */

void test_basic_auth_valid_credentials(void) {
    /* "admin:password" → "YWRtaW46cGFzc3dvcmQ=" */
    http_request_t req;
    http_request_init(&req);
    add_header(&req, "Authorization", "Basic YWRtaW46cGFzc3dvcmQ=");

    char user[64] = {0}, pass[64] = {0};
    int rc = httpd_get_basic_auth_credentials(&req, user, sizeof(user), pass, sizeof(pass));
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("admin", user);
    TEST_ASSERT_EQUAL_STRING("password", pass);
}

void test_basic_auth_no_header_returns_error(void) {
    http_request_t req;
    http_request_init(&req);

    char user[64], pass[64];
    int rc = httpd_get_basic_auth_credentials(&req, user, sizeof(user), pass, sizeof(pass));
    TEST_ASSERT_EQUAL_INT(-1, rc);
}

void test_basic_auth_wrong_scheme_returns_error(void) {
    http_request_t req;
    http_request_init(&req);
    add_header(&req, "Authorization", "Bearer sometoken123");

    char user[64], pass[64];
    int rc = httpd_get_basic_auth_credentials(&req, user, sizeof(user), pass, sizeof(pass));
    TEST_ASSERT_EQUAL_INT(-1, rc);
}

void test_basic_auth_null_params_returns_error(void) {
    http_request_t req;
    http_request_init(&req);
    add_header(&req, "Authorization", "Basic YWRtaW46cGFzc3dvcmQ=");

    char user[64];
    /* pass buffer NULL */
    int rc = httpd_get_basic_auth_credentials(&req, user, sizeof(user), NULL, 0);
    TEST_ASSERT_EQUAL_INT(-1, rc);
}

/* ================================================================
 * httpd_get_session_token
 * ================================================================ */

void test_get_session_token_valid_cookie(void) {
    http_request_t req;
    http_request_init(&req);
    add_header(&req, "Cookie", "session=abc123");

    char token[64] = {0};
    int rc = httpd_get_session_token(&req, token, sizeof(token));
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("abc123", token);
}

void test_get_session_token_cookie_with_other_fields(void) {
    http_request_t req;
    http_request_init(&req);
    add_header(&req, "Cookie", "lang=en; session=tok42; path=/");

    char token[64] = {0};
    int rc = httpd_get_session_token(&req, token, sizeof(token));
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("tok42", token);
}

void test_get_session_token_no_cookie_header_returns_error(void) {
    http_request_t req;
    http_request_init(&req);

    char token[64];
    int rc = httpd_get_session_token(&req, token, sizeof(token));
    TEST_ASSERT_EQUAL_INT(-1, rc);
}

void test_get_session_token_no_session_key_returns_error(void) {
    http_request_t req;
    http_request_init(&req);
    add_header(&req, "Cookie", "user=bob; theme=dark");

    char token[64];
    int rc = httpd_get_session_token(&req, token, sizeof(token));
    TEST_ASSERT_EQUAL_INT(-1, rc);
}

/* ================================================================
 * httpd_is_demo_mode
 * ================================================================ */

void test_is_demo_mode_false_by_default(void) {
    g_config.demo_mode = false;
    TEST_ASSERT_EQUAL_INT(0, httpd_is_demo_mode());
}

void test_is_demo_mode_true_when_set(void) {
    g_config.demo_mode = true;
    TEST_ASSERT_EQUAL_INT(1, httpd_is_demo_mode());
    g_config.demo_mode = false;
}

/* ================================================================
 * httpd_get_authenticated_user — auth-disabled path
 * ================================================================ */

void test_get_authenticated_user_auth_disabled_returns_admin(void) {
    g_config.web_auth_enabled = false;

    http_request_t req;
    http_request_init(&req);

    user_t user;
    memset(&user, 0, sizeof(user));
    int rc = httpd_get_authenticated_user(&req, &user);
    TEST_ASSERT_EQUAL_INT(1, rc);
    TEST_ASSERT_EQUAL_STRING("admin", user.username);
    TEST_ASSERT_EQUAL_INT(USER_ROLE_ADMIN, user.role);
    TEST_ASSERT_TRUE(user.is_active);
}

void test_get_authenticated_user_null_params_returns_zero(void) {
    http_request_t req;
    http_request_init(&req);
    int rc = httpd_get_authenticated_user(&req, NULL);
    TEST_ASSERT_EQUAL_INT(0, rc);
}

/* ================================================================
 * httpd_check_admin_privileges — auth-disabled path
 * ================================================================ */

void test_check_admin_privileges_auth_disabled_returns_one(void) {
    g_config.web_auth_enabled = false;

    http_request_t req;
    http_request_init(&req);
    http_response_t res;
    http_response_init(&res);

    int rc = httpd_check_admin_privileges(&req, &res);
    TEST_ASSERT_EQUAL_INT(1, rc);

    http_response_free(&res);
}

void test_check_admin_privileges_no_auth_returns_zero(void) {
    /* auth enabled, no credentials → 0 and 401 response */
    g_config.web_auth_enabled = true;

    http_request_t req;
    http_request_init(&req);
    http_response_t res;
    http_response_init(&res);

    int rc = httpd_check_admin_privileges(&req, &res);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_INT(401, res.status_code);

    http_response_free(&res);
}

/* ================================================================
 * main
 * ================================================================ */

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_parse_json_valid_body);
    RUN_TEST(test_parse_json_invalid_body_returns_null);
    RUN_TEST(test_parse_json_null_request_returns_null);
    RUN_TEST(test_parse_json_empty_body_returns_null);
    RUN_TEST(test_basic_auth_valid_credentials);
    RUN_TEST(test_basic_auth_no_header_returns_error);
    RUN_TEST(test_basic_auth_wrong_scheme_returns_error);
    RUN_TEST(test_basic_auth_null_params_returns_error);
    RUN_TEST(test_get_session_token_valid_cookie);
    RUN_TEST(test_get_session_token_cookie_with_other_fields);
    RUN_TEST(test_get_session_token_no_cookie_header_returns_error);
    RUN_TEST(test_get_session_token_no_session_key_returns_error);
    RUN_TEST(test_is_demo_mode_false_by_default);
    RUN_TEST(test_is_demo_mode_true_when_set);
    RUN_TEST(test_get_authenticated_user_auth_disabled_returns_admin);
    RUN_TEST(test_get_authenticated_user_null_params_returns_zero);
    RUN_TEST(test_check_admin_privileges_auth_disabled_returns_one);
    RUN_TEST(test_check_admin_privileges_no_auth_returns_zero);
    return UNITY_END();
}

