#define _POSIX_C_SOURCE 200809L

#include <cjson/cJSON.h>
#include <ctype.h>
#include <errno.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>

#include "core/authorization.h"
#include "database/db_auth.h"
#include "database/db_authorization.h"
#include "database/db_fleet_query.h"
#include "utils/strings.h"
#include "web/api_handlers_authorization.h"
#include "web/httpd_utils.h"

static cJSON *action_to_json(const authorization_action_metadata_t *metadata) {
    cJSON *item = cJSON_CreateObject();
    if (!item) return NULL;
    cJSON_AddStringToObject(item, "key", metadata->key);
    cJSON_AddStringToObject(item, "category", metadata->category);
    cJSON_AddStringToObject(item, "description", metadata->description);
    cJSON_AddBoolToObject(item, "camera_scoped", metadata->camera_scoped);
    cJSON_AddBoolToObject(item, "destructive", metadata->destructive);
    return item;
}

static void set_json_response(http_response_t *res, cJSON *json) {
    char *body = cJSON_PrintUnformatted(json);
    cJSON_Delete(json);
    if (!body) {
        http_response_set_json_error(res, 500, "Failed to serialize response");
        return;
    }
    http_response_set_json(res, 200, body);
    free(body);
}

static void set_json_response_status(http_response_t *res, int status,
                                     cJSON *json) {
    char *body = cJSON_PrintUnformatted(json);
    cJSON_Delete(json);
    if (!body) {
        http_response_set_json_error(res, 500, "Failed to serialize response");
        return;
    }
    http_response_set_json(res, status, body);
    free(body);
}

