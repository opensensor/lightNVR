#define _GNU_SOURCE

#include <cjson/cJSON.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "core/authorization.h"
#include "database/db_audit.h"
#include "utils/strings.h"
#include "web/api_handlers_audit.h"
#include "web/audit_log.h"
#include "web/httpd_utils.h"

static bool authorize_audit_admin(const http_request_t *req,
                                  http_response_t *res, user_t *user) {
    authorization_evaluation_t evaluation;
    return httpd_authorize_action(req, res, AUTHZ_SYSTEM_ADMIN, NULL, user,
                                  &evaluation) != 0;
}

static bool parse_int64_query(const http_request_t *req, const char *name,
                              int64_t default_value, int64_t minimum,
                              int64_t maximum, int64_t *output,
                              http_response_t *res) {
    char value[64];
    if (http_request_get_query_param(req, name, value, sizeof(value)) < 0) {
        *output = default_value;
        return true;
    }
    char *end = NULL;
    errno = 0;
    long long parsed = strtoll(value, &end, 10);
    if (errno == ERANGE || !end || *end != '\0' ||
        parsed < minimum || parsed > maximum) {
        http_response_set_json_error(res, 400, "Invalid audit query parameter");
        return false;
    }
    *output = (int64_t)parsed;
    return true;
}

static void read_text_query(const http_request_t *req, const char *name,
                            char *output, size_t output_size) {
    output[0] = '\0';
    char value[1024];
    if (http_request_get_query_param(req, name, value, sizeof(value)) >= 0) {
        copy_trimmed_value(output, output_size, value, 0);
    }
}

static bool parse_audit_query(const http_request_t *req, audit_query_t *query,
                              http_response_t *res) {
    memset(query, 0, sizeof(*query));
    int64_t page = 1;
    int64_t page_size = 100;
    if (!parse_int64_query(req, "page", 1, 1, 1000000, &page, res) ||
        !parse_int64_query(req, "page_size", 100, 1,
                           AUDIT_PAGE_SIZE_MAX, &page_size, res) ||
        !parse_int64_query(req, "since", 0, 0, INT64_MAX,
                           &query->since, res) ||
        !parse_int64_query(req, "until", 0, 0, INT64_MAX,
                           &query->until, res) ||
        !parse_int64_query(req, "principal_user_id", 0, 0, INT64_MAX,
                           &query->principal_user_id, res)) {
        return false;
    }
    if (query->since > 0 && query->until > 0 && query->since > query->until) {
        http_response_set_json_error(res, 400,
                                     "Audit since must not exceed until");
        return false;
    }
    query->page = (int)page;
    query->page_size = (int)page_size;
    read_text_query(req, "action", query->action, sizeof(query->action));
    read_text_query(req, "outcome", query->outcome, sizeof(query->outcome));
    read_text_query(req, "target_uuid", query->target_uuid,
                    sizeof(query->target_uuid));
    read_text_query(req, "request_id", query->request_id,
                    sizeof(query->request_id));
    return true;
}

static cJSON *audit_event_to_json(const audit_event_t *event) {
    cJSON *item = cJSON_CreateObject();
    if (!item) return NULL;
    cJSON_AddStringToObject(item, "uuid", event->uuid);
    cJSON_AddNumberToObject(item, "occurred_at", (double)event->occurred_at);
    cJSON_AddStringToObject(item, "request_id", event->request_id);
    if (event->principal_user_id > 0) {
        cJSON_AddNumberToObject(item, "principal_user_id",
                                (double)event->principal_user_id);
    } else {
        cJSON_AddNullToObject(item, "principal_user_id");
    }
    cJSON_AddStringToObject(item, "principal_username",
                            event->principal_username);
    cJSON_AddStringToObject(item, "auth_method", event->auth_method);
    if (event->api_token_uuid[0]) {
        cJSON_AddStringToObject(item, "api_token_uuid",
                                event->api_token_uuid);
    } else {
        cJSON_AddNullToObject(item, "api_token_uuid");
    }
    cJSON_AddStringToObject(item, "action", event->action);
    if (event->target_type[0]) {
        cJSON_AddStringToObject(item, "target_type", event->target_type);
    } else {
        cJSON_AddNullToObject(item, "target_type");
    }
    if (event->target_uuid[0]) {
        cJSON_AddStringToObject(item, "target_uuid", event->target_uuid);
    } else {
        cJSON_AddNullToObject(item, "target_uuid");
    }
    cJSON_AddStringToObject(item, "outcome", event->outcome);
    cJSON_AddStringToObject(item, "remote_address", event->remote_address);
    cJSON *details = cJSON_Parse(event->details_json);
    if (cJSON_IsObject(details)) {
        cJSON_AddItemToObject(item, "details", details);
    } else {
        cJSON_Delete(details);
        cJSON_AddNullToObject(item, "details");
    }
    return item;
}

