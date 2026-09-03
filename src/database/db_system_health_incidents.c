#define _POSIX_C_SOURCE 200809L

#include "database/db_system_health_incidents.h"

#include <cjson/cJSON.h>
#include <ctype.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "database/db_core.h"
#include "utils/strings.h"
#include "utils/uuid.h"

#define INCIDENT_FIELDS \
    "uuid,condition_code,subject,scope,state,severity,first_seen_at_ms," \
    "last_seen_at_ms,COALESCE(closed_at_ms,0),last_observation_json," \
    "COALESCE(alert_event_id,''),COALESCE(recovery_event_id,'')," \
    "reconciliation_state,boot_id,run_id,revision"

#define TRANSITION_FIELDS \
    "id,transition_uuid,incident_uuid,kind,COALESCE(from_state,'')," \
    "to_state,severity,observed_at_ms,safe_observation_json," \
    "COALESCE(event_id,''),reconciliation_state,boot_id,run_id"

static bool transaction_begin(sqlite3 *database) {
    return sqlite3_exec(database, "BEGIN IMMEDIATE;", NULL, NULL, NULL) ==
           SQLITE_OK;
}

static bool transaction_finish(sqlite3 *database, bool success) {
    if (success &&
        sqlite3_exec(database, "COMMIT;", NULL, NULL, NULL) == SQLITE_OK)
        return true;
    sqlite3_exec(database, "ROLLBACK;", NULL, NULL, NULL);
    return false;
}

static bool valid_logical_id(const char *value, size_t capacity) {
    size_t length;
    if (!value || capacity < 2U ||
        (length = strnlen(value, capacity)) == 0 || length >= capacity)
        return false;
    for (size_t index = 0; index < length; ++index) {
        unsigned char current = (unsigned char)value[index];
        if (!(isalnum(current) || current == '_' || current == '.' ||
              current == ':' || current == '-')) return false;
    }
    return true;
}

static bool normalize_observation(
    const char *input, char output[SYSTEM_HEALTH_INCIDENT_OBSERVATION_MAX]) {
    if (!input || strnlen(input, SYSTEM_HEALTH_INCIDENT_OBSERVATION_MAX) >=
                      SYSTEM_HEALTH_INCIDENT_OBSERVATION_MAX)
        return false;
    cJSON *root = cJSON_ParseWithOpts(input, NULL, true);
    if (!cJSON_IsObject(root)) {
        cJSON_Delete(root);
        return false;
    }
    char *encoded = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if (!encoded) return false;
    size_t length = strlen(encoded);
    bool valid = length >= 2U &&
                 length < SYSTEM_HEALTH_INCIDENT_OBSERVATION_MAX;
    if (valid) safe_strcpy(output, encoded,
                           SYSTEM_HEALTH_INCIDENT_OBSERVATION_MAX, 0);
    cJSON_free(encoded);
    return valid;
}

static const char *state_name(system_health_state_t state) {
    switch (state) {
        case SYSTEM_HEALTH_STATE_OPEN: return "open";
        case SYSTEM_HEALTH_STATE_RECOVERING: return "recovering";
        case SYSTEM_HEALTH_STATE_CLOSED: return "closed";
        default: return NULL;
    }
}

static system_health_state_t parse_state(const char *value, bool *valid) {
    *valid = true;
    if (strcmp(value, "open") == 0) return SYSTEM_HEALTH_STATE_OPEN;
    if (strcmp(value, "recovering") == 0)
        return SYSTEM_HEALTH_STATE_RECOVERING;
    if (strcmp(value, "closed") == 0) return SYSTEM_HEALTH_STATE_CLOSED;
    *valid = false;
    return SYSTEM_HEALTH_STATE_UNKNOWN;
}

static const char *severity_name(system_health_severity_t severity) {
    switch (severity) {
        case SYSTEM_HEALTH_SEVERITY_WARNING: return "warning";
        case SYSTEM_HEALTH_SEVERITY_ERROR: return "error";
        case SYSTEM_HEALTH_SEVERITY_CRITICAL: return "critical";
        default: return NULL;
    }
}

static system_health_severity_t parse_severity(const char *value) {
    if (strcmp(value, "warning") == 0)
        return SYSTEM_HEALTH_SEVERITY_WARNING;
    if (strcmp(value, "error") == 0) return SYSTEM_HEALTH_SEVERITY_ERROR;
    if (strcmp(value, "critical") == 0)
        return SYSTEM_HEALTH_SEVERITY_CRITICAL;
    return SYSTEM_HEALTH_SEVERITY_NONE;
}

static const char *reconciliation_name(system_health_reconciliation_t value) {
    switch (value) {
        case SYSTEM_HEALTH_RECONCILIATION_NONE: return "none";
        case SYSTEM_HEALTH_RECONCILIATION_ALERT_PENDING:
            return "alert_pending";
        case SYSTEM_HEALTH_RECONCILIATION_RECOVERY_PENDING:
            return "recovery_pending";
        case SYSTEM_HEALTH_RECONCILIATION_RECONCILED: return "reconciled";
        case SYSTEM_HEALTH_RECONCILIATION_DELIVERY_FAILED:
            return "delivery_failed";
    }
    return NULL;
}

static system_health_reconciliation_t parse_reconciliation(const char *value) {
    if (strcmp(value, "alert_pending") == 0)
        return SYSTEM_HEALTH_RECONCILIATION_ALERT_PENDING;
    if (strcmp(value, "recovery_pending") == 0)
        return SYSTEM_HEALTH_RECONCILIATION_RECOVERY_PENDING;
    if (strcmp(value, "reconciled") == 0)
        return SYSTEM_HEALTH_RECONCILIATION_RECONCILED;
    if (strcmp(value, "delivery_failed") == 0)
        return SYSTEM_HEALTH_RECONCILIATION_DELIVERY_FAILED;
    return SYSTEM_HEALTH_RECONCILIATION_NONE;
}

