#define _POSIX_C_SOURCE 200809L

#include "web/api_handlers_storage_policies.h"

#include <cjson/cJSON.h>
#include <errno.h>
#include <math.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>

#include "core/authorization.h"
#include "core/camera_selector.h"
#include "database/db_fleet_query.h"
#include "database/db_storage_policies.h"
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

static bool send_json(http_response_t *res, int status, cJSON *object) {
    char *body = object ? cJSON_PrintUnformatted(object) : NULL;
    cJSON_Delete(object);
    if (!body) {
        http_response_set_json_error(res, 500,
                                     "Failed to serialize storage policy response");
        return false;
    }
    http_response_set_json(res, status, body);
    free(body);
    return true;
}

static cJSON *policy_to_json(const storage_policy_t *policy) {
    cJSON *object = cJSON_CreateObject();
    cJSON *selector = cJSON_Parse(policy->selector_json);
    if (!object || !selector) {
        cJSON_Delete(object);
        cJSON_Delete(selector);
        return NULL;
    }
    cJSON_AddStringToObject(object, "uuid", policy->uuid);
    cJSON_AddStringToObject(object, "name", policy->name);
    cJSON_AddBoolToObject(object, "enabled", policy->enabled);
    cJSON_AddNumberToObject(object, "priority", policy->priority);
    cJSON_AddItemToObject(object, "selector", selector);
    cJSON_AddStringToObject(object, "primary_target_uuid",
                           policy->primary_target_uuid);
    cJSON_AddStringToObject(object, "fallback_mode", policy->fallback_mode);
    if (policy->fallback_target_uuid[0]) {
        cJSON_AddStringToObject(object, "fallback_target_uuid",
                               policy->fallback_target_uuid);
    } else {
        cJSON_AddNullToObject(object, "fallback_target_uuid");
    }
    cJSON_AddNumberToObject(object, "revision", (double)policy->revision);
    cJSON_AddNumberToObject(object, "created_at", (double)policy->created_at);
    cJSON_AddNumberToObject(object, "updated_at", (double)policy->updated_at);
    return object;
}

static void set_db_error(http_response_t *res,
                         db_storage_policy_result_t result,
                         const char *validation_error) {
    switch (result) {
        case DB_STORAGE_POLICY_NOT_FOUND:
            http_response_set_json_error(res, 404, "Storage policy not found");
            break;
        case DB_STORAGE_POLICY_CONFLICT:
            http_response_set_json_error(res, 409,
                                         "A storage policy already uses that name");
            break;
        case DB_STORAGE_POLICY_STALE:
            http_response_set_json_error(
                res, 409, "Storage policy was changed by another administrator");
            break;
        case DB_STORAGE_POLICY_LIMIT:
            http_response_set_json_error(res, 409,
                                         "Storage policy limit reached");
            break;
        case DB_STORAGE_POLICY_INVALID:
            http_response_set_json_error(
                res, 400, validation_error && validation_error[0]
                    ? validation_error : "Invalid storage policy");
            break;
        default:
            http_response_set_json_error(res, 500,
                                         "Storage policy operation failed");
            break;
    }
}

static bool extract_uuid(const http_request_t *req,
                         char uuid[LIGHTNVR_UUID_STRING_SIZE],
                         http_response_t *res) {
    char value[MAX_PATH_LENGTH];
    if (http_request_extract_path_param(
            req, "/api/storage-policies/", value, sizeof(value)) != 0) {
        http_response_set_json_error(res, 400, "Invalid storage policy path");
        return false;
    }
    char *slash = strchr(value, '/');
    if (slash) *slash = '\0';
    if (!lightnvr_uuid_is_valid(value)) {
        http_response_set_json_error(res, 400,
                                     "Invalid storage policy UUID");
        return false;
    }
    safe_strcpy(uuid, value, LIGHTNVR_UUID_STRING_SIZE, 0);
    return true;
}

