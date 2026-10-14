#define _POSIX_C_SOURCE 200809L

#include "database/db_storage_lifecycle.h"

#include <pthread.h>
#include <sqlite3.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "core/logger.h"

#include "database/db_core.h"
#include "database/db_storage_migrations.h"
#include "database/db_storage_pools.h"
#include "database/db_storage_targets.h"
#include "storage/storage_deletion.h"
#include "utils/strings.h"
#include "utils/uuid.h"

#define LIFECYCLE_CANDIDATE_MAX 128

typedef struct {
    uint64_t recording_id;
    char source_target_uuid[LIGHTNVR_UUID_STRING_SIZE];
    int required_copy_count;
    int current_copy_count;
    char replication_pool_uuid[LIGHTNVR_UUID_STRING_SIZE];
    int migration_after_days;
    bool keep_hot;
    bool archive_due;
    char migration_target_uuid[LIGHTNVR_UUID_STRING_SIZE];
} lifecycle_candidate_t;

/* Both tables matter: deleted policies can still govern captured recordings. */
static int policies_present(sqlite3 *db) {
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db,
        "SELECT EXISTS(SELECT 1 FROM storage_policies) OR "
        "EXISTS(SELECT 1 FROM storage_policy_versions);", -1, &statement, NULL);
    int present = -1;
    if (result == SQLITE_OK && sqlite3_step(statement) == SQLITE_ROW)
        present = sqlite3_column_int(statement, 0);
    sqlite3_finalize(statement);
    return present;
}

/* No fallback to the shared connection: a failed background read must not
 * turn into an API-blocking scan. */
static int open_policy_reader(sqlite3 **db) {
    if (db_open_readonly_connection(db) != 0) return -1;
    int present = policies_present(*db);
    if (present <= 0) {
        db_close_readonly_connection(*db);
        *db = NULL;
    }
    return present;
}

static void copy_column(char *destination, size_t size,
                        sqlite3_stmt *statement, int column) {
    const char *value = (const char *)sqlite3_column_text(statement, column);
    safe_strcpy(destination, value ? value : "", size, 0);
}

