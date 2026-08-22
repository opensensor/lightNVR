/**
 * @file test_api_handlers_locations.c
 * @brief Layer 2 tests for UUID-based fleet location HTTP handlers.
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
#include "database/db_core.h"
#include "database/db_locations.h"
#include "database/db_streams.h"
#include "utils/strings.h"
#include "web/api_handlers_locations.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_location_handlers_test.db"

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

static camera_location_t create_location(const char *name,
                                         const char *parent_uuid) {
    camera_location_t location;
    memset(&location, 0, sizeof(location));
    safe_strcpy(location.name, name, sizeof(location.name), 0);
    safe_strcpy(location.type, "area", sizeof(location.type), 0);
    safe_strcpy(location.metadata_json, "{}", sizeof(location.metadata_json), 0);
    if (parent_uuid) {
        safe_strcpy(location.parent_uuid, parent_uuid,
                    sizeof(location.parent_uuid), 0);
    }
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_create(&location));
    return location;
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
    do {
        sqlite3_exec(db,
                     "DELETE FROM camera_locations WHERE is_system = 0 "
                     "AND NOT EXISTS (SELECT 1 FROM camera_locations child "
                     "WHERE child.parent_uuid = camera_locations.uuid);",
                     NULL, NULL, NULL);
    } while (sqlite3_changes(db) > 0);
}

void tearDown(void) {}

void test_get_locations_returns_seeded_unassigned_root(void) {
    http_request_t request;
    http_response_t response;
    init_request(&request, "/api/locations", NULL);
    http_response_init(&response);

    handle_get_locations(&request, &response);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);

    cJSON *root = parse_response(&response);
    cJSON *locations = cJSON_GetObjectItemCaseSensitive(root, "locations");
    cJSON *unassigned =
        cJSON_GetObjectItemCaseSensitive(root, "unassigned_uuid");
    TEST_ASSERT_TRUE(cJSON_IsArray(locations));
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(locations));
    TEST_ASSERT_TRUE(cJSON_IsString(unassigned));
    cJSON *item = cJSON_GetArrayItem(locations, 0);
    TEST_ASSERT_EQUAL_STRING(
        "Unassigned",
        cJSON_GetObjectItemCaseSensitive(item, "name")->valuestring);
    TEST_ASSERT_TRUE(cJSON_IsTrue(
        cJSON_GetObjectItemCaseSensitive(item, "is_system")));

    cJSON_Delete(root);
    http_response_free(&response);
}

void test_location_crud_preserves_hierarchy_and_metadata(void) {
    http_request_t request;
    http_response_t response;
    init_request(&request, "/api/locations",
                 "{\"name\":\"SJC\",\"type\":\"site\"}");
    http_response_init(&response);
    handle_post_location(&request, &response);
    TEST_ASSERT_EQUAL_INT(201, response.status_code);
    cJSON *site_json = parse_response(&response);
    const char *site_value =
        cJSON_GetObjectItemCaseSensitive(site_json, "uuid")->valuestring;
    char site_uuid[CAMERA_UUID_STRING_SIZE];
    safe_strcpy(site_uuid, site_value, sizeof(site_uuid), 0);
    cJSON_Delete(site_json);
    http_response_free(&response);

    char body[1024];
    snprintf(body, sizeof(body),
             "{\"name\":\"Building C\",\"type\":\"building\","
             "\"parent_uuid\":\"%s\",\"sort_order\":30,"
             "\"metadata\":{\"address\":\"100 Main St\"}}",
             site_uuid);
    init_request(&request, "/api/locations", body);
    http_response_init(&response);
    handle_post_location(&request, &response);
    TEST_ASSERT_EQUAL_INT(201, response.status_code);
    cJSON *building_json = parse_response(&response);
    char building_uuid[CAMERA_UUID_STRING_SIZE];
    safe_strcpy(building_uuid,
                cJSON_GetObjectItemCaseSensitive(building_json, "uuid")->valuestring,
                sizeof(building_uuid), 0);
    TEST_ASSERT_EQUAL_STRING(
        site_uuid,
        cJSON_GetObjectItemCaseSensitive(building_json,
                                         "parent_uuid")->valuestring);
    cJSON *metadata =
        cJSON_GetObjectItemCaseSensitive(building_json, "metadata");
    TEST_ASSERT_EQUAL_STRING(
        "100 Main St",
        cJSON_GetObjectItemCaseSensitive(metadata, "address")->valuestring);
    cJSON_Delete(building_json);
    http_response_free(&response);

    char path[MAX_PATH_LENGTH];
    snprintf(path, sizeof(path), "/api/locations/%s", building_uuid);
    init_request(&request, path,
                 "{\"name\":\"Building Charlie\",\"sort_order\":10}");
    http_response_init(&response);
    handle_put_location(&request, &response);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    building_json = parse_response(&response);
    TEST_ASSERT_EQUAL_STRING(
        "Building Charlie",
        cJSON_GetObjectItemCaseSensitive(building_json, "name")->valuestring);
    TEST_ASSERT_EQUAL_INT(
        10, cJSON_GetObjectItemCaseSensitive(building_json,
                                             "sort_order")->valueint);
    cJSON_Delete(building_json);
    http_response_free(&response);

    snprintf(path, sizeof(path), "/api/locations/%s", site_uuid);
    init_request(&request, path, NULL);
    http_response_init(&response);
    handle_delete_location(&request, &response);
    TEST_ASSERT_EQUAL_INT(409, response.status_code);
    http_response_free(&response);

    snprintf(path, sizeof(path), "/api/locations/%s", building_uuid);
    init_request(&request, path, NULL);
    http_response_init(&response);
    handle_delete_location(&request, &response);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    http_response_free(&response);

    snprintf(path, sizeof(path), "/api/locations/%s", site_uuid);
    init_request(&request, path, NULL);
    http_response_init(&response);
    handle_delete_location(&request, &response);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    http_response_free(&response);
}

void test_put_camera_location_assigns_by_stable_uuid(void) {
    camera_location_t location = create_location("North Lobby", NULL);
    stream_config_t stream = create_stream("lobby_camera");

    char path[MAX_PATH_LENGTH];
    char body[128];
    snprintf(path, sizeof(path), "/api/cameras/%s/location",
             stream.camera_uuid);
    snprintf(body, sizeof(body), "{\"location_uuid\":\"%s\"}",
             location.uuid);
    http_request_t request;
    http_response_t response;
    init_request(&request, path, body);
    http_response_init(&response);
    handle_put_camera_location(&request, &response);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);

    cJSON *json = parse_response(&response);
    TEST_ASSERT_EQUAL_STRING(
        stream.camera_uuid,
        cJSON_GetObjectItemCaseSensitive(json, "camera_uuid")->valuestring);
    TEST_ASSERT_EQUAL_STRING(
        location.uuid,
        cJSON_GetObjectItemCaseSensitive(json, "location_uuid")->valuestring);
    cJSON_Delete(json);
    http_response_free(&response);

    TEST_ASSERT_EQUAL_INT(0,
                          get_stream_config_by_uuid(stream.camera_uuid, &stream));
    TEST_ASSERT_EQUAL_STRING(location.uuid, stream.location_uuid);
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_CONFLICT,
                          db_location_delete(location.uuid));
}

void test_location_cycle_returns_conflict(void) {
    camera_location_t parent = create_location("Parent", NULL);
    camera_location_t child = create_location("Child", parent.uuid);

    char path[MAX_PATH_LENGTH];
    char body[128];
    snprintf(path, sizeof(path), "/api/locations/%s", parent.uuid);
    snprintf(body, sizeof(body), "{\"parent_uuid\":\"%s\"}", child.uuid);
    http_request_t request;
    http_response_t response;
    init_request(&request, path, body);
    http_response_init(&response);
    handle_put_location(&request, &response);
    TEST_ASSERT_EQUAL_INT(409, response.status_code);
    http_response_free(&response);
}

void test_location_handlers_require_admin_when_auth_enabled(void) {
    g_config.web_auth_enabled = true;
    http_request_t request;
    http_response_t response;
    init_request(&request, "/api/locations", NULL);
    http_response_init(&response);

    handle_get_locations(&request, &response);
    TEST_ASSERT_EQUAL_INT(401, response.status_code);
    http_response_free(&response);
}

void test_location_handlers_reject_invalid_payloads(void) {
    http_request_t request;
    http_response_t response;
    init_request(&request, "/api/locations",
                 "{\"name\":\"Bad Metadata\",\"metadata\":[]}");
    http_response_init(&response);
    handle_post_location(&request, &response);
    TEST_ASSERT_EQUAL_INT(400, response.status_code);
    http_response_free(&response);

    init_request(&request, "/api/cameras/not-a-uuid/location",
                 "{\"location_uuid\":\"also-bad\"}");
    http_response_init(&response);
    handle_put_camera_location(&request, &response);
    TEST_ASSERT_EQUAL_INT(400, response.status_code);
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
    RUN_TEST(test_get_locations_returns_seeded_unassigned_root);
    RUN_TEST(test_location_crud_preserves_hierarchy_and_metadata);
    RUN_TEST(test_put_camera_location_assigns_by_stable_uuid);
    RUN_TEST(test_location_cycle_returns_conflict);
    RUN_TEST(test_location_handlers_require_admin_when_auth_enabled);
    RUN_TEST(test_location_handlers_reject_invalid_payloads);
    int result = UNITY_END();

    shutdown_database();
    unlink(TEST_DB_PATH);
    free(g_config.streams);
    g_config.streams = NULL;
    return result;
}
