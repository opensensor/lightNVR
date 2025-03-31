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
#include "core/logger.h"

// Current schema version - increment this when adding new migrations
#define CURRENT_SCHEMA_VERSION 5

// Migration function type
typedef int (*migration_func_t)(void);

// Migration function prototypes
static int migration_v1_to_v2(void);
static int migration_v2_to_v3(void);
static int migration_v3_to_v4(void);
static int migration_v4_to_v5(void);

// Array of migration functions
static migration_func_t migrations[] = {
    NULL,               // No migration for v0->v1 (initial schema)
    migration_v1_to_v2, // v1->v2
    migration_v2_to_v3, // v2->v3
    migration_v3_to_v4, // v3->v4
    migration_v4_to_v5  // v4->v5
};

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
        sqlite3_free(err_msg);
        return -1;
    }
    
    // Check if we need to initialize the version
    sqlite3_stmt *stmt;
    rc = sqlite3_prepare_v2(db, "SELECT COUNT(*) FROM schema_version;", -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
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
                sqlite3_free(err_msg);
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
    sqlite3_stmt *stmt;
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
        return -1;
    }
    
    if (sqlite3_step(stmt) == SQLITE_ROW) {
        version = sqlite3_column_int(stmt, 0);
    }
    
    sqlite3_finalize(stmt);
    
    return version;
}

/**
 * Update the schema version
 */
static int update_schema_version(int new_version) {
    int rc;
    char *err_msg = NULL;
    
    sqlite3 *db = get_db_handle();
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    char sql[256];
    snprintf(sql, sizeof(sql), 
            "UPDATE schema_version SET version = %d, updated_at = strftime('%%s','now') WHERE id = 1;",
            new_version);
    
    rc = sqlite3_exec(db, sql, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to update schema version: %s", err_msg);
        sqlite3_free(err_msg);
        return -1;
    }
    
    log_info("Updated database schema to version %d", new_version);
    return 0;
}

/**
 * Run all pending schema migrations
 */
int run_schema_migrations(void) {
    int current_version = get_schema_version();
    
    if (current_version < 0) {
        log_error("Failed to get current schema version");
        return -1;
    }
    
    log_info("Current database schema version: %d", current_version);
    
    if (current_version >= CURRENT_SCHEMA_VERSION) {
        log_info("Database schema is up to date (version %d)", current_version);
        return 0;
    }
    
    log_info("Running schema migrations from version %d to %d", 
            current_version, CURRENT_SCHEMA_VERSION);
    
    // Run each migration in sequence
    for (int version = current_version; version < CURRENT_SCHEMA_VERSION; version++) {
        log_info("Running migration from version %d to %d", version, version + 1);
        
        if (migrations[version] == NULL) {
            log_error("No migration function defined for version %d to %d", 
                     version, version + 1);
            return -1;
        }
        
        // Begin transaction for this migration
        if (begin_transaction() != 0) {
            log_error("Failed to begin transaction for migration");
            return -1;
        }
        
        // Run the migration
        int rc = migrations[version]();
        if (rc != 0) {
            log_error("Migration from version %d to %d failed", version, version + 1);
            rollback_transaction();
            return -1;
        }
        
        // Update the schema version
        rc = update_schema_version(version + 1);
        if (rc != 0) {
            log_error("Failed to update schema version after migration");
            rollback_transaction();
            return -1;
        }
        
        // Commit the transaction
        if (commit_transaction() != 0) {
            log_error("Failed to commit transaction for migration");
            return -1;
        }
        
        log_info("Successfully migrated from version %d to %d", version, version + 1);
    }
    
    log_info("All migrations completed successfully. Current schema version: %d", 
            CURRENT_SCHEMA_VERSION);
    
    return 0;
}

/**
 * Check if a column exists in a table
 */
