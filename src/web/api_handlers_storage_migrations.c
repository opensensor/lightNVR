#define _POSIX_C_SOURCE 200809L

#include "web/api_handlers_storage_migrations.h"

#include <cjson/cJSON.h>
#include <math.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "core/authorization.h"
#include "database/db_core.h"
#include "storage/storage_migration.h"
#include "database/db_storage_migrations.h"
#include "storage/storage_migration.h"
#include "utils/strings.h"
#include "utils/uuid.h"
#include "web/audit_log.h"
#include "web/httpd_utils.h"

static bool authorize_storage(const http_request_t *req,
                              http_response_t *res, user_t *user) {
    authorization_evaluation_t evaluation;
    return httpd_authorize_action(req, res, AUTHZ_STORAGE_CONFIGURE, NULL,
                                  user, &evaluation) != 0;
}

static cJSON *job_json(const storage_migration_job_t *job) {
    cJSON *root = cJSON_CreateObject();
    if (!root) return NULL;
    cJSON_AddStringToObject(root, "uuid", job->uuid);
    cJSON_AddNumberToObject(root, "recording_id", (double)job->recording_id);
    cJSON_AddStringToObject(root, "operation", job->operation);
    cJSON_AddStringToObject(root, "source_target_uuid",
                           job->source_target_uuid);
    cJSON_AddStringToObject(root, "source_object_key",
                           job->source_object_key);
    cJSON_AddStringToObject(root, "destination_target_uuid",
                           job->destination_target_uuid);
    cJSON_AddStringToObject(root, "destination_object_key",
                           job->destination_object_key);
    cJSON_AddStringToObject(root, "state", job->state);
    cJSON_AddStringToObject(root, "checksum_mode", "sha256");
    if (job->checksum[0]) {
        cJSON_AddStringToObject(root, "checksum", job->checksum);
    } else {
        cJSON_AddNullToObject(root, "checksum");
    }
    cJSON_AddNumberToObject(root, "bytes_total", (double)job->bytes_total);
    cJSON_AddNumberToObject(root, "bytes_copied", (double)job->bytes_copied);
    double progress = job->bytes_total > 0
        ? (double)job->bytes_copied / (double)job->bytes_total : 0.0;
    cJSON_AddNumberToObject(root, "progress", progress);
    cJSON_AddNumberToObject(root, "attempt_count", job->attempt_count);
    cJSON_AddNumberToObject(root, "max_attempts", job->max_attempts);
    if (job->next_attempt_at > 0) {
        cJSON_AddNumberToObject(root, "next_attempt_at",
                                (double)job->next_attempt_at);
    } else {
        cJSON_AddNullToObject(root, "next_attempt_at");
    }
    cJSON_AddStringToObject(root, "last_error", job->last_error);
    cJSON_AddBoolToObject(root, "cancel_requested", job->cancel_requested);
    cJSON_AddNumberToObject(root, "bandwidth_limit_bps",
                            (double)job->bandwidth_limit_bps);
    cJSON_AddNumberToObject(root, "window_start_minute",
                            job->window_start_minute);
    cJSON_AddNumberToObject(root, "window_end_minute",
                            job->window_end_minute);
    cJSON_AddNumberToObject(root, "revision", (double)job->revision);
    cJSON_AddNumberToObject(root, "created_at", (double)job->created_at);
    cJSON_AddNumberToObject(root, "updated_at", (double)job->updated_at);
    if (job->started_at > 0) {
        cJSON_AddNumberToObject(root, "started_at", (double)job->started_at);
    } else {
        cJSON_AddNullToObject(root, "started_at");
    }
    if (job->completed_at > 0) {
        cJSON_AddNumberToObject(root, "completed_at",
                                (double)job->completed_at);
    } else {
        cJSON_AddNullToObject(root, "completed_at");
    }
    return root;
}

static void send_json(http_response_t *res, int status, cJSON *root) {
    char *encoded = root ? cJSON_PrintUnformatted(root) : NULL;
    cJSON_Delete(root);
    if (!encoded) {
        http_response_set_json_error(res, 500,
                                     "Failed to serialize migration response");
        return;
    }
    http_response_set_json(res, status, encoded);
    free(encoded);
}