static const char *action_name(system_health_incident_action_t action) {
    switch (action) {
        case SYSTEM_HEALTH_INCIDENT_OPEN: return "open";
        case SYSTEM_HEALTH_INCIDENT_ESCALATE: return "escalation";
        case SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE: return "material_change";
        case SYSTEM_HEALTH_INCIDENT_RECOVER: return "recovery";
        case SYSTEM_HEALTH_INCIDENT_ONE_SHOT: return "one_shot";
    }
    return NULL;
}

static system_health_incident_action_t parse_action(const char *value,
                                                     bool *valid) {
    *valid = true;
    if (strcmp(value, "open") == 0) return SYSTEM_HEALTH_INCIDENT_OPEN;
    if (strcmp(value, "escalation") == 0)
        return SYSTEM_HEALTH_INCIDENT_ESCALATE;
    if (strcmp(value, "material_change") == 0)
        return SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE;
    if (strcmp(value, "recovery") == 0)
        return SYSTEM_HEALTH_INCIDENT_RECOVER;
    if (strcmp(value, "one_shot") == 0)
        return SYSTEM_HEALTH_INCIDENT_ONE_SHOT;
    *valid = false;
    return SYSTEM_HEALTH_INCIDENT_OPEN;
}

static system_health_scope_t parse_scope(const char *value, bool *valid) {
    *valid = true;
    if (strcmp(value, "process") == 0) return SYSTEM_HEALTH_SCOPE_PROCESS;
    if (strcmp(value, "container") == 0)
        return SYSTEM_HEALTH_SCOPE_CONTAINER;
    if (strcmp(value, "host") == 0) return SYSTEM_HEALTH_SCOPE_HOST;
    if (strcmp(value, "filesystem") == 0)
        return SYSTEM_HEALTH_SCOPE_FILESYSTEM;
    if (strcmp(value, "device") == 0) return SYSTEM_HEALTH_SCOPE_DEVICE;
    *valid = false;
    return SYSTEM_HEALTH_SCOPE_HOST;
}

static void copy_column(char *destination, size_t capacity,
                        sqlite3_stmt *statement, int column) {
    const char *text = (const char *)sqlite3_column_text(statement, column);
    safe_strcpy(destination, text ? text : "", capacity, 0);
}

static bool populate_incident(sqlite3_stmt *statement,
                              system_health_incident_record_t *incident) {
    bool scope_valid;
    bool state_valid;
    memset(incident, 0, sizeof(*incident));
    copy_column(incident->uuid, sizeof(incident->uuid), statement, 0);
    copy_column(incident->condition_code, sizeof(incident->condition_code),
                statement, 1);
    copy_column(incident->subject, sizeof(incident->subject), statement, 2);
    incident->scope = parse_scope(
        (const char *)sqlite3_column_text(statement, 3), &scope_valid);
    incident->state = parse_state(
        (const char *)sqlite3_column_text(statement, 4), &state_valid);
    incident->severity = parse_severity(
        (const char *)sqlite3_column_text(statement, 5));
    incident->first_seen_at_ms = sqlite3_column_int64(statement, 6);
    incident->last_seen_at_ms = sqlite3_column_int64(statement, 7);
    incident->closed_at_ms = sqlite3_column_int64(statement, 8);
    copy_column(incident->observation_json, sizeof(incident->observation_json),
                statement, 9);
    copy_column(incident->alert_event_id, sizeof(incident->alert_event_id),
                statement, 10);
    copy_column(incident->recovery_event_id,
                sizeof(incident->recovery_event_id), statement, 11);
    incident->reconciliation = parse_reconciliation(
        (const char *)sqlite3_column_text(statement, 12));
    copy_column(incident->boot_id, sizeof(incident->boot_id), statement, 13);
    copy_column(incident->run_id, sizeof(incident->run_id), statement, 14);
    incident->revision = sqlite3_column_int64(statement, 15);
    return scope_valid && state_valid &&
           incident->severity != SYSTEM_HEALTH_SEVERITY_NONE;
}

