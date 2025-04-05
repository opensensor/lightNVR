#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <pthread.h>
#include <sqlite3.h>

#include "database/db_schema.h"
#include "database/db_schema_utils.h"
#include "database/db_core.h"
#include "core/logger.h"

// Cache for column existence to avoid repeated database queries
typedef struct {
    char table_name[64];
    char column_name[64];
    bool exists;
} column_cache_entry_t;

// Cache for column existence
static column_cache_entry_t *column_cache = NULL;
static int column_cache_size = 0;
static int column_cache_capacity = 0;
static pthread_mutex_t column_cache_mutex = PTHREAD_MUTEX_INITIALIZER;
static bool schema_initialized = false;

// Forward declaration of the cached_column_exists function
bool cached_column_exists(const char *table_name, const char *column_name);

/**
 * Initialize the schema cache
 * This should be called during server startup
 */
void init_schema_cache(void) {
    pthread_mutex_lock(&column_cache_mutex);

    if (!schema_initialized) {
        // Initialize the column cache
        column_cache_capacity = 32;  // Start with space for 32 entries
        column_cache = malloc(column_cache_capacity * sizeof(column_cache_entry_t));
        if (!column_cache) {
            log_error("Failed to allocate memory for column cache");
            pthread_mutex_unlock(&column_cache_mutex);
            return;
        }

        column_cache_size = 0;

        // Run schema migrations once at startup
        if (init_schema_management() != 0) {
            log_error("Failed to initialize schema management");
            free(column_cache);
            column_cache = NULL;
            pthread_mutex_unlock(&column_cache_mutex);
            return;
        }

        if (run_schema_migrations() != 0) {
            log_error("Failed to run schema migrations");
            free(column_cache);
            column_cache = NULL;
            pthread_mutex_unlock(&column_cache_mutex);
            return;
        }

        // Pre-cache common column checks directly using column_exists
        // These are the columns that are frequently checked in the codebase
        bool detection_exists = column_exists("streams", "detection_based_recording");
        bool protocol_exists = column_exists("streams", "protocol");
        bool onvif_exists = column_exists("streams", "is_onvif");
        bool record_audio_exists = column_exists("streams", "record_audio");
        // is_deleted column has been removed in migration_v5_to_v6

        // Add them to the cache manually
        if (column_cache_size < column_cache_capacity) {
            strncpy(column_cache[column_cache_size].table_name, "streams", sizeof(column_cache[column_cache_size].table_name) - 1);
            strncpy(column_cache[column_cache_size].column_name, "detection_based_recording", sizeof(column_cache[column_cache_size].column_name) - 1);
            column_cache[column_cache_size].exists = detection_exists;
            column_cache_size++;
        }

        if (column_cache_size < column_cache_capacity) {
            strncpy(column_cache[column_cache_size].table_name, "streams", sizeof(column_cache[column_cache_size].table_name) - 1);
            strncpy(column_cache[column_cache_size].column_name, "protocol", sizeof(column_cache[column_cache_size].column_name) - 1);
            column_cache[column_cache_size].exists = protocol_exists;
            column_cache_size++;
        }

        if (column_cache_size < column_cache_capacity) {
            strncpy(column_cache[column_cache_size].table_name, "streams", sizeof(column_cache[column_cache_size].table_name) - 1);
            strncpy(column_cache[column_cache_size].column_name, "is_onvif", sizeof(column_cache[column_cache_size].column_name) - 1);
            column_cache[column_cache_size].exists = onvif_exists;
            column_cache_size++;
        }

        // Add record_audio column to cache
        if (column_cache_size < column_cache_capacity) {
            strncpy(column_cache[column_cache_size].table_name, "streams", sizeof(column_cache[column_cache_size].table_name) - 1);
            strncpy(column_cache[column_cache_size].column_name, "record_audio", sizeof(column_cache[column_cache_size].column_name) - 1);
            column_cache[column_cache_size].exists = record_audio_exists;
            column_cache_size++;
        }

        // is_deleted column has been removed in migration_v5_to_v6

        schema_initialized = true;
        log_info("Schema cache initialized with %d entries", column_cache_size);
    }

    pthread_mutex_unlock(&column_cache_mutex);
}

