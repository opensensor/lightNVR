#ifndef LIGHTNVR_STORAGE_SOURCE_H
#define LIGHTNVR_STORAGE_SOURCE_H
#include <stdint.h>
#include "core/config.h"
#include "database/db_storage_targets.h"

enum { STORAGE_SOURCE_READY = 0, STORAGE_SOURCE_PREPARING = 1,
       STORAGE_SOURCE_ERROR = -1, STORAGE_SOURCE_MISSING = -2,
       STORAGE_SOURCE_DELETING = -3 };
/* Call only after recording/camera authorization. Resolves healthy local copies
 * first; remote media is prepared by the bounded retrieval worker. */
typedef struct {
    uint64_t recording_id;
    storage_target_t target;
    char key[MAX_PATH_LENGTH];
    uint64_t size;
} storage_remote_source_t;
/* Inspect usable replicas without staging bytes or creating read leases. */
bool storage_source_available(uint64_t id);
/* Describe the selected verified remote replica without staging its bytes. */
int storage_source_remote(uint64_t id, storage_remote_source_t *source);
bool storage_source_touch(uint64_t id);
/* Only for an existing active reader; permits renewal during pending deletion. */
bool storage_source_renew_lease(uint64_t id);
int storage_source_resolve(uint64_t recording_id, char path[MAX_PATH_LENGTH],
                           char error[256]);
int storage_source_process_one(void);
int storage_source_worker_start(void);
void storage_source_worker_shutdown(void);
/* Safe eviction of idle cache bytes. Returns actual removed bytes. */
uint64_t storage_source_trim(uint64_t bytes_needed);
bool storage_source_has_lease(uint64_t recording_id);
#endif
