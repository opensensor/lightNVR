/**
 * @file test_authorization.c
 * @brief Action catalog, compatibility policy, selector grants, and simulation.
 */

#define _POSIX_C_SOURCE 200809L

#include <cjson/cJSON.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <unistd.h>

#include "unity.h"
#include "core/authorization.h"
#include "core/config.h"
#include "database/db_auth.h"
#include "database/db_authorization.h"
#include "database/db_camera_tags.h"
#include "database/db_core.h"
#include "database/db_fleet_query.h"
#include "database/db_streams.h"
#include "utils/strings.h"
#include "web/api_handlers_authorization.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_authorization_test.db"
#define OPERATOR_ROLE_UUID "00000000-0000-4000-8000-000000000002"
#define ADMIN_ROLE_UUID "00000000-0000-4000-8000-000000000001"

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
    int count = db_camera_tag_count();
    TEST_ASSERT_GREATER_THAN(0, count);
    camera_tag_t *tags = calloc((size_t)count, sizeof(*tags));
    TEST_ASSERT_NOT_NULL(tags);
    TEST_ASSERT_EQUAL_INT(count, db_camera_tag_list(tags, count));
    camera_tag_t found;
    memset(&found, 0, sizeof(found));
    for (int i = 0; i < count; i++) {
        if (strcasecmp(tags[i].label, label) == 0) found = tags[i];
    }
    free(tags);
    TEST_ASSERT_TRUE(found.uuid[0] != '\0');
    return found;
}

static fleet_camera_t *load_camera(const char *camera_uuid,
                                   fleet_camera_t **inventory) {
    int count = 0;
    TEST_ASSERT_EQUAL_INT(0, db_fleet_camera_load(inventory, &count));
    for (int i = 0; i < count; i++) {
        if (strcmp((*inventory)[i].camera_uuid, camera_uuid) == 0) {
            return &(*inventory)[i];
        }
    }
    TEST_FAIL_MESSAGE("Camera missing from fleet inventory");
    return NULL;
}

static void insert_grant(const char *uuid, int64_t user_id,
                         const char *role_uuid, const char *scope_type,
                         const char *selector_json) {
    sqlite3 *db = get_db_handle();
    sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_prepare_v2(
            db, "INSERT INTO authz_grants "
                "(uuid,user_id,role_uuid,scope_type,selector_json) "
                "VALUES (?,?,?,?,?);", -1, &stmt, NULL));
    sqlite3_bind_text(stmt, 1, uuid, -1, SQLITE_TRANSIENT);
    sqlite3_bind_int64(stmt, 2, user_id);
    sqlite3_bind_text(stmt, 3, role_uuid, -1, SQLITE_TRANSIENT);
    sqlite3_bind_text(stmt, 4, scope_type, -1, SQLITE_TRANSIENT);
    if (selector_json) {
        sqlite3_bind_text(stmt, 5, selector_json, -1, SQLITE_TRANSIENT);
    } else {
        sqlite3_bind_null(stmt, 5);
    }
    TEST_ASSERT_EQUAL_INT(SQLITE_DONE, sqlite3_step(stmt));
    sqlite3_finalize(stmt);
}

static void create_grant(int64_t user_id, const char *role_uuid,
                         const char *scope_type, const char *selector_json,
                         char grant_uuid[CAMERA_UUID_STRING_SIZE]) {
    TEST_ASSERT_EQUAL_INT(
        0, db_authorization_create_user_grant(user_id, role_uuid, scope_type,
                                              selector_json, grant_uuid));
    TEST_ASSERT_EQUAL_UINT(36, strlen(grant_uuid));
}

