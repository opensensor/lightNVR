#define _POSIX_C_SOURCE 200809L
#include "storage/storage_source.h"

#include <errno.h>
#include <dirent.h>
#include <fcntl.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdatomic.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/statvfs.h>
#include <time.h>
#include <unistd.h>

#include "core/path_utils.h"
#include "database/db_core.h"
#include "database/db_storage_targets.h"
#include "storage/storage_s3.h"
#include "utils/strings.h"

#define SOURCE_LEASE_SECONDS 120
#define SOURCE_QUEUE_LIMIT 16

static pthread_t source_thread;
static pthread_mutex_t source_mutex = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t source_condition = PTHREAD_COND_INITIALIZER;
static bool source_running;
static atomic_bool source_stop;

typedef struct {
    uint64_t recording_id;
    uint64_t reserve_bytes;
    time_t space_checked_at;
} fetch_context_t;

/* Targets sharing the cache filesystem share its physical reserve too. */
static bool capture_space(uint64_t *available, uint64_t *reserve) {
    struct statvfs fs;
    struct stat info;
    if (statvfs(g_config.storage_path, &fs) || stat(g_config.storage_path, &info)) return false;
    unsigned pct = g_config.storage_min_free_pct > 0 ? (unsigned)g_config.storage_min_free_pct : 10;
    if (pct > 100) pct = 100;
    *available = (uint64_t)fs.f_bavail * fs.f_frsize;
    *reserve = (uint64_t)fs.f_blocks * fs.f_frsize / 100 * pct;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return false;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    bool found = false;
    if (sqlite3_prepare_v2(db, "SELECT COALESCE(MAX(reserve_bytes),0) FROM storage_targets "
        "WHERE target_type='filesystem' AND (filesystem_device=? OR filesystem_device=0);", -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, (sqlite3_int64)info.st_dev);
        if (sqlite3_step(stmt) == SQLITE_ROW) {
            uint64_t target_reserve = (uint64_t)sqlite3_column_int64(stmt, 0);
            if (target_reserve > *reserve) *reserve = target_reserve;
            found = true;
        }
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return found;
}

typedef struct {
    char target[LIGHTNVR_UUID_STRING_SIZE];
    char key[MAX_PATH_LENGTH];
    char path[MAX_PATH_LENGTH];
    char checksum[65];
    uint64_t bytes;
} recording_source_t;

static uint64_t cache_limit(void) {
    const char *value = getenv("LIGHTNVR_ARCHIVE_CACHE_MB");
    char *end = NULL;
    unsigned long long mb = value ? strtoull(value, &end, 10) : 2048;
    if (value && (!end || *end || mb > 1048576)) mb = 2048;
    return mb * 1024ULL * 1024ULL;
}

static bool source_lease(sqlite3 *db, uint64_t id) {
    bool saved = false;
    sqlite3_stmt *statement = NULL;
    if (sqlite3_prepare_v2(db, "INSERT INTO storage_read_leases(recording_id,expires_at) "
        "VALUES(?,strftime('%s','now')+120) ON CONFLICT(recording_id) DO UPDATE SET expires_at=excluded.expires_at;",
        -1, &statement, NULL) == SQLITE_OK) {
        sqlite3_bind_int64(statement, 1, (sqlite3_int64)id);
        saved = sqlite3_step(statement) == SQLITE_DONE && sqlite3_changes(db) == 1;
    }
    if (statement) sqlite3_finalize(statement);
    return saved;
}

/* Renew an existing response's lease even after logical deletion was accepted.
 * New readers still go through storage_source_touch and cannot enter then. */
bool storage_source_renew_lease(uint64_t id) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return false;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    bool renewed = false;
    if (sqlite3_prepare_v2(db, "UPDATE storage_read_leases SET expires_at=strftime('%s','now')+120 "
        "WHERE recording_id=? AND EXISTS(SELECT 1 FROM recordings WHERE id=recording_id);",
        -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, (sqlite3_int64)id);
        renewed = sqlite3_step(stmt) == SQLITE_DONE && sqlite3_changes(db) == 1;
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return renewed;
}

bool storage_source_touch(uint64_t id) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return false;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    bool ready = false;
    if (sqlite3_prepare_v2(db, "SELECT 1 FROM recordings WHERE id=? AND deletion_pending=0;", -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, (sqlite3_int64)id);
        ready = sqlite3_step(stmt) == SQLITE_ROW;
    }
    if (stmt) sqlite3_finalize(stmt);
    if (ready) ready = source_lease(db, id);
    stmt = NULL;
    if (ready && sqlite3_prepare_v2(db, "SELECT file_path FROM storage_retrieval_jobs WHERE recording_id=? AND state='ready';",
                                  -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, (sqlite3_int64)id);
        if (sqlite3_step(stmt) == SQLITE_ROW) {
            struct timespec times[2] = {{.tv_nsec = UTIME_NOW}, {.tv_nsec = UTIME_OMIT}};
            utimensat(AT_FDCWD, (const char *)sqlite3_column_text(stmt, 0), times, AT_SYMLINK_NOFOLLOW);
        }
    }
    if (stmt) sqlite3_finalize(stmt);
    stmt = NULL;
    if (ready && sqlite3_prepare_v2(db, "UPDATE storage_retrieval_jobs SET last_access_at=strftime('%s','now') WHERE recording_id=?;",
                                  -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, (sqlite3_int64)id);
        sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return ready;
}


bool storage_source_has_lease(uint64_t id) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return true;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    bool active = true;
    if (sqlite3_prepare_v2(db, "SELECT 1 FROM storage_read_leases WHERE recording_id=? AND expires_at>strftime('%s','now');",
                         -1, &statement, NULL) == SQLITE_OK) {
        sqlite3_bind_int64(statement, 1, (sqlite3_int64)id);
        active = sqlite3_step(statement) != SQLITE_DONE;
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return active;
}

/* During catalog failure only reclaim old, completed files in our cache.
 * Never walk recording directories, follow symlinks, or remove partial fetches. */
static uint64_t trim_cache_without_catalog(uint64_t needed) {
    int root = open(g_config.storage_path, O_RDONLY | O_DIRECTORY | O_CLOEXEC | O_NOFOLLOW);
    if (root < 0) return 0;
    int fd = openat(root, "archive-cache", O_RDONLY | O_DIRECTORY | O_CLOEXEC | O_NOFOLLOW);
    close(root);
    if (fd < 0) return 0;
    DIR *directory = fdopendir(fd);
    if (!directory) { close(fd); return 0; }
    uint64_t removed = 0;
    time_t cutoff = time(NULL) - (needed ? SOURCE_LEASE_SECONDS : 3600);
    struct dirent *entry;
    int scanned = 0, deleted = 0;
    while (scanned++ < 4096 && deleted < 32 && (entry = readdir(directory))) {
        const char *name = entry->d_name, *p = name;
        while (*p >= '0' && *p <= '9') p++;
        if (p == name || p - name > 20 || *p++ != '-') continue;
        if (strlen(p) != 68 || strcmp(p + 64, ".mp4")) continue;
        bool valid = true;
        for (int i = 0; i < 64; i++)
            if (!((p[i] >= '0' && p[i] <= '9') || (p[i] >= 'a' && p[i] <= 'f'))) valid = false;
        struct stat info;
        if (!valid || fstatat(fd, name, &info, AT_SYMLINK_NOFOLLOW) || !S_ISREG(info.st_mode) ||
            info.st_mtime >= cutoff || info.st_atime >= cutoff) continue;
        if (!unlinkat(fd, name, 0)) {
            removed += (uint64_t)info.st_size;
            deleted++;
            if (needed && removed >= needed) break;
        }
    }
    closedir(directory);
    return removed;
}

uint64_t storage_source_trim(uint64_t needed) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return trim_cache_without_catalog(needed);
    uint64_t removed = 0;
    bool catalog_failed = false;
    /* Serialize selection/unlink against source resolution. Only app-owned,
     * completed cache files are involved; no remote I/O occurs under this lock. */
    pthread_mutex_lock(mutex);
    for (int n = 0; n < 32; n++) {
        sqlite3_stmt *statement = NULL;
        uint64_t id = 0, bytes = 0;
        char path[MAX_PATH_LENGTH] = {0};
        const char *sql = "SELECT recording_id,file_path,size_bytes FROM storage_retrieval_jobs j "
            "WHERE state IN('ready','failed') AND last_access_at<strftime('%s','now')-? "
            "AND NOT EXISTS(SELECT 1 FROM storage_read_leases l WHERE l.recording_id=j.recording_id "
            "AND l.expires_at>strftime('%s','now')) ORDER BY last_access_at LIMIT 1;";
        if (sqlite3_prepare_v2(db, sql, -1, &statement, NULL) == SQLITE_OK) {
            sqlite3_bind_int(statement, 1, needed ? SOURCE_LEASE_SECONDS : 3600);
            int rc = sqlite3_step(statement);
            catalog_failed = rc != SQLITE_ROW && rc != SQLITE_DONE;
            if (rc == SQLITE_ROW) {
                id = (uint64_t)sqlite3_column_int64(statement, 0);
                safe_strcpy(path, (const char *)sqlite3_column_text(statement, 1), sizeof(path), 0);
                bytes = (uint64_t)sqlite3_column_int64(statement, 2);
            }
        } else catalog_failed = true;
        if (statement) sqlite3_finalize(statement);
        if (!id) break;
        int deleted = unlink(path);
        if (deleted != 0 && errno != ENOENT) break;
        if (deleted == 0) removed += bytes;
        statement = NULL;
        if (sqlite3_prepare_v2(db, "DELETE FROM storage_retrieval_jobs WHERE recording_id=?;", -1, &statement, NULL) == SQLITE_OK) {
            sqlite3_bind_int64(statement, 1, (sqlite3_int64)id);
            sqlite3_step(statement);
        }
        if (statement) sqlite3_finalize(statement);
        if (needed && removed >= needed) break;
    }
    pthread_mutex_unlock(mutex);
    if (catalog_failed) removed += trim_cache_without_catalog(needed > removed ? needed - removed : 0);
    return removed;
}