static bool string_value(const cJSON *body, const char *key,
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

static bool apply_body(const cJSON *body, storage_policy_t *policy,
                       bool create, int64_t *revision,
                       char validation_error[STORAGE_TARGET_ERROR_MAX],
                       http_response_t *res) {
    if (!cJSON_IsObject(body)) {
        http_response_set_json_error(res, 400,
                                     "Request body must be a JSON object");
        return false;
    }
    if (!string_value(body, "name", policy->name, sizeof(policy->name),
                      create, res) ||
        !string_value(body, "primary_target_uuid",
                      policy->primary_target_uuid,
                      sizeof(policy->primary_target_uuid), create, res) ||
        !string_value(body, "fallback_mode", policy->fallback_mode,
                      sizeof(policy->fallback_mode), false, res)) return false;

    const cJSON *fallback = cJSON_GetObjectItemCaseSensitive(
        body, "fallback_target_uuid");
    if (fallback) {
        if (cJSON_IsNull(fallback)) {
            policy->fallback_target_uuid[0] = '\0';
        } else if (!cJSON_IsString(fallback) || !fallback->valuestring ||
                   strnlen(fallback->valuestring,
                           sizeof(policy->fallback_target_uuid)) >=
                       sizeof(policy->fallback_target_uuid)) {
            http_response_set_json_error(
                res, 400, "fallback_target_uuid must be a UUID or null");
            return false;
        } else {
            safe_strcpy(policy->fallback_target_uuid, fallback->valuestring,
                        sizeof(policy->fallback_target_uuid), 0);
        }
    }
    const cJSON *enabled = cJSON_GetObjectItemCaseSensitive(body, "enabled");
    if (enabled) {
        if (!cJSON_IsBool(enabled)) {
            http_response_set_json_error(res, 400, "enabled must be boolean");
            return false;
        }
        policy->enabled = cJSON_IsTrue(enabled);
    }
    const cJSON *priority = cJSON_GetObjectItemCaseSensitive(body, "priority");
    if (priority) {
        if (!cJSON_IsNumber(priority) || !isfinite(priority->valuedouble) ||
            floor(priority->valuedouble) != priority->valuedouble ||
            priority->valuedouble < -1000000 ||
            priority->valuedouble > 1000000) {
            http_response_set_json_error(res, 400,
                                         "priority must be an integer");
            return false;
        }
        policy->priority = priority->valueint;
    }
    const cJSON *selector = cJSON_GetObjectItemCaseSensitive(body, "selector");
    if (selector) {
        char *text = cJSON_PrintUnformatted(selector);
        if (!text || strnlen(text, sizeof(policy->selector_json)) >=
                         sizeof(policy->selector_json)) {
            free(text);
            http_response_set_json_error(res, 400,
                                         "selector is too large");
            return false;
        }
        safe_strcpy(policy->selector_json, text,
                    sizeof(policy->selector_json), 0);
        free(text);
    } else if (create) {
        http_response_set_json_error(res, 400, "selector is required");
        return false;
    }
    if (!create) {
        const cJSON *revision_item = cJSON_GetObjectItemCaseSensitive(
            body, "revision");
        if (!cJSON_IsNumber(revision_item) ||
            revision_item->valuedouble < 1 ||
            floor(revision_item->valuedouble) !=
                revision_item->valuedouble) {
            http_response_set_json_error(
                res, 400, "revision must be a positive integer");
            return false;
        }
        *revision = (int64_t)revision_item->valuedouble;
    }
    db_storage_policy_result_t result = db_storage_policy_validate(
        policy, validation_error, STORAGE_TARGET_ERROR_MAX);
    if (result != DB_STORAGE_POLICY_OK) {
        set_db_error(res, result, validation_error);
        return false;
    }
    return true;
}

static void audit_policy(const http_request_t *req, const user_t *user,
                         const char *uuid, const char *operation,
                         const char *outcome) {
    audit_log_operation(req, user, "storage.configure", "storage_policy",
                        uuid, operation, outcome, NULL);
}

static bool policy_precedes(const storage_policy_t *left,
                            const storage_policy_t *right) {
    if (!right) return true;
    if (left->priority != right->priority) {
        return left->priority > right->priority;
    }
    int name_order = strcasecmp(left->name, right->name);
    if (name_order != 0) return name_order < 0;
    return strcmp(left->uuid, right->uuid) < 0;
}

static cJSON *preview_winner_json(const storage_policy_t *policy,
                                  const storage_policy_t *draft) {
    if (!policy) return cJSON_CreateNull();
    cJSON *winner = cJSON_CreateObject();
    if (!winner) return NULL;
    bool is_draft = policy == draft;
    cJSON_AddBoolToObject(winner, "draft", is_draft);
    if (policy->uuid[0]) {
        cJSON_AddStringToObject(winner, "policy_uuid", policy->uuid);
    } else {
        cJSON_AddNullToObject(winner, "policy_uuid");
    }
    cJSON_AddStringToObject(winner, "policy_name", policy->name);
    cJSON_AddNumberToObject(winner, "priority", policy->priority);
    cJSON_AddStringToObject(winner, "primary_target_uuid",
                            policy->primary_target_uuid);
    cJSON_AddStringToObject(winner, "fallback_mode",
                            policy->fallback_mode);
    if (policy->fallback_target_uuid[0]) {
        cJSON_AddStringToObject(winner, "fallback_target_uuid",
                                policy->fallback_target_uuid);
    } else {
        cJSON_AddNullToObject(winner, "fallback_target_uuid");
    }
    return winner;
}

void handle_post_storage_policy_preview(const http_request_t *req,
                                        http_response_t *res) {
    user_t user;
    if (!authorize_storage(req, res, &user)) return;
    cJSON *body = httpd_parse_json_body(req);
    if (!cJSON_IsObject(body)) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400,
                                     "Request body must be a JSON object");
        return;
    }

    storage_policy_t draft;
    memset(&draft, 0, sizeof(draft));
    draft.enabled = true;
    draft.priority = 100;
    safe_strcpy(draft.fallback_mode, "default",
                sizeof(draft.fallback_mode), 0);
    bool editing = false;
    const cJSON *uuid_item = cJSON_GetObjectItemCaseSensitive(body, "uuid");
    if (uuid_item) {
        if (!cJSON_IsString(uuid_item) || !uuid_item->valuestring ||
            !lightnvr_uuid_is_valid(uuid_item->valuestring)) {
            cJSON_Delete(body);
            http_response_set_json_error(res, 400,
                                         "uuid must be a valid policy UUID");
            return;
        }
        db_storage_policy_result_t loaded = db_storage_policy_get(
            uuid_item->valuestring, &draft);
        if (loaded != DB_STORAGE_POLICY_OK) {
            cJSON_Delete(body);
            set_db_error(res, loaded, NULL);
            return;
        }
        editing = true;
    }

    int64_t revision = 0;
    char validation_error[STORAGE_TARGET_ERROR_MAX] = {0};
    if (!apply_body(body, &draft, !editing, &revision, validation_error, res)) {
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);

    cJSON *draft_selector_json = cJSON_Parse(draft.selector_json);
    char selector_error[FLEET_SELECTOR_ERROR_MAX] = {0};
    fleet_selector_t *draft_selector = fleet_selector_parse(
        draft_selector_json, selector_error, sizeof(selector_error));
    cJSON_Delete(draft_selector_json);
    if (!draft_selector) {
        http_response_set_json_error(res, 400,
            selector_error[0] ? selector_error : "Invalid selector");
        return;
    }

    int total = db_storage_policy_count();
    storage_policy_t *policies = total > 0 && total <= STORAGE_POLICY_MAX_COUNT
        ? calloc((size_t)total, sizeof(*policies)) : NULL;
    fleet_selector_t **selectors = total > 0 && total <= STORAGE_POLICY_MAX_COUNT
        ? calloc((size_t)total, sizeof(*selectors)) : NULL;
    int *overlaps = total > 0 && total <= STORAGE_POLICY_MAX_COUNT
        ? calloc((size_t)total, sizeof(*overlaps)) : NULL;
    if (total < 0 || total > STORAGE_POLICY_MAX_COUNT ||
        (total > 0 && (!policies || !selectors || !overlaps))) {
        fleet_selector_free(draft_selector);
        free(policies);
        free(selectors);
        free(overlaps);
        http_response_set_json_error(res, 500,
                                     "Failed to load storage policies");
        return;
    }
    int policy_count = total > 0
        ? db_storage_policy_list(policies, total, true) : 0;
    if (policy_count < 0) {
        fleet_selector_free(draft_selector);
        free(policies);
        free(selectors);
        free(overlaps);
        http_response_set_json_error(res, 500,
                                     "Failed to load storage policies");
        return;
    }
    for (int index = 0; index < policy_count; index++) {
        if (editing && strcmp(policies[index].uuid, draft.uuid) == 0) continue;
        cJSON *json = cJSON_Parse(policies[index].selector_json);
        selectors[index] = json
            ? fleet_selector_parse(json, NULL, 0) : NULL;
        cJSON_Delete(json);
        if (!selectors[index]) {
            for (int cleanup = 0; cleanup < policy_count; cleanup++) {
                fleet_selector_free(selectors[cleanup]);
            }
            fleet_selector_free(draft_selector);
            free(policies);
            free(selectors);
            free(overlaps);
            http_response_set_json_error(
                res, 500, "Failed to compile existing storage policies");
            return;
        }
    }

    fleet_camera_t *cameras = NULL;
    int camera_count = 0;
    if (db_fleet_camera_load(&cameras, &camera_count) != 0) {
        for (int index = 0; index < policy_count; index++) {
            fleet_selector_free(selectors[index]);
        }
        fleet_selector_free(draft_selector);
        free(policies);
        free(selectors);
        free(overlaps);
        http_response_set_json_error(res, 500,
                                     "Failed to load camera inventory");
        return;
    }
    fleet_camera_enrich_runtime_health(cameras, camera_count);

    cJSON *root = cJSON_CreateObject();
    cJSON *samples = cJSON_CreateArray();
    cJSON *conflicts = cJSON_CreateArray();
    if (!root || !samples || !conflicts) {
        cJSON_Delete(root);
        cJSON_Delete(samples);
        cJSON_Delete(conflicts);
        root = NULL;
    } else {
        cJSON_AddItemToObject(root, "sample", samples);
        cJSON_AddItemToObject(root, "conflicts", conflicts);
    }

    int matched = 0;
    int effective = 0;
    int conflict_cameras = 0;
    for (int camera_index = 0; root && camera_index < camera_count;
         camera_index++) {
        if (!fleet_selector_matches(draft_selector, &cameras[camera_index],
                                    NULL)) continue;
        matched++;
        const storage_policy_t *winner = draft.enabled ? &draft : NULL;
        int camera_overlaps = 0;
        for (int policy_index = 0; policy_index < policy_count;
             policy_index++) {
            if (!selectors[policy_index] ||
                !fleet_selector_matches(selectors[policy_index],
                                        &cameras[camera_index], NULL)) {
                continue;
            }
            if (draft.enabled) {
                overlaps[policy_index]++;
                camera_overlaps++;
            }
            if (policy_precedes(&policies[policy_index], winner)) {
                winner = &policies[policy_index];
            }
        }
        if (camera_overlaps > 0) conflict_cameras++;
        if (winner == &draft) effective++;
        if (cJSON_GetArraySize(samples) < 50) {
            cJSON *sample = cJSON_CreateObject();
            cJSON *winner_json = preview_winner_json(winner, &draft);
            if (!sample || !winner_json) {
                cJSON_Delete(sample);
                cJSON_Delete(winner_json);
                cJSON_Delete(root);
                root = NULL;
                break;
            }
            cJSON_AddStringToObject(sample, "camera_uuid",
                                    cameras[camera_index].camera_uuid);
            cJSON_AddStringToObject(sample, "camera_name",
                                    cameras[camera_index].name);
            cJSON_AddNumberToObject(sample, "overlapping_policy_count",
                                    camera_overlaps);
            cJSON_AddItemToObject(sample, "effective_policy", winner_json);
            cJSON_AddItemToArray(samples, sample);
        }
    }

    int conflict_policy_count = 0;
    for (int index = 0; root && index < policy_count; index++) {
        if (overlaps[index] <= 0) continue;
        cJSON *conflict = cJSON_CreateObject();
        if (!conflict) {
            cJSON_Delete(root);
            root = NULL;
            break;
        }
        cJSON_AddStringToObject(conflict, "policy_uuid",
                                policies[index].uuid);
        cJSON_AddStringToObject(conflict, "policy_name",
                                policies[index].name);
        cJSON_AddNumberToObject(conflict, "priority",
                                policies[index].priority);
        cJSON_AddNumberToObject(conflict, "overlap_camera_count",
                                overlaps[index]);
        cJSON_AddBoolToObject(conflict, "draft_precedes",
                              policy_precedes(&draft, &policies[index]));
        cJSON_AddItemToArray(conflicts, conflict);
        conflict_policy_count++;
    }
    if (root) {
        cJSON_AddBoolToObject(root, "draft_enabled", draft.enabled);
        cJSON_AddNumberToObject(root, "matched_camera_count", matched);
        cJSON_AddNumberToObject(root, "effective_camera_count", effective);
        cJSON_AddNumberToObject(
            root, "shadowed_camera_count",
            draft.enabled ? matched - effective : 0);
        cJSON_AddNumberToObject(root, "conflict_camera_count",
                                conflict_cameras);
        cJSON_AddNumberToObject(root, "conflict_policy_count",
                                conflict_policy_count);
        cJSON_AddBoolToObject(root, "sample_truncated", matched > 50);
    }

    free(cameras);
    for (int index = 0; index < policy_count; index++) {
        fleet_selector_free(selectors[index]);
    }
    fleet_selector_free(draft_selector);
    free(policies);
    free(selectors);
    free(overlaps);
    if (!root) {
        http_response_set_json_error(res, 500,
                                     "Failed to create policy preview");
        return;
    }
    send_json(res, 200, root);
}

