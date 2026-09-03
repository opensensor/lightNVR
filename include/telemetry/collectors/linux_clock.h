/** @file linux_clock.h Injectable Linux clock-health sampling. */

#ifndef LIGHTNVR_LINUX_CLOCK_HEALTH_H
#define LIGHTNVR_LINUX_CLOCK_HEALTH_H

#include <stdbool.h>
#include <stdint.h>

#include "telemetry/system_health_types.h"

#define LINUX_CLOCK_DEFAULT_STARTUP_GRACE_MS (10ULL * 60ULL * 1000ULL)
#define LINUX_CLOCK_DEFAULT_JUMP_THRESHOLD_MS 2000LL

typedef int (*linux_clock_read_time_fn)(void *context, int64_t *milliseconds);
typedef int (*linux_clock_read_sync_fn)(void *context, bool *synchronized);

typedef struct {
    void *context;
    linux_clock_read_time_fn realtime;
    linux_clock_read_time_fn monotonic;
    linux_clock_read_sync_fn synchronization;
} linux_clock_source_t;

typedef struct {
    system_health_capability_t capability;
    bool synchronization_known;
    bool synchronized;
    int64_t realtime_ms;
    uint64_t monotonic_ms;
} linux_clock_sample_t;

typedef struct {
    bool initialized;
    uint64_t startup_monotonic_ms;
    int64_t previous_realtime_ms;
    uint64_t previous_monotonic_ms;
} linux_clock_state_t;

typedef struct {
    system_health_capability_t capability;
    bool synchronization_known;
    bool synchronized;
    bool startup_grace_active;
    bool jump_detected;
    int64_t jump_ms;
} linux_clock_result_t;

int linux_clock_sample(const linux_clock_source_t *source,
                       linux_clock_sample_t *sample);
linux_clock_result_t linux_clock_evaluate(linux_clock_state_t *state,
                                          const linux_clock_sample_t *sample,
                                          uint64_t startup_grace_ms,
                                          int64_t jump_threshold_ms);
linux_clock_source_t linux_clock_default_source(void);

#endif /* LIGHTNVR_LINUX_CLOCK_HEALTH_H */
