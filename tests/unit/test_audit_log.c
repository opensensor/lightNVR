/**
 * @file test_audit_log.c
 * @brief Durable audit storage, redaction, retention, and HTTP contract tests.
 */

#define _POSIX_C_SOURCE 200809L

#include <cjson/cJSON.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

#include "core/config.h"
#include "core/logger.h"
#include "database/db_audit.h"
#include "database/db_auth.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#include "database/db_system_settings.h"
#include "unity.h"
#include "utils/strings.h"
#include "web/api_handlers_audit.h"
#include "web/api_handlers.h"
#include "web/audit_log.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_audit_log_test.db"

/* Test-only seams in db_audit.c, kept out of the public header. */
extern void db_audit_set_prune_budget_ms_for_testing(int budget_ms);
extern int db_audit_get_prune_interval_seconds_for_testing(void);
extern void db_audit_set_automatic_prune_state_for_testing(int64_t last_prune_at,
                                                           int interval_seconds);

#include "web/audit_summary.h"

extern void audit_log_set_clock_for_testing(int64_t (*clock_fn)(void));
extern int audit_log_reset_summaries_for_testing(size_t capacity);

static int64_t fake_now = 0;
static int64_t fake_clock(void) { return fake_now; }

static int64_t admin_user_id = 0;

static audit_event_input_t event_input(const char *request_id,
                                       const char *action,
                                       const char *outcome) {
    audit_event_input_t input = {
        .request_id = request_id,
        .principal_user_id = admin_user_id,
        .principal_username = "admin",
        .auth_method = "session",
        .action = action,
        .target_type = "camera",
        .target_uuid = "camera-0001",
        .outcome = outcome,
        .remote_address = "192.0.2.42",
        .details_json = "{\"reason\":\"unit_test\"}",
    };
    return input;
}

void setUp(void) {
    sqlite3 *db = get_db_handle();
    TEST_ASSERT_NOT_NULL(db);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK,
                          sqlite3_exec(db, "DELETE FROM audit_events;", NULL,
                                      NULL, NULL));
    TEST_ASSERT_EQUAL_INT(0, db_audit_set_retention_days(365));
    db_set_system_setting(AUDIT_DECISION_MODES_SETTING_KEY, "{}");
    g_config.web_auth_enabled = false;

    fake_now = 1757754000;
    audit_log_set_clock_for_testing(fake_clock);
    TEST_ASSERT_EQUAL_INT(0, audit_log_reset_summaries_for_testing(0));
}

void tearDown(void) {
    /* Unconditional: a failed assertion returns before a test's own cleanup,
     * and these seams are process-global, so reset them here rather than
     * letting a failure leak a zero budget or altered cadence into later
     * tests. */
    db_audit_set_prune_budget_ms_for_testing(-1);
    db_audit_set_automatic_prune_state_for_testing(
        (int64_t)time(NULL), AUDIT_PRUNE_INTERVAL_SECONDS);
}

void test_append_and_query_round_trip(void) {
    audit_event_input_t input =
        event_input("request-round-trip", "recordings.export", "success");
    char uuid[AUDIT_EVENT_UUID_MAX];
    TEST_ASSERT_EQUAL_INT(0, db_audit_append(&input, uuid));
    TEST_ASSERT_EQUAL_UINT(36, strlen(uuid));

    audit_query_t query = {.page = 1, .page_size = 20};
    safe_strcpy(query.action, "recordings.export", sizeof(query.action), 0);
    safe_strcpy(query.outcome, "success", sizeof(query.outcome), 0);
    safe_strcpy(query.target_uuid, "camera-0001",
                sizeof(query.target_uuid), 0);
    safe_strcpy(query.request_id, "request-round-trip",
                sizeof(query.request_id), 0);
    query.principal_user_id = admin_user_id;
    audit_page_t page;
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    TEST_ASSERT_EQUAL_INT64(1, page.total);
    TEST_ASSERT_EQUAL_INT(1, page.count);
    TEST_ASSERT_EQUAL_STRING(uuid, page.events[0].uuid);
    TEST_ASSERT_EQUAL_STRING("admin", page.events[0].principal_username);
    TEST_ASSERT_EQUAL_STRING("session", page.events[0].auth_method);
    TEST_ASSERT_EQUAL_STRING("{\"reason\":\"unit_test\"}",
                             page.events[0].details_json);
    db_audit_page_free(&page);
}

void test_append_rejects_invalid_or_non_object_payloads(void) {
    audit_event_input_t input =
        event_input("request-invalid", "system.admin", "unexpected");
    TEST_ASSERT_EQUAL_INT(-1, db_audit_append(&input, NULL));
    input.outcome = "denied";
    input.details_json = "[\"not-an-object\"]";
    TEST_ASSERT_EQUAL_INT(-1, db_audit_append(&input, NULL));
    input.details_json = "{\"bad\":true}";
    input.request_id = "request\r\ninvalid";
    TEST_ASSERT_EQUAL_INT(-1, db_audit_append(&input, NULL));

    audit_query_t query = {.page = 1, .page_size = 20};
    audit_page_t page;
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    TEST_ASSERT_EQUAL_INT64(0, page.total);
    TEST_ASSERT_EQUAL_INT(0, page.count);
    db_audit_page_free(&page);
}

void test_query_paginates_newest_first(void) {
    audit_event_input_t first =
        event_input("request-page-1", "ptz.control", "allowed");
    audit_event_input_t second =
        event_input("request-page-2", "ptz.control", "denied");
    audit_event_input_t third =
        event_input("request-page-3", "ptz.control", "success");
    first.occurred_at = 1000;
    second.occurred_at = 2000;
    third.occurred_at = 3000;
    TEST_ASSERT_EQUAL_INT(0, db_audit_append(&first, NULL));
    TEST_ASSERT_EQUAL_INT(0, db_audit_append(&second, NULL));
    TEST_ASSERT_EQUAL_INT(0, db_audit_append(&third, NULL));

    audit_query_t query = {.page = 2, .page_size = 2};
    safe_strcpy(query.action, "ptz.control", sizeof(query.action), 0);
    audit_page_t page;
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    TEST_ASSERT_EQUAL_INT64(3, page.total);
    TEST_ASSERT_EQUAL_INT(1, page.count);
    TEST_ASSERT_EQUAL_STRING("request-page-1", page.events[0].request_id);
    db_audit_page_free(&page);
}

void test_retention_prunes_expired_events(void) {
    TEST_ASSERT_EQUAL_INT(0, db_audit_set_retention_days(3650));
    audit_event_input_t expired =
        event_input("request-expired", "camera.configure", "success");
    expired.occurred_at = (int64_t)time(NULL) - 45LL * 24 * 60 * 60;
    TEST_ASSERT_EQUAL_INT(0, db_audit_append(&expired, NULL));

    TEST_ASSERT_EQUAL_INT(0, db_audit_set_retention_days(30));
    int deleted = 0;
    TEST_ASSERT_EQUAL_INT(0, db_audit_prune(&deleted));
    TEST_ASSERT_EQUAL_INT(1, deleted);

    int retention_days = 0;
    TEST_ASSERT_EQUAL_INT(0, db_audit_get_retention_days(&retention_days));
    TEST_ASSERT_EQUAL_INT(30, retention_days);
}

/*
 * Pruning runs from the audit insert path with the global database mutex
 * held. It used to issue one open-ended DELETE, so the first prune after a
 * backlog accumulated -- or an administrator lowering the retention window on
 * an existing table -- blocked every other database user for as long as the
 * delete took (measured at 2m31s for 5.33M rows). It now deletes in bounded
 * batches, which must still drain the whole backlog and must leave events
 * inside the retention window alone.
 *
 * Tagged with its own target_uuid so it neither depends on nor disturbs the
 * rows other tests in this binary leave behind.
 */
void test_retention_drains_large_backlog_without_touching_live_events(void) {
    const int expired_count = AUDIT_PRUNE_BATCH_ROWS + 100;  /* > one batch */
    const int fresh_count = 5;
    const int64_t now = (int64_t)time(NULL);

    TEST_ASSERT_EQUAL_INT(0, db_audit_set_retention_days(3650));

    for (int i = 0; i < expired_count; i++) {
        audit_event_input_t expired =
            event_input("request-backlog", "camera.configure", "success");
        expired.target_uuid = "camera-backlog-expired";
        expired.occurred_at = now - 45LL * 24 * 60 * 60;
        TEST_ASSERT_EQUAL_INT(0, db_audit_append(&expired, NULL));
    }
    for (int i = 0; i < fresh_count; i++) {
        audit_event_input_t fresh =
            event_input("request-backlog", "camera.configure", "success");
        fresh.target_uuid = "camera-backlog-fresh";
        fresh.occurred_at = now;
        TEST_ASSERT_EQUAL_INT(0, db_audit_append(&fresh, NULL));
    }

    TEST_ASSERT_EQUAL_INT(0, db_audit_set_retention_days(30));

    /* Drains across however many bounded passes it takes, and terminates. */
    int guard = 0;
    for (;;) {
        int deleted = 0;
        TEST_ASSERT_EQUAL_INT(0, db_audit_prune(&deleted));
        if (deleted == 0) break;
        TEST_ASSERT_TRUE_MESSAGE(++guard < 1000, "prune did not converge");
    }

    audit_query_t expired_query = {.page = 1, .page_size = 1};
    safe_strcpy(expired_query.target_uuid, "camera-backlog-expired",
                sizeof(expired_query.target_uuid), 0);
    audit_page_t expired_page;
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&expired_query, &expired_page));
    TEST_ASSERT_EQUAL_INT64(0, expired_page.total);
    db_audit_page_free(&expired_page);

    audit_query_t fresh_query = {.page = 1, .page_size = 1};
    safe_strcpy(fresh_query.target_uuid, "camera-backlog-fresh",
                sizeof(fresh_query.target_uuid), 0);
    audit_page_t fresh_page;
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&fresh_query, &fresh_page));
    TEST_ASSERT_EQUAL_INT64(fresh_count, fresh_page.total);
    db_audit_page_free(&fresh_page);
}