static int get_active_locked(sqlite3 *database, const char *condition_code,
                             const char *subject,
                             system_health_incident_record_t *incident) {
    sqlite3_stmt *statement = NULL;
    const char *sql = "SELECT " INCIDENT_FIELDS
        " FROM system_health_incidents WHERE condition_code=? AND subject=? "
        "AND state<>'closed' LIMIT 1;";
    int result = sqlite3_prepare_v2(database, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, condition_code, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, subject, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    int found = 0;
    if (result == SQLITE_ROW)
        found = populate_incident(statement, incident) ? 1 : -1;
    else if (result != SQLITE_DONE)
        found = -1;
    if (statement) sqlite3_finalize(statement);
    return found;
}

static int get_uuid_locked(sqlite3 *database, const char *uuid,
                           system_health_incident_record_t *incident) {
    sqlite3_stmt *statement = NULL;
    const char *sql = "SELECT " INCIDENT_FIELDS
        " FROM system_health_incidents WHERE uuid=? LIMIT 1;";
    int result = sqlite3_prepare_v2(database, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    int found = 0;
    if (result == SQLITE_ROW)
        found = populate_incident(statement, incident) ? 1 : -1;
    else if (result != SQLITE_DONE)
        found = -1;
    if (statement) sqlite3_finalize(statement);
    return found;
}

static bool valid_signal(system_health_incident_action_t action,
                         const system_health_incident_signal_t *signal,
                         char normalized[SYSTEM_HEALTH_INCIDENT_OBSERVATION_MAX]) {
    const char *condition_code;
    if (!signal || !action_name(action) || signal->observed_at_ms <= 0 ||
        !(condition_code = system_health_condition_code(signal->condition)) ||
        !valid_logical_id(signal->subject, sizeof(signal->subject)) ||
        signal->scope < SYSTEM_HEALTH_SCOPE_PROCESS ||
        signal->scope >= SYSTEM_HEALTH_SCOPE_COUNT ||
        !reconciliation_name(signal->reconciliation) ||
        !valid_logical_id(signal->boot_id, sizeof(signal->boot_id)) ||
        !lightnvr_uuid_is_valid(signal->run_id) ||
        (signal->event_id[0] &&
         !lightnvr_uuid_is_valid(signal->event_id)) ||
        !normalize_observation(signal->observation_json, normalized)) {
        return false;
    }
    (void)condition_code;
    if (action == SYSTEM_HEALTH_INCIDENT_RECOVER)
        return signal->state == SYSTEM_HEALTH_STATE_CLOSED &&
               (signal->severity == SYSTEM_HEALTH_SEVERITY_NONE ||
                severity_name(signal->severity));
    if (!severity_name(signal->severity)) return false;
    if (action == SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE)
        return signal->state == SYSTEM_HEALTH_STATE_OPEN ||
               signal->state == SYSTEM_HEALTH_STATE_RECOVERING;
    if (action == SYSTEM_HEALTH_INCIDENT_ONE_SHOT)
        return signal->state == SYSTEM_HEALTH_STATE_CLOSED;
    return signal->state == SYSTEM_HEALTH_STATE_OPEN;
}

static int insert_incident_locked(
    sqlite3 *database, const char *uuid, const char *condition_code,
    const system_health_incident_signal_t *signal, const char *normalized,
    bool closed) {
    const char *sql =
        "INSERT INTO system_health_incidents("
        "uuid,condition_code,subject,scope,state,severity,first_seen_at_ms,"
        "last_seen_at_ms,closed_at_ms,last_observation_json,alert_event_id,"
        "recovery_event_id,reconciliation_state,boot_id,run_id) "
        "VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?);";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(database, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, condition_code, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 3, signal->subject, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 4, system_health_scope_name(signal->scope),
                          -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 5, closed ? "closed" : "open", -1,
                          SQLITE_STATIC);
        sqlite3_bind_text(statement, 6, severity_name(signal->severity), -1,
                          SQLITE_STATIC);
        sqlite3_bind_int64(statement, 7, signal->observed_at_ms);
        sqlite3_bind_int64(statement, 8, signal->observed_at_ms);
        if (closed) sqlite3_bind_int64(statement, 9, signal->observed_at_ms);
        else sqlite3_bind_null(statement, 9);
        sqlite3_bind_text(statement, 10, normalized, -1, SQLITE_TRANSIENT);
        if (signal->event_id[0])
            sqlite3_bind_text(statement, 11, signal->event_id, -1,
                              SQLITE_TRANSIENT);
        else sqlite3_bind_null(statement, 11);
        sqlite3_bind_null(statement, 12);
        sqlite3_bind_text(statement, 13,
            reconciliation_name(signal->reconciliation), -1, SQLITE_STATIC);
        sqlite3_bind_text(statement, 14, signal->boot_id, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 15, signal->run_id, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    return result == SQLITE_DONE ? 0 : -1;
}

static int insert_transition_locked(
    sqlite3 *database, const char *transition_uuid, const char *incident_uuid,
    system_health_incident_action_t action, const char *from_state,
    const char *to_state, system_health_severity_t severity,
    const system_health_incident_signal_t *signal, const char *normalized) {
    const char *sql =
        "INSERT INTO system_health_incident_transitions("
        "transition_uuid,incident_uuid,kind,from_state,to_state,severity,"
        "observed_at_ms,safe_observation_json,event_id,reconciliation_state,"
        "boot_id,run_id) VALUES(?,?,?,?,?,?,?,?,?,?,?,?);";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(database, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, transition_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, incident_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 3, action_name(action), -1, SQLITE_STATIC);
        if (from_state)
            sqlite3_bind_text(statement, 4, from_state, -1, SQLITE_STATIC);
        else sqlite3_bind_null(statement, 4);
        sqlite3_bind_text(statement, 5, to_state, -1, SQLITE_STATIC);
        sqlite3_bind_text(statement, 6, severity_name(severity), -1,
                          SQLITE_STATIC);
        sqlite3_bind_int64(statement, 7, signal->observed_at_ms);
        sqlite3_bind_text(statement, 8, normalized, -1, SQLITE_TRANSIENT);
        if (signal->event_id[0])
            sqlite3_bind_text(statement, 9, signal->event_id, -1,
                              SQLITE_TRANSIENT);
        else sqlite3_bind_null(statement, 9);
        sqlite3_bind_text(statement, 10,
            reconciliation_name(signal->reconciliation), -1, SQLITE_STATIC);
        sqlite3_bind_text(statement, 11, signal->boot_id, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 12, signal->run_id, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    return result == SQLITE_DONE ? 0 : -1;
}

static int update_resume_locked(
    sqlite3 *database, const system_health_incident_record_t *existing,
    const system_health_incident_signal_t *signal, const char *normalized,
    bool reopen) {
    const char *sql =
        "UPDATE system_health_incidents SET state=?,last_seen_at_ms=?,"
        "last_observation_json=?,alert_event_id=COALESCE(?,alert_event_id),"
        "reconciliation_state=?,boot_id=?,run_id=?,revision=revision+1 "
        "WHERE uuid=? AND revision=?;";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(database, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, reopen ? "open" : state_name(existing->state),
                          -1, SQLITE_STATIC);
        sqlite3_bind_int64(statement, 2, signal->observed_at_ms);
        sqlite3_bind_text(statement, 3, normalized, -1, SQLITE_TRANSIENT);
        if (signal->event_id[0])
            sqlite3_bind_text(statement, 4, signal->event_id, -1,
                              SQLITE_TRANSIENT);
        else sqlite3_bind_null(statement, 4);
        sqlite3_bind_text(statement, 5,
            reconciliation_name(signal->reconciliation), -1, SQLITE_STATIC);
        sqlite3_bind_text(statement, 6, signal->boot_id, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 7, signal->run_id, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 8, existing->uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 9, existing->revision);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(database) : 0;
    if (statement) sqlite3_finalize(statement);
    return result == SQLITE_DONE && changed == 1 ? 0 : -1;
}

static int update_transition_locked(
    sqlite3 *database, const system_health_incident_record_t *existing,
    system_health_incident_action_t action,
    const system_health_incident_signal_t *signal, const char *normalized,
    system_health_state_t target_state,
    system_health_severity_t target_severity) {
    const char *sql =
        "UPDATE system_health_incidents SET state=?,severity=?,"
        "last_seen_at_ms=?,closed_at_ms=?,last_observation_json=?,"
        "alert_event_id=CASE WHEN ?=0 THEN COALESCE(?,alert_event_id) ELSE alert_event_id END,"
        "recovery_event_id=CASE WHEN ?=1 THEN COALESCE(?,recovery_event_id) ELSE recovery_event_id END,"
        "reconciliation_state=?,boot_id=?,run_id=?,revision=revision+1 "
        "WHERE uuid=? AND revision=?;";
    bool recovery = action == SYSTEM_HEALTH_INCIDENT_RECOVER;
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(database, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, state_name(target_state), -1,
                          SQLITE_STATIC);
        sqlite3_bind_text(statement, 2, severity_name(target_severity), -1,
                          SQLITE_STATIC);
        sqlite3_bind_int64(statement, 3, signal->observed_at_ms);
        if (recovery) sqlite3_bind_int64(statement, 4, signal->observed_at_ms);
        else sqlite3_bind_null(statement, 4);
        sqlite3_bind_text(statement, 5, normalized, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 6, recovery ? 1 : 0);
        if (signal->event_id[0])
            sqlite3_bind_text(statement, 7, signal->event_id, -1,
                              SQLITE_TRANSIENT);
        else sqlite3_bind_null(statement, 7);
        sqlite3_bind_int(statement, 8, recovery ? 1 : 0);
        if (signal->event_id[0])
            sqlite3_bind_text(statement, 9, signal->event_id, -1,
                              SQLITE_TRANSIENT);
        else sqlite3_bind_null(statement, 9);
        sqlite3_bind_text(statement, 10,
            reconciliation_name(signal->reconciliation), -1, SQLITE_STATIC);
        sqlite3_bind_text(statement, 11, signal->boot_id, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 12, signal->run_id, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 13, existing->uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 14, existing->revision);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(database) : 0;
    if (statement) sqlite3_finalize(statement);
    return result == SQLITE_DONE && changed == 1 ? 0 : -1;
}

db_system_health_result_t db_system_health_incident_apply(
    system_health_incident_action_t action,
    const system_health_incident_signal_t *signal,
    system_health_incident_record_t *incident_out) {
    char normalized[SYSTEM_HEALTH_INCIDENT_OBSERVATION_MAX];
    if (!incident_out || !valid_signal(action, signal, normalized))
        return DB_SYSTEM_HEALTH_INVALID;
    sqlite3 *database = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!database || !mutex) return DB_SYSTEM_HEALTH_ERROR;
    const char *condition_code = system_health_condition_code(signal->condition);
    char incident_uuid[LIGHTNVR_UUID_STRING_SIZE];
    char transition_uuid[LIGHTNVR_UUID_STRING_SIZE];
    if (lightnvr_uuid_generate_v4(incident_uuid) != 0 ||
        lightnvr_uuid_generate_v4(transition_uuid) != 0)
        return DB_SYSTEM_HEALTH_ERROR;

    pthread_mutex_lock(mutex);
    if (!transaction_begin(database)) {
        pthread_mutex_unlock(mutex);
        return DB_SYSTEM_HEALTH_ERROR;
    }
    system_health_incident_record_t existing;
    int found = get_active_locked(database, condition_code, signal->subject,
                                  &existing);
    db_system_health_result_t outcome = DB_SYSTEM_HEALTH_ERROR;
    bool success = false;

    if (found < 0) goto finish;
    if (action == SYSTEM_HEALTH_INCIDENT_ONE_SHOT) {
        if (insert_incident_locked(database, incident_uuid, condition_code,
                                   signal, normalized, true) != 0 ||
            insert_transition_locked(database, transition_uuid, incident_uuid,
                action, NULL, "closed", signal->severity, signal,
                normalized) != 0)
            goto finish;
        success = get_uuid_locked(database, incident_uuid, incident_out) == 1;
        outcome = success ? DB_SYSTEM_HEALTH_OK : DB_SYSTEM_HEALTH_ERROR;
        goto finish;
    }
    if (action == SYSTEM_HEALTH_INCIDENT_OPEN) {
        if (found == 0) {
            if (insert_incident_locked(database, incident_uuid, condition_code,
                                       signal, normalized, false) != 0 ||
                insert_transition_locked(database, transition_uuid,
                    incident_uuid, action, NULL, "open", signal->severity,
                    signal, normalized) != 0)
                goto finish;
            success = get_uuid_locked(database, incident_uuid, incident_out) == 1;
            outcome = success ? DB_SYSTEM_HEALTH_OK : DB_SYSTEM_HEALTH_ERROR;
        } else if (signal->observed_at_ms < existing.last_seen_at_ms ||
                   signal->scope != existing.scope ||
                   signal->severity != existing.severity) {
            outcome = DB_SYSTEM_HEALTH_CONFLICT;
        } else {
            bool reopen = existing.state == SYSTEM_HEALTH_STATE_RECOVERING;
            if (update_resume_locked(database, &existing, signal, normalized,
                                     reopen) != 0)
                goto finish;
            if (reopen && insert_transition_locked(
                    database, transition_uuid, existing.uuid,
                    SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE, "recovering",
                    "open", existing.severity, signal, normalized) != 0)
                goto finish;
            success = get_uuid_locked(database, existing.uuid, incident_out) == 1;
            outcome = success ? DB_SYSTEM_HEALTH_RESUMED
                              : DB_SYSTEM_HEALTH_ERROR;
        }
        goto finish;
    }
    if (found == 0) {
        outcome = DB_SYSTEM_HEALTH_NOT_FOUND;
        goto finish;
    }
    if (signal->observed_at_ms < existing.last_seen_at_ms ||
        signal->scope != existing.scope) {
        outcome = DB_SYSTEM_HEALTH_CONFLICT;
        goto finish;
    }
    system_health_state_t target_state = existing.state;
    system_health_severity_t target_severity = existing.severity;
    if (action == SYSTEM_HEALTH_INCIDENT_ESCALATE) {
        if (signal->severity <= existing.severity) {
            outcome = DB_SYSTEM_HEALTH_CONFLICT;
            goto finish;
        }
        target_state = SYSTEM_HEALTH_STATE_OPEN;
        target_severity = signal->severity;
    } else if (action == SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE) {
        if (signal->severity != existing.severity) {
            outcome = DB_SYSTEM_HEALTH_CONFLICT;
            goto finish;
        }
        target_state = signal->state;
    } else if (action == SYSTEM_HEALTH_INCIDENT_RECOVER) {
        if (signal->severity != SYSTEM_HEALTH_SEVERITY_NONE &&
            signal->severity != existing.severity) {
            outcome = DB_SYSTEM_HEALTH_CONFLICT;
            goto finish;
        }
        target_state = SYSTEM_HEALTH_STATE_CLOSED;
    } else {
        outcome = DB_SYSTEM_HEALTH_INVALID;
        goto finish;
    }
    if (update_transition_locked(database, &existing, action, signal,
                                 normalized, target_state,
                                 target_severity) != 0 ||
        insert_transition_locked(database, transition_uuid, existing.uuid,
            action, state_name(existing.state), state_name(target_state),
            target_severity, signal, normalized) != 0)
        goto finish;
    success = get_uuid_locked(database, existing.uuid, incident_out) == 1;
    outcome = success ? DB_SYSTEM_HEALTH_OK : DB_SYSTEM_HEALTH_ERROR;

finish:
    success = success && transaction_finish(database, true);
    if (!success) transaction_finish(database, false);
    pthread_mutex_unlock(mutex);
    return success ? outcome :
        (outcome == DB_SYSTEM_HEALTH_CONFLICT ||
         outcome == DB_SYSTEM_HEALTH_NOT_FOUND ||
         outcome == DB_SYSTEM_HEALTH_INVALID ? outcome : DB_SYSTEM_HEALTH_ERROR);
}

db_system_health_result_t db_system_health_incident_get_active(
    system_health_condition_t condition, const char *subject,
    system_health_incident_record_t *incident_out) {
    const char *code = system_health_condition_code(condition);
    if (!code || !valid_logical_id(subject, SYSTEM_HEALTH_INCIDENT_SUBJECT_MAX) ||
        !incident_out) return DB_SYSTEM_HEALTH_INVALID;
    sqlite3 *database = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!database || !mutex) return DB_SYSTEM_HEALTH_ERROR;
    pthread_mutex_lock(mutex);
    int found = get_active_locked(database, code, subject, incident_out);
    pthread_mutex_unlock(mutex);
    return found > 0 ? DB_SYSTEM_HEALTH_OK
         : found == 0 ? DB_SYSTEM_HEALTH_NOT_FOUND : DB_SYSTEM_HEALTH_ERROR;
}

db_system_health_result_t db_system_health_incident_get_uuid(
    const char *incident_uuid,
    system_health_incident_record_t *incident_out) {
    if (!lightnvr_uuid_is_valid(incident_uuid) || !incident_out)
        return DB_SYSTEM_HEALTH_INVALID;
    sqlite3 *database = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!database || !mutex) return DB_SYSTEM_HEALTH_ERROR;
    pthread_mutex_lock(mutex);
    int found = get_uuid_locked(database, incident_uuid, incident_out);
    pthread_mutex_unlock(mutex);
    return found > 0 ? DB_SYSTEM_HEALTH_OK
         : found == 0 ? DB_SYSTEM_HEALTH_NOT_FOUND : DB_SYSTEM_HEALTH_ERROR;
}

int db_system_health_incident_list(
    bool include_closed, const system_health_incident_cursor_t *cursor,
    system_health_incident_record_t *incidents, int max_count,
    system_health_incident_cursor_t *next_cursor) {
    if (!incidents || !next_cursor || max_count < 1 ||
        max_count > SYSTEM_HEALTH_INCIDENT_PAGE_MAX ||
        (cursor && cursor->valid &&
         (cursor->last_seen_at_ms <= 0 ||
          !lightnvr_uuid_is_valid(cursor->uuid)))) return -1;
    sqlite3 *database = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!database || !mutex) return -1;
    const char *sql =
        "SELECT " INCIDENT_FIELDS " FROM system_health_incidents WHERE "
        "(? OR state<>'closed') AND "
        "(?=0 OR last_seen_at_ms<? OR (last_seen_at_ms=? AND uuid<?)) "
        "ORDER BY last_seen_at_ms DESC,uuid DESC LIMIT ?;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(database, sql, -1, &statement, NULL);
    bool has_cursor = cursor && cursor->valid;
    if (result == SQLITE_OK) {
        sqlite3_bind_int(statement, 1, include_closed ? 1 : 0);
        sqlite3_bind_int(statement, 2, has_cursor ? 1 : 0);
        sqlite3_bind_int64(statement, 3,
            has_cursor ? cursor->last_seen_at_ms : 0);
        sqlite3_bind_int64(statement, 4,
            has_cursor ? cursor->last_seen_at_ms : 0);
        sqlite3_bind_text(statement, 5, has_cursor ? cursor->uuid : "", -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 6, max_count + 1);
    }
    int count = 0;
    bool more = false;
    while (result == SQLITE_OK || result == SQLITE_ROW) {
        result = sqlite3_step(statement);
        if (result != SQLITE_ROW) break;
        system_health_incident_record_t row;
        if (!populate_incident(statement, &row)) {
            result = SQLITE_ERROR;
            break;
        }
        if (count == max_count) {
            more = true;
            break;
        }
        incidents[count++] = row;
    }
    if (statement) sqlite3_finalize(statement);
    memset(next_cursor, 0, sizeof(*next_cursor));
    if (more && count > 0) {
        next_cursor->valid = true;
        next_cursor->last_seen_at_ms = incidents[count - 1].last_seen_at_ms;
        safe_strcpy(next_cursor->uuid, incidents[count - 1].uuid,
                    sizeof(next_cursor->uuid), 0);
    }
    pthread_mutex_unlock(mutex);
    return result == SQLITE_DONE || more ? count : -1;
}

