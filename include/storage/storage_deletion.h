#ifndef LIGHTNVR_STORAGE_DELETION_H
#define LIGHTNVR_STORAGE_DELETION_H
#include <stdint.h>

/* Atomically claim logical deletion and journal every physical copy. Protected
 * recordings and actively transferring recordings return -2 without mutation. */
int storage_recording_delete(uint64_t recording_id, const char *reason,
                             uint64_t *local_bytes_removed);
/* Recheck the applied policy/override inside the deletion transaction. Returns
 * -2 if a previously selected recording is no longer eligible for expiry. */
int storage_recording_expire_policy(uint64_t recording_id);
/* Shared by legacy candidate selection and the transactional eligibility check.
 * An applied policy or per-recording override takes precedence over global age. */
#define STORAGE_LEGACY_EXPIRY_PREDICATE \
    "r.end_time<?1 AND r.is_complete=1 AND COALESCE(r.retention_override_days,-1)<>0 AND " \
    "(COALESCE(p.minimum_retention_days,0)=0 OR r.start_time<strftime('%s','now')-p.minimum_retention_days*86400) AND " \
    "((r.retention_override_days>0 AND r.start_time<strftime('%s','now')-r.retention_override_days*86400) OR " \
    "(COALESCE(r.retention_override_days,-1)<0 AND (p.recording_id IS NULL OR " \
    "(p.maximum_retention_days>0 AND r.start_time<strftime('%s','now')-p.maximum_retention_days*86400))))"
int storage_recording_expire_age(uint64_t recording_id, int64_t cutoff);
/* Process one durable deletion item. Network I/O occurs only in the worker. */
int storage_deletion_process_one(void);
#endif