/*
 * Guards the batching itself, not just deletion correctness. With the budget
 * forced to zero the deadline has already passed by the time the first batch
 * returns, so exactly one batch is deleted -- an unbounded DELETE would take
 * the whole backlog in the first pass and fail the first assertion. Also
 * covers the cadence transition: shortened while a backlog remains, restored
 * once drained.
 */
void test_prune_stops_on_budget_and_resumes_until_drained(void) {
    const int backlog = AUDIT_PRUNE_BATCH_ROWS * 2;
    const int64_t expired_at = (int64_t)time(NULL) - 45LL * 24 * 60 * 60;

    TEST_ASSERT_EQUAL_INT(0, db_audit_set_retention_days(3650));
    for (int i = 0; i < backlog; i++) {
        audit_event_input_t event =
            event_input("request-budget", "camera.configure", "success");
        event.target_uuid = "camera-budget";
        event.occurred_at = expired_at;
        TEST_ASSERT_EQUAL_INT(0, db_audit_append(&event, NULL));
    }
    TEST_ASSERT_EQUAL_INT(0, db_audit_set_retention_days(30));

    db_audit_set_prune_budget_ms_for_testing(0);

    int deleted = 0;
    TEST_ASSERT_EQUAL_INT(0, db_audit_prune(&deleted));
    TEST_ASSERT_EQUAL_INT(AUDIT_PRUNE_BATCH_ROWS, deleted);
    TEST_ASSERT_EQUAL_INT(AUDIT_PRUNE_BACKLOG_INTERVAL_SECONDS,
                          db_audit_get_prune_interval_seconds_for_testing());

    int guard = 0;
    for (;;) {
        int pass_deleted = 0;
        TEST_ASSERT_EQUAL_INT(0, db_audit_prune(&pass_deleted));
        deleted += pass_deleted;
        if (pass_deleted == 0) break;
        TEST_ASSERT_TRUE_MESSAGE(++guard < 100, "prune did not converge");
    }
    TEST_ASSERT_EQUAL_INT(backlog, deleted);
    TEST_ASSERT_EQUAL_INT(AUDIT_PRUNE_INTERVAL_SECONDS,
                          db_audit_get_prune_interval_seconds_for_testing());
    /* Budget override is reset unconditionally in tearDown(). */
}

static int64_t audit_rows_for_target(const char *target_uuid) {
    audit_query_t query = {.page = 1, .page_size = 1};
    safe_strcpy(query.target_uuid, target_uuid, sizeof(query.target_uuid), 0);
    audit_page_t page;
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    int64_t total = page.total;
    db_audit_page_free(&page);
    return total;
}

static void append_prune_trigger(void) {
    audit_event_input_t trigger =
        event_input("request-auto-trigger", "camera.configure", "success");
    trigger.target_uuid = "camera-auto-trigger";
    TEST_ASSERT_EQUAL_INT(0, db_audit_append(&trigger, NULL));
}

/*
 * Production pruning is driven from db_audit_append(), not db_audit_prune(),
 * so exercise that caller directly: a budget-limited pass must shorten the
 * insert-path eligibility window, an append inside that window must not
 * prune, an append past it must run the next batch, and the hourly cadence
 * must return once the backlog is gone.
 */
void test_automatic_prune_on_append_follows_backlog_cadence(void) {
    const int backlog = AUDIT_PRUNE_BATCH_ROWS * 2;
    const int64_t expired_at = (int64_t)time(NULL) - 45LL * 24 * 60 * 60;

    TEST_ASSERT_EQUAL_INT(0, db_audit_set_retention_days(3650));
    for (int i = 0; i < backlog; i++) {
        audit_event_input_t event =
            event_input("request-auto", "camera.configure", "success");
        event.target_uuid = "camera-auto-expired";
        event.occurred_at = expired_at;
        TEST_ASSERT_EQUAL_INT(0, db_audit_append(&event, NULL));
    }
    TEST_ASSERT_EQUAL_INT(0, db_audit_set_retention_days(30));
    db_audit_set_prune_budget_ms_for_testing(0);

    /* Eligible under the hourly cadence: one batch, then the backlog window. */
    db_audit_set_automatic_prune_state_for_testing(
        (int64_t)time(NULL) - AUDIT_PRUNE_INTERVAL_SECONDS,
        AUDIT_PRUNE_INTERVAL_SECONDS);
    append_prune_trigger();
    TEST_ASSERT_EQUAL_INT64(backlog - AUDIT_PRUNE_BATCH_ROWS,
                            audit_rows_for_target("camera-auto-expired"));
    TEST_ASSERT_EQUAL_INT(AUDIT_PRUNE_BACKLOG_INTERVAL_SECONDS,
                          db_audit_get_prune_interval_seconds_for_testing());

    /* Inside the backlog window: the append must not prune. */
    db_audit_set_automatic_prune_state_for_testing(
        (int64_t)time(NULL), AUDIT_PRUNE_BACKLOG_INTERVAL_SECONDS);
    append_prune_trigger();
    TEST_ASSERT_EQUAL_INT64(backlog - AUDIT_PRUNE_BATCH_ROWS,
                            audit_rows_for_target("camera-auto-expired"));

    /* Past the backlog window: the next batch runs. It deletes a full batch,
     * so the budget-limited pass still reports a backlog. */
    db_audit_set_automatic_prune_state_for_testing(
        (int64_t)time(NULL) - AUDIT_PRUNE_BACKLOG_INTERVAL_SECONDS,
        AUDIT_PRUNE_BACKLOG_INTERVAL_SECONDS);
    append_prune_trigger();
    TEST_ASSERT_EQUAL_INT64(0, audit_rows_for_target("camera-auto-expired"));
    TEST_ASSERT_EQUAL_INT(AUDIT_PRUNE_BACKLOG_INTERVAL_SECONDS,
                          db_audit_get_prune_interval_seconds_for_testing());

    /* A pass that finds nothing restores the hourly cadence. */
    db_audit_set_automatic_prune_state_for_testing(
        (int64_t)time(NULL) - AUDIT_PRUNE_BACKLOG_INTERVAL_SECONDS,
        AUDIT_PRUNE_BACKLOG_INTERVAL_SECONDS);
    append_prune_trigger();
    TEST_ASSERT_EQUAL_INT(AUDIT_PRUNE_INTERVAL_SECONDS,
                          db_audit_get_prune_interval_seconds_for_testing());
}

/*
 * The common case is a populated table with nothing expired. Ordering the
 * inner query by id instead of occurred_at makes SQLite scan every surviving
 * row while the global database mutex is held, so assert on the query plan
 * directly -- a timing assertion would be flaky, and deletion counts alone
 * cannot tell a scan from an index seek.
 */
void test_prune_batch_query_uses_the_retention_index(void) {
    const int fresh_rows = 500;
    const int64_t now = (int64_t)time(NULL);
    TEST_ASSERT_EQUAL_INT(0, db_audit_set_retention_days(30));
    for (int i = 0; i < fresh_rows; i++) {
        audit_event_input_t event =
            event_input("request-noexpiry", "camera.configure", "success");
        event.target_uuid = "camera-noexpiry";
        event.occurred_at = now;
        TEST_ASSERT_EQUAL_INT(0, db_audit_append(&event, NULL));
    }

    int deleted = -1;
    TEST_ASSERT_EQUAL_INT(0, db_audit_prune(&deleted));
    TEST_ASSERT_EQUAL_INT(0, deleted);

    sqlite3 *db = get_db_handle();
    TEST_ASSERT_NOT_NULL(db);
    char explain[512];
    snprintf(explain, sizeof(explain), "EXPLAIN QUERY PLAN %s",
             AUDIT_PRUNE_BATCH_SQL);
    sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK,
                          sqlite3_prepare_v2(db, explain, -1, &stmt, NULL));
    sqlite3_bind_int64(stmt, 1, now);
    sqlite3_bind_int(stmt, 2, AUDIT_PRUNE_BATCH_ROWS);

    bool uses_retention_index = false;
    bool scans_table = false;
    while (sqlite3_step(stmt) == SQLITE_ROW) {
        const char *detail = (const char *)sqlite3_column_text(stmt, 3);
        if (!detail) continue;
        if (strstr(detail, "idx_audit_events_occurred")) {
            uses_retention_index = true;
        }
        if (strstr(detail, "SCAN audit_events")) scans_table = true;
    }
    sqlite3_finalize(stmt);

    TEST_ASSERT_TRUE_MESSAGE(uses_retention_index,
                             "prune batch must use idx_audit_events_occurred");
    TEST_ASSERT_FALSE_MESSAGE(scans_table,
                              "prune batch must not scan audit_events");
}

void test_web_helper_redacts_sensitive_detail_fields(void) {
    http_request_t req;
    http_request_init(&req);
    safe_strcpy(req.path, "/api/example", sizeof(req.path), 0);
    safe_strcpy(req.method_str, "POST", sizeof(req.method_str), 0);
    safe_strcpy(req.client_ip, "198.51.100.9", sizeof(req.client_ip), 0);
    user_t user = {0};
    user.id = admin_user_id;
    safe_strcpy(user.username, "admin", sizeof(user.username), 0);
    safe_strcpy(user.authentication_method, "session",
                sizeof(user.authentication_method), 0);
    cJSON *details = cJSON_CreateObject();
    cJSON_AddStringToObject(details, "password", "do-not-store");
    cJSON_AddStringToObject(details, "bearer_token", "also-secret");
    cJSON_AddStringToObject(details, "license_plate", "TEST123");
    cJSON_AddStringToObject(details, "api_token_uuid", "safe-id");
    cJSON_AddStringToObject(details, "reason", "operator request");
    audit_log_append(&req, &user, "example.update", "example", "target-1",
                     "success", details);
    cJSON_Delete(details);

    audit_query_t query = {.page = 1, .page_size = 20};
    safe_strcpy(query.request_id, req.request_id, sizeof(query.request_id), 0);
    audit_page_t page;
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    TEST_ASSERT_EQUAL_INT(1, page.count);
    TEST_ASSERT_NULL(strstr(page.events[0].details_json, "do-not-store"));
    TEST_ASSERT_NULL(strstr(page.events[0].details_json, "also-secret"));
    TEST_ASSERT_NULL(strstr(page.events[0].details_json, "TEST123"));
    TEST_ASSERT_NOT_NULL(strstr(page.events[0].details_json,
                                "\"password\":\"[REDACTED]\""));
    TEST_ASSERT_NOT_NULL(strstr(page.events[0].details_json,
                                "\"license_plate\":\"[REDACTED]\""));
    TEST_ASSERT_NOT_NULL(strstr(page.events[0].details_json,
                                "\"api_token_uuid\":\"safe-id\""));
    db_audit_page_free(&page);
}

