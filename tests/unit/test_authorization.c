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
#include <time.h>
#include <unistd.h>

#include "unity.h"
#include "core/authorization.h"
#include "core/config.h"
#include "database/db_auth.h"
#include "database/db_api_tokens.h"
#include "database/db_authorization.h"
#include "database/db_camera_collections.h"
#include "database/db_camera_tags.h"
#include "database/db_core.h"
#include "database/db_fleet_query.h"
#include "database/db_recordings.h"
#include "database/db_streams.h"
#include "utils/strings.h"
#include "web/api_handlers.h"
#include "web/api_handlers_authorization.h"
#include "web/api_handlers_ptz.h"
#include "web/api_handlers_recordings.h"
#include "web/api_handlers_recordings_batch_download.h"
#include "web/api_handlers_recordings_download.h"
#include "web/api_handlers_users.h"
#include "web/httpd_utils.h"
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
                                              selector_json, NULL,
                                              grant_uuid));
    TEST_ASSERT_EQUAL_UINT(36, strlen(grant_uuid));
}

static cJSON *call_handler_path(
    void (*handler)(const http_request_t *, http_response_t *),
    http_method_t method, const char *path, const char *body,
    const char *api_key, int expected_status) {
    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    req.method = method;
    if (path) safe_strcpy(req.path, path, sizeof(req.path), 0);
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

static cJSON *call_handler(
    void (*handler)(const http_request_t *, http_response_t *),
    http_method_t method, const char *body, const char *api_key,
    int expected_status) {
    return call_handler_path(handler, method, NULL, body, api_key,
                             expected_status);
}

static int authorize_stream_with_key(const char *api_key,
                                     authorization_action_t action,
                                     const char *stream_name,
                                     int expected_status) {
    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    safe_strcpy(req.client_ip, "127.0.0.1", sizeof(req.client_ip), 0);
    safe_strcpy(req.headers[0].name, "X-API-Key",
                sizeof(req.headers[0].name), 0);
    safe_strcpy(req.headers[0].value, api_key,
                sizeof(req.headers[0].value), 0);
    req.num_headers = 1;
    int allowed =
        httpd_authorize_stream_action(&req, &res, action, stream_name);
    TEST_ASSERT_EQUAL_INT(expected_status, res.status_code);
    http_response_free(&res);
    return allowed;
}

void setUp(void) {
    sqlite3 *db = get_db_handle();
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
    sqlite3_exec(db, "DELETE FROM authz_grants;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM authz_roles WHERE is_builtin=0;", NULL, NULL,
                 NULL);
    sqlite3_exec(db, "DELETE FROM recordings;", NULL, NULL, NULL);
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

void test_shared_collection_grants_track_membership_and_guard_scope(void) {
    stream_config_t first = create_camera("Collection North", "North");
    stream_config_t second = create_camera("Collection South", "South");
    camera_collection_t collection;
    memset(&collection, 0, sizeof(collection));
    safe_strcpy(collection.name, "North operators", sizeof(collection.name), 0);
    safe_strcpy(collection.collection_type, "static",
                sizeof(collection.collection_type), 0);
    collection.is_shared = true;
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_OK,
                          db_camera_collection_create(&collection));
    const char *first_members[] = {first.camera_uuid};
    TEST_ASSERT_EQUAL_INT(
        DB_CAMERA_COLLECTION_OK,
        db_camera_collection_set_members(collection.uuid, first_members, 1));

    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("collectionoperator", "password123", NULL,
                               USER_ROLE_USER, true, &user_id));
    int64_t version = 0;
    TEST_ASSERT_EQUAL_INT(0, db_authorization_get_policy_version(&version));
    char path[128];
    snprintf(path, sizeof(path), "/api/authorization/users/%lld",
             (long long)user_id);
    char body[768];
    snprintf(body, sizeof(body),
             "{\"expected_policy_version\":%lld,\"mode\":\"policy\","
             "\"grants\":[{\"role_uuid\":\"%s\",\"scope\":{"
             "\"type\":\"collection\",\"collection_uuid\":\"%s\"}}]}",
             (long long)version, OPERATOR_ROLE_UUID, collection.uuid);
    cJSON *json = call_handler_path(
        handle_put_user_authorization, HTTP_METHOD_PUT, path, body, NULL, 200);
    int64_t grant_version = (int64_t)cJSON_GetObjectItemCaseSensitive(
        json, "policy_version")->valuedouble;
    cJSON_Delete(json);

    user_t user;
    TEST_ASSERT_EQUAL_INT(0, db_auth_get_user_by_id(user_id, &user));
    fleet_camera_t *inventory = NULL;
    fleet_camera_t *first_camera = load_camera(first.camera_uuid, &inventory);
    authorization_evaluation_t evaluation;
    TEST_ASSERT_EQUAL_INT(
        0, authorization_evaluate(&user, AUTHZ_LIVE_VIEW, first_camera,
                                  &evaluation));
    TEST_ASSERT_EQUAL_INT(AUTHZ_DECISION_ALLOW, evaluation.decision);
    free(inventory);
    inventory = NULL;
    fleet_camera_t *second_camera = load_camera(second.camera_uuid, &inventory);
    TEST_ASSERT_EQUAL_INT(
        0, authorization_evaluate(&user, AUTHZ_LIVE_VIEW, second_camera,
                                  &evaluation));
    TEST_ASSERT_EQUAL_INT(AUTHZ_DECISION_DENY, evaluation.decision);
    free(inventory);

    const char *second_members[] = {second.camera_uuid};
    TEST_ASSERT_EQUAL_INT(
        DB_CAMERA_COLLECTION_OK,
        db_camera_collection_set_members(collection.uuid, second_members, 1));
    int64_t membership_version = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_authorization_get_policy_version(&membership_version));
    TEST_ASSERT_EQUAL_INT64(grant_version + 1, membership_version);

    inventory = NULL;
    first_camera = load_camera(first.camera_uuid, &inventory);
    TEST_ASSERT_EQUAL_INT(
        0, authorization_evaluate(&user, AUTHZ_LIVE_VIEW, first_camera,
                                  &evaluation));
    TEST_ASSERT_EQUAL_INT(AUTHZ_DECISION_DENY, evaluation.decision);
    free(inventory);
    inventory = NULL;
    second_camera = load_camera(second.camera_uuid, &inventory);
    TEST_ASSERT_EQUAL_INT(
        0, authorization_evaluate(&user, AUTHZ_LIVE_VIEW, second_camera,
                                  &evaluation));
    TEST_ASSERT_EQUAL_INT(AUTHZ_DECISION_ALLOW, evaluation.decision);
    free(inventory);

    json = call_handler_path(
        handle_get_user_authorization, HTTP_METHOD_GET, path, NULL, NULL, 200);
    cJSON *scope = cJSON_GetObjectItemCaseSensitive(
        cJSON_GetArrayItem(
            cJSON_GetObjectItemCaseSensitive(json, "grants"), 0), "scope");
    TEST_ASSERT_EQUAL_STRING(
        "collection",
        cJSON_GetObjectItemCaseSensitive(scope, "type")->valuestring);
    TEST_ASSERT_EQUAL_STRING(
        collection.uuid,
        cJSON_GetObjectItemCaseSensitive(scope,
                                         "collection_uuid")->valuestring);
    cJSON_Delete(json);

    collection.is_shared = false;
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_CONFLICT,
                          db_camera_collection_update(&collection));
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_CONFLICT,
                          db_camera_collection_delete(collection.uuid));

    int64_t cleared_version = 0;
    TEST_ASSERT_EQUAL_INT(
        DB_AUTHORIZATION_OK,
        db_authorization_replace_user_policy(user_id, "legacy", NULL, 0,
                                             membership_version,
                                             &cleared_version));
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_OK,
                          db_camera_collection_delete(collection.uuid));

    camera_collection_t private_collection;
    memset(&private_collection, 0, sizeof(private_collection));
    safe_strcpy(private_collection.name, "Personal camera view",
                sizeof(private_collection.name), 0);
    safe_strcpy(private_collection.collection_type, "static",
                sizeof(private_collection.collection_type), 0);
    private_collection.is_shared = false;
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_OK,
                          db_camera_collection_create(&private_collection));
    snprintf(body, sizeof(body),
             "{\"expected_policy_version\":%lld,\"mode\":\"policy\","
             "\"grants\":[{\"role_uuid\":\"%s\",\"scope\":{"
             "\"type\":\"collection\",\"collection_uuid\":\"%s\"}}]}",
             (long long)cleared_version, OPERATOR_ROLE_UUID,
             private_collection.uuid);
    json = call_handler_path(handle_put_user_authorization, HTTP_METHOD_PUT,
                             path, body, NULL, 400);
    cJSON_Delete(json);
}