void handle_get_authorization_actions(const http_request_t *req,
                                      http_response_t *res) {
    user_t requester;
    authorization_evaluation_t requester_evaluation;
    if (!httpd_authorize_action(req, res, AUTHZ_USERS_MANAGE, NULL,
                                &requester, &requester_evaluation)) {
        return;
    }

    cJSON *response = cJSON_CreateObject();
    cJSON *actions = cJSON_CreateArray();
    if (!response || !actions) {
        cJSON_Delete(response);
        cJSON_Delete(actions);
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    int count = 0;
    const authorization_action_metadata_t *catalog =
        authorization_action_catalog(&count);
    for (int i = 0; i < count; i++) {
        cJSON *item = action_to_json(&catalog[i]);
        if (!item) {
            cJSON_Delete(actions);
            cJSON_Delete(response);
            http_response_set_json_error(res, 500,
                                         "Failed to create response");
            return;
        }
        cJSON_AddItemToArray(actions, item);
    }
    cJSON_AddItemToObject(response, "actions", actions);
    cJSON_AddNumberToObject(response, "count", count);
    set_json_response(res, response);
}

static fleet_camera_t *find_camera(fleet_camera_t *cameras, int count,
                                   const char *camera_uuid) {
    for (int i = 0; i < count; i++) {
        if (strcasecmp(cameras[i].camera_uuid, camera_uuid) == 0) {
            return &cameras[i];
        }
    }
    return NULL;
}

void handle_post_authorization_simulate(const http_request_t *req,
                                        http_response_t *res) {
    user_t requester;
    authorization_evaluation_t requester_evaluation;
    if (!httpd_authorize_action(req, res, AUTHZ_USERS_MANAGE, NULL,
                                &requester, &requester_evaluation)) {
        return;
    }

    cJSON *body = httpd_parse_json_body(req);
    if (!cJSON_IsObject(body)) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400,
                                     "Request body must be a JSON object");
        return;
    }
    const cJSON *user_id = cJSON_GetObjectItemCaseSensitive(body, "user_id");
    const cJSON *action_key =
        cJSON_GetObjectItemCaseSensitive(body, "action");
    const cJSON *camera_uuid =
        cJSON_GetObjectItemCaseSensitive(body, "camera_uuid");
    if (!cJSON_IsNumber(user_id) || user_id->valuedouble != user_id->valueint ||
        user_id->valueint <= 0 || !cJSON_IsString(action_key) ||
        !action_key->valuestring) {
        cJSON_Delete(body);
        http_response_set_json_error(
            res, 400, "user_id and a valid action are required");
        return;
    }

    authorization_action_t action =
        authorization_action_from_key(action_key->valuestring);
    const authorization_action_metadata_t *metadata =
        authorization_action_metadata(action);
    if (!metadata) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400, "Unknown authorization action");
        return;
    }
    if (metadata->camera_scoped &&
        (!cJSON_IsString(camera_uuid) || !camera_uuid->valuestring ||
         camera_uuid->valuestring[0] == '\0')) {
        cJSON_Delete(body);
        http_response_set_json_error(
            res, 400, "camera_uuid is required for this action");
        return;
    }
    if (!metadata->camera_scoped && camera_uuid) {
        cJSON_Delete(body);
        http_response_set_json_error(
            res, 400, "camera_uuid is not valid for this global action");
        return;
    }

    user_t user;
    if (db_auth_get_user_by_id((int64_t)user_id->valuedouble, &user) != 0) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 404, "User not found");
        return;
    }

    fleet_camera_t *cameras = NULL;
    fleet_camera_t *camera = NULL;
    int camera_count = 0;
    if (metadata->camera_scoped) {
        if (db_fleet_camera_load(&cameras, &camera_count) != 0) {
            cJSON_Delete(body);
            http_response_set_json_error(res, 500,
                                         "Failed to load camera inventory");
            return;
        }
        camera = find_camera(cameras, camera_count, camera_uuid->valuestring);
        if (!camera) {
            free(cameras);
            cJSON_Delete(body);
            http_response_set_json_error(res, 404, "Camera not found");
            return;
        }
        fleet_camera_enrich_runtime_health(camera, 1);
    }

    authorization_evaluation_t evaluation;
    int result = authorization_evaluate(&user, action, camera, &evaluation);
    cJSON_Delete(body);
    if (result != 0) {
        free(cameras);
        http_response_set_json_error(res, 500,
                                     "Authorization policy evaluation failed");
        return;
    }

    cJSON *response = cJSON_CreateObject();
    cJSON *principal = cJSON_CreateObject();
    cJSON *resource = metadata->camera_scoped ? cJSON_CreateObject() : NULL;
    if (!response || !principal || (metadata->camera_scoped && !resource)) {
        cJSON_Delete(response);
        cJSON_Delete(principal);
        cJSON_Delete(resource);
        free(cameras);
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    cJSON_AddBoolToObject(response, "allowed",
                          evaluation.decision == AUTHZ_DECISION_ALLOW);
    cJSON_AddStringToObject(response, "action", metadata->key);
    cJSON_AddStringToObject(response, "source",
                            authorization_decision_source_name(
                                evaluation.source));
    cJSON_AddNumberToObject(response, "policy_version",
                            (double)evaluation.policy_version);
    cJSON_AddStringToObject(response, "explanation",
                            evaluation.explanation);
    if (evaluation.grant_uuid[0]) {
        cJSON_AddStringToObject(response, "grant_uuid",
                                evaluation.grant_uuid);
    } else {
        cJSON_AddNullToObject(response, "grant_uuid");
    }
    if (evaluation.role_uuid[0]) {
        cJSON_AddStringToObject(response, "role_uuid",
                                evaluation.role_uuid);
    } else {
        cJSON_AddNullToObject(response, "role_uuid");
    }
    cJSON_AddStringToObject(response, "role", evaluation.role_name);
    cJSON_AddNumberToObject(principal, "id", (double)user.id);
    cJSON_AddStringToObject(principal, "username", user.username);
    cJSON_AddStringToObject(principal, "authorization_mode",
                            user.authorization_mode);
    cJSON_AddItemToObject(response, "principal", principal);
    if (resource) {
        cJSON_AddStringToObject(resource, "camera_uuid", camera->camera_uuid);
        cJSON_AddStringToObject(resource, "name", camera->name);
        cJSON_AddItemToObject(response, "resource", resource);
    } else {
        cJSON_AddNullToObject(response, "resource");
    }
    free(cameras);
    set_json_response(res, response);
}

static bool authorize_policy_manager(const http_request_t *req,
                                     http_response_t *res, user_t *requester) {
    authorization_evaluation_t evaluation;
    return httpd_authorize_action(req, res, AUTHZ_USERS_MANAGE, NULL,
                                  requester, &evaluation) != 0;
}

static bool valid_uuid(const char *value) {
    if (!value || strlen(value) != CAMERA_UUID_STRING_SIZE - 1) return false;
    for (int i = 0; i < CAMERA_UUID_STRING_SIZE - 1; i++) {
        unsigned char c = (unsigned char)value[i];
        if (i == 8 || i == 13 || i == 18 || i == 23) {
            if (c != '-') return false;
        } else if (!isxdigit(c)) {
            return false;
        }
    }
    return true;
}