void test_operation_helper_adds_standard_envelope_and_redacts_context(void) {
    http_request_t req;
    http_request_init(&req);
    req.method = HTTP_METHOD_POST;
    safe_strcpy(req.path, "/api/streams/front/ptz/move",
                sizeof(req.path), 0);
    safe_strcpy(req.method_str, "POST", sizeof(req.method_str), 0);
    safe_strcpy(req.client_ip, "198.51.100.10", sizeof(req.client_ip), 0);
    user_t user = {0};
    user.id = admin_user_id;
    safe_strcpy(user.username, "admin", sizeof(user.username), 0);
    safe_strcpy(user.authentication_method, "session",
                sizeof(user.authentication_method), 0);
    cJSON *context = cJSON_CreateObject();
    cJSON_AddStringToObject(context, "reason", "completed");
    cJSON_AddStringToObject(context, "onvif_password", "never-store-this");
    audit_log_operation(&req, &user, "ptz.control", "camera", "camera-2",
                        "continuous_move", "success", context);
    cJSON_Delete(context);

    audit_query_t query = {.page = 1, .page_size = 20};
    safe_strcpy(query.request_id, req.request_id, sizeof(query.request_id), 0);
    audit_page_t page;
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    TEST_ASSERT_EQUAL_INT(1, page.count);
    TEST_ASSERT_EQUAL_STRING("ptz.control", page.events[0].action);
    TEST_ASSERT_EQUAL_STRING("success", page.events[0].outcome);
    TEST_ASSERT_NOT_NULL(strstr(page.events[0].details_json,
                                "\"event_type\":\"operation.outcome\""));
    TEST_ASSERT_NOT_NULL(strstr(page.events[0].details_json,
                                "\"operation\":\"continuous_move\""));
    TEST_ASSERT_NOT_NULL(strstr(page.events[0].details_json,
                                "\"onvif_password\":\"[REDACTED]\""));
    TEST_ASSERT_NULL(strstr(page.events[0].details_json, "never-store-this"));
    db_audit_page_free(&page);
}

void test_camera_configuration_route_outcomes_cover_success_failure_and_error(void) {
    http_request_t stream_req;
    http_request_init(&stream_req);
    safe_strcpy(stream_req.path, "/api/streams", sizeof(stream_req.path), 0);
    safe_strcpy(stream_req.method_str, "POST",
                sizeof(stream_req.method_str), 0);
    stream_req.body = "{\"name\":\"audit-camera\","
                      "\"url\":\"rtsp://operator:secret@camera/live\"}";
    stream_req.body_len = strlen(stream_req.body);
    http_response_t stream_res;
    http_response_init(&stream_res);
    http_response_set_json(
        &stream_res, 201,
        "{\"camera_uuid\":\"aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa\"}");
    audit_log_sensitive_operation_outcome(&stream_req, &stream_res);
    http_response_free(&stream_res);

    stream_config_t deleted_stream;
    memset(&deleted_stream, 0, sizeof(deleted_stream));
    safe_strcpy(deleted_stream.name, "audit-delete",
                sizeof(deleted_stream.name), 0);
    safe_strcpy(deleted_stream.url, "rtsp://camera/live",
                sizeof(deleted_stream.url), 0);
    safe_strcpy(deleted_stream.codec, "h264", sizeof(deleted_stream.codec), 0);
    deleted_stream.enabled = true;
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&deleted_stream));
    TEST_ASSERT_EQUAL_INT(
        0, get_stream_config_by_name("audit-delete", &deleted_stream));
    http_request_t delete_req;
    http_request_init(&delete_req);
    safe_strcpy(delete_req.path, "/api/streams/audit-delete",
                sizeof(delete_req.path), 0);
    safe_strcpy(delete_req.method_str, "DELETE",
                sizeof(delete_req.method_str), 0);
    audit_sensitive_operation_context_t delete_context;
    audit_log_sensitive_operation_begin(&delete_req, &delete_context);
    TEST_ASSERT_EQUAL_INT(0, delete_stream_config("audit-delete"));
    http_response_t delete_res;
    http_response_init(&delete_res);
    delete_res.status_code = 202;
    audit_log_sensitive_operation_end(&delete_req, &delete_res,
                                      &delete_context);

    http_request_t tag_req;
    http_request_init(&tag_req);
    safe_strcpy(tag_req.path,
                "/api/camera-tags/11111111-1111-4111-8111-111111111111",
                sizeof(tag_req.path), 0);
    safe_strcpy(tag_req.method_str, "PUT", sizeof(tag_req.method_str), 0);
    http_response_t tag_res;
    http_response_init(&tag_res);
    tag_res.status_code = 400;
    audit_log_sensitive_operation_outcome(&tag_req, &tag_res);

    http_request_t onvif_req;
    http_request_init(&onvif_req);
    safe_strcpy(onvif_req.path, "/api/onvif/device/test",
                sizeof(onvif_req.path), 0);
    safe_strcpy(onvif_req.method_str, "POST",
                sizeof(onvif_req.method_str), 0);
    onvif_req.body = "{\"password\":\"do-not-audit\","
                     "\"url\":\"http://camera/onvif/device_service\"}";
    onvif_req.body_len = strlen(onvif_req.body);
    http_response_t onvif_res;
    http_response_init(&onvif_res);
    onvif_res.status_code = 502;
    audit_log_sensitive_operation_outcome(&onvif_req, &onvif_res);

    audit_query_t query = {.page = 1, .page_size = 20};
    safe_strcpy(query.action, "camera.configure", sizeof(query.action), 0);
    audit_page_t page;
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    TEST_ASSERT_EQUAL_INT(4, page.count);
    bool create_success = false;
    bool delete_success = false;
    bool tag_failure = false;
    bool onvif_error = false;
    for (int i = 0; i < page.count; i++) {
        if (strcmp(page.events[i].outcome, "success") == 0 &&
            strstr(page.events[i].details_json,
                   "\"operation\":\"stream.create\"") != NULL) {
            create_success = true;
            TEST_ASSERT_EQUAL_STRING(
                "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa",
                page.events[i].target_uuid);
        }
        if (strcmp(page.events[i].outcome, "success") == 0 &&
            strstr(page.events[i].details_json,
                   "\"operation\":\"stream.delete\"") != NULL) {
            delete_success = true;
            TEST_ASSERT_EQUAL_STRING(deleted_stream.camera_uuid,
                                     page.events[i].target_uuid);
        }
        tag_failure |= strcmp(page.events[i].outcome, "failure") == 0 &&
            strstr(page.events[i].details_json,
                   "\"operation\":\"camera_tag.update\"") != NULL;
        onvif_error |= strcmp(page.events[i].outcome, "error") == 0 &&
            strstr(page.events[i].details_json,
                   "\"operation\":\"onvif.test\"") != NULL;
        TEST_ASSERT_NULL(strstr(page.events[i].details_json, "do-not-audit"));
        TEST_ASSERT_NULL(strstr(page.events[i].details_json, "operator:secret"));
    }
    TEST_ASSERT_TRUE(create_success);
    TEST_ASSERT_TRUE(delete_success);
    TEST_ASSERT_TRUE(tag_failure);
    TEST_ASSERT_TRUE(onvif_error);
    db_audit_page_free(&page);
}

void test_system_mutation_route_outcomes_are_audited(void) {
    const char *paths[] = {
        "/api/system/logs/clear", "/api/settings"
    };
    const int statuses[] = {500, 400};
    for (int i = 0; i < 2; i++) {
        http_request_t req;
        http_request_init(&req);
        safe_strcpy(req.path, paths[i], sizeof(req.path), 0);
        safe_strcpy(req.method_str, "POST", sizeof(req.method_str), 0);
        http_response_t res;
        http_response_init(&res);
        res.status_code = statuses[i];
        audit_log_sensitive_operation_outcome(&req, &res);
        http_response_free(&res);
    }

    audit_query_t query = {.page = 1, .page_size = 20};
    safe_strcpy(query.action, "system.admin", sizeof(query.action), 0);
    audit_page_t page;
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    TEST_ASSERT_EQUAL_INT(2, page.count);
    bool logs_error = false;
    bool settings_failure = false;
    for (int i = 0; i < page.count; i++) {
        logs_error |= strcmp(page.events[i].outcome, "error") == 0 &&
            strstr(page.events[i].details_json,
                   "\"operation\":\"system.logs.clear\"") != NULL;
        settings_failure |= strcmp(page.events[i].outcome, "failure") == 0 &&
            strstr(page.events[i].details_json,
                   "\"operation\":\"settings.update\"") != NULL;
    }
    TEST_ASSERT_TRUE(logs_error);
    TEST_ASSERT_TRUE(settings_failure);
    db_audit_page_free(&page);
}

