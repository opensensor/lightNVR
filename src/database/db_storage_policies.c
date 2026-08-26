#define _POSIX_C_SOURCE 200809L

#include "database/db_storage_policies.h"

#include <cjson/cJSON.h>
#include <ctype.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdarg.h>
#include <stdatomic.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "core/camera_selector.h"
#include "database/db_core.h"
#include "database/db_storage_pools.h"
#include "utils/strings.h"

#define STORAGE_POLICY_SELECT_FIELDS \
    "uuid,name,enabled,priority,selector_json,primary_target_uuid," \
    "fallback_mode,COALESCE(fallback_target_uuid,''),revision," \
    "created_at,updated_at,COALESCE(primary_pool_uuid,'')," \
    "minimum_retention_days,desired_retention_days,maximum_retention_days," \
    "required_copy_count,COALESCE(replication_pool_uuid,'')," \
    "migration_after_days,COALESCE(migration_target_uuid,''),pressure_priority"

static atomic_uint_fast64_t policy_generation = 1;

static void set_error(char *error, size_t error_size,
                      const char *format, ...) {
    if (!error || error_size == 0 || error[0] != '\0') return;
    va_list arguments;
    va_start(arguments, format);
    vsnprintf(error, error_size, format, arguments);
    va_end(arguments);
}

static void copy_column(char *destination, size_t destination_size,
                        sqlite3_stmt *statement, int column) {
    const char *value = (const char *)sqlite3_column_text(statement, column);
    safe_strcpy(destination, value ? value : "", destination_size, 0);
}

static void populate(sqlite3_stmt *statement, storage_policy_t *policy) {
    memset(policy, 0, sizeof(*policy));
    copy_column(policy->uuid, sizeof(policy->uuid), statement, 0);
    copy_column(policy->name, sizeof(policy->name), statement, 1);
    policy->enabled = sqlite3_column_int(statement, 2) != 0;
    policy->priority = sqlite3_column_int(statement, 3);
    copy_column(policy->selector_json, sizeof(policy->selector_json),
                statement, 4);
    copy_column(policy->primary_target_uuid,
                sizeof(policy->primary_target_uuid), statement, 5);
    copy_column(policy->fallback_mode, sizeof(policy->fallback_mode),
                statement, 6);
    copy_column(policy->fallback_target_uuid,
                sizeof(policy->fallback_target_uuid), statement, 7);
    policy->revision = sqlite3_column_int64(statement, 8);
    policy->created_at = sqlite3_column_int64(statement, 9);
    policy->updated_at = sqlite3_column_int64(statement, 10);
    copy_column(policy->primary_pool_uuid,
                sizeof(policy->primary_pool_uuid), statement, 11);
    policy->minimum_retention_days = sqlite3_column_int(statement, 12);
    policy->desired_retention_days = sqlite3_column_int(statement, 13);
    policy->maximum_retention_days = sqlite3_column_int(statement, 14);
    policy->required_copy_count = sqlite3_column_int(statement, 15);
    copy_column(policy->replication_pool_uuid,
                sizeof(policy->replication_pool_uuid), statement, 16);
    policy->migration_after_days = sqlite3_column_int(statement, 17);
    copy_column(policy->migration_target_uuid,
                sizeof(policy->migration_target_uuid), statement, 18);
    policy->pressure_priority = sqlite3_column_int(statement, 19);
}

static bool valid_name(const char *input, char *normalized, size_t size) {
    if (!input || copy_trimmed_value(normalized, size, input, 0) == 0) {
        return false;
    }
    for (const unsigned char *cursor = (const unsigned char *)normalized;
         *cursor; cursor++) {
        if (iscntrl(*cursor)) return false;
    }
    return true;
}

static bool normalize_selector(storage_policy_t *policy, char *error,
                               size_t error_size) {
    cJSON *json = cJSON_Parse(policy->selector_json);
    if (!json) {
        set_error(error, error_size, "selector must be valid JSON");
        return false;
    }
    char selector_error[FLEET_SELECTOR_ERROR_MAX] = {0};
    fleet_selector_t *selector = fleet_selector_parse(
        json, selector_error, sizeof(selector_error));
    if (!selector) {
        set_error(error, error_size, "%s",
                  selector_error[0] ? selector_error : "invalid selector");
        cJSON_Delete(json);
        return false;
    }
    char *canonical = cJSON_PrintUnformatted(json);
    bool valid = canonical &&
        strnlen(canonical, sizeof(policy->selector_json)) <
            sizeof(policy->selector_json);
    if (valid) {
        safe_strcpy(policy->selector_json, canonical,
                    sizeof(policy->selector_json), 0);
    } else {
        set_error(error, error_size, "selector exceeds %d bytes",
                  STORAGE_POLICY_SELECTOR_MAX - 1);
    }
    free(canonical);
    fleet_selector_free(selector);
    cJSON_Delete(json);
    return valid;
}