static bool populate_transition(
    sqlite3_stmt *statement, system_health_incident_transition_t *transition) {
    bool action_valid;
    bool from_valid = true;
    bool to_valid;
    memset(transition, 0, sizeof(*transition));
    transition->id = sqlite3_column_int64(statement, 0);
    copy_column(transition->transition_uuid,
                sizeof(transition->transition_uuid), statement, 1);
    copy_column(transition->incident_uuid, sizeof(transition->incident_uuid),
                statement, 2);
    transition->action = parse_action(
        (const char *)sqlite3_column_text(statement, 3), &action_valid);
    const char *from = (const char *)sqlite3_column_text(statement, 4);
    transition->from_state_valid = from && from[0];
    if (transition->from_state_valid)
        transition->from_state = parse_state(from, &from_valid);
    transition->to_state = parse_state(
        (const char *)sqlite3_column_text(statement, 5), &to_valid);
    transition->severity = parse_severity(
        (const char *)sqlite3_column_text(statement, 6));
    transition->observed_at_ms = sqlite3_column_int64(statement, 7);
    copy_column(transition->observation_json,
                sizeof(transition->observation_json), statement, 8);
    copy_column(transition->event_id, sizeof(transition->event_id), statement, 9);
    transition->reconciliation = parse_reconciliation(
        (const char *)sqlite3_column_text(statement, 10));
    copy_column(transition->boot_id, sizeof(transition->boot_id), statement, 11);
    copy_column(transition->run_id, sizeof(transition->run_id), statement, 12);
    return action_valid && from_valid && to_valid &&
           transition->severity != SYSTEM_HEALTH_SEVERITY_NONE;
}

