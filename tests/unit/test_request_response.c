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
#include <stdio.h>
#include "unity.h"
#include "web/request_response.h"
#include "web/libuv_connection.h"
#include "core/logger.h"
#include "utils/strings.h"

/* ---- Unity boilerplate ---- */
static bool fail_next_allocation;
void *__real_malloc(size_t size);
void *__wrap_malloc(size_t size) {
    if (fail_next_allocation) { fail_next_allocation = false; return NULL; }
    return __real_malloc(size);
}

void setUp(void)    { fail_next_allocation = false; }
void tearDown(void) { fail_next_allocation = false; }

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

void test_url_decode_requires_two_hex_digits(void) {
    const struct { const char *encoded; const char *expected; } cases[] = {
        {"101%0Gjunk", "101%0Gjunk"}, {"101%0xjunk", "101%0xjunk"},
        {"101%-0junk", "101%-0junk"}, {"101%+0junk", "101% 0junk"},
        {"101% 0junk", "101% 0junk"}, {"101%1Gjunk", "101%1Gjunk"},
        {"101%GGjunk", "101%GGjunk"}, {"101%0", "101%0"}, {"101%", "101%"},
        {"101%2500", "101%00"},
    };
    for (size_t i = 0; i < sizeof(cases) / sizeof(cases[0]); i++) {
        char out[64];
        TEST_ASSERT_EQUAL_INT(0, url_decode(cases[i].encoded, out, sizeof(out)));
        TEST_ASSERT_EQUAL_STRING_MESSAGE(cases[i].expected, out, cases[i].encoded);
    }
}

void test_url_decode_preserves_every_nonzero_encoded_byte(void) {
    for (unsigned int byte = 1; byte <= 255; byte++) {
        for (int uppercase = 0; uppercase < 2; uppercase++) {
            char encoded[16], out[16];
            snprintf(encoded, sizeof(encoded), uppercase ? "%%%02Xtail" : "%%%02xtail", byte);
            memset(out, 0x7f, sizeof(out));
            TEST_ASSERT_EQUAL_INT(0, url_decode(encoded, out, sizeof(out)));
            TEST_ASSERT_EQUAL_UINT8(byte, (unsigned char)out[0]);
            TEST_ASSERT_EQUAL_MEMORY("tail", out + 1, 5);
        }
    }
}

void test_query_decode_uses_full_input_and_bounds_output(void) {
    http_request_t req;
    http_request_init(&req);
    safe_strcpy(req.query_string, "value=%41%42%43%44&next=ignored", sizeof(req.query_string), 0);
    char out[4];
    TEST_ASSERT_EQUAL_INT(3, http_request_get_query_param(&req, "value", out, sizeof(out)));
    TEST_ASSERT_EQUAL_STRING("ABC", out);
    char one[1] = {'x'};
    TEST_ASSERT_EQUAL_INT(0, http_request_get_query_param(&req, "value", one, sizeof(one)));
    TEST_ASSERT_EQUAL_CHAR('\0', one[0]);
    char larger[32];
    TEST_ASSERT_EQUAL_INT(4, http_request_get_query_param(&req, "value", larger, sizeof(larger)));
    TEST_ASSERT_EQUAL_STRING("ABCD", larger);
    safe_strcpy(req.query_string, "value=%4&next=1", sizeof(req.query_string), 0);
    TEST_ASSERT_EQUAL_INT(2, http_request_get_query_param(&req, "value", larger, sizeof(larger)));
    TEST_ASSERT_EQUAL_STRING("%4", larger);
}

void test_query_decode_near_request_capacity_needs_no_allocation(void) {
    http_request_t req;
    http_request_init(&req);
    memcpy(req.query_string, "value=", 6);
    for (int i = 0; i < 330; i++) memcpy(req.query_string + 6 + i * 3, "%41", 3);
    memcpy(req.query_string + 996, "&next=ignored", 14);
    char out[331];
    fail_next_allocation = true;
    int length = http_request_get_query_param(&req, "value", out, sizeof(out));
    bool allocation_unused = fail_next_allocation;
    fail_next_allocation = false;
    TEST_ASSERT_EQUAL_INT(330, length);
    TEST_ASSERT_TRUE(allocation_unused);
    for (int i = 0; i < 330; i++) TEST_ASSERT_EQUAL_CHAR('A', out[i]);
    TEST_ASSERT_EQUAL_CHAR('\0', out[330]);
}

