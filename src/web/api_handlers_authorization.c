#define _POSIX_C_SOURCE 200809L

#include <cjson/cJSON.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>

#include "core/authorization.h"
#include "database/db_auth.h"
#include "database/db_fleet_query.h"
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
