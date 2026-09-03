/** @file system_health_provider.h Optional bounded hardware-provider contract. */

#ifndef LIGHTNVR_SYSTEM_HEALTH_PROVIDER_H
#define LIGHTNVR_SYSTEM_HEALTH_PROVIDER_H

#include <stddef.h>

#include "telemetry/system_health_collector.h"

#define SYSTEM_HEALTH_PROVIDER_NAME_LENGTH 32U

typedef struct {
    char id[SYSTEM_HEALTH_ID_LENGTH];
    system_health_scope_t scope;
    system_health_capability_t capability;
} system_health_provider_resource_t;

typedef struct {
    system_health_provider_resource_t resources[SYSTEM_HEALTH_MAX_DEVICES];
    size_t count;
    size_t dropped;
} system_health_provider_inventory_t;

typedef int (*system_health_provider_discover_fn)(
    void *state, const system_health_collect_context_t *context,
    system_health_provider_inventory_t *inventory);

typedef struct {
    char name[SYSTEM_HEALTH_PROVIDER_NAME_LENGTH];
    system_health_capability_t capability;
    void *state;
    system_health_provider_discover_fn discover;
    system_health_collect_fn collect;
    system_health_collector_destroy_fn destroy;
} system_health_provider_t;

#endif /* LIGHTNVR_SYSTEM_HEALTH_PROVIDER_H */
