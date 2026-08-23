#define _POSIX_C_SOURCE 200809L

#include <cjson/cJSON.h>
#include <ctype.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#define LOG_COMPONENT "AuditDB"
#include "core/logger.h"
#include "database/db_audit.h"
#include "database/db_core.h"
#include "utils/strings.h"

static int64_t last_automatic_prune_at = 0;

static bool valid_outcome(const char *outcome) {
    return outcome &&
        (strcmp(outcome, "allowed") == 0 ||
         strcmp(outcome, "denied") == 0 ||
         strcmp(outcome, "success") == 0 ||
         strcmp(outcome, "failure") == 0 ||
         strcmp(outcome, "error") == 0);
}

static bool valid_safe_text(const char *value, size_t maximum,
                            bool required) {
    if (!value || value[0] == '\0') return !required;
    size_t length = strlen(value);
    if (length >= maximum) return false;
    for (const unsigned char *cursor = (const unsigned char *)value;
         *cursor; cursor++) {
        if (iscntrl(*cursor)) return false;
    }
    return true;
}

static bool valid_details(const char *details) {
    if (!details || details[0] == '\0' || strlen(details) >= AUDIT_DETAILS_MAX) {
        return false;
    }
    cJSON *parsed = cJSON_Parse(details);
    bool valid = cJSON_IsObject(parsed);
    cJSON_Delete(parsed);
    return valid;
}

static int retention_days_locked(sqlite3 *db) {
    sqlite3_stmt *stmt = NULL;
    int days = AUDIT_RETENTION_DEFAULT_DAYS;
    int rc = sqlite3_prepare_v2(
        db, "SELECT value FROM system_settings "
            "WHERE key='audit_retention_days' LIMIT 1;", -1, &stmt, NULL);
    if (rc == SQLITE_OK && sqlite3_step(stmt) == SQLITE_ROW) {
        const char *value = (const char *)sqlite3_column_text(stmt, 0);
        char *end = NULL;
        long parsed = value ? strtol(value, &end, 10) : 0;
        if (end && *end == '\0' && parsed >= 1 &&
            parsed <= AUDIT_RETENTION_MAX_DAYS) {
            days = (int)parsed;
        }
    }
    if (stmt) sqlite3_finalize(stmt);
    return days;
}

static int prune_locked(sqlite3 *db, int *deleted_count) {
    int retention_days = retention_days_locked(db);
    int64_t cutoff = (int64_t)time(NULL) -
        (int64_t)retention_days * 24 * 60 * 60;
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db, "DELETE FROM audit_events WHERE occurred_at < ?;", -1,
        &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, cutoff);
        rc = sqlite3_step(stmt);
    }
    if (deleted_count) {
        *deleted_count = rc == SQLITE_DONE ? sqlite3_changes(db) : 0;
    }
    if (stmt) sqlite3_finalize(stmt);
    return rc == SQLITE_DONE ? 0 : -1;
}

