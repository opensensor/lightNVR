#define _POSIX_C_SOURCE 200809L

#include "telemetry/system_health_evaluator.h"

#include <math.h>
#include <pthread.h>
#include <stdatomic.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include <cjson/cJSON.h>

#include "core/event_producers.h"
#include "core/logger.h"
#include "telemetry/collectors/linux_restart.h"
#include "telemetry/system_health.h"
#include "utils/uuid.h"

#define EVALUATOR_CANDIDATE_MAX 96U
#define EVALUATOR_AVAILABILITY_MAX 128U

typedef struct {
    system_health_condition_t condition;
    char subject[SYSTEM_HEALTH_ID_LENGTH];
    system_health_scope_t scope;
    system_health_observation_t observation;
    system_health_severity_t desired;
    bool recovery_safe;
    bool immediate;
    bool one_shot;
} evaluator_candidate_t;

typedef struct {
    bool used;
    system_health_condition_t condition;
    char subject[SYSTEM_HEALTH_ID_LENGTH];
    system_health_scope_t scope;
    system_health_state_t state;
    system_health_severity_t severity;
    system_health_severity_t pending_severity;
    uint64_t pending_since_ms;
    uint64_t recovery_since_ms;
    uint64_t last_material_ms;
    double last_material_value;
    bool last_material_value_valid;
    char incident_id[LIGHTNVR_UUID_STRING_SIZE];
    int64_t first_observed_wall_ms;
    uint64_t first_observed_monotonic_ms;
    int64_t last_observed_wall_ms;
    system_health_observation_t last_observation;
    uint64_t last_episode_sample_ms;
    int64_t last_persisted_wall_ms;

    bool persistence_pending;
    system_health_incident_action_t pending_action;
    system_health_state_t pending_previous_state;
    system_health_severity_t pending_previous_severity;
    char pending_event_id[LIGHTNVR_UUID_STRING_SIZE];
    system_health_state_t pending_target_state;
    system_health_severity_t pending_target_severity;
    evaluator_candidate_t pending_candidate;
    int64_t pending_wall_ms;
    uint64_t next_retry_ms;
    uint32_t retry_delay_ms;
} evaluator_slot_t;

typedef struct {
    bool used;
    bool available_ever;
    char metric[SYSTEM_HEALTH_METRIC_LENGTH];
    char subject[SYSTEM_HEALTH_ID_LENGTH];
    system_health_scope_t scope;
    uint64_t unavailable_since_ms;
    uint64_t seen_sequence;
    system_health_capability_t capability;
} availability_slot_t;

struct system_health_evaluator {
    system_health_evaluator_config_t config;
    evaluator_slot_t slots[SYSTEM_HEALTH_MAX_INCIDENTS];
    availability_slot_t availability[EVALUATOR_AVAILABILITY_MAX];
    system_health_evaluator_stats_t stats;
    uint64_t last_snapshot_sequence;
    pthread_mutex_t view_lock;
    system_health_incident_view_t views[2][SYSTEM_HEALTH_MAX_INCIDENTS];
    size_t view_counts[2];
    unsigned int active_view;
    system_health_evaluator_stats_t published_stats;
};

static atomic_uint_fast64_t
    immediate_counts[SYSTEM_HEALTH_CONDITION_COUNT]
                    [SYSTEM_HEALTH_IMMEDIATE_RESOURCE_COUNT];

static uint64_t monotonic_ms(void) {
    struct timespec value;
    if (clock_gettime(CLOCK_MONOTONIC, &value) != 0) return 0U;
    return (uint64_t)value.tv_sec * 1000U + (uint64_t)value.tv_nsec / 1000000U;
}

static int64_t wall_time_ms(void) {
    struct timespec value;
    if (clock_gettime(CLOCK_REALTIME, &value) != 0) return 0;
    return (int64_t)value.tv_sec * 1000 + value.tv_nsec / 1000000;
}

static void copy_string(char *destination, size_t capacity,
                        const char *source) {
    if (!destination || capacity == 0U) return;
    snprintf(destination, capacity, "%s", source ? source : "");
}

void system_health_evaluator_config_defaults(
    system_health_evaluator_config_t *config,
    const system_health_policy_t *policy) {
    if (!config) return;
    memset(config, 0, sizeof(*config));
    if (policy) config->policy = *policy;
    config->stale_after_ms = 180000U;
    config->retry_initial_ms = 1000U;
    config->retry_max_ms = 60000U;
    config->material_change_debounce_ms = 300000U;
    config->material_change_ratio = .10;
    copy_string(config->boot_id, sizeof(config->boot_id), "unknown-boot");
    (void)lightnvr_uuid_generate_v4(config->run_id);
}

system_health_evaluator_t *system_health_evaluator_create(
    const system_health_evaluator_config_t *config) {
    if (!config || !config->boot_id[0] ||
        !lightnvr_uuid_is_valid(config->run_id) ||
        config->retry_initial_ms == 0U ||
        config->retry_max_ms < config->retry_initial_ms ||
        config->material_change_ratio <= 0.0 ||
        !isfinite(config->material_change_ratio)) return NULL;
    system_health_evaluator_t *evaluator = calloc(1U, sizeof(*evaluator));
    if (!evaluator) return NULL;
    if (pthread_mutex_init(&evaluator->view_lock, NULL) != 0) {
        free(evaluator);
        return NULL;
    }
    evaluator->config = *config;
    return evaluator;
}

void system_health_evaluator_destroy(system_health_evaluator_t *evaluator) {
    if (evaluator) pthread_mutex_destroy(&evaluator->view_lock);
    free(evaluator);
}

static evaluator_slot_t *find_slot(system_health_evaluator_t *evaluator,
                                   system_health_condition_t condition,
                                   const char *subject, bool create) {
    evaluator_slot_t *empty = NULL;
    for (size_t index = 0; index < SYSTEM_HEALTH_MAX_INCIDENTS; ++index) {
        evaluator_slot_t *slot = &evaluator->slots[index];
        if (slot->used && slot->condition == condition &&
            strcmp(slot->subject, subject) == 0) return slot;
        if (!slot->used && !empty) empty = slot;
    }
    if (!create || !empty) return NULL;
    memset(empty, 0, sizeof(*empty));
    empty->used = true;
    empty->condition = condition;
    copy_string(empty->subject, sizeof(empty->subject), subject);
    empty->state = SYSTEM_HEALTH_STATE_UNKNOWN;
    evaluator->stats.tracked_conditions++;
    return empty;
}

int system_health_evaluator_resume(
    system_health_evaluator_t *evaluator,
    const system_health_incident_record_t *incident) {
    system_health_condition_t condition;
    if (!evaluator || !incident || !incident->uuid[0] ||
        !system_health_condition_from_code(incident->condition_code,
                                           &condition) ||
        incident->state == SYSTEM_HEALTH_STATE_CLOSED) return -1;
    evaluator_slot_t *slot = find_slot(evaluator, condition, incident->subject,
                                       true);
    if (!slot) return -1;
    slot->scope = incident->scope;
    slot->state = incident->state;
    slot->severity = incident->severity;
    slot->last_persisted_wall_ms = incident->last_seen_at_ms;
    slot->first_observed_wall_ms = incident->first_seen_at_ms;
    slot->last_observed_wall_ms = incident->last_seen_at_ms;
    copy_string(slot->incident_id, sizeof(slot->incident_id), incident->uuid);
    evaluator->stats.active_incidents++;
    return 0;
}

static const system_health_observation_t *find_observation(
    const system_health_snapshot_t *snapshot, const char *metric,
    const char *resource) {
    for (size_t index = 0; index < snapshot->observation_count; ++index) {
        const system_health_observation_t *observation =
            &snapshot->observations[index];
        if (strcmp(observation->metric, metric) == 0 &&
            (!resource || strcmp(observation->resource_id, resource) == 0))
            return observation;
    }
    return NULL;
}

static bool metric_suffix(const char *metric, const char *suffix) {
    size_t metric_length = strlen(metric);
    size_t suffix_length = strlen(suffix);
    return metric_length >= suffix_length &&
           strcmp(metric + metric_length - suffix_length, suffix) == 0;
}

static evaluator_candidate_t *add_candidate(
    evaluator_candidate_t *candidates, size_t *count,
    system_health_condition_t condition, const char *subject,
    const system_health_observation_t *observation) {
    for (size_t index = 0; index < *count; ++index) {
        if (candidates[index].condition == condition &&
            strcmp(candidates[index].subject, subject) == 0)
            return &candidates[index];
    }
    if (*count >= EVALUATOR_CANDIDATE_MAX) return NULL;
    evaluator_candidate_t *candidate = &candidates[(*count)++];
    memset(candidate, 0, sizeof(*candidate));
    candidate->condition = condition;
    copy_string(candidate->subject, sizeof(candidate->subject), subject);
    if (observation) {
        candidate->scope = observation->scope;
        candidate->observation = *observation;
    }
    return candidate;
}

static void apply_threshold(const system_health_condition_policy_t *rule,
                            evaluator_candidate_t *candidate) {
    if (!candidate || !candidate->observation.value_valid || !rule->enabled ||
        rule->direction == SYSTEM_HEALTH_THRESHOLD_NONE ||
        candidate->observation.unit != rule->unit) return;
    double value = candidate->observation.value;
    bool critical = rule->direction == SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE
        ? value < rule->critical_threshold : value > rule->critical_threshold;
    bool warning = rule->direction == SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE
        ? value < rule->warning_threshold : value > rule->warning_threshold;
    candidate->recovery_safe =
        rule->direction == SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE
            ? value > rule->recovery_threshold
            : value < rule->recovery_threshold;
    candidate->desired = critical ? SYSTEM_HEALTH_SEVERITY_CRITICAL
                                  : warning ? SYSTEM_HEALTH_SEVERITY_WARNING
                                            : SYSTEM_HEALTH_SEVERITY_NONE;
}