void test_scoped_api_token_intersects_user_policy_and_revokes(void) {
    stream_config_t allowed = create_camera("Token Allowed", "Token");
    stream_config_t denied = create_camera("Token Denied", "Token");
    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("tokenoperator", "password123", NULL,
                               USER_ROLE_USER, true, &user_id));
    TEST_ASSERT_EQUAL_INT(0,
                          db_authorization_set_user_mode(user_id, "policy"));
    char grant_uuid[CAMERA_UUID_STRING_SIZE];
    create_grant(user_id, OPERATOR_ROLE_UUID, "all", NULL, grant_uuid);

    int64_t expiry = (int64_t)time(NULL) + 3600;
    char path[128];
    snprintf(path, sizeof(path), "/api/authorization/users/%lld/tokens",
             (long long)user_id);
    char body[1536];
    snprintf(body, sizeof(body),
             "{\"description\":\"North PTZ integration\","
             "\"expires_at\":%lld,\"actions\":[\"ptz.control\"],"
             "\"scope\":{\"type\":\"selector\",\"selector\":{"
             "\"version\":1,\"expression\":{\"op\":\"camera_uuid\","
             "\"values\":[\"%s\"]}}}}",
             (long long)expiry, allowed.camera_uuid);
    cJSON *json = call_handler_path(
        handle_post_user_api_token, HTTP_METHOD_POST, path, body, NULL, 201);
    cJSON *secret_item = cJSON_GetObjectItemCaseSensitive(json, "secret");
    cJSON *created = cJSON_GetObjectItemCaseSensitive(json, "token");
    TEST_ASSERT_TRUE(cJSON_IsString(secret_item));
    TEST_ASSERT_TRUE(strncmp(secret_item->valuestring, "lnvr_", 5) == 0);
    char secret[API_TOKEN_SECRET_MAX];
    char token_uuid[CAMERA_UUID_STRING_SIZE];
    safe_strcpy(secret, secret_item->valuestring, sizeof(secret), 0);
    safe_strcpy(token_uuid,
                cJSON_GetObjectItemCaseSensitive(created, "uuid")->valuestring,
                sizeof(token_uuid), 0);
    cJSON_Delete(json);

    sqlite3 *db = get_db_handle();
    sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_prepare_v2(
            db, "SELECT token_hash FROM authz_api_tokens WHERE uuid=?;", -1,
            &stmt, NULL));
    sqlite3_bind_text(stmt, 1, token_uuid, -1, SQLITE_TRANSIENT);
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(stmt));
    const char *stored_hash = (const char *)sqlite3_column_text(stmt, 0);
    TEST_ASSERT_NOT_NULL(stored_hash);
    TEST_ASSERT_EQUAL_UINT(64, strlen(stored_hash));
    TEST_ASSERT_NOT_EQUAL(0, strcmp(secret, stored_hash));
    sqlite3_finalize(stmt);

    json = call_handler_path(handle_get_user_api_tokens, HTTP_METHOD_GET,
                             path, NULL, NULL, 200);
    TEST_ASSERT_EQUAL_INT(
        1, cJSON_GetObjectItemCaseSensitive(json, "count")->valueint);
    TEST_ASSERT_NULL(cJSON_GetObjectItemCaseSensitive(
        cJSON_GetArrayItem(cJSON_GetObjectItemCaseSensitive(json, "tokens"), 0),
        "secret"));
    cJSON_Delete(json);

    g_config.web_auth_enabled = true;
    TEST_ASSERT_EQUAL_INT(
        1, authorize_stream_with_key(secret, AUTHZ_PTZ_CONTROL,
                                     allowed.name, 200));
    TEST_ASSERT_EQUAL_INT(
        0, authorize_stream_with_key(secret, AUTHZ_PTZ_CONTROL,
                                     denied.name, 403));
    TEST_ASSERT_EQUAL_INT(
        0, authorize_stream_with_key(secret, AUTHZ_EVIDENCE_PROTECT,
                                     allowed.name, 403));

    http_request_t request;
    http_request_init(&request);
    safe_strcpy(request.client_ip, "127.0.0.1", sizeof(request.client_ip), 0);
    safe_strcpy(request.headers[0].name, "X-API-Key",
                sizeof(request.headers[0].name), 0);
    safe_strcpy(request.headers[0].value, secret,
                sizeof(request.headers[0].value), 0);
    request.num_headers = 1;
    user_t authenticated;
    TEST_ASSERT_EQUAL_INT(
        0, httpd_get_authenticated_user(&request, &authenticated));

    g_config.web_auth_enabled = false;
    char revoke_path[192];
    snprintf(revoke_path, sizeof(revoke_path),
             "/api/authorization/users/%lld/tokens/%s",
             (long long)user_id, token_uuid);
    json = call_handler_path(handle_delete_user_api_token,
                             HTTP_METHOD_DELETE, revoke_path, NULL, NULL, 200);
    cJSON_Delete(json);
    g_config.web_auth_enabled = true;
    TEST_ASSERT_EQUAL_INT(
        0, authorize_stream_with_key(secret, AUTHZ_PTZ_CONTROL,
                                     allowed.name, 401));
    g_config.web_auth_enabled = false;

    snprintf(body, sizeof(body),
             "{\"description\":\"No expiry\","
             "\"actions\":[\"ptz.control\"],"
             "\"scope\":{\"type\":\"all\"}}");
    json = call_handler_path(handle_post_user_api_token, HTTP_METHOD_POST,
                             path, body, NULL, 400);
    cJSON_Delete(json);
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

