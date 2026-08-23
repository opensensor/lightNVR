#define _POSIX_C_SOURCE 200809L

#include <pthread.h>
#include <sqlite3.h>
#include <stdlib.h>
#include <string.h>
#include <cjson/cJSON.h>

#include "core/camera_selector.h"
#include "core/logger.h"
#include "database/db_authorization.h"
#include "database/db_core.h"
#include "utils/strings.h"

static void copy_column(char *destination, size_t destination_size,
                        sqlite3_stmt *stmt, int column) {
    const char *value = (const char *)sqlite3_column_text(stmt, column);
    safe_strcpy(destination, value ? value : "", destination_size, 0);
}

int db_authorization_load_user_grants(int64_t user_id, const char *action_key,
                                      authorization_grant_t **grants,
                                      int *grant_count,
                                      int64_t *policy_version) {
    if (!grants || !grant_count || user_id <= 0 || !action_key ||
        action_key[0] == '\0') {
        return -1;
    }
    *grants = NULL;
    *grant_count = 0;
    if (policy_version) *policy_version = 0;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;

    const char *sql =
        "SELECT g.uuid, g.role_uuid, r.name, g.scope_type, "
        "       COALESCE(g.selector_json, '') "
        "FROM authz_grants g "
        "JOIN authz_roles r ON r.uuid = g.role_uuid "
        "JOIN authz_role_actions ra ON ra.role_uuid = g.role_uuid "
        "WHERE g.user_id = ? AND g.enabled = 1 AND ra.action_key = ? "
        "ORDER BY g.created_at, g.uuid "
        "LIMIT ?;";

    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    if (policy_version) {
        int version_rc = sqlite3_prepare_v2(
            db, "SELECT version FROM authz_policy_state WHERE id = 1;", -1,
            &stmt, NULL);
        if (version_rc != SQLITE_OK || sqlite3_step(stmt) != SQLITE_ROW) {
            if (stmt) sqlite3_finalize(stmt);
            pthread_mutex_unlock(mutex);
            return -1;
        }
        *policy_version = sqlite3_column_int64(stmt, 0);
        sqlite3_finalize(stmt);
        stmt = NULL;
    }
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare authorization grant query: %s",
                  sqlite3_errmsg(db));
        pthread_mutex_unlock(mutex);
        return -1;
    }
    sqlite3_bind_int64(stmt, 1, user_id);
    sqlite3_bind_text(stmt, 2, action_key, -1, SQLITE_TRANSIENT);
    sqlite3_bind_int(stmt, 3, AUTHORIZATION_MAX_USER_GRANTS + 1);

    int capacity = 8;
    authorization_grant_t *loaded =
        calloc((size_t)capacity, sizeof(*loaded));
    if (!loaded) {
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(mutex);
        return -1;
    }

    int count = 0;
    while ((rc = sqlite3_step(stmt)) == SQLITE_ROW) {
        if (count >= AUTHORIZATION_MAX_USER_GRANTS) {
            log_error("User %lld exceeds the authorization grant limit",
                      (long long)user_id);
            rc = SQLITE_TOOBIG;
            break;
        }
        if (count == capacity) {
            int next_capacity = capacity * 2;
            authorization_grant_t *resized =
                realloc(loaded, (size_t)next_capacity * sizeof(*loaded));
            if (!resized) {
                rc = SQLITE_NOMEM;
                break;
            }
            loaded = resized;
            memset(&loaded[capacity], 0,
                   (size_t)(next_capacity - capacity) * sizeof(*loaded));
            capacity = next_capacity;
        }
        authorization_grant_t *grant = &loaded[count++];
        copy_column(grant->uuid, sizeof(grant->uuid), stmt, 0);
        copy_column(grant->role_uuid, sizeof(grant->role_uuid), stmt, 1);
        copy_column(grant->role_name, sizeof(grant->role_name), stmt, 2);
        copy_column(grant->scope_type, sizeof(grant->scope_type), stmt, 3);
        const char *selector = (const char *)sqlite3_column_text(stmt, 4);
        if (selector && strlen(selector) >= sizeof(grant->selector_json)) {
            log_error("Authorization selector exceeds supported size for grant %s",
                      grant->uuid);
            rc = SQLITE_TOOBIG;
            break;
        }
        safe_strcpy(grant->selector_json, selector ? selector : "",
                    sizeof(grant->selector_json), 0);
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);

    if (rc != SQLITE_DONE) {
        free(loaded);
        return -1;
    }
    if (count == 0) {
        free(loaded);
        loaded = NULL;
    }
    *grants = loaded;
    *grant_count = count;
    return 0;
}

int db_authorization_get_policy_version(int64_t *version) {
    if (!version) return -1;
    *version = 0;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;

    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db, "SELECT version FROM authz_policy_state WHERE id = 1;", -1,
        &stmt, NULL);
    if (rc == SQLITE_OK && sqlite3_step(stmt) == SQLITE_ROW) {
        *version = sqlite3_column_int64(stmt, 0);
        rc = SQLITE_OK;
    } else {
        rc = SQLITE_ERROR;
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return rc == SQLITE_OK ? 0 : -1;
}

