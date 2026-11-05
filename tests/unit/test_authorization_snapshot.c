/* SPDX-License-Identifier: GPL-3.0-or-later */
#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#include <ftw.h>
#include <sqlite3.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include "unity.h"
#include "database/db_authorization_snapshot.h"
#include "database/db_authorization.h"
#include "database/db_api_tokens.h"
#include "database/db_camera_collections.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#include "database/db_fleet_query.h"
#include "web/fleet_health.h"
#include "video/go2rtc/go2rtc_api.h"
#include "video/go2rtc/go2rtc_integration.h"
#include "telemetry/stream_metrics.h"
#include "utils/strings.h"

#define VIEWER "00000000-0000-4000-8000-000000000003"
#define ADMIN "00000000-0000-4000-8000-000000000001"
static char directory[] = "/tmp/lightnvr-snapshot-XXXXXX";
static char db_path[256];
static user_t principal;
static stream_config_t camera_a, camera_b;
static authorization_snapshot_t *owned[64];
static int owned_count;
static int allocation_failure = -1;
static const char *sql_failure;
static time_t observed_time;
static int injected_health = -1;
static bool detailed_metrics, injected_live;
static int metrics_captures, live_captures, compared_inputs;
static fleet_camera_t *reference_views[3];
static authorization_context_t *reference_context;
static const fleet_camera_t *expected_camera;

int __real_authorization_evaluate_in_context(authorization_context_t *, const user_t *,
    authorization_action_t, const fleet_camera_t *, authorization_evaluation_t *);
bool __real_go2rtc_integration_is_initialized(void);
bool __real_go2rtc_api_get_stream_activity(go2rtc_stream_activity_t *, size_t);

int __wrap_authorization_evaluate_in_context(authorization_context_t *context,
    const user_t *user, authorization_action_t action, const fleet_camera_t *camera,
    authorization_evaluation_t *evaluation) {
    if (expected_camera) {
        TEST_ASSERT_NOT_NULL(camera);
        /* Both inventories are calloc-initialized by the real DB loader. Compare
         * every immutable field (including tags/locations/capabilities), then
         * every observation field individually, without comparing tail padding. */
        TEST_ASSERT_EQUAL_MEMORY(expected_camera, camera, offsetof(fleet_camera_t, health));
        TEST_ASSERT_EQUAL_INT(expected_camera->health, camera->health);
        TEST_ASSERT_EQUAL_INT(expected_camera->availability, camera->availability);
        TEST_ASSERT_EQUAL_INT64(expected_camera->health_changed_at, camera->health_changed_at);
        TEST_ASSERT_EQUAL_INT64(expected_camera->first_video_at, camera->first_video_at);
        TEST_ASSERT_EQUAL_INT64(expected_camera->last_frame_ts, camera->last_frame_ts);
        TEST_ASSERT_EQUAL_INT64(expected_camera->last_recording_at, camera->last_recording_at);
        TEST_ASSERT_EQUAL_MEMORY(&expected_camera->current_fps, &camera->current_fps, sizeof(double));
        TEST_ASSERT_EQUAL_INT(expected_camera->recording_active, camera->recording_active);
        compared_inputs++;
    }
    return __real_authorization_evaluate_in_context(context, user, action, camera, evaluation);
}
bool __wrap_go2rtc_integration_is_initialized(void) {
    return injected_live || __real_go2rtc_integration_is_initialized();
}
bool __wrap_go2rtc_api_get_stream_activity(go2rtc_stream_activity_t *streams, size_t count) {
    if (!injected_live) return __real_go2rtc_api_get_stream_activity(streams, count);
    live_captures++;
    for (size_t i = 0; i < count; i++) streams[i].video_active = strcmp(streams[i].stream_name, "B") == 0;
    return true;
}

