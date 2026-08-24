#define _POSIX_C_SOURCE 200809L

#include <pthread.h>
#include <sqlite3.h>
#include <stdlib.h>
#include <string.h>
#include <cjson/cJSON.h>

#include "core/authorization.h"
#include "core/camera_selector.h"
#include "core/logger.h"
#include "database/db_authorization.h"
#include "database/db_core.h"
#include "utils/strings.h"

#define AUTHZ_ADMIN_ROLE_UUID "00000000-0000-4000-8000-000000000001"
#define AUTHZ_OPERATOR_ROLE_UUID "00000000-0000-4000-8000-000000000002"
#define AUTHZ_VIEWER_ROLE_UUID "00000000-0000-4000-8000-000000000003"
#define AUTHZ_API_ROLE_UUID "00000000-0000-4000-8000-000000000004"

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
        "       COALESCE(g.selector_json, ''), "
        "       COALESCE(g.collection_uuid, '') "
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
        copy_column(grant->collection_uuid, sizeof(grant->collection_uuid),
                    stmt, 5);
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

static bool sqlite_constraint_result(int rc) {
    return (rc & 0xff) == SQLITE_CONSTRAINT;
}

static bool shared_collection_exists_locked(sqlite3 *db,
                                            const char *collection_uuid) {
    if (!collection_uuid || collection_uuid[0] == '\0') return false;
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db,
        "SELECT 1 FROM camera_collections WHERE uuid=? AND is_shared=1;",
        -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, collection_uuid, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);
    return rc == SQLITE_ROW;
}