void handle_get_storage_policies(const http_request_t *req,
                                 http_response_t *res) {
    user_t user;
    if (!authorize_storage(req, res, &user)) return;
    int total = db_storage_policy_count();
    if (total < 0 || total > STORAGE_POLICY_MAX_COUNT) {
        set_db_error(res, DB_STORAGE_POLICY_ERROR, NULL);
        return;
    }
    storage_policy_t *policies = total
        ? calloc((size_t)total, sizeof(*policies)) : NULL;
    if (total && !policies) {
        http_response_set_json_error(res, 500, "Out of memory");
        return;
    }
    int count = total ? db_storage_policy_list(policies, total, false) : 0;
    cJSON *root = cJSON_CreateObject();
    cJSON *items = cJSON_CreateArray();
    if (count < 0 || !root || !items) {
        free(policies);
        cJSON_Delete(root);
        cJSON_Delete(items);
        set_db_error(res, DB_STORAGE_POLICY_ERROR, NULL);
        return;
    }
    cJSON_AddNumberToObject(root, "count", count);
    cJSON_AddItemToObject(root, "policies", items);
    for (int index = 0; index < count; index++) {
        cJSON *item = policy_to_json(&policies[index]);
        if (!item) {
            free(policies);
            cJSON_Delete(root);
            set_db_error(res, DB_STORAGE_POLICY_ERROR, NULL);
            return;
        }
        cJSON_AddItemToArray(items, item);
    }
    free(policies);
    send_json(res, 200, root);
}

