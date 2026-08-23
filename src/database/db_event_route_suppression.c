#define _POSIX_C_SOURCE 200809L

#include "database/db_event_route_suppression.h"

#include <ctype.h>
#include <limits.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "database/db_core.h"
#include "utils/strings.h"
#include "utils/uuid.h"

static bool valid_text(const char *value, size_t maximum) {
    if (!value || value[0] == '\0' || strlen(value) >= maximum) return false;
    for (const unsigned char *cursor = (const unsigned char *)value;
         *cursor; cursor++) {
        if (iscntrl(*cursor)) return false;
    }
    return true;
}

static bool valid_input(const event_route_t *route, const char *event_type,
                        const char *subject, int64_t now) {
    return route && lightnvr_uuid_is_valid(route->uuid) &&
        route->revision >= 1 && now > 0 &&
        valid_text(event_type, EVENT_TYPE_MAX) &&
        valid_text(subject, EVENT_SUBJECT_MAX);
}

static bool valid_route_identity(const char *route_uuid,
                                 int64_t route_revision,
                                 const char *event_type,
                                 const char *subject, int64_t now) {
    return lightnvr_uuid_is_valid(route_uuid) && route_revision >= 1 &&
        now > 0 && valid_text(event_type, EVENT_TYPE_MAX) &&
        valid_text(subject, EVENT_SUBJECT_MAX);
}

static bool begin(sqlite3 *db) {
    return sqlite3_exec(db, "BEGIN IMMEDIATE;", NULL, NULL, NULL) == SQLITE_OK;
}

static bool finish(sqlite3 *db, bool success) {
    if (success && sqlite3_exec(db, "COMMIT;", NULL, NULL, NULL) == SQLITE_OK) {
        return true;
    }
    sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
    return false;
}

static event_suppression_result_t route_is_current_locked(
    sqlite3 *db, const event_route_t *route) {
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        db, "SELECT enabled,revision FROM event_routes WHERE uuid=?;", -1,
        &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, route->uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    event_suppression_result_t outcome = EVENT_SUPPRESSION_ERROR;
    if (result == SQLITE_ROW) {
        bool enabled = sqlite3_column_int(statement, 0) != 0;
        int64_t revision = sqlite3_column_int64(statement, 1);
        outcome = enabled && revision == route->revision
            ? EVENT_SUPPRESSION_PERMIT : EVENT_SUPPRESSION_STALE;
    } else if (result == SQLITE_DONE) {
        outcome = EVENT_SUPPRESSION_STALE;
    }
    if (statement) sqlite3_finalize(statement);
    return outcome;
}

static event_suppression_result_t load_current_route_locked(
    sqlite3 *db, const char *route_uuid, int64_t route_revision,
    event_route_t *route) {
    memset(route, 0, sizeof(*route));
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        db, "SELECT enabled,revision,debounce_seconds,cooldown_seconds,"
            "grouping_window_seconds,max_events_per_minute "
            "FROM event_routes WHERE uuid=?;", -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, route_uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    event_suppression_result_t outcome = EVENT_SUPPRESSION_ERROR;
    if (result == SQLITE_ROW) {
        bool enabled = sqlite3_column_int(statement, 0) != 0;
        int64_t revision = sqlite3_column_int64(statement, 1);
        if (!enabled || revision != route_revision) {
            outcome = EVENT_SUPPRESSION_STALE;
        } else {
            safe_strcpy(route->uuid, route_uuid, sizeof(route->uuid), 0);
            route->revision = revision;
            route->debounce_seconds = sqlite3_column_int(statement, 2);
            route->cooldown_seconds = sqlite3_column_int(statement, 3);
            route->grouping_window_seconds = sqlite3_column_int(statement, 4);
            route->max_events_per_minute = sqlite3_column_int(statement, 5);
            outcome = EVENT_SUPPRESSION_PERMIT;
        }
    } else if (result == SQLITE_DONE) {
        outcome = EVENT_SUPPRESSION_STALE;
    }
    if (statement) sqlite3_finalize(statement);
    return outcome;
}

static int load_state_locked(sqlite3 *db, const char *route_uuid,
                             const char *event_type, const char *subject,
                             event_route_suppression_state_t *state) {
    memset(state, 0, sizeof(*state));
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        db, "SELECT last_observed_at,last_allowed_at,rate_window_started_at,"
            "rate_window_count,group_started_at,suppressed_count,"
            "last_allowed_event_id,last_reason "
            "FROM event_route_suppression_state "
            "WHERE route_uuid=? AND event_type=? AND subject=?;", -1,
        &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, route_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, event_type, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 3, subject, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    int found = 0;
    if (result == SQLITE_ROW) {
        state->last_observed_at = sqlite3_column_int64(statement, 0);
        state->last_allowed_at = sqlite3_column_int64(statement, 1);
        state->rate_window_started_at = sqlite3_column_int64(statement, 2);
        state->rate_window_count = sqlite3_column_int(statement, 3);
        state->group_started_at = sqlite3_column_int64(statement, 4);
        state->suppressed_count = sqlite3_column_int64(statement, 5);
        const char *event_id =
            (const char *)sqlite3_column_text(statement, 6);
        const char *reason = (const char *)sqlite3_column_text(statement, 7);
        safe_strcpy(state->last_allowed_event_id,
                    event_id ? event_id : "",
                    sizeof(state->last_allowed_event_id), 0);
        safe_strcpy(state->last_reason, reason ? reason : "",
                    sizeof(state->last_reason), 0);
        found = 1;
    } else if (result != SQLITE_DONE) {
        found = -1;
    }
    if (statement) sqlite3_finalize(statement);
    return found;
}

