#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sqlite3.h>
#include <stdbool.h>

#include "database/db_schema.h"
#include "database/db_core.h"
#include "database/db_schema_utils.h"
#include "database/db_motion_config.h"
#include "database/db_zones.h"
#include "core/logger.h"

// Note: Schema migrations are now handled by the SQL-file based migration system
// in db_migrations.c. This file only maintains backward compatibility with the
// legacy schema_version table.

/**
 * Initialize the schema management system
 */
int init_schema_management(void) {
    int rc;
    char *err_msg = NULL;

    sqlite3 *db = get_db_handle();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    // Create schema_version table if it doesn't exist
    const char *create_version_table =
        "CREATE TABLE IF NOT EXISTS schema_version ("
        "id INTEGER PRIMARY KEY CHECK (id = 1),"  // Only one row allowed
        "version INTEGER NOT NULL,"
        "updated_at INTEGER NOT NULL"
        ");";

    rc = sqlite3_exec(db, create_version_table, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create schema_version table: %s", err_msg);
        if (err_msg) {
            sqlite3_free(err_msg);
            err_msg = NULL;
        }
        return -1;
    }

    // Check if we need to initialize the version
    sqlite3_stmt *stmt;
    rc = sqlite3_prepare_v2(db, "SELECT COUNT(*) FROM schema_version;", -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return rc;
    }

    if (sqlite3_step(stmt) == SQLITE_ROW) {
        int count = sqlite3_column_int(stmt, 0);
        sqlite3_finalize(stmt);

        if (count == 0) {
            // No version record, initialize to version 1
            const char *init_version =
                "INSERT INTO schema_version (id, version, updated_at) VALUES (1, 1, strftime('%s','now'));";

            rc = sqlite3_exec(db, init_version, NULL, NULL, &err_msg);
            if (rc != SQLITE_OK) {
                log_error("Failed to initialize schema version: %s", err_msg);
                if (err_msg) {
                    sqlite3_free(err_msg);
                    err_msg = NULL;
                }
                return -1;
            }

            log_info("Initialized database schema to version 1");
        }
    } else {
        sqlite3_finalize(stmt);
        log_error("Failed to check schema_version table");
        return -1;
    }

    return 0;
}

/**
 * Get the current schema version
 */
int get_schema_version(void) {
    int rc;
    sqlite3_stmt *stmt = NULL;
    int version = -1;

    sqlite3 *db = get_db_handle();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    const char *sql = "SELECT version FROM schema_version WHERE id = 1;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return rc;
    }

    if (sqlite3_step(stmt) == SQLITE_ROW) {
        version = sqlite3_column_int(stmt, 0);
    }

    if (stmt) {
        sqlite3_finalize(stmt);
        stmt = NULL;
    }

    return version;
}

/**
 * Legacy function - no longer used
 * Migrations are now handled by the SQL-file based system in db_migrations.c
 * This stub is kept for backward compatibility only
 */
int run_schema_migrations(void) {
    log_warn("run_schema_migrations() is deprecated - migrations are now handled by run_database_migrations()");
    return 0;
}

/**
 * Check if a column exists in a table
 */
bool column_exists(const char *table_name, const char *column_name) {
    int rc;
    sqlite3_stmt *stmt = NULL;
    sqlite3_stmt *check_table_stmt = NULL;
    bool exists = false;

    sqlite3 *db = get_db_handle();

    if (!db || !table_name || !column_name) {
        log_error("Invalid parameters for column_exists: table=%s, column=%s",
                 table_name ? table_name : "NULL",
                 column_name ? column_name : "NULL");
        return false;
    }

    log_debug("Checking if column %s exists in table %s", column_name, table_name);

    // First check if the table exists
    char check_table_sql[256];
    snprintf(check_table_sql, sizeof(check_table_sql),
            "SELECT name FROM sqlite_master WHERE type='table' AND name='%s';", table_name);

    rc = sqlite3_prepare_v2(db, check_table_sql, -1, &check_table_stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement to check table existence");
        return false;
    }

    bool table_exists = false;
    if (sqlite3_step(check_table_stmt) == SQLITE_ROW) {
        table_exists = true;
    }

    if (check_table_stmt) {
        sqlite3_finalize(check_table_stmt);
        check_table_stmt = NULL;
    }

    if (!table_exists) {
        log_error("Table %s does not exist", table_name);
        return false;
    }

    // Now check if the column exists
    char sql[256];
    snprintf(sql, sizeof(sql), "PRAGMA table_info(%s);", table_name);

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement to check column existence");
        return false;
    }

    while (sqlite3_step(stmt) == SQLITE_ROW) {
        const char *name = (const char *)sqlite3_column_text(stmt, 1);
        if (name && strcmp(name, column_name) == 0) {
            exists = true;
            break;
        }
    }

    if (stmt) {
        sqlite3_finalize(stmt);
        stmt = NULL;
    }

    log_debug("Column %s %s in table %s", column_name, exists ? "exists" : "does not exist", table_name);
    return exists;
}

