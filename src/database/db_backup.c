#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sqlite3.h>
#include <pthread.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <errno.h>
#include <stdbool.h>
#include <fcntl.h>
#include <limits.h>
#include <libgen.h>

#include "database/db_core.h"
#include "database/db_backup.h"
#include "database/db_schema_utils.h"
#include "core/logger.h"

// Flag to indicate if a backup is in progress
static bool backup_in_progress = false;
static pthread_mutex_t backup_mutex = PTHREAD_MUTEX_INITIALIZER;

/* Copy in bounded batches so a large database backup cannot populate the
 * container's entire page cache in one sqlite3_backup_step() call.  At the
 * usual 4 KiB SQLite page size this is 16 MiB per batch. */
#define BACKUP_STEP_PAGES 4096
#define BACKUP_BUSY_RETRIES 50
#define BACKUP_BUSY_RETRY_US 100000
#define BACKUP_VERIFY_PROGRESS_OPS 100000

static void release_file_cache(int fd, const char *path) {
#ifdef POSIX_FADV_DONTNEED
    int advise_rc = posix_fadvise(fd, 0, 0, POSIX_FADV_DONTNEED);
    if (advise_rc != 0 && advise_rc != EINVAL && advise_rc != ENOSYS) {
        log_warn("Failed to release filesystem cache for %s: %s",
                 path, strerror(advise_rc));
    }
#else
    (void)fd;
    (void)path;
#endif
}

static void release_path_cache(const char *path) {
    int fd = open(path, O_RDONLY | O_CLOEXEC);
    if (fd < 0) {
        if (errno != ENOENT) {
            log_warn("Failed to open path for cache release: %s (%s)",
                     path, strerror(errno));
        }
        return;
    }

    release_file_cache(fd, path);
    close(fd);
}

typedef struct {
    int fd;
    const char *path;
} cache_release_progress_t;

static int release_cache_during_verification(void *opaque) {
    cache_release_progress_t *progress = (cache_release_progress_t *)opaque;
    if (progress && progress->fd >= 0) {
        release_file_cache(progress->fd, progress->path);
    }
    return 0;
}

static int sync_path_to_disk(const char *path) {
    int fd = open(path, O_RDONLY);
    if (fd < 0) {
        log_error("Failed to open path for sync: %s (%s)", path, strerror(errno));
        return -1;
    }

    if (fsync(fd) != 0) {
        log_error("Failed to fsync path: %s (%s)", path, strerror(errno));
        close(fd);
        return -1;
    }

    /* A backup is recovery data, not a hot working set.  Keeping every page
     * resident after a full copy and verification can OOM a memory-limited
     * NVR whose database is measured in gigabytes. */
    release_file_cache(fd, path);
    close(fd);
    return 0;
}

static int sync_parent_directory(const char *path) {
    char path_copy[PATH_MAX];
    if (snprintf(path_copy, sizeof(path_copy), "%s", path) >= (int)sizeof(path_copy)) {
        log_error("Path too long while syncing parent directory: %s", path);
        return -1;
    }

    char *dir_name = dirname(path_copy);
    int dir_fd = open(dir_name, O_RDONLY | O_DIRECTORY);
    if (dir_fd < 0) {
        log_error("Failed to open parent directory for sync: %s (%s)", dir_name, strerror(errno));
        return -1;
    }

    if (fsync(dir_fd) != 0) {
        log_error("Failed to fsync parent directory: %s (%s)", dir_name, strerror(errno));
        close(dir_fd);
        return -1;
    }

    close(dir_fd);
    return 0;
}