void *__real_malloc(size_t);
void *__real_calloc(size_t, size_t);
void *__real_realloc(void *, size_t);
int __real_sqlite3_prepare_v2(sqlite3 *, const char *, int, sqlite3_stmt **, const char **);
time_t __real_time(time_t *);
int __real_metrics_get_max_streams(void);
int __real_metrics_snapshot_all(stream_metrics_t *, int);
static int fail_allocation(void) {
    if (allocation_failure < 0) return 0;
    if (allocation_failure-- == 0) { allocation_failure = -1; return 1; }
    return 0;
}
void *__wrap_malloc(size_t n) { return fail_allocation() ? NULL : __real_malloc(n); }
void *__wrap_calloc(size_t n, size_t size) { return fail_allocation() ? NULL : __real_calloc(n, size); }
void *__wrap_realloc(void *p, size_t n) { return fail_allocation() ? NULL : __real_realloc(p, n); }
int __wrap_sqlite3_prepare_v2(sqlite3 *db, const char *sql, int n, sqlite3_stmt **out, const char **tail) {
    if (sql_failure && strstr(sql, sql_failure)) { *out = NULL; return SQLITE_NOMEM; }
    return __real_sqlite3_prepare_v2(db, sql, n, out, tail);
}
time_t __wrap_time(time_t *out) {
    if (!observed_time) return __real_time(out);
    if (out) *out = observed_time;
    return observed_time;
}
int __wrap_metrics_get_max_streams(void) {
    return injected_health >= 0 ? 1 : __real_metrics_get_max_streams();
}
int __wrap_metrics_snapshot_all(stream_metrics_t *out, int max) {
    if (injected_health < 0) return __real_metrics_snapshot_all(out, max);
    metrics_captures++;
    if (max < 1) return 0;
    memset(out, 0, sizeof(*out));
    safe_strcpy(out->stream_name, "A", sizeof(out->stream_name), 0);
    out->health_status = injected_health;
    out->last_frame_ts = injected_health == STREAM_HEALTH_UP ? time(NULL) : time(NULL) - 120;
    if (detailed_metrics) {
        out->health_changed_at = time(NULL) - 17;
        out->current_fps = 12.5;
        out->last_completed_segment_ts = time(NULL) - 8;
        out->expected_segment_duration = 30;
    }
    return 1;
}

static void sql(const char *text) {
    char *error = NULL;
    int rc = sqlite3_exec(get_db_handle(), text, NULL, NULL, &error);
    if (rc != SQLITE_OK) fprintf(stderr, "SQL error: %s\n", error ? error : "unknown");
    sqlite3_free(error);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, rc);
}
static stream_config_t camera(const char *name, const char *uuid) {
    stream_config_t c = {0};
    safe_strcpy(c.name, name, sizeof(c.name), 0);
    safe_strcpy(c.url, "rtsp://fixture.invalid/live", sizeof(c.url), 0);
    safe_strcpy(c.codec, "h264", sizeof(c.codec), 0);
    safe_strcpy(c.camera_uuid, uuid, sizeof(c.camera_uuid), 0);
    c.enabled = true; c.streaming_enabled = true;
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&c));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name(name, &c));
    return c;
}
static void policy(const char *role, const char *scope, const char *selector, const char *collection) {
    sql("DELETE FROM authz_grants WHERE user_id=(SELECT id FROM users WHERE username='snapshot');");
    TEST_ASSERT_EQUAL_INT(0, db_authorization_set_user_mode(principal.id, "policy"));
    TEST_ASSERT_EQUAL_INT(0, db_authorization_create_user_grant(principal.id, role, scope, selector, collection, NULL));
    TEST_ASSERT_EQUAL_INT(0, db_auth_get_user_by_id(principal.id, &principal));
}
static authorization_snapshot_t *capture(void) {
    authorization_snapshot_t *snapshot = NULL;
    TEST_ASSERT_EQUAL_INT(0, authorization_snapshot_create(&principal, &snapshot));
    TEST_ASSERT_NOT_NULL(snapshot);
    TEST_ASSERT_LESS_THAN_INT(64, owned_count);
    owned[owned_count++] = snapshot;
    return snapshot;
}
/* Loader orders by UUID, independently of stream name. */
static int index_a(void) { return strcmp(camera_a.camera_uuid, camera_b.camera_uuid) < 0 ? 0 : 1; }
static int allows(authorization_snapshot_t *snapshot, authorization_action_t action, int index, const char *resource) {
    authorization_evaluation_t evaluation;
    TEST_ASSERT_EQUAL_INT(0, authorization_snapshot_evaluate(snapshot, action, index, resource, &evaluation));
    return evaluation.decision == AUTHZ_DECISION_ALLOW;
}
void setUp(void) {
    allocation_failure = -1; sql_failure = NULL; observed_time = 0; injected_health = -1;
    owned_count = 0;
    detailed_metrics = injected_live = false;
    metrics_captures = live_captures = compared_inputs = 0;
    expected_camera = NULL;
    reference_context = NULL;
    memset(reference_views, 0, sizeof(reference_views));
    sql("DELETE FROM authz_grants WHERE user_id IN (SELECT id FROM users WHERE username != 'admin'); DELETE FROM camera_collections; DELETE FROM streams; DELETE FROM users WHERE username != 'admin';");
    int64_t id;
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_user("snapshot", "test-password-123", NULL, USER_ROLE_VIEWER, true, &id));
    TEST_ASSERT_EQUAL_INT(0, db_auth_get_user_by_id(id, &principal));
    safe_strcpy(principal.authentication_method, "basic", sizeof(principal.authentication_method), 0);
    camera_a = camera("A", "10000000-0000-4000-8000-000000000001");
    camera_b = camera("B", "20000000-0000-4000-8000-000000000002");
}
void tearDown(void) {
    allocation_failure = -1; sql_failure = NULL; observed_time = 0; injected_health = -1;
    expected_camera = NULL;
    detailed_metrics = injected_live = false;
    authorization_context_free(reference_context);
    reference_context = NULL;
    for (int i = 0; i < 3; i++) { free(reference_views[i]); reference_views[i] = NULL; }
    for (int i = 0; i < owned_count; i++) authorization_snapshot_free(owned[i]);
    owned_count = 0;
}

