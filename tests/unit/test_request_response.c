/**
 * @file test_request_response.c
 * @brief Layer 2 unit tests — HTTP request/response helpers
 *
 * Tests url_decode, header/query-param extraction, path-param extraction,
 * response init/free, set_json, set_json_error, add_header, add_cors_headers.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <cjson/cJSON.h>
#include <string.h>
#include "unity.h"
#include "web/request_response.h"
#include "web/libuv_connection.h"
#include "core/logger.h"
#include "utils/strings.h"

/* ---- Unity boilerplate ---- */
void setUp(void)    {}
void tearDown(void) {}

/* ================================================================
 * url_decode
 * ================================================================ */

void test_url_decode_percent_20(void) {
    char out[64];
    TEST_ASSERT_EQUAL_INT(0, url_decode("hello%20world", out, sizeof(out)));
    TEST_ASSERT_EQUAL_STRING("hello world", out);
}

void test_url_decode_percent_2F(void) {
    char out[64];
    TEST_ASSERT_EQUAL_INT(0, url_decode("a%2Fb", out, sizeof(out)));
    TEST_ASSERT_EQUAL_STRING("a/b", out);
}

void test_url_decode_plus_as_space(void) {
    char out[64];
    TEST_ASSERT_EQUAL_INT(0, url_decode("hello+world", out, sizeof(out)));
    TEST_ASSERT_EQUAL_STRING("hello world", out);
}

void test_url_decode_no_encoding(void) {
    char out[64];
    TEST_ASSERT_EQUAL_INT(0, url_decode("plain", out, sizeof(out)));
    TEST_ASSERT_EQUAL_STRING("plain", out);
}

/* ================================================================
 * http_request_get_header
 * ================================================================ */

void test_get_header_found(void) {
    http_request_t req;
    http_request_init(&req);
    safe_strcpy(req.headers[0].name, "Content-Type", sizeof(req.headers[0].name), 0);
    safe_strcpy(req.headers[0].value, "application/json", sizeof(req.headers[0].value), 0);
    req.num_headers = 1;

    const char *v = http_request_get_header(&req, "Content-Type");
    TEST_ASSERT_NOT_NULL(v);
    TEST_ASSERT_EQUAL_STRING("application/json", v);
}

void test_get_header_case_insensitive(void) {
    http_request_t req;
    http_request_init(&req);
    safe_strcpy(req.headers[0].name, "content-type", sizeof(req.headers[0].name), 0);
    safe_strcpy(req.headers[0].value, "text/plain", sizeof(req.headers[0].value), 0);
    req.num_headers = 1;

    const char *v = http_request_get_header(&req, "CONTENT-TYPE");
    TEST_ASSERT_NOT_NULL(v);
    TEST_ASSERT_EQUAL_STRING("text/plain", v);
}

void test_get_header_not_found(void) {
    http_request_t req;
    http_request_init(&req);
    req.num_headers = 0;
    TEST_ASSERT_NULL(http_request_get_header(&req, "X-Missing"));
}

/* ================================================================
 * http_request_get_query_param
 * ================================================================ */

void test_get_query_param_found(void) {
    http_request_t req;
    http_request_init(&req);
    safe_strcpy(req.query_string, "page=2&limit=10", sizeof(req.query_string), 0);

    char val[32];
    int rc = http_request_get_query_param(&req, "page", val, sizeof(val));
    TEST_ASSERT_GREATER_OR_EQUAL(0, rc);
    TEST_ASSERT_EQUAL_STRING("2", val);
}

void test_get_query_param_not_found(void) {
    http_request_t req;
    http_request_init(&req);
    safe_strcpy(req.query_string, "a=1", sizeof(req.query_string), 0);

    char val[32];
    int rc = http_request_get_query_param(&req, "missing", val, sizeof(val));
    TEST_ASSERT_EQUAL_INT(-1, rc);
}

/* ================================================================
 * http_request_extract_path_param
 * ================================================================ */

void test_extract_path_param(void) {
    http_request_t req;
    http_request_init(&req);
    safe_strcpy(req.path, "/api/streams/42", sizeof(req.path), 0);

    char param[32];
    int rc = http_request_extract_path_param(&req, "/api/streams/", param, sizeof(param));
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("42", param);
}

void test_extract_path_param_encoded(void) {
    http_request_t req;
    http_request_init(&req);
    safe_strcpy(req.path, "/api/streams/four%20score%20and%202", sizeof(req.path), 0);

    char param[32];
    int rc = http_request_extract_path_param(&req, "/api/streams/", param, sizeof(param));
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("four score and 2", param);
}

