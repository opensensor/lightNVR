#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <string.h>

#include "telemetry/system_health_evaluator.h"
#include "unity.h"

typedef struct {
    system_health_transition_t transitions[32];
    size_t count;
} transition_log_t;

typedef struct {
    unsigned int failures_remaining;
    unsigned int calls;
    int64_t prior_wall;
} fake_persistence_t;

void setUp(void) {}
void tearDown(void) {}

static void capture_transition(const system_health_transition_t *transition,
                               void *context) {
    transition_log_t *log = context;
    if (log->count < 32U) log->transitions[log->count++] = *transition;
}

static db_system_health_result_t fake_persist(
    system_health_incident_action_t action,
    const system_health_incident_signal_t *signal,
    system_health_incident_record_t *incident, void *context) {
    fake_persistence_t *state = context;
    (void)action;
    state->calls++;
    if (signal->observed_at_ms < state->prior_wall)
        return DB_SYSTEM_HEALTH_CONFLICT;
    if (state->failures_remaining > 0U) {
        state->failures_remaining--;
        return DB_SYSTEM_HEALTH_ERROR;
    }
    memset(incident, 0, sizeof(*incident));
    snprintf(incident->uuid, sizeof(incident->uuid),
             "11111111-1111-4111-8111-111111111111");
    incident->last_seen_at_ms = signal->observed_at_ms;
    state->prior_wall = signal->observed_at_ms;
    return DB_SYSTEM_HEALTH_OK;
}

static system_health_policy_t policy(void) {
    system_health_policy_settings_t settings;
    system_health_policy_settings_defaults(&settings);
    system_health_policy_t result;
    char error[SYSTEM_HEALTH_POLICY_ERROR_LENGTH];
    TEST_ASSERT_EQUAL_INT(0, system_health_policy_build(
        &settings, NULL, &result, NULL, 0U, error));
    return result;
}

static system_health_evaluator_t *make_evaluator(
    system_health_policy_t *health_policy, transition_log_t *log,
    fake_persistence_t *persistence) {
    system_health_evaluator_config_t config;
    system_health_evaluator_config_defaults(&config, health_policy);
    snprintf(config.boot_id, sizeof(config.boot_id), "test-boot");
    snprintf(config.run_id, sizeof(config.run_id),
             "22222222-2222-4222-8222-222222222222");
    config.stale_after_ms = 1000U;
    config.retry_initial_ms = 1000U;
    config.retry_max_ms = 4000U;
    config.material_change_debounce_ms = 1000000U;
    config.transition_sink = capture_transition;
    config.transition_sink_context = log;
    if (persistence) {
        config.persist = fake_persist;
        config.persist_context = persistence;
    }
    return system_health_evaluator_create(&config);
}

static void begin_snapshot(system_health_snapshot_t *snapshot, uint64_t sequence,
                           uint64_t monotonic, int64_t wall) {
    memset(snapshot, 0, sizeof(*snapshot));
    snapshot->sequence = sequence;
    snapshot->completed_monotonic_ms = monotonic;
    snapshot->completed_wall_time_ms = wall;
}

static void add_observation(system_health_snapshot_t *snapshot,
                            const char *metric, const char *resource,
                            system_health_scope_t scope, double value,
                            system_health_unit_t unit, uint64_t sampled) {
    system_health_observation_t observation;
    memset(&observation, 0, sizeof(observation));
    snprintf(observation.metric, sizeof(observation.metric), "%s", metric);
    snprintf(observation.resource_id, sizeof(observation.resource_id), "%s",
             resource);
    observation.scope = scope;
    observation.sampled_monotonic_ms = sampled;
    observation.observed_wall_time_ms = snapshot->completed_wall_time_ms;
    system_health_observation_set_available(&observation, value, unit);
    TEST_ASSERT_TRUE(system_health_snapshot_append(snapshot, &observation));
}

static void add_unavailable(system_health_snapshot_t *snapshot,
                            const char *metric, const char *resource,
                            system_health_scope_t scope,
                            system_health_capability_t capability,
                            uint64_t sampled) {
    system_health_observation_t observation;
    memset(&observation, 0, sizeof(observation));
    snprintf(observation.metric, sizeof(observation.metric), "%s", metric);
    snprintf(observation.resource_id, sizeof(observation.resource_id), "%s",
             resource);
    observation.scope = scope;
    observation.sampled_monotonic_ms = sampled;
    observation.observed_wall_time_ms = snapshot->completed_wall_time_ms;
    system_health_observation_set_unavailable(&observation, capability);
    TEST_ASSERT_TRUE(system_health_snapshot_append(snapshot, &observation));
}