void test_audit_http_settings_list_and_csv_export(void) {
    audit_event_input_t input =
        event_input("request-csv", "recordings.export", "success");
    input.principal_username = "=HYPERLINK(\"https://invalid\")";
    TEST_ASSERT_EQUAL_INT(0, db_audit_append(&input, NULL));

    http_request_t put_req;
    http_request_init(&put_req);
    safe_strcpy(put_req.path, "/api/audit/settings",
                sizeof(put_req.path), 0);
    safe_strcpy(put_req.method_str, "PUT", sizeof(put_req.method_str), 0);
    put_req.body = "{\"retention_days\":90}";
    put_req.body_len = strlen(put_req.body);
    http_response_t put_res;
    http_response_init(&put_res);
    handle_put_audit_settings(&put_req, &put_res);
    TEST_ASSERT_EQUAL_INT(200, put_res.status_code);
    TEST_ASSERT_NOT_NULL(strstr(put_res.body, "\"retention_days\":90"));
    http_response_free(&put_res);

    http_request_t list_req;
    http_request_init(&list_req);
    safe_strcpy(list_req.path, "/api/audit/events", sizeof(list_req.path), 0);
    safe_strcpy(list_req.method_str, "GET", sizeof(list_req.method_str), 0);
    safe_strcpy(list_req.query_string,
                "page=1&page_size=10&action=recordings.export",
                sizeof(list_req.query_string), 0);
    http_response_t list_res;
    http_response_init(&list_res);
    handle_get_audit_events(&list_req, &list_res);
    TEST_ASSERT_EQUAL_INT(200, list_res.status_code);
    cJSON *list = cJSON_Parse(list_res.body);
    TEST_ASSERT_TRUE(cJSON_IsObject(list));
    TEST_ASSERT_EQUAL_INT(
        1, cJSON_GetObjectItemCaseSensitive(list, "count")->valueint);
    cJSON_Delete(list);
    http_response_free(&list_res);

    http_request_t export_req;
    http_request_init(&export_req);
    safe_strcpy(export_req.path, "/api/audit/events/export",
                sizeof(export_req.path), 0);
    safe_strcpy(export_req.method_str, "GET",
                sizeof(export_req.method_str), 0);
    safe_strcpy(export_req.query_string,
                "page=1&page_size=20&action=recordings.export",
                sizeof(export_req.query_string), 0);
    http_response_t export_res;
    http_response_init(&export_res);
    handle_get_audit_export(&export_req, &export_res);
    TEST_ASSERT_EQUAL_INT(200, export_res.status_code);
    TEST_ASSERT_EQUAL_STRING("text/csv; charset=utf-8",
                             export_res.content_type);
    TEST_ASSERT_NOT_NULL(strstr(export_res.body,
                                "\"'=HYPERLINK(\"\"https://invalid\"\")\""));
    http_response_free(&export_res);
}

void test_login_success_and_denial_are_audited_without_credentials(void) {
    g_config.login_rate_limit_enabled = false;
    g_config.force_mfa_on_login = false;

    http_request_t denied_req;
    http_request_init(&denied_req);
    safe_strcpy(denied_req.path, "/api/auth/login",
                sizeof(denied_req.path), 0);
    safe_strcpy(denied_req.method_str, "POST",
                sizeof(denied_req.method_str), 0);
    safe_strcpy(denied_req.client_ip, "203.0.113.10",
                sizeof(denied_req.client_ip), 0);
    denied_req.body = "{\"username\":\"unknown\",\"password\":\"bad-secret\"}";
    denied_req.body_len = strlen(denied_req.body);
    http_response_t denied_res;
    http_response_init(&denied_res);
    handle_auth_login(&denied_req, &denied_res);
    TEST_ASSERT_EQUAL_INT(401, denied_res.status_code);
    http_response_free(&denied_res);

    http_request_t success_req;
    http_request_init(&success_req);
    safe_strcpy(success_req.path, "/api/auth/login",
                sizeof(success_req.path), 0);
    safe_strcpy(success_req.method_str, "POST",
                sizeof(success_req.method_str), 0);
    safe_strcpy(success_req.client_ip, "203.0.113.11",
                sizeof(success_req.client_ip), 0);
    success_req.body = "{\"username\":\"admin\",\"password\":\"admin\"}";
    success_req.body_len = strlen(success_req.body);
    http_response_t success_res;
    http_response_init(&success_res);
    handle_auth_login(&success_req, &success_res);
    TEST_ASSERT_EQUAL_INT(200, success_res.status_code);
    http_response_free(&success_res);

    audit_query_t query = {.page = 1, .page_size = 20};
    safe_strcpy(query.action, "auth.login", sizeof(query.action), 0);
    audit_page_t page;
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    TEST_ASSERT_EQUAL_INT64(2, page.total);
    TEST_ASSERT_EQUAL_INT(2, page.count);
    bool found_success = false;
    bool found_denied = false;
    for (int i = 0; i < page.count; i++) {
        found_success |= strcmp(page.events[i].outcome, "success") == 0;
        found_denied |= strcmp(page.events[i].outcome, "denied") == 0;
        TEST_ASSERT_NULL(strstr(page.events[i].details_json, "bad-secret"));
        TEST_ASSERT_NULL(strstr(page.events[i].details_json, "\"password\""));
    }
    TEST_ASSERT_TRUE(found_success);
    TEST_ASSERT_TRUE(found_denied);
    db_audit_page_free(&page);
}

void test_audit_api_denies_and_records_unauthenticated_access(void) {
    g_config.web_auth_enabled = true;
    http_request_t req;
    http_request_init(&req);
    safe_strcpy(req.path, "/api/audit/events", sizeof(req.path), 0);
    safe_strcpy(req.method_str, "GET", sizeof(req.method_str), 0);
    safe_strcpy(req.client_ip, "192.0.2.200", sizeof(req.client_ip), 0);
    http_response_t res;
    http_response_init(&res);
    handle_get_audit_events(&req, &res);
    TEST_ASSERT_EQUAL_INT(401, res.status_code);
    http_response_free(&res);

    audit_query_t query = {.page = 1, .page_size = 20};
    safe_strcpy(query.request_id, req.request_id, sizeof(query.request_id), 0);
    audit_page_t page;
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    TEST_ASSERT_EQUAL_INT(1, page.count);
    TEST_ASSERT_EQUAL_STRING("system.admin", page.events[0].action);
    TEST_ASSERT_EQUAL_STRING("denied", page.events[0].outcome);
    TEST_ASSERT_EQUAL_STRING("unauthenticated", page.events[0].auth_method);
    db_audit_page_free(&page);
}

void test_decision_modes_default_to_record_when_unset(void) {
    audit_decision_mode_t modes[AUTHZ_ACTION_COUNT];
    for (int i = 0; i < AUTHZ_ACTION_COUNT; i++) modes[i] = AUDIT_DECISION_MODE_OFF;
    TEST_ASSERT_EQUAL_INT(0, db_audit_load_decision_modes(modes));
    for (int i = 0; i < AUTHZ_ACTION_COUNT; i++) {
        TEST_ASSERT_EQUAL_INT(AUDIT_DECISION_MODE_RECORD, modes[i]);
    }
}

void test_decision_modes_round_trip_and_store_only_non_defaults(void) {
    audit_decision_mode_t modes[AUTHZ_ACTION_COUNT] = {0};
    modes[AUTHZ_LIVE_VIEW] = AUDIT_DECISION_MODE_SUMMARIZE;
    modes[AUTHZ_SYSTEM_ADMIN] = AUDIT_DECISION_MODE_OFF;
    TEST_ASSERT_EQUAL_INT(0, db_audit_save_decision_modes(modes));

    char stored[512];
    TEST_ASSERT_EQUAL_INT(0, db_get_system_setting(
        AUDIT_DECISION_MODES_SETTING_KEY, stored, sizeof(stored)));
    cJSON *object = cJSON_Parse(stored);
    TEST_ASSERT_TRUE(cJSON_IsObject(object));
    TEST_ASSERT_EQUAL_INT(2, cJSON_GetArraySize(object));
    TEST_ASSERT_EQUAL_STRING("summarize",
        cJSON_GetObjectItemCaseSensitive(object, "live.view")->valuestring);
    TEST_ASSERT_EQUAL_STRING("off",
        cJSON_GetObjectItemCaseSensitive(object, "system.admin")->valuestring);
    cJSON_Delete(object);

    audit_decision_mode_t loaded[AUTHZ_ACTION_COUNT];
    TEST_ASSERT_EQUAL_INT(0, db_audit_load_decision_modes(loaded));
    for (int i = 0; i < AUTHZ_ACTION_COUNT; i++) {
        TEST_ASSERT_EQUAL_INT(modes[i], loaded[i]);
    }
}

void test_decision_modes_ignore_invalid_stored_entries(void) {
    TEST_ASSERT_EQUAL_INT(0, db_set_system_setting(
        AUDIT_DECISION_MODES_SETTING_KEY,
        "{\"live.view\":\"summarize\",\"no.such.action\":\"off\","
        "\"system.admin\":\"shout\",\"recordings.replay\":7}"));
    audit_decision_mode_t loaded[AUTHZ_ACTION_COUNT];
    TEST_ASSERT_EQUAL_INT(0, db_audit_load_decision_modes(loaded));
    TEST_ASSERT_EQUAL_INT(AUDIT_DECISION_MODE_SUMMARIZE, loaded[AUTHZ_LIVE_VIEW]);
    TEST_ASSERT_EQUAL_INT(AUDIT_DECISION_MODE_RECORD, loaded[AUTHZ_SYSTEM_ADMIN]);
    TEST_ASSERT_EQUAL_INT(AUDIT_DECISION_MODE_RECORD, loaded[AUTHZ_RECORDINGS_REPLAY]);

    TEST_ASSERT_EQUAL_INT(0, db_set_system_setting(
        AUDIT_DECISION_MODES_SETTING_KEY, "not json"));
    TEST_ASSERT_EQUAL_INT(0, db_audit_load_decision_modes(loaded));
    TEST_ASSERT_EQUAL_INT(AUDIT_DECISION_MODE_RECORD, loaded[AUTHZ_LIVE_VIEW]);
}

static void decision_request(http_request_t *req, const char *method,
                             const char *path, const char *client_ip) {
    http_request_init(req);
    safe_strcpy(req->method_str, method, sizeof(req->method_str), 0);
    safe_strcpy(req->path, path, sizeof(req->path), 0);
    safe_strcpy(req->client_ip, client_ip, sizeof(req->client_ip), 0);
}

static user_t decision_user(void) {
    user_t user;
    memset(&user, 0, sizeof(user));
    user.id = admin_user_id;
    safe_strcpy(user.username, "admin", sizeof(user.username), 0);
    safe_strcpy(user.authentication_method, "session",
                sizeof(user.authentication_method), 0);
    return user;
}

static fleet_camera_t decision_camera(const char *uuid) {
    fleet_camera_t camera;
    memset(&camera, 0, sizeof(camera));
    safe_strcpy(camera.camera_uuid, uuid, sizeof(camera.camera_uuid), 0);
    return camera;
}

