#define _POSIX_C_SOURCE 200809L

#include "database/db_storage_pools.h"

#include <ctype.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdio.h>
#include <string.h>

#include "database/db_core.h"
#include "utils/strings.h"
#include "utils/uuid.h"

#define STORAGE_POOL_SELECT_FIELDS \
    "uuid,name,strategy,enabled,allocation_cursor,revision,created_at,updated_at"

static void copy_column(char *destination, size_t destination_size,
                        sqlite3_stmt *statement, int column) {
    const char *value = (const char *)sqlite3_column_text(statement, column);
    safe_strcpy(destination, value ? value : "", destination_size, 0);
}

static db_storage_pool_result_t load_members_locked(
    sqlite3 *db, storage_pool_t *pool) {
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        db, "SELECT target_uuid,position,weight FROM storage_pool_members "
            "WHERE pool_uuid=? ORDER BY position,target_uuid;", -1,
        &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, pool->uuid, -1, SQLITE_TRANSIENT);
    }
    pool->member_count = 0;
    while (result == SQLITE_OK || result == SQLITE_ROW) {
        result = sqlite3_step(statement);
        if (result != SQLITE_ROW) break;
        if (pool->member_count >= STORAGE_POOL_MEMBER_MAX) {
            sqlite3_finalize(statement);
            return DB_STORAGE_POOL_LIMIT;
        }
        storage_pool_member_t *member =
            &pool->members[pool->member_count++];
        copy_column(member->target_uuid, sizeof(member->target_uuid),
                    statement, 0);
        member->position = sqlite3_column_int(statement, 1);
        member->weight = sqlite3_column_int(statement, 2);
    }
    if (statement) sqlite3_finalize(statement);
    return result == SQLITE_DONE ? DB_STORAGE_POOL_OK : DB_STORAGE_POOL_ERROR;
}

static void populate(sqlite3_stmt *statement, storage_pool_t *pool) {
    memset(pool, 0, sizeof(*pool));
    copy_column(pool->uuid, sizeof(pool->uuid), statement, 0);
    copy_column(pool->name, sizeof(pool->name), statement, 1);
    copy_column(pool->strategy, sizeof(pool->strategy), statement, 2);
    pool->enabled = sqlite3_column_int(statement, 3) != 0;
    pool->allocation_cursor = sqlite3_column_int(statement, 4);
    pool->revision = sqlite3_column_int64(statement, 5);
    pool->created_at = sqlite3_column_int64(statement, 6);
    pool->updated_at = sqlite3_column_int64(statement, 7);
}

static db_storage_pool_result_t get_locked(
    sqlite3 *db, const char *uuid, storage_pool_t *pool) {
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        db, "SELECT " STORAGE_POOL_SELECT_FIELDS
            " FROM storage_pools WHERE uuid=? LIMIT 1;", -1,
        &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (result != SQLITE_ROW) {
        if (statement) sqlite3_finalize(statement);
        return result == SQLITE_DONE ? DB_STORAGE_POOL_NOT_FOUND
                                     : DB_STORAGE_POOL_ERROR;
    }
    populate(statement, pool);
    sqlite3_finalize(statement);
    return load_members_locked(db, pool);
}