static bool within_interval(int64_t now, int64_t previous, int seconds) {
    if (seconds <= 0 || previous <= 0) return false;
    return now < previous || now - previous < seconds;
}

static int64_t latest_state_time(
    const event_route_suppression_state_t *state, int64_t now) {
    int64_t latest = now;
    if (state->last_observed_at > latest) latest = state->last_observed_at;
    if (state->last_allowed_at > latest) latest = state->last_allowed_at;
    if (state->rate_window_started_at > latest) {
        latest = state->rate_window_started_at;
    }
    if (state->group_started_at > latest) latest = state->group_started_at;
    return latest;
}

static const char *reason_name(event_suppression_result_t result) {
    switch (result) {
        case EVENT_SUPPRESSION_DEBOUNCE: return "debounce";
        case EVENT_SUPPRESSION_COOLDOWN: return "cooldown";
        case EVENT_SUPPRESSION_GROUPING: return "grouping";
        case EVENT_SUPPRESSION_RATE: return "rate";
        default: return "allowed";
    }
}

static event_suppression_result_t decide(
    const event_route_t *route,
    const event_route_suppression_state_t *state, int64_t now) {
    if (within_interval(now, state->last_observed_at,
                        route->debounce_seconds)) {
        return EVENT_SUPPRESSION_DEBOUNCE;
    }
    if (within_interval(now, state->last_allowed_at,
                        route->cooldown_seconds)) {
        return EVENT_SUPPRESSION_COOLDOWN;
    }
    if (within_interval(now, state->group_started_at,
                        route->grouping_window_seconds)) {
        return EVENT_SUPPRESSION_GROUPING;
    }
    bool active_rate_window = state->rate_window_started_at > 0 &&
        (now < state->rate_window_started_at ||
         now - state->rate_window_started_at < 60);
    if (route->max_events_per_minute > 0 && active_rate_window &&
        state->rate_window_count >= route->max_events_per_minute) {
        return EVENT_SUPPRESSION_RATE;
    }
    return EVENT_SUPPRESSION_PERMIT;
}

static int save_state_locked(sqlite3 *db, const event_route_t *route,
                             const char *event_type, const char *subject,
                             const event_route_suppression_state_t *state,
                             int64_t updated_at) {
    const char *sql =
        "INSERT INTO event_route_suppression_state("
        "route_uuid,event_type,subject,last_observed_at,last_allowed_at,"
        "rate_window_started_at,rate_window_count,group_started_at,"
        "suppressed_count,last_allowed_event_id,last_reason,updated_at) "
        "VALUES(?,?,?,?,?,?,?,?,?,?,?,?) "
        "ON CONFLICT(route_uuid,event_type,subject) DO UPDATE SET "
        "last_observed_at=excluded.last_observed_at,"
        "last_allowed_at=excluded.last_allowed_at,"
        "rate_window_started_at=excluded.rate_window_started_at,"
        "rate_window_count=excluded.rate_window_count,"
        "group_started_at=excluded.group_started_at,"
        "suppressed_count=excluded.suppressed_count,"
        "last_allowed_event_id=excluded.last_allowed_event_id,"
        "last_reason=excluded.last_reason,updated_at=excluded.updated_at;";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, route->uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, event_type, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 3, subject, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 4, state->last_observed_at);
        sqlite3_bind_int64(statement, 5, state->last_allowed_at);
        sqlite3_bind_int64(statement, 6, state->rate_window_started_at);
        sqlite3_bind_int(statement, 7, state->rate_window_count);
        sqlite3_bind_int64(statement, 8, state->group_started_at);
        sqlite3_bind_int64(statement, 9, state->suppressed_count);
        sqlite3_bind_text(statement, 10, state->last_allowed_event_id, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 11, state->last_reason, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 12, updated_at);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    return result == SQLITE_DONE ? 0 : -1;
}