db_storage_policy_result_t db_storage_policy_validate(
    storage_policy_t *policy, char *error, size_t error_size) {
    if (error && error_size > 0) error[0] = '\0';
    if (!policy) {
        set_error(error, error_size, "storage policy is required");
        return DB_STORAGE_POLICY_INVALID;
    }
    char normalized_name[STORAGE_POLICY_NAME_MAX];
    if (!valid_name(policy->name, normalized_name,
                    sizeof(normalized_name))) {
        set_error(error, error_size, "policy name cannot be blank");
        return DB_STORAGE_POLICY_INVALID;
    }
    safe_strcpy(policy->name, normalized_name, sizeof(policy->name), 0);
    if (policy->priority < -1000000 || policy->priority > 1000000) {
        set_error(error, error_size,
                  "priority must be between -1000000 and 1000000");
        return DB_STORAGE_POLICY_INVALID;
    }
    if (!normalize_selector(policy, error, error_size)) {
        return DB_STORAGE_POLICY_INVALID;
    }
    storage_target_t target;
    if (!lightnvr_uuid_is_valid(policy->primary_target_uuid) ||
        db_storage_target_get(policy->primary_target_uuid, &target) !=
            DB_STORAGE_TARGET_OK) {
        set_error(error, error_size, "primary target does not exist");
        return DB_STORAGE_POLICY_INVALID;
    }
    if (policy->primary_pool_uuid[0]) {
        storage_pool_t pool;
        if (!lightnvr_uuid_is_valid(policy->primary_pool_uuid) ||
            db_storage_pool_get(policy->primary_pool_uuid, &pool) !=
                DB_STORAGE_POOL_OK) {
            set_error(error, error_size, "primary pool does not exist");
            return DB_STORAGE_POLICY_INVALID;
        }
    }
    bool named_fallback = strcmp(policy->fallback_mode, "target") == 0;
    bool valid_mode = named_fallback ||
        strcmp(policy->fallback_mode, "default") == 0 ||
        strcmp(policy->fallback_mode, "pause") == 0 ||
        strcmp(policy->fallback_mode, "fail") == 0;
    if (!valid_mode) {
        set_error(error, error_size,
                  "fallback_mode must be default, target, pause, or fail");
        return DB_STORAGE_POLICY_INVALID;
    }
    if (named_fallback) {
        if (!lightnvr_uuid_is_valid(policy->fallback_target_uuid) ||
            strcmp(policy->fallback_target_uuid,
                   policy->primary_target_uuid) == 0 ||
            db_storage_target_get(policy->fallback_target_uuid, &target) !=
                DB_STORAGE_TARGET_OK) {
            set_error(error, error_size,
                      "fallback target must exist and differ from primary");
            return DB_STORAGE_POLICY_INVALID;
        }
    } else if (policy->fallback_target_uuid[0] != '\0') {
        set_error(error, error_size,
                  "fallback_target_uuid is only valid for target fallback");
        return DB_STORAGE_POLICY_INVALID;
    }
    if (policy->minimum_retention_days < 0 ||
        policy->desired_retention_days < 0 ||
        policy->maximum_retention_days < 0 ||
        policy->minimum_retention_days > 36500 ||
        policy->desired_retention_days > 36500 ||
        policy->maximum_retention_days > 36500 ||
        (policy->desired_retention_days > 0 &&
         policy->minimum_retention_days > policy->desired_retention_days) ||
        (policy->maximum_retention_days > 0 &&
         ((policy->desired_retention_days > 0 &&
           policy->desired_retention_days > policy->maximum_retention_days) ||
          policy->minimum_retention_days > policy->maximum_retention_days))) {
        set_error(error, error_size,
                  "retention days must satisfy minimum <= desired <= maximum");
        return DB_STORAGE_POLICY_INVALID;
    }
    if (policy->required_copy_count == 0) policy->required_copy_count = 1;
    if (policy->required_copy_count < 1 || policy->required_copy_count > 8) {
        set_error(error, error_size, "required_copy_count must be 1-8");
        return DB_STORAGE_POLICY_INVALID;
    }
    if (policy->required_copy_count > 1) {
        storage_pool_t pool;
        if (!lightnvr_uuid_is_valid(policy->replication_pool_uuid) ||
            db_storage_pool_get(policy->replication_pool_uuid, &pool) !=
                DB_STORAGE_POOL_OK) {
            set_error(error, error_size,
                      "replication pool cannot satisfy required copy count");
            return DB_STORAGE_POLICY_INVALID;
        }
        int usable_members = pool.member_count;
        bool may_contain_primary = false;
        if (policy->primary_pool_uuid[0]) {
            storage_pool_t primary_pool;
            if (db_storage_pool_get(policy->primary_pool_uuid, &primary_pool) ==
                    DB_STORAGE_POOL_OK) {
                for (int replica = 0; replica < pool.member_count; replica++) {
                    for (int primary = 0;
                         primary < primary_pool.member_count; primary++) {
                        if (strcmp(pool.members[replica].target_uuid,
                                   primary_pool.members[primary].target_uuid) == 0) {
                            may_contain_primary = true;
                        }
                    }
                }
            }
        } else {
            for (int index = 0; index < pool.member_count; index++) {
                if (strcmp(pool.members[index].target_uuid,
                           policy->primary_target_uuid) == 0) {
                    usable_members--;
                }
            }
        }
        if ((policy->primary_pool_uuid[0] && may_contain_primary &&
             pool.member_count < policy->required_copy_count) ||
            (!policy->primary_pool_uuid[0] &&
             usable_members < policy->required_copy_count - 1) ||
            (policy->primary_pool_uuid[0] && !may_contain_primary &&
             pool.member_count < policy->required_copy_count - 1)) {
            set_error(error, error_size,
                      "replication pool cannot satisfy distinct copy count");
            return DB_STORAGE_POLICY_INVALID;
        }
    } else if (policy->replication_pool_uuid[0]) {
        set_error(error, error_size,
                  "replication_pool_uuid requires more than one copy");
        return DB_STORAGE_POLICY_INVALID;
    }
    if (policy->migration_after_days < 0 ||
        policy->migration_after_days > 36500) {
        set_error(error, error_size, "migration_after_days must be 0-36500");
        return DB_STORAGE_POLICY_INVALID;
    }
    if (policy->migration_after_days > 0) {
        if (!lightnvr_uuid_is_valid(policy->migration_target_uuid) ||
            db_storage_target_get(policy->migration_target_uuid, &target) !=
                DB_STORAGE_TARGET_OK) {
            set_error(error, error_size, "migration target does not exist");
            return DB_STORAGE_POLICY_INVALID;
        }
        if (strcmp(policy->migration_target_uuid,
                   policy->primary_target_uuid) == 0) {
            set_error(error, error_size,
                      "migration target must differ from initial target");
            return DB_STORAGE_POLICY_INVALID;
        }
        const char *pool_uuids[] = {
            policy->primary_pool_uuid, policy->replication_pool_uuid
        };
        for (size_t pool_index = 0; pool_index < 2; pool_index++) {
            if (!pool_uuids[pool_index][0]) continue;
            storage_pool_t pool;
            if (db_storage_pool_get(pool_uuids[pool_index], &pool) !=
                DB_STORAGE_POOL_OK) continue;
            for (int member = 0; member < pool.member_count; member++) {
                if (strcmp(pool.members[member].target_uuid,
                           policy->migration_target_uuid) == 0) {
                    set_error(error, error_size,
                        "migration target must not be an initial or replica pool member");
                    return DB_STORAGE_POLICY_INVALID;
                }
            }
        }
    } else if (policy->migration_target_uuid[0]) {
        set_error(error, error_size,
                  "migration_target_uuid requires migration_after_days");
        return DB_STORAGE_POLICY_INVALID;
    }
    if (policy->pressure_priority < -1000000 ||
        policy->pressure_priority > 1000000) {
        set_error(error, error_size,
                  "pressure_priority must be between -1000000 and 1000000");
        return DB_STORAGE_POLICY_INVALID;
    }
    return DB_STORAGE_POLICY_OK;
}

