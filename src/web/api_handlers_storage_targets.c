#define _POSIX_C_SOURCE 200809L

#include "web/api_handlers_storage_targets.h"

#include <cjson/cJSON.h>
#include <errno.h>
#include <math.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "core/authorization.h"
#include "database/db_storage_targets.h"
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

static bool set_json_response(http_response_t *res, int status,
                              cJSON *object) {
    char *body = object ? cJSON_PrintUnformatted(object) : NULL;
    cJSON_Delete(object);
    if (!body) {
        http_response_set_json_error(res, 500,
                                     "Failed to serialize storage target response");
        return false;
    }
    http_response_set_json(res, status, body);
    free(body);
    return true;
}

static cJSON *target_to_json(const storage_target_t *target,
                             bool duplicate_filesystem) {
    cJSON *object = cJSON_CreateObject();
    cJSON *health = cJSON_CreateObject();
    if (!object || !health) {
        cJSON_Delete(object);
        cJSON_Delete(health);
        return NULL;
    }
    cJSON_AddStringToObject(object, "uuid", target->uuid);
    cJSON_AddStringToObject(object, "name", target->name);
    cJSON_AddStringToObject(object, "target_type", target->target_type);
    cJSON_AddStringToObject(object, "root_path", target->root_path);
    cJSON_AddBoolToObject(object, "enabled", target->enabled);
    cJSON_AddBoolToObject(object, "is_default", target->is_default);
    cJSON_AddStringToObject(object, "storage_class", target->storage_class);
    cJSON_AddNumberToObject(object, "reserve_bytes",
                            (double)target->reserve_bytes);
    cJSON_AddNumberToObject(object, "high_watermark_pct",
                            target->high_watermark_pct);
    cJSON_AddNumberToObject(object, "low_watermark_pct",
                            target->low_watermark_pct);
    cJSON_AddNumberToObject(object, "recording_count",
                            (double)target->recording_count);
    cJSON_AddNumberToObject(object, "recording_bytes",
                            (double)target->recording_bytes);
    cJSON_AddNumberToObject(object, "revision", (double)target->revision);
    cJSON_AddNumberToObject(object, "created_at",
                            (double)target->created_at);
    cJSON_AddNumberToObject(object, "updated_at",
                            (double)target->updated_at);

    uint64_t used = target->capacity_bytes > target->available_bytes
        ? target->capacity_bytes - target->available_bytes : 0;
    double used_pct = target->capacity_bytes > 0
        ? 100.0 * (double)used / (double)target->capacity_bytes : 0.0;
    cJSON_AddStringToObject(health, "status", target->health_status);
    cJSON_AddNumberToObject(health, "capacity_bytes",
                            (double)target->capacity_bytes);
    cJSON_AddNumberToObject(health, "available_bytes",
                            (double)target->available_bytes);
    cJSON_AddNumberToObject(health, "used_bytes", (double)used);
    cJSON_AddNumberToObject(health, "used_pct", used_pct);
    cJSON_AddNumberToObject(health, "filesystem_device",
                            (double)target->filesystem_device);
    cJSON_AddBoolToObject(health, "duplicate_filesystem",
                          duplicate_filesystem);
    if (target->last_probe_at > 0) {
        cJSON_AddNumberToObject(health, "last_probe_at",
                                (double)target->last_probe_at);
    } else {
        cJSON_AddNullToObject(health, "last_probe_at");
    }
    if (target->last_success_at > 0) {
        cJSON_AddNumberToObject(health, "last_success_at",
                                (double)target->last_success_at);
    } else {
        cJSON_AddNullToObject(health, "last_success_at");
    }
    cJSON_AddStringToObject(health, "last_error", target->last_error);
    cJSON_AddItemToObject(object, "health", health);
    return object;
}