static void cpu_sample(system_health_snapshot_t *snapshot, uint64_t sequence,
                       uint64_t monotonic, double cpu, double some,
                       double full) {
    begin_snapshot(snapshot, sequence, monotonic, 1700000000000LL +
                                             (int64_t)monotonic);
    add_observation(snapshot, "host.cpu.busy_ratio", "host",
                    SYSTEM_HEALTH_SCOPE_HOST, cpu, SYSTEM_HEALTH_UNIT_RATIO,
                    monotonic);
    add_observation(snapshot, "host.pressure.cpu.some_ratio", "host",
                    SYSTEM_HEALTH_SCOPE_HOST, some, SYSTEM_HEALTH_UNIT_RATIO,
                    monotonic);
    add_observation(snapshot, "host.pressure.cpu.full_ratio", "host",
                    SYSTEM_HEALTH_SCOPE_HOST, full, SYSTEM_HEALTH_UNIT_RATIO,
                    monotonic);
}

static void test_cpu_spike_open_escalate_and_recover_once(void) {
    system_health_policy_t health_policy = policy();
    system_health_condition_policy_t *cpu =
        &health_policy.conditions[SYSTEM_HEALTH_CONDITION_CPU_SATURATION];
    cpu->warning_for_seconds = 60U;
    cpu->critical_for_seconds = 60U;
    cpu->recovery_for_seconds = 30U;
    transition_log_t log = {0};
    system_health_evaluator_t *evaluator = make_evaluator(
        &health_policy, &log, NULL);
    TEST_ASSERT_NOT_NULL(evaluator);
    system_health_snapshot_t snapshot;
    system_health_evaluation_context_t context = {
        .service_degraded_known = true, .service_degraded = true};

    cpu_sample(&snapshot, 1U, 1000U, .92, .10, 0.0);
    TEST_ASSERT_EQUAL_INT(0, system_health_evaluator_evaluate(
        evaluator, &snapshot, &context));
    cpu_sample(&snapshot, 2U, 31000U, .50, 0.0, 0.0);
    TEST_ASSERT_EQUAL_INT(0, system_health_evaluator_evaluate(
        evaluator, &snapshot, &context));
    TEST_ASSERT_EQUAL_UINT64(0U, log.count);

    cpu_sample(&snapshot, 3U, 100000U, .92, .10, 0.0);
    system_health_evaluator_evaluate(evaluator, &snapshot, &context);
    cpu_sample(&snapshot, 4U, 160000U, .92, .10, 0.0);
    system_health_evaluator_evaluate(evaluator, &snapshot, &context);
    cpu_sample(&snapshot, 5U, 170000U, .97, .10, .01);
    system_health_evaluator_evaluate(evaluator, &snapshot, &context);
    cpu_sample(&snapshot, 6U, 230000U, .97, .10, .01);
    system_health_evaluator_evaluate(evaluator, &snapshot, &context);
    cpu_sample(&snapshot, 7U, 240000U, .50, 0.0, 0.0);
    system_health_evaluator_evaluate(evaluator, &snapshot, &context);
    cpu_sample(&snapshot, 8U, 270000U, .50, 0.0, 0.0);
    system_health_evaluator_evaluate(evaluator, &snapshot, &context);

    TEST_ASSERT_EQUAL_UINT64(4U, log.count);
    TEST_ASSERT_EQUAL_INT(SYSTEM_HEALTH_INCIDENT_OPEN,
                          log.transitions[0].action);
    TEST_ASSERT_EQUAL_INT(SYSTEM_HEALTH_INCIDENT_ESCALATE,
                          log.transitions[1].action);
    TEST_ASSERT_EQUAL_INT(SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE,
                          log.transitions[2].action);
    TEST_ASSERT_EQUAL_INT(SYSTEM_HEALTH_INCIDENT_RECOVER,
                          log.transitions[3].action);
    TEST_ASSERT_EQUAL_INT(SYSTEM_HEALTH_SEVERITY_CRITICAL,
                          log.transitions[3].previous_severity);
    for (size_t index = 0; index < log.count; ++index)
        TEST_ASSERT_TRUE(lightnvr_uuid_is_valid(
            log.transitions[index].event_id));
    system_health_evaluator_destroy(evaluator);
}