static int run_integrity_check(sqlite3 *db_handle, const char *path_label) {
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db_handle, "PRAGMA integrity_check;", -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare integrity check for %s: %s",
                  path_label, sqlite3_errmsg(db_handle));
        return -1;
    }

    rc = sqlite3_step(stmt);
    if (rc != SQLITE_ROW) {
        log_error("Failed to execute integrity check for %s: %s",
                  path_label, sqlite3_errmsg(db_handle));
        sqlite3_finalize(stmt);
        return -1;
    }

    const char *result = (const char *)sqlite3_column_text(stmt, 0);
    if (!result || strcmp(result, "ok") != 0) {
        log_error("Integrity check failed for %s: %s", path_label, result ? result : "unknown error");
        sqlite3_finalize(stmt);
        return -1;
    }

    sqlite3_finalize(stmt);
    return 0;
}

// Backup the database to a specified path
int backup_database(const char *source_path, const char *dest_path) {
    int rc = -1;
    sqlite3 *source_db = NULL;
    sqlite3 *dest_db = NULL;
    sqlite3_backup *backup = NULL;
    char temp_path[PATH_MAX];
    int source_cache_fd = -1;
    int dest_cache_fd = -1;
    
    pthread_mutex_lock(&backup_mutex);
    if (backup_in_progress) {
        pthread_mutex_unlock(&backup_mutex);
        log_warn("Backup already in progress, skipping");
        return -1;
    }

    backup_in_progress = true;
    pthread_mutex_unlock(&backup_mutex);

    if (snprintf(temp_path, sizeof(temp_path), "%s.tmp", dest_path) >= (int)sizeof(temp_path)) {
        log_error("Destination path is too long for temporary backup file: %s", dest_path);
        goto cleanup;
    }

    unlink(temp_path);

    log_info("Starting database backup from %s to %s", source_path, dest_path);
    
    // Open the source database
    rc = sqlite3_open_v2(source_path, &source_db, SQLITE_OPEN_READONLY, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to open source database for backup: %s",
                  source_db ? sqlite3_errmsg(source_db) : sqlite3_errstr(rc));
        goto cleanup;
    }

    /* Pin one WAL snapshot across the incremental steps.  Without an explicit
     * read transaction, every live detection/recording write can restart an
     * online backup from page one between batches. */
    rc = sqlite3_exec(source_db, "BEGIN;", NULL, NULL, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to start backup snapshot: %s", sqlite3_errmsg(source_db));
        goto cleanup;
    }
    
    // Open the destination database
    rc = sqlite3_open_v2(temp_path, &dest_db, SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to open destination database for backup: %s",
                  dest_db ? sqlite3_errmsg(dest_db) : sqlite3_errstr(rc));
        goto cleanup;
    }

    source_cache_fd = open(source_path, O_RDONLY | O_CLOEXEC);
    if (source_cache_fd < 0) {
        log_warn("Could not open source database for cache control: %s",
                 strerror(errno));
    }
    dest_cache_fd = open(temp_path, O_RDWR | O_CLOEXEC);
    if (dest_cache_fd < 0) {
        log_warn("Could not open backup database for cache control: %s",
                 strerror(errno));
    }
    
    // Initialize the backup
    backup = sqlite3_backup_init(dest_db, "main", source_db, "main");
    if (!backup) {
        log_error("Failed to initialize backup: %s", sqlite3_errmsg(dest_db));
        goto cleanup;
    }
    
    /* Copy incrementally and discard each batch from the kernel page cache.
     * The destination is fdatasync()'d first so POSIX_FADV_DONTNEED can evict
     * clean pages instead of leaving the full backup charged to the cgroup.
     * Source pages are clean and can be discarded immediately. */
    int busy_retries = 0;
    while (1) {
        rc = sqlite3_backup_step(backup, BACKUP_STEP_PAGES);
        if (rc == SQLITE_BUSY || rc == SQLITE_LOCKED) {
            if (++busy_retries > BACKUP_BUSY_RETRIES) {
                log_error("Database backup remained busy after %d retries",
                          BACKUP_BUSY_RETRIES);
                goto cleanup;
            }
            usleep(BACKUP_BUSY_RETRY_US);
            continue;
        }
        if (rc != SQLITE_OK && rc != SQLITE_DONE) {
            log_error("Failed to perform backup: %s", sqlite3_errmsg(dest_db));
            goto cleanup;
        }

        busy_retries = 0;
        if (dest_cache_fd >= 0) {
            if (fdatasync(dest_cache_fd) != 0) {
                log_error("Failed to flush backup batch for %s: %s",
                          temp_path, strerror(errno));
                rc = SQLITE_IOERR_FSYNC;
                goto cleanup;
            }
            release_file_cache(dest_cache_fd, temp_path);
        }
        if (source_cache_fd >= 0) {
            release_file_cache(source_cache_fd, source_path);
        }

        if (rc == SQLITE_DONE) {
            break;
        }
    }
    
    // Finish the backup
    rc = sqlite3_backup_finish(backup);
    backup = NULL;
    if (rc != SQLITE_OK) {
        log_error("Failed to finish backup: %s", sqlite3_errmsg(dest_db));
        goto cleanup;
    }

    /* Retain the full integrity guarantee, but release clean pages as SQLite
     * scans the backup.  Without the progress callback this second full-file
     * pass recreates the multi-gigabyte cache footprint bounded above. */
    cache_release_progress_t verification_progress = {
        .fd = dest_cache_fd,
        .path = temp_path,
    };
    if (dest_cache_fd >= 0) {
        sqlite3_progress_handler(dest_db, BACKUP_VERIFY_PROGRESS_OPS,
                                 release_cache_during_verification,
                                 &verification_progress);
    }
    int verification_rc = run_integrity_check(dest_db, temp_path);
    sqlite3_progress_handler(dest_db, 0, NULL, NULL);
    if (verification_rc != 0) {
        rc = SQLITE_CORRUPT;
        goto cleanup;
    }
    
    // Close the databases
    sqlite3_close(source_db);
    source_db = NULL;
    sqlite3_close(dest_db);
    dest_db = NULL;

    if (source_cache_fd >= 0) {
        release_file_cache(source_cache_fd, source_path);
        close(source_cache_fd);
        source_cache_fd = -1;
    }
    if (dest_cache_fd >= 0) {
        release_file_cache(dest_cache_fd, temp_path);
        close(dest_cache_fd);
        dest_cache_fd = -1;
    }

    if (rename(temp_path, dest_path) != 0) {
        log_error("Failed to publish backup %s -> %s: %s", temp_path, dest_path, strerror(errno));
        rc = SQLITE_IOERR;
        goto cleanup;
    }

    if (sync_path_to_disk(dest_path) != 0 || sync_parent_directory(dest_path) != 0) {
        rc = SQLITE_IOERR_FSYNC;
        goto cleanup;
    }
    
    log_info("Database backup completed successfully");
    rc = 0;