void handle_post_storage_policy(const http_request_t *req,
                                http_response_t *res) {
    user_t user;
    if (!authorize_storage(req, res, &user)) return;
    storage_policy_t policy;
    memset(&policy, 0, sizeof(policy));
    policy.enabled = true;
    policy.priority = 100;
    safe_strcpy(policy.fallback_mode, "default",
                sizeof(policy.fallback_mode), 0);
    cJSON *body = httpd_parse_json_body(req);
    int64_t revision = 0;
    char error[STORAGE_TARGET_ERROR_MAX] = {0};
    if (!apply_body(body, &policy, true, &revision, error, res)) {
        cJSON_Delete(body);
        audit_policy(req, &user, NULL, "policy_create", "failure");
        return;
    }
    cJSON_Delete(body);
    db_storage_policy_result_t result = db_storage_policy_create(&policy);
    if (result != DB_STORAGE_POLICY_OK) {
        set_db_error(res, result, error);
        audit_policy(req, &user, NULL, "policy_create", "failure");
        return;
    }
    audit_policy(req, &user, policy.uuid, "policy_create", "success");
    send_json(res, 201, policy_to_json(&policy));
}

void handle_get_storage_policy(const http_request_t *req,
                               http_response_t *res) {
    user_t user;
    if (!authorize_storage(req, res, &user)) return;
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    if (!extract_uuid(req, uuid, res)) return;
    storage_policy_t policy;
    db_storage_policy_result_t result = db_storage_policy_get(uuid, &policy);
    if (result != DB_STORAGE_POLICY_OK) {
        set_db_error(res, result, NULL);
        return;
    }
    send_json(res, 200, policy_to_json(&policy));
}

