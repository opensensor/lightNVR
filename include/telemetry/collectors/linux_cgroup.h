/** @file linux_cgroup.h Linux cgroup v1/v2 health collector. */

#ifndef LIGHTNVR_TELEMETRY_COLLECTORS_LINUX_CGROUP_H
#define LIGHTNVR_TELEMETRY_COLLECTORS_LINUX_CGROUP_H

#include <stdbool.h>
#include <stdint.h>

#include "telemetry/system_health_collector.h"

typedef enum {
    LINUX_CGROUP_UNKNOWN = 0,
    LINUX_CGROUP_V1,
    LINUX_CGROUP_V2
} linux_cgroup_version_t;

typedef struct {
    bool unlimited;
    uint64_t value;
} linux_cgroup_limit_t;

typedef struct {
    linux_cgroup_limit_t quota_usec;
    uint64_t period_usec;
} linux_cgroup_cpu_max_t;

typedef struct {
    uint64_t usage_usec;
    bool throttle_present;
    uint64_t periods;
    uint64_t throttled_periods;
    uint64_t throttled_usec;
} linux_cgroup_cpu_stat_t;

typedef struct {
    uint64_t low;
    uint64_t high;
    uint64_t max;
    uint64_t oom;
    uint64_t oom_kill;
} linux_cgroup_memory_events_t;

typedef struct {
    bool cpu_valid;
    bool oom_valid;
    linux_cgroup_version_t version;
    linux_cgroup_cpu_stat_t cpu;
    uint64_t oom_kills;
    uint64_t sampled_monotonic_ms;
} linux_cgroup_state_t;

int linux_cgroup_parse_limit(const char *text, linux_cgroup_limit_t *out);
int linux_cgroup_parse_cpu_max(const char *text,
                               linux_cgroup_cpu_max_t *out);
int linux_cgroup_parse_cpu_stat(const char *text,
                                linux_cgroup_cpu_stat_t *out);
int linux_cgroup_parse_memory_events(
    const char *text, linux_cgroup_memory_events_t *out);

void linux_cgroup_state_init(linux_cgroup_state_t *state);
bool linux_cgroup_collector_init(system_health_collector_t *collector,
                                 linux_cgroup_state_t *state);

#endif /* LIGHTNVR_TELEMETRY_COLLECTORS_LINUX_CGROUP_H */
