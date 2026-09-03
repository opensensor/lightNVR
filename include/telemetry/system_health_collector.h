/** @file system_health_collector.h Bounded portable collector contract. */

#ifndef LIGHTNVR_SYSTEM_HEALTH_COLLECTOR_H
#define LIGHTNVR_SYSTEM_HEALTH_COLLECTOR_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "telemetry/system_health_types.h"

#define SYSTEM_HEALTH_COLLECTOR_NAME_LENGTH 32U

typedef enum {
    SYSTEM_HEALTH_TIER_FAST = 0,
    SYSTEM_HEALTH_TIER_NORMAL,
    SYSTEM_HEALTH_TIER_SLOW,
    SYSTEM_HEALTH_TIER_DEVICE
} system_health_sampling_tier_t;

typedef struct {
    uint64_t monotonic_ms;
    int64_t wall_time_ms;
    const char *proc_root;
    const char *sys_root;
    const char *cgroup_root;
} system_health_collect_context_t;

typedef struct {
    system_health_observation_t *items;
    size_t capacity;
    size_t count;
    size_t dropped;
} system_health_observation_sink_t;

typedef int (*system_health_collect_fn)(
    void *state, const system_health_collect_context_t *context,
    system_health_observation_sink_t *sink);
typedef void (*system_health_collector_destroy_fn)(void *state);

typedef struct {
    char name[SYSTEM_HEALTH_COLLECTOR_NAME_LENGTH];
    system_health_scope_t scope;
    system_health_sampling_tier_t tier;
    uint32_t interval_seconds;
    uint32_t stale_after_seconds;
    void *state;
    system_health_collect_fn collect;
    system_health_collector_destroy_fn destroy;
} system_health_collector_t;

bool system_health_observation_sink_append(
    system_health_observation_sink_t *sink,
    const system_health_observation_t *observation);

#endif /* LIGHTNVR_SYSTEM_HEALTH_COLLECTOR_H */