static cJSON *call_handler(
    void (*handler)(const http_request_t *, http_response_t *),
    http_method_t method, const char *body, const char *api_key,
    int expected_status) {
    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    req.method = method;
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

void setUp(void) {
    sqlite3 *db = get_db_handle();
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
    sqlite3_exec(db, "DELETE FROM authz_grants;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM streams;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM camera_tags;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM users WHERE username != 'admin';",
                 NULL, NULL, NULL);
}

void tearDown(void) {
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
}

void test_action_catalog_is_stable_and_complete(void) {
    int count = 0;
    const authorization_action_metadata_t *catalog =
        authorization_action_catalog(&count);
    TEST_ASSERT_EQUAL_INT(15, count);
    TEST_ASSERT_EQUAL_STRING("live.view", catalog[0].key);
    TEST_ASSERT_EQUAL_STRING("system.admin", catalog[count - 1].key);
    for (int i = 0; i < count; i++) {
        TEST_ASSERT_EQUAL_INT(
            catalog[i].action,
            authorization_action_from_key(catalog[i].key));
        TEST_ASSERT_NOT_NULL(authorization_action_metadata(catalog[i].action));
    }
    TEST_ASSERT_EQUAL_INT(AUTHZ_ACTION_INVALID,
                          authorization_action_from_key("unknown.action"));

    sqlite3 *db = get_db_handle();
    sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_prepare_v2(
            db, "SELECT category,description,camera_scoped,destructive "
                "FROM authz_actions WHERE action_key = ?;", -1, &stmt, NULL));
    for (int i = 0; i < count; i++) {
        sqlite3_reset(stmt);
        sqlite3_clear_bindings(stmt);
        sqlite3_bind_text(stmt, 1, catalog[i].key, -1, SQLITE_TRANSIENT);
        TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(stmt));
        TEST_ASSERT_EQUAL_STRING(
            catalog[i].category, (const char *)sqlite3_column_text(stmt, 0));
        TEST_ASSERT_EQUAL_STRING(
            catalog[i].description, (const char *)sqlite3_column_text(stmt, 1));
        TEST_ASSERT_EQUAL_INT(catalog[i].camera_scoped ? 1 : 0,
                              sqlite3_column_int(stmt, 2));
        TEST_ASSERT_EQUAL_INT(catalog[i].destructive ? 1 : 0,
                              sqlite3_column_int(stmt, 3));
    }
    sqlite3_finalize(stmt);
}

void test_legacy_roles_preserve_access_and_allowed_tags(void) {
    stream_config_t outside = create_camera("Outside", "Outdoor");
    stream_config_t inside = create_camera("Inside", "Indoor");
    int64_t viewer_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("legacyviewer", "password123", NULL,
                               USER_ROLE_VIEWER, true, &viewer_id));
    TEST_ASSERT_EQUAL_INT(0,
                          db_auth_set_allowed_tags(viewer_id, "Outdoor"));
    user_t viewer;
    TEST_ASSERT_EQUAL_INT(0, db_auth_get_user_by_id(viewer_id, &viewer));
    TEST_ASSERT_EQUAL_STRING("legacy", viewer.authorization_mode);

    fleet_camera_t *outside_inventory = NULL;
    fleet_camera_t *outside_camera =
        load_camera(outside.camera_uuid, &outside_inventory);
    fleet_camera_t *inside_inventory = NULL;
    fleet_camera_t *inside_camera =
        load_camera(inside.camera_uuid, &inside_inventory);
    authorization_evaluation_t evaluation;
    TEST_ASSERT_EQUAL_INT(
        0, authorization_evaluate(&viewer, AUTHZ_LIVE_VIEW, outside_camera,
                                  &evaluation));
    TEST_ASSERT_EQUAL_INT(AUTHZ_DECISION_ALLOW, evaluation.decision);
    TEST_ASSERT_EQUAL_INT(AUTHZ_SOURCE_LEGACY_ROLE, evaluation.source);
    TEST_ASSERT_EQUAL_INT(
        0, authorization_evaluate(&viewer, AUTHZ_LIVE_VIEW, inside_camera,
                                  &evaluation));
    TEST_ASSERT_EQUAL_INT(AUTHZ_DECISION_DENY, evaluation.decision);
    TEST_ASSERT_EQUAL_INT(
        0, authorization_evaluate(&viewer, AUTHZ_PTZ_CONTROL, outside_camera,
                                  &evaluation));
    TEST_ASSERT_EQUAL_INT(AUTHZ_DECISION_DENY, evaluation.decision);
    free(outside_inventory);
    free(inside_inventory);
}