/**
 * Add a column to a table if it doesn't exist
 */
int add_column_if_not_exists(const char *table_name, const char *column_name, const char *column_def) {
    int rc;
    char *err_msg = NULL;
    sqlite3_stmt *check_table_stmt = NULL;
    sqlite3_stmt *check_column_stmt = NULL;

    sqlite3 *db = get_db_handle();

    if (!db || !table_name || !column_name || !column_def) {
        log_error("Invalid parameters for add_column_if_not_exists: table=%s, column=%s, def=%s",
                 table_name ? table_name : "NULL",
                 column_name ? column_name : "NULL",
                 column_def ? column_def : "NULL");
        return -1;
    }

    log_info("Checking if column %s exists in table %s", column_name, table_name);

    // First check if the table exists
    char check_table_sql[256];
    snprintf(check_table_sql, sizeof(check_table_sql),
            "SELECT name FROM sqlite_master WHERE type='table' AND name='%s';", table_name);

    rc = sqlite3_prepare_v2(db, check_table_sql, -1, &check_table_stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement to check table existence");
        return -1;
    }

    bool table_exists = false;
    if (sqlite3_step(check_table_stmt) == SQLITE_ROW) {
        table_exists = true;
    }

    if (check_table_stmt) {
        sqlite3_finalize(check_table_stmt);
        check_table_stmt = NULL;
    }

    if (!table_exists) {
        log_error("Table %s does not exist", table_name);
        return -1;
    }

    // Check if column already exists (directly, not using column_exists function to avoid confusion)
    char check_column_sql[256];
    snprintf(check_column_sql, sizeof(check_column_sql), "PRAGMA table_info(%s);", table_name);

    rc = sqlite3_prepare_v2(db, check_column_sql, -1, &check_column_stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement to check column existence");
        return -1;
    }

    bool col_found = false;
    while (sqlite3_step(check_column_stmt) == SQLITE_ROW) {
        const char *name = (const char *)sqlite3_column_text(check_column_stmt, 1);
        if (name && strcmp(name, column_name) == 0) {
            col_found = true;
            break;
        }
    }

    if (check_column_stmt) {
        sqlite3_finalize(check_column_stmt);
        check_column_stmt = NULL;
    }

    if (col_found) {
        log_info("Column %s already exists in table %s", column_name, table_name);
        return 0;
    }

    // Column doesn't exist, add it
    log_info("Adding column %s to table %s with definition: %s", column_name, table_name, column_def);

    char sql[512];
    snprintf(sql, sizeof(sql), "ALTER TABLE %s ADD COLUMN %s %s;",
            table_name, column_name, column_def);

    rc = sqlite3_exec(db, sql, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to add column %s to table %s: %s (SQL: %s)",
                 column_name, table_name, err_msg, sql);
        if (err_msg) {
            sqlite3_free(err_msg);
            err_msg = NULL;
        }
        return -1;
    }

    // Verify the column was added
    rc = sqlite3_prepare_v2(db, check_column_sql, -1, &check_column_stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement to verify column was added");
        return -1;
    }

    bool column_added = false;
    while (sqlite3_step(check_column_stmt) == SQLITE_ROW) {
        const char *name = (const char *)sqlite3_column_text(check_column_stmt, 1);
        if (name && strcmp(name, column_name) == 0) {
            column_added = true;
            break;
        }
    }

    if (check_column_stmt) {
        sqlite3_finalize(check_column_stmt);
        check_column_stmt = NULL;
    }

    if (!column_added) {
        log_error("Column %s was not added to table %s despite no error", column_name, table_name);
        return -1;
    }

    log_info("Successfully added column %s to table %s", column_name, table_name);
    return 0;
}

/**
 * Legacy migration functions removed
 * All migrations are now handled by SQL files in db/migrations/
 * See db_migrations.c for the new migration system
 */