static void allow_decision(const char *method, const char *path,
                           const char *client_ip, const char *camera_uuid,
                           const char *outcome) {
    http_request_t req;
    decision_request(&req, method, path, client_ip);
    user_t user = decision_user();
    fleet_camera_t camera = decision_camera(camera_uuid);
    authorization_evaluation_t evaluation;
    memset(&evaluation, 0, sizeof(evaluation));
    evaluation.decision = AUTHZ_DECISION_ALLOW;
    evaluation.source = AUTHZ_SOURCE_POLICY_GRANT;
    safe_strcpy(evaluation.explanation, "Allowed by Administrator role grant",
                sizeof(evaluation.explanation), 0);
    audit_log_authorization(&req, &user, AUTHZ_LIVE_VIEW, &camera, &evaluation,
                            outcome);
}

static int64_t live_view_rows(const char *outcome) {
    audit_query_t query = {.page = 1, .page_size = 50};
    safe_strcpy(query.action, "live.view", sizeof(query.action), 0);
    if (outcome) safe_strcpy(query.outcome, outcome, sizeof(query.outcome), 0);
    audit_page_t page;
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    int64_t total = page.total;
    db_audit_page_free(&page);
    return total;
}

static void set_live_view_mode(audit_decision_mode_t mode) {
    audit_decision_mode_t modes[AUTHZ_ACTION_COUNT] = {0};
    modes[AUTHZ_LIVE_VIEW] = mode;
    TEST_ASSERT_EQUAL_INT(0, audit_log_set_decision_modes(modes));
}

void test_record_mode_writes_one_row_per_allowed_read(void) {
    for (int i = 0; i < 3; i++) {
        allow_decision("GET", "/api/detection/results/cam", "192.0.2.10", "cam-1", "allowed");
    }
    TEST_ASSERT_EQUAL_INT64(3, live_view_rows("allowed"));
}

void test_off_mode_skips_allowed_reads_but_records_denials(void) {
    set_live_view_mode(AUDIT_DECISION_MODE_OFF);
    allow_decision("GET", "/api/detection/results/cam", "192.0.2.10", "cam-1", "allowed");
    allow_decision("GET", "/api/detection/results/cam", "192.0.2.10", "cam-1", "allowed");
    allow_decision("GET", "/api/detection/results/cam", "192.0.2.10", "cam-1", "denied");
    TEST_ASSERT_EQUAL_INT64(0, live_view_rows("allowed"));
    TEST_ASSERT_EQUAL_INT64(1, live_view_rows("denied"));
}

void test_summarize_buffers_until_flush_then_writes_one_counted_row(void) {
    set_live_view_mode(AUDIT_DECISION_MODE_SUMMARIZE);
    for (int i = 0; i < 5; i++) {
        fake_now = 1757754000 + i;
        allow_decision("GET", "/api/detection/results/cam", "192.0.2.10", "cam-1", "allowed");
    }
    TEST_ASSERT_EQUAL_INT64(0, live_view_rows("allowed"));
    TEST_ASSERT_EQUAL_UINT(1, audit_log_flush_summaries(false));
    TEST_ASSERT_EQUAL_INT64(1, live_view_rows("allowed"));

    audit_query_t query = {.page = 1, .page_size = 1};
    safe_strcpy(query.action, "live.view", sizeof(query.action), 0);
    audit_page_t page;
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    TEST_ASSERT_EQUAL_INT64(1757754000, page.events[0].occurred_at);
    TEST_ASSERT_EQUAL_STRING("cam-1", page.events[0].target_uuid);
    TEST_ASSERT_EQUAL_STRING("192.0.2.10", page.events[0].remote_address);
    cJSON *details = cJSON_Parse(page.events[0].details_json);
    TEST_ASSERT_TRUE(cJSON_IsObject(details));
    TEST_ASSERT_EQUAL_STRING("authorization.summary",
        cJSON_GetObjectItemCaseSensitive(details, "event_type")->valuestring);
    TEST_ASSERT_EQUAL_INT(5, cJSON_GetObjectItemCaseSensitive(details, "count")->valueint);
    TEST_ASSERT_EQUAL_INT(1757754000, (int)cJSON_GetObjectItemCaseSensitive(details, "first_at")->valuedouble);
    TEST_ASSERT_EQUAL_INT(1757754004, (int)cJSON_GetObjectItemCaseSensitive(details, "last_at")->valuedouble);
    TEST_ASSERT_EQUAL_INT(AUDIT_SUMMARY_WINDOW_SECONDS,
        cJSON_GetObjectItemCaseSensitive(details, "window_seconds")->valueint);
    TEST_ASSERT_EQUAL_STRING("policy_grant",
        cJSON_GetObjectItemCaseSensitive(details, "decision_source")->valuestring);
    cJSON_Delete(details);
    db_audit_page_free(&page);
}

void test_head_decision_is_summarized_like_get(void) {
    set_live_view_mode(AUDIT_DECISION_MODE_SUMMARIZE);
    allow_decision("HEAD", "/api/detection/results/cam", "192.0.2.10", "cam-1", "allowed");
    TEST_ASSERT_EQUAL_INT64(0, live_view_rows("allowed"));
    TEST_ASSERT_EQUAL_UINT(1, audit_log_flush_summaries(false));
    TEST_ASSERT_EQUAL_INT64(1, live_view_rows("allowed"));

    audit_query_t query = {.page = 1, .page_size = 1};
    safe_strcpy(query.action, "live.view", sizeof(query.action), 0);
    audit_page_t page;
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    TEST_ASSERT_EQUAL_INT(1, page.count);
    cJSON *details = cJSON_Parse(page.events[0].details_json);
    TEST_ASSERT_TRUE(cJSON_IsObject(details));
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetObjectItemCaseSensitive(details, "count")->valueint);
    TEST_ASSERT_EQUAL_STRING("HEAD",
        cJSON_GetObjectItemCaseSensitive(details, "method")->valuestring);
    cJSON_Delete(details);
    db_audit_page_free(&page);
}

/*
 * Guards item 1: AUDIT_SUMMARY_PATH_MAX must match http_request_t.path
 * (MAX_PATH_LENGTH, 512), not the smaller value audit rows also allow.
 * The path here is longer than the old 256-byte limit and shorter than 512,
 * so it fails only if the summary path buffer truncates it.
 */
void test_summary_row_records_full_detail_fields_and_long_path(void) {
    set_live_view_mode(AUDIT_DECISION_MODE_SUMMARIZE);
    char long_path[300];
    const char *prefix = "/api/detection/results/";
    memset(long_path, 'x', sizeof(long_path) - 1);
    long_path[sizeof(long_path) - 1] = '\0';
    memcpy(long_path, prefix, strlen(prefix));
    TEST_ASSERT_TRUE(strlen(long_path) > 256);
    TEST_ASSERT_TRUE(strlen(long_path) < MAX_PATH_LENGTH);

    allow_decision("GET", long_path, "192.0.2.10", "cam-1", "allowed");
    TEST_ASSERT_EQUAL_UINT(1, audit_log_flush_summaries(false));

    audit_query_t query = {.page = 1, .page_size = 1};
    safe_strcpy(query.action, "live.view", sizeof(query.action), 0);
    audit_page_t page;
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    TEST_ASSERT_EQUAL_INT(1, page.count);
    cJSON *details = cJSON_Parse(page.events[0].details_json);
    TEST_ASSERT_TRUE(cJSON_IsObject(details));
    TEST_ASSERT_EQUAL_STRING("GET",
        cJSON_GetObjectItemCaseSensitive(details, "method")->valuestring);
    TEST_ASSERT_EQUAL_STRING(long_path,
        cJSON_GetObjectItemCaseSensitive(details, "path")->valuestring);
    TEST_ASSERT_EQUAL_STRING("policy_grant",
        cJSON_GetObjectItemCaseSensitive(details, "decision_source")->valuestring);
    TEST_ASSERT_EQUAL_STRING("Allowed by Administrator role grant",
        cJSON_GetObjectItemCaseSensitive(details, "explanation")->valuestring);
    TEST_ASSERT_EQUAL_INT(AUDIT_SUMMARY_WINDOW_SECONDS,
        cJSON_GetObjectItemCaseSensitive(details, "window_seconds")->valueint);
    TEST_ASSERT_TRUE(cJSON_GetObjectItemCaseSensitive(details, "first_at")->valuedouble > 0);
    TEST_ASSERT_TRUE(cJSON_GetObjectItemCaseSensitive(details, "last_at")->valuedouble > 0);
    cJSON_Delete(details);
    db_audit_page_free(&page);
}

void test_summaries_split_by_client_and_camera_but_not_path(void) {
    set_live_view_mode(AUDIT_DECISION_MODE_SUMMARIZE);
    allow_decision("GET", "/api/detection/results/cam", "192.0.2.10", "cam-1", "allowed");
    allow_decision("GET", "/api/hls/cam/index.m3u8", "192.0.2.10", "cam-1", "allowed");
    allow_decision("GET", "/api/detection/results/cam", "100.64.0.9", "cam-1", "allowed");
    allow_decision("GET", "/api/detection/results/cam", "192.0.2.10", "cam-2", "allowed");
    TEST_ASSERT_EQUAL_UINT(3, audit_log_flush_summaries(false));
    TEST_ASSERT_EQUAL_INT64(3, live_view_rows("allowed"));
}