static void test_refresh_identity_and_keep_snapshot_after_deactivation(void) {
    principal.role = USER_ROLE_ADMIN; /* Stale caller must not elevate DB viewer. */
    authorization_snapshot_t *old = capture();
    TEST_ASSERT_TRUE(allows(old, AUTHZ_LIVE_VIEW, index_a(), "raw"));
    TEST_ASSERT_FALSE(allows(old, AUTHZ_CAMERA_CONFIGURE, index_a(), "raw"));
    sql("UPDATE users SET is_active=0 WHERE username='snapshot';");
    TEST_ASSERT_TRUE(allows(old, AUTHZ_LIVE_VIEW, index_a(), "raw"));
    authorization_snapshot_t *next = (void *)1;
    TEST_ASSERT_EQUAL_INT(-2, authorization_snapshot_create(&principal, &next));
    TEST_ASSERT_NULL(next);
}

static void test_lazy_collection_membership_reads_use_original_transaction(void) {
    camera_collection_t collection = {0};
    safe_strcpy(collection.name, "Snapshot collection", sizeof(collection.name), 0);
    safe_strcpy(collection.collection_type, "static", sizeof(collection.collection_type), 0);
    collection.is_shared = true;
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_OK, db_camera_collection_create(&collection));
    const char *members[] = {camera_a.camera_uuid};
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_OK, db_camera_collection_set_members(collection.uuid, members, 1));
    policy(VIEWER, "collection", NULL, collection.uuid);
    authorization_snapshot_t *old = capture();
    members[0] = camera_b.camera_uuid;
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_COLLECTION_OK, db_camera_collection_set_members(collection.uuid, members, 1));
    /* Neither old grants nor collection were evaluated before the writer committed. */
    TEST_ASSERT_TRUE(allows(old, AUTHZ_LIVE_VIEW, index_a(), "raw"));
    TEST_ASSERT_FALSE(allows(old, AUTHZ_LIVE_VIEW, 1-index_a(), "raw"));
    authorization_snapshot_t *next = capture();
    TEST_ASSERT_FALSE(allows(next, AUTHZ_LIVE_VIEW, index_a(), "raw"));
    TEST_ASSERT_TRUE(allows(next, AUTHZ_LIVE_VIEW, 1-index_a(), "raw"));
}

