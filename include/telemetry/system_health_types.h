/**
 * @file system_health_types.h
 * @brief Bounded public data contracts for host and hardware health.
 *
 * These records cross collector, policy, API, and metrics boundaries.  Keep
 * them pointer-free so a completed snapshot can be copied atomically by its
 * owner.  Strings are stable logical identifiers, never raw paths, addresses,
 * device serials, or provider error messages.
 */

#ifndef LIGHTNVR_SYSTEM_HEALTH_TYPES_H
#define LIGHTNVR_SYSTEM_HEALTH_TYPES_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

/* Public-cardinality and memory bounds. */
#define SYSTEM_HEALTH_MAX_FILESYSTEMS 18U /* root, recording, and 16 targets */
#define SYSTEM_HEALTH_MAX_INTERFACES 16U
#define SYSTEM_HEALTH_MAX_SENSORS 32U
#define SYSTEM_HEALTH_MAX_DEVICES 16U
#define SYSTEM_HEALTH_MAX_INCIDENTS 64U
#define SYSTEM_HEALTH_RING_SAMPLES 120U
#define SYSTEM_HEALTH_MAX_COLLECTORS 32U
#define SYSTEM_HEALTH_MAX_OBSERVATIONS 256U
#define SYSTEM_HEALTH_ID_LENGTH 64U
#define SYSTEM_HEALTH_METRIC_LENGTH 64U

typedef enum {
    SYSTEM_HEALTH_SCOPE_PROCESS = 0,
    SYSTEM_HEALTH_SCOPE_CONTAINER,
    SYSTEM_HEALTH_SCOPE_HOST,
    SYSTEM_HEALTH_SCOPE_FILESYSTEM,
    SYSTEM_HEALTH_SCOPE_DEVICE,
    SYSTEM_HEALTH_SCOPE_COUNT
} system_health_scope_t;

typedef enum {
    SYSTEM_HEALTH_CAPABILITY_AVAILABLE = 0,
    SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
    SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED,
    SYSTEM_HEALTH_CAPABILITY_STALE,
    SYSTEM_HEALTH_CAPABILITY_ERROR,
    SYSTEM_HEALTH_CAPABILITY_COUNT
} system_health_capability_t;

typedef enum {
    SYSTEM_HEALTH_FRESHNESS_UNKNOWN = 0,
    SYSTEM_HEALTH_FRESHNESS_FRESH,
    SYSTEM_HEALTH_FRESHNESS_STALE
} system_health_freshness_t;

typedef enum {
    SYSTEM_HEALTH_UNIT_NONE = 0,
    SYSTEM_HEALTH_UNIT_RATIO,
    SYSTEM_HEALTH_UNIT_BYTES,
    SYSTEM_HEALTH_UNIT_COUNT,
    SYSTEM_HEALTH_UNIT_SECONDS,
    SYSTEM_HEALTH_UNIT_CELSIUS,
    SYSTEM_HEALTH_UNIT_HERTZ,
    SYSTEM_HEALTH_UNIT_BOOLEAN
} system_health_unit_t;

typedef enum {
    SYSTEM_HEALTH_SEVERITY_NONE = 0,
    SYSTEM_HEALTH_SEVERITY_WARNING,
    SYSTEM_HEALTH_SEVERITY_ERROR,
    SYSTEM_HEALTH_SEVERITY_CRITICAL
} system_health_severity_t;

typedef enum {
    SYSTEM_HEALTH_STATE_UNKNOWN = 0,
    SYSTEM_HEALTH_STATE_HEALTHY,
    SYSTEM_HEALTH_STATE_PENDING,
    SYSTEM_HEALTH_STATE_OPEN,
    SYSTEM_HEALTH_STATE_RECOVERING,
    SYSTEM_HEALTH_STATE_CLOSED
} system_health_state_t;

