#include "telemetry/system_health_types.h"
#include "telemetry/system_health_collector.h"

#include <string.h>

static const char *const condition_codes[SYSTEM_HEALTH_CONDITION_COUNT] = {
    "memory.available_low",
    "memory.oom_kill",
    "memory.swap_thrash",
    "cpu.saturation",
    "cpu.throttled",
    "io.pressure",
    "filesystem.bytes_low",
    "filesystem.inodes_low",
    "filesystem.read_only",
    "filesystem.write_failed",
    "thermal.high",
    "network.link_down",
    "network.error_rate",
    "clock.unsynchronized",
    "clock.jump",
    "process.fd_exhaustion",
    "process.pid_exhaustion",
    "process.allocation_failed",
    "storage.device_prefail",
    "storage.device_critical",
    "hardware.ecc_corrected",
    "hardware.ecc_uncorrectable",
    "hardware.fan_failed",
    "hardware.power_unstable",
    "health.collector_stale",
    "system.unexpected_restart",
    "event.delivery_degraded"
};

static const char *bounded_name(int value, const char *const *names,
                                size_t name_count) {
    if (value < 0 || (size_t)value >= name_count) return "unknown";
    return names[value];
}

const char *system_health_scope_name(system_health_scope_t scope) {
    static const char *const names[] = {
        "process", "container", "host", "filesystem", "device"
    };
    return bounded_name((int)scope, names, sizeof(names) / sizeof(names[0]));
}

const char *system_health_capability_name(system_health_capability_t capability) {
    static const char *const names[] = {
        "available", "unsupported", "permission_denied", "stale", "error"
    };
    return bounded_name((int)capability, names,
                        sizeof(names) / sizeof(names[0]));
}

const char *system_health_unit_name(system_health_unit_t unit) {
    static const char *const names[] = {
        "none", "ratio", "bytes", "count", "seconds", "celsius",
        "hertz", "boolean"
    };
    return bounded_name((int)unit, names, sizeof(names) / sizeof(names[0]));
}

const char *system_health_severity_name(system_health_severity_t severity) {
    static const char *const names[] = {
        "none", "warning", "error", "critical"
    };
    return bounded_name((int)severity, names,
                        sizeof(names) / sizeof(names[0]));
}

const char *system_health_condition_code(system_health_condition_t condition) {
    if (condition < 0 || condition >= SYSTEM_HEALTH_CONDITION_COUNT) {
        return NULL;
    }
    return condition_codes[condition];
}

bool system_health_condition_from_code(const char *code,
                                       system_health_condition_t *condition_out) {
    if (!code || !condition_out) return false;
    for (int i = 0; i < SYSTEM_HEALTH_CONDITION_COUNT; ++i) {
        if (strcmp(code, condition_codes[i]) == 0) {
            *condition_out = (system_health_condition_t)i;
            return true;
        }
    }
    return false;
}

void system_health_observation_set_available(
    system_health_observation_t *observation, double value,
    system_health_unit_t unit) {
    if (!observation) return;
    observation->capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    observation->freshness = SYSTEM_HEALTH_FRESHNESS_FRESH;
    observation->unit = unit;
    observation->value_valid = true;
    observation->value = value;
}

void system_health_observation_set_unavailable(
    system_health_observation_t *observation,
    system_health_capability_t capability) {
    if (!observation) return;
    if (capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE) {
        capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
    }
    observation->capability = capability;
    observation->freshness = capability == SYSTEM_HEALTH_CAPABILITY_STALE
        ? SYSTEM_HEALTH_FRESHNESS_STALE : SYSTEM_HEALTH_FRESHNESS_UNKNOWN;
    observation->unit = SYSTEM_HEALTH_UNIT_NONE;
    observation->value_valid = false;
    observation->value = 0.0;
}

bool system_health_snapshot_append(system_health_snapshot_t *snapshot,
                                   const system_health_observation_t *observation) {
    if (!snapshot || !observation) return false;
    if (snapshot->observation_count >= SYSTEM_HEALTH_MAX_OBSERVATIONS) {
        snapshot->observations_dropped++;
        return false;
    }
    snapshot->observations[snapshot->observation_count++] = *observation;
    return true;
}

bool system_health_observation_sink_append(
    system_health_observation_sink_t *sink,
    const system_health_observation_t *observation) {
    if (!sink || !observation || !sink->items || sink->count >= sink->capacity) {
        if (sink) sink->dropped++;
        return false;
    }
    sink->items[sink->count++] = *observation;
    return true;
}
