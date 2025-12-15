/**
 * @file sqlite_migrate.c
 * @brief Lightweight SQLite migration library implementation
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dirent.h>
#include <sys/stat.h>
#include <ctype.h>

#include "database/sqlite_migrate.h"
#include "core/logger.h"

/**
 * Internal migration context
 */
struct sqlite_migrate {
    sqlite3 *db;
    migrate_config_t config;
    migration_t *migrations;      // Sorted array of all migrations
    int migration_count;
};

/**
 * Default migrations table name
 */
#define DEFAULT_MIGRATIONS_TABLE "schema_migrations"

/**
 * Create the migrations tracking table if it doesn't exist
 */
static int create_migrations_table(sqlite_migrate_t *ctx) {
    const char *table_name = ctx->config.migrations_table ? 
                             ctx->config.migrations_table : DEFAULT_MIGRATIONS_TABLE;
    
    char sql[512];
    snprintf(sql, sizeof(sql),
        "CREATE TABLE IF NOT EXISTS %s ("
        "  version TEXT PRIMARY KEY,"
        "  applied_at INTEGER NOT NULL DEFAULT (strftime('%%s', 'now'))"
        ");", table_name);
    
    char *err_msg = NULL;
    int rc = sqlite3_exec(ctx->db, sql, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create migrations table: %s", err_msg);
        sqlite3_free(err_msg);
        return -1;
    }
    
    return 0;
}

/**
 * Check if a migration has been applied
 */
static bool is_migration_applied(sqlite_migrate_t *ctx, const char *version) {
    const char *table_name = ctx->config.migrations_table ? 
                             ctx->config.migrations_table : DEFAULT_MIGRATIONS_TABLE;
    
    char sql[256];
    snprintf(sql, sizeof(sql), 
        "SELECT 1 FROM %s WHERE version = ?;", table_name);
    
    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(ctx->db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        return false;
    }
    
    sqlite3_bind_text(stmt, 1, version, -1, SQLITE_STATIC);
    
    bool applied = (sqlite3_step(stmt) == SQLITE_ROW);
    sqlite3_finalize(stmt);
    
    return applied;
}

/**
 * Record a migration as applied
 */
static int record_migration_applied(sqlite_migrate_t *ctx, const char *version) {
    const char *table_name = ctx->config.migrations_table ? 
                             ctx->config.migrations_table : DEFAULT_MIGRATIONS_TABLE;
    
    char sql[256];
    snprintf(sql, sizeof(sql),
        "INSERT INTO %s (version) VALUES (?);", table_name);
    
    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(ctx->db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare migration record: %s", sqlite3_errmsg(ctx->db));
        return -1;
    }
    
    sqlite3_bind_text(stmt, 1, version, -1, SQLITE_STATIC);
    
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    
    if (rc != SQLITE_DONE) {
        log_error("Failed to record migration: %s", sqlite3_errmsg(ctx->db));
        return -1;
    }
    
    return 0;
}

/**
 * Remove a migration record (for rollback)
 */
static int remove_migration_record(sqlite_migrate_t *ctx, const char *version) {
    const char *table_name = ctx->config.migrations_table ? 
                             ctx->config.migrations_table : DEFAULT_MIGRATIONS_TABLE;
    
    char sql[256];
    snprintf(sql, sizeof(sql),
        "DELETE FROM %s WHERE version = ?;", table_name);
    
    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(ctx->db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        return -1;
    }
    
    sqlite3_bind_text(stmt, 1, version, -1, SQLITE_STATIC);
    
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    
    return (rc == SQLITE_DONE) ? 0 : -1;
}

/**
 * Parse version and description from filename
 * Format: YYYYMMDDHHMMSS_description.sql
 */
