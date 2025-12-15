/**
 * @file db_migrations.c
 * @brief Database migration runner for lightNVR
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/stat.h>

#include "database/db_migrations.h"
#include "database/sqlite_migrate.h"
#include "database/db_core.h"
#include "core/logger.h"

/**
 * Default migrations directory (relative to executable or absolute)
 */
#define DEFAULT_MIGRATIONS_DIR "/usr/share/lightnvr/migrations"
#define LOCAL_MIGRATIONS_DIR "./db/migrations"

/**
 * Mapping from old integer schema versions to migration file versions
 * Used for backward compatibility with existing databases
 */
static const char *LEGACY_VERSION_MAP[] = {
    NULL,      // v0 - no migrations
    "0001",    // v1 - initial_schema
    "0002",    // v2 - add_detection_columns
    "0003",    // v3 - add_protocol_onvif
    "0004",    // v4 - add_record_audio
    "0005",    // v5 - add_detections_table
    "0006",    // v6 - add_users_table
    "0007",    // v7 - add_motion_settings
    "0008",    // v8 - add_zones_table
    "0009",    // v9 - add_detection_tracking
    "0010",    // v10 - add_trigger_type
    "0011",    // v11 - add_detection_api_url
    "0012",    // v12 - add_backchannel
    "0013",    // v13 - add_retention_policy
    "0014",    // v14 - add_ptz_support
    "0015",    // v15 - add_buffer_strategy
};
#define LEGACY_VERSION_COUNT (sizeof(LEGACY_VERSION_MAP) / sizeof(LEGACY_VERSION_MAP[0]))

/**
 * Find the migrations directory
 * Checks multiple locations in order of preference
 */
static const char *find_migrations_dir(void) {
    static char migrations_path[512] = {0};
    
    // Check environment variable first
    const char *env_path = getenv("LIGHTNVR_MIGRATIONS_DIR");
    if (env_path && access(env_path, R_OK) == 0) {
        return env_path;
    }
    
    // Check local development path
    if (access(LOCAL_MIGRATIONS_DIR, R_OK) == 0) {
        return LOCAL_MIGRATIONS_DIR;
    }
    
    // Check installed path
    if (access(DEFAULT_MIGRATIONS_DIR, R_OK) == 0) {
        return DEFAULT_MIGRATIONS_DIR;
    }
    
    // Try relative to executable
    char exe_path[256];
    ssize_t len = readlink("/proc/self/exe", exe_path, sizeof(exe_path) - 1);
    if (len > 0) {
        exe_path[len] = '\0';
        // Find last slash
        char *last_slash = strrchr(exe_path, '/');
        if (last_slash) {
            *last_slash = '\0';
            snprintf(migrations_path, sizeof(migrations_path), 
                     "%s/../share/lightnvr/migrations", exe_path);
            if (access(migrations_path, R_OK) == 0) {
                return migrations_path;
            }
        }
    }
    
    log_warn("No migrations directory found, using embedded migrations only");
    return NULL;
}

/**
 * Migration progress callback
 */
static void migration_callback(const char *version, const char *description,
                               migrate_status_t status, void *user_data) {
    (void)user_data;

    const char *status_str;
    switch (status) {
        case MIGRATE_STATUS_APPLIED:
            status_str = "APPLIED";
            break;
        case MIGRATE_STATUS_FAILED:
            status_str = "FAILED";
            break;
        default:
            status_str = "PENDING";
            break;
    }

    log_info("Migration %s_%s: %s", version, description, status_str);
}

/**
 * Check if we need to migrate from legacy schema_version system
 * Returns the legacy version number, or 0 if not using legacy system
 */
static int get_legacy_schema_version(sqlite3 *db) {
    // Check if schema_version table exists
    sqlite3_stmt *stmt;
    const char *check_sql = "SELECT name FROM sqlite_master WHERE type='table' AND name='schema_version';";

    int rc = sqlite3_prepare_v2(db, check_sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        return 0;
    }

    bool has_legacy_table = (sqlite3_step(stmt) == SQLITE_ROW);
    sqlite3_finalize(stmt);

    if (!has_legacy_table) {
        return 0;
    }

    // Get the version from schema_version table
    const char *version_sql = "SELECT version FROM schema_version WHERE id = 1;";
    rc = sqlite3_prepare_v2(db, version_sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        return 0;
    }

    int version = 0;
    if (sqlite3_step(stmt) == SQLITE_ROW) {
        version = sqlite3_column_int(stmt, 0);
    }
    sqlite3_finalize(stmt);

    return version;
}

/**
 * Check if schema_migrations table already exists
 */
static bool has_new_migrations_table(sqlite3 *db) {
    sqlite3_stmt *stmt;
    const char *sql = "SELECT name FROM sqlite_master WHERE type='table' AND name='schema_migrations';";

    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        return false;
    }

    bool exists = (sqlite3_step(stmt) == SQLITE_ROW);
    sqlite3_finalize(stmt);

    return exists;
}

