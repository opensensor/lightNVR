#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sys/time.h>
#include <sqlite3.h>
#include <pthread.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <errno.h>
#include <libgen.h>
#include <stdbool.h>
#include <fcntl.h>

#include "database/db_core.h"
#include "database/db_schema.h"
#include "core/logger.h"

// Database handle
static sqlite3 *db = NULL;

// Mutex for thread safety
static pthread_mutex_t db_mutex;

// Database path for backup/recovery operations
static char db_file_path[1024] = {0};

// Backup file path
static char db_backup_path[1024] = {0};

// Backup interval in seconds (default: 1 hour)
static int backup_interval = 3600;

// Last backup time
static time_t last_backup_time = 0;

// Flag to indicate if WAL mode is enabled
static bool wal_mode_enabled = false;

// Flag to indicate if a backup is in progress
static bool backup_in_progress = false;

// Forward declarations
static int backup_database(const char *source_path, const char *dest_path);
static int restore_database_from_backup(const char *backup_path, const char *db_path);
static int check_and_repair_database(void);

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
            
            char *parent_dir = dirname(parent_path);
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

// Initialize the database
int init_database(const char *db_path) {
    int rc;
    char *err_msg = NULL;
    bool is_new_database = false;
    
    log_info("Initializing database at path: %s", db_path);
    
    // Store the database path for backup/recovery operations
    strncpy(db_file_path, db_path, sizeof(db_file_path) - 1);
    db_file_path[sizeof(db_file_path) - 1] = '\0';
    
    // Create backup path by appending .bak to the database path
    snprintf(db_backup_path, sizeof(db_backup_path), "%s.bak", db_path);
    log_info("Backup path set to: %s", db_backup_path);
    
    // Check if database already exists
    FILE *test_file = fopen(db_path, "r");
    if (test_file) {
        log_info("Database file already exists");
        fclose(test_file);
        
        // Check if the database is valid
        sqlite3 *test_db;
        rc = sqlite3_open_v2(db_path, &test_db, SQLITE_OPEN_READONLY, NULL);
        if (rc != SQLITE_OK) {
            log_error("Database file exists but appears to be corrupt: %s", sqlite3_errmsg(test_db));
            sqlite3_close(test_db);
            
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
            sqlite3_stmt *stmt;
            rc = sqlite3_prepare_v2(test_db, "PRAGMA integrity_check;", -1, &stmt, NULL);
            if (rc == SQLITE_OK) {
                if (sqlite3_step(stmt) == SQLITE_ROW) {
                    const char *result = (const char *)sqlite3_column_text(stmt, 0);
                    if (result && strcmp(result, "ok") != 0) {
                        log_error("Database integrity check failed: %s", result);
                        
                        // Try to restore from backup
                        sqlite3_finalize(stmt);
                        sqlite3_close(test_db);
                        
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
                sqlite3_finalize(stmt);
            }
            sqlite3_close(test_db);
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
    
    char *dir = dirname(dir_path);
    log_info("Creating database directory if needed: %s", dir);
    if (create_directory(dir) != 0) {
        log_error("Failed to create database directory: %s", dir);
        free(dir_path);
        return -1;
    }
    free(dir_path);
    
    // Check directory permissions
    struct stat st;
    if (stat(dir, &st) == 0) {
        log_info("Database directory permissions: %o", st.st_mode & 0777);
        if ((st.st_mode & 0200) == 0) {
            log_warn("Database directory is not writable");
        }
    }
    
    // Open database with extended options for better error handling
    log_info("Opening database at: %s", db_path);
    rc = sqlite3_open_v2(db_path, &db, 
                        SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | 
                        SQLITE_OPEN_FULLMUTEX | SQLITE_OPEN_SHAREDCACHE, 
                        NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to open database: %s", sqlite3_errmsg(db));
        sqlite3_close(db);
        db = NULL;
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
        sqlite3_free(err_msg);
        // Continue anyway, but with less crash protection
    } else {
        // Verify WAL mode was enabled
        sqlite3_stmt *stmt;
        rc = sqlite3_prepare_v2(db, "PRAGMA journal_mode;", -1, &stmt, NULL);
        if (rc == SQLITE_OK && sqlite3_step(stmt) == SQLITE_ROW) {
            const char *mode = (const char *)sqlite3_column_text(stmt, 0);
            if (mode && strcmp(mode, "wal") == 0) {
                log_info("WAL mode successfully enabled");
                wal_mode_enabled = true;
            } else {
                log_warn("WAL mode not enabled, current mode: %s", mode ? mode : "unknown");
            }
        }
        if (stmt) {
            sqlite3_finalize(stmt);
        }
    }
    
    // Set synchronous mode to NORMAL for better performance while maintaining safety
    log_info("Setting synchronous mode to NORMAL");
    rc = sqlite3_exec(db, "PRAGMA synchronous=NORMAL;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_warn("Failed to set synchronous mode: %s", err_msg);
        sqlite3_free(err_msg);
        // Continue anyway
    }
    
    // Enable auto_vacuum to keep the database file size manageable
    log_info("Enabling auto_vacuum");
    rc = sqlite3_exec(db, "PRAGMA auto_vacuum=INCREMENTAL;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_warn("Failed to enable auto_vacuum: %s", err_msg);
        sqlite3_free(err_msg);
        // Continue anyway
    }
    
    // Check if we can write to the database
    log_info("Testing database write capability");
    rc = sqlite3_exec(db, "CREATE TABLE IF NOT EXISTS test_table (id INTEGER);", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create test table: %s", err_msg);
        sqlite3_free(err_msg);
        sqlite3_close(db);
        db = NULL;
        return -1;
    }
    
    // Drop test table
    rc = sqlite3_exec(db, "DROP TABLE test_table;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_warn("Failed to drop test table: %s", err_msg);
        sqlite3_free(err_msg);
        // Continue anyway
    } else {
        log_info("Database write test successful");
    }
    
    // Enable foreign keys
    rc = sqlite3_exec(db, "PRAGMA foreign_keys = ON;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to enable foreign keys: %s", err_msg);
        sqlite3_free(err_msg);
        sqlite3_close(db);
        db = NULL;
        return -1;
    }
    
    // Create events table
    const char *create_events_table = 
        "CREATE TABLE IF NOT EXISTS events ("
        "id INTEGER PRIMARY KEY AUTOINCREMENT,"
        "type INTEGER NOT NULL,"
        "timestamp INTEGER NOT NULL,"
        "stream_name TEXT,"
        "description TEXT NOT NULL,"
        "details TEXT"
        ");";
    
    rc = sqlite3_exec(db, create_events_table, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create events table: %s", err_msg);
        sqlite3_free(err_msg);
        sqlite3_close(db);
        db = NULL;
        return -1;
    }
    
    // Create detections table
    const char *create_detections_table = 
        "CREATE TABLE IF NOT EXISTS detections ("
        "id INTEGER PRIMARY KEY AUTOINCREMENT,"
        "stream_name TEXT NOT NULL,"
        "timestamp INTEGER NOT NULL,"
        "label TEXT NOT NULL,"
        "confidence REAL NOT NULL,"
        "x REAL NOT NULL,"
        "y REAL NOT NULL,"
        "width REAL NOT NULL,"
        "height REAL NOT NULL"
        ");";
    
    rc = sqlite3_exec(db, create_detections_table, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create detections table: %s", err_msg);
        sqlite3_free(err_msg);
        sqlite3_close(db);
        db = NULL;
        return -1;
    }
    
    // Create recordings table
    const char *create_recordings_table = 
        "CREATE TABLE IF NOT EXISTS recordings ("
        "id INTEGER PRIMARY KEY AUTOINCREMENT,"
        "stream_name TEXT NOT NULL,"
        "file_path TEXT NOT NULL,"
        "start_time INTEGER NOT NULL,"
        "end_time INTEGER,"
        "size_bytes INTEGER,"
        "width INTEGER,"
        "height INTEGER,"
        "fps INTEGER,"
        "codec TEXT,"
        "is_complete INTEGER DEFAULT 0"
        ");";
        
    // Create streams table for storing stream configurations
    const char *create_streams_table = 
        "CREATE TABLE IF NOT EXISTS streams ("
        "id INTEGER PRIMARY KEY AUTOINCREMENT,"
        "name TEXT NOT NULL UNIQUE,"
        "url TEXT NOT NULL,"
        "enabled INTEGER DEFAULT 1,"
        "streaming_enabled INTEGER DEFAULT 1,"
        "width INTEGER DEFAULT 1280,"
        "height INTEGER DEFAULT 720,"
        "fps INTEGER DEFAULT 30,"
        "codec TEXT DEFAULT 'h264',"
        "priority INTEGER DEFAULT 5,"
        "record INTEGER DEFAULT 1,"
        "segment_duration INTEGER DEFAULT 900"
        ");";
    
    rc = sqlite3_exec(db, create_recordings_table, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create recordings table: %s", err_msg);
        sqlite3_free(err_msg);
        sqlite3_close(db);
        db = NULL;
        return -1;
    }
    
    // Create streams table
    rc = sqlite3_exec(db, create_streams_table, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create streams table: %s", err_msg);
        sqlite3_free(err_msg);
        sqlite3_close(db);
        db = NULL;
        return -1;
    }
    
    // Create indexes for faster queries
    const char *create_indexes = 
        "CREATE INDEX IF NOT EXISTS idx_events_timestamp ON events (timestamp);"
        "CREATE INDEX IF NOT EXISTS idx_events_type ON events (type);"
        "CREATE INDEX IF NOT EXISTS idx_events_stream ON events (stream_name);"
        "CREATE INDEX IF NOT EXISTS idx_recordings_start_time ON recordings (start_time);"
        "CREATE INDEX IF NOT EXISTS idx_recordings_end_time ON recordings (end_time);"
        "CREATE INDEX IF NOT EXISTS idx_recordings_stream ON recordings (stream_name);"
        "CREATE INDEX IF NOT EXISTS idx_recordings_complete_stream_start ON recordings (is_complete, stream_name, start_time);"
        "CREATE INDEX IF NOT EXISTS idx_streams_name ON streams (name);"
        "CREATE INDEX IF NOT EXISTS idx_detections_stream_timestamp ON detections (stream_name, timestamp);"
        "CREATE INDEX IF NOT EXISTS idx_detections_timestamp ON detections (timestamp);";
    
    rc = sqlite3_exec(db, create_indexes, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create indexes: %s", err_msg);
        sqlite3_free(err_msg);
        sqlite3_close(db);
        db = NULL;
        return -1;
    }
    
    // Initialize schema management system
    log_info("Initializing schema management system");
    rc = init_schema_management();
    if (rc != 0) {
        log_error("Failed to initialize schema management system");
        sqlite3_close(db);
        db = NULL;
        return -1;
    }
    
    // Run schema migrations
    log_info("Running schema migrations");
    rc = run_schema_migrations();
    if (rc != 0) {
        log_error("Failed to run schema migrations");
        sqlite3_close(db);
        db = NULL;
        return -1;
    }
    
    log_info("Database initialized successfully");
    
    // Create an initial backup if this is a new database
    if (is_new_database) {
        log_info("Creating initial backup of new database");
        if (backup_database(db_file_path, db_backup_path) == 0) {
            log_info("Initial backup created successfully");
            last_backup_time = time(NULL);
        } else {
            log_warn("Failed to create initial backup");
        }
    }
    
    return 0;
}

// Shutdown the database
void shutdown_database(void) {
    log_info("Starting database shutdown process");
    
    // Create a final backup before shutting down
    if (db != NULL && db_file_path[0] != '\0') {
        log_info("Creating final backup before shutdown");
        if (backup_database(db_file_path, db_backup_path) == 0) {
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
    
    log_info("Database shutdown complete");
}

// Begin a database transaction with improved error handling
int begin_transaction(void) {
    int rc;
    char *err_msg = NULL;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    // Set a timeout for acquiring the mutex
    struct timespec timeout;
    clock_gettime(CLOCK_REALTIME, &timeout);
    timeout.tv_sec += 5; // 5 second timeout
    
    int lock_result = pthread_mutex_timedlock(&db_mutex, &timeout);
    if (lock_result != 0) {
        log_error("Failed to acquire database mutex for transaction: %s", strerror(lock_result));
        return -1;
    }
    
    rc = sqlite3_exec(db, "BEGIN TRANSACTION;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to begin transaction: %s", err_msg);
        sqlite3_free(err_msg);
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    // Note: We do NOT unlock the mutex here - it will be unlocked by commit_transaction or rollback_transaction
    return 0;
}

// Commit a database transaction with improved error handling
int commit_transaction(void) {
    int rc;
    char *err_msg = NULL;
    
    if (!db) {
        log_error("Database not initialized");
        pthread_mutex_unlock(&db_mutex); // Always unlock the mutex
        return -1;
    }
    
    rc = sqlite3_exec(db, "COMMIT;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to commit transaction: %s", err_msg);
        sqlite3_free(err_msg);
        
        // Try to rollback on commit failure
        char *rollback_err = NULL;
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, &rollback_err);
        if (rollback_err) {
            sqlite3_free(rollback_err);
        }
        
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    pthread_mutex_unlock(&db_mutex);
    return 0;
}

// Rollback a database transaction with improved error handling
int rollback_transaction(void) {
    int rc;
    char *err_msg = NULL;
    
    if (!db) {
        log_error("Database not initialized");
        pthread_mutex_unlock(&db_mutex); // Always unlock the mutex
        return -1;
    }
    
    rc = sqlite3_exec(db, "ROLLBACK;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to rollback transaction: %s", err_msg);
        sqlite3_free(err_msg);
        // Continue to unlock the mutex even on error
    }
    
    pthread_mutex_unlock(&db_mutex);
    return (rc == SQLITE_OK) ? 0 : -1;
}

// Get the database size
int64_t get_database_size(void) {
    int rc;
    sqlite3_stmt *stmt;
    int64_t size = -1;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    pthread_mutex_lock(&db_mutex);
    
    const char *sql = "PRAGMA page_count;";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    if (sqlite3_step(stmt) == SQLITE_ROW) {
        int64_t page_count = sqlite3_column_int64(stmt, 0);
        sqlite3_finalize(stmt);
        
        sql = "PRAGMA page_size;";
        rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
        if (rc != SQLITE_OK) {
            log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
            pthread_mutex_unlock(&db_mutex);
            return -1;
        }
        
        if (sqlite3_step(stmt) == SQLITE_ROW) {
            int64_t page_size = sqlite3_column_int64(stmt, 0);
            size = page_count * page_size;
        }
    }
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(&db_mutex);
    
    return size;
}

// Vacuum the database to reclaim space
int vacuum_database(void) {
    int rc;
    char *err_msg = NULL;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    pthread_mutex_lock(&db_mutex);
    
    rc = sqlite3_exec(db, "VACUUM;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to vacuum database: %s", err_msg);
        sqlite3_free(err_msg);
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    pthread_mutex_unlock(&db_mutex);
    return 0;
}

// Check database integrity with more detailed diagnostics
int check_database_integrity(void) {
    int rc;
    sqlite3_stmt *stmt;
    int result = -1;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    pthread_mutex_lock(&db_mutex);
    
    // First run a quick check
    const char *sql = "PRAGMA quick_check;";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare quick_check statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    if (sqlite3_step(stmt) == SQLITE_ROW) {
        const char *quick_check_result = (const char *)sqlite3_column_text(stmt, 0);
        if (quick_check_result && strcmp(quick_check_result, "ok") == 0) {
            log_info("Database quick check passed");
            result = 0; // Database is valid
        } else {
            log_warn("Database quick check failed: %s", quick_check_result ? quick_check_result : "unknown error");
            // Continue with full integrity check
            result = -1;
        }
    }
    
    sqlite3_finalize(stmt);
    
    // If quick check failed or we want to be thorough, run full integrity check
    if (result != 0) {
        sql = "PRAGMA integrity_check;";
        
        rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
        if (rc != SQLITE_OK) {
            log_error("Failed to prepare integrity_check statement: %s", sqlite3_errmsg(db));
            pthread_mutex_unlock(&db_mutex);
            return -1;
        }
        
        if (sqlite3_step(stmt) == SQLITE_ROW) {
            const char *integrity_result = (const char *)sqlite3_column_text(stmt, 0);
            if (integrity_result && strcmp(integrity_result, "ok") == 0) {
                log_info("Database integrity check passed");
                result = 0; // Database is valid
            } else {
                log_error("Database integrity check failed: %s", integrity_result ? integrity_result : "unknown error");
                
                // Try to repair the database
                if (check_and_repair_database() == 0) {
                    log_info("Database repaired successfully");
                    result = 0;
                } else {
                    log_error("Failed to repair database");
                    result = -1;
                }
            }
        }
        
        sqlite3_finalize(stmt);
    }
    
    pthread_mutex_unlock(&db_mutex);
    
    return result;
}

// Execute a SQL query and get the results
int database_execute_query(const char *sql, void **result, int *rows, int *cols) {
    int rc;
    char *err_msg = NULL;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    if (!sql || !result || !rows || !cols) {
        log_error("Invalid parameters for database_execute_query");
        return -1;
    }
    
    pthread_mutex_lock(&db_mutex);
    
    // Execute the query and get results as a table
    rc = sqlite3_get_table(db, sql, (char ***)result, rows, cols, &err_msg);
    
    if (rc != SQLITE_OK) {
        log_error("SQL error: %s", err_msg);
        sqlite3_free(err_msg);
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    pthread_mutex_unlock(&db_mutex);
    return 0;
}

// Get the database handle (for internal use by other database modules)
sqlite3 *get_db_handle(void) {
    return db;
}

// Get the database mutex (for internal use by other database modules)
pthread_mutex_t *get_db_mutex(void) {
    return &db_mutex;
}

// Backup the database to a specified path
static int backup_database(const char *source_path, const char *dest_path) {
    int rc;
    sqlite3 *source_db = NULL;
    sqlite3 *dest_db = NULL;
    sqlite3_backup *backup = NULL;
    
    if (backup_in_progress) {
        log_warn("Backup already in progress, skipping");
        return -1;
    }
    
    backup_in_progress = true;
    
    log_info("Starting database backup from %s to %s", source_path, dest_path);
    
    // Open the source database
    rc = sqlite3_open_v2(source_path, &source_db, SQLITE_OPEN_READONLY, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to open source database for backup: %s", sqlite3_errmsg(source_db));
        sqlite3_close(source_db);
        backup_in_progress = false;
        return -1;
    }
    
    // Open the destination database
    rc = sqlite3_open_v2(dest_path, &dest_db, SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to open destination database for backup: %s", sqlite3_errmsg(dest_db));
        sqlite3_close(source_db);
        sqlite3_close(dest_db);
        backup_in_progress = false;
        return -1;
    }
    
    // Initialize the backup
    backup = sqlite3_backup_init(dest_db, "main", source_db, "main");
    if (!backup) {
        log_error("Failed to initialize backup: %s", sqlite3_errmsg(dest_db));
        sqlite3_close(source_db);
        sqlite3_close(dest_db);
        backup_in_progress = false;
        return -1;
    }
    
    // Perform the backup
    rc = sqlite3_backup_step(backup, -1);
    if (rc != SQLITE_DONE) {
        log_error("Failed to perform backup: %s", sqlite3_errmsg(dest_db));
        sqlite3_backup_finish(backup);
        sqlite3_close(source_db);
        sqlite3_close(dest_db);
        backup_in_progress = false;
        return -1;
    }
    
    // Finish the backup
    rc = sqlite3_backup_finish(backup);
    if (rc != SQLITE_OK) {
        log_error("Failed to finish backup: %s", sqlite3_errmsg(dest_db));
        sqlite3_close(source_db);
        sqlite3_close(dest_db);
        backup_in_progress = false;
        return -1;
    }
    
    // Close the databases
    sqlite3_close(source_db);
    sqlite3_close(dest_db);
    
    log_info("Database backup completed successfully");
    backup_in_progress = false;
    return 0;
}

// Restore database from backup
static int restore_database_from_backup(const char *backup_path, const char *db_path) {
    int rc;
    
    log_info("Restoring database from backup: %s to %s", backup_path, db_path);
    
    // First, check if the backup file exists
    struct stat st;
    if (stat(backup_path, &st) != 0) {
        log_error("Backup file does not exist: %s", backup_path);
        return -1;
    }
    
    // Close the current database if it's open
    if (db) {
        log_info("Closing current database before restore");
        sqlite3_close_v2(db);
        db = NULL;
    }
    
    // Copy the backup file to the database file
    FILE *src = fopen(backup_path, "rb");
    if (!src) {
        log_error("Failed to open backup file for reading: %s", strerror(errno));
        return -1;
    }
    
    FILE *dst = fopen(db_path, "wb");
    if (!dst) {
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
    sqlite3_stmt *stmt;
    rc = sqlite3_prepare_v2(test_db, "PRAGMA integrity_check;", -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        if (sqlite3_step(stmt) == SQLITE_ROW) {
            const char *result = (const char *)sqlite3_column_text(stmt, 0);
            if (result && strcmp(result, "ok") != 0) {
                log_error("Restored database integrity check failed: %s", result);
                sqlite3_finalize(stmt);
                sqlite3_close(test_db);
                return -1;
            }
        }
        sqlite3_finalize(stmt);
    }
    
    sqlite3_close(test_db);
    
    log_info("Database restored successfully from backup");
    return 0;
}

// Check and repair database
static int check_and_repair_database(void) {
    int rc;
    char *err_msg = NULL;
    
    log_info("Checking and repairing database");
    
    // First, try to restore from backup if available
    if (db_backup_path[0] != '\0') {
        struct stat st;
        if (stat(db_backup_path, &st) == 0) {
            log_info("Backup file exists, attempting to restore");
            
            // Close the current database
            if (db) {
                sqlite3_close_v2(db);
                db = NULL;
            }
            
            if (restore_database_from_backup(db_backup_path, db_file_path) == 0) {
                log_info("Successfully restored database from backup");
                
                // Reopen the database
                rc = sqlite3_open_v2(db_file_path, &db, 
                                    SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | 
                                    SQLITE_OPEN_FULLMUTEX | SQLITE_OPEN_SHAREDCACHE, 
                                    NULL);
                if (rc != SQLITE_OK) {
                    log_error("Failed to reopen database after restore: %s", sqlite3_errmsg(db));
                    sqlite3_close(db);
                    db = NULL;
                    return -1;
                }
                
                return 0;
            } else {
                log_error("Failed to restore database from backup");
                // Continue with other repair attempts
            }
        }
    }
    
    // If we can't restore from backup, try to repair the database
    log_info("Attempting to repair database with PRAGMA integrity_check");
    
    // Try to open the database in read-write mode
    if (!db) {
        rc = sqlite3_open_v2(db_file_path, &db, 
                            SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | 
                            SQLITE_OPEN_FULLMUTEX | SQLITE_OPEN_SHAREDCACHE, 
                            NULL);
        if (rc != SQLITE_OK) {
            log_error("Failed to open database for repair: %s", sqlite3_errmsg(db));
            sqlite3_close(db);
            db = NULL;
            return -1;
        }
    }
    
    // Run integrity check
    rc = sqlite3_exec(db, "PRAGMA integrity_check;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Integrity check failed: %s", err_msg);
        sqlite3_free(err_msg);
        
        // Try to recover by recreating the database
        log_warn("Attempting to recover by recreating the database");
        
        // Close the current database
        sqlite3_close_v2(db);
        db = NULL;
        
        // Rename the corrupt database
        char corrupt_path[1024];
        snprintf(corrupt_path, sizeof(corrupt_path), "%s.corrupt", db_file_path);
        if (rename(db_file_path, corrupt_path) != 0) {
            log_error("Failed to rename corrupt database: %s", strerror(errno));
            return -1;
        }
        
        log_info("Renamed corrupt database to %s", corrupt_path);
        
        // Create a new database
        rc = sqlite3_open_v2(db_file_path, &db, 
                            SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | 
                            SQLITE_OPEN_FULLMUTEX | SQLITE_OPEN_SHAREDCACHE, 
                            NULL);
        if (rc != SQLITE_OK) {
            log_error("Failed to create new database: %s", sqlite3_errmsg(db));
            sqlite3_close(db);
            db = NULL;
            return -1;
        }
        
        log_info("Created new database");
        
        // Initialize the new database with the schema
        // This will be done by the init_database function when it's called again
        
        return 0;
    }
    
    log_info("Database repair completed successfully");
    return 0;
}