db_storage_pool_result_t db_storage_pool_validate(
    storage_pool_t *pool, char *error, size_t error_size) {
    if (error && error_size) error[0] = '\0';
    if (!pool) return DB_STORAGE_POOL_INVALID;
    char name[STORAGE_POOL_NAME_MAX];
    if (copy_trimmed_value(name, sizeof(name), pool->name, 0) == 0) {
        if (error && error_size) snprintf(error, error_size,
                                          "pool name cannot be blank");
        return DB_STORAGE_POOL_INVALID;
    }
    for (const unsigned char *cursor = (const unsigned char *)name;
         *cursor; cursor++) {
        if (iscntrl(*cursor)) return DB_STORAGE_POOL_INVALID;
    }
    safe_strcpy(pool->name, name, sizeof(pool->name), 0);
    if (strcmp(pool->strategy, "most_free") != 0 &&
        strcmp(pool->strategy, "round_robin") != 0 &&
        strcmp(pool->strategy, "priority") != 0) {
        if (error && error_size) snprintf(error, error_size,
            "strategy must be most_free, round_robin, or priority");
        return DB_STORAGE_POOL_INVALID;
    }
    if (pool->member_count < 1 ||
        pool->member_count > STORAGE_POOL_MEMBER_MAX) {
        if (error && error_size) snprintf(error, error_size,
                                          "pool requires at least one target");
        return DB_STORAGE_POOL_INVALID;
    }
    for (int index = 0; index < pool->member_count; index++) {
        storage_pool_member_t *member = &pool->members[index];
        storage_target_t target;
        if (!lightnvr_uuid_is_valid(member->target_uuid) ||
            db_storage_target_get(member->target_uuid, &target) !=
                DB_STORAGE_TARGET_OK) {
            if (error && error_size) snprintf(error, error_size,
                                               "pool target does not exist");
            return DB_STORAGE_POOL_INVALID;
        }
        if (member->weight < 1 || member->weight > 1000) {
            if (error && error_size) snprintf(error, error_size,
                                               "member weight must be 1-1000");
            return DB_STORAGE_POOL_INVALID;
        }
        member->position = index;
        for (int prior = 0; prior < index; prior++) {
            if (strcmp(member->target_uuid,
                       pool->members[prior].target_uuid) == 0) {
                if (error && error_size) snprintf(error, error_size,
                                                  "pool targets must be unique");
                return DB_STORAGE_POOL_INVALID;
            }
        }
    }
    return DB_STORAGE_POOL_OK;
}

int db_storage_pool_count(void) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int count = -1;
    if (sqlite3_prepare_v2(db, "SELECT count(*) FROM storage_pools;", -1,
                           &statement, NULL) == SQLITE_OK &&
        sqlite3_step(statement) == SQLITE_ROW) {
        count = sqlite3_column_int(statement, 0);
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return count;
}

int db_storage_pool_list(storage_pool_t *pools, int max_count) {
    if (!pools || max_count <= 0) return -1;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        db, "SELECT " STORAGE_POOL_SELECT_FIELDS
            " FROM storage_pools ORDER BY name COLLATE NOCASE,uuid;", -1,
        &statement, NULL);
    int count = 0;
    while ((result == SQLITE_OK || result == SQLITE_ROW) &&
           count < max_count) {
        result = sqlite3_step(statement);
        if (result != SQLITE_ROW) break;
        populate(statement, &pools[count]);
        if (load_members_locked(db, &pools[count]) != DB_STORAGE_POOL_OK) {
            count = -1;
            break;
        }
        count++;
    }
    if (count >= 0 && result != SQLITE_DONE && result != SQLITE_ROW) count = -1;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return count;
}

db_storage_pool_result_t db_storage_pool_get(
    const char *uuid, storage_pool_t *pool) {
    if (!lightnvr_uuid_is_valid(uuid) || !pool) return DB_STORAGE_POOL_INVALID;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_POOL_ERROR;
    pthread_mutex_lock(mutex);
    db_storage_pool_result_t result = get_locked(db, uuid, pool);
    pthread_mutex_unlock(mutex);
    return result;
}

static bool write_members_locked(sqlite3 *db, const storage_pool_t *pool) {
    sqlite3_stmt *statement = NULL;
    if (sqlite3_prepare_v2(db,
            "INSERT INTO storage_pool_members(pool_uuid,target_uuid,position,weight)"
            " VALUES(?,?,?,?);", -1, &statement, NULL) != SQLITE_OK) {
        return false;
    }
    bool success = true;
    for (int index = 0; index < pool->member_count; index++) {
        sqlite3_reset(statement);
        sqlite3_clear_bindings(statement);
        sqlite3_bind_text(statement, 1, pool->uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, pool->members[index].target_uuid,
                          -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 3, index);
        sqlite3_bind_int(statement, 4, pool->members[index].weight);
        if (sqlite3_step(statement) != SQLITE_DONE) {
            success = false;
            break;
        }
    }
    sqlite3_finalize(statement);
    return success;
}

static db_storage_pool_result_t sqlite_result(sqlite3 *db, int result) {
    int code = sqlite3_extended_errcode(db);
    return result == SQLITE_CONSTRAINT || code == SQLITE_CONSTRAINT_UNIQUE ||
           code == SQLITE_CONSTRAINT_PRIMARYKEY
        ? DB_STORAGE_POOL_CONFLICT : DB_STORAGE_POOL_ERROR;
}