static system_health_observation_t ratio_observation(
    const system_health_observation_t *available,
    const system_health_observation_t *capacity, const char *metric) {
    system_health_observation_t result = *available;
    copy_string(result.metric, sizeof(result.metric), metric);
    if (!available->value_valid || !capacity || !capacity->value_valid ||
        capacity->value <= 0.0) {
        result.value_valid = false;
        result.value = 0.0;
        result.capability = available->capability !=
                                    SYSTEM_HEALTH_CAPABILITY_AVAILABLE
                                ? available->capability
                                : SYSTEM_HEALTH_CAPABILITY_ERROR;
    } else {
        result.value = available->value / capacity->value;
        result.unit = SYSTEM_HEALTH_UNIT_RATIO;
    }
    return result;
}

static availability_slot_t *availability_slot(
    system_health_evaluator_t *evaluator,
    const system_health_observation_t *observation) {
    availability_slot_t *empty = NULL;
    for (size_t index = 0; index < EVALUATOR_AVAILABILITY_MAX; ++index) {
        availability_slot_t *slot = &evaluator->availability[index];
        if (slot->used && strcmp(slot->metric, observation->metric) == 0 &&
            strcmp(slot->subject, observation->resource_id) == 0) return slot;
        if (!slot->used && !empty) empty = slot;
    }
    if (!empty) return NULL;
    memset(empty, 0, sizeof(*empty));
    empty->used = true;
    copy_string(empty->metric, sizeof(empty->metric), observation->metric);
    copy_string(empty->subject, sizeof(empty->subject), observation->resource_id);
    empty->scope = observation->scope;
    return empty;
}

static evaluator_candidate_t *candidate_from_observation(
    system_health_evaluator_t *evaluator, evaluator_candidate_t *candidates,
    size_t *count, system_health_condition_t condition, const char *subject,
    const system_health_observation_t *observation) {
    evaluator_candidate_t *candidate = add_candidate(
        candidates, count, condition, subject, observation);
    if (candidate)
        apply_threshold(&evaluator->config.policy.conditions[condition],
                        candidate);
    return candidate;
}

static void merge_candidate_evidence(
    evaluator_candidate_t *candidate,
    const system_health_observation_t *observation,
    system_health_severity_t desired, bool recovery_safe, bool immediate,
    bool one_shot) {
    if (!candidate || !observation) return;
    bool had_evidence = candidate->desired != SYSTEM_HEALTH_SEVERITY_NONE ||
        candidate->recovery_safe || candidate->immediate || candidate->one_shot;
    system_health_severity_t previous_desired = candidate->desired;
    if (desired > candidate->desired) {
        candidate->desired = desired;
        candidate->scope = observation->scope;
        candidate->observation = *observation;
    }
    if (desired != SYSTEM_HEALTH_SEVERITY_NONE) {
        candidate->recovery_safe = false;
    } else if (!had_evidence) {
        candidate->recovery_safe = recovery_safe;
    } else if (previous_desired == SYSTEM_HEALTH_SEVERITY_NONE) {
        candidate->recovery_safe = candidate->recovery_safe && recovery_safe;
    }
    candidate->immediate = candidate->immediate || immediate;
    candidate->one_shot = had_evidence
        ? candidate->one_shot && one_shot : one_shot;
}

static bool observation_available(const system_health_observation_t *value) {
    return value && value->capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE &&
           value->freshness != SYSTEM_HEALTH_FRESHNESS_STALE &&
           value->value_valid && isfinite(value->value);
}

static void collect_stale_candidates(
    system_health_evaluator_t *evaluator,
    const system_health_snapshot_t *snapshot, evaluator_candidate_t *candidates,
    size_t *candidate_count) {
    uint64_t now = snapshot->completed_monotonic_ms;
    for (size_t index = 0; index < snapshot->observation_count; ++index) {
        const system_health_observation_t *observation =
            &snapshot->observations[index];
        availability_slot_t *slot = availability_slot(evaluator, observation);
        if (!slot) continue;
        slot->seen_sequence = snapshot->sequence;
        slot->capability = observation->capability;
        if (observation_available(observation)) {
            slot->available_ever = true;
            slot->unavailable_since_ms = 0U;
        } else if (slot->available_ever && slot->unavailable_since_ms == 0U) {
            slot->unavailable_since_ms = now;
        }
    }

    for (size_t index = 0; index < EVALUATOR_AVAILABILITY_MAX; ++index) {
        availability_slot_t *availability = &evaluator->availability[index];
        if (!availability->used || !availability->available_ever) continue;
        evaluator_candidate_t *candidate = add_candidate(
            candidates, candidate_count, SYSTEM_HEALTH_CONDITION_COLLECTOR_STALE,
            availability->subject, NULL);
        if (!candidate) continue;
        candidate->scope = availability->scope;
        copy_string(candidate->observation.metric,
                    sizeof(candidate->observation.metric), "collector.freshness");
        copy_string(candidate->observation.resource_id,
                    sizeof(candidate->observation.resource_id),
                    availability->subject);
        candidate->observation.scope = availability->scope;
        candidate->observation.sampled_monotonic_ms = now;
        candidate->observation.observed_wall_time_ms =
            snapshot->completed_wall_time_ms;
        bool overdue = availability->unavailable_since_ms > 0U &&
            now >= availability->unavailable_since_ms &&
            now - availability->unavailable_since_ms >=
                evaluator->config.stale_after_ms;
        if (overdue) {
            candidate->desired = SYSTEM_HEALTH_SEVERITY_WARNING;
            candidate->observation.capability = availability->capability;
            candidate->observation.freshness = SYSTEM_HEALTH_FRESHNESS_STALE;
            candidate->observation.value_valid = false;
            candidate->recovery_safe = false;
        }
    }
    for (size_t candidate_index = 0; candidate_index < *candidate_count;
         ++candidate_index) {
        evaluator_candidate_t *candidate = &candidates[candidate_index];
        if (candidate->condition != SYSTEM_HEALTH_CONDITION_COLLECTOR_STALE)
            continue;
        bool all_available = true;
        for (size_t index = 0; index < EVALUATOR_AVAILABILITY_MAX; ++index) {
            availability_slot_t *availability = &evaluator->availability[index];
            if (availability->used && availability->available_ever &&
                strcmp(availability->subject, candidate->subject) == 0 &&
                availability->unavailable_since_ms != 0U) {
                all_available = false;
                break;
            }
        }
        candidate->recovery_safe = all_available;
        if (all_available)
            system_health_observation_set_available(
                &candidate->observation, 1.0, SYSTEM_HEALTH_UNIT_BOOLEAN);
    }
}

