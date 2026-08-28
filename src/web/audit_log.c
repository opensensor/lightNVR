#define _POSIX_C_SOURCE 200809L

#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LOG_COMPONENT "Audit"
#include "core/logger.h"
#include "database/db_audit.h"
#include "database/db_fleet_query.h"
#include "utils/strings.h"
#include "web/audit_log.h"
#include "web/httpd_utils.h"

static bool contains_case_insensitive(const char *value,
                                      const char *needle) {
    if (!value || !needle || !needle[0]) return false;
    size_t needle_length = strlen(needle);
    for (const char *start = value; *start; start++) {
        size_t index = 0;
        while (index < needle_length && start[index] &&
               tolower((unsigned char)start[index]) ==
                   tolower((unsigned char)needle[index])) {
            index++;
        }
        if (index == needle_length) return true;
    }
    return false;
}

static bool has_case_insensitive_suffix(const char *value,
                                        const char *suffix) {
    if (!value || !suffix) return false;
    size_t value_length = strlen(value);
    size_t suffix_length = strlen(suffix);
    if (suffix_length > value_length) return false;
    return contains_case_insensitive(value + value_length - suffix_length,
                                     suffix);
}

static bool sensitive_detail_key(const char *key) {
    if (!key) return false;
    if (contains_case_insensitive(key, "password") ||
        contains_case_insensitive(key, "passwd") ||
        contains_case_insensitive(key, "passphrase") ||
        contains_case_insensitive(key, "secret") ||
        contains_case_insensitive(key, "credential") ||
        contains_case_insensitive(key, "authorization") ||
        contains_case_insensitive(key, "cookie") ||
        contains_case_insensitive(key, "api_key") ||
        contains_case_insensitive(key, "apikey") ||
        contains_case_insensitive(key, "plate")) {
        return true;
    }
    return contains_case_insensitive(key, "token") &&
        !has_case_insensitive_suffix(key, "_token_uuid") &&
        !has_case_insensitive_suffix(key, "_token_id");
}

static void replace_control_characters(char *value) {
    if (!value) return;
    for (unsigned char *cursor = (unsigned char *)value; *cursor; cursor++) {
        if (iscntrl(*cursor)) *cursor = '_';
    }
}

static cJSON *redacted_details_copy(const cJSON *source,
                                    const char *property_name) {
    if (property_name && sensitive_detail_key(property_name)) {
        return cJSON_CreateString("[REDACTED]");
    }
    if (cJSON_IsObject(source)) {
        cJSON *copy = cJSON_CreateObject();
        if (!copy) return NULL;
        for (const cJSON *child = source->child; child; child = child->next) {
            cJSON *safe_child = redacted_details_copy(child, child->string);
            if (!safe_child ||
                !cJSON_AddItemToObject(copy, child->string ? child->string : "",
                                       safe_child)) {
                cJSON_Delete(safe_child);
                cJSON_Delete(copy);
                return NULL;
            }
        }
        return copy;
    }
    if (cJSON_IsArray(source)) {
        cJSON *copy = cJSON_CreateArray();
        if (!copy) return NULL;
        for (const cJSON *child = source->child; child; child = child->next) {
            cJSON *safe_child = redacted_details_copy(child, NULL);
            if (!safe_child || !cJSON_AddItemToArray(copy, safe_child)) {
                cJSON_Delete(safe_child);
                cJSON_Delete(copy);
                return NULL;
            }
        }
        return copy;
    }
    return cJSON_Duplicate(source, true);
}

