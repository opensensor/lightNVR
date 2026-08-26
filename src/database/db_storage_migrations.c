#define _POSIX_C_SOURCE 200809L

#include "database/db_storage_migrations.h"

#include <pthread.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "database/db_core.h"
#include "utils/strings.h"
#include "utils/uuid.h"

#define MIGRATION_SELECT_FIELDS \
    "j.uuid,j.recording_id,COALESCE(j.owner_user_id,0)," \
    "j.operation,j.source_target_uuid,j.source_object_key,j.destination_target_uuid," \
    "j.destination_object_key,j.state,j.checksum,j.bytes_total," \
    "j.bytes_copied,j.attempt_count,j.max_attempts,j.next_attempt_at," \
    "j.last_error,j.cancel_requested,j.bandwidth_limit_bps," \
    "j.window_start_minute,j.window_end_minute," \
    "j.revision,j.created_at,j.updated_at," \
    "COALESCE(j.started_at,0),COALESCE(j.completed_at,0) "

static void copy_column(char *destination, size_t size,
                        sqlite3_stmt *statement, int column) {
    const char *value = (const char *)sqlite3_column_text(statement, column);
    safe_strcpy(destination, value ? value : "", size, 0);
}

static void populate(sqlite3_stmt *statement, storage_migration_job_t *job) {
    memset(job, 0, sizeof(*job));
    copy_column(job->uuid, sizeof(job->uuid), statement, 0);
    job->recording_id = (uint64_t)sqlite3_column_int64(statement, 1);
    job->owner_user_id = sqlite3_column_int64(statement, 2);
    copy_column(job->operation, sizeof(job->operation), statement, 3);
    copy_column(job->source_target_uuid, sizeof(job->source_target_uuid),
                statement, 4);
    copy_column(job->source_object_key, sizeof(job->source_object_key),
                statement, 5);
    copy_column(job->destination_target_uuid,
                sizeof(job->destination_target_uuid), statement, 6);
    copy_column(job->destination_object_key,
                sizeof(job->destination_object_key), statement, 7);
    copy_column(job->state, sizeof(job->state), statement, 8);
    copy_column(job->checksum, sizeof(job->checksum), statement, 9);
    job->bytes_total = (uint64_t)sqlite3_column_int64(statement, 10);
    job->bytes_copied = (uint64_t)sqlite3_column_int64(statement, 11);
    job->attempt_count = sqlite3_column_int(statement, 12);
    job->max_attempts = sqlite3_column_int(statement, 13);
    job->next_attempt_at = sqlite3_column_int64(statement, 14);
    copy_column(job->last_error, sizeof(job->last_error), statement, 15);
    job->cancel_requested = sqlite3_column_int(statement, 16) != 0;
    job->bandwidth_limit_bps =
        (uint64_t)sqlite3_column_int64(statement, 17);
    job->window_start_minute = sqlite3_column_int(statement, 18);
    job->window_end_minute = sqlite3_column_int(statement, 19);
    job->revision = sqlite3_column_int64(statement, 20);
    job->created_at = sqlite3_column_int64(statement, 21);
    job->updated_at = sqlite3_column_int64(statement, 22);
    job->started_at = sqlite3_column_int64(statement, 23);
    job->completed_at = sqlite3_column_int64(statement, 24);
}

static db_storage_migration_result_t get_locked(
    sqlite3 *db, const char *uuid, storage_migration_job_t *job) {
    const char *sql = "SELECT " MIGRATION_SELECT_FIELDS
        "FROM storage_migration_jobs j WHERE j.uuid=? LIMIT 1;";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    db_storage_migration_result_t outcome = DB_STORAGE_MIGRATION_NOT_FOUND;
    if (result == SQLITE_ROW) {
        populate(statement, job);
        outcome = DB_STORAGE_MIGRATION_OK;
    } else if (result != SQLITE_DONE) {
        outcome = DB_STORAGE_MIGRATION_ERROR;
    }
    if (statement) sqlite3_finalize(statement);
    return outcome;
}

db_storage_migration_result_t db_storage_migration_create(
    uint64_t recording_id, const char *destination_target_uuid,
    int64_t owner_user_id, storage_migration_job_t *job) {
    return db_storage_migration_create_operation(
        recording_id, destination_target_uuid, "move", owner_user_id, job);
}

