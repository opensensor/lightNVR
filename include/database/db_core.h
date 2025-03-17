#ifndef LIGHTNVR_DB_CORE_H
#define LIGHTNVR_DB_CORE_H

#include <stdint.h>
#include <sqlite3.h>
#include <pthread.h>

/**
 * Initialize the database
 * 
 * @param db_path Path to the database file
 * @return 0 on success, non-zero on failure
 */
int init_database(const char *db_path);

/**
 * Shutdown the database
 */
void shutdown_database(void);

/**
 * Begin a database transaction
 * 
 * @return 0 on success, non-zero on failure
 */
int begin_transaction(void);

/**
 * Commit a database transaction
 * 
 * @return 0 on success, non-zero on failure
 */
int commit_transaction(void);

/**
 * Rollback a database transaction
 * 
 * @return 0 on success, non-zero on failure
 */
int rollback_transaction(void);

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

/**
 * Get the database handle (for internal use by other database modules)
 * 
 * @return SQLite database handle
 */
sqlite3 *get_db_handle(void);

/**
 * Get the database mutex (for internal use by other database modules)
 * 
 * @return Pointer to the database mutex
 */
pthread_mutex_t *get_db_mutex(void);

#endif // LIGHTNVR_DB_CORE_H