static bool used_target(sqlite3 *db, uint64_t recording_id,
                        const char *target_uuid) {
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db,
        "SELECT 1 FROM recordings r WHERE r.id=? AND r.storage_target_uuid=? "
        "UNION ALL SELECT 1 FROM storage_recording_copies c "
        "WHERE c.recording_id=? AND c.target_uuid=? LIMIT 1;", -1,
        &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_int64(statement, 1, (sqlite3_int64)recording_id);
        sqlite3_bind_text(statement, 2, target_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 3, (sqlite3_int64)recording_id);
        sqlite3_bind_text(statement, 4, target_uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    return result == SQLITE_ROW;
}

static bool select_replica_target(const lifecycle_candidate_t *candidate,
                                  char uuid[LIGHTNVR_UUID_STRING_SIZE]) {
    storage_pool_t pool;
    if (db_storage_pool_get(candidate->replication_pool_uuid, &pool) !=
            DB_STORAGE_POOL_OK || !pool.enabled) return false;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    int selected = -1;
    uint64_t best_free = 0;
    int start = strcmp(pool.strategy, "round_robin") == 0
        ? pool.allocation_cursor % pool.member_count : 0;
    for (int offset = 0; offset < pool.member_count; offset++) {
        int index = (start + offset) % pool.member_count;
        storage_target_t target;
        if (db_storage_target_get(pool.members[index].target_uuid, &target) !=
                DB_STORAGE_TARGET_OK || !target.enabled ||
            strcmp(target.health_status, "healthy") != 0 ||
            (target.mount_required &&
             !db_storage_target_mount_guard_active(&target)) ||
            target.available_bytes <= target.reserve_bytes) continue;
        pthread_mutex_lock(mutex);
        bool used = used_target(db, candidate->recording_id, target.uuid);
        pthread_mutex_unlock(mutex);
        if (used) continue;
        uint64_t free_bytes = target.available_bytes - target.reserve_bytes;
        if (selected < 0 || strcmp(pool.strategy, "most_free") != 0 ||
            free_bytes > best_free) {
            selected = index;
            best_free = free_bytes;
            if (strcmp(pool.strategy, "most_free") != 0) break;
        }
    }
    if (selected < 0) return false;
    safe_strcpy(uuid, pool.members[selected].target_uuid,
                LIGHTNVR_UUID_STRING_SIZE, 0);
    return true;
}

static int lifecycle_schedule(int max_jobs) {
    if (max_jobs < 1) return 0;
    if (max_jobs > LIFECYCLE_CANDIDATE_MAX) max_jobs = LIFECYCLE_CANDIDATE_MAX;
    sqlite3 *db = NULL;
    int present = open_policy_reader(&db);
    if (present <= 0) return present;
    lifecycle_candidate_t candidates[LIFECYCLE_CANDIDATE_MAX];
    memset(candidates, 0, sizeof(candidates));
    const char *sql =
        "WITH eligible AS (SELECT r.id,COALESCE(r.storage_target_uuid,'') source,p.required_copy_count,"
        "(SELECT count(*) FROM storage_recording_copies c WHERE c.recording_id=r.id) copies,"
        "COALESCE(p.replication_pool_uuid,'') pool,p.migration_after_days,"
        "COALESCE(p.migration_target_uuid,'') destination,"
        "(p.hot_residency_seconds>=0 AND r.start_time>strftime('%s','now')-p.hot_residency_seconds) keep_hot,"
        "(p.migration_target_uuid IS NOT NULL AND r.storage_target_uuid<>p.migration_target_uuid AND ("
        "(r.protected=1 AND p.archive_protected=1) OR "
        "(p.migration_after_days>0 AND r.start_time<=strftime('%s','now')-p.migration_after_days*86400) OR "
        "(p.migration_after_days=0 AND p.archive_after_seconds>=0 AND "
        "r.start_time<=strftime('%s','now')-p.archive_after_seconds))) age_due,"
        "(p.archive_on_pressure=1 AND hot.target_type='filesystem' AND hot.capacity_bytes>0 AND "
        "(hot.available_bytes<=hot.reserve_bytes OR "
        "100.0*(hot.capacity_bytes-hot.available_bytes)/hot.capacity_bytes>=hot.high_watermark_pct)) pressure_due,"
        "EXISTS(SELECT 1 FROM storage_recording_copies c WHERE c.recording_id=r.id "
        "AND c.target_uuid=p.migration_target_uuid) archived,r.start_time,"
        "(r.protected AND p.archive_protected) protected_priority "
        "FROM recordings r JOIN storage_recording_policies p ON p.recording_id=r.id AND p.enabled=1 AND "
        "p.uuid=COALESCE(r.storage_policy_uuid,substr(r.placement_reason,instr(r.placement_reason,':')+1)) "
        "JOIN storage_targets hot ON hot.uuid=r.storage_target_uuid "
        "WHERE r.is_complete=1 AND r.deletion_pending=0 AND "
        "(r.protected=1 OR r.retention_override_days=0 OR "
        "(r.retention_override_days>0 AND r.start_time>strftime('%s','now')-r.retention_override_days*86400) OR "
        "(COALESCE(r.retention_override_days,-1)<0 AND (p.maximum_retention_days=0 OR r.start_time> "
        "strftime('%s','now')-p.maximum_retention_days*86400))) AND "
        "NOT EXISTS(SELECT 1 FROM storage_migration_jobs j WHERE j.recording_id=r.id "
        "AND j.state<>'completed')) "
        "SELECT id,source,required_copy_count,copies,pool,migration_after_days,destination,"
        "(keep_hot AND NOT pressure_due),(age_due OR pressure_due) FROM eligible WHERE "
        "required_copy_count>1+copies OR (destination<>'' AND destination<>source AND "
        "(age_due OR pressure_due) AND (NOT archived OR "
        "((NOT keep_hot OR pressure_due) AND required_copy_count<=copies))) "
        "ORDER BY pressure_due DESC,protected_priority DESC,start_time LIMIT ?;";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) sqlite3_bind_int(statement, 1, max_jobs);
    int count = 0;
    while ((result == SQLITE_OK || result == SQLITE_ROW) &&
           count < max_jobs) {
        result = sqlite3_step(statement);
        if (result != SQLITE_ROW) break;
        lifecycle_candidate_t *candidate = &candidates[count++];
        candidate->recording_id =
            (uint64_t)sqlite3_column_int64(statement, 0);
        copy_column(candidate->source_target_uuid,
                    sizeof(candidate->source_target_uuid), statement, 1);
        candidate->required_copy_count = sqlite3_column_int(statement, 2);
        candidate->current_copy_count = sqlite3_column_int(statement, 3);
        copy_column(candidate->replication_pool_uuid,
                    sizeof(candidate->replication_pool_uuid), statement, 4);
        candidate->migration_after_days = sqlite3_column_int(statement, 5);
        candidate->keep_hot = sqlite3_column_int(statement, 7) != 0;
        candidate->archive_due = sqlite3_column_int(statement, 8) != 0;
        copy_column(candidate->migration_target_uuid,
                    sizeof(candidate->migration_target_uuid), statement, 6);
    }
    if (statement) sqlite3_finalize(statement);
    db_close_readonly_connection(db);
    if (result != SQLITE_DONE && result != SQLITE_ROW) return -1;

    int scheduled = 0;
    for (int index = 0; index < count; index++) {
        lifecycle_candidate_t *candidate = &candidates[index];
        char destination[LIGHTNVR_UUID_STRING_SIZE] = {0};
        const char *operation = "move";
        if (candidate->required_copy_count > 1 +
                candidate->current_copy_count) {
            operation = "copy";
            if (!select_replica_target(candidate, destination)) continue;
        } else if (candidate->archive_due) {
            safe_strcpy(destination, candidate->migration_target_uuid, sizeof(destination), 0);
            if (candidate->keep_hot) operation = "copy";
        } else continue;
        storage_migration_job_t job;
        db_storage_migration_result_t created =
            db_storage_migration_create_operation(candidate->recording_id,
                destination, operation, 0, &job);
        if (created == DB_STORAGE_MIGRATION_OK) scheduled++;
    }
    return scheduled;
}