static void set_db_error(http_response_t *res,
                         db_storage_migration_result_t result) {
    switch (result) {
        case DB_STORAGE_MIGRATION_NOT_FOUND:
            http_response_set_json_error(res, 404,
                                         "Recording, target, or job not found");
            break;
        case DB_STORAGE_MIGRATION_CONFLICT:
            http_response_set_json_error(
                res, 409,
                "Migration conflicts with the recording, target, or current job state");
            break;
        case DB_STORAGE_MIGRATION_SOURCE_INCOMPLETE:
            http_response_set_json_error(
                res, 409, "Only complete recordings can be migrated");
            break;
        case DB_STORAGE_MIGRATION_TARGET_UNAVAILABLE:
            http_response_set_json_error(
                res, 422, "Destination storage target is unavailable");
            break;
        case DB_STORAGE_MIGRATION_INVALID:
            http_response_set_json_error(res, 400,
                                         "Invalid storage migration request");
            break;
        default:
            http_response_set_json_error(res, 500,
                                         "Storage migration operation failed");
            break;
    }
}

static bool extract_job_uuid(const http_request_t *req,
                             char uuid[LIGHTNVR_UUID_STRING_SIZE],
                             http_response_t *res) {
    char value[MAX_PATH_LENGTH];
    if (http_request_extract_path_param(
            req, "/api/storage-migrations/", value, sizeof(value)) != 0) {
        http_response_set_json_error(res, 400,
                                     "Invalid storage migration path");
        return false;
    }
    char *slash = strchr(value, '/');
    if (slash) *slash = '\0';
    if (!lightnvr_uuid_is_valid(value)) {
        http_response_set_json_error(res, 400,
                                     "Invalid storage migration UUID");
        return false;
    }
    safe_strcpy(uuid, value, LIGHTNVR_UUID_STRING_SIZE, 0);
    return true;
}

void handle_get_storage_migrations(const http_request_t *req,
                                   http_response_t *res) {
    user_t user;
    if (!authorize_storage(req, res, &user)) return;
    storage_migration_job_t *jobs = calloc(STORAGE_MIGRATION_MAX_VISIBLE,
                                           sizeof(*jobs));
    if (!jobs) {
        http_response_set_json_error(res, 500, "Out of memory");
        return;
    }
    int count = db_storage_migration_list(
        jobs, STORAGE_MIGRATION_MAX_VISIBLE);
    cJSON *root = cJSON_CreateObject();
    cJSON *items = root ? cJSON_AddArrayToObject(root, "jobs") : NULL;
    if (count < 0 || !root || !items) {
        free(jobs);
        cJSON_Delete(root);
        http_response_set_json_error(res, 500,
                                     "Failed to list storage migrations");
        return;
    }
    for (int index = 0; index < count; index++) {
        cJSON *item = job_json(&jobs[index]);
        if (!item || !cJSON_AddItemToArray(items, item)) {
            cJSON_Delete(item);
            free(jobs);
            cJSON_Delete(root);
            http_response_set_json_error(
                res, 500, "Failed to create migration response");
            return;
        }
    }
    free(jobs);
    cJSON_AddNumberToObject(root, "count", count);
    cJSON_AddNumberToObject(root, "worker_concurrency", 1);
    send_json(res, 200, root);
}

void handle_post_storage_migration(const http_request_t *req,
                                   http_response_t *res) {
    user_t user;
    if (!authorize_storage(req, res, &user)) return;
    cJSON *body = httpd_parse_json_body(req);
    const cJSON *recording = body
        ? cJSON_GetObjectItemCaseSensitive(body, "recording_id") : NULL;
    const cJSON *destination = body
        ? cJSON_GetObjectItemCaseSensitive(body,
                                           "destination_target_uuid") : NULL;
    const cJSON *operation = body
        ? cJSON_GetObjectItemCaseSensitive(body, "operation") : NULL;
    bool valid_id = cJSON_IsNumber(recording) &&
        isfinite(recording->valuedouble) && recording->valuedouble >= 1.0 &&
        recording->valuedouble <= 9007199254740991.0 &&
        floor(recording->valuedouble) == recording->valuedouble;
    bool valid_destination = cJSON_IsString(destination) &&
        destination->valuestring &&
        lightnvr_uuid_is_valid(destination->valuestring);
    bool valid_operation = !operation ||
        (cJSON_IsString(operation) && operation->valuestring &&
         (strcmp(operation->valuestring, "move") == 0 ||
          strcmp(operation->valuestring, "copy") == 0));
    if (!cJSON_IsObject(body) || !valid_id || !valid_destination ||
        !valid_operation) {
        cJSON_Delete(body);
        http_response_set_json_error(
            res, 400,
            "recording_id, destination_target_uuid, and a valid operation are required");
        return;
    }
    uint64_t recording_id = (uint64_t)recording->valuedouble;
    char destination_uuid[LIGHTNVR_UUID_STRING_SIZE];
    safe_strcpy(destination_uuid, destination->valuestring,
                sizeof(destination_uuid), 0);
    char operation_name[STORAGE_MIGRATION_OPERATION_MAX];
    safe_strcpy(operation_name,
                operation ? operation->valuestring : "move",
                sizeof(operation_name), 0);
    cJSON_Delete(body);

    storage_migration_job_t job;
    db_storage_migration_result_t result =
        db_storage_migration_create_operation(
            recording_id, destination_uuid, operation_name, user.id, &job);
    if (result != DB_STORAGE_MIGRATION_OK) {
        set_db_error(res, result);
        audit_log_operation(req, &user, "storage.configure",
                            "storage_migration", NULL, "migration_create",
                            result == DB_STORAGE_MIGRATION_ERROR
                                ? "error" : "failure", NULL);
        return;
    }
    cJSON *details = cJSON_CreateObject();
    if (details) {
        cJSON_AddNumberToObject(details, "recording_id",
                               (double)recording_id);
        cJSON_AddStringToObject(details, "destination_target_uuid",
                               destination_uuid);
        cJSON_AddStringToObject(details, "operation", operation_name);
    }
    audit_log_operation(req, &user, "storage.configure",
                        "storage_migration", job.uuid, "migration_create",
                        "success", details);
    cJSON_Delete(details);
    storage_migration_worker_wake();
    send_json(res, 202, job_json(&job));
}