int db_audit_append(const audit_event_input_t *input,
                    char event_uuid[AUDIT_EVENT_UUID_MAX]) {
    if (event_uuid) event_uuid[0] = '\0';
    if (!input ||
        !valid_safe_text(input->request_id, AUDIT_REQUEST_ID_MAX, true) ||
        !valid_safe_text(input->principal_username, AUDIT_USERNAME_MAX, false) ||
        !valid_safe_text(input->auth_method, AUDIT_AUTH_METHOD_MAX, true) ||
        !valid_safe_text(input->api_token_uuid, AUDIT_EVENT_UUID_MAX, false) ||
        !valid_safe_text(input->action, AUDIT_ACTION_MAX, true) ||
        !valid_safe_text(input->target_type, AUDIT_TARGET_TYPE_MAX, false) ||
        !valid_safe_text(input->target_uuid, AUDIT_QUERY_VALUE_MAX, false) ||
        !valid_outcome(input->outcome) ||
        !valid_safe_text(input->remote_address, AUDIT_REMOTE_ADDRESS_MAX, false) ||
        !valid_details(input->details_json)) {
        return -1;
    }

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    const char *sql =
        "INSERT INTO audit_events "
        "(uuid,occurred_at,request_id,principal_user_id,principal_username,"
        "auth_method,api_token_uuid,action,target_type,target_uuid,outcome,"
        "remote_address,details_json) VALUES ("
        "lower(hex(randomblob(4))||'-'||hex(randomblob(2))||'-4'||"
        "substr(hex(randomblob(2)),2)||'-'||"
        "substr('89ab',(abs(random())%4)+1,1)||substr(hex(randomblob(2)),2)||"
        "'-'||hex(randomblob(6))),?,?,?,?,?,?,?,?,?,?,?,?);";
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    int64_t occurred_at = input->occurred_at > 0
        ? input->occurred_at : (int64_t)time(NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, occurred_at);
        sqlite3_bind_text(stmt, 2, input->request_id, -1, SQLITE_TRANSIENT);
        if (input->principal_user_id > 0) {
            sqlite3_bind_int64(stmt, 3, input->principal_user_id);
        } else {
            sqlite3_bind_null(stmt, 3);
        }
        sqlite3_bind_text(stmt, 4,
                          input->principal_username ? input->principal_username : "",
                          -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 5, input->auth_method, -1, SQLITE_TRANSIENT);
        if (input->api_token_uuid && input->api_token_uuid[0]) {
            sqlite3_bind_text(stmt, 6, input->api_token_uuid, -1,
                              SQLITE_TRANSIENT);
        } else {
            sqlite3_bind_null(stmt, 6);
        }
        sqlite3_bind_text(stmt, 7, input->action, -1, SQLITE_TRANSIENT);
        if (input->target_type && input->target_type[0]) {
            sqlite3_bind_text(stmt, 8, input->target_type, -1,
                              SQLITE_TRANSIENT);
        } else {
            sqlite3_bind_null(stmt, 8);
        }
        if (input->target_uuid && input->target_uuid[0]) {
            sqlite3_bind_text(stmt, 9, input->target_uuid, -1,
                              SQLITE_TRANSIENT);
        } else {
            sqlite3_bind_null(stmt, 9);
        }
        sqlite3_bind_text(stmt, 10, input->outcome, -1, SQLITE_TRANSIENT);
        if (input->remote_address && input->remote_address[0]) {
            sqlite3_bind_text(stmt, 11, input->remote_address, -1,
                              SQLITE_TRANSIENT);
        } else {
            sqlite3_bind_null(stmt, 11);
        }
        sqlite3_bind_text(stmt, 12, input->details_json, -1,
                          SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);

    if (rc == SQLITE_DONE && event_uuid) {
        stmt = NULL;
        rc = sqlite3_prepare_v2(
            db, "SELECT uuid FROM audit_events "
                "WHERE id=last_insert_rowid();", -1, &stmt, NULL);
        if (rc == SQLITE_OK && sqlite3_step(stmt) == SQLITE_ROW) {
            const char *uuid = (const char *)sqlite3_column_text(stmt, 0);
            safe_strcpy(event_uuid, uuid ? uuid : "",
                        AUDIT_EVENT_UUID_MAX, 0);
            rc = SQLITE_DONE;
        }
        if (stmt) sqlite3_finalize(stmt);
    }

    int64_t now = (int64_t)time(NULL);
    if (rc == SQLITE_DONE && now - last_automatic_prune_at >= 3600) {
        int deleted = 0;
        if (prune_locked(db, &deleted) == 0) {
            last_automatic_prune_at = now;
            if (deleted > 0) log_info("Pruned %d expired audit events", deleted);
        }
    }
    pthread_mutex_unlock(mutex);
    return rc == SQLITE_DONE ? 0 : -1;
}

static void add_query_filters(char *sql, size_t sql_size,
                              const audit_query_t *query) {
    if (query->since > 0) safe_strcat(sql, " AND occurred_at>=?", sql_size);
    if (query->until > 0) safe_strcat(sql, " AND occurred_at<=?", sql_size);
    if (query->principal_user_id > 0) {
        safe_strcat(sql, " AND principal_user_id=?", sql_size);
    }
    if (query->action[0]) safe_strcat(sql, " AND action=?", sql_size);
    if (query->outcome[0]) safe_strcat(sql, " AND outcome=?", sql_size);
    if (query->target_uuid[0]) safe_strcat(sql, " AND target_uuid=?", sql_size);
    if (query->request_id[0]) safe_strcat(sql, " AND request_id=?", sql_size);
}

static int bind_query_filters(sqlite3_stmt *stmt,
                              const audit_query_t *query) {
    int index = 1;
    if (query->since > 0) sqlite3_bind_int64(stmt, index++, query->since);
    if (query->until > 0) sqlite3_bind_int64(stmt, index++, query->until);
    if (query->principal_user_id > 0) {
        sqlite3_bind_int64(stmt, index++, query->principal_user_id);
    }
    if (query->action[0]) {
        sqlite3_bind_text(stmt, index++, query->action, -1, SQLITE_TRANSIENT);
    }
    if (query->outcome[0]) {
        sqlite3_bind_text(stmt, index++, query->outcome, -1, SQLITE_TRANSIENT);
    }
    if (query->target_uuid[0]) {
        sqlite3_bind_text(stmt, index++, query->target_uuid, -1,
                          SQLITE_TRANSIENT);
    }
    if (query->request_id[0]) {
        sqlite3_bind_text(stmt, index++, query->request_id, -1,
                          SQLITE_TRANSIENT);
    }
    return index;
}

static void copy_column(char *destination, size_t destination_size,
                        sqlite3_stmt *stmt, int column) {
    const char *value = (const char *)sqlite3_column_text(stmt, column);
    safe_strcpy(destination, value ? value : "", destination_size, 0);
}