static void build_candidates(
    system_health_evaluator_t *evaluator,
    const system_health_snapshot_t *snapshot,
    const system_health_evaluation_context_t *context,
    evaluator_candidate_t candidates[EVALUATOR_CANDIDATE_MAX],
    size_t *candidate_count) {
    *candidate_count = 0U;
    collect_stale_candidates(evaluator, snapshot, candidates, candidate_count);

    const system_health_observation_t *memory = NULL;
    const system_health_observation_t *cpu = NULL;
    bool cpu_pressure = false;
    bool cpu_full_pressure = false;
    bool io_full_pressure = false;
    bool startup_grace = false;
    for (size_t index = 0; index < snapshot->observation_count; ++index) {
        const system_health_observation_t *observation =
            &snapshot->observations[index];
        if (metric_suffix(observation->metric, "memory.available_ratio") &&
            observation_available(observation) &&
            (!memory || observation->scope == SYSTEM_HEALTH_SCOPE_CONTAINER))
            memory = observation;
        if ((strcmp(observation->metric, "host.cpu.busy_ratio") == 0 ||
             metric_suffix(observation->metric, "cpu.usage_ratio")) &&
            observation_available(observation) &&
            (!cpu || observation->scope == SYSTEM_HEALTH_SCOPE_CONTAINER))
            cpu = observation;
        if ((strcmp(observation->metric, "host.pressure.cpu.some_ratio") == 0 ||
             strcmp(observation->metric,
                    "host.pressure.cpu.some_seconds_delta") == 0) &&
            observation_available(observation) && observation->value > 0.0)
            cpu_pressure = true;
        if (strcmp(observation->metric, "host.pressure.cpu.full_ratio") == 0 &&
            observation_available(observation) && observation->value > 0.0)
            cpu_full_pressure = true;
        if (strcmp(observation->metric, "host.pressure.io.full_ratio") == 0 &&
            observation_available(observation) && observation->value > 0.0)
            io_full_pressure = true;
        if (strcmp(observation->metric, "clock.startup_grace") == 0 &&
            observation_available(observation) && observation->value != 0.0)
            startup_grace = true;
    }
    if (memory)
        (void)candidate_from_observation(
            evaluator, candidates, candidate_count,
            SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW, "memory", memory);
    if (cpu) {
        evaluator_candidate_t *candidate = candidate_from_observation(
            evaluator, candidates, candidate_count,
            SYSTEM_HEALTH_CONDITION_CPU_SATURATION, "cpu", cpu);
        bool degraded = context && context->service_degraded_known &&
                        context->service_degraded;
        if (candidate && candidate->desired != SYSTEM_HEALTH_SEVERITY_NONE &&
            !cpu_pressure && !degraded) {
            candidate->desired = SYSTEM_HEALTH_SEVERITY_NONE;
            candidate->recovery_safe = false;
        } else if (candidate &&
                   candidate->desired == SYSTEM_HEALTH_SEVERITY_CRITICAL &&
                   !cpu_full_pressure && !degraded) {
            candidate->desired = SYSTEM_HEALTH_SEVERITY_WARNING;
        }
    }

    for (size_t index = 0; index < snapshot->observation_count; ++index) {
        const system_health_observation_t *observation =
            &snapshot->observations[index];
        if (!observation_available(observation)) continue;
        const char *metric = observation->metric;
        if (metric_suffix(metric, "cpu.throttled_ratio")) {
            evaluator_candidate_t *candidate = candidate_from_observation(
                evaluator, candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_CPU_THROTTLED, "cpu", observation);
            bool degraded = context && context->service_degraded_known &&
                            context->service_degraded;
            if (candidate &&
                candidate->desired == SYSTEM_HEALTH_SEVERITY_CRITICAL &&
                !degraded)
                candidate->desired = SYSTEM_HEALTH_SEVERITY_WARNING;
        } else if (strcmp(metric, "host.pressure.io.some_seconds_delta") == 0) {
            evaluator_candidate_t *candidate = candidate_from_observation(
                evaluator, candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_IO_PRESSURE, "io", observation);
            if (candidate &&
                candidate->desired == SYSTEM_HEALTH_SEVERITY_CRITICAL &&
                !io_full_pressure)
                candidate->desired = SYSTEM_HEALTH_SEVERITY_WARNING;
        } else if (strcmp(metric, "process.fd_ratio") == 0) {
            (void)candidate_from_observation(
                evaluator, candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_PROCESS_FD_EXHAUSTION, "process",
                observation);
        } else if (strcmp(metric, "process.pid_ratio") == 0) {
            (void)candidate_from_observation(
                evaluator, candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_PROCESS_PID_EXHAUSTION, "process",
                observation);
        } else if (metric_suffix(metric, "pids.available_ratio")) {
            system_health_observation_t used = *observation;
            copy_string(used.metric, sizeof(used.metric), "process.pid_ratio");
            used.value = 1.0 - observation->value;
            (void)candidate_from_observation(
                evaluator, candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_PROCESS_PID_EXHAUSTION, "process",
                &used);
        } else if (strcmp(metric, "network.error_drop_ratio") == 0) {
            (void)candidate_from_observation(
                evaluator, candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_NETWORK_ERROR_RATE,
                observation->resource_id, observation);
        } else if (metric_suffix(metric, "memory.oom_kills_delta") &&
                   observation->value > 0.0) {
            evaluator_candidate_t *candidate = add_candidate(
                candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_MEMORY_OOM_KILL, "memory", observation);
            if (candidate) {
                candidate->desired = SYSTEM_HEALTH_SEVERITY_CRITICAL;
                candidate->immediate = true;
                candidate->one_shot = true;
            }
        } else if (strcmp(metric, "filesystem.read_only") == 0) {
            evaluator_candidate_t *candidate = add_candidate(
                candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_FILESYSTEM_READ_ONLY,
                observation->resource_id, observation);
            if (candidate) {
                candidate->desired = observation->value != 0.0
                    ? SYSTEM_HEALTH_SEVERITY_CRITICAL
                    : SYSTEM_HEALTH_SEVERITY_NONE;
                candidate->recovery_safe = observation->value == 0.0;
                candidate->immediate = observation->value != 0.0;
            }
        } else if (strcmp(metric, "filesystem.mount_present") == 0) {
            evaluator_candidate_t *candidate = add_candidate(
                candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_FILESYSTEM_WRITE_FAILED,
                observation->resource_id, observation);
            if (candidate && observation->value == 0.0) {
                bool no_fallback = context &&
                    context->viable_recording_targets_known &&
                    context->viable_recording_targets == 0U;
                candidate->desired = no_fallback
                    ? SYSTEM_HEALTH_SEVERITY_CRITICAL
                    : SYSTEM_HEALTH_SEVERITY_ERROR;
                candidate->immediate = true;
                candidate->recovery_safe = false;
            } else if (candidate) {
                candidate->recovery_safe = true;
            }
        } else if (strcmp(metric, "filesystem.write_failed") == 0) {
            evaluator_candidate_t *candidate = add_candidate(
                candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_FILESYSTEM_WRITE_FAILED,
                observation->resource_id, observation);
            if (candidate) {
                candidate->desired = observation->value != 0.0
                    ? SYSTEM_HEALTH_SEVERITY_CRITICAL
                    : SYSTEM_HEALTH_SEVERITY_NONE;
                candidate->recovery_safe = observation->value == 0.0;
                candidate->immediate = observation->value != 0.0;
            }
        } else if (strcmp(metric, "clock.synchronized") == 0 && !startup_grace) {
            evaluator_candidate_t *candidate = add_candidate(
                candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_CLOCK_UNSYNCHRONIZED, "clock",
                observation);
            if (candidate) {
                candidate->desired = observation->value == 0.0
                    ? SYSTEM_HEALTH_SEVERITY_WARNING
                    : SYSTEM_HEALTH_SEVERITY_NONE;
                candidate->recovery_safe = observation->value != 0.0;
                candidate->immediate = observation->value == 0.0;
            }
        } else if (strcmp(metric, "clock.jump_seconds") == 0) {
            evaluator_candidate_t *candidate = add_candidate(
                candidates, candidate_count, SYSTEM_HEALTH_CONDITION_CLOCK_JUMP,
                "clock", observation);
            if (candidate) {
                candidate->desired = observation->value < -2.0
                    ? SYSTEM_HEALTH_SEVERITY_CRITICAL
                    : SYSTEM_HEALTH_SEVERITY_NONE;
                candidate->recovery_safe = observation->value >= -2.0;
                candidate->immediate = candidate->desired !=
                                       SYSTEM_HEALTH_SEVERITY_NONE;
            }
        } else if (strcmp(metric, "storage.device.prefail") == 0) {
            evaluator_candidate_t *candidate = add_candidate(
                candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_PREFAIL,
                observation->resource_id, observation);
            merge_candidate_evidence(
                candidate, observation,
                observation->value != 0.0 ? SYSTEM_HEALTH_SEVERITY_WARNING
                                          : SYSTEM_HEALTH_SEVERITY_NONE,
                observation->value == 0.0, observation->value != 0.0, false);
        } else if (strcmp(metric, "storage.device.critical") == 0) {
            evaluator_candidate_t *candidate = add_candidate(
                candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_CRITICAL,
                observation->resource_id, observation);
            merge_candidate_evidence(
                candidate, observation,
                observation->value != 0.0 ? SYSTEM_HEALTH_SEVERITY_CRITICAL
                                          : SYSTEM_HEALTH_SEVERITY_NONE,
                observation->value == 0.0, observation->value != 0.0, false);
        } else if (strcmp(metric, "storage.device.pre_eol") == 0) {
            evaluator_candidate_t *prefail = add_candidate(
                candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_PREFAIL,
                observation->resource_id, observation);
            merge_candidate_evidence(
                prefail, observation,
                observation->value >= 2.0 ? SYSTEM_HEALTH_SEVERITY_WARNING
                                          : SYSTEM_HEALTH_SEVERITY_NONE,
                observation->value < 2.0, observation->value >= 2.0, false);
            evaluator_candidate_t *critical = add_candidate(
                candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_CRITICAL,
                observation->resource_id, observation);
            merge_candidate_evidence(
                critical, observation,
                observation->value >= 3.0 ? SYSTEM_HEALTH_SEVERITY_CRITICAL
                                          : SYSTEM_HEALTH_SEVERITY_NONE,
                observation->value < 3.0, observation->value >= 3.0, false);
        } else if (strcmp(metric, "storage.device.life_used_ratio") == 0 ||
                   strcmp(metric,
                          "storage.device.percentage_used_ratio") == 0) {
            evaluator_candidate_t *candidate = add_candidate(
                candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_PREFAIL,
                observation->resource_id, observation);
            system_health_severity_t desired = observation->value >= 0.95
                ? SYSTEM_HEALTH_SEVERITY_CRITICAL
                : observation->value >= 0.80
                    ? SYSTEM_HEALTH_SEVERITY_WARNING
                    : SYSTEM_HEALTH_SEVERITY_NONE;
            merge_candidate_evidence(candidate, observation, desired,
                                     observation->value < 0.75, false, false);
        } else if (strcmp(metric,
                          "storage.device.available_spare_ratio") == 0) {
            evaluator_candidate_t *candidate = add_candidate(
                candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_PREFAIL,
                observation->resource_id, observation);
            system_health_severity_t desired = observation->value <= 0.05
                ? SYSTEM_HEALTH_SEVERITY_CRITICAL
                : observation->value <= 0.10
                    ? SYSTEM_HEALTH_SEVERITY_WARNING
                    : SYSTEM_HEALTH_SEVERITY_NONE;
            merge_candidate_evidence(candidate, observation, desired,
                                     observation->value > 0.15, false, false);
        } else if (strcmp(metric,
                          "storage.device.media_errors_delta") == 0 &&
                   observation->value > 0.0) {
            evaluator_candidate_t *candidate = add_candidate(
                candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_CRITICAL,
                observation->resource_id, observation);
            merge_candidate_evidence(
                candidate, observation, SYSTEM_HEALTH_SEVERITY_CRITICAL,
                false, true, true);
        } else if (strcmp(metric, "hardware.ecc.corrected_delta") == 0 &&
                   observation->value > 0.0) {
            evaluator_candidate_t *candidate = add_candidate(
                candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_CORRECTED,
                observation->resource_id, observation);
            merge_candidate_evidence(
                candidate, observation, SYSTEM_HEALTH_SEVERITY_WARNING,
                false, false, true);
        } else if (strcmp(metric,
                          "hardware.ecc.uncorrectable_delta") == 0 &&
                   observation->value > 0.0) {
            evaluator_candidate_t *candidate = add_candidate(
                candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_UNCORRECTABLE,
                observation->resource_id, observation);
            merge_candidate_evidence(
                candidate, observation, SYSTEM_HEALTH_SEVERITY_CRITICAL,
                false, true, true);
        } else if (strcmp(metric, "hardware.fan.failed") == 0) {
            const system_health_observation_t *hot = find_observation(
                snapshot, "hardware.fan.hot", observation->resource_id);
            bool hot_now = observation_available(hot) && hot->value != 0.0;
            evaluator_candidate_t *candidate = add_candidate(
                candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_HARDWARE_FAN_FAILED,
                observation->resource_id, observation);
            system_health_severity_t desired = observation->value == 0.0
                ? SYSTEM_HEALTH_SEVERITY_NONE
                : hot_now ? SYSTEM_HEALTH_SEVERITY_CRITICAL
                          : SYSTEM_HEALTH_SEVERITY_WARNING;
            merge_candidate_evidence(
                candidate, observation, desired, observation->value == 0.0,
                hot_now && observation->value != 0.0, false);
        } else if (strcmp(metric, "hardware.power.unstable") == 0 ||
                   strcmp(metric, "hardware.throttled") == 0) {
            bool unstable = strcmp(metric, "hardware.power.unstable") == 0;
            evaluator_candidate_t *candidate = add_candidate(
                candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_HARDWARE_POWER_UNSTABLE,
                observation->resource_id, observation);
            system_health_severity_t desired = observation->value == 0.0
                ? SYSTEM_HEALTH_SEVERITY_NONE
                : unstable ? SYSTEM_HEALTH_SEVERITY_CRITICAL
                           : SYSTEM_HEALTH_SEVERITY_WARNING;
            merge_candidate_evidence(
                candidate, observation, desired, observation->value == 0.0,
                unstable && observation->value != 0.0, false);
        } else if (strcmp(metric, "kernel.machine_check_delta") == 0 &&
                   observation->value > 0.0) {
            evaluator_candidate_t *candidate = add_candidate(
                candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_UNCORRECTABLE,
                observation->resource_id, observation);
            if (candidate) {
                candidate->desired = SYSTEM_HEALTH_SEVERITY_CRITICAL;
                candidate->immediate = true;
                candidate->one_shot = true;
            }
        } else if (strcmp(metric, "kernel.block_io_error_delta") == 0 &&
                   observation->value > 0.0) {
            evaluator_candidate_t *candidate = add_candidate(
                candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_CRITICAL,
                observation->resource_id, observation);
            if (candidate) {
                candidate->desired = SYSTEM_HEALTH_SEVERITY_CRITICAL;
                candidate->immediate = true;
                candidate->one_shot = true;
            }
        } else if (strcmp(metric, "kernel.filesystem_remount_delta") == 0 &&
                   observation->value > 0.0) {
            evaluator_candidate_t *candidate = add_candidate(
                candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_FILESYSTEM_READ_ONLY,
                observation->resource_id, observation);
            if (candidate) {
                candidate->desired = SYSTEM_HEALTH_SEVERITY_CRITICAL;
                candidate->immediate = true;
                candidate->one_shot = true;
            }
        } else if (strcmp(metric, "kernel.thermal_shutdown_delta") == 0 &&
                   observation->value > 0.0) {
            evaluator_candidate_t *candidate = add_candidate(
                candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_THERMAL_HIGH,
                observation->resource_id, observation);
            if (candidate) {
                candidate->desired = SYSTEM_HEALTH_SEVERITY_CRITICAL;
                candidate->immediate = true;
                candidate->one_shot = true;
            }
        } else if (strcmp(metric, "kernel.oom_kill_delta") == 0 &&
                   observation->value > 0.0) {
            evaluator_candidate_t *candidate = add_candidate(
                candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_MEMORY_OOM_KILL,
                observation->resource_id, observation);
            if (candidate) {
                candidate->desired = SYSTEM_HEALTH_SEVERITY_CRITICAL;
                candidate->immediate = true;
                candidate->one_shot = true;
            }
        }
    }

    for (size_t index = 0; index < snapshot->observation_count; ++index) {
        const system_health_observation_t *available = &snapshot->observations[index];
        if (strcmp(available->metric, "filesystem.available_bytes") == 0) {
            const system_health_observation_t *capacity = find_observation(
                snapshot, "filesystem.capacity_bytes", available->resource_id);
            system_health_observation_t ratio = ratio_observation(
                available, capacity, "filesystem.available_bytes_ratio");
            (void)candidate_from_observation(
                evaluator, candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_FILESYSTEM_BYTES_LOW,
                available->resource_id, &ratio);
        } else if (strcmp(available->metric,
                          "filesystem.available_inodes") == 0) {
            const system_health_observation_t *capacity = find_observation(
                snapshot, "filesystem.capacity_inodes", available->resource_id);
            system_health_observation_t ratio = ratio_observation(
                available, capacity, "filesystem.available_inodes_ratio");
            (void)candidate_from_observation(
                evaluator, candidates, candidate_count,
                SYSTEM_HEALTH_CONDITION_FILESYSTEM_INODES_LOW,
                available->resource_id, &ratio);
        } else if (strcmp(available->metric, "network.carrier") == 0) {
            const system_health_observation_t *primary = find_observation(
                snapshot, "network.primary", available->resource_id);
            if (observation_available(primary) && primary->value != 0.0 &&
                observation_available(available)) {
                evaluator_candidate_t *candidate = add_candidate(
                    candidates, candidate_count,
                    SYSTEM_HEALTH_CONDITION_NETWORK_LINK_DOWN,
                    available->resource_id, available);
                if (candidate) {
                    candidate->desired = available->value == 0.0
                        ? (context && context->recording_expected_known &&
                           context->recording_expected
                               ? SYSTEM_HEALTH_SEVERITY_CRITICAL
                               : SYSTEM_HEALTH_SEVERITY_WARNING)
                        : SYSTEM_HEALTH_SEVERITY_NONE;
                    candidate->recovery_safe = available->value != 0.0;
                }
            }
        } else if (strcmp(available->metric,
                          "thermal.temperature_celsius") == 0) {
            const system_health_observation_t *critical = find_observation(
                snapshot, "thermal.critical_celsius", available->resource_id);
            if (observation_available(available) &&
                observation_available(critical)) {
                evaluator_candidate_t *candidate = add_candidate(
                    candidates, candidate_count,
                    SYSTEM_HEALTH_CONDITION_THERMAL_HIGH,
                    available->resource_id, available);
                if (candidate) {
                    candidate->desired = available->value >= critical->value
                        ? SYSTEM_HEALTH_SEVERITY_CRITICAL
                        : available->value >= critical->value - 10.0
                            ? SYSTEM_HEALTH_SEVERITY_WARNING
                            : SYSTEM_HEALTH_SEVERITY_NONE;
                    candidate->recovery_safe =
                        available->value < critical->value - 15.0;
                    candidate->immediate = candidate->desired ==
                                           SYSTEM_HEALTH_SEVERITY_CRITICAL;
                }
            }
        }
    }
}