static void set_db_error(http_response_t *res,
                         db_storage_target_result_t result,
                         const char *validation_error) {
    switch (result) {
        case DB_STORAGE_TARGET_NOT_FOUND:
            http_response_set_json_error(res, 404, "Storage target not found");
            break;
        case DB_STORAGE_TARGET_CONFLICT:
            http_response_set_json_error(
                res, 409, "A storage target already uses that name or root path");
            break;
        case DB_STORAGE_TARGET_STALE:
            http_response_set_json_error(
                res, 409, "Storage target was changed by another administrator");
            break;
        case DB_STORAGE_TARGET_LIMIT:
            http_response_set_json_error(res, 409,
                                         "Storage target limit reached");
            break;
        case DB_STORAGE_TARGET_IN_USE:
            http_response_set_json_error(
                res, 409,
                "Default or nonempty storage targets cannot be removed or repointed");
            break;
        case DB_STORAGE_TARGET_UNAVAILABLE:
            http_response_set_json_error(
                res, 422,
                "Storage target is unavailable or failed its write probe");
            break;
        case DB_STORAGE_TARGET_INVALID:
            http_response_set_json_error(
                res, 400, validation_error && validation_error[0]
                    ? validation_error : "Invalid storage target");
            break;
        default:
            http_response_set_json_error(res, 500,
                                         "Storage target operation failed");
            break;
    }
}

static bool extract_target_uuid(
    const http_request_t *req, char uuid[LIGHTNVR_UUID_STRING_SIZE],
    http_response_t *res) {
    char value[MAX_PATH_LENGTH];
    if (http_request_extract_path_param(
            req, "/api/storage-targets/", value, sizeof(value)) != 0) {
        http_response_set_json_error(res, 400, "Invalid storage target path");
        return false;
    }
    char *slash = strchr(value, '/');
    if (slash) *slash = '\0';
    if (!lightnvr_uuid_is_valid(value)) {
        http_response_set_json_error(res, 400,
                                     "Invalid storage target UUID");
        return false;
    }
    safe_strcpy(uuid, value, LIGHTNVR_UUID_STRING_SIZE, 0);
    return true;
}

static bool json_string(const cJSON *body, const char *key,
                        char *destination, size_t destination_size,
                        bool required, http_response_t *res) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(body, key);
    if (!item) {
        if (!required) return true;
        char message[160];
        snprintf(message, sizeof(message), "%s is required", key);
        http_response_set_json_error(res, 400, message);
        return false;
    }
    if (!cJSON_IsString(item) || !item->valuestring ||
        strnlen(item->valuestring, destination_size) >= destination_size) {
        char message[160];
        snprintf(message, sizeof(message), "%s must be a valid string", key);
        http_response_set_json_error(res, 400, message);
        return false;
    }
    safe_strcpy(destination, item->valuestring, destination_size, 0);
    return true;
}

static bool json_bool(const cJSON *body, const char *key, bool *destination,
                      bool required, http_response_t *res) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(body, key);
    if (!item) {
        if (!required) return true;
        char message[160];
        snprintf(message, sizeof(message), "%s is required", key);
        http_response_set_json_error(res, 400, message);
        return false;
    }
    if (!cJSON_IsBool(item)) {
        char message[160];
        snprintf(message, sizeof(message), "%s must be boolean", key);
        http_response_set_json_error(res, 400, message);
        return false;
    }
    *destination = cJSON_IsTrue(item);
    return true;
}

static bool json_number(const cJSON *body, const char *key, double *destination,
                        bool required, http_response_t *res) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(body, key);
    if (!item) {
        if (!required) return true;
        char message[160];
        snprintf(message, sizeof(message), "%s is required", key);
        http_response_set_json_error(res, 400, message);
        return false;
    }
    if (!cJSON_IsNumber(item) || !isfinite(item->valuedouble)) {
        char message[160];
        snprintf(message, sizeof(message), "%s must be a finite number", key);
        http_response_set_json_error(res, 400, message);
        return false;
    }
    *destination = item->valuedouble;
    return true;
}