static void populate_event(sqlite3_stmt *stmt, audit_event_t *event) {
    memset(event, 0, sizeof(*event));
    event->id = sqlite3_column_int64(stmt, 0);
    copy_column(event->uuid, sizeof(event->uuid), stmt, 1);
    event->occurred_at = sqlite3_column_int64(stmt, 2);
    copy_column(event->request_id, sizeof(event->request_id), stmt, 3);
    event->principal_user_id = sqlite3_column_int64(stmt, 4);
    copy_column(event->principal_username, sizeof(event->principal_username),
                stmt, 5);
    copy_column(event->auth_method, sizeof(event->auth_method), stmt, 6);
    copy_column(event->api_token_uuid, sizeof(event->api_token_uuid), stmt, 7);
    copy_column(event->action, sizeof(event->action), stmt, 8);
    copy_column(event->target_type, sizeof(event->target_type), stmt, 9);
    copy_column(event->target_uuid, sizeof(event->target_uuid), stmt, 10);
    copy_column(event->outcome, sizeof(event->outcome), stmt, 11);
    copy_column(event->remote_address, sizeof(event->remote_address), stmt, 12);
    copy_column(event->details_json, sizeof(event->details_json), stmt, 13);
}

int db_audit_query(const audit_query_t *query, audit_page_t *page) {
    if (!query || !page || query->page < 1 || query->page_size < 1 ||
        query->page_size > AUDIT_PAGE_SIZE_MAX ||
        (query->outcome[0] && !valid_outcome(query->outcome))) {
        return -1;
    }
    memset(page, 0, sizeof(*page));
    page->page = query->page;
    page->page_size = query->page_size;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);

    char count_sql[1024] = "SELECT count(*) FROM audit_events WHERE 1=1";
    add_query_filters(count_sql, sizeof(count_sql), query);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, count_sql, -1, &stmt, NULL);
    if (rc == SQLITE_OK) bind_query_filters(stmt, query);
    if (rc == SQLITE_OK) rc = sqlite3_step(stmt);
    if (rc == SQLITE_ROW) page->total = sqlite3_column_int64(stmt, 0);
    if (stmt) sqlite3_finalize(stmt);
    if (rc != SQLITE_ROW) {
        pthread_mutex_unlock(mutex);
        return -1;
    }

    char select_sql[2048] =
        "SELECT id,uuid,occurred_at,request_id,"
        "COALESCE(principal_user_id,0),principal_username,auth_method,"
        "COALESCE(api_token_uuid,''),action,COALESCE(target_type,''),"
        "COALESCE(target_uuid,''),outcome,COALESCE(remote_address,''),"
        "details_json FROM audit_events WHERE 1=1";
    add_query_filters(select_sql, sizeof(select_sql), query);
    safe_strcat(select_sql,
                " ORDER BY occurred_at DESC,id DESC LIMIT ? OFFSET ?;",
                sizeof(select_sql));
    stmt = NULL;
    rc = sqlite3_prepare_v2(db, select_sql, -1, &stmt, NULL);
    int next_index = 1;
    if (rc == SQLITE_OK) next_index = bind_query_filters(stmt, query);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int(stmt, next_index++, query->page_size);
        sqlite3_bind_int64(
            stmt, next_index,
            (sqlite3_int64)(query->page - 1) * query->page_size);
    }
    audit_event_t *events = rc == SQLITE_OK
        ? calloc((size_t)query->page_size, sizeof(*events)) : NULL;
    if (rc == SQLITE_OK && !events) rc = SQLITE_NOMEM;
    int count = 0;
    if (rc == SQLITE_OK) {
        while ((rc = sqlite3_step(stmt)) == SQLITE_ROW) {
            populate_event(stmt, &events[count++]);
        }
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    if (rc != SQLITE_DONE) {
        free(events);
        return -1;
    }
    if (count == 0) {
        free(events);
        events = NULL;
    }
    page->events = events;
    page->count = count;
    return 0;
}

void db_audit_page_free(audit_page_t *page) {
    if (!page) return;
    free(page->events);
    memset(page, 0, sizeof(*page));
}

int db_audit_get_retention_days(int *retention_days) {
    if (!retention_days) return -1;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    *retention_days = retention_days_locked(db);
    pthread_mutex_unlock(mutex);
    return 0;
}

int db_audit_set_retention_days(int retention_days) {
    if (retention_days < 1 || retention_days > AUDIT_RETENTION_MAX_DAYS) {
        return -1;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db,
        "INSERT INTO system_settings(key,value,updated_at) "
        "VALUES('audit_retention_days',?,strftime('%s','now')) "
        "ON CONFLICT(key) DO UPDATE SET value=excluded.value,"
        "updated_at=excluded.updated_at;",
        -1, &stmt, NULL);
    char value[16];
    snprintf(value, sizeof(value), "%d", retention_days);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, value, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return rc == SQLITE_DONE ? 0 : -1;
}

int db_audit_prune(int *deleted_count) {
    if (deleted_count) *deleted_count = 0;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    int result = prune_locked(db, deleted_count);
    if (result == 0) last_automatic_prune_at = (int64_t)time(NULL);
    pthread_mutex_unlock(mutex);
    return result;
}