event_suppression_result_t db_event_route_suppression_check(
    const event_route_t *route, const char *event_type, const char *subject,
    int64_t now) {
    if (!valid_input(route, event_type, subject, now)) {
        return EVENT_SUPPRESSION_ERROR;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return EVENT_SUPPRESSION_ERROR;
    pthread_mutex_lock(mutex);
    if (!begin(db)) {
        pthread_mutex_unlock(mutex);
        return EVENT_SUPPRESSION_ERROR;
    }
    event_suppression_result_t current =
        route_is_current_locked(db, route);
    event_route_suppression_state_t state;
    int found = current == EVENT_SUPPRESSION_PERMIT
        ? load_state_locked(db, route->uuid, event_type, subject, &state) : -1;
    if (current != EVENT_SUPPRESSION_PERMIT || found < 0) {
        finish(db, false);
        pthread_mutex_unlock(mutex);
        return current == EVENT_SUPPRESSION_STALE
            ? current : EVENT_SUPPRESSION_ERROR;
    }
    event_suppression_result_t decision = decide(route, &state, now);
    bool success = true;
    if (decision != EVENT_SUPPRESSION_PERMIT) {
        if (now > state.last_observed_at) state.last_observed_at = now;
        if (state.suppressed_count < INT64_MAX) state.suppressed_count++;
        safe_strcpy(state.last_reason, reason_name(decision),
                    sizeof(state.last_reason), 0);
        success = save_state_locked(
            db, route, event_type, subject, &state,
            latest_state_time(&state, now)) == 0;
    }
    success = finish(db, success);
    pthread_mutex_unlock(mutex);
    return success ? decision : EVENT_SUPPRESSION_ERROR;
}

event_suppression_result_t db_event_route_suppression_record_allowed(
    const char *route_uuid, int64_t route_revision, const char *event_type,
    const char *subject, const char *event_id, int64_t now) {
    if (!valid_route_identity(route_uuid, route_revision, event_type, subject,
                              now) ||
        !lightnvr_uuid_is_valid(event_id)) {
        return EVENT_SUPPRESSION_ERROR;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return EVENT_SUPPRESSION_ERROR;
    pthread_mutex_lock(mutex);
    if (!begin(db)) {
        pthread_mutex_unlock(mutex);
        return EVENT_SUPPRESSION_ERROR;
    }
    event_route_t route;
    event_suppression_result_t current = load_current_route_locked(
        db, route_uuid, route_revision, &route);
    event_route_suppression_state_t state;
    int found = current == EVENT_SUPPRESSION_PERMIT
        ? load_state_locked(db, route_uuid, event_type, subject, &state) : -1;
    if (current != EVENT_SUPPRESSION_PERMIT || found < 0) {
        finish(db, false);
        pthread_mutex_unlock(mutex);
        return current == EVENT_SUPPRESSION_STALE
            ? current : EVENT_SUPPRESSION_ERROR;
    }
    if (found > 0 &&
        strcmp(state.last_allowed_event_id, event_id) == 0) {
        bool committed = finish(db, true);
        pthread_mutex_unlock(mutex);
        return committed ? EVENT_SUPPRESSION_PERMIT
                         : EVENT_SUPPRESSION_ERROR;
    }
    int64_t effective_now = latest_state_time(&state, now);
    state.last_observed_at = effective_now;
    state.last_allowed_at = effective_now;
    state.group_started_at = route.grouping_window_seconds > 0
        ? effective_now : 0;
    bool active_rate_window = state.rate_window_started_at > 0 &&
        effective_now - state.rate_window_started_at < 60;
    if (route.max_events_per_minute > 0) {
        if (!active_rate_window) {
            state.rate_window_started_at = effective_now;
            state.rate_window_count = 1;
        } else if (state.rate_window_count < INT_MAX) {
            state.rate_window_count++;
        }
    } else {
        state.rate_window_started_at = 0;
        state.rate_window_count = 0;
    }
    safe_strcpy(state.last_allowed_event_id, event_id,
                sizeof(state.last_allowed_event_id), 0);
    safe_strcpy(state.last_reason, "allowed", sizeof(state.last_reason), 0);
    bool success = save_state_locked(db, &route, event_type, subject, &state,
                                     effective_now) == 0;
    success = finish(db, success);
    pthread_mutex_unlock(mutex);
    return success ? EVENT_SUPPRESSION_PERMIT : EVENT_SUPPRESSION_ERROR;
}

int db_event_route_suppression_get(
    const char *route_uuid, const char *event_type, const char *subject,
    event_route_suppression_state_t *state) {
    if (!lightnvr_uuid_is_valid(route_uuid) ||
        !valid_text(event_type, EVENT_TYPE_MAX) ||
        !valid_text(subject, EVENT_SUBJECT_MAX) || !state) {
        return -1;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    int result = load_state_locked(db, route_uuid, event_type, subject, state);
    pthread_mutex_unlock(mutex);
    return result;
}

int db_event_route_suppression_prune(int64_t updated_before, int limit,
                                     int *deleted_count) {
    if (deleted_count) *deleted_count = 0;
    if (updated_before <= 0 || limit <= 0 || limit > 10000) return -1;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        db, "DELETE FROM event_route_suppression_state WHERE rowid IN ("
            "SELECT rowid FROM event_route_suppression_state "
            "WHERE updated_at<? ORDER BY updated_at,rowid LIMIT ?);", -1,
        &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_int64(statement, 1, updated_before);
        sqlite3_bind_int(statement, 2, limit);
        result = sqlite3_step(statement);
    }
    if (deleted_count && result == SQLITE_DONE) {
        *deleted_count = sqlite3_changes(db);
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return result == SQLITE_DONE ? 0 : -1;
}
