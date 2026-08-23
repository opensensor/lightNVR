/**
 * @file test_api_handlers_event_destinations.c
 * @brief Destination profile CRUD, redaction, RBAC, and audit API tests.
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
#include "database/db_event_destinations.h"
#include "unity.h"
#include "utils/memory.h"
#include "utils/strings.h"
#include "web/api_handlers_event_destinations.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_api_event_destinations.db"

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
    sqlite3_exec(db, "DELETE FROM event_destinations;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM audit_events;", NULL, NULL, NULL);
    remove_user("destinationviewer");
}

void tearDown(void) {
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
}

void test_destination_crud_is_secretless_versioned_and_audited(void) {
    const char *create_body =
        "{\"name\":\"SJC operations\","
        "\"description\":\"Cloud event input\","
        "\"broker\":{\"host\":\"mqtt.sjc.example\","
        "\"port\":8883,"
        "\"topic_template\":\"sjc/{type}/{subject_id}\","
        "\"keepalive_seconds\":45,\"qos\":1},"
        "\"authentication\":{\"username\":\"publisher\","
        "\"password\":\"not-in-responses\"},"
        "\"tls\":{\"mode\":\"system\"}}";
    cJSON *json = call(handle_post_event_destination, HTTP_METHOD_POST,
                       "/api/event-destinations", NULL, create_body, NULL, 201);
    char uuid[EVENT_DESTINATION_UUID_MAX];
    safe_strcpy(uuid,
        cJSON_GetObjectItemCaseSensitive(json, "uuid")->valuestring,
        sizeof(uuid), 0);
    TEST_ASSERT_EQUAL_INT(1,
        cJSON_GetObjectItemCaseSensitive(json, "revision")->valueint);
    TEST_ASSERT_EQUAL_INT(
        0, strncmp("lightnvr-",
                   cJSON_GetObjectItemCaseSensitive(
                       cJSON_GetObjectItemCaseSensitive(json, "broker"),
                       "client_id")->valuestring,
                   strlen("lightnvr-")));
    cJSON *authentication =
        cJSON_GetObjectItemCaseSensitive(json, "authentication");
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        authentication, "password_configured")));
    TEST_ASSERT_NULL(cJSON_GetObjectItemCaseSensitive(authentication,
                                                       "password"));
    cJSON_Delete(json);

    json = call(handle_get_event_destinations, HTTP_METHOD_GET,
                "/api/event-destinations", NULL, NULL, NULL, 200);
    TEST_ASSERT_EQUAL_INT(1,
        cJSON_GetObjectItemCaseSensitive(json, "count")->valueint);
    TEST_ASSERT_EQUAL_STRING(
        "mqtt:default",
        cJSON_GetObjectItemCaseSensitive(
            cJSON_GetObjectItemCaseSensitive(json, "default_destination"),
            "key")->valuestring);
    cJSON_Delete(json);

    char path[128];
    snprintf(path, sizeof(path), "/api/event-destinations/%s", uuid);
    json = call(handle_put_event_destination, HTTP_METHOD_PUT, path, NULL,
                "{\"revision\":1,\"description\":\"Updated\"}", NULL,
                200);
    TEST_ASSERT_EQUAL_INT(2,
        cJSON_GetObjectItemCaseSensitive(json, "revision")->valueint);
    cJSON_Delete(json);

    char password[EVENT_DESTINATION_PASSWORD_MAX] = {0};
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_DESTINATION_OK,
        db_event_destination_get_password(uuid, 2, password,
                                          sizeof(password)));
    TEST_ASSERT_EQUAL_STRING("not-in-responses", password);
    secure_zero_memory(password, sizeof(password));

    json = call(handle_put_event_destination, HTTP_METHOD_PUT, path, NULL,
                "{\"revision\":2,\"authentication\":{\"password\":null}}",
                NULL, 200);
    authentication = cJSON_GetObjectItemCaseSensitive(json, "authentication");
    TEST_ASSERT_FALSE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        authentication, "password_configured")));
    TEST_ASSERT_NULL(cJSON_GetObjectItemCaseSensitive(authentication,
                                                       "password"));
    cJSON_Delete(json);
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_DESTINATION_OK,
        db_event_destination_get_password(uuid, 3, password,
                                          sizeof(password)));
    TEST_ASSERT_EQUAL_STRING("", password);

    json = call(handle_delete_event_destination, HTTP_METHOD_DELETE, path,
                "revision=2", NULL, NULL, 409);
    cJSON_Delete(json);
    json = call(handle_delete_event_destination, HTTP_METHOD_DELETE, path,
                "revision=3", NULL, NULL, 200);
    cJSON_Delete(json);

    TEST_ASSERT_TRUE(audit_has_operation("destination_create"));
    TEST_ASSERT_TRUE(audit_has_operation("destination_update"));
    TEST_ASSERT_TRUE(audit_has_operation("destination_delete"));
}

void test_invalid_fields_and_insecure_tls_combinations_fail_closed(void) {
    cJSON *json = call(handle_post_event_destination, HTTP_METHOD_POST,
                       "/api/event-destinations", NULL,
                       "{\"name\":\"Typo\",\"broker\":{"
                       "\"host\":\"mqtt.example\",\"prt\":8883}}",
                       NULL, 400);
    cJSON_Delete(json);
    json = call(handle_post_event_destination, HTTP_METHOD_POST,
                "/api/event-destinations", NULL,
                "{\"name\":\"Bad TLS\",\"broker\":{"
                "\"host\":\"mqtt.example\"},\"tls\":{"
                "\"mode\":\"custom_ca\",\"ca_file\":\"relative.pem\"}}",
                NULL, 400);
    cJSON_Delete(json);
}

void test_viewer_cannot_read_or_mutate_destination_profiles(void) {
    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("destinationviewer", "password123", NULL,
                               USER_ROLE_VIEWER, true, &user_id));
    char api_key[128] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_generate_api_key(user_id, api_key, sizeof(api_key)));
    g_config.web_auth_enabled = true;
    cJSON *json = call(handle_get_event_destinations, HTTP_METHOD_GET,
                       "/api/event-destinations", NULL, NULL, api_key, 403);
    cJSON_Delete(json);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0 || db_auth_init() != 0) {
        fprintf(stderr,
                "FATAL: failed to initialize event destination API test\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_destination_crud_is_secretless_versioned_and_audited);
    RUN_TEST(test_invalid_fields_and_insecure_tls_combinations_fail_closed);
    RUN_TEST(test_viewer_cannot_read_or_mutate_destination_profiles);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
