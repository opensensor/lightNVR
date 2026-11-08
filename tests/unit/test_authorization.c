/**
 * @file test_authorization.c
 * @brief Action catalog, compatibility policy, selector grants, and simulation.
 */

#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700

#include <cjson/cJSON.h>
#include <ftw.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <time.h>
#include <unistd.h>

#include "unity.h"
#include "core/authorization.h"
#include "core/camera_collection_filter.h"
#include "core/config.h"
#include "database/db_auth.h"
#include "database/db_api_tokens.h"
#include "database/db_audit.h"
#include "database/db_authorization.h"
#include "database/db_camera_collections.h"
#include "database/db_camera_tags.h"
#include "database/db_core.h"
#include "database/db_detections.h"
#include "database/db_fleet_query.h"
#include "database/db_locations.h"
#include "database/db_recordings.h"
#include "database/db_streams.h"
#include "telemetry/stream_metrics.h"
#include "telemetry/player_telemetry.h"
#include "utils/strings.h"
#include "web/api_handlers.h"
#include "web/api_handlers_health.h"
#include "web/api_handlers_authorization.h"
#include "web/api_handlers_investigations.h"
#include "web/api_handlers_locations.h"
#include "web/api_handlers_lpr.h"
#include "web/api_handlers_metrics.h"
#include "web/api_handlers_ptz.h"
#include "web/api_handlers_recordings.h"
#include "web/api_handlers_recordings_batch_download.h"
#include "web/api_handlers_recordings_download.h"
#include "web/api_handlers_users.h"
#include "web/api_handlers_totp.h"
#include "web/api_handlers_timeline.h"
#include "web/batch_delete_progress.h"
#include "web/httpd_utils.h"
#include "web/request_response.h"

static int timeline_summary_calls;
static int timeline_summary_count;
static bool fail_timeline_summary;

int __real_get_recording_detection_summaries_for_cameras(
    const recording_metadata_t *, int, recording_detection_summary_t *);
int __wrap_get_recording_detection_summaries_for_cameras(
    const recording_metadata_t *records, int count, recording_detection_summary_t *summaries) {
    timeline_summary_calls++;
    timeline_summary_count = count;
    if (fail_timeline_summary) return -1;
    return __real_get_recording_detection_summaries_for_cameras(records, count, summaries);
}

static char test_directory[] = "/tmp/lightnvr-authorization-XXXXXX";
static char test_db_path[sizeof(test_directory) + sizeof("/authorization.db")];
static char *saved_lpr_master_key;
static bool restore_lpr_environment;
static int saved_max_streams;
#define OPERATOR_ROLE_UUID "00000000-0000-4000-8000-000000000002"
#define ADMIN_ROLE_UUID "00000000-0000-4000-8000-000000000001"

static int remove_test_path(const char *path, const struct stat *status,
                             int type, struct FTW *walk) {
    (void)status; (void)type; (void)walk;
    return remove(path);
}

static void cleanup_test_db_path(void) {
    /* Include SQLite sidecars and initial/final backup artifacts. Never follow
     * symlinks while removing this process's private temporary directory. */
    if (nftw(test_directory, remove_test_path, 16, FTW_DEPTH | FTW_PHYS) != 0)
        perror("Failed to remove authorization test directory");
}

static int initialize_test_db_path(void) {
    if (!mkdtemp(test_directory)) return -1;
    snprintf(test_db_path, sizeof(test_db_path), "%s/authorization.db", test_directory);
    if (atexit(cleanup_test_db_path) != 0) {
        cleanup_test_db_path();
        return -1;
    }
    return 0;
}

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
    if (grant_uuid) TEST_ASSERT_EQUAL_UINT(36, strlen(grant_uuid));
}

static void seed_legacy_allowed_tags(int64_t user_id, const char *tags) {
    sqlite3 *db = get_db_handle();
    sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_prepare_v2(
            db, "UPDATE users SET authorization_mode='legacy',allowed_tags=? "
                "WHERE id=?;", -1, &stmt, NULL));
    sqlite3_bind_text(stmt, 1, tags, -1, SQLITE_TRANSIENT);
    sqlite3_bind_int64(stmt, 2, user_id);
    TEST_ASSERT_EQUAL_INT(SQLITE_DONE, sqlite3_step(stmt));
    sqlite3_finalize(stmt);
}

static void create_scoped_token(int64_t user_id, uint64_t action_mask,
                                const char *scope_type,
                                const char *selector_json,
                                api_token_t *token,
                                char secret[API_TOKEN_SECRET_MAX]) {
    api_token_create_t input;
    memset(&input, 0, sizeof(input));
    input.user_id = user_id;
    input.created_by_user_id = user_id;
    input.description = "authorization regression";
    input.action_mask = action_mask;
    input.scope_type = scope_type;
    input.selector_json = selector_json;
    input.expires_at = (int64_t)time(NULL) + 3600;
    TEST_ASSERT_EQUAL_INT(DB_API_TOKEN_OK,
                          db_api_token_create(&input, token, secret));
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
    if (path) {
        const char *query = strchr(path, '?');
        if (query) {
            size_t path_length = (size_t)(query - path);
            if (path_length >= sizeof(req.path)) path_length = sizeof(req.path) - 1;
            memcpy(req.path, path, path_length);
            req.path[path_length] = '\0';
            safe_strcpy(req.query_string, query + 1,
                        sizeof(req.query_string), 0);
        } else {
            safe_strcpy(req.path, path, sizeof(req.path), 0);
        }
    }
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

static bool audit_has_operation(const char *action, const char *outcome,
                                const char *operation) {
    audit_query_t query = {.page = 1, .page_size = 100};
    safe_strcpy(query.action, action, sizeof(query.action), 0);
    safe_strcpy(query.outcome, outcome, sizeof(query.outcome), 0);
    audit_page_t page;
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    char needle[128];
    snprintf(needle, sizeof(needle), "\"operation\":\"%s\"", operation);
    bool found = false;
    for (int i = 0; i < page.count; i++) {
        if (strstr(page.events[i].details_json, needle)) {
            found = true;
            break;
        }
    }
    db_audit_page_free(&page);
    return found;
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
    saved_max_streams = g_config.max_streams;
    restore_lpr_environment = false;
    const char *key = getenv("LIGHTNVR_LPR_MASTER_KEY_HEX");
    saved_lpr_master_key = key ? strdup(key) : NULL;
    if (key) TEST_ASSERT_NOT_NULL(saved_lpr_master_key);
    restore_lpr_environment = true;
    sqlite3 *db = get_db_handle();
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
    sqlite3_exec(
        db,
        "DELETE FROM authz_grants WHERE user_id NOT IN "
        "(SELECT id FROM users WHERE username='admin');",
        NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM authz_roles WHERE is_builtin=0;", NULL, NULL,
                 NULL);
    sqlite3_exec(db, "DELETE FROM detections;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM recordings;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM streams;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM camera_tags;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM audit_events;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM users WHERE username != 'admin';",
                 NULL, NULL, NULL);
}

void tearDown(void) {
    player_telemetry_shutdown();
    metrics_shutdown();
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
    g_config.max_streams = saved_max_streams;
    int restored = 0;
    if (restore_lpr_environment) {
        restored = saved_lpr_master_key
            ? setenv("LIGHTNVR_LPR_MASTER_KEY_HEX", saved_lpr_master_key, 1)
            : unsetenv("LIGHTNVR_LPR_MASTER_KEY_HEX");
    }
    free(saved_lpr_master_key);
    saved_lpr_master_key = NULL;
    restore_lpr_environment = false;
    TEST_ASSERT_EQUAL_INT_MESSAGE(0, restored, "Failed to restore LPR environment");
}

void test_action_catalog_is_stable_and_complete(void) {
    int count = 0;
    const authorization_action_metadata_t *catalog =
        authorization_action_catalog(&count);
    TEST_ASSERT_EQUAL_INT(AUTHZ_ACTION_COUNT, count);
    TEST_ASSERT_EQUAL_STRING("live.view", catalog[0].key);
    TEST_ASSERT_EQUAL_STRING("observations.annotate", catalog[count - 1].key);
    /* INT_01 P2: appended at bit 19, camera-scoped and non-destructive. */
    TEST_ASSERT_EQUAL_INT(19, (int)AUTHZ_OBSERVATIONS_ANNOTATE);
    TEST_ASSERT_EQUAL_STRING("lpr.delete", catalog[AUTHZ_LPR_DELETE].key);
    TEST_ASSERT_TRUE(catalog[AUTHZ_OBSERVATIONS_ANNOTATE].camera_scoped);
    TEST_ASSERT_FALSE(catalog[AUTHZ_OBSERVATIONS_ANNOTATE].destructive);
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

void test_legacy_role_compatibility_has_no_runtime_allowed_tags_path(void) {
    stream_config_t outside = create_camera("Outside", "Outdoor");
    stream_config_t inside = create_camera("Inside", "Indoor");
    int64_t viewer_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("legacyviewer", "password123", NULL,
                               USER_ROLE_VIEWER, true, &viewer_id));
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
    TEST_ASSERT_EQUAL_INT(AUTHZ_DECISION_ALLOW, evaluation.decision);
    TEST_ASSERT_EQUAL_INT(
        0, authorization_evaluate(&viewer, AUTHZ_PTZ_CONTROL, outside_camera,
                                  &evaluation));
    TEST_ASSERT_EQUAL_INT(AUTHZ_DECISION_DENY, evaluation.decision);
    free(outside_inventory);
    free(inside_inventory);
}

void test_legacy_principals_migrate_to_idempotent_selector_grants(void) {
    stream_config_t outside = create_camera("Migrated Outside", "Outdoor");
    stream_config_t inside = create_camera("Migrated Inside", "Indoor");
    int64_t viewer_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("migratedviewer", "password123", NULL,
                               USER_ROLE_VIEWER, true, &viewer_id));
    seed_legacy_allowed_tags(viewer_id, " Outdoor ");
    /* A legacy principal may have stale/pre-authored grants, but those grants
     * were never part of its active access decision. Migration must replace
     * this broader grant instead of silently widening the tag-scoped viewer. */
    insert_grant("aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa", viewer_id,
                 OPERATOR_ROLE_UUID, "all", NULL);

    int migrated = -1;
    TEST_ASSERT_EQUAL_INT(
        0, db_authorization_migrate_legacy_users(&migrated));
    TEST_ASSERT_EQUAL_INT(1, migrated);

    user_t viewer;
    TEST_ASSERT_EQUAL_INT(0, db_auth_get_user_by_id(viewer_id, &viewer));
    TEST_ASSERT_EQUAL_STRING("policy", viewer.authorization_mode);
    sqlite3_stmt *allowed_tags_stmt = NULL;
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_prepare_v2(get_db_handle(),
                           "SELECT allowed_tags FROM users WHERE id=?;", -1,
                           &allowed_tags_stmt, NULL));
    sqlite3_bind_int64(allowed_tags_stmt, 1, viewer_id);
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(allowed_tags_stmt));
    TEST_ASSERT_EQUAL_INT(SQLITE_NULL,
                          sqlite3_column_type(allowed_tags_stmt, 0));
    sqlite3_finalize(allowed_tags_stmt);

    char mode[USER_AUTHORIZATION_MODE_MAX] = {0};
    authorization_grant_t *grants = NULL;
    int grant_count = 0;
    int64_t policy_version = 0;
    TEST_ASSERT_EQUAL_INT(
        DB_AUTHORIZATION_OK,
        db_authorization_get_user_policy(viewer_id, mode, &grants,
                                         &grant_count, &policy_version));
    TEST_ASSERT_EQUAL_STRING("policy", mode);
    TEST_ASSERT_EQUAL_INT(1, grant_count);
    TEST_ASSERT_EQUAL_STRING("Viewer", grants[0].role_name);
    TEST_ASSERT_EQUAL_STRING("selector", grants[0].scope_type);
    TEST_ASSERT_NOT_NULL(strstr(grants[0].selector_json, "tag_any"));
    free(grants);

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
    TEST_ASSERT_EQUAL_INT(AUTHZ_SOURCE_POLICY_GRANT, evaluation.source);
    TEST_ASSERT_EQUAL_INT(
        0, authorization_evaluate(&viewer, AUTHZ_LIVE_VIEW, inside_camera,
                                  &evaluation));
    TEST_ASSERT_EQUAL_INT(AUTHZ_DECISION_DENY, evaluation.decision);
    free(outside_inventory);
    free(inside_inventory);

    int64_t version_before = 0;
    int64_t version_after = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_authorization_get_policy_version(&version_before));
    migrated = -1;
    TEST_ASSERT_EQUAL_INT(
        0, db_authorization_migrate_legacy_users(&migrated));
    TEST_ASSERT_EQUAL_INT(0, migrated);
    TEST_ASSERT_EQUAL_INT(
        0, db_authorization_get_policy_version(&version_after));
    TEST_ASSERT_EQUAL_INT64(version_before, version_after);
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
        AUTHZ_ACTION_COUNT,
        cJSON_GetObjectItemCaseSensitive(json, "count")->valueint);
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