void test_path_decode_does_not_consume_query_or_trimmed_slash(void) {
    http_request_t req;
    http_request_init(&req);
    char out[32];
    safe_strcpy(req.path, "/api/streams/%34%32/?note=ignored", sizeof(req.path), 0);
    TEST_ASSERT_EQUAL_INT(0, http_request_extract_path_param(&req, "/api/streams/", out, sizeof(out)));
    TEST_ASSERT_EQUAL_STRING("42", out);
    safe_strcpy(req.path, "/api/streams/%4?2=ignored", sizeof(req.path), 0);
    TEST_ASSERT_EQUAL_INT(0, http_request_extract_path_param(&req, "/api/streams/", out, sizeof(out)));
    TEST_ASSERT_EQUAL_STRING("%4", out);
}

void test_response_body_replacement_supports_aliases_and_allocation_failure(void) {
    http_response_t res;
    http_response_init(&res);
    TEST_ASSERT_EQUAL_INT(0, http_response_set_body(&res, "prefix payload"));
    TEST_ASSERT_EQUAL_INT(0, http_response_set_body(&res, res.body));
    TEST_ASSERT_EQUAL_STRING("prefix payload", res.body);
    TEST_ASSERT_EQUAL_INT(0, http_response_set_body(&res, (char *)res.body + 7));
    TEST_ASSERT_EQUAL_STRING("payload", res.body);
    void *previous = res.body;
    size_t previous_length = res.body_length;
    fail_next_allocation = true;
    int result = http_response_set_body(&res, "replacement");
    fail_next_allocation = false;
    TEST_ASSERT_EQUAL_INT(-1, result);
    TEST_ASSERT_EQUAL_PTR(previous, res.body);
    TEST_ASSERT_EQUAL_UINT(previous_length, res.body_length);
    TEST_ASSERT_TRUE(res.body_allocated);
    TEST_ASSERT_EQUAL_STRING("payload", res.body);
    http_response_free(&res);
    TEST_ASSERT_NULL(res.body);
    http_response_free(&res);
}


void test_decoders_reject_encoded_nul_and_clear_partial_output(void) {
    const char *values[] = {"%00", "42%00junk", "42%00", "long-prefix%00tail"};
    http_request_t req;
    http_request_init(&req);
    for (size_t i = 0; i < sizeof(values) / sizeof(values[0]); i++) {
        for (size_t capacity = 1; capacity <= 32; capacity++) {
            char out[32];
            memset(out, 'x', sizeof(out));
            TEST_ASSERT_EQUAL_INT(-1, url_decode(values[i], out, capacity));
            TEST_ASSERT_EQUAL_CHAR('\0', out[0]);
            snprintf(req.query_string, sizeof(req.query_string), "value=%s&next=ok", values[i]);
            memset(out, 'x', sizeof(out));
            TEST_ASSERT_EQUAL_INT(-1, http_request_get_query_param(&req, "value", out, capacity));
            TEST_ASSERT_EQUAL_CHAR('\0', out[0]);
            snprintf(req.path, sizeof(req.path), "/api/streams/%s", values[i]);
            memset(out, 'x', sizeof(out));
            TEST_ASSERT_EQUAL_INT(-1, http_request_extract_path_param(&req, "/api/streams/", out, capacity));
            TEST_ASSERT_EQUAL_CHAR('\0', out[0]);
        }
    }
    char out[32];
    safe_strcpy(req.query_string, "value=ok&next=%00", sizeof(req.query_string), 0);
    TEST_ASSERT_EQUAL_INT(2, http_request_get_query_param(&req, "value", out, sizeof(out)));
    TEST_ASSERT_EQUAL_STRING("ok", out);
    safe_strcpy(req.path, "/api/streams/42/?next=%00", sizeof(req.path), 0);
    TEST_ASSERT_EQUAL_INT(0, http_request_extract_path_param(&req, "/api/streams/", out, sizeof(out)));
    TEST_ASSERT_EQUAL_STRING("42", out);
}