int db_authorization_create_user_grant(
    int64_t user_id, const char *role_uuid, const char *scope_type,
    const char *selector_json, const char *collection_uuid,
    char grant_uuid[CAMERA_UUID_STRING_SIZE]) {
    bool all_scope = scope_type && strcmp(scope_type, "all") == 0;
    bool selector_scope = scope_type && strcmp(scope_type, "selector") == 0;
    bool collection_scope =
        scope_type && strcmp(scope_type, "collection") == 0;
    if (user_id <= 0 || !role_uuid ||
        (!all_scope && !selector_scope && !collection_scope) ||
        (all_scope && (selector_json || collection_uuid)) ||
        (selector_scope &&
         (!valid_selector(selector_json) || collection_uuid)) ||
        (collection_scope && (selector_json || !collection_uuid ||
                              collection_uuid[0] == '\0'))) {
        return -1;
    }
    if (grant_uuid) grant_uuid[0] = '\0';

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    const char *sql =
        "INSERT INTO authz_grants "
        "(uuid,user_id,role_uuid,scope_type,selector_json,collection_uuid) "
        "VALUES ("
        "lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || "
        "substr(hex(randomblob(2)),2) || '-' || "
        "substr('89ab',(abs(random()) % 4) + 1,1) || "
        "substr(hex(randomblob(2)),2) || '-' || hex(randomblob(6))),"
        "?,?,?,?,?);";

    pthread_mutex_lock(mutex);
    if (!begin_policy_change(db)) {
        pthread_mutex_unlock(mutex);
        return -1;
    }
    if (collection_scope &&
        !shared_collection_exists_locked(db, collection_uuid)) {
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
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
        if (collection_uuid) {
            sqlite3_bind_text(stmt, 5, collection_uuid, -1,
                              SQLITE_TRANSIENT);
        } else {
            sqlite3_bind_null(stmt, 5);
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

static int64_t policy_version_locked(sqlite3 *db) {
    sqlite3_stmt *stmt = NULL;
    int64_t version = -1;
    if (sqlite3_prepare_v2(
            db, "SELECT version FROM authz_policy_state WHERE id = 1;", -1,
            &stmt, NULL) == SQLITE_OK &&
        sqlite3_step(stmt) == SQLITE_ROW) {
        version = sqlite3_column_int64(stmt, 0);
    }
    if (stmt) sqlite3_finalize(stmt);
    return version;
}

static uint64_t action_bit(const char *action_key) {
    authorization_action_t action = authorization_action_from_key(action_key);
    return action >= 0 && action < 64 ? UINT64_C(1) << action : 0;
}

static bool valid_action_mask(uint64_t action_mask) {
    if (action_mask == 0) return false;
    uint64_t valid_mask = AUTHZ_ACTION_COUNT == 64
        ? UINT64_MAX
        : (UINT64_C(1) << AUTHZ_ACTION_COUNT) - 1;
    return (action_mask & ~valid_mask) == 0;
}

static bool load_role_actions_locked(sqlite3 *db, const char *role_uuid,
                                     uint64_t *action_mask) {
    const char *sql =
        "SELECT action_key FROM authz_role_actions WHERE role_uuid = ?;";
    sqlite3_stmt *stmt = NULL;
    if (sqlite3_prepare_v2(db, sql, -1, &stmt, NULL) != SQLITE_OK) {
        return false;
    }
    sqlite3_bind_text(stmt, 1, role_uuid, -1, SQLITE_TRANSIENT);
    uint64_t mask = 0;
    int rc = SQLITE_ROW;
    while ((rc = sqlite3_step(stmt)) == SQLITE_ROW) {
        const char *key = (const char *)sqlite3_column_text(stmt, 0);
        uint64_t bit = action_bit(key);
        if (bit == 0) {
            sqlite3_finalize(stmt);
            return false;
        }
        mask |= bit;
    }
    sqlite3_finalize(stmt);
    if (rc != SQLITE_DONE) return false;
    *action_mask = mask;
    return true;
}

static bool insert_role_actions_locked(sqlite3 *db, const char *role_uuid,
                                       uint64_t action_mask) {
    sqlite3_stmt *stmt = NULL;
    if (sqlite3_prepare_v2(
            db,
            "INSERT INTO authz_role_actions (role_uuid,action_key) VALUES (?,?);",
            -1, &stmt, NULL) != SQLITE_OK) {
        return false;
    }
    int action_count = 0;
    const authorization_action_metadata_t *catalog =
        authorization_action_catalog(&action_count);
    bool success = true;
    for (int i = 0; i < action_count; i++) {
        if ((action_mask & (UINT64_C(1) << catalog[i].action)) == 0) continue;
        sqlite3_reset(stmt);
        sqlite3_clear_bindings(stmt);
        sqlite3_bind_text(stmt, 1, role_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 2, catalog[i].key, -1, SQLITE_TRANSIENT);
        if (sqlite3_step(stmt) != SQLITE_DONE) {
            success = false;
            break;
        }
    }
    sqlite3_finalize(stmt);
    return success;
}

int db_authorization_role_count(void) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int count = -1;
    if (sqlite3_prepare_v2(db, "SELECT COUNT(*) FROM authz_roles;", -1,
                          &stmt, NULL) == SQLITE_OK &&
        sqlite3_step(stmt) == SQLITE_ROW) {
        count = sqlite3_column_int(stmt, 0);
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return count;
}

int db_authorization_role_list(authorization_role_t *roles, int max_count) {
    if (!roles || max_count <= 0) return -1;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    const char *sql =
        "SELECT uuid,name,description,is_builtin,created_at,updated_at "
        "FROM authz_roles ORDER BY is_builtin DESC,name COLLATE NOCASE LIMIT ?;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    if (sqlite3_prepare_v2(db, sql, -1, &stmt, NULL) != SQLITE_OK) {
        pthread_mutex_unlock(mutex);
        return -1;
    }
    sqlite3_bind_int(stmt, 1, max_count);
    int count = 0;
    int rc = SQLITE_ROW;
    while ((rc = sqlite3_step(stmt)) == SQLITE_ROW) {
        authorization_role_t *role = &roles[count];
        memset(role, 0, sizeof(*role));
        copy_column(role->uuid, sizeof(role->uuid), stmt, 0);
        copy_column(role->name, sizeof(role->name), stmt, 1);
        copy_column(role->description, sizeof(role->description), stmt, 2);
        role->is_builtin = sqlite3_column_int(stmt, 3) != 0;
        role->created_at = sqlite3_column_int64(stmt, 4);
        role->updated_at = sqlite3_column_int64(stmt, 5);
        count++;
    }
    sqlite3_finalize(stmt);
    bool success = rc == SQLITE_DONE;
    for (int i = 0; success && i < count; i++) {
        success = load_role_actions_locked(db, roles[i].uuid,
                                           &roles[i].action_mask);
    }
    pthread_mutex_unlock(mutex);
    return success ? count : -1;
}

db_authorization_result_t db_authorization_load_roles(
    authorization_role_t **roles, int *role_count, int64_t *policy_version) {
    if (!roles || !role_count || !policy_version) {
        return DB_AUTHORIZATION_INVALID;
    }
    *roles = NULL;
    *role_count = 0;
    *policy_version = 0;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_AUTHORIZATION_ERROR;
    pthread_mutex_lock(mutex);
    int64_t version = policy_version_locked(db);
    sqlite3_stmt *stmt = NULL;
    int count = -1;
    if (version >= 0 &&
        sqlite3_prepare_v2(db, "SELECT COUNT(*) FROM authz_roles;", -1,
                          &stmt, NULL) == SQLITE_OK &&
        sqlite3_step(stmt) == SQLITE_ROW) {
        count = sqlite3_column_int(stmt, 0);
    }
    if (stmt) sqlite3_finalize(stmt);
    if (count < 0) {
        pthread_mutex_unlock(mutex);
        return DB_AUTHORIZATION_ERROR;
    }
    authorization_role_t *loaded = count > 0
        ? calloc((size_t)count, sizeof(*loaded)) : NULL;
    if (count > 0 && !loaded) {
        pthread_mutex_unlock(mutex);
        return DB_AUTHORIZATION_ERROR;
    }
    const char *sql =
        "SELECT uuid,name,description,is_builtin,created_at,updated_at "
        "FROM authz_roles ORDER BY is_builtin DESC,name COLLATE NOCASE;";
    stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    int loaded_count = 0;
    if (rc == SQLITE_OK) {
        while ((rc = sqlite3_step(stmt)) == SQLITE_ROW) {
            if (loaded_count >= count) {
                rc = SQLITE_TOOBIG;
                break;
            }
            authorization_role_t *role = &loaded[loaded_count++];
            copy_column(role->uuid, sizeof(role->uuid), stmt, 0);
            copy_column(role->name, sizeof(role->name), stmt, 1);
            copy_column(role->description, sizeof(role->description), stmt, 2);
            role->is_builtin = sqlite3_column_int(stmt, 3) != 0;
            role->created_at = sqlite3_column_int64(stmt, 4);
            role->updated_at = sqlite3_column_int64(stmt, 5);
        }
    }
    if (stmt) sqlite3_finalize(stmt);
    bool success = rc == SQLITE_DONE && loaded_count == count;
    for (int i = 0; success && i < loaded_count; i++) {
        success = load_role_actions_locked(db, loaded[i].uuid,
                                           &loaded[i].action_mask);
    }
    pthread_mutex_unlock(mutex);
    if (!success) {
        free(loaded);
        return DB_AUTHORIZATION_ERROR;
    }
    *roles = loaded;
    *role_count = loaded_count;
    *policy_version = version;
    return DB_AUTHORIZATION_OK;
}

db_authorization_result_t db_authorization_role_get(
    const char *uuid, authorization_role_t *role) {
    if (!uuid || !role) return DB_AUTHORIZATION_INVALID;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_AUTHORIZATION_ERROR;
    const char *sql =
        "SELECT uuid,name,description,is_builtin,created_at,updated_at "
        "FROM authz_roles WHERE uuid = ?;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, uuid, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    db_authorization_result_t result = DB_AUTHORIZATION_ERROR;
    if (rc == SQLITE_ROW) {
        memset(role, 0, sizeof(*role));
        copy_column(role->uuid, sizeof(role->uuid), stmt, 0);
        copy_column(role->name, sizeof(role->name), stmt, 1);
        copy_column(role->description, sizeof(role->description), stmt, 2);
        role->is_builtin = sqlite3_column_int(stmt, 3) != 0;
        role->created_at = sqlite3_column_int64(stmt, 4);
        role->updated_at = sqlite3_column_int64(stmt, 5);
        result = DB_AUTHORIZATION_OK;
    } else if (rc == SQLITE_DONE) {
        result = DB_AUTHORIZATION_NOT_FOUND;
    }
    if (stmt) sqlite3_finalize(stmt);
    if (result == DB_AUTHORIZATION_OK &&
        !load_role_actions_locked(db, uuid, &role->action_mask)) {
        result = DB_AUTHORIZATION_ERROR;
    }
    pthread_mutex_unlock(mutex);
    return result;
}

db_authorization_result_t db_authorization_role_create(
    authorization_role_t *role, int64_t expected_version,
    int64_t *new_version) {
    if (!role || !new_version || expected_version < 1 ||
        role->name[0] == '\0' || !valid_action_mask(role->action_mask)) {
        return DB_AUTHORIZATION_INVALID;
    }
    *new_version = 0;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_AUTHORIZATION_ERROR;
    const char *sql =
        "INSERT INTO authz_roles (uuid,name,description,is_builtin) VALUES ("
        "lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || "
        "substr(hex(randomblob(2)),2) || '-' || "
        "substr('89ab',(abs(random()) % 4) + 1,1) || "
        "substr(hex(randomblob(2)),2) || '-' || hex(randomblob(6))),?,?,0);";
    pthread_mutex_lock(mutex);
    if (!begin_policy_change(db)) {
        pthread_mutex_unlock(mutex);
        return DB_AUTHORIZATION_ERROR;
    }
    int64_t current_version = policy_version_locked(db);
    if (current_version != expected_version) {
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        pthread_mutex_unlock(mutex);
        return current_version < 0 ? DB_AUTHORIZATION_ERROR
                                   : DB_AUTHORIZATION_STALE;
    }
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, role->name, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 2, role->description, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (stmt) {
        sqlite3_finalize(stmt);
        stmt = NULL;
    }
    char uuid[CAMERA_UUID_STRING_SIZE] = {0};
    if (rc == SQLITE_DONE &&
        sqlite3_prepare_v2(
            db, "SELECT uuid FROM authz_roles WHERE rowid=last_insert_rowid();",
            -1, &stmt, NULL) == SQLITE_OK &&
        sqlite3_step(stmt) == SQLITE_ROW) {
        copy_column(uuid, sizeof(uuid), stmt, 0);
    }
    if (stmt) sqlite3_finalize(stmt);
    bool success = uuid[0] != '\0' &&
                   insert_role_actions_locked(db, uuid, role->action_mask);
    bool committed = finish_policy_change(db, success);
    pthread_mutex_unlock(mutex);
    if (!committed) {
        return sqlite_constraint_result(rc) ? DB_AUTHORIZATION_CONFLICT
                                            : DB_AUTHORIZATION_ERROR;
    }
    safe_strcpy(role->uuid, uuid, sizeof(role->uuid), 0);
    role->is_builtin = false;
    *new_version = current_version + 1;
    return DB_AUTHORIZATION_OK;
}

db_authorization_result_t db_authorization_role_update(
    const authorization_role_t *role, int64_t expected_version,
    int64_t *new_version) {
    if (!role || !new_version || expected_version < 1 ||
        role->uuid[0] == '\0' || role->name[0] == '\0' ||
        !valid_action_mask(role->action_mask)) {
        return DB_AUTHORIZATION_INVALID;
    }
    *new_version = 0;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_AUTHORIZATION_ERROR;
    pthread_mutex_lock(mutex);
    if (!begin_policy_change(db)) {
        pthread_mutex_unlock(mutex);
        return DB_AUTHORIZATION_ERROR;
    }
    int64_t current_version = policy_version_locked(db);
    if (current_version != expected_version) {
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        pthread_mutex_unlock(mutex);
        return current_version < 0 ? DB_AUTHORIZATION_ERROR
                                   : DB_AUTHORIZATION_STALE;
    }
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db, "SELECT is_builtin FROM authz_roles WHERE uuid=?;", -1, &stmt,
        NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, role->uuid, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (rc == SQLITE_DONE) {
        sqlite3_finalize(stmt);
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        pthread_mutex_unlock(mutex);
        return DB_AUTHORIZATION_NOT_FOUND;
    }
    if (rc != SQLITE_ROW || sqlite3_column_int(stmt, 0) != 0) {
        bool immutable = rc == SQLITE_ROW;
        sqlite3_finalize(stmt);
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        pthread_mutex_unlock(mutex);
        return immutable ? DB_AUTHORIZATION_IMMUTABLE
                         : DB_AUTHORIZATION_ERROR;
    }
    sqlite3_finalize(stmt);
    stmt = NULL;
    rc = sqlite3_prepare_v2(
        db, "UPDATE authz_roles SET name=?,description=?,updated_at="
            "strftime('%s','now') WHERE uuid=?;", -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, role->name, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 2, role->description, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 3, role->uuid, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);
    bool success = rc == SQLITE_DONE;
    if (success) {
        stmt = NULL;
        success = sqlite3_prepare_v2(
            db, "DELETE FROM authz_role_actions WHERE role_uuid=?;", -1,
            &stmt, NULL) == SQLITE_OK;
        if (success) {
            sqlite3_bind_text(stmt, 1, role->uuid, -1, SQLITE_TRANSIENT);
            success = sqlite3_step(stmt) == SQLITE_DONE;
        }
        if (stmt) sqlite3_finalize(stmt);
    }
    if (success) {
        success = insert_role_actions_locked(db, role->uuid,
                                             role->action_mask);
    }
    bool committed = finish_policy_change(db, success);
    pthread_mutex_unlock(mutex);
    if (!committed) {
        return sqlite_constraint_result(rc) ? DB_AUTHORIZATION_CONFLICT
                                            : DB_AUTHORIZATION_ERROR;
    }
    *new_version = current_version + 1;
    return DB_AUTHORIZATION_OK;
}

db_authorization_result_t db_authorization_role_delete(
    const char *uuid, int64_t expected_version, int64_t *new_version) {
    if (!uuid || uuid[0] == '\0' || !new_version || expected_version < 1) {
        return DB_AUTHORIZATION_INVALID;
    }
    *new_version = 0;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_AUTHORIZATION_ERROR;
    pthread_mutex_lock(mutex);
    if (!begin_policy_change(db)) {
        pthread_mutex_unlock(mutex);
        return DB_AUTHORIZATION_ERROR;
    }
    int64_t current_version = policy_version_locked(db);
    if (current_version != expected_version) {
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        pthread_mutex_unlock(mutex);
        return current_version < 0 ? DB_AUTHORIZATION_ERROR
                                   : DB_AUTHORIZATION_STALE;
    }
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db, "SELECT is_builtin FROM authz_roles WHERE uuid=?;", -1, &stmt,
        NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, uuid, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (rc == SQLITE_DONE) {
        sqlite3_finalize(stmt);
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        pthread_mutex_unlock(mutex);
        return DB_AUTHORIZATION_NOT_FOUND;
    }
    if (rc != SQLITE_ROW || sqlite3_column_int(stmt, 0) != 0) {
        bool immutable = rc == SQLITE_ROW;
        sqlite3_finalize(stmt);
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        pthread_mutex_unlock(mutex);
        return immutable ? DB_AUTHORIZATION_IMMUTABLE
                         : DB_AUTHORIZATION_ERROR;
    }
    sqlite3_finalize(stmt);
    stmt = NULL;
    rc = sqlite3_prepare_v2(
        db, "DELETE FROM authz_roles WHERE uuid=?;", -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, uuid, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);
    bool success = rc == SQLITE_DONE && sqlite3_changes(db) == 1;
    bool committed = finish_policy_change(db, success);
    pthread_mutex_unlock(mutex);
    if (committed) {
        *new_version = current_version + 1;
        return DB_AUTHORIZATION_OK;
    }
    return sqlite_constraint_result(rc) ? DB_AUTHORIZATION_IN_USE
                                        : DB_AUTHORIZATION_ERROR;
}

db_authorization_result_t db_authorization_get_user_policy(
    int64_t user_id, char mode[USER_AUTHORIZATION_MODE_MAX],
    authorization_grant_t **grants, int *grant_count,
    int64_t *policy_version) {
    if (user_id <= 0 || !mode || !grants || !grant_count || !policy_version) {
        return DB_AUTHORIZATION_INVALID;
    }
    mode[0] = '\0';
    *grants = NULL;
    *grant_count = 0;
    *policy_version = 0;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_AUTHORIZATION_ERROR;
    pthread_mutex_lock(mutex);
    *policy_version = policy_version_locked(db);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db, "SELECT authorization_mode FROM users WHERE id=?;", -1, &stmt,
        NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, user_id);
        rc = sqlite3_step(stmt);
    }
    if (rc == SQLITE_DONE) {
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(mutex);
        return DB_AUTHORIZATION_NOT_FOUND;
    }
    if (rc != SQLITE_ROW || *policy_version < 0) {
        if (stmt) sqlite3_finalize(stmt);
        pthread_mutex_unlock(mutex);
        return DB_AUTHORIZATION_ERROR;
    }
    copy_column(mode, USER_AUTHORIZATION_MODE_MAX, stmt, 0);
    sqlite3_finalize(stmt);
    stmt = NULL;
    const char *sql =
        "SELECT g.uuid,g.user_id,g.role_uuid,r.name,g.scope_type,"
        "COALESCE(g.selector_json,''),COALESCE(g.collection_uuid,''),"
        "g.enabled,g.created_at,g.updated_at "
        "FROM authz_grants g JOIN authz_roles r ON r.uuid=g.role_uuid "
        "WHERE g.user_id=? ORDER BY g.created_at,g.uuid LIMIT ?;";
    if (sqlite3_prepare_v2(db, sql, -1, &stmt, NULL) != SQLITE_OK) {
        if (stmt) sqlite3_finalize(stmt);
        pthread_mutex_unlock(mutex);
        return DB_AUTHORIZATION_ERROR;
    }
    sqlite3_bind_int64(stmt, 1, user_id);
    sqlite3_bind_int(stmt, 2, AUTHORIZATION_MAX_USER_GRANTS + 1);
    int capacity = 8;
    authorization_grant_t *loaded = calloc((size_t)capacity, sizeof(*loaded));
    if (!loaded) {
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(mutex);
        return DB_AUTHORIZATION_ERROR;
    }
    int count = 0;
    while ((rc = sqlite3_step(stmt)) == SQLITE_ROW) {
        if (count >= AUTHORIZATION_MAX_USER_GRANTS) {
            rc = SQLITE_TOOBIG;
            break;
        }
        if (count == capacity) {
            int next_capacity = capacity * 2;
            authorization_grant_t *resized = realloc(
                loaded, (size_t)next_capacity * sizeof(*loaded));
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
        grant->user_id = sqlite3_column_int64(stmt, 1);
        copy_column(grant->role_uuid, sizeof(grant->role_uuid), stmt, 2);
        copy_column(grant->role_name, sizeof(grant->role_name), stmt, 3);
        copy_column(grant->scope_type, sizeof(grant->scope_type), stmt, 4);
        const char *selector = (const char *)sqlite3_column_text(stmt, 5);
        if (selector && strlen(selector) >= sizeof(grant->selector_json)) {
            rc = SQLITE_TOOBIG;
            break;
        }
        safe_strcpy(grant->selector_json, selector ? selector : "",
                    sizeof(grant->selector_json), 0);
        copy_column(grant->collection_uuid, sizeof(grant->collection_uuid),
                    stmt, 6);
        grant->enabled = sqlite3_column_int(stmt, 7) != 0;
        grant->created_at = sqlite3_column_int64(stmt, 8);
        grant->updated_at = sqlite3_column_int64(stmt, 9);
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    if (rc != SQLITE_DONE) {
        free(loaded);
        return DB_AUTHORIZATION_ERROR;
    }
    if (count == 0) {
        free(loaded);
        loaded = NULL;
    }
    *grants = loaded;
    *grant_count = count;
    return DB_AUTHORIZATION_OK;
}

static bool duplicate_grant_input(const authorization_grant_input_t *grants,
                                  int index) {
    for (int i = 0; i < index; i++) {
        if (strcmp(grants[i].role_uuid, grants[index].role_uuid) == 0 &&
            strcmp(grants[i].scope_type, grants[index].scope_type) == 0 &&
            strcmp(grants[i].selector_json,
                   grants[index].selector_json) == 0 &&
            strcmp(grants[i].collection_uuid,
                   grants[index].collection_uuid) == 0) {
            return true;
        }
    }
    return false;
}

static bool valid_grant_input(const authorization_grant_input_t *grant) {
    if (!grant || grant->role_uuid[0] == '\0') return false;
    if (strcmp(grant->scope_type, "all") == 0) {
        return grant->selector_json[0] == '\0' &&
               grant->collection_uuid[0] == '\0';
    }
    if (strcmp(grant->scope_type, "selector") == 0) {
        return grant->collection_uuid[0] == '\0' &&
               valid_selector(grant->selector_json);
    }
    return strcmp(grant->scope_type, "collection") == 0 &&
           grant->selector_json[0] == '\0' &&
           grant->collection_uuid[0] != '\0';
}

db_authorization_result_t db_authorization_replace_user_policy(
    int64_t user_id, const char *mode,
    const authorization_grant_input_t *grants, int grant_count,
    int64_t expected_version, int64_t *new_version) {
    if (user_id <= 0 || !mode || !new_version || expected_version < 1 ||
        grant_count < 0 || grant_count > AUTHORIZATION_MAX_USER_GRANTS ||
        (grant_count > 0 && !grants) ||
        (strcmp(mode, "legacy") != 0 && strcmp(mode, "policy") != 0)) {
        return DB_AUTHORIZATION_INVALID;
    }
    *new_version = 0;
    for (int i = 0; i < grant_count; i++) {
        if (!valid_grant_input(&grants[i]) ||
            duplicate_grant_input(grants, i)) {
            return DB_AUTHORIZATION_INVALID;
        }
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_AUTHORIZATION_ERROR;
    pthread_mutex_lock(mutex);
    if (!begin_policy_change(db)) {
        pthread_mutex_unlock(mutex);
        return DB_AUTHORIZATION_ERROR;
    }
    int64_t current_version = policy_version_locked(db);
    if (current_version != expected_version) {
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        pthread_mutex_unlock(mutex);
        return current_version < 0 ? DB_AUTHORIZATION_ERROR
                                   : DB_AUTHORIZATION_STALE;
    }
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, "SELECT 1 FROM users WHERE id=?;", -1,
                                &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, user_id);
        rc = sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);
    if (rc == SQLITE_DONE) {
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        pthread_mutex_unlock(mutex);
        return DB_AUTHORIZATION_NOT_FOUND;
    }
    if (rc != SQLITE_ROW) {
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        pthread_mutex_unlock(mutex);
        return DB_AUTHORIZATION_ERROR;
    }
    const char *role_sql = "SELECT 1 FROM authz_roles WHERE uuid=?;";
    for (int i = 0; i < grant_count; i++) {
        stmt = NULL;
        rc = sqlite3_prepare_v2(db, role_sql, -1, &stmt, NULL);
        if (rc == SQLITE_OK) {
            sqlite3_bind_text(stmt, 1, grants[i].role_uuid, -1,
                              SQLITE_TRANSIENT);
            rc = sqlite3_step(stmt);
        }
        if (stmt) sqlite3_finalize(stmt);
        if (rc != SQLITE_ROW) {
            sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
            pthread_mutex_unlock(mutex);
            return rc == SQLITE_DONE ? DB_AUTHORIZATION_INVALID
                                     : DB_AUTHORIZATION_ERROR;
        }
        if (strcmp(grants[i].scope_type, "collection") == 0 &&
            !shared_collection_exists_locked(db,
                                             grants[i].collection_uuid)) {
            sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
            pthread_mutex_unlock(mutex);
            return DB_AUTHORIZATION_INVALID;
        }
    }
    stmt = NULL;
    rc = sqlite3_prepare_v2(db, "DELETE FROM authz_grants WHERE user_id=?;",
                            -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, user_id);
        rc = sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);
    const char *insert_sql =
        "INSERT INTO authz_grants "
        "(uuid,user_id,role_uuid,scope_type,selector_json,collection_uuid) "
        "VALUES ("
        "lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || "
        "substr(hex(randomblob(2)),2) || '-' || "
        "substr('89ab',(abs(random()) % 4) + 1,1) || "
        "substr(hex(randomblob(2)),2) || '-' || hex(randomblob(6))),"
        "?,?,?,?,?);";
    for (int i = 0; rc == SQLITE_DONE && i < grant_count; i++) {
        stmt = NULL;
        rc = sqlite3_prepare_v2(db, insert_sql, -1, &stmt, NULL);
        if (rc == SQLITE_OK) {
            sqlite3_bind_int64(stmt, 1, user_id);
            sqlite3_bind_text(stmt, 2, grants[i].role_uuid, -1,
                              SQLITE_TRANSIENT);
            sqlite3_bind_text(stmt, 3, grants[i].scope_type, -1,
                              SQLITE_TRANSIENT);
            if (grants[i].selector_json[0]) {
                sqlite3_bind_text(stmt, 4, grants[i].selector_json, -1,
                                  SQLITE_TRANSIENT);
            } else {
                sqlite3_bind_null(stmt, 4);
            }
            if (grants[i].collection_uuid[0]) {
                sqlite3_bind_text(stmt, 5, grants[i].collection_uuid, -1,
                                  SQLITE_TRANSIENT);
            } else {
                sqlite3_bind_null(stmt, 5);
            }
            rc = sqlite3_step(stmt);
        }
        if (stmt) sqlite3_finalize(stmt);
    }
    if (rc == SQLITE_DONE) {
        stmt = NULL;
        rc = sqlite3_prepare_v2(
            db, "UPDATE users SET authorization_mode=?,updated_at="
                "strftime('%s','now') WHERE id=?;", -1, &stmt, NULL);
        if (rc == SQLITE_OK) {
            sqlite3_bind_text(stmt, 1, mode, -1, SQLITE_TRANSIENT);
            sqlite3_bind_int64(stmt, 2, user_id);
            rc = sqlite3_step(stmt);
        }
        if (stmt) sqlite3_finalize(stmt);
    }
    bool committed = finish_policy_change(db, rc == SQLITE_DONE);
    if (committed) *new_version = current_version + 1;
    pthread_mutex_unlock(mutex);
    return committed ? DB_AUTHORIZATION_OK : DB_AUTHORIZATION_ERROR;
}