void test_recording_list_intersects_explicit_stream_collection_and_policy_scope(void) {
    stream_config_t camera = create_camera("ReplayIntersection", "Review");
    camera_collection_t collection;
    memset(&collection, 0, sizeof(collection));
    safe_strcpy(collection.name, "Replay intersection",
                sizeof(collection.name), 0);
    safe_strcpy(collection.collection_type, "static",
                sizeof(collection.collection_type), 0);
    collection.is_shared = true;
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_OK,
                          db_camera_collection_create(&collection));
    const char *members[] = {camera.camera_uuid};
    TEST_ASSERT_EQUAL_INT(
        DB_CAMERA_COLLECTION_OK,
        db_camera_collection_set_members(collection.uuid, members, 1));

    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("replayintersection", "password123", NULL,
                               USER_ROLE_USER, true, &user_id));
    TEST_ASSERT_EQUAL_INT(0,
                          db_authorization_set_user_mode(user_id, "policy"));
    char grant_uuid[CAMERA_UUID_STRING_SIZE];
    create_grant(user_id, OPERATOR_ROLE_UUID, "all", NULL, grant_uuid);
    char api_key[128] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_generate_api_key(user_id, api_key, sizeof(api_key)));

    recording_metadata_t recording;
    memset(&recording, 0, sizeof(recording));
    safe_strcpy(recording.stream_name, camera.name,
                sizeof(recording.stream_name), 0);
    safe_strcpy(recording.camera_uuid, camera.camera_uuid,
                sizeof(recording.camera_uuid), 0);
    safe_strcpy(recording.file_path, "/tmp/replay-intersection.mp4",
                sizeof(recording.file_path), 0);
    recording.start_time = 1700000000;
    recording.end_time = 1700000060;
    recording.is_complete = true;
    recording.retention_override_days = -1;
    TEST_ASSERT_NOT_EQUAL(0, add_recording_metadata(&recording));

    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    req.method = HTTP_METHOD_GET;
    safe_strcpy(req.method_str, "GET", sizeof(req.method_str), 0);
    safe_strcpy(req.path, "/api/recordings", sizeof(req.path), 0);
    snprintf(req.query_string, sizeof(req.query_string),
             "stream=%s&collection_uuid=%s", camera.name, collection.uuid);
    safe_strcpy(req.client_ip, "127.0.0.1", sizeof(req.client_ip), 0);
    safe_strcpy(req.headers[0].name, "X-API-Key",
                sizeof(req.headers[0].name), 0);
    safe_strcpy(req.headers[0].value, api_key,
                sizeof(req.headers[0].value), 0);
    req.num_headers = 1;
    g_config.web_auth_enabled = true;
    handle_get_recordings(&req, &res);
    g_config.web_auth_enabled = false;
    TEST_ASSERT_EQUAL_INT(200, res.status_code);
    cJSON *json = cJSON_Parse((const char *)res.body);
    TEST_ASSERT_NOT_NULL(json);
    TEST_ASSERT_EQUAL_INT(
        1, cJSON_GetObjectItemCaseSensitive(
               cJSON_GetObjectItemCaseSensitive(json, "pagination"),
               "total")->valueint);
    TEST_ASSERT_EQUAL_INT(
        1, cJSON_GetArraySize(cJSON_GetObjectItemCaseSensitive(
               json, "recordings")));
    cJSON_Delete(json);
    http_response_free(&res);
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
    char owner_key[128] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_generate_api_key(user_id, owner_key, sizeof(owner_key)));

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
    g_config.web_auth_enabled = true;
    cJSON *json = call_handler_path(
        handle_post_user_api_token, HTTP_METHOD_POST, path, body, owner_key,
        201);
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
                             path, NULL, owner_key, 200);
    TEST_ASSERT_EQUAL_INT(
        1, cJSON_GetObjectItemCaseSensitive(json, "count")->valueint);
    TEST_ASSERT_NULL(cJSON_GetObjectItemCaseSensitive(
        cJSON_GetArrayItem(cJSON_GetObjectItemCaseSensitive(json, "tokens"), 0),
        "secret"));
    cJSON_Delete(json);

    TEST_ASSERT_EQUAL_INT(
        1, authorize_stream_with_key(secret, AUTHZ_PTZ_CONTROL,
                                     allowed.name, 200));
    TEST_ASSERT_EQUAL_INT(
        0, authorize_stream_with_key(secret, AUTHZ_PTZ_CONTROL,
                                     denied.name, 403));
    TEST_ASSERT_EQUAL_INT(
        0, authorize_stream_with_key(secret, AUTHZ_EVIDENCE_PROTECT,
                                     allowed.name, 403));

    audit_query_t token_use_query = {.page = 1, .page_size = 20};
    safe_strcpy(token_use_query.action, "api_token.use",
                sizeof(token_use_query.action), 0);
    audit_page_t token_use_page;
    TEST_ASSERT_EQUAL_INT(0,
                          db_audit_query(&token_use_query, &token_use_page));
    TEST_ASSERT_EQUAL_INT64(1, token_use_page.total);
    TEST_ASSERT_EQUAL_STRING("success", token_use_page.events[0].outcome);
    TEST_ASSERT_EQUAL_STRING(token_uuid,
                             token_use_page.events[0].target_uuid);
    TEST_ASSERT_NOT_NULL(strstr(token_use_page.events[0].details_json,
                                "\"reason\":\"active\""));
    db_audit_page_free(&token_use_page);

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
    TEST_ASSERT_EQUAL_INT(0,
                          db_audit_query(&token_use_query, &token_use_page));
    TEST_ASSERT_EQUAL_INT64(2, token_use_page.total);
    bool revoked_use = false;
    for (int i = 0; i < token_use_page.count; i++) {
        revoked_use |= strcmp(token_use_page.events[i].outcome, "denied") == 0 &&
            strstr(token_use_page.events[i].details_json,
                   "\"reason\":\"revoked\"") != NULL;
    }
    TEST_ASSERT_TRUE(revoked_use);
    db_audit_page_free(&token_use_page);

    api_token_t expired_token;
    char expired_secret[API_TOKEN_SECRET_MAX];
    create_scoped_token(user_id,
                        authorization_action_bit(AUTHZ_PTZ_CONTROL), "all",
                        NULL, &expired_token, expired_secret);
    stmt = NULL;
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_prepare_v2(
            db, "UPDATE authz_api_tokens SET created_at=?,expires_at=? "
                "WHERE uuid=?;", -1,
            &stmt, NULL));
    sqlite3_bind_int64(stmt, 1, (sqlite3_int64)time(NULL) - 100);
    sqlite3_bind_int64(stmt, 2, (sqlite3_int64)time(NULL) - 1);
    sqlite3_bind_text(stmt, 3, expired_token.uuid, -1, SQLITE_TRANSIENT);
    TEST_ASSERT_EQUAL_INT(SQLITE_DONE, sqlite3_step(stmt));
    sqlite3_finalize(stmt);
    TEST_ASSERT_EQUAL_INT(
        0, authorize_stream_with_key(expired_secret, AUTHZ_PTZ_CONTROL,
                                     allowed.name, 401));
    const char *unknown_secret =
        "lnvr_0000000000000000000000000000000000000000000000000000000000000000";
    TEST_ASSERT_EQUAL_INT(
        0, authorize_stream_with_key(unknown_secret, AUTHZ_PTZ_CONTROL,
                                     allowed.name, 401));
    TEST_ASSERT_EQUAL_INT(0,
                          db_audit_query(&token_use_query, &token_use_page));
    TEST_ASSERT_EQUAL_INT64(4, token_use_page.total);
    bool expired_use = false;
    bool unknown_use = false;
    for (int i = 0; i < token_use_page.count; i++) {
        expired_use |= strstr(token_use_page.events[i].details_json,
                              "\"reason\":\"expired\"") != NULL;
        unknown_use |= strstr(token_use_page.events[i].details_json,
                              "\"reason\":\"unknown\"") != NULL;
    }
    TEST_ASSERT_TRUE(expired_use);
    TEST_ASSERT_TRUE(unknown_use);
    db_audit_page_free(&token_use_page);
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
        AUTHZ_ACTION_COUNT,
        cJSON_GetObjectItemCaseSensitive(json, "count")->valueint);
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

    snprintf(body, sizeof(body),
             "{\"expected_policy_version\":%lld,\"mode\":\"legacy\","
             "\"grants\":[]}", (long long)policy_version);
    json = call_handler_path(handle_put_user_authorization, HTTP_METHOD_PUT,
                             user_path, body, NULL, 400);
    cJSON_Delete(json);

    json = call_handler_path(handle_get_user_authorization, HTTP_METHOD_GET,
                             user_path, NULL, NULL, 200);
    cJSON *scope = cJSON_GetObjectItemCaseSensitive(
        cJSON_GetArrayItem(
            cJSON_GetObjectItemCaseSensitive(json, "grants"), 0), "scope");
    TEST_ASSERT_EQUAL_STRING(
        "all", cJSON_GetObjectItemCaseSensitive(scope, "type")->valuestring);
    cJSON_Delete(json);

    snprintf(body, sizeof(body),
             "{\"expected_policy_version\":%lld,\"mode\":\"policy\","
             "\"grants\":[{\"role_uuid\":\"%s\","
             "\"scope\":{\"type\":\"all\"}}]}",
             (long long)role_version, role_uuid);
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
                             api_key, 403);
    cJSON_Delete(json);
    TEST_ASSERT_EQUAL_INT(0, get_recording_metadata_by_id(allowed_id, &reloaded));
    TEST_ASSERT_TRUE(reloaded.protected); /* No partial mutation of allowed member. */
    TEST_ASSERT_EQUAL_INT(0, get_recording_metadata_by_id(denied_id, &reloaded));
    TEST_ASSERT_FALSE(reloaded.protected);
    /* The rejected batch did not unprotect A; explicitly do that authorized
     * operation before the independent delete checks later in this test. */
    snprintf(path, sizeof(path), "/api/recordings/%llu/protect", (unsigned long long)allowed_id);
    json = call_handler_path(handle_put_recording_protect, HTTP_METHOD_PUT,
                             path, "{\"protected\":false}", api_key, 200);
    cJSON_Delete(json);

    char ptz_path[MAX_STREAM_NAME + 64];
    snprintf(ptz_path, sizeof(ptz_path), "/api/streams/%s/ptz/move",
             denied.name);
    json = call_handler_path(handle_ptz_move, HTTP_METHOD_POST, ptz_path,
                             "{\"pan\":1}", api_key, 403);
    cJSON_Delete(json);

    snprintf(ptz_path, sizeof(ptz_path), "/api/streams/%s/ptz/absolute",
             allowed.name);
    json = call_handler_path(handle_ptz_absolute, HTTP_METHOD_POST, ptz_path,
                             "{}", api_key, 400);
    cJSON_Delete(json);

    snprintf(path, sizeof(path), "/api/recordings/download/%llu",
             (unsigned long long)allowed_id);
    json = call_handler_path(handle_recordings_download, HTTP_METHOD_GET,
                             path, NULL, api_key, 404);
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
             "{\"ids\":[\"%llu\"],\"filename\":\"allowed.zip\"}",
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
                             status_path, NULL, api_key, 409);
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

    recording_metadata_t batch_recording = allowed_recording;
    safe_strcpy(batch_recording.file_path,
                "/tmp/lightnvr-scoped-batch-recording.mp4",
                sizeof(batch_recording.file_path), 0);
    uint64_t batch_id = add_recording_metadata(&batch_recording);
    TEST_ASSERT_NOT_EQUAL(0, batch_id);
    snprintf(batch_body, sizeof(batch_body), "{\"ids\":[%llu]}",
             (unsigned long long)batch_id);
    json = call_handler_path(handle_batch_delete_recordings, HTTP_METHOD_POST,
                             "/api/recordings/batch-delete", batch_body,
                             api_key, 202);
    char delete_job_id[64];
    safe_strcpy(
        delete_job_id,
        cJSON_GetObjectItemCaseSensitive(json, "job_id")->valuestring,
        sizeof(delete_job_id), 0);
    cJSON_Delete(json);
    bool delete_finished = false;
    snprintf(status_path, sizeof(status_path),
             "/api/recordings/batch-delete/progress/%s", delete_job_id);
    json = call_handler_path(handle_batch_delete_progress, HTTP_METHOD_GET,
                             status_path, NULL, other_api_key, 404);
    cJSON_Delete(json);
    for (int attempt = 0; attempt < 100 && !delete_finished; attempt++) {
        snprintf(status_path, sizeof(status_path),
                 "/api/recordings/batch-delete/progress/%s", delete_job_id);
        json = call_handler_path(handle_batch_delete_progress, HTTP_METHOD_GET,
                                 status_path, NULL, api_key, 200);
        delete_finished = cJSON_IsTrue(
            cJSON_GetObjectItemCaseSensitive(json, "complete"));
        cJSON_Delete(json);
        if (!delete_finished) {
            const struct timespec delay = {.tv_sec = 0,
                                           .tv_nsec = 1000000};
            nanosleep(&delay, NULL);
        }
    }
    TEST_ASSERT_TRUE(delete_finished);
    TEST_ASSERT_NOT_EQUAL(
        0, get_recording_metadata_by_id(batch_id, &reloaded));
    TEST_ASSERT_TRUE(audit_has_operation("evidence.protect", "success",
                                         "protect"));
    TEST_ASSERT_TRUE(audit_has_operation("evidence.protect", "failure",
                                         "batch_unprotect"));
    TEST_ASSERT_TRUE(audit_has_operation("ptz.control", "failure",
                                         "absolute_move"));
    TEST_ASSERT_TRUE(audit_has_operation("recordings.export", "failure",
                                         "download_recording"));
    TEST_ASSERT_TRUE(audit_has_operation("recordings.export", "failure",
                                         "create_batch_archive"));
    TEST_ASSERT_TRUE(audit_has_operation("recordings.export", "error",
                                         "download_batch_archive"));
    TEST_ASSERT_TRUE(audit_has_operation("recording.delete", "success",
                                         "delete_recording"));
    TEST_ASSERT_TRUE(audit_has_operation("recording.delete", "success",
                                         "batch_delete"));
    g_config.web_auth_enabled = false;
}


/*
 * Regression: a policy-scoped principal must not be able to enumerate the
 * whole fleet through a list endpoint. Filtering has to happen before totals
 * and facets are derived, so the helper the list handlers share is the unit
 * under test.
 */
void test_visible_camera_filter_hides_cameras_outside_grant(void) {
    stream_config_t lobby = create_camera("Filter Lobby", "Lobby");
    create_camera("Filter Vault", "Vault");
    camera_tag_t lobby_tag = find_tag("Lobby");
    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("listscoped", "password123", NULL,
                               USER_ROLE_USER, true, &user_id));
    TEST_ASSERT_EQUAL_INT(0,
                          db_authorization_set_user_mode(user_id, "policy"));

    char selector[512];
    snprintf(selector, sizeof(selector),
             "{\"version\":1,\"expression\":{\"op\":\"tag_any\","
             "\"uuids\":[\"%s\"]}}",
             lobby_tag.uuid);
    char grant_uuid[CAMERA_UUID_STRING_SIZE];
    create_grant(user_id, OPERATOR_ROLE_UUID, "selector", selector,
                 grant_uuid);

    user_t user;
    TEST_ASSERT_EQUAL_INT(0, db_auth_get_user_by_id(user_id, &user));

    fleet_camera_t *cameras = NULL;
    int count = 0;
    TEST_ASSERT_EQUAL_INT(0, db_fleet_camera_load(&cameras, &count));
    TEST_ASSERT_GREATER_THAN(1, count);
    int total = count;

    g_config.web_auth_enabled = true;
    TEST_ASSERT_EQUAL_INT(
        0, authorization_filter_visible_cameras(&user, cameras, &count));
    TEST_ASSERT_EQUAL_INT(1, count);
    TEST_ASSERT_EQUAL_STRING(lobby.camera_uuid, cameras[0].camera_uuid);
    free(cameras);

    /* An unscoped legacy admin still sees everything. */
    user_t admin;
    memset(&admin, 0, sizeof(admin));
    admin.id = 999999;
    admin.role = USER_ROLE_ADMIN;
    admin.is_active = true;
    cameras = NULL;
    count = 0;
    TEST_ASSERT_EQUAL_INT(0, db_fleet_camera_load(&cameras, &count));
    TEST_ASSERT_EQUAL_INT(
        0, authorization_filter_visible_cameras(&admin, cameras, &count));
    TEST_ASSERT_EQUAL_INT(total, count);
    free(cameras);

    /* With authentication off the handlers never populate a principal, so the
     * filter must pass the inventory through rather than hide every camera. */
    g_config.web_auth_enabled = false;
    user_t unpopulated;
    memset(&unpopulated, 0, sizeof(unpopulated));
    cameras = NULL;
    count = 0;
    TEST_ASSERT_EQUAL_INT(0, db_fleet_camera_load(&cameras, &count));
    TEST_ASSERT_EQUAL_INT(
        0, authorization_filter_visible_cameras(&unpopulated, cameras, &count));
    TEST_ASSERT_EQUAL_INT(total, count);
    free(cameras);
}

void test_investigation_search_facets_are_camera_scope_authorized(void) {
    stream_config_t allowed = create_camera("Search Allowed", "Search");
    stream_config_t denied = create_camera("Search Denied", "Search");
    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("searchscoped", "password123", NULL,
                               USER_ROLE_USER, true, &user_id));
    TEST_ASSERT_EQUAL_INT(0,
                          db_authorization_set_user_mode(user_id, "policy"));
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

    sqlite3_stmt *statement = NULL;
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_prepare_v2(
            get_db_handle(),
            "INSERT INTO detections "
            "(camera_uuid,stream_name,timestamp,label,confidence,source) "
            "VALUES (?,?,?,?,0.9,'');",
            -1, &statement, NULL));
    const stream_config_t *test_cameras[] = {&allowed, &denied};
    for (int i = 0; i < 2; i++) {
        sqlite3_reset(statement);
        sqlite3_clear_bindings(statement);
        sqlite3_bind_text(statement, 1, test_cameras[i]->camera_uuid, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, test_cameras[i]->name, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 3, 1700000100 + i);
        sqlite3_bind_text(statement, 4, i == 0 ? "person" : "vehicle", -1,
                          SQLITE_STATIC);
        TEST_ASSERT_EQUAL_INT(SQLITE_DONE, sqlite3_step(statement));
    }
    sqlite3_finalize(statement);

    char body[768];
    snprintf(body, sizeof(body),
             "{\"camera_uuids\":[\"%s\"],\"start_time\":1700000000,"
             "\"end_time\":1700001000}",
             allowed.camera_uuid);
    g_config.web_auth_enabled = true;
    cJSON *json = call_handler_path(
        handle_post_investigation_search, HTTP_METHOD_POST,
        "/api/investigations/search", body, api_key, 200);
    TEST_ASSERT_EQUAL_INT(
        1, cJSON_GetObjectItemCaseSensitive(
               cJSON_GetObjectItemCaseSensitive(json, "page"),
               "total")->valueint);
    const cJSON *camera_facets = cJSON_GetObjectItemCaseSensitive(
        cJSON_GetObjectItemCaseSensitive(json, "facets"), "cameras");
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(camera_facets));
    TEST_ASSERT_EQUAL_STRING(
        allowed.camera_uuid,
        cJSON_GetObjectItemCaseSensitive(cJSON_GetArrayItem(camera_facets, 0),
                                         "value")->valuestring);
    cJSON_Delete(json);

    snprintf(body, sizeof(body),
             "{\"selector\":{\"version\":1,\"expression\":{"
             "\"op\":\"all\"}},\"start_time\":1700000000,"
             "\"end_time\":1700001000}");
    json = call_handler_path(
        handle_post_investigation_search, HTTP_METHOD_POST,
        "/api/investigations/search", body, api_key, 200);
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        json, "selector_applied")));
    TEST_ASSERT_EQUAL_INT(
        1, cJSON_GetObjectItemCaseSensitive(json, "camera_count")->valueint);
    TEST_ASSERT_EQUAL_INT(
        1, cJSON_GetObjectItemCaseSensitive(
               cJSON_GetObjectItemCaseSensitive(json, "page"),
               "total")->valueint);
    camera_facets = cJSON_GetObjectItemCaseSensitive(
        cJSON_GetObjectItemCaseSensitive(json, "facets"), "cameras");
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(camera_facets));
    TEST_ASSERT_EQUAL_STRING(
        allowed.camera_uuid,
        cJSON_GetObjectItemCaseSensitive(cJSON_GetArrayItem(camera_facets, 0),
                                         "value")->valuestring);
    cJSON_Delete(json);

    snprintf(body, sizeof(body),
             "{\"camera_uuids\":[\"%s\",\"%s\"],"
             "\"start_time\":1700000000,\"end_time\":1700001000}",
             allowed.camera_uuid, denied.camera_uuid);
    json = call_handler_path(
        handle_post_investigation_search, HTTP_METHOD_POST,
        "/api/investigations/search", body, api_key, 403);
    TEST_ASSERT_NOT_NULL(cJSON_GetObjectItemCaseSensitive(json, "error"));
    cJSON_Delete(json);
    g_config.web_auth_enabled = false;
}

void test_investigation_recording_preview_explains_action_permissions(void) {
    stream_config_t camera = create_camera("Replay Only", "Review");

    int64_t policy_version = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_authorization_get_policy_version(&policy_version));
    authorization_role_t role;
    memset(&role, 0, sizeof(role));
    safe_strcpy(role.name, "Replay only", sizeof(role.name), 0);
    safe_strcpy(role.description, "Can review but not preserve or export",
                sizeof(role.description), 0);
    role.action_mask = authorization_action_bit(AUTHZ_RECORDINGS_REPLAY);
    int64_t next_version = 0;
    TEST_ASSERT_EQUAL_INT(
        DB_AUTHORIZATION_OK,
        db_authorization_role_create(&role, policy_version, &next_version));

    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("replayonly", "password123", NULL,
                               USER_ROLE_VIEWER, true, &user_id));
    TEST_ASSERT_EQUAL_INT(
        0, db_authorization_set_user_mode(user_id, "policy"));
    char grant_uuid[CAMERA_UUID_STRING_SIZE];
    create_grant(user_id, role.uuid, "all", NULL, grant_uuid);
    char api_key[128] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_generate_api_key(user_id, api_key, sizeof(api_key)));

    recording_metadata_t recording;
    memset(&recording, 0, sizeof(recording));
    safe_strcpy(recording.stream_name, camera.name,
                sizeof(recording.stream_name), 0);
    safe_strcpy(recording.camera_uuid, camera.camera_uuid,
                sizeof(recording.camera_uuid), 0);
    safe_strcpy(recording.file_path, "/tmp/lightnvr-replay-only.mp4",
                sizeof(recording.file_path), 0);
    recording.start_time = 1700000000;
    recording.end_time = 1700000060;
    recording.is_complete = true;
    recording.retention_override_days = -1;
    TEST_ASSERT_NOT_EQUAL(0, add_recording_metadata(&recording));

    char body[512];
    snprintf(body, sizeof(body),
             "{\"camera_uuids\":[\"%s\"],"
             "\"start_time\":1700000000,\"end_time\":1700000060}",
             camera.camera_uuid);
    g_config.web_auth_enabled = true;
    cJSON *json = call_handler_path(
        handle_post_investigation_recording_preview, HTTP_METHOD_POST,
        "/api/investigations/recordings/preview", body, api_key, 200);
    const cJSON *recordings =
        cJSON_GetObjectItemCaseSensitive(json, "recordings");
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(recordings));
    const cJSON *item = cJSON_GetArrayItem(recordings, 0);
    TEST_ASSERT_FALSE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        item, "can_protect")));
    TEST_ASSERT_FALSE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        item, "can_export")));
    const cJSON *permissions =
        cJSON_GetObjectItemCaseSensitive(json, "permissions");
    TEST_ASSERT_EQUAL_INT(
        1, cJSON_GetObjectItemCaseSensitive(
               cJSON_GetObjectItemCaseSensitive(permissions, "protect"),
               "denied_count")->valueint);
    TEST_ASSERT_FALSE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        cJSON_GetObjectItemCaseSensitive(permissions, "export"),
        "all_allowed")));
    cJSON_Delete(json);
    g_config.web_auth_enabled = false;
}

