/**
 * @file test_api_handlers_camera_tags.c
 * @brief Layer 2 tests for normalized camera tag HTTP handlers.
 */

#define _POSIX_C_SOURCE 200809L

#include <cjson/cJSON.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sqlite3.h>

#include "unity.h"
#include "core/config.h"
#include "database/db_camera_tags.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#include "utils/strings.h"
#include "web/api_handlers_camera_tags.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_camera_tag_handlers_test.db"

extern config_t g_config;

static cJSON *parse_response(const http_response_t *response) {
    TEST_ASSERT_NOT_NULL(response->body);
    cJSON *json = cJSON_Parse((const char *)response->body);
    TEST_ASSERT_NOT_NULL(json);
    return json;
}

static void init_request(http_request_t *request, const char *path,
                         const char *body) {
    http_request_init(request);
    safe_strcpy(request->path, path, sizeof(request->path), 0);
    if (body) {
        request->body = (void *)body;
        request->body_len = strlen(body);
    }
}

static camera_tag_t create_tag(const char *label) {
    camera_tag_t tag;
    memset(&tag, 0, sizeof(tag));
    safe_strcpy(tag.label, label, sizeof(tag.label), 0);
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_TAG_OK, db_camera_tag_create(&tag));
    return tag;
}

static stream_config_t create_stream(const char *name) {
    stream_config_t stream;
    memset(&stream, 0, sizeof(stream));
    safe_strcpy(stream.name, name, sizeof(stream.name), 0);
    safe_strcpy(stream.url, "rtsp://camera/stream", sizeof(stream.url), 0);
    safe_strcpy(stream.codec, "h264", sizeof(stream.codec), 0);
    stream.enabled = true;
    stream.streaming_enabled = true;
    stream.width = 1920;
    stream.height = 1080;
    stream.fps = 25;
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&stream));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name(name, &stream));
    return stream;
}

void setUp(void) {
    g_config.web_auth_enabled = false;
    sqlite3 *db = get_db_handle();
    sqlite3_exec(db, "DELETE FROM streams;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM camera_tags;", NULL, NULL, NULL);
}

void tearDown(void) {}

void test_camera_tag_dictionary_crud(void) {
    http_request_t request;
    http_response_t response;
    init_request(&request, "/api/camera-tags",
                 "{\"label\":\"Outdoor\",\"color\":\"#12ab34\","
                 "\"description\":\"Exterior cameras\"}");
    http_response_init(&response);
    handle_post_camera_tag(&request, &response);
    TEST_ASSERT_EQUAL_INT(201, response.status_code);
    cJSON *created = parse_response(&response);
    char tag_uuid[CAMERA_UUID_STRING_SIZE];
    safe_strcpy(tag_uuid,
                cJSON_GetObjectItemCaseSensitive(created, "uuid")->valuestring,
                sizeof(tag_uuid), 0);
    TEST_ASSERT_EQUAL_STRING(
        "#12ab34",
        cJSON_GetObjectItemCaseSensitive(created, "color")->valuestring);
    cJSON_Delete(created);
    http_response_free(&response);

    init_request(&request, "/api/camera-tags",
                 "{\"label\":\"outdoor\"}");
    http_response_init(&response);
    handle_post_camera_tag(&request, &response);
    TEST_ASSERT_EQUAL_INT(409, response.status_code);
    http_response_free(&response);

    char path[MAX_PATH_LENGTH];
    snprintf(path, sizeof(path), "/api/camera-tags/%s", tag_uuid);
    init_request(&request, path,
                 "{\"label\":\"Exterior\",\"description\":\"Renamed\"}");
    http_response_init(&response);
    handle_put_camera_tag(&request, &response);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    cJSON *updated = parse_response(&response);
    TEST_ASSERT_EQUAL_STRING(
        "Exterior",
        cJSON_GetObjectItemCaseSensitive(updated, "label")->valuestring);
    cJSON_Delete(updated);
    http_response_free(&response);

    init_request(&request, "/api/camera-tags", NULL);
    http_response_init(&response);
    handle_get_camera_tags(&request, &response);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    cJSON *list = parse_response(&response);
    TEST_ASSERT_EQUAL_INT(
        1, cJSON_GetObjectItemCaseSensitive(list, "count")->valueint);
    cJSON_Delete(list);
    http_response_free(&response);

    snprintf(path, sizeof(path), "/api/camera-tags/%s", tag_uuid);
    init_request(&request, path, NULL);
    http_response_init(&response);
    handle_delete_camera_tag(&request, &response);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    http_response_free(&response);
}