static void test_immediate_fault_and_unknown_do_not_false_recover(void) {
    system_health_policy_t health_policy = policy();
    transition_log_t log = {0};
    system_health_evaluator_t *evaluator = make_evaluator(
        &health_policy, &log, NULL);
    system_health_snapshot_t snapshot;
    begin_snapshot(&snapshot, 1U, 1000U, 1700000001000LL);
    add_observation(&snapshot, "filesystem.read_only", "recording",
                    SYSTEM_HEALTH_SCOPE_FILESYSTEM, 1.0,
                    SYSTEM_HEALTH_UNIT_BOOLEAN, 1000U);
    system_health_evaluator_evaluate(evaluator, &snapshot, NULL);
    TEST_ASSERT_EQUAL_UINT64(1U, log.count);
    TEST_ASSERT_EQUAL_INT(SYSTEM_HEALTH_SEVERITY_CRITICAL,
                          log.transitions[0].severity);

    begin_snapshot(&snapshot, 2U, 2000U, 1700000002000LL);
    add_unavailable(&snapshot, "filesystem.read_only", "recording",
                    SYSTEM_HEALTH_SCOPE_FILESYSTEM,
                    SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED, 2000U);
    system_health_evaluator_evaluate(evaluator, &snapshot, NULL);
    system_health_incident_view_t active[4];
    TEST_ASSERT_EQUAL_UINT64(1U, system_health_evaluator_active_copy(
        evaluator, active, 4U));
    TEST_ASSERT_EQUAL_INT(SYSTEM_HEALTH_STATE_OPEN, active[0].state);
    system_health_evaluator_destroy(evaluator);
}

static void test_cached_one_shot_is_deduplicated_by_sample_time(void) {
    system_health_policy_t health_policy = policy();
    transition_log_t log = {0};
    system_health_evaluator_t *evaluator = make_evaluator(
        &health_policy, &log, NULL);
    system_health_snapshot_t snapshot;
    begin_snapshot(&snapshot, 1U, 1000U, 1700000001000LL);
    add_observation(&snapshot, "container.memory.oom_kills_delta", "container",
                    SYSTEM_HEALTH_SCOPE_CONTAINER, 1.0,
                    SYSTEM_HEALTH_UNIT_COUNT, 900U);
    system_health_evaluator_evaluate(evaluator, &snapshot, NULL);
    snapshot.sequence = 2U;
    snapshot.completed_monotonic_ms = 2000U;
    snapshot.completed_wall_time_ms++;
    system_health_evaluator_evaluate(evaluator, &snapshot, NULL);
    TEST_ASSERT_EQUAL_UINT64(1U, log.count);
    TEST_ASSERT_EQUAL_INT(SYSTEM_HEALTH_INCIDENT_ONE_SHOT,
                          log.transitions[0].action);
    TEST_ASSERT_TRUE(lightnvr_uuid_is_valid(log.transitions[0].event_id));
    system_health_evaluator_destroy(evaluator);
}

static void test_persistence_failure_freezes_and_retries_with_backoff(void) {
    system_health_policy_t health_policy = policy();
    transition_log_t log = {0};
    fake_persistence_t persistence = {.failures_remaining = 1U};
    system_health_evaluator_t *evaluator = make_evaluator(
        &health_policy, &log, &persistence);
    system_health_snapshot_t snapshot;
    begin_snapshot(&snapshot, 1U, 1000U, 1700000001000LL);
    add_observation(&snapshot, "filesystem.read_only", "recording",
                    SYSTEM_HEALTH_SCOPE_FILESYSTEM, 1.0,
                    SYSTEM_HEALTH_UNIT_BOOLEAN, 1000U);
    system_health_evaluator_evaluate(evaluator, &snapshot, NULL);
    system_health_incident_view_t active[2];
    TEST_ASSERT_EQUAL_UINT64(1U, system_health_evaluator_active_copy(
        evaluator, active, 2U));
    TEST_ASSERT_TRUE(active[0].persistence_pending);
    TEST_ASSERT_EQUAL_STRING("", active[0].incident_id);
    system_health_evaluator_stats_t stats;
    system_health_evaluator_get_stats(evaluator, &stats);
    TEST_ASSERT_EQUAL_INT64(1700000001000LL,
                            stats.oldest_pending_wall_time_ms);
    snapshot.sequence = 2U;
    snapshot.completed_monotonic_ms = 1500U;
    snapshot.completed_wall_time_ms += 500;
    system_health_evaluator_evaluate(evaluator, &snapshot, NULL);
    TEST_ASSERT_EQUAL_UINT64(1U, persistence.calls);
    snapshot.sequence = 3U;
    snapshot.completed_monotonic_ms = 2001U;
    snapshot.completed_wall_time_ms += 501;
    system_health_evaluator_evaluate(evaluator, &snapshot, NULL);
    TEST_ASSERT_EQUAL_UINT64(2U, persistence.calls);
    TEST_ASSERT_EQUAL_UINT64(1U, log.count);
    system_health_evaluator_get_stats(evaluator, &stats);
    TEST_ASSERT_EQUAL_UINT64(0U, stats.pending_persistence);
    TEST_ASSERT_EQUAL_UINT64(1U, stats.persistence_retries);
    TEST_ASSERT_EQUAL_INT64(0, stats.oldest_pending_wall_time_ms);
    system_health_evaluator_destroy(evaluator);
}