/*
 * The effective mask is what stops a policy manager from minting authority it
 * does not hold, so it must reflect grants and legacy roles.
 */
void test_effective_action_mask_reflects_grants_and_roles(void) {
    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("maskuser", "password123", NULL,
                               USER_ROLE_USER, true, &user_id));
    TEST_ASSERT_EQUAL_INT(0,
                          db_authorization_set_user_mode(user_id, "policy"));
    char grant_uuid[CAMERA_UUID_STRING_SIZE];
    create_grant(user_id, OPERATOR_ROLE_UUID, "all", NULL, grant_uuid);

    user_t user;
    TEST_ASSERT_EQUAL_INT(0, db_auth_get_user_by_id(user_id, &user));
    uint64_t mask = 0;
    TEST_ASSERT_EQUAL_INT(0, authorization_effective_action_mask(&user, &mask));
    TEST_ASSERT_TRUE((mask & authorization_action_bit(AUTHZ_PTZ_CONTROL)) != 0);
    /* Operator deliberately excludes policy administration. */
    TEST_ASSERT_TRUE((mask & authorization_action_bit(AUTHZ_USERS_MANAGE)) == 0);
    TEST_ASSERT_TRUE((mask & authorization_action_bit(AUTHZ_SYSTEM_ADMIN)) == 0);
    /* Built-in roles fail closed for observations.annotate (migration 0096). */
    TEST_ASSERT_TRUE(
        (mask & authorization_action_bit(AUTHZ_OBSERVATIONS_ANNOTATE)) == 0);

    /* A legacy administrator holds the entire catalog. */
    user_t admin;
    memset(&admin, 0, sizeof(admin));
    admin.id = 1;
    admin.role = USER_ROLE_ADMIN;
    admin.is_active = true;
    TEST_ASSERT_EQUAL_INT(0, authorization_effective_action_mask(&admin, &mask));
    int catalog_count = 0;
    const authorization_action_metadata_t *catalog =
        authorization_action_catalog(&catalog_count);
    for (int i = 0; i < catalog_count; i++) {
        TEST_ASSERT_TRUE(
            (mask & authorization_action_bit(catalog[i].action)) != 0);
    }

    /* An inactive principal holds nothing. */
    admin.is_active = false;
    TEST_ASSERT_EQUAL_INT(0, authorization_effective_action_mask(&admin, &mask));
    TEST_ASSERT_EQUAL_UINT64(UINT64_C(0), mask);
}

void test_effective_action_mask_intersects_token_and_current_owner_permissions(void) {
    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_user("tokenmask", "password123", NULL,
        USER_ROLE_USER, true, &user_id));
    TEST_ASSERT_EQUAL_INT(0, db_authorization_set_user_mode(user_id, "policy"));
    char grant_uuid[CAMERA_UUID_STRING_SIZE];
    create_grant(user_id, OPERATOR_ROLE_UUID, "all", NULL, grant_uuid);
    api_token_t token;
    char secret[API_TOKEN_SECRET_MAX];
    /* Seed an overbroad token through the database fixture to verify the
     * intersection itself, independently of token-issuance checks. */
    create_scoped_token(user_id, authorization_action_bit(AUTHZ_LIVE_VIEW) |
        authorization_action_bit(AUTHZ_SYSTEM_ADMIN), "all", NULL, &token, secret);
    http_request_t req;
    http_request_init(&req);
    safe_strcpy(req.headers[0].name, "X-API-Key", sizeof(req.headers[0].name), 0);
    safe_strcpy(req.headers[0].value, secret, sizeof(req.headers[0].value), 0);
    req.num_headers = 1;
    g_config.web_auth_enabled = true;
    user_t authenticated = {0};
    TEST_ASSERT_TRUE(httpd_check_action_access(&req, &authenticated));
    TEST_ASSERT_TRUE(authenticated.authenticated_via_scoped_token);
    uint64_t mask = UINT64_MAX;
    TEST_ASSERT_EQUAL_INT(0, authorization_effective_action_mask(&authenticated, &mask));
    TEST_ASSERT_EQUAL_UINT64(authorization_action_bit(AUTHZ_LIVE_VIEW), mask);
    /* PTZ belongs to the owner but not the token; system.admin belongs to the
     * token's declared mask but not the owner. Neither may be effective. */
    TEST_ASSERT_FALSE(mask & authorization_action_bit(AUTHZ_PTZ_CONTROL));
    TEST_ASSERT_FALSE(mask & authorization_action_bit(AUTHZ_SYSTEM_ADMIN));

    char sql[256];
    snprintf(sql, sizeof(sql), "UPDATE authz_grants SET enabled=0 WHERE uuid='%s'", grant_uuid);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(get_db_handle(), sql, NULL, NULL, NULL));
    mask = UINT64_MAX;
    TEST_ASSERT_EQUAL_INT(0, authorization_effective_action_mask(&authenticated, &mask));
    TEST_ASSERT_EQUAL_UINT64(0, mask);
    snprintf(sql, sizeof(sql), "UPDATE authz_grants SET enabled=1 WHERE uuid='%s'", grant_uuid);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(get_db_handle(), sql, NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(DB_API_TOKEN_OK, db_api_token_revoke(user_id, token.uuid));
    mask = UINT64_MAX;
    TEST_ASSERT_EQUAL_INT(0, authorization_effective_action_mask(&authenticated, &mask));
    TEST_ASSERT_EQUAL_UINT64(0, mask);
}

/*
 * users.manage must not be a back door to system.admin: authoring a role that
 * carries more than the requester holds has to be refused.
 */
void test_policy_manager_cannot_grant_actions_it_lacks(void) {
    int64_t manager_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("limitedmanager", "password123", NULL,
                               USER_ROLE_USER, true, &manager_id));
    TEST_ASSERT_EQUAL_INT(
        0, db_authorization_set_user_mode(manager_id, "policy"));

    /* A role that can administer users but nothing else. */
    authorization_role_t manager_role;
    memset(&manager_role, 0, sizeof(manager_role));
    safe_strcpy(manager_role.name, "User Administrator",
                sizeof(manager_role.name), 0);
    manager_role.action_mask = authorization_action_bit(AUTHZ_USERS_MANAGE);
    int64_t version = 0;
    TEST_ASSERT_EQUAL_INT(0, db_authorization_get_policy_version(&version));
    int64_t new_version = 0;
    TEST_ASSERT_EQUAL_INT(
        DB_AUTHORIZATION_OK,
        db_authorization_role_create(&manager_role, version, &new_version));
    char grant_uuid[CAMERA_UUID_STRING_SIZE];
    create_grant(manager_id, manager_role.uuid, "all", NULL, grant_uuid);

    char api_key[128] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_generate_api_key(manager_id, api_key, sizeof(api_key)));

    int64_t current_version = 0;
    TEST_ASSERT_EQUAL_INT(0,
                          db_authorization_get_policy_version(&current_version));

    /* Escalating: a new role carrying system.admin is refused. */
    char escalate_body[256];
    snprintf(escalate_body, sizeof(escalate_body),
             "{\"expected_policy_version\":%lld,\"name\":\"Escalate\","
             "\"actions\":[\"system.admin\"]}",
             (long long)current_version);
    g_config.web_auth_enabled = true;
    cJSON *denied = call_handler_path(
        handle_post_authorization_role, HTTP_METHOD_POST,
        "/api/authorization/roles", escalate_body, api_key, 403);
    cJSON_Delete(denied);

    /* Non-escalating: a role bounded by what the manager holds is accepted. */
    char allowed_body[256];
    snprintf(allowed_body, sizeof(allowed_body),
             "{\"expected_policy_version\":%lld,"
             "\"name\":\"Delegated User Admin\","
             "\"actions\":[\"users.manage\"]}",
             (long long)current_version);
    cJSON *allowed = call_handler_path(
        handle_post_authorization_role, HTTP_METHOD_POST,
        "/api/authorization/roles", allowed_body, api_key, 201);
    cJSON_Delete(allowed);
    g_config.web_auth_enabled = false;
}

/*
 * The persisted mask layout is a compatibility contract with every issued
 * token, so the table and the compiled catalog must agree bit for bit.
 */
void test_action_catalog_bit_layout_matches_database(void) {
    TEST_ASSERT_EQUAL_INT(0, db_authorization_verify_action_catalog());

    int count = 0;
    const authorization_action_metadata_t *catalog =
        authorization_action_catalog(&count);
    sqlite3 *db = get_db_handle();
    sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_prepare_v2(db,
                           "SELECT bit_index FROM authz_actions "
                           "WHERE action_key = ?;", -1, &stmt, NULL));
    for (int i = 0; i < count; i++) {
        sqlite3_reset(stmt);
        sqlite3_clear_bindings(stmt);
        sqlite3_bind_text(stmt, 1, catalog[i].key, -1, SQLITE_TRANSIENT);
        TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(stmt));
        TEST_ASSERT_EQUAL_INT64((int64_t)catalog[i].action,
                                sqlite3_column_int64(stmt, 0));
        TEST_ASSERT_EQUAL_UINT64(UINT64_C(1) << catalog[i].action,
                                 authorization_action_bit(catalog[i].action));
    }
    sqlite3_finalize(stmt);

    /* Drift in the stored layout must fail startup rather than re-map tokens. */
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_exec(db,
                     "UPDATE authz_actions SET bit_index = 63 "
                     "WHERE action_key = 'system.admin';",
                     NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(-1, db_authorization_verify_action_catalog());

    stmt = NULL;
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_prepare_v2(db,
                           "UPDATE authz_actions SET bit_index = ? "
                           "WHERE action_key = 'system.admin';",
                           -1, &stmt, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK,
                          sqlite3_bind_int(stmt, 1, AUTHZ_SYSTEM_ADMIN));
    TEST_ASSERT_EQUAL_INT(SQLITE_DONE, sqlite3_step(stmt));
    sqlite3_finalize(stmt);

    TEST_ASSERT_EQUAL_INT(0, db_authorization_verify_action_catalog());
}

/*
 * streams.camera_uuid defaults to '' so the 0048 backfill could run; the guard
 * added in 0056 must reject that value instead of letting the unique index
 * accept one row and fail every later insert.
 */
void test_streams_reject_empty_camera_uuid(void) {
    sqlite3 *db = get_db_handle();
    TEST_ASSERT_EQUAL_INT(
        SQLITE_CONSTRAINT,
        sqlite3_exec(db,
                     "INSERT INTO streams (name,url) "
                     "VALUES ('No UUID','rtsp://camera/live');",
                     NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_CONSTRAINT_TRIGGER, sqlite3_extended_errcode(db));
    sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_prepare_v2(db,
                           "SELECT count(*) FROM streams WHERE name='No UUID';",
                           -1, &stmt, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(stmt));
    TEST_ASSERT_EQUAL_INT(0, sqlite3_column_int(stmt, 0));
    sqlite3_finalize(stmt);
}

static void assert_player_telemetry_status(const char *key, const char *camera,
                                           int expected_status, int expected_count) {
    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    req.method = HTTP_METHOD_POST;
    safe_strcpy(req.path, "/api/telemetry/player", sizeof(req.path), 0);
    safe_strcpy(req.method_str, "POST", sizeof(req.method_str), 0);
    safe_strcpy(req.client_ip, "127.0.0.1", sizeof(req.client_ip), 0);
    if (key) {
        safe_strcpy(req.headers[0].name, "X-API-Key", sizeof(req.headers[0].name), 0);
        safe_strcpy(req.headers[0].value, key, sizeof(req.headers[0].value), 0);
        req.num_headers = 1;
    }
    char body[256];
    snprintf(body, sizeof(body), "{\"stream_name\":\"%s\",\"session_id\":\"test-session\",\"transport\":\"hls\",\"ttff_ms\":10}", camera);
    req.body = body;
    req.body_len = strlen(body);
    handle_post_player_telemetry(&req, &res);
    int status = res.status_code;
    http_response_free(&res);
    TEST_ASSERT_EQUAL_INT(expected_status, status);
    TEST_ASSERT_EQUAL_INT(expected_count, player_telemetry_count());
}

void test_player_telemetry_requires_live_view_authorization(void) {
    stream_config_t allowed = create_camera("telemetry_allowed", "");
    stream_config_t denied = create_camera("telemetry_denied", "");
    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_user("telemetryuser", "password123", NULL,
        USER_ROLE_USER, true, &user_id));
    TEST_ASSERT_EQUAL_INT(0, db_authorization_set_user_mode(user_id, "policy"));
    char selector[256], grant_uuid[CAMERA_UUID_STRING_SIZE];
    snprintf(selector, sizeof(selector),
        "{\"version\":1,\"expression\":{\"op\":\"camera_uuid\",\"values\":[\"%s\"]}}", allowed.camera_uuid);
    create_grant(user_id, OPERATOR_ROLE_UUID, "selector", selector, grant_uuid);
    char api_key[128];
    TEST_ASSERT_EQUAL_INT(0, db_auth_generate_api_key(user_id, api_key, sizeof(api_key)));
    g_config.web_auth_enabled = true;
    player_telemetry_init();
    assert_player_telemetry_status(NULL, allowed.name, 401, 0);
    assert_player_telemetry_status(api_key, denied.name, 403, 0);
    assert_player_telemetry_status(api_key, allowed.name, 204, 1);
    player_telemetry_event_t event;
    TEST_ASSERT_EQUAL_INT(1, player_telemetry_snapshot(&event, 1));
    TEST_ASSERT_EQUAL_STRING(allowed.name, event.stream_name);
    TEST_ASSERT_EQUAL_STRING("test-session", event.session_id);
    TEST_ASSERT_EQUAL_STRING("hls", event.transport);
    TEST_ASSERT_TRUE(event.ttff_ms == 10.0);
    TEST_ASSERT_TRUE(event.timestamp > 0);

    api_token_t token;
    char secret[API_TOKEN_SECRET_MAX];
    create_scoped_token(user_id, authorization_action_bit(AUTHZ_PTZ_CONTROL),
        "all", NULL, &token, secret);
    assert_player_telemetry_status(secret, allowed.name, 403, 1);
    create_scoped_token(user_id, authorization_action_bit(AUTHZ_LIVE_VIEW),
        "selector", selector, &token, secret);
    assert_player_telemetry_status(secret, denied.name, 403, 1);
    assert_player_telemetry_status(secret, allowed.name, 204, 2);
}

static void assert_metrics_status(const char *api_key, int expected_status) {
    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    req.method = HTTP_METHOD_GET;
    safe_strcpy(req.path, "/api/metrics", sizeof(req.path), 0);
    safe_strcpy(req.method_str, "GET", sizeof(req.method_str), 0);
    safe_strcpy(req.client_ip, "127.0.0.1", sizeof(req.client_ip), 0);
    if (api_key) {
        safe_strcpy(req.headers[0].name, "X-API-Key", sizeof(req.headers[0].name), 0);
        safe_strcpy(req.headers[0].value, api_key, sizeof(req.headers[0].value), 0);
        req.num_headers = 1;
    }
    handle_get_metrics(&req, &res);
    int status = res.status_code;
    bool has_metrics = res.body && strstr((const char *)res.body,
                                          "# TYPE lightnvr_stream_up gauge");
    http_response_free(&res);
    TEST_ASSERT_EQUAL_INT(expected_status, status);
    if (expected_status == 200) TEST_ASSERT_TRUE(has_metrics);
}

void test_metrics_require_system_admin_authorization(void) {
    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_user("metricsoperator", "password123", NULL,
                                                USER_ROLE_USER, true, &user_id));
    TEST_ASSERT_EQUAL_INT(0, db_authorization_set_user_mode(user_id, "policy"));
    create_grant(user_id, OPERATOR_ROLE_UUID, "all", NULL, NULL);
    char api_key[128] = {0};
    TEST_ASSERT_EQUAL_INT(0, db_auth_generate_api_key(user_id, api_key, sizeof(api_key)));
    /* Initialize the endpoint's metrics dependency so the allowed case can
     * return a real Prometheus response. tearDown also handles assertion failures. */
    TEST_ASSERT_EQUAL_INT(0, metrics_init(4));
    g_config.web_auth_enabled = true;

    assert_metrics_status(NULL, 401);
    assert_metrics_status(api_key, 403);
    create_grant(user_id, ADMIN_ROLE_UUID, "all", NULL, NULL);
    assert_metrics_status(api_key, 200);
}

