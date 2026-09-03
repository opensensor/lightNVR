#define _POSIX_C_SOURCE 200809L

#include "database/db_event_outbox.h"

#include <ctype.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#define LOG_COMPONENT "EventOutboxDB"
#include "core/logger.h"
#include "database/db_core.h"
#include "utils/strings.h"

static bool valid_text(const char *value, size_t maximum, bool required) {
    if (!value || value[0] == '\0') return !required;
    if (strlen(value) >= maximum) return false;
    for (const unsigned char *cursor = (const unsigned char *)value;
         *cursor; cursor++) {
        if (iscntrl(*cursor)) return false;
    }
    return true;
}

static event_outbox_limits_t normalized_limits(
    const event_outbox_limits_t *limits) {
    event_outbox_limits_t result = {
        .max_rows = EVENT_OUTBOX_DEFAULT_MAX_ROWS,
        .max_bytes = EVENT_OUTBOX_DEFAULT_MAX_BYTES,
    };
    if (limits) {
        if (limits->max_rows > 0) result.max_rows = limits->max_rows;
        if (limits->max_bytes > 0) result.max_bytes = limits->max_bytes;
    }
    return result;
}

static int execute_sql(sqlite3 *db, const char *sql) {
    char *message = NULL;
    int rc = sqlite3_exec(db, sql, NULL, NULL, &message);
    if (rc != SQLITE_OK) {
        log_error("Outbox SQL failed: %s", message ? message : sqlite3_errmsg(db));
    }
    sqlite3_free(message);
    return rc == SQLITE_OK ? 0 : -1;
}

static void rollback(sqlite3 *db) {
    sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
}

static int capacity_locked(sqlite3 *db, int64_t *rows, int64_t *bytes) {
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db, "SELECT count(*),COALESCE(sum(envelope_bytes),0) "
            "FROM event_outbox;", -1, &stmt, NULL);
    if (rc == SQLITE_OK && sqlite3_step(stmt) == SQLITE_ROW) {
        *rows = sqlite3_column_int64(stmt, 0);
        *bytes = sqlite3_column_int64(stmt, 1);
        rc = SQLITE_DONE;
    }
    if (stmt) sqlite3_finalize(stmt);
    return rc == SQLITE_DONE ? 0 : -1;
}

static int delete_candidate_locked(sqlite3 *db, bool terminal,
                                   event_severity_t incoming) {
    const char *sql = terminal
        ? "DELETE FROM event_outbox WHERE id=(SELECT id FROM event_outbox "
          "WHERE state IN ('delivered','dead') ORDER BY updated_at,id LIMIT 1);"
        : "DELETE FROM event_outbox WHERE id=(SELECT id FROM event_outbox "
          "WHERE state='pending' AND severity<? "
          "ORDER BY severity,created_at,id LIMIT 1);";
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc == SQLITE_OK && !terminal) {
        sqlite3_bind_int(stmt, 1, (int)incoming);
    }
    if (rc == SQLITE_OK) rc = sqlite3_step(stmt);
    int changed = rc == SQLITE_DONE ? sqlite3_changes(db) : -1;
    if (stmt) sqlite3_finalize(stmt);
    return changed;
}

static bool has_capacity(int64_t rows, int64_t bytes, int64_t payload_bytes,
                         const event_outbox_limits_t *limits) {
    return rows < limits->max_rows && payload_bytes <= limits->max_bytes &&
        bytes <= limits->max_bytes - payload_bytes;
}

static int duplicate_locked(sqlite3 *db, const event_envelope_t *event,
                            const char *destination, int64_t *row_id) {
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db, "SELECT id FROM event_outbox WHERE event_source=? AND event_id=? "
            "AND destination=? LIMIT 1;", -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, event->source, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 2, event->id, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 3, destination, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    int found = 0;
    if (rc == SQLITE_ROW) {
        if (row_id) *row_id = sqlite3_column_int64(stmt, 0);
        found = 1;
    } else if (rc != SQLITE_DONE) {
        found = -1;
    }
    if (stmt) sqlite3_finalize(stmt);
    return found;
}