int db_system_health_transition_list(
    const char *incident_uuid, const system_health_transition_cursor_t *cursor,
    system_health_incident_transition_t *transitions, int max_count,
    system_health_transition_cursor_t *next_cursor) {
    if (!lightnvr_uuid_is_valid(incident_uuid) || !transitions || !next_cursor ||
        max_count < 1 || max_count > SYSTEM_HEALTH_INCIDENT_PAGE_MAX ||
        (cursor && cursor->valid &&
         (cursor->observed_at_ms <= 0 || cursor->id <= 0))) return -1;
    sqlite3 *database = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!database || !mutex) return -1;
    const char *sql =
        "SELECT " TRANSITION_FIELDS
        " FROM system_health_incident_transitions WHERE incident_uuid=? AND "
        "(?=0 OR observed_at_ms<? OR (observed_at_ms=? AND id<?)) "
        "ORDER BY observed_at_ms DESC,id DESC LIMIT ?;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(database, sql, -1, &statement, NULL);
    bool has_cursor = cursor && cursor->valid;
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, incident_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 2, has_cursor ? 1 : 0);
        sqlite3_bind_int64(statement, 3,
            has_cursor ? cursor->observed_at_ms : 0);
        sqlite3_bind_int64(statement, 4,
            has_cursor ? cursor->observed_at_ms : 0);
        sqlite3_bind_int64(statement, 5, has_cursor ? cursor->id : 0);
        sqlite3_bind_int(statement, 6, max_count + 1);
    }
    int count = 0;
    bool more = false;
    while (result == SQLITE_OK || result == SQLITE_ROW) {
        result = sqlite3_step(statement);
        if (result != SQLITE_ROW) break;
        system_health_incident_transition_t row;
        if (!populate_transition(statement, &row)) {
            result = SQLITE_ERROR;
            break;
        }
        if (count == max_count) {
            more = true;
            break;
        }
        transitions[count++] = row;
    }
    if (statement) sqlite3_finalize(statement);
    memset(next_cursor, 0, sizeof(*next_cursor));
    if (more && count > 0) {
        next_cursor->valid = true;
        next_cursor->observed_at_ms = transitions[count - 1].observed_at_ms;
        next_cursor->id = transitions[count - 1].id;
    }
    pthread_mutex_unlock(mutex);
    return result == SQLITE_DONE || more ? count : -1;
}

