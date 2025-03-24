#ifndef LIGHTNVR_DB_SCHEMA_H
#define LIGHTNVR_DB_SCHEMA_H

#include <stdint.h>
#include <stdbool.h>

/**
 * Initialize the database schema management system
 * This should be called during database initialization
 * 
 * @return 0 on success, non-zero on failure
 */
int init_schema_management(void);

/**
 * Run all pending schema migrations
 * This will check the current schema version and run any migrations needed
 * to bring the database up to the latest version
 * 
 * @return 0 on success, non-zero on failure
 */
int run_schema_migrations(void);

/**
 * Get the current schema version
 * 
 * @return Current schema version, or -1 on error
 */
int get_schema_version(void);

/**
 * Check if a column exists in a table
 * 
 * @param table_name Name of the table to check
 * @param column_name Name of the column to check for
 * @return true if the column exists, false otherwise
 */
bool column_exists(const char *table_name, const char *column_name);

/**
 * Add a column to a table if it doesn't exist
 * 
 * @param table_name Name of the table to modify
 * @param column_name Name of the column to add
 * @param column_def Column definition (type, constraints, etc.)
 * @return 0 on success, non-zero on failure
 */
int add_column_if_not_exists(const char *table_name, const char *column_name, const char *column_def);

#endif // LIGHTNVR_DB_SCHEMA_H