void test_policy_mode_defaults_deny_and_matches_selector_grant(void) {
    stream_config_t outside = create_camera("Policy Outside", "Outdoor");
    stream_config_t inside = create_camera("Policy Inside", "Indoor");
    camera_tag_t outdoor_tag = find_tag("Outdoor");
    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("policyuser", "password123", NULL,
                               USER_ROLE_USER, true, &user_id));
    TEST_ASSERT_EQUAL_INT(0,
                          db_authorization_set_user_mode(user_id, "policy"));
    user_t user;
    TEST_ASSERT_EQUAL_INT(0, db_auth_get_user_by_id(user_id, &user));

    fleet_camera_t *outside_inventory = NULL;
    fleet_camera_t *outside_camera =
        load_camera(outside.camera_uuid, &outside_inventory);
    authorization_evaluation_t evaluation;
    TEST_ASSERT_EQUAL_INT(
        0, authorization_evaluate(&user, AUTHZ_LIVE_VIEW, outside_camera,
                                  &evaluation));
    TEST_ASSERT_EQUAL_INT(AUTHZ_DECISION_DENY, evaluation.decision);

    char selector[512];
    snprintf(selector, sizeof(selector),
             "{\"version\":1,\"expression\":{\"op\":\"tag_any\","
             "\"uuids\":[\"%s\"]}}",
             outdoor_tag.uuid);
    char grant_uuid[CAMERA_UUID_STRING_SIZE];
    create_grant(user_id, OPERATOR_ROLE_UUID, "selector", selector,
                 grant_uuid);

    TEST_ASSERT_EQUAL_INT(
        0, authorization_evaluate(&user, AUTHZ_LIVE_VIEW, outside_camera,
                                  &evaluation));
    TEST_ASSERT_EQUAL_INT(AUTHZ_DECISION_ALLOW, evaluation.decision);
    TEST_ASSERT_EQUAL_INT(AUTHZ_SOURCE_POLICY_GRANT, evaluation.source);
    TEST_ASSERT_EQUAL_STRING("Operator", evaluation.role_name);

    fleet_camera_t *inside_inventory = NULL;
    fleet_camera_t *inside_camera =
        load_camera(inside.camera_uuid, &inside_inventory);
    TEST_ASSERT_EQUAL_INT(
        0, authorization_evaluate(&user, AUTHZ_LIVE_VIEW, inside_camera,
                                  &evaluation));
    TEST_ASSERT_EQUAL_INT(AUTHZ_DECISION_DENY, evaluation.decision);
    TEST_ASSERT_EQUAL_INT(
        0, authorization_evaluate(&user, AUTHZ_USERS_MANAGE, NULL,
                                  &evaluation));
    TEST_ASSERT_EQUAL_INT(AUTHZ_DECISION_DENY, evaluation.decision);
    free(outside_inventory);
    free(inside_inventory);
}

void test_all_scope_admin_grant_allows_global_action_and_bumps_version(void) {
    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("policyadmin", "password123", NULL,
                               USER_ROLE_USER, true, &user_id));
    TEST_ASSERT_EQUAL_INT(0,
                          db_authorization_set_user_mode(user_id, "policy"));
    int64_t before = 0;
    TEST_ASSERT_EQUAL_INT(0, db_authorization_get_policy_version(&before));
    char grant_uuid[CAMERA_UUID_STRING_SIZE];
    create_grant(user_id, ADMIN_ROLE_UUID, "all", NULL, grant_uuid);
    int64_t after = 0;
    TEST_ASSERT_EQUAL_INT(0, db_authorization_get_policy_version(&after));
    TEST_ASSERT_EQUAL_INT64(before + 1, after);

    user_t user;
    TEST_ASSERT_EQUAL_INT(0, db_auth_get_user_by_id(user_id, &user));
    authorization_evaluation_t evaluation;
    TEST_ASSERT_EQUAL_INT(
        0, authorization_evaluate(&user, AUTHZ_USERS_MANAGE, NULL,
                                  &evaluation));
    TEST_ASSERT_EQUAL_INT(AUTHZ_DECISION_ALLOW, evaluation.decision);
    TEST_ASSERT_EQUAL_STRING(grant_uuid, evaluation.grant_uuid);

    char api_key[128] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_generate_api_key(user_id, api_key, sizeof(api_key)));
    g_config.web_auth_enabled = true;
    cJSON *json = call_handler(handle_get_authorization_actions,
                               HTTP_METHOD_GET, NULL, api_key, 200);
    TEST_ASSERT_EQUAL_INT(
        15, cJSON_GetObjectItemCaseSensitive(json, "count")->valueint);
    cJSON_Delete(json);
    g_config.web_auth_enabled = false;
}