int db_system_health_transition_list_pending(
    int64_t after_id, system_health_incident_transition_t *transitions,
    int max_count) {
    if (after_id < 0 || !transitions || max_count < 1 ||
        max_count > SYSTEM_HEALTH_INCIDENT_PAGE_MAX) return -1;
    sqlite3 *database = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!database || !mutex) return -1;
    const char *sql =
        "SELECT " TRANSITION_FIELDS
        " FROM system_health_incident_transitions "
        "WHERE id>? AND event_id IS NOT NULL AND "
        "reconciliation_state IN ('alert_pending','recovery_pending') "
        "ORDER BY id ASC LIMIT ?;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(database, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_int64(statement, 1, after_id);
        sqlite3_bind_int(statement, 2, max_count);
    }
    int count = 0;
    while (result == SQLITE_OK || result == SQLITE_ROW) {
        result = sqlite3_step(statement);
        if (result != SQLITE_ROW) break;
        if (!populate_transition(statement, &transitions[count++])) {
            result = SQLITE_ERROR;
            break;
        }
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return result == SQLITE_DONE ? count : -1;
}

db_system_health_result_t db_system_health_transition_set_reconciliation(
    const char *incident_uuid, const char *event_id,
    system_health_reconciliation_t reconciliation) {
    if (!lightnvr_uuid_is_valid(incident_uuid) ||
        !lightnvr_uuid_is_valid(event_id) ||
        !reconciliation_name(reconciliation)) return DB_SYSTEM_HEALTH_INVALID;
    sqlite3 *database = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!database || !mutex) return DB_SYSTEM_HEALTH_ERROR;
    const char *transition_sql =
        "UPDATE system_health_incident_transitions "
        "SET reconciliation_state=? WHERE incident_uuid=? AND event_id=?;";
    pthread_mutex_lock(mutex);
    if (!transaction_begin(database)) {
        pthread_mutex_unlock(mutex);
        return DB_SYSTEM_HEALTH_ERROR;
    }
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(database, transition_sql, -1, &statement,
                                    NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, reconciliation_name(reconciliation),
                          -1, SQLITE_STATIC);
        sqlite3_bind_text(statement, 2, incident_uuid, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 3, event_id, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(database) : 0;
    if (statement) sqlite3_finalize(statement);
    statement = NULL;
    if (result == SQLITE_DONE && changed == 1) {
        const char *incident_sql =
            "UPDATE system_health_incidents SET reconciliation_state=COALESCE("
            "(SELECT reconciliation_state FROM "
            "system_health_incident_transitions WHERE incident_uuid=? AND "
            "reconciliation_state IN ('alert_pending','recovery_pending') "
            "ORDER BY id DESC LIMIT 1),?),revision=revision+1 WHERE uuid=?;";
        result = sqlite3_prepare_v2(database, incident_sql, -1, &statement,
                                    NULL);
        if (result == SQLITE_OK) {
            sqlite3_bind_text(statement, 1, incident_uuid, -1,
                              SQLITE_TRANSIENT);
            sqlite3_bind_text(statement, 2,
                              reconciliation_name(reconciliation), -1,
                              SQLITE_STATIC);
            sqlite3_bind_text(statement, 3, incident_uuid, -1,
                              SQLITE_TRANSIENT);
            result = sqlite3_step(statement);
        }
        changed = result == SQLITE_DONE ? sqlite3_changes(database) : 0;
        if (statement) sqlite3_finalize(statement);
    }
    bool success = result == SQLITE_DONE && changed == 1 &&
                   transaction_finish(database, true);
    if (!success) transaction_finish(database, false);
    pthread_mutex_unlock(mutex);
    return success ? DB_SYSTEM_HEALTH_OK
         : result == SQLITE_DONE && changed == 0
             ? DB_SYSTEM_HEALTH_NOT_FOUND : DB_SYSTEM_HEALTH_ERROR;
}

