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

#include "database/db_core.h"
#include "database/db_schema.h"
#include "core/logger.h"

// Database handle
static sqlite3 *db = NULL;

// Mutex for thread safety
static pthread_mutex_t db_mutex;

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
    
    log_info("Initializing database at path: %s", db_path);
    
    // Check if database already exists
    FILE *test_file = fopen(db_path, "r");
    if (test_file) {
        log_info("Database file already exists");
        fclose(test_file);
    } else {
        log_info("Database file does not exist, will be created");
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
    
    // Open database
    log_info("Opening database at: %s", db_path);
    rc = sqlite3_open(db_path, &db);
    if (rc != SQLITE_OK) {
        log_error("Failed to open database: %s", sqlite3_errmsg(db));
        sqlite3_close(db);
        db = NULL;
        return -1;
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
    return 0;
}

// Shutdown the database
void shutdown_database(void) {
    log_info("Starting database shutdown process");
    
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
        timeout.tv_sec += 5; // 5 second timeout
        
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
        // Finalize all prepared statements before closing the database
        // This helps prevent "corrupted size vs. prev_size in fastbins" errors
        int stmt_count = 0;
        sqlite3_stmt *stmt;
        
        log_info("Finalizing all prepared statements");
        while ((stmt = sqlite3_next_stmt(db, NULL)) != NULL) {
            log_info("Finalizing prepared statement %d during database shutdown", ++stmt_count);
            sqlite3_finalize(stmt);
        }
        log_info("Finalized %d prepared statements", stmt_count);
        
        // Add a small delay to ensure all statements are properly finalized
        usleep(100000);  // 100ms - increased from 50ms
        
        // Make a local copy of the database handle
        sqlite3 *db_to_close = db;
        // Set global handle to NULL first to prevent use-after-free
        db = NULL;
        
        // Use sqlite3_close_v2 which is more forgiving with open statements
        log_info("Closing database with sqlite3_close_v2");
        int rc = sqlite3_close_v2(db_to_close);
        if (rc != SQLITE_OK) {
            log_warn("Error closing database: %s (code: %d)", sqlite3_errmsg(db_to_close), rc);
            
            // If there's an error, try to finalize any remaining statements
            log_info("Attempting to finalize any remaining statements");
            stmt_count = 0;
            while ((stmt = sqlite3_next_stmt(db_to_close, NULL)) != NULL) {
                log_info("Finalizing remaining statement %d", ++stmt_count);
                sqlite3_finalize(stmt);
            }
            
            // Add another delay
            usleep(100000);  // 100ms
            
            // Try closing again
            log_info("Retrying database close");
            rc = sqlite3_close_v2(db_to_close);
            if (rc != SQLITE_OK) {
                log_error("Failed to close database after retry: %s (code: %d)", sqlite3_errmsg(db_to_close), rc);
            }
        }
    } else {
        log_warn("Database handle is already NULL during shutdown");
    }
    
    // Only unlock if we successfully locked
    if (lock_result == 0 || (lock_result == EBUSY && pthread_mutex_trylock(&db_mutex) == 0)) {
        pthread_mutex_unlock(&db_mutex);
    }
    
    // Add a longer delay before destroying the mutex to ensure no threads are still using it
    log_info("Waiting before destroying database mutex");
    usleep(200000);  // 200ms - increased from 100ms
    
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

// Check database integrity
int check_database_integrity(void) {
    int rc;
    sqlite3_stmt *stmt;
    int result = -1;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    pthread_mutex_lock(&db_mutex);
    
    const char *sql = "PRAGMA integrity_check;";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    if (sqlite3_step(stmt) == SQLITE_ROW) {
        const char *integrity_result = (const char *)sqlite3_column_text(stmt, 0);
        if (integrity_result && strcmp(integrity_result, "ok") == 0) {
            result = 0; // Database is valid
        } else {
            log_error("Database integrity check failed: %s", integrity_result ? integrity_result : "unknown error");
            result = -1;
        }
    }
    
    sqlite3_finalize(stmt);
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
