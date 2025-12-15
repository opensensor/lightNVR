/**
 * @file db_query_builder.h
 * @brief Dynamic SQL query builder based on available columns
 * 
 * Provides utilities for building SELECT queries dynamically based on
 * which columns exist in the database, eliminating the need for cascading
 * if/else chains when handling schema evolution.
 */

#ifndef DB_QUERY_BUILDER_H
#define DB_QUERY_BUILDER_H

#include <stdbool.h>
#include <stdint.h>
#include <sqlite3.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Maximum number of columns we can track
 */
#define MAX_TRACKED_COLUMNS 64

/**
 * Maximum SQL query length
 */
#define MAX_QUERY_LEN 4096

/**
 * Column information structure
 */
typedef struct {
    char name[64];          // Column name
    int index;              // Index in result set (-1 if not present)
    bool present;           // Whether column exists in table
} column_info_t;

/**
 * Query builder context
 */
typedef struct {
    const char *table_name;
    column_info_t columns[MAX_TRACKED_COLUMNS];
    int column_count;
    char query[MAX_QUERY_LEN];
    int result_column_count;    // Number of columns in the result
} query_builder_t;

/**
 * Initialize a query builder for a table
 * 
 * @param qb Query builder to initialize
 * @param table_name Name of the table
 * @return 0 on success, -1 on error
 */
int qb_init(query_builder_t *qb, const char *table_name);

/**
 * Add a column to track in the query
 * 
 * @param qb Query builder
 * @param column_name Name of the column to add
 * @param is_required If true, query fails if column doesn't exist
 * @return 0 on success, -1 on error
 */
int qb_add_column(query_builder_t *qb, const char *column_name, bool is_required);

/**
 * Build a SELECT query with only the available columns
 * 
 * @param qb Query builder
 * @param where_clause Optional WHERE clause (without "WHERE" keyword), NULL for none
 * @param order_by Optional ORDER BY clause (without "ORDER BY" keyword), NULL for none
 * @return Pointer to the generated query string, or NULL on error
 */
const char *qb_build_select(query_builder_t *qb, const char *where_clause, const char *order_by);

/**
 * Check if a column is present in the result
 * 
 * @param qb Query builder
 * @param column_name Name of the column
 * @return true if column is in the result, false otherwise
 */
bool qb_has_column(const query_builder_t *qb, const char *column_name);

/**
 * Get the result index for a column
 * 
 * @param qb Query builder
 * @param column_name Name of the column
 * @return Index in result set, or -1 if not present
 */
int qb_get_column_index(const query_builder_t *qb, const char *column_name);

/**
 * Helper: Get integer column value with default
 */
int qb_get_int(sqlite3_stmt *stmt, const query_builder_t *qb, 
               const char *column_name, int default_value);

/**
 * Helper: Get text column value with safe copy
 */
const char *qb_get_text(sqlite3_stmt *stmt, const query_builder_t *qb,
                        const char *column_name, char *buffer, size_t buffer_len,
                        const char *default_value);

/**
 * Helper: Get double column value with default
 */
double qb_get_double(sqlite3_stmt *stmt, const query_builder_t *qb,
                     const char *column_name, double default_value);

/**
 * Helper: Get boolean column value with default
 */
bool qb_get_bool(sqlite3_stmt *stmt, const query_builder_t *qb,
                 const char *column_name, bool default_value);

#ifdef __cplusplus
}
#endif

#endif /* DB_QUERY_BUILDER_H */