static void test_policy_and_inventory_cannot_mix_across_commit(void) {
    char selector[512];
    snprintf(selector, sizeof(selector), "{\"version\":1,\"expression\":{\"op\":\"camera_uuid\",\"values\":[\"%s\"]}}", camera_a.camera_uuid);
    policy(VIEWER, "selector", selector, NULL);
    authorization_snapshot_t *old = capture();
    sql("BEGIN IMMEDIATE; DELETE FROM authz_grants WHERE user_id=(SELECT id FROM users WHERE username='snapshot'); DELETE FROM streams WHERE name='A'; UPDATE authz_policy_state SET version=version+1; COMMIT;");
    TEST_ASSERT_EQUAL_INT(2, authorization_snapshot_camera_count(old));
    TEST_ASSERT_TRUE(allows(old, AUTHZ_LIVE_VIEW, index_a(), "raw"));
    authorization_snapshot_t *next = capture();
    TEST_ASSERT_EQUAL_INT(1, authorization_snapshot_camera_count(next));
    TEST_ASSERT_FALSE(allows(next, AUTHZ_LIVE_VIEW, 0, "raw"));
}

static void test_scoped_token_expiry_and_revocation_are_snapshot_bound(void) {
    policy(ADMIN, "all", NULL, NULL);
    api_token_create_t input = {.user_id=principal.id, .created_by_user_id=principal.id,
        .description="snapshot fixture", .action_mask=authorization_action_bit(AUTHZ_LIVE_VIEW),
        .scope_type="all", .expires_at=(int64_t)time(NULL)+3600};
    api_token_t token; char secret[API_TOKEN_SECRET_MAX];
    TEST_ASSERT_EQUAL_INT(DB_API_TOKEN_OK, db_api_token_create(&input, &token, secret));
    TEST_ASSERT_TRUE(authorization_snapshot_allows_non_scoped_identity(capture()));
    TEST_ASSERT_FALSE(authorization_snapshot_allows_non_scoped_identity(NULL));
    principal.authenticated_via_scoped_token = true;
    safe_strcpy(principal.api_token_uuid, token.uuid, sizeof(principal.api_token_uuid), 0);
    authorization_snapshot_t *old = capture();
    TEST_ASSERT_FALSE(authorization_snapshot_allows_non_scoped_identity(old));
    observed_time = (time_t)input.expires_at + 1;
    TEST_ASSERT_TRUE(allows(old, AUTHZ_LIVE_VIEW, index_a(), "raw"));
    TEST_ASSERT_FALSE(allows(old, AUTHZ_CAMERA_CONFIGURE, index_a(), "raw"));
    authorization_snapshot_t *next = NULL;
    TEST_ASSERT_EQUAL_INT(-2, authorization_snapshot_create(&principal, &next));
    TEST_ASSERT_NULL(next);
    observed_time = 0;
    TEST_ASSERT_EQUAL_INT(DB_API_TOKEN_OK, db_api_token_revoke(principal.id, token.uuid));
    TEST_ASSERT_TRUE(allows(old, AUTHZ_LIVE_VIEW, index_a(), "raw"));
    TEST_ASSERT_EQUAL_INT(-2, authorization_snapshot_create(&principal, &next));
    TEST_ASSERT_NULL(next);
}

static void test_runtime_observations_are_captured_once_and_raw_stays_raw(void) {
    policy(VIEWER, "selector", "{\"version\":1,\"expression\":{\"op\":\"health\",\"values\":[\"up\"]}}", NULL);
    injected_health = STREAM_HEALTH_UP;
    authorization_snapshot_t *old = capture();
    TEST_ASSERT_FALSE(allows(old, AUTHZ_LIVE_VIEW, index_a(), "raw"));
    TEST_ASSERT_TRUE(allows(old, AUTHZ_LIVE_VIEW, index_a(), "runtime"));
    TEST_ASSERT_TRUE(allows(old, AUTHZ_LIVE_VIEW, index_a(), "live")); /* go2rtc uninitialized */
    injected_health = STREAM_HEALTH_DOWN;
    authorization_snapshot_t *next = capture();
    TEST_ASSERT_TRUE(allows(old, AUTHZ_LIVE_VIEW, index_a(), "runtime"));
    TEST_ASSERT_FALSE(allows(next, AUTHZ_LIVE_VIEW, index_a(), "runtime"));
    injected_health = -1; /* No metrics subsystem: preserve UNKNOWN. */
    next = capture();
    TEST_ASSERT_FALSE(allows(next, AUTHZ_LIVE_VIEW, index_a(), "runtime"));
}

/* Independent reference retains the original three-full-copy algorithm. Real
 * enrichment and authorization functions exercise all observation inputs, not
 * merely a health selector's current subset of fields. Alternating resources
 * and cameras also detects any accidental pointer-address memoization. */
