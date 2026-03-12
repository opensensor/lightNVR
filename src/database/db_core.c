#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sys/time.h>
#include <dirent.h>
#include <sqlite3.h>
#include <pthread.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <errno.h>
#include <libgen.h>
#include <stdbool.h>
#include <fcntl.h>
#include <limits.h>

#include "database/db_core.h"
#include "database/db_schema.h"
#include "database/db_migrations.h"
#include "database/db_backup.h"
#include "core/config.h"
#include "core/logger.h"

// Database handle
static sqlite3 *db = NULL;

// Mutex for thread safety
static pthread_mutex_t db_mutex;

// Database path for backup/recovery operations
static char db_file_path[1024] = {0};

// Backup file path
static char db_backup_path[1024] = {0};

// Last backup time
static time_t last_backup_time = 0;

// Flag to indicate if WAL mode is enabled
static bool wal_mode_enabled = false;

// No longer tracking prepared statements - each function is responsible for finalizing its own statements

static int create_directory(const char *path);

static int sync_path_if_exists(const char *path) {
    int fd = open(path, O_RDONLY);
    if (fd < 0) {
        if (errno == ENOENT) {
            return 0;
        }
        log_error("Failed to open path for sync: %s (%s)", path, strerror(errno));
        return -1;
    }

    if (fsync(fd) != 0) {
        log_error("Failed to fsync path: %s (%s)", path, strerror(errno));
        close(fd);
        return -1;
    }

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

static int flush_database_to_disk(void) {
    int sync_failed = 0;

    if (!db || db_file_path[0] == '\0') {
        log_error("Database not initialized, cannot flush to disk");
        return -1;
    }

    pthread_mutex_lock(&db_mutex);

#if SQLITE_VERSION_NUMBER >= 3007017
    int cacheflush_rc = sqlite3_db_cacheflush(db);
    if (cacheflush_rc != SQLITE_OK) {
        log_warn("sqlite3_db_cacheflush failed: %s", sqlite3_errmsg(db));
    }
#endif

    if (wal_mode_enabled) {
        int checkpoint_rc = sqlite3_exec(db, "PRAGMA wal_checkpoint(FULL);", NULL, NULL, NULL);
        if (checkpoint_rc != SQLITE_OK) {
            log_error("Failed to checkpoint WAL before backup: %s", sqlite3_errmsg(db));
            pthread_mutex_unlock(&db_mutex);
            return -1;
        }
    }

    pthread_mutex_unlock(&db_mutex);

    sync_failed |= sync_path_if_exists(db_file_path);

    char wal_path[PATH_MAX];
    if (snprintf(wal_path, sizeof(wal_path), "%s-wal", db_file_path) < (int)sizeof(wal_path)) {
        sync_failed |= sync_path_if_exists(wal_path);
    }

    char shm_path[PATH_MAX];
    if (snprintf(shm_path, sizeof(shm_path), "%s-shm", db_file_path) < (int)sizeof(shm_path)) {
        sync_failed |= sync_path_if_exists(shm_path);
    }

    sync_failed |= sync_parent_directory(db_file_path);

    return sync_failed ? -1 : 0;
}

static int get_backup_directory(char *backup_dir, size_t backup_dir_size) {
    if (snprintf(backup_dir, backup_dir_size, "%s.backups", db_file_path) >= (int)backup_dir_size) {
        log_error("Backup directory path is too long for database %s", db_file_path);
        return -1;
    }

    if (create_directory(backup_dir) != 0) {
        log_error("Failed to create backup directory: %s", backup_dir);
        return -1;
    }

    return 0;
}

static int build_timestamped_backup_path(const char *backup_dir, char *backup_path, size_t backup_path_size) {
    time_t now = time(NULL);
    struct tm utc_tm;
    char timestamp[32];

    if (!gmtime_r(&now, &utc_tm)) {
        log_error("Failed to format backup timestamp");
        return -1;
    }

    if (strftime(timestamp, sizeof(timestamp), "%Y%m%dT%H%M%SZ", &utc_tm) == 0) {
        log_error("Failed to build backup timestamp string");
        return -1;
    }

    for (int suffix = 0; suffix < 100; suffix++) {
        int written = (suffix == 0)
            ? snprintf(backup_path, backup_path_size, "%s/%s.sqlite3", backup_dir, timestamp)
            : snprintf(backup_path, backup_path_size, "%s/%s-%d.sqlite3", backup_dir, timestamp, suffix);
        if (written < 0 || written >= (int)backup_path_size) {
            log_error("Backup file path is too long for directory %s", backup_dir);
            return -1;
        }
        if (access(backup_path, F_OK) != 0) {
            return 0;
        }
    }

    log_error("Failed to generate a unique backup filename in %s", backup_dir);
    return -1;
}

static int copy_file_contents(const char *source_path, const char *dest_path) {
    int src_fd = -1;
    int dst_fd = -1;
    char buffer[8192];

    src_fd = open(source_path, O_RDONLY);
    if (src_fd < 0) {
        log_error("Failed to open source file for copy: %s (%s)", source_path, strerror(errno));
        return -1;
    }

    dst_fd = open(dest_path, O_WRONLY | O_CREAT | O_TRUNC, 0600);
    if (dst_fd < 0) {
        log_error("Failed to open destination file for copy: %s (%s)", dest_path, strerror(errno));
        close(src_fd);
        return -1;
    }

    while (1) {
        ssize_t bytes_read = read(src_fd, buffer, sizeof(buffer));
        if (bytes_read == 0) {
            break;
        }
        if (bytes_read < 0) {
            log_error("Failed to read source file during copy: %s (%s)", source_path, strerror(errno));
            close(src_fd);
            close(dst_fd);
            return -1;
        }

        ssize_t total_written = 0;
        while (total_written < bytes_read) {
            ssize_t bytes_written = write(dst_fd, buffer + total_written, (size_t)(bytes_read - total_written));
            if (bytes_written < 0) {
                log_error("Failed to write destination file during copy: %s (%s)", dest_path, strerror(errno));
                close(src_fd);
                close(dst_fd);
                return -1;
            }
            total_written += bytes_written;
        }
    }

    close(src_fd);
    close(dst_fd);
    return 0;
}

static int refresh_latest_backup(const char *timestamped_backup_path) {
    char temp_latest_path[PATH_MAX];

    if (snprintf(temp_latest_path, sizeof(temp_latest_path), "%s.tmp", db_backup_path) >= (int)sizeof(temp_latest_path)) {
        log_error("Latest backup temp path is too long: %s", db_backup_path);
        return -1;
    }

    unlink(temp_latest_path);

    if (link(timestamped_backup_path, temp_latest_path) != 0) {
        log_warn("Failed to hard-link latest backup, falling back to copy: %s", strerror(errno));
        if (copy_file_contents(timestamped_backup_path, temp_latest_path) != 0) {
            unlink(temp_latest_path);
            return -1;
        }
    }

    if (rename(temp_latest_path, db_backup_path) != 0) {
        log_error("Failed to publish latest backup %s -> %s: %s",
                  temp_latest_path, db_backup_path, strerror(errno));
        unlink(temp_latest_path);
        return -1;
    }

    if (sync_path_if_exists(db_backup_path) != 0 || sync_parent_directory(db_backup_path) != 0) {
        return -1;
    }

    return 0;
}

static int compare_backup_paths(const void *lhs, const void *rhs) {
    const char *const *left = (const char *const *)lhs;
    const char *const *right = (const char *const *)rhs;
    return strcmp(*left, *right);
}

static int prune_timestamped_backups(const char *backup_dir, int retention_count) {
    DIR *dir = opendir(backup_dir);
    if (!dir) {
        log_error("Failed to open backup directory for pruning: %s (%s)", backup_dir, strerror(errno));
        return -1;
    }

    char **backup_paths = NULL;
    size_t backup_count = 0;
    size_t backup_capacity = 0;
    int result = 0;
    struct dirent *entry;

    while ((entry = readdir(dir)) != NULL) {
        if (entry->d_name[0] == '.') {
            continue;
        }

        char full_path[PATH_MAX];
        if (snprintf(full_path, sizeof(full_path), "%s/%s", backup_dir, entry->d_name) >= (int)sizeof(full_path)) {
            log_warn("Skipping overlong backup path in %s", backup_dir);
            continue;
        }

        struct stat st;
        if (stat(full_path, &st) != 0 || !S_ISREG(st.st_mode)) {
            continue;
        }

        if (backup_count == backup_capacity) {
            size_t new_capacity = backup_capacity == 0 ? 8 : backup_capacity * 2;
            char **new_paths = (char **)realloc((void *)backup_paths, new_capacity * sizeof(*backup_paths));
            if (!new_paths) {
                log_error("Out of memory while pruning backups");
                result = -1;
                break;
            }
            backup_paths = new_paths;
            backup_capacity = new_capacity;
        }

        backup_paths[backup_count] = strdup(full_path);
        if (!backup_paths[backup_count]) {
            log_error("Out of memory while recording backup path");
            result = -1;
            break;
        }
        backup_count++;
    }

    closedir(dir);

    if (result == 0 && backup_paths != NULL && backup_count > (size_t)retention_count) {
        qsort((void *)backup_paths, backup_count, sizeof(*backup_paths), compare_backup_paths);
        size_t remove_count = backup_count - (size_t)retention_count;
        for (size_t i = 0; i < remove_count; i++) {
            if (unlink(backup_paths[i]) != 0) {
                log_warn("Failed to prune old backup %s: %s", backup_paths[i], strerror(errno));
                result = -1;
            } else {
                log_info("Pruned old database backup: %s", backup_paths[i]);
            }
        }
    }

    for (size_t i = 0; i < backup_count; i++) {
        free(backup_paths[i]);
    }
    free((void *)backup_paths);

    return result;
}

static int run_post_backup_script(const char *backup_path, const char *backup_dir) {
    if (g_config.db_post_backup_script[0] == '\0') {
        return 0;
    }

    log_info("Running post-backup script: %s", g_config.db_post_backup_script);

    pid_t pid = fork();
    if (pid < 0) {
        log_error("Failed to fork post-backup script process: %s", strerror(errno));
        return -1;
    }

    if (pid == 0) {
        const char *argv[] = {
            g_config.db_post_backup_script,
            backup_path,
            db_file_path,
            backup_dir,
            NULL
        };
        execv(g_config.db_post_backup_script, (char *const *)argv);
        fprintf(stderr, "Failed to execute post-backup script %s: %s\n",
                g_config.db_post_backup_script, strerror(errno));
        _exit(127);
    }

    int status = 0;
    while (waitpid(pid, &status, 0) < 0) {
        if (errno != EINTR) {
            log_error("waitpid failed for post-backup script: %s", strerror(errno));
            return -1;
        }
    }

    if (WIFEXITED(status) && WEXITSTATUS(status) == 0) {
        log_info("Post-backup script completed successfully");
        return 0;
    }

    if (WIFEXITED(status)) {
        log_error("Post-backup script failed with exit code %d", WEXITSTATUS(status));
    } else if (WIFSIGNALED(status)) {
        log_error("Post-backup script terminated by signal %d", WTERMSIG(status));
    } else {
        log_error("Post-backup script did not complete successfully");
    }

    return -1;
}

static int perform_database_backup_cycle(const char *reason, bool run_post_backup_hook) {
    char backup_dir[PATH_MAX];
    char timestamped_backup_path[PATH_MAX];

    if (!db || db_file_path[0] == '\0') {
        log_error("Database not initialized, cannot create %s backup", reason);
        return -1;
    }

    log_info("Starting %s database backup cycle", reason);

    if (flush_database_to_disk() != 0) {
        log_error("Failed to flush database to disk before %s backup", reason);
        return -1;
    }

    if (get_backup_directory(backup_dir, sizeof(backup_dir)) != 0) {
        return -1;
    }

    if (build_timestamped_backup_path(backup_dir, timestamped_backup_path, sizeof(timestamped_backup_path)) != 0) {
        return -1;
    }

    if (backup_database(db_file_path, timestamped_backup_path) != 0) {
        log_error("Failed to create %s database backup", reason);
        return -1;
    }

    if (refresh_latest_backup(timestamped_backup_path) != 0) {
        log_error("Failed to refresh latest backup link for %s backup", reason);
        return -1;
    }

    if (run_post_backup_hook && g_config.db_post_backup_script[0] != '\0' &&
        run_post_backup_script(timestamped_backup_path, backup_dir) != 0) {
        log_warn("Post-backup script failed after %s backup", reason);
    }

    if (prune_timestamped_backups(backup_dir, g_config.db_backup_retention_count) != 0) {
        log_warn("Failed to fully enforce backup retention after %s backup", reason);
    }

    log_info("Completed %s database backup cycle: %s", reason, timestamped_backup_path);
    return 0;
}

// Create directory if it doesn't exist
static int create_directory(const char *path) {
    struct stat st;

    // Check if directory already exists
    if (stat(path, &st) == 0) {
        if (S_ISDIR(st.st_mode)) {
            return 0; // Directory exists
        } else {
            return -1; // Path exists but is not a directory
        }
    }

    // Create directory with permissions 0755
    if (mkdir(path, 0755) != 0) {
        if (errno == ENOENT) {
            // Parent directory doesn't exist, try to create it recursively
            char *parent_path = strdup(path);
            if (!parent_path) {
                return -1;
            }

            const char *parent_dir = dirname(parent_path);
            int ret = create_directory(parent_dir);
            free(parent_path);

            if (ret != 0) {
                return -1;
            }

            // Try again to create the directory
            if (mkdir(path, 0755) != 0) {
                return -1;
            }
        } else {
            return -1;
        }
    }

    return 0;
}

// Function to checkpoint the database WAL file
int checkpoint_database(void) {
    int rc = SQLITE_OK;

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    if (!wal_mode_enabled) {
        log_info("WAL mode not enabled, no checkpoint needed");
        return 0;
    }

    pthread_mutex_lock(&db_mutex);

    log_info("Checkpointing WAL file");
    rc = sqlite3_exec(db, "PRAGMA wal_checkpoint(FULL);", NULL, NULL, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to checkpoint WAL: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }

    log_info("WAL checkpoint successful");
    pthread_mutex_unlock(&db_mutex);
    return 0;
}

int maybe_run_scheduled_database_backup(void) {
    if (!db || db_file_path[0] == '\0') {
        return 0;
    }

    if (g_config.db_backup_interval_minutes <= 0) {
        return 0;
    }

    time_t now = time(NULL);
    time_t interval_seconds = (time_t)g_config.db_backup_interval_minutes * 60;
    if (last_backup_time != 0 && now - last_backup_time < interval_seconds) {
        return 0;
    }

    if (perform_database_backup_cycle("scheduled", true) != 0) {
        return -1;
    }

    last_backup_time = now;
    return 0;
}

// Initialize the database
int init_database(const char *db_path) {
    int rc;
    char *err_msg = NULL;
    bool is_new_database = false;
    sqlite3 *test_db = NULL;
    sqlite3_stmt *stmt = NULL;

    log_info("Initializing database at path: %s", db_path);

    // Initialize SQLite with custom memory management
    // First, ensure SQLite is shut down to reset any previous state
    sqlite3_shutdown();

    // Configure SQLite for aggressive memory management
    rc = sqlite3_config(SQLITE_CONFIG_MEMSTATUS, 1);
    if (rc != SQLITE_OK) {
        log_warn("Failed to enable memory status tracking: %d", rc);
    }

    // Set a smaller page size to reduce memory usage
    rc = sqlite3_config(SQLITE_CONFIG_PAGECACHE, NULL, 0, 0);
    if (rc != SQLITE_OK) {
        log_warn("Failed to configure page cache: %d", rc);
    }

    // Initialize SQLite
    rc = sqlite3_initialize();
    if (rc != SQLITE_OK) {
        log_error("Failed to initialize SQLite: %d", rc);
        return -1;
    }

    // Set a soft heap limit to prevent excessive memory usage
    sqlite3_soft_heap_limit64((sqlite3_int64)8 * 1024 * 1024); // 8MB soft limit

    // Store the database path for backup/recovery operations
    strncpy(db_file_path, db_path, sizeof(db_file_path) - 1);
    db_file_path[sizeof(db_file_path) - 1] = '\0';

    // Create backup path by appending .bak to the database path
    snprintf(db_backup_path, sizeof(db_backup_path), "%s.bak", db_path);
    log_info("Backup path set to: %s", db_backup_path);

    struct stat backup_stat;
    if (stat(db_backup_path, &backup_stat) == 0) {
        last_backup_time = backup_stat.st_mtime;
    } else {
        last_backup_time = 0;
    }

    // Check if database already exists
    FILE *test_file = fopen(db_path, "r");
    if (test_file) {
        log_info("Database file already exists");
        fclose(test_file);

        // Check if the database is valid
        rc = sqlite3_open_v2(db_path, &test_db, SQLITE_OPEN_READONLY, NULL);
        if (rc != SQLITE_OK) {
            log_error("Database file exists but appears to be corrupt: %s",
                     test_db ? sqlite3_errmsg(test_db) : "unknown error");

            if (test_db) {
                sqlite3_close_v2(test_db);
                test_db = NULL;
            }

            // Check if we have a backup
            test_file = fopen(db_backup_path, "r");
            if (test_file) {
                log_info("Backup database file exists, attempting recovery");
                fclose(test_file);

                // Try to restore from backup
                if (restore_database_from_backup(db_backup_path, db_path) != 0) {
                    log_error("Failed to restore database from backup");
                    // Continue anyway, we'll try to create a new database
                } else {
                    log_info("Successfully restored database from backup");
                }
            } else {
                log_warn("No backup database file found, will create a new database");
            }
        } else {
            // Run integrity check on the database
            rc = sqlite3_prepare_v2(test_db, "PRAGMA integrity_check;", -1, &stmt, NULL);
            if (rc == SQLITE_OK) {
                if (sqlite3_step(stmt) == SQLITE_ROW) {
                    const char *result = (const char *)sqlite3_column_text(stmt, 0);
                    if (result && strcmp(result, "ok") != 0) {
                        log_error("Database integrity check failed: %s", result);

                        // Try to restore from backup
                        if (stmt) {
                            sqlite3_finalize(stmt);
                            stmt = NULL;
                        }

                        if (test_db) {
                            sqlite3_close_v2(test_db);
                            test_db = NULL;
                        }

                        test_file = fopen(db_backup_path, "r");
                        if (test_file) {
                            log_info("Backup database file exists, attempting recovery");
                            fclose(test_file);

                            if (restore_database_from_backup(db_backup_path, db_path) != 0) {
                                log_error("Failed to restore database from backup");
                                // Continue anyway, we'll try to repair the database
                            } else {
                                log_info("Successfully restored database from backup");
                            }
                        } else {
                            log_warn("No backup database file found, will attempt to repair");
                        }
                    } else {
                        log_info("Database integrity check passed");
                    }
                }
            }
            sqlite3_finalize(stmt);
            stmt = NULL;

            if (test_db) {
                // Release any cached schema before closing
                sqlite3_exec(test_db, "PRAGMA schema_version;", NULL, NULL, NULL);
                sqlite3_close_v2(test_db);
                test_db = NULL;
            }
        }
    } else {
        log_info("Database file does not exist, will be created");
        is_new_database = true;
    }

    // Initialize mutex
    if (pthread_mutex_init(&db_mutex, NULL) != 0) {
        log_error("Failed to initialize database mutex");
        return -1;
    }

    // Create directory for database if needed
    char *dir_path = strdup(db_path);
    if (!dir_path) {
        log_error("Failed to allocate memory for database directory path");
        return -1;
    }

    // Make a copy of the directory name before freeing dir_path
    char *dir = dirname(dir_path);
    char *dir_copy = strdup(dir);
    if (!dir_copy) {
        log_error("Failed to allocate memory for directory name copy");
        free(dir_path);
        return -1;
    }

    log_info("Creating database directory if needed: %s", dir_copy);
    if (create_directory(dir_copy) != 0) {
        log_error("Failed to create database directory: %s", dir_copy);
        free(dir_path);
        free(dir_copy);
        return -1;
    }
    free(dir_path);

    // Check directory permissions
    struct stat st;
    if (stat(dir_copy, &st) == 0) {
        log_info("Database directory permissions: %o", st.st_mode & 0777);
        if ((st.st_mode & 0200) == 0) {
            log_warn("Database directory is not writable");
        }
    }

    // Free the directory name copy
    free(dir_copy);

    // Open database with extended options for better error handling
    log_info("Opening database at: %s", db_path);
    rc = sqlite3_open_v2(db_path, &db,
                        SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE |
                        SQLITE_OPEN_FULLMUTEX | SQLITE_OPEN_SHAREDCACHE,
                        NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to open database: %s", db ? sqlite3_errmsg(db) : "unknown error");
        if (db) {
            sqlite3_close_v2(db); // Use close_v2 for better cleanup
            db = NULL;
        }
        return -1;
    }

    // Set busy timeout to avoid "database is locked" errors
    rc = sqlite3_busy_timeout(db, 10000);  // 10 seconds
    if (rc != SQLITE_OK) {
        log_warn("Failed to set busy timeout: %s", sqlite3_errmsg(db));
        // Continue anyway
    }

    // Enable WAL mode for better performance and crash resistance
    log_info("Enabling WAL mode for better crash resistance");
    rc = sqlite3_exec(db, "PRAGMA journal_mode=WAL;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to enable WAL mode: %s", err_msg);
        if (err_msg) {
            sqlite3_free(err_msg);
            err_msg = NULL;
        }
        // Continue anyway, but with less crash protection
    } else {
        // Verify WAL mode was enabled
        sqlite3_stmt *wal_stmt;
        rc = sqlite3_prepare_v2(db, "PRAGMA journal_mode;", -1, &wal_stmt, NULL);
        if (rc == SQLITE_OK && sqlite3_step(wal_stmt) == SQLITE_ROW) {
            const char *mode = (const char *)sqlite3_column_text(wal_stmt, 0);
            if (mode && strcmp(mode, "wal") == 0) {
                log_info("WAL mode successfully enabled");
                wal_mode_enabled = true;
            } else {
                log_warn("WAL mode not enabled, current mode: %s", mode ? mode : "unknown");
            }
        }
        if (wal_stmt) {
            sqlite3_finalize(wal_stmt);
        }
    }

    // Set synchronous mode to NORMAL for better performance while maintaining safety
    log_info("Setting synchronous mode to NORMAL");
    rc = sqlite3_exec(db, "PRAGMA synchronous=NORMAL;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_warn("Failed to set synchronous mode: %s", err_msg);
        if (err_msg) {
            sqlite3_free(err_msg);
            err_msg = NULL;
        }
        // Continue anyway
    }

    // Enable auto_vacuum to keep the database file size manageable
    log_info("Enabling auto_vacuum");
    rc = sqlite3_exec(db, "PRAGMA auto_vacuum=INCREMENTAL;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_warn("Failed to enable auto_vacuum: %s", err_msg);
        if (err_msg) {
            sqlite3_free(err_msg);
            err_msg = NULL;
        }
        // Continue anyway
    }

    // Check if we can write to the database
    log_info("Testing database write capability");
    rc = sqlite3_exec(db, "CREATE TABLE IF NOT EXISTS test_table (id INTEGER);", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create test table: %s", err_msg);
        if (err_msg) {
            sqlite3_free(err_msg);
            err_msg = NULL;
        }
        sqlite3_close_v2(db); // Use close_v2 for better cleanup
        db = NULL;
        return -1;
    }

    // Drop test table
    rc = sqlite3_exec(db, "DROP TABLE test_table;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_warn("Failed to drop test table: %s", err_msg);
        if (err_msg) {
            sqlite3_free(err_msg);
            err_msg = NULL;
        }
        // Continue anyway
    } else {
        log_info("Database write test successful");
    }

    // Enable foreign keys
    rc = sqlite3_exec(db, "PRAGMA foreign_keys = ON;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to enable foreign keys: %s", err_msg);
        if (err_msg) {
            sqlite3_free(err_msg);
            err_msg = NULL;
        }
        sqlite3_close_v2(db); // Use close_v2 for better cleanup
        db = NULL;
        return -1;
    }

    // Note: All table creation is handled by SQL migrations in db/migrations/
    // The migration system supports both filesystem SQL files and embedded migrations
    // compiled into the binary. See db_embedded_migrations.h and db_migrations.c.

    // Run database migrations (creates all tables, indexes, etc.)
    log_info("Running database migrations");
    rc = run_database_migrations();
    if (rc != 0) {
        log_error("Failed to run database migrations");
        // Finalize any remaining statements before closing
        sqlite3_stmt *cleanup_stmt;
        while ((cleanup_stmt = sqlite3_next_stmt(db, NULL)) != NULL) {
            log_info("Finalizing statement during error cleanup");
            sqlite3_finalize(cleanup_stmt);
        }
        sqlite3_close_v2(db); // Use close_v2 for better cleanup
        db = NULL;
        return -1;
    }

    log_info("Database initialized successfully");

    // Create an initial backup if this is a new database
    if (is_new_database) {
        log_info("Creating initial backup of new database");
        if (perform_database_backup_cycle("initial", true) == 0) {
            log_info("Initial backup created successfully");
            last_backup_time = time(NULL);
        } else {
            log_warn("Failed to create initial backup");
        }
    }

    return 0;
}