db_storage_pool_result_t db_storage_pool_create(storage_pool_t *pool) {
    if (db_storage_pool_validate(pool, NULL, 0) != DB_STORAGE_POOL_OK ||
        lightnvr_uuid_generate_v4(pool->uuid) != 0) {
        return DB_STORAGE_POOL_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_POOL_ERROR;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *count_statement = NULL;
    int count = -1;
    if (sqlite3_prepare_v2(db, "SELECT count(*) FROM storage_pools;", -1,
                           &count_statement, NULL) == SQLITE_OK &&
        sqlite3_step(count_statement) == SQLITE_ROW) {
        count = sqlite3_column_int(count_statement, 0);
    }
    if (count_statement) sqlite3_finalize(count_statement);
    if (count < 0 || count >= STORAGE_POOL_MAX_COUNT) {
        pthread_mutex_unlock(mutex);
        return count < 0 ? DB_STORAGE_POOL_ERROR : DB_STORAGE_POOL_LIMIT;
    }
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db,
        "INSERT INTO storage_pools(uuid,name,strategy,enabled) VALUES(?,?,?,?);",
        -1, &statement, NULL);
    if (result == SQLITE_OK &&
        sqlite3_exec(db, "BEGIN IMMEDIATE;", NULL, NULL, NULL) == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, pool->uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, pool->name, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 3, pool->strategy, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 4, pool->enabled ? 1 : 0);
        result = sqlite3_step(statement);
        if (result == SQLITE_DONE && write_members_locked(db, pool) &&
            sqlite3_exec(db, "COMMIT;", NULL, NULL, NULL) == SQLITE_OK) {
            result = SQLITE_DONE;
        } else {
            sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
            if (result == SQLITE_DONE) result = SQLITE_ERROR;
        }
    }
    if (statement) sqlite3_finalize(statement);
    db_storage_pool_result_t outcome = result == SQLITE_DONE
        ? get_locked(db, pool->uuid, pool) : sqlite_result(db, result);
    pthread_mutex_unlock(mutex);
    return outcome;
}

db_storage_pool_result_t db_storage_pool_update(
    storage_pool_t *pool, int64_t expected_revision) {
    if (!pool || !lightnvr_uuid_is_valid(pool->uuid) || expected_revision < 1 ||
        db_storage_pool_validate(pool, NULL, 0) != DB_STORAGE_POOL_OK) {
        return DB_STORAGE_POOL_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_POOL_ERROR;
    pthread_mutex_lock(mutex);
    bool began = sqlite3_exec(db, "BEGIN IMMEDIATE;", NULL, NULL, NULL) == SQLITE_OK;
    sqlite3_stmt *statement = NULL;
    int result = began ? sqlite3_prepare_v2(db,
        "UPDATE storage_pools SET name=?,strategy=?,enabled=?,revision=revision+1,"
        "updated_at=strftime('%s','now') WHERE uuid=? AND revision=?;",
        -1, &statement, NULL) : SQLITE_ERROR;
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, pool->name, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, pool->strategy, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 3, pool->enabled ? 1 : 0);
        sqlite3_bind_text(statement, 4, pool->uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 5, expected_revision);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (statement) sqlite3_finalize(statement);
    if (changed == 1) {
        statement = NULL;
        result = sqlite3_prepare_v2(db,
            "DELETE FROM storage_pool_members WHERE pool_uuid=?;", -1,
            &statement, NULL);
        if (result == SQLITE_OK) {
            sqlite3_bind_text(statement, 1, pool->uuid, -1, SQLITE_TRANSIENT);
            result = sqlite3_step(statement);
        }
        if (statement) sqlite3_finalize(statement);
        if (result == SQLITE_DONE && write_members_locked(db, pool) &&
            sqlite3_exec(db, "COMMIT;", NULL, NULL, NULL) == SQLITE_OK) {
            result = SQLITE_DONE;
        } else {
            sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
            if (result == SQLITE_DONE) result = SQLITE_ERROR;
        }
    } else if (began) {
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
    }
    db_storage_pool_result_t outcome;
    if (result != SQLITE_DONE) {
        outcome = sqlite_result(db, result);
    } else if (changed == 0) {
        storage_pool_t existing;
        outcome = get_locked(db, pool->uuid, &existing) == DB_STORAGE_POOL_OK
            ? DB_STORAGE_POOL_STALE : DB_STORAGE_POOL_NOT_FOUND;
    } else {
        outcome = get_locked(db, pool->uuid, pool);
    }
    pthread_mutex_unlock(mutex);
    return outcome;
}