static int parse_migration_filename(const char *filename, char *version, char *description) {
    // Must end with .sql
    size_t len = strlen(filename);
    if (len < 5 || strcmp(filename + len - 4, ".sql") != 0) {
        return -1;
    }
    
    // Find the underscore separating version from description
    const char *underscore = strchr(filename, '_');
    if (!underscore || underscore == filename) {
        return -1;
    }
    
    // Version is everything before the underscore
    size_t version_len = underscore - filename;
    if (version_len >= MIGRATE_VERSION_LEN) {
        return -1;
    }
    
    // Verify version is all digits
    for (size_t i = 0; i < version_len; i++) {
        if (!isdigit((unsigned char)filename[i])) {
            return -1;
        }
    }
    
    strncpy(version, filename, version_len);
    version[version_len] = '\0';

    // Description is everything after underscore, minus .sql
    const char *desc_start = underscore + 1;
    size_t desc_len = len - 4 - (desc_start - filename);
    if (desc_len >= MIGRATE_DESC_LEN) {
        desc_len = MIGRATE_DESC_LEN - 1;
    }

    strncpy(description, desc_start, desc_len);
    description[desc_len] = '\0';

    // Replace underscores with spaces for readability
    for (char *p = description; *p; p++) {
        if (*p == '_') *p = ' ';
    }

    return 0;
}

/**
 * Compare migrations by version for sorting
 */
static int compare_migrations(const void *a, const void *b) {
    const migration_t *ma = (const migration_t *)a;
    const migration_t *mb = (const migration_t *)b;
    return strcmp(ma->version, mb->version);
}

/**
 * Read SQL file content
 */
static char *read_sql_file(const char *filepath) {
    FILE *fp = fopen(filepath, "r");
    if (!fp) {
        log_error("Failed to open migration file: %s", filepath);
        return NULL;
    }

    // Get file size
    fseek(fp, 0, SEEK_END);
    long size = ftell(fp);
    fseek(fp, 0, SEEK_SET);

    if (size <= 0 || size > MIGRATE_SQL_MAX_LEN) {
        log_error("Migration file too large or empty: %s", filepath);
        fclose(fp);
        return NULL;
    }

    char *content = malloc(size + 1);
    if (!content) {
        fclose(fp);
        return NULL;
    }

    size_t read = fread(content, 1, size, fp);
    fclose(fp);

    content[read] = '\0';
    return content;
}

/**
 * Extract UP or DOWN section from SQL content
 * Returns malloc'd string, caller must free
 */
static char *extract_sql_section(const char *content, const char *marker) {
    char search[64];
    snprintf(search, sizeof(search), "-- migrate:%s", marker);

    const char *start = strstr(content, search);
    if (!start) {
        return NULL;
    }

    // Move past the marker line
    start = strchr(start, '\n');
    if (!start) {
        return NULL;
    }
    start++; // Skip newline

    // Find the end (next marker or end of file)
    const char *end = strstr(start, "-- migrate:");
    if (!end) {
        end = content + strlen(content);
    }

    size_t len = end - start;
    char *sql = malloc(len + 1);
    if (!sql) {
        return NULL;
    }

    memcpy(sql, start, len);
    sql[len] = '\0';

    // Trim trailing whitespace
    while (len > 0 && isspace((unsigned char)sql[len - 1])) {
        sql[--len] = '\0';
    }

    return sql;
}

/**
 * Scan migrations directory for SQL files
 */
static int scan_migrations_dir(sqlite_migrate_t *ctx) {
    if (!ctx->config.migrations_dir) {
        return 0; // No directory configured
    }

    DIR *dir = opendir(ctx->config.migrations_dir);
    if (!dir) {
        log_warn("Migrations directory not found: %s", ctx->config.migrations_dir);
        return 0;
    }

    // Count SQL files first
    int count = 0;
    struct dirent *entry;
    while ((entry = readdir(dir)) != NULL) {
        if (entry->d_type == DT_REG || entry->d_type == DT_UNKNOWN) {
            size_t len = strlen(entry->d_name);
            if (len > 4 && strcmp(entry->d_name + len - 4, ".sql") == 0) {
                count++;
            }
        }
    }

    if (count == 0) {
        closedir(dir);
        return 0;
    }

    // Allocate migrations array
    ctx->migrations = calloc(count, sizeof(migration_t));
    if (!ctx->migrations) {
        closedir(dir);
        return -1;
    }

    // Scan again and populate
    rewinddir(dir);
    int idx = 0;
    while ((entry = readdir(dir)) != NULL && idx < count) {
        if (entry->d_type != DT_REG && entry->d_type != DT_UNKNOWN) {
            continue;
        }

        migration_t *m = &ctx->migrations[idx];

        if (parse_migration_filename(entry->d_name, m->version, m->description) != 0) {
            continue; // Skip invalid filenames
        }

        snprintf(m->filepath, sizeof(m->filepath), "%s/%s",
                 ctx->config.migrations_dir, entry->d_name);

        m->is_embedded = false;
        m->status = is_migration_applied(ctx, m->version) ?
                    MIGRATE_STATUS_APPLIED : MIGRATE_STATUS_PENDING;

        idx++;
    }

    closedir(dir);
    ctx->migration_count = idx;

    // Sort by version
    qsort(ctx->migrations, ctx->migration_count, sizeof(migration_t), compare_migrations);

    return ctx->migration_count;
}