static int lifecycle_expire(int maximum) {
    if (maximum < 1) return 0;
    if (maximum > LIFECYCLE_CANDIDATE_MAX) maximum = LIFECYCLE_CANDIDATE_MAX;
    sqlite3 *db = NULL;
    int present = open_policy_reader(&db);
    if (present <= 0) return present;
    uint64_t ids[LIFECYCLE_CANDIDATE_MAX];
    int count = 0;
    sqlite3_stmt *stmt = NULL;
    const char *sql = "SELECT r.id FROM recordings r JOIN storage_recording_policies p ON p.recording_id=r.id "
        "WHERE r.is_complete=1 AND r.protected=0 AND r.deletion_pending=0 AND "
        "COALESCE(r.retention_override_days,-1)<>0 AND "
        "(p.minimum_retention_days=0 OR r.start_time<strftime('%s','now')-p.minimum_retention_days*86400) AND "
        "((r.retention_override_days>0 AND r.start_time<strftime('%s','now')-r.retention_override_days*86400) OR "
        "(COALESCE(r.retention_override_days,-1)<0 AND p.maximum_retention_days>0 AND "
        "r.start_time<strftime('%s','now')-p.maximum_retention_days*86400)) AND "
        "NOT EXISTS(SELECT 1 FROM storage_migration_jobs j WHERE j.recording_id=r.id "
        "AND j.state IN('copying','verifying','committing')) ORDER BY r.start_time LIMIT ?;";
    int result = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_int(stmt, 1, maximum);
        while (count < maximum && (result = sqlite3_step(stmt)) == SQLITE_ROW) ids[count++] = (uint64_t)sqlite3_column_int64(stmt, 0);
    }
    if (stmt) sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    if (result != SQLITE_DONE && result != SQLITE_ROW) return -1;
    int accepted = 0;
    for (int i = 0; i < count; i++) if (storage_recording_expire_policy(ids[i]) >= 0) accepted++;
    return accepted;
}