db_system_health_result_t db_system_health_incident_set_reconciliation(
    const char *incident_uuid, bool recovery_event, const char *event_id,
    system_health_reconciliation_t reconciliation) {
    if (!lightnvr_uuid_is_valid(incident_uuid) ||
        (event_id && event_id[0] && !lightnvr_uuid_is_valid(event_id)) ||
        !reconciliation_name(reconciliation)) return DB_SYSTEM_HEALTH_INVALID;
    sqlite3 *database = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!database || !mutex) return DB_SYSTEM_HEALTH_ERROR;
    const char *sql = recovery_event
        ? "UPDATE system_health_incidents SET recovery_event_id=?,"
          "reconciliation_state=?,revision=revision+1 WHERE uuid=?;"
        : "UPDATE system_health_incidents SET alert_event_id=?,"
          "reconciliation_state=?,revision=revision+1 WHERE uuid=?;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(database, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        if (event_id && event_id[0])
            sqlite3_bind_text(statement, 1, event_id, -1, SQLITE_TRANSIENT);
        else sqlite3_bind_null(statement, 1);
        sqlite3_bind_text(statement, 2, reconciliation_name(reconciliation),
                          -1, SQLITE_STATIC);
        sqlite3_bind_text(statement, 3, incident_uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(database) : 0;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return result != SQLITE_DONE ? DB_SYSTEM_HEALTH_ERROR
         : changed == 1 ? DB_SYSTEM_HEALTH_OK : DB_SYSTEM_HEALTH_NOT_FOUND;
}

int db_system_health_transition_prune(int64_t closed_before_ms, int limit,
                                      int *deleted_count) {
    if (closed_before_ms <= 0 || limit < 1 ||
        limit > SYSTEM_HEALTH_TRANSITION_RETENTION_BATCH_MAX ||
        !deleted_count) return -1;
    sqlite3 *database = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!database || !mutex) return -1;
    const char *sql =
        "DELETE FROM system_health_incident_transitions WHERE id IN ("
        "SELECT t.id FROM system_health_incident_transitions t "
        "JOIN system_health_incidents i ON i.uuid=t.incident_uuid "
        "WHERE i.state='closed' AND t.observed_at_ms<? "
        "ORDER BY t.observed_at_ms,t.id LIMIT ?);";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(database, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_int64(statement, 1, closed_before_ms);
        sqlite3_bind_int(statement, 2, limit);
        result = sqlite3_step(statement);
    }
    *deleted_count = result == SQLITE_DONE ? sqlite3_changes(database) : 0;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return result == SQLITE_DONE ? 0 : -1;
}