void test_stream_health_filters_camera_metrics_by_live_view_scope(void) {
    stream_config_t visible = create_camera("Health Visible", "Outdoor");
    stream_config_t hidden = create_camera("Health Hidden", "Indoor");
    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("healthviewer", "password123", NULL,
                               USER_ROLE_VIEWER, true, &user_id));
    TEST_ASSERT_EQUAL_INT(0,
                          db_authorization_set_user_mode(user_id, "policy"));
    char selector[512];
    snprintf(selector, sizeof(selector),
             "{\"version\":1,\"expression\":{\"op\":\"camera_uuid\","
             "\"values\":[\"%s\"]}}",
             visible.camera_uuid);
    create_grant(user_id, "00000000-0000-4000-8000-000000000003",
                 "selector", selector, NULL);
    char api_key[128] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_generate_api_key(user_id, api_key, sizeof(api_key)));

    TEST_ASSERT_EQUAL_INT(0, metrics_init(4));
    metrics_record_frame(visible.name, 1024, true);
    metrics_record_frame(hidden.name, 2048, true);
    g_config.web_auth_enabled = true;

    cJSON *json = call_handler_path(
        handle_get_health, HTTP_METHOD_GET, "/api/health", NULL, api_key, 200);
    cJSON *streams = cJSON_GetObjectItemCaseSensitive(json, "streams");
    cJSON *details = cJSON_GetObjectItemCaseSensitive(json, "streams_detail");
    TEST_ASSERT_EQUAL_INT(
        1, cJSON_GetObjectItemCaseSensitive(streams, "total")->valueint);
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(details));
    TEST_ASSERT_EQUAL_STRING(
        visible.name,
        cJSON_GetObjectItemCaseSensitive(cJSON_GetArrayItem(details, 0),
                                         "name")->valuestring);
    cJSON_Delete(json);

    json = call_handler_path(
        handle_get_health, HTTP_METHOD_GET,
        "/api/health?sparklines=true&history_only=true", NULL, api_key, 200);
    details = cJSON_GetObjectItemCaseSensitive(json, "streams_detail");
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(details));
    TEST_ASSERT_EQUAL_STRING(
        visible.name,
        cJSON_GetObjectItemCaseSensitive(cJSON_GetArrayItem(details, 0),
                                         "name")->valuestring);
    cJSON_Delete(json);

    /* Keep unauthenticated process probes healthy without disclosing cameras. */
    json = call_handler_path(handle_get_health, HTTP_METHOD_GET,
                             "/api/health", NULL, NULL, 200);
    streams = cJSON_GetObjectItemCaseSensitive(json, "streams");
    details = cJSON_GetObjectItemCaseSensitive(json, "streams_detail");
    TEST_ASSERT_EQUAL_INT(
        0, cJSON_GetObjectItemCaseSensitive(streams, "total")->valueint);
    TEST_ASSERT_EQUAL_INT(0, cJSON_GetArraySize(details));
    cJSON_Delete(json);

    g_config.web_auth_enabled = false;
    metrics_shutdown();
}

void test_user_create_rejects_retired_allowed_tags_field(void) {
    cJSON *json = call_handler_path(
        handle_users_create, HTTP_METHOD_POST, "/api/auth/users",
        "{\"username\":\"retiredtags\",\"password\":\"password123\","
        "\"role\":2,\"allowed_tags\":\"Outdoor\"}", NULL, 400);
    cJSON_Delete(json);
    user_t user;
    TEST_ASSERT_NOT_EQUAL(0,
                          db_auth_get_user_by_username("retiredtags", &user));
}

void test_user_create_rejects_noninteger_and_out_of_range_roles(void) {
    const char *roles[] = {"-1", "4", "0.5", "-0.5", "1e100", "-1e100",
                           "1e999", "\"2\"", "null", "true", "{}", "[]"};
    for (size_t i = 0; i < sizeof(roles) / sizeof(roles[0]); i++) {
        char body[256];
        snprintf(body, sizeof(body),
                 "{\"username\":\"badrole\",\"password\":\"password123\",\"role\":%s}", roles[i]);
        cJSON *json = call_handler_path(handle_users_create, HTTP_METHOD_POST,
                                        "/api/auth/users", body, NULL, 400);
        cJSON_Delete(json);
        user_t user;
        TEST_ASSERT_NOT_EQUAL(0, db_auth_get_user_by_username("badrole", &user));
    }
}

void test_user_create_accepts_integer_and_default_roles(void) {
    const char *roles[] = {",\"role\":0", ",\"role\":1", ",\"role\":2", ",\"role\":3", ""};
    for (size_t i = 0; i < sizeof(roles) / sizeof(roles[0]); i++) {
        char body[256];
        snprintf(body, sizeof(body),
                 "{\"username\":\"validrole%zu\",\"password\":\"password123\"%s}", i, roles[i]);
        cJSON *json = call_handler_path(handle_users_create, HTTP_METHOD_POST,
                                        "/api/auth/users", body, NULL, 200);
        TEST_ASSERT_EQUAL_INT(i == 4 ? USER_ROLE_USER : (int)i,
                             cJSON_GetObjectItem(json, "role")->valueint);
        cJSON_Delete(json);
    }
}

void test_user_update_rolls_back_every_field_and_sessions_on_failure(void) {
    int64_t target_id = 0;
    int64_t duplicate_id = 0;
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_user("atomicuser", "password123", NULL,
                                               USER_ROLE_VIEWER, true, &target_id));
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_user("duplicate", "password123", NULL,
                                               USER_ROLE_VIEWER, true, &duplicate_id));
    char token[128];
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_session(target_id, NULL, NULL, 3600, token, sizeof(token)));
    char path[128];
    snprintf(path, sizeof(path), "/api/auth/users/%lld", (long long)target_id);
    const char *body = "{\"email\":\"changed@example.com\",\"is_active\":false,"
                       "\"allowed_login_cidrs\":\"127.0.0.1\",\"password\":\"replacement123\"}";
    sqlite3 *db = get_db_handle();
    for (int failure = 0; failure < 5; failure++) {
        if (failure == 0) TEST_ASSERT_EQUAL_INT(0, db_auth_set_password_lock(target_id, true));
        if (failure == 1) TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(db,
            "CREATE TEMP TRIGGER fail_account_update BEFORE UPDATE OF allowed_login_cidrs ON users "
            "BEGIN SELECT RAISE(ABORT, 'CIDR write failed'); END;", NULL, NULL, NULL));
        if (failure == 2) TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(db,
            "CREATE TEMP TRIGGER fail_account_update BEFORE UPDATE OF password_hash ON users "
            "BEGIN SELECT RAISE(ABORT, 'password write failed'); END;", NULL, NULL, NULL));
        if (failure == 3) TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(db,
            "CREATE TEMP TRIGGER fail_account_update BEFORE DELETE ON sessions "
            "BEGIN SELECT RAISE(ABORT, 'session deletion failed'); END;", NULL, NULL, NULL));
        const char *request = failure == 4
            ? "{\"username\":\"duplicate\",\"password\":\"replacement123\"}" : body;
        cJSON *json = call_handler_path(handle_users_update, HTTP_METHOD_PUT, path,
                                        request, NULL, failure == 0 ? 403 : failure == 4 ? 409 : 500);
        cJSON_Delete(json);
        TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(db, "DROP TRIGGER IF EXISTS fail_account_update;", NULL, NULL, NULL));
        TEST_ASSERT_EQUAL_INT(0, db_auth_set_password_lock(target_id, false));
        user_t user;
        TEST_ASSERT_EQUAL_INT(0, db_auth_get_user_by_id(target_id, &user));
        TEST_ASSERT_EQUAL_STRING("atomicuser", user.username);
        TEST_ASSERT_EQUAL_STRING("", user.email);
        TEST_ASSERT_EQUAL_INT(USER_ROLE_VIEWER, user.role);
        TEST_ASSERT_TRUE(user.is_active);
        TEST_ASSERT_FALSE(user.has_login_cidr_restriction);
        int64_t authenticated_id = 0;
        TEST_ASSERT_EQUAL_INT(0, db_auth_authenticate("atomicuser", "password123", &authenticated_id));
        TEST_ASSERT_EQUAL_INT(0, db_auth_validate_session(token, &authenticated_id));
    }
    cJSON *json = call_handler_path(handle_users_update, HTTP_METHOD_PUT, path, body, NULL, 200);
    cJSON_Delete(json);
    user_t user;
    TEST_ASSERT_EQUAL_INT(0, db_auth_get_user_by_id(target_id, &user));
    TEST_ASSERT_EQUAL_STRING("changed@example.com", user.email);
    TEST_ASSERT_EQUAL_INT(USER_ROLE_VIEWER, user.role);
    TEST_ASSERT_FALSE(user.is_active);
    TEST_ASSERT_TRUE(user.has_login_cidr_restriction);
    int64_t authenticated_id = 0;
    TEST_ASSERT_NOT_EQUAL(0, db_auth_validate_session(token, &authenticated_id));
    TEST_ASSERT_EQUAL_INT(0, db_auth_update_user(target_id, NULL, NULL, -1, 1));
    TEST_ASSERT_EQUAL_INT(0, db_auth_authenticate("atomicuser", "replacement123", &authenticated_id));
}

void test_user_update_rejects_roles_before_changing_credentials(void) {
    int64_t target_id = 0;
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_user(
        "rolecheck", "password123", NULL, USER_ROLE_VIEWER, true, &target_id));
    char path[128];
    snprintf(path, sizeof(path), "/api/auth/users/%lld", (long long)target_id);
    char token[128];
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_session(
        target_id, NULL, NULL, 3600, token, sizeof(token)));
    const char *invalid_roles[] = {
        "0", "1", "2", "3", "-2", "-1", "-1.5", "-0.5", "0.5", "3.5", "4", "1e100",
        "\"2\"", "null", "true", "{}", "[]"
    };
    for (size_t i = 0; i < sizeof(invalid_roles) / sizeof(invalid_roles[0]); i++) {
        char body[256];
        snprintf(body, sizeof(body),
                 "{\"role\":%s,\"password\":\"replacement123\","
                 "\"email\":\"changed@example.com\"}", invalid_roles[i]);
        cJSON *json = call_handler_path(handle_users_update, HTTP_METHOD_PUT,
                                        path, body, NULL, 400);
        TEST_ASSERT_EQUAL_STRING("role updates are not supported via this endpoint; update authorization grants instead",
                                 cJSON_GetObjectItem(json, "error")->valuestring);
        cJSON_Delete(json);
        user_t user;
        TEST_ASSERT_EQUAL_INT(0, db_auth_get_user_by_id(target_id, &user));
        TEST_ASSERT_EQUAL_INT(USER_ROLE_VIEWER, user.role);
        TEST_ASSERT_EQUAL_STRING("", user.email);
        int64_t authenticated_id = 0;
        TEST_ASSERT_EQUAL_INT(0, db_auth_authenticate(
            "rolecheck", "password123", &authenticated_id));
        TEST_ASSERT_EQUAL_INT64(target_id, authenticated_id);
        TEST_ASSERT_EQUAL_INT(0, db_auth_validate_session(token, &authenticated_id));
    }
}

void test_user_update_rejects_nonstring_password_without_changes(void) {
    int64_t target_id = 0;
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_user("passwordtype", "password123", NULL,
                                               USER_ROLE_VIEWER, true, &target_id));
    char path[128];
    snprintf(path, sizeof(path), "/api/auth/users/%lld", (long long)target_id);
    const char *values[] = {"null", "true", "42", "{}", "[]"};
    for (size_t i = 0; i < sizeof(values) / sizeof(values[0]); i++) {
        char body[256];
        snprintf(body, sizeof(body), "{\"email\":\"changed@example.com\",\"password\":%s}", values[i]);
        cJSON *json = call_handler_path(handle_users_update, HTTP_METHOD_PUT, path, body, NULL, 400);
        TEST_ASSERT_EQUAL_STRING("password must be a string", cJSON_GetObjectItem(json, "error")->valuestring);
        cJSON_Delete(json);
        user_t user;
        TEST_ASSERT_EQUAL_INT(0, db_auth_get_user_by_id(target_id, &user));
        TEST_ASSERT_EQUAL_STRING("", user.email);
        int64_t authenticated_id = 0;
        TEST_ASSERT_EQUAL_INT(0, db_auth_authenticate("passwordtype", "password123", &authenticated_id));
    }
    cJSON *json = call_handler_path(handle_users_update, HTTP_METHOD_PUT, path,
                                    "{\"password\":\"\",\"email\":\"updated@example.com\"}", NULL, 200);
    cJSON_Delete(json);
    int64_t authenticated_id = 0;
    TEST_ASSERT_EQUAL_INT(0, db_auth_authenticate("passwordtype", "password123", &authenticated_id));
}

void test_user_creation_requires_actions_and_all_camera_scope(void) {
    int64_t manager_id = 0;
    // Numeric Admin metadata must not bypass the actual policy grants.
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_user("creationmanager", "password123", NULL,
                                               USER_ROLE_ADMIN, true, &manager_id));
    TEST_ASSERT_EQUAL_INT(0, db_authorization_set_user_mode(manager_id, "policy"));
    authorization_role_t manager_role;
    memset(&manager_role, 0, sizeof(manager_role));
    safe_strcpy(manager_role.name, "Creation manager", sizeof(manager_role.name), 0);
    manager_role.action_mask = authorization_action_bit(AUTHZ_USERS_MANAGE);
    int64_t version = 0, new_version = 0;
    TEST_ASSERT_EQUAL_INT(0, db_authorization_get_policy_version(&version));
    TEST_ASSERT_EQUAL_INT(DB_AUTHORIZATION_OK,
        db_authorization_role_create(&manager_role, version, &new_version));
    create_grant(manager_id, manager_role.uuid, "all", NULL, NULL);
    char api_key[128];
    TEST_ASSERT_EQUAL_INT(0, db_auth_generate_api_key(manager_id, api_key, sizeof(api_key)));
    g_config.web_auth_enabled = true;
    for (int role = 0; role <= 3; role++) {
        char body[256];
        snprintf(body, sizeof(body),
                 "{\"username\":\"deniedchild\",\"password\":\"password123\",\"role\":%d}", role);
        cJSON *json = call_handler_path(handle_users_create, HTTP_METHOD_POST,
                                        "/api/auth/users", body, api_key, 403);
        cJSON_Delete(json);
        user_t user;
        TEST_ASSERT_NOT_EQUAL(0, db_auth_get_user_by_username("deniedchild", &user));
    }
    stream_config_t camera = create_camera("Creation scope", "Test");
    char selector[512];
    snprintf(selector, sizeof(selector),
             "{\"version\":1,\"expression\":{\"op\":\"camera_uuid\",\"values\":[\"%s\"]}}", camera.camera_uuid);
    const char *viewer_role = "00000000-0000-4000-8000-000000000003";
    create_grant(manager_id, viewer_role, "selector", selector, NULL);
    const char *viewer_body = "{\"username\":\"viewerchild\",\"password\":\"password123\",\"role\":2}";
    cJSON *json = call_handler_path(handle_users_create, HTTP_METHOD_POST,
                                    "/api/auth/users", viewer_body, api_key, 403);
    cJSON_Delete(json);
    // A bounded manager can create a role it actually holds over all cameras.
    create_grant(manager_id, viewer_role, "all", NULL, NULL);
    json = call_handler_path(handle_users_create, HTTP_METHOD_POST,
                             "/api/auth/users", viewer_body, api_key, 200);
    cJSON_Delete(json);
    // The default Operator role must be checked too.
    json = call_handler_path(handle_users_create, HTTP_METHOD_POST, "/api/auth/users",
        "{\"username\":\"defaultchild\",\"password\":\"password123\"}", api_key, 403);
    cJSON_Delete(json);
    create_grant(manager_id, ADMIN_ROLE_UUID, "all", NULL, NULL);
    json = call_handler_path(handle_users_create, HTTP_METHOD_POST, "/api/auth/users",
        "{\"username\":\"adminchild\",\"password\":\"password123\",\"role\":0}", api_key, 200);
    cJSON_Delete(json);
    user_t manager;
    TEST_ASSERT_EQUAL_INT(0, db_auth_get_user_by_id(manager_id, &manager));
    uint64_t mask = 0;
    TEST_ASSERT_EQUAL_INT(0, authorization_effective_action_mask(&manager, &mask));
    api_token_t token;
    char secret[API_TOKEN_SECRET_MAX];
    create_scoped_token(manager_id, mask, "all", NULL, &token, secret);
    json = call_handler_path(handle_users_create, HTTP_METHOD_POST, "/api/auth/users",
        "{\"username\":\"tokenchild\",\"password\":\"password123\",\"role\":0}", secret, 403);
    cJSON_Delete(json);
    g_config.web_auth_enabled = false;
}

void test_user_update_preserves_omitted_role(void) {
    int64_t target_id = 0;
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_user(
        "rolecheck", "password123", NULL, USER_ROLE_VIEWER, true, &target_id));
    char path[128];
    snprintf(path, sizeof(path), "/api/auth/users/%lld", (long long)target_id);
    cJSON *json = call_handler_path(handle_users_update, HTTP_METHOD_PUT,
                                    path, "{\"email\":\"new@example.com\"}", NULL, 200);
    TEST_ASSERT_EQUAL_INT(USER_ROLE_VIEWER, cJSON_GetObjectItem(json, "role")->valueint);
    TEST_ASSERT_EQUAL_STRING("new@example.com", cJSON_GetObjectItem(json, "email")->valuestring);
    cJSON_Delete(json);
}

