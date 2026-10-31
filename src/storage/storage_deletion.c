/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#include "storage/storage_deletion.h"

#include <errno.h>
#include <limits.h>
#include <dirent.h>
#include "core/config.h"
#include "video/recording_path.h"
#include "video/mp4_recording.h"
#include <pthread.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

#include "database/db_core.h"
#include "database/db_storage_targets.h"
#include "storage/storage_s3.h"
#include "utils/strings.h"
#include "utils/uuid.h"

/* Seconds between full-ledger finalization sweeps in the worker. Deletions are
 * normally finalized by uuid right after their objects complete; the sweep
 * only recovers deletions interrupted between those two steps. */
#define STORAGE_DELETION_SWEEP_INTERVAL_SEC 60
/* Rows removed per prune transaction, bounding how long the mutex is held. */
#define STORAGE_DELETION_PRUNE_BATCH 500
/* Due objects considered per worker pass, in earliest-due order, before the
 * FIFO ordering below picks among them. Bounds the sort to a window the
 * partial index can deliver without scanning the ledger. */
#define STORAGE_DELETION_DUE_WINDOW 256

typedef struct {
    int64_t id;
    char deletion[LIGHTNVR_UUID_STRING_SIZE];
    char target[LIGHTNVR_UUID_STRING_SIZE];
    char key[MAX_PATH_LENGTH];
    char upload_id[1024];
    char path[MAX_PATH_LENGTH];
    uint64_t size;
    int stale_guard; /* 0 ordinary, 1 expected missing, 2 expected empty, -1 invalid */
    uint64_t expected_device, expected_inode;
} deletion_item_t;

typedef struct {
    const recording_metadata_t *expected;
    time_t cutoff;
    int kind;
    uint64_t device, inode;
} stale_prune_t;

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

/* An unattributed legacy path has no persisted mount identity. ENOENT is
 * authoritative only below the configured root while that root is available.
 * Unknown roots remain retryable rather than discarding their metadata. This
 * cannot detect an unmounted filesystem whose mount-point directory remains;
 * attributed targets continue to use their existing mount guard below. */
static bool legacy_missing_path_is_authoritative(const char *path) {
    const char *root = g_config.storage_path;
    size_t length = strlen(root);
    while (length > 1 && root[length - 1] == '/') length--;
    if (!length || strncmp(path, root, length) != 0 ||
        (!(length == 1 && root[0] == '/') && path[length] != '/')) return false;
    /* A lexical parent traversal must not borrow an unrelated root's health. */
    const char *component = path + length;
    while ((component = strstr(component, "/..")) != NULL) {
        if (component[3] == '/' || component[3] == '\0') return false;
        component += 3;
    }
    struct stat info;
    return stat(root, &info) == 0 && S_ISDIR(info.st_mode);
}

/* Legacy storage has no persisted mount identity. Require an available
 * canonical directory and canonical parent beneath it; unknown or symlinked
 * path spelling is retained. Attributed targets are not inferred from paths. */
static bool stale_path_is_authoritative(const char *path) {
    char root[PATH_MAX], parent[PATH_MAX], copy[MAX_PATH_LENGTH], canonical[PATH_MAX];
    struct stat info;
    if (!path || path[0] != '/' || strlen(path) >= sizeof(copy) ||
        !realpath(g_config.storage_path, root) || stat(root, &info) != 0 || !S_ISDIR(info.st_mode)) return false;
    safe_strcpy(copy, path, sizeof(copy), 0);
    char *name = strrchr(copy, '/');
    if (!name || !name[1] || strcmp(name + 1, ".") == 0 || strcmp(name + 1, "..") == 0) return false;
    *name++ = '\0';
    if (!realpath(copy[0] ? copy : "/", parent)) return false;
    size_t length = strlen(root);
    if (strcmp(parent, root) != 0 &&
        (strncmp(parent, root, length) != 0 || (length != 1 && parent[length] != '/'))) return false;
    int written = snprintf(canonical, sizeof(canonical), "%s%s%s", parent,
                           strcmp(parent, "/") == 0 ? "" : "/", name);
    return written > 0 && written < (int)sizeof(canonical) && strcmp(canonical, path) == 0;
}

