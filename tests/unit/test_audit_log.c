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
#include "unity.h"
#include "utils/strings.h"
#include "web/api_handlers_audit.h"
#include "web/api_handlers.h"
#include "web/audit_log.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_audit_log_test.db"

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

void tearDown(void) {}

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
    TEST_ASSERT_NOT_NULL(strstr(page.events[0].details_json,
                                "\"password\":\"[REDACTED]\""));
    TEST_ASSERT_NOT_NULL(strstr(page.events[0].details_json,
                                "\"api_token_uuid\":\"safe-id\""));
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
    RUN_TEST(test_web_helper_redacts_sensitive_detail_fields);
    RUN_TEST(test_audit_http_settings_list_and_csv_export);
    RUN_TEST(test_login_success_and_denial_are_audited_without_credentials);
    RUN_TEST(test_audit_api_denies_and_records_unauthenticated_access);
    int result = UNITY_END();

    shutdown_database();
    shutdown_logger();
    unlink(TEST_DB_PATH);
    return result;
}