static bool extract_role_uuid(const http_request_t *req,
                              char uuid[CAMERA_UUID_STRING_SIZE],
                              http_response_t *res) {
    char value[MAX_PATH_LENGTH];
    if (http_request_extract_path_param(req, "/api/authorization/roles/",
                                        value, sizeof(value)) != 0 ||
        strchr(value, '/') || !valid_uuid(value)) {
        http_response_set_json_error(res, 400, "Invalid role UUID");
        return false;
    }
    safe_strcpy(uuid, value, CAMERA_UUID_STRING_SIZE, 0);
    return true;
}

static bool extract_user_id(const http_request_t *req, int64_t *user_id,
                            http_response_t *res) {
    char value[MAX_PATH_LENGTH];
    if (http_request_extract_path_param(req, "/api/authorization/users/",
                                        value, sizeof(value)) != 0 ||
        value[0] == '\0' || strchr(value, '/')) {
        http_response_set_json_error(res, 400, "Invalid user ID");
        return false;
    }
    char *end = NULL;
    errno = 0;
    long long parsed = strtoll(value, &end, 10);
    if (errno == ERANGE || !end || *end != '\0' || parsed <= 0) {
        http_response_set_json_error(res, 400, "Invalid user ID");
        return false;
    }
    *user_id = (int64_t)parsed;
    return true;
}

static bool parse_expected_version(const cJSON *body, int64_t *version,
                                   http_response_t *res) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(
        body, "expected_policy_version");
    double numeric = cJSON_IsNumber(item) ? item->valuedouble : 0;
    int64_t parsed = numeric >= 1 && numeric <= 9007199254740991.0
        ? (int64_t)numeric : 0;
    if (!cJSON_IsNumber(item) || parsed < 1 || (double)parsed != numeric) {
        http_response_set_json_error(
            res, 400, "expected_policy_version must be a positive integer");
        return false;
    }
    *version = parsed;
    return true;
}

static cJSON *role_to_json(const authorization_role_t *role) {
    cJSON *object = cJSON_CreateObject();
    cJSON *actions = cJSON_CreateArray();
    if (!object || !actions) {
        cJSON_Delete(object);
        cJSON_Delete(actions);
        return NULL;
    }
    cJSON_AddStringToObject(object, "uuid", role->uuid);
    cJSON_AddStringToObject(object, "name", role->name);
    cJSON_AddStringToObject(object, "description", role->description);
    cJSON_AddBoolToObject(object, "builtin", role->is_builtin);
    int count = 0;
    const authorization_action_metadata_t *catalog =
        authorization_action_catalog(&count);
    for (int i = 0; i < count; i++) {
        if ((role->action_mask & (UINT64_C(1) << catalog[i].action)) != 0) {
            cJSON_AddItemToArray(actions, cJSON_CreateString(catalog[i].key));
        }
    }
    cJSON_AddItemToObject(object, "actions", actions);
    cJSON_AddNumberToObject(object, "created_at", (double)role->created_at);
    cJSON_AddNumberToObject(object, "updated_at", (double)role->updated_at);
    return object;
}

static bool parse_action_mask(const cJSON *body, uint64_t *action_mask,
                              http_response_t *res) {
    const cJSON *actions = cJSON_GetObjectItemCaseSensitive(body, "actions");
    int count = cJSON_IsArray(actions) ? cJSON_GetArraySize(actions) : -1;
    if (count < 1 || count > AUTHZ_ACTION_COUNT) {
        http_response_set_json_error(
            res, 400, "actions must be a non-empty action-key array");
        return false;
    }
    uint64_t mask = 0;
    for (int i = 0; i < count; i++) {
        const cJSON *item = cJSON_GetArrayItem(actions, i);
        authorization_action_t action = cJSON_IsString(item)
            ? authorization_action_from_key(item->valuestring)
            : AUTHZ_ACTION_INVALID;
        if (action == AUTHZ_ACTION_INVALID ||
            (mask & (UINT64_C(1) << action)) != 0) {
            http_response_set_json_error(
                res, 400, "actions contains an unknown or duplicate key");
            return false;
        }
        mask |= UINT64_C(1) << action;
    }
    *action_mask = mask;
    return true;
}

static bool contains_non_space(const char *value) {
    if (!value) return false;
    for (const unsigned char *cursor = (const unsigned char *)value;
         *cursor; cursor++) {
        if (!isspace(*cursor)) return true;
    }
    return false;
}