/* Recheck delayed guarded work after restart. This is conservative per-stream
 * ownership, not a lock against arbitrary external filesystem writers. */
static bool stale_owner_is_quiescent(const deletion_item_t *item) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return false;
    char stream[MAX_STREAM_NAME] = {0};
    sqlite3_stmt *statement = NULL;
    pthread_mutex_lock(mutex);
    const char *query = "SELECT r.stream_name FROM recordings r "
        "JOIN storage_deletions d ON d.recording_id=r.id "
        "JOIN streams s ON s.name=r.stream_name WHERE d.uuid=?1 AND r.file_path=?2 "
        "AND s.enabled=0 AND r.deletion_pending=1 AND r.is_complete=0 AND r.protected=0 "
        "AND r.size_bytes=0 AND COALESCE(r.end_time,0)=0 AND r.storage_target_uuid IS NULL "
        "AND NOT EXISTS(SELECT 1 FROM storage_migration_jobs WHERE recording_id=r.id) "
        "AND NOT EXISTS(SELECT 1 FROM storage_recording_copies WHERE recording_id=r.id) "
        "AND NOT EXISTS(SELECT 1 FROM storage_retrieval_jobs WHERE recording_id=r.id)";
    if (sqlite3_prepare_v2(db, query, -1, &statement, NULL) == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, item->deletion, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, item->path, -1, SQLITE_TRANSIENT);
        if (sqlite3_step(statement) == SQLITE_ROW)
            safe_strcpy(stream, (const char *)sqlite3_column_text(statement, 0), sizeof(stream), 0);
    }
    sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return stream[0] && !stream_has_continuous_writer(stream) && !mp4_recording_thread_alive(stream);
}

