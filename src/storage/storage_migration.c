#define _GNU_SOURCE
#define _POSIX_C_SOURCE 200809L

#include "storage/storage_migration.h"

#include <errno.h>
#include <fcntl.h>
#include <libgen.h>
#include <mbedtls/sha256.h>
#include <pthread.h>
#include <stdatomic.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

#include "core/logger.h"
#include "core/path_utils.h"
#include "database/db_storage_lifecycle.h"
#include "database/db_core.h"
#include <sqlite3.h>
#include "database/db_storage_migrations.h"
#include "database/db_storage_targets.h"
#include "storage/storage_target_health.h"
#include "storage/storage_s3.h"
#include "storage/storage_deletion.h"
#include "storage/storage_source.h"
#include "utils/strings.h"

#define MIGRATION_COPY_BUFFER (256U * 1024U)
#define MIGRATION_PROGRESS_INTERVAL (4ULL * 1024ULL * 1024ULL)

typedef struct {
    pthread_mutex_t mutex;
    pthread_cond_t condition;
    pthread_t thread;
    bool running;
    bool exited;
} migration_worker_control_t;

static migration_worker_control_t worker = {
    .mutex = PTHREAD_MUTEX_INITIALIZER,
    .condition = PTHREAD_COND_INITIALIZER,
    .running = false,
    .exited = true,
};
static atomic_bool stop_requested = false;

static bool migration_cancel_requested(const storage_migration_job_t *job) {
    return job && db_storage_migration_cancel_requested(job->uuid);
}

static bool transfer_cancelled(void *context) {
    return atomic_load(&stop_requested) || migration_cancel_requested(context);
}

static int throttle_copy(const storage_migration_job_t *job,
                         const struct timespec *started, uint64_t copied) {
    if (!job || job->bandwidth_limit_bps == 0 || !started) return 0;
    struct timespec now;
    if (clock_gettime(CLOCK_MONOTONIC, &now) != 0) return 0;
    long double expected_ns =
        ((long double)copied * 1000000000.0L) /
        (long double)job->bandwidth_limit_bps;
    int64_t elapsed_ns =
        (int64_t)(now.tv_sec - started->tv_sec) * 1000000000LL +
        (int64_t)(now.tv_nsec - started->tv_nsec);
    if (expected_ns <= (long double)elapsed_ns) return 0;
    int64_t wait_ns = (int64_t)(expected_ns - (long double)elapsed_ns);
    struct timespec delay = {
        .tv_sec = wait_ns / 1000000000LL,
        .tv_nsec = wait_ns % 1000000000LL,
    };
    while (nanosleep(&delay, &delay) != 0 && errno == EINTR) {
        if (atomic_load(&stop_requested)) return -1;
        if (migration_cancel_requested(job)) return 1;
    }
    return migration_cancel_requested(job) ? 1 : 0;
}

static void set_error(char error[STORAGE_MIGRATION_ERROR_MAX],
                      const char *operation, const char *path) {
    int saved_errno = errno;
    snprintf(error, STORAGE_MIGRATION_ERROR_MAX, "%s%s%s: %s",
             operation, path ? " " : "", path ? path : "",
             strerror(saved_errno));
}

static void digest_hex(const unsigned char digest[32],
                       char output[STORAGE_MIGRATION_CHECKSUM_MAX]) {
    static const char symbols[] = "0123456789abcdef";
    for (size_t index = 0; index < 32; index++) {
        output[index * 2] = symbols[digest[index] >> 4];
        output[index * 2 + 1] = symbols[digest[index] & 0x0f];
    }
    output[64] = '\0';
}

static int finish_digest(mbedtls_sha256_context *context,
                         char output[STORAGE_MIGRATION_CHECKSUM_MAX]) {
    unsigned char digest[32];
    int result = mbedtls_sha256_finish(context, digest);
    mbedtls_sha256_free(context);
    if (result != 0) return -1;
    digest_hex(digest, output);
    return 0;
}