void test_role_and_policy_database_mutations_are_atomic(void) {
    int64_t version = 0;
    TEST_ASSERT_EQUAL_INT(0, db_authorization_get_policy_version(&version));
    authorization_role_t role;
    memset(&role, 0, sizeof(role));
    safe_strcpy(role.name, "Evidence reviewer", sizeof(role.name), 0);
    safe_strcpy(role.description, "Reviews without export",
                sizeof(role.description), 0);
    role.action_mask = (UINT64_C(1) << AUTHZ_LIVE_VIEW) |
                       (UINT64_C(1) << AUTHZ_RECORDINGS_REPLAY);
    int64_t next_version = 0;
    TEST_ASSERT_EQUAL_INT(
        DB_AUTHORIZATION_OK,
        db_authorization_role_create(&role, version, &next_version));
    TEST_ASSERT_EQUAL_INT64(version + 1, next_version);
    TEST_ASSERT_EQUAL_UINT(36, strlen(role.uuid));
    TEST_ASSERT_EQUAL_INT(5, db_authorization_role_count());

    authorization_role_t duplicate = role;
    duplicate.uuid[0] = '\0';
    int64_t ignored_version = 0;
    TEST_ASSERT_EQUAL_INT(
        DB_AUTHORIZATION_CONFLICT,
        db_authorization_role_create(&duplicate, next_version,
                                     &ignored_version));
    TEST_ASSERT_EQUAL_INT(0,
                          db_authorization_get_policy_version(&ignored_version));
    TEST_ASSERT_EQUAL_INT64(next_version, ignored_version);

    authorization_role_t loaded;
    TEST_ASSERT_EQUAL_INT(DB_AUTHORIZATION_OK,
                          db_authorization_role_get(role.uuid, &loaded));
    TEST_ASSERT_EQUAL_STRING(role.name, loaded.name);
    TEST_ASSERT_EQUAL_UINT64(role.action_mask, loaded.action_mask);
    safe_strcpy(role.name, "Evidence auditor", sizeof(role.name), 0);
    TEST_ASSERT_EQUAL_INT(
        DB_AUTHORIZATION_STALE,
        db_authorization_role_update(&role, version, &version));
    TEST_ASSERT_EQUAL_INT(
        DB_AUTHORIZATION_OK,
        db_authorization_role_update(&role, next_version, &version));
    TEST_ASSERT_EQUAL_INT64(next_version + 1, version);

    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("atomicpolicy", "password123", NULL,
                               USER_ROLE_VIEWER, true, &user_id));
    authorization_grant_input_t grant;
    memset(&grant, 0, sizeof(grant));
    safe_strcpy(grant.role_uuid, role.uuid, sizeof(grant.role_uuid), 0);
    safe_strcpy(grant.scope_type, "all", sizeof(grant.scope_type), 0);
    int64_t grants_version = 0;
    int64_t read_version = 0;
    authorization_grant_input_t invalid_grant = grant;
    safe_strcpy(invalid_grant.scope_type, "selector",
                sizeof(invalid_grant.scope_type), 0);
    safe_strcpy(invalid_grant.selector_json, "{\"invalid\":true}",
                sizeof(invalid_grant.selector_json), 0);
    TEST_ASSERT_EQUAL_INT(
        DB_AUTHORIZATION_INVALID,
        db_authorization_replace_user_policy(user_id, "policy",
                                             &invalid_grant, 1, version,
                                             &grants_version));
    TEST_ASSERT_EQUAL_INT(0,
                          db_authorization_get_policy_version(&read_version));
    TEST_ASSERT_EQUAL_INT64(version, read_version);
    authorization_grant_input_t duplicate_grants[2] = {grant, grant};
    TEST_ASSERT_EQUAL_INT(
        DB_AUTHORIZATION_INVALID,
        db_authorization_replace_user_policy(user_id, "policy",
                                             duplicate_grants, 2, version,
                                             &grants_version));
    TEST_ASSERT_EQUAL_INT(
        DB_AUTHORIZATION_OK,
        db_authorization_replace_user_policy(user_id, "policy", &grant, 1,
                                             version, &grants_version));
    TEST_ASSERT_EQUAL_INT64(version + 1, grants_version);

    char mode[USER_AUTHORIZATION_MODE_MAX];
    authorization_grant_t *grants = NULL;
    int grant_count = 0;
    read_version = 0;
    TEST_ASSERT_EQUAL_INT(
        DB_AUTHORIZATION_OK,
        db_authorization_get_user_policy(user_id, mode, &grants,
                                         &grant_count, &read_version));
    TEST_ASSERT_EQUAL_STRING("policy", mode);
    TEST_ASSERT_EQUAL_INT(1, grant_count);
    TEST_ASSERT_EQUAL_STRING(role.uuid, grants[0].role_uuid);
    TEST_ASSERT_EQUAL_STRING("all", grants[0].scope_type);
    TEST_ASSERT_EQUAL_INT64(grants_version, read_version);
    free(grants);

    ignored_version = 0;
    TEST_ASSERT_EQUAL_INT(
        DB_AUTHORIZATION_STALE,
        db_authorization_replace_user_policy(user_id, "legacy", NULL, 0,
                                             version, &ignored_version));
    TEST_ASSERT_EQUAL_INT(
        DB_AUTHORIZATION_IN_USE,
        db_authorization_role_delete(role.uuid, grants_version,
                                     &ignored_version));
    TEST_ASSERT_EQUAL_INT(0, db_authorization_get_policy_version(&read_version));
    TEST_ASSERT_EQUAL_INT64(grants_version, read_version);

    TEST_ASSERT_EQUAL_INT(
        DB_AUTHORIZATION_OK,
        db_authorization_replace_user_policy(user_id, "legacy", NULL, 0,
                                             grants_version, &version));
    TEST_ASSERT_EQUAL_INT(
        DB_AUTHORIZATION_OK,
        db_authorization_role_delete(role.uuid, version, &next_version));
    TEST_ASSERT_EQUAL_INT64(version + 1, next_version);
    TEST_ASSERT_EQUAL_INT(
        DB_AUTHORIZATION_IMMUTABLE,
        db_authorization_role_delete(ADMIN_ROLE_UUID, next_version,
                                     &ignored_version));
    TEST_ASSERT_EQUAL_INT(4, db_authorization_role_count());
}