static void set_page_json(http_response_t *res, const audit_page_t *page) {
    cJSON *root = cJSON_CreateObject();
    cJSON *events = cJSON_CreateArray();
    if (!root || !events) {
        cJSON_Delete(root);
        cJSON_Delete(events);
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    cJSON_AddNumberToObject(root, "page", page->page);
    cJSON_AddNumberToObject(root, "page_size", page->page_size);
    cJSON_AddNumberToObject(root, "count", page->count);
    cJSON_AddNumberToObject(root, "total", (double)page->total);
    cJSON_AddItemToObject(root, "events", events);
    for (int i = 0; i < page->count; i++) {
        cJSON *item = audit_event_to_json(&page->events[i]);
        if (!item) {
            cJSON_Delete(root);
            http_response_set_json_error(res, 500,
                                         "Failed to create response");
            return;
        }
        cJSON_AddItemToArray(events, item);
    }
    char *body = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if (!body) {
        http_response_set_json_error(res, 500, "Failed to serialize response");
        return;
    }
    http_response_set_json(res, 200, body);
    free(body);
}

void handle_get_audit_events(const http_request_t *req, http_response_t *res) {
    user_t user;
    if (!authorize_audit_admin(req, res, &user)) return;
    audit_query_t query;
    if (!parse_audit_query(req, &query, res)) return;
    audit_page_t page;
    if (db_audit_query(&query, &page) != 0) {
        http_response_set_json_error(res, 500, "Failed to query audit events");
        return;
    }
    set_page_json(res, &page);
    db_audit_page_free(&page);
}

static void csv_cell(FILE *stream, const char *value) {
    fputc('"', stream);
    if (value && strchr("=+-@", value[0])) {
        /* Keep spreadsheet applications from interpreting exported cells. */
        fputc('\'', stream);
    }
    for (const char *cursor = value ? value : ""; *cursor; cursor++) {
        if (*cursor == '"') fputc('"', stream);
        fputc(*cursor, stream);
    }
    fputc('"', stream);
}

void handle_get_audit_export(const http_request_t *req, http_response_t *res) {
    user_t user;
    if (!authorize_audit_admin(req, res, &user)) return;
    audit_query_t query;
    if (!parse_audit_query(req, &query, res)) return;
    audit_page_t page;
    if (db_audit_query(&query, &page) != 0) {
        http_response_set_json_error(res, 500, "Failed to query audit events");
        return;
    }
    char *body = NULL;
    size_t body_size = 0;
    FILE *stream = open_memstream(&body, &body_size);
    if (!stream) {
        db_audit_page_free(&page);
        http_response_set_json_error(res, 500, "Failed to create audit export");
        return;
    }
    fputs("uuid,occurred_at,request_id,principal_user_id,principal_username,"
          "auth_method,api_token_uuid,action,target_type,target_uuid,outcome,"
          "remote_address,details_json\n", stream);
    for (int i = 0; i < page.count; i++) {
        const audit_event_t *event = &page.events[i];
        char occurred_at[32];
        char principal_user_id[32];
        snprintf(occurred_at, sizeof(occurred_at), "%lld",
                 (long long)event->occurred_at);
        if (event->principal_user_id > 0) {
            snprintf(principal_user_id, sizeof(principal_user_id), "%lld",
                     (long long)event->principal_user_id);
        } else {
            principal_user_id[0] = '\0';
        }
        const char *cells[] = {
            event->uuid, occurred_at, event->request_id, principal_user_id,
            event->principal_username, event->auth_method,
            event->api_token_uuid, event->action, event->target_type,
            event->target_uuid, event->outcome, event->remote_address,
            event->details_json,
        };
        for (size_t column = 0; column < sizeof(cells) / sizeof(cells[0]);
             column++) {
            if (column > 0) fputc(',', stream);
            csv_cell(stream, cells[column]);
        }
        fputc('\n', stream);
    }
    if (fclose(stream) != 0 || !body) {
        free(body);
        db_audit_page_free(&page);
        http_response_set_json_error(res, 500, "Failed to create audit export");
        return;
    }
    if (http_response_set_body(res, body) != 0) {
        free(body);
        db_audit_page_free(&page);
        http_response_set_json_error(res, 500,
                                     "Failed to create audit export response");
        return;
    }
    res->status_code = 200;
    safe_strcpy(res->content_type, "text/csv; charset=utf-8",
                sizeof(res->content_type), 0);
    http_response_add_header(res, "Content-Disposition",
                             "attachment; filename=lightnvr-audit.csv");
    char total[32];
    snprintf(total, sizeof(total), "%lld", (long long)page.total);
    http_response_add_header(res, "X-Total-Count", total);
    http_response_add_cors_headers(res);
    free(body);
    db_audit_page_free(&page);
}

void handle_get_audit_settings(const http_request_t *req,
                               http_response_t *res) {
    user_t user;
    if (!authorize_audit_admin(req, res, &user)) return;
    int retention_days = 0;
    if (db_audit_get_retention_days(&retention_days) != 0) {
        http_response_set_json_error(res, 500,
                                     "Failed to load audit settings");
        return;
    }
    char body[96];
    snprintf(body, sizeof(body), "{\"retention_days\":%d}", retention_days);
    http_response_set_json(res, 200, body);
}

void handle_put_audit_settings(const http_request_t *req,
                               http_response_t *res) {
    user_t user;
    if (!authorize_audit_admin(req, res, &user)) return;
    cJSON *body = httpd_parse_json_body(req);
    cJSON *retention = cJSON_IsObject(body)
        ? cJSON_GetObjectItemCaseSensitive(body, "retention_days") : NULL;
    double number = cJSON_IsNumber(retention) ? retention->valuedouble : 0;
    int retention_days = (int)number;
    if (!cJSON_IsNumber(retention) || number != retention_days ||
        retention_days < 1 || retention_days > AUDIT_RETENTION_MAX_DAYS) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400,
                                     "retention_days must be 1-3650");
        return;
    }
    int previous_days = AUDIT_RETENTION_DEFAULT_DAYS;
    db_audit_get_retention_days(&previous_days);
    if (db_audit_set_retention_days(retention_days) != 0) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 500,
                                     "Failed to save audit settings");
        return;
    }
    int deleted_count = 0;
    if (db_audit_prune(&deleted_count) != 0) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 500,
                                     "Audit setting saved but pruning failed");
        return;
    }
    cJSON *details = cJSON_CreateObject();
    if (details) {
        cJSON_AddStringToObject(details, "event_type",
                                "audit.retention_update");
        cJSON_AddNumberToObject(details, "previous_days", previous_days);
        cJSON_AddNumberToObject(details, "retention_days", retention_days);
        cJSON_AddNumberToObject(details, "pruned_events", deleted_count);
    }
    audit_log_append(req, &user, "audit.retention.update", "audit_log", NULL,
                     "success", details);
    cJSON_Delete(details);
    cJSON_Delete(body);
    char response_body[128];
    snprintf(response_body, sizeof(response_body),
             "{\"retention_days\":%d,\"pruned_events\":%d}",
             retention_days, deleted_count);
    http_response_set_json(res, 200, response_body);
}
