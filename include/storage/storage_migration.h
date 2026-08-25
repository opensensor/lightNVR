#ifndef LIGHTNVR_STORAGE_MIGRATION_H
#define LIGHTNVR_STORAGE_MIGRATION_H

/* Start/stop the single bounded-concurrency migration worker. */
int storage_migration_worker_start(void);
void storage_migration_worker_shutdown(void);
void storage_migration_worker_wake(void);

/* Process one due job synchronously. Primarily useful for deterministic tests. */
int storage_migration_process_one(void);

#endif /* LIGHTNVR_STORAGE_MIGRATION_H */
