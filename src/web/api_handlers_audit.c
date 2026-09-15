#define _GNU_SOURCE

#include <cjson/cJSON.h>
#include <errno.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "core/authorization.h"
#include "database/db_audit.h"
#include "utils/strings.h"
#include "web/api_handlers_audit.h"
#include "web/audit_log.h"
#include "web/audit_summary.h"
#include "web/httpd_utils.h"

/* Serializes handle_put_audit_settings() from reading the previous decision
 * modes through persisting/swapping them and building the response, so two
 * concurrent PUTs cannot race on the previous->next diff or observe a
 * response snapshot from a request other than their own. Nothing else takes
 * this mutex; it is always acquired before the summary mutex (audit_summary.c)
 * and the database mutex, never after. */
static pthread_mutex_t audit_settings_put_mutex = PTHREAD_MUTEX_INITIALIZER;

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
    read_text_query(req, "event_type", query->event_type,
                    sizeof(query->event_type));
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
    if (value) {
        const char *cursor = value;
        while (*cursor &&
               ((unsigned char)*cursor <= 0x20 || (unsigned char)*cursor == 0x7f)) {
            cursor++;
        }
        if (*cursor && strchr("=+-@", *cursor)) {
            /* Keep spreadsheet applications from interpreting exported cells. */
            fputc('\'', stream);
        }
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

static void respond_audit_settings(http_response_t *res, int retention_days,
                                   int pruned_events, bool include_pruned) {
    cJSON *root = cJSON_CreateObject();
    cJSON *modes = root ? cJSON_AddArrayToObject(root, "allowed_decision_modes") : NULL;
    int count = 0;
    const authorization_action_metadata_t *catalog = authorization_action_catalog(&count);
    if (!root || !modes || !catalog) {
        cJSON_Delete(root);
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    cJSON_AddNumberToObject(root, "retention_days", retention_days);
    cJSON_AddNumberToObject(root, "summary_window_seconds", AUDIT_SUMMARY_WINDOW_SECONDS);
    if (include_pruned) cJSON_AddNumberToObject(root, "pruned_events", pruned_events);
    for (int i = 0; i < count; i++) {
        cJSON *item = cJSON_CreateObject();
        if (!item) continue;
        cJSON_AddStringToObject(item, "action", catalog[i].key);
        cJSON_AddStringToObject(item, "category", catalog[i].category);
        cJSON_AddStringToObject(item, "description", catalog[i].description);
        cJSON_AddStringToObject(item, "mode", audit_decision_mode_name(
            audit_log_get_decision_mode(catalog[i].action)));
        cJSON_AddItemToArray(modes, item);
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

void handle_get_audit_settings(const http_request_t *req,
                               http_response_t *res) {
    user_t user;
    if (!authorize_audit_admin(req, res, &user)) return;
    int retention_days = 0;
    if (db_audit_get_retention_days(&retention_days) != 0) {
        http_response_set_json_error(res, 500, "Failed to load audit settings");
        return;
    }
    respond_audit_settings(res, retention_days, 0, false);
}

void handle_put_audit_settings(const http_request_t *req,
                               http_response_t *res) {
    user_t user;
    if (!authorize_audit_admin(req, res, &user)) return;
    cJSON *body = httpd_parse_json_body(req);
    if (!cJSON_IsObject(body)) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400, "Audit settings body must be a JSON object");
        return;
    }
    const cJSON *retention = cJSON_GetObjectItemCaseSensitive(body, "retention_days");
    const cJSON *modes_json = cJSON_GetObjectItemCaseSensitive(body, "allowed_decision_modes");
    if (!retention && !modes_json) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400, "Provide retention_days, allowed_decision_modes, or both");
        return;
    }

    /* Validate everything before saving anything. */
    int retention_days = 0;
    if (retention) {
        double number = cJSON_IsNumber(retention) ? retention->valuedouble : 0;
        retention_days = (int)number;
        if (!cJSON_IsNumber(retention) || number != retention_days ||
            retention_days < 1 || retention_days > AUDIT_RETENTION_MAX_DAYS) {
            cJSON_Delete(body);
            http_response_set_json_error(res, 400, "retention_days must be 1-3650");
            return;
        }
    }
    /* Everything from here through the response below is serialized: see the
     * comment on audit_settings_put_mutex above. */
    pthread_mutex_lock(&audit_settings_put_mutex);

    cJSON *changes = NULL;
    int deleted_count = 0;

    audit_decision_mode_t previous[AUTHZ_ACTION_COUNT];
    audit_decision_mode_t next[AUTHZ_ACTION_COUNT];
    for (int i = 0; i < AUTHZ_ACTION_COUNT; i++) {
        previous[i] = next[i] = audit_log_get_decision_mode((authorization_action_t)i);
    }
    if (modes_json) {
        if (!cJSON_IsObject(modes_json)) {
            cJSON_Delete(body);
            http_response_set_json_error(res, 400, "allowed_decision_modes must be an object");
            goto unlock_and_return;
        }
        for (const cJSON *item = modes_json->child; item; item = item->next) {
            char message[160];
            authorization_action_t action = authorization_action_from_key(item->string);
            if (action == AUTHZ_ACTION_INVALID) {
                snprintf(message, sizeof(message), "Unknown audit action: %s",
                         item->string ? item->string : "");
                cJSON_Delete(body);
                http_response_set_json_error(res, 400, message);
                goto unlock_and_return;
            }
            audit_decision_mode_t mode;
            if (!cJSON_IsString(item) ||
                audit_decision_mode_from_name(item->valuestring, &mode) != 0) {
                snprintf(message, sizeof(message), "Invalid audit decision mode for %s",
                         item->string);
                cJSON_Delete(body);
                http_response_set_json_error(res, 400, message);
                goto unlock_and_return;
            }
            next[action] = mode;
        }
    }

    if (retention) {
        int previous_days = AUDIT_RETENTION_DEFAULT_DAYS;
        if (db_audit_get_retention_days(&previous_days) != 0) {
            cJSON_Delete(body);
            http_response_set_json_error(res, 500, "Failed to load audit settings");
            goto unlock_and_return;
        }
        if (db_audit_set_retention_days(retention_days) != 0) {
            cJSON_Delete(body);
            http_response_set_json_error(res, 500, "Failed to save audit settings");
            goto unlock_and_return;
        }
        if (db_audit_prune(&deleted_count) != 0) {
            cJSON_Delete(body);
            http_response_set_json_error(res, 500, "Audit setting saved but pruning failed");
            goto unlock_and_return;
        }
        cJSON *details = cJSON_CreateObject();
        if (details) {
            cJSON_AddStringToObject(details, "event_type", "audit.retention_update");
            cJSON_AddNumberToObject(details, "previous_days", previous_days);
            cJSON_AddNumberToObject(details, "retention_days", retention_days);
            cJSON_AddNumberToObject(details, "pruned_events", deleted_count);
        }
        audit_log_append(req, &user, "audit.retention.update", "audit_log", NULL,
                         "success", details);
        cJSON_Delete(details);
    } else if (db_audit_get_retention_days(&retention_days) != 0) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 500, "Failed to load audit settings");
        goto unlock_and_return;
    }

    changes = cJSON_CreateArray();
    int count = 0;
    const authorization_action_metadata_t *catalog = authorization_action_catalog(&count);
    if (!changes) {
        /* Allocation failure must not silently drop a validated mode change:
         * without the array we cannot build the audit event, so fail hard
         * rather than fall through to a 200 that never persisted it. */
        for (int i = 0; catalog && i < count; i++) {
            if (previous[catalog[i].action] != next[catalog[i].action]) {
                cJSON_Delete(body);
                http_response_set_json_error(res, 500, "Failed to create response");
                goto unlock_and_return;
            }
        }
    }
    for (int i = 0; changes && catalog && i < count; i++) {
        authorization_action_t action = catalog[i].action;
        if (previous[action] == next[action]) continue;
        cJSON *change = cJSON_CreateObject();
        if (!change ||
            !cJSON_AddStringToObject(change, "action", catalog[i].key) ||
            !cJSON_AddStringToObject(change, "previous", audit_decision_mode_name(previous[action])) ||
            !cJSON_AddStringToObject(change, "mode", audit_decision_mode_name(next[action])) ||
            !cJSON_AddItemToArray(changes, change)) {
            cJSON_Delete(change);
            cJSON_Delete(changes);
            changes = NULL;
            cJSON_Delete(body);
            http_response_set_json_error(res, 500, "Failed to create response");
            goto unlock_and_return;
        }
    }
    if (changes && cJSON_GetArraySize(changes) > 0) {
        /* Build the audit event before publishing the modes: an allocation
         * failure here must not leave the new modes live with an event that
         * omits the changes that produced them. */
        cJSON *details = cJSON_CreateObject();
        if (!details ||
            !cJSON_AddStringToObject(details, "event_type", "audit.decision_modes.update") ||
            !cJSON_AddItemToObject(details, "changes", changes)) {
            cJSON_Delete(details);
            cJSON_Delete(changes);
            changes = NULL;
            cJSON_Delete(body);
            http_response_set_json_error(res, 500, "Failed to create response");
            goto unlock_and_return;
        }
        changes = NULL; /* details owns the array now */
        if (audit_log_set_decision_modes(next) != 0) {
            cJSON_Delete(details);
            cJSON_Delete(body);
            http_response_set_json_error(res, 500, "Failed to save audit decision modes");
            goto unlock_and_return;
        }
        audit_log_append(req, &user, "audit.settings.update", "audit_log", NULL,
                         "success", details);
        cJSON_Delete(details);
    }
    cJSON_Delete(changes);
    changes = NULL;
    cJSON_Delete(body);
    body = NULL;
    respond_audit_settings(res, retention_days, deleted_count, retention != NULL);

unlock_and_return:
    pthread_mutex_unlock(&audit_settings_put_mutex);
}
