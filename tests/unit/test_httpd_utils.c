/**
 * @file test_httpd_utils.c
 * @brief Layer 2 Unity tests for web/httpd_utils.c
 *
 * Tests:
 *   httpd_parse_json_body()             - JSON body parsing
 *   httpd_get_basic_auth_credentials()  - Base64 decode of Authorization header
 *   httpd_get_session_token()           - Cookie session= extraction
 *   httpd_get_cookie_value()            - Generic cookie extraction
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
#include <unistd.h>

#include <sqlite3.h>

#include <cjson/cJSON.h>

#include "unity.h"
#include "web/httpd_utils.h"
#include "web/request_response.h"
#include "core/config.h"
#include "database/db_auth.h"
#include "database/db_core.h"

/* ---- external globals from lightnvr_lib ---- */
extern config_t g_config;

#define TEST_DB_PATH "/tmp/lightnvr_test_httpd_utils.db"

/* ---- header helper ---- */
static void add_header(http_request_t *req, const char *name, const char *value) {
    if (req->num_headers >= MAX_HEADERS) return;
    strncpy(req->headers[req->num_headers].name,  name,  127);
    req->headers[req->num_headers].name[127]  = '\0';
    strncpy(req->headers[req->num_headers].value, value, 1023);
    req->headers[req->num_headers].value[1023] = '\0';
    req->num_headers++;
}

static const char *find_response_header(const http_response_t *res, const char *name) {
    if (!res || !name) return NULL;
    for (int i = 0; i < res->num_headers; i++) {
        if (strcmp(res->headers[i].name, name) == 0) {
            return res->headers[i].value;
        }
    }
    return NULL;
}

static void clear_auth_data(void) {
    sqlite3 *db = get_db_handle();
    TEST_ASSERT_NOT_NULL(db);

    char *errmsg = NULL;
    int rc = sqlite3_exec(db,
        "DELETE FROM trusted_devices;"
        "DELETE FROM sessions;"
        "DELETE FROM users WHERE username != 'admin';",
        NULL, NULL, &errmsg);
    if (rc != SQLITE_OK) {
        if (errmsg) {
            fprintf(stderr, "clear_auth_data failed: %s\n", errmsg);
            sqlite3_free(errmsg);
        }
        TEST_FAIL_MESSAGE("Failed to clear auth data");
    }
}

