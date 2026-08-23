/**
 * @file test_api_handlers_event_routes.c
 * @brief Event catalog, route CRUD, preview, RBAC, and audit API tests.
 */

#define _POSIX_C_SOURCE 200809L

#include <cjson/cJSON.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "core/config.h"
#include "database/db_audit.h"
#include "database/db_auth.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#include "unity.h"
#include "utils/strings.h"
#include "web/api_handlers_event_routes.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_api_event_routes.db"
#define DETECTION_TYPE "io.lightnvr.detection.object.v1"

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
                   const char *body, const char *api_key,
                   int expected_status) {
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
    if (api_key) {
        safe_strcpy(request.headers[0].name, "X-API-Key",
                    sizeof(request.headers[0].name), 0);
        safe_strcpy(request.headers[0].value, api_key,
                    sizeof(request.headers[0].value), 0);
        request.num_headers = 1;
    }
    handler(&request, &response);
    TEST_ASSERT_EQUAL_INT(expected_status, response.status_code);
    cJSON *json = response.body
        ? cJSON_Parse((const char *)response.body) : NULL;
    TEST_ASSERT_NOT_NULL(json);
    http_response_free(&response);
    return json;
}

static stream_config_t create_camera(const char *name) {
    stream_config_t stream;
    memset(&stream, 0, sizeof(stream));
    safe_strcpy(stream.name, name, sizeof(stream.name), 0);
    safe_strcpy(stream.url, "rtsp://camera/live", sizeof(stream.url), 0);
    safe_strcpy(stream.codec, "h264", sizeof(stream.codec), 0);
    stream.enabled = true;
    stream.streaming_enabled = true;
    stream.record = true;
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&stream));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name(name, &stream));
    return stream;
}

static bool audit_has_operation(const char *operation) {
    audit_query_t query = {.page = 1, .page_size = 100};
    safe_strcpy(query.action, "events.configure", sizeof(query.action), 0);
    safe_strcpy(query.outcome, "success", sizeof(query.outcome), 0);
    audit_page_t page;
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    char needle[96];
    snprintf(needle, sizeof(needle), "\"operation\":\"%s\"", operation);
    bool found = false;
    for (int index = 0; index < page.count; index++) {
        if (strstr(page.events[index].details_json, needle)) {
            found = true;
            break;
        }
    }
    db_audit_page_free(&page);
    return found;
}

static void remove_user(const char *username) {
    user_t user;
    if (db_auth_get_user_by_username(username, &user) == 0) {
        db_auth_delete_user(user.id);
    }
}

void setUp(void) {
    sqlite3 *db = get_db_handle();
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
    sqlite3_exec(db, "DELETE FROM event_routes;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM streams;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM audit_events;", NULL, NULL, NULL);
    remove_user("routeviewer");
}

void tearDown(void) {
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
}

void test_catalog_and_route_crud_are_versioned_and_audited(void) {
    cJSON *json = call(handle_get_event_catalog, HTTP_METHOD_GET,
                       "/api/events/catalog", NULL, NULL, NULL, 200);
    TEST_ASSERT_EQUAL_INT(4,
        cJSON_GetObjectItemCaseSensitive(json, "count")->valueint);
    cJSON_Delete(json);

    const char *create_body =
        "{\"name\":\"Critical detections\","
        "\"description\":\"Send selected detections\","
        "\"event_types\":[\"" DETECTION_TYPE "\"],"
        "\"predicate\":{\"version\":1,\"detection\":{"
        "\"labels_any\":[\"person\"],\"min_confidence\":0.8}},"
        "\"schedule\":{\"version\":1,\"timezone\":\"UTC\","
        "\"windows\":[]},"
        "\"suppression\":{\"cooldown_seconds\":30}}";
    json = call(handle_post_event_route, HTTP_METHOD_POST,
                "/api/event-routes", NULL, create_body, NULL, 201);
    char uuid[37];
    safe_strcpy(uuid,
        cJSON_GetObjectItemCaseSensitive(json, "uuid")->valuestring,
        sizeof(uuid), 0);
    TEST_ASSERT_EQUAL_INT(1,
        cJSON_GetObjectItemCaseSensitive(json, "revision")->valueint);
    TEST_ASSERT_EQUAL_INT(30,
        cJSON_GetObjectItemCaseSensitive(
            cJSON_GetObjectItemCaseSensitive(json, "suppression"),
            "cooldown_seconds")->valueint);
    cJSON_Delete(json);

    json = call(handle_get_event_routes, HTTP_METHOD_GET,
                "/api/event-routes", NULL, NULL, NULL, 200);
    TEST_ASSERT_EQUAL_INT(1,
        cJSON_GetObjectItemCaseSensitive(json, "count")->valueint);
    cJSON_Delete(json);

    char path[128];
    snprintf(path, sizeof(path), "/api/event-routes/%s", uuid);
    json = call(handle_put_event_route, HTTP_METHOD_PUT, path, NULL,
                "{\"revision\":1,\"name\":\"Critical people\","
                "\"enabled\":false}", NULL, 200);
    TEST_ASSERT_EQUAL_INT(2,
        cJSON_GetObjectItemCaseSensitive(json, "revision")->valueint);
    TEST_ASSERT_FALSE(cJSON_IsTrue(
        cJSON_GetObjectItemCaseSensitive(json, "enabled")));
    cJSON_Delete(json);

    json = call(handle_put_event_route, HTTP_METHOD_PUT, path, NULL,
                "{\"revision\":1,\"enabled\":true}", NULL, 409);
    cJSON_Delete(json);
    json = call(handle_delete_event_route, HTTP_METHOD_DELETE, path,
                "revision=1", NULL, NULL, 409);
    cJSON_Delete(json);
    json = call(handle_delete_event_route, HTTP_METHOD_DELETE, path,
                "revision=2", NULL, NULL, 200);
    cJSON_Delete(json);

    TEST_ASSERT_TRUE(audit_has_operation("route_create"));
    TEST_ASSERT_TRUE(audit_has_operation("route_update"));
    TEST_ASSERT_TRUE(audit_has_operation("route_delete"));
}