void test_policy_management_handlers_and_conflict_guards(void) {
    int64_t version = 0;
    TEST_ASSERT_EQUAL_INT(0, db_authorization_get_policy_version(&version));
    cJSON *json = call_handler_path(
        handle_get_authorization_roles, HTTP_METHOD_GET,
        "/api/authorization/roles", NULL, NULL, 200);
    TEST_ASSERT_EQUAL_INT(
        4, cJSON_GetObjectItemCaseSensitive(json, "count")->valueint);
    TEST_ASSERT_EQUAL_INT64(
        version,
        (int64_t)cJSON_GetObjectItemCaseSensitive(
            json, "policy_version")->valuedouble);
    cJSON_Delete(json);

    char body[1024];
    snprintf(body, sizeof(body),
             "{\"expected_policy_version\":%lld,"
             "\"name\":\"Live desk\",\"description\":\"Live only\","
             "\"actions\":[\"live.view\"]}",
             (long long)version);
    json = call_handler_path(
        handle_post_authorization_role, HTTP_METHOD_POST,
        "/api/authorization/roles", body, NULL, 201);
    cJSON *created = cJSON_GetObjectItemCaseSensitive(json, "role");
    char role_uuid[CAMERA_UUID_STRING_SIZE];
    safe_strcpy(
        role_uuid,
        cJSON_GetObjectItemCaseSensitive(created, "uuid")->valuestring,
        sizeof(role_uuid), 0);
    int64_t role_version = (int64_t)cJSON_GetObjectItemCaseSensitive(
        json, "policy_version")->valuedouble;
    TEST_ASSERT_EQUAL_INT64(version + 1, role_version);
    cJSON_Delete(json);

    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("managedpolicy", "password123", NULL,
                               USER_ROLE_VIEWER, true, &user_id));
    char user_path[128];
    snprintf(user_path, sizeof(user_path), "/api/authorization/users/%lld",
             (long long)user_id);
    snprintf(body, sizeof(body),
             "{\"expected_policy_version\":%lld,\"mode\":\"policy\","
             "\"grants\":[{\"role_uuid\":\"%s\","
             "\"scope\":{\"type\":\"all\"}}]}",
             (long long)role_version, role_uuid);
    json = call_handler_path(handle_put_user_authorization, HTTP_METHOD_PUT,
                             user_path, body, NULL, 200);
    TEST_ASSERT_EQUAL_STRING(
        "policy", cJSON_GetObjectItemCaseSensitive(json, "mode")->valuestring);
    TEST_ASSERT_EQUAL_INT(
        1, cJSON_GetObjectItemCaseSensitive(json, "grant_count")->valueint);
    int64_t policy_version = (int64_t)cJSON_GetObjectItemCaseSensitive(
        json, "policy_version")->valuedouble;
    cJSON_Delete(json);

    json = call_handler_path(handle_get_user_authorization, HTTP_METHOD_GET,
                             user_path, NULL, NULL, 200);
    cJSON *scope = cJSON_GetObjectItemCaseSensitive(
        cJSON_GetArrayItem(
            cJSON_GetObjectItemCaseSensitive(json, "grants"), 0), "scope");
    TEST_ASSERT_EQUAL_STRING(
        "all", cJSON_GetObjectItemCaseSensitive(scope, "type")->valuestring);
    cJSON_Delete(json);

    json = call_handler_path(handle_put_user_authorization, HTTP_METHOD_PUT,
                             user_path, body, NULL, 409);
    cJSON_Delete(json);

    char role_path[160];
    snprintf(role_path, sizeof(role_path), "/api/authorization/roles/%s",
             role_uuid);
    snprintf(body, sizeof(body),
             "{\"expected_policy_version\":%lld}",
             (long long)policy_version);
    json = call_handler_path(handle_delete_authorization_role,
                             HTTP_METHOD_DELETE, role_path, body, NULL, 409);
    cJSON_Delete(json);

    char builtin_path[160];
    snprintf(builtin_path, sizeof(builtin_path),
             "/api/authorization/roles/%s", ADMIN_ROLE_UUID);
    snprintf(body, sizeof(body),
             "{\"expected_policy_version\":%lld,\"name\":\"Changed\","
             "\"actions\":[\"users.manage\"]}",
             (long long)policy_version);
    json = call_handler_path(handle_put_authorization_role, HTTP_METHOD_PUT,
                             builtin_path, body, NULL, 409);
    cJSON_Delete(json);

    user_t admin;
    TEST_ASSERT_EQUAL_INT(0, db_auth_get_user_by_username("admin", &admin));
    char self_path[128];
    snprintf(self_path, sizeof(self_path), "/api/authorization/users/%lld",
             (long long)admin.id);
    snprintf(body, sizeof(body),
             "{\"expected_policy_version\":%lld,\"mode\":\"policy\","
             "\"grants\":[{\"role_uuid\":\"%s\","
             "\"scope\":{\"type\":\"all\"}}]}",
             (long long)policy_version, OPERATOR_ROLE_UUID);
    char admin_api_key[128] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_generate_api_key(admin.id, admin_api_key,
                                    sizeof(admin_api_key)));
    g_config.web_auth_enabled = true;
    json = call_handler_path(handle_put_user_authorization, HTTP_METHOD_PUT,
                             self_path, body, admin_api_key, 409);
    cJSON_Delete(json);
    g_config.web_auth_enabled = false;

    int64_t viewer_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("policyreader", "password123", NULL,
                               USER_ROLE_VIEWER, true, &viewer_id));
    char api_key[128] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_generate_api_key(viewer_id, api_key, sizeof(api_key)));
    g_config.web_auth_enabled = true;
    json = call_handler_path(handle_get_authorization_roles, HTTP_METHOD_GET,
                             "/api/authorization/roles", NULL, api_key, 403);
    cJSON_Delete(json);
    g_config.web_auth_enabled = false;
}