/* ---- Unity boilerplate ---- */
void setUp(void) {
    /* Ensure auth is enabled by default so we control path in each test */
    g_config.web_auth_enabled = true;
    g_config.demo_mode = false;
    g_config.trusted_proxy_cidrs[0] = '\0';
    clear_auth_data();
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

void test_get_api_key_prefers_x_api_key_header(void) {
    http_request_t req;
    http_request_init(&req);
    add_header(&req, "X-API-Key", "  key-from-header  ");
    add_header(&req, "Authorization", "Bearer ignored-token");

    char api_key[64] = {0};
    int rc = httpd_get_api_key(&req, api_key, sizeof(api_key));
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("key-from-header", api_key);
}

void test_get_api_key_supports_bearer_token(void) {
    http_request_t req;
    http_request_init(&req);
    add_header(&req, "Authorization", "Bearer token-123");

    char api_key[64] = {0};
    int rc = httpd_get_api_key(&req, api_key, sizeof(api_key));
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("token-123", api_key);
}

void test_get_api_key_returns_error_when_missing(void) {
    http_request_t req;
    http_request_init(&req);

    char api_key[64] = {0};
    int rc = httpd_get_api_key(&req, api_key, sizeof(api_key));
    TEST_ASSERT_EQUAL_INT(-1, rc);
}

void test_get_effective_client_ip_uses_peer_by_default(void) {
    g_config.trusted_proxy_cidrs[0] = '\0';

    http_request_t req;
    http_request_init(&req);
    strncpy(req.client_ip, "198.51.100.10", sizeof(req.client_ip) - 1);
    add_header(&req, "X-Forwarded-For", "192.0.2.25");

    char client_ip[64] = {0};
    int rc = httpd_get_effective_client_ip(&req, client_ip, sizeof(client_ip));
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("198.51.100.10", client_ip);
}

void test_get_effective_client_ip_uses_forwarded_for_from_trusted_proxy(void) {
    strncpy(g_config.trusted_proxy_cidrs, "127.0.0.1/32", sizeof(g_config.trusted_proxy_cidrs) - 1);

    http_request_t req;
    http_request_init(&req);
    strncpy(req.client_ip, "127.0.0.1", sizeof(req.client_ip) - 1);
    add_header(&req, "X-Forwarded-For", "198.51.100.44, 127.0.0.1");

    char client_ip[64] = {0};
    int rc = httpd_get_effective_client_ip(&req, client_ip, sizeof(client_ip));
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("198.51.100.44", client_ip);
}

void test_get_effective_client_ip_uses_forwarded_for_from_comma_separated_trusted_proxies(void) {
    strncpy(g_config.trusted_proxy_cidrs, "10.0.0.0/8, 127.0.0.1/32",
            sizeof(g_config.trusted_proxy_cidrs) - 1);

    http_request_t req;
    http_request_init(&req);
    strncpy(req.client_ip, "127.0.0.1", sizeof(req.client_ip) - 1);
    add_header(&req, "X-Forwarded-For", "198.51.100.44, 127.0.0.1");

    char client_ip[64] = {0};
    int rc = httpd_get_effective_client_ip(&req, client_ip, sizeof(client_ip));
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("198.51.100.44", client_ip);
}

void test_get_effective_client_ip_ignores_forwarded_for_from_untrusted_proxy(void) {
    strncpy(g_config.trusted_proxy_cidrs, "127.0.0.1/32", sizeof(g_config.trusted_proxy_cidrs) - 1);

    http_request_t req;
    http_request_init(&req);
    strncpy(req.client_ip, "198.51.100.10", sizeof(req.client_ip) - 1);
    add_header(&req, "X-Forwarded-For", "192.0.2.25");

    char client_ip[64] = {0};
    int rc = httpd_get_effective_client_ip(&req, client_ip, sizeof(client_ip));
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("198.51.100.10", client_ip);
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

void test_get_cookie_value_extracts_named_cookie(void) {
    http_request_t req;
    http_request_init(&req);
    add_header(&req, "Cookie", "lang=en; trusted_device=trust123; theme=dark");

    char value[64] = {0};
    int rc = httpd_get_cookie_value(&req, "trusted_device", value, sizeof(value));
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("trust123", value);
}

void test_get_cookie_value_returns_error_for_missing_cookie(void) {
    http_request_t req;
    http_request_init(&req);
    add_header(&req, "Cookie", "lang=en; theme=dark");

    char value[64] = {0};
    int rc = httpd_get_cookie_value(&req, "trusted_device", value, sizeof(value));
    TEST_ASSERT_EQUAL_INT(-1, rc);
}

void test_auth_cookie_helpers_add_and_clear_expected_headers(void) {
    http_response_t res;
    http_response_init(&res);
    g_config.auth_absolute_timeout_hours = 48;

    httpd_add_session_cookie(&res, "session-token");
    httpd_clear_session_cookie(&res);

    TEST_ASSERT_EQUAL_INT(2, res.num_headers);
    TEST_ASSERT_EQUAL_STRING("Set-Cookie", res.headers[0].name);
    TEST_ASSERT_NOT_NULL(strstr(res.headers[0].value, "session=session-token"));
    TEST_ASSERT_NOT_NULL(strstr(res.headers[0].value, "Max-Age=172800"));
    TEST_ASSERT_EQUAL_STRING("Set-Cookie", res.headers[1].name);
    TEST_ASSERT_EQUAL_STRING("session=; Path=/; Max-Age=0; HttpOnly; SameSite=Lax", res.headers[1].value);
}

void test_trusted_device_cookie_helpers_honor_lifetime_setting(void) {
    http_response_t res;
    http_response_init(&res);
    g_config.trusted_device_days = 7;

    httpd_add_trusted_device_cookie(&res, "trusted-token");
    TEST_ASSERT_EQUAL_INT(1, res.num_headers);
    TEST_ASSERT_NOT_NULL(strstr(res.headers[0].value, "trusted_device=trusted-token"));
    TEST_ASSERT_NOT_NULL(strstr(res.headers[0].value, "Max-Age=604800"));

    httpd_clear_trusted_device_cookie(&res);
    TEST_ASSERT_EQUAL_INT(2, res.num_headers);
    TEST_ASSERT_EQUAL_STRING("trusted_device=; Path=/; Max-Age=0; HttpOnly; SameSite=Lax", res.headers[1].value);

    http_response_t disabled_res;
    http_response_init(&disabled_res);
    g_config.trusted_device_days = 0;
    httpd_add_trusted_device_cookie(&disabled_res, "skipped-token");
    TEST_ASSERT_NULL(find_response_header(&disabled_res, "Set-Cookie"));
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

void test_get_authenticated_user_allows_basic_auth_from_allowed_ip(void) {
    int64_t uid = 0;
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_user("test", "password123", NULL, USER_ROLE_USER, true, &uid));
    TEST_ASSERT_EQUAL_INT(0, db_auth_set_allowed_login_cidrs(uid, "192.0.2.0/24"));

    http_request_t req;
    http_request_init(&req);
    add_header(&req, "Authorization", "Basic dGVzdDpwYXNzd29yZDEyMw==");
    strncpy(req.client_ip, "192.0.2.25", sizeof(req.client_ip) - 1);

    user_t user;
    memset(&user, 0, sizeof(user));
    int rc = httpd_get_authenticated_user(&req, &user);
    TEST_ASSERT_EQUAL_INT(1, rc);
    TEST_ASSERT_EQUAL_STRING("test", user.username);
}

void test_get_authenticated_user_rejects_basic_auth_from_disallowed_ip(void) {
    int64_t uid = 0;
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_user("test", "password123", NULL, USER_ROLE_USER, true, &uid));
    TEST_ASSERT_EQUAL_INT(0, db_auth_set_allowed_login_cidrs(uid, "192.0.2.0/24"));

    http_request_t req;
    http_request_init(&req);
    add_header(&req, "Authorization", "Basic dGVzdDpwYXNzd29yZDEyMw==");
    strncpy(req.client_ip, "198.51.100.25", sizeof(req.client_ip) - 1);

    user_t user;
    memset(&user, 0, sizeof(user));
    int rc = httpd_get_authenticated_user(&req, &user);
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_get_authenticated_user_rejects_session_from_disallowed_ip(void) {
    int64_t uid = 0;
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_user("sessip", "password123", NULL, USER_ROLE_USER, true, &uid));
    TEST_ASSERT_EQUAL_INT(0, db_auth_set_allowed_login_cidrs(uid, "192.0.2.0/24"));

    char token[128] = {0};
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_session(uid, "192.0.2.10", "TestAgent", 3600, token, sizeof(token)));

    http_request_t req;
    http_request_init(&req);
    char cookie[160] = {0};
    snprintf(cookie, sizeof(cookie), "session=%s", token);
    add_header(&req, "Cookie", cookie);
    strncpy(req.client_ip, "198.51.100.10", sizeof(req.client_ip) - 1);
    strncpy(req.user_agent, "TestAgent", sizeof(req.user_agent) - 1);

    user_t user;
    memset(&user, 0, sizeof(user));
    int rc = httpd_get_authenticated_user(&req, &user);
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_get_authenticated_user_allows_api_key_from_trusted_forwarded_ip(void) {
    int64_t uid = 0;
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_user("apitest", "password123", NULL, USER_ROLE_API, true, &uid));
    TEST_ASSERT_EQUAL_INT(0, db_auth_set_allowed_login_cidrs(uid, "192.0.2.0/24"));

    char api_key[64] = {0};
    TEST_ASSERT_EQUAL_INT(0, db_auth_generate_api_key(uid, api_key, sizeof(api_key)));
    strncpy(g_config.trusted_proxy_cidrs, "127.0.0.1/32", sizeof(g_config.trusted_proxy_cidrs) - 1);

    http_request_t req;
    http_request_init(&req);
    add_header(&req, "X-API-Key", api_key);
    add_header(&req, "X-Forwarded-For", "192.0.2.55, 127.0.0.1");
    strncpy(req.client_ip, "127.0.0.1", sizeof(req.client_ip) - 1);

    user_t user;
    memset(&user, 0, sizeof(user));
    int rc = httpd_get_authenticated_user(&req, &user);
    TEST_ASSERT_EQUAL_INT(1, rc);
    TEST_ASSERT_EQUAL_STRING("apitest", user.username);
}

