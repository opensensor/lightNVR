/** @file system_health_policy.h Immutable host-health policy snapshots. */

#ifndef LIGHTNVR_SYSTEM_HEALTH_POLICY_H
#define LIGHTNVR_SYSTEM_HEALTH_POLICY_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "telemetry/system_health_types.h"

#define SYSTEM_HEALTH_PROFILE_LENGTH 16U
#define SYSTEM_HEALTH_PROVIDER_LENGTH 16U
#define SYSTEM_HEALTH_POLICY_ERROR_LENGTH 256U
#define SYSTEM_HEALTH_OVERRIDE_JSON_MAX 16384U
#define SYSTEM_HEALTH_OVERRIDES_SETTING_KEY "health_condition_overrides"

#define SYSTEM_HEALTH_FAST_INTERVAL_MIN 5U
#define SYSTEM_HEALTH_FAST_INTERVAL_MAX 60U
#define SYSTEM_HEALTH_NORMAL_INTERVAL_MIN 15U
#define SYSTEM_HEALTH_NORMAL_INTERVAL_MAX 600U
#define SYSTEM_HEALTH_SLOW_INTERVAL_MIN 60U
#define SYSTEM_HEALTH_SLOW_INTERVAL_MAX 3600U
#define SYSTEM_HEALTH_DEVICE_INTERVAL_MIN 300U
#define SYSTEM_HEALTH_DEVICE_INTERVAL_MAX 86400U
#define SYSTEM_HEALTH_PRESENCE_INTERVAL_MIN 15U
#define SYSTEM_HEALTH_PRESENCE_INTERVAL_MAX 3600U
#define SYSTEM_HEALTH_RETENTION_DAYS_MIN 7U
#define SYSTEM_HEALTH_RETENTION_DAYS_MAX 3650U

typedef enum {
    SYSTEM_HEALTH_PROFILE_BALANCED = 0,
    SYSTEM_HEALTH_PROFILE_CONSERVATIVE,
    SYSTEM_HEALTH_PROFILE_DISABLED,
    SYSTEM_HEALTH_PROFILE_CUSTOM
} system_health_profile_t;

typedef enum {
    SYSTEM_HEALTH_THRESHOLD_NONE = 0,
    SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE,
    SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE
} system_health_threshold_direction_t;

/** Scalar settings persisted in the [health] INI domain. */
typedef struct {
    bool enabled;
    char profile[SYSTEM_HEALTH_PROFILE_LENGTH];
    uint32_t fast_interval_seconds;
    uint32_t normal_interval_seconds;
    uint32_t slow_interval_seconds;
    uint32_t device_interval_seconds;
    bool write_probe_enabled;
    char hardware_provider[SYSTEM_HEALTH_PROVIDER_LENGTH];
    uint32_t presence_interval_seconds;
    uint32_t incident_retention_days;
} system_health_policy_settings_t;

typedef struct {
    system_health_condition_t condition;
    system_health_profile_t profile;
    bool enabled;
    bool overridden;
    system_health_threshold_direction_t direction;
    system_health_unit_t unit;
    double warning_threshold;
    double critical_threshold;
    double recovery_threshold;
    uint32_t warning_for_seconds;
    uint32_t critical_for_seconds;
    uint32_t recovery_for_seconds;
} system_health_condition_policy_t;

typedef struct {
    uint64_t generation;
    system_health_policy_settings_t settings;
    system_health_condition_policy_t conditions[SYSTEM_HEALTH_CONDITION_COUNT];
} system_health_policy_t;

void system_health_policy_settings_defaults(
    system_health_policy_settings_t *settings);
bool system_health_policy_profile_from_name(const char *name,
                                            system_health_profile_t *profile);
const char *system_health_policy_profile_name(system_health_profile_t profile);

int system_health_policy_validate_settings(
    const system_health_policy_settings_t *settings,
    char error[SYSTEM_HEALTH_POLICY_ERROR_LENGTH]);

/**
 * Build a complete policy without changing the active policy. override_json
 * may be NULL/empty. canonical_json receives the deterministic persisted form.
 */
int system_health_policy_build(
    const system_health_policy_settings_t *settings, const char *override_json,
    system_health_policy_t *policy, char *canonical_json,
    size_t canonical_json_size,
    char error[SYSTEM_HEALTH_POLICY_ERROR_LENGTH]);

/** Atomically replace active policy after validating the complete candidate. */
int system_health_policy_replace(
    const system_health_policy_t *candidate,
    char error[SYSTEM_HEALTH_POLICY_ERROR_LENGTH]);

/** Copy the current immutable policy; no lock remains held on return. */
int system_health_policy_snapshot(system_health_policy_t *snapshot);

/** Serialize the complete effective policy for administrative visibility. */
int system_health_policy_serialize(const system_health_policy_t *policy,
                                   char *json, size_t json_size);

#endif /* LIGHTNVR_SYSTEM_HEALTH_POLICY_H */
