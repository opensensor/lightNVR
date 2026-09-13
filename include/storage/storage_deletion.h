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
/* Process one durable deletion item. Network I/O occurs only in the worker. */
int storage_deletion_process_one(void);
#endif