static int sha256_file(const char *path, uint64_t *size,
                       char checksum[STORAGE_MIGRATION_CHECKSUM_MAX],
                       char error[STORAGE_MIGRATION_ERROR_MAX]) {
    int descriptor = open(path, O_RDONLY | O_CLOEXEC | O_NOFOLLOW);
    if (descriptor < 0) {
        set_error(error, "Cannot open", path);
        return -1;
    }
    struct stat status;
    if (fstat(descriptor, &status) != 0 || !S_ISREG(status.st_mode) ||
        status.st_size < 0) {
        if (errno == 0) errno = EINVAL;
        set_error(error, "Not a readable regular file", path);
        close(descriptor);
        return -1;
    }
    mbedtls_sha256_context context;
    mbedtls_sha256_init(&context);
    if (mbedtls_sha256_starts(&context, 0) != 0) {
        mbedtls_sha256_free(&context);
        close(descriptor);
        safe_strcpy(error, "Could not initialize SHA-256",
                    STORAGE_MIGRATION_ERROR_MAX, 0);
        return -1;
    }
    unsigned char *buffer = malloc(MIGRATION_COPY_BUFFER);
    if (!buffer) {
        mbedtls_sha256_free(&context);
        close(descriptor);
        safe_strcpy(error, "Out of memory while verifying recording",
                    STORAGE_MIGRATION_ERROR_MAX, 0);
        return -1;
    }
    uint64_t total = 0;
    int result = 0;
    for (;;) {
        ssize_t count = read(descriptor, buffer, MIGRATION_COPY_BUFFER);
        if (count == 0) break;
        if (count < 0) {
            if (errno == EINTR) continue;
            set_error(error, "Cannot read", path);
            result = -1;
            break;
        }
        if (mbedtls_sha256_update(&context, buffer, (size_t)count) != 0) {
            safe_strcpy(error, "SHA-256 update failed",
                        STORAGE_MIGRATION_ERROR_MAX, 0);
            result = -1;
            break;
        }
        total += (uint64_t)count;
    }
    free(buffer);
    close(descriptor);
    if (result == 0 && finish_digest(&context, checksum) != 0) {
        safe_strcpy(error, "Could not finish SHA-256",
                    STORAGE_MIGRATION_ERROR_MAX, 0);
        result = -1;
    } else if (result != 0) {
        mbedtls_sha256_free(&context);
    }
    if (result == 0 && size) *size = total;
    return result;
}

static int write_all(int descriptor, const unsigned char *buffer,
                     size_t length) {
    size_t offset = 0;
    while (offset < length) {
        ssize_t written = write(descriptor, buffer + offset, length - offset);
        if (written < 0) {
            if (errno == EINTR) continue;
            return -1;
        }
        if (written == 0) {
            errno = EIO;
            return -1;
        }
        offset += (size_t)written;
    }
    return 0;
}

static int fsync_parent(const char *path) {
    char buffer[MAX_PATH_LENGTH];
    safe_strcpy(buffer, path, sizeof(buffer), 0);
    char *parent = dirname(buffer);
    int descriptor = open(parent, O_RDONLY | O_DIRECTORY | O_CLOEXEC |
                          O_NOFOLLOW);
    if (descriptor < 0) return -1;
    int result = fsync(descriptor);
    close(descriptor);
    return result;
}

static int verified_existing_destination(
    const char *source_path, const char *destination_path,
    uint64_t *bytes_total, char checksum[STORAGE_MIGRATION_CHECKSUM_MAX],
    char error[STORAGE_MIGRATION_ERROR_MAX]) {
    struct stat destination_status;
    if (lstat(destination_path, &destination_status) != 0) {
        if (errno == ENOENT) return 0;
        set_error(error, "Cannot inspect destination", destination_path);
        return -1;
    }
    if (!S_ISREG(destination_status.st_mode)) {
        safe_strcpy(error, "Destination exists and is not a regular file",
                    STORAGE_MIGRATION_ERROR_MAX, 0);
        return -2;
    }
    char source_checksum[STORAGE_MIGRATION_CHECKSUM_MAX];
    char destination_checksum[STORAGE_MIGRATION_CHECKSUM_MAX];
    uint64_t source_size = 0;
    uint64_t destination_size = 0;
    if (sha256_file(source_path, &source_size, source_checksum, error) != 0 ||
        sha256_file(destination_path, &destination_size,
                    destination_checksum, error) != 0) {
        return -1;
    }
    if (source_size != destination_size ||
        strcmp(source_checksum, destination_checksum) != 0) {
        safe_strcpy(error,
                    "Destination path already contains different data",
                    STORAGE_MIGRATION_ERROR_MAX, 0);
        return -2;
    }
    *bytes_total = source_size;
    safe_strcpy(checksum, source_checksum,
                STORAGE_MIGRATION_CHECKSUM_MAX, 0);
    return 1;
}