static bool apply_body(const cJSON *body, storage_target_t *target,
                       bool create, int64_t *revision,
                       http_response_t *res) {
    if (!cJSON_IsObject(body)) {
        http_response_set_json_error(res, 400,
                                     "Request body must be a JSON object");
        return false;
    }
    if (!json_string(body, "name", target->name, sizeof(target->name),
                     create, res) ||
        !json_string(body, "root_path", target->root_path,
                     sizeof(target->root_path), create, res) ||
        !json_string(body, "storage_class", target->storage_class,
                     sizeof(target->storage_class), false, res) ||
        !json_bool(body, "enabled", &target->enabled, false, res)) {
        return false;
    }
    double number = 0.0;
    if (cJSON_HasObjectItem(body, "reserve_bytes")) {
        if (!json_number(body, "reserve_bytes", &number, true, res) ||
            number < 0.0 || number > (double)INT64_MAX) {
            http_response_set_json_error(res, 400,
                                         "reserve_bytes is out of range");
            return false;
        }
        target->reserve_bytes = (uint64_t)number;
    }
    if (cJSON_HasObjectItem(body, "high_watermark_pct") &&
        !json_number(body, "high_watermark_pct",
                     &target->high_watermark_pct, true, res)) return false;
    if (cJSON_HasObjectItem(body, "low_watermark_pct") &&
        !json_number(body, "low_watermark_pct",
                     &target->low_watermark_pct, true, res)) return false;
    if (!create) {
        if (!json_number(body, "revision", &number, true, res) ||
            number < 1.0 || floor(number) != number ||
            number > (double)INT64_MAX) {
            http_response_set_json_error(res, 400,
                                         "revision must be a positive integer");
            return false;
        }
        *revision = (int64_t)number;
    }
    char validation_error[STORAGE_TARGET_ERROR_MAX] = {0};
    db_storage_target_result_t result = db_storage_target_validate(
        target, validation_error, sizeof(validation_error));
    if (result != DB_STORAGE_TARGET_OK) {
        set_db_error(res, result, validation_error);
        return false;
    }
    return true;
}

static void audit_target(const http_request_t *req, const user_t *user,
                         const char *uuid, const char *operation,
                         const char *outcome, const char *reason) {
    cJSON *context = cJSON_CreateObject();
    if (context && reason) cJSON_AddStringToObject(context, "reason", reason);
    audit_log_operation(req, user, "storage.configure", "storage_target",
                        uuid, operation, outcome, context);
    cJSON_Delete(context);
}

void handle_get_storage_targets(const http_request_t *req,
                                http_response_t *res) {
    user_t user;
    if (!authorize_storage(req, res, &user)) return;
    int total = db_storage_target_count();
    if (total < 0 || total > STORAGE_TARGET_MAX_COUNT) {
        set_db_error(res, DB_STORAGE_TARGET_ERROR, NULL);
        return;
    }
    storage_target_t *targets = total > 0
        ? calloc((size_t)total, sizeof(*targets)) : NULL;
    if (total > 0 && !targets) {
        http_response_set_json_error(res, 500, "Out of memory");
        return;
    }
    int count = total == 0 ? 0 : db_storage_target_list(targets, total);
    if (count < 0) {
        free(targets);
        set_db_error(res, DB_STORAGE_TARGET_ERROR, NULL);
        return;
    }
    cJSON *root = cJSON_CreateObject();
    cJSON *items = cJSON_CreateArray();
    if (!root || !items) {
        cJSON_Delete(root);
        cJSON_Delete(items);
        free(targets);
        set_db_error(res, DB_STORAGE_TARGET_ERROR, NULL);
        return;
    }
    cJSON_AddNumberToObject(root, "count", count);
    cJSON_AddItemToObject(root, "targets", items);
    for (int index = 0; index < count; index++) {
        bool duplicate = false;
        if (targets[index].filesystem_device != 0) {
            for (int other = 0; other < count; other++) {
                if (other != index &&
                    targets[other].filesystem_device ==
                        targets[index].filesystem_device) {
                    duplicate = true;
                    break;
                }
            }
        }
        cJSON *item = target_to_json(&targets[index], duplicate);
        if (!item) {
            cJSON_Delete(root);
            free(targets);
            set_db_error(res, DB_STORAGE_TARGET_ERROR, NULL);
            return;
        }
        cJSON_AddItemToArray(items, item);
    }
    free(targets);
    set_json_response(res, 200, root);
}

void handle_post_storage_target(const http_request_t *req,
                                http_response_t *res) {
    user_t user;
    if (!authorize_storage(req, res, &user)) return;
    cJSON *body = httpd_parse_json_body(req);
    storage_target_t target;
    memset(&target, 0, sizeof(target));
    safe_strcpy(target.target_type, "filesystem", sizeof(target.target_type), 0);
    safe_strcpy(target.storage_class, "hot", sizeof(target.storage_class), 0);
    target.enabled = true;
    target.high_watermark_pct = 90.0;
    target.low_watermark_pct = 80.0;
    int64_t unused_revision = 0;
    if (!apply_body(body, &target, true, &unused_revision, res)) {
        cJSON_Delete(body);
        audit_target(req, &user, NULL, "target_create", "failure",
                     "invalid_request");
        return;
    }
    cJSON_Delete(body);
    db_storage_target_result_t result = db_storage_target_create(&target);
    if (result != DB_STORAGE_TARGET_OK) {
        set_db_error(res, result, target.last_error);
        audit_target(req, &user, NULL, "target_create",
                     result == DB_STORAGE_TARGET_ERROR ? "error" : "failure",
                     "persistence_or_probe_failed");
        return;
    }
    audit_target(req, &user, target.uuid, "target_create", "success",
                 "created");
    set_json_response(res, 201, target_to_json(&target, false));
}