void test_summary_groups_credentials_and_preserves_first_sample(void) {
    set_live_view_mode(AUDIT_DECISION_MODE_SUMMARIZE);
    http_request_t req;
    decision_request(&req, "GET", "/api/streams", "192.0.2.10");
    user_t user = decision_user();
    user.authenticated_via_scoped_token = true;
    safe_strcpy(user.authentication_method, "scoped_token", sizeof(user.authentication_method), 0);
    safe_strcpy(user.api_token_uuid, "token-first", sizeof(user.api_token_uuid), 0);
    audit_log_authorization(&req, &user, AUTHZ_LIVE_VIEW, NULL, NULL, "allowed");
    safe_strcpy(user.api_token_uuid, "token-second", sizeof(user.api_token_uuid), 0);
    audit_log_authorization(&req, &user, AUTHZ_LIVE_VIEW, NULL, NULL, "allowed");
    user.authenticated_via_scoped_token = false;
    safe_strcpy(user.authentication_method, "session", sizeof(user.authentication_method), 0);
    audit_log_authorization(&req, &user, AUTHZ_LIVE_VIEW, NULL, NULL, "allowed");

    TEST_ASSERT_EQUAL_UINT(1, audit_log_flush_summaries(false));
    audit_query_t query = {.page = 1, .page_size = 10};
    audit_page_t page;
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    TEST_ASSERT_EQUAL_INT(1, page.count);
    TEST_ASSERT_EQUAL_STRING("scoped_token", page.events[0].auth_method);
    TEST_ASSERT_EQUAL_STRING("token-first", page.events[0].api_token_uuid);
    cJSON *details = cJSON_Parse(page.events[0].details_json);
    TEST_ASSERT_NOT_NULL(details);
    TEST_ASSERT_EQUAL_INT(3, cJSON_GetObjectItemCaseSensitive(details, "count")->valueint);
    cJSON_Delete(details);
    db_audit_page_free(&page);
}

/* Pause a request after it selects summarize but before it adds its entry. */
static pthread_mutex_t transition_test_mutex = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t transition_test_cond = PTHREAD_COND_INITIALIZER;
static bool clock_entered, release_clock, transition_started, transition_done;
static int transition_result;

static int64_t paused_summary_clock(void) {
    pthread_mutex_lock(&transition_test_mutex);
    if (!clock_entered) {
        clock_entered = true;
        pthread_cond_broadcast(&transition_test_cond);
        while (!release_clock) pthread_cond_wait(&transition_test_cond, &transition_test_mutex);
    }
    pthread_mutex_unlock(&transition_test_mutex);
    return fake_now;
}

static void *paused_decision_worker(void *unused) {
    (void)unused;
    allow_decision("GET", "/api/streams", "192.0.2.10", "cam-1", "allowed");
    return NULL;
}

static void *off_transition_worker(void *unused) {
    (void)unused;
    pthread_mutex_lock(&transition_test_mutex);
    transition_started = true;
    pthread_cond_broadcast(&transition_test_cond);
    pthread_mutex_unlock(&transition_test_mutex);
    audit_decision_mode_t modes[AUTHZ_ACTION_COUNT] = {0};
    modes[AUTHZ_LIVE_VIEW] = AUDIT_DECISION_MODE_OFF;
    transition_result = audit_log_set_decision_modes(modes);
    pthread_mutex_lock(&transition_test_mutex);
    transition_done = true;
    pthread_cond_broadcast(&transition_test_cond);
    pthread_mutex_unlock(&transition_test_mutex);
    return NULL;
}

void test_mode_transition_waits_for_inflight_summary(void) {
    set_live_view_mode(AUDIT_DECISION_MODE_SUMMARIZE);
    clock_entered = release_clock = transition_started = transition_done = false;
    audit_log_set_clock_for_testing(paused_summary_clock);
    pthread_t request_thread, transition_thread;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&request_thread, NULL, paused_decision_worker, NULL));
    pthread_mutex_lock(&transition_test_mutex);
    while (!clock_entered) pthread_cond_wait(&transition_test_cond, &transition_test_mutex);
    pthread_mutex_unlock(&transition_test_mutex);
    int create_result = pthread_create(&transition_thread, NULL, off_transition_worker, NULL);
    pthread_mutex_lock(&transition_test_mutex);
    if (create_result == 0) {
        while (!transition_started) pthread_cond_wait(&transition_test_cond, &transition_test_mutex);
        struct timespec deadline;
        clock_gettime(CLOCK_REALTIME, &deadline);
        deadline.tv_sec++;
        int wait_result = 0;
        while (!transition_done && wait_result == 0) {
            wait_result = pthread_cond_timedwait(&transition_test_cond, &transition_test_mutex, &deadline);
        }
    }
    bool completed_while_paused = transition_done;
    release_clock = true;
    pthread_cond_broadcast(&transition_test_cond);
    pthread_mutex_unlock(&transition_test_mutex);
    pthread_join(request_thread, NULL);
    if (create_result == 0) pthread_join(transition_thread, NULL);
    audit_log_set_clock_for_testing(fake_clock);

    TEST_ASSERT_EQUAL_INT(0, create_result);
    TEST_ASSERT_FALSE(completed_while_paused);
    TEST_ASSERT_EQUAL_INT(0, transition_result);
    TEST_ASSERT_EQUAL_UINT(0, audit_summary_pending());
    TEST_ASSERT_EQUAL_INT64(1, live_view_rows("allowed"));
    allow_decision("GET", "/api/streams", "192.0.2.10", "cam-1", "allowed");
    TEST_ASSERT_EQUAL_UINT(0, audit_log_flush_summaries(false));
    TEST_ASSERT_EQUAL_INT64(1, live_view_rows("allowed"));
}

void test_mutating_requests_are_recorded_even_when_summarized_or_off(void) {
    set_live_view_mode(AUDIT_DECISION_MODE_SUMMARIZE);
    allow_decision("POST", "/api/streams/cam/snapshot", "192.0.2.10", "cam-1", "allowed");
    TEST_ASSERT_EQUAL_INT64(1, live_view_rows("allowed"));
    set_live_view_mode(AUDIT_DECISION_MODE_OFF);
    allow_decision("DELETE", "/api/streams/cam", "192.0.2.10", "cam-1", "allowed");
    TEST_ASSERT_EQUAL_INT64(2, live_view_rows("allowed"));
}

void test_denied_and_error_are_never_summarized(void) {
    set_live_view_mode(AUDIT_DECISION_MODE_SUMMARIZE);
    allow_decision("GET", "/api/detection/results/cam", "192.0.2.10", "cam-1", "denied");
    allow_decision("GET", "/api/detection/results/cam", "192.0.2.10", "cam-1", "error");
    TEST_ASSERT_EQUAL_INT64(1, live_view_rows("denied"));
    TEST_ASSERT_EQUAL_INT64(1, live_view_rows("error"));
    TEST_ASSERT_EQUAL_UINT(0, audit_log_flush_summaries(false));
}

void test_closed_window_flush_keeps_current_window(void) {
    set_live_view_mode(AUDIT_DECISION_MODE_SUMMARIZE);
    fake_now = 1757754000;
    allow_decision("GET", "/api/detection/results/cam", "192.0.2.10", "cam-1", "allowed");
    fake_now = 1757754000 + AUDIT_SUMMARY_WINDOW_SECONDS;
    allow_decision("GET", "/api/detection/results/cam", "192.0.2.10", "cam-1", "allowed");
    TEST_ASSERT_EQUAL_UINT(1, audit_log_flush_summaries(true));
    TEST_ASSERT_EQUAL_INT64(1, live_view_rows("allowed"));
    TEST_ASSERT_EQUAL_UINT(1, audit_log_flush_summaries(false));
    TEST_ASSERT_EQUAL_INT64(2, live_view_rows("allowed"));
}

void test_backward_clock_flushes_instead_of_stranding(void) {
    set_live_view_mode(AUDIT_DECISION_MODE_SUMMARIZE);
    fake_now = 1757754000 + AUDIT_SUMMARY_WINDOW_SECONDS;
    allow_decision("GET", "/api/detection/results/cam", "192.0.2.10", "cam-1", "allowed");
    fake_now = 1757754000;
    TEST_ASSERT_EQUAL_UINT(1, audit_log_flush_summaries(true));
    TEST_ASSERT_EQUAL_INT64(1, live_view_rows("allowed"));
}

void test_full_table_flushes_early_without_losing_counts(void) {
    TEST_ASSERT_EQUAL_INT(0, audit_log_reset_summaries_for_testing(2));
    set_live_view_mode(AUDIT_DECISION_MODE_SUMMARIZE);
    allow_decision("GET", "/api/detection/results/cam", "192.0.2.1", "cam-1", "allowed");
    allow_decision("GET", "/api/detection/results/cam", "192.0.2.2", "cam-1", "allowed");
    TEST_ASSERT_EQUAL_INT64(0, live_view_rows("allowed"));
    allow_decision("GET", "/api/detection/results/cam", "192.0.2.3", "cam-1", "allowed");
    TEST_ASSERT_EQUAL_INT64(2, live_view_rows("allowed"));
    TEST_ASSERT_EQUAL_UINT(1, audit_log_flush_summaries(false));
    TEST_ASSERT_EQUAL_INT64(3, live_view_rows("allowed"));
}

void test_mode_change_flushes_pending_summaries(void) {
    set_live_view_mode(AUDIT_DECISION_MODE_SUMMARIZE);
    allow_decision("GET", "/api/detection/results/cam", "192.0.2.10", "cam-1", "allowed");
    allow_decision("GET", "/api/detection/results/cam", "192.0.2.10", "cam-1", "allowed");
    TEST_ASSERT_EQUAL_INT64(0, live_view_rows("allowed"));
    set_live_view_mode(AUDIT_DECISION_MODE_RECORD);
    TEST_ASSERT_EQUAL_INT64(1, live_view_rows("allowed"));
    TEST_ASSERT_EQUAL_INT(AUDIT_DECISION_MODE_RECORD,
                          audit_log_get_decision_mode(AUTHZ_LIVE_VIEW));
}

void test_failed_summary_write_is_dropped_without_crashing(void) {
    set_live_view_mode(AUDIT_DECISION_MODE_SUMMARIZE);
    /* A control character makes db_audit_append() reject the row. It must sit
     * between two printable characters: httpd_get_effective_client_ip()'s
     * non-IP-literal fallback trims leading/trailing non-printable bytes
     * (copy_trimmed_value(), isgraph()-based), so one at either end would be
     * silently stripped before reaching db_audit_append()'s validation. */
    allow_decision("GET", "/api/detection/results/cam", "192.0.2.1" "\x01" "0", "cam-1", "allowed");
    TEST_ASSERT_EQUAL_UINT(1, audit_log_flush_summaries(false));
    TEST_ASSERT_EQUAL_INT64(0, live_view_rows("allowed"));
    TEST_ASSERT_EQUAL_UINT(0, audit_log_flush_summaries(false));
}