static int64_t clamped_wall(evaluator_slot_t *slot, int64_t wall) {
    if (wall <= 0) wall = 1;
    if (wall < slot->last_persisted_wall_ms) wall = slot->last_persisted_wall_ms;
    return wall;
}

static void signal_from_candidate(
    const system_health_evaluator_t *evaluator, const evaluator_slot_t *slot,
    const evaluator_candidate_t *candidate,
    system_health_incident_action_t action, system_health_state_t target_state,
    system_health_severity_t target_severity, int64_t wall,
    system_health_incident_signal_t *signal) {
    memset(signal, 0, sizeof(*signal));
    signal->condition = slot->condition;
    copy_string(signal->subject, sizeof(signal->subject), slot->subject);
    signal->scope = slot->scope;
    signal->state = target_state;
    signal->severity = action == SYSTEM_HEALTH_INCIDENT_RECOVER
                           ? SYSTEM_HEALTH_SEVERITY_NONE
                           : target_severity;
    signal->observed_at_ms = wall;
    signal->reconciliation = SYSTEM_HEALTH_RECONCILIATION_NONE;
    copy_string(signal->boot_id, sizeof(signal->boot_id),
                evaluator->config.boot_id);
    copy_string(signal->run_id, sizeof(signal->run_id), evaluator->config.run_id);
    if (candidate->observation.value_valid) {
        snprintf(signal->observation_json, sizeof(signal->observation_json),
                 "{\"metric\":\"%s\",\"resource\":\"%s\","
                 "\"value\":%.12g,\"unit\":\"%s\"}",
                 candidate->observation.metric, slot->subject,
                 candidate->observation.value,
                 system_health_unit_name(candidate->observation.unit));
    } else {
        snprintf(signal->observation_json, sizeof(signal->observation_json),
                 "{\"metric\":\"%s\",\"resource\":\"%s\","
                 "\"capability\":\"%s\"}",
                 candidate->observation.metric, slot->subject,
                 system_health_capability_name(
                     candidate->observation.capability));
    }
}

static void fill_transition(
    const system_health_evaluator_t *evaluator, const evaluator_slot_t *slot,
    const evaluator_candidate_t *candidate,
    system_health_incident_action_t action, system_health_state_t previous_state,
    system_health_severity_t previous_severity,
    system_health_state_t target_state,
    system_health_severity_t target_severity, const char *event_id,
    int64_t wall, bool persisted,
    system_health_transition_t *transition) {
    memset(transition, 0, sizeof(*transition));
    transition->action = action;
    copy_string(transition->incident_id, sizeof(transition->incident_id),
                slot->incident_id);
    copy_string(transition->event_id, sizeof(transition->event_id), event_id);
    transition->condition = slot->condition;
    copy_string(transition->subject, sizeof(transition->subject), slot->subject);
    transition->scope = slot->scope;
    transition->previous_state = previous_state;
    transition->state = target_state;
    transition->previous_severity = previous_severity;
    transition->severity = target_severity;
    transition->observation = candidate->observation;
    const system_health_condition_policy_t *rule =
        &evaluator->config.policy.conditions[slot->condition];
    transition->threshold_direction = rule->direction;
    transition->threshold_value = target_severity ==
                                           SYSTEM_HEALTH_SEVERITY_CRITICAL
                                       ? rule->critical_threshold
                                       : rule->warning_threshold;
    uint32_t dwell_seconds = target_severity ==
                                     SYSTEM_HEALTH_SEVERITY_CRITICAL
                                 ? rule->critical_for_seconds
                                 : rule->warning_for_seconds;
    if (action == SYSTEM_HEALTH_INCIDENT_RECOVER) {
        transition->threshold_value = rule->recovery_threshold;
        dwell_seconds = rule->recovery_for_seconds;
    }
    transition->threshold_for_ms = dwell_seconds * 1000U;
    transition->first_observed_at_ms = slot->first_observed_wall_ms;
    transition->incident_duration_ms =
        slot->first_observed_monotonic_ms > 0U &&
                candidate->observation.sampled_monotonic_ms >=
                    slot->first_observed_monotonic_ms
            ? candidate->observation.sampled_monotonic_ms -
                  slot->first_observed_monotonic_ms
            : 0U;
    transition->observed_at_ms = wall;
    transition->persisted = persisted;
}