event_outbox_enqueue_result_t db_event_outbox_enqueue(
    const event_envelope_t *event, const char *destination,
    const char *topic, const event_outbox_limits_t *limits,
    int64_t *row_id, int *shed_count) {
    if (row_id) *row_id = 0;
    if (shed_count) *shed_count = 0;
    if (!valid_text(destination, EVENT_OUTBOX_DESTINATION_MAX, true) ||
        !valid_text(topic, EVENT_OUTBOX_TOPIC_MAX, true)) {
        return EVENT_OUTBOX_ERROR;
    }
    char validation_error[256] = {0};
    char *serialized = event_envelope_serialize(
        event, validation_error, sizeof(validation_error));
    if (!serialized) return EVENT_OUTBOX_ERROR;
    int64_t serialized_bytes = (int64_t)strlen(serialized);
    const event_type_definition_t *definition = event_registry_find(event->type);
    event_outbox_limits_t bounds = normalized_limits(limits);
    if (!definition || bounds.max_rows < 1 || bounds.max_bytes < 1 ||
        serialized_bytes > bounds.max_bytes) {
        free(serialized);
        return EVENT_OUTBOX_FULL;
    }

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) {
        free(serialized);
        return EVENT_OUTBOX_ERROR;
    }
    pthread_mutex_lock(mutex);
    if (execute_sql(db, "BEGIN IMMEDIATE;") != 0) {
        pthread_mutex_unlock(mutex);
        free(serialized);
        return EVENT_OUTBOX_ERROR;
    }

    int duplicate = duplicate_locked(db, event, destination, row_id);
    if (duplicate != 0) {
        int transaction_result = 0;
        if (duplicate < 0) {
            rollback(db);
        } else {
            transaction_result = execute_sql(db, "COMMIT;");
            if (transaction_result != 0) rollback(db);
        }
        pthread_mutex_unlock(mutex);
        free(serialized);
        return duplicate > 0 && transaction_result == 0
            ? EVENT_OUTBOX_DUPLICATE : EVENT_OUTBOX_ERROR;
    }

    int64_t rows = 0;
    int64_t bytes = 0;
    if (capacity_locked(db, &rows, &bytes) != 0) {
        rollback(db);
        pthread_mutex_unlock(mutex);
        free(serialized);
        return EVENT_OUTBOX_ERROR;
    }
    int shed = 0;
    while (!has_capacity(rows, bytes, serialized_bytes, &bounds)) {
        int changed = delete_candidate_locked(db, true, event->severity);
        if (changed == 0 && event->severity >= EVENT_SEVERITY_ERROR) {
            changed = delete_candidate_locked(db, false, event->severity);
            if (changed > 0) shed += changed;
        }
        if (changed < 0 || (changed == 0) ||
            capacity_locked(db, &rows, &bytes) != 0) {
            if (changed < 0) {
                rollback(db);
                pthread_mutex_unlock(mutex);
                free(serialized);
                return EVENT_OUTBOX_ERROR;
            }
            rollback(db);
            pthread_mutex_unlock(mutex);
            free(serialized);
            return EVENT_OUTBOX_FULL;
        }
    }

    int64_t now = (int64_t)time(NULL);
    const char *sql =
        "INSERT INTO event_outbox(event_id,event_source,event_type,subject,"
        "destination,topic,envelope_json,envelope_bytes,severity,"
        "next_attempt_at,expires_at,created_at,updated_at) "
        "VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?);";
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, event->id, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 2, event->source, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 3, event->type, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 4, event->subject, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 5, destination, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 6, topic, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 7, serialized, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(stmt, 8, serialized_bytes);
        sqlite3_bind_int(stmt, 9, (int)event->severity);
        sqlite3_bind_int64(stmt, 10, now);
        sqlite3_bind_int64(stmt, 11, (int64_t)event->expires_at);
        sqlite3_bind_int64(stmt, 12, now);
        sqlite3_bind_int64(stmt, 13, now);
        rc = sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);
    if (rc != SQLITE_DONE || execute_sql(db, "COMMIT;") != 0) {
        rollback(db);
        pthread_mutex_unlock(mutex);
        free(serialized);
        return EVENT_OUTBOX_ERROR;
    }
    if (row_id) *row_id = sqlite3_last_insert_rowid(db);
    if (shed_count) *shed_count = shed;
    pthread_mutex_unlock(mutex);
    free(serialized);
    return EVENT_OUTBOX_ENQUEUED;
}

