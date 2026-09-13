/**
 * @file test_audit_log.c
 * @brief Durable audit storage, redaction, retention, and HTTP contract tests.
 */

#define _POSIX_C_SOURCE 200809L

#include <cjson/cJSON.h>
#include <stdio.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

#include "core/config.h"
#include "core/logger.h"
#include "database/db_audit.h"
#include "database/db_auth.h"
#include "database/db_core.h"
#include "database/db_streams.h"
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
    g_config.web_auth_enabled = false;
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
    int result = UNITY_END();

    shutdown_database();
    shutdown_logger();
    unlink(TEST_DB_PATH);
    return result;
}