bool column_exists(const char *table_name, const char *column_name) {
    int rc;
    sqlite3_stmt *stmt;
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
    
    sqlite3_stmt *check_table_stmt;
    rc = sqlite3_prepare_v2(db, check_table_sql, -1, &check_table_stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement to check table existence: %s", sqlite3_errmsg(db));
        return false;
    }
    
    bool table_exists = false;
    if (sqlite3_step(check_table_stmt) == SQLITE_ROW) {
        table_exists = true;
    }
    sqlite3_finalize(check_table_stmt);
    
    if (!table_exists) {
        log_error("Table %s does not exist", table_name);
        return false;
    }
    
    // Now check if the column exists
    char sql[256];
    snprintf(sql, sizeof(sql), "PRAGMA table_info(%s);", table_name);
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement to check column existence: %s", sqlite3_errmsg(db));
        return false;
    }
    
    while (sqlite3_step(stmt) == SQLITE_ROW) {
        const char *name = (const char *)sqlite3_column_text(stmt, 1);
        if (name && strcmp(name, column_name) == 0) {
            exists = true;
            break;
        }
    }
    
    sqlite3_finalize(stmt);
    
    log_debug("Column %s %s in table %s", column_name, exists ? "exists" : "does not exist", table_name);
    return exists;
}

/**
 * Add a column to a table if it doesn't exist
 */