static int copy_and_verify(
    storage_migration_job_t *job, const char *source_path,
    const char *destination_path,
    char checksum[STORAGE_MIGRATION_CHECKSUM_MAX],
    char error[STORAGE_MIGRATION_ERROR_MAX]) {
    int existing = verified_existing_destination(
        source_path, destination_path, &job->bytes_total, checksum, error);
    if (existing != 0) return existing > 0 ? 0 : existing;

    char temporary_path[MAX_PATH_LENGTH];
    int length = snprintf(temporary_path, sizeof(temporary_path),
                          "%s.migration-%s.part", destination_path,
                          job->uuid);
    if (length < 0 || (size_t)length >= sizeof(temporary_path)) {
        safe_strcpy(error, "Destination path is too long for migration",
                    STORAGE_MIGRATION_ERROR_MAX, 0);
        return -2;
    }
    if (ensure_path(temporary_path) != 0) {
        set_error(error, "Cannot create destination directory for",
                  destination_path);
        return -1;
    }
    int source = open(source_path, O_RDONLY | O_CLOEXEC | O_NOFOLLOW);
    if (source < 0) {
        set_error(error, "Cannot open source", source_path);
        return -1;
    }
    struct stat source_status;
    if (fstat(source, &source_status) != 0 ||
        !S_ISREG(source_status.st_mode) || source_status.st_size < 0) {
        if (errno == 0) errno = EINVAL;
        set_error(error, "Source is not a complete regular file", source_path);
        close(source);
        return -2;
    }
    int destination = open(temporary_path,
                           O_WRONLY | O_CREAT | O_TRUNC | O_CLOEXEC |
                               O_NOFOLLOW,
                           source_status.st_mode & 0777);
    if (destination < 0) {
        set_error(error, "Cannot create temporary destination",
                  temporary_path);
        close(source);
        return -1;
    }
    unsigned char *buffer = malloc(MIGRATION_COPY_BUFFER);
    if (!buffer) {
        safe_strcpy(error, "Out of memory while copying recording",
                    STORAGE_MIGRATION_ERROR_MAX, 0);
        close(destination);
        close(source);
        return -1;
    }
    mbedtls_sha256_context context;
    mbedtls_sha256_init(&context);
    if (mbedtls_sha256_starts(&context, 0) != 0) {
        safe_strcpy(error, "Could not initialize SHA-256",
                    STORAGE_MIGRATION_ERROR_MAX, 0);
        mbedtls_sha256_free(&context);
        free(buffer);
        close(destination);
        close(source);
        return -1;
    }
    uint64_t copied = 0;
    uint64_t reported = 0;
    int result = 0;
    struct timespec copy_started;
    clock_gettime(CLOCK_MONOTONIC, &copy_started);
    for (;;) {
        if (atomic_load(&stop_requested)) {
            safe_strcpy(error, "Migration worker is shutting down",
                        STORAGE_MIGRATION_ERROR_MAX, 0);
            result = -1;
            break;
        }
        if (migration_cancel_requested(job)) {
            safe_strcpy(error, "Cancelled by operator",
                        STORAGE_MIGRATION_ERROR_MAX, 0);
            result = -3;
            break;
        }
        ssize_t count = read(source, buffer, MIGRATION_COPY_BUFFER);
        if (count == 0) break;
        if (count < 0) {
            if (errno == EINTR) continue;
            set_error(error, "Cannot read source", source_path);
            result = -1;
            break;
        }
        if (write_all(destination, buffer, (size_t)count) != 0) {
            set_error(error, "Cannot write temporary destination",
                      temporary_path);
            result = -1;
            break;
        }
        if (mbedtls_sha256_update(&context, buffer, (size_t)count) != 0) {
            safe_strcpy(error, "SHA-256 update failed",
                        STORAGE_MIGRATION_ERROR_MAX, 0);
            result = -1;
            break;
        }
        copied += (uint64_t)count;
        int throttled = throttle_copy(job, &copy_started, copied);
        if (throttled != 0) {
            safe_strcpy(error, throttled > 0 ? "Cancelled by operator" :
                        "Migration worker is shutting down",
                        STORAGE_MIGRATION_ERROR_MAX, 0);
            result = throttled > 0 ? -3 : -1;
            break;
        }
        if (copied - reported >= MIGRATION_PROGRESS_INTERVAL) {
            if (db_storage_migration_update_progress(
                    job->uuid, "copying", copied,
                    (uint64_t)source_status.st_size) !=
                DB_STORAGE_MIGRATION_OK) {
                safe_strcpy(error, "Could not persist migration progress",
                            STORAGE_MIGRATION_ERROR_MAX, 0);
                result = -1;
                break;
            }
            reported = copied;
        }
    }
    if (result == 0 && fsync(destination) != 0) {
        set_error(error, "Cannot sync temporary destination", temporary_path);
        result = -1;
    }
    free(buffer);
    close(destination);
    close(source);
    char source_checksum[STORAGE_MIGRATION_CHECKSUM_MAX];
    if (result == 0 && finish_digest(&context, source_checksum) != 0) {
        safe_strcpy(error, "Could not finish source SHA-256",
                    STORAGE_MIGRATION_ERROR_MAX, 0);
        result = -1;
    } else if (result != 0) {
        mbedtls_sha256_free(&context);
    }
    if (result != 0) {
        if (result == -3) unlink(temporary_path);
        return result;
    }

    job->bytes_total = copied;
    if (db_storage_migration_update_progress(
            job->uuid, "verifying", copied, copied) !=
        DB_STORAGE_MIGRATION_OK) {
        safe_strcpy(error, "Could not persist verification state",
                    STORAGE_MIGRATION_ERROR_MAX, 0);
        return -1;
    }
    uint64_t destination_size = 0;
    char destination_checksum[STORAGE_MIGRATION_CHECKSUM_MAX];
    if (sha256_file(temporary_path, &destination_size,
                    destination_checksum, error) != 0) {
        return -1;
    }
    if (destination_size != copied ||
        strcmp(source_checksum, destination_checksum) != 0) {
        safe_strcpy(error, "Destination checksum verification failed",
                    STORAGE_MIGRATION_ERROR_MAX, 0);
        unlink(temporary_path);
        return -1;
    }
    if (migration_cancel_requested(job)) {
        safe_strcpy(error, "Cancelled by operator",
                    STORAGE_MIGRATION_ERROR_MAX, 0);
        unlink(temporary_path);
        return -3;
    }
    if (lstat(destination_path, &source_status) == 0) {
        safe_strcpy(error, "Destination appeared while migration was copying",
                    STORAGE_MIGRATION_ERROR_MAX, 0);
        unlink(temporary_path);
        return -2;
    }
    if (errno != ENOENT || rename(temporary_path, destination_path) != 0) {
        set_error(error, "Cannot publish verified destination",
                  destination_path);
        return -1;
    }
    if (fsync_parent(destination_path) != 0) {
        set_error(error, "Cannot sync destination directory for",
                  destination_path);
        return -1;
    }
    safe_strcpy(checksum, source_checksum,
                STORAGE_MIGRATION_CHECKSUM_MAX, 0);
    return 0;
}