static bool persistence_succeeded(db_system_health_result_t result) {
    return result == DB_SYSTEM_HEALTH_OK || result == DB_SYSTEM_HEALTH_RESUMED;
}

static void deliver_transition(
    system_health_evaluator_t *evaluator, evaluator_slot_t *slot,
    const evaluator_candidate_t *candidate,
    system_health_incident_action_t action, system_health_state_t previous_state,
    system_health_severity_t previous_severity,
    system_health_state_t target_state,
    system_health_severity_t target_severity, const char *event_id,
    int64_t wall, bool persisted) {
    if (!evaluator->config.transition_sink) return;
    system_health_transition_t transition;
    fill_transition(evaluator, slot, candidate, action, previous_state,
                    previous_severity, target_state, target_severity, event_id, wall,
                    persisted, &transition);
    evaluator->config.transition_sink(&transition,
                                      evaluator->config.transition_sink_context);
}

static bool emit_action(
    system_health_evaluator_t *evaluator, evaluator_slot_t *slot,
    const evaluator_candidate_t *candidate,
    system_health_incident_action_t action, system_health_state_t previous_state,
    system_health_severity_t previous_severity,
    system_health_state_t target_state,
    system_health_severity_t target_severity, uint64_t now, int64_t wall) {
    wall = clamped_wall(slot, wall);
    char event_id[LIGHTNVR_UUID_STRING_SIZE] = {0};
    if (evaluator->config.transition_sink &&
        lightnvr_uuid_generate_v4(event_id) != 0) return false;
    if (!evaluator->config.persist) {
        if (!slot->incident_id[0])
            (void)lightnvr_uuid_generate_v4(slot->incident_id);
        slot->last_persisted_wall_ms = wall;
        evaluator->stats.transitions++;
        deliver_transition(evaluator, slot, candidate, action, previous_state,
                           previous_severity, target_state, target_severity,
                           event_id, wall, false);
        return true;
    }

    system_health_incident_signal_t signal;
    signal_from_candidate(evaluator, slot, candidate, action, target_state,
                          target_severity, wall, &signal);
    if (event_id[0]) {
        copy_string(signal.event_id, sizeof(signal.event_id), event_id);
        signal.reconciliation = action == SYSTEM_HEALTH_INCIDENT_RECOVER
            ? SYSTEM_HEALTH_RECONCILIATION_RECOVERY_PENDING
            : SYSTEM_HEALTH_RECONCILIATION_ALERT_PENDING;
    }
    system_health_incident_record_t incident;
    db_system_health_result_t result = evaluator->config.persist(
        action, &signal, &incident, evaluator->config.persist_context);
    if (persistence_succeeded(result)) {
        copy_string(slot->incident_id, sizeof(slot->incident_id), incident.uuid);
        slot->last_persisted_wall_ms = incident.last_seen_at_ms;
        evaluator->stats.transitions++;
        deliver_transition(evaluator, slot, candidate, action, previous_state,
                           previous_severity, target_state, target_severity,
                           event_id, wall, true);
        return true;
    }

    slot->persistence_pending = true;
    slot->pending_action = action;
    slot->pending_previous_state = previous_state;
    slot->pending_previous_severity = previous_severity;
    copy_string(slot->pending_event_id, sizeof(slot->pending_event_id),
                event_id);
    slot->pending_target_state = target_state;
    slot->pending_target_severity = target_severity;
    slot->pending_candidate = *candidate;
    slot->pending_wall_ms = wall;
    slot->retry_delay_ms = evaluator->config.retry_initial_ms;
    slot->next_retry_ms = now + slot->retry_delay_ms;
    evaluator->stats.persistence_failures++;
    evaluator->stats.pending_persistence++;
    return false;
}

static void retry_pending(system_health_evaluator_t *evaluator,
                          evaluator_slot_t *slot, uint64_t now) {
    if (!slot->persistence_pending || now < slot->next_retry_ms ||
        !evaluator->config.persist) return;
    system_health_incident_signal_t signal;
    signal_from_candidate(evaluator, slot, &slot->pending_candidate,
                          slot->pending_action, slot->pending_target_state,
                          slot->pending_target_severity,
                          clamped_wall(slot, slot->pending_wall_ms), &signal);
    if (slot->pending_event_id[0]) {
        copy_string(signal.event_id, sizeof(signal.event_id),
                    slot->pending_event_id);
        signal.reconciliation =
            slot->pending_action == SYSTEM_HEALTH_INCIDENT_RECOVER
                ? SYSTEM_HEALTH_RECONCILIATION_RECOVERY_PENDING
                : SYSTEM_HEALTH_RECONCILIATION_ALERT_PENDING;
    }
    system_health_incident_record_t incident;
    db_system_health_result_t result = evaluator->config.persist(
        slot->pending_action, &signal, &incident,
        evaluator->config.persist_context);
    evaluator->stats.persistence_retries++;
    if (persistence_succeeded(result)) {
        copy_string(slot->incident_id, sizeof(slot->incident_id), incident.uuid);
        slot->last_persisted_wall_ms = incident.last_seen_at_ms;
        deliver_transition(evaluator, slot, &slot->pending_candidate,
                           slot->pending_action, slot->pending_previous_state,
                           slot->pending_previous_severity,
                           slot->pending_target_state,
                           slot->pending_target_severity,
                           slot->pending_event_id,
                           signal.observed_at_ms, true);
        slot->persistence_pending = false;
        slot->pending_event_id[0] = '\0';
        evaluator->stats.pending_persistence--;
        evaluator->stats.transitions++;
        return;
    }
    evaluator->stats.persistence_failures++;
    uint64_t doubled = (uint64_t)slot->retry_delay_ms * 2U;
    slot->retry_delay_ms = doubled > evaluator->config.retry_max_ms
        ? evaluator->config.retry_max_ms : (uint32_t)doubled;
    slot->next_retry_ms = now + slot->retry_delay_ms;
}

static bool material_change(const system_health_evaluator_t *evaluator,
                            const evaluator_slot_t *slot,
                            const evaluator_candidate_t *candidate,
                            uint64_t now) {
    if (!candidate->observation.value_valid ||
        !slot->last_material_value_valid ||
        now - slot->last_material_ms <
            evaluator->config.material_change_debounce_ms) return false;
    double scale = fabs(slot->last_material_value);
    if (scale < 1e-9) scale = 1.0;
    return fabs(candidate->observation.value - slot->last_material_value) /
               scale >= evaluator->config.material_change_ratio;
}

static uint64_t severity_dwell_ms(
    const system_health_condition_policy_t *rule,
    system_health_severity_t severity) {
    uint32_t seconds = severity == SYSTEM_HEALTH_SEVERITY_CRITICAL
                           ? rule->critical_for_seconds
                           : rule->warning_for_seconds;
    return (uint64_t)seconds * 1000U;
}

