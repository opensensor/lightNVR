/**
 * @file test_api_handlers_camera_collections.c
 * @brief Collection CRUD, dynamic evaluation, preview, and visibility tests.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <unistd.h>
#include <sqlite3.h>
#include <cjson/cJSON.h>

#include "unity.h"
#include "core/config.h"
#include "database/db_auth.h"
#include "database/db_authorization.h"
#include "database/db_camera_tags.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#include "utils/strings.h"
#include "web/api_handlers_camera_collections.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_api_camera_collections_test.db"

static stream_config_t create_camera(const char *name, const char *tags) {
    stream_config_t stream;
    memset(&stream, 0, sizeof(stream));
    safe_strcpy(stream.name, name, sizeof(stream.name), 0);
    safe_strcpy(stream.url, "rtsp://camera/live", sizeof(stream.url), 0);
    safe_strcpy(stream.tags, tags ? tags : "", sizeof(stream.tags), 0);
    safe_strcpy(stream.codec, "h264", sizeof(stream.codec), 0);
    stream.enabled = true;
    stream.streaming_enabled = true;
    stream.record = true;
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&stream));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name(name, &stream));
    return stream;
}

static camera_tag_t find_tag(const char *label) {
    int total = db_camera_tag_count();
    TEST_ASSERT_GREATER_THAN(0, total);
    camera_tag_t *tags = calloc((size_t)total, sizeof(*tags));
    TEST_ASSERT_NOT_NULL(tags);
    TEST_ASSERT_EQUAL_INT(total, db_camera_tag_list(tags, total));
    camera_tag_t found;
    memset(&found, 0, sizeof(found));
    for (int i = 0; i < total; i++) {
        if (strcasecmp(tags[i].label, label) == 0) found = tags[i];
    }
    free(tags);
    TEST_ASSERT_TRUE(found.uuid[0] != '\0');
    return found;
}

static cJSON *call(void (*handler)(const http_request_t *, http_response_t *),
                   http_method_t method, const char *path, const char *body,
                   const char *api_key, int expected_status) {
    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    req.method = method;
    safe_strcpy(req.path, path, sizeof(req.path), 0);
    safe_strcpy(req.uri, path, sizeof(req.uri), 0);
    safe_strcpy(req.client_ip, "127.0.0.1", sizeof(req.client_ip), 0);
    if (body) {
        req.body = (void *)body;
        req.body_len = strlen(body);
    }
    if (api_key) {
        safe_strcpy(req.headers[0].name, "X-API-Key",
                    sizeof(req.headers[0].name), 0);
        safe_strcpy(req.headers[0].value, api_key,
                    sizeof(req.headers[0].value), 0);
        req.num_headers = 1;
    }
    handler(&req, &res);
    TEST_ASSERT_EQUAL_INT(expected_status, res.status_code);
    cJSON *json = res.body ? cJSON_Parse((const char *)res.body) : NULL;
    TEST_ASSERT_NOT_NULL(json);
    http_response_free(&res);
    return json;
}

static void collection_path(char *path, size_t size, const char *uuid,
                            const char *suffix) {
    snprintf(path, size, "/api/camera-collections/%s%s", uuid,
             suffix ? suffix : "");
}

static void remove_test_user(const char *username) {
    user_t user;
    if (db_auth_get_user_by_username(username, &user) == 0) {
        db_auth_delete_user(user.id);
    }
}

void setUp(void) {
    sqlite3 *db = get_db_handle();
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
    sqlite3_exec(db, "DELETE FROM camera_collections;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM streams;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM camera_tags;", NULL, NULL, NULL);
    remove_test_user("collectionviewer");
}

void tearDown(void) {
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
}

void test_static_collection_crud_and_members(void) {
    stream_config_t first = create_camera("First", "Outdoor");
    stream_config_t second = create_camera("Second", "Indoor");
    cJSON *json = call(handle_post_camera_collection, HTTP_METHOD_POST,
                       "/api/camera-collections",
                       "{\"name\":\"Guard Tour\",\"type\":\"static\","
                       "\"description\":\"North route\"}",
                       NULL, 201);
    const char *uuid_value =
        cJSON_GetObjectItemCaseSensitive(json, "uuid")->valuestring;
    char uuid[CAMERA_UUID_STRING_SIZE];
    safe_strcpy(uuid, uuid_value, sizeof(uuid), 0);
    TEST_ASSERT_EQUAL_INT(0,
        cJSON_GetObjectItemCaseSensitive(json, "effective_count")->valueint);
    cJSON_Delete(json);

    char path[MAX_PATH_LENGTH];
    collection_path(path, sizeof(path), uuid, "/members");
    char body[256];
    snprintf(body, sizeof(body),
             "{\"camera_uuids\":[\"%s\",\"%s\"]}",
             first.camera_uuid, second.camera_uuid);
    json = call(handle_put_camera_collection_members, HTTP_METHOD_PUT,
                path, body, NULL, 200);
    TEST_ASSERT_EQUAL_INT(2,
        cJSON_GetObjectItemCaseSensitive(json, "count")->valueint);
    cJSON_Delete(json);

    collection_path(path, sizeof(path), uuid, NULL);
    json = call(handle_get_camera_collection, HTTP_METHOD_GET,
                path, NULL, NULL, 200);
    TEST_ASSERT_EQUAL_INT(2,
        cJSON_GetObjectItemCaseSensitive(json, "member_count")->valueint);
    TEST_ASSERT_EQUAL_INT(2,
        cJSON_GetObjectItemCaseSensitive(json, "effective_count")->valueint);
    cJSON_Delete(json);

    json = call(handle_delete_camera_collection, HTTP_METHOD_DELETE,
                path, NULL, NULL, 200);
    cJSON_Delete(json);
    json = call(handle_get_camera_collection, HTTP_METHOD_GET,
                path, NULL, NULL, 404);
    cJSON_Delete(json);
}

void test_smart_collection_membership_updates_with_tags(void) {
    create_camera("Outside One", "Outdoor");
    create_camera("Inside", "Indoor");
    camera_tag_t outdoor = find_tag("Outdoor");
    char body[1536];
    snprintf(body, sizeof(body),
             "{\"name\":\"All Outdoor\",\"type\":\"smart\","
             "\"selector\":{\"version\":1,\"expression\":{"
             "\"op\":\"tag_any\",\"uuids\":[\"%s\"]}}}",
             outdoor.uuid);
    cJSON *json = call(handle_post_camera_collection, HTTP_METHOD_POST,
                       "/api/camera-collections", body, NULL, 201);
    char uuid[CAMERA_UUID_STRING_SIZE];
    safe_strcpy(uuid,
        cJSON_GetObjectItemCaseSensitive(json, "uuid")->valuestring,
        sizeof(uuid), 0);
    TEST_ASSERT_EQUAL_INT(1,
        cJSON_GetObjectItemCaseSensitive(json, "effective_count")->valueint);
    cJSON_Delete(json);

    create_camera("Outside Two", "Outdoor");
    char path[MAX_PATH_LENGTH];
    collection_path(path, sizeof(path), uuid, NULL);
    json = call(handle_get_camera_collection, HTTP_METHOD_GET,
                path, NULL, NULL, 200);
    TEST_ASSERT_EQUAL_INT(2,
        cJSON_GetObjectItemCaseSensitive(json, "effective_count")->valueint);
    TEST_ASSERT_TRUE(cJSON_IsObject(
        cJSON_GetObjectItemCaseSensitive(json, "selector")));
    cJSON_Delete(json);

    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("collectionviewer", "password123", NULL,
                               USER_ROLE_VIEWER, true, &user_id));
    char api_key[128] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_generate_api_key(user_id, api_key, sizeof(api_key)));
    g_config.web_auth_enabled = true;
    json = call(handle_get_camera_collection, HTTP_METHOD_GET,
                path, NULL, api_key, 200);
    TEST_ASSERT_TRUE(cJSON_IsNull(
        cJSON_GetObjectItemCaseSensitive(json, "selector")));
    TEST_ASSERT_TRUE(cJSON_IsTrue(
        cJSON_GetObjectItemCaseSensitive(json, "selector_redacted")));
    cJSON_Delete(json);
}

void test_preview_returns_count_and_bounded_sample(void) {
    create_camera("Preview One", "");
    create_camera("Preview Two", "");
    cJSON *json = call(handle_post_camera_collection, HTTP_METHOD_POST,
                       "/api/camera-collections",
                       "{\"name\":\"Everything\",\"type\":\"smart\","
                       "\"selector\":{\"version\":1,\"expression\":{"
                       "\"op\":\"all\"}}}", NULL, 201);
    char path[MAX_PATH_LENGTH];
    collection_path(path, sizeof(path),
        cJSON_GetObjectItemCaseSensitive(json, "uuid")->valuestring,
        "/preview");
    cJSON_Delete(json);
    json = call(handle_post_camera_collection_preview, HTTP_METHOD_POST,
                path, "{}", NULL, 200);
    TEST_ASSERT_EQUAL_INT(2,
        cJSON_GetObjectItemCaseSensitive(json, "matched_count")->valueint);
    TEST_ASSERT_EQUAL_INT(2, cJSON_GetArraySize(
        cJSON_GetObjectItemCaseSensitive(json, "sample")));
    cJSON_Delete(json);
}

void test_rejects_invalid_smart_selector_and_non_admin_mutation(void) {
    cJSON *json = call(handle_post_camera_collection, HTTP_METHOD_POST,
                       "/api/camera-collections",
                       "{\"name\":\"Broken\",\"type\":\"smart\"}",
                       NULL, 400);
    cJSON_Delete(json);
    json = call(handle_post_camera_collection, HTTP_METHOD_POST,
                "/api/camera-collections",
                "{\"name\":\"Broken\",\"type\":\"smart\","
                "\"selector\":{\"version\":1,\"expression\":{"
                "\"op\":\"sql\"}}}", NULL, 400);
    cJSON_Delete(json);

    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("collectionviewer", "password123", NULL,
                               USER_ROLE_VIEWER, true, &user_id));
    char api_key[128] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_generate_api_key(user_id, api_key, sizeof(api_key)));
    g_config.web_auth_enabled = true;
    json = call(handle_post_camera_collection, HTTP_METHOD_POST,
                "/api/camera-collections",
                "{\"name\":\"Denied\",\"type\":\"static\"}",
                api_key, 403);
    cJSON_Delete(json);
}

void test_rejects_malformed_collection_and_member_uuids(void) {
    cJSON *json = call(handle_get_camera_collection, HTTP_METHOD_GET,
                       "/api/camera-collections/"
                       "00000000-0000-4000-8000-00000000000g",
                       NULL, NULL, 400);
    cJSON_Delete(json);

    json = call(handle_post_camera_collection, HTTP_METHOD_POST,
                "/api/camera-collections",
                "{\"name\":\"UUID Validation\",\"type\":\"static\"}",
                NULL, 201);
    char member_path[MAX_PATH_LENGTH];
    collection_path(member_path, sizeof(member_path),
        cJSON_GetObjectItemCaseSensitive(json, "uuid")->valuestring,
        "/members");
    cJSON_Delete(json);
    json = call(handle_put_camera_collection_members, HTTP_METHOD_PUT,
                member_path,
                "{\"camera_uuids\":["
                "\"00000000-0000-4000-8000-00000000000g\"]}",
                NULL, 400);
    cJSON_Delete(json);
}

void test_private_visibility_and_rbac_filter_counts_and_members(void) {
    stream_config_t outside = create_camera("Outside", "Outdoor");
    stream_config_t inside = create_camera("Inside", "Indoor");
    cJSON *json = call(handle_post_camera_collection, HTTP_METHOD_POST,
                       "/api/camera-collections",
                       "{\"name\":\"Private All\",\"type\":\"static\","
                       "\"shared\":false}", NULL, 201);
    char uuid[CAMERA_UUID_STRING_SIZE];
    safe_strcpy(uuid,
        cJSON_GetObjectItemCaseSensitive(json, "uuid")->valuestring,
        sizeof(uuid), 0);
    cJSON_Delete(json);
    char member_path[MAX_PATH_LENGTH];
    collection_path(member_path, sizeof(member_path), uuid, "/members");
    char member_body[256];
    snprintf(member_body, sizeof(member_body),
             "{\"camera_uuids\":[\"%s\",\"%s\"]}",
             outside.camera_uuid, inside.camera_uuid);
    json = call(handle_put_camera_collection_members, HTTP_METHOD_PUT,
                member_path, member_body, NULL, 200);
    cJSON_Delete(json);
    char path[MAX_PATH_LENGTH];
    collection_path(path, sizeof(path), uuid, NULL);
    json = call(handle_get_camera_collection, HTTP_METHOD_GET,
                path, NULL, NULL, 200);
    TEST_ASSERT_FALSE(cJSON_IsTrue(
        cJSON_GetObjectItemCaseSensitive(json, "shared")));
    TEST_ASSERT_EQUAL_INT(2,
        cJSON_GetObjectItemCaseSensitive(json, "member_count")->valueint);
    cJSON_Delete(json);

    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("collectionviewer", "password123", NULL,
                               USER_ROLE_VIEWER, true, &user_id));
    TEST_ASSERT_EQUAL_INT(0,
                          db_authorization_set_user_mode(user_id, "policy"));
    char selector[512];
    snprintf(selector, sizeof(selector),
             "{\"version\":1,\"expression\":{\"op\":\"camera_uuid\","
             "\"values\":[\"%s\"]}}",
             outside.camera_uuid);
    TEST_ASSERT_EQUAL_INT(
        0, db_authorization_create_user_grant(
               user_id, "00000000-0000-4000-8000-000000000003",
               "selector", selector, NULL, NULL));
    char api_key[128] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_generate_api_key(user_id, api_key, sizeof(api_key)));
    g_config.web_auth_enabled = true;
    json = call(handle_get_camera_collections, HTTP_METHOD_GET,
                "/api/camera-collections", NULL, api_key, 200);
    TEST_ASSERT_EQUAL_INT(0,
        cJSON_GetObjectItemCaseSensitive(json, "count")->valueint);
    cJSON_Delete(json);
    json = call(handle_get_camera_collection, HTTP_METHOD_GET,
                path, NULL, api_key, 404);
    cJSON_Delete(json);

    g_config.web_auth_enabled = false;
    json = call(handle_put_camera_collection, HTTP_METHOD_PUT,
                path, "{\"shared\":true}", NULL, 200);
    cJSON_Delete(json);
    g_config.web_auth_enabled = true;
    json = call(handle_get_camera_collection, HTTP_METHOD_GET,
                path, NULL, api_key, 200);
    TEST_ASSERT_EQUAL_INT(1,
        cJSON_GetObjectItemCaseSensitive(json, "member_count")->valueint);
    TEST_ASSERT_EQUAL_INT(1,
        cJSON_GetObjectItemCaseSensitive(json, "effective_count")->valueint);
    cJSON_Delete(json);
    json = call(handle_get_camera_collections, HTTP_METHOD_GET,
                "/api/camera-collections", NULL, api_key, 200);
    cJSON *listed = cJSON_GetArrayItem(
        cJSON_GetObjectItemCaseSensitive(json, "collections"), 0);
    TEST_ASSERT_EQUAL_INT(1,
        cJSON_GetObjectItemCaseSensitive(listed, "member_count")->valueint);
    cJSON_Delete(json);
    json = call(handle_get_camera_collection_members, HTTP_METHOD_GET,
                member_path, NULL, api_key, 200);
    TEST_ASSERT_EQUAL_INT(1,
        cJSON_GetObjectItemCaseSensitive(json, "count")->valueint);
    TEST_ASSERT_EQUAL_STRING(outside.camera_uuid,
        cJSON_GetArrayItem(
            cJSON_GetObjectItemCaseSensitive(json, "camera_uuids"), 0)->valuestring);
    cJSON_Delete(json);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_static_collection_crud_and_members);
    RUN_TEST(test_smart_collection_membership_updates_with_tags);
    RUN_TEST(test_preview_returns_count_and_bounded_sample);
    RUN_TEST(test_rejects_invalid_smart_selector_and_non_admin_mutation);
    RUN_TEST(test_rejects_malformed_collection_and_member_uuids);
    RUN_TEST(test_private_visibility_and_rbac_filter_counts_and_members);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