void audit_log_append(const http_request_t *req, const user_t *user,
                      const char *action, const char *target_type,
                      const char *target_uuid, const char *outcome,
                      const cJSON *details) {
    if (!req || !action || !outcome) return;
    cJSON *redacted = details ? redacted_details_copy(details, NULL) : NULL;
    char *serialized = redacted ? cJSON_PrintUnformatted(redacted) : NULL;
    cJSON_Delete(redacted);
    const char *details_json = serialized ? serialized : "{}";
    char remote_address[AUDIT_REMOTE_ADDRESS_MAX] = {0};
    if (httpd_get_effective_client_ip(req, remote_address,
                                      sizeof(remote_address)) != 0) {
        safe_strcpy(remote_address, req->client_ip,
                    sizeof(remote_address), 0);
    }
    audit_event_input_t input = {
        .request_id = req->request_id,
        .principal_user_id = user ? user->id : 0,
        .principal_username = user ? user->username : "",
        .auth_method = user && user->authentication_method[0]
            ? user->authentication_method : "unauthenticated",
        .api_token_uuid = user && user->authenticated_via_scoped_token
            ? user->api_token_uuid : NULL,
        .action = action,
        .target_type = target_type,
        .target_uuid = target_uuid,
        .outcome = outcome,
        .remote_address = remote_address,
        .details_json = details_json,
    };
    if (db_audit_append(&input, NULL) != 0) {
        log_error("Failed to persist audit event for action %s", action);
    }
    free(serialized);
}

void audit_log_operation(const http_request_t *req, const user_t *user,
                         const char *action, const char *target_type,
                         const char *target_uuid, const char *operation,
                         const char *outcome, const cJSON *context) {
    if (!req || !action || !operation || !outcome) return;
    cJSON *details = cJSON_CreateObject();
    if (details) {
        cJSON_AddStringToObject(details, "event_type", "operation.outcome");
        cJSON_AddStringToObject(details, "method", req->method_str);
        cJSON_AddStringToObject(details, "path", req->path);
        cJSON_AddStringToObject(details, "operation", operation);
        if (context) {
            cJSON *context_copy = cJSON_Duplicate(context, true);
            if (context_copy) {
                cJSON_AddItemToObject(details, "context", context_copy);
            }
        }
    }
    audit_log_append(req, user, action, target_type, target_uuid, outcome,
                     details);
    cJSON_Delete(details);
}

void audit_log_authorization(const http_request_t *req, const user_t *user,
                             authorization_action_t action,
                             const fleet_camera_t *camera,
                             const authorization_evaluation_t *evaluation,
                             const char *outcome) {
    const authorization_action_metadata_t *metadata =
        authorization_action_metadata(action);
    if (!metadata || !req || !outcome) return;
    cJSON *details = cJSON_CreateObject();
    if (details) {
        cJSON_AddStringToObject(details, "event_type",
                               "authorization.decision");
        cJSON_AddStringToObject(details, "method", req->method_str);
        cJSON_AddStringToObject(details, "path", req->path);
        if (evaluation) {
            cJSON_AddStringToObject(
                details, "decision_source",
                authorization_decision_source_name(evaluation->source));
            cJSON_AddNumberToObject(details, "policy_version",
                                    (double)evaluation->policy_version);
            cJSON_AddStringToObject(details, "explanation",
                                    evaluation->explanation);
            if (evaluation->grant_uuid[0]) {
                cJSON_AddStringToObject(details, "grant_uuid",
                                        evaluation->grant_uuid);
            }
            if (evaluation->role_uuid[0]) {
                cJSON_AddStringToObject(details, "role_uuid",
                                        evaluation->role_uuid);
            }
        }
    }
    audit_log_append(req, user, metadata->key,
                     camera ? "camera" : "system",
                     camera ? camera->camera_uuid : NULL,
                     outcome, details);
    cJSON_Delete(details);
}