static void evaluate_candidate(system_health_evaluator_t *evaluator,
                               const evaluator_candidate_t *candidate,
                               uint64_t now, int64_t wall) {
    if (candidate->one_shot) {
        if (candidate->desired != SYSTEM_HEALTH_SEVERITY_NONE)
            (void)system_health_evaluator_one_shot(
                evaluator, candidate->condition, candidate->subject,
                candidate->scope, candidate->desired,
                &candidate->observation, wall);
        return;
    }
    evaluator_slot_t *slot = find_slot(evaluator, candidate->condition,
                                       candidate->subject, true);
    if (!slot) return;
    slot->scope = candidate->scope;
    slot->last_observation = candidate->observation;
    slot->last_observed_wall_ms = wall;
    retry_pending(evaluator, slot, now);
    if (slot->persistence_pending) return; /* Freeze transitions until durable. */

    const system_health_condition_policy_t *rule =
        &evaluator->config.policy.conditions[candidate->condition];
    if (!rule->enabled) return;
    if (slot->state == SYSTEM_HEALTH_STATE_UNKNOWN) {
        if (candidate->desired == SYSTEM_HEALTH_SEVERITY_NONE) {
            if (candidate->recovery_safe) slot->state = SYSTEM_HEALTH_STATE_HEALTHY;
            return;
        }
        slot->state = SYSTEM_HEALTH_STATE_PENDING;
        slot->pending_severity = candidate->desired;
        slot->pending_since_ms = now;
    }

    if (slot->state == SYSTEM_HEALTH_STATE_HEALTHY ||
        slot->state == SYSTEM_HEALTH_STATE_PENDING) {
        if (candidate->desired == SYSTEM_HEALTH_SEVERITY_NONE) {
            slot->state = SYSTEM_HEALTH_STATE_HEALTHY;
            slot->pending_since_ms = 0U;
            slot->pending_severity = SYSTEM_HEALTH_SEVERITY_NONE;
            return;
        }
        if (slot->state != SYSTEM_HEALTH_STATE_PENDING ||
            slot->pending_severity != candidate->desired) {
            slot->state = SYSTEM_HEALTH_STATE_PENDING;
            slot->pending_severity = candidate->desired;
            slot->pending_since_ms = now;
        }
        uint64_t dwell = severity_dwell_ms(rule, candidate->desired);
        if (!candidate->immediate && now - slot->pending_since_ms < dwell)
            return;
        system_health_state_t previous_state = slot->state;
        slot->state = SYSTEM_HEALTH_STATE_OPEN;
        slot->severity = candidate->desired;
        slot->first_observed_monotonic_ms = slot->pending_since_ms;
        slot->first_observed_wall_ms = wall -
            (int64_t)(now - slot->pending_since_ms);
        slot->last_material_ms = now;
        slot->last_material_value = candidate->observation.value;
        slot->last_material_value_valid = candidate->observation.value_valid;
        evaluator->stats.active_incidents++;
        (void)emit_action(evaluator, slot, candidate,
                          SYSTEM_HEALTH_INCIDENT_OPEN, previous_state,
                          SYSTEM_HEALTH_SEVERITY_NONE,
                          SYSTEM_HEALTH_STATE_OPEN, slot->severity, now, wall);
        return;
    }

    if (slot->state == SYSTEM_HEALTH_STATE_RECOVERING) {
        if (candidate->desired != SYSTEM_HEALTH_SEVERITY_NONE) {
            system_health_state_t previous_state = slot->state;
            slot->state = SYSTEM_HEALTH_STATE_OPEN;
            slot->recovery_since_ms = 0U;
            (void)emit_action(evaluator, slot, candidate,
                              SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE,
                              previous_state, slot->severity,
                              SYSTEM_HEALTH_STATE_OPEN, slot->severity, now,
                              wall);
            return;
        }
        if (!candidate->recovery_safe) return;
        uint64_t dwell = (uint64_t)rule->recovery_for_seconds * 1000U;
        if (now - slot->recovery_since_ms < dwell) return;
        system_health_severity_t previous_severity = slot->severity;
        slot->state = SYSTEM_HEALTH_STATE_CLOSED;
        slot->severity = SYSTEM_HEALTH_SEVERITY_NONE;
        if (evaluator->stats.active_incidents > 0U)
            evaluator->stats.active_incidents--;
        (void)emit_action(evaluator, slot, candidate,
                          SYSTEM_HEALTH_INCIDENT_RECOVER,
                          SYSTEM_HEALTH_STATE_RECOVERING, previous_severity,
                          SYSTEM_HEALTH_STATE_CLOSED, previous_severity, now,
                          wall);
        return;
    }

    if (slot->state != SYSTEM_HEALTH_STATE_OPEN) return;
    if (candidate->desired > slot->severity) {
        if (slot->pending_severity != candidate->desired) {
            slot->pending_severity = candidate->desired;
            slot->pending_since_ms = now;
        }
        uint64_t dwell = severity_dwell_ms(rule, candidate->desired);
        if (!candidate->immediate && now - slot->pending_since_ms < dwell)
            return;
        system_health_severity_t previous = slot->severity;
        slot->severity = candidate->desired;
        slot->pending_severity = SYSTEM_HEALTH_SEVERITY_NONE;
        slot->last_material_ms = now;
        (void)emit_action(evaluator, slot, candidate,
                          SYSTEM_HEALTH_INCIDENT_ESCALATE,
                          SYSTEM_HEALTH_STATE_OPEN, previous,
                          SYSTEM_HEALTH_STATE_OPEN, slot->severity, now, wall);
        return;
    }
    if (candidate->recovery_safe) {
        system_health_state_t previous_state = slot->state;
        slot->state = SYSTEM_HEALTH_STATE_RECOVERING;
        slot->recovery_since_ms = now;
        (void)emit_action(evaluator, slot, candidate,
                          SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE,
                          previous_state, slot->severity,
                          SYSTEM_HEALTH_STATE_RECOVERING, slot->severity, now,
                          wall);
        return;
    }
    if (candidate->desired == slot->severity &&
        material_change(evaluator, slot, candidate, now)) {
        slot->last_material_ms = now;
        slot->last_material_value = candidate->observation.value;
        slot->last_material_value_valid = true;
        (void)emit_action(evaluator, slot, candidate,
                          SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE,
                          SYSTEM_HEALTH_STATE_OPEN, slot->severity,
                          SYSTEM_HEALTH_STATE_OPEN, slot->severity, now, wall);
    }
}

static void add_immediate_candidates(
    evaluator_candidate_t candidates[EVALUATOR_CANDIDATE_MAX],
    size_t *candidate_count, uint64_t monotonic, int64_t wall) {
    for (int condition_index = 0;
         condition_index < SYSTEM_HEALTH_CONDITION_COUNT; ++condition_index) {
        for (int resource_index = 0;
             resource_index < SYSTEM_HEALTH_IMMEDIATE_RESOURCE_COUNT;
             ++resource_index) {
            uint64_t count = atomic_exchange(
                &immediate_counts[condition_index][resource_index], 0U);
            if (count == 0U) continue;
            const char *subject = resource_index == SYSTEM_HEALTH_IMMEDIATE_PROCESS
                ? "process" : resource_index == SYSTEM_HEALTH_IMMEDIATE_ROOT
                    ? "root" : "recording";
            system_health_observation_t observation;
            memset(&observation, 0, sizeof(observation));
            copy_string(observation.metric, sizeof(observation.metric),
                        system_health_condition_code(
                            (system_health_condition_t)condition_index));
            copy_string(observation.resource_id,
                        sizeof(observation.resource_id), subject);
            observation.scope = resource_index == SYSTEM_HEALTH_IMMEDIATE_PROCESS
                ? SYSTEM_HEALTH_SCOPE_PROCESS : SYSTEM_HEALTH_SCOPE_FILESYSTEM;
            observation.sampled_monotonic_ms = monotonic;
            observation.observed_wall_time_ms = wall;
            system_health_observation_set_available(
                &observation, (double)count, SYSTEM_HEALTH_UNIT_COUNT);
            evaluator_candidate_t *candidate = add_candidate(
                candidates, candidate_count,
                (system_health_condition_t)condition_index, subject,
                &observation);
            if (!candidate) continue;
            candidate->desired = SYSTEM_HEALTH_SEVERITY_CRITICAL;
            candidate->immediate = true;
            candidate->one_shot = condition_index ==
                                      SYSTEM_HEALTH_CONDITION_MEMORY_OOM_KILL ||
                                  condition_index ==
                                      SYSTEM_HEALTH_CONDITION_PROCESS_ALLOCATION_FAILED;
        }
    }
}

static void publish_views(system_health_evaluator_t *evaluator) {
    unsigned int destination = evaluator->active_view == 0U ? 1U : 0U;
    size_t count = 0U;
    int64_t oldest_pending_wall_time_ms = 0;
    for (size_t index = 0; index < SYSTEM_HEALTH_MAX_INCIDENTS; ++index) {
        const evaluator_slot_t *slot = &evaluator->slots[index];
        if (slot->used && slot->persistence_pending &&
            slot->pending_wall_ms > 0 &&
            (oldest_pending_wall_time_ms == 0 ||
             slot->pending_wall_ms < oldest_pending_wall_time_ms)) {
            oldest_pending_wall_time_ms = slot->pending_wall_ms;
        }
        if (!slot->used ||
            (slot->state != SYSTEM_HEALTH_STATE_OPEN &&
             slot->state != SYSTEM_HEALTH_STATE_RECOVERING &&
             !slot->persistence_pending)) continue;
        system_health_incident_view_t *view =
            &evaluator->views[destination][count++];
        memset(view, 0, sizeof(*view));
        copy_string(view->incident_id, sizeof(view->incident_id),
                    slot->incident_id);
        view->condition = slot->condition;
        copy_string(view->subject, sizeof(view->subject), slot->subject);
        view->scope = slot->scope;
        view->state = slot->state;
        view->severity = slot->severity;
        view->first_observed_at_ms = slot->first_observed_wall_ms;
        view->last_observed_at_ms = slot->last_observed_wall_ms;
        view->observation = slot->last_observation;
        view->persistence_pending = slot->persistence_pending;
    }
    pthread_mutex_lock(&evaluator->view_lock);
    evaluator->view_counts[destination] = count;
    evaluator->stats.oldest_pending_wall_time_ms =
        oldest_pending_wall_time_ms;
    evaluator->published_stats = evaluator->stats;
    evaluator->active_view = destination;
    pthread_mutex_unlock(&evaluator->view_lock);
}

int system_health_evaluator_one_shot(
    system_health_evaluator_t *evaluator, system_health_condition_t condition,
    const char *subject, system_health_scope_t scope,
    system_health_severity_t severity,
    const system_health_observation_t *observation, int64_t observed_at_ms) {
    if (!evaluator || !subject || !subject[0] || !observation ||
        condition < 0 || condition >= SYSTEM_HEALTH_CONDITION_COUNT ||
        severity < SYSTEM_HEALTH_SEVERITY_WARNING ||
        severity > SYSTEM_HEALTH_SEVERITY_CRITICAL) return -1;
    evaluator_slot_t *slot = find_slot(evaluator, condition, subject, true);
    if (!slot || slot->persistence_pending) return -1;
    if (slot->last_episode_sample_ms == observation->sampled_monotonic_ms)
        return 0;
    slot->last_episode_sample_ms = observation->sampled_monotonic_ms;
    slot->scope = scope;
    slot->state = SYSTEM_HEALTH_STATE_CLOSED;
    slot->severity = severity;
    slot->first_observed_wall_ms = observed_at_ms;
    slot->first_observed_monotonic_ms = observation->sampled_monotonic_ms;
    slot->last_observed_wall_ms = observed_at_ms;
    slot->last_observation = *observation;
    evaluator_candidate_t candidate;
    memset(&candidate, 0, sizeof(candidate));
    candidate.condition = condition;
    candidate.scope = scope;
    candidate.observation = *observation;
    candidate.desired = severity;
    candidate.one_shot = true;
    copy_string(candidate.subject, sizeof(candidate.subject), subject);
    bool result = emit_action(
        evaluator, slot, &candidate, SYSTEM_HEALTH_INCIDENT_ONE_SHOT,
        SYSTEM_HEALTH_STATE_UNKNOWN, SYSTEM_HEALTH_SEVERITY_NONE,
        SYSTEM_HEALTH_STATE_CLOSED, severity,
        observation->sampled_monotonic_ms, observed_at_ms);
    publish_views(evaluator);
    return result ? 0 : -1;
}