void test_camera_assignment_api_uses_tag_uuids_and_updates_legacy(void) {
    camera_tag_t critical = create_tag("critical");
    camera_tag_t entrance = create_tag("entrance");
    stream_config_t stream = create_stream("front_door");

    char path[MAX_PATH_LENGTH];
    char body[256];
    snprintf(path, sizeof(path), "/api/cameras/%s/tags", stream.camera_uuid);
    snprintf(body, sizeof(body),
             "{\"tag_uuids\":[\"%s\",\"%s\"]}",
             entrance.uuid, critical.uuid);
    http_request_t request;
    http_response_t response;
    init_request(&request, path, body);
    http_response_init(&response);
    handle_put_camera_tag_assignments(&request, &response);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    cJSON *result = parse_response(&response);
    cJSON *tags = cJSON_GetObjectItemCaseSensitive(result, "tags");
    TEST_ASSERT_TRUE(cJSON_IsArray(tags));
    TEST_ASSERT_EQUAL_INT(2, cJSON_GetArraySize(tags));
    TEST_ASSERT_EQUAL_STRING(
        "critical",
        cJSON_GetObjectItemCaseSensitive(cJSON_GetArrayItem(tags, 0),
                                         "label")->valuestring);
    cJSON_Delete(result);
    http_response_free(&response);

    TEST_ASSERT_EQUAL_INT(0,
                          get_stream_config_by_uuid(stream.camera_uuid, &stream));
    TEST_ASSERT_EQUAL_STRING("critical,entrance", stream.tags);

    init_request(&request, path, NULL);
    http_response_init(&response);
    handle_get_camera_tag_assignments(&request, &response);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    result = parse_response(&response);
    TEST_ASSERT_EQUAL_INT(
        2, cJSON_GetObjectItemCaseSensitive(result, "count")->valueint);
    cJSON_Delete(result);
    http_response_free(&response);
}

void test_merge_api_moves_assignments_to_target(void) {
    camera_tag_t preferred = create_tag("entrance");
    camera_tag_t duplicate = create_tag("entry");
    stream_config_t stream = create_stream("merge_api");
    const char *assigned[] = {duplicate.uuid};
    TEST_ASSERT_EQUAL_INT(
        DB_CAMERA_TAG_OK,
        db_camera_tag_set_for_camera(stream.camera_uuid, assigned, 1));

    char path[MAX_PATH_LENGTH];
    char body[128];
    snprintf(path, sizeof(path), "/api/camera-tags/%s/merge", duplicate.uuid);
    snprintf(body, sizeof(body), "{\"target_uuid\":\"%s\"}",
             preferred.uuid);
    http_request_t request;
    http_response_t response;
    init_request(&request, path, body);
    http_response_init(&response);
    handle_post_camera_tag_merge(&request, &response);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    http_response_free(&response);

    camera_tag_t assigned_tags[2];
    TEST_ASSERT_EQUAL_INT(
        1, db_camera_tag_list_for_camera(stream.camera_uuid, assigned_tags, 2));
    TEST_ASSERT_EQUAL_STRING(preferred.uuid, assigned_tags[0].uuid);
    TEST_ASSERT_EQUAL_INT(0,
                          get_stream_config_by_uuid(stream.camera_uuid, &stream));
    TEST_ASSERT_EQUAL_STRING("entrance", stream.tags);
}

void test_camera_tag_handlers_validate_payload_and_auth(void) {
    http_request_t request;
    http_response_t response;
    init_request(&request, "/api/camera-tags",
                 "{\"label\":\"bad\",\"color\":\"red\"}");
    http_response_init(&response);
    handle_post_camera_tag(&request, &response);
    TEST_ASSERT_EQUAL_INT(400, response.status_code);
    http_response_free(&response);

    init_request(&request, "/api/camera-tags",
                 "{\"label\":\"ambiguous,tag\"}");
    http_response_init(&response);
    handle_post_camera_tag(&request, &response);
    TEST_ASSERT_EQUAL_INT(400, response.status_code);
    http_response_free(&response);

    stream_config_t stream = create_stream("validation");
    char path[MAX_PATH_LENGTH];
    snprintf(path, sizeof(path), "/api/cameras/%s/tags", stream.camera_uuid);
    init_request(&request, path, "{\"tag_uuids\":[\"bad\"]}");
    http_response_init(&response);
    handle_put_camera_tag_assignments(&request, &response);
    TEST_ASSERT_EQUAL_INT(400, response.status_code);
    http_response_free(&response);

    g_config.web_auth_enabled = true;
    init_request(&request, "/api/camera-tags", NULL);
    http_response_init(&response);
    handle_get_camera_tags(&request, &response);
    TEST_ASSERT_EQUAL_INT(401, response.status_code);
    http_response_free(&response);
}

int main(void) {
    load_default_config(&g_config);
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_camera_tag_dictionary_crud);
    RUN_TEST(test_camera_assignment_api_uses_tag_uuids_and_updates_legacy);
    RUN_TEST(test_merge_api_moves_assignments_to_target);
    RUN_TEST(test_camera_tag_handlers_validate_payload_and_auth);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    free(g_config.streams);
    g_config.streams = NULL;
    return result;
}