void test_policy_role_update_cannot_lock_out_requester(void) {
    int64_t version = 0;
    TEST_ASSERT_EQUAL_INT(0, db_authorization_get_policy_version(&version));
    authorization_role_t role;
    memset(&role, 0, sizeof(role));
    safe_strcpy(role.name, "Policy manager", sizeof(role.name), 0);
    role.action_mask = UINT64_C(1) << AUTHZ_USERS_MANAGE;
    int64_t role_version = 0;
    TEST_ASSERT_EQUAL_INT(
        DB_AUTHORIZATION_OK,
        db_authorization_role_create(&role, version, &role_version));

    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("custommanager", "password123", NULL,
                               USER_ROLE_USER, true, &user_id));
    authorization_grant_input_t grant;
    memset(&grant, 0, sizeof(grant));
    safe_strcpy(grant.role_uuid, role.uuid, sizeof(grant.role_uuid), 0);
    safe_strcpy(grant.scope_type, "all", sizeof(grant.scope_type), 0);
    int64_t policy_version = 0;
    TEST_ASSERT_EQUAL_INT(
        DB_AUTHORIZATION_OK,
        db_authorization_replace_user_policy(user_id, "policy", &grant, 1,
                                             role_version, &policy_version));
    char api_key[128] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_generate_api_key(user_id, api_key, sizeof(api_key)));
    char path[160];
    snprintf(path, sizeof(path), "/api/authorization/roles/%s", role.uuid);
    char body[512];
    snprintf(body, sizeof(body),
             "{\"expected_policy_version\":%lld,"
             "\"name\":\"Policy manager\","
             "\"actions\":[\"live.view\"]}",
             (long long)policy_version);
    g_config.web_auth_enabled = true;
    cJSON *json = call_handler_path(
        handle_put_authorization_role, HTTP_METHOD_PUT, path, body, api_key,
        409);
    cJSON_Delete(json);
    g_config.web_auth_enabled = false;

    authorization_role_t unchanged;
    TEST_ASSERT_EQUAL_INT(DB_AUTHORIZATION_OK,
                          db_authorization_role_get(role.uuid, &unchanged));
    TEST_ASSERT_EQUAL_UINT64(UINT64_C(1) << AUTHZ_USERS_MANAGE,
                             unchanged.action_mask);
    int64_t unchanged_version = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_authorization_get_policy_version(&unchanged_version));
    TEST_ASSERT_EQUAL_INT64(policy_version, unchanged_version);
}

