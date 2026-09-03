#define _POSIX_C_SOURCE 200809L

#include <sqlite3.h>
#include <pthread.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "database/db_core.h"
#include "database/db_system_health_incidents.h"
#include "unity.h"
#include "utils/strings.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_system_health_incidents.db"

#ifdef SYSTEM_HEALTH_STANDALONE_TEST
#ifndef TEST_MIGRATION_PATH
#error TEST_MIGRATION_PATH is required for the standalone scoped test
#endif
static sqlite3 *standalone_database;
static pthread_mutex_t standalone_mutex = PTHREAD_MUTEX_INITIALIZER;

sqlite3 *get_db_handle(void) { return standalone_database; }
pthread_mutex_t *get_db_mutex(void) { return &standalone_mutex; }
int safe_strcpy(char *destination, const char *source, size_t destination_size,
                size_t source_size) {
    if (!destination || !source || destination_size == 0) return -1;
    size_t length = source_size ? strnlen(source, source_size) : strlen(source);
    if (length >= destination_size) length = destination_size - 1U;
    memcpy(destination, source, length);
    destination[length] = '\0';
    return 0;
}

int init_database(const char *path) {
    FILE *file = fopen(TEST_MIGRATION_PATH, "rb");
    if (!file || sqlite3_open(path, &standalone_database) != SQLITE_OK) {
        if (file) fclose(file);
        return -1;
    }
    if (fseek(file, 0, SEEK_END) != 0) return -1;
    long size = ftell(file);
    if (size <= 0 || fseek(file, 0, SEEK_SET) != 0) return -1;
    char *text = malloc((size_t)size + 1U);
    if (!text || fread(text, 1, (size_t)size, file) != (size_t)size) {
        free(text);
        fclose(file);
        return -1;
    }
    fclose(file);
    text[size] = '\0';
    char *up = strstr(text, "-- migrate:up");
    char *down = strstr(text, "-- migrate:down");
    if (!up || !down || down <= up) {
        free(text);
        return -1;
    }
    *down = '\0';
    int result = sqlite3_exec(standalone_database, up + 13,
                              NULL, NULL, NULL);
    free(text);
    return result == SQLITE_OK ? 0 : -1;
}

void shutdown_database(void) {
    if (standalone_database) sqlite3_close(standalone_database);
    standalone_database = NULL;
}
#endif

static const char RUN_A[] = "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa";
static const char RUN_B[] = "bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb";
static const char RUN_C[] = "cccccccc-cccc-4ccc-8ccc-cccccccccccc";
static const char EVENT_A[] = "11111111-1111-4111-8111-111111111111";
static const char EVENT_B[] = "22222222-2222-4222-8222-222222222222";

void setUp(void) {
    sqlite3 *database = get_db_handle();
    sqlite3_exec(database,
        "DROP TRIGGER IF EXISTS test_abort_health_transition;"
        "DELETE FROM system_health_incident_transitions;"
        "DELETE FROM system_health_incidents;"
        "DELETE FROM system_health_process_runs;",
        NULL, NULL, NULL);
}

void tearDown(void) {}

static int scalar_int(const char *sql) {
    sqlite3_stmt *statement = NULL;
    if (sqlite3_prepare_v2(get_db_handle(), sql, -1, &statement, NULL) !=
        SQLITE_OK) return -1;
    int value = sqlite3_step(statement) == SQLITE_ROW
        ? sqlite3_column_int(statement, 0) : -1;
    sqlite3_finalize(statement);
    return value;
}

static system_health_incident_signal_t signal_at(int64_t at) {
    system_health_incident_signal_t signal;
    memset(&signal, 0, sizeof(signal));
    signal.condition = SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW;
    safe_strcpy(signal.subject, "host", sizeof(signal.subject), 0);
    signal.scope = SYSTEM_HEALTH_SCOPE_HOST;
    signal.state = SYSTEM_HEALTH_STATE_OPEN;
    signal.severity = SYSTEM_HEALTH_SEVERITY_WARNING;
    signal.observed_at_ms = at;
    safe_strcpy(signal.observation_json,
                "{\"available_ratio\":0.08,\"unit\":\"ratio\"}",
                sizeof(signal.observation_json), 0);
    signal.reconciliation = SYSTEM_HEALTH_RECONCILIATION_ALERT_PENDING;
    safe_strcpy(signal.boot_id, "boot-a", sizeof(signal.boot_id), 0);
    safe_strcpy(signal.run_id, RUN_A, sizeof(signal.run_id), 0);
    return signal;
}