void handle_get_storage_migration(const http_request_t *req,
                                  http_response_t *res) {
    user_t user;
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    if (!authorize_storage(req, res, &user) ||
        !extract_job_uuid(req, uuid, res)) return;
    storage_migration_job_t job;
    db_storage_migration_result_t result =
        db_storage_migration_get(uuid, &job);
    if (result != DB_STORAGE_MIGRATION_OK) {
        set_db_error(res, result);
        return;
    }
    send_json(res, 200, job_json(&job));
}

static void handle_job_action(const http_request_t *req,
                              http_response_t *res, bool retry) {
    user_t user;
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    if (!authorize_storage(req, res, &user) ||
        !extract_job_uuid(req, uuid, res)) return;
    storage_migration_job_t job;
    db_storage_migration_result_t result = retry
        ? db_storage_migration_retry(uuid, &job)
        : db_storage_migration_request_cancel(uuid, &job);
    if (result != DB_STORAGE_MIGRATION_OK) {
        set_db_error(res, result);
        audit_log_operation(req, &user, "storage.configure",
                            "storage_migration", uuid,
                            retry ? "migration_retry" : "migration_cancel",
                            result == DB_STORAGE_MIGRATION_ERROR
                                ? "error" : "failure", NULL);
        return;
    }
    audit_log_operation(req, &user, "storage.configure",
                        "storage_migration", uuid,
                        retry ? "migration_retry" : "migration_cancel",
                        "success", NULL);
    if (retry || job.cancel_requested) storage_migration_worker_wake();
    send_json(res, 200, job_json(&job));
}

void handle_post_storage_migration_cancel(const http_request_t *req,
                                          http_response_t *res) {
    handle_job_action(req, res, false);
}

void handle_post_storage_migration_retry(const http_request_t *req,
                                         http_response_t *res) {
    handle_job_action(req, res, true);
}