static int upsert_violation(sqlite3 *db, const char *policy_uuid,
                            uint64_t recording_id, const char *scope_key,
                            const char *type, const char *details) {
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    if (lightnvr_uuid_generate_v4(uuid) != 0) return -1;
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db,
        "INSERT INTO storage_policy_violations(uuid,policy_uuid,recording_id,"
        "scope_key,violation_type,details) SELECT ?1,?2,NULLIF(?3,0),?4,?5,?6 "
        "WHERE EXISTS(SELECT 1 FROM storage_policies WHERE uuid=?2) "
        "AND (?3=0 OR EXISTS(SELECT 1 FROM recordings WHERE id=?3)) "
        "ON CONFLICT(policy_uuid,scope_key,violation_type) DO UPDATE SET "
        "details=excluded.details,last_seen_at=strftime('%s','now'),resolved_at=NULL;",
        -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, policy_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 3, (sqlite3_int64)recording_id);
        sqlite3_bind_text(statement, 4, scope_key, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 5, type, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 6, details, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    return result == SQLITE_DONE ? 0 : -1;
}

typedef struct violation {
    struct violation *next;
    char policy_uuid[LIGHTNVR_UUID_STRING_SIZE];
    char resolve_uuid[LIGHTNVR_UUID_STRING_SIZE];
    uint64_t recording_id;
    char scope[96];
    char type[32];
    char details[STORAGE_MIGRATION_ERROR_MAX];
} violation_t;

static int collect_violation(violation_t **head, const char *policy_uuid,
                             uint64_t recording_id, const char *scope,
                             const char *type, const char *details) {
    violation_t *item = calloc(1, sizeof(*item));
    if (!item) return -1;
    safe_strcpy(item->policy_uuid, policy_uuid, sizeof(item->policy_uuid), 0);
    item->recording_id = recording_id;
    safe_strcpy(item->scope, scope, sizeof(item->scope), 0);
    safe_strcpy(item->type, type, sizeof(item->type), 0);
    safe_strcpy(item->details, details, sizeof(item->details), 0);
    item->next = *head;
    *head = item;
    return 0;
}

static int compare_violations(const void *left, const void *right) {
    const violation_t *a = *(const violation_t *const *)left;
    const violation_t *b = *(const violation_t *const *)right;
    int result = strcmp(a->policy_uuid, b->policy_uuid);
    if (!result) result = strcmp(a->scope, b->scope);
    if (!result) result = strcmp(a->type, b->type);
    return result;
}

/* Diff the old active set on the same read snapshot, so resolving conditions
 * also uses primary-key writes rather than a table-wide UPDATE under lock. */