void test_process_runs_distinguish_unclean_restart_and_boot_change(void) {
    system_health_process_run_t current;
    system_health_process_run_t previous;
    bool had_previous = true;

    TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_OK,
        db_system_health_run_open(RUN_A, "boot-a", 100, &current,
                                  &previous, &had_previous));
    TEST_ASSERT_FALSE(had_previous);
    TEST_ASSERT_EQUAL_STRING(RUN_A, current.run_id);
    TEST_ASSERT_FALSE(current.clean_close);

    TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_OK,
        db_system_health_run_open(RUN_B, "boot-a", 200, &current,
                                  &previous, &had_previous));
    TEST_ASSERT_TRUE(had_previous);
    TEST_ASSERT_EQUAL_STRING(RUN_A, previous.run_id);
    TEST_ASSERT_FALSE(previous.clean_close);
    TEST_ASSERT_EQUAL_STRING("boot-a", previous.boot_id);

    TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_OK,
        db_system_health_run_close(RUN_B, 250));
    TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_OK,
        db_system_health_run_open(RUN_C, "boot-b", 300, &current,
                                  &previous, &had_previous));
    TEST_ASSERT_TRUE(had_previous);
    TEST_ASSERT_EQUAL_STRING(RUN_B, previous.run_id);
    TEST_ASSERT_TRUE(previous.clean_close);
    TEST_ASSERT_EQUAL_STRING("boot-a", previous.boot_id);
    TEST_ASSERT_EQUAL_STRING("boot-b", current.boot_id);

    TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_CONFLICT,
        db_system_health_run_open(RUN_C, "boot-b", 301, &current,
                                  &previous, &had_previous));
    TEST_ASSERT_EQUAL_INT(3,
        scalar_int("SELECT count(*) FROM system_health_process_runs;"));
}

void test_incident_lifecycle_resumes_without_duplicate_open(void) {
    system_health_incident_signal_t signal = signal_at(1000);
    system_health_incident_record_t incident;
    char incident_uuid[LIGHTNVR_UUID_STRING_SIZE];

    safe_strcpy(signal.event_id, EVENT_A, sizeof(signal.event_id), 0);
    TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_OK,
        db_system_health_incident_apply(SYSTEM_HEALTH_INCIDENT_OPEN,
                                        &signal, &incident));
    safe_strcpy(incident_uuid, incident.uuid, sizeof(incident_uuid), 0);
    TEST_ASSERT_EQUAL_INT(1, scalar_int(
        "SELECT count(*) FROM system_health_incident_transitions;"));

    signal.observed_at_ms = 1100;
    safe_strcpy(signal.run_id, RUN_B, sizeof(signal.run_id), 0);
    TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_RESUMED,
        db_system_health_incident_apply(SYSTEM_HEALTH_INCIDENT_OPEN,
                                        &signal, &incident));
    TEST_ASSERT_EQUAL_STRING(incident_uuid, incident.uuid);
    TEST_ASSERT_EQUAL_STRING(RUN_B, incident.run_id);
    TEST_ASSERT_EQUAL_INT(1, scalar_int(
        "SELECT count(*) FROM system_health_incident_transitions;"));

    signal.observed_at_ms = 1200;
    signal.severity = SYSTEM_HEALTH_SEVERITY_ERROR;
    TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_OK,
        db_system_health_incident_apply(SYSTEM_HEALTH_INCIDENT_ESCALATE,
                                        &signal, &incident));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_SEVERITY_ERROR, incident.severity);

    signal.observed_at_ms = 1300;
    signal.state = SYSTEM_HEALTH_STATE_RECOVERING;
    TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_OK,
        db_system_health_incident_apply(
            SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE, &signal, &incident));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_STATE_RECOVERING, incident.state);

    signal.observed_at_ms = 1400;
    signal.state = SYSTEM_HEALTH_STATE_CLOSED;
    signal.severity = SYSTEM_HEALTH_SEVERITY_NONE;
    signal.reconciliation = SYSTEM_HEALTH_RECONCILIATION_RECOVERY_PENDING;
    safe_strcpy(signal.event_id, EVENT_B, sizeof(signal.event_id), 0);
    TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_OK,
        db_system_health_incident_apply(SYSTEM_HEALTH_INCIDENT_RECOVER,
                                        &signal, &incident));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_STATE_CLOSED, incident.state);
    TEST_ASSERT_EQUAL_INT64(1400, incident.closed_at_ms);
    TEST_ASSERT_EQUAL_STRING(EVENT_B, incident.recovery_event_id);
    TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_NOT_FOUND,
        db_system_health_incident_get_active(
            SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW, "host", &incident));
    TEST_ASSERT_EQUAL_INT(4, scalar_int(
        "SELECT count(*) FROM system_health_incident_transitions;"));

    signal.observed_at_ms = 1500;
    signal.state = SYSTEM_HEALTH_STATE_CLOSED;
    signal.severity = SYSTEM_HEALTH_SEVERITY_WARNING;
    signal.reconciliation = SYSTEM_HEALTH_RECONCILIATION_NONE;
    signal.event_id[0] = '\0';
    TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_OK,
        db_system_health_incident_apply(SYSTEM_HEALTH_INCIDENT_ONE_SHOT,
                                        &signal, &incident));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_STATE_CLOSED, incident.state);
    TEST_ASSERT_EQUAL_INT(2,
        scalar_int("SELECT count(*) FROM system_health_incidents;"));
    TEST_ASSERT_EQUAL_INT(5, scalar_int(
        "SELECT count(*) FROM system_health_incident_transitions;"));
}

