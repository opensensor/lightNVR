#ifndef LIGHTNVR_DB_SCHEMA_CACHE_H
#define LIGHTNVR_DB_SCHEMA_CACHE_H

#include <stdbool.h>

/**
 * Initialize the schema cache
 * This should be called during server startup
 */
void init_schema_cache(void);

/**
 * Check if a column exists in a table, using the cache if available
 * This is a drop-in replacement for column_exists() that uses caching
 * to avoid repeated database queries
 * 
 * @param table_name Name of the table to check
 * @param column_name Name of the column to check for
 * @return true if the column exists, false otherwise
 */
bool cached_column_exists(const char *table_name, const char *column_name);

/**
 * Free the schema cache
 * This should be called during server shutdown
 */
void free_schema_cache(void);

#endif // LIGHTNVR_DB_SCHEMA_CACHE_H