db_storage_pool_result_t db_storage_pool_delete(
    const char *uuid, int64_t expected_revision) {
    if (!lightnvr_uuid_is_valid(uuid) || expected_revision < 1)
        return DB_STORAGE_POOL_INVALID;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_POOL_ERROR;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db,
        "DELETE FROM storage_pools WHERE uuid=? AND revision=?;", -1,
        &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 2, expected_revision);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (statement) sqlite3_finalize(statement);
    db_storage_pool_result_t outcome;
    if (result == SQLITE_CONSTRAINT ||
        sqlite3_extended_errcode(db) == SQLITE_CONSTRAINT_FOREIGNKEY) {
        outcome = DB_STORAGE_POOL_IN_USE;
    } else if (result != SQLITE_DONE) {
        outcome = DB_STORAGE_POOL_ERROR;
    } else if (changed == 0) {
        storage_pool_t existing;
        outcome = get_locked(db, uuid, &existing) == DB_STORAGE_POOL_OK
            ? DB_STORAGE_POOL_STALE : DB_STORAGE_POOL_NOT_FOUND;
    } else {
        outcome = DB_STORAGE_POOL_OK;
    }
    pthread_mutex_unlock(mutex);
    return outcome;
}

db_storage_pool_result_t db_storage_pool_allocate(
    const char *uuid, const char *exclude_target_uuid,
    storage_target_t *target) {
    storage_pool_t pool;
    db_storage_pool_result_t loaded = db_storage_pool_get(uuid, &pool);
    if (loaded != DB_STORAGE_POOL_OK || !target) return loaded;
    if (!pool.enabled) return DB_STORAGE_POOL_UNAVAILABLE;
    int selected = -1;
    uint64_t best_free = 0;
    int start = strcmp(pool.strategy, "round_robin") == 0
        ? pool.allocation_cursor % pool.member_count : 0;
    for (int offset = 0; offset < pool.member_count; offset++) {
        int index = (start + offset) % pool.member_count;
        storage_target_t candidate;
        if (exclude_target_uuid && exclude_target_uuid[0] &&
            strcmp(pool.members[index].target_uuid, exclude_target_uuid) == 0)
            continue;
        if (db_storage_target_get(pool.members[index].target_uuid, &candidate) !=
                DB_STORAGE_TARGET_OK || !candidate.enabled ||
            strcmp(candidate.health_status, "healthy") != 0 ||
            (candidate.mount_required &&
             !db_storage_target_mount_guard_active(&candidate)) ||
            candidate.available_bytes <= candidate.reserve_bytes) continue;
        uint64_t free_bytes = candidate.available_bytes - candidate.reserve_bytes;
        if (selected < 0 || strcmp(pool.strategy, "most_free") != 0 ||
            free_bytes > best_free) {
            selected = index;
            best_free = free_bytes;
            *target = candidate;
            if (strcmp(pool.strategy, "most_free") != 0) break;
        }
    }
    if (selected < 0) return DB_STORAGE_POOL_UNAVAILABLE;
    if (strcmp(pool.strategy, "round_robin") == 0) {
        sqlite3 *db = get_db_handle();
        pthread_mutex_t *mutex = get_db_mutex();
        if (db && mutex) {
            pthread_mutex_lock(mutex);
            sqlite3_stmt *statement = NULL;
            if (sqlite3_prepare_v2(db,
                "UPDATE storage_pools SET allocation_cursor=? WHERE uuid=?;",
                -1, &statement, NULL) == SQLITE_OK) {
                sqlite3_bind_int(statement, 1,
                                 (selected + 1) % pool.member_count);
                sqlite3_bind_text(statement, 2, pool.uuid, -1,
                                  SQLITE_TRANSIENT);
                sqlite3_step(statement);
            }
            if (statement) sqlite3_finalize(statement);
            pthread_mutex_unlock(mutex);
        }
    }
    return DB_STORAGE_POOL_OK;
}