void test_users_api_reports_authorization_mode(void) {
    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("modebadge", "password123", NULL,
                               USER_ROLE_VIEWER, true, &user_id));
    TEST_ASSERT_EQUAL_INT(0,
                          db_authorization_set_user_mode(user_id, "policy"));
    cJSON *json = call_handler_path(handle_users_list, HTTP_METHOD_GET,
                                    "/api/auth/users", NULL, NULL, 200);
    cJSON *users = cJSON_GetObjectItemCaseSensitive(json, "users");
    cJSON *matched = NULL;
    cJSON *user = NULL;
    cJSON_ArrayForEach(user, users) {
        cJSON *username = cJSON_GetObjectItemCaseSensitive(user, "username");
        if (cJSON_IsString(username) &&
            strcmp(username->valuestring, "modebadge") == 0) {
            matched = user;
            break;
        }
    }
    TEST_ASSERT_NOT_NULL(matched);
    TEST_ASSERT_EQUAL_STRING(
        "policy",
        cJSON_GetObjectItemCaseSensitive(
            matched, "authorization_mode")->valuestring);
    cJSON_Delete(json);
}

void test_sensitive_handlers_enforce_camera_scoped_policy(void) {
    stream_config_t allowed = create_camera("Scoped Camera", "Outdoor");
    stream_config_t denied = create_camera("Other Camera", "Indoor");
    allowed.ptz_enabled = true;
    denied.ptz_enabled = true;
    TEST_ASSERT_EQUAL_INT(0, update_stream_config(allowed.name, &allowed));
    TEST_ASSERT_EQUAL_INT(0, update_stream_config(denied.name, &denied));

    fleet_camera_t resolved;
    TEST_ASSERT_EQUAL_INT(
        0, db_fleet_camera_find_by_name(allowed.name, &resolved));
    TEST_ASSERT_EQUAL_STRING(allowed.camera_uuid, resolved.camera_uuid);
    TEST_ASSERT_EQUAL_INT(
        1, db_fleet_camera_find_by_name("Missing Camera", &resolved));

    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("scopedoperator", "password123", NULL,
                               USER_ROLE_VIEWER, true, &user_id));
    TEST_ASSERT_EQUAL_INT(
        0, db_authorization_set_user_mode(user_id, "policy"));
    char selector[512];
    snprintf(selector, sizeof(selector),
             "{\"version\":1,\"expression\":{\"op\":\"camera_uuid\","
             "\"values\":[\"%s\"]}}",
             allowed.camera_uuid);
    char grant_uuid[CAMERA_UUID_STRING_SIZE];
    create_grant(user_id, OPERATOR_ROLE_UUID, "selector", selector,
                 grant_uuid);
    char api_key[128] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_generate_api_key(user_id, api_key, sizeof(api_key)));

    recording_metadata_t allowed_recording;
    memset(&allowed_recording, 0, sizeof(allowed_recording));
    safe_strcpy(allowed_recording.stream_name, allowed.name,
                sizeof(allowed_recording.stream_name), 0);
    safe_strcpy(allowed_recording.file_path, "/tmp/lightnvr-scoped-recording.mp4",
                sizeof(allowed_recording.file_path), 0);
    allowed_recording.start_time = 100;
    allowed_recording.end_time = 200;
    allowed_recording.is_complete = true;
    allowed_recording.retention_override_days = -1;
    uint64_t allowed_id = add_recording_metadata(&allowed_recording);
    TEST_ASSERT_NOT_EQUAL(0, allowed_id);

    recording_metadata_t denied_recording = allowed_recording;
    safe_strcpy(denied_recording.stream_name, denied.name,
                sizeof(denied_recording.stream_name), 0);
    safe_strcpy(denied_recording.file_path, "/tmp/lightnvr-denied-recording.mp4",
                sizeof(denied_recording.file_path), 0);
    uint64_t denied_id = add_recording_metadata(&denied_recording);
    TEST_ASSERT_NOT_EQUAL(0, denied_id);

    char path[128];
    snprintf(path, sizeof(path), "/api/recordings/%llu/protect",
             (unsigned long long)allowed_id);
    g_config.web_auth_enabled = true;
    cJSON *json = call_handler_path(
        handle_put_recording_protect, HTTP_METHOD_PUT, path,
        "{\"protected\":true}", api_key, 200);
    cJSON_Delete(json);
    recording_metadata_t reloaded;
    TEST_ASSERT_EQUAL_INT(
        0, get_recording_metadata_by_id(allowed_id, &reloaded));
    TEST_ASSERT_TRUE(reloaded.protected);

    snprintf(path, sizeof(path), "/api/recordings/%llu/protect",
             (unsigned long long)denied_id);
    json = call_handler_path(handle_put_recording_protect, HTTP_METHOD_PUT,
                             path, "{\"protected\":true}", api_key, 403);
    cJSON_Delete(json);
    TEST_ASSERT_EQUAL_INT(
        0, get_recording_metadata_by_id(denied_id, &reloaded));
    TEST_ASSERT_FALSE(reloaded.protected);

    char batch_body[256];
    snprintf(batch_body, sizeof(batch_body),
             "{\"ids\":[%llu,%llu],\"protected\":false}",
             (unsigned long long)allowed_id,
             (unsigned long long)denied_id);
    json = call_handler_path(handle_batch_protect_recordings,
                             HTTP_METHOD_POST,
                             "/api/recordings/batch-protect", batch_body,
                             api_key, 200);
    TEST_ASSERT_EQUAL_INT(
        1, cJSON_GetObjectItemCaseSensitive(json, "success_count")->valueint);
    TEST_ASSERT_EQUAL_INT(
        1, cJSON_GetObjectItemCaseSensitive(json, "fail_count")->valueint);
    cJSON_Delete(json);

    char ptz_path[MAX_STREAM_NAME + 64];
    snprintf(ptz_path, sizeof(ptz_path), "/api/streams/%s/ptz/move",
             denied.name);
    json = call_handler_path(handle_ptz_move, HTTP_METHOD_POST, ptz_path,
                             "{\"pan\":1}", api_key, 403);
    cJSON_Delete(json);

    snprintf(path, sizeof(path), "/api/recordings/download/%llu",
             (unsigned long long)denied_id);
    json = call_handler_path(handle_recordings_download, HTTP_METHOD_GET,
                             path, NULL, api_key, 403);
    cJSON_Delete(json);

    snprintf(batch_body, sizeof(batch_body),
             "{\"ids\":[%llu,%llu],\"filename\":\"evidence.zip\"}",
             (unsigned long long)allowed_id,
             (unsigned long long)denied_id);
    json = call_handler_path(handle_batch_download_recordings,
                             HTTP_METHOD_POST,
                             "/api/recordings/batch-download", batch_body,
                             api_key, 403);
    cJSON_Delete(json);

    snprintf(batch_body, sizeof(batch_body),
             "{\"ids\":[%llu],\"filename\":\"allowed.zip\"}",
             (unsigned long long)allowed_id);
    json = call_handler_path(handle_batch_download_recordings,
                             HTTP_METHOD_POST,
                             "/api/recordings/batch-download", batch_body,
                             api_key, 202);
    char download_token[64];
    safe_strcpy(
        download_token,
        cJSON_GetObjectItemCaseSensitive(json, "token")->valuestring,
        sizeof(download_token), 0);
    cJSON_Delete(json);

    int64_t other_user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("otherexporter", "password123", NULL,
                               USER_ROLE_VIEWER, true, &other_user_id));
    char other_api_key[128] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_generate_api_key(other_user_id, other_api_key,
                                    sizeof(other_api_key)));
    char status_path[160];
    snprintf(status_path, sizeof(status_path),
             "/api/recordings/batch-download/status/%s", download_token);
    json = call_handler_path(handle_batch_download_status, HTTP_METHOD_GET,
                             status_path, NULL, other_api_key, 404);
    cJSON_Delete(json);

    bool download_finished = false;
    for (int attempt = 0; attempt < 100 && !download_finished; attempt++) {
        json = call_handler_path(handle_batch_download_status,
                                 HTTP_METHOD_GET, status_path, NULL, api_key,
                                 200);
        const char *status = cJSON_GetObjectItemCaseSensitive(
            json, "status")->valuestring;
        download_finished = strcmp(status, "complete") == 0 ||
                            strcmp(status, "error") == 0;
        cJSON_Delete(json);
        if (!download_finished) {
            const struct timespec delay = {.tv_sec = 0,
                                           .tv_nsec = 1000000};
            nanosleep(&delay, NULL);
        }
    }
    TEST_ASSERT_TRUE(download_finished);
    snprintf(status_path, sizeof(status_path),
             "/api/recordings/batch-download/result/%s", download_token);
    json = call_handler_path(handle_batch_download_result, HTTP_METHOD_GET,
                             status_path, NULL, api_key, 500);
    cJSON_Delete(json);

    snprintf(path, sizeof(path), "/api/recordings/%llu",
             (unsigned long long)denied_id);
    json = call_handler_path(handle_delete_recording, HTTP_METHOD_DELETE,
                             path, NULL, api_key, 403);
    cJSON_Delete(json);
    TEST_ASSERT_EQUAL_INT(
        0, get_recording_metadata_by_id(denied_id, &reloaded));

    snprintf(path, sizeof(path), "/api/recordings/%llu",
             (unsigned long long)allowed_id);
    json = call_handler_path(handle_delete_recording, HTTP_METHOD_DELETE,
                             path, NULL, api_key, 200);
    cJSON_Delete(json);
    TEST_ASSERT_NOT_EQUAL(
        0, get_recording_metadata_by_id(allowed_id, &reloaded));
    g_config.web_auth_enabled = false;
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
    RUN_TEST(test_shared_collection_grants_track_membership_and_guard_scope);
    RUN_TEST(test_scoped_api_token_intersects_user_policy_and_revokes);
    RUN_TEST(test_action_catalog_and_simulation_handlers);
    RUN_TEST(test_role_and_policy_database_mutations_are_atomic);
    RUN_TEST(test_policy_management_handlers_and_conflict_guards);
    RUN_TEST(test_policy_role_update_cannot_lock_out_requester);
    RUN_TEST(test_users_api_reports_authorization_mode);
    RUN_TEST(test_sensitive_handlers_enforce_camera_scoped_policy);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