static void settings_request(http_request_t *req, const char *method,
                             const char *body) {
    http_request_init(req);
    safe_strcpy(req->path, "/api/audit/settings", sizeof(req->path), 0);
    safe_strcpy(req->method_str, method, sizeof(req->method_str), 0);
    if (body) {
        /* http_request_t.body is void *; the handler only reads it. */
        req->body = (void *)body;
        req->body_len = strlen(body);
    }
}

static int64_t audit_rows_for_action(const char *action) {
    audit_query_t query = {.page = 1, .page_size = 1};
    safe_strcpy(query.action, action, sizeof(query.action), 0);
    audit_page_t page;
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    int64_t total = page.total;
    db_audit_page_free(&page);
    return total;
}

void test_audit_settings_get_lists_catalog_modes(void) {
    http_request_t req;
    settings_request(&req, "GET", NULL);
    http_response_t res;
    http_response_init(&res);
    handle_get_audit_settings(&req, &res);
    TEST_ASSERT_EQUAL_INT(200, res.status_code);

    cJSON *root = cJSON_Parse(res.body);
    TEST_ASSERT_TRUE(cJSON_IsObject(root));
    TEST_ASSERT_EQUAL_INT(AUDIT_SUMMARY_WINDOW_SECONDS,
        cJSON_GetObjectItemCaseSensitive(root, "summary_window_seconds")->valueint);
    cJSON *modes = cJSON_GetObjectItemCaseSensitive(root, "allowed_decision_modes");
    TEST_ASSERT_TRUE(cJSON_IsArray(modes));
    int catalog_count = 0;
    TEST_ASSERT_NOT_NULL(authorization_action_catalog(&catalog_count));
    TEST_ASSERT_EQUAL_INT(catalog_count, cJSON_GetArraySize(modes));
    cJSON *first = cJSON_GetArrayItem(modes, 0);
    TEST_ASSERT_EQUAL_STRING("live.view",
        cJSON_GetObjectItemCaseSensitive(first, "action")->valuestring);
    TEST_ASSERT_EQUAL_STRING("Live video",
        cJSON_GetObjectItemCaseSensitive(first, "category")->valuestring);
    TEST_ASSERT_EQUAL_STRING("record",
        cJSON_GetObjectItemCaseSensitive(first, "mode")->valuestring);
    cJSON_Delete(root);
    http_response_free(&res);
}

void test_audit_settings_put_modes_only_updates_modes(void) {
    http_request_t req;
    settings_request(&req, "PUT",
        "{\"allowed_decision_modes\":{\"live.view\":\"summarize\"}}");
    http_response_t res;
    http_response_init(&res);
    handle_put_audit_settings(&req, &res);
    TEST_ASSERT_EQUAL_INT(200, res.status_code);
    TEST_ASSERT_NULL(strstr(res.body, "pruned_events"));
    TEST_ASSERT_NOT_NULL(strstr(res.body, "\"retention_days\":365"));
    http_response_free(&res);

    TEST_ASSERT_EQUAL_INT(AUDIT_DECISION_MODE_SUMMARIZE,
                          audit_log_get_decision_mode(AUTHZ_LIVE_VIEW));
    audit_decision_mode_t stored[AUTHZ_ACTION_COUNT];
    TEST_ASSERT_EQUAL_INT(0, db_audit_load_decision_modes(stored));
    TEST_ASSERT_EQUAL_INT(AUDIT_DECISION_MODE_SUMMARIZE, stored[AUTHZ_LIVE_VIEW]);
    TEST_ASSERT_EQUAL_INT64(1, audit_rows_for_action("audit.settings.update"));

    audit_query_t query = {.page = 1, .page_size = 1};
    safe_strcpy(query.action, "audit.settings.update", sizeof(query.action), 0);
    audit_page_t page;
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    TEST_ASSERT_EQUAL_INT(1, page.count);
    cJSON *details = cJSON_Parse(page.events[0].details_json);
    TEST_ASSERT_TRUE(cJSON_IsObject(details));
    TEST_ASSERT_EQUAL_STRING("audit.decision_modes.update",
        cJSON_GetObjectItemCaseSensitive(details, "event_type")->valuestring);
    cJSON *changes = cJSON_GetObjectItemCaseSensitive(details, "changes");
    TEST_ASSERT_TRUE(cJSON_IsArray(changes));
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(changes));
    cJSON *change = cJSON_GetArrayItem(changes, 0);
    TEST_ASSERT_EQUAL_STRING("live.view",
        cJSON_GetObjectItemCaseSensitive(change, "action")->valuestring);
    TEST_ASSERT_EQUAL_STRING("record",
        cJSON_GetObjectItemCaseSensitive(change, "previous")->valuestring);
    TEST_ASSERT_EQUAL_STRING("summarize",
        cJSON_GetObjectItemCaseSensitive(change, "mode")->valuestring);
    cJSON_Delete(details);
    db_audit_page_free(&page);
}

static int allocation_fail_at, allocation_index;
static bool allocation_failed, failure_after_mode_publish;

static void *fail_one_json_allocation(size_t size) {
    if (allocation_index++ == allocation_fail_at) {
        allocation_failed = true;
        failure_after_mode_publish =
            audit_log_get_decision_mode(AUTHZ_LIVE_VIEW) != AUDIT_DECISION_MODE_RECORD;
        return NULL;
    }
    return malloc(size);
}

void test_audit_settings_allocation_failures_do_not_drop_mode_changes(void) {
    /* Sweep parsing, change objects and their fields, and persistence. Fail
     * only once so the handler can allocate its error response and clean up. */
    int pre_publish_failures = 0;
    for (int fail_at = 0; fail_at < 256; fail_at++) {
        TEST_ASSERT_EQUAL_INT(0, audit_log_reset_summaries_for_testing(0));
        TEST_ASSERT_EQUAL_INT(0, db_set_system_setting(AUDIT_DECISION_MODES_SETTING_KEY, "{}"));
        TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(get_db_handle(),
            "DELETE FROM audit_events;", NULL, NULL, NULL));
        http_request_t req;
        settings_request(&req, "PUT",
            "{\"allowed_decision_modes\":{\"live.view\":\"summarize\",\"system.admin\":\"off\"}}");
        http_response_t res;
        http_response_init(&res);
        allocation_fail_at = fail_at;
        allocation_index = 0;
        allocation_failed = failure_after_mode_publish = false;
        cJSON_Hooks hooks = {.malloc_fn = fail_one_json_allocation, .free_fn = free};
        cJSON_InitHooks(&hooks);
        handle_put_audit_settings(&req, &res);
        cJSON_InitHooks(NULL);
        if (!allocation_failed || failure_after_mode_publish) {
            http_response_free(&res);
            continue;
        }
        pre_publish_failures++;
        audit_decision_mode_t live = audit_log_get_decision_mode(AUTHZ_LIVE_VIEW);
        audit_decision_mode_t admin = audit_log_get_decision_mode(AUTHZ_SYSTEM_ADMIN);
        audit_decision_mode_t stored[AUTHZ_ACTION_COUNT];
        TEST_ASSERT_EQUAL_INT(0, db_audit_load_decision_modes(stored));
        TEST_ASSERT_EQUAL_INT(live, stored[AUTHZ_LIVE_VIEW]);
        TEST_ASSERT_EQUAL_INT(admin, stored[AUTHZ_SYSTEM_ADMIN]);
        if (res.status_code == 200) {
            TEST_ASSERT_EQUAL_INT(AUDIT_DECISION_MODE_SUMMARIZE, live);
            TEST_ASSERT_EQUAL_INT(AUDIT_DECISION_MODE_OFF, admin);
            audit_query_t query = {.page = 1, .page_size = 1};
            safe_strcpy(query.action, "audit.settings.update", sizeof(query.action), 0);
            audit_page_t page;
            TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
            TEST_ASSERT_EQUAL_INT(1, page.count);
            cJSON *details = cJSON_Parse(page.events[0].details_json);
            cJSON *changes = cJSON_GetObjectItemCaseSensitive(details, "changes");
            TEST_ASSERT_EQUAL_INT(2, cJSON_GetArraySize(changes));
            for (int i = 0; i < 2; i++) {
                cJSON *change = cJSON_GetArrayItem(changes, i);
                TEST_ASSERT_TRUE(cJSON_IsString(cJSON_GetObjectItemCaseSensitive(change, "action")));
                TEST_ASSERT_TRUE(cJSON_IsString(cJSON_GetObjectItemCaseSensitive(change, "previous")));
                TEST_ASSERT_TRUE(cJSON_IsString(cJSON_GetObjectItemCaseSensitive(change, "mode")));
            }
            cJSON_Delete(details);
            db_audit_page_free(&page);
        } else {
            TEST_ASSERT_TRUE(res.status_code == 400 || res.status_code == 500);
            TEST_ASSERT_EQUAL_INT(AUDIT_DECISION_MODE_RECORD, live);
            TEST_ASSERT_EQUAL_INT(AUDIT_DECISION_MODE_RECORD, admin);
            TEST_ASSERT_EQUAL_INT64(0, audit_rows_for_action("audit.settings.update"));
        }
        http_response_free(&res);
    }
    TEST_ASSERT_TRUE(pre_publish_failures > 20);
}

void test_audit_settings_put_rejects_invalid_body_atomically(void) {
    const char *bodies[] = {
        "{\"retention_days\":30,\"allowed_decision_modes\":{\"live.view\":\"summarize\",\"no.such.action\":\"off\"}}",
        "{\"retention_days\":30,\"allowed_decision_modes\":{\"live.view\":\"loud\"}}",
        "{\"retention_days\":0,\"allowed_decision_modes\":{\"live.view\":\"summarize\"}}",
        "{\"allowed_decision_modes\":[\"live.view\"]}",
        "{}",
    };
    for (size_t i = 0; i < sizeof(bodies) / sizeof(bodies[0]); i++) {
        http_request_t req;
        settings_request(&req, "PUT", bodies[i]);
        http_response_t res;
        http_response_init(&res);
        handle_put_audit_settings(&req, &res);
        TEST_ASSERT_EQUAL_INT(400, res.status_code);
        http_response_free(&res);
    }
    int retention_days = 0;
    TEST_ASSERT_EQUAL_INT(0, db_audit_get_retention_days(&retention_days));
    TEST_ASSERT_EQUAL_INT(365, retention_days);
    TEST_ASSERT_EQUAL_INT(AUDIT_DECISION_MODE_RECORD,
                          audit_log_get_decision_mode(AUTHZ_LIVE_VIEW));
    TEST_ASSERT_EQUAL_INT64(0, audit_rows_for_action("audit.settings.update"));
}