void test_password_change_supports_large_user_ids(void) {
    const int64_t ids[] = {INT64_C(1000000), INT64_MAX};
    for (size_t i = 0; i < sizeof(ids) / sizeof(ids[0]); i++) {
        int64_t initial_id = 0;
        TEST_ASSERT_EQUAL_INT(0, db_auth_create_user("largeid", "password123", NULL,
                                                   USER_ROLE_VIEWER, true, &initial_id));
        sqlite3_stmt *stmt = NULL;
        TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(get_db_handle(),
            "UPDATE users SET id = ? WHERE id = ?", -1, &stmt, NULL));
        sqlite3_bind_int64(stmt, 1, ids[i]);
        sqlite3_bind_int64(stmt, 2, initial_id);
        TEST_ASSERT_EQUAL_INT(SQLITE_DONE, sqlite3_step(stmt));
        sqlite3_finalize(stmt);
        char token[128];
        TEST_ASSERT_EQUAL_INT(0, db_auth_create_session(ids[i], NULL, NULL, 3600, token, sizeof(token)));
        char path[128];
        snprintf(path, sizeof(path), "/api/auth/users/%lld/password", (long long)ids[i]);
        cJSON *json = call_handler_path(handle_users_change_password, HTTP_METHOD_PUT,
                                        path, "{\"new_password\":\"replacement123\"}", NULL, 200);
        cJSON_Delete(json);
        TEST_ASSERT_EQUAL_INT(0, db_auth_verify_password(ids[i], "replacement123"));
        TEST_ASSERT_NOT_EQUAL(0, db_auth_verify_password(ids[i], "password123"));
        TEST_ASSERT_NOT_EQUAL(0, db_auth_validate_session(token, NULL));
        TEST_ASSERT_EQUAL_INT(0, db_auth_delete_user(ids[i]));
    }
}

void test_password_change_rejects_malformed_and_overflowing_ids(void) {
    int64_t target_id = 0;
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_user("malformedid", "password123", NULL,
                                               USER_ROLE_VIEWER, true, &target_id));
    sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(get_db_handle(),
        "UPDATE users SET id = ? WHERE id = ?", -1, &stmt, NULL));
    sqlite3_bind_int64(stmt, 1, INT64_MAX);
    sqlite3_bind_int64(stmt, 2, target_id);
    TEST_ASSERT_EQUAL_INT(SQLITE_DONE, sqlite3_step(stmt));
    sqlite3_finalize(stmt);
    const char *ids[] = {"9223372036854775808", "9999999999999999999", "-1", "0",
                         "abc", "9223372036854775807x", "+9223372036854775807",
                         "%209223372036854775807"};
    for (size_t i = 0; i < sizeof(ids) / sizeof(ids[0]); i++) {
        char path[128];
        snprintf(path, sizeof(path), "/api/auth/users/%s/password", ids[i]);
        cJSON *json = call_handler_path(handle_users_change_password, HTTP_METHOD_PUT,
                                        path, "{\"new_password\":\"replacement123\"}", NULL, 400);
        cJSON_Delete(json);
        // strtoll saturation or a numeric prefix must never select this user.
        TEST_ASSERT_EQUAL_INT(0, db_auth_verify_password(INT64_MAX, "password123"));
    }
}

void test_user_routes_reject_malformed_ids_without_mutation(void) {
    int64_t target_id = 0;
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_user("routevalidation", "password123", NULL,
                                               USER_ROLE_VIEWER, true, &target_id));
    char original_key[64];
    TEST_ASSERT_EQUAL_INT(0, db_auth_generate_api_key(target_id, original_key, sizeof(original_key)));
    struct {
        void (*handler)(const http_request_t *, http_response_t *);
        http_method_t method;
        const char *suffix;
        const char *body;
    } routes[] = {
        {handle_users_get, HTTP_METHOD_GET, "", NULL},
        {handle_users_update, HTTP_METHOD_PUT, "", "{\"email\":\"changed@example.com\"}"},
        {handle_users_delete, HTTP_METHOD_DELETE, "", NULL},
        {handle_users_generate_api_key, HTTP_METHOD_POST, "/api-key", NULL},
        {handle_users_change_password, HTTP_METHOD_PUT, "/password", "{\"new_password\":\"replacement123\"}"},
        {handle_users_password_lock, HTTP_METHOD_PUT, "/password-lock", "{\"locked\":true}"},
        {handle_users_clear_login_lockout, HTTP_METHOD_POST, "/login-lockout/clear", NULL}
    };
    char malformed[7][80];
    snprintf(malformed[0], sizeof(malformed[0]), "%lldx", (long long)target_id);
    snprintf(malformed[1], sizeof(malformed[1]), "+%lld", (long long)target_id);
    snprintf(malformed[2], sizeof(malformed[2]), "%%20%lld", (long long)target_id);
    snprintf(malformed[3], sizeof(malformed[3]), "%lld%%00junk", (long long)target_id);
    snprintf(malformed[4], sizeof(malformed[4]), "%lld/extra", (long long)target_id);
    snprintf(malformed[5], sizeof(malformed[5]), " %lld", (long long)target_id);
    snprintf(malformed[6], sizeof(malformed[6]), "%lld%%2fextra", (long long)target_id);
    const char *ids[] = {"0", "-1", "abc", "9223372036854775808",
                         "9999999999999999999999999999999999999999",
                         malformed[0], malformed[1], malformed[2], malformed[3],
                         malformed[4], malformed[5], malformed[6]};
    for (size_t route = 0; route < sizeof(routes) / sizeof(routes[0]); ++route) {
        for (size_t i = 0; i < sizeof(ids) / sizeof(ids[0]); ++i) {
            char path[160];
            snprintf(path, sizeof(path), "/api/auth/users/%s%s", ids[i], routes[route].suffix);
            cJSON *json = call_handler_path(routes[route].handler, routes[route].method,
                                            path, routes[route].body, NULL, 400);
            cJSON_Delete(json);
            user_t user;
            TEST_ASSERT_EQUAL_INT(0, db_auth_get_user_by_id(target_id, &user));
            TEST_ASSERT_EQUAL_STRING(original_key, user.api_key);
            TEST_ASSERT_FALSE(user.password_change_locked);
            TEST_ASSERT_TRUE(strcmp(user.email, "changed@example.com") != 0);
        }
        char path[160];
        snprintf(path, sizeof(path), "/api/auth/users/%lld%s/unexpected",
                 (long long)target_id, routes[route].suffix);
        cJSON_Delete(call_handler_path(routes[route].handler, routes[route].method,
                                       path, routes[route].body, NULL, 400));
    }
    TEST_ASSERT_EQUAL_INT(0, db_auth_verify_password(target_id, "password123"));
}

void test_user_routes_accept_full_width_ids_and_exact_suffixes(void) {
    const int64_t ids[] = {INT64_C(2147483648), INT64_MAX};
    for (size_t i = 0; i < sizeof(ids) / sizeof(ids[0]); ++i) {
        int64_t initial_id;
        TEST_ASSERT_EQUAL_INT(0, db_auth_create_user("routeboundary", "password123", NULL,
                                                   USER_ROLE_VIEWER, true, &initial_id));
        sqlite3_stmt *stmt = NULL;
        TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(get_db_handle(),
            "UPDATE users SET id = ? WHERE id = ?", -1, &stmt, NULL));
        sqlite3_bind_int64(stmt, 1, ids[i]);
        sqlite3_bind_int64(stmt, 2, initial_id);
        TEST_ASSERT_EQUAL_INT(SQLITE_DONE, sqlite3_step(stmt));
        sqlite3_finalize(stmt);
        char path[128];
        snprintf(path, sizeof(path), "/api/auth/users/%lld/?check=1", (long long)ids[i]);
        cJSON_Delete(call_handler_path(handle_users_get, HTTP_METHOD_GET, path, NULL, NULL, 200));
        cJSON_Delete(call_handler_path(handle_users_update, HTTP_METHOD_PUT, path,
                                       "{\"email\":\"boundary@example.com\"}", NULL, 200));
        snprintf(path, sizeof(path), "/api/auth/users/%lld/api-key", (long long)ids[i]);
        cJSON_Delete(call_handler_path(handle_users_generate_api_key, HTTP_METHOD_POST,
                                       path, NULL, NULL, 200));
        snprintf(path, sizeof(path), "/api/auth/users/%lld/password-lock", (long long)ids[i]);
        cJSON_Delete(call_handler_path(handle_users_password_lock, HTTP_METHOD_PUT,
                                       path, "{\"locked\":true}", NULL, 200));
        user_t user;
        TEST_ASSERT_EQUAL_INT(0, db_auth_get_user_by_id(ids[i], &user));
        TEST_ASSERT_TRUE(user.password_change_locked);
        TEST_ASSERT_EQUAL_STRING("boundary@example.com", user.email);
        TEST_ASSERT_TRUE(user.api_key[0] != '\0');
        cJSON_Delete(call_handler_path(handle_users_password_lock, HTTP_METHOD_PUT,
                                       path, "{\"locked\":false}", NULL, 200));
        snprintf(path, sizeof(path), "/api/auth/users/%lld/login-lockout/clear", (long long)ids[i]);
        cJSON_Delete(call_handler_path(handle_users_clear_login_lockout, HTTP_METHOD_POST,
                                       path, NULL, NULL, 200));
        snprintf(path, sizeof(path), "/api/auth/users/%lld", (long long)ids[i]);
        cJSON_Delete(call_handler_path(handle_users_delete, HTTP_METHOD_DELETE, path, NULL, NULL, 200));
        TEST_ASSERT_NOT_EQUAL(0, db_auth_get_user_by_id(ids[i], &user));
    }
}

void test_users_manage_grant_controls_cross_user_password_and_totp(void) {
    int64_t manager_id = 0;
    int64_t target_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("grantmanager", "password123", NULL,
                               USER_ROLE_VIEWER, true, &manager_id));
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("granttarget", "password123", NULL,
                               USER_ROLE_VIEWER, true, &target_id));
    TEST_ASSERT_EQUAL_INT(0,
                          db_authorization_set_user_mode(manager_id, "policy"));
    create_grant(manager_id, ADMIN_ROLE_UUID, "all", NULL, NULL);
    char api_key[128] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_generate_api_key(manager_id, api_key, sizeof(api_key)));
    char path[128];
    snprintf(path, sizeof(path), "/api/auth/users/%lld/password",
             (long long)target_id);
    g_config.web_auth_enabled = true;
    cJSON *json = call_handler_path(
        handle_users_change_password, HTTP_METHOD_PUT, path,
        "{\"new_password\":\"replacement123\"}", api_key, 200);
    cJSON_Delete(json);
    snprintf(path, sizeof(path), "/api/auth/users/%lld/totp/status",
             (long long)target_id);
    json = call_handler_path(handle_totp_status, HTTP_METHOD_GET, path, NULL,
                             api_key, 200);
    cJSON_Delete(json);
    g_config.web_auth_enabled = false;
}

void test_scoped_token_cannot_use_identity_self_service_or_other_progress(void) {
    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("scopedself", "password123", NULL,
                               USER_ROLE_USER, true, &user_id));
    TEST_ASSERT_EQUAL_INT(0,
                          db_authorization_set_user_mode(user_id, "policy"));
    create_grant(user_id, OPERATOR_ROLE_UUID, "all", NULL, NULL);
    api_token_t first_token;
    api_token_t second_token;
    char first_secret[API_TOKEN_SECRET_MAX];
    char second_secret[API_TOKEN_SECRET_MAX];
    create_scoped_token(user_id, authorization_action_bit(AUTHZ_LIVE_VIEW),
                        "all", NULL, &first_token, first_secret);
    create_scoped_token(user_id, authorization_action_bit(AUTHZ_LIVE_VIEW),
                        "all", NULL, &second_token, second_secret);

    char path[192];
    snprintf(path, sizeof(path), "/api/auth/users/%lld", (long long)user_id);
    g_config.web_auth_enabled = true;
    cJSON *json = call_handler_path(handle_users_update, HTTP_METHOD_PUT, path,
                                    "{\"email\":\"new@example.com\"}",
                                    first_secret, 403);
    cJSON_Delete(json);
    snprintf(path, sizeof(path), "/api/auth/users/%lld/password",
             (long long)user_id);
    json = call_handler_path(
        handle_users_change_password, HTTP_METHOD_PUT, path,
        "{\"old_password\":\"password123\",\"new_password\":\"replacement123\"}",
        first_secret, 403);
    cJSON_Delete(json);
    snprintf(path, sizeof(path), "/api/auth/users/%lld/totp/setup",
             (long long)user_id);
    json = call_handler_path(handle_totp_setup, HTTP_METHOD_POST, path, "{}",
                             first_secret, 403);
    cJSON_Delete(json);

    char job_id[64];
    TEST_ASSERT_EQUAL_INT(
        0, batch_delete_progress_create_job_for_principal(
               1, user_id, first_token.uuid, job_id));
    snprintf(path, sizeof(path),
             "/api/recordings/batch-delete/progress/%s", job_id);
    json = call_handler_path(handle_batch_delete_progress, HTTP_METHOD_GET,
                             path, NULL, first_secret, 200);
    cJSON_Delete(json);
    json = call_handler_path(handle_batch_delete_progress, HTTP_METHOD_GET,
                             path, NULL, second_secret, 404);
    cJSON_Delete(json);
    g_config.web_auth_enabled = false;
}

void test_recording_file_check_requires_scoped_replay_access(void) {
    stream_config_t allowed = create_camera("File Check Allowed", "Review");
    stream_config_t denied = create_camera("File Check Denied", "Review");
    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("filechecker", "password123", NULL,
                               USER_ROLE_USER, true, &user_id));
    TEST_ASSERT_EQUAL_INT(0,
                          db_authorization_set_user_mode(user_id, "policy"));
    char selector[512];
    snprintf(selector, sizeof(selector),
             "{\"version\":1,\"expression\":{\"op\":\"camera_uuid\","
             "\"values\":[\"%s\"]}}",
             allowed.camera_uuid);
    create_grant(user_id, OPERATOR_ROLE_UUID, "selector", selector, NULL);
    api_token_t token;
    char secret[API_TOKEN_SECRET_MAX];
    create_scoped_token(
        user_id, authorization_action_bit(AUTHZ_RECORDINGS_REPLAY), "all",
        NULL, &token, secret);

    recording_metadata_t recording;
    memset(&recording, 0, sizeof(recording));
    safe_strcpy(recording.stream_name, allowed.name,
                sizeof(recording.stream_name), 0);
    safe_strcpy(recording.camera_uuid, allowed.camera_uuid,
                sizeof(recording.camera_uuid), 0);
    safe_strcpy(recording.file_path, "/tmp/lightnvr-file-check-allowed.mp4",
                sizeof(recording.file_path), 0);
    recording.start_time = 100;
    recording.end_time = 200;
    recording.is_complete = true;
    recording.retention_override_days = -1;
    TEST_ASSERT_NOT_EQUAL(0, add_recording_metadata(&recording));
    safe_strcpy(recording.stream_name, denied.name,
                sizeof(recording.stream_name), 0);
    safe_strcpy(recording.camera_uuid, denied.camera_uuid,
                sizeof(recording.camera_uuid), 0);
    safe_strcpy(recording.file_path, "/tmp/lightnvr-file-check-denied.mp4",
                sizeof(recording.file_path), 0);
    TEST_ASSERT_NOT_EQUAL(0, add_recording_metadata(&recording));

    g_config.web_auth_enabled = true;
    cJSON *json = call_handler_path(
        handle_check_recording_file, HTTP_METHOD_GET,
        "/api/recordings/files/check?path=/tmp/lightnvr-file-check-allowed.mp4",
        NULL, NULL, 401);
    cJSON_Delete(json);
    json = call_handler_path(
        handle_check_recording_file, HTTP_METHOD_GET,
        "/api/recordings/files/check?path=/tmp/lightnvr-file-check-denied.mp4",
        NULL, secret, 403);
    cJSON_Delete(json);
    json = call_handler_path(
        handle_check_recording_file, HTTP_METHOD_GET,
        "/api/recordings/files/check?path=/tmp/lightnvr-file-check-allowed.mp4",
        NULL, secret, 200);
    cJSON_Delete(json);
    g_config.web_auth_enabled = false;
}

void test_collection_resolution_uses_requested_replay_action(void) {
    stream_config_t camera = create_camera("Replay Only Collection", "Review");
    camera_collection_t collection;
    memset(&collection, 0, sizeof(collection));
    safe_strcpy(collection.name, "Replay only", sizeof(collection.name), 0);
    safe_strcpy(collection.collection_type, "static",
                sizeof(collection.collection_type), 0);
    collection.is_shared = true;
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_OK,
                          db_camera_collection_create(&collection));
    const char *members[] = {camera.camera_uuid};
    TEST_ASSERT_EQUAL_INT(
        DB_CAMERA_COLLECTION_OK,
        db_camera_collection_set_members(collection.uuid, members, 1));

    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("replayonly", "password123", NULL,
                               USER_ROLE_VIEWER, true, &user_id));
    authorization_role_t replay_role;
    memset(&replay_role, 0, sizeof(replay_role));
    safe_strcpy(replay_role.name, "Replay only test role",
                sizeof(replay_role.name), 0);
    replay_role.action_mask = authorization_action_bit(AUTHZ_RECORDINGS_REPLAY);
    int64_t version = 0;
    int64_t new_version = 0;
    TEST_ASSERT_EQUAL_INT(0, db_authorization_get_policy_version(&version));
    TEST_ASSERT_EQUAL_INT(
        DB_AUTHORIZATION_OK,
        db_authorization_role_create(&replay_role, version, &new_version));
    TEST_ASSERT_EQUAL_INT(0,
                          db_authorization_set_user_mode(user_id, "policy"));
    create_grant(user_id, replay_role.uuid, "all", NULL, NULL);
    user_t user;
    TEST_ASSERT_EQUAL_INT(0, db_auth_get_user_by_id(user_id, &user));

    char **stream_names = NULL;
    int stream_count = 0;
    g_config.web_auth_enabled = true;
    TEST_ASSERT_EQUAL_INT(
        CAMERA_COLLECTION_FILTER_OK,
        camera_collection_filter_resolve_stream_names_for_action(
            collection.uuid, &user, AUTHZ_RECORDINGS_REPLAY, &stream_names,
            &stream_count));
    TEST_ASSERT_EQUAL_INT(1, stream_count);
    TEST_ASSERT_EQUAL_STRING(camera.name, stream_names[0]);
    camera_collection_filter_free_stream_names(stream_names, stream_count);
    stream_names = NULL;
    stream_count = 0;
    TEST_ASSERT_EQUAL_INT(
        CAMERA_COLLECTION_FILTER_OK,
        camera_collection_filter_resolve_stream_names(
            collection.uuid, &user, &stream_names, &stream_count));
    TEST_ASSERT_EQUAL_INT(0, stream_count);
    camera_collection_filter_free_stream_names(stream_names, stream_count);
    g_config.web_auth_enabled = false;
}

