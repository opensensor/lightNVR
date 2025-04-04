#ifndef LIGHTNVR_DB_SCHEMA_UTILS_H
#define LIGHTNVR_DB_SCHEMA_UTILS_H

#include <sqlite3.h>

/**
 * Safely prepare a statement and track it for automatic finalization
 * 
 * @param sql The SQL query to prepare
 * @param stmt Pointer to the statement handle that will be set
 * @return SQLITE_OK on success, or an error code
 */
int safe_prepare_statement(const char *sql, sqlite3_stmt **stmt);

/**
 * Safely finalize a statement and untrack it
 * 
 * @param stmt The statement to finalize
 */
void safe_finalize_statement(sqlite3_stmt *stmt);

#endif // LIGHTNVR_DB_SCHEMA_UTILS_H