static bool valid_selector(const char *selector_json) {
    if (!selector_json || selector_json[0] == '\0' ||
        strlen(selector_json) >= AUTHORIZATION_SELECTOR_MAX) {
        return false;
    }
    cJSON *json = cJSON_Parse(selector_json);
    if (!json) return false;
    char error[FLEET_SELECTOR_ERROR_MAX] = {0};
    fleet_selector_t *selector =
        fleet_selector_parse(json, error, sizeof(error));
    cJSON_Delete(json);
    if (!selector) return false;
    fleet_selector_free(selector);
    return true;
}

static bool begin_policy_change(sqlite3 *db) {
    return sqlite3_exec(db, "BEGIN IMMEDIATE;", NULL, NULL, NULL) == SQLITE_OK;
}

static bool finish_policy_change(sqlite3 *db, bool success) {
    if (success &&
        sqlite3_exec(db,
                     "UPDATE authz_policy_state "
                     "SET version = version + 1, "
                     "updated_at = strftime('%s', 'now') WHERE id = 1;",
                     NULL, NULL, NULL) == SQLITE_OK &&
        sqlite3_exec(db, "COMMIT;", NULL, NULL, NULL) == SQLITE_OK) {
        return true;
    }
    sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
    return false;
}

int db_authorization_create_user_grant(
    int64_t user_id, const char *role_uuid, const char *scope_type,
    const char *selector_json,
    char grant_uuid[CAMERA_UUID_STRING_SIZE]) {
    bool all_scope = scope_type && strcmp(scope_type, "all") == 0;
    bool selector_scope = scope_type && strcmp(scope_type, "selector") == 0;
    if (user_id <= 0 || !role_uuid || (!all_scope && !selector_scope) ||
        (all_scope && selector_json) ||
        (selector_scope && !valid_selector(selector_json))) {
        return -1;
    }
    if (grant_uuid) grant_uuid[0] = '\0';

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    const char *sql =
        "INSERT INTO authz_grants "
        "(uuid,user_id,role_uuid,scope_type,selector_json) VALUES ("
        "lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || "
        "substr(hex(randomblob(2)),2) || '-' || "
        "substr('89ab',(abs(random()) % 4) + 1,1) || "
        "substr(hex(randomblob(2)),2) || '-' || hex(randomblob(6))),"
        "?,?,?,?);";

    pthread_mutex_lock(mutex);
    if (!begin_policy_change(db)) {
        pthread_mutex_unlock(mutex);
        return -1;
    }
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, user_id);
        sqlite3_bind_text(stmt, 2, role_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 3, scope_type, -1, SQLITE_TRANSIENT);
        if (selector_json) {
            sqlite3_bind_text(stmt, 4, selector_json, -1, SQLITE_TRANSIENT);
        } else {
            sqlite3_bind_null(stmt, 4);
        }
        rc = sqlite3_step(stmt);
    }
    if (stmt) {
        sqlite3_finalize(stmt);
        stmt = NULL;
    }

    char created_uuid[CAMERA_UUID_STRING_SIZE] = {0};
    if (rc == SQLITE_DONE) {
        rc = sqlite3_prepare_v2(
            db, "SELECT uuid FROM authz_grants WHERE rowid = last_insert_rowid();",
            -1, &stmt, NULL);
        if (rc == SQLITE_OK && sqlite3_step(stmt) == SQLITE_ROW) {
            copy_column(created_uuid, sizeof(created_uuid), stmt, 0);
            rc = SQLITE_DONE;
        }
    }
    if (stmt) sqlite3_finalize(stmt);
    bool committed = finish_policy_change(
        db, rc == SQLITE_DONE && created_uuid[0] != '\0');
    pthread_mutex_unlock(mutex);
    if (!committed) return -1;
    if (grant_uuid) {
        safe_strcpy(grant_uuid, created_uuid, CAMERA_UUID_STRING_SIZE, 0);
    }
    return 0;
}

int db_authorization_set_user_mode(int64_t user_id, const char *mode) {
    if (user_id <= 0 || !mode ||
        (strcmp(mode, "legacy") != 0 && strcmp(mode, "policy") != 0)) {
        return -1;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;

    pthread_mutex_lock(mutex);
    if (!begin_policy_change(db)) {
        pthread_mutex_unlock(mutex);
        return -1;
    }
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db, "UPDATE users SET authorization_mode = ?, updated_at = "
            "strftime('%s', 'now') WHERE id = ?;", -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, mode, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(stmt, 2, user_id);
        rc = sqlite3_step(stmt);
    }
    int changed = sqlite3_changes(db);
    if (stmt) sqlite3_finalize(stmt);
    bool committed = finish_policy_change(
        db, rc == SQLITE_DONE && changed == 1);
    pthread_mutex_unlock(mutex);
    return committed ? 0 : -1;
}