void test_delegated_policy_scope_cannot_widen_or_chain_scoped_tokens(void) {
    stream_config_t allowed = create_camera("Delegation Allowed", "Delegate");
    int64_t manager_id = 0;
    int64_t target_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("scopedmanager", "password123", NULL,
                               USER_ROLE_USER, true, &manager_id));
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("delegatetarget", "password123", NULL,
                               USER_ROLE_VIEWER, true, &target_id));

    authorization_role_t user_manager;
    memset(&user_manager, 0, sizeof(user_manager));
    safe_strcpy(user_manager.name, "Scoped delegation user manager",
                sizeof(user_manager.name), 0);
    user_manager.action_mask =
        authorization_action_bit(AUTHZ_USERS_MANAGE);
    int64_t version = 0;
    int64_t new_version = 0;
    TEST_ASSERT_EQUAL_INT(0, db_authorization_get_policy_version(&version));
    TEST_ASSERT_EQUAL_INT(
        DB_AUTHORIZATION_OK,
        db_authorization_role_create(&user_manager, version, &new_version));
    TEST_ASSERT_EQUAL_INT(0,
                          db_authorization_set_user_mode(manager_id, "policy"));
    TEST_ASSERT_EQUAL_INT(0,
                          db_authorization_set_user_mode(target_id, "policy"));
    create_grant(manager_id, user_manager.uuid, "all", NULL, NULL);
    char selector[512];
    snprintf(selector, sizeof(selector),
             "{\"version\":1,\"expression\":{\"op\":\"camera_uuid\","
             "\"values\":[\"%s\"]}}",
             allowed.camera_uuid);
    create_grant(manager_id, OPERATOR_ROLE_UUID, "selector", selector, NULL);
    char manager_key[128] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_generate_api_key(manager_id, manager_key,
                                    sizeof(manager_key)));

    TEST_ASSERT_EQUAL_INT(0, db_authorization_get_policy_version(&version));
    char policy_path[128];
    char policy_body[768];
    snprintf(policy_path, sizeof(policy_path),
             "/api/authorization/users/%lld", (long long)target_id);
    snprintf(policy_body, sizeof(policy_body),
             "{\"expected_policy_version\":%lld,\"mode\":\"policy\","
             "\"grants\":[{\"role_uuid\":\"%s\","
             "\"scope\":{\"type\":\"all\"}}]}",
             (long long)version, OPERATOR_ROLE_UUID);
    g_config.web_auth_enabled = true;
    cJSON *json = call_handler_path(
        handle_put_user_authorization, HTTP_METHOD_PUT, policy_path,
        policy_body, manager_key, 403);
    cJSON_Delete(json);

    char invalid_policy_body[256];
    snprintf(invalid_policy_body, sizeof(invalid_policy_body),
             "{\"expected_policy_version\":%lld,\"mode\":\"legacy\","
             "\"grants\":[]}",
             (long long)version);
    json = call_handler_path(handle_put_user_authorization, HTTP_METHOD_PUT,
                             policy_path, invalid_policy_body, manager_key,
                             400);
    cJSON_Delete(json);

    char unknown_role_policy_body[512];
    snprintf(unknown_role_policy_body, sizeof(unknown_role_policy_body),
             "{\"expected_policy_version\":%lld,\"mode\":\"policy\","
             "\"grants\":[{\"role_uuid\":"
             "\"00000000-0000-4000-8000-000000000099\","
             "\"scope\":{\"type\":\"all\"}}]}",
             (long long)version);
    json = call_handler_path(handle_put_user_authorization, HTTP_METHOD_PUT,
                             policy_path, unknown_role_policy_body,
                             manager_key, 400);
    cJSON_Delete(json);

    char manager_policy_path[128];
    char empty_policy_body[256];
    snprintf(manager_policy_path, sizeof(manager_policy_path),
             "/api/authorization/users/%lld", (long long)manager_id);
    snprintf(empty_policy_body, sizeof(empty_policy_body),
             "{\"expected_policy_version\":%lld,\"mode\":\"policy\","
             "\"grants\":[]}",
             (long long)version);
    json = call_handler_path(handle_put_user_authorization, HTTP_METHOD_PUT,
                             manager_policy_path, empty_policy_body,
                             manager_key, 409);
    cJSON_Delete(json);

    char stale_policy_body[1024];
    snprintf(stale_policy_body, sizeof(stale_policy_body),
             "{\"expected_policy_version\":%lld,\"mode\":\"policy\","
             "\"grants\":[{\"role_uuid\":\"%s\","
             "\"scope\":{\"type\":\"selector\",\"selector\":%s}}]}",
             (long long)(version - 1), OPERATOR_ROLE_UUID, selector);
    json = call_handler_path(handle_put_user_authorization, HTTP_METHOD_PUT,
                             policy_path, stale_policy_body, manager_key,
                             409);
    cJSON_Delete(json);

    char token_path[128];
    snprintf(token_path, sizeof(token_path),
             "/api/authorization/users/%lld/tokens", (long long)manager_id);
    char token_body[512];
    snprintf(token_body, sizeof(token_body),
             "{\"description\":\"No chaining\",\"expires_at\":%lld,"
             "\"actions\":[\"users.manage\"],"
             "\"scope\":{\"type\":\"all\"}}",
             (long long)time(NULL) + 3600);
    json = call_handler_path(handle_post_user_api_token, HTTP_METHOD_POST,
                             token_path, token_body, manager_key, 201);
    const cJSON *secret = cJSON_GetObjectItemCaseSensitive(json, "secret");
    TEST_ASSERT_TRUE(cJSON_IsString(secret));
    char scoped_secret[API_TOKEN_SECRET_MAX];
    safe_strcpy(scoped_secret, secret->valuestring, sizeof(scoped_secret), 0);
    cJSON_Delete(json);

    json = call_handler_path(handle_put_user_authorization, HTTP_METHOD_PUT,
                             policy_path, policy_body, scoped_secret, 403);
    cJSON_Delete(json);
    json = call_handler_path(handle_post_user_api_token, HTTP_METHOD_POST,
                             token_path, token_body, scoped_secret, 403);
    cJSON_Delete(json);

    char overbroad_body[512];
    snprintf(overbroad_body, sizeof(overbroad_body),
             "{\"description\":\"Scope expansion\",\"expires_at\":%lld,"
             "\"actions\":[\"camera.configure\"],"
             "\"scope\":{\"type\":\"all\"}}",
             (long long)time(NULL) + 3600);
    json = call_handler_path(handle_post_user_api_token, HTTP_METHOD_POST,
                             token_path, overbroad_body, manager_key, 403);
    cJSON_Delete(json);
    char invalid_body[512];
    snprintf(invalid_body, sizeof(invalid_body),
             "{\"description\":\"Invalid actions\",\"expires_at\":%lld,"
             "\"actions\":[],\"scope\":{\"type\":\"all\"}}",
             (long long)time(NULL) + 3600);
    json = call_handler_path(handle_post_user_api_token, HTTP_METHOD_POST,
                             token_path, invalid_body, manager_key, 400);
    cJSON_Delete(json);

    const char *missing_token_uuid =
        "00000000-0000-4000-8000-000000000098";
    char missing_token_path[192];
    snprintf(missing_token_path, sizeof(missing_token_path),
             "/api/authorization/users/%lld/tokens/%s",
             (long long)manager_id, missing_token_uuid);
    json = call_handler_path(handle_delete_user_api_token,
                             HTTP_METHOD_DELETE, missing_token_path, NULL,
                             manager_key, 404);
    cJSON_Delete(json);

    audit_query_t query = {.page = 1, .page_size = 50};
    safe_strcpy(query.action, "authorization.policy.update",
                sizeof(query.action), 0);
    safe_strcpy(query.outcome, "denied", sizeof(query.outcome), 0);
    audit_page_t page;
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    TEST_ASSERT_EQUAL_INT64(2, page.total);
    char target_text[32];
    snprintf(target_text, sizeof(target_text), "%lld", (long long)target_id);
    bool policy_scope_denied = false;
    bool scoped_policy_denied = false;
    for (int i = 0; i < page.count; i++) {
        TEST_ASSERT_EQUAL_STRING(target_text, page.events[i].target_uuid);
        policy_scope_denied |= strstr(
            page.events[i].details_json,
            "\"reason\":\"scope_exceeds_authority\"") != NULL;
        scoped_policy_denied |= strstr(
            page.events[i].details_json,
            "\"reason\":\"scoped_token_policy_mutation\"") != NULL;
        TEST_ASSERT_NULL(strstr(page.events[i].details_json, scoped_secret));
        TEST_ASSERT_NULL(strstr(page.events[i].details_json, manager_key));
    }
    TEST_ASSERT_TRUE(policy_scope_denied);
    TEST_ASSERT_TRUE(scoped_policy_denied);
    db_audit_page_free(&page);

    memset(&query, 0, sizeof(query));
    query.page = 1;
    query.page_size = 50;
    safe_strcpy(query.action, "authorization.policy.update",
                sizeof(query.action), 0);
    safe_strcpy(query.outcome, "failure", sizeof(query.outcome), 0);
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    bool invalid_mode_failed = false;
    bool invalid_role_failed = false;
    bool self_lockout_failed = false;
    bool conflict_failed = false;
    for (int i = 0; i < page.count; i++) {
        invalid_mode_failed |= strstr(
            page.events[i].details_json,
            "\"reason\":\"invalid_authorization_mode\"") != NULL;
        invalid_role_failed |= strstr(
            page.events[i].details_json,
            "\"reason\":\"invalid_grant_role\"") != NULL;
        self_lockout_failed |= strstr(
            page.events[i].details_json,
            "\"reason\":\"self_lockout_prevented\"") != NULL;
        conflict_failed |= strstr(
            page.events[i].details_json,
            "\"reason\":\"policy_version_conflict\"") != NULL;
        TEST_ASSERT_NULL(strstr(page.events[i].details_json, scoped_secret));
        TEST_ASSERT_NULL(strstr(page.events[i].details_json, manager_key));
    }
    TEST_ASSERT_TRUE(invalid_mode_failed);
    TEST_ASSERT_TRUE(invalid_role_failed);
    TEST_ASSERT_TRUE(self_lockout_failed);
    TEST_ASSERT_TRUE(conflict_failed);
    db_audit_page_free(&page);

    memset(&query, 0, sizeof(query));
    query.page = 1;
    query.page_size = 50;
    safe_strcpy(query.action, "api_token.create", sizeof(query.action), 0);
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    bool chaining_denied = false;
    bool scope_denied = false;
    bool validation_failed = false;
    for (int i = 0; i < page.count; i++) {
        chaining_denied |= strcmp(page.events[i].outcome, "denied") == 0 &&
            strstr(page.events[i].details_json,
                   "scoped_token_management_denied") != NULL;
        scope_denied |= strcmp(page.events[i].outcome, "denied") == 0 &&
            strstr(page.events[i].details_json,
                   "scope_exceeds_authority") != NULL;
        validation_failed |= strcmp(page.events[i].outcome, "failure") == 0 &&
            strstr(page.events[i].details_json,
                   "invalid_actions_or_scope") != NULL;
        TEST_ASSERT_NULL(strstr(page.events[i].details_json, scoped_secret));
        TEST_ASSERT_NULL(strstr(page.events[i].details_json, manager_key));
    }
    TEST_ASSERT_TRUE(chaining_denied);
    TEST_ASSERT_TRUE(scope_denied);
    TEST_ASSERT_TRUE(validation_failed);
    db_audit_page_free(&page);

    memset(&query, 0, sizeof(query));
    query.page = 1;
    query.page_size = 50;
    safe_strcpy(query.action, "api_token.revoke", sizeof(query.action), 0);
    safe_strcpy(query.outcome, "failure", sizeof(query.outcome), 0);
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    TEST_ASSERT_EQUAL_INT64(1, page.total);
    TEST_ASSERT_EQUAL_STRING(missing_token_uuid, page.events[0].target_uuid);
    TEST_ASSERT_NOT_NULL(strstr(page.events[0].details_json,
                                "\"reason\":\"token_not_found\""));
    TEST_ASSERT_NULL(strstr(page.events[0].details_json, scoped_secret));
    TEST_ASSERT_NULL(strstr(page.events[0].details_json, manager_key));
    db_audit_page_free(&page);
    g_config.web_auth_enabled = false;
}

void test_scoped_location_read_filters_direct_child_count(void) {
    camera_location_t parent = {0};
    camera_location_t visible_child = {0};
    camera_location_t hidden_child = {0};
    safe_strcpy(parent.name, "Scoped Parent", sizeof(parent.name), 0);
    safe_strcpy(parent.type, "site", sizeof(parent.type), 0);
    safe_strcpy(parent.metadata_json, "{}", sizeof(parent.metadata_json), 0);
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_create(&parent));
    safe_strcpy(visible_child.name, "Visible Child",
                sizeof(visible_child.name), 0);
    safe_strcpy(visible_child.type, "area", sizeof(visible_child.type), 0);
    safe_strcpy(visible_child.metadata_json, "{}",
                sizeof(visible_child.metadata_json), 0);
    safe_strcpy(visible_child.parent_uuid, parent.uuid,
                sizeof(visible_child.parent_uuid), 0);
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK,
                          db_location_create(&visible_child));
    safe_strcpy(hidden_child.name, "Hidden Child", sizeof(hidden_child.name), 0);
    safe_strcpy(hidden_child.type, "area", sizeof(hidden_child.type), 0);
    safe_strcpy(hidden_child.metadata_json, "{}",
                sizeof(hidden_child.metadata_json), 0);
    safe_strcpy(hidden_child.parent_uuid, parent.uuid,
                sizeof(hidden_child.parent_uuid), 0);
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_create(&hidden_child));

    stream_config_t visible = create_camera("Location Visible", "Location");
    stream_config_t hidden = create_camera("Location Hidden", "Location");
    sqlite3 *db = get_db_handle();
    sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_prepare_v2(
            db, "UPDATE streams SET location_uuid=? WHERE camera_uuid=?;",
            -1, &stmt, NULL));
    sqlite3_bind_text(stmt, 1, visible_child.uuid, -1, SQLITE_TRANSIENT);
    sqlite3_bind_text(stmt, 2, visible.camera_uuid, -1, SQLITE_TRANSIENT);
    TEST_ASSERT_EQUAL_INT(SQLITE_DONE, sqlite3_step(stmt));
    sqlite3_reset(stmt);
    sqlite3_clear_bindings(stmt);
    sqlite3_bind_text(stmt, 1, hidden_child.uuid, -1, SQLITE_TRANSIENT);
    sqlite3_bind_text(stmt, 2, hidden.camera_uuid, -1, SQLITE_TRANSIENT);
    TEST_ASSERT_EQUAL_INT(SQLITE_DONE, sqlite3_step(stmt));
    sqlite3_finalize(stmt);

    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("locationviewer", "password123", NULL,
                               USER_ROLE_VIEWER, true, &user_id));
    TEST_ASSERT_EQUAL_INT(0,
                          db_authorization_set_user_mode(user_id, "policy"));
    char selector[512];
    snprintf(selector, sizeof(selector),
             "{\"version\":1,\"expression\":{\"op\":\"camera_uuid\","
             "\"values\":[\"%s\"]}}",
             visible.camera_uuid);
    create_grant(user_id, OPERATOR_ROLE_UUID, "selector", selector, NULL);
    char api_key[128] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_generate_api_key(user_id, api_key, sizeof(api_key)));
    char path[128];
    snprintf(path, sizeof(path), "/api/locations/%s", parent.uuid);
    g_config.web_auth_enabled = true;
    cJSON *json = call_handler_path(handle_get_location, HTTP_METHOD_GET, path,
                                    NULL, api_key, 200);
    TEST_ASSERT_EQUAL_INT(
        1, cJSON_GetObjectItemCaseSensitive(json, "child_count")->valueint);
    cJSON_Delete(json);
    g_config.web_auth_enabled = false;
}

static authorization_role_t create_camera_action_user(
    const char *username, const char *camera_uuid, uint64_t action_mask,
    char api_key[128]) {
    int64_t user_id = 0;
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_user(
        username, "password123", NULL, USER_ROLE_VIEWER, true, &user_id));
    TEST_ASSERT_EQUAL_INT(0, db_authorization_set_user_mode(user_id, "policy"));
    authorization_role_t role = {0};
    safe_strcpy(role.name, username, sizeof(role.name), 0);
    role.action_mask = action_mask;
    int64_t version = 0, next_version = 0;
    TEST_ASSERT_EQUAL_INT(0, db_authorization_get_policy_version(&version));
    TEST_ASSERT_EQUAL_INT(DB_AUTHORIZATION_OK,
        db_authorization_role_create(&role, version, &next_version));
    char selector[256], grant_uuid[CAMERA_UUID_STRING_SIZE];
    snprintf(selector, sizeof(selector),
             "{\"version\":1,\"expression\":{\"op\":\"camera_uuid\","
             "\"values\":[\"%s\"]}}", camera_uuid);
    create_grant(user_id, role.uuid, "selector", selector, grant_uuid);
    TEST_ASSERT_EQUAL_INT(0, db_auth_generate_api_key(user_id, api_key, 128));
    return role;
}