void audit_log_login(const http_request_t *req, const user_t *user,
                     const char *attempted_username,
                     const char *authentication_method,
                     const char *outcome, const char *reason) {
    if (!req || !authentication_method || !outcome || !reason) return;
    user_t principal = {0};
    if (user) principal = *user;
    if (!principal.username[0] && attempted_username) {
        safe_strcpy(principal.username, attempted_username,
                    sizeof(principal.username), 0);
    }
    replace_control_characters(principal.username);
    safe_strcpy(principal.authentication_method, authentication_method,
                sizeof(principal.authentication_method), 0);
    cJSON *details = cJSON_CreateObject();
    if (details) {
        cJSON_AddStringToObject(details, "event_type",
                                "authentication.login");
        cJSON_AddStringToObject(details, "reason", reason);
    }
    char target_user_id[32] = {0};
    if (principal.id > 0) {
        snprintf(target_user_id, sizeof(target_user_id), "%lld",
                 (long long)principal.id);
    }
    audit_log_append(req, &principal, "auth.login", "user",
                     target_user_id[0] ? target_user_id : NULL,
                     outcome, details);
    cJSON_Delete(details);
}

static bool path_segment(const char *path, const char *prefix,
                         char *value, size_t value_size) {
    if (!path || !prefix || !value || value_size == 0 ||
        strncmp(path, prefix, strlen(prefix)) != 0) return false;
    const char *start = path + strlen(prefix);
    const char *end = strchr(start, '/');
    size_t length = end ? (size_t)(end - start) : strlen(start);
    if (length == 0 || length >= value_size) return false;
    char encoded[MAX_STREAM_NAME] = {0};
    if (length >= sizeof(encoded)) return false;
    memcpy(encoded, start, length);
    encoded[length] = '\0';
    return url_decode(encoded, value, value_size) == 0;
}

static bool json_string_field(const http_request_t *req, const char *key,
                              char *value, size_t value_size) {
    cJSON *body = httpd_parse_json_body(req);
    if (!body) return false;
    const cJSON *field = cJSON_GetObjectItemCaseSensitive(body, key);
    bool copied = cJSON_IsString(field) && field->valuestring &&
                  safe_strcpy(value, field->valuestring, value_size, 0) == 0;
    cJSON_Delete(body);
    return copied;
}

