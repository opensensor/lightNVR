#ifndef LIGHTNVR_STORAGE_PLACEMENT_H
#define LIGHTNVR_STORAGE_PLACEMENT_H

#include <stdbool.h>
#include <stdint.h>

#include "core/config.h"
#include "database/db_storage_targets.h"
#include "utils/uuid.h"

typedef enum {
    STORAGE_PLACEMENT_READY = 0,
    STORAGE_PLACEMENT_PAUSED,
    STORAGE_PLACEMENT_FAILED,
    STORAGE_PLACEMENT_ERROR
} storage_placement_status_t;

typedef struct {
    storage_placement_status_t status;
    bool target_is_default;
    char target_uuid[LIGHTNVR_UUID_STRING_SIZE];
    char target_root[MAX_PATH_LENGTH];
    char object_key[STORAGE_TARGET_OBJECT_KEY_MAX];
    char policy_uuid[LIGHTNVR_UUID_STRING_SIZE];
    int64_t policy_version;
    char reason[64];
} storage_placement_t;

/* Resolve the effective policy for a camera name and select a healthy target. */
int storage_placement_select(const char *stream_name,
                             storage_placement_t *placement);

/* Explicit invalidation for tests and inventory changes; policy CRUD is automatic. */
void storage_placement_cache_invalidate(void);

#endif /* LIGHTNVR_STORAGE_PLACEMENT_H */