static int expire_locked(sqlite3 *db, int64_t now, int *expired_count) {
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db, "UPDATE event_outbox SET state='dead',updated_at=?,dead_at=?,"
            "lease_expires_at=NULL,last_error='expired' "
            "WHERE state IN ('pending','delivering') AND expires_at<=?;",
        -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, now);
        sqlite3_bind_int64(stmt, 2, now);
        sqlite3_bind_int64(stmt, 3, now);
        rc = sqlite3_step(stmt);
    }
    if (expired_count) {
        *expired_count = rc == SQLITE_DONE ? sqlite3_changes(db) : 0;
    }
    if (stmt) sqlite3_finalize(stmt);
    return rc == SQLITE_DONE ? 0 : -1;
}

static int release_stale_leases_locked(sqlite3 *db, int64_t now) {
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db, "UPDATE event_outbox SET state='pending',next_attempt_at=?,"
            "updated_at=?,lease_expires_at=NULL "
            "WHERE state='delivering' AND lease_expires_at<=? "
            "AND expires_at>?;", -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, now);
        sqlite3_bind_int64(stmt, 2, now);
        sqlite3_bind_int64(stmt, 3, now);
        sqlite3_bind_int64(stmt, 4, now);
        rc = sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);
    return rc == SQLITE_DONE ? 0 : -1;
}

static void copy_column(char *output, size_t output_size,
                        sqlite3_stmt *stmt, int column) {
    const char *value = (const char *)sqlite3_column_text(stmt, column);
    safe_strcpy(output, value ? value : "", output_size, 0);
}

void db_event_outbox_item_clear(event_outbox_item_t *item) {
    if (!item) return;
    free(item->envelope_json);
    memset(item, 0, sizeof(*item));
}