void test_invalid_stored_selector_fails_closed(void) {
    stream_config_t camera_config = create_camera("Invalid Policy", "Outdoor");
    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("invalidpolicy", "password123", NULL,
                               USER_ROLE_USER, true, &user_id));
    TEST_ASSERT_EQUAL_INT(0,
                          db_authorization_set_user_mode(user_id, "policy"));
    insert_grant("10000000-0000-4000-8000-000000000003", user_id,
                 OPERATOR_ROLE_UUID, "selector", "{\"invalid\":true}");
    user_t user;
    TEST_ASSERT_EQUAL_INT(0, db_auth_get_user_by_id(user_id, &user));
    fleet_camera_t *inventory = NULL;
    fleet_camera_t *camera = load_camera(camera_config.camera_uuid, &inventory);
    authorization_evaluation_t evaluation;
    TEST_ASSERT_EQUAL_INT(
        -1, authorization_evaluate(&user, AUTHZ_LIVE_VIEW, camera,
                                   &evaluation));
    TEST_ASSERT_EQUAL_INT(AUTHZ_DECISION_DENY, evaluation.decision);
    free(inventory);
}

void test_action_catalog_and_simulation_handlers(void) {
    stream_config_t camera = create_camera("Simulation", "Outdoor");
    int64_t viewer_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("simulationviewer", "password123", NULL,
                               USER_ROLE_VIEWER, true, &viewer_id));

    cJSON *json = call_handler(handle_get_authorization_actions,
                               HTTP_METHOD_GET, NULL, NULL, 200);
    TEST_ASSERT_EQUAL_INT(
        15, cJSON_GetObjectItemCaseSensitive(json, "count")->valueint);
    cJSON_Delete(json);

    char request_body[512];
    snprintf(request_body, sizeof(request_body),
             "{\"user_id\":%lld,\"action\":\"live.view\","
             "\"camera_uuid\":\"%s\"}",
             (long long)viewer_id, camera.camera_uuid);
    json = call_handler(handle_post_authorization_simulate,
                        HTTP_METHOD_POST, request_body, NULL, 200);
    TEST_ASSERT_TRUE(cJSON_IsTrue(
        cJSON_GetObjectItemCaseSensitive(json, "allowed")));
    TEST_ASSERT_EQUAL_STRING(
        "legacy_role",
        cJSON_GetObjectItemCaseSensitive(json, "source")->valuestring);
    cJSON_Delete(json);

    char api_key[128] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_generate_api_key(viewer_id, api_key, sizeof(api_key)));
    g_config.web_auth_enabled = true;
    json = call_handler(handle_post_authorization_simulate,
                        HTTP_METHOD_POST, request_body, api_key, 403);
    cJSON_Delete(json);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    if (db_auth_init() != 0) {
        fprintf(stderr, "FATAL: db_auth_init failed\n");
        shutdown_database();
        unlink(TEST_DB_PATH);
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_action_catalog_is_stable_and_complete);
    RUN_TEST(test_legacy_roles_preserve_access_and_allowed_tags);
    RUN_TEST(test_policy_mode_defaults_deny_and_matches_selector_grant);
    RUN_TEST(test_all_scope_admin_grant_allows_global_action_and_bumps_version);
    RUN_TEST(test_invalid_stored_selector_fails_closed);
    RUN_TEST(test_action_catalog_and_simulation_handlers);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
