/** @file system_health_evaluator.h Bounded health incident state machine. */

#ifndef LIGHTNVR_SYSTEM_HEALTH_EVALUATOR_H
#define LIGHTNVR_SYSTEM_HEALTH_EVALUATOR_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "database/db_system_health_incidents.h"
#include "telemetry/system_health_policy.h"

typedef struct system_health_evaluator system_health_evaluator_t;

typedef struct {
    bool recording_expected_known;
    bool recording_expected;
    bool service_degraded_known;
    bool service_degraded;
    bool viable_recording_targets_known;
    uint32_t viable_recording_targets;
} system_health_evaluation_context_t;

typedef struct {
    system_health_incident_action_t action;
    char incident_id[LIGHTNVR_UUID_STRING_SIZE];
    char event_id[LIGHTNVR_UUID_STRING_SIZE];
    system_health_condition_t condition;
    char subject[SYSTEM_HEALTH_ID_LENGTH];
    system_health_scope_t scope;
    system_health_state_t previous_state;
    system_health_state_t state;
    system_health_severity_t previous_severity;
    system_health_severity_t severity;
    system_health_observation_t observation;
    system_health_threshold_direction_t threshold_direction;
    double threshold_value;
    uint32_t threshold_for_ms;
    int64_t first_observed_at_ms;
    uint64_t incident_duration_ms;
    int64_t observed_at_ms;
    bool persisted;
} system_health_transition_t;

typedef db_system_health_result_t (*system_health_persist_transition_fn)(
    system_health_incident_action_t action,
    const system_health_incident_signal_t *signal,
    system_health_incident_record_t *incident_out, void *context);
typedef void (*system_health_transition_sink_fn)(
    const system_health_transition_t *transition, void *context);

typedef struct {
    system_health_policy_t policy;
    char boot_id[SYSTEM_HEALTH_INCIDENT_BOOT_ID_MAX];
    char run_id[LIGHTNVR_UUID_STRING_SIZE];
    uint32_t stale_after_ms;
    uint32_t retry_initial_ms;
    uint32_t retry_max_ms;
    uint32_t material_change_debounce_ms;
    double material_change_ratio;
    system_health_persist_transition_fn persist;
    void *persist_context;
    system_health_transition_sink_fn transition_sink;
    void *transition_sink_context;
} system_health_evaluator_config_t;

typedef struct {
    size_t tracked_conditions;
    size_t active_incidents;
    size_t pending_persistence;
    uint64_t transitions;
    uint64_t persistence_failures;
    uint64_t persistence_retries;
    int64_t oldest_pending_wall_time_ms;
} system_health_evaluator_stats_t;

typedef struct {
    char incident_id[LIGHTNVR_UUID_STRING_SIZE];
    system_health_condition_t condition;
    char subject[SYSTEM_HEALTH_ID_LENGTH];
    system_health_scope_t scope;
    system_health_state_t state;
    system_health_severity_t severity;
    int64_t first_observed_at_ms;
    int64_t last_observed_at_ms;
    system_health_observation_t observation;
    bool persistence_pending;
} system_health_incident_view_t;

typedef enum {
    SYSTEM_HEALTH_IMMEDIATE_PROCESS = 0,
    SYSTEM_HEALTH_IMMEDIATE_ROOT,
    SYSTEM_HEALTH_IMMEDIATE_RECORDING,
    SYSTEM_HEALTH_IMMEDIATE_RESOURCE_COUNT
} system_health_immediate_resource_t;

typedef enum {
    SYSTEM_HEALTH_RESTART_NONE = 0,
    SYSTEM_HEALTH_RESTART_PROCESS,
    SYSTEM_HEALTH_RESTART_HOST
} system_health_restart_classification_t;

void system_health_evaluator_config_defaults(
    system_health_evaluator_config_t *config,
    const system_health_policy_t *policy);
system_health_evaluator_t *system_health_evaluator_create(
    const system_health_evaluator_config_t *config);
void system_health_evaluator_destroy(system_health_evaluator_t *evaluator);

int system_health_evaluator_resume(
    system_health_evaluator_t *evaluator,
    const system_health_incident_record_t *incident);
int system_health_evaluator_evaluate(
    system_health_evaluator_t *evaluator,
    const system_health_snapshot_t *snapshot,
    const system_health_evaluation_context_t *context);
int system_health_evaluator_one_shot(
    system_health_evaluator_t *evaluator, system_health_condition_t condition,
    const char *subject, system_health_scope_t scope,
    system_health_severity_t severity,
    const system_health_observation_t *observation, int64_t observed_at_ms);
void system_health_evaluator_get_stats(
    const system_health_evaluator_t *evaluator,
    system_health_evaluator_stats_t *stats);
size_t system_health_evaluator_active_copy(
    const system_health_evaluator_t *evaluator,
    system_health_incident_view_t *incidents, size_t capacity);
system_health_restart_classification_t system_health_classify_previous_run(
    const system_health_process_run_t *previous, bool had_previous,
    const char *current_boot_id);

/** Lock-free evidence seam for allocation/resource failure call sites. */
void system_health_evaluator_note_immediate(
    system_health_condition_t condition,
    system_health_immediate_resource_t resource);

/** Production service: evaluates sampler generations and owns run markers. */
int system_health_evaluator_service_start(void);
void system_health_evaluator_service_set_transition_sink(
    system_health_transition_sink_fn sink, void *context);
void system_health_evaluator_service_set_context(
    const system_health_evaluation_context_t *context);
size_t system_health_evaluator_service_active_copy(
    system_health_incident_view_t *incidents, size_t capacity);
void system_health_evaluator_service_get_stats(
    system_health_evaluator_stats_t *stats);
bool system_health_evaluator_service_copy_run(
    system_health_process_run_t *run);
void system_health_evaluator_service_shutdown(bool clean_shutdown);

#endif /* LIGHTNVR_SYSTEM_HEALTH_EVALUATOR_H */