static bool populate_run(sqlite3_stmt *statement,
                         system_health_process_run_t *run) {
    memset(run, 0, sizeof(*run));
    copy_column(run->run_id, sizeof(run->run_id), statement, 0);
    copy_column(run->boot_id, sizeof(run->boot_id), statement, 1);
    run->started_at_ms = sqlite3_column_int64(statement, 2);
    run->closed_at_ms = sqlite3_column_int64(statement, 3);
    run->clean_close = sqlite3_column_int(statement, 4) != 0;
    return lightnvr_uuid_is_valid(run->run_id) &&
           valid_logical_id(run->boot_id, sizeof(run->boot_id));
}

static int latest_run_locked(sqlite3 *database,
                             system_health_process_run_t *run) {
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        database, "SELECT run_id,boot_id,started_at_ms,"
                  "COALESCE(closed_at_ms,0),clean_close "
                  "FROM system_health_process_runs "
                  "ORDER BY started_at_ms DESC,run_id DESC LIMIT 1;",
        -1, &statement, NULL);
    if (result == SQLITE_OK) result = sqlite3_step(statement);
    int found = 0;
    if (result == SQLITE_ROW) found = populate_run(statement, run) ? 1 : -1;
    else if (result != SQLITE_DONE) found = -1;
    if (statement) sqlite3_finalize(statement);
    return found;
}

db_system_health_result_t db_system_health_run_open(
    const char *run_id, const char *boot_id, int64_t started_at_ms,
    system_health_process_run_t *run_out,
    system_health_process_run_t *previous_out, bool *had_previous) {
    if (!run_out || !previous_out || !had_previous ||
        !valid_logical_id(boot_id, SYSTEM_HEALTH_INCIDENT_BOOT_ID_MAX) ||
        started_at_ms <= 0 || (run_id && !lightnvr_uuid_is_valid(run_id)))
        return DB_SYSTEM_HEALTH_INVALID;
    char generated[LIGHTNVR_UUID_STRING_SIZE];
    if (!run_id) {
        if (lightnvr_uuid_generate_v4(generated) != 0)
            return DB_SYSTEM_HEALTH_ERROR;
        run_id = generated;
    }
    sqlite3 *database = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!database || !mutex) return DB_SYSTEM_HEALTH_ERROR;
    pthread_mutex_lock(mutex);
    if (!transaction_begin(database)) {
        pthread_mutex_unlock(mutex);
        return DB_SYSTEM_HEALTH_ERROR;
    }
    int previous = latest_run_locked(database, previous_out);
    sqlite3_stmt *statement = NULL;
    int result = previous < 0 ? SQLITE_ERROR : sqlite3_prepare_v2(
        database, "INSERT INTO system_health_process_runs("
                  "run_id,boot_id,started_at_ms) VALUES(?,?,?);",
        -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, run_id, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, boot_id, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 3, started_at_ms);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    bool success = result == SQLITE_DONE;
    if (success) {
        safe_strcpy(run_out->run_id, run_id, sizeof(run_out->run_id), 0);
        safe_strcpy(run_out->boot_id, boot_id, sizeof(run_out->boot_id), 0);
        run_out->started_at_ms = started_at_ms;
        run_out->closed_at_ms = 0;
        run_out->clean_close = false;
    }
    success = transaction_finish(database, success);
    pthread_mutex_unlock(mutex);
    if (!success) return result == SQLITE_CONSTRAINT
        ? DB_SYSTEM_HEALTH_CONFLICT : DB_SYSTEM_HEALTH_ERROR;
    *had_previous = previous > 0;
    if (!*had_previous) memset(previous_out, 0, sizeof(*previous_out));
    return DB_SYSTEM_HEALTH_OK;
}

db_system_health_result_t db_system_health_run_close(
    const char *run_id, int64_t closed_at_ms) {
    if (!lightnvr_uuid_is_valid(run_id) || closed_at_ms <= 0)
        return DB_SYSTEM_HEALTH_INVALID;
    sqlite3 *database = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!database || !mutex) return DB_SYSTEM_HEALTH_ERROR;
    pthread_mutex_lock(mutex);
    if (!transaction_begin(database)) {
        pthread_mutex_unlock(mutex);
        return DB_SYSTEM_HEALTH_ERROR;
    }
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        database, "UPDATE system_health_process_runs SET closed_at_ms=?,"
                  "clean_close=1 WHERE run_id=? AND clean_close=0 "
                  "AND started_at_ms<=?;", -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_int64(statement, 1, closed_at_ms);
        sqlite3_bind_text(statement, 2, run_id, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 3, closed_at_ms);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(database) : 0;
    if (statement) sqlite3_finalize(statement);
    bool success = result == SQLITE_DONE;
    success = transaction_finish(database, success);
    pthread_mutex_unlock(mutex);
    if (!success) return DB_SYSTEM_HEALTH_ERROR;
    return changed == 1 ? DB_SYSTEM_HEALTH_OK : DB_SYSTEM_HEALTH_NOT_FOUND;
}

db_system_health_result_t db_system_health_run_latest(
    system_health_process_run_t *run_out) {
    if (!run_out) return DB_SYSTEM_HEALTH_INVALID;
    sqlite3 *database = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!database || !mutex) return DB_SYSTEM_HEALTH_ERROR;
    pthread_mutex_lock(mutex);
    int found = latest_run_locked(database, run_out);
    pthread_mutex_unlock(mutex);
    return found > 0 ? DB_SYSTEM_HEALTH_OK
         : found == 0 ? DB_SYSTEM_HEALTH_NOT_FOUND : DB_SYSTEM_HEALTH_ERROR;
}