void test_lpr_camera_summary_uses_lpr_permissions_and_scope(void) {
    stream_config_t camera = create_camera("LPR_allowed", "");
    create_camera("LPR_hidden", "");
    char api_key[128] = {0};
    authorization_role_t role = create_camera_action_user(
        "lpr_picker_user", camera.camera_uuid,
        authorization_action_bit(AUTHZ_LPR_READ) |
            authorization_action_bit(AUTHZ_LPR_SEARCH), api_key);
    g_config.max_streams = 16;
    g_config.web_auth_enabled = true;
    setenv("LIGHTNVR_LPR_MASTER_KEY_HEX",
           "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f", 1);

    // A valid LPR search must not require a grant to watch live video.
    char body[256];
    snprintf(body, sizeof(body),
             "{\"camera_uuid\":\"%s\",\"start_at\":1787920000000,"
             "\"end_at\":1787930000000}", camera.camera_uuid);
    cJSON *json = call_handler_path(handle_post_lpr_search, HTTP_METHOD_POST,
                                    "/api/lpr/search", body, api_key, 200);
    cJSON_Delete(json);
    json = call_handler_path(handle_get_streams, HTTP_METHOD_GET,
        "/api/streams?summary=true&surface=live", NULL, api_key, 200);
    TEST_ASSERT_EQUAL_INT(0, cJSON_GetArraySize(
        cJSON_GetObjectItemCaseSensitive(json, "streams")));
    cJSON_Delete(json);

    const char *path = "/api/streams?summary=true&surface=lpr&page_size=1"
                       "&include_admin_url=true";
    json = call_handler_path(handle_get_streams, HTTP_METHOD_GET,
                             path, NULL, api_key, 200);
    const cJSON *streams = cJSON_GetObjectItemCaseSensitive(json, "streams");
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(streams));
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetObjectItemCaseSensitive(json, "total")->valueint);
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetObjectItemCaseSensitive(json, "total_pages")->valueint);
    const cJSON *item = cJSON_GetArrayItem(streams, 0);
    TEST_ASSERT_EQUAL_STRING(camera.camera_uuid,
        cJSON_GetObjectItemCaseSensitive(item, "camera_uuid")->valuestring);
    TEST_ASSERT_EQUAL_STRING(camera.name,
        cJSON_GetObjectItemCaseSensitive(item, "name")->valuestring);
    // The LPR picker returns identity only, regardless of requested fields.
    TEST_ASSERT_EQUAL_INT(2, cJSON_GetArraySize(item));
    cJSON_Delete(json);

    // Read and search are both required; a live-only grant is insufficient.
    authorization_action_t insufficient[] = {
        AUTHZ_LPR_READ, AUTHZ_LPR_SEARCH, AUTHZ_LIVE_VIEW
    };
    for (size_t i = 0; i < sizeof(insufficient) / sizeof(insufficient[0]); ++i) {
        int64_t version = 0, next_version = 0;
        role.action_mask = authorization_action_bit(insufficient[i]);
        TEST_ASSERT_EQUAL_INT(0, db_authorization_get_policy_version(&version));
        TEST_ASSERT_EQUAL_INT(DB_AUTHORIZATION_OK,
            db_authorization_role_update(&role, version, &next_version));
        json = call_handler_path(handle_get_streams, HTTP_METHOD_GET,
                                 path, NULL, api_key, 200);
        TEST_ASSERT_EQUAL_INT(0, cJSON_GetArraySize(
            cJSON_GetObjectItemCaseSensitive(json, "streams")));
        TEST_ASSERT_EQUAL_INT(0,
            cJSON_GetObjectItemCaseSensitive(json, "total")->valueint);
        cJSON_Delete(json);
    }
}

void test_timeline_uuid_link_survives_rename_and_enforces_replay_scope(void) {
    stream_config_t camera = create_camera("old_lpr_camera", "");
    recording_metadata_t recording = {0};
    safe_strcpy(recording.stream_name, camera.name, sizeof(recording.stream_name), 0);
    safe_strcpy(recording.camera_uuid, camera.camera_uuid, sizeof(recording.camera_uuid), 0);
    safe_strcpy(recording.file_path, "/tmp/lpr-rename-test.mp4", sizeof(recording.file_path), 0);
    recording.start_time = 1787920000;
    recording.end_time = 1787920600;
    recording.is_complete = true;
    uint64_t recording_id = add_recording_metadata(&recording);
    TEST_ASSERT_NOT_EQUAL(0, recording_id);

    detection_result_t detection = {0};
    detection.count = 1;
    safe_strcpy(detection.detections[0].label, "vehicle",
                sizeof(detection.detections[0].label), 0);
    detection.detections[0].confidence = 1.0f;
    TEST_ASSERT_EQUAL_INT(0, store_external_motion_detections(
        camera.name, &detection, 1787920100, recording_id));
    TEST_ASSERT_EQUAL_INT(1, close_external_motion_detections(camera.camera_uuid, camera.name, 1787920200));
    safe_strcpy(camera.name, "new_lpr_camera", sizeof(camera.name), 0);
    TEST_ASSERT_EQUAL_INT(0, update_stream_config("old_lpr_camera", &camera));

    // Reuse the historical name for a different camera and recording.
    stream_config_t other = create_camera("old_lpr_camera", "");
    safe_strcpy(recording.camera_uuid, other.camera_uuid, sizeof(recording.camera_uuid), 0);
    safe_strcpy(recording.file_path, "/tmp/lpr-other-test.mp4", sizeof(recording.file_path), 0);
    uint64_t other_id = add_recording_metadata(&recording);
    TEST_ASSERT_NOT_EQUAL(0, other_id);
    TEST_ASSERT_EQUAL_INT(0, store_external_motion_detections(
        other.name, &detection, 1787920300, other_id));
    TEST_ASSERT_EQUAL_INT(1, close_external_motion_detections(other.camera_uuid, other.name, 1787920400));

    char api_key[128] = {0};
    create_camera_action_user("timeline_uuid_user", camera.camera_uuid,
        authorization_action_bit(AUTHZ_RECORDINGS_REPLAY), api_key);
    g_config.web_auth_enabled = true;
    char path[512];
    snprintf(path, sizeof(path),
             "/api/timeline/segments?camera_uuid=%s&stream=old_lpr_camera"
             "&start=2026-08-28T00:00:00Z&end=2026-08-29T00:00:00Z",
             camera.camera_uuid);
    cJSON *json = call_handler_path(handle_get_timeline_segments, HTTP_METHOD_GET,
                                    path, NULL, api_key, 200);
    const cJSON *segments = cJSON_GetObjectItemCaseSensitive(json, "segments");
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(segments));
    TEST_ASSERT_EQUAL_UINT64(recording_id, (uint64_t)cJSON_GetObjectItemCaseSensitive(
        cJSON_GetArrayItem(segments, 0), "id")->valuedouble);
    const cJSON *intervals = cJSON_GetObjectItemCaseSensitive(json, "detection_intervals");
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(intervals));
    TEST_ASSERT_EQUAL_INT64(1787920100, (int64_t)cJSON_GetObjectItemCaseSensitive(
        cJSON_GetArrayItem(intervals, 0), "start_timestamp")->valuedouble);
    TEST_ASSERT_EQUAL_STRING(camera.name,
        cJSON_GetObjectItemCaseSensitive(json, "stream")->valuestring);
    cJSON_Delete(json);
    json = call_handler_path(handle_get_timeline_segments, HTTP_METHOD_GET,
        "/api/timeline/segments?stream=new_lpr_camera&start=2026-08-28T00:00:00Z&end=2026-08-29T00:00:00Z",
        NULL, api_key, 200);
    segments = cJSON_GetObjectItemCaseSensitive(json, "segments");
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(segments));
    TEST_ASSERT_EQUAL_UINT64(recording_id, (uint64_t)cJSON_GetObjectItemCaseSensitive(
        cJSON_GetArrayItem(segments, 0), "id")->valuedouble);
    intervals = cJSON_GetObjectItemCaseSensitive(json, "detection_intervals");
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(intervals));
    cJSON_Delete(json);


    snprintf(path, sizeof(path), "/api/timeline/segments-by-ids?ids=%llu",
             (unsigned long long)recording_id);
    json = call_handler_path(handle_get_timeline_segments_by_ids, HTTP_METHOD_GET,
                             path, NULL, api_key, 200);
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(
        cJSON_GetObjectItemCaseSensitive(json, "segments")));
    cJSON_Delete(json);

    snprintf(path, sizeof(path), "/api/timeline/segments-by-ids?ids=%llu",
             (unsigned long long)other_id);
    json = call_handler_path(handle_get_timeline_segments_by_ids, HTTP_METHOD_GET,
                             path, NULL, api_key, 200);
    TEST_ASSERT_EQUAL_INT(0, cJSON_GetArraySize(
        cJSON_GetObjectItemCaseSensitive(json, "segments")));
    cJSON_Delete(json);

    // A permitted display name must not override a denied UUID.
    snprintf(path, sizeof(path),
             "/api/timeline/segments?camera_uuid=%s&stream=new_lpr_camera",
             other.camera_uuid);
    json = call_handler_path(handle_get_timeline_segments, HTTP_METHOD_GET,
                             path, NULL, api_key, 403);
    cJSON_Delete(json);
    snprintf(path, sizeof(path),
             "/api/timeline/segments?camera_uuid=%s-extra&stream=new_lpr_camera",
             camera.camera_uuid);
    json = call_handler_path(handle_get_timeline_segments, HTTP_METHOD_GET,
                             path, NULL, api_key, 400);
    cJSON_Delete(json);
    json = call_handler_path(handle_get_timeline_segments, HTTP_METHOD_GET,
        "/api/timeline/segments?camera_uuid=&stream=new_lpr_camera", NULL, api_key, 400);
    cJSON_Delete(json);
    // Name-only callers keep their existing lookup and authorization semantics.
    json = call_handler_path(handle_get_timeline_segments, HTTP_METHOD_GET,
        "/api/timeline/segments?stream=old_lpr_camera", NULL, api_key, 403);
    cJSON_Delete(json);
}

void test_timeline_ids_batch_only_authorized_recordings_and_keep_open_detection_bounds(void) {
    stream_config_t camera = create_camera("timeline_batch", "");
    stream_config_t denied = create_camera("timeline_denied", "");
    time_t now = time(NULL);
    recording_metadata_t rec = {0};
    safe_strcpy(rec.stream_name, camera.name, sizeof(rec.stream_name), 0);
    safe_strcpy(rec.camera_uuid, camera.camera_uuid, sizeof(rec.camera_uuid), 0);
    safe_strcpy(rec.file_path, "/tmp/timeline-batch-open.mp4", sizeof(rec.file_path), 0);
    rec.start_time = now - 100;
    rec.end_time = 0;
    rec.is_complete = false;
    uint64_t open_id = add_recording_metadata(&rec);
    TEST_ASSERT_NOT_EQUAL(0, open_id);
    detection_result_t detection = {0};
    detection.count = 1;
    safe_strcpy(detection.detections[0].label, "person", sizeof(detection.detections[0].label), 0);
    detection.detections[0].confidence = 1.0f;
    // Unlinked detections require the effective current end time for matching.
    TEST_ASSERT_EQUAL_INT(0, store_detections_in_db_for_camera(
        camera.camera_uuid, camera.name, &detection, now - 10, 0));

    safe_strcpy(rec.file_path, "/tmp/timeline-batch-complete.mp4", sizeof(rec.file_path), 0);
    rec.start_time = now - 300;
    rec.end_time = now - 200;
    rec.is_complete = true;
    uint64_t complete_id = add_recording_metadata(&rec);
    TEST_ASSERT_NOT_EQUAL(0, complete_id);
    safe_strcpy(rec.stream_name, denied.name, sizeof(rec.stream_name), 0);
    safe_strcpy(rec.camera_uuid, denied.camera_uuid, sizeof(rec.camera_uuid), 0);
    safe_strcpy(rec.file_path, "/tmp/timeline-batch-denied.mp4", sizeof(rec.file_path), 0);
    uint64_t denied_id = add_recording_metadata(&rec);
    TEST_ASSERT_NOT_EQUAL(0, denied_id);

    char api_key[128] = {0};
    create_camera_action_user("timeline_batch_user", camera.camera_uuid,
        authorization_action_bit(AUTHZ_RECORDINGS_REPLAY), api_key);
    g_config.web_auth_enabled = true;
    char path[512];
    snprintf(path, sizeof(path), "/api/timeline/segments-by-ids?ids=%llu,%llu,99999999,%llu,%llu",
        (unsigned long long)open_id, (unsigned long long)denied_id,
        (unsigned long long)complete_id, (unsigned long long)open_id);
    timeline_summary_calls = timeline_summary_count = 0;
    cJSON *json = call_handler_path(handle_get_timeline_segments_by_ids, HTTP_METHOD_GET,
                                   path, NULL, api_key, 200);
    TEST_ASSERT_EQUAL_INT(1, timeline_summary_calls);
    TEST_ASSERT_EQUAL_INT(3, timeline_summary_count);
    const cJSON *segments = cJSON_GetObjectItemCaseSensitive(json, "segments");
    TEST_ASSERT_EQUAL_INT(3, cJSON_GetArraySize(segments));
    for (int i = 0; i < 3; i++) {
        const cJSON *segment = cJSON_GetArrayItem(segments, i);
        TEST_ASSERT_EQUAL_UINT64(i == 1 ? complete_id : open_id,
            (uint64_t)cJSON_GetObjectItemCaseSensitive(segment, "id")->valuedouble);
        TEST_ASSERT_EQUAL_INT(i != 1,
            cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(segment, "has_detection")));
        TEST_ASSERT_EQUAL_INT(i != 1,
            cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(segment, "in_progress")));
        if (i != 1) TEST_ASSERT_TRUE(
            cJSON_GetObjectItemCaseSensitive(segment, "end_timestamp")->valuedouble >= (double)now);
    }
    cJSON_Delete(json);

    fail_timeline_summary = true;
    json = call_handler_path(handle_get_timeline_segments_by_ids, HTTP_METHOD_GET,
                             path, NULL, api_key, 500);
    fail_timeline_summary = false;
    TEST_ASSERT_EQUAL_STRING("Failed to get recording detections",
        cJSON_GetObjectItemCaseSensitive(json, "error")->valuestring);
    cJSON_Delete(json);

    snprintf(path, sizeof(path), "/api/timeline/segments-by-ids?ids=%llu,99999999",
        (unsigned long long)denied_id);
    timeline_summary_calls = 0;
    json = call_handler_path(handle_get_timeline_segments_by_ids, HTTP_METHOD_GET,
                             path, NULL, api_key, 200);
    TEST_ASSERT_EQUAL_INT(0, timeline_summary_calls);
    TEST_ASSERT_EQUAL_INT(0, cJSON_GetArraySize(cJSON_GetObjectItemCaseSensitive(json, "segments")));
    cJSON_Delete(json);
}

void test_timeline_ids_keep_safe_numbers_and_exact_large_strings(void) {
    const uint64_t ids[] = {42, UINT64_C(9007199254740991), UINT64_C(9007199254740992),
                            UINT64_C(9007199254740993), UINT64_C(9223372036854775807)};
    stream_config_t camera = create_camera("timeline_exact_ids", "");
    recording_metadata_t rec = {0};
    safe_strcpy(rec.stream_name, camera.name, sizeof(rec.stream_name), 0);
    safe_strcpy(rec.camera_uuid, camera.camera_uuid, sizeof(rec.camera_uuid), 0);
    rec.is_complete = true;
    /* Create the rows before assigning large primary keys so sqlite_sequence
     * stays small for the other tests that rely on auto-assigned numeric IDs. */
    for (int i = 0; i < 5; i++) {
        snprintf(rec.file_path, sizeof(rec.file_path), "/tmp/timeline-exact-%d.mp4", i);
        rec.start_time = 1787920000 + i * 10;
        rec.end_time = rec.start_time + 5;
        TEST_ASSERT_NOT_EQUAL(0, add_recording_metadata(&rec));
    }
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(get_db_handle(),
        "UPDATE recordings SET id=-id", NULL, NULL, NULL));
    for (int i = 0; i < 5; i++) {
        char sql[256];
        snprintf(sql, sizeof(sql), "UPDATE recordings SET id=%llu WHERE file_path='/tmp/timeline-exact-%d.mp4'",
                 (unsigned long long)ids[i], i);
        TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(get_db_handle(), sql, NULL, NULL, NULL));
    }
    char api_key[128] = {0};
    create_camera_action_user("timeline_exact_ids_user", camera.camera_uuid,
        authorization_action_bit(AUTHZ_RECORDINGS_REPLAY), api_key);
    g_config.web_auth_enabled = true;
    char range_path[256];
    snprintf(range_path, sizeof(range_path), "/api/timeline/segments?camera_uuid=%s&start=2026-08-28T00:00:00Z&end=2026-08-29T00:00:00Z", camera.camera_uuid);
    const char *paths[] = {range_path, "/api/timeline/segments-by-ids?ids=42,9007199254740991,9007199254740992,9007199254740993,9223372036854775807"};
    void (*handlers[])(const http_request_t *, http_response_t *) = {
        handle_get_timeline_segments, handle_get_timeline_segments_by_ids};
    for (int endpoint = 0; endpoint < 2; endpoint++) {
        cJSON *json = call_handler_path(handlers[endpoint], HTTP_METHOD_GET, paths[endpoint], NULL, api_key, 200);
        const cJSON *segments = cJSON_GetObjectItemCaseSensitive(json, "segments");
        TEST_ASSERT_EQUAL_INT(5, cJSON_GetArraySize(segments));
        for (int i = 0; i < 5; i++) {
            const cJSON *id = cJSON_GetObjectItemCaseSensitive(cJSON_GetArrayItem(segments, i), "id");
            if (i < 2) {
                TEST_ASSERT_TRUE(cJSON_IsNumber(id));
                TEST_ASSERT_EQUAL_UINT64(ids[i], (uint64_t)id->valuedouble);
            } else {
                char decimal[32]; snprintf(decimal, sizeof(decimal), "%llu", (unsigned long long)ids[i]);
                TEST_ASSERT_TRUE(cJSON_IsString(id));
                TEST_ASSERT_EQUAL_STRING(decimal, id->valuestring);
            }
        }
        cJSON_Delete(json);
    }
    /* Export must resolve the exact large string ID before checking permission.
     * This user has replay only, so a correctly resolved recording yields 403. */
    cJSON_Delete(call_handler_path(handle_batch_download_recordings, HTTP_METHOD_POST,
        "/api/recordings/batch-download", "{\"ids\":[\"9007199254740993\"]}", api_key, 403));
    const char *bad_ids[] = {"9007199254740992", "1.5", "\"18446744073709551616\"", "\"1junk\"", "\"-1\"", "\"0\""};
    for (size_t i = 0; i < sizeof(bad_ids) / sizeof(bad_ids[0]); i++) {
        char body[128]; snprintf(body, sizeof(body), "{\"ids\":[%s]}", bad_ids[i]);
        cJSON_Delete(call_handler_path(handle_batch_download_recordings, HTTP_METHOD_POST,
            "/api/recordings/batch-download", body, api_key, 400));
    }
}

