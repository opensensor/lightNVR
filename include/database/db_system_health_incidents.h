/** @file db_system_health_incidents.h Durable host-health episode repository. */

#ifndef LIGHTNVR_DB_SYSTEM_HEALTH_INCIDENTS_H
#define LIGHTNVR_DB_SYSTEM_HEALTH_INCIDENTS_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "telemetry/system_health_types.h"
#include "utils/uuid.h"

#define SYSTEM_HEALTH_INCIDENT_SUBJECT_MAX SYSTEM_HEALTH_ID_LENGTH
#define SYSTEM_HEALTH_INCIDENT_OBSERVATION_MAX 2049U
#define SYSTEM_HEALTH_INCIDENT_BOOT_ID_MAX 64U
#define SYSTEM_HEALTH_INCIDENT_PAGE_MAX 100
#define SYSTEM_HEALTH_TRANSITION_RETENTION_BATCH_MAX 1000

typedef enum {
    DB_SYSTEM_HEALTH_OK = 0,
    DB_SYSTEM_HEALTH_RESUMED = 1,
    DB_SYSTEM_HEALTH_NOT_FOUND = 2,
    DB_SYSTEM_HEALTH_INVALID = -2,
    DB_SYSTEM_HEALTH_CONFLICT = -3,
    DB_SYSTEM_HEALTH_ERROR = -1
} db_system_health_result_t;

typedef enum {
    SYSTEM_HEALTH_INCIDENT_OPEN = 0,
    SYSTEM_HEALTH_INCIDENT_ESCALATE,
    SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE,
    SYSTEM_HEALTH_INCIDENT_RECOVER,
    SYSTEM_HEALTH_INCIDENT_ONE_SHOT
} system_health_incident_action_t;

typedef enum {
    SYSTEM_HEALTH_RECONCILIATION_NONE = 0,
    SYSTEM_HEALTH_RECONCILIATION_ALERT_PENDING,
    SYSTEM_HEALTH_RECONCILIATION_RECOVERY_PENDING,
    SYSTEM_HEALTH_RECONCILIATION_RECONCILED,
    SYSTEM_HEALTH_RECONCILIATION_DELIVERY_FAILED
} system_health_reconciliation_t;

typedef struct {
    system_health_condition_t condition;
    char subject[SYSTEM_HEALTH_INCIDENT_SUBJECT_MAX];
    system_health_scope_t scope;
    system_health_state_t state;
    system_health_severity_t severity;
    int64_t observed_at_ms;
    char observation_json[SYSTEM_HEALTH_INCIDENT_OBSERVATION_MAX];
    char event_id[LIGHTNVR_UUID_STRING_SIZE];
    system_health_reconciliation_t reconciliation;
    char boot_id[SYSTEM_HEALTH_INCIDENT_BOOT_ID_MAX];
    char run_id[LIGHTNVR_UUID_STRING_SIZE];
} system_health_incident_signal_t;

typedef struct {
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    char condition_code[SYSTEM_HEALTH_METRIC_LENGTH];
    char subject[SYSTEM_HEALTH_INCIDENT_SUBJECT_MAX];
    system_health_scope_t scope;
    system_health_state_t state;
    system_health_severity_t severity;
    int64_t first_seen_at_ms;
    int64_t last_seen_at_ms;
    int64_t closed_at_ms;
    char observation_json[SYSTEM_HEALTH_INCIDENT_OBSERVATION_MAX];
    char alert_event_id[LIGHTNVR_UUID_STRING_SIZE];
    char recovery_event_id[LIGHTNVR_UUID_STRING_SIZE];
    system_health_reconciliation_t reconciliation;
    char boot_id[SYSTEM_HEALTH_INCIDENT_BOOT_ID_MAX];
    char run_id[LIGHTNVR_UUID_STRING_SIZE];
    int64_t revision;
} system_health_incident_record_t;

typedef struct {
    bool valid;
    int64_t last_seen_at_ms;
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
} system_health_incident_cursor_t;

typedef struct {
    int64_t id;
    char transition_uuid[LIGHTNVR_UUID_STRING_SIZE];
    char incident_uuid[LIGHTNVR_UUID_STRING_SIZE];
    system_health_incident_action_t action;
    system_health_state_t from_state;
    bool from_state_valid;
    system_health_state_t to_state;
    system_health_severity_t severity;
    int64_t observed_at_ms;
    char observation_json[SYSTEM_HEALTH_INCIDENT_OBSERVATION_MAX];
    char event_id[LIGHTNVR_UUID_STRING_SIZE];
    system_health_reconciliation_t reconciliation;
    char boot_id[SYSTEM_HEALTH_INCIDENT_BOOT_ID_MAX];
    char run_id[LIGHTNVR_UUID_STRING_SIZE];
} system_health_incident_transition_t;

typedef struct {
    bool valid;
    int64_t observed_at_ms;
    int64_t id;
} system_health_transition_cursor_t;

typedef struct {
    char run_id[LIGHTNVR_UUID_STRING_SIZE];
    char boot_id[SYSTEM_HEALTH_INCIDENT_BOOT_ID_MAX];
    int64_t started_at_ms;
    int64_t closed_at_ms;
    bool clean_close;
} system_health_process_run_t;

db_system_health_result_t db_system_health_incident_apply(
    system_health_incident_action_t action,
    const system_health_incident_signal_t *signal,
    system_health_incident_record_t *incident_out);

db_system_health_result_t db_system_health_incident_get_active(
    system_health_condition_t condition, const char *subject,
    system_health_incident_record_t *incident_out);
db_system_health_result_t db_system_health_incident_get_uuid(
    const char *incident_uuid,
    system_health_incident_record_t *incident_out);

int db_system_health_incident_list(
    bool include_closed, const system_health_incident_cursor_t *cursor,
    system_health_incident_record_t *incidents, int max_count,
    system_health_incident_cursor_t *next_cursor);

int db_system_health_transition_list(
    const char *incident_uuid, const system_health_transition_cursor_t *cursor,
    system_health_incident_transition_t *transitions, int max_count,
    system_health_transition_cursor_t *next_cursor);

/** List pending event reconciliations in ascending durable row order. */
int db_system_health_transition_list_pending(
    int64_t after_id, system_health_incident_transition_t *transitions,
    int max_count);

/** Update one exact transition after idempotent outbox enqueue succeeds/fails. */
db_system_health_result_t db_system_health_transition_set_reconciliation(
    const char *incident_uuid, const char *event_id,
    system_health_reconciliation_t reconciliation);

db_system_health_result_t db_system_health_incident_set_reconciliation(
    const char *incident_uuid, bool recovery_event, const char *event_id,
    system_health_reconciliation_t reconciliation);

int db_system_health_transition_prune(int64_t closed_before_ms, int limit,
                                      int *deleted_count);

db_system_health_result_t db_system_health_run_open(
    const char *run_id, const char *boot_id, int64_t started_at_ms,
    system_health_process_run_t *run_out,
    system_health_process_run_t *previous_out, bool *had_previous);

db_system_health_result_t db_system_health_run_close(
    const char *run_id, int64_t closed_at_ms);

db_system_health_result_t db_system_health_run_latest(
    system_health_process_run_t *run_out);

#endif /* LIGHTNVR_DB_SYSTEM_HEALTH_INCIDENTS_H */