/** Stable, versioned condition registry. Append only; never renumber. */
typedef enum {
    SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW = 0,
    SYSTEM_HEALTH_CONDITION_MEMORY_OOM_KILL,
    SYSTEM_HEALTH_CONDITION_MEMORY_SWAP_THRASH,
    SYSTEM_HEALTH_CONDITION_CPU_SATURATION,
    SYSTEM_HEALTH_CONDITION_CPU_THROTTLED,
    SYSTEM_HEALTH_CONDITION_IO_PRESSURE,
    SYSTEM_HEALTH_CONDITION_FILESYSTEM_BYTES_LOW,
    SYSTEM_HEALTH_CONDITION_FILESYSTEM_INODES_LOW,
    SYSTEM_HEALTH_CONDITION_FILESYSTEM_READ_ONLY,
    SYSTEM_HEALTH_CONDITION_FILESYSTEM_WRITE_FAILED,
    SYSTEM_HEALTH_CONDITION_THERMAL_HIGH,
    SYSTEM_HEALTH_CONDITION_NETWORK_LINK_DOWN,
    SYSTEM_HEALTH_CONDITION_NETWORK_ERROR_RATE,
    SYSTEM_HEALTH_CONDITION_CLOCK_UNSYNCHRONIZED,
    SYSTEM_HEALTH_CONDITION_CLOCK_JUMP,
    SYSTEM_HEALTH_CONDITION_PROCESS_FD_EXHAUSTION,
    SYSTEM_HEALTH_CONDITION_PROCESS_PID_EXHAUSTION,
    SYSTEM_HEALTH_CONDITION_PROCESS_ALLOCATION_FAILED,
    SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_PREFAIL,
    SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_CRITICAL,
    SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_CORRECTED,
    SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_UNCORRECTABLE,
    SYSTEM_HEALTH_CONDITION_HARDWARE_FAN_FAILED,
    SYSTEM_HEALTH_CONDITION_HARDWARE_POWER_UNSTABLE,
    SYSTEM_HEALTH_CONDITION_COLLECTOR_STALE,
    SYSTEM_HEALTH_CONDITION_UNEXPECTED_RESTART,
    SYSTEM_HEALTH_CONDITION_EVENT_DELIVERY_DEGRADED,
    SYSTEM_HEALTH_CONDITION_COUNT
} system_health_condition_t;

/** Numeric sample. value_valid is false for every unavailable capability. */
typedef struct {
    char metric[SYSTEM_HEALTH_METRIC_LENGTH];
    char resource_id[SYSTEM_HEALTH_ID_LENGTH];
    system_health_scope_t scope;
    system_health_capability_t capability;
    system_health_freshness_t freshness;
    system_health_unit_t unit;
    bool value_valid;
    double value;
    uint64_t sampled_monotonic_ms;
    int64_t observed_wall_time_ms;
} system_health_observation_t;

/** Provenance for a monotonic kernel/provider counter delta. */
typedef struct {
    bool delta_valid;
    bool reset_detected;
    uint64_t current;
    uint64_t previous;
    uint64_t delta;
    uint64_t interval_ms;
    uint64_t sampled_monotonic_ms;
} system_health_counter_t;

typedef struct {
    system_health_observation_t observations[SYSTEM_HEALTH_MAX_OBSERVATIONS];
    size_t observation_count;
    size_t observations_dropped;
    uint64_t sequence;
    uint64_t completed_monotonic_ms;
    int64_t completed_wall_time_ms;
} system_health_snapshot_t;

const char *system_health_scope_name(system_health_scope_t scope);
const char *system_health_capability_name(system_health_capability_t capability);
const char *system_health_unit_name(system_health_unit_t unit);
const char *system_health_severity_name(system_health_severity_t severity);
const char *system_health_condition_code(system_health_condition_t condition);
bool system_health_condition_from_code(const char *code,
                                       system_health_condition_t *condition_out);

void system_health_observation_set_available(
    system_health_observation_t *observation, double value,
    system_health_unit_t unit);
void system_health_observation_set_unavailable(
    system_health_observation_t *observation,
    system_health_capability_t capability);
bool system_health_snapshot_append(system_health_snapshot_t *snapshot,
                                   const system_health_observation_t *observation);

#endif /* LIGHTNVR_SYSTEM_HEALTH_TYPES_H */