void test_path_decode_accepts_exact_fit_and_rejects_truncation(void) {
    const char *values[] = {"ABC", "%41%42%43"};
    http_request_t req;
    http_request_init(&req);
    for (size_t i = 0; i < sizeof(values) / sizeof(values[0]); i++) {
        snprintf(req.path, sizeof(req.path), "/api/streams/%s", values[i]);
        for (size_t capacity = 1; capacity <= 5; capacity++) {
            char out[5];
            memset(out, 'x', sizeof(out));
            int result = http_request_extract_path_param(&req, "/api/streams/", out, capacity);
            TEST_ASSERT_EQUAL_INT(capacity >= 4 ? 0 : -1, result);
            TEST_ASSERT_EQUAL_STRING(capacity >= 4 ? "ABC" : "", out);
        }
    }
}

void test_json_failure_preserves_complete_response(void) {
    http_response_t res;
    http_response_init(&res);
    res.status_code = 202;
    safe_strcpy(res.content_type, "text/plain", sizeof(res.content_type), 0);
    TEST_ASSERT_EQUAL_INT(0, http_response_add_header(&res, "X-Keep", "yes"));
    TEST_ASSERT_EQUAL_INT(0, http_response_set_body(&res, "previous"));
    http_response_t before = res;
    fail_next_allocation = true;
    int result = http_response_set_json(&res, 500, "{}");
    fail_next_allocation = false;
    TEST_ASSERT_EQUAL_INT(-1, result);
    TEST_ASSERT_EQUAL_MEMORY(&before, &res, sizeof(res));
    TEST_ASSERT_EQUAL_STRING("previous", res.body);
    const int invalid_counts[] = {-1, MAX_HEADERS - 7, MAX_HEADERS, MAX_HEADERS + 1};
    for (size_t i = 0; i < sizeof(invalid_counts) / sizeof(invalid_counts[0]); i++) {
        res.num_headers = invalid_counts[i];
        before = res;
        TEST_ASSERT_EQUAL_INT(-1, http_response_set_json(&res, 500, "{}"));
        TEST_ASSERT_EQUAL_MEMORY(&before, &res, sizeof(res));
        TEST_ASSERT_EQUAL_STRING("previous", res.body);
    }
    http_response_free(&res);
}