int db_event_outbox_claim_due(const char *destination, int64_t now,
                              int lease_seconds,
                              event_outbox_item_t *item) {
    if (!valid_text(destination, EVENT_OUTBOX_DESTINATION_MAX, true) ||
        !item) return -1;
    memset(item, 0, sizeof(*item));
    if (now <= 0) now = (int64_t)time(NULL);
    if (lease_seconds <= 0) lease_seconds = EVENT_OUTBOX_DEFAULT_LEASE_SECONDS;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    if (execute_sql(db, "BEGIN IMMEDIATE;") != 0) {
        pthread_mutex_unlock(mutex);
        return -1;
    }
    if (expire_locked(db, now, NULL) != 0 ||
        release_stale_leases_locked(db, now) != 0) {
        rollback(db);
        pthread_mutex_unlock(mutex);
        return -1;
    }

    const char *sql =
        "SELECT id,event_id,event_source,event_type,subject,destination,topic,"
        "envelope_json,envelope_bytes,severity,attempt_count,next_attempt_at,"
        "expires_at,created_at FROM event_outbox WHERE destination=? "
        "AND state='pending' AND next_attempt_at<=? AND expires_at>? "
        "ORDER BY severity DESC,next_attempt_at,id LIMIT 1;";
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, destination, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(stmt, 2, now);
        sqlite3_bind_int64(stmt, 3, now);
        rc = sqlite3_step(stmt);
    }
    if (rc == SQLITE_DONE) {
        sqlite3_finalize(stmt);
        if (execute_sql(db, "COMMIT;") != 0) {
            rollback(db);
            pthread_mutex_unlock(mutex);
            return -1;
        }
        pthread_mutex_unlock(mutex);
        return 0;
    }
    if (rc != SQLITE_ROW) {
        if (stmt) sqlite3_finalize(stmt);
        rollback(db);
        pthread_mutex_unlock(mutex);
        return -1;
    }

    item->row_id = sqlite3_column_int64(stmt, 0);
    copy_column(item->event_id, sizeof(item->event_id), stmt, 1);
    copy_column(item->event_source, sizeof(item->event_source), stmt, 2);
    copy_column(item->event_type, sizeof(item->event_type), stmt, 3);
    copy_column(item->subject, sizeof(item->subject), stmt, 4);
    copy_column(item->destination, sizeof(item->destination), stmt, 5);
    copy_column(item->topic, sizeof(item->topic), stmt, 6);
    const char *envelope = (const char *)sqlite3_column_text(stmt, 7);
    int envelope_size = sqlite3_column_bytes(stmt, 7);
    int64_t stored_size = sqlite3_column_int64(stmt, 8);
    item->severity = (event_severity_t)sqlite3_column_int(stmt, 9);
    item->attempt_count = sqlite3_column_int(stmt, 10) + 1;
    item->next_attempt_at = sqlite3_column_int64(stmt, 11);
    item->expires_at = sqlite3_column_int64(stmt, 12);
    item->created_at = sqlite3_column_int64(stmt, 13);
    if (!envelope || envelope_size <= 0 || stored_size != envelope_size ||
        envelope_size > (int)EVENT_ENVELOPE_MAX_BYTES) {
        sqlite3_finalize(stmt);
        rollback(db);
        pthread_mutex_unlock(mutex);
        db_event_outbox_item_clear(item);
        return -1;
    }
    item->envelope_json = malloc((size_t)envelope_size + 1);
    if (!item->envelope_json) {
        sqlite3_finalize(stmt);
        rollback(db);
        pthread_mutex_unlock(mutex);
        db_event_outbox_item_clear(item);
        return -1;
    }
    memcpy(item->envelope_json, envelope, (size_t)envelope_size);
    item->envelope_json[envelope_size] = '\0';
    item->envelope_bytes = (size_t)envelope_size;
    sqlite3_finalize(stmt);

    stmt = NULL;
    rc = sqlite3_prepare_v2(
        db, "UPDATE event_outbox SET state='delivering',attempt_count="
            "attempt_count+1,last_attempt_at=?,lease_expires_at=?,updated_at=? "
            "WHERE id=? AND state='pending';", -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, now);
        sqlite3_bind_int64(stmt, 2, now + lease_seconds);
        sqlite3_bind_int64(stmt, 3, now);
        sqlite3_bind_int64(stmt, 4, item->row_id);
        rc = sqlite3_step(stmt);
    }
    int changed = rc == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (stmt) sqlite3_finalize(stmt);
    if (rc != SQLITE_DONE || changed != 1 || execute_sql(db, "COMMIT;") != 0) {
        rollback(db);
        pthread_mutex_unlock(mutex);
        db_event_outbox_item_clear(item);
        return -1;
    }
    pthread_mutex_unlock(mutex);
    return 1;
}

static int transition_delivering(int64_t row_id, const char *sql,
                                 int64_t first_time, int64_t second_time,
                                 const char *error) {
    if (row_id <= 0 || !valid_text(error, EVENT_OUTBOX_ERROR_MAX, false)) {
        return -1;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, first_time);
        sqlite3_bind_int64(stmt, 2, second_time);
        sqlite3_bind_text(stmt, 3, error ? error : "", -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(stmt, 4, row_id);
        rc = sqlite3_step(stmt);
    }
    int changed = rc == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return rc == SQLITE_DONE && changed == 1 ? 0 : -1;
}

int db_event_outbox_mark_delivered(int64_t row_id, int64_t delivered_at) {
    if (delivered_at <= 0) delivered_at = (int64_t)time(NULL);
    return transition_delivering(
        row_id, "UPDATE event_outbox SET state='delivered',delivered_at=?,"
                "updated_at=?,dead_at=NULL,lease_expires_at=NULL,last_error=? "
                "WHERE id=? AND state='delivering';",
        delivered_at, delivered_at, "");
}

int db_event_outbox_mark_retry(int64_t row_id, int64_t next_attempt_at,
                               const char *error) {
    if (next_attempt_at <= 0) return -1;
    return transition_delivering(
        row_id, "UPDATE event_outbox SET state='pending',next_attempt_at=?,"
                "updated_at=?,dead_at=NULL,lease_expires_at=NULL,last_error=? "
                "WHERE id=? AND state='delivering';",
        next_attempt_at, (int64_t)time(NULL), error);
}

int db_event_outbox_mark_dead(int64_t row_id, int64_t failed_at,
                              const char *error) {
    if (failed_at <= 0) failed_at = (int64_t)time(NULL);
    return transition_delivering(
        row_id, "UPDATE event_outbox SET state='dead',updated_at=?,dead_at=?,"
                "lease_expires_at=NULL,last_error=? "
                "WHERE id=? AND state='delivering';",
        failed_at, failed_at, error);
}