void test_audit_settings_put_unchanged_modes_writes_no_event(void) {
    http_request_t req;
    settings_request(&req, "PUT",
        "{\"allowed_decision_modes\":{\"live.view\":\"record\"}}");
    http_response_t res;
    http_response_init(&res);
    handle_put_audit_settings(&req, &res);
    TEST_ASSERT_EQUAL_INT(200, res.status_code);
    http_response_free(&res);
    TEST_ASSERT_EQUAL_INT64(0, audit_rows_for_action("audit.settings.update"));
}

void test_audit_settings_put_combined_retention_and_modes(void) {
    http_request_t req;
    settings_request(&req, "PUT",
        "{\"retention_days\":120,\"allowed_decision_modes\":{\"live.view\":\"off\"}}");
    http_response_t res;
    http_response_init(&res);
    handle_put_audit_settings(&req, &res);
    TEST_ASSERT_EQUAL_INT(200, res.status_code);

    cJSON *root = cJSON_Parse(res.body);
    TEST_ASSERT_TRUE(cJSON_IsObject(root));
    TEST_ASSERT_NOT_NULL(cJSON_GetObjectItemCaseSensitive(root, "pruned_events"));
    TEST_ASSERT_EQUAL_INT(120,
        cJSON_GetObjectItemCaseSensitive(root, "retention_days")->valueint);
    cJSON *modes = cJSON_GetObjectItemCaseSensitive(root, "allowed_decision_modes");
    TEST_ASSERT_TRUE(cJSON_IsArray(modes));
    bool found_off = false;
    for (int i = 0; i < cJSON_GetArraySize(modes); i++) {
        cJSON *item = cJSON_GetArrayItem(modes, i);
        if (strcmp(cJSON_GetObjectItemCaseSensitive(item, "action")->valuestring,
                   "live.view") == 0) {
            TEST_ASSERT_EQUAL_STRING("off",
                cJSON_GetObjectItemCaseSensitive(item, "mode")->valuestring);
            found_off = true;
        }
    }
    TEST_ASSERT_TRUE(found_off);
    cJSON_Delete(root);
    http_response_free(&res);

    int retention_days = 0;
    TEST_ASSERT_EQUAL_INT(0, db_audit_get_retention_days(&retention_days));
    TEST_ASSERT_EQUAL_INT(120, retention_days);
    TEST_ASSERT_EQUAL_INT(AUDIT_DECISION_MODE_OFF,
                          audit_log_get_decision_mode(AUTHZ_LIVE_VIEW));
    TEST_ASSERT_EQUAL_INT64(1, audit_rows_for_action("audit.retention.update"));
    TEST_ASSERT_EQUAL_INT64(1, audit_rows_for_action("audit.settings.update"));
}

void test_query_filters_by_details_event_type(void) {
    audit_event_input_t summary = event_input("request-summary", "live.view", "allowed");
    summary.details_json = "{\"event_type\":\"authorization.summary\",\"count\":4}";
    audit_event_input_t decision = event_input("request-decision", "live.view", "allowed");
    decision.details_json = "{\"event_type\":\"authorization.decision\"}";
    TEST_ASSERT_EQUAL_INT(0, db_audit_append(&summary, NULL));
    TEST_ASSERT_EQUAL_INT(0, db_audit_append(&decision, NULL));

    audit_query_t query = {.page = 1, .page_size = 20};
    safe_strcpy(query.event_type, "authorization.summary", sizeof(query.event_type), 0);
    audit_page_t page;
    TEST_ASSERT_EQUAL_INT(0, db_audit_query(&query, &page));
    TEST_ASSERT_EQUAL_INT64(1, page.total);
    TEST_ASSERT_EQUAL_STRING("request-summary", page.events[0].request_id);
    db_audit_page_free(&page);

    http_request_t req;
    http_request_init(&req);
    safe_strcpy(req.path, "/api/audit/events", sizeof(req.path), 0);
    safe_strcpy(req.method_str, "GET", sizeof(req.method_str), 0);
    safe_strcpy(req.query_string,
                "page=1&page_size=10&action=live.view&event_type=authorization.summary",
                sizeof(req.query_string), 0);
    http_response_t res;
    http_response_init(&res);
    handle_get_audit_events(&req, &res);
    TEST_ASSERT_EQUAL_INT(200, res.status_code);
    cJSON *list = cJSON_Parse(res.body);
    TEST_ASSERT_TRUE(cJSON_IsObject(list));
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetObjectItemCaseSensitive(list, "count")->valueint);
    cJSON_Delete(list);
    http_response_free(&res);
}

void test_decision_modes_init_loads_stored_modes_into_memory(void) {
    TEST_ASSERT_EQUAL_INT(0, db_set_system_setting(
        AUDIT_DECISION_MODES_SETTING_KEY,
        "{\"live.view\":\"summarize\",\"system.admin\":\"off\"}"));
    TEST_ASSERT_EQUAL_INT(AUDIT_DECISION_MODE_RECORD,
                          audit_log_get_decision_mode(AUTHZ_LIVE_VIEW));
    TEST_ASSERT_EQUAL_INT(0, audit_log_decision_modes_init());
    TEST_ASSERT_EQUAL_INT(AUDIT_DECISION_MODE_SUMMARIZE,
                          audit_log_get_decision_mode(AUTHZ_LIVE_VIEW));
    TEST_ASSERT_EQUAL_INT(AUDIT_DECISION_MODE_OFF,
                          audit_log_get_decision_mode(AUTHZ_SYSTEM_ADMIN));
    TEST_ASSERT_EQUAL_INT(AUDIT_DECISION_MODE_RECORD,
                          audit_log_get_decision_mode(AUTHZ_RECORDINGS_REPLAY));

    /* The summary table is live after init. */
    allow_decision("GET", "/api/detection/results/cam", "192.0.2.10", "cam-1", "allowed");
    TEST_ASSERT_EQUAL_INT64(0, live_view_rows("allowed"));
    TEST_ASSERT_EQUAL_UINT(1, audit_log_flush_summaries(false));
}

int main(void) {
    unlink(TEST_DB_PATH);
    init_logger();
    if (init_database(TEST_DB_PATH) != 0 || db_auth_init() != 0) {
        fprintf(stderr, "FATAL: failed to initialize audit test database\n");
        return 1;
    }
    user_t admin;
    if (db_auth_get_user_by_username("admin", &admin) != 0) {
        fprintf(stderr, "FATAL: failed to load test administrator\n");
        shutdown_database();
        return 1;
    }
    admin_user_id = admin.id;

    UNITY_BEGIN();
    RUN_TEST(test_append_and_query_round_trip);
    RUN_TEST(test_append_rejects_invalid_or_non_object_payloads);
    RUN_TEST(test_query_paginates_newest_first);
    RUN_TEST(test_retention_prunes_expired_events);
    RUN_TEST(test_retention_drains_large_backlog_without_touching_live_events);
    RUN_TEST(test_prune_stops_on_budget_and_resumes_until_drained);
    RUN_TEST(test_automatic_prune_on_append_follows_backlog_cadence);
    RUN_TEST(test_prune_batch_query_uses_the_retention_index);
    RUN_TEST(test_web_helper_redacts_sensitive_detail_fields);
    RUN_TEST(test_operation_helper_adds_standard_envelope_and_redacts_context);
    RUN_TEST(test_camera_configuration_route_outcomes_cover_success_failure_and_error);
    RUN_TEST(test_system_mutation_route_outcomes_are_audited);
    RUN_TEST(test_audit_http_settings_list_and_csv_export);
    RUN_TEST(test_login_success_and_denial_are_audited_without_credentials);
    RUN_TEST(test_audit_api_denies_and_records_unauthenticated_access);
    RUN_TEST(test_decision_modes_default_to_record_when_unset);
    RUN_TEST(test_decision_modes_round_trip_and_store_only_non_defaults);
    RUN_TEST(test_decision_modes_ignore_invalid_stored_entries);
    RUN_TEST(test_record_mode_writes_one_row_per_allowed_read);
    RUN_TEST(test_off_mode_skips_allowed_reads_but_records_denials);
    RUN_TEST(test_summarize_buffers_until_flush_then_writes_one_counted_row);
    RUN_TEST(test_head_decision_is_summarized_like_get);
    RUN_TEST(test_summary_row_records_full_detail_fields_and_long_path);
    RUN_TEST(test_summaries_split_by_client_and_camera_but_not_path);
    RUN_TEST(test_summary_groups_credentials_and_preserves_first_sample);
    RUN_TEST(test_mode_transition_waits_for_inflight_summary);
    RUN_TEST(test_mutating_requests_are_recorded_even_when_summarized_or_off);
    RUN_TEST(test_denied_and_error_are_never_summarized);
    RUN_TEST(test_closed_window_flush_keeps_current_window);
    RUN_TEST(test_backward_clock_flushes_instead_of_stranding);
    RUN_TEST(test_full_table_flushes_early_without_losing_counts);
    RUN_TEST(test_mode_change_flushes_pending_summaries);
    RUN_TEST(test_failed_summary_write_is_dropped_without_crashing);
    RUN_TEST(test_audit_settings_get_lists_catalog_modes);
    RUN_TEST(test_audit_settings_put_modes_only_updates_modes);
    RUN_TEST(test_audit_settings_put_rejects_invalid_body_atomically);
    RUN_TEST(test_audit_settings_allocation_failures_do_not_drop_mode_changes);
    RUN_TEST(test_audit_settings_put_unchanged_modes_writes_no_event);
    RUN_TEST(test_audit_settings_put_combined_retention_and_modes);
    RUN_TEST(test_query_filters_by_details_event_type);
    RUN_TEST(test_decision_modes_init_loads_stored_modes_into_memory);
    int result = UNITY_END();

    shutdown_database();
    shutdown_logger();
    unlink(TEST_DB_PATH);
    return result;
}