static int destination_retained(const storage_migration_job_t *job) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int retained = -1; // Database uncertainty cannot authorize removal.
    if (sqlite3_prepare_v2(db, "SELECT 1 FROM recordings WHERE storage_target_uuid=?1 AND object_key=?2 "
        "UNION ALL SELECT 1 FROM storage_recording_copies WHERE target_uuid=?1 AND object_key=?2 LIMIT 1;",
        -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, job->destination_target_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 2, job->destination_object_key, -1, SQLITE_TRANSIENT);
        int rc = sqlite3_step(stmt);
        retained = rc == SQLITE_ROW ? 1 : (rc == SQLITE_DONE ? 0 : -1);
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return retained;
}

static int finish_cleanup(storage_migration_job_t *job, const char *source_path) {
    if (storage_source_has_lease(job->recording_id)) {
        db_storage_migration_defer_cleanup(job, "Source cleanup waits for active recording readers");
        return -1;
    }
    storage_target_t destination, source;
    char error[STORAGE_MIGRATION_ERROR_MAX] = {0}, path[MAX_PATH_LENGTH], hash[65];
    uint64_t bytes = 0;
    int verified = -1;
    if (destination_retained(job) == 1 &&
        storage_target_probe_and_publish(job->destination_target_uuid, false, &destination) == DB_STORAGE_TARGET_OK) {
        if (!strcmp(destination.target_type, "s3")) {
            storage_transfer_control_t control = {.cancelled = transfer_cancelled, .context = job,
                                                  .bandwidth_bps = job->bandwidth_limit_bps};
            verified = storage_s3_verify(&destination, job->destination_object_key, job->bytes_total,
                                          job->checksum, &control, error);
        } else if (db_storage_target_mount_guard_active(&destination) &&
            db_storage_target_resolve_path(destination.uuid, job->destination_object_key, path) == 0 &&
            sha256_file(path, &bytes, hash, error) == 0 && bytes == job->bytes_total && !strcmp(hash, job->checksum))
            verified = 0;
    }
    if (verified != 0 || db_storage_target_get(job->source_target_uuid, &source) != DB_STORAGE_TARGET_OK) {
        db_storage_migration_defer_cleanup(job, "Source retained: committed destination cannot be verified");
        return -1;
    }
    int removed;
    if (!strcmp(source.target_type, "s3")) {
        removed = storage_s3_probe(&source, false) == 0 ?
            storage_s3_delete(&source, job->source_object_key, error) : -1;
    } else {
        removed = db_storage_target_mount_guard_active(&source) ? unlink(source_path) : -1;
        if (removed != 0 && db_storage_target_mount_guard_active(&source) && errno == ENOENT) removed = 0;
        if (removed == 0) fsync_parent(source_path);
    }
    if (removed != 0) {
        db_storage_migration_defer_cleanup(job, "Verified destination committed; source cleanup failed");
        return -1;
    }
    return db_storage_migration_complete_cleanup(job->uuid) == DB_STORAGE_MIGRATION_OK ? 0 : -1;
}

