#define _POSIX_C_SOURCE 200809L

#include "web/api_handlers_storage_migrations.h"

#include <cjson/cJSON.h>
#include <math.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "core/authorization.h"
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
    cJSON_AddStringToObject(root, "operation", "move");
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
                "Recording already has an active migration or is on that target");
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
    if (http_request_extract_path_param(
            req, "/api/storage-migrations/", uuid,
            LIGHTNVR_UUID_STRING_SIZE) != 0 ||
        !lightnvr_uuid_is_valid(uuid)) {
        http_response_set_json_error(res, 400,
                                     "Invalid storage migration UUID");
        return false;
    }
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
    bool valid_id = cJSON_IsNumber(recording) &&
        isfinite(recording->valuedouble) && recording->valuedouble >= 1.0 &&
        recording->valuedouble <= 9007199254740991.0 &&
        floor(recording->valuedouble) == recording->valuedouble;
    bool valid_destination = cJSON_IsString(destination) &&
        destination->valuestring &&
        lightnvr_uuid_is_valid(destination->valuestring);
    if (!cJSON_IsObject(body) || !valid_id || !valid_destination) {
        cJSON_Delete(body);
        http_response_set_json_error(
            res, 400,
            "recording_id and destination_target_uuid are required");
        return;
    }
    uint64_t recording_id = (uint64_t)recording->valuedouble;
    char destination_uuid[LIGHTNVR_UUID_STRING_SIZE];
    safe_strcpy(destination_uuid, destination->valuestring,
                sizeof(destination_uuid), 0);
    cJSON_Delete(body);

    storage_migration_job_t job;
    db_storage_migration_result_t result = db_storage_migration_create(
        recording_id, destination_uuid, user.id, &job);
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