int system_health_evaluator_evaluate(
    system_health_evaluator_t *evaluator,
    const system_health_snapshot_t *snapshot,
    const system_health_evaluation_context_t *context) {
    if (!evaluator || !snapshot || snapshot->sequence == 0U ||
        snapshot->completed_monotonic_ms == 0U ||
        snapshot->completed_wall_time_ms <= 0) return -1;
    if (snapshot->sequence <= evaluator->last_snapshot_sequence) return 0;
    evaluator->last_snapshot_sequence = snapshot->sequence;

    for (size_t index = 0; index < SYSTEM_HEALTH_MAX_INCIDENTS; ++index) {
        if (evaluator->slots[index].used)
            retry_pending(evaluator, &evaluator->slots[index],
                          snapshot->completed_monotonic_ms);
    }
    evaluator_candidate_t candidates[EVALUATOR_CANDIDATE_MAX];
    size_t count;
    build_candidates(evaluator, snapshot, context, candidates, &count);
    add_immediate_candidates(candidates, &count,
                             snapshot->completed_monotonic_ms,
                             snapshot->completed_wall_time_ms);
    for (size_t index = 0; index < count; ++index)
        evaluate_candidate(evaluator, &candidates[index],
                           snapshot->completed_monotonic_ms,
                           snapshot->completed_wall_time_ms);
    publish_views(evaluator);
    return 0;
}

void system_health_evaluator_get_stats(
    const system_health_evaluator_t *evaluator,
    system_health_evaluator_stats_t *stats) {
    if (!stats) return;
    memset(stats, 0, sizeof(*stats));
    if (!evaluator) return;
    pthread_mutex_lock((pthread_mutex_t *)&evaluator->view_lock);
    *stats = evaluator->published_stats;
    pthread_mutex_unlock((pthread_mutex_t *)&evaluator->view_lock);
}

size_t system_health_evaluator_active_copy(
    const system_health_evaluator_t *evaluator,
    system_health_incident_view_t *incidents, size_t capacity) {
    if (!evaluator || !incidents || capacity == 0U) return 0U;
    pthread_mutex_lock((pthread_mutex_t *)&evaluator->view_lock);
    size_t count = evaluator->view_counts[evaluator->active_view];
    if (count > capacity) count = capacity;
    memcpy(incidents, evaluator->views[evaluator->active_view],
           count * sizeof(*incidents));
    pthread_mutex_unlock((pthread_mutex_t *)&evaluator->view_lock);
    return count;
}

void system_health_evaluator_note_immediate(
    system_health_condition_t condition,
    system_health_immediate_resource_t resource) {
    if (condition < 0 || condition >= SYSTEM_HEALTH_CONDITION_COUNT ||
        resource < 0 || resource >= SYSTEM_HEALTH_IMMEDIATE_RESOURCE_COUNT)
        return;
    atomic_fetch_add(&immediate_counts[condition][resource], 1U);
}

system_health_restart_classification_t system_health_classify_previous_run(
    const system_health_process_run_t *previous, bool had_previous,
    const char *current_boot_id) {
    if (!had_previous || !previous || previous->clean_close ||
        !current_boot_id || !current_boot_id[0]) return SYSTEM_HEALTH_RESTART_NONE;
    return strcmp(previous->boot_id, current_boot_id) == 0
        ? SYSTEM_HEALTH_RESTART_PROCESS : SYSTEM_HEALTH_RESTART_HOST;
}

typedef struct {
    pthread_mutex_t lock;
    pthread_cond_t wake;
    pthread_t thread;
    bool initialized;
    bool running;
    bool disabled;
    system_health_evaluator_t *evaluator;
    system_health_evaluation_context_t context;
    system_health_process_run_t run;
    system_health_transition_sink_fn transition_sink;
    void *transition_sink_context;
} evaluator_service_t;

static evaluator_service_t service = {
    .lock = PTHREAD_MUTEX_INITIALIZER,
    .wake = PTHREAD_COND_INITIALIZER,
};

static db_system_health_result_t persist_to_database(
    system_health_incident_action_t action,
    const system_health_incident_signal_t *signal,
    system_health_incident_record_t *incident_out, void *context) {
    (void)context;
    return db_system_health_incident_apply(action, signal, incident_out);
}

static system_health_unit_t replay_unit(const char *name) {
    if (!name) return SYSTEM_HEALTH_UNIT_NONE;
    for (int unit = SYSTEM_HEALTH_UNIT_NONE;
         unit <= SYSTEM_HEALTH_UNIT_BOOLEAN; ++unit) {
        if (strcmp(name, system_health_unit_name((system_health_unit_t)unit)) == 0)
            return (system_health_unit_t)unit;
    }
    return SYSTEM_HEALTH_UNIT_NONE;
}

static system_health_capability_t replay_capability(const char *name) {
    if (!name) return SYSTEM_HEALTH_CAPABILITY_ERROR;
    for (int capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
         capability < SYSTEM_HEALTH_CAPABILITY_COUNT; ++capability) {
        if (strcmp(name, system_health_capability_name(
                             (system_health_capability_t)capability)) == 0)
            return (system_health_capability_t)capability;
    }
    return SYSTEM_HEALTH_CAPABILITY_ERROR;
}

static bool replay_transition(
    const system_health_policy_t *policy,
    const system_health_incident_transition_t *stored,
    const system_health_incident_record_t *incident,
    system_health_transition_t *transition) {
    if (!policy || !stored || !incident || !transition) return false;
    memset(transition, 0, sizeof(*transition));
    if (!system_health_condition_from_code(incident->condition_code,
                                           &transition->condition))
        return false;
    transition->action = stored->action;
    copy_string(transition->incident_id, sizeof(transition->incident_id),
                stored->incident_uuid);
    copy_string(transition->event_id, sizeof(transition->event_id),
                stored->event_id);
    copy_string(transition->subject, sizeof(transition->subject),
                incident->subject);
    transition->scope = incident->scope;
    transition->previous_state = stored->from_state_valid
        ? stored->from_state : SYSTEM_HEALTH_STATE_UNKNOWN;
    transition->state = stored->to_state;
    transition->severity = stored->severity;
    transition->previous_severity =
        stored->action == SYSTEM_HEALTH_INCIDENT_RECOVER
            ? stored->severity : SYSTEM_HEALTH_SEVERITY_NONE;
    transition->observed_at_ms = stored->observed_at_ms;
    transition->first_observed_at_ms = incident->first_seen_at_ms;
    transition->incident_duration_ms = stored->observed_at_ms >=
                                               incident->first_seen_at_ms
        ? (uint64_t)(stored->observed_at_ms - incident->first_seen_at_ms) : 0U;
    transition->persisted = true;

    const system_health_condition_policy_t *rule =
        &policy->conditions[transition->condition];
    transition->threshold_direction = rule->direction;
    if (stored->action == SYSTEM_HEALTH_INCIDENT_RECOVER) {
        transition->threshold_value = rule->recovery_threshold;
        transition->threshold_for_ms = rule->recovery_for_seconds * 1000U;
    } else if (stored->severity == SYSTEM_HEALTH_SEVERITY_CRITICAL) {
        transition->threshold_value = rule->critical_threshold;
        transition->threshold_for_ms = rule->critical_for_seconds * 1000U;
    } else {
        transition->threshold_value = rule->warning_threshold;
        transition->threshold_for_ms = rule->warning_for_seconds * 1000U;
    }

    cJSON *observation = cJSON_ParseWithOpts(stored->observation_json,
                                             NULL, true);
    if (!cJSON_IsObject(observation)) {
        cJSON_Delete(observation);
        return false;
    }
    const cJSON *metric = cJSON_GetObjectItemCaseSensitive(observation,
                                                           "metric");
    const cJSON *resource = cJSON_GetObjectItemCaseSensitive(observation,
                                                             "resource");
    const cJSON *value = cJSON_GetObjectItemCaseSensitive(observation, "value");
    const cJSON *unit = cJSON_GetObjectItemCaseSensitive(observation, "unit");
    const cJSON *capability = cJSON_GetObjectItemCaseSensitive(
        observation, "capability");
    copy_string(transition->observation.metric,
                sizeof(transition->observation.metric),
                cJSON_IsString(metric) ? metric->valuestring :
                                         incident->condition_code);
    copy_string(transition->observation.resource_id,
                sizeof(transition->observation.resource_id),
                cJSON_IsString(resource) ? resource->valuestring :
                                           incident->subject);
    transition->observation.scope = incident->scope;
    transition->observation.observed_wall_time_ms = stored->observed_at_ms;
    if (cJSON_IsNumber(value) && isfinite(value->valuedouble)) {
        system_health_observation_set_available(
            &transition->observation, value->valuedouble,
            replay_unit(cJSON_IsString(unit) ? unit->valuestring : NULL));
    } else {
        system_health_observation_set_unavailable(
            &transition->observation,
            replay_capability(cJSON_IsString(capability)
                                  ? capability->valuestring : NULL));
    }
    cJSON_Delete(observation);
    return true;
}

static void production_transition_sink(
    const system_health_transition_t *transition, void *context) {
    (void)context;
    char error[256];
    if (event_producer_publish_system_health_transition(
            transition, transition->event_id, error, sizeof(error)) != 0) {
        log_warn("Health transition %s remains pending: %s",
                 transition->event_id, error);
        return;
    }
    if (db_system_health_transition_set_reconciliation(
            transition->incident_id, transition->event_id,
            SYSTEM_HEALTH_RECONCILIATION_RECONCILED) != DB_SYSTEM_HEALTH_OK) {
        log_warn("Health transition %s was enqueued but not reconciled",
                 transition->event_id);
    }
}