void test_extract_path_param_with_query(void) {
    http_request_t req;
    http_request_init(&req);
    safe_strcpy(req.path, "/api/streams/42?test=yes", sizeof(req.path), 0);

    char param[32];
    int rc = http_request_extract_path_param(&req, "/api/streams/", param, sizeof(param));
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("42", param);
}

/* ================================================================
 * http_response helpers
 * ================================================================ */

void test_response_init_and_free(void) {
    http_response_t res;
    http_response_init(&res);
    /* http_response_init sets default status_code to 200 */
    TEST_ASSERT_EQUAL_INT(200, res.status_code);
    http_response_free(&res);
    TEST_PASS();
}

void test_response_set_json(void) {
    http_response_t res;
    http_response_init(&res);
    int rc = http_response_set_json(&res, 200, "{\"ok\":true}");
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_INT(200, res.status_code);
    TEST_ASSERT_NOT_NULL(res.body);
    http_response_free(&res);
}

void test_response_set_json_error(void) {
    http_response_t res;
    http_response_init(&res);
    const char *message = "not \"found\"\\here\r\nnext";
    int rc = http_response_set_json_error(&res, 404, message);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_INT(404, res.status_code);
    TEST_ASSERT_NOT_NULL(res.body);
    cJSON *body = cJSON_Parse(res.body);
    TEST_ASSERT_NOT_NULL(body);
    cJSON *error = cJSON_GetObjectItemCaseSensitive(body, "error");
    TEST_ASSERT_TRUE(cJSON_IsString(error));
    TEST_ASSERT_EQUAL_STRING(message, error->valuestring);
    cJSON_Delete(body);
    http_response_free(&res);
}

void test_response_add_header(void) {
    http_response_t res;
    http_response_init(&res);
    int rc = http_response_add_header(&res, "X-Custom", "value");
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_GREATER_THAN_INT(0, res.num_headers);
    http_response_free(&res);
}

void test_response_add_cors_headers(void) {
    http_response_t res;
    http_response_init(&res);
    http_response_add_cors_headers(&res);
    /* Expect at least one CORS header added */
    TEST_ASSERT_GREATER_THAN_INT(0, res.num_headers);
    http_response_free(&res);
}

void test_libuv_connection_reset_preserves_client_ip(void) {
    uv_loop_t loop;
    TEST_ASSERT_EQUAL_INT(0, uv_loop_init(&loop));

    libuv_server_t server;
    memset(&server, 0, sizeof(server));
    server.loop = &loop;

    libuv_connection_t *conn = libuv_connection_create(&server);
    TEST_ASSERT_NOT_NULL(conn);

    safe_strcpy(conn->request.client_ip, "192.0.2.55", sizeof(conn->request.client_ip), 0);
    safe_strcpy(conn->request.user_agent, "RegressionAgent/1.0", sizeof(conn->request.user_agent), 0);
    safe_strcpy(conn->request.path, "/api/auth/login", sizeof(conn->request.path), 0);

    libuv_connection_reset(conn);

    TEST_ASSERT_EQUAL_STRING("192.0.2.55", conn->request.client_ip);
    TEST_ASSERT_EQUAL_STRING("", conn->request.user_agent);
    TEST_ASSERT_EQUAL_STRING("", conn->request.path);
    TEST_ASSERT_EQUAL_INT(1, conn->requests_handled);

    libuv_connection_close(conn);
    uv_run(&loop, UV_RUN_DEFAULT);
    TEST_ASSERT_EQUAL_INT(0, uv_loop_close(&loop));
}

/* ================================================================
 * main
 * ================================================================ */

int main(void) {
    init_logger();

    UNITY_BEGIN();

    RUN_TEST(test_url_decode_percent_20);
    RUN_TEST(test_url_decode_percent_2F);
    RUN_TEST(test_url_decode_plus_as_space);
    RUN_TEST(test_url_decode_no_encoding);

    RUN_TEST(test_get_header_found);
    RUN_TEST(test_get_header_case_insensitive);
    RUN_TEST(test_get_header_not_found);

    RUN_TEST(test_get_query_param_found);
    RUN_TEST(test_get_query_param_not_found);

    RUN_TEST(test_extract_path_param);
    RUN_TEST(test_extract_path_param_encoded);
    RUN_TEST(test_extract_path_param_with_query);

    RUN_TEST(test_response_init_and_free);
    RUN_TEST(test_response_set_json);
    RUN_TEST(test_response_set_json_error);
    RUN_TEST(test_response_add_header);
    RUN_TEST(test_response_add_cors_headers);
    RUN_TEST(test_libuv_connection_reset_preserves_client_ip);

    int result = UNITY_END();
    shutdown_logger();
    return result;
}