void test_pending_transition_reconciliation_is_transition_specific(void) {
    system_health_incident_signal_t signal = signal_at(1000);
    safe_strcpy(signal.event_id, EVENT_A, sizeof(signal.event_id), 0);
    system_health_incident_record_t incident;
    TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_OK,
        db_system_health_incident_apply(SYSTEM_HEALTH_INCIDENT_OPEN,
                                        &signal, &incident));

    system_health_incident_transition_t pending[2];
    TEST_ASSERT_EQUAL_INT(1, db_system_health_transition_list_pending(
        0, pending, 2));
    TEST_ASSERT_EQUAL_STRING(incident.uuid, pending[0].incident_uuid);
    TEST_ASSERT_EQUAL_STRING(EVENT_A, pending[0].event_id);
    system_health_incident_record_t by_uuid;
    TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_OK,
        db_system_health_incident_get_uuid(incident.uuid, &by_uuid));
    TEST_ASSERT_EQUAL_STRING(incident.uuid, by_uuid.uuid);

    TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_NOT_FOUND,
        db_system_health_transition_set_reconciliation(
            pending[0].incident_uuid, EVENT_B,
            SYSTEM_HEALTH_RECONCILIATION_RECONCILED));
    TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_OK,
        db_system_health_transition_set_reconciliation(
            pending[0].incident_uuid, EVENT_A,
            SYSTEM_HEALTH_RECONCILIATION_RECONCILED));
    TEST_ASSERT_EQUAL_INT(0, db_system_health_transition_list_pending(
        0, pending, 2));
    TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_OK,
        db_system_health_incident_get_uuid(incident.uuid, &by_uuid));
    TEST_ASSERT_EQUAL_INT(SYSTEM_HEALTH_RECONCILIATION_RECONCILED,
                          by_uuid.reconciliation);
}

static int raw_incident(const char *condition, const char *subject,
                        const char *severity, const char *json) {
    const char *sql =
        "INSERT INTO system_health_incidents("
        "uuid,condition_code,subject,scope,state,severity,first_seen_at_ms,"
        "last_seen_at_ms,last_observation_json,boot_id,run_id) "
        "VALUES('99999999-9999-4999-8999-999999999999',?,?,"
        "'host','open',?,1,1,?,'boot-a',?);";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(get_db_handle(), sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, condition, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, subject, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 3, severity, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 4, json, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 5, RUN_A, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    return result;
}

void test_constraints_and_transaction_rollback_reject_unsafe_state(void) {
    TEST_ASSERT_EQUAL(SQLITE_CONSTRAINT,
        raw_incident("not.registered", "host", "warning", "{}"));
    TEST_ASSERT_EQUAL(SQLITE_CONSTRAINT,
        raw_incident("memory.available_low", "/dev/sda", "warning", "{}"));
    TEST_ASSERT_EQUAL(SQLITE_CONSTRAINT,
        raw_incident("memory.available_low", "host", "fatal", "{}"));
    char oversized[2050];
    memset(oversized, 'x', sizeof(oversized));
    oversized[0] = '{';
    oversized[sizeof(oversized) - 2] = '}';
    oversized[sizeof(oversized) - 1] = '\0';
    TEST_ASSERT_EQUAL(SQLITE_CONSTRAINT,
        raw_incident("memory.available_low", "host", "warning", oversized));

    system_health_incident_signal_t signal = signal_at(1000);
    system_health_incident_record_t incident;
    TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_INVALID,
        db_system_health_incident_apply(SYSTEM_HEALTH_INCIDENT_OPEN,
            &(system_health_incident_signal_t){
                .condition = SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW,
                .scope = SYSTEM_HEALTH_SCOPE_HOST,
                .state = SYSTEM_HEALTH_STATE_OPEN,
                .severity = SYSTEM_HEALTH_SEVERITY_WARNING,
                .observed_at_ms = 1,
                .subject = "/root/disk",
                .observation_json = "{}",
                .boot_id = "boot-a",
                .run_id = "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa"
            }, &incident));
    TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_OK,
        db_system_health_incident_apply(SYSTEM_HEALTH_INCIDENT_OPEN,
                                        &signal, &incident));
    int64_t revision = incident.revision;
    TEST_ASSERT_EQUAL(SQLITE_OK, sqlite3_exec(get_db_handle(),
        "CREATE TRIGGER test_abort_health_transition BEFORE INSERT ON "
        "system_health_incident_transitions WHEN NEW.kind='material_change' "
        "BEGIN SELECT RAISE(ABORT,'test rollback'); END;",
        NULL, NULL, NULL));
    signal.observed_at_ms = 1100;
    signal.state = SYSTEM_HEALTH_STATE_RECOVERING;
    TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_ERROR,
        db_system_health_incident_apply(
            SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE, &signal, &incident));
    TEST_ASSERT_EQUAL(SQLITE_OK, sqlite3_exec(get_db_handle(),
        "DROP TRIGGER test_abort_health_transition;", NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_OK,
        db_system_health_incident_get_active(
            SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW, "host", &incident));
    TEST_ASSERT_EQUAL_INT64(revision, incident.revision);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_STATE_OPEN, incident.state);
    TEST_ASSERT_EQUAL_INT(1, scalar_int(
        "SELECT count(*) FROM system_health_incident_transitions;"));
}

