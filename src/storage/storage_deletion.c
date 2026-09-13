#define _POSIX_C_SOURCE 200809L
#include "storage/storage_deletion.h"

#include <errno.h>
#include <dirent.h>
#include "core/config.h"
#include "video/recording_path.h"
#include <pthread.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

#include "database/db_core.h"
#include "database/db_storage_targets.h"
#include "storage/storage_s3.h"
#include "utils/strings.h"
#include "utils/uuid.h"

typedef struct {
    int64_t id;
    char target[LIGHTNVR_UUID_STRING_SIZE];
    char key[MAX_PATH_LENGTH];
    char upload_id[1024];
    char path[MAX_PATH_LENGTH];
    uint64_t size;
} deletion_item_t;

static int execute_id(sqlite3 *db, const char *sql, uint64_t id, const char *uuid) {
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_int64(statement, 1, (sqlite3_int64)id);
        if (uuid) sqlite3_bind_text(statement, 2, uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    return result == SQLITE_DONE ? 0 : -1;
}

static int process_item(const deletion_item_t *item, bool local_only, uint64_t *removed) {
    storage_target_t target;
    char path[MAX_PATH_LENGTH] = {0}, error[STORAGE_TARGET_ERROR_MAX] = {0};
    bool success = false, file_removed = false;
    if (item->target[0]) {
        if (db_storage_target_get(item->target, &target) != DB_STORAGE_TARGET_OK) {
            safe_strcpy(error, "Deletion target is unavailable", sizeof(error), 0);
        } else if (strcmp(target.target_type, "s3") == 0) {
            if (local_only) return 0;
            /* Do not interpret a missing bucket or configuration drift as an
             * authoritative missing object. Reads/deletes remain allowed for
             * disabled targets so disablement cannot strand their inventory. */
            if (storage_s3_probe(&target, false) == 0)
                success = storage_s3_abort_upload(&target, item->key, item->upload_id, error) == 0 &&
                    storage_s3_delete(&target, item->key, error) == 0;
            else safe_strcpy(error, target.last_error, sizeof(error), 0);
        } else if (!db_storage_target_mount_guard_active(&target)) {
            safe_strcpy(error, "Deletion target mount is absent", sizeof(error), 0);
        } else if (db_storage_target_resolve_path(item->target, item->key, path) != 0) {
            safe_strcpy(error, "Deletion object identity is invalid", sizeof(error), 0);
        }
    } else safe_strcpy(path, item->path, sizeof(path), 0);
    if (path[0]) {
        struct stat info;
        if (lstat(path, &info) == 0 && !S_ISREG(info.st_mode)) {
            safe_strcpy(error, "Deletion object is not a regular file", sizeof(error), 0);
        } else if (unlink(path) == 0) success = file_removed = true;
        else if (errno == ENOENT) success = true;
        else safe_strcpy(error, "Cannot remove recording file", sizeof(error), 0);
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    const char *sql = "UPDATE storage_deletion_objects SET state=?,last_error=?,"
        "attempt_count=attempt_count+1,next_attempt_at=CASE WHEN ? THEN 0 ELSE "
        "strftime('%s','now')+MIN(3600,30*(1<<MIN(attempt_count,7))) END,"
        "target_uuid=CASE WHEN ? THEN NULL ELSE target_uuid END,updated_at=strftime('%s','now') WHERE id=?;";
    if (sqlite3_prepare_v2(db, sql, -1, &statement, NULL) == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, success ? "completed" : "retry_wait", -1, SQLITE_STATIC);
        sqlite3_bind_text(statement, 2, success ? "" : (*error ? error : "Deletion failed"), -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 3, success);
        sqlite3_bind_int(statement, 4, success);
        sqlite3_bind_int64(statement, 5, item->id);
        sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    if (file_removed && removed) *removed += item->size;
    return 1;
}

static void finalize_deletions_locked(sqlite3 *db) {
    if (sqlite3_exec(db, "BEGIN IMMEDIATE;", NULL, NULL, NULL) != SQLITE_OK) return;
    const char *sql = "UPDATE storage_deletions SET completed_at=strftime('%s','now') "
        "WHERE completed_at IS NULL AND NOT EXISTS(SELECT 1 FROM storage_deletion_objects o "
        "WHERE o.deletion_uuid=storage_deletions.uuid AND o.state<>'completed');"
        "UPDATE detections SET recording_id=NULL WHERE recording_id IN "
        "(SELECT r.id FROM recordings r JOIN storage_deletions d ON d.recording_id=r.id "
        "WHERE r.deletion_pending=1 AND d.completed_at IS NOT NULL);"
        "DELETE FROM recordings WHERE deletion_pending=1 AND EXISTS(SELECT 1 FROM storage_deletions d "
        "WHERE d.recording_id=recordings.id AND d.completed_at IS NOT NULL);COMMIT;";
    if (sqlite3_exec(db, sql, NULL, NULL, NULL) != SQLITE_OK)
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
}

static int process_due(const char *deletion_uuid, bool local_only, uint64_t *removed) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    deletion_item_t items[32];
    int count = 0;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    const char *sql = "SELECT o.id,COALESCE(o.target_uuid,''),o.object_key,o.file_path,o.size_bytes,o.upload_id "
        "FROM storage_deletion_objects o LEFT JOIN storage_targets t ON t.uuid=o.target_uuid "
        "WHERE o.state<>'completed' AND o.next_attempt_at<=strftime('%s','now') "
        "AND (? IS NULL OR o.deletion_uuid=?) AND (?=0 OR COALESCE(t.target_type,'filesystem')='filesystem') "
        "AND NOT EXISTS(SELECT 1 FROM storage_deletions d JOIN storage_read_leases l ON l.recording_id=d.recording_id "
        "WHERE d.uuid=o.deletion_uuid AND l.expires_at>strftime('%s','now')) "
        "AND NOT EXISTS(SELECT 1 FROM storage_deletions d JOIN storage_retrieval_jobs j ON j.recording_id=d.recording_id "
        "WHERE d.uuid=o.deletion_uuid AND j.state='fetching') "
        "ORDER BY o.id LIMIT ?;";
    if (sqlite3_prepare_v2(db, sql, -1, &statement, NULL) == SQLITE_OK) {
        if (deletion_uuid) {
            sqlite3_bind_text(statement, 1, deletion_uuid, -1, SQLITE_TRANSIENT);
            sqlite3_bind_text(statement, 2, deletion_uuid, -1, SQLITE_TRANSIENT);
        }
        sqlite3_bind_int(statement, 3, local_only);
        sqlite3_bind_int(statement, 4, local_only ? 32 : 1);
        while (count < 32 && sqlite3_step(statement) == SQLITE_ROW) {
            deletion_item_t *item = &items[count++];
            memset(item, 0, sizeof(*item));
            item->id = sqlite3_column_int64(statement, 0);
            safe_strcpy(item->target, (const char *)sqlite3_column_text(statement, 1), sizeof(item->target), 0);
            safe_strcpy(item->key, (const char *)sqlite3_column_text(statement, 2), sizeof(item->key), 0);
            safe_strcpy(item->path, (const char *)sqlite3_column_text(statement, 3), sizeof(item->path), 0);
            item->size = (uint64_t)sqlite3_column_int64(statement, 4);
            safe_strcpy(item->upload_id, (const char *)sqlite3_column_text(statement, 5), sizeof(item->upload_id), 0);
        }
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    for (int i = 0; i < count; i++) process_item(&items[i], local_only, removed);
    pthread_mutex_lock(mutex);
    finalize_deletions_locked(db);
    pthread_mutex_unlock(mutex);
    return count;
}

static bool recovery_read_only(void) {
    const char *value = getenv("LIGHTNVR_STORAGE_READ_ONLY");
    return value && !strcmp(value, "1");
}
int storage_deletion_process_one(void) { return recovery_read_only() ? 0 : process_due(NULL, false, NULL); }

static int inventory_path(sqlite3 *db, const char *uuid, const char *path) {
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, "INSERT INTO storage_deletion_objects(deletion_uuid,file_path) VALUES(?,?);", -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 2, path, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);
    return rc == SQLITE_DONE ? 0 : -1;
}

static int inventory_derivatives(sqlite3 *db, const char *uuid, uint64_t id) {
    // Journal generated names as well as existing files: an in-flight consumer
    // may finish writing before the recording's read lease expires.
    char path[MAX_PATH_LENGTH], directory[MAX_PATH_LENGTH];
    for (int i = 0; i < 3; i++) {
        int n = snprintf(path, sizeof(path), "%s/thumbnails/%llu_%d.jpg", g_config.storage_path, (unsigned long long)id, i);
        if (n < 0 || n >= (int)sizeof(path) || inventory_path(db, uuid, path)) return -1;
    }
    if (!build_recording_transcode_cache_path(g_config.storage_path, id, path, sizeof(path)) && inventory_path(db, uuid, path)) return -1;
    int n = snprintf(directory, sizeof(directory), "%s/thumbnails/investigation/%llu", g_config.storage_path, (unsigned long long)id);
    if (n < 0 || n >= (int)sizeof(directory)) return -1;
    DIR *dir = opendir(directory);
    if (!dir) return errno == ENOENT ? 0 : -1;
    int result = 0;
    struct dirent *entry;
    while ((entry = readdir(dir))) {
        size_t digits = strspn(entry->d_name, "0123456789");
        if (!digits || strcmp(entry->d_name + digits, ".jpg")) continue;
        n = snprintf(path, sizeof(path), "%s/%s", directory, entry->d_name);
        if (n < 0 || n >= (int)sizeof(path) || inventory_path(db, uuid, path)) { result = -1; break; }
    }
    closedir(dir);
    return result;
}

static int recording_delete(uint64_t id, const char *reason, uint64_t *removed,
                            bool require_policy_expiry, int64_t age_cutoff) {
    if (recovery_read_only()) return -1;
    if (removed) *removed = 0;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || !id) return -1;
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    if (lightnvr_uuid_generate_v4(uuid)) return -1;
    pthread_mutex_lock(mutex);
    if (sqlite3_exec(db, "BEGIN IMMEDIATE;", NULL, NULL, NULL) != SQLITE_OK) {
        pthread_mutex_unlock(mutex);
        return -1;
    }
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, "SELECT protected,EXISTS(SELECT 1 FROM storage_migration_jobs "
        "WHERE recording_id=?1 AND state IN('copying','verifying','committing')),deletion_pending FROM recordings WHERE id=?1;",
        -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_int64(statement, 1, (sqlite3_int64)id);
        result = sqlite3_step(statement);
    }
    bool blocked = result == SQLITE_ROW && (sqlite3_column_int(statement, 0) || sqlite3_column_int(statement, 1));
    bool pending = result == SQLITE_ROW && sqlite3_column_int(statement, 2);
    if (statement) sqlite3_finalize(statement);
    if (blocked || pending || result != SQLITE_ROW) {
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        pthread_mutex_unlock(mutex);
        return blocked ? -2 : (pending ? 1 : (result == SQLITE_DONE ? 0 : -1));
    }
    if (age_cutoff) {
        statement = NULL;
        result = sqlite3_prepare_v2(db,
            "SELECT 1 FROM recordings r LEFT JOIN storage_recording_policies p ON p.recording_id=r.id "
            "WHERE " STORAGE_LEGACY_EXPIRY_PREDICATE " AND r.id=?2;", -1, &statement, NULL);
        if (result == SQLITE_OK) {
            sqlite3_bind_int64(statement, 1, age_cutoff);
            sqlite3_bind_int64(statement, 2, (sqlite3_int64)id);
            result = sqlite3_step(statement);
        }
        if (statement) sqlite3_finalize(statement);
        if (result != SQLITE_ROW) {
            sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
            pthread_mutex_unlock(mutex);
            return result == SQLITE_DONE ? -2 : -1;
        }
    }
    if (require_policy_expiry) {
        /* Keep this predicate consistent with db_storage_lifecycle_expire. A
         * policy application or override edit may occur after candidate selection. */
        statement = NULL;
        result = sqlite3_prepare_v2(db,
            "SELECT 1 FROM recordings r JOIN storage_recording_policies p ON p.recording_id=r.id "
            "WHERE r.id=? AND r.is_complete=1 AND COALESCE(r.retention_override_days,-1)<>0 AND "
            "(p.minimum_retention_days=0 OR r.start_time<strftime('%s','now')-p.minimum_retention_days*86400) AND "
            "((r.retention_override_days>0 AND r.start_time<strftime('%s','now')-r.retention_override_days*86400) OR "
            "(COALESCE(r.retention_override_days,-1)<0 AND p.maximum_retention_days>0 AND "
            "r.start_time<strftime('%s','now')-p.maximum_retention_days*86400));", -1, &statement, NULL);
        if (result == SQLITE_OK) {
            sqlite3_bind_int64(statement, 1, (sqlite3_int64)id);
            result = sqlite3_step(statement);
        }
        if (statement) sqlite3_finalize(statement);
        if (result != SQLITE_ROW) {
            sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
            pthread_mutex_unlock(mutex);
            return result == SQLITE_DONE ? -2 : -1;
        }
    }
    result = SQLITE_OK;
    statement = NULL;
    if (result == SQLITE_OK) result = sqlite3_prepare_v2(db,
        "INSERT INTO storage_deletions(uuid,recording_id,camera_uuid,stream_name,reason) "
        "SELECT ?2,id,COALESCE(camera_uuid,''),stream_name,?3 FROM recordings WHERE id=?1;", -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_int64(statement, 1, (sqlite3_int64)id);
        sqlite3_bind_text(statement, 2, uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 3, reason ? reason : "recording deletion", -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    const char *inventory[] = {
        "INSERT INTO storage_deletion_objects(deletion_uuid,target_uuid,object_key,file_path,size_bytes) "
        "SELECT ?2,storage_target_uuid,COALESCE(object_key,''),file_path,MAX(size_bytes,0) FROM recordings "
        "WHERE id=?1 AND (file_path<>'' OR storage_target_uuid IS NOT NULL);",
        "INSERT INTO storage_deletion_objects(deletion_uuid,target_uuid,object_key,size_bytes) "
        "SELECT ?2,target_uuid,object_key,size_bytes FROM storage_recording_copies WHERE recording_id=?1;",
        "INSERT INTO storage_deletion_objects(deletion_uuid,target_uuid,object_key,size_bytes) "
        "SELECT ?2,source_target_uuid,source_object_key,bytes_total FROM storage_migration_jobs "
        "WHERE recording_id=?1 AND state='cleanup_pending';",
        "INSERT INTO storage_deletion_objects(deletion_uuid,target_uuid,object_key,size_bytes,upload_id) "
        "SELECT ?2,destination_target_uuid,destination_object_key,bytes_total,upload_id FROM storage_migration_jobs "
        "WHERE recording_id=?1 AND state NOT IN('completed','cleanup_pending');",
        "INSERT INTO storage_deletion_objects(deletion_uuid,file_path,size_bytes) "
        "SELECT ?2,file_path,size_bytes FROM storage_retrieval_jobs WHERE recording_id=?1;",
        "UPDATE storage_migration_jobs SET cancel_requested=1,state='cancelled' WHERE recording_id=?1 "
        "AND state NOT IN('completed','cleanup_pending');",
        "UPDATE recordings SET deletion_pending=1 WHERE id=?1;"
    };
    for (size_t i = 0; result == SQLITE_DONE && i < sizeof(inventory) / sizeof(inventory[0]); i++)
        if (execute_id(db, inventory[i], id, i < 5 ? uuid : NULL)) result = SQLITE_ERROR;
    if (result == SQLITE_DONE && inventory_derivatives(db, uuid, id)) result = SQLITE_ERROR;
    bool committed = result == SQLITE_DONE && sqlite3_exec(db, "COMMIT;", NULL, NULL, NULL) == SQLITE_OK;
    if (!committed) sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
    pthread_mutex_unlock(mutex);
    if (!committed) return -1;
    /* Preserve synchronous local reclamation for the pressure controller. Remote
     * work is handled by the background worker and remains in the journal. */
    process_due(uuid, true, removed);
    pthread_mutex_lock(mutex);
    statement = NULL;
    bool still_pending = true;
    if (sqlite3_prepare_v2(db, "SELECT 1 FROM recordings WHERE id=?;", -1, &statement, NULL) == SQLITE_OK) {
        sqlite3_bind_int64(statement, 1, (sqlite3_int64)id);
        still_pending = sqlite3_step(statement) == SQLITE_ROW;
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return still_pending ? 1 : 0;
}

int storage_recording_delete(uint64_t id, const char *reason, uint64_t *removed) {
    return recording_delete(id, reason, removed, false, 0);
}

int storage_recording_expire_policy(uint64_t id) {
    return recording_delete(id, "policy expiry", NULL, true, 0);
}

int storage_recording_expire_age(uint64_t id, int64_t cutoff) {
    if (!cutoff) return -2;
    return recording_delete(id, "legacy age retention", NULL, false, cutoff);
}