/*
 * Reconcile the compiled action catalog with the authz_actions table.
 *
 * authz_actions is the referential target for authz_role_actions and records
 * the bit position each action occupies inside a persisted API-token
 * action_mask. Those positions are frozen once a token has been issued, so a
 * mismatch between the table and the running binary means either the catalog
 * was reordered or a migration did not apply. Both silently re-map existing
 * token permissions, so refuse to start rather than serve a policy the
 * operator did not author.
 */
int db_authorization_verify_action_catalog(void) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;

    int catalog_count = 0;
    (void)authorization_action_catalog(&catalog_count);

    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db, "SELECT action_key,bit_index FROM authz_actions;", -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to read the authorization action catalog: %s",
                  sqlite3_errmsg(db));
        if (stmt) sqlite3_finalize(stmt);
        pthread_mutex_unlock(mutex);
        return -1;
    }

    int matched = 0;
    int mismatches = 0;
    while ((rc = sqlite3_step(stmt)) == SQLITE_ROW) {
        const char *key = (const char *)sqlite3_column_text(stmt, 0);
        int64_t bit_index = sqlite3_column_int64(stmt, 1);
        authorization_action_t action = authorization_action_from_key(key);
        if (action == AUTHZ_ACTION_INVALID) {
            log_error("Action '%s' exists in authz_actions but not in this "
                      "build's catalog", key ? key : "(null)");
            mismatches++;
            continue;
        }
        if (bit_index != (int64_t)action) {
            log_error("Action '%s' is stored at mask bit %lld but this build "
                      "uses bit %d; API token permissions would be re-mapped",
                      key, (long long)bit_index, (int)action);
            mismatches++;
            continue;
        }
        matched++;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);

    if (rc != SQLITE_DONE) return -1;
    if (matched != catalog_count) {
        log_error("Authorization action catalog has %d of %d expected actions",
                  matched, catalog_count);
        return -1;
    }
    return mismatches == 0 ? 0 : -1;
}