static void test_restart_classification_never_claims_a_cause(void) {
    system_health_process_run_t previous;
    memset(&previous, 0, sizeof(previous));
    snprintf(previous.boot_id, sizeof(previous.boot_id), "boot-a");
    TEST_ASSERT_EQUAL_INT(SYSTEM_HEALTH_RESTART_NONE,
        system_health_classify_previous_run(&previous, false, "boot-a"));
    previous.clean_close = true;
    TEST_ASSERT_EQUAL_INT(SYSTEM_HEALTH_RESTART_NONE,
        system_health_classify_previous_run(&previous, true, "boot-a"));
    previous.clean_close = false;
    TEST_ASSERT_EQUAL_INT(SYSTEM_HEALTH_RESTART_PROCESS,
        system_health_classify_previous_run(&previous, true, "boot-a"));
    TEST_ASSERT_EQUAL_INT(SYSTEM_HEALTH_RESTART_HOST,
        system_health_classify_previous_run(&previous, true, "boot-b"));
}

static void test_atomic_immediate_resource_evidence_opens_off_caller(void) {
    system_health_policy_t health_policy = policy();
    transition_log_t log = {0};
    system_health_evaluator_t *evaluator = make_evaluator(
        &health_policy, &log, NULL);
    system_health_evaluator_note_immediate(
        SYSTEM_HEALTH_CONDITION_PROCESS_FD_EXHAUSTION,
        SYSTEM_HEALTH_IMMEDIATE_PROCESS);
    system_health_snapshot_t snapshot;
    begin_snapshot(&snapshot, 1U, 1000U, 1700000001000LL);
    system_health_evaluator_evaluate(evaluator, &snapshot, NULL);
    TEST_ASSERT_EQUAL_UINT64(1U, log.count);
    TEST_ASSERT_EQUAL_INT(SYSTEM_HEALTH_SEVERITY_CRITICAL,
                          log.transitions[0].severity);
    TEST_ASSERT_EQUAL_STRING("process", log.transitions[0].subject);
    system_health_evaluator_destroy(evaluator);
}

static const system_health_transition_t *transition_for(
    const transition_log_t *log, system_health_condition_t condition) {
    for (size_t index = 0; index < log->count; ++index) {
        if (log->transitions[index].condition == condition)
            return &log->transitions[index];
    }
    return NULL;
}

