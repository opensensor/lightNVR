#define _POSIX_C_SOURCE 200809L

#include "unity.h"

#include <cjson/cJSON.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "core/config.h"
#include "database/db_core.h"
#include "database/db_lpr_reads.h"
#include "database/db_streams.h"
#include "utils/strings.h"
#include "web/api_handlers_lpr.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_api_lpr.db"

static const char *test_key =
    "000102030405060708090a0b0c0d0e0f"
    "101112131415161718191a1b1c1d1e1f";

static stream_config_t create_camera(void) {
    stream_config_t stream;
    memset(&stream, 0, sizeof(stream));
    safe_strcpy(stream.name, "Drive LPR", sizeof(stream.name), 0);
    safe_strcpy(stream.url, "rtsp://camera/live", sizeof(stream.url), 0);
    safe_strcpy(stream.codec, "h264", sizeof(stream.codec), 0);
    stream.enabled = true;
    stream.streaming_enabled = true;
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&stream));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name(stream.name, &stream));
    return stream;
}

static cJSON *call_handler(
    void (*handler)(const http_request_t *, http_response_t *),
    http_method_t method, const char *path, const char *body,
    int expected_status, bool expect_download) {
    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    request.method = method;
    safe_strcpy(request.method_str,
                method == HTTP_METHOD_DELETE ? "DELETE" : "POST",
                sizeof(request.method_str), 0);
    safe_strcpy(request.path, path, sizeof(request.path), 0);
    safe_strcpy(request.uri, request.path, sizeof(request.uri), 0);
    safe_strcpy(request.client_ip, "127.0.0.1", sizeof(request.client_ip), 0);
    request.body = (void *)body;
    request.body_len = body ? strlen(body) : 0;
    handler(&request, &response);
    TEST_ASSERT_EQUAL_INT(expected_status, response.status_code);
    bool has_download = false;
    for (int i = 0; i < response.num_headers; ++i) {
        if (strcmp(response.headers[i].name, "Content-Disposition") == 0) {
            has_download = true;
        }
    }
    TEST_ASSERT_EQUAL_INT(expect_download, has_download);
    cJSON *json = cJSON_Parse((const char *)response.body);
    TEST_ASSERT_NOT_NULL(json);
    http_response_free(&response);
    return json;
}

static cJSON *call_search(const char *body, int expected_status) {
    return call_handler(handle_post_lpr_search, HTTP_METHOD_POST,
                        "/api/lpr/search", body, expected_status, false);
}

void setUp(void) {
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
    setenv("LIGHTNVR_LPR_MASTER_KEY_HEX", test_key, 1);
    sqlite3_exec(get_db_handle(), "DELETE FROM audit_events;", NULL, NULL, NULL);
    sqlite3_exec(get_db_handle(), "DELETE FROM lpr_reads;", NULL, NULL, NULL);
    sqlite3_exec(get_db_handle(), "DELETE FROM streams;", NULL, NULL, NULL);
}
void tearDown(void) {}

static void test_search_returns_protected_read_and_audits_fingerprint_only(void) {
    stream_config_t camera = create_camera();
    lpr_read_input_t input;
    memset(&input, 0, sizeof(input));
    safe_strcpy(input.camera_uuid, camera.camera_uuid, sizeof(input.camera_uuid), 0);
    safe_strcpy(input.stream_name, camera.name, sizeof(input.stream_name), 0);
    input.observed_at_ms = 1787920496789LL;
    safe_strcpy(input.source, "onvif_profile_m", sizeof(input.source), 0);
    safe_strcpy(input.plate, "TEST123", sizeof(input.plate), 0);
    TEST_ASSERT_EQUAL_INT(0, db_lpr_read_insert(&input, NULL));

    char body[512];
    snprintf(body, sizeof(body),
             "{\"camera_uuid\":\"%s\",\"start_at\":1787920000000,"
             "\"end_at\":1787930000000,\"match\":\"exact\","
             "\"plate\":\"test-123\",\"limit\":10}", camera.camera_uuid);
    cJSON *json = call_search(body, 200);
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetObjectItemCaseSensitive(json, "count")->valueint);
    cJSON *item = cJSON_GetArrayItem(
        cJSON_GetObjectItemCaseSensitive(json, "reads"), 0);
    TEST_ASSERT_EQUAL_STRING("TEST123",
        cJSON_GetObjectItemCaseSensitive(item, "plate")->valuestring);
    cJSON_Delete(json);

    sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(
        get_db_handle(),
        "SELECT details_json FROM audit_events WHERE action='lpr.search' "
        "AND outcome='success' ORDER BY id DESC LIMIT 1;", -1, &stmt, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(stmt));
    const char *details = (const char *)sqlite3_column_text(stmt, 0);
    TEST_ASSERT_NOT_NULL(strstr(details, "query_fingerprint"));
    TEST_ASSERT_NULL(strstr(details, "TEST123"));
    TEST_ASSERT_NULL(strstr(details, "test-123"));
    sqlite3_finalize(stmt);
}

