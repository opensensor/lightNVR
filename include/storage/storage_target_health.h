#ifndef LIGHTNVR_STORAGE_STORAGE_TARGET_HEALTH_H
#define LIGHTNVR_STORAGE_STORAGE_TARGET_HEALTH_H

#include <stdbool.h>

#include "database/db_storage_targets.h"

/*
 * Probe a configured target, persist its health, and publish a normalized
 * event when availability changes. Event delivery failure never changes the
 * database probe result.
 */
db_storage_target_result_t storage_target_probe_and_publish(
    const char *uuid, bool write_test, storage_target_t *target);

/* Refresh every configured target through the transition-aware probe. */
int storage_target_refresh_health_and_publish(void);

#endif /* LIGHTNVR_STORAGE_STORAGE_TARGET_HEALTH_H */