// Reset SQLite internal state to prevent memory leaks
static void reset_sqlite_internal_state(void) {
    log_info("Resetting SQLite internal state to prevent memory leaks");

    // Reset SQLite's internal memory management
    sqlite3_config(SQLITE_CONFIG_MEMSTATUS, 1);

    // Release unused memory back to the system
    sqlite3_release_memory(INT_MAX);

    // Reset SQLite's internal thread-local storage
    sqlite3_config(SQLITE_CONFIG_SINGLETHREAD);
    sqlite3_config(SQLITE_CONFIG_MULTITHREAD);

    log_info("SQLite internal state reset complete");
}

// Shutdown the database
void shutdown_database(void) {
    log_info("Starting database shutdown process");

    // Create a final backup before shutting down
    if (db != NULL && db_file_path[0] != '\0') {
        log_info("Creating final backup before shutdown");
        if (perform_database_backup_cycle("shutdown", true) == 0) {
            log_info("Final backup created successfully");
        } else {
            log_warn("Failed to create final backup");
        }
    }

    // First, ensure all threads have stopped using the database
    // by waiting a bit longer before acquiring the mutex
    usleep(500000);  // 500ms to allow in-flight operations to complete

    // Use a try-lock first to avoid deadlocks if the mutex is already locked
    int lock_result = pthread_mutex_trylock(&db_mutex);

    if (lock_result == 0) {
        // Successfully acquired the lock
        log_info("Successfully acquired database mutex for shutdown");
    } else if (lock_result == EBUSY) {
        // Mutex is already locked, wait with timeout
        log_warn("Database mutex is busy, waiting with timeout...");

        struct timespec timeout;
        clock_gettime(CLOCK_REALTIME, &timeout);
        timeout.tv_sec += 10; // Increased to 10 second timeout

        lock_result = pthread_mutex_timedlock(&db_mutex, &timeout);
        if (lock_result != 0) {
            log_error("Failed to acquire database mutex for shutdown: %s", strerror(lock_result));
            log_warn("Proceeding with database shutdown without lock - this may cause issues");
            // Continue without the lock - better than leaving the database open
        } else {
            log_info("Acquired database mutex after waiting");
        }
    } else {
        // Other error
        log_error("Error trying to lock database mutex: %s", strerror(lock_result));
        log_warn("Proceeding with database shutdown without lock - this may cause issues");
        // Continue without the lock - better than leaving the database open
    }

    if (db != NULL) {
        // Store the database handle locally but DO NOT set the global to NULL yet
        sqlite3 *db_to_close = db;

        // If WAL mode is enabled, checkpoint the database to ensure all changes are written to the main database file
        if (wal_mode_enabled) {
            log_info("Checkpointing WAL before closing database");
            int rc = sqlite3_exec(db_to_close, "PRAGMA wal_checkpoint(FULL);", NULL, NULL, NULL);
            if (rc != SQLITE_OK) {
                log_warn("Failed to checkpoint WAL: %s", sqlite3_errmsg(db_to_close));
            } else {
                log_info("WAL checkpoint successful");
            }
        }

        // No longer using tracked statements - we now rely on sqlite3_next_stmt to find all statements

        // Finalize all prepared statements before closing the database
        // This helps prevent "corrupted size vs. prev_size in fastbins" errors
        int stmt_count = 0;
        sqlite3_stmt *stmt;

        log_info("Finalizing all prepared statements");

        // First pass: finalize all statements we can find
        while ((stmt = sqlite3_next_stmt(db_to_close, NULL)) != NULL) {
            log_info("Finalizing prepared statement %d during database shutdown", ++stmt_count);
            sqlite3_finalize(stmt);
        }
        log_info("Finalized %d prepared statements", stmt_count);

        // Add a longer delay to ensure all statements are properly finalized
        // and any pending operations have completed
        usleep(500000);  // 500ms

        // Second pass: check for any remaining statements
        stmt_count = 0;
        while ((stmt = sqlite3_next_stmt(db_to_close, NULL)) != NULL) {
            log_info("Finalizing remaining statement %d in second pass", ++stmt_count);
            sqlite3_finalize(stmt);
        }

        if (stmt_count > 0) {
            log_info("Finalized %d additional statements in second pass", stmt_count);
            // Add another delay if we found more statements
            usleep(200000);  // 200ms
        }

        // Release any cached schema before closing
        log_info("Releasing cached schema resources");
        sqlite3_exec(db_to_close, "PRAGMA schema_version;", NULL, NULL, NULL);

        // Use sqlite3_close_v2 which is more forgiving with open statements
        log_info("Closing database with sqlite3_close_v2");
        int rc = sqlite3_close_v2(db_to_close);

        if (rc != SQLITE_OK) {
            log_warn("Error closing database: %s (code: %d)", sqlite3_errmsg(db_to_close), rc);

            // If there's an error, try to finalize any remaining statements
            log_info("Attempting to finalize any remaining statements");
            stmt_count = 0;
            while ((stmt = sqlite3_next_stmt(db_to_close, NULL)) != NULL) {
                log_info("Finalizing remaining statement %d in error recovery", ++stmt_count);
                sqlite3_finalize(stmt);
            }

            // Add another delay
            usleep(300000);  // 300ms

            // Try closing again
            log_info("Retrying database close");
            rc = sqlite3_close_v2(db_to_close);
            if (rc != SQLITE_OK) {
                log_error("Failed to close database after retry: %s (code: %d)", sqlite3_errmsg(db_to_close), rc);
            } else {
                log_info("Successfully closed database on retry");
            }
        } else {
            log_info("Successfully closed database");
        }

        // Only set the global handle to NULL after the database is successfully closed
        // or after all attempts to close it have been made
        db = NULL;

        // Reset SQLite internal state to prevent memory leaks
        reset_sqlite_internal_state();
    } else {
        log_warn("Database handle is already NULL during shutdown");
    }

    // Only unlock if we successfully locked
    if (lock_result == 0 || (lock_result == EBUSY && pthread_mutex_trylock(&db_mutex) == 0)) {
        pthread_mutex_unlock(&db_mutex);
    }

    // Add a longer delay before destroying the mutex to ensure no threads are still using it
    log_info("Waiting before destroying database mutex");
    usleep(500000);  // 500ms

    // Destroy the mutex
    log_info("Destroying database mutex");
    pthread_mutex_destroy(&db_mutex);

    // Final SQLite cleanup
    sqlite3_shutdown();

    log_info("Database shutdown complete");
}


// Get the database handle (for internal use by other database modules)
sqlite3 *get_db_handle(void) {
    return db;
}

// Get the database mutex (for internal use by other database modules)
pthread_mutex_t *get_db_mutex(void) {
    return &db_mutex;
}

// These functions have been moved to db_backup.c
