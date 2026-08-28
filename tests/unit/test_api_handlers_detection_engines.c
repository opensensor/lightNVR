#define _POSIX_C_SOURCE 200809L

#include "unity.h"

#include <cjson/cJSON.h>
#include <sqlite3.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "core/config.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#include "utils/strings.h"
#include "web/api_handlers_detection_engines.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_api_detection_engines.db"

void setUp(void) {
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
    sqlite3_exec(get_db_handle(), "DELETE FROM streams;", NULL, NULL, NULL);
}
void tearDown(void) {}

static cJSON *call(void (*handler)(const http_request_t *, http_response_t *),
                   http_method_t method, const char *body, int status) {
    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    request.method = method;
    safe_strcpy(request.method_str, method == HTTP_METHOD_GET ? "GET" : "PUT",
                sizeof(request.method_str), 0);
    safe_strcpy(request.path, "/api/streams/Gate/detection-engines",
                sizeof(request.path), 0);
    safe_strcpy(request.uri, request.path, sizeof(request.uri), 0);
    safe_strcpy(request.client_ip, "127.0.0.1", sizeof(request.client_ip), 0);
    if (body) {
        request.body = (void *)body;
        request.body_len = strlen(body);
    }
    handler(&request, &response);
    TEST_ASSERT_EQUAL_INT(status, response.status_code);
    cJSON *json = cJSON_Parse((const char *)response.body);
    TEST_ASSERT_NOT_NULL(json);
    http_response_free(&response);
    return json;
}

static void test_get_and_replace_motion_plus_object_configuration(void) {
    stream_config_t stream;
    memset(&stream, 0, sizeof(stream));
    safe_strcpy(stream.name, "Gate", sizeof(stream.name), 0);
    safe_strcpy(stream.url, "rtsp://camera/live", sizeof(stream.url), 0);
    safe_strcpy(stream.codec, "h264", sizeof(stream.codec), 0);
    safe_strcpy(stream.detection_model, "object.tflite",
                sizeof(stream.detection_model), 0);
    stream.detection_threshold = 0.7f;
    stream.detection_interval = 5;
    stream.enabled = true;
    stream.streaming_enabled = true;
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&stream));

    const char *body =
        "{\"engines\":[{\"key\":\"motion-fast\",\"type\":\"motion\","
        "\"model_path\":\"motion\",\"enabled\":true,\"threshold\":0.2,"
        "\"interval_seconds\":1,\"sort_order\":-10,\"config\":{}}]}";
    cJSON *json = call(handle_put_detection_engines, HTTP_METHOD_PUT, body, 200);
    TEST_ASSERT_EQUAL_INT(2, cJSON_GetObjectItemCaseSensitive(json, "count")->valueint);
    TEST_ASSERT_TRUE(cJSON_IsTrue(
        cJSON_GetObjectItemCaseSensitive(json, "restart_required")));
    cJSON_Delete(json);

    json = call(handle_get_detection_engines, HTTP_METHOD_GET, NULL, 200);
    TEST_ASSERT_EQUAL_INT(2, cJSON_GetObjectItemCaseSensitive(json, "count")->valueint);
    TEST_ASSERT_EQUAL_STRING("any_of",
        cJSON_GetObjectItemCaseSensitive(json, "trigger_policy")->valuestring);
    cJSON_Delete(json);
}

int main(void) {
    unlink(TEST_DB_PATH);
    setenv("LIGHTNVR_MIGRATIONS_DIR", "./db/migrations", 1);
    if (init_database(TEST_DB_PATH) != 0) return 1;
    UNITY_BEGIN();
    RUN_TEST(test_get_and_replace_motion_plus_object_configuration);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
