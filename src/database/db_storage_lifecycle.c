#define _POSIX_C_SOURCE 200809L

#include "database/db_storage_lifecycle.h"

#include <pthread.h>
#include <sqlite3.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "database/db_core.h"
#include "database/db_storage_migrations.h"
#include "database/db_storage_pools.h"
#include "database/db_storage_targets.h"
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
    char migration_target_uuid[LIGHTNVR_UUID_STRING_SIZE];
} lifecycle_candidate_t;

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

int db_storage_lifecycle_schedule(int max_jobs) {
    if (max_jobs < 1) return 0;
    if (max_jobs > LIFECYCLE_CANDIDATE_MAX) max_jobs = LIFECYCLE_CANDIDATE_MAX;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    lifecycle_candidate_t candidates[LIFECYCLE_CANDIDATE_MAX];
    memset(candidates, 0, sizeof(candidates));
    pthread_mutex_lock(mutex);
    const char *sql =
        "SELECT r.id,COALESCE(r.storage_target_uuid,''),p.required_copy_count,"
        "(SELECT count(*) FROM storage_recording_copies c WHERE c.recording_id=r.id),"
        "COALESCE(p.replication_pool_uuid,''),p.migration_after_days,"
        "COALESCE(p.migration_target_uuid,'') "
        "FROM recordings r JOIN storage_policies p ON p.enabled=1 AND "
        "p.uuid=substr(r.placement_reason,instr(r.placement_reason,':')+1) "
        "WHERE r.is_complete=1 AND r.protected=0 AND "
        "(p.maximum_retention_days=0 OR r.start_time > "
        "strftime('%s','now')-p.maximum_retention_days*86400) AND "
        "NOT EXISTS(SELECT 1 FROM storage_migration_jobs j WHERE j.recording_id=r.id "
        "AND j.state NOT IN('completed','failed','cancelled')) AND "
        "NOT EXISTS(SELECT 1 FROM storage_migration_jobs j WHERE j.recording_id=r.id "
        "AND j.state IN('failed','cancelled')) AND ("
        "p.required_copy_count > 1+(SELECT count(*) FROM storage_recording_copies c "
        "WHERE c.recording_id=r.id) OR (p.migration_after_days>0 AND "
        "r.start_time <= strftime('%s','now')-p.migration_after_days*86400 AND "
        "r.storage_target_uuid<>p.migration_target_uuid)) "
        "ORDER BY r.start_time LIMIT ?;";
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
        copy_column(candidate->migration_target_uuid,
                    sizeof(candidate->migration_target_uuid), statement, 6);
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
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
        } else {
            safe_strcpy(destination, candidate->migration_target_uuid,
                        sizeof(destination), 0);
        }
        storage_migration_job_t job;
        db_storage_migration_result_t created =
            db_storage_migration_create_operation(candidate->recording_id,
                destination, operation, 0, &job);
        if (created == DB_STORAGE_MIGRATION_OK) scheduled++;
    }
    return scheduled;
}