int add_column_if_not_exists(const char *table_name, const char *column_name, const char *column_def) {
    int rc;
    char *err_msg = NULL;
    
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
    
    sqlite3_stmt *check_table_stmt;
    rc = sqlite3_prepare_v2(db, check_table_sql, -1, &check_table_stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement to check table existence: %s", sqlite3_errmsg(db));
        return -1;
    }
    
    bool table_exists = false;
    if (sqlite3_step(check_table_stmt) == SQLITE_ROW) {
        table_exists = true;
    }
    sqlite3_finalize(check_table_stmt);
    
    if (!table_exists) {
        log_error("Table %s does not exist", table_name);
        return -1;
    }
    
    // Check if column already exists (directly, not using column_exists function to avoid confusion)
    char check_column_sql[256];
    snprintf(check_column_sql, sizeof(check_column_sql), "PRAGMA table_info(%s);", table_name);
    
    sqlite3_stmt *check_column_stmt;
    rc = sqlite3_prepare_v2(db, check_column_sql, -1, &check_column_stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement to check column existence: %s", sqlite3_errmsg(db));
        return -1;
    }
    
    bool column_exists = false;
    while (sqlite3_step(check_column_stmt) == SQLITE_ROW) {
        const char *name = (const char *)sqlite3_column_text(check_column_stmt, 1);
        if (name && strcmp(name, column_name) == 0) {
            column_exists = true;
            break;
        }
    }
    sqlite3_finalize(check_column_stmt);
    
    if (column_exists) {
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
        sqlite3_free(err_msg);
        return -1;
    }
    
    // Verify the column was added
    rc = sqlite3_prepare_v2(db, check_column_sql, -1, &check_column_stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement to verify column was added: %s", sqlite3_errmsg(db));
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
    sqlite3_finalize(check_column_stmt);
    
    if (!column_added) {
        log_error("Column %s was not added to table %s despite no error", column_name, table_name);
        return -1;
    }
    
    log_info("Successfully added column %s to table %s", column_name, table_name);
    return 0;
}

/**
 * Migration from version 1 to 2
 * - Add detection columns to streams table
 */
static int migration_v1_to_v2(void) {
    log_info("Running migration from v1 to v2: Adding detection columns to streams table");
    
    int rc = 0;
    
    // Add detection columns to streams table - add detailed logging
    log_info("Adding detection_based_recording column");
    rc |= add_column_if_not_exists("streams", "detection_based_recording", "INTEGER DEFAULT 0");
    
    log_info("Adding detection_model column");
    rc |= add_column_if_not_exists("streams", "detection_model", "TEXT DEFAULT ''");
    
    log_info("Adding detection_threshold column");
    rc |= add_column_if_not_exists("streams", "detection_threshold", "REAL DEFAULT 0.5");
    
    log_info("Adding detection_interval column");
    rc |= add_column_if_not_exists("streams", "detection_interval", "INTEGER DEFAULT 10");
    
    log_info("Adding pre_detection_buffer column");
    rc |= add_column_if_not_exists("streams", "pre_detection_buffer", "INTEGER DEFAULT 0");
    
    log_info("Adding post_detection_buffer column");
    rc |= add_column_if_not_exists("streams", "post_detection_buffer", "INTEGER DEFAULT 3");
    
    log_info("Completed migration v1 to v2 with result: %d", rc);
    return rc;
}

/**
 * Migration from version 2 to 3
 * - Add protocol and is_onvif columns to streams table
 */
static int migration_v2_to_v3(void) {
    log_info("Running migration from v2 to v3: Adding protocol and is_onvif columns to streams table");
    
    int rc = 0;
    
    // Add protocol and is_onvif columns to streams table
    log_info("Adding protocol column");
    rc |= add_column_if_not_exists("streams", "protocol", "INTEGER DEFAULT 0");
    
    log_info("Adding is_onvif column");
    rc |= add_column_if_not_exists("streams", "is_onvif", "INTEGER DEFAULT 0");
    
    log_info("Completed migration v2 to v3 with result: %d", rc);
    return rc;
}

/**
 * Migration from version 3 to 4
 * - Add record_audio column to streams table
 */
static int migration_v3_to_v4(void) {
    log_info("Running migration from v3 to v4: Adding record_audio column to streams table");
    
    int rc = 0;
    
    // Add record_audio column to streams table
    log_info("Adding record_audio column");
    rc |= add_column_if_not_exists("streams", "record_audio", "INTEGER DEFAULT 1");
    
    log_info("Completed migration v3 to v4 with result: %d", rc);
    return rc;
}

/**
 * Migration from version 4 to 5
 * - Add users table for authentication
 */
static int migration_v4_to_v5(void) {
    log_info("Running migration from v4 to v5: Adding users table for authentication");
    
    int rc;
    char *err_msg = NULL;
    
    sqlite3 *db = get_db_handle();
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    // Create users table
    const char *create_users_table = 
        "CREATE TABLE IF NOT EXISTS users ("
        "id INTEGER PRIMARY KEY AUTOINCREMENT,"
        "username TEXT NOT NULL UNIQUE,"
        "password_hash TEXT NOT NULL,"  // Store hashed passwords, not plaintext
        "salt TEXT NOT NULL,"           // Salt for password hashing
        "role TEXT NOT NULL,"           // Role: admin, user, etc.
        "email TEXT,"                   // Optional email for password reset
        "created_at INTEGER NOT NULL,"  // Unix timestamp
        "updated_at INTEGER NOT NULL,"  // Unix timestamp
        "last_login INTEGER,"           // Unix timestamp of last login
        "is_active INTEGER DEFAULT 1,"  // Whether the user is active
        "api_key TEXT"                  // API key for programmatic access
        ");";
    
    rc = sqlite3_exec(db, create_users_table, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create users table: %s", err_msg);
        sqlite3_free(err_msg);
        return -1;
    }
    
    // Create indexes for faster lookups
    const char *create_username_index = 
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_users_username ON users (username);";
    
    rc = sqlite3_exec(db, create_username_index, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create username index: %s", err_msg);
        sqlite3_free(err_msg);
        return -1;
    }
    
    const char *create_api_key_index = 
        "CREATE INDEX IF NOT EXISTS idx_users_api_key ON users (api_key);";
    
    rc = sqlite3_exec(db, create_api_key_index, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create api_key index: %s", err_msg);
        sqlite3_free(err_msg);
        return -1;
    }
    
    // Create sessions table for managing user sessions
    const char *create_sessions_table = 
        "CREATE TABLE IF NOT EXISTS sessions ("
        "id INTEGER PRIMARY KEY AUTOINCREMENT,"
        "user_id INTEGER NOT NULL,"
        "token TEXT NOT NULL UNIQUE,"   // Session token
        "created_at INTEGER NOT NULL,"  // Unix timestamp
        "expires_at INTEGER NOT NULL,"  // Unix timestamp
        "ip_address TEXT,"              // IP address of the client
        "user_agent TEXT,"              // User agent of the client
        "FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE"
        ");";
    
    rc = sqlite3_exec(db, create_sessions_table, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create sessions table: %s", err_msg);
        sqlite3_free(err_msg);
        return -1;
    }
    
    // Create indexes for faster lookups
    const char *create_token_index = 
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_sessions_token ON sessions (token);";
    
    rc = sqlite3_exec(db, create_token_index, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create token index: %s", err_msg);
        sqlite3_free(err_msg);
        return -1;
    }
    
    const char *create_user_id_index = 
        "CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON sessions (user_id);";
    
    rc = sqlite3_exec(db, create_user_id_index, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create user_id index: %s", err_msg);
        sqlite3_free(err_msg);
        return -1;
    }
    
    log_info("Completed migration v4 to v5 with result: %d", rc);
    return 0;
}