/* Selection is shared by playback, staging and status; it never queues work or
 * acquires a lease. A failed retrieval demotes that replica, while an in-flight
 * fetch stays pinned to its persisted identity. */
static int select_source(uint64_t id, recording_source_t *selected, bool *available, char error[256]) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    *available = false;
    if (!db || !mutex) return STORAGE_SOURCE_ERROR;
    recording_source_t sources[9];
    int count = 0;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    const char *sql = "SELECT COALESCE(storage_target_uuid,''),COALESCE(object_key,''),file_path,archive_checksum,size_bytes,deletion_pending "
        "FROM recordings WHERE id=?1 UNION ALL "
        "SELECT c.target_uuid,c.object_key,'',c.checksum,c.size_bytes,r.deletion_pending "
        "FROM storage_recording_copies c JOIN recordings r ON r.id=c.recording_id WHERE r.id=?1 UNION ALL "
        "SELECT j.source_target_uuid,j.source_object_key,'',j.checksum,j.bytes_total,r.deletion_pending "
        "FROM storage_migration_jobs j JOIN recordings r ON r.id=j.recording_id "
        "WHERE r.id=?1 AND j.state='cleanup_pending' LIMIT 9;";
    int rc = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(statement, 1, (sqlite3_int64)id);
        while (count < 9 && (rc = sqlite3_step(statement)) == SQLITE_ROW) {
            if (sqlite3_column_int(statement, 5)) {
                sqlite3_finalize(statement);
                pthread_mutex_unlock(mutex);
                safe_strcpy(error, "Recording deletion is pending", 256, 0);
                return STORAGE_SOURCE_DELETING;
            }
            recording_source_t *source = &sources[count++];
            memset(source, 0, sizeof(*source));
            safe_strcpy(source->target, (const char *)sqlite3_column_text(statement, 0), sizeof(source->target), 0);
            safe_strcpy(source->key, (const char *)sqlite3_column_text(statement, 1), sizeof(source->key), 0);
            safe_strcpy(source->path, (const char *)sqlite3_column_text(statement, 2), sizeof(source->path), 0);
            safe_strcpy(source->checksum, (const char *)sqlite3_column_text(statement, 3), sizeof(source->checksum), 0);
            source->bytes = (uint64_t)sqlite3_column_int64(statement, 4);
        }
    }
    if (statement) sqlite3_finalize(statement);
    char job_target[37] = {0}, job_key[MAX_PATH_LENGTH] = {0}, job_state[16] = {0};
    statement = NULL;
    if (sqlite3_prepare_v2(db, "SELECT target_uuid,object_key,state FROM storage_retrieval_jobs WHERE recording_id=?;",
                          -1, &statement, NULL) == SQLITE_OK) {
        sqlite3_bind_int64(statement, 1, (sqlite3_int64)id);
        if (sqlite3_step(statement) == SQLITE_ROW) {
            safe_strcpy(job_target, (const char *)sqlite3_column_text(statement, 0), sizeof(job_target), 0);
            safe_strcpy(job_key, (const char *)sqlite3_column_text(statement, 1), sizeof(job_key), 0);
            safe_strcpy(job_state, (const char *)sqlite3_column_text(statement, 2), sizeof(job_state), 0);
        }
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    if (!count) return rc == SQLITE_DONE ? STORAGE_SOURCE_MISSING : STORAGE_SOURCE_ERROR;
    int best = -10000;
    bool external = false;
    for (int i = 0; i < count; i++) {
        recording_source_t *source = &sources[i];
        if (source->target[0]) {
            storage_target_t target;
            if (db_storage_target_get(source->target, &target) != DB_STORAGE_TARGET_OK) continue;
            if (!strcmp(target.target_type, "s3")) {
                if (strlen(source->checksum) != 64) continue;
                bool current = !strcmp(job_target, source->target) && !strcmp(job_key, source->key);
                bool failed = current && !strcmp(job_state, "failed");
                bool healthy = strcmp(target.health_status, "unavailable") != 0;
                int score = (healthy ? 100 : 0) - (failed ? 200 : 0);
                if (current && !failed) score += 500;
                if (current && !strcmp(job_state, "fetching")) score = 1000;
                if (score > best) {
                    *selected = *source;
                    *available = healthy && !failed;
                    external = true;
                    best = score;
                }
                continue;
            }
            if (!db_storage_target_mount_guard_active(&target) ||
                db_storage_target_resolve_path(source->target, source->key, source->path)) continue;
        }
        struct stat status;
        if (source->path[0] && stat(source->path, &status) == 0 && S_ISREG(status.st_mode)) {
            *selected = *source;
            *available = true;
            return STORAGE_SOURCE_READY;
        }
    }
    if (external) {
        // A verified completed cache remains usable while its provider is down.
        if (!strcmp(job_state, "ready") && !strcmp(job_target, selected->target) && !strcmp(job_key, selected->key)) {
            char cached[MAX_PATH_LENGTH];
            struct stat info;
            int n = snprintf(cached, sizeof(cached), "%s/archive-cache/%llu-%s.mp4", g_config.storage_path,
                             (unsigned long long)id, selected->checksum);
            if (n > 0 && n < (int)sizeof(cached) && stat(cached, &info) == 0 && S_ISREG(info.st_mode) &&
                (uint64_t)info.st_size == selected->bytes) {
                safe_strcpy(selected->path, cached, sizeof(selected->path), 0);
                *available = true;
                return STORAGE_SOURCE_READY;
            }
        }
        return STORAGE_SOURCE_PREPARING;
    }
    safe_strcpy(error, "No readable recording copy is currently available", 256, 0);
    return STORAGE_SOURCE_MISSING;
}