static int process_item(const deletion_item_t *item, bool local_only, uint64_t *removed) {
    storage_target_t target;
    char path[MAX_PATH_LENGTH] = {0}, error[STORAGE_TARGET_ERROR_MAX] = {0};
    bool success = false, file_removed = false;
    if (item->stale_guard < 0 || (item->stale_guard && item->target[0])) {
        safe_strcpy(error, "Invalid or unsupported stale deletion guard", sizeof(error), 0);
    } else if (item->stale_guard && !stale_owner_is_quiescent(item)) {
        safe_strcpy(error, "Guarded deletion ownership changed or is active; retained", sizeof(error), 0);
    } else if (item->target[0]) {
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
        int status = lstat(path, &info);
        bool missing = status != 0 && errno == ENOENT;
        if (item->stale_guard && !stale_path_is_authoritative(path)) {
            safe_strcpy(error, "Guarded deletion storage is unavailable or unknown", sizeof(error), 0);
        } else if (item->stale_guard == 1) {
            if (missing) success = true;
            else safe_strcpy(error, "Stale missing path appeared or cannot be verified; retained", sizeof(error), 0);
        } else if (item->stale_guard == 2 && missing) {
            success = true;
        } else if (item->stale_guard == 2 && (status != 0 || !S_ISREG(info.st_mode) || info.st_size != 0 ||
                   (uint64_t)info.st_dev != item->expected_device || (uint64_t)info.st_ino != item->expected_inode)) {
            safe_strcpy(error, "Stale empty file identity or contents changed; retained", sizeof(error), 0);
        } else if (status == 0 && !S_ISREG(info.st_mode)) {
            safe_strcpy(error, "Deletion object is not a regular file", sizeof(error), 0);
        } else if (unlink(path) == 0) success = file_removed = true;
        else if (errno == ENOENT) {
            if (item->target[0] || legacy_missing_path_is_authoritative(path)) success = true;
            else safe_strcpy(error, "Legacy deletion root is unavailable or unknown", sizeof(error), 0);
        }
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

static int execute_uuid(sqlite3 *db, const char *sql, const char *uuid) {
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        if (uuid) sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    return result == SQLITE_DONE ? 0 : -1;
}

/* Close deletions whose objects have all completed, detach their detections
 * and drop the recording rows. With a uuid only that ledger entry is inspected
 * (primary key + idx_storage_deletion_object_deletion); without one the sweep
 * walks the open entries through idx_storage_deletion_completed and the
 * pending recordings through idx_recordings_deletion_pending, so its cost
 * follows the in-flight set rather than the size of the ledger or catalog. */
static int finalize_deletions_locked(sqlite3 *db, const char *uuid) {
    static const char *const scoped[] = {
        "UPDATE storage_deletions SET completed_at=strftime('%s','now') WHERE uuid=?1 AND completed_at IS NULL "
        "AND NOT EXISTS(SELECT 1 FROM storage_deletion_objects o WHERE o.deletion_uuid=?1 AND o.state<>'completed');",
        "UPDATE detections SET recording_id=NULL WHERE recording_id IN "
        "(SELECT recording_id FROM storage_deletions WHERE uuid=?1 AND completed_at IS NOT NULL);",
        "DELETE FROM recordings WHERE deletion_pending=1 AND id IN "
        "(SELECT recording_id FROM storage_deletions WHERE uuid=?1 AND completed_at IS NOT NULL);"
    };
    static const char *const sweep[] = {
        "UPDATE storage_deletions SET completed_at=strftime('%s','now') "
        "WHERE completed_at IS NULL AND NOT EXISTS(SELECT 1 FROM storage_deletion_objects o "
        "WHERE o.deletion_uuid=storage_deletions.uuid AND o.state<>'completed');",
        "UPDATE detections SET recording_id=NULL WHERE recording_id IN "
        "(SELECT r.id FROM recordings r JOIN storage_deletions d ON d.recording_id=r.id "
        "WHERE r.deletion_pending=1 AND d.completed_at IS NOT NULL);",
        "DELETE FROM recordings WHERE deletion_pending=1 AND EXISTS(SELECT 1 FROM storage_deletions d "
        "WHERE d.recording_id=recordings.id AND d.completed_at IS NOT NULL);"
    };
    const char *const *steps = uuid ? scoped : sweep;
    if (sqlite3_exec(db, "BEGIN IMMEDIATE;", NULL, NULL, NULL) != SQLITE_OK) return -1;
    int result = 0;
    for (size_t i = 0; result == 0 && i < 3; i++) result = execute_uuid(db, steps[i], uuid);
    if (result == 0 && sqlite3_exec(db, "COMMIT;", NULL, NULL, NULL) == SQLITE_OK) return 0;
    sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
    return -1;
}

#define DELETION_DUE_SELECT \
    "SELECT o.id,o.deletion_uuid,COALESCE(o.target_uuid,''),o.object_key,o.file_path,o.size_bytes,o.upload_id,o.stale_guard_kind,o.stale_guard_dev,o.stale_guard_ino " \
    "FROM storage_deletion_objects o LEFT JOIN storage_targets t ON t.uuid=o.target_uuid WHERE "
#define DELETION_DUE_FILTER \
    " AND (?2=0 OR COALESCE(t.target_type,'filesystem')='filesystem') " \
    "AND NOT EXISTS(SELECT 1 FROM storage_deletions d JOIN storage_read_leases l ON l.recording_id=d.recording_id " \
    "WHERE d.uuid=o.deletion_uuid AND l.expires_at>strftime('%s','now')) " \
    "AND NOT EXISTS(SELECT 1 FROM storage_deletions d JOIN storage_retrieval_jobs j ON j.recording_id=d.recording_id " \
    "WHERE d.uuid=o.deletion_uuid AND j.state='fetching') ORDER BY o.id LIMIT ?3;"

static int process_due(const char *deletion_uuid, bool local_only, uint64_t *removed) {
    /* One deletion: probe its objects through idx_storage_deletion_object_deletion.
     * Sweep: take the earliest-due window from idx_storage_deletion_object_open
     * (a partial index over open objects) and keep FIFO order inside it; the
     * plain ORDER BY o.id form makes the planner walk the whole table. */
    static const char *const scoped = DELETION_DUE_SELECT
        "o.deletion_uuid=?1 AND o.state<>'completed' AND o.next_attempt_at<=strftime('%s','now')" DELETION_DUE_FILTER;
    static const char *const sweep = DELETION_DUE_SELECT
        "o.id IN (SELECT id FROM storage_deletion_objects WHERE state<>'completed' "
        "AND next_attempt_at<=strftime('%s','now') ORDER BY next_attempt_at,id LIMIT ?1)" DELETION_DUE_FILTER;
    static time_t last_sweep;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    deletion_item_t items[32];
    int count = 0;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    if (sqlite3_prepare_v2(db, deletion_uuid ? scoped : sweep, -1, &statement, NULL) == SQLITE_OK) {
        if (deletion_uuid) sqlite3_bind_text(statement, 1, deletion_uuid, -1, SQLITE_TRANSIENT);
        else sqlite3_bind_int(statement, 1, STORAGE_DELETION_DUE_WINDOW);
        sqlite3_bind_int(statement, 2, local_only);
        sqlite3_bind_int(statement, 3, local_only ? 32 : 1);
        while (count < 32 && sqlite3_step(statement) == SQLITE_ROW) {
            deletion_item_t *item = &items[count++];
            memset(item, 0, sizeof(*item));
            item->id = sqlite3_column_int64(statement, 0);
            safe_strcpy(item->deletion, (const char *)sqlite3_column_text(statement, 1), sizeof(item->deletion), 0);
            safe_strcpy(item->target, (const char *)sqlite3_column_text(statement, 2), sizeof(item->target), 0);
            safe_strcpy(item->key, (const char *)sqlite3_column_text(statement, 3), sizeof(item->key), 0);
            safe_strcpy(item->path, (const char *)sqlite3_column_text(statement, 4), sizeof(item->path), 0);
            item->size = (uint64_t)sqlite3_column_int64(statement, 5);
            safe_strcpy(item->upload_id, (const char *)sqlite3_column_text(statement, 6), sizeof(item->upload_id), 0);
            const char *kind = (const char *)sqlite3_column_text(statement, 7);
            bool has_device = sqlite3_column_type(statement, 8) != SQLITE_NULL;
            bool has_inode = sqlite3_column_type(statement, 9) != SQLITE_NULL;
            int64_t device = sqlite3_column_int64(statement, 8), inode = sqlite3_column_int64(statement, 9);
            if (!kind && !has_device && !has_inode) item->stale_guard = 0;
            else if (kind && strcmp(kind, "missing") == 0 && !has_device && !has_inode) item->stale_guard = 1;
            else if (kind && strcmp(kind, "empty") == 0 && has_device && has_inode && device >= 0 && inode > 0) {
                item->stale_guard = 2; item->expected_device = (uint64_t)device; item->expected_inode = (uint64_t)inode;
            } else item->stale_guard = -1;
        }
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    for (int i = 0; i < count; i++) process_item(&items[i], local_only, removed);
    pthread_mutex_lock(mutex);
    if (deletion_uuid) {
        finalize_deletions_locked(db, deletion_uuid);
    } else {
        for (int i = 0; i < count; i++) {
            int seen = 0;
            for (int j = 0; j < i && !seen; j++) seen = strcmp(items[j].deletion, items[i].deletion) == 0;
            if (!seen) finalize_deletions_locked(db, items[i].deletion);
        }
        time_t now = time(NULL);
        if (now - last_sweep >= STORAGE_DELETION_SWEEP_INTERVAL_SEC) {
            last_sweep = now;
            finalize_deletions_locked(db, NULL);
        }
    }
    pthread_mutex_unlock(mutex);
    return count;
}

static bool recovery_read_only(void) {
    const char *value = getenv("LIGHTNVR_STORAGE_READ_ONLY");
    return value && !strcmp(value, "1");
}
int storage_deletion_process_one(void) { return recovery_read_only() ? 0 : process_due(NULL, false, NULL); }

int storage_deletion_finalize_all(void) {
    if (recovery_read_only()) return 0;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    int result = finalize_deletions_locked(db, NULL);
    pthread_mutex_unlock(mutex);
    return result;
}

/* Remove one batch of finished ledger entries. Both statements select the same
 * rows (same ordered window, one transaction, and the first touches only the
 * child table), so objects are removed explicitly even when the connection has
 * foreign-key enforcement off; with it on, the cascade finds nothing left. */
static int prune_batch_locked(sqlite3 *db, int64_t cutoff, int limit) {
    static const char *const steps[] = {
        "DELETE FROM storage_deletion_objects WHERE deletion_uuid IN (SELECT uuid FROM storage_deletions "
        "WHERE completed_at IS NOT NULL AND completed_at<?1 ORDER BY completed_at,rowid LIMIT ?2);",
        "DELETE FROM storage_deletions WHERE rowid IN (SELECT rowid FROM storage_deletions "
        "WHERE completed_at IS NOT NULL AND completed_at<?1 ORDER BY completed_at,rowid LIMIT ?2);"
    };
    if (sqlite3_exec(db, "BEGIN IMMEDIATE;", NULL, NULL, NULL) != SQLITE_OK) return -1;
    int pruned = 0;
    for (size_t i = 0; pruned >= 0 && i < 2; i++) {
        sqlite3_stmt *statement = NULL;
        int result = sqlite3_prepare_v2(db, steps[i], -1, &statement, NULL);
        if (result == SQLITE_OK) {
            sqlite3_bind_int64(statement, 1, (sqlite3_int64)cutoff);
            sqlite3_bind_int(statement, 2, limit);
            result = sqlite3_step(statement);
        }
        if (statement) sqlite3_finalize(statement);
        pruned = result == SQLITE_DONE ? sqlite3_changes(db) : -1;
    }
    if (pruned >= 0 && sqlite3_exec(db, "COMMIT;", NULL, NULL, NULL) == SQLITE_OK) return pruned;
    sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
    return -1;
}

int storage_deletion_prune_completed(int older_than_days, int max_rows) {
    if (recovery_read_only()) return 0;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    if (older_than_days <= 0) older_than_days = STORAGE_DELETION_PRUNE_DEFAULT_DAYS;
    if (max_rows <= 0) max_rows = STORAGE_DELETION_PRUNE_DEFAULT_ROWS;
    int64_t cutoff = (int64_t)time(NULL) - (int64_t)older_than_days * 86400;
    int pruned = 0;
    while (pruned < max_rows) {
        int limit = max_rows - pruned;
        if (limit > STORAGE_DELETION_PRUNE_BATCH) limit = STORAGE_DELETION_PRUNE_BATCH;
        pthread_mutex_lock(mutex);
        int batch = prune_batch_locked(db, cutoff, limit);
        pthread_mutex_unlock(mutex);
        if (batch < 0) return pruned ? pruned : -1;
        pruned += batch;
        if (batch < limit) break;
    }
    return pruned;
}

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

#define DERIVATIVE_PATHS_MAX 32

typedef struct {
    int count;
    char paths[DERIVATIVE_PATHS_MAX][MAX_PATH_LENGTH];
} derivative_paths_t;

static void derivative_add(derivative_paths_t *out, const char *path) {
    /* Beyond the cap the caller's post-deletion sweep removes the rest. */
    if (out->count >= DERIVATIVE_PATHS_MAX) return;
    safe_strcpy(out->paths[out->count++], path, MAX_PATH_LENGTH, 0);
}

/*
 * Probe the filesystem for a recording's derivatives (thumbnails, transcode
 * cache, investigation stills) before the ledger transaction opens. Journal
 * only derivatives that exist: a recording without any then costs one ledger
 * row and one unlink instead of five. These access() and readdir() calls
 * used to run inside BEGIN IMMEDIATE while holding the global database
 * mutex, so a slow volume stalled every API request for the length of each
 * deletion. Derivatives a still-running consumer writes after this point are
 * removed by the caller's post-deletion cleanup (storage_manager).
 */
static int collect_derivatives(uint64_t id, derivative_paths_t *out) {
    char path[MAX_PATH_LENGTH], directory[MAX_PATH_LENGTH];
    out->count = 0;
    for (int i = 0; i < 3; i++) {
        int n = snprintf(path, sizeof(path), "%s/thumbnails/%llu_%d.jpg", g_config.storage_path, (unsigned long long)id, i);
        if (n < 0 || n >= (int)sizeof(path)) return -1;
        if (access(path, F_OK) == 0) derivative_add(out, path);
    }
    if (!build_recording_transcode_cache_path(g_config.storage_path, id, path, sizeof(path)) &&
        access(path, F_OK) == 0) derivative_add(out, path);
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
        if (n < 0 || n >= (int)sizeof(path)) { result = -1; break; }
        derivative_add(out, path);
    }
    closedir(dir);
    return result;
}

static int inventory_derivatives(sqlite3 *db, const char *uuid, const derivative_paths_t *paths) {
    for (int i = 0; i < paths->count; i++) {
        if (inventory_path(db, uuid, paths->paths[i])) return -1;
    }
    return 0;
}

static int recording_delete(uint64_t id, const char *reason, uint64_t *removed,
                            bool require_policy_expiry, int64_t age_cutoff, const stale_prune_t *stale) {
    if (recovery_read_only()) return -1;
    if (removed) *removed = 0;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || !id) return -1;
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    if (lightnvr_uuid_generate_v4(uuid)) return -1;
    derivative_paths_t derivatives;
    if (collect_derivatives(id, &derivatives) != 0) return -1;
    if (stale && derivatives.count) return -2;
    pthread_mutex_lock(mutex);
    if (sqlite3_exec(db, "BEGIN IMMEDIATE;", NULL, NULL, NULL) != SQLITE_OK) {
        pthread_mutex_unlock(mutex);
        return -1;
    }
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, "SELECT protected,EXISTS(SELECT 1 FROM storage_migration_jobs "
        "WHERE recording_id=?1 AND state IN('copying','verifying','committing')),deletion_pending,is_complete,"
        "file_path,stream_name,start_time,COALESCE(end_time,0),size_bytes,COALESCE(storage_target_uuid,''),"
        "EXISTS(SELECT 1 FROM streams s WHERE s.name=r.stream_name AND s.enabled=0),"
        "EXISTS(SELECT 1 FROM storage_recording_copies WHERE recording_id=r.id),"
        "EXISTS(SELECT 1 FROM storage_migration_jobs WHERE recording_id=r.id),"
        "EXISTS(SELECT 1 FROM storage_retrieval_jobs WHERE recording_id=r.id) "
        "FROM recordings r WHERE id=?1;",
        -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_int64(statement, 1, (sqlite3_int64)id);
        result = sqlite3_step(statement);
    }
    /* Check inside the deletion transaction: an ID selected from the live
     * recordings list must never remove a file the recorder is writing. */
    bool stale_matches = false;
    if (stale && result == SQLITE_ROW) {
        const recording_metadata_t *expected = stale->expected;
        const char *path = (const char *)sqlite3_column_text(statement, 4);
        const char *stream = (const char *)sqlite3_column_text(statement, 5);
        const char *target = (const char *)sqlite3_column_text(statement, 9);
        stale_matches = !sqlite3_column_int(statement, 3) && path && stream && target && !target[0] &&
            strcmp(path, expected->file_path) == 0 && strcmp(stream, expected->stream_name) == 0 &&
            sqlite3_column_int64(statement, 6) == expected->start_time && expected->start_time < stale->cutoff &&
            sqlite3_column_int64(statement, 7) == expected->end_time && expected->end_time == 0 &&
            sqlite3_column_int64(statement, 8) == (sqlite3_int64)expected->size_bytes && expected->size_bytes == 0 &&
            sqlite3_column_int(statement, 10) && !sqlite3_column_int(statement, 11) &&
            !sqlite3_column_int(statement, 12) && !sqlite3_column_int(statement, 13);
    }
    bool blocked = result == SQLITE_ROW && (sqlite3_column_int(statement, 0) ||
        sqlite3_column_int(statement, 1) || (stale ? !stale_matches : !sqlite3_column_int(statement, 3)));
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
    if (result == SQLITE_DONE && inventory_derivatives(db, uuid, &derivatives)) result = SQLITE_ERROR;
    if (result == SQLITE_DONE && stale) {
        statement = NULL;
        result = sqlite3_prepare_v2(db, "UPDATE storage_deletion_objects SET stale_guard_kind=?,stale_guard_dev=?,stale_guard_ino=? WHERE deletion_uuid=?", -1, &statement, NULL);
        if (result == SQLITE_OK) {
            sqlite3_bind_text(statement, 1, stale->kind == 1 ? "missing" : "empty", -1, SQLITE_STATIC);
            if (stale->kind == 2) {
                sqlite3_bind_int64(statement, 2, (sqlite3_int64)stale->device);
                sqlite3_bind_int64(statement, 3, (sqlite3_int64)stale->inode);
            } else { sqlite3_bind_null(statement, 2); sqlite3_bind_null(statement, 3); }
            sqlite3_bind_text(statement, 4, uuid, -1, SQLITE_TRANSIENT);
            result = sqlite3_step(statement);
        }
        sqlite3_finalize(statement);
    }
    bool committed = result == SQLITE_DONE && sqlite3_exec(db, "COMMIT;", NULL, NULL, NULL) == SQLITE_OK;
    if (!committed) sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
    pthread_mutex_unlock(mutex);
    if (!committed) return -1;
    /* Preserve synchronous local reclamation for the pressure controller. Remote
     * work is handled by the background worker and remains in the journal. Only
     * this deletion's ledger entry is inspected and finalized here. */
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
    return recording_delete(id, reason, removed, false, 0, NULL);
}