/**
 * Add embedded migrations to the context
 */
static int add_embedded_migrations(sqlite_migrate_t *ctx) {
    if (!ctx->config.embedded_migrations || ctx->config.embedded_count == 0) {
        return 0;
    }

    int existing = ctx->migration_count;
    int new_count = existing + (int)ctx->config.embedded_count;

    migration_t *new_array = realloc(ctx->migrations, new_count * sizeof(migration_t));
    if (!new_array) {
        return -1;
    }
    ctx->migrations = new_array;

    for (size_t i = 0; i < ctx->config.embedded_count; i++) {
        const migration_t *src = &ctx->config.embedded_migrations[i];
        migration_t *dst = &ctx->migrations[existing + i];

        memcpy(dst, src, sizeof(migration_t));
        dst->is_embedded = true;
        dst->status = is_migration_applied(ctx, dst->version) ?
                      MIGRATE_STATUS_APPLIED : MIGRATE_STATUS_PENDING;
    }

    ctx->migration_count = new_count;

    // Re-sort by version
    qsort(ctx->migrations, ctx->migration_count, sizeof(migration_t), compare_migrations);

    return ctx->config.embedded_count;
}

/**
 * Check if a column exists in a table
 */
static bool column_exists(sqlite3 *db, const char *table, const char *column) {
    char sql[256];
    snprintf(sql, sizeof(sql), "PRAGMA table_info(%s);", table);

    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        return false;
    }

    bool exists = false;
    while (sqlite3_step(stmt) == SQLITE_ROW) {
        const char *col_name = (const char *)sqlite3_column_text(stmt, 1);
        if (col_name && strcasecmp(col_name, column) == 0) {
            exists = true;
            break;
        }
    }

    sqlite3_finalize(stmt);
    return exists;
}

/**
 * Check if a table exists in the database
 */
static bool table_exists(sqlite3 *db, const char *table) {
    const char *sql = "SELECT 1 FROM sqlite_master WHERE type='table' AND name=?;";

    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        return false;
    }

    sqlite3_bind_text(stmt, 1, table, -1, SQLITE_STATIC);
    bool exists = (sqlite3_step(stmt) == SQLITE_ROW);
    sqlite3_finalize(stmt);

    return exists;
}

/**
 * Check if an index exists in the database
 */
static bool index_exists(sqlite3 *db, const char *index_name) {
    const char *sql = "SELECT 1 FROM sqlite_master WHERE type='index' AND name=?;";

    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        return false;
    }

    sqlite3_bind_text(stmt, 1, index_name, -1, SQLITE_STATIC);
    bool exists = (sqlite3_step(stmt) == SQLITE_ROW);
    sqlite3_finalize(stmt);

    return exists;
}

/**
 * Execute a single SQL statement with idempotent handling for schema changes.
 * Returns 0 on success, -1 on error, 1 if skipped (already exists).
 */
