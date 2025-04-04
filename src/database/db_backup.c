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

#include "database/db_core.h"
#include "database/db_backup.h"
#include "database/db_schema_utils.h"
#include "core/logger.h"

// Flag to indicate if a backup is in progress
static bool backup_in_progress = false;

// Backup the database to a specified path
int backup_database(const char *source_path, const char *dest_path) {
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
int restore_database_from_backup(const char *backup_path, const char *db_path) {
    int rc;
    sqlite3 *db = get_db_handle();
    
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
        // Note: We don't set the global db to NULL here, as that's handled by the core module
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
    sqlite3_stmt *stmt = NULL;
    const char *integrity_sql = "PRAGMA integrity_check;";
    
    rc = sqlite3_prepare_v2(test_db, integrity_sql, -1, &stmt, NULL);
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
int check_and_repair_database(void) {
    int rc;
    char *err_msg = NULL;
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