int storage_recording_expire_policy(uint64_t id) {
    return recording_delete(id, "policy expiry", NULL, true, 0, NULL);
}

int storage_recording_expire_age(uint64_t id, int64_t cutoff) {
    if (!cutoff) return -2;
    return recording_delete(id, "legacy age retention", NULL, false, cutoff, NULL);
}

int storage_recording_prune_stale(const recording_metadata_t *expected,
                                   time_t cutoff, const struct stat *expected_empty) {
    if (!expected || !expected->id || expected->is_complete || expected->size_bytes || expected->end_time ||
        !expected->stream_name[0] || expected->start_time >= cutoff || cutoff > time(NULL) - 3600 ||
        !stale_path_is_authoritative(expected->file_path) ||
        stream_has_continuous_writer(expected->stream_name) || mp4_recording_thread_alive(expected->stream_name)) return -2;
    stale_prune_t guard = { .expected = expected, .cutoff = cutoff, .kind = expected_empty ? 2 : 1 };
    struct stat current;
    int status = lstat(expected->file_path, &current);
    if (expected_empty) {
        if (status != 0 || !S_ISREG(expected_empty->st_mode) || expected_empty->st_size ||
            !S_ISREG(current.st_mode) || current.st_size || current.st_dev != expected_empty->st_dev ||
            current.st_ino != expected_empty->st_ino || (uint64_t)current.st_dev > INT64_MAX ||
            (uint64_t)current.st_ino > INT64_MAX || !current.st_ino) return -2;
        guard.device = (uint64_t)current.st_dev; guard.inode = (uint64_t)current.st_ino;
    } else if (status == 0 || errno != ENOENT) return -2;
    return recording_delete(expected->id, "confirmed stale incomplete recording", NULL, false, 0, &guard);
}