bool storage_source_available(uint64_t id) {
    recording_source_t source;
    char error[256] = {0};
    bool available;
    select_source(id, &source, &available, error);
    return available;
}

int storage_source_remote(uint64_t id, storage_remote_source_t *source) {
    if (!source) return STORAGE_SOURCE_ERROR;
    recording_source_t selected;
    char error[256] = {0};
    bool available;
    int result = select_source(id, &selected, &available, error);
    if (result == STORAGE_SOURCE_READY) return STORAGE_SOURCE_MISSING; // Local replica wins.
    if (result != STORAGE_SOURCE_PREPARING) return result;
    if (!storage_source_touch(id)) return STORAGE_SOURCE_DELETING;
    memset(source, 0, sizeof(*source));
    source->recording_id = id;
    safe_strcpy(source->key, selected.key, sizeof(source->key), 0);
    source->size = selected.bytes;
    return db_storage_target_get(selected.target, &source->target) == DB_STORAGE_TARGET_OK ?
        STORAGE_SOURCE_READY : STORAGE_SOURCE_ERROR;
}

int storage_source_resolve(uint64_t id, char path[MAX_PATH_LENGTH], char error[256]) {
    if (!id || !path || !error) return STORAGE_SOURCE_ERROR;
    path[0] = 0; error[0] = 0;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return STORAGE_SOURCE_ERROR;
    recording_source_t selected;
    bool available;
    int result = select_source(id, &selected, &available, error);
    if (result == STORAGE_SOURCE_READY) {
        if (!storage_source_touch(id)) return STORAGE_SOURCE_DELETING;
        safe_strcpy(path, selected.path, MAX_PATH_LENGTH, 0);
        return STORAGE_SOURCE_READY;
    }
    if (result != STORAGE_SOURCE_PREPARING) return result;
    if (!storage_source_touch(id)) return STORAGE_SOURCE_DELETING;
    recording_source_t *source = &selected;
    if (strlen(source->checksum) != 64) {
        safe_strcpy(error, "Archive copy has no verified checksum", 256, 0);
        return STORAGE_SOURCE_ERROR;
    }
    int n = snprintf(path, MAX_PATH_LENGTH, "%s/archive-cache/%llu-%s.mp4", g_config.storage_path,
                     (unsigned long long)id, source->checksum);
    if (n < 0 || n >= MAX_PATH_LENGTH || ensure_path(path)) {
        safe_strcpy(error, "Cannot create recording retrieval cache", 256, 0);
        return STORAGE_SOURCE_ERROR;
    }
    storage_source_trim(source->bytes);
    uint64_t free_bytes, reserve;
    if (!capture_space(&free_bytes, &reserve)) return STORAGE_SOURCE_ERROR;
    uint64_t limit = cache_limit();
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int state = STORAGE_SOURCE_PREPARING;
    if (sqlite3_prepare_v2(db, "SELECT state,last_error,next_attempt_at FROM storage_retrieval_jobs "
        "WHERE recording_id=? AND target_uuid=? AND object_key=?;", -1, &statement, NULL) == SQLITE_OK) {
        sqlite3_bind_int64(statement, 1, (sqlite3_int64)id);
        sqlite3_bind_text(statement, 2, source->target, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 3, source->key, -1, SQLITE_TRANSIENT);
        if (sqlite3_step(statement) == SQLITE_ROW) {
            const char *value = (const char *)sqlite3_column_text(statement, 0);
            struct stat info;
            if (!strcmp(value, "ready") && stat(path, &info) == 0 && (uint64_t)info.st_size == source->bytes)
                state = STORAGE_SOURCE_READY;
            else if (!strcmp(value, "failed") && sqlite3_column_int64(statement, 2) > time(NULL)) {
                safe_strcpy(error, (const char *)sqlite3_column_text(statement, 1), 256, 0);
                state = STORAGE_SOURCE_ERROR;
            }
        }
    }
    if (statement) sqlite3_finalize(statement);
    if (state == STORAGE_SOURCE_PREPARING) {
        uint64_t used = 0, pending_bytes = 0;
        int jobs = SOURCE_QUEUE_LIMIT;
        statement = NULL;
        if (sqlite3_prepare_v2(db, "SELECT COALESCE(sum(size_bytes),0),"
            "COALESCE(sum(CASE WHEN state IN('queued','fetching') THEN size_bytes ELSE 0 END),0),"
            "COALESCE(sum(state IN('queued','fetching')),0) FROM storage_retrieval_jobs WHERE recording_id<>?;", -1, &statement, NULL) == SQLITE_OK) {
            sqlite3_bind_int64(statement, 1, (sqlite3_int64)id);
            if (sqlite3_step(statement) == SQLITE_ROW) {
                used = (uint64_t)sqlite3_column_int64(statement, 0);
                pending_bytes = (uint64_t)sqlite3_column_int64(statement, 1);
                jobs = sqlite3_column_int(statement, 2);
            }
        }
        if (statement) sqlite3_finalize(statement);
        if (source->bytes > limit || used > limit - source->bytes || free_bytes <= reserve ||
            pending_bytes > free_bytes - reserve || source->bytes > free_bytes - reserve - pending_bytes ||
            jobs >= SOURCE_QUEUE_LIMIT) {
            safe_strcpy(error, "Archive playback cache is full or capture reserve would be exceeded", 256, 0);
            state = STORAGE_SOURCE_ERROR;
        } else {
            statement = NULL;
            const char *insert = "INSERT INTO storage_retrieval_jobs(recording_id,target_uuid,object_key,checksum,size_bytes,file_path) "
                "SELECT ?1,?2,?3,?4,?5,?6 FROM recordings WHERE id=?1 AND deletion_pending=0 "
                "ON CONFLICT(recording_id) DO UPDATE SET state=CASE WHEN storage_retrieval_jobs.state IN('ready','failed') "
                "THEN 'queued' ELSE storage_retrieval_jobs.state END,last_access_at=strftime('%s','now'),"
                "target_uuid=excluded.target_uuid,object_key=excluded.object_key,checksum=excluded.checksum,"
                "size_bytes=excluded.size_bytes,file_path=excluded.file_path "
                "WHERE storage_retrieval_jobs.state<>'fetching';";
            if (sqlite3_prepare_v2(db, insert, -1, &statement, NULL) == SQLITE_OK) {
                sqlite3_bind_int64(statement, 1, (sqlite3_int64)id);
                sqlite3_bind_text(statement, 2, source->target, -1, SQLITE_TRANSIENT);
                sqlite3_bind_text(statement, 3, source->key, -1, SQLITE_TRANSIENT);
                sqlite3_bind_text(statement, 4, source->checksum, -1, SQLITE_TRANSIENT);
                sqlite3_bind_int64(statement, 5, (sqlite3_int64)source->bytes);
                sqlite3_bind_text(statement, 6, path, -1, SQLITE_TRANSIENT);
                if (sqlite3_step(statement) != SQLITE_DONE) state = STORAGE_SOURCE_ERROR;
            } else state = STORAGE_SOURCE_ERROR;
            if (statement) sqlite3_finalize(statement);
        }
    }
    if (state == STORAGE_SOURCE_READY) {
        statement = NULL;
        if (sqlite3_prepare_v2(db, "UPDATE storage_retrieval_jobs SET last_access_at=strftime('%s','now') WHERE recording_id=?;",
                              -1, &statement, NULL) == SQLITE_OK) {
            sqlite3_bind_int64(statement, 1, (sqlite3_int64)id);
            sqlite3_step(statement);
        }
        if (statement) sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    if (state == STORAGE_SOURCE_PREPARING) {
        pthread_mutex_lock(&source_mutex);
        pthread_cond_signal(&source_condition);
        pthread_mutex_unlock(&source_mutex);
    }
    return state;
}

static bool fetch_cancelled(void *context) {
    if (atomic_load(&source_stop)) return true;
    if (!context) return false;
    fetch_context_t *fetch = context;
    uint64_t id = fetch->recording_id;
    time_t now = time(NULL);
    if (now != fetch->space_checked_at) {
        struct statvfs fs;
        fetch->space_checked_at = now;
        if (statvfs(g_config.storage_path, &fs) ||
            (uint64_t)fs.f_bavail * fs.f_frsize <= fetch->reserve_bytes) return true;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return true;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    bool cancel = true;
    if (sqlite3_prepare_v2(db, "SELECT deletion_pending FROM recordings WHERE id=?;", -1, &statement, NULL) == SQLITE_OK) {
        sqlite3_bind_int64(statement, 1, (sqlite3_int64)id);
        if (sqlite3_step(statement) == SQLITE_ROW) cancel = sqlite3_column_int(statement, 0) != 0;
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return cancel;
}

int storage_source_process_one(void) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    recording_source_t source = {0};
    uint64_t id = 0;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    if (sqlite3_prepare_v2(db, "SELECT j.recording_id,j.target_uuid,j.object_key,j.checksum,j.size_bytes,j.file_path "
        "FROM storage_retrieval_jobs j JOIN recordings r ON r.id=j.recording_id WHERE r.deletion_pending=0 "
        "AND j.state IN('queued','fetching') ORDER BY j.last_access_at LIMIT 1;", -1, &statement, NULL) == SQLITE_OK &&
        sqlite3_step(statement) == SQLITE_ROW) {
        id = (uint64_t)sqlite3_column_int64(statement, 0);
        safe_strcpy(source.target, (const char *)sqlite3_column_text(statement, 1), sizeof(source.target), 0);
        safe_strcpy(source.key, (const char *)sqlite3_column_text(statement, 2), sizeof(source.key), 0);
        safe_strcpy(source.checksum, (const char *)sqlite3_column_text(statement, 3), sizeof(source.checksum), 0);
        source.bytes = (uint64_t)sqlite3_column_int64(statement, 4);
        safe_strcpy(source.path, (const char *)sqlite3_column_text(statement, 5), sizeof(source.path), 0);
    }
    if (statement) sqlite3_finalize(statement);
    statement = NULL;
    if (id && sqlite3_prepare_v2(db, "UPDATE storage_retrieval_jobs SET state='fetching' WHERE recording_id=?;", -1, &statement, NULL) == SQLITE_OK) {
        sqlite3_bind_int64(statement, 1, (sqlite3_int64)id);
        sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    if (!id) return 0;
    storage_target_t target;
    char error[256] = {0}, temporary[MAX_PATH_LENGTH];
    int n = snprintf(temporary, sizeof(temporary), "%s.part", source.path);
    int result = -1;
    uint64_t available = 0, reserve = 0;
    bool headroom = capture_space(&available, &reserve);
    fetch_context_t fetch = {.recording_id = id, .reserve_bytes = reserve};
    headroom = headroom && available > reserve && source.bytes <= available - reserve;
    if (!headroom) safe_strcpy(error, "Archive retrieval waits for capture headroom", sizeof(error), 0);
    if (headroom && n > 0 && n < (int)sizeof(temporary) && db_storage_target_get(source.target, &target) == DB_STORAGE_TARGET_OK) {
        unlink(temporary);
        storage_transfer_control_t control = {.cancelled = fetch_cancelled, .context = &fetch,
                                              .bandwidth_bps = target.migration_bandwidth_bps};
        result = storage_s3_download(&target, source.key, temporary, source.bytes, source.checksum, &control, error);
        if (result == 0 && rename(temporary, source.path)) result = -1;
        if (result != 0) unlink(temporary);
    }
    if (fetch_cancelled(&fetch)) {
        unlink(source.path);
        result = -1;
    }
    pthread_mutex_lock(mutex);
    statement = NULL;
    if (sqlite3_prepare_v2(db, "UPDATE storage_retrieval_jobs SET state=?,last_error=?,next_attempt_at=strftime('%s','now')+30 "
        "WHERE recording_id=?3;", -1, &statement, NULL) == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, result == 0 ? "ready" : "failed", -1, SQLITE_STATIC);
        sqlite3_bind_text(statement, 2, result == 0 ? "" : (*error ? error : "Archive retrieval failed"), -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 3, (sqlite3_int64)id);
        if (sqlite3_step(statement) != SQLITE_DONE || sqlite3_changes(db) != 1) unlink(source.path);
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return 1;
}

static void *source_main(void *unused) {
    (void)unused;
    while (!atomic_load(&source_stop)) {
        storage_source_trim(0);
        if (storage_source_process_one() > 0) continue;
        pthread_mutex_lock(&source_mutex);
        if (!atomic_load(&source_stop)) {
            struct timespec deadline;
            clock_gettime(CLOCK_REALTIME, &deadline);
            deadline.tv_sec++;
            pthread_cond_timedwait(&source_condition, &source_mutex, &deadline);
        }
        pthread_mutex_unlock(&source_mutex);
    }
    return NULL;
}

int storage_source_worker_start(void) {
    pthread_mutex_lock(&source_mutex);
    if (source_running) { pthread_mutex_unlock(&source_mutex); return 0; }
    atomic_store(&source_stop, false);
    int result = pthread_create(&source_thread, NULL, source_main, NULL);
    source_running = result == 0;
    pthread_mutex_unlock(&source_mutex);
    return result == 0 ? 0 : -1;
}

void storage_source_worker_shutdown(void) {
    pthread_mutex_lock(&source_mutex);
    bool running = source_running;
    atomic_store(&source_stop, true);
    pthread_cond_signal(&source_condition);
    pthread_mutex_unlock(&source_mutex);
    if (running) pthread_join(source_thread, NULL);
    pthread_mutex_lock(&source_mutex);
    source_running = false;
    pthread_mutex_unlock(&source_mutex);
}