void test_get_authenticated_user_rejects_api_key_with_spoofed_forwarded_ip_from_untrusted_proxy(void) {
    int64_t uid = 0;
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_user("apireject", "password123", NULL, USER_ROLE_API, true, &uid));
    TEST_ASSERT_EQUAL_INT(0, db_auth_set_allowed_login_cidrs(uid, "192.0.2.0/24"));

    char api_key[64] = {0};
    char auth_header[96] = {0};
    TEST_ASSERT_EQUAL_INT(0, db_auth_generate_api_key(uid, api_key, sizeof(api_key)));
    snprintf(auth_header, sizeof(auth_header), "Bearer %s", api_key);
    strncpy(g_config.trusted_proxy_cidrs, "127.0.0.1/32", sizeof(g_config.trusted_proxy_cidrs) - 1);

    http_request_t req;
    http_request_init(&req);
    add_header(&req, "Authorization", auth_header);
    add_header(&req, "X-Forwarded-For", "192.0.2.55");
    strncpy(req.client_ip, "198.51.100.10", sizeof(req.client_ip) - 1);

    user_t user;
    memset(&user, 0, sizeof(user));
    int rc = httpd_get_authenticated_user(&req, &user);
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
    unlink(TEST_DB_PATH);
    TEST_ASSERT_EQUAL_INT(0, init_database(TEST_DB_PATH));
    TEST_ASSERT_EQUAL_INT(0, db_auth_init());

    UNITY_BEGIN();
    RUN_TEST(test_parse_json_valid_body);
    RUN_TEST(test_parse_json_invalid_body_returns_null);
    RUN_TEST(test_parse_json_null_request_returns_null);
    RUN_TEST(test_parse_json_empty_body_returns_null);
    RUN_TEST(test_basic_auth_valid_credentials);
    RUN_TEST(test_basic_auth_no_header_returns_error);
    RUN_TEST(test_basic_auth_wrong_scheme_returns_error);
    RUN_TEST(test_basic_auth_null_params_returns_error);
    RUN_TEST(test_get_api_key_prefers_x_api_key_header);
    RUN_TEST(test_get_api_key_supports_bearer_token);
    RUN_TEST(test_get_api_key_returns_error_when_missing);
    RUN_TEST(test_get_effective_client_ip_uses_peer_by_default);
    RUN_TEST(test_get_effective_client_ip_uses_forwarded_for_from_trusted_proxy);
    RUN_TEST(test_get_effective_client_ip_uses_forwarded_for_from_comma_separated_trusted_proxies);
    RUN_TEST(test_get_effective_client_ip_ignores_forwarded_for_from_untrusted_proxy);
    RUN_TEST(test_get_session_token_valid_cookie);
    RUN_TEST(test_get_session_token_cookie_with_other_fields);
    RUN_TEST(test_get_session_token_no_cookie_header_returns_error);
    RUN_TEST(test_get_session_token_no_session_key_returns_error);
    RUN_TEST(test_get_cookie_value_extracts_named_cookie);
    RUN_TEST(test_get_cookie_value_returns_error_for_missing_cookie);
    RUN_TEST(test_auth_cookie_helpers_add_and_clear_expected_headers);
    RUN_TEST(test_trusted_device_cookie_helpers_honor_lifetime_setting);
    RUN_TEST(test_is_demo_mode_false_by_default);
    RUN_TEST(test_is_demo_mode_true_when_set);
    RUN_TEST(test_get_authenticated_user_auth_disabled_returns_admin);
    RUN_TEST(test_get_authenticated_user_null_params_returns_zero);
    RUN_TEST(test_get_authenticated_user_allows_basic_auth_from_allowed_ip);
    RUN_TEST(test_get_authenticated_user_rejects_basic_auth_from_disallowed_ip);
    RUN_TEST(test_get_authenticated_user_rejects_session_from_disallowed_ip);
    RUN_TEST(test_get_authenticated_user_allows_api_key_from_trusted_forwarded_ip);
    RUN_TEST(test_get_authenticated_user_rejects_api_key_with_spoofed_forwarded_ip_from_untrusted_proxy);
    RUN_TEST(test_check_admin_privileges_auth_disabled_returns_one);
    RUN_TEST(test_check_admin_privileges_no_auth_returns_zero);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}