static void test_search_requires_time_scope_and_available_key(void) {
    stream_config_t camera = create_camera();
    char body[512];
    snprintf(body, sizeof(body),
             "{\"camera_uuid\":\"%s\",\"match\":\"partial\","
             "\"plate\":\"ABC\"}", camera.camera_uuid);
    cJSON *json = call_search(body, 400);
    cJSON_Delete(json);

    snprintf(body, sizeof(body),
             "{\"camera_uuid\":\"%s\",\"start_at\":1787920000000,"
             "\"end_at\":1787930000000}", camera.camera_uuid);
    unsetenv("LIGHTNVR_LPR_MASTER_KEY_HEX");
    json = call_search(body, 503);
    cJSON_Delete(json);
}

static void test_export_and_individual_delete_are_audited(void) {
    stream_config_t camera = create_camera();
    lpr_read_input_t input;
    memset(&input, 0, sizeof(input));
    safe_strcpy(input.camera_uuid, camera.camera_uuid, sizeof(input.camera_uuid), 0);
    safe_strcpy(input.stream_name, camera.name, sizeof(input.stream_name), 0);
    input.observed_at_ms = 1787920496789LL;
    safe_strcpy(input.source, "onvif_profile_m", sizeof(input.source), 0);
    safe_strcpy(input.plate, "EXPORT9", sizeof(input.plate), 0);
    char read_uuid[LPR_READ_UUID_SIZE];
    TEST_ASSERT_EQUAL_INT(0, db_lpr_read_insert(&input, read_uuid));

    char body[512];
    snprintf(body, sizeof(body),
             "{\"camera_uuid\":\"%s\",\"start_at\":1787920000000,"
             "\"end_at\":1787930000000,\"limit\":1000}", camera.camera_uuid);
    cJSON *json = call_handler(handle_post_lpr_export, HTTP_METHOD_POST,
                               "/api/lpr/export", body, 200, true);
    TEST_ASSERT_EQUAL_INT(1,
        cJSON_GetObjectItemCaseSensitive(json, "count")->valueint);
    cJSON_Delete(json);

    char path[128];
    snprintf(path, sizeof(path), "/api/lpr/reads/%s", read_uuid);
    json = call_handler(handle_delete_lpr_read, HTTP_METHOD_DELETE, path,
                        NULL, 200, false);
    TEST_ASSERT_TRUE(cJSON_IsTrue(
        cJSON_GetObjectItemCaseSensitive(json, "deleted")));
    cJSON_Delete(json);

    sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(
        get_db_handle(),
        "SELECT COUNT(*) FROM audit_events WHERE "
        "(action='lpr.export' OR action='lpr.delete') AND outcome='success';",
        -1, &stmt, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(stmt));
    TEST_ASSERT_EQUAL_INT(2, sqlite3_column_int(stmt, 0));
    sqlite3_finalize(stmt);
}

int main(void) {
    unlink(TEST_DB_PATH);
    setenv("LIGHTNVR_MIGRATIONS_DIR", "./db/migrations", 1);
    setenv("LIGHTNVR_LPR_MASTER_KEY_HEX", test_key, 1);
    if (init_database(TEST_DB_PATH) != 0) return 1;
    UNITY_BEGIN();
    RUN_TEST(test_search_returns_protected_read_and_audits_fingerprint_only);
    RUN_TEST(test_search_requires_time_scope_and_available_key);
    RUN_TEST(test_export_and_individual_delete_are_audited);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