void test_cursor_limits_and_retention_preserve_active_history(void) {
    system_health_incident_signal_t signal = signal_at(100);
    system_health_incident_record_t active;
    system_health_incident_record_t closed;
    TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_OK,
        db_system_health_incident_apply(SYSTEM_HEALTH_INCIDENT_OPEN,
                                        &signal, &active));
    for (int index = 0; index < 3; ++index) {
        signal.observed_at_ms = 200 + index * 100;
        signal.state = SYSTEM_HEALTH_STATE_CLOSED;
        safe_strcpy(signal.subject, index == 0 ? "device:a" :
                    index == 1 ? "device:b" : "device:c",
                    sizeof(signal.subject), 0);
        TEST_ASSERT_EQUAL_INT(DB_SYSTEM_HEALTH_OK,
            db_system_health_incident_apply(SYSTEM_HEALTH_INCIDENT_ONE_SHOT,
                                            &signal, &closed));
    }
    system_health_incident_record_t page[2];
    system_health_incident_cursor_t next;
    TEST_ASSERT_EQUAL_INT(2,
        db_system_health_incident_list(true, NULL, page, 2, &next));
    TEST_ASSERT_TRUE(next.valid);
    TEST_ASSERT_EQUAL_INT64(400, page[0].last_seen_at_ms);
    system_health_incident_cursor_t final;
    TEST_ASSERT_EQUAL_INT(2,
        db_system_health_incident_list(true, &next, page, 2, &final));
    TEST_ASSERT_FALSE(final.valid);
    TEST_ASSERT_EQUAL_INT(-1, db_system_health_incident_list(
        true, NULL, page, SYSTEM_HEALTH_INCIDENT_PAGE_MAX + 1, &next));

    int deleted = -1;
    TEST_ASSERT_EQUAL_INT(0,
        db_system_health_transition_prune(1000, 100, &deleted));
    TEST_ASSERT_EQUAL_INT(3, deleted);
    sqlite3_stmt *statement = NULL;
    TEST_ASSERT_EQUAL(SQLITE_OK, sqlite3_prepare_v2(get_db_handle(),
        "SELECT count(*) FROM system_health_incident_transitions "
        "WHERE incident_uuid=?;", -1, &statement, NULL));
    sqlite3_bind_text(statement, 1, active.uuid, -1, SQLITE_TRANSIENT);
    TEST_ASSERT_EQUAL(SQLITE_ROW, sqlite3_step(statement));
    TEST_ASSERT_EQUAL_INT(1, sqlite3_column_int(statement, 0));
    sqlite3_finalize(statement);
    TEST_ASSERT_EQUAL_INT(4,
        scalar_int("SELECT count(*) FROM system_health_incidents;"));
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_process_runs_distinguish_unclean_restart_and_boot_change);
    RUN_TEST(test_incident_lifecycle_resumes_without_duplicate_open);
    RUN_TEST(test_pending_transition_reconciliation_is_transition_specific);
    RUN_TEST(test_constraints_and_transaction_rollback_reject_unsafe_state);
    RUN_TEST(test_cursor_limits_and_retention_preserve_active_history);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
