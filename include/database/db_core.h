#ifndef LIGHTNVR_DB_CORE_H
#define LIGHTNVR_DB_CORE_H

#include <sqlite3.h>
#include <pthread.h>

// Include other database module headers
#include "database/db_transaction.h"
#include "database/db_maintenance.h"
#include "database/db_backup.h"

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

/**
 * Checkpoint the database WAL file
 * This ensures all changes are written to the main database file
 * 
 * @return 0 on success, non-zero on failure
 */
int checkpoint_database(void);

#endif // LIGHTNVR_DB_CORE_H