void test_timeline_ids_reject_malformed_tokens_without_partial_results(void) {
    stream_config_t camera = create_camera("timeline_strict_ids", "");
    recording_metadata_t rec = {0};
    safe_strcpy(rec.stream_name, camera.name, sizeof(rec.stream_name), 0);
    safe_strcpy(rec.camera_uuid, camera.camera_uuid, sizeof(rec.camera_uuid), 0);
    safe_strcpy(rec.file_path, "/tmp/timeline-strict-ids.mp4", sizeof(rec.file_path), 0);
    rec.start_time = time(NULL) - 100;
    rec.end_time = rec.start_time + 60;
    rec.is_complete = true;
    uint64_t id = add_recording_metadata(&rec);
    TEST_ASSERT_NOT_EQUAL(0, id);
    char api_key[128] = {0};
    create_camera_action_user("timeline_strict_ids_user", camera.camera_uuid,
        authorization_action_bit(AUTHZ_RECORDINGS_REPLAY), api_key);
    g_config.web_auth_enabled = true;
    const char *invalid[] = {
        "0", "000", "-1", "%2B1", "%20-1", "%09%2B1", "invalid",
        "1junk", "1.5", "0x10", "1%20junk", "%20%09",
        "18446744073709551616", "9999999999999999999999999999999999999999"
    };
    char path[512];
    for (size_t i = 0; i < sizeof(invalid) / sizeof(invalid[0]); i++) {
        /* A valid ID before and after the bad token must not yield a partial 200. */
        snprintf(path, sizeof(path), "/api/timeline/segments-by-ids?ids=%llu,%s,%llu",
            (unsigned long long)id, invalid[i], (unsigned long long)id);
        timeline_summary_calls = 0;
        cJSON *json = call_handler_path(handle_get_timeline_segments_by_ids, HTTP_METHOD_GET,
                                       path, NULL, api_key, 400);
        TEST_ASSERT_EQUAL_INT(0, timeline_summary_calls);
        TEST_ASSERT_EQUAL_STRING("Invalid recording ID in ids parameter",
            cJSON_GetObjectItemCaseSensitive(json, "error")->valuestring);
        TEST_ASSERT_NULL(cJSON_GetObjectItemCaseSensitive(json, "segments"));
        cJSON_Delete(json);
    }
    /* Decimal IDs allow surrounding whitespace and leading zeros. An in-range
     * uint64 that is not present is still omitted, and duplicates keep order. */
    snprintf(path, sizeof(path), "/api/timeline/segments-by-ids?ids=%%09%llu%%20%%0A,18446744073709551615,00%llu",
        (unsigned long long)id, (unsigned long long)id);
    cJSON *json = call_handler_path(handle_get_timeline_segments_by_ids, HTTP_METHOD_GET,
                                   path, NULL, api_key, 200);
    const cJSON *segments = cJSON_GetObjectItemCaseSensitive(json, "segments");
    TEST_ASSERT_EQUAL_INT(2, cJSON_GetArraySize(segments));
    for (int i = 0; i < 2; i++) TEST_ASSERT_EQUAL_UINT64(id,
        (uint64_t)cJSON_GetObjectItemCaseSensitive(cJSON_GetArrayItem(segments, i), "id")->valuedouble);
    cJSON_Delete(json);
}

void test_timeline_ids_preserve_duplicates_at_query_size_limit(void) {
    stream_config_t camera = create_camera("timeline_query_limit", "");
    recording_metadata_t rec = {0};
    safe_strcpy(rec.stream_name, camera.name, sizeof(rec.stream_name), 0);
    safe_strcpy(rec.camera_uuid, camera.camera_uuid, sizeof(rec.camera_uuid), 0);
    safe_strcpy(rec.file_path, "/tmp/timeline-query-limit.mp4", sizeof(rec.file_path), 0);
    rec.start_time = time(NULL) - 100;
    rec.end_time = rec.start_time + 60;
    rec.is_complete = true;
    uint64_t id = add_recording_metadata(&rec);
    TEST_ASSERT_NOT_EQUAL(0, id);
    char api_key[128] = {0};
    create_camera_action_user("timeline_query_limit_user", camera.camera_uuid,
        authorization_action_bit(AUTHZ_RECORDINGS_REPLAY), api_key);
    g_config.web_auth_enabled = true;

    /* Fill the actual transport buffer, not the larger handler-local buffer.
     * Padding with commas exercises the existing empty-token semantics. */
    char query[sizeof(((http_request_t *)0)->query_string)] = "ids=";
    char token[32];
    snprintf(token, sizeof(token), "%llu,", (unsigned long long)id);
    size_t used = strlen(query), token_length = strlen(token);
    int count = 0;
    while (used + token_length < sizeof(query)) {
        memcpy(query + used, token, token_length);
        used += token_length;
        count++;
    }
    memset(query + used, ',', sizeof(query) - 1 - used);
    query[sizeof(query) - 1] = '\0';
    char path[sizeof(query) + 64];
    snprintf(path, sizeof(path), "/api/timeline/segments-by-ids?%s", query);
    timeline_summary_calls = timeline_summary_count = 0;
    cJSON *json = call_handler_path(handle_get_timeline_segments_by_ids, HTTP_METHOD_GET,
                                   path, NULL, api_key, 200);
    const cJSON *segments = cJSON_GetObjectItemCaseSensitive(json, "segments");
    TEST_ASSERT_EQUAL_INT(count, cJSON_GetArraySize(segments));
    TEST_ASSERT_EQUAL_INT(count, cJSON_GetObjectItemCaseSensitive(json, "segment_count")->valueint);
    TEST_ASSERT_EQUAL_INT(1, timeline_summary_calls);
    TEST_ASSERT_EQUAL_INT(count, timeline_summary_count);
    for (int i = 0; i < count; i++) {
        TEST_ASSERT_EQUAL_UINT64(id, (uint64_t)cJSON_GetObjectItemCaseSensitive(
            cJSON_GetArrayItem(segments, i), "id")->valuedouble);
    }
    cJSON_Delete(json);
}

void test_timeline_manifest_rejects_invalid_bounds_before_querying_recordings(void) {
    stream_config_t camera = create_camera("manifest_bounds", "");
    char api_key[128] = {0};
    create_camera_action_user("manifest_bounds_user", camera.camera_uuid,
        authorization_action_bit(AUTHZ_RECORDINGS_REPLAY), api_key);
    g_config.web_auth_enabled = true;
    const char *invalid[] = {"not-a-date", "2026-02-30", "2026-08-28T00:00:00Zjunk"};
    for (size_t i = 0; i < sizeof(invalid) / sizeof(invalid[0]); i++) {
        for (int end = 0; end < 2; end++) {
            char path[512];
            snprintf(path, sizeof(path), "/api/timeline/manifest?stream=%s&%s=%s",
                camera.name, end ? "end" : "start", invalid[i]);
            cJSON *json = call_handler_path(handle_timeline_manifest, HTTP_METHOD_GET,
                                            path, NULL, api_key, 400);
            TEST_ASSERT_EQUAL_STRING(end ? "Invalid end time format" : "Invalid start time format",
                cJSON_GetObjectItemCaseSensitive(json, "error")->valuestring);
            cJSON_Delete(json);
        }
    }
    cJSON *json = call_handler_path(handle_timeline_manifest, HTTP_METHOD_GET,
        "/api/timeline/manifest?stream=manifest_bounds&start=2026-08-28T00:00:00.123Z&end=2026-08-28T23:59:59.999Z",
        NULL, api_key, 404);
    TEST_ASSERT_EQUAL_STRING("No recordings found for the specified time range",
        cJSON_GetObjectItemCaseSensitive(json, "error")->valuestring);
    cJSON_Delete(json);
}

void test_timeline_segments_reject_unparseable_time_bounds(void) {
    stream_config_t camera = create_camera("timeline_bounds", "");
    char api_key[128] = {0};
    create_camera_action_user("timeline_bounds_user", camera.camera_uuid,
        authorization_action_bit(AUTHZ_RECORDINGS_REPLAY), api_key);
    g_config.web_auth_enabled = true;

    const char *queries[] = {
        "&start=not-a-date",
        "&start=2026-99-99&end=2026-08-29T00:00:00Z",
        "&end=not-a-date",
        "&start=2026-08-28T00:00:00Z&end=2026-99-99"
    };
    for (size_t i = 0; i < sizeof(queries) / sizeof(queries[0]); i++) {
        char path[512];
        snprintf(path, sizeof(path), "/api/timeline/segments?camera_uuid=%s%s",
                 camera.camera_uuid, queries[i]);
        cJSON *json = call_handler_path(handle_get_timeline_segments, HTTP_METHOD_GET,
                                        path, NULL, api_key, 400);
        const cJSON *error = cJSON_GetObjectItemCaseSensitive(json, "error");
        TEST_ASSERT_TRUE(cJSON_IsString(error));
        TEST_ASSERT_EQUAL_STRING(i < 2 ? "Invalid start parameter" : "Invalid end parameter",
                                 error->valuestring);
        TEST_ASSERT_NULL(cJSON_GetObjectItemCaseSensitive(json, "segments"));
        cJSON_Delete(json);
    }

    const char *invalid_bounds[] = {
        "2026-02-30", "2025-02-29", "2026-04-31T12:00:00Z",
        "2026-08-28junk", "2026-08-28T00:00:00Zjunk",
        "2026-08-28T00:00:00.123Zjunk", "2026-08-28T00:00:00.Z",
        "2026-08-28T00:00:00.", "2026-08-28T00:00:60Z",
        "2026-08-28T00:00:00.00000000000000000000000000000000000000000000000000000Z"
    };
    for (size_t i = 0; i < sizeof(invalid_bounds) / sizeof(invalid_bounds[0]); i++) {
        for (int is_end = 0; is_end < 2; is_end++) {
            char path[512];
            snprintf(path, sizeof(path), "/api/timeline/segments?camera_uuid=%s&%s=%s",
                     camera.camera_uuid, is_end ? "end" : "start", invalid_bounds[i]);
            cJSON *json = call_handler_path(handle_get_timeline_segments, HTTP_METHOD_GET,
                                            path, NULL, api_key, 400);
            const cJSON *error = cJSON_GetObjectItemCaseSensitive(json, "error");
            TEST_ASSERT_TRUE(cJSON_IsString(error));
            TEST_ASSERT_EQUAL_STRING(is_end ? "Invalid end parameter" : "Invalid start parameter",
                                     error->valuestring);
            cJSON_Delete(json);
        }
    }

    /* Optional bounds keep their defaults; date-only and browser ISO strings
     * remain accepted through the same authenticated endpoint. */
    const char *valid_queries[] = {
        "", "&start=2026-08-28&end=2026-08-29",
        "&start=2026-08-28T00:00:00.000Z&end=2026-08-29T00:00:00.000Z",
        "&start=2026-08-28T00:00:00.123Z&end=2026-08-28T23:59:59.999Z",
        "&start=2026-08-28T00:00:00.1&end=2026-08-28T23:59:59.123456Z",
        "&start=2026-08-28T00:00:00&end=2026-08-28T23:59:59Z",
        "&start=2026-08-28T00%3A00%3A00.123Z&end=2026-08-28T23%3A59%3A59.999Z",
        "&start=2024-02-29&end=2024-03-01"
    };
    for (size_t i = 0; i < sizeof(valid_queries) / sizeof(valid_queries[0]); i++) {
        char path[512];
        snprintf(path, sizeof(path), "/api/timeline/segments?stream=%s%s",
                 camera.name, valid_queries[i]);
        cJSON *json = call_handler_path(handle_get_timeline_segments, HTTP_METHOD_GET,
                                        path, NULL, api_key, 200);
        TEST_ASSERT_TRUE(cJSON_IsArray(cJSON_GetObjectItemCaseSensitive(json, "segments")));
        cJSON_Delete(json);
    }
}

int main(void) {
    if (initialize_test_db_path() != 0) {
        fprintf(stderr, "FATAL: temporary database directory setup failed\n");
        return 1;
    }
    if (init_database(test_db_path) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    if (db_auth_init() != 0) {
        fprintf(stderr, "FATAL: db_auth_init failed\n");
        shutdown_database();
        unlink(test_db_path);
        return 1;
    }
    if (batch_delete_progress_init() != 0) {
        fprintf(stderr, "FATAL: batch delete progress init failed\n");
        shutdown_database();
        unlink(test_db_path);
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_action_catalog_is_stable_and_complete);
    RUN_TEST(test_lpr_camera_summary_uses_lpr_permissions_and_scope);
    RUN_TEST(test_timeline_uuid_link_survives_rename_and_enforces_replay_scope);
    RUN_TEST(test_timeline_segments_reject_unparseable_time_bounds);
    RUN_TEST(test_timeline_ids_batch_only_authorized_recordings_and_keep_open_detection_bounds);
    RUN_TEST(test_timeline_ids_preserve_duplicates_at_query_size_limit);
    RUN_TEST(test_timeline_ids_reject_malformed_tokens_without_partial_results);
    RUN_TEST(test_timeline_ids_keep_safe_numbers_and_exact_large_strings);
    RUN_TEST(test_timeline_manifest_rejects_invalid_bounds_before_querying_recordings);
    RUN_TEST(test_legacy_role_compatibility_has_no_runtime_allowed_tags_path);
    RUN_TEST(test_legacy_principals_migrate_to_idempotent_selector_grants);
    RUN_TEST(test_policy_mode_defaults_deny_and_matches_selector_grant);
    RUN_TEST(test_all_scope_admin_grant_allows_global_action_and_bumps_version);
    RUN_TEST(test_invalid_stored_selector_fails_closed);
    RUN_TEST(test_shared_collection_grants_track_membership_and_guard_scope);
    RUN_TEST(test_recording_list_intersects_explicit_stream_collection_and_policy_scope);
    RUN_TEST(test_scoped_api_token_intersects_user_policy_and_revokes);
    RUN_TEST(test_action_catalog_and_simulation_handlers);
    RUN_TEST(test_role_and_policy_database_mutations_are_atomic);
    RUN_TEST(test_policy_management_handlers_and_conflict_guards);
    RUN_TEST(test_policy_role_update_cannot_lock_out_requester);
    RUN_TEST(test_users_api_reports_authorization_mode);
    RUN_TEST(test_sensitive_handlers_enforce_camera_scoped_policy);
    RUN_TEST(test_visible_camera_filter_hides_cameras_outside_grant);
    RUN_TEST(test_investigation_search_facets_are_camera_scope_authorized);
    RUN_TEST(test_investigation_recording_preview_explains_action_permissions);
    RUN_TEST(test_effective_action_mask_reflects_grants_and_roles);
    RUN_TEST(test_effective_action_mask_intersects_token_and_current_owner_permissions);
    RUN_TEST(test_policy_manager_cannot_grant_actions_it_lacks);
    RUN_TEST(test_action_catalog_bit_layout_matches_database);
    RUN_TEST(test_streams_reject_empty_camera_uuid);
    RUN_TEST(test_player_telemetry_requires_live_view_authorization);
    RUN_TEST(test_metrics_require_system_admin_authorization);
    RUN_TEST(test_stream_health_filters_camera_metrics_by_live_view_scope);
    RUN_TEST(test_user_create_rejects_retired_allowed_tags_field);
    RUN_TEST(test_user_create_rejects_noninteger_and_out_of_range_roles);
    RUN_TEST(test_user_create_accepts_integer_and_default_roles);
    RUN_TEST(test_user_update_rolls_back_every_field_and_sessions_on_failure);
    RUN_TEST(test_user_update_rejects_roles_before_changing_credentials);
    RUN_TEST(test_user_update_preserves_omitted_role);
    RUN_TEST(test_user_update_rejects_nonstring_password_without_changes);
    RUN_TEST(test_user_creation_requires_actions_and_all_camera_scope);
    RUN_TEST(test_password_change_supports_large_user_ids);
    RUN_TEST(test_password_change_rejects_malformed_and_overflowing_ids);
    RUN_TEST(test_user_routes_reject_malformed_ids_without_mutation);
    RUN_TEST(test_user_routes_accept_full_width_ids_and_exact_suffixes);
    RUN_TEST(test_users_manage_grant_controls_cross_user_password_and_totp);
    RUN_TEST(test_scoped_token_cannot_use_identity_self_service_or_other_progress);
    RUN_TEST(test_recording_file_check_requires_scoped_replay_access);
    RUN_TEST(test_collection_resolution_uses_requested_replay_action);
    RUN_TEST(test_delegated_policy_scope_cannot_widen_or_chain_scoped_tokens);
    RUN_TEST(test_scoped_location_read_filters_direct_child_count);
    int result = UNITY_END();
    batch_delete_progress_cleanup();
    shutdown_database();
    unlink(test_db_path);
    return result;
}