db_storage_migration_result_t db_storage_migration_create_operation(
    uint64_t recording_id, const char *destination_target_uuid,
    const char *operation, int64_t owner_user_id,
    storage_migration_job_t *job) {
    if (recording_id == 0 || owner_user_id < 0 || !job ||
        !lightnvr_uuid_is_valid(destination_target_uuid) || !operation ||
        (strcmp(operation, "move") != 0 && strcmp(operation, "copy") != 0)) {
        return DB_STORAGE_MIGRATION_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_MIGRATION_ERROR;

    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    if (lightnvr_uuid_generate_v4(uuid) != 0) {
        return DB_STORAGE_MIGRATION_ERROR;
    }
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    const char *recording_sql =
        "SELECT is_complete,COALESCE(storage_target_uuid,''),"
        "COALESCE(object_key,''),MAX(COALESCE(size_bytes,0),0) "
        "FROM recordings WHERE id=? LIMIT 1;";
    int result = sqlite3_prepare_v2(db, recording_sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_int64(statement, 1, (sqlite3_int64)recording_id);
        result = sqlite3_step(statement);
    }
    if (result != SQLITE_ROW) {
        if (statement) sqlite3_finalize(statement);
        pthread_mutex_unlock(mutex);
        return result == SQLITE_DONE ? DB_STORAGE_MIGRATION_NOT_FOUND
                                     : DB_STORAGE_MIGRATION_ERROR;
    }
    bool complete = sqlite3_column_int(statement, 0) != 0;
    char source_target[LIGHTNVR_UUID_STRING_SIZE];
    char source_key[STORAGE_TARGET_OBJECT_KEY_MAX];
    const char *source_target_value =
        (const char *)sqlite3_column_text(statement, 1);
    const char *source_key_value =
        (const char *)sqlite3_column_text(statement, 2);
    safe_strcpy(source_target, source_target_value ? source_target_value : "",
                sizeof(source_target), 0);
    safe_strcpy(source_key, source_key_value ? source_key_value : "",
                sizeof(source_key), 0);
    uint64_t bytes_total = (uint64_t)sqlite3_column_int64(statement, 3);
    sqlite3_finalize(statement);
    statement = NULL;
    if (!complete || source_target[0] == '\0' || source_key[0] == '\0') {
        pthread_mutex_unlock(mutex);
        return !complete ? DB_STORAGE_MIGRATION_SOURCE_INCOMPLETE
                         : DB_STORAGE_MIGRATION_INVALID;
    }
    if (strcmp(source_target, destination_target_uuid) == 0) {
        pthread_mutex_unlock(mutex);
        return DB_STORAGE_MIGRATION_CONFLICT;
    }
    if (strcmp(operation, "copy") == 0) {
        result = sqlite3_prepare_v2(db,
            "SELECT 1 FROM storage_recording_copies WHERE recording_id=? "
            "AND target_uuid=? LIMIT 1;", -1, &statement, NULL);
        if (result == SQLITE_OK) {
            sqlite3_bind_int64(statement, 1, (sqlite3_int64)recording_id);
            sqlite3_bind_text(statement, 2, destination_target_uuid, -1,
                              SQLITE_TRANSIENT);
            result = sqlite3_step(statement);
        }
        if (statement) sqlite3_finalize(statement);
        statement = NULL;
        if (result == SQLITE_ROW) {
            pthread_mutex_unlock(mutex);
            return DB_STORAGE_MIGRATION_CONFLICT;
        }
        if (result != SQLITE_DONE) {
            pthread_mutex_unlock(mutex);
            return DB_STORAGE_MIGRATION_ERROR;
        }
    }

    const char *target_sql =
        "SELECT enabled,health_status,root_path,migration_bandwidth_bps,"
        "archival_window_start_minute,archival_window_end_minute FROM storage_targets "
        "WHERE uuid=? LIMIT 1;";
    result = sqlite3_prepare_v2(db, target_sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, destination_target_uuid, -1,
                          SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (result != SQLITE_ROW) {
        if (statement) sqlite3_finalize(statement);
        pthread_mutex_unlock(mutex);
        return result == SQLITE_DONE ? DB_STORAGE_MIGRATION_NOT_FOUND
                                     : DB_STORAGE_MIGRATION_ERROR;
    }
    bool enabled = sqlite3_column_int(statement, 0) != 0;
    const char *health = (const char *)sqlite3_column_text(statement, 1);
    const char *root_value = (const char *)sqlite3_column_text(statement, 2);
    char health_status[STORAGE_TARGET_HEALTH_MAX];
    char destination_root[MAX_PATH_LENGTH];
    uint64_t bandwidth_limit_bps =
        (uint64_t)sqlite3_column_int64(statement, 3);
    int window_start_minute = sqlite3_column_int(statement, 4);
    int window_end_minute = sqlite3_column_int(statement, 5);
    safe_strcpy(health_status, health ? health : "", sizeof(health_status), 0);
    safe_strcpy(destination_root, root_value ? root_value : "",
                sizeof(destination_root), 0);
    sqlite3_finalize(statement);
    statement = NULL;
    if (!enabled || health_status[0] == '\0' ||
        strcmp(health_status, "unavailable") == 0 ||
        strcmp(health_status, "disabled") == 0) {
        pthread_mutex_unlock(mutex);
        return DB_STORAGE_MIGRATION_TARGET_UNAVAILABLE;
    }
    char destination_path[MAX_PATH_LENGTH];
    int written = snprintf(destination_path, sizeof(destination_path),
                           "%s/%s", destination_root, source_key);
    if (written < 0 || (size_t)written >= sizeof(destination_path) ||
        (size_t)written + strlen(".migration-") + strlen(uuid) +
            strlen(".part") >= sizeof(destination_path)) {
        pthread_mutex_unlock(mutex);
        return DB_STORAGE_MIGRATION_INVALID;
    }

    const char *insert_sql =
        "INSERT INTO storage_migration_jobs("
        "uuid,recording_id,owner_user_id,operation,source_target_uuid,"
        "source_object_key,destination_target_uuid,destination_object_key,"
        "bytes_total,bandwidth_limit_bps,window_start_minute,window_end_minute)"
        " VALUES(?,?,?,?,?,?,?,?,?,?,?,?);";
    result = sqlite3_prepare_v2(db, insert_sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 2, (sqlite3_int64)recording_id);
        if (owner_user_id > 0) {
            sqlite3_bind_int64(statement, 3, owner_user_id);
        } else {
            sqlite3_bind_null(statement, 3);
        }
        sqlite3_bind_text(statement, 4, operation, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 5, source_target, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 6, source_key, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 7, destination_target_uuid, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 8, source_key, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 9, (sqlite3_int64)bytes_total);
        sqlite3_bind_int64(statement, 10,
                           (sqlite3_int64)bandwidth_limit_bps);
        sqlite3_bind_int(statement, 11, window_start_minute);
        sqlite3_bind_int(statement, 12, window_end_minute);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    if (result != SQLITE_DONE) {
        pthread_mutex_unlock(mutex);
        return result == SQLITE_CONSTRAINT ? DB_STORAGE_MIGRATION_CONFLICT
                                           : DB_STORAGE_MIGRATION_ERROR;
    }
    db_storage_migration_result_t outcome = get_locked(db, uuid, job);
    pthread_mutex_unlock(mutex);
    return outcome;
}

db_storage_migration_result_t db_storage_migration_get(
    const char *uuid, storage_migration_job_t *job) {
    if (!lightnvr_uuid_is_valid(uuid) || !job) {
        return DB_STORAGE_MIGRATION_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_MIGRATION_ERROR;
    pthread_mutex_lock(mutex);
    db_storage_migration_result_t result = get_locked(db, uuid, job);
    pthread_mutex_unlock(mutex);
    return result;
}

int db_storage_migration_list(storage_migration_job_t *jobs, int max_count) {
    if (!jobs || max_count < 1 || max_count > STORAGE_MIGRATION_MAX_VISIBLE) {
        return -1;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    const char *sql = "SELECT " MIGRATION_SELECT_FIELDS
        "FROM storage_migration_jobs j ORDER BY j.created_at DESC,j.uuid "
        "LIMIT ?;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) sqlite3_bind_int(statement, 1, max_count);
    int count = 0;
    while ((result == SQLITE_OK || result == SQLITE_ROW) &&
           count < max_count) {
        result = sqlite3_step(statement);
        if (result != SQLITE_ROW) break;
        populate(statement, &jobs[count++]);
    }
    if (result != SQLITE_DONE && result != SQLITE_ROW) count = -1;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return count;
}

int db_storage_migration_claim_due(storage_migration_job_t *job) {
    if (!job) return -1;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    const char *select_sql = "SELECT " MIGRATION_SELECT_FIELDS
        "FROM storage_migration_jobs j WHERE "
        "(j.state='cleanup_pending' AND j.next_attempt_at<=strftime('%s','now')) OR "
        "(j.cancel_requested=0 AND j.attempt_count<j.max_attempts AND ("
        "j.state IN('queued','copying','verifying','committing') OR "
        "(j.state='retry_wait' AND j.next_attempt_at<=strftime('%s','now'))) AND ("
        "j.window_start_minute=j.window_end_minute OR "
        "(j.window_start_minute<j.window_end_minute AND "
        " (CAST(strftime('%H','now','localtime') AS INTEGER)*60+"
        "  CAST(strftime('%M','now','localtime') AS INTEGER))>=j.window_start_minute AND "
        " (CAST(strftime('%H','now','localtime') AS INTEGER)*60+"
        "  CAST(strftime('%M','now','localtime') AS INTEGER))<j.window_end_minute) OR "
        "(j.window_start_minute>j.window_end_minute AND ("
        " (CAST(strftime('%H','now','localtime') AS INTEGER)*60+"
        "  CAST(strftime('%M','now','localtime') AS INTEGER))>=j.window_start_minute OR "
        " (CAST(strftime('%H','now','localtime') AS INTEGER)*60+"
        "  CAST(strftime('%M','now','localtime') AS INTEGER))<j.window_end_minute)))) "
        "ORDER BY CASE WHEN j.state='cleanup_pending' THEN 0 ELSE 1 END,"
        "j.created_at,j.uuid LIMIT 1;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, select_sql, -1, &statement, NULL);
    if (result == SQLITE_OK) result = sqlite3_step(statement);
    if (result == SQLITE_DONE) {
        sqlite3_finalize(statement);
        pthread_mutex_unlock(mutex);
        return 0;
    }
    if (result != SQLITE_ROW) {
        if (statement) sqlite3_finalize(statement);
        pthread_mutex_unlock(mutex);
        return -1;
    }
    populate(statement, job);
    sqlite3_finalize(statement);
    statement = NULL;
    if (strcmp(job->state, "cleanup_pending") != 0) {
        const char *safe_update_sql =
            "UPDATE storage_migration_jobs SET state='copying',"
            "attempt_count=attempt_count+1,bytes_copied=0,last_error='',"
            "next_attempt_at=0,started_at=COALESCE(started_at,strftime('%s','now')),"
            "updated_at=strftime('%s','now'),revision=revision+1 WHERE uuid=?;";
        result = sqlite3_prepare_v2(db, safe_update_sql, -1, &statement, NULL);
        if (result == SQLITE_OK) {
            sqlite3_bind_text(statement, 1, job->uuid, -1, SQLITE_TRANSIENT);
            result = sqlite3_step(statement);
        }
        if (statement) sqlite3_finalize(statement);
        if (result != SQLITE_DONE ||
            get_locked(db, job->uuid, job) != DB_STORAGE_MIGRATION_OK) {
            pthread_mutex_unlock(mutex);
            return -1;
        }
    }
    pthread_mutex_unlock(mutex);
    return 1;
}

static bool valid_worker_state(const char *state) {
    return state && (strcmp(state, "copying") == 0 ||
                     strcmp(state, "verifying") == 0 ||
                     strcmp(state, "committing") == 0);
}

db_storage_migration_result_t db_storage_migration_update_progress(
    const char *uuid, const char *state, uint64_t bytes_copied,
    uint64_t bytes_total) {
    if (!lightnvr_uuid_is_valid(uuid) || !valid_worker_state(state) ||
        bytes_copied > bytes_total || bytes_total > (uint64_t)INT64_MAX) {
        return DB_STORAGE_MIGRATION_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_MIGRATION_ERROR;
    const char *sql =
        "UPDATE storage_migration_jobs SET state=?,bytes_copied=?,"
        "bytes_total=?,updated_at=strftime('%s','now'),revision=revision+1 "
        "WHERE uuid=? AND state NOT IN('completed','failed','cancelled');";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, state, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 2, (sqlite3_int64)bytes_copied);
        sqlite3_bind_int64(statement, 3, (sqlite3_int64)bytes_total);
        sqlite3_bind_text(statement, 4, uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return result == SQLITE_DONE && changed == 1
        ? DB_STORAGE_MIGRATION_OK : DB_STORAGE_MIGRATION_ERROR;
}

db_storage_migration_result_t db_storage_migration_record_failure(
    const storage_migration_job_t *job, const char *error, bool retryable) {
    if (!job || !lightnvr_uuid_is_valid(job->uuid) || !error) {
        return DB_STORAGE_MIGRATION_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_MIGRATION_ERROR;
    bool retry = retryable && job->attempt_count < job->max_attempts;
    int shift = job->attempt_count > 8 ? 8 : job->attempt_count;
    int backoff = 1 << shift;
    char normalized_error[STORAGE_MIGRATION_ERROR_MAX];
    safe_strcpy(normalized_error, error, sizeof(normalized_error), 0);
    const char *sql = retry
        ? "UPDATE storage_migration_jobs SET state='retry_wait',last_error=?,"
          "next_attempt_at=strftime('%s','now')+?,updated_at=strftime('%s','now'),"
          "revision=revision+1 WHERE uuid=?;"
        : "UPDATE storage_migration_jobs SET state='failed',last_error=?,"
          "next_attempt_at=0,completed_at=strftime('%s','now'),"
          "updated_at=strftime('%s','now'),revision=revision+1 WHERE uuid=?;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, normalized_error, -1,
                          SQLITE_TRANSIENT);
        int parameter = 2;
        if (retry) sqlite3_bind_int(statement, parameter++, backoff);
        sqlite3_bind_text(statement, parameter, job->uuid, -1,
                          SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return result == SQLITE_DONE ? DB_STORAGE_MIGRATION_OK
                                 : DB_STORAGE_MIGRATION_ERROR;
}

db_storage_migration_result_t db_storage_migration_commit_location(
    const storage_migration_job_t *job, const char *destination_path,
    const char *checksum) {
    if (!job || !lightnvr_uuid_is_valid(job->uuid) || !destination_path ||
        destination_path[0] != '/' || !checksum || strlen(checksum) != 64) {
        return DB_STORAGE_MIGRATION_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_MIGRATION_ERROR;
    pthread_mutex_lock(mutex);
    char *error = NULL;
    int result = sqlite3_exec(db, "BEGIN IMMEDIATE;", NULL, NULL, &error);
    if (error) sqlite3_free(error);
    sqlite3_stmt *statement = NULL;
    if (result == SQLITE_OK) {
        const char *recording_sql =
            "UPDATE recordings SET storage_target_uuid=?,object_key=?,"
            "file_path=?,placement_reason='migration:'||? WHERE id=? AND "
            "is_complete=1 AND storage_target_uuid=? AND object_key=?;";
        result = sqlite3_prepare_v2(db, recording_sql, -1, &statement, NULL);
        if (result == SQLITE_OK) {
            sqlite3_bind_text(statement, 1, job->destination_target_uuid, -1,
                              SQLITE_TRANSIENT);
            sqlite3_bind_text(statement, 2, job->destination_object_key, -1,
                              SQLITE_TRANSIENT);
            sqlite3_bind_text(statement, 3, destination_path, -1,
                              SQLITE_TRANSIENT);
            sqlite3_bind_text(statement, 4, job->uuid, -1, SQLITE_TRANSIENT);
            sqlite3_bind_int64(statement, 5,
                               (sqlite3_int64)job->recording_id);
            sqlite3_bind_text(statement, 6, job->source_target_uuid, -1,
                              SQLITE_TRANSIENT);
            sqlite3_bind_text(statement, 7, job->source_object_key, -1,
                              SQLITE_TRANSIENT);
            result = sqlite3_step(statement);
        }
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (statement) sqlite3_finalize(statement);
    statement = NULL;
    if (result == SQLITE_DONE && changed == 1) {
        const char *job_sql =
            "UPDATE storage_migration_jobs SET state='cleanup_pending',"
            "checksum=?,bytes_copied=bytes_total,last_error='',"
            "updated_at=strftime('%s','now'),revision=revision+1 WHERE uuid=? "
            "AND state NOT IN('completed','failed','cancelled') "
            "AND cancel_requested=0;";
        result = sqlite3_prepare_v2(db, job_sql, -1, &statement, NULL);
        if (result == SQLITE_OK) {
            sqlite3_bind_text(statement, 1, checksum, -1, SQLITE_TRANSIENT);
            sqlite3_bind_text(statement, 2, job->uuid, -1, SQLITE_TRANSIENT);
            result = sqlite3_step(statement);
        }
        changed = result == SQLITE_DONE ? sqlite3_changes(db) : 0;
    }
    if (statement) sqlite3_finalize(statement);
    bool committed = result == SQLITE_DONE && changed == 1;
    error = NULL;
    if (committed) {
        result = sqlite3_exec(db, "COMMIT;", NULL, NULL, &error);
        committed = result == SQLITE_OK;
    } else {
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
    }
    if (error) sqlite3_free(error);
    if (!committed && result != SQLITE_OK) {
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
    }
    pthread_mutex_unlock(mutex);
    if (committed) return DB_STORAGE_MIGRATION_OK;
    return changed == 0 ? DB_STORAGE_MIGRATION_SOURCE_CHANGED
                        : DB_STORAGE_MIGRATION_ERROR;
}

db_storage_migration_result_t db_storage_migration_complete_cleanup(
    const char *uuid) {
    if (!lightnvr_uuid_is_valid(uuid)) return DB_STORAGE_MIGRATION_INVALID;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_MIGRATION_ERROR;
    const char *sql =
        "UPDATE storage_migration_jobs SET state='completed',last_error='',"
        "next_attempt_at=0,completed_at=strftime('%s','now'),"
        "updated_at=strftime('%s','now'),revision=revision+1 "
        "WHERE uuid=? AND state='cleanup_pending';";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return result == SQLITE_DONE && changed == 1
        ? DB_STORAGE_MIGRATION_OK : DB_STORAGE_MIGRATION_ERROR;
}

db_storage_migration_result_t db_storage_migration_defer_cleanup(
    const storage_migration_job_t *job, const char *error) {
    if (!job || !lightnvr_uuid_is_valid(job->uuid) || !error) {
        return DB_STORAGE_MIGRATION_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_MIGRATION_ERROR;
    int shift = job->attempt_count > 8 ? 8 : job->attempt_count;
    int backoff = 1 << shift;
    char normalized_error[STORAGE_MIGRATION_ERROR_MAX];
    safe_strcpy(normalized_error, error, sizeof(normalized_error), 0);
    const char *sql =
        "UPDATE storage_migration_jobs SET last_error=?,"
        "attempt_count=attempt_count+1,"
        "next_attempt_at=strftime('%s','now')+?,"
        "updated_at=strftime('%s','now'),revision=revision+1 "
        "WHERE uuid=? AND state='cleanup_pending';";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, normalized_error, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 2, backoff);
        sqlite3_bind_text(statement, 3, job->uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return result == SQLITE_DONE && changed == 1
        ? DB_STORAGE_MIGRATION_OK : DB_STORAGE_MIGRATION_ERROR;
}

db_storage_migration_result_t db_storage_migration_commit_copy(
    const storage_migration_job_t *job, const char *checksum) {
    if (!job || strcmp(job->operation, "copy") != 0 ||
        !lightnvr_uuid_is_valid(job->uuid) || !checksum ||
        strlen(checksum) != 64 || job->bytes_total > (uint64_t)INT64_MAX) {
        return DB_STORAGE_MIGRATION_INVALID;
    }
    char copy_uuid[LIGHTNVR_UUID_STRING_SIZE];
    if (lightnvr_uuid_generate_v4(copy_uuid) != 0) {
        return DB_STORAGE_MIGRATION_ERROR;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_MIGRATION_ERROR;
    pthread_mutex_lock(mutex);
    int result = sqlite3_exec(db, "BEGIN IMMEDIATE;", NULL, NULL, NULL);
    sqlite3_stmt *statement = NULL;
    int source_matches = 0;
    if (result == SQLITE_OK) {
        const char *source_sql =
            "SELECT count(*) FROM recordings WHERE id=? AND is_complete=1 "
            "AND storage_target_uuid=? AND object_key=?;";
        result = sqlite3_prepare_v2(db, source_sql, -1, &statement, NULL);
        if (result == SQLITE_OK) {
            sqlite3_bind_int64(statement, 1,
                               (sqlite3_int64)job->recording_id);
            sqlite3_bind_text(statement, 2, job->source_target_uuid, -1,
                              SQLITE_TRANSIENT);
            sqlite3_bind_text(statement, 3, job->source_object_key, -1,
                              SQLITE_TRANSIENT);
            if (sqlite3_step(statement) == SQLITE_ROW) {
                source_matches = sqlite3_column_int(statement, 0);
                result = SQLITE_OK;
            } else {
                result = SQLITE_ERROR;
            }
        }
    }
    if (statement) sqlite3_finalize(statement);
    statement = NULL;
    if (result == SQLITE_OK && source_matches == 1) {
        const char *copy_sql =
            "INSERT INTO storage_recording_copies("
            "uuid,recording_id,target_uuid,object_key,checksum,size_bytes) "
            "VALUES(?,?,?,?,?,?);";
        result = sqlite3_prepare_v2(db, copy_sql, -1, &statement, NULL);
        if (result == SQLITE_OK) {
            sqlite3_bind_text(statement, 1, copy_uuid, -1, SQLITE_TRANSIENT);
            sqlite3_bind_int64(statement, 2,
                               (sqlite3_int64)job->recording_id);
            sqlite3_bind_text(statement, 3, job->destination_target_uuid, -1,
                              SQLITE_TRANSIENT);
            sqlite3_bind_text(statement, 4, job->destination_object_key, -1,
                              SQLITE_TRANSIENT);
            sqlite3_bind_text(statement, 5, checksum, -1, SQLITE_TRANSIENT);
            sqlite3_bind_int64(statement, 6,
                               (sqlite3_int64)job->bytes_total);
            result = sqlite3_step(statement);
        }
    }
    if (statement) sqlite3_finalize(statement);
    statement = NULL;
    int changed = result == SQLITE_DONE ? 1 : 0;
    if (changed) {
        const char *job_sql =
            "UPDATE storage_migration_jobs SET state='completed',checksum=?,"
            "bytes_copied=bytes_total,last_error='',next_attempt_at=0,"
            "completed_at=strftime('%s','now'),updated_at=strftime('%s','now'),"
            "revision=revision+1 WHERE uuid=? AND state='committing' "
            "AND cancel_requested=0;";
        result = sqlite3_prepare_v2(db, job_sql, -1, &statement, NULL);
        if (result == SQLITE_OK) {
            sqlite3_bind_text(statement, 1, checksum, -1, SQLITE_TRANSIENT);
            sqlite3_bind_text(statement, 2, job->uuid, -1, SQLITE_TRANSIENT);
            result = sqlite3_step(statement);
        }
        changed = result == SQLITE_DONE ? sqlite3_changes(db) : 0;
    }
    if (statement) sqlite3_finalize(statement);
    bool committed = result == SQLITE_DONE && changed == 1;
    if (committed) {
        committed = sqlite3_exec(db, "COMMIT;", NULL, NULL, NULL) == SQLITE_OK;
    }
    if (!committed) sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
    int extended = sqlite3_extended_errcode(db);
    pthread_mutex_unlock(mutex);
    if (committed) return DB_STORAGE_MIGRATION_OK;
    if (source_matches != 1) return DB_STORAGE_MIGRATION_SOURCE_CHANGED;
    return extended == SQLITE_CONSTRAINT
        ? DB_STORAGE_MIGRATION_CONFLICT : DB_STORAGE_MIGRATION_ERROR;
}

bool db_storage_migration_cancel_requested(const char *uuid) {
    if (!lightnvr_uuid_is_valid(uuid)) return false;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return false;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    bool requested = false;
    if (sqlite3_prepare_v2(
            db, "SELECT cancel_requested FROM storage_migration_jobs "
                "WHERE uuid=? LIMIT 1;", -1, &statement, NULL) == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        requested = sqlite3_step(statement) == SQLITE_ROW &&
            sqlite3_column_int(statement, 0) != 0;
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return requested;
}

db_storage_migration_result_t db_storage_migration_request_cancel(
    const char *uuid, storage_migration_job_t *job) {
    if (!lightnvr_uuid_is_valid(uuid) || !job) {
        return DB_STORAGE_MIGRATION_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_MIGRATION_ERROR;
    pthread_mutex_lock(mutex);
    storage_migration_job_t current;
    db_storage_migration_result_t outcome = get_locked(db, uuid, &current);
    if (outcome != DB_STORAGE_MIGRATION_OK) {
        pthread_mutex_unlock(mutex);
        return outcome;
    }
    bool immediately = strcmp(current.state, "queued") == 0 ||
        strcmp(current.state, "retry_wait") == 0;
    bool running = strcmp(current.state, "copying") == 0 ||
        strcmp(current.state, "verifying") == 0 ||
        strcmp(current.state, "committing") == 0;
    if (!immediately && !running) {
        pthread_mutex_unlock(mutex);
        return DB_STORAGE_MIGRATION_CONFLICT;
    }
    const char *sql = immediately
        ? "UPDATE storage_migration_jobs SET state='cancelled',"
          "cancel_requested=1,last_error='Cancelled by operator',"
          "next_attempt_at=0,completed_at=strftime('%s','now'),"
          "updated_at=strftime('%s','now'),revision=revision+1 WHERE uuid=?;"
        : "UPDATE storage_migration_jobs SET cancel_requested=1,"
          "last_error='Cancellation requested by operator',"
          "updated_at=strftime('%s','now'),revision=revision+1 WHERE uuid=?;";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    outcome = result == SQLITE_DONE && sqlite3_changes(db) == 1
        ? get_locked(db, uuid, job) : DB_STORAGE_MIGRATION_ERROR;
    pthread_mutex_unlock(mutex);
    return outcome;
}

db_storage_migration_result_t db_storage_migration_mark_cancelled(
    const char *uuid, storage_migration_job_t *job) {
    if (!lightnvr_uuid_is_valid(uuid) || !job) {
        return DB_STORAGE_MIGRATION_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_MIGRATION_ERROR;
    const char *sql =
        "UPDATE storage_migration_jobs SET state='cancelled',"
        "cancel_requested=1,last_error='Cancelled by operator',"
        "next_attempt_at=0,completed_at=strftime('%s','now'),"
        "updated_at=strftime('%s','now'),revision=revision+1 WHERE uuid=? "
        "AND cancel_requested=1 AND state IN('copying','verifying','committing');";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (statement) sqlite3_finalize(statement);
    db_storage_migration_result_t outcome = changed == 1
        ? get_locked(db, uuid, job) : DB_STORAGE_MIGRATION_CONFLICT;
    pthread_mutex_unlock(mutex);
    return outcome;
}

db_storage_migration_result_t db_storage_migration_retry(
    const char *uuid, storage_migration_job_t *job) {
    if (!lightnvr_uuid_is_valid(uuid) || !job) {
        return DB_STORAGE_MIGRATION_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_MIGRATION_ERROR;
    const char *sql =
        "UPDATE storage_migration_jobs SET state='queued',cancel_requested=0,"
        "checksum='',bytes_copied=0,attempt_count=0,next_attempt_at=0,"
        "last_error='',started_at=NULL,completed_at=NULL,"
        "updated_at=strftime('%s','now'),revision=revision+1 WHERE uuid=? "
        "AND state IN('failed','cancelled');";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (statement) sqlite3_finalize(statement);
    db_storage_migration_result_t outcome;
    if (result == SQLITE_CONSTRAINT) {
        outcome = DB_STORAGE_MIGRATION_CONFLICT;
    } else if (changed == 1) {
        outcome = get_locked(db, uuid, job);
    } else {
        storage_migration_job_t current;
        outcome = get_locked(db, uuid, &current) == DB_STORAGE_MIGRATION_OK
            ? DB_STORAGE_MIGRATION_CONFLICT : DB_STORAGE_MIGRATION_NOT_FOUND;
    }
    pthread_mutex_unlock(mutex);
    return outcome;
}