static void test_compact_observations_match_three_inventory_reference(void) {
    observed_time = 1700000000;
    injected_health = STREAM_HEALTH_UP;
    detailed_metrics = injected_live = true;
    policy(VIEWER, "selector", "{\"version\":1,\"expression\":{\"op\":\"health\",\"values\":[\"up\"]}}", NULL);
    sql("INSERT OR REPLACE INTO camera_observations(camera_uuid,first_video_at,last_video_at,last_recording_at) "
        "SELECT camera_uuid,1699999000,1700000002,1699999500 FROM streams WHERE name='A';");
    int count = 0;
    TEST_ASSERT_EQUAL_INT(0, db_fleet_camera_load(&reference_views[0], &count));
    TEST_ASSERT_EQUAL_INT(2, count);
    for (int view = 1; view < 3; view++) {
        reference_views[view] = malloc((size_t)count * sizeof(fleet_camera_t));
        TEST_ASSERT_NOT_NULL(reference_views[view]);
        memcpy(reference_views[view], reference_views[view-1], (size_t)count * sizeof(fleet_camera_t));
        TEST_ASSERT_EQUAL_INT(0, view == 1
            ? fleet_camera_capture_runtime_health(reference_views[view], count)
            : fleet_health_capture_go2rtc_activity(reference_views[view], count));
    }
    reference_context = authorization_context_create();
    TEST_ASSERT_NOT_NULL(reference_context);
    metrics_captures = live_captures = 0;
    authorization_snapshot_t *snapshot = capture();
    TEST_ASSERT_EQUAL_INT(1, metrics_captures);
    TEST_ASSERT_EQUAL_INT(1, live_captures);
    TEST_ASSERT_EQUAL_INT(FLEET_HEALTH_UNKNOWN, reference_views[0][index_a()].health);
    TEST_ASSERT_EQUAL_INT(FLEET_HEALTH_UP, reference_views[1][index_a()].health);
    TEST_ASSERT_EQUAL_INT(FLEET_HEALTH_UNKNOWN, reference_views[1][1-index_a()].health);
    TEST_ASSERT_EQUAL_INT(FLEET_HEALTH_UP, reference_views[2][1-index_a()].health);
    /* Capture must stay frozen after both sources and the clock change. */
    injected_health = STREAM_HEALTH_DOWN;
    injected_live = false;
    observed_time += 300;
    const int order[] = {2, 0, 1, 0, 2, 1};
    const char *resources[] = {"raw", "runtime", "live"};
    const authorization_action_t actions[] = {AUTHZ_LIVE_VIEW, AUTHZ_CAMERA_CONFIGURE};
    for (size_t pass = 0; pass < sizeof(order)/sizeof(order[0]); pass++) {
        int view = order[pass];
        for (int i = 0; i < count; i++) {
            for (size_t a = 0; a < sizeof(actions)/sizeof(actions[0]); a++) {
                authorization_evaluation_t expected, actual;
                TEST_ASSERT_EQUAL_INT(0, __real_authorization_evaluate_in_context(reference_context,
                    &principal, actions[a], &reference_views[view][i], &expected));
                expected_camera = &reference_views[view][i];
                TEST_ASSERT_EQUAL_INT(0, authorization_snapshot_evaluate(snapshot, actions[a], i, resources[view], &actual));
                expected_camera = NULL;
                TEST_ASSERT_EQUAL_INT(expected.decision, actual.decision);
                TEST_ASSERT_EQUAL_INT(expected.source, actual.source);
                TEST_ASSERT_EQUAL_INT64(expected.policy_version, actual.policy_version);
                TEST_ASSERT_EQUAL_STRING(expected.grant_uuid, actual.grant_uuid);
                TEST_ASSERT_EQUAL_STRING(expected.token_uuid, actual.token_uuid);
                TEST_ASSERT_EQUAL_STRING(expected.role_uuid, actual.role_uuid);
                TEST_ASSERT_EQUAL_STRING(expected.role_name, actual.role_name);
                TEST_ASSERT_EQUAL_STRING(expected.explanation, actual.explanation);
            }
        }
    }
    TEST_ASSERT_EQUAL_INT(24, compared_inputs);
    TEST_ASSERT_EQUAL_INT(1, metrics_captures);
    TEST_ASSERT_EQUAL_INT(1, live_captures);
}