void handle_put_storage_policy(const http_request_t *req,
                               http_response_t *res) {
    user_t user;
    if (!authorize_storage(req, res, &user)) return;
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    if (!extract_uuid(req, uuid, res)) return;
    storage_policy_t policy;
    db_storage_policy_result_t result = db_storage_policy_get(uuid, &policy);
    if (result != DB_STORAGE_POLICY_OK) {
        set_db_error(res, result, NULL);
        return;
    }
    cJSON *body = httpd_parse_json_body(req);
    int64_t revision = 0;
    char error[STORAGE_TARGET_ERROR_MAX] = {0};
    if (!apply_body(body, &policy, false, &revision, error, res)) {
        cJSON_Delete(body);
        audit_policy(req, &user, uuid, "policy_update", "failure");
        return;
    }
    cJSON_Delete(body);
    result = db_storage_policy_update(&policy, revision);
    if (result != DB_STORAGE_POLICY_OK) {
        set_db_error(res, result, error);
        audit_policy(req, &user, uuid, "policy_update", "failure");
        return;
    }
    audit_policy(req, &user, uuid, "policy_update", "success");
    send_json(res, 200, policy_to_json(&policy));
}

static bool revision_query(const http_request_t *req, int64_t *revision,
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

void handle_delete_storage_policy(const http_request_t *req,
                                  http_response_t *res) {
    user_t user;
    if (!authorize_storage(req, res, &user)) return;
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    int64_t revision = 0;
    if (!extract_uuid(req, uuid, res) ||
        !revision_query(req, &revision, res)) return;
    db_storage_policy_result_t result = db_storage_policy_delete(
        uuid, revision);
    if (result != DB_STORAGE_POLICY_OK) {
        set_db_error(res, result, NULL);
        audit_policy(req, &user, uuid, "policy_delete", "failure");
        return;
    }
    audit_policy(req, &user, uuid, "policy_delete", "success");
    http_response_set_json(res, 200, "{\"success\":true}");
}