static const char *camera_configuration_operation(
    const http_request_t *req, char *target_type, size_t target_type_size,
    char *identity, size_t identity_size, bool *identity_is_camera_uuid,
    authorization_action_t *action) {
    const char *path = req->path;
    const char *method = req->method_str;
    *identity_is_camera_uuid = false;
    *action = AUTHZ_CAMERA_CONFIGURE;
    safe_strcpy(target_type, "camera", target_type_size, 0);

    if (strcmp(path, "/api/system/logs/clear") == 0 &&
        strcmp(method, "POST") == 0) {
        *action = AUTHZ_SYSTEM_ADMIN;
        safe_strcpy(target_type, "system_logs", target_type_size, 0);
        return "system.logs.clear";
    }
    if (strcmp(path, "/api/settings") == 0 && strcmp(method, "POST") == 0) {
        *action = AUTHZ_SYSTEM_ADMIN;
        safe_strcpy(target_type, "settings", target_type_size, 0);
        return "settings.update";
    }

    if (strcmp(path, "/api/streams/test") == 0 && strcmp(method, "POST") == 0)
        return "stream.test";
    if (strcmp(path, "/api/streams") == 0 && strcmp(method, "POST") == 0) {
        (void)json_string_field(req, "name", identity, identity_size);
        return "stream.create";
    }
    if (strcmp(path, "/api/motion/trigger") == 0 && strcmp(method, "POST") == 0) {
        (void)json_string_field(req, "stream", identity, identity_size);
        return "motion.trigger";
    }
    if (strncmp(path, "/api/onvif/", 11) == 0 && strcmp(method, "POST") == 0) {
        if (strstr(path, "/discovery/discover")) return "onvif.discovery";
        if (strstr(path, "/device/add")) return "onvif.add";
        if (strstr(path, "/device/test")) return "onvif.test";
    }
    if (strncmp(path, "/api/streams/", 13) == 0 &&
        path_segment(path, "/api/streams/", identity, identity_size)) {
        if (strstr(path, "/refresh") && strcmp(method, "POST") == 0)
            return "stream.refresh";
        if (strstr(path, "/recording") && strcmp(method, "POST") == 0)
            return "manual_recording.update";
        if (strstr(path, "/retention") && strcmp(method, "PUT") == 0)
            return "stream.retention.update";
        if (strstr(path, "/zones") && strcmp(method, "POST") == 0)
            return "zones.update";
        if (strstr(path, "/zones") && strcmp(method, "DELETE") == 0)
            return "zones.delete";
        if (strstr(path, "/imaging/settings") && strcmp(method, "PUT") == 0)
            return "imaging.update";
        if (strstr(path, "/daynight") && strcmp(method, "PUT") == 0)
            return "daynight.update";
        if (!strchr(path + 13, '/') && strcmp(method, "PUT") == 0)
            return "stream.update";
        if (!strchr(path + 13, '/') && strcmp(method, "DELETE") == 0)
            return "stream.delete";
    }
    if (strncmp(path, "/api/cameras/", 13) == 0 &&
        path_segment(path, "/api/cameras/", identity, identity_size)) {
        *identity_is_camera_uuid = true;
        if (strstr(path, "/location") && strcmp(method, "PUT") == 0)
            return "camera.location.assign";
        if (strstr(path, "/tags") && strcmp(method, "PUT") == 0)
            return "camera.tags.assign";
    }
    if (strncmp(path, "/api/locations", 14) == 0) {
        safe_strcpy(target_type, "location", target_type_size, 0);
        if (strcmp(path, "/api/locations") == 0 && strcmp(method, "POST") == 0)
            return "location.create";
        (void)path_segment(path, "/api/locations/", identity, identity_size);
        if (strcmp(method, "PUT") == 0) return "location.update";
        if (strcmp(method, "DELETE") == 0) return "location.delete";
    }
    if (strncmp(path, "/api/live/plans", 15) == 0) {
        safe_strcpy(target_type, "operator_floor_plan", target_type_size, 0);
        if (strcmp(path, "/api/live/plans") == 0 &&
            strcmp(method, "POST") == 0) return "operator_floor_plan.create";
        (void)path_segment(path, "/api/live/plans/", identity, identity_size);
        if (strcmp(method, "PUT") == 0) return "operator_floor_plan.update";
        if (strcmp(method, "DELETE") == 0) return "operator_floor_plan.delete";
    }
    if (strncmp(path, "/api/camera-tags", 16) == 0) {
        safe_strcpy(target_type, "camera_tag", target_type_size, 0);
        if (strcmp(path, "/api/camera-tags") == 0 && strcmp(method, "POST") == 0)
            return "camera_tag.create";
        (void)path_segment(path, "/api/camera-tags/", identity, identity_size);
        if (strstr(path, "/merge") && strcmp(method, "POST") == 0)
            return "camera_tag.merge";
        if (strcmp(method, "PUT") == 0) return "camera_tag.update";
        if (strcmp(method, "DELETE") == 0) return "camera_tag.delete";
    }
    if (strncmp(path, "/api/camera-collections", 23) == 0) {
        safe_strcpy(target_type, "camera_collection", target_type_size, 0);
        if (strcmp(path, "/api/camera-collections") == 0 &&
            strcmp(method, "POST") == 0) return "camera_collection.create";
        (void)path_segment(path, "/api/camera-collections/", identity,
                           identity_size);
        if (strstr(path, "/members") && strcmp(method, "PUT") == 0)
            return "camera_collection.members.update";
        if (strcmp(method, "PUT") == 0) return "camera_collection.update";
        if (strcmp(method, "DELETE") == 0) return "camera_collection.delete";
    }
    return NULL;
}