cleanup:
    if (backup) {
        sqlite3_backup_finish(backup);
    }
    if (source_db) {
        sqlite3_close(source_db);
    }
    if (dest_db) {
        sqlite3_close(dest_db);
    }
    if (source_cache_fd >= 0) {
        close(source_cache_fd);
    }
    if (dest_cache_fd >= 0) {
        close(dest_cache_fd);
    }
    if (rc != 0) {
        unlink(temp_path);
    }

    if (rc == 0) {
        /* The source was scanned sequentially solely for this backup.  Drop
         * those pages once more in case SQLite read ahead after the last batch. */
        release_path_cache(source_path);
        release_path_cache(dest_path);
    }

    pthread_mutex_lock(&backup_mutex);
    backup_in_progress = false;
    pthread_mutex_unlock(&backup_mutex);

    return rc == 0 ? 0 : -1;
}

// Restore database from backup
int restore_database_from_backup(const char *backup_path, const char *db_path) {
    int rc;
    sqlite3 *db = get_db_handle();
    
    log_info("Restoring database from backup: %s to %s", backup_path, db_path);

    // Close the current database if it's open
    if (db) {
        log_info("Closing current database before restore");
        sqlite3_close_v2(db);
        // Note: We don't set the global db to NULL here, as that's handled by the core module
    }

    // Copy the backup file to the database file.
    // Attempt fopen directly instead of stat-then-fopen to avoid TOCTOU race condition.
    FILE *src = fopen(backup_path, "rb");
    if (!src) {
        log_error("Failed to open backup file for reading: %s (error: %s)", backup_path, strerror(errno));
        return -1;
    }
    
    // Open database file with restricted permissions (0600: owner rw only)
    int dst_fd = open(db_path, O_WRONLY | O_CREAT | O_TRUNC, 0600);
    FILE *dst = (dst_fd >= 0) ? fdopen(dst_fd, "wb") : NULL;
    if (!dst) {
        if (dst_fd >= 0) close(dst_fd);
        log_error("Failed to open database file for writing: %s", strerror(errno));
        fclose(src);
        return -1;
    }
    
    char buffer[8192];
    size_t bytes;
    
    while ((bytes = fread(buffer, 1, sizeof(buffer), src)) > 0) {
        if (fwrite(buffer, 1, bytes, dst) != bytes) {
            log_error("Failed to write to database file: %s", strerror(errno));
            fclose(src);
            fclose(dst);
            return -1;
        }
    }

    if (ferror(src)) {
        log_error("Error reading backup file: %s", strerror(errno));
        fclose(src);
        fclose(dst);
        return -1;
    }

    fclose(src);
    fclose(dst);
    
    // Verify the restored database
    sqlite3 *test_db;
    rc = sqlite3_open_v2(db_path, &test_db, SQLITE_OPEN_READONLY, NULL);
    if (rc != SQLITE_OK) {
        log_error("Restored database appears to be corrupt: %s", sqlite3_errmsg(test_db));
        sqlite3_close(test_db);
        return -1;
    }
    
    // Run integrity check on the restored database
    sqlite3_stmt *stmt = NULL;
    const char *integrity_sql = "PRAGMA integrity_check;";
    
    rc = sqlite3_prepare_v2(test_db, integrity_sql, -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_finalize(stmt);
        stmt = NULL;
        if (run_integrity_check(test_db, db_path) != 0) {
            sqlite3_close(test_db);
            return -1;
        }
    }
    
    sqlite3_close(test_db);
    
    log_info("Database restored successfully from backup");
    return 0;
}