/**
 * Check if a column exists in a table, using the cache if available
 *
 * @param table_name Name of the table to check
 * @param column_name Name of the column to check for
 * @return true if the column exists, false otherwise
 */
bool cached_column_exists(const char *table_name, const char *column_name) {
    if (!table_name || !column_name) {
        return false;
    }

    // Check if we need to initialize the schema cache
    // IMPORTANT: We don't call init_schema_cache() here to avoid recursive deadlock
    // The schema cache should be initialized at server startup
    if (!schema_initialized) {
        // If not initialized, fall back to direct column check
        return column_exists(table_name, column_name);
    }

    pthread_mutex_lock(&column_cache_mutex);

    // Check if the column is already in the cache
    for (int i = 0; i < column_cache_size; i++) {
        if (strcmp(column_cache[i].table_name, table_name) == 0 &&
            strcmp(column_cache[i].column_name, column_name) == 0) {
            // Found in cache
            bool exists = column_cache[i].exists;
            pthread_mutex_unlock(&column_cache_mutex);
            return exists;
        }
    }

    // Not in cache, check the database
    bool exists = column_exists(table_name, column_name);

    // Add to cache
    if (column_cache_size >= column_cache_capacity) {
        // Resize the cache
        int new_capacity = column_cache_capacity * 2;
        column_cache_entry_t *new_cache = realloc(column_cache,
                                                 new_capacity * sizeof(column_cache_entry_t));
        if (!new_cache) {
            log_error("Failed to resize column cache");
            pthread_mutex_unlock(&column_cache_mutex);
            return exists;
        }

        column_cache = new_cache;
        column_cache_capacity = new_capacity;
    }

    // Add the new entry
    strncpy(column_cache[column_cache_size].table_name, table_name, sizeof(column_cache[column_cache_size].table_name) - 1);
    column_cache[column_cache_size].table_name[sizeof(column_cache[column_cache_size].table_name) - 1] = '\0';

    strncpy(column_cache[column_cache_size].column_name, column_name, sizeof(column_cache[column_cache_size].column_name) - 1);
    column_cache[column_cache_size].column_name[sizeof(column_cache[column_cache_size].column_name) - 1] = '\0';

    column_cache[column_cache_size].exists = exists;
    column_cache_size++;

    pthread_mutex_unlock(&column_cache_mutex);

    return exists;
}

/**
 * Free the schema cache
 * This should be called during server shutdown
 */
void free_schema_cache(void) {
    pthread_mutex_lock(&column_cache_mutex);

    if (column_cache) {
        free(column_cache);
        column_cache = NULL;
        column_cache_size = 0;
        column_cache_capacity = 0;
    }

    // Reset all cached schema information
    sqlite3 *db = NULL;

    // Try to get the database handle safely
    pthread_mutex_t *db_mutex = get_db_mutex();
    if (db_mutex) {
        pthread_mutex_lock(db_mutex);
        db = get_db_handle();

        if (db) {
            // Release any cached schema information
            sqlite3_exec(db, "PRAGMA schema_version;", NULL, NULL, NULL);

            // Finalize any prepared statements related to schema queries
            sqlite3_stmt *stmt;
            while ((stmt = sqlite3_next_stmt(db, NULL)) != NULL) {
                const char *sql = sqlite3_sql(stmt);
                if (sql && (strstr(sql, "PRAGMA") || strstr(sql, "sqlite_master"))) {
                    log_info("Finalizing schema-related statement: %s", sql);
                    sqlite3_finalize(stmt);
                }
            }
        } else {
            log_warn("Database handle not available during schema cache cleanup");
        }

        pthread_mutex_unlock(db_mutex);
    } else {
        log_warn("Database mutex not available during schema cache cleanup");
    }

    schema_initialized = false;

    pthread_mutex_unlock(&column_cache_mutex);
}