void test_preview_resolves_fleet_selector_without_publishing(void) {
    stream_config_t north = create_camera("North Door");
    create_camera("South Door");
    char body[2048];
    snprintf(body, sizeof(body),
             "{\"name\":\"North only\","
             "\"event_types\":[\"%s\"],"
             "\"camera_scope\":{\"type\":\"selector\","
             "\"selector\":{\"version\":1,\"expression\":{"
             "\"op\":\"camera_uuid\",\"values\":[\"%s\"]}}},"
             "\"schedule\":{\"version\":1,\"timezone\":\"UTC\","
             "\"windows\":[]}}",
             DETECTION_TYPE, north.camera_uuid);
    cJSON *json = call(handle_post_event_route_preview, HTTP_METHOD_POST,
                       "/api/event-routes/preview", NULL, body, NULL, 200);
    TEST_ASSERT_EQUAL_INT(1,
        cJSON_GetObjectItemCaseSensitive(json,
                                         "matched_camera_count")->valueint);
    TEST_ASSERT_FALSE(cJSON_IsTrue(
        cJSON_GetObjectItemCaseSensitive(json, "would_publish")));
    cJSON *sample =
        cJSON_GetObjectItemCaseSensitive(json, "camera_sample");
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(sample));
    TEST_ASSERT_EQUAL_STRING(
        north.camera_uuid,
        cJSON_GetObjectItemCaseSensitive(cJSON_GetArrayItem(sample, 0),
                                         "camera_uuid")->valuestring);
    cJSON_Delete(json);
}

void test_invalid_drafts_and_viewer_mutations_fail_closed(void) {
    cJSON *json = call(handle_post_event_route, HTTP_METHOD_POST,
                       "/api/event-routes", NULL,
                       "{\"name\":\"Unknown\","
                       "\"event_types\":[\"io.lightnvr.unknown.v1\"]}",
                       NULL, 400);
    cJSON_Delete(json);
    json = call(handle_post_event_route, HTTP_METHOD_POST,
                "/api/event-routes", NULL,
                "{\"name\":\"Typo\",\"event_types\":[\""
                DETECTION_TYPE "\"],\"cooldwon_seconds\":10}",
                NULL, 400);
    cJSON_Delete(json);

    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("routeviewer", "password123", NULL,
                               USER_ROLE_VIEWER, true, &user_id));
    char api_key[128] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_generate_api_key(user_id, api_key, sizeof(api_key)));
    g_config.web_auth_enabled = true;
    json = call(handle_get_event_routes, HTTP_METHOD_GET,
                "/api/event-routes", NULL, NULL, api_key, 403);
    cJSON_Delete(json);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0 || db_auth_init() != 0) {
        fprintf(stderr, "FATAL: failed to initialize event route API test\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_catalog_and_route_crud_are_versioned_and_audited);
    RUN_TEST(test_preview_resolves_fleet_selector_without_publishing);
    RUN_TEST(test_invalid_drafts_and_viewer_mutations_fail_closed);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