static bool parse_role_body(const cJSON *body, authorization_role_t *role,
                            int64_t *expected_version,
                            http_response_t *res) {
    if (!cJSON_IsObject(body) ||
        !parse_expected_version(body, expected_version, res)) {
        if (!cJSON_IsObject(body)) {
            http_response_set_json_error(res, 400,
                                         "Request body must be an object");
        }
        return false;
    }
    const cJSON *name = cJSON_GetObjectItemCaseSensitive(body, "name");
    const cJSON *description =
        cJSON_GetObjectItemCaseSensitive(body, "description");
    if (!cJSON_IsString(name) || !name->valuestring ||
        !contains_non_space(name->valuestring) ||
        strlen(name->valuestring) >= sizeof(role->name)) {
        http_response_set_json_error(res, 400, "Invalid role name");
        return false;
    }
    if (description &&
        (!cJSON_IsString(description) || !description->valuestring ||
         strlen(description->valuestring) >= sizeof(role->description))) {
        http_response_set_json_error(res, 400, "Invalid role description");
        return false;
    }
    safe_strcpy(role->name, name->valuestring, sizeof(role->name), 0);
    safe_strcpy(role->description,
                description ? description->valuestring : "",
                sizeof(role->description), 0);
    return parse_action_mask(body, &role->action_mask, res);
}

static void set_authorization_db_error(http_response_t *res,
                                       db_authorization_result_t result) {
    switch (result) {
        case DB_AUTHORIZATION_NOT_FOUND:
            http_response_set_json_error(res, 404, "Policy resource not found");
            break;
        case DB_AUTHORIZATION_CONFLICT:
            http_response_set_json_error(res, 409,
                                         "A role with that name already exists");
            break;
        case DB_AUTHORIZATION_IMMUTABLE:
            http_response_set_json_error(res, 409,
                                         "Built-in roles are immutable");
            break;
        case DB_AUTHORIZATION_IN_USE:
            http_response_set_json_error(res, 409,
                                         "Role is still referenced by a grant");
            break;
        case DB_AUTHORIZATION_STALE:
            http_response_set_json_error(
                res, 409, "Policy changed; reload before saving again");
            break;
        case DB_AUTHORIZATION_INVALID:
            http_response_set_json_error(res, 400,
                                         "Invalid authorization policy request");
            break;
        default:
            http_response_set_json_error(res, 500,
                                         "Authorization policy operation failed");
            break;
    }
}