static db_storage_policy_result_t get_locked(
    sqlite3 *db, const char *uuid, storage_policy_t *policy) {
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        db, "SELECT " STORAGE_POLICY_SELECT_FIELDS
            " FROM storage_policies WHERE uuid=? LIMIT 1;",
        -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    db_storage_policy_result_t outcome = DB_STORAGE_POLICY_NOT_FOUND;
    if (result == SQLITE_ROW) {
        populate(statement, policy);
        outcome = DB_STORAGE_POLICY_OK;
    } else if (result != SQLITE_DONE) {
        outcome = DB_STORAGE_POLICY_ERROR;
    }
    if (statement) sqlite3_finalize(statement);
    return outcome;
}

int db_storage_policy_count(void) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int count = -1;
    if (sqlite3_prepare_v2(db, "SELECT count(*) FROM storage_policies;", -1,
                           &statement, NULL) == SQLITE_OK &&
        sqlite3_step(statement) == SQLITE_ROW) {
        count = sqlite3_column_int(statement, 0);
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return count;
}

int db_storage_policy_list(storage_policy_t *policies, int max_count,
                           bool enabled_only) {
    if (!policies || max_count <= 0) return -1;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    const char *sql = enabled_only
        ? "SELECT " STORAGE_POLICY_SELECT_FIELDS
          " FROM storage_policies WHERE enabled=1"
          " ORDER BY priority DESC,name COLLATE NOCASE,uuid;"
        : "SELECT " STORAGE_POLICY_SELECT_FIELDS
          " FROM storage_policies"
          " ORDER BY priority DESC,name COLLATE NOCASE,uuid;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    int count = 0;
    if (result == SQLITE_OK) {
        while (count < max_count &&
               (result = sqlite3_step(statement)) == SQLITE_ROW) {
            populate(statement, &policies[count++]);
        }
    }
    if (result != SQLITE_DONE && result != SQLITE_ROW) count = -1;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return count;
}