void handle_get_storage_target(const http_request_t *req,
                               http_response_t *res) {
    user_t user;
    if (!authorize_storage(req, res, &user)) return;
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    if (!extract_target_uuid(req, uuid, res)) return;
    storage_target_t target;
    db_storage_target_result_t result = db_storage_target_get(uuid, &target);
    if (result != DB_STORAGE_TARGET_OK) {
        set_db_error(res, result, NULL);
        return;
    }
    set_json_response(res, 200, target_to_json(&target, false));
}

void handle_put_storage_target(const http_request_t *req,
                               http_response_t *res) {
    user_t user;
    if (!authorize_storage(req, res, &user)) return;
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    if (!extract_target_uuid(req, uuid, res)) return;
    storage_target_t target;
    db_storage_target_result_t result = db_storage_target_get(uuid, &target);
    if (result != DB_STORAGE_TARGET_OK) {
        set_db_error(res, result, NULL);
        return;
    }
    cJSON *body = httpd_parse_json_body(req);
    int64_t revision = 0;
    if (!apply_body(body, &target, false, &revision, res)) {
        cJSON_Delete(body);
        audit_target(req, &user, uuid, "target_update", "failure",
                     "invalid_request");
        return;
    }
    cJSON_Delete(body);
    result = db_storage_target_update(&target, revision);
    if (result != DB_STORAGE_TARGET_OK) {
        set_db_error(res, result, target.last_error);
        audit_target(req, &user, uuid, "target_update",
                     result == DB_STORAGE_TARGET_ERROR ? "error" : "failure",
                     "update_failed");
        return;
    }
    audit_target(req, &user, uuid, "target_update", "success", "updated");
    set_json_response(res, 200, target_to_json(&target, false));
}

static bool parse_revision_query(const http_request_t *req, int64_t *revision,
                                 http_response_t *res) {
    char value[64];
    if (http_request_get_query_param(req, "revision", value,
                                     sizeof(value)) < 0) {
        http_response_set_json_error(res, 400,
                                     "revision query parameter is required");
        return false;
    }
    errno = 0;
    char *end = NULL;
    long long parsed = strtoll(value, &end, 10);
    if (errno == ERANGE || !end || *end != '\0' || parsed < 1) {
        http_response_set_json_error(res, 400,
                                     "revision must be a positive integer");
        return false;
    }
    *revision = parsed;
    return true;
}

void handle_delete_storage_target(const http_request_t *req,
                                  http_response_t *res) {
    user_t user;
    if (!authorize_storage(req, res, &user)) return;
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    int64_t revision = 0;
    if (!extract_target_uuid(req, uuid, res) ||
        !parse_revision_query(req, &revision, res)) return;
    db_storage_target_result_t result =
        db_storage_target_delete(uuid, revision);
    if (result != DB_STORAGE_TARGET_OK) {
        set_db_error(res, result, NULL);
        audit_target(req, &user, uuid, "target_delete",
                     result == DB_STORAGE_TARGET_ERROR ? "error" : "failure",
                     "delete_failed");
        return;
    }
    audit_target(req, &user, uuid, "target_delete", "success", "deleted");
    http_response_set_json(res, 200, "{\"success\":true}");
}

void handle_post_storage_target_probe(const http_request_t *req,
                                      http_response_t *res) {
    user_t user;
    if (!authorize_storage(req, res, &user)) return;
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    if (!extract_target_uuid(req, uuid, res)) return;
    storage_target_t target;
    memset(&target, 0, sizeof(target));
    db_storage_target_result_t result =
        db_storage_target_probe(uuid, true, &target);
    if (result != DB_STORAGE_TARGET_OK) {
        set_db_error(res, result,
                     target.last_error[0] ? target.last_error : NULL);
        audit_target(req, &user, uuid, "target_probe", "failure",
                     "probe_failed");
        return;
    }
    audit_target(req, &user, uuid, "target_probe", "success", "healthy");
    set_json_response(res, 200, target_to_json(&target, false));
}