// Check and repair database
int check_and_repair_database(void) {
    int rc;
    sqlite3 *db = get_db_handle();
    
    log_info("Checking and repairing database");
    
    // If we can't restore from backup, try to repair the database
    log_info("Attempting to repair database with PRAGMA integrity_check");
    
    // Try to open the database in read-write mode if it's not already open
    if (!db) {
        // We need to get the database path from the core module
        // For now, we'll assume the database is already open
        log_error("Database not initialized, cannot repair");
        return -1;
    }
    
    // Run integrity check using a prepared statement for better error handling
    sqlite3_stmt *stmt;
    rc = sqlite3_prepare_v2(db, "PRAGMA integrity_check;", -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare integrity check statement");
        return -1;
    }
    
    bool integrity_ok = false;
    if (sqlite3_step(stmt) == SQLITE_ROW) {
        const char *result = (const char *)sqlite3_column_text(stmt, 0);
        if (result && strcmp(result, "ok") == 0) {
            integrity_ok = true;
        } else {
            log_error("Integrity check failed: %s", result ? result : "unknown error");
        }
    } else {
        log_error("Failed to execute integrity check");
    }
    
    sqlite3_finalize(stmt);
    
    if (!integrity_ok) {
        // Try to recover by recreating the database
        log_warn("Attempting to recover by recreating the database");
        
        // This would require knowing the database path, which we don't have here
        // In a real implementation, we would need to get this from the core module
        log_error("Cannot recreate database without knowing the path");
        return -1;
    }
    
    log_info("Database repair completed successfully");
    return 0;
}