static int execute_single_statement(sqlite3 *db, const char *sql) {
    // Skip empty statements
    while (*sql && isspace((unsigned char)*sql)) sql++;
    if (*sql == '\0') return 0;

    // Check for ALTER TABLE ... ADD COLUMN and handle idempotently
    if (strncasecmp(sql, "ALTER TABLE", 11) == 0) {
        // Parse: ALTER TABLE tablename ADD COLUMN colname ...
        char table[128] = {0};
        char column[128] = {0};

        const char *p = sql + 11;
        while (*p && isspace((unsigned char)*p)) p++;

        // Extract table name
        int i = 0;
        while (*p && !isspace((unsigned char)*p) && i < 127) {
            table[i++] = *p++;
        }
        table[i] = '\0';

        // Look for ADD COLUMN
        while (*p && isspace((unsigned char)*p)) p++;
        if (strncasecmp(p, "ADD", 3) == 0) {
            p += 3;
            while (*p && isspace((unsigned char)*p)) p++;
            if (strncasecmp(p, "COLUMN", 6) == 0) {
                p += 6;
                while (*p && isspace((unsigned char)*p)) p++;

                // Extract column name
                i = 0;
                while (*p && !isspace((unsigned char)*p) && i < 127) {
                    column[i++] = *p++;
                }
                column[i] = '\0';

                // Check if column already exists
                if (table[0] && column[0] && column_exists(db, table, column)) {
                    log_info("Column %s.%s already exists, skipping", table, column);
                    return 1; // Skipped
                }
            }
        }
    }

    // Check for CREATE TABLE IF NOT EXISTS - let SQLite handle it
    // Check for CREATE INDEX IF NOT EXISTS - let SQLite handle it
    // These are naturally idempotent with IF NOT EXISTS

    // Execute the statement
    char *err_msg = NULL;
    int rc = sqlite3_exec(db, sql, NULL, NULL, &err_msg);

    if (rc != SQLITE_OK) {
        // Check for common idempotent errors that should be ignored
        if (err_msg) {
            // "duplicate column name" - column already exists
            if (strstr(err_msg, "duplicate column name") != NULL) {
                log_info("Column already exists (idempotent): %s", err_msg);
                sqlite3_free(err_msg);
                return 1; // Skipped, but not an error
            }
            // "table already exists" without IF NOT EXISTS
            if (strstr(err_msg, "table") != NULL && strstr(err_msg, "already exists") != NULL) {
                log_info("Table already exists (idempotent): %s", err_msg);
                sqlite3_free(err_msg);
                return 1; // Skipped
            }
            // "index already exists" without IF NOT EXISTS
            if (strstr(err_msg, "index") != NULL && strstr(err_msg, "already exists") != NULL) {
                log_info("Index already exists (idempotent): %s", err_msg);
                sqlite3_free(err_msg);
                return 1; // Skipped
            }
        }

        log_error("SQL execution failed: %s", err_msg);
        log_error("SQL was: %.200s", sql);
        sqlite3_free(err_msg);
        return -1;
    }

    return 0;
}

/**
 * Execute SQL statements (handles multiple statements separated by semicolons)
 * Makes schema changes idempotent by checking for existing columns/tables.
 */
static int execute_sql(sqlite3 *db, const char *sql) {
    // For simple single statements or transactions, use direct exec
    if (strncasecmp(sql, "BEGIN", 5) == 0 ||
        strncasecmp(sql, "COMMIT", 6) == 0 ||
        strncasecmp(sql, "ROLLBACK", 8) == 0) {
        char *err_msg = NULL;
        int rc = sqlite3_exec(db, sql, NULL, NULL, &err_msg);
        if (rc != SQLITE_OK) {
            log_error("SQL execution failed: %s", err_msg);
            sqlite3_free(err_msg);
            return -1;
        }
        return 0;
    }

    // Split SQL into individual statements and execute each one
    // This allows us to handle each ALTER TABLE independently
    const char *start = sql;
    const char *end;
    char *stmt_copy = NULL;
    size_t stmt_size = 0;

    while (*start) {
        // Skip leading whitespace
        while (*start && isspace((unsigned char)*start)) start++;
        if (*start == '\0') break;

        // Skip comments
        if (start[0] == '-' && start[1] == '-') {
            while (*start && *start != '\n') start++;
            continue;
        }

        // Find end of statement (semicolon)
        end = start;
        int in_string = 0;
        while (*end) {
            if (*end == '\'' && !in_string) {
                in_string = 1;
            } else if (*end == '\'' && in_string) {
                in_string = 0;
            } else if (*end == ';' && !in_string) {
                break;
            }
            end++;
        }

        if (end == start) {
            start = end + 1;
            continue;
        }

        // Copy statement for execution
        size_t len = end - start;
        if (len + 2 > stmt_size) {
            stmt_size = len + 256;
            char *new_copy = realloc(stmt_copy, stmt_size);
            if (!new_copy) {
                free(stmt_copy);
                return -1;
            }
            stmt_copy = new_copy;
        }

        memcpy(stmt_copy, start, len);
        stmt_copy[len] = ';';
        stmt_copy[len + 1] = '\0';

        // Execute this statement
        int result = execute_single_statement(db, stmt_copy);
        if (result < 0) {
            free(stmt_copy);
            return -1;
        }

        // Move to next statement
        start = (*end) ? end + 1 : end;
    }

    free(stmt_copy);
    return 0;
}