static void test_hardware_provider_evidence_maps_to_bounded_conditions(void) {
    system_health_policy_t health_policy = policy();
    transition_log_t log = {0};
    system_health_evaluator_t *evaluator = make_evaluator(
        &health_policy, &log, NULL);
    system_health_snapshot_t snapshot;
    begin_snapshot(&snapshot, 1U, 1000U, 1700000001000LL);
    add_observation(&snapshot, "storage.device.prefail", "device.abc",
                    SYSTEM_HEALTH_SCOPE_DEVICE, 1.0,
                    SYSTEM_HEALTH_UNIT_BOOLEAN, 1000U);
    add_observation(&snapshot, "hardware.ecc.corrected_delta", "edac0",
                    SYSTEM_HEALTH_SCOPE_HOST, 2.0,
                    SYSTEM_HEALTH_UNIT_COUNT, 1000U);
    add_observation(&snapshot, "hardware.ecc.uncorrectable_delta", "edac0",
                    SYSTEM_HEALTH_SCOPE_HOST, 1.0,
                    SYSTEM_HEALTH_UNIT_COUNT, 1000U);
    add_observation(&snapshot, "hardware.fan.hot", "fan0",
                    SYSTEM_HEALTH_SCOPE_HOST, 0.0,
                    SYSTEM_HEALTH_UNIT_BOOLEAN, 1000U);
    add_observation(&snapshot, "hardware.fan.failed", "fan0",
                    SYSTEM_HEALTH_SCOPE_HOST, 1.0,
                    SYSTEM_HEALTH_UNIT_BOOLEAN, 1000U);
    add_observation(&snapshot, "hardware.power.unstable", "board",
                    SYSTEM_HEALTH_SCOPE_HOST, 1.0,
                    SYSTEM_HEALTH_UNIT_BOOLEAN, 1000U);
    TEST_ASSERT_EQUAL_INT(0, system_health_evaluator_evaluate(
        evaluator, &snapshot, NULL));

    const system_health_transition_t *prefail = transition_for(
        &log, SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_PREFAIL);
    const system_health_transition_t *corrected = transition_for(
        &log, SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_CORRECTED);
    const system_health_transition_t *uncorrectable = transition_for(
        &log, SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_UNCORRECTABLE);
    const system_health_transition_t *fan = transition_for(
        &log, SYSTEM_HEALTH_CONDITION_HARDWARE_FAN_FAILED);
    const system_health_transition_t *power = transition_for(
        &log, SYSTEM_HEALTH_CONDITION_HARDWARE_POWER_UNSTABLE);
    TEST_ASSERT_NOT_NULL(prefail);
    TEST_ASSERT_NOT_NULL(corrected);
    TEST_ASSERT_NOT_NULL(uncorrectable);
    TEST_ASSERT_NOT_NULL(fan);
    TEST_ASSERT_NOT_NULL(power);
    TEST_ASSERT_EQUAL_INT(SYSTEM_HEALTH_SEVERITY_WARNING,
                          prefail->severity);
    TEST_ASSERT_EQUAL_INT(SYSTEM_HEALTH_INCIDENT_ONE_SHOT,
                          corrected->action);
    TEST_ASSERT_EQUAL_INT(SYSTEM_HEALTH_SEVERITY_CRITICAL,
                          uncorrectable->severity);
    TEST_ASSERT_EQUAL_INT(SYSTEM_HEALTH_SEVERITY_WARNING, fan->severity);
    TEST_ASSERT_EQUAL_INT(SYSTEM_HEALTH_SEVERITY_CRITICAL, power->severity);
    system_health_evaluator_destroy(evaluator);

    memset(&log, 0, sizeof(log));
    evaluator = make_evaluator(&health_policy, &log, NULL);
    begin_snapshot(&snapshot, 1U, 1000U, 1700000001000LL);
    add_observation(&snapshot, "hardware.fan.hot", "fan0",
                    SYSTEM_HEALTH_SCOPE_HOST, 1.0,
                    SYSTEM_HEALTH_UNIT_BOOLEAN, 1000U);
    add_observation(&snapshot, "hardware.fan.failed", "fan0",
                    SYSTEM_HEALTH_SCOPE_HOST, 1.0,
                    SYSTEM_HEALTH_UNIT_BOOLEAN, 1000U);
    system_health_evaluator_evaluate(evaluator, &snapshot, NULL);
    fan = transition_for(&log, SYSTEM_HEALTH_CONDITION_HARDWARE_FAN_FAILED);
    TEST_ASSERT_NOT_NULL(fan);
    TEST_ASSERT_EQUAL_INT(SYSTEM_HEALTH_SEVERITY_CRITICAL, fan->severity);
    system_health_evaluator_destroy(evaluator);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_cpu_spike_open_escalate_and_recover_once);
    RUN_TEST(test_immediate_fault_and_unknown_do_not_false_recover);
    RUN_TEST(test_cached_one_shot_is_deduplicated_by_sample_time);
    RUN_TEST(test_persistence_failure_freezes_and_retries_with_backoff);
    RUN_TEST(test_restart_classification_never_claims_a_cause);
    RUN_TEST(test_atomic_immediate_resource_evidence_opens_off_caller);
    RUN_TEST(test_hardware_provider_evidence_maps_to_bounded_conditions);
    return UNITY_END();
}