static int upsert_violation(sqlite3 *db, const char *policy_uuid,
                            uint64_t recording_id, const char *scope_key,
                            const char *type, const char *details) {
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    if (lightnvr_uuid_generate_v4(uuid) != 0) return -1;
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db,
        "INSERT INTO storage_policy_violations(uuid,policy_uuid,recording_id,"
        "scope_key,violation_type,details) VALUES(?,?,NULLIF(?,0),?,?,?) "
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

int db_storage_lifecycle_reconcile(void) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    if (sqlite3_exec(db, "BEGIN IMMEDIATE;", NULL, NULL, NULL) != SQLITE_OK ||
        sqlite3_exec(db, "UPDATE storage_policy_violations SET "
            "resolved_at=strftime('%s','now') WHERE resolved_at IS NULL;",
            NULL, NULL, NULL) != SQLITE_OK) {
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        pthread_mutex_unlock(mutex);
        return -1;
    }
    int active = 0;
    sqlite3_stmt *statement = NULL;
    const char *copy_sql =
        "SELECT p.uuid,r.id,p.required_copy_count,1+count(c.uuid) "
        "FROM recordings r JOIN storage_policies p ON p.enabled=1 AND "
        "p.uuid=substr(r.placement_reason,instr(r.placement_reason,':')+1) "
        "LEFT JOIN storage_recording_copies c ON c.recording_id=r.id "
        "WHERE r.is_complete=1 AND p.required_copy_count>1 GROUP BY p.uuid,r.id "
        "HAVING 1+count(c.uuid)<p.required_copy_count;";
    int result = sqlite3_prepare_v2(db, copy_sql, -1, &statement, NULL);
    while (result == SQLITE_OK && sqlite3_step(statement) == SQLITE_ROW) {
        const char *policy = (const char *)sqlite3_column_text(statement, 0);
        uint64_t recording_id = (uint64_t)sqlite3_column_int64(statement, 1);
        char scope[64];
        char details[160];
        snprintf(scope, sizeof(scope), "recording:%llu",
                 (unsigned long long)recording_id);
        snprintf(details, sizeof(details), "requires %d copies; %d verified",
                 sqlite3_column_int(statement, 2),
                 sqlite3_column_int(statement, 3));
        if (upsert_violation(db, policy, recording_id, scope,
                             "copy_count", details) == 0) active++;
    }
    if (statement) sqlite3_finalize(statement);

    const char *failed_sql =
        "SELECT p.uuid,r.id,j.uuid,COALESCE(j.last_error,'') FROM "
        "storage_migration_jobs j JOIN recordings r ON r.id=j.recording_id "
        "JOIN storage_policies p ON p.uuid=substr(r.placement_reason,"
        "instr(r.placement_reason,':')+1) WHERE j.state='failed' AND NOT EXISTS("
        "SELECT 1 FROM storage_migration_jobs newer WHERE "
        "newer.recording_id=j.recording_id AND newer.state='completed' AND "
        "newer.created_at>=j.created_at);";
    statement = NULL;
    result = sqlite3_prepare_v2(db, failed_sql, -1, &statement, NULL);
    while (result == SQLITE_OK && sqlite3_step(statement) == SQLITE_ROW) {
        const char *policy = (const char *)sqlite3_column_text(statement, 0);
        uint64_t recording_id = (uint64_t)sqlite3_column_int64(statement, 1);
        const char *job = (const char *)sqlite3_column_text(statement, 2);
        const char *error = (const char *)sqlite3_column_text(statement, 3);
        char scope[96];
        char details[STORAGE_MIGRATION_ERROR_MAX];
        snprintf(scope, sizeof(scope), "job:%s", job ? job : "unknown");
        snprintf(details, sizeof(details), "%s", error ? error : "Migration failed");
        if (upsert_violation(db, policy, recording_id, scope,
                             "migration_failed", details) == 0) active++;
    }
    if (statement) sqlite3_finalize(statement);

    const char *target_sql =
        "SELECT p.uuid,t.uuid,t.name,COALESCE(t.health_status,'unknown') "
        "FROM storage_policies p JOIN storage_targets t ON "
        "t.uuid=p.primary_target_uuid WHERE p.enabled=1 AND "
        "p.primary_pool_uuid IS NULL AND (t.enabled=0 OR t.health_status<>'healthy');";
    statement = NULL;
    result = sqlite3_prepare_v2(db, target_sql, -1, &statement, NULL);
    while (result == SQLITE_OK && sqlite3_step(statement) == SQLITE_ROW) {
        const char *policy = (const char *)sqlite3_column_text(statement, 0);
        const char *target = (const char *)sqlite3_column_text(statement, 1);
        const char *name = (const char *)sqlite3_column_text(statement, 2);
        const char *health = (const char *)sqlite3_column_text(statement, 3);
        char scope[96];
        char details[200];
        snprintf(scope, sizeof(scope), "target:%s", target ? target : "unknown");
        snprintf(details, sizeof(details), "%s is %s",
                 name ? name : "Primary target", health ? health : "unavailable");
        if (upsert_violation(db, policy, 0, scope,
                             "target_unavailable", details) == 0) active++;
    }
    if (statement) sqlite3_finalize(statement);

    const char *pool_sql =
        "SELECT p.uuid,sp.uuid,sp.name FROM storage_policies p JOIN "
        "storage_pools sp ON sp.uuid=p.primary_pool_uuid WHERE p.enabled=1 AND "
        "(sp.enabled=0 OR NOT EXISTS(SELECT 1 FROM storage_pool_members pm "
        "JOIN storage_targets t ON t.uuid=pm.target_uuid WHERE pm.pool_uuid=sp.uuid "
        "AND t.enabled=1 AND t.health_status='healthy' AND "
        "t.available_bytes>t.reserve_bytes));";
    statement = NULL;
    result = sqlite3_prepare_v2(db, pool_sql, -1, &statement, NULL);
    while (result == SQLITE_OK && sqlite3_step(statement) == SQLITE_ROW) {
        const char *policy = (const char *)sqlite3_column_text(statement, 0);
        const char *pool = (const char *)sqlite3_column_text(statement, 1);
        const char *name = (const char *)sqlite3_column_text(statement, 2);
        char scope[96];
        char details[200];
        snprintf(scope, sizeof(scope), "pool:%s", pool ? pool : "unknown");
        snprintf(details, sizeof(details), "%s has no healthy member",
                 name ? name : "Primary pool");
        if (upsert_violation(db, policy, 0, scope,
                             "target_unavailable", details) == 0) active++;
    }
    if (statement) sqlite3_finalize(statement);

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
        "p.uuid=substr(r.placement_reason,instr(r.placement_reason,':')+1) "
        "WHERE p.enabled=1 AND p.minimum_retention_days>0 GROUP BY p.uuid;";
    statement = NULL;
    result = sqlite3_prepare_v2(db, minimum_sql, -1, &statement, NULL);
    while (result == SQLITE_OK && sqlite3_step(statement) == SQLITE_ROW) {
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
            if (upsert_violation(db, policy, 0, "policy",
                                 "minimum_retention", details) == 0) active++;
        }
    }
    if (statement) sqlite3_finalize(statement);

    if (sqlite3_exec(db, "COMMIT;", NULL, NULL, NULL) != SQLITE_OK) {
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        active = -1;
    }
    pthread_mutex_unlock(mutex);
    return active;
}
