/** @file system_health.h Tiered host-health sampler lifecycle. */

#ifndef LIGHTNVR_SYSTEM_HEALTH_H
#define LIGHTNVR_SYSTEM_HEALTH_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "telemetry/system_health_collector.h"
#include "telemetry/system_health_policy.h"
#include "telemetry/system_health_provider.h"

#define SYSTEM_HEALTH_PATH_LENGTH 1024U
#define SYSTEM_HEALTH_TIER_COUNT 4U

typedef struct {
    bool enabled;
    bool register_builtin_collectors;
    uint32_t tier_interval_ms[SYSTEM_HEALTH_TIER_COUNT];
    uint32_t collector_deadline_ms[SYSTEM_HEALTH_TIER_COUNT];
    char proc_root[SYSTEM_HEALTH_PATH_LENGTH];
    char sys_root[SYSTEM_HEALTH_PATH_LENGTH];
    char cgroup_root[SYSTEM_HEALTH_PATH_LENGTH];
    char root_path[SYSTEM_HEALTH_PATH_LENGTH];
    char recording_path[SYSTEM_HEALTH_PATH_LENGTH];
    char hardware_provider[SYSTEM_HEALTH_PROVIDER_LENGTH];
} system_health_options_t;

typedef struct {
    uint64_t sequence;
    uint64_t completed_monotonic_ms;
    int64_t completed_wall_time_ms;
    uint32_t observation_count;
    uint32_t observations_dropped;
} system_health_summary_t;

typedef struct {
    bool initialized;
    bool enabled;
    bool ring_allocated;
    uint32_t worker_threads;
    uint64_t generations_completed;
    uint64_t collections_completed;
    uint64_t collection_errors;
    uint64_t collection_timeouts;
    uint64_t overlap_skips;
    uint64_t observations_dropped;
    uint64_t coverage_overflows;
    uint32_t abandoned_helpers;
} system_health_stats_t;

/** Bounded, credential-free runtime view of one collector. */
typedef struct {
    char name[SYSTEM_HEALTH_COLLECTOR_NAME_LENGTH];
    system_health_scope_t scope;
    system_health_sampling_tier_t tier;
    uint32_t interval_seconds;
    uint32_t stale_after_seconds;
    uint64_t attempts;
    uint64_t completions;
    uint64_t failures;
    uint64_t timeouts;
    uint64_t overlap_skips;
    uint64_t last_duration_ms;
    uint64_t maximum_duration_ms;
    uint64_t last_attempt_monotonic_ms;
    uint64_t last_success_monotonic_ms;
    bool busy;
    bool stale;
} system_health_collector_stats_t;

void system_health_options_defaults(system_health_options_t *options);
int system_health_options_from_policy(
    const system_health_policy_settings_t *settings, const char *recording_path,
    system_health_options_t *options);

/** Initialize the singleton without starting workers. Idempotent shutdown is safe. */
int system_health_init(const system_health_options_t *options);
bool system_health_register_collector(const system_health_collector_t *collector);
bool system_health_register_provider(const system_health_provider_t *provider);
int system_health_start(void);
/** Coalesced wake-up for presence changes or an administrator refresh. */
bool system_health_request_tier(system_health_sampling_tier_t tier);
void system_health_shutdown(void);

/** Run one tier synchronously; useful for startup and deterministic tests. */
int system_health_collect_tier(system_health_sampling_tier_t tier);

/** Readers only copy completed immutable generations and never trigger work. */
bool system_health_snapshot_copy(system_health_snapshot_t *snapshot);
size_t system_health_summary_copy(system_health_summary_t *summaries,
                                  size_t capacity);
void system_health_get_stats(system_health_stats_t *stats);
size_t system_health_collector_stats_copy(
    system_health_collector_stats_t *stats, size_t capacity);

#endif /* LIGHTNVR_SYSTEM_HEALTH_H */