void handle_get_storage_archive(const http_request_t *req, http_response_t *res) {
    user_t user;
    if (!authorize_storage(req, res, &user)) return;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) { http_response_set_json_error(res, 503, "Catalog unavailable"); return; }
    cJSON *root = cJSON_CreateObject();
    cJSON *items = root ? cJSON_AddArrayToObject(root, "pending_deletions") : NULL;
    if (!items) { cJSON_Delete(root); http_response_set_json_error(res, 500, "Out of memory"); return; }
    cJSON_AddBoolToObject(root, "s3_enabled", LIGHTNVR_ENABLE_S3 != 0);
    const char *read_only = getenv("LIGHTNVR_STORAGE_READ_ONLY");
    cJSON_AddBoolToObject(root, "recovery_read_only", read_only && !strcmp(read_only, "1"));
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, "SELECT d.uuid,d.recording_id,d.created_at,"
        "count(*),MAX(o.attempt_count),MIN(o.next_attempt_at),MAX(o.last_error) "
        "FROM storage_deletions d JOIN storage_deletion_objects o ON o.deletion_uuid=d.uuid "
        "WHERE o.state<>'completed' GROUP BY d.uuid ORDER BY d.created_at LIMIT 100;", -1, &stmt, NULL);
    if (rc == SQLITE_OK) while ((rc = sqlite3_step(stmt)) == SQLITE_ROW) {
        cJSON *item = cJSON_CreateObject();
        if (!item) { rc = SQLITE_NOMEM; break; }
        cJSON_AddStringToObject(item, "uuid", (const char *)sqlite3_column_text(stmt, 0));
        cJSON_AddNumberToObject(item, "recording_id", (double)sqlite3_column_int64(stmt, 1));
        cJSON_AddNumberToObject(item, "created_at", (double)sqlite3_column_int64(stmt, 2));
        cJSON_AddNumberToObject(item, "remaining_objects", sqlite3_column_int(stmt, 3));
        cJSON_AddNumberToObject(item, "attempts", sqlite3_column_int(stmt, 4));
        cJSON_AddNumberToObject(item, "next_attempt_at", (double)sqlite3_column_int64(stmt, 5));
        cJSON_AddStringToObject(item, "last_error", (const char *)sqlite3_column_text(stmt, 6));
        cJSON_AddItemToArray(items, item);
    }
    if (stmt) sqlite3_finalize(stmt);
    bool ok = rc == SQLITE_DONE;
    stmt = NULL;
    rc = sqlite3_prepare_v2(db, "SELECT "
        "COALESCE((SELECT sum(recording_bytes+replica_bytes) FROM storage_targets WHERE target_type='s3'),0),"
        "COALESCE((SELECT sum(bytes_total) FROM storage_migration_jobs WHERE state IN('queued','copying','verifying','committing','retry_wait')),0),"
        "COALESCE((SELECT sum(size_bytes) FROM storage_retrieval_jobs WHERE state='ready'),0),"
        "(SELECT count(*) FROM storage_retrieval_jobs WHERE state IN('queued','fetching')),"
        "(SELECT count(*) FROM storage_deletions WHERE completed_at IS NULL);", -1, &stmt, NULL);
    if (rc == SQLITE_OK && sqlite3_step(stmt) == SQLITE_ROW) {
        const char *fields[] = {"archive_bytes", "transfer_backlog_bytes", "retrieval_cache_bytes", "retrieval_jobs", "pending_deletion_count"};
        for (int i = 0; i < 5; i++) cJSON_AddNumberToObject(root, fields[i], (double)sqlite3_column_int64(stmt, i));
    } else ok = false;
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    if (!ok) { cJSON_Delete(root); http_response_set_json_error(res, 500, "Unable to read archive operations"); return; }
    send_json(res, 200, root);
}

void handle_post_storage_archive_retry(const http_request_t *req, http_response_t *res) {
    user_t user;
    if (!authorize_storage(req, res, &user)) return;
    cJSON *body = req->body ? cJSON_ParseWithLength(req->body, req->body_len) : NULL;
    const cJSON *id = cJSON_GetObjectItemCaseSensitive(body, "deletion_uuid");
    if (!cJSON_IsString(id) || !lightnvr_uuid_is_valid(id->valuestring)) {
        cJSON_Delete(body); http_response_set_json_error(res, 400, "deletion_uuid is required"); return;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    int rc = SQLITE_ERROR, changed = 0;
    if (db && mutex) {
        pthread_mutex_lock(mutex);
        sqlite3_stmt *stmt = NULL;
        rc = sqlite3_prepare_v2(db, "UPDATE storage_deletion_objects SET next_attempt_at=0 "
            "WHERE deletion_uuid=? AND state<>'completed';", -1, &stmt, NULL);
        if (rc == SQLITE_OK) {
            sqlite3_bind_text(stmt, 1, id->valuestring, -1, SQLITE_TRANSIENT);
            rc = sqlite3_step(stmt); changed = sqlite3_changes(db);
        }
        if (stmt) sqlite3_finalize(stmt);
        pthread_mutex_unlock(mutex);
    }
    audit_log_operation(req, &user, "storage.configure", "storage_deletion", id->valuestring,
                        "retry", rc == SQLITE_DONE && changed ? "success" : "failure", NULL);
    cJSON_Delete(body);
    if (rc != SQLITE_DONE || !changed) { http_response_set_json_error(res, rc == SQLITE_DONE ? 404 : 500, "Pending deletion not found or unavailable"); return; }
    storage_migration_worker_wake();
    http_response_set_json(res, 202, "{\"status\":\"retry_queued\"}");
}