void handle_get_authorization_roles(const http_request_t *req,
                                    http_response_t *res) {
    user_t requester;
    if (!authorize_policy_manager(req, res, &requester)) return;
    authorization_role_t *roles = NULL;
    int count = 0;
    int64_t policy_version = 0;
    if (db_authorization_load_roles(&roles, &count, &policy_version) !=
        DB_AUTHORIZATION_OK) {
        http_response_set_json_error(res, 500, "Failed to load roles");
        return;
    }
    cJSON *root = cJSON_CreateObject();
    cJSON *items = cJSON_CreateArray();
    if (!root || !items) {
        cJSON_Delete(root);
        cJSON_Delete(items);
        free(roles);
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    cJSON_AddNumberToObject(root, "policy_version", (double)policy_version);
    cJSON_AddNumberToObject(root, "count", count);
    cJSON_AddItemToObject(root, "roles", items);
    for (int i = 0; i < count; i++) {
        cJSON *item = role_to_json(&roles[i]);
        if (!item) {
            cJSON_Delete(root);
            free(roles);
            http_response_set_json_error(res, 500,
                                         "Failed to create response");
            return;
        }
        cJSON_AddItemToArray(items, item);
    }
    free(roles);
    set_json_response(res, root);
}

static void set_role_mutation_response(http_response_t *res, int status,
                                       const char *uuid,
                                       int64_t policy_version) {
    authorization_role_t role;
    if (db_authorization_role_get(uuid, &role) != DB_AUTHORIZATION_OK) {
        http_response_set_json_error(res, 500, "Failed to reload role");
        return;
    }
    cJSON *root = cJSON_CreateObject();
    cJSON *role_json = role_to_json(&role);
    if (!root || !role_json) {
        cJSON_Delete(root);
        cJSON_Delete(role_json);
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    cJSON_AddNumberToObject(root, "policy_version", (double)policy_version);
    cJSON_AddItemToObject(root, "role", role_json);
    set_json_response_status(res, status, root);
}

static int retains_management_after_role_update(
    const user_t *requester, const authorization_role_t *proposed_role) {
    if (strcmp(requester->authorization_mode, "policy") != 0) return 1;
    char mode[USER_AUTHORIZATION_MODE_MAX];
    authorization_grant_t *grants = NULL;
    int grant_count = 0;
    int64_t policy_version = 0;
    if (db_authorization_get_user_policy(
            requester->id, mode, &grants, &grant_count, &policy_version) !=
        DB_AUTHORIZATION_OK) {
        return -1;
    }
    bool retained = false;
    for (int i = 0; i < grant_count && !retained; i++) {
        if (!grants[i].enabled || strcmp(grants[i].scope_type, "all") != 0) {
            continue;
        }
        uint64_t action_mask = 0;
        if (strcmp(grants[i].role_uuid, proposed_role->uuid) == 0) {
            action_mask = proposed_role->action_mask;
        } else {
            authorization_role_t role;
            if (db_authorization_role_get(grants[i].role_uuid, &role) !=
                DB_AUTHORIZATION_OK) {
                free(grants);
                return -1;
            }
            action_mask = role.action_mask;
        }
        retained =
            (action_mask & (UINT64_C(1) << AUTHZ_USERS_MANAGE)) != 0;
    }
    free(grants);
    return retained ? 1 : 0;
}

void handle_post_authorization_role(const http_request_t *req,
                                    http_response_t *res) {
    user_t requester;
    if (!authorize_policy_manager(req, res, &requester)) return;
    cJSON *body = httpd_parse_json_body(req);
    authorization_role_t role;
    memset(&role, 0, sizeof(role));
    int64_t expected_version = 0;
    if (!parse_role_body(body, &role, &expected_version, res)) {
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);
    int64_t new_version = 0;
    db_authorization_result_t result = db_authorization_role_create(
        &role, expected_version, &new_version);
    if (result != DB_AUTHORIZATION_OK) {
        set_authorization_db_error(res, result);
        return;
    }
    set_role_mutation_response(res, 201, role.uuid, new_version);
}

void handle_put_authorization_role(const http_request_t *req,
                                   http_response_t *res) {
    user_t requester;
    if (!authorize_policy_manager(req, res, &requester)) return;
    authorization_role_t role;
    memset(&role, 0, sizeof(role));
    if (!extract_role_uuid(req, role.uuid, res)) return;
    cJSON *body = httpd_parse_json_body(req);
    int64_t expected_version = 0;
    if (!parse_role_body(body, &role, &expected_version, res)) {
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);
    int retains_management =
        retains_management_after_role_update(&requester, &role);
    if (retains_management <= 0) {
        http_response_set_json_error(
            res, retains_management == 0 ? 409 : 500,
            retains_management == 0
                ? "This change would remove your own policy-management access"
                : "Failed to verify policy-management access");
        return;
    }
    int64_t new_version = 0;
    db_authorization_result_t result = db_authorization_role_update(
        &role, expected_version, &new_version);
    if (result != DB_AUTHORIZATION_OK) {
        set_authorization_db_error(res, result);
        return;
    }
    set_role_mutation_response(res, 200, role.uuid, new_version);
}

void handle_delete_authorization_role(const http_request_t *req,
                                      http_response_t *res) {
    user_t requester;
    if (!authorize_policy_manager(req, res, &requester)) return;
    char uuid[CAMERA_UUID_STRING_SIZE];
    if (!extract_role_uuid(req, uuid, res)) return;
    cJSON *body = httpd_parse_json_body(req);
    int64_t expected_version = 0;
    if (!cJSON_IsObject(body) ||
        !parse_expected_version(body, &expected_version, res)) {
        if (!cJSON_IsObject(body)) {
            http_response_set_json_error(res, 400,
                                         "Request body must be an object");
        }
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);
    int64_t new_version = 0;
    db_authorization_result_t result = db_authorization_role_delete(
        uuid, expected_version, &new_version);
    if (result != DB_AUTHORIZATION_OK) {
        set_authorization_db_error(res, result);
        return;
    }
    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddNumberToObject(response, "policy_version", (double)new_version);
    set_json_response(res, response);
}

static cJSON *grant_to_json(const authorization_grant_t *grant) {
    cJSON *object = cJSON_CreateObject();
    cJSON *scope = cJSON_CreateObject();
    if (!object || !scope) {
        cJSON_Delete(object);
        cJSON_Delete(scope);
        return NULL;
    }
    cJSON_AddStringToObject(object, "uuid", grant->uuid);
    cJSON_AddStringToObject(object, "role_uuid", grant->role_uuid);
    cJSON_AddStringToObject(object, "role", grant->role_name);
    cJSON_AddBoolToObject(object, "enabled", grant->enabled);
    cJSON_AddNumberToObject(object, "created_at", (double)grant->created_at);
    cJSON_AddNumberToObject(object, "updated_at", (double)grant->updated_at);
    cJSON_AddStringToObject(scope, "type", grant->scope_type);
    if (strcmp(grant->scope_type, "selector") == 0) {
        cJSON *selector = cJSON_Parse(grant->selector_json);
        if (!selector) {
            cJSON_Delete(object);
            cJSON_Delete(scope);
            return NULL;
        }
        cJSON_AddItemToObject(scope, "selector", selector);
    } else {
        cJSON_AddNullToObject(scope, "selector");
    }
    cJSON_AddItemToObject(object, "scope", scope);
    return object;
}

static void set_user_policy_response(http_response_t *res, int64_t user_id,
                                     int status) {
    char mode[USER_AUTHORIZATION_MODE_MAX];
    authorization_grant_t *grants = NULL;
    int grant_count = 0;
    int64_t policy_version = 0;
    db_authorization_result_t result = db_authorization_get_user_policy(
        user_id, mode, &grants, &grant_count, &policy_version);
    if (result != DB_AUTHORIZATION_OK) {
        set_authorization_db_error(res, result);
        return;
    }
    cJSON *root = cJSON_CreateObject();
    cJSON *items = cJSON_CreateArray();
    if (!root || !items) {
        cJSON_Delete(root);
        cJSON_Delete(items);
        free(grants);
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    cJSON_AddNumberToObject(root, "user_id", (double)user_id);
    cJSON_AddStringToObject(root, "mode", mode);
    cJSON_AddNumberToObject(root, "policy_version", (double)policy_version);
    cJSON_AddNumberToObject(root, "grant_count", grant_count);
    cJSON_AddItemToObject(root, "grants", items);
    for (int i = 0; i < grant_count; i++) {
        cJSON *item = grant_to_json(&grants[i]);
        if (!item) {
            cJSON_Delete(root);
            free(grants);
            http_response_set_json_error(res, 500,
                                         "Failed to create response");
            return;
        }
        cJSON_AddItemToArray(items, item);
    }
    free(grants);
    set_json_response_status(res, status, root);
}

void handle_get_user_authorization(const http_request_t *req,
                                   http_response_t *res) {
    user_t requester;
    if (!authorize_policy_manager(req, res, &requester)) return;
    int64_t user_id = 0;
    if (!extract_user_id(req, &user_id, res)) return;
    set_user_policy_response(res, user_id, 200);
}

static bool parse_grants(const cJSON *body,
                         authorization_grant_input_t **grants_out,
                         int *grant_count_out, http_response_t *res) {
    const cJSON *grants = cJSON_GetObjectItemCaseSensitive(body, "grants");
    if (!cJSON_IsArray(grants)) {
        http_response_set_json_error(res, 400, "grants must be an array");
        return false;
    }
    int count = cJSON_GetArraySize(grants);
    if (count > AUTHORIZATION_MAX_USER_GRANTS) {
        http_response_set_json_error(res, 400, "Grant limit exceeded");
        return false;
    }
    authorization_grant_input_t *parsed = count > 0
        ? calloc((size_t)count, sizeof(*parsed)) : NULL;
    if (count > 0 && !parsed) {
        http_response_set_json_error(res, 500, "Out of memory");
        return false;
    }
    for (int i = 0; i < count; i++) {
        const cJSON *grant = cJSON_GetArrayItem(grants, i);
        const cJSON *role_uuid = cJSON_IsObject(grant)
            ? cJSON_GetObjectItemCaseSensitive(grant, "role_uuid") : NULL;
        const cJSON *scope = cJSON_IsObject(grant)
            ? cJSON_GetObjectItemCaseSensitive(grant, "scope") : NULL;
        const cJSON *type = cJSON_IsObject(scope)
            ? cJSON_GetObjectItemCaseSensitive(scope, "type") : NULL;
        const cJSON *selector = cJSON_IsObject(scope)
            ? cJSON_GetObjectItemCaseSensitive(scope, "selector") : NULL;
        if (!cJSON_IsString(role_uuid) ||
            !valid_uuid(role_uuid->valuestring) || !cJSON_IsString(type) ||
            (strcmp(type->valuestring, "all") != 0 &&
             strcmp(type->valuestring, "selector") != 0)) {
            free(parsed);
            http_response_set_json_error(res, 400, "Invalid grant");
            return false;
        }
        safe_strcpy(parsed[i].role_uuid, role_uuid->valuestring,
                    sizeof(parsed[i].role_uuid), 0);
        safe_strcpy(parsed[i].scope_type, type->valuestring,
                    sizeof(parsed[i].scope_type), 0);
        if (strcmp(type->valuestring, "all") == 0) {
            if (selector && !cJSON_IsNull(selector)) {
                free(parsed);
                http_response_set_json_error(
                    res, 400, "All-camera grants cannot include a selector");
                return false;
            }
            continue;
        }
        if (!selector) {
            free(parsed);
            http_response_set_json_error(
                res, 400, "Selector grants require a selector");
            return false;
        }
        char *serialized = cJSON_PrintUnformatted(selector);
        if (!serialized ||
            strlen(serialized) >= sizeof(parsed[i].selector_json)) {
            free(serialized);
            free(parsed);
            http_response_set_json_error(res, 400,
                                         "Grant selector is too large");
            return false;
        }
        safe_strcpy(parsed[i].selector_json, serialized,
                    sizeof(parsed[i].selector_json), 0);
        free(serialized);
    }
    *grants_out = parsed;
    *grant_count_out = count;
    return true;
}

static bool grants_allow_self_management(
    const authorization_grant_input_t *grants, int grant_count) {
    for (int i = 0; i < grant_count; i++) {
        if (strcmp(grants[i].scope_type, "all") != 0) continue;
        authorization_role_t role;
        if (db_authorization_role_get(grants[i].role_uuid, &role) !=
            DB_AUTHORIZATION_OK) {
            continue;
        }
        if ((role.action_mask & (UINT64_C(1) << AUTHZ_USERS_MANAGE)) != 0) {
            return true;
        }
    }
    return false;
}

void handle_put_user_authorization(const http_request_t *req,
                                   http_response_t *res) {
    user_t requester;
    if (!authorize_policy_manager(req, res, &requester)) return;
    int64_t user_id = 0;
    if (!extract_user_id(req, &user_id, res)) return;
    cJSON *body = httpd_parse_json_body(req);
    if (!cJSON_IsObject(body)) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400,
                                     "Request body must be an object");
        return;
    }
    int64_t expected_version = 0;
    const cJSON *mode = cJSON_GetObjectItemCaseSensitive(body, "mode");
    if (!parse_expected_version(body, &expected_version, res)) {
        cJSON_Delete(body);
        return;
    }
    if (!cJSON_IsString(mode) || !mode->valuestring ||
        (strcmp(mode->valuestring, "legacy") != 0 &&
         strcmp(mode->valuestring, "policy") != 0)) {
        http_response_set_json_error(res, 400,
                                     "mode must be legacy or policy");
        cJSON_Delete(body);
        return;
    }
    authorization_grant_input_t *grants = NULL;
    int grant_count = 0;
    if (!parse_grants(body, &grants, &grant_count, res)) {
        cJSON_Delete(body);
        return;
    }
    if (user_id == requester.id &&
        ((strcmp(mode->valuestring, "policy") == 0 &&
          !grants_allow_self_management(grants, grant_count)) ||
         (strcmp(mode->valuestring, "legacy") == 0 &&
          requester.role != USER_ROLE_ADMIN))) {
        free(grants);
        cJSON_Delete(body);
        http_response_set_json_error(
            res, 409, "This change would remove your own policy-management access");
        return;
    }
    char requested_mode[USER_AUTHORIZATION_MODE_MAX];
    safe_strcpy(requested_mode, mode->valuestring, sizeof(requested_mode), 0);
    cJSON_Delete(body);
    int64_t new_version = 0;
    db_authorization_result_t result = db_authorization_replace_user_policy(
        user_id, requested_mode, grants, grant_count, expected_version,
        &new_version);
    free(grants);
    if (result != DB_AUTHORIZATION_OK) {
        set_authorization_db_error(res, result);
        return;
    }
    set_user_policy_response(res, user_id, 200);
}
