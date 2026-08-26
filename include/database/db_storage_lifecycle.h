#ifndef LIGHTNVR_DB_STORAGE_LIFECYCLE_H
#define LIGHTNVR_DB_STORAGE_LIFECYCLE_H

/* Schedule up to max_jobs policy-driven replication or age migration jobs. */
int db_storage_lifecycle_schedule(int max_jobs);

/* Refresh persistent lifecycle violations. Returns active condition count. */
int db_storage_lifecycle_reconcile(void);

#endif
