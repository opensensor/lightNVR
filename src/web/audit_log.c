#define _POSIX_C_SOURCE 200809L

#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LOG_COMPONENT "Audit"
#include "core/logger.h"
#include "database/db_audit.h"
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
        contains_case_insensitive(key, "apikey")) {
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