static int collect_resolutions(sqlite3 *db, violation_t *active, int count,
                               violation_t **resolved) {
    violation_t **index = count ? calloc((size_t)count, sizeof(*index)) : NULL;
    if (count && !index) return -1;
    int i = 0;
    for (violation_t *item = active; item; item = item->next) index[i++] = item;
    if (count) qsort(index, (size_t)count, sizeof(*index), compare_violations);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db,
        "SELECT uuid,policy_uuid,scope_key,violation_type FROM "
        "storage_policy_violations WHERE resolved_at IS NULL;",
        -1, &statement, NULL);
    while ((result == SQLITE_OK || result == SQLITE_ROW) &&
           (result = sqlite3_step(statement)) == SQLITE_ROW) {
        violation_t key = {0};
        copy_column(key.policy_uuid, sizeof(key.policy_uuid), statement, 1);
        copy_column(key.scope, sizeof(key.scope), statement, 2);
        copy_column(key.type, sizeof(key.type), statement, 3);
        violation_t *key_pointer = &key;
        if (count && bsearch(&key_pointer, index, (size_t)count,
                             sizeof(*index), compare_violations)) continue;
        violation_t *item = calloc(1, sizeof(*item));
        if (!item) { result = SQLITE_NOMEM; break; }
        copy_column(item->resolve_uuid, sizeof(item->resolve_uuid), statement, 0);
        item->next = *resolved;
        *resolved = item;
    }
    sqlite3_finalize(statement);
    free(index);
    return result == SQLITE_DONE ? 0 : -1;
}

static int write_violations(violation_t *items) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    while (items) {
        pthread_mutex_lock(mutex);
        if (sqlite3_exec(db, "BEGIN IMMEDIATE;", NULL, NULL, NULL) != SQLITE_OK) {
            pthread_mutex_unlock(mutex);
            return -1; /* Never roll back another owner's transaction. */
        }
        int result = SQLITE_OK;
        for (int count = 0; items && count < 64 && result == SQLITE_OK;
             count++, items = items->next) {
            if (items->resolve_uuid[0]) {
                sqlite3_stmt *statement = NULL;
                result = sqlite3_prepare_v2(db,
                    "UPDATE storage_policy_violations SET resolved_at=strftime('%s','now') "
                    "WHERE uuid=? AND resolved_at IS NULL;", -1, &statement, NULL);
                if (result == SQLITE_OK) {
                    sqlite3_bind_text(statement, 1, items->resolve_uuid, -1, SQLITE_TRANSIENT);
                    result = sqlite3_step(statement) == SQLITE_DONE ? SQLITE_OK : SQLITE_ERROR;
                }
                sqlite3_finalize(statement);
            } else if (upsert_violation(db, items->policy_uuid, items->recording_id,
                                        items->scope, items->type, items->details) != 0) {
                result = SQLITE_ERROR;
            }
        }
        if (result != SQLITE_OK ||
            sqlite3_exec(db, "COMMIT;", NULL, NULL, NULL) != SQLITE_OK) {
            sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
            pthread_mutex_unlock(mutex);
            return -1;
        }
        pthread_mutex_unlock(mutex);
    }
    return 0;
}