db_storage_policy_result_t db_storage_policy_get(
    const char *uuid, storage_policy_t *policy) {
    if (!lightnvr_uuid_is_valid(uuid) || !policy) {
        return DB_STORAGE_POLICY_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_POLICY_ERROR;
    pthread_mutex_lock(mutex);
    db_storage_policy_result_t result = get_locked(db, uuid, policy);
    pthread_mutex_unlock(mutex);
    return result;
}

static db_storage_policy_result_t sqlite_result(sqlite3 *db, int result) {
    if (result == SQLITE_CONSTRAINT ||
        sqlite3_extended_errcode(db) == SQLITE_CONSTRAINT_UNIQUE) {
        return DB_STORAGE_POLICY_CONFLICT;
    }
    return DB_STORAGE_POLICY_ERROR;
}

db_storage_policy_result_t db_storage_policy_create(storage_policy_t *policy) {
    char error[FLEET_SELECTOR_ERROR_MAX] = {0};
    if (db_storage_policy_validate(policy, error, sizeof(error)) !=
        DB_STORAGE_POLICY_OK) {
        return DB_STORAGE_POLICY_INVALID;
    }
    if (lightnvr_uuid_generate_v4(policy->uuid) != 0) {
        return DB_STORAGE_POLICY_ERROR;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_POLICY_ERROR;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *count_statement = NULL;
    int count = -1;
    if (sqlite3_prepare_v2(db, "SELECT count(*) FROM storage_policies;", -1,
                           &count_statement, NULL) == SQLITE_OK &&
        sqlite3_step(count_statement) == SQLITE_ROW) {
        count = sqlite3_column_int(count_statement, 0);
    }
    if (count_statement) sqlite3_finalize(count_statement);
    if (count < 0 || count >= STORAGE_POLICY_MAX_COUNT) {
        pthread_mutex_unlock(mutex);
        return count < 0 ? DB_STORAGE_POLICY_ERROR : DB_STORAGE_POLICY_LIMIT;
    }
    const char *sql =
        "INSERT INTO storage_policies(uuid,name,enabled,priority,selector_json,"
        "primary_target_uuid,fallback_mode,fallback_target_uuid,primary_pool_uuid,"
        "minimum_retention_days,desired_retention_days,maximum_retention_days,"
        "required_copy_count,replication_pool_uuid,migration_after_days,"
        "migration_target_uuid,pressure_priority)"
        " VALUES(?,?,?,?,?,?,?,NULLIF(?,''),NULLIF(?,''),?,?,?,?,NULLIF(?,''),?,"
        "NULLIF(?,''),?);";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, policy->uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, policy->name, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 3, policy->enabled ? 1 : 0);
        sqlite3_bind_int(statement, 4, policy->priority);
        sqlite3_bind_text(statement, 5, policy->selector_json, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 6, policy->primary_target_uuid, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 7, policy->fallback_mode, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 8, policy->fallback_target_uuid, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 9, policy->primary_pool_uuid, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 10, policy->minimum_retention_days);
        sqlite3_bind_int(statement, 11, policy->desired_retention_days);
        sqlite3_bind_int(statement, 12, policy->maximum_retention_days);
        sqlite3_bind_int(statement, 13, policy->required_copy_count);
        sqlite3_bind_text(statement, 14, policy->replication_pool_uuid, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 15, policy->migration_after_days);
        sqlite3_bind_text(statement, 16, policy->migration_target_uuid, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 17, policy->pressure_priority);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    db_storage_policy_result_t outcome = result == SQLITE_DONE
        ? DB_STORAGE_POLICY_OK : sqlite_result(db, result);
    if (outcome == DB_STORAGE_POLICY_OK) {
        outcome = get_locked(db, policy->uuid, policy);
    }
    pthread_mutex_unlock(mutex);
    if (outcome == DB_STORAGE_POLICY_OK) {
        atomic_fetch_add_explicit(&policy_generation, 1,
                                  memory_order_relaxed);
    }
    return outcome;
}

db_storage_policy_result_t db_storage_policy_update(
    storage_policy_t *policy, int64_t expected_revision) {
    char error[FLEET_SELECTOR_ERROR_MAX] = {0};
    if (!lightnvr_uuid_is_valid(policy ? policy->uuid : NULL) ||
        expected_revision < 1 ||
        db_storage_policy_validate(policy, error, sizeof(error)) !=
            DB_STORAGE_POLICY_OK) {
        return DB_STORAGE_POLICY_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_POLICY_ERROR;
    const char *sql =
        "UPDATE storage_policies SET name=?,enabled=?,priority=?,"
        "selector_json=?,primary_target_uuid=?,fallback_mode=?,"
        "fallback_target_uuid=NULLIF(?,''),primary_pool_uuid=NULLIF(?,''),"
        "minimum_retention_days=?,desired_retention_days=?,maximum_retention_days=?,"
        "required_copy_count=?,replication_pool_uuid=NULLIF(?,''),"
        "migration_after_days=?,migration_target_uuid=NULLIF(?,''),"
        "pressure_priority=?,revision=revision+1,"
        "updated_at=strftime('%s','now') WHERE uuid=? AND revision=?;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, policy->name, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 2, policy->enabled ? 1 : 0);
        sqlite3_bind_int(statement, 3, policy->priority);
        sqlite3_bind_text(statement, 4, policy->selector_json, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 5, policy->primary_target_uuid, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 6, policy->fallback_mode, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 7, policy->fallback_target_uuid, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 8, policy->primary_pool_uuid, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 9, policy->minimum_retention_days);
        sqlite3_bind_int(statement, 10, policy->desired_retention_days);
        sqlite3_bind_int(statement, 11, policy->maximum_retention_days);
        sqlite3_bind_int(statement, 12, policy->required_copy_count);
        sqlite3_bind_text(statement, 13, policy->replication_pool_uuid, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 14, policy->migration_after_days);
        sqlite3_bind_text(statement, 15, policy->migration_target_uuid, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 16, policy->pressure_priority);
        sqlite3_bind_text(statement, 17, policy->uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 18, expected_revision);
        result = sqlite3_step(statement);
    }
    db_storage_policy_result_t outcome;
    if (result != SQLITE_DONE) {
        outcome = sqlite_result(db, result);
    } else if (sqlite3_changes(db) == 0) {
        storage_policy_t existing;
        outcome = get_locked(db, policy->uuid, &existing) ==
                DB_STORAGE_POLICY_OK
            ? DB_STORAGE_POLICY_STALE : DB_STORAGE_POLICY_NOT_FOUND;
    } else {
        outcome = get_locked(db, policy->uuid, policy);
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    if (outcome == DB_STORAGE_POLICY_OK) {
        atomic_fetch_add_explicit(&policy_generation, 1,
                                  memory_order_relaxed);
    }
    return outcome;
}

db_storage_policy_result_t db_storage_policy_delete(
    const char *uuid, int64_t expected_revision) {
    if (!lightnvr_uuid_is_valid(uuid) || expected_revision < 1) {
        return DB_STORAGE_POLICY_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_POLICY_ERROR;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        db, "DELETE FROM storage_policies WHERE uuid=? AND revision=?;", -1,
        &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 2, expected_revision);
        result = sqlite3_step(statement);
    }
    db_storage_policy_result_t outcome;
    if (result != SQLITE_DONE) {
        outcome = DB_STORAGE_POLICY_ERROR;
    } else if (sqlite3_changes(db) == 0) {
        storage_policy_t existing;
        outcome = get_locked(db, uuid, &existing) == DB_STORAGE_POLICY_OK
            ? DB_STORAGE_POLICY_STALE : DB_STORAGE_POLICY_NOT_FOUND;
    } else {
        outcome = DB_STORAGE_POLICY_OK;
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    if (outcome == DB_STORAGE_POLICY_OK) {
        atomic_fetch_add_explicit(&policy_generation, 1,
                                  memory_order_relaxed);
    }
    return outcome;
}

uint64_t db_storage_policy_generation(void) {
    return atomic_load_explicit(&policy_generation, memory_order_relaxed);
}