#define LEGACY_ALLOWED_TAGS_MAX 256

typedef struct {
    int64_t user_id;
    user_role_t role;
    char allowed_tags[LEGACY_ALLOWED_TAGS_MAX];
    bool has_allowed_tags;
} legacy_principal_t;

static const char *compatibility_role_uuid(user_role_t role) {
    switch (role) {
        case USER_ROLE_ADMIN: return AUTHZ_ADMIN_ROLE_UUID;
        case USER_ROLE_USER: return AUTHZ_OPERATOR_ROLE_UUID;
        case USER_ROLE_VIEWER: return AUTHZ_VIEWER_ROLE_UUID;
        case USER_ROLE_API: return AUTHZ_API_ROLE_UUID;
        default: return NULL;
    }
}

static bool lookup_or_create_tag_uuid_locked(
    sqlite3 *db, const char *label, char uuid[CAMERA_UUID_STRING_SIZE]) {
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db, "SELECT uuid FROM camera_tags WHERE label=? COLLATE NOCASE;", -1,
        &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, label, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (rc == SQLITE_ROW) {
        copy_column(uuid, CAMERA_UUID_STRING_SIZE, stmt, 0);
        sqlite3_finalize(stmt);
        return uuid[0] != '\0';
    }
    if (stmt) sqlite3_finalize(stmt);
    if (rc != SQLITE_DONE) return false;

    const char *insert_sql =
        "INSERT INTO camera_tags (uuid,label) VALUES ("
        "lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || "
        "substr(hex(randomblob(2)),2) || '-' || "
        "substr('89ab',(abs(random()) % 4) + 1,1) || "
        "substr(hex(randomblob(2)),2) || '-' || hex(randomblob(6))),?);";
    stmt = NULL;
    rc = sqlite3_prepare_v2(db, insert_sql, -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, label, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);
    if (rc != SQLITE_DONE) return false;

    stmt = NULL;
    rc = sqlite3_prepare_v2(
        db, "SELECT uuid FROM camera_tags WHERE label=? COLLATE NOCASE;", -1,
        &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, label, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (rc == SQLITE_ROW) copy_column(uuid, CAMERA_UUID_STRING_SIZE, stmt, 0);
    if (stmt) sqlite3_finalize(stmt);
    return rc == SQLITE_ROW && uuid[0] != '\0';
}

static char *trim_legacy_tag(char *value) {
    if (!value) return value;
    while (*value == ' ' || *value == '\t' || *value == '\r' ||
           *value == '\n') {
        value++;
    }
    size_t length = strlen(value);
    while (length > 0 &&
           (value[length - 1] == ' ' || value[length - 1] == '\t' ||
            value[length - 1] == '\r' || value[length - 1] == '\n')) {
        value[--length] = '\0';
    }
    return value;
}

static char *legacy_tag_selector_locked(sqlite3 *db,
                                        const char *allowed_tags) {
    if (!allowed_tags || allowed_tags[0] == '\0') return NULL;
    char copy[LEGACY_ALLOWED_TAGS_MAX];
    safe_strcpy(copy, allowed_tags, sizeof(copy), 0);
    cJSON *root = cJSON_CreateObject();
    cJSON *expression = cJSON_CreateObject();
    cJSON *uuids = cJSON_CreateArray();
    if (!root || !expression || !uuids) {
        cJSON_Delete(root);
        cJSON_Delete(expression);
        cJSON_Delete(uuids);
        return NULL;
    }
    cJSON_AddNumberToObject(root, "version", 1);
    cJSON_AddStringToObject(expression, "op", "tag_any");
    cJSON_AddItemToObject(expression, "uuids", uuids);
    cJSON_AddItemToObject(root, "expression", expression);

    int tag_count = 0;
    char *saveptr = NULL;
    for (char *item = strtok_r(copy, ",", &saveptr); item;
         item = strtok_r(NULL, ",", &saveptr)) {
        char *label = trim_legacy_tag(item);
        if (!label[0]) continue;
        char uuid[CAMERA_UUID_STRING_SIZE] = {0};
        if (!lookup_or_create_tag_uuid_locked(db, label, uuid) ||
            !cJSON_AddItemToArray(uuids, cJSON_CreateString(uuid))) {
            cJSON_Delete(root);
            return NULL;
        }
        tag_count++;
    }
    if (tag_count == 0) {
        cJSON_Delete(root);
        return NULL;
    }
    char *serialized = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if (serialized && strlen(serialized) >= AUTHORIZATION_SELECTOR_MAX) {
        free(serialized);
        return NULL;
    }
    return serialized;
}

int db_authorization_migrate_legacy_users(int *migrated_count) {
    if (migrated_count) *migrated_count = 0;
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
        db,
        "SELECT id,role,allowed_tags FROM users "
        "WHERE authorization_mode='legacy' ORDER BY id;",
        -1, &stmt, NULL);
    legacy_principal_t *principals = NULL;
    int count = 0;
    int capacity = 0;
    int step_rc = rc == SQLITE_OK ? SQLITE_ROW : rc;
    while (rc == SQLITE_OK && (step_rc = sqlite3_step(stmt)) == SQLITE_ROW) {
        if (count == capacity) {
            int next = capacity == 0 ? 8 : capacity * 2;
            void *resized = realloc(principals,
                                    (size_t)next * sizeof(*principals));
            if (!resized) {
                rc = SQLITE_NOMEM;
                break;
            }
            principals = resized;
            capacity = next;
        }
        legacy_principal_t *principal = &principals[count++];
        memset(principal, 0, sizeof(*principal));
        principal->user_id = sqlite3_column_int64(stmt, 0);
        principal->role = (user_role_t)sqlite3_column_int(stmt, 1);
        const char *tags = (const char *)sqlite3_column_text(stmt, 2);
        principal->has_allowed_tags = tags && tags[0] != '\0';
        safe_strcpy(principal->allowed_tags, tags ? tags : "",
                    sizeof(principal->allowed_tags), 0);
    }
    if (stmt) sqlite3_finalize(stmt);
    if (rc != SQLITE_OK || step_rc != SQLITE_DONE) {
        free(principals);
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        pthread_mutex_unlock(mutex);
        return -1;
    }
    if (count == 0) {
        free(principals);
        bool committed =
            sqlite3_exec(db, "COMMIT;", NULL, NULL, NULL) == SQLITE_OK;
        if (!committed) sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        pthread_mutex_unlock(mutex);
        return committed ? 0 : -1;
    }

    const char *insert_sql =
        "INSERT INTO authz_grants "
        "(uuid,user_id,role_uuid,scope_type,selector_json,collection_uuid) "
        "VALUES ("
        "lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || "
        "substr(hex(randomblob(2)),2) || '-' || "
        "substr('89ab',(abs(random()) % 4) + 1,1) || "
        "substr(hex(randomblob(2)),2) || '-' || hex(randomblob(6))),"
        "?,?,?,?,NULL);";
    bool success = true;
    for (int i = 0; success && i < count; i++) {
        const char *role_uuid = compatibility_role_uuid(principals[i].role);
        if (!role_uuid) {
            success = false;
            break;
        }
        /* Grants attached to a legacy-mode user were never an active access
         * boundary. Do not trust stale/pre-authored grants during upgrade:
         * replacing them is the only way to prove migration cannot widen the
         * currently active role + allowed_tags compatibility policy. */
        stmt = NULL;
        rc = sqlite3_prepare_v2(
            db, "DELETE FROM authz_grants WHERE user_id=?;", -1, &stmt,
            NULL);
        if (rc == SQLITE_OK) {
            sqlite3_bind_int64(stmt, 1, principals[i].user_id);
            rc = sqlite3_step(stmt);
        }
        if (stmt) sqlite3_finalize(stmt);
        if (rc != SQLITE_DONE) {
            success = false;
            break;
        }

        char *selector = NULL;
        if (principals[i].has_allowed_tags) {
            selector = legacy_tag_selector_locked(
                db, principals[i].allowed_tags);
            if (!selector) {
                success = false;
                break;
            }
        }
        stmt = NULL;
        rc = sqlite3_prepare_v2(db, insert_sql, -1, &stmt, NULL);
        if (rc == SQLITE_OK) {
            sqlite3_bind_int64(stmt, 1, principals[i].user_id);
            sqlite3_bind_text(stmt, 2, role_uuid, -1, SQLITE_STATIC);
            sqlite3_bind_text(stmt, 3, selector ? "selector" : "all", -1,
                              SQLITE_STATIC);
            if (selector) {
                sqlite3_bind_text(stmt, 4, selector, -1,
                                  SQLITE_TRANSIENT);
            } else {
                sqlite3_bind_null(stmt, 4);
            }
            rc = sqlite3_step(stmt);
        }
        if (stmt) sqlite3_finalize(stmt);
        if (rc != SQLITE_DONE) success = false;
        free(selector);
        if (!success) break;

        stmt = NULL;
        rc = sqlite3_prepare_v2(
            db,
            "UPDATE users SET authorization_mode='policy',allowed_tags=NULL,"
            "updated_at=strftime('%s','now') WHERE id=? AND "
            "authorization_mode='legacy';",
            -1, &stmt, NULL);
        if (rc == SQLITE_OK) {
            sqlite3_bind_int64(stmt, 1, principals[i].user_id);
            rc = sqlite3_step(stmt);
        }
        if (stmt) sqlite3_finalize(stmt);
        if (rc != SQLITE_DONE || sqlite3_changes(db) != 1) success = false;
    }
    free(principals);

    bool committed = finish_policy_change(db, success);
    pthread_mutex_unlock(mutex);
    if (!committed) return -1;
    if (migrated_count) *migrated_count = count;
    if (count > 0) {
        log_info("Migrated %d legacy authorization principal(s) to policy grants",
                 count);
    }
    return 0;
}