/**
 * Apply a single migration (UP)
 */
static int apply_migration(sqlite_migrate_t *ctx, migration_t *m) {
    const char *sql_up = m->sql_up;
    char *allocated_sql = NULL;

    // For filesystem migrations, read the file
    if (!m->is_embedded) {
        char *content = read_sql_file(m->filepath);
        if (!content) {
            return -1;
        }

        allocated_sql = extract_sql_section(content, "up");
        free(content);

        if (!allocated_sql) {
            log_error("No 'migrate:up' section found in %s", m->filepath);
            return -1;
        }

        sql_up = allocated_sql;
    }

    if (!sql_up || strlen(sql_up) == 0) {
        log_error("Empty UP migration: %s", m->version);
        if (allocated_sql) free(allocated_sql);
        return -1;
    }

    // Begin transaction
    if (execute_sql(ctx->db, "BEGIN TRANSACTION;") != 0) {
        if (allocated_sql) free(allocated_sql);
        return -1;
    }

    // Execute the migration
    int result = execute_sql(ctx->db, sql_up);

    if (result == 0) {
        // Record the migration
        result = record_migration_applied(ctx, m->version);
    }

    if (result == 0) {
        execute_sql(ctx->db, "COMMIT;");
        m->status = MIGRATE_STATUS_APPLIED;
    } else {
        execute_sql(ctx->db, "ROLLBACK;");
        m->status = MIGRATE_STATUS_FAILED;
    }

    if (allocated_sql) free(allocated_sql);

    return result;
}

/**
 * Rollback a single migration (DOWN)
 */
static int rollback_migration(sqlite_migrate_t *ctx, migration_t *m) {
    const char *sql_down = m->sql_down;
    char *allocated_sql = NULL;

    // For filesystem migrations, read the file
    if (!m->is_embedded) {
        char *content = read_sql_file(m->filepath);
        if (!content) {
            return -1;
        }

        allocated_sql = extract_sql_section(content, "down");
        free(content);

        if (!allocated_sql) {
            log_error("No 'migrate:down' section found in %s", m->filepath);
            return -1;
        }

        sql_down = allocated_sql;
    }

    if (!sql_down || strlen(sql_down) == 0) {
        log_warn("Empty DOWN migration: %s (skipping rollback)", m->version);
        if (allocated_sql) free(allocated_sql);
        return 0; // Not an error, just nothing to do
    }

    // Begin transaction
    if (execute_sql(ctx->db, "BEGIN TRANSACTION;") != 0) {
        if (allocated_sql) free(allocated_sql);
        return -1;
    }

    // Execute the rollback
    int result = execute_sql(ctx->db, sql_down);

    if (result == 0) {
        // Remove the migration record
        result = remove_migration_record(ctx, m->version);
    }

    if (result == 0) {
        execute_sql(ctx->db, "COMMIT;");
        m->status = MIGRATE_STATUS_PENDING;
    } else {
        execute_sql(ctx->db, "ROLLBACK;");
    }

    if (allocated_sql) free(allocated_sql);

    return result;
}

/* ============================================================================
 * Public API Implementation
 * ============================================================================ */

