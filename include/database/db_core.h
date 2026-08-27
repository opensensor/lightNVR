#ifndef LIGHTNVR_DB_CORE_H
#define LIGHTNVR_DB_CORE_H

#include <sqlite3.h>
#include <pthread.h>

// Include other database module headers
#include "database/db_transaction.h"
#include "database/db_maintenance.h"
#include "database/db_backup.h"

/**
 * Flags controlling how much work init_database_ex() does at startup.
 *
 * The default path is what a long-running server needs.  Short-lived one-shot
 * invocations (e.g. --generate-go2rtc-config) only read a few rows and must not
 * pay for a consistency sweep, a schema migration pass, or a full-size backup on
 * the way out — on a multi-GB database each of those costs minutes during which
 * nothing is listening on the HTTP port.
 */
typedef enum {
    DB_INIT_DEFAULT    = 0,
    DB_INIT_NO_CHECK   = 1 << 0,  // Skip the startup consistency check
    DB_INIT_NO_MIGRATE = 1 << 1,  // Skip schema migrations (read-only callers)
    DB_INIT_NO_BACKUP  = 1 << 2,  // Skip the backup shutdown_database() takes
    /** Read-only consumer: read some rows, exit. Cheapest possible open. */
    DB_INIT_READ_ONLY  = DB_INIT_NO_CHECK | DB_INIT_NO_MIGRATE | DB_INIT_NO_BACKUP,
} db_init_flags_t;

/**
 * Initialize the database with explicit control over startup work
 *
 * @param db_path Path to the database file
 * @param flags Bitwise OR of db_init_flags_t
 * @return 0 on success, non-zero on failure
 */
int init_database_ex(const char *db_path, unsigned flags);

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
 * Open an independent read-only connection to the active database.
 * Long analytics reads use this with WAL snapshots instead of monopolizing
 * the application's writer mutex.
 */
int db_open_readonly_connection(sqlite3 **connection);

/** Close a connection returned by db_open_readonly_connection(). */
void db_close_readonly_connection(sqlite3 *connection);

/**
 * Checkpoint the database WAL file
 * This ensures all changes are written to the main database file
 * 
 * @return 0 on success, non-zero on failure
 */
int checkpoint_database(void);

/**
 * Run periodic database backup work when the configured interval is due.
 *
 * @return 0 on success or no-op, non-zero on failure
 */
int maybe_run_scheduled_database_backup(void);

#endif // LIGHTNVR_DB_CORE_H