int db_event_outbox_expire(int64_t now, int *expired_count) {
    if (expired_count) *expired_count = 0;
    if (now <= 0) now = (int64_t)time(NULL);
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    int result = expire_locked(db, now, expired_count);
    pthread_mutex_unlock(mutex);
    return result;
}

int db_event_outbox_prune_terminal(int64_t updated_before, int limit,
                                   int *deleted_count) {
    if (deleted_count) *deleted_count = 0;
    if (updated_before <= 0 || limit <= 0 || limit > 100000) return -1;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db, "DELETE FROM event_outbox WHERE id IN (SELECT id FROM event_outbox "
            "WHERE state IN ('delivered','dead') AND updated_at<? "
            "ORDER BY updated_at,id LIMIT ?);", -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, updated_before);
        sqlite3_bind_int(stmt, 2, limit);
        rc = sqlite3_step(stmt);
    }
    if (deleted_count) {
        *deleted_count = rc == SQLITE_DONE ? sqlite3_changes(db) : 0;
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return rc == SQLITE_DONE ? 0 : -1;
}

int db_event_outbox_get_stats(const char *destination, int64_t now,
                              event_outbox_stats_t *stats) {
    if (!stats || (destination &&
        !valid_text(destination, EVENT_OUTBOX_DESTINATION_MAX, true))) {
        return -1;
    }
    memset(stats, 0, sizeof(*stats));
    if (now <= 0) now = (int64_t)time(NULL);
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);

    const char *group_sql = destination
        ? "SELECT state,count(*),COALESCE(sum(envelope_bytes),0),"
          "min(created_at) FROM event_outbox WHERE destination=? GROUP BY state;"
        : "SELECT state,count(*),COALESCE(sum(envelope_bytes),0),"
          "min(created_at) FROM event_outbox GROUP BY state;";
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, group_sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        pthread_mutex_unlock(mutex);
        return -1;
    }
    if (destination) {
        sqlite3_bind_text(stmt, 1, destination, -1, SQLITE_TRANSIENT);
    }
    while ((rc = sqlite3_step(stmt)) == SQLITE_ROW) {
        const char *state = (const char *)sqlite3_column_text(stmt, 0);
        int64_t rows = sqlite3_column_int64(stmt, 1);
        int64_t bytes = sqlite3_column_int64(stmt, 2);
        int64_t oldest = sqlite3_column_int64(stmt, 3);
        stats->total_rows += rows;
        stats->total_bytes += bytes;
        if (state && strcmp(state, "pending") == 0) {
            stats->pending_rows = rows;
            stats->oldest_pending_at = oldest;
        } else if (state && strcmp(state, "delivering") == 0) {
            stats->delivering_rows = rows;
        } else if (state && strcmp(state, "delivered") == 0) {
            stats->delivered_rows = rows;
        } else if (state && strcmp(state, "dead") == 0) {
            stats->dead_rows = rows;
        }
    }
    if (stmt) sqlite3_finalize(stmt);
    if (rc != SQLITE_DONE) {
        pthread_mutex_unlock(mutex);
        return -1;
    }

    const char *due_sql = destination
        ? "SELECT count(*) FROM event_outbox WHERE destination=? AND "
          "state='pending' AND next_attempt_at<=? AND expires_at>?;"
        : "SELECT count(*) FROM event_outbox WHERE state='pending' "
          "AND next_attempt_at<=? AND expires_at>?;";
    stmt = NULL;
    rc = sqlite3_prepare_v2(db, due_sql, -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        int parameter = 1;
        if (destination) {
            sqlite3_bind_text(stmt, parameter++, destination, -1,
                              SQLITE_TRANSIENT);
        }
        sqlite3_bind_int64(stmt, parameter++, now);
        sqlite3_bind_int64(stmt, parameter, now);
        rc = sqlite3_step(stmt);
    }
    if (rc == SQLITE_ROW) stats->due_rows = sqlite3_column_int64(stmt, 0);
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return rc == SQLITE_ROW ? 0 : -1;
}
