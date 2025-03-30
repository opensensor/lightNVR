#ifndef LIGHTNVR_DB_MAINTENANCE_H
#define LIGHTNVR_DB_MAINTENANCE_H

#include <stdint.h>

/**
 * Get the database size
 * 
 * @return Database size in bytes, or -1 on error
 */
int64_t get_database_size(void);

/**
 * Vacuum the database to reclaim space
 * 
 * @return 0 on success, non-zero on failure
 */
int vacuum_database(void);

/**
 * Check database integrity
 * 
 * @return 0 if database is valid, non-zero otherwise
 */
int check_database_integrity(void);

/**
 * Execute a SQL query and get the results
 * 
 * @param sql SQL query to execute
 * @param result Pointer to store the result set
 * @param rows Pointer to store the number of rows
 * @param cols Pointer to store the number of columns
 * @return 0 on success, non-zero on failure
 */
int database_execute_query(const char *sql, void **result, int *rows, int *cols);

#endif // LIGHTNVR_DB_MAINTENANCE_H
