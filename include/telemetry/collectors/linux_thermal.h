/**
 * @file linux_thermal.h
 * @brief Bounded Linux thermal-zone and hwmon temperature collection.
 */

#ifndef LIGHTNVR_LINUX_THERMAL_H
#define LIGHTNVR_LINUX_THERMAL_H

#include <stddef.h>
#include <stdint.h>

#include "telemetry/system_health_collector.h"

#define LINUX_THERMAL_STATE_CAPACITY (SYSTEM_HEALTH_MAX_SENSORS * 2U)

typedef struct {
    char id[SYSTEM_HEALTH_ID_LENGTH];
    char limit_metric[SYSTEM_HEALTH_METRIC_LENGTH];
    bool present;
    bool seen;
    uint8_t missing_cycles;
} linux_thermal_sensor_state_t;

typedef struct {
    linux_thermal_sensor_state_t sensors[LINUX_THERMAL_STATE_CAPACITY];
    size_t sensor_count;
    uint64_t resources_dropped_total;
} linux_thermal_state_t;

typedef struct {
    char id[SYSTEM_HEALTH_ID_LENGTH];
    system_health_capability_t temperature_capability;
    bool temperature_valid;
    double temperature_celsius;
    system_health_capability_t limit_capability;
    bool limit_valid;
    bool limit_is_critical;
    double limit_celsius;
} linux_thermal_sensor_sample_t;

typedef struct {
    linux_thermal_sensor_sample_t sensors[SYSTEM_HEALTH_MAX_SENSORS];
    size_t sensor_count;
    size_t resources_dropped;
    system_health_capability_t capability;
} linux_thermal_result_t;

void linux_thermal_state_init(linux_thermal_state_t *state);

/** Map a filesystem error to the public capability contract. */
system_health_capability_t linux_thermal_capability_from_errno(int error_number);

/**
 * Collect from <context->sys_root>/class/thermal and /class/hwmon.
 * Missing optional classes and channels are capabilities, not hard failures.
 */
int linux_thermal_collect(linux_thermal_state_t *state,
                          const system_health_collect_context_t *context,
                          system_health_observation_sink_t *sink,
                          linux_thermal_result_t *result);

/** Fill a coordinator descriptor around caller-owned state. */
bool linux_thermal_collector_init(system_health_collector_t *collector,
                                  linux_thermal_state_t *state,
                                  uint32_t interval_seconds,
                                  uint32_t stale_after_seconds);

#endif /* LIGHTNVR_LINUX_THERMAL_H */
