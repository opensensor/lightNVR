/**
 * @file test_api_handlers_fleet.c
 * @brief Fleet inventory, server query, facets, preview, and RBAC tests.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <unistd.h>
#include <sqlite3.h>

#include "unity.h"
#include "core/config.h"
#include "database/db_auth.h"
#include "database/db_camera_tags.h"
#include "database/db_core.h"
#include "database/db_fleet_query.h"
#include "database/db_locations.h"
#include "database/db_streams.h"
#include "utils/strings.h"
#include "web/api_handlers_fleet.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_fleet_query_test.db"

static stream_config_t make_stream(const char *name, const char *url,
                                   const char *tags, bool enabled) {
    stream_config_t stream;
    memset(&stream, 0, sizeof(stream));
    safe_strcpy(stream.name, name, sizeof(stream.name), 0);
    safe_strcpy(stream.url, url, sizeof(stream.url), 0);
    safe_strcpy(stream.tags, tags ? tags : "", sizeof(stream.tags), 0);
    safe_strcpy(stream.codec, "h264", sizeof(stream.codec), 0);
    stream.enabled = enabled;
    stream.streaming_enabled = enabled;
    stream.record = true;
    stream.width = 1920;
    stream.height = 1080;
    stream.fps = 25;
    return stream;
}

static camera_location_t create_location(const char *name, const char *type,
                                         const char *parent_uuid) {
    camera_location_t location;
    memset(&location, 0, sizeof(location));
    safe_strcpy(location.name, name, sizeof(location.name), 0);
    safe_strcpy(location.type, type, sizeof(location.type), 0);
    safe_strcpy(location.metadata_json, "{}", sizeof(location.metadata_json), 0);
    if (parent_uuid) {
        safe_strcpy(location.parent_uuid, parent_uuid,
                    sizeof(location.parent_uuid), 0);
    }
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_create(&location));
    return location;
}

static stream_config_t create_camera(const char *name, const char *url,
                                     const char *tags, bool enabled,
                                     const char *location_uuid) {
    stream_config_t stream = make_stream(name, url, tags, enabled);
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&stream));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name(name, &stream));
    if (location_uuid) {
        TEST_ASSERT_EQUAL_INT(
            DB_LOCATION_OK,
            db_location_assign_camera(stream.camera_uuid, location_uuid));
        safe_strcpy(stream.location_uuid, location_uuid,
                    sizeof(stream.location_uuid), 0);
    }
    return stream;
}

static camera_tag_t find_tag(const char *label) {
    camera_tag_t result;
    memset(&result, 0, sizeof(result));
    int total = db_camera_tag_count();
    TEST_ASSERT_GREATER_THAN(0, total);
    camera_tag_t *tags = calloc((size_t)total, sizeof(*tags));
    TEST_ASSERT_NOT_NULL(tags);
    int count = db_camera_tag_list(tags, total);
    TEST_ASSERT_EQUAL_INT(total, count);
    for (int i = 0; i < count; i++) {
        if (strcasecmp(tags[i].label, label) == 0) {
            result = tags[i];
            break;
        }
    }
    free(tags);
    TEST_ASSERT_TRUE(result.uuid[0] != '\0');
    return result;
}

static cJSON *call_handler(void (*handler)(const http_request_t *,
                                           http_response_t *),
                           const char *body, const char *api_key,
                           int expected_status) {
    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    req.method = HTTP_METHOD_POST;
    safe_strcpy(req.method_str, "POST", sizeof(req.method_str), 0);
    safe_strcpy(req.path, "/api/fleet/cameras/query", sizeof(req.path), 0);
    safe_strcpy(req.client_ip, "127.0.0.1", sizeof(req.client_ip), 0);
    req.body = (void *)body;
    req.body_len = strlen(body);
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

void setUp(void) {
    sqlite3 *db = get_db_handle();
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
    sqlite3_exec(db, "DELETE FROM streams;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM camera_tags;", NULL, NULL, NULL);
    do {
        sqlite3_exec(db,
                     "DELETE FROM camera_locations WHERE is_system = 0 "
                     "AND NOT EXISTS (SELECT 1 FROM camera_locations child "
                     "WHERE child.parent_uuid = camera_locations.uuid);",
                     NULL, NULL, NULL);
    } while (sqlite3_changes(db) > 0);
    user_t user;
    if (db_auth_get_user_by_username("fleetviewer", &user) == 0) {
        db_auth_delete_user(user.id);
    }
}

void tearDown(void) {
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
}

void test_inventory_loads_hierarchy_tags_and_redacts_credentials(void) {
    camera_location_t site = create_location("SJC", "site", NULL);
    camera_location_t building =
        create_location("Building C", "building", site.uuid);
    stream_config_t camera = create_camera(
        "North Door",
        "rtsp://admin:supersecret@10.0.0.10/live?token=querysecret",
        "Outdoor,Critical", true, building.uuid);

    fleet_camera_t *cameras = NULL;
    int count = 0;
    TEST_ASSERT_EQUAL_INT(0, db_fleet_camera_load(&cameras, &count));
    TEST_ASSERT_EQUAL_INT(1, count);
    TEST_ASSERT_EQUAL_STRING(camera.camera_uuid, cameras[0].camera_uuid);
    TEST_ASSERT_EQUAL_STRING("SJC / Building C", cameras[0].location_path);
    TEST_ASSERT_EQUAL_INT(2, cameras[0].location_depth);
    TEST_ASSERT_EQUAL_STRING(site.uuid,
                             cameras[0].location_ancestor_uuids[0]);
    TEST_ASSERT_EQUAL_STRING(building.uuid,
                             cameras[0].location_ancestor_uuids[1]);
    TEST_ASSERT_EQUAL_INT(2, cameras[0].tag_count);
    TEST_ASSERT_NULL(strstr(cameras[0].address, "admin"));
    TEST_ASSERT_NULL(strstr(cameras[0].address, "supersecret"));
    TEST_ASSERT_NULL(strstr(cameras[0].address, "querysecret"));
    TEST_ASSERT_NOT_NULL(strstr(cameras[0].address, "10.0.0.10"));
    TEST_ASSERT_EQUAL_STRING("rtsp://10.0.0.10", cameras[0].address);
    free(cameras);
}

void test_query_composes_selector_search_sort_pagination_and_facets(void) {
    camera_location_t site = create_location("SJC", "site", NULL);
    camera_location_t building =
        create_location("Building C", "building", site.uuid);
    stream_config_t north = create_camera(
        "North Door", "rtsp://10.0.0.10/live", "Outdoor,Critical", true,
        building.uuid);
    create_camera("South Door", "rtsp://10.0.0.11/live", "Outdoor", true,
                  building.uuid);
    create_camera("Office", "rtsp://10.0.0.12/live", "Indoor", false,
                  site.uuid);
    camera_tag_t outdoor = find_tag("Outdoor");

    char body[2048];
    snprintf(body, sizeof(body),
             "{\"selector\":{\"version\":1,\"expression\":{"
             "\"op\":\"and\",\"children\":["
             "{\"op\":\"location_subtree\",\"uuid\":\"%s\"},"
             "{\"op\":\"tag_any\",\"uuids\":[\"%s\"]}]}},"
             "\"search\":\"Door\",\"page\":1,\"page_size\":1,"
             "\"sort_by\":\"name\",\"sort_order\":\"asc\"}",
             site.uuid, outdoor.uuid);
    cJSON *json = call_handler(handle_post_fleet_camera_query, body, NULL, 200);
    TEST_ASSERT_EQUAL_INT(2,
        cJSON_GetObjectItemCaseSensitive(json, "total")->valueint);
    TEST_ASSERT_EQUAL_INT(2,
        cJSON_GetObjectItemCaseSensitive(json, "total_pages")->valueint);
    cJSON *items = cJSON_GetObjectItemCaseSensitive(json, "cameras");
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(items));
    cJSON *first = cJSON_GetArrayItem(items, 0);
    TEST_ASSERT_EQUAL_STRING(north.camera_uuid,
        cJSON_GetObjectItemCaseSensitive(first, "camera_uuid")->valuestring);
    TEST_ASSERT_NULL(strstr(
        cJSON_GetObjectItemCaseSensitive(first, "address")->valuestring, "@"));

    cJSON *facets = cJSON_GetObjectItemCaseSensitive(json, "facets");
    cJSON *tag_facets = cJSON_GetObjectItemCaseSensitive(facets, "tags");
    bool found_outdoor = false;
    cJSON *item = NULL;
    cJSON_ArrayForEach(item, tag_facets) {
        cJSON *uuid = cJSON_GetObjectItemCaseSensitive(item, "uuid");
        if (uuid && strcmp(uuid->valuestring, outdoor.uuid) == 0) {
            TEST_ASSERT_EQUAL_INT(2,
                cJSON_GetObjectItemCaseSensitive(item, "count")->valueint);
            found_outdoor = true;
        }
    }
    TEST_ASSERT_TRUE(found_outdoor);
    cJSON_Delete(json);
}

void test_preview_returns_bounded_match_explanation(void) {
    stream_config_t camera = create_camera(
        "Preview Camera", "rtsp://10.0.0.20/live", "Outdoor", true, NULL);
    camera_tag_t outdoor = find_tag("Outdoor");
    char body[1024];
    snprintf(body, sizeof(body),
             "{\"camera_uuid\":\"%s\",\"selector\":{\"version\":1,"
             "\"expression\":{\"op\":\"tag_any\",\"uuids\":[\"%s\"]}}}",
             camera.camera_uuid, outdoor.uuid);
    cJSON *json = call_handler(handle_post_fleet_selector_preview,
                               body, NULL, 200);
    TEST_ASSERT_TRUE(cJSON_IsTrue(
        cJSON_GetObjectItemCaseSensitive(json, "preview")));
    cJSON *items = cJSON_GetObjectItemCaseSensitive(json, "cameras");
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(items));
    cJSON *clauses = cJSON_GetObjectItemCaseSensitive(
        cJSON_GetArrayItem(items, 0), "matched_clauses");
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(clauses));
    TEST_ASSERT_NOT_NULL(strstr(cJSON_GetArrayItem(clauses, 0)->valuestring,
                                "tag_any"));
    cJSON_Delete(json);
}

void test_existing_tag_rbac_is_applied_before_totals_and_facets(void) {
    create_camera("Outside", "rtsp://10.0.0.30/live", "Outdoor", true, NULL);
    create_camera("Inside", "rtsp://10.0.0.31/live", "Indoor", true, NULL);
    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("fleetviewer", "password123", NULL,
                               USER_ROLE_VIEWER, true, &user_id));
    TEST_ASSERT_EQUAL_INT(0, db_auth_set_allowed_tags(user_id, "Outdoor"));
    char api_key[128] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_generate_api_key(user_id, api_key, sizeof(api_key)));
    g_config.web_auth_enabled = true;

    cJSON *json = call_handler(handle_post_fleet_camera_query, "{}",
                               api_key, 200);
    TEST_ASSERT_EQUAL_INT(1,
        cJSON_GetObjectItemCaseSensitive(json, "total")->valueint);
    cJSON *cameras = cJSON_GetObjectItemCaseSensitive(json, "cameras");
    TEST_ASSERT_EQUAL_STRING(
        "Outside", cJSON_GetObjectItemCaseSensitive(
                       cJSON_GetArrayItem(cameras, 0), "name")->valuestring);
    cJSON_Delete(json);

    json = call_handler(handle_post_fleet_camera_query, "{}", NULL, 401);
    cJSON_Delete(json);
}

void test_rejects_malformed_selector_and_oversized_page(void) {
    cJSON *json = call_handler(
        handle_post_fleet_camera_query,
        "{\"selector\":{\"version\":1,\"expression\":{\"op\":\"sql\"}}}",
        NULL, 400);
    TEST_ASSERT_NOT_NULL(cJSON_GetObjectItemCaseSensitive(json, "error"));
    cJSON_Delete(json);
    json = call_handler(handle_post_fleet_camera_query,
                        "{\"page_size\":201}", NULL, 400);
    TEST_ASSERT_NOT_NULL(cJSON_GetObjectItemCaseSensitive(json, "error"));
    cJSON_Delete(json);
}

void test_thousand_camera_fixture_returns_only_requested_page(void) {
    camera_location_t root;
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_get_unassigned(&root));
    sqlite3 *db = get_db_handle();
    TEST_ASSERT_EQUAL_INT(SQLITE_OK,
                          sqlite3_exec(db, "BEGIN;", NULL, NULL, NULL));
    sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_prepare_v2(
            db,
            "INSERT INTO streams "
            "(name, url, enabled, record, camera_uuid, location_uuid) "
            "VALUES (?, ?, 1, 1, ?, ?);",
            -1, &stmt, NULL));
    for (int i = 0; i < 1000; i++) {
        char name[64];
        char url[128];
        char uuid[CAMERA_UUID_STRING_SIZE];
        snprintf(name, sizeof(name), "Fleet Camera %04d", i);
        snprintf(url, sizeof(url), "rtsp://10.20.%d.%d/live",
                 (i / 250) + 1, (i % 250) + 1);
        snprintf(uuid, sizeof(uuid), "10000000-0000-4000-8000-%012d", i);
        sqlite3_bind_text(stmt, 1, name, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 2, url, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 3, uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 4, root.uuid, -1, SQLITE_TRANSIENT);
        TEST_ASSERT_EQUAL_INT(SQLITE_DONE, sqlite3_step(stmt));
        sqlite3_reset(stmt);
        sqlite3_clear_bindings(stmt);
    }
    sqlite3_finalize(stmt);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK,
                          sqlite3_exec(db, "COMMIT;", NULL, NULL, NULL));

    cJSON *json = call_handler(
        handle_post_fleet_camera_query,
        "{\"page\":10,\"page_size\":25,\"facets\":false,"
        "\"sort_by\":\"camera_uuid\"}",
        NULL, 200);
    TEST_ASSERT_EQUAL_INT(1000,
        cJSON_GetObjectItemCaseSensitive(json, "total")->valueint);
    TEST_ASSERT_EQUAL_INT(40,
        cJSON_GetObjectItemCaseSensitive(json, "total_pages")->valueint);
    TEST_ASSERT_EQUAL_INT(25, cJSON_GetArraySize(
        cJSON_GetObjectItemCaseSensitive(json, "cameras")));
    TEST_ASSERT_NULL(cJSON_GetObjectItemCaseSensitive(json, "facets"));
    cJSON_Delete(json);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_inventory_loads_hierarchy_tags_and_redacts_credentials);
    RUN_TEST(test_query_composes_selector_search_sort_pagination_and_facets);
    RUN_TEST(test_preview_returns_bounded_match_explanation);
    RUN_TEST(test_existing_tag_rbac_is_applied_before_totals_and_facets);
    RUN_TEST(test_rejects_malformed_selector_and_oversized_page);
    RUN_TEST(test_thousand_camera_fixture_returns_only_requested_page);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
