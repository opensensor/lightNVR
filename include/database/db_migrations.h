/**
 * @file db_migrations.h
 * @brief Database migration runner for lightNVR
 * 
 * Integrates the sqlite_migrate library with lightNVR's database system.
 * Runs migrations from db/migrations/ directory at startup.
 */

#ifndef DB_MIGRATIONS_H
#define DB_MIGRATIONS_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Run all pending database migrations
 * 
 * This should be called early in the application startup,
 * after the database connection is established but before
 * any other database operations.
 * 
 * @return 0 on success, -1 on error
 */
int run_database_migrations(void);

/**
 * Get the current database schema version
 * 
 * @param version Buffer to receive version string
 * @param version_len Size of version buffer
 * @return 0 on success, -1 on error
 */
int get_database_version(char *version, int version_len);

/**
 * Print migration status to log
 * 
 * Lists all migrations and their applied/pending status.
 */
void print_migration_status(void);

#ifdef __cplusplus
}
#endif

#endif /* DB_MIGRATIONS_H */