/* One worker owns transfers and cleanup. Keep failed/cancelled destinations in
 * the journal until every unpublished artifact has been removed. */
static void cleanup_abandoned(void) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return;
    char uuid[37] = {0}, upload[1024] = {0};
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    const char *sql = "SELECT uuid,upload_id FROM storage_migration_jobs WHERE "
        "next_attempt_at<=strftime('%s','now') AND ((artifacts_cleaned=0 AND "
        "(state IN('failed','cancelled') OR (cancel_requested=1 AND state IN('copying','verifying','committing')))) "
        "OR (state='completed' AND upload_id<>'')) ORDER BY updated_at LIMIT 1;";
    if (sqlite3_prepare_v2(db, sql, -1, &stmt, NULL) == SQLITE_OK && sqlite3_step(stmt) == SQLITE_ROW) {
        safe_strcpy(uuid, (const char *)sqlite3_column_text(stmt, 0), sizeof(uuid), 0);
        safe_strcpy(upload, (const char *)sqlite3_column_text(stmt, 1), sizeof(upload), 0);
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    if (!*uuid) return;
    storage_migration_job_t job;
    storage_target_t target;
    int result = -1;
    char error[256] = {0}, path[MAX_PATH_LENGTH], temporary[MAX_PATH_LENGTH];
    if (db_storage_migration_get(uuid, &job) == DB_STORAGE_MIGRATION_OK &&
        db_storage_target_get(job.destination_target_uuid, &target) == DB_STORAGE_TARGET_OK) {
        bool retained = destination_retained(&job);
        if (!strcmp(target.target_type, "s3")) {
            result = storage_s3_probe(&target, false);
            if (!result) result = storage_s3_abort_upload(&target, job.destination_object_key, upload, error);
            if (!result && !retained) result = storage_s3_delete(&target, job.destination_object_key, error);
        } else if (db_storage_target_mount_guard_active(&target) &&
                   !db_storage_target_resolve_path(target.uuid, job.destination_object_key, path)) {
            result = retained || unlink(path) == 0 || errno == ENOENT ? 0 : -1;
            int n = snprintf(temporary, sizeof(temporary), "%s.migration-%s.part", path, uuid);
            if (n > 0 && n < (int)sizeof(temporary) && unlink(temporary) && errno != ENOENT) result = -1;
        }
    }
    pthread_mutex_lock(mutex);
    stmt = NULL;
    sql = result == 0 ?
        "UPDATE storage_migration_jobs SET artifacts_cleaned=1,upload_id='',upload_parts='',"
        "state=CASE WHEN cancel_requested=1 THEN 'cancelled' ELSE state END WHERE uuid=?;" :
        "UPDATE storage_migration_jobs SET next_attempt_at=strftime('%s','now')+60,"
        "last_error='Unpublished transfer artifacts await cleanup' WHERE uuid=?;";
    if (sqlite3_prepare_v2(db, sql, -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, uuid, -1, SQLITE_TRANSIENT);
        sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
}

static int restore_to_filesystem(storage_migration_job_t *job, const storage_target_t *source,
                                 const storage_target_t *destination, const char *path,
                                 char checksum[65], char error[256]) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    checksum[0] = 0;
    if (sqlite3_prepare_v2(db, "SELECT archive_checksum FROM recordings WHERE id=?1 "
        "AND storage_target_uuid=?2 AND object_key=?3 AND deletion_pending=0;", -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, (sqlite3_int64)job->recording_id);
        sqlite3_bind_text(stmt, 2, job->source_target_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 3, job->source_object_key, -1, SQLITE_TRANSIENT);
        if (sqlite3_step(stmt) == SQLITE_ROW)
            safe_strcpy(checksum, (const char *)sqlite3_column_text(stmt, 0), 65, 0);
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    if (strlen(checksum) != 64) { safe_strcpy(error, "Archive source identity changed or has no checksum", 256, 0); return -2; }
    struct stat info;
    if (lstat(path, &info) == 0) {
        uint64_t bytes;
        char hash[65];
        if (sha256_file(path, &bytes, hash, error) || bytes != job->bytes_total || strcmp(hash, checksum)) {
            safe_strcpy(error, "Restore destination contains different data", 256, 0); return -2;
        }
        return 0;
    }
    if (errno != ENOENT) return -1;
    if (destination->available_bytes <= destination->reserve_bytes ||
        job->bytes_total > destination->available_bytes - destination->reserve_bytes) {
        safe_strcpy(error, "Restore would exceed destination free-space reserve", 256, 0); return -1;
    }
    char temporary[MAX_PATH_LENGTH];
    int n = snprintf(temporary, sizeof(temporary), "%s.migration-%s.part", path, job->uuid);
    if (n < 0 || n >= (int)sizeof(temporary) || ensure_path(temporary)) return -1;
    if (unlink(temporary) && errno != ENOENT) return -1;
    storage_transfer_control_t control = {.cancelled = transfer_cancelled, .context = job,
                                          .bandwidth_bps = job->bandwidth_limit_bps};
    int result = storage_s3_download(source, job->source_object_key, temporary, job->bytes_total, checksum, &control, error);
    if (!result && transfer_cancelled(job)) result = -3;
    if (!result) {
        // Publishing a restore must never replace a file that appeared meanwhile.
        if (link(temporary, path)) result = errno == EEXIST ? -2 : -1;
        else if (fsync_parent(path)) result = -1;
    }
    unlink(temporary);
    return result;
}

int storage_migration_process_one(void) {
    const char *read_only = getenv("LIGHTNVR_STORAGE_READ_ONLY");
    if (read_only && !strcmp(read_only, "1")) return 0;
    cleanup_abandoned();
    storage_migration_job_t job;
    int claimed = db_storage_migration_claim_due(&job);
    if (claimed <= 0) return claimed;

    storage_target_t destination_target;
    if (db_storage_target_get(job.destination_target_uuid, &destination_target) != DB_STORAGE_TARGET_OK) {
        db_storage_migration_record_failure(&job, "Destination target no longer exists", false);
        return 1;
    }
    storage_target_t source_target;
    if (db_storage_target_get(job.source_target_uuid, &source_target) != DB_STORAGE_TARGET_OK) {
        db_storage_migration_record_failure(&job, "Source target no longer exists", false);
        return 1;
    }
    bool object_source = !strcmp(source_target.target_type, "s3");
    bool object_destination = strcmp(destination_target.target_type, "s3") == 0;
    char source_path[MAX_PATH_LENGTH] = {0};
    char destination_path[MAX_PATH_LENGTH] = {0};
    if (!object_source && !db_storage_target_mount_guard_active(&source_target)) {
        if (!strcmp(job.state, "cleanup_pending"))
            db_storage_migration_defer_cleanup(&job, "Source mount is unavailable");
        else db_storage_migration_record_failure(&job, "Source mount is unavailable", true);
        return 1;
    }
    if ((!object_source && db_storage_target_resolve_path(job.source_target_uuid,
                                       job.source_object_key,
                                       source_path) != 0) ||
        (!object_destination && db_storage_target_resolve_path(job.destination_target_uuid,
                                       job.destination_object_key,
                                       destination_path) != 0)) {
        db_storage_migration_record_failure(
            &job, "Storage target identity no longer resolves", false);
        return 1;
    }
    if (strcmp(job.state, "cleanup_pending") == 0) {
        finish_cleanup(&job, source_path);
        return 1;
    }

    if (storage_target_probe_and_publish(job.destination_target_uuid, false,
                                         &destination_target) !=
            DB_STORAGE_TARGET_OK ||
        !destination_target.enabled ||
        !db_storage_target_mount_guard_active(&destination_target)) {
        db_storage_migration_record_failure(
            &job, "Destination target is unavailable", true);
        return 1;
    }

    char checksum[STORAGE_MIGRATION_CHECKSUM_MAX];
    char error[STORAGE_MIGRATION_ERROR_MAX] = {0};
    int copied;
    if (object_source && object_destination) {
        int resolved = storage_source_resolve(job.recording_id, source_path, error);
        if (resolved != STORAGE_SOURCE_READY) {
            // The bounded retrieval worker also supplies verified restore/drain sources.
            if (resolved == STORAGE_SOURCE_PREPARING) db_storage_migration_defer_source(job.uuid);
            else db_storage_migration_record_failure(&job, error, true);
            return 1;
        }
    }
    bool retained_destination = destination_retained(&job);
    if (object_destination) {
        storage_transfer_control_t control = {
            .cancelled = transfer_cancelled, .context = &job,
            .bandwidth_bps = job.bandwidth_limit_bps, .job_uuid = job.uuid,
        };
        copied = sha256_file(source_path, &job.bytes_total, checksum, error);
        uint64_t remote_size = 0;
        if (copied == 0) {
            int found = storage_s3_stat(&destination_target, job.destination_object_key, &remote_size, error);
            if (found == STORAGE_S3_OK) {
                copied = remote_size == job.bytes_total ?
                    storage_s3_verify(&destination_target, job.destination_object_key,
                                      job.bytes_total, checksum, &control, error) : STORAGE_S3_CONFLICT;
                // Only reconcile an unpublished object owned by this job. Never
                // remove a retained replica or act on uncertain catalog state.
                if (copied == STORAGE_S3_CONFLICT && !transfer_cancelled(&job) && destination_retained(&job) == 0) {
                    copied = storage_s3_delete(&destination_target, job.destination_object_key, error);
                    if (!copied) found = STORAGE_S3_MISSING;
                }
            } else if (found != STORAGE_S3_MISSING) copied = -1;
            if (!copied && found == STORAGE_S3_MISSING)
                copied = storage_s3_upload(&destination_target, job.destination_object_key,
                                           source_path, checksum, &control, error);
        }
        if (copied == 0) {
            db_storage_migration_update_progress(job.uuid, "verifying", job.bytes_total, job.bytes_total);
            copied = storage_s3_verify(&destination_target, job.destination_object_key,
                                       job.bytes_total, checksum, &control, error);
        }
    } else if (object_source) {
        copied = restore_to_filesystem(&job, &source_target, &destination_target, destination_path, checksum, error);
    } else {
        copied = copy_and_verify(&job, source_path, destination_path, checksum, error);
    }
    if (copied != 0) {
        if (copied == -3) {
            db_storage_migration_mark_cancelled(job.uuid, &job);
            return 1;
        }
        db_storage_migration_record_failure(&job, error[0] ? error :
                                             "Recording copy failed",
                                             copied != -2);
        return 1;
    }
    if (migration_cancel_requested(&job)) {
        if (retained_destination) {
            db_storage_migration_mark_cancelled(job.uuid, &job);
            return 1;
        }
        if (object_destination) {
            if (storage_s3_delete(&destination_target, job.destination_object_key, error) != 0) {
                db_storage_migration_record_failure(&job, "Cancelled upload awaits destination cleanup", true);
                return 1;
            }
        } else unlink(destination_path);
        db_storage_migration_mark_cancelled(job.uuid, &job);
        return 1;
    }
    if (db_storage_migration_update_progress(
            job.uuid, "committing", job.bytes_total, job.bytes_total) !=
        DB_STORAGE_MIGRATION_OK) {
        db_storage_migration_record_failure(
            &job, "Could not persist commit state", true);
        return 1;
    }
    if (storage_target_probe_and_publish(job.destination_target_uuid, false,
                                         &destination_target) !=
            DB_STORAGE_TARGET_OK ||
        !destination_target.enabled ||
        !db_storage_target_mount_guard_active(&destination_target)) {
        db_storage_migration_record_failure(
            &job, "Destination target became unavailable before commit", true);
        return 1;
    }
    if (!object_source && !db_storage_target_mount_guard_active(&source_target)) {
        db_storage_migration_record_failure(&job, "Source mount became unavailable before commit", true);
        return 1;
    }
    db_storage_migration_result_t committed = strcmp(job.operation, "copy") == 0
        ? db_storage_migration_commit_copy(&job, checksum)
        : db_storage_migration_commit_location(&job, destination_path, checksum);
    if (committed != DB_STORAGE_MIGRATION_OK) {
        if (!retained_destination && object_destination) {
            if (storage_s3_delete(&destination_target, job.destination_object_key, error) != 0) {
                db_storage_migration_record_failure(&job, "Uncommitted archive copy awaits cleanup", true);
                return 1;
            }
        } else if (!retained_destination) unlink(destination_path);
        if (migration_cancel_requested(&job)) {
            db_storage_migration_mark_cancelled(job.uuid, &job);
        } else if (committed == DB_STORAGE_MIGRATION_SOURCE_CHANGED) {
            db_storage_migration_record_failure(
                &job, "Recording location changed before migration commit",
                false);
        } else {
            db_storage_migration_record_failure(
                &job, "Could not atomically commit recording location", true);
        }
        return 1;
    }
    if (strcmp(job.operation, "move") == 0) {
        safe_strcpy(job.checksum, checksum, sizeof(job.checksum), 0);
        finish_cleanup(&job, source_path);
    } else {
        log_info("Storage replication %s completed for recording %llu",
                 job.uuid, (unsigned long long)job.recording_id);
    }
    return 1;
}

static void *migration_worker_main(void *unused) {
    (void)unused;
    log_set_thread_context("StorageMigration", NULL);
    time_t last_lifecycle_check = 0;
    for (;;) {
        pthread_mutex_lock(&worker.mutex);
        bool running = worker.running;
        pthread_mutex_unlock(&worker.mutex);
        if (!running) break;
        int result = storage_migration_process_one();
        storage_deletion_process_one();
        time_t now = time(NULL);
        if (now - last_lifecycle_check >= 60) {
            last_lifecycle_check = now;
            // Object stores have no statvfs heartbeat. Probe periodically even
            // when their last outage prevented any new job from being queued.
            int total = db_storage_target_count();
            storage_target_t *targets = total > 0 && total <= STORAGE_TARGET_MAX_COUNT ? calloc((size_t)total, sizeof(*targets)) : NULL;
            if (targets) {
                int count = db_storage_target_list(targets, total);
                for (int i = 0; i < count && !atomic_load(&stop_requested); i++)
                    if (targets[i].enabled && !strcmp(targets[i].target_type, "s3"))
                        storage_target_probe_and_publish(targets[i].uuid, false, &targets[i]);
                free(targets);
            }
            db_storage_lifecycle_expire(32);
            int scheduled = db_storage_lifecycle_schedule(16);
            int violations = db_storage_lifecycle_reconcile();
            if (scheduled > 0) {
                log_info("Scheduled %d policy-driven storage lifecycle job(s)",
                         scheduled);
                continue;
            }
            if (scheduled < 0 || violations < 0) {
                log_warn("Could not refresh storage lifecycle policy state");
            }
        }
        if (result > 0) continue;
        pthread_mutex_lock(&worker.mutex);
        if (worker.running) {
            struct timespec deadline;
            clock_gettime(CLOCK_REALTIME, &deadline);
            deadline.tv_sec += result < 0 ? 5 : 1;
            pthread_cond_timedwait(&worker.condition, &worker.mutex,
                                   &deadline);
        }
        pthread_mutex_unlock(&worker.mutex);
    }
    pthread_mutex_lock(&worker.mutex);
    worker.exited = true;
    pthread_mutex_unlock(&worker.mutex);
    return NULL;
}

int storage_migration_worker_start(void) {
    pthread_mutex_lock(&worker.mutex);
    if (worker.running) {
        pthread_mutex_unlock(&worker.mutex);
        return 0;
    }
    atomic_store(&stop_requested, false);
    worker.running = true;
    worker.exited = false;
    if (pthread_create(&worker.thread, NULL, migration_worker_main, NULL) != 0) {
        worker.running = false;
        worker.exited = true;
        pthread_mutex_unlock(&worker.mutex);
        return -1;
    }
    pthread_mutex_unlock(&worker.mutex);
    log_info("Durable storage migration worker started");
    return 0;
}

void storage_migration_worker_wake(void) {
    pthread_mutex_lock(&worker.mutex);
    pthread_cond_broadcast(&worker.condition);
    pthread_mutex_unlock(&worker.mutex);
}

void storage_migration_worker_shutdown(void) {
    pthread_mutex_lock(&worker.mutex);
    if (!worker.running) {
        pthread_mutex_unlock(&worker.mutex);
        return;
    }
    atomic_store(&stop_requested, true);
    worker.running = false;
    pthread_cond_broadcast(&worker.condition);
    pthread_mutex_unlock(&worker.mutex);
    pthread_join(worker.thread, NULL);
    log_info("Durable storage migration worker stopped");
}