void audit_log_sensitive_operation_begin(
    const http_request_t *req, audit_sensitive_operation_context_t *context) {
    if (!context) return;
    memset(context, 0, sizeof(*context));
    if (!req) return;
    const char *operation = camera_configuration_operation(
        req, context->target_type, sizeof(context->target_type),
        context->identity, sizeof(context->identity),
        &context->identity_is_camera_uuid, &context->action);
    if (!operation) return;
    context->applicable = true;
    safe_strcpy(context->operation, operation, sizeof(context->operation), 0);
    context->has_user = httpd_check_action_access(req, &context->user) != 0;
    if (context->identity_is_camera_uuid) {
        safe_strcpy(context->target_uuid, context->identity,
                    sizeof(context->target_uuid), 0);
    } else if (strcmp(context->target_type, "camera") == 0 &&
               context->identity[0] != '\0') {
        fleet_camera_t camera;
        if (db_fleet_camera_find_by_name(context->identity, &camera) == 0) {
            safe_strcpy(context->target_uuid, camera.camera_uuid,
                        sizeof(context->target_uuid), 0);
        }
    } else if (context->identity[0] != '\0') {
        safe_strcpy(context->target_uuid, context->identity,
                    sizeof(context->target_uuid), 0);
    }
}

static void sensitive_target_from_response(
    const http_response_t *res, audit_sensitive_operation_context_t *context) {
    if (!res || !context || context->target_uuid[0] != '\0' ||
        res->status_code < 200 || res->status_code >= 400 || !res->body ||
        res->body_length == 0) return;
    cJSON *body = cJSON_ParseWithLength((const char *)res->body,
                                       res->body_length);
    if (!cJSON_IsObject(body)) {
        cJSON_Delete(body);
        return;
    }
    const cJSON *uuid = cJSON_GetObjectItemCaseSensitive(body, "camera_uuid");
    if (!cJSON_IsString(uuid)) {
        uuid = cJSON_GetObjectItemCaseSensitive(body, "uuid");
    }
    if (cJSON_IsString(uuid) && uuid->valuestring &&
        strlen(uuid->valuestring) == CAMERA_UUID_STRING_SIZE - 1) {
        safe_strcpy(context->target_uuid, uuid->valuestring,
                    sizeof(context->target_uuid), 0);
    }
    const cJSON *stream_name =
        cJSON_GetObjectItemCaseSensitive(body, "stream_name");
    if (context->target_uuid[0] == '\0' && cJSON_IsString(stream_name) &&
        stream_name->valuestring) {
        fleet_camera_t camera;
        if (db_fleet_camera_find_by_name(stream_name->valuestring, &camera) == 0) {
            safe_strcpy(context->target_uuid, camera.camera_uuid,
                        sizeof(context->target_uuid), 0);
        }
    }
    cJSON_Delete(body);
}

void audit_log_sensitive_operation_end(
    const http_request_t *req, const http_response_t *res,
    audit_sensitive_operation_context_t *context) {
    if (!req || !res || !context || !context->applicable ||
        !context->has_user || res->status_code <= 0 ||
        res->status_code == 401 || res->status_code == 403) return;
    sensitive_target_from_response(res, context);

    const char *outcome = res->status_code >= 200 && res->status_code < 400
        ? "success" : (res->status_code >= 500 ? "error" : "failure");
    cJSON *audit_details = cJSON_CreateObject();
    if (audit_details) {
        cJSON_AddNumberToObject(audit_details, "http_status", res->status_code);
    }
    const authorization_action_metadata_t *metadata =
        authorization_action_metadata(context->action);
    audit_log_operation(req, &context->user,
                        metadata ? metadata->key : "unknown",
                        context->target_type,
                        context->target_uuid[0] ? context->target_uuid : NULL,
                        context->operation, outcome, audit_details);
    cJSON_Delete(audit_details);
}

void audit_log_sensitive_operation_outcome(const http_request_t *req,
                                           const http_response_t *res) {
    audit_sensitive_operation_context_t context;
    audit_log_sensitive_operation_begin(req, &context);
    audit_log_sensitive_operation_end(req, res, &context);
}