/**
 * Pre-populate schema_migrations table from legacy schema version
 */
static int migrate_from_legacy_schema(sqlite3 *db, int legacy_version) {
    log_info("Migrating from legacy schema version %d to new migration system", legacy_version);

    // Create schema_migrations table
    const char *create_sql =
        "CREATE TABLE IF NOT EXISTS schema_migrations ("
        "  version TEXT PRIMARY KEY,"
        "  applied_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))"
        ");";

    char *err_msg = NULL;
    int rc = sqlite3_exec(db, create_sql, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create schema_migrations table: %s", err_msg);
        sqlite3_free(err_msg);
        return -1;
    }

    // Insert records for all migrations up to the legacy version
    const char *insert_sql = "INSERT OR IGNORE INTO schema_migrations (version) VALUES (?);";
    sqlite3_stmt *stmt;

    rc = sqlite3_prepare_v2(db, insert_sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare insert statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    int migrated = 0;
    for (int i = 1; i <= legacy_version && i < (int)LEGACY_VERSION_COUNT; i++) {
        if (LEGACY_VERSION_MAP[i] == NULL) {
            continue;
        }

        sqlite3_reset(stmt);
        sqlite3_bind_text(stmt, 1, LEGACY_VERSION_MAP[i], -1, SQLITE_STATIC);

        rc = sqlite3_step(stmt);
        if (rc != SQLITE_DONE) {
            log_warn("Failed to insert migration record for version %d: %s",
                     i, sqlite3_errmsg(db));
        } else {
            migrated++;
        }
    }

    sqlite3_finalize(stmt);

    log_info("Migrated %d legacy schema versions to new migration system", migrated);

    return 0;
}

int run_database_migrations(void) {
    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Cannot run migrations: no database connection");
        return -1;
    }

    // Check for backward compatibility with legacy schema_version system
    if (!has_new_migrations_table(db)) {
        int legacy_version = get_legacy_schema_version(db);
        if (legacy_version > 0) {
            // Migrate from legacy system
            if (migrate_from_legacy_schema(db, legacy_version) != 0) {
                log_error("Failed to migrate from legacy schema system");
                return -1;
            }
        }
    }

    const char *migrations_dir = find_migrations_dir();

    migrate_config_t config = {
        .migrations_dir = migrations_dir,
        .migrations_table = "schema_migrations",
        .embedded_migrations = NULL,
        .embedded_count = 0,
        .callback = migration_callback,
        .callback_data = NULL,
        .dry_run = false,
        .verbose = true
    };

    sqlite_migrate_t *ctx = migrate_init(db, &config);
    if (!ctx) {
        log_error("Failed to initialize migration system");
        return -1;
    }

    migrate_stats_t stats;
    int result = migrate_up(ctx, &stats);

    if (result == 0) {
        log_info("Migrations complete: %d total, %d applied, %d pending",
                 stats.total, stats.applied, stats.pending);
    } else {
        log_error("Migration failed: %d migrations failed", stats.failed);
    }

    migrate_free(ctx);

    return result;
}

int get_database_version(char *version, int version_len) {
    sqlite3 *db = get_db_handle();
    if (!db) {
        return -1;
    }
    
    const char *migrations_dir = find_migrations_dir();
    
    migrate_config_t config = {
        .migrations_dir = migrations_dir,
        .migrations_table = "schema_migrations",
        .verbose = false
    };
    
    sqlite_migrate_t *ctx = migrate_init(db, &config);
    if (!ctx) {
        return -1;
    }
    
    int result = migrate_get_version(ctx, version, version_len);
    migrate_free(ctx);
    
    return result;
}

void print_migration_status(void) {
    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Cannot print migration status: no database connection");
        return;
    }
    
    const char *migrations_dir = find_migrations_dir();
    
    migrate_config_t config = {
        .migrations_dir = migrations_dir,
        .migrations_table = "schema_migrations",
        .verbose = false
    };
    
    sqlite_migrate_t *ctx = migrate_init(db, &config);
    if (!ctx) {
        log_error("Failed to initialize migration system");
        return;
    }

    migration_t migrations[100];
    int count = migrate_status(ctx, migrations, 100);

    if (count < 0) {
        log_error("Failed to get migration status");
        migrate_free(ctx);
        return;
    }

    log_info("=== Migration Status ===");
    for (int i = 0; i < count; i++) {
        const char *status_str;
        switch (migrations[i].status) {
            case MIGRATE_STATUS_APPLIED:
                status_str = "[x]";
                break;
            case MIGRATE_STATUS_FAILED:
                status_str = "[!]";
                break;
            default:
                status_str = "[ ]";
                break;
        }
        log_info("%s %s_%s", status_str, migrations[i].version, migrations[i].description);
    }
    log_info("========================");

    migrate_free(ctx);
}

