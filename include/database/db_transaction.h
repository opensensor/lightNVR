#ifndef LIGHTNVR_DB_TRANSACTION_H
#define LIGHTNVR_DB_TRANSACTION_H

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

#endif // LIGHTNVR_DB_TRANSACTION_H