void test_json_exact_header_capacity_and_aliased_body(void) {
    http_response_t res;
    http_response_init(&res);
    TEST_ASSERT_EQUAL_INT(0, http_response_set_body(&res, "prefix {}"));
    while (res.num_headers < MAX_HEADERS - 8) {
        TEST_ASSERT_EQUAL_INT(0, http_response_add_header(&res, "X-Keep", "yes"));
    }
    TEST_ASSERT_EQUAL_INT(0, http_response_set_json(&res, 201, (char *)res.body + 7));
    TEST_ASSERT_EQUAL_INT(MAX_HEADERS, res.num_headers);
    TEST_ASSERT_EQUAL_INT(201, res.status_code);
    TEST_ASSERT_EQUAL_STRING("application/json", res.content_type);
    TEST_ASSERT_EQUAL_STRING("{}", res.body);
    TEST_ASSERT_EQUAL_STRING("X-Keep", res.headers[0].name);
    TEST_ASSERT_EQUAL_STRING("Expires", res.headers[MAX_HEADERS - 1].name);
    http_response_free(&res);
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
 * request correlation IDs
 * ================================================================ */

void test_request_init_generates_unique_correlation_ids(void) {
    http_request_t first;
    http_request_t second;
    http_request_init(&first);
    http_request_init(&second);

    size_t first_length = strnlen(first.request_id, sizeof(first.request_id));
    size_t second_length = strnlen(second.request_id, sizeof(second.request_id));
    TEST_ASSERT_TRUE_MESSAGE(first_length > 0 && first_length < sizeof(first.request_id),
                             "First request ID must be nonempty and NUL-terminated");
    TEST_ASSERT_TRUE_MESSAGE(second_length > 0 && second_length < sizeof(second.request_id),
                             "Second request ID must be nonempty and NUL-terminated");
    TEST_ASSERT_NOT_EQUAL(0, strcmp(first.request_id, second.request_id));
}

void test_request_id_accepts_safe_ingress_value(void) {
    http_request_t req;
    http_request_init(&req);

    TEST_ASSERT_EQUAL_INT(
        0, http_request_set_request_id(&req, "edge-01:request_42.trace"));
    TEST_ASSERT_EQUAL_STRING("edge-01:request_42.trace", req.request_id);
}

void test_request_id_rejects_unsafe_value_without_replacing_generated_id(void) {
    http_request_t req;
    http_request_init(&req);
    char generated[REQUEST_ID_MAX];
    safe_strcpy(generated, req.request_id, sizeof(generated), 0);

    TEST_ASSERT_EQUAL_INT(
        -1, http_request_set_request_id(&req, "request\r\nInjected: yes"));
    TEST_ASSERT_EQUAL_STRING(generated, req.request_id);
    TEST_ASSERT_EQUAL_INT(-1, http_request_set_request_id(&req, ""));
    TEST_ASSERT_EQUAL_STRING(generated, req.request_id);
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
    bool wildcard = false, exposed = false;
    for (int i = 0; i < res.num_headers; i++) {
        TEST_ASSERT_NOT_EQUAL(0, strcmp("Access-Control-Allow-Credentials", res.headers[i].name));
        if (strcmp("Access-Control-Allow-Origin", res.headers[i].name) == 0) {
            TEST_ASSERT_EQUAL_STRING("*", res.headers[i].value);
            wildcard = true;
        }
        if (strcmp("Access-Control-Expose-Headers", res.headers[i].name) == 0) {
            TEST_ASSERT_NOT_NULL(strstr(res.headers[i].value, "X-Request-ID"));
            TEST_ASSERT_NOT_NULL(strstr(res.headers[i].value, "Content-Disposition"));
            exposed = true;
        }
    }
    TEST_ASSERT_TRUE(wildcard);
    TEST_ASSERT_TRUE(exposed);
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

    RUN_TEST(test_decoders_reject_encoded_nul_and_clear_partial_output);
    RUN_TEST(test_path_decode_accepts_exact_fit_and_rejects_truncation);
    RUN_TEST(test_json_failure_preserves_complete_response);
    RUN_TEST(test_json_exact_header_capacity_and_aliased_body);
    RUN_TEST(test_url_decode_percent_20);
    RUN_TEST(test_url_decode_percent_2F);
    RUN_TEST(test_url_decode_plus_as_space);
    RUN_TEST(test_url_decode_no_encoding);
    RUN_TEST(test_query_decode_uses_full_input_and_bounds_output);
    RUN_TEST(test_query_decode_near_request_capacity_needs_no_allocation);
    RUN_TEST(test_path_decode_does_not_consume_query_or_trimmed_slash);
    RUN_TEST(test_response_body_replacement_supports_aliases_and_allocation_failure);
    RUN_TEST(test_url_decode_requires_two_hex_digits);
    RUN_TEST(test_url_decode_preserves_every_nonzero_encoded_byte);

    RUN_TEST(test_get_header_found);
    RUN_TEST(test_get_header_case_insensitive);
    RUN_TEST(test_get_header_not_found);

    RUN_TEST(test_request_init_generates_unique_correlation_ids);
    RUN_TEST(test_request_id_accepts_safe_ingress_value);
    RUN_TEST(test_request_id_rejects_unsafe_value_without_replacing_generated_id);

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
