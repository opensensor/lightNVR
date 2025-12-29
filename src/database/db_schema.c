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

// Current schema version - increment this when adding new migrations
#define CURRENT_SCHEMA_VERSION 16

// Migration function type
typedef int (*migration_func_t)(void);

// Migration function prototypes
static int migration_v1_to_v2(void);
static int migration_v2_to_v3(void);
static int migration_v3_to_v4(void);
static int migration_v4_to_v5(void);
static int migration_v5_to_v6(void);
static int migration_v6_to_v7(void);
static int migration_v7_to_v8(void);
static int migration_v8_to_v9(void);
static int migration_v9_to_v10(void);
static int migration_v10_to_v11(void);
static int migration_v11_to_v12(void);
static int migration_v12_to_v13(void);
static int migration_v13_to_v14(void);
static int migration_v14_to_v15(void);
static int migration_v15_to_v16(void);

// Array of migration functions
static migration_func_t migrations[] = {
    NULL,               // No migration for v0->v1 (initial schema)
    migration_v1_to_v2, // v1->v2
    migration_v2_to_v3, // v2->v3
    migration_v3_to_v4, // v3->v4
    migration_v4_to_v5, // v4->v5
    migration_v5_to_v6, // v5->v6
    migration_v6_to_v7, // v6->v7
    migration_v7_to_v8, // v7->v8
    migration_v8_to_v9, // v8->v9
    migration_v9_to_v10, // v9->v10
    migration_v10_to_v11, // v10->v11
    migration_v11_to_v12, // v11->v12
    migration_v12_to_v13, // v12->v13 - Recording retention policies
    migration_v13_to_v14, // v13->v14 - PTZ support
    migration_v14_to_v15, // v14->v15 - Buffer strategy
    migration_v15_to_v16  // v15->v16 - ONVIF credentials
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
        if (err_msg) {
            sqlite3_free(err_msg);
            err_msg = NULL;
        }
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

    bool column_exists = false;
    while (sqlite3_step(check_column_stmt) == SQLITE_ROW) {
        const char *name = (const char *)sqlite3_column_text(check_column_stmt, 1);
        if (name && strcmp(name, column_name) == 0) {
            column_exists = true;
            break;
        }
    }

    if (check_column_stmt) {
        sqlite3_finalize(check_column_stmt);
        check_column_stmt = NULL;
    }

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

/**
 * Migration from version 5 to 6
 * - Check for is_deleted column, update streams, and drop the column
 */
static int migration_v5_to_v6(void) {
    log_info("Running migration from v5 to v6: Checking for is_deleted column and simplifying status fields");

    int rc = 0;
    char *err_msg = NULL;
    sqlite3 *db = get_db_handle();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    // Check if is_deleted column exists
    if (column_exists("streams", "is_deleted")) {
        log_info("is_deleted column exists, updating streams and dropping column");

        // Update any deleted streams to not be active
        const char *update_sql = "UPDATE streams SET enabled = 0 WHERE is_deleted = 1;";
        rc = sqlite3_exec(db, update_sql, NULL, NULL, &err_msg);
        if (rc != SQLITE_OK) {
            log_error("Failed to update streams: %s", err_msg);
            if (err_msg) {
                sqlite3_free(err_msg);
                err_msg = NULL;
            }
            return -1;
        }
        log_info("Updated streams with is_deleted=1 to enabled=0");

        // Drop the is_deleted column
        // SQLite doesn't directly support dropping columns, so we need to:
        // 1. Create a new table without the column
        // 2. Copy data from old table to new table
        // 3. Drop old table
        // 4. Rename new table to old table name

        // Create new table without is_deleted column
        const char *create_new_table =
            "CREATE TABLE streams_new ("
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
            "segment_duration INTEGER DEFAULT 900,"
            "detection_based_recording INTEGER DEFAULT 0,"
            "detection_model TEXT DEFAULT '',"
            "detection_threshold REAL DEFAULT 0.5,"
            "detection_interval INTEGER DEFAULT 10,"
            "pre_detection_buffer INTEGER DEFAULT 0,"
            "post_detection_buffer INTEGER DEFAULT 3,"
            "protocol INTEGER DEFAULT 0,"
            "is_onvif INTEGER DEFAULT 0,"
            "record_audio INTEGER DEFAULT 1"
            ");";

        rc = sqlite3_exec(db, create_new_table, NULL, NULL, &err_msg);
        if (rc != SQLITE_OK) {
            log_error("Failed to create new streams table: %s", err_msg);
            if (err_msg) {
                sqlite3_free(err_msg);
                err_msg = NULL;
            }
            return -1;
        }

        // Copy data from old table to new table
        const char *copy_data =
            "INSERT INTO streams_new "
            "SELECT id, name, url, enabled, streaming_enabled, width, height, fps, codec, "
            "priority, record, segment_duration, detection_based_recording, detection_model, "
            "detection_threshold, detection_interval, pre_detection_buffer, post_detection_buffer, "
            "protocol, is_onvif, record_audio "
            "FROM streams;";

        rc = sqlite3_exec(db, copy_data, NULL, NULL, &err_msg);
        if (rc != SQLITE_OK) {
            log_error("Failed to copy data to new streams table: %s", err_msg);
            if (err_msg) {
                sqlite3_free(err_msg);
                err_msg = NULL;
            }
            return -1;
        }

        // Drop old table
        const char *drop_old_table = "DROP TABLE streams;";
        rc = sqlite3_exec(db, drop_old_table, NULL, NULL, &err_msg);
        if (rc != SQLITE_OK) {
            log_error("Failed to drop old streams table: %s", err_msg);
            if (err_msg) {
                sqlite3_free(err_msg);
                err_msg = NULL;
            }
            return -1;
        }

        // Rename new table to old table name
        const char *rename_table = "ALTER TABLE streams_new RENAME TO streams;";
        rc = sqlite3_exec(db, rename_table, NULL, NULL, &err_msg);
        if (rc != SQLITE_OK) {
            log_error("Failed to rename new streams table: %s", err_msg);
            if (err_msg) {
                sqlite3_free(err_msg);
                err_msg = NULL;
            }
            return -1;
        }

        // Recreate indexes
        const char *recreate_indexes =
            "CREATE INDEX IF NOT EXISTS idx_streams_name ON streams (name);";
        rc = sqlite3_exec(db, recreate_indexes, NULL, NULL, &err_msg);
        if (rc != SQLITE_OK) {
            log_error("Failed to recreate indexes on streams table: %s", err_msg);
            if (err_msg) {
                sqlite3_free(err_msg);
                err_msg = NULL;
            }
            return -1;
        }

        log_info("Successfully dropped is_deleted column from streams table");
    } else {
        log_info("is_deleted column does not exist, no changes needed");
    }

    log_info("Completed migration v5 to v6 with result: %d", rc);
    return rc;
}

/**
 * Migration from version 6 to 7
 * - Add motion recording configuration tables
 */
static int migration_v6_to_v7(void) {
    log_info("Running migration from v6 to v7: Adding motion recording configuration tables");

    // Initialize motion recording configuration tables
    int rc = init_motion_config_table();
    if (rc != 0) {
        log_error("Failed to initialize motion recording configuration tables");
        return -1;
    }

    log_info("Completed migration v6 to v7 successfully");
    return 0;
}

/**
 * Migration from version 7 to 8
 * - Add detection zones table
 */
static int migration_v7_to_v8(void) {
    log_info("Running migration from v7 to v8: Adding detection zones table");

    // Initialize detection zones table
    int rc = init_zones_table();
    if (rc != 0) {
        log_error("Failed to initialize detection zones table");
        return -1;
    }

    log_info("Completed migration v7 to v8 successfully");
    return 0;
}

/**
 * Migration from version 8 to 9
 * - Add track_id and zone_id columns to detections table
 */
static int migration_v8_to_v9(void) {
    log_info("Running migration from v8 to v9: Adding track_id and zone_id columns to detections table");

    int rc = 0;

    // Add track_id column to detections table
    log_info("Adding track_id column to detections table");
    rc |= add_column_if_not_exists("detections", "track_id", "INTEGER DEFAULT -1");

    // Add zone_id column to detections table
    log_info("Adding zone_id column to detections table");
    rc |= add_column_if_not_exists("detections", "zone_id", "TEXT DEFAULT ''");

    if (rc != 0) {
        log_error("Failed to add columns to detections table");
        return -1;
    }

    log_info("Completed migration v8 to v9 successfully");
    return 0;
}

/**
 * Migration from version 9 to 10
 * - Add trigger_type column to recordings table
 */
static int migration_v9_to_v10(void) {
    log_info("Running migration from v9 to v10: Adding trigger_type column to recordings table");

    int rc = 0;

    // Add trigger_type column to recordings table
    // Values: 'scheduled', 'detection', 'motion', 'manual'
    log_info("Adding trigger_type column to recordings table");
    rc |= add_column_if_not_exists("recordings", "trigger_type", "TEXT DEFAULT 'scheduled'");

    if (rc != 0) {
        log_error("Failed to add trigger_type column to recordings table");
        return -1;
    }

    log_info("Completed migration v9 to v10 successfully");
    return 0;
}

/**
 * Migration from version 10 to 11
 * - Add detection_api_url column to streams table for per-stream detection API override
 */
static int migration_v10_to_v11(void) {
    log_info("Running migration from v10 to v11: Adding detection_api_url column to streams table");

    int rc = 0;

    // Add detection_api_url column to streams table
    // This allows per-stream override of the global detection API URL
    log_info("Adding detection_api_url column to streams table");
    rc |= add_column_if_not_exists("streams", "detection_api_url", "TEXT DEFAULT ''");

    if (rc != 0) {
        log_error("Failed to add detection_api_url column to streams table");
        return -1;
    }

    log_info("Completed migration v10 to v11 successfully");
    return 0;
}

/**
 * Migration from version 11 to 12
 * - Add backchannel_enabled column to streams table for two-way audio support
 */
static int migration_v11_to_v12(void) {
    log_info("Running migration from v11 to v12: Adding backchannel_enabled column to streams table");

    int rc = 0;

    // Add backchannel_enabled column to streams table
    // This enables two-way audio (backchannel) support for cameras that support it
    log_info("Adding backchannel_enabled column to streams table");
    rc |= add_column_if_not_exists("streams", "backchannel_enabled", "INTEGER DEFAULT 0");

    if (rc != 0) {
        log_error("Failed to add backchannel_enabled column to streams table");
        return -1;
    }

    log_info("Completed migration v11 to v12 successfully");
    return 0;
}

/**
 * Migration from version 12 to 13
 * - Add per-stream retention policy columns to streams table
 * - Add recording protection columns to recordings table
 * - Add indexes for efficient retention policy queries
 */
static int migration_v12_to_v13(void) {
    log_info("Running migration from v12 to v13: Adding recording retention policy support");

    int rc = 0;
    char *err_msg = NULL;
    sqlite3 *db = get_db_handle();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    // Add retention policy columns to streams table
    log_info("Adding retention_days column to streams table");
    rc |= add_column_if_not_exists("streams", "retention_days", "INTEGER DEFAULT 30");

    log_info("Adding detection_retention_days column to streams table");
    rc |= add_column_if_not_exists("streams", "detection_retention_days", "INTEGER DEFAULT 90");

    log_info("Adding max_storage_mb column to streams table");
    rc |= add_column_if_not_exists("streams", "max_storage_mb", "INTEGER DEFAULT 0");

    if (rc != 0) {
        log_error("Failed to add retention columns to streams table");
        return -1;
    }

    // Add recording protection columns to recordings table
    log_info("Adding protected column to recordings table");
    rc |= add_column_if_not_exists("recordings", "protected", "INTEGER DEFAULT 0");

    log_info("Adding retention_override_days column to recordings table");
    rc |= add_column_if_not_exists("recordings", "retention_override_days", "INTEGER DEFAULT NULL");

    if (rc != 0) {
        log_error("Failed to add protection columns to recordings table");
        return -1;
    }

    // Create indexes for efficient queries
    log_info("Creating index on recordings(protected)");
    int idx_rc = sqlite3_exec(db,
        "CREATE INDEX IF NOT EXISTS idx_recordings_protected ON recordings(protected);",
        NULL, NULL, &err_msg);
    if (idx_rc != SQLITE_OK) {
        log_warn("Failed to create idx_recordings_protected: %s", err_msg);
        sqlite3_free(err_msg);
        err_msg = NULL;
        // Non-fatal, continue
    }

    log_info("Creating index on recordings(trigger_type)");
    idx_rc = sqlite3_exec(db,
        "CREATE INDEX IF NOT EXISTS idx_recordings_trigger_type ON recordings(trigger_type);",
        NULL, NULL, &err_msg);
    if (idx_rc != SQLITE_OK) {
        log_warn("Failed to create idx_recordings_trigger_type: %s", err_msg);
        sqlite3_free(err_msg);
        err_msg = NULL;
        // Non-fatal, continue
    }

    log_info("Creating index on recordings(stream_name, start_time)");
    idx_rc = sqlite3_exec(db,
        "CREATE INDEX IF NOT EXISTS idx_recordings_stream_time ON recordings(stream_name, start_time);",
        NULL, NULL, &err_msg);
    if (idx_rc != SQLITE_OK) {
        log_warn("Failed to create idx_recordings_stream_time: %s", err_msg);
        sqlite3_free(err_msg);
        err_msg = NULL;
        // Non-fatal, continue
    }

    log_info("Completed migration v12 to v13 successfully");
    return 0;
}

/**
 * Migration from version 13 to 14
 * - Add PTZ (Pan-Tilt-Zoom) configuration columns to streams table
 */
static int migration_v13_to_v14(void) {
    log_info("Running migration from v13 to v14: Adding PTZ configuration columns to streams table");

    int rc = 0;

    // Add PTZ columns to streams table
    log_info("Adding ptz_enabled column to streams table");
    rc |= add_column_if_not_exists("streams", "ptz_enabled", "INTEGER DEFAULT 0");

    log_info("Adding ptz_max_x column to streams table");
    rc |= add_column_if_not_exists("streams", "ptz_max_x", "INTEGER DEFAULT 0");

    log_info("Adding ptz_max_y column to streams table");
    rc |= add_column_if_not_exists("streams", "ptz_max_y", "INTEGER DEFAULT 0");

    log_info("Adding ptz_max_z column to streams table");
    rc |= add_column_if_not_exists("streams", "ptz_max_z", "INTEGER DEFAULT 0");

    log_info("Adding ptz_has_home column to streams table");
    rc |= add_column_if_not_exists("streams", "ptz_has_home", "INTEGER DEFAULT 0");

    if (rc != 0) {
        log_error("Failed to add PTZ columns to streams table");
        return -1;
    }

    log_info("Completed migration v13 to v14 successfully");
    return 0;
}

/**
 * Migration from version 14 to 15
 * - Add buffer_strategy column to streams table for per-stream pre-detection buffer strategy
 */
static int migration_v14_to_v15(void) {
    log_info("Running migration from v14 to v15: Adding buffer_strategy column to streams table");

    int rc = 0;

    // Add buffer_strategy column to streams table
    // Values: 'auto', 'none', 'go2rtc', 'hls_segment', 'memory_packet', 'mmap_hybrid'
    log_info("Adding buffer_strategy column to streams table");
    rc |= add_column_if_not_exists("streams", "buffer_strategy", "TEXT DEFAULT 'auto'");

    if (rc != 0) {
        log_error("Failed to add buffer_strategy column to streams table");
        return -1;
    }

    log_info("Completed migration v14 to v15 successfully");
    return 0;
}

/**
 * Migration from version 15 to 16
 * - Add ONVIF credentials columns to streams table
 */
static int migration_v15_to_v16(void) {
    log_info("Running migration from v15 to v16: Adding ONVIF credentials columns to streams table");

    int rc = 0;

    // Add onvif_username column to streams table
    log_info("Adding onvif_username column to streams table");
    rc |= add_column_if_not_exists("streams", "onvif_username", "TEXT DEFAULT ''");

    // Add onvif_password column to streams table
    log_info("Adding onvif_password column to streams table");
    rc |= add_column_if_not_exists("streams", "onvif_password", "TEXT DEFAULT ''");

    // Add onvif_profile column to streams table
    log_info("Adding onvif_profile column to streams table");
    rc |= add_column_if_not_exists("streams", "onvif_profile", "TEXT DEFAULT ''");

    if (rc != 0) {
        log_error("Failed to add ONVIF credentials columns to streams table");
        return -1;
    }

    log_info("Completed migration v15 to v16 successfully");
    return 0;
}