static int lifecycle_reconcile(void) {
    sqlite3 *db = NULL;
    int present = open_policy_reader(&db);
    if (present <= 0) return present;
    violation_t *violations = NULL;
    violation_t *resolved = NULL;
    /* All five reads see one committed snapshot. Close it before writing. */
    if (sqlite3_exec(db, "BEGIN;", NULL, NULL, NULL) != SQLITE_OK) {
        db_close_readonly_connection(db);
        return -1;
    }
    int active = 0;
    sqlite3_stmt *statement = NULL;
    const char *copy_sql =
        "SELECT p.uuid,r.id,p.required_copy_count,1+count(c.uuid) "
        "FROM recordings r JOIN storage_recording_policies p ON p.recording_id=r.id AND p.enabled=1 AND "
        "p.uuid=COALESCE(r.storage_policy_uuid,substr(r.placement_reason,instr(r.placement_reason,':')+1)) "
        "LEFT JOIN storage_recording_copies c ON c.recording_id=r.id "
        "WHERE r.is_complete=1 AND p.required_copy_count>1 GROUP BY p.uuid,r.id "
        "HAVING 1+count(c.uuid)<p.required_copy_count;";
    int result = sqlite3_prepare_v2(db, copy_sql, -1, &statement, NULL);
    while ((result == SQLITE_OK || result == SQLITE_ROW) &&
           (result = sqlite3_step(statement)) == SQLITE_ROW) {
        const char *policy = (const char *)sqlite3_column_text(statement, 0);
        uint64_t recording_id = (uint64_t)sqlite3_column_int64(statement, 1);
        char scope[64];
        char details[160];
        snprintf(scope, sizeof(scope), "recording:%llu",
                 (unsigned long long)recording_id);
        snprintf(details, sizeof(details), "requires %d copies; %d verified",
                 sqlite3_column_int(statement, 2),
                 sqlite3_column_int(statement, 3));
        if (collect_violation(&violations, policy, recording_id, scope,
                             "copy_count", details) == 0) active++;
        else { result = SQLITE_NOMEM; break; }
    }
    if (statement) sqlite3_finalize(statement);
    statement = NULL;
    if (result != SQLITE_DONE) goto read_failed;

    const char *failed_sql =
        "SELECT p.uuid,r.id,j.uuid,COALESCE(j.last_error,'') FROM "
        "storage_migration_jobs j JOIN recordings r ON r.id=j.recording_id "
        "JOIN storage_recording_policies p ON p.recording_id=r.id AND p.uuid=substr(r.placement_reason,"
        "instr(r.placement_reason,':')+1) WHERE j.state='failed' AND NOT EXISTS("
        "SELECT 1 FROM storage_migration_jobs newer WHERE "
        "newer.recording_id=j.recording_id AND newer.state='completed' AND "
        "newer.created_at>=j.created_at);";
    statement = NULL;
    result = sqlite3_prepare_v2(db, failed_sql, -1, &statement, NULL);
    while ((result == SQLITE_OK || result == SQLITE_ROW) &&
           (result = sqlite3_step(statement)) == SQLITE_ROW) {
        const char *policy = (const char *)sqlite3_column_text(statement, 0);
        uint64_t recording_id = (uint64_t)sqlite3_column_int64(statement, 1);
        const char *job = (const char *)sqlite3_column_text(statement, 2);
        const char *error = (const char *)sqlite3_column_text(statement, 3);
        char scope[96];
        char details[STORAGE_MIGRATION_ERROR_MAX];
        snprintf(scope, sizeof(scope), "job:%s", job ? job : "unknown");
        snprintf(details, sizeof(details), "%s", error ? error : "Migration failed");
        if (collect_violation(&violations, policy, recording_id, scope,
                             "migration_failed", details) == 0) active++;
        else { result = SQLITE_NOMEM; break; }
    }
    if (statement) sqlite3_finalize(statement);
    statement = NULL;
    if (result != SQLITE_DONE) goto read_failed;

    const char *target_sql =
        "SELECT p.uuid,t.uuid,t.name,COALESCE(t.health_status,'unknown') "
        "FROM storage_policies p JOIN storage_targets t ON "
        "t.uuid=p.primary_target_uuid WHERE p.enabled=1 AND "
        "p.primary_pool_uuid IS NULL AND (t.enabled=0 OR t.health_status<>'healthy');";
    statement = NULL;
    result = sqlite3_prepare_v2(db, target_sql, -1, &statement, NULL);
    while ((result == SQLITE_OK || result == SQLITE_ROW) &&
           (result = sqlite3_step(statement)) == SQLITE_ROW) {
        const char *policy = (const char *)sqlite3_column_text(statement, 0);
        const char *target = (const char *)sqlite3_column_text(statement, 1);
        const char *name = (const char *)sqlite3_column_text(statement, 2);
        const char *health = (const char *)sqlite3_column_text(statement, 3);
        char scope[96];
        char details[200];
        snprintf(scope, sizeof(scope), "target:%s", target ? target : "unknown");
        snprintf(details, sizeof(details), "%s is %s",
                 name ? name : "Primary target", health ? health : "unavailable");
        if (collect_violation(&violations, policy, 0, scope,
                             "target_unavailable", details) == 0) active++;
        else { result = SQLITE_NOMEM; break; }
    }
    if (statement) sqlite3_finalize(statement);
    statement = NULL;
    if (result != SQLITE_DONE) goto read_failed;

    const char *pool_sql =
        "SELECT p.uuid,sp.uuid,sp.name FROM storage_policies p JOIN "
        "storage_pools sp ON sp.uuid=p.primary_pool_uuid WHERE p.enabled=1 AND "
        "(sp.enabled=0 OR NOT EXISTS(SELECT 1 FROM storage_pool_members pm "
        "JOIN storage_targets t ON t.uuid=pm.target_uuid WHERE pm.pool_uuid=sp.uuid "
        "AND t.enabled=1 AND t.health_status='healthy' AND "
        "t.available_bytes>t.reserve_bytes));";
    statement = NULL;
    result = sqlite3_prepare_v2(db, pool_sql, -1, &statement, NULL);
    while ((result == SQLITE_OK || result == SQLITE_ROW) &&
           (result = sqlite3_step(statement)) == SQLITE_ROW) {
        const char *policy = (const char *)sqlite3_column_text(statement, 0);
        const char *pool = (const char *)sqlite3_column_text(statement, 1);
        const char *name = (const char *)sqlite3_column_text(statement, 2);
        char scope[96];
        char details[200];
        snprintf(scope, sizeof(scope), "pool:%s", pool ? pool : "unknown");
        snprintf(details, sizeof(details), "%s has no healthy member",
                 name ? name : "Primary pool");
        if (collect_violation(&violations, policy, 0, scope,
                             "target_unavailable", details) == 0) active++;
        else { result = SQLITE_NOMEM; break; }
    }
    if (statement) sqlite3_finalize(statement);
    statement = NULL;
    if (result != SQLITE_DONE) goto read_failed;

    const char *minimum_sql =
        "SELECT p.uuid,p.minimum_retention_days,"
        "COALESCE(SUM(CASE WHEN r.start_time>=strftime('%s','now')-2592000 "
        "THEN r.size_bytes ELSE 0 END),0),"
        "MIN(CASE WHEN r.start_time>=strftime('%s','now')-2592000 THEN r.start_time END),"
        "MAX(CASE WHEN r.start_time>=strftime('%s','now')-2592000 THEN r.start_time END),"
        "CASE WHEN p.primary_pool_uuid IS NOT NULL THEN COALESCE((SELECT "
        "SUM(group_usable) FROM (SELECT MAX(MAX(CAST(t.capacity_bytes*"
        "t.high_watermark_pct/100 AS INTEGER)-t.reserve_bytes,0)) group_usable "
        "FROM storage_pool_members pm JOIN storage_targets t ON t.uuid=pm.target_uuid "
        "WHERE pm.pool_uuid=p.primary_pool_uuid AND t.enabled=1 GROUP BY "
        "CASE WHEN t.filesystem_device<>0 THEN printf('%llu',t.filesystem_device) "
        "ELSE t.uuid END)),0) "
        "ELSE MAX(CAST(pt.capacity_bytes*pt.high_watermark_pct/100 AS INTEGER)-"
        "pt.reserve_bytes,0) END "
        "FROM storage_policies p LEFT JOIN storage_targets pt ON "
        "pt.uuid=p.primary_target_uuid LEFT JOIN recordings r ON "
        "p.uuid=COALESCE(r.storage_policy_uuid,substr(r.placement_reason,instr(r.placement_reason,':')+1)) "
        "WHERE p.enabled=1 AND p.minimum_retention_days>0 GROUP BY p.uuid;";
    statement = NULL;
    result = sqlite3_prepare_v2(db, minimum_sql, -1, &statement, NULL);
    while ((result == SQLITE_OK || result == SQLITE_ROW) &&
           (result = sqlite3_step(statement)) == SQLITE_ROW) {
        const char *policy = (const char *)sqlite3_column_text(statement, 0);
        int minimum_days = sqlite3_column_int(statement, 1);
        long double bytes = (long double)sqlite3_column_int64(statement, 2);
        int64_t first = sqlite3_column_int64(statement, 3);
        int64_t last = sqlite3_column_int64(statement, 4);
        long double usable = (long double)sqlite3_column_int64(statement, 5);
        long double sample_days = first > 0 && last >= first
            ? (long double)(last - first) / 86400.0L + 1.0L : 0.0L;
        long double daily = sample_days > 0.0L ? bytes / sample_days : 0.0L;
        long double forecast_days = daily > 0.0L ? usable / daily : 0.0L;
        if (daily > 0.0L && forecast_days < (long double)minimum_days) {
            char details[200];
            snprintf(details, sizeof(details),
                     "%.1Lf forecast days is below %d-day minimum",
                     forecast_days, minimum_days);
            if (collect_violation(&violations, policy, 0, "policy",
                                 "minimum_retention", details) == 0) active++;
            else { result = SQLITE_NOMEM; break; }
        }
    }
    if (statement) sqlite3_finalize(statement);
    statement = NULL;
    if (result != SQLITE_DONE) goto read_failed;

    if (collect_resolutions(db, violations, active, &resolved) != 0) goto read_failed;
    db_close_readonly_connection(db);
    /* Each batch holds the shared mutex for at most 64 indexed writes. A
     * failed read changes nothing; a failed write leaves remaining conditions
     * for the next pass. Resolve stale entries only after all upserts succeed. */
    if (write_violations(violations) != 0 || write_violations(resolved) != 0)
        active = -1;
    goto cleanup;

read_failed:
    log_error("Storage lifecycle reconciliation read failed: %s", sqlite3_errmsg(db));
    db_close_readonly_connection(db);
    active = -1;
cleanup:
    while (resolved) {
        violation_t *next = resolved->next;
        free(resolved);
        resolved = next;
    }
    while (violations) {
        violation_t *next = violations->next;
        free(violations);
        violations = next;
    }
    return active;
}

