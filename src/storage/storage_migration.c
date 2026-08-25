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
#include "database/db_storage_migrations.h"
#include "database/db_storage_targets.h"
#include "storage/storage_target_health.h"
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
    for (;;) {
        if (atomic_load(&stop_requested)) {
            safe_strcpy(error, "Migration worker is shutting down",
                        STORAGE_MIGRATION_ERROR_MAX, 0);
            result = -1;
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
    if (result != 0) return -1;

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

static int finish_cleanup(storage_migration_job_t *job,
                          const char *source_path) {
    if (unlink(source_path) != 0 && errno != ENOENT) {
        char error[STORAGE_MIGRATION_ERROR_MAX];
        set_error(error, "Verified destination committed; source cleanup failed",
                  source_path);
        db_storage_migration_defer_cleanup(job, error);
        log_warn("Storage migration %s deferred source cleanup: %s",
                 job->uuid, error);
        return -1;
    }
    if (fsync_parent(source_path) != 0 && errno != ENOENT) {
        log_warn("Storage migration %s could not sync source directory",
                 job->uuid);
    }
    if (db_storage_migration_complete_cleanup(job->uuid) !=
        DB_STORAGE_MIGRATION_OK) {
        log_error("Storage migration %s could not persist completion",
                  job->uuid);
        return -1;
    }
    log_info("Storage migration %s completed for recording %llu",
             job->uuid, (unsigned long long)job->recording_id);
    return 0;
}

int storage_migration_process_one(void) {
    storage_migration_job_t job;
    int claimed = db_storage_migration_claim_due(&job);
    if (claimed <= 0) return claimed;

    char source_path[MAX_PATH_LENGTH];
    char destination_path[MAX_PATH_LENGTH];
    if (db_storage_target_resolve_path(job.source_target_uuid,
                                       job.source_object_key,
                                       source_path) != 0 ||
        db_storage_target_resolve_path(job.destination_target_uuid,
                                       job.destination_object_key,
                                       destination_path) != 0) {
        db_storage_migration_record_failure(
            &job, "Storage target identity no longer resolves", false);
        return 1;
    }
    if (strcmp(job.state, "cleanup_pending") == 0) {
        finish_cleanup(&job, source_path);
        return 1;
    }

    storage_target_t destination_target;
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
    int copied = copy_and_verify(&job, source_path, destination_path,
                                 checksum, error);
    if (copied != 0) {
        db_storage_migration_record_failure(&job, error[0] ? error :
                                             "Recording copy failed",
                                             copied != -2);
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
    db_storage_migration_result_t committed =
        db_storage_migration_commit_location(&job, destination_path, checksum);
    if (committed != DB_STORAGE_MIGRATION_OK) {
        if (committed == DB_STORAGE_MIGRATION_SOURCE_CHANGED) {
            unlink(destination_path);
            db_storage_migration_record_failure(
                &job, "Recording location changed before migration commit",
                false);
        } else {
            db_storage_migration_record_failure(
                &job, "Could not atomically commit recording location", true);
        }
        return 1;
    }
    finish_cleanup(&job, source_path);
    return 1;
}

static void *migration_worker_main(void *unused) {
    (void)unused;
    log_set_thread_context("StorageMigration", NULL);
    for (;;) {
        pthread_mutex_lock(&worker.mutex);
        bool running = worker.running;
        pthread_mutex_unlock(&worker.mutex);
        if (!running) break;
        int result = storage_migration_process_one();
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