static void test_database_errors_never_return_partial_snapshot_or_allow(void) {
    policy(VIEWER, "all", NULL, NULL);
    sql_failure = "SELECT role,is_active";
    authorization_snapshot_t *snapshot = (void *)1;
    TEST_ASSERT_EQUAL_INT(-1, authorization_snapshot_create(&principal, &snapshot));
    TEST_ASSERT_NULL(snapshot);
    sql_failure = NULL;
    snapshot = capture();
    sql_failure = "FROM authz_grants";
    authorization_evaluation_t evaluation;
    TEST_ASSERT_EQUAL_INT(-1, authorization_snapshot_evaluate(snapshot, AUTHZ_LIVE_VIEW, index_a(), "raw", &evaluation));
    TEST_ASSERT_EQUAL_INT(AUTHZ_DECISION_DENY, evaluation.decision);
    sql_failure = NULL;
    TEST_ASSERT_TRUE(allows(capture(), AUTHZ_LIVE_VIEW, index_a(), "raw"));
}

static void test_allocation_failures_cleanup_and_retry(void) {
    int failures = 0;
    for (int nth = 0; nth < 32; nth++) {
        authorization_snapshot_t *snapshot = NULL;
        allocation_failure = nth;
        int rc = authorization_snapshot_create(&principal, &snapshot);
        allocation_failure = -1;
        if (rc == 0) { authorization_snapshot_free(snapshot); break; }
        failures++;
        TEST_ASSERT_EQUAL_INT(-1, rc);
        TEST_ASSERT_NULL(snapshot);
    }
    TEST_ASSERT_GREATER_THAN_INT(3, failures); /* snapshot, context, inventory, observations */
    TEST_ASSERT_TRUE(allows(capture(), AUTHZ_LIVE_VIEW, index_a(), "raw"));
}

static void test_empty_inventory_and_invalid_inputs(void) {
    sql("DELETE FROM streams;");
    authorization_snapshot_t *snapshot = capture();
    TEST_ASSERT_EQUAL_INT(0, authorization_snapshot_camera_count(snapshot));
    TEST_ASSERT_FALSE(allows(snapshot, AUTHZ_SYSTEM_ADMIN, -1, "raw"));
    authorization_evaluation_t evaluation;
    TEST_ASSERT_EQUAL_INT(-1, authorization_snapshot_evaluate(snapshot, AUTHZ_LIVE_VIEW, 0, "raw", &evaluation));
    TEST_ASSERT_EQUAL_INT(-1, authorization_snapshot_evaluate(snapshot, AUTHZ_LIVE_VIEW, -1, "bad", &evaluation));
    principal.id = 0;
    authorization_snapshot_t *invalid = (void *)1;
    TEST_ASSERT_EQUAL_INT(-2, authorization_snapshot_create(&principal, &invalid));
    TEST_ASSERT_NULL(invalid);
    authorization_snapshot_free(NULL);
}

static int remove_path(const char *name, const struct stat *st, int type, struct FTW *walk) {
    (void)st; (void)type; (void)walk; return remove(name);
}
int main(void) {
    if (!mkdtemp(directory)) return 1;
    snprintf(db_path, sizeof(db_path), "%s/db.sqlite", directory);
    if (init_database(db_path) != 0 || db_auth_init() != 0) return 1;
    UNITY_BEGIN();
    RUN_TEST(test_refresh_identity_and_keep_snapshot_after_deactivation);
    RUN_TEST(test_lazy_collection_membership_reads_use_original_transaction);
    RUN_TEST(test_policy_and_inventory_cannot_mix_across_commit);
    RUN_TEST(test_scoped_token_expiry_and_revocation_are_snapshot_bound);
    RUN_TEST(test_runtime_observations_are_captured_once_and_raw_stays_raw);
    RUN_TEST(test_compact_observations_match_three_inventory_reference);
    RUN_TEST(test_database_errors_never_return_partial_snapshot_or_allow);
    RUN_TEST(test_allocation_failures_cleanup_and_retry);
    RUN_TEST(test_empty_inventory_and_invalid_inputs);
    int result = UNITY_END();
    shutdown_database();
    if (nftw(directory, remove_path, 16, FTW_DEPTH | FTW_PHYS) != 0) return 1;
    return result;
}
