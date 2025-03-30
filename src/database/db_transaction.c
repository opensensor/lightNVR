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

#include "database/db_core.h"
#include "database/db_transaction.h"
#include "core/logger.h"

// Begin a database transaction with improved error handling
int begin_transaction(void) {
    int rc;
    char *err_msg = NULL;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    // Set a timeout for acquiring the mutex
    struct timespec timeout;
    clock_gettime(CLOCK_REALTIME, &timeout);
    timeout.tv_sec += 5; // 5 second timeout
    
    int lock_result = pthread_mutex_timedlock(db_mutex, &timeout);
    if (lock_result != 0) {
        log_error("Failed to acquire database mutex for transaction: %s", strerror(lock_result));
        return -1;
    }
    
    rc = sqlite3_exec(db, "BEGIN TRANSACTION;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to begin transaction: %s", err_msg);
        sqlite3_free(err_msg);
        pthread_mutex_unlock(db_mutex);
        return -1;
    }
    
    // Note: We do NOT unlock the mutex here - it will be unlocked by commit_transaction or rollback_transaction
    return 0;
}

// Commit a database transaction with improved error handling
int commit_transaction(void) {
    int rc;
    char *err_msg = NULL;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    
    if (!db) {
        log_error("Database not initialized");
        pthread_mutex_unlock(db_mutex); // Always unlock the mutex
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
        
        pthread_mutex_unlock(db_mutex);
        return -1;
    }
    
    pthread_mutex_unlock(db_mutex);
    return 0;
}

// Rollback a database transaction with improved error handling
int rollback_transaction(void) {
    int rc;
    char *err_msg = NULL;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    
    if (!db) {
        log_error("Database not initialized");
        pthread_mutex_unlock(db_mutex); // Always unlock the mutex
        return -1;
    }
    
    rc = sqlite3_exec(db, "ROLLBACK;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to rollback transaction: %s", err_msg);
        sqlite3_free(err_msg);
        // Continue to unlock the mutex even on error
    }
    
    pthread_mutex_unlock(db_mutex);
    return (rc == SQLITE_OK) ? 0 : -1;
}
