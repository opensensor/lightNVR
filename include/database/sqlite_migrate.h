/**
 * @file sqlite_migrate.h
 * @brief Lightweight SQLite migration library for embedded systems
 * 
 * A simple, pure C migration library inspired by dbmate.
 * Supports both filesystem SQL files and compile-time embedded migrations.
 * 
 * Migration files use the naming convention: YYYYMMDDHHMMSS_description.sql
 * Example: 20231215120000_create_streams_table.sql
 * 
 * SQL file format:
 *   -- migrate:up
 *   CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);
 *   
 *   -- migrate:down
 *   DROP TABLE IF EXISTS users;
 */

#ifndef SQLITE_MIGRATE_H
#define SQLITE_MIGRATE_H

#include <stdbool.h>
#include <stdint.h>
#include <sqlite3.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Maximum length for migration version string (timestamp)
 */
#define MIGRATE_VERSION_LEN 32

/**
 * Maximum length for migration description
 */
#define MIGRATE_DESC_LEN 256

/**
 * Maximum length for migration file path
 */
#define MIGRATE_PATH_LEN 512

/**
 * Maximum SQL content length (1MB should be plenty)
 */
#define MIGRATE_SQL_MAX_LEN (1024 * 1024)

/**
 * Migration status
 */
typedef enum {
    MIGRATE_STATUS_PENDING,    // Not yet applied
    MIGRATE_STATUS_APPLIED,    // Successfully applied
    MIGRATE_STATUS_FAILED      // Failed to apply
} migrate_status_t;

/**
 * Single migration entry
 */
typedef struct {
    char version[MIGRATE_VERSION_LEN];     // Timestamp version (e.g., "20231215120000")
    char description[MIGRATE_DESC_LEN];    // Human-readable description
    char filepath[MIGRATE_PATH_LEN];       // Path to SQL file (if filesystem-based)
    const char *sql_up;                    // UP migration SQL (for embedded migrations)
    const char *sql_down;                  // DOWN migration SQL (for embedded migrations)
    migrate_status_t status;               // Current status
    bool is_embedded;                      // True if embedded, false if filesystem
} migration_t;

/**
 * Migration context/handle
 */
typedef struct sqlite_migrate sqlite_migrate_t;

/**
 * Migration callback for progress reporting
 * 
 * @param version Migration version being processed
 * @param description Migration description
 * @param status Result status
 * @param user_data User-provided context
 */
typedef void (*migrate_callback_t)(const char *version, const char *description,
                                   migrate_status_t status, void *user_data);

/**
 * Configuration for the migration system
 */
typedef struct {
    const char *migrations_dir;            // Directory containing .sql files (NULL for embedded only)
    const char *migrations_table;          // Table name for tracking (default: "schema_migrations")
    const migration_t *embedded_migrations; // Array of embedded migrations (NULL-terminated)
    size_t embedded_count;                 // Number of embedded migrations
    migrate_callback_t callback;           // Optional progress callback
    void *callback_data;                   // User data for callback
    bool dry_run;                          // If true, don't actually apply migrations
    bool verbose;                          // Enable verbose logging
} migrate_config_t;

/**
 * Migration statistics
 */
typedef struct {
    int total;                             // Total migrations found
    int applied;                           // Already applied
    int pending;                           // Pending to apply
    int failed;                            // Failed during this run
} migrate_stats_t;

/**
 * Initialize the migration system
 * 
 * @param db SQLite database handle
 * @param config Migration configuration
 * @return Migration context, or NULL on error
 */
sqlite_migrate_t *migrate_init(sqlite3 *db, const migrate_config_t *config);

/**
 * Free migration context
 * 
 * @param ctx Migration context
 */
void migrate_free(sqlite_migrate_t *ctx);

/**
 * Run all pending migrations (UP)
 * 
 * @param ctx Migration context
 * @param stats Optional pointer to receive statistics
 * @return 0 on success, -1 on error
 */
int migrate_up(sqlite_migrate_t *ctx, migrate_stats_t *stats);

/**
 * Roll back the most recent migration (DOWN)
 * 
 * @param ctx Migration context
 * @return 0 on success, -1 on error
 */
int migrate_down(sqlite_migrate_t *ctx);

/**
 * Roll back N migrations
 * 
 * @param ctx Migration context
 * @param count Number of migrations to roll back
 * @return Number of migrations rolled back, or -1 on error
 */
int migrate_down_n(sqlite_migrate_t *ctx, int count);

/**
 * Get status of all migrations
 * 
 * @param ctx Migration context
 * @param migrations Output array (caller allocates)
 * @param max_count Maximum entries to return
 * @return Number of migrations found, or -1 on error
 */
int migrate_status(sqlite_migrate_t *ctx, migration_t *migrations, int max_count);

/**
 * Get current schema version (most recent applied migration)
 * 
 * @param ctx Migration context
 * @param version Output buffer for version string
 * @param version_len Size of version buffer
 * @return 0 on success, -1 on error
 */
int migrate_get_version(sqlite_migrate_t *ctx, char *version, size_t version_len);

#ifdef __cplusplus
}
#endif

#endif /* SQLITE_MIGRATE_H */