static void log_lifecycle_duration(const char *task, struct timespec start,
                                   int result) {
    struct timespec end;
    clock_gettime(CLOCK_MONOTONIC, &end);
    double elapsed_ms = (end.tv_sec - start.tv_sec) * 1000.0 +
                        (end.tv_nsec - start.tv_nsec) / 1000000.0;
    if (result < 0 || elapsed_ms >= 1000.0)
        log_warn("Storage lifecycle %s: result=%d duration_ms=%.1f",
                 task, result, elapsed_ms);
    else
        log_debug("Storage lifecycle %s: result=%d duration_ms=%.1f",
                  task, result, elapsed_ms);
}

int db_storage_lifecycle_schedule(int max_jobs) {
    struct timespec start;
    clock_gettime(CLOCK_MONOTONIC, &start);
    log_debug("Storage lifecycle schedule: starting");
    int result = lifecycle_schedule(max_jobs);
    log_lifecycle_duration("schedule", start, result);
    return result;
}

int db_storage_lifecycle_expire(int maximum) {
    struct timespec start;
    clock_gettime(CLOCK_MONOTONIC, &start);
    log_debug("Storage lifecycle expire: starting");
    int result = lifecycle_expire(maximum);
    log_lifecycle_duration("expire", start, result);
    return result;
}

int db_storage_lifecycle_reconcile(void) {
    struct timespec start;
    clock_gettime(CLOCK_MONOTONIC, &start);
    log_debug("Storage lifecycle reconcile: starting");
    /* Serialize reconciliation passes without blocking ordinary DB callers. */
    static pthread_mutex_t reconcile_mutex = PTHREAD_MUTEX_INITIALIZER;
    pthread_mutex_lock(&reconcile_mutex);
    int result = lifecycle_reconcile();
    pthread_mutex_unlock(&reconcile_mutex);
    log_lifecycle_duration("reconcile", start, result);
    return result;
}