static void reconcile_pending_transitions(
    const system_health_policy_t *policy) {
    int64_t after_id = 0;
    for (;;) {
        system_health_incident_transition_t pending[16];
        int count = db_system_health_transition_list_pending(
            after_id, pending, (int)(sizeof(pending) / sizeof(pending[0])));
        if (count < 0) {
            log_warn("Could not enumerate pending health event transitions");
            return;
        }
        for (int index = 0; index < count; ++index) {
            after_id = pending[index].id;
            system_health_incident_record_t incident;
            system_health_transition_t transition;
            if (db_system_health_incident_get_uuid(
                    pending[index].incident_uuid, &incident) !=
                    DB_SYSTEM_HEALTH_OK ||
                !replay_transition(policy, &pending[index], &incident,
                                   &transition)) {
                log_warn("Could not rebuild pending health transition %s",
                         pending[index].transition_uuid);
                continue;
            }
            production_transition_sink(&transition, NULL);
        }
        if (count < (int)(sizeof(pending) / sizeof(pending[0]))) return;
    }
}

static void *evaluator_service_main(void *argument) {
    (void)argument;
    uint64_t last_sequence = 0U;
    uint64_t last_reconcile_ms = 0U;
    for (;;) {
        pthread_mutex_lock(&service.lock);
        bool running = service.running;
        system_health_evaluation_context_t context = service.context;
        pthread_mutex_unlock(&service.lock);
        if (!running) break;

        system_health_snapshot_t snapshot;
        if (system_health_snapshot_copy(&snapshot) &&
            snapshot.sequence > last_sequence) {
            (void)system_health_evaluator_evaluate(service.evaluator, &snapshot,
                                                    &context);
            last_sequence = snapshot.sequence;
        }
        uint64_t now = monotonic_ms();
        if (service.transition_sink == production_transition_sink &&
            now - last_reconcile_ms >= 5000U) {
            reconcile_pending_transitions(&service.evaluator->config.policy);
            last_reconcile_ms = now;
        }

        struct timespec deadline;
        clock_gettime(CLOCK_REALTIME, &deadline);
        deadline.tv_nsec += 250000000L;
        if (deadline.tv_nsec >= 1000000000L) {
            deadline.tv_sec++;
            deadline.tv_nsec -= 1000000000L;
        }
        pthread_mutex_lock(&service.lock);
        if (service.running)
            (void)pthread_cond_timedwait(&service.wake, &service.lock,
                                         &deadline);
        pthread_mutex_unlock(&service.lock);
    }
    return NULL;
}

int system_health_evaluator_service_start(void) {
    pthread_mutex_lock(&service.lock);
    if (service.initialized) {
        pthread_mutex_unlock(&service.lock);
        return 0;
    }
    system_health_policy_t policy;
    if (system_health_policy_snapshot(&policy) != 0) {
        pthread_mutex_unlock(&service.lock);
        return -1;
    }
    if (!policy.settings.enabled ||
        strcmp(policy.settings.profile, "disabled") == 0) {
        service.initialized = true;
        service.disabled = true;
        pthread_mutex_unlock(&service.lock);
        return 0;
    }

    char run_id[LIGHTNVR_UUID_STRING_SIZE];
    if (lightnvr_uuid_generate_v4(run_id) != 0) {
        pthread_mutex_unlock(&service.lock);
        return -1;
    }
    linux_restart_evidence_t evidence;
    if (linux_restart_read_evidence("/proc", run_id, monotonic_ms(),
                                    &evidence) != 0 ||
        evidence.capability != SYSTEM_HEALTH_CAPABILITY_AVAILABLE) {
        pthread_mutex_unlock(&service.lock);
        return -1;
    }
    system_health_evaluator_config_t config;
    system_health_evaluator_config_defaults(&config, &policy);
    copy_string(config.boot_id, sizeof(config.boot_id), evidence.boot_id);
    copy_string(config.run_id, sizeof(config.run_id), run_id);
    config.persist = persist_to_database;
    config.transition_sink = service.transition_sink
        ? service.transition_sink : production_transition_sink;
    config.transition_sink_context = service.transition_sink_context;
    service.transition_sink = config.transition_sink;
    system_health_evaluator_t *evaluator =
        system_health_evaluator_create(&config);
    if (!evaluator) {
        pthread_mutex_unlock(&service.lock);
        return -1;
    }

    system_health_incident_record_t active[SYSTEM_HEALTH_MAX_INCIDENTS];
    system_health_incident_cursor_t next;
    int active_count = db_system_health_incident_list(
        false, NULL, active, SYSTEM_HEALTH_MAX_INCIDENTS, &next);
    if (active_count < 0) {
        system_health_evaluator_destroy(evaluator);
        pthread_mutex_unlock(&service.lock);
        return -1;
    }
    for (int index = 0; index < active_count; ++index)
        (void)system_health_evaluator_resume(evaluator, &active[index]);
    publish_views(evaluator);

    system_health_process_run_t previous;
    bool had_previous = false;
    int64_t started_wall = wall_time_ms();
    db_system_health_result_t run_result = db_system_health_run_open(
        run_id, evidence.boot_id, started_wall, &service.run, &previous,
        &had_previous);
    if (run_result != DB_SYSTEM_HEALTH_OK) {
        system_health_evaluator_destroy(evaluator);
        memset(&service.run, 0, sizeof(service.run));
        pthread_mutex_unlock(&service.lock);
        return -1;
    }
    service.evaluator = evaluator;
    service.context.viable_recording_targets_known = false;

    system_health_restart_classification_t restart =
        system_health_classify_previous_run(&previous, had_previous,
                                            evidence.boot_id);
    if (restart != SYSTEM_HEALTH_RESTART_NONE) {
        system_health_observation_t observation;
        memset(&observation, 0, sizeof(observation));
        copy_string(observation.metric, sizeof(observation.metric),
                    restart == SYSTEM_HEALTH_RESTART_PROCESS
                        ? "system.process_restart" : "system.host_reboot");
        copy_string(observation.resource_id,
                    sizeof(observation.resource_id), "host");
        observation.scope = SYSTEM_HEALTH_SCOPE_HOST;
        observation.sampled_monotonic_ms = monotonic_ms();
        observation.observed_wall_time_ms = started_wall;
        system_health_observation_set_available(
            &observation, 1.0, SYSTEM_HEALTH_UNIT_COUNT);
        (void)system_health_evaluator_one_shot(
            evaluator, SYSTEM_HEALTH_CONDITION_UNEXPECTED_RESTART, "host",
            SYSTEM_HEALTH_SCOPE_HOST, SYSTEM_HEALTH_SEVERITY_ERROR,
            &observation, started_wall);
    }

    if (service.transition_sink == production_transition_sink)
        reconcile_pending_transitions(&config.policy);

    service.running = true;
    if (pthread_create(&service.thread, NULL, evaluator_service_main, NULL) != 0) {
        service.running = false;
        (void)db_system_health_run_close(service.run.run_id, wall_time_ms());
        system_health_evaluator_destroy(evaluator);
        service.evaluator = NULL;
        memset(&service.run, 0, sizeof(service.run));
        pthread_mutex_unlock(&service.lock);
        return -1;
    }
    service.initialized = true;
    pthread_mutex_unlock(&service.lock);
    return 0;
}

void system_health_evaluator_service_set_context(
    const system_health_evaluation_context_t *context) {
    if (!context) return;
    pthread_mutex_lock(&service.lock);
    service.context = *context;
    pthread_cond_signal(&service.wake);
    pthread_mutex_unlock(&service.lock);
}

void system_health_evaluator_service_set_transition_sink(
    system_health_transition_sink_fn sink, void *context) {
    pthread_mutex_lock(&service.lock);
    if (!service.initialized) {
        service.transition_sink = sink;
        service.transition_sink_context = context;
    }
    pthread_mutex_unlock(&service.lock);
}

size_t system_health_evaluator_service_active_copy(
    system_health_incident_view_t *incidents, size_t capacity) {
    pthread_mutex_lock(&service.lock);
    system_health_evaluator_t *evaluator = service.evaluator;
    size_t count = evaluator
        ? system_health_evaluator_active_copy(evaluator, incidents, capacity)
        : 0U;
    pthread_mutex_unlock(&service.lock);
    return count;
}

void system_health_evaluator_service_get_stats(
    system_health_evaluator_stats_t *stats) {
    if (!stats) return;
    pthread_mutex_lock(&service.lock);
    if (service.evaluator)
        system_health_evaluator_get_stats(service.evaluator, stats);
    else
        memset(stats, 0, sizeof(*stats));
    pthread_mutex_unlock(&service.lock);
}

bool system_health_evaluator_service_copy_run(
    system_health_process_run_t *run) {
    if (!run) return false;
    pthread_mutex_lock(&service.lock);
    bool available = service.initialized && !service.disabled &&
                     service.run.run_id[0] != '\0';
    if (available)
        *run = service.run;
    else
        memset(run, 0, sizeof(*run));
    pthread_mutex_unlock(&service.lock);
    return available;
}

void system_health_evaluator_service_shutdown(bool clean_shutdown) {
    pthread_mutex_lock(&service.lock);
    if (!service.initialized) {
        pthread_mutex_unlock(&service.lock);
        return;
    }
    if (service.disabled) {
        service.disabled = false;
        service.initialized = false;
        pthread_mutex_unlock(&service.lock);
        return;
    }
    service.running = false;
    pthread_cond_signal(&service.wake);
    pthread_mutex_unlock(&service.lock);
    pthread_join(service.thread, NULL);

    if (clean_shutdown && service.run.run_id[0])
        (void)db_system_health_run_close(service.run.run_id, wall_time_ms());
    pthread_mutex_lock(&service.lock);
    system_health_evaluator_destroy(service.evaluator);
    service.evaluator = NULL;
    memset(&service.run, 0, sizeof(service.run));
    memset(&service.context, 0, sizeof(service.context));
    service.initialized = false;
    pthread_mutex_unlock(&service.lock);
}