sqlite_migrate_t *migrate_init(sqlite3 *db, const migrate_config_t *config) {
    if (!db) {
        log_error("migrate_init: NULL database handle");
        return NULL;
    }

    sqlite_migrate_t *ctx = calloc(1, sizeof(sqlite_migrate_t));
    if (!ctx) {
        return NULL;
    }

    ctx->db = db;
    if (config) {
        memcpy(&ctx->config, config, sizeof(migrate_config_t));
    }

    // Create migrations table
    if (create_migrations_table(ctx) != 0) {
        free(ctx);
        return NULL;
    }

    // Scan for migrations
    if (scan_migrations_dir(ctx) < 0) {
        free(ctx);
        return NULL;
    }

    // Add embedded migrations
    if (add_embedded_migrations(ctx) < 0) {
        free(ctx->migrations);
        free(ctx);
        return NULL;
    }

    if (ctx->config.verbose) {
        log_info("Found %d migrations", ctx->migration_count);
    }

    return ctx;
}

void migrate_free(sqlite_migrate_t *ctx) {
    if (!ctx) return;

    free(ctx->migrations);
    free(ctx);
}

int migrate_up(sqlite_migrate_t *ctx, migrate_stats_t *stats) {
    if (!ctx) return -1;

    migrate_stats_t local_stats = {0};
    local_stats.total = ctx->migration_count;

    for (int i = 0; i < ctx->migration_count; i++) {
        migration_t *m = &ctx->migrations[i];

        if (m->status == MIGRATE_STATUS_APPLIED) {
            local_stats.applied++;
            continue;
        }

        local_stats.pending++;

        if (ctx->config.verbose) {
            log_info("Applying: %s_%s", m->version, m->description);
        }

        if (ctx->config.dry_run) {
            log_info("[DRY RUN] Would apply: %s_%s", m->version, m->description);
            continue;
        }

        int result = apply_migration(ctx, m);

        if (ctx->config.callback) {
            ctx->config.callback(m->version, m->description, m->status,
                                ctx->config.callback_data);
        }

        if (result != 0) {
            log_error("Failed to apply migration: %s_%s", m->version, m->description);
            local_stats.failed++;

            if (stats) *stats = local_stats;
            return -1;
        }

        if (ctx->config.verbose) {
            log_info("Applied: %s_%s", m->version, m->description);
        }

        local_stats.applied++;
        local_stats.pending--;
    }

    if (stats) *stats = local_stats;

    return 0;
}

int migrate_down(sqlite_migrate_t *ctx) {
    return migrate_down_n(ctx, 1);
}

int migrate_down_n(sqlite_migrate_t *ctx, int count) {
    if (!ctx || count <= 0) return -1;

    int rolled_back = 0;

    // Find applied migrations in reverse order
    for (int i = ctx->migration_count - 1; i >= 0 && rolled_back < count; i--) {
        migration_t *m = &ctx->migrations[i];

        if (m->status != MIGRATE_STATUS_APPLIED) {
            continue;
        }

        if (ctx->config.verbose) {
            log_info("Rolling back: %s_%s", m->version, m->description);
        }

        if (ctx->config.dry_run) {
            log_info("[DRY RUN] Would rollback: %s_%s", m->version, m->description);
            rolled_back++;
            continue;
        }

        int result = rollback_migration(ctx, m);

        if (ctx->config.callback) {
            ctx->config.callback(m->version, m->description, m->status,
                                ctx->config.callback_data);
        }

        if (result != 0) {
            log_error("Failed to rollback migration: %s_%s", m->version, m->description);
            return -1;
        }

        if (ctx->config.verbose) {
            log_info("Rolled back: %s_%s", m->version, m->description);
        }

        rolled_back++;
    }

    return rolled_back;
}

int migrate_status(sqlite_migrate_t *ctx, migration_t *migrations, int max_count) {
    if (!ctx) return -1;

    int count = (ctx->migration_count < max_count) ? ctx->migration_count : max_count;

    for (int i = 0; i < count; i++) {
        memcpy(&migrations[i], &ctx->migrations[i], sizeof(migration_t));
    }

    return count;
}

int migrate_get_version(sqlite_migrate_t *ctx, char *version, size_t version_len) {
    if (!ctx || !version || version_len == 0) return -1;

    version[0] = '\0';

    // Find the most recent applied migration
    for (int i = ctx->migration_count - 1; i >= 0; i--) {
        if (ctx->migrations[i].status == MIGRATE_STATUS_APPLIED) {
            strncpy(version, ctx->migrations[i].version, version_len - 1);
            version[version_len - 1] = '\0';
            return 0;
        }
    }

    return 0; // No migrations applied yet, return empty string
}
