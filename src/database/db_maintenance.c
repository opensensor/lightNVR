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
#include <errno.h>
#include <stdint.h>

#include "database/db_core.h"
#include "database/db_maintenance.h"
#include "database/db_backup.h"
#include "core/logger.h"

// Get the database size
int64_t get_database_size(void) {
    int rc;
    sqlite3_stmt *stmt;
    int64_t size = -1;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    pthread_mutex_lock(db_mutex);
    
    const char *sql = "PRAGMA page_count;";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }
    
    if (sqlite3_step(stmt) == SQLITE_ROW) {
        int64_t page_count = sqlite3_column_int64(stmt, 0);
        sqlite3_finalize(stmt);
        
        sql = "PRAGMA page_size;";
        rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
        if (rc != SQLITE_OK) {
            log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
            pthread_mutex_unlock(db_mutex);
            return -1;
        }
        
        if (sqlite3_step(stmt) == SQLITE_ROW) {
            int64_t page_size = sqlite3_column_int64(stmt, 0);
            size = page_count * page_size;
        }
    }
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    
    return size;
}

// Vacuum the database to reclaim space
int vacuum_database(void) {
    int rc;
    char *err_msg = NULL;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    pthread_mutex_lock(db_mutex);
    
    rc = sqlite3_exec(db, "VACUUM;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to vacuum database: %s", err_msg);
        sqlite3_free(err_msg);
        pthread_mutex_unlock(db_mutex);
        return -1;
    }
    
    pthread_mutex_unlock(db_mutex);
    return 0;
}

// Check database integrity with more detailed diagnostics
int check_database_integrity(void) {
    int rc;
    sqlite3_stmt *stmt;
    int result = -1;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    pthread_mutex_lock(db_mutex);
    
    // First run a quick check
    const char *sql = "PRAGMA quick_check;";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare quick_check statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
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
            pthread_mutex_unlock(db_mutex);
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
    
    pthread_mutex_unlock(db_mutex);
    
    return result;
}

// Execute a SQL query and get the results
int database_execute_query(const char *sql, void **result, int *rows, int *cols) {
    int rc;
    char *err_msg = NULL;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    if (!sql || !result || !rows || !cols) {
        log_error("Invalid parameters for database_execute_query");
        return -1;
    }
    
    pthread_mutex_lock(db_mutex);
    
    // Execute the query and get results as a table
    rc = sqlite3_get_table(db, sql, (char ***)result, rows, cols, &err_msg);
    
    if (rc != SQLITE_OK) {
        log_error("SQL error: %s", err_msg);
        sqlite3_free(err_msg);
        pthread_mutex_unlock(db_mutex);
        return -1;
    }
    
    pthread_mutex_unlock(db_mutex);
    return 0;
}
