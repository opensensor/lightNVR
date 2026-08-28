#define _POSIX_C_SOURCE 200809L

#include "web/api_handlers_operator_floor_plans.h"

#include <cjson/cJSON.h>
#include <math.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "core/authorization.h"
#include "database/db_auth.h"
#include "database/db_fleet_query.h"
#include "database/db_operator_floor_plans.h"
#include "utils/strings.h"
#include "utils/uuid.h"
#include "web/httpd_utils.h"

static bool authenticate(const http_request_t *req, http_response_t *res,
                         user_t *user) {
    memset(user, 0, sizeof(*user));
    if (!httpd_check_action_access(req, user)) {
        http_response_set_json_error(res, 401, "Unauthorized");
        return false;
    }
    return true;
}

static bool read_only_identity(const user_t *user) {
    return user->authenticated_via_scoped_token ||
        strcmp(user->authentication_method, "demo") == 0;
}

static bool can_modify(const user_t *user) {
    authorization_evaluation_t evaluation;
    return !read_only_identity(user) &&
        authorization_evaluate(user, AUTHZ_CAMERA_CONFIGURE, NULL,
                               &evaluation) == 0 &&
        evaluation.decision == AUTHZ_DECISION_ALLOW;
}

static bool load_authorized_cameras(
    const user_t *user, authorization_action_t action,
    fleet_camera_t **cameras, int *camera_count) {
    *cameras = NULL;
    *camera_count = 0;
    return db_fleet_camera_load(cameras, camera_count) == 0 &&
        authorization_filter_cameras(user, action, *cameras,
                                     camera_count) == 0;
}

static bool camera_allowed(const char *camera_uuid,
                           const fleet_camera_t *cameras, int camera_count) {
    for (int index = 0; index < camera_count; index++) {
        if (strcmp(cameras[index].camera_uuid, camera_uuid) == 0) return true;
    }
    return false;
}

static bool copy_string(const cJSON *body, const char *key,
                        char *destination, size_t size, bool required) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(body, key);
    if (!item) return !required;
    if (!cJSON_IsString(item) || !item->valuestring ||
        (required && item->valuestring[0] == '\0') ||
        strlen(item->valuestring) >= size) return false;
    safe_strcpy(destination, item->valuestring, size, 0);
    return true;
}

static bool optional_uuid(const cJSON *body, const char *key,
                          char *destination, size_t size) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(body, key);
    if (!item || cJSON_IsNull(item)) {
        destination[0] = '\0';
        return true;
    }
    if (!cJSON_IsString(item) || !item->valuestring ||
        !lightnvr_uuid_is_valid(item->valuestring)) return false;
    safe_strcpy(destination, item->valuestring, size, 0);
    return true;
}

static bool integer_field(const cJSON *body, const char *key, int minimum,
                          int maximum, int fallback, int *value) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(body, key);
    if (!item) {
        *value = fallback;
        return true;
    }
    if (!cJSON_IsNumber(item) || !isfinite(item->valuedouble) ||
        floor(item->valuedouble) != item->valuedouble ||
        item->valueint < minimum || item->valueint > maximum) return false;
    *value = item->valueint;
    return true;
}

static bool number_field(const cJSON *body, const char *key, double minimum,
                         double maximum, double fallback, double *value) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(body, key);
    if (!item) {
        *value = fallback;
        return true;
    }
    if (!cJSON_IsNumber(item) || !isfinite(item->valuedouble) ||
        item->valuedouble < minimum || item->valuedouble > maximum) {
        return false;
    }
    *value = item->valuedouble;
    return true;
}

static bool parse_revision(const cJSON *body, int64_t *revision) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(body, "revision");
    if (!cJSON_IsNumber(item) || !isfinite(item->valuedouble) ||
        floor(item->valuedouble) != item->valuedouble ||
        item->valuedouble < 1 || item->valuedouble > (double)INT64_MAX) {
        return false;
    }
    *revision = (int64_t)item->valuedouble;
    return true;
}

static bool parse_plan(
    const cJSON *body, bool creating, operator_floor_plan_t *plan,
    operator_floor_plan_camera_t **cameras, int *camera_count,
    http_response_t *res) {
    memset(plan, 0, sizeof(*plan));
    *cameras = NULL;
    *camera_count = 0;
    if (!cJSON_IsObject(body) ||
        !copy_string(body, "name", plan->name, sizeof(plan->name), true) ||
        !optional_uuid(body, "location_uuid", plan->location_uuid,
                       sizeof(plan->location_uuid)) ||
        !optional_uuid(body, "parent_plan_uuid", plan->parent_plan_uuid,
                       sizeof(plan->parent_plan_uuid)) ||
        !integer_field(body, "canvas_width", 400, 4000, 1200,
                       &plan->canvas_width) ||
        !integer_field(body, "canvas_height", 300, 4000, 800,
                       &plan->canvas_height)) {
        http_response_set_json_error(res, 400, "Invalid floor plan fields");
        return false;
    }
    const cJSON *items = cJSON_GetObjectItemCaseSensitive(body, "cameras");
    if (!items && creating) return true;
    if (!cJSON_IsArray(items)) {
        http_response_set_json_error(res, 400, "cameras must be an array");
        return false;
    }
    int count = cJSON_GetArraySize(items);
    if (count < 0 || count > OPERATOR_FLOOR_PLAN_MAX_CAMERAS) {
        http_response_set_json_error(res, 400, "Too many floor plan cameras");
        return false;
    }
    operator_floor_plan_camera_t *parsed = count > 0
        ? calloc((size_t)count, sizeof(*parsed)) : NULL;
    if (count > 0 && !parsed) {
        http_response_set_json_error(res, 500, "Memory allocation failed");
        return false;
    }
    bool valid = true;
    for (int index = 0; valid && index < count; index++) {
        const cJSON *item = cJSON_GetArrayItem(items, index);
        valid = cJSON_IsObject(item) &&
            copy_string(item, "camera_uuid", parsed[index].camera_uuid,
                        sizeof(parsed[index].camera_uuid), true) &&
            lightnvr_uuid_is_valid(parsed[index].camera_uuid) &&
            number_field(item, "x", 0.0, 1.0, 0.5, &parsed[index].x) &&
            number_field(item, "y", 0.0, 1.0, 0.5, &parsed[index].y) &&
            number_field(item, "rotation", -180.0, 180.0, 0.0,
                         &parsed[index].rotation) &&
            number_field(item, "fov", 1.0, 180.0, 65.0,
                         &parsed[index].fov);
        for (int previous = 0; valid && previous < index; previous++) {
            if (strcmp(parsed[previous].camera_uuid,
                       parsed[index].camera_uuid) == 0) valid = false;
        }
    }
    if (!valid) {
        free(parsed);
        http_response_set_json_error(res, 400,
                                     "Invalid floor plan camera placement");
        return false;
    }
    *cameras = parsed;
    *camera_count = count;
    return true;
}

static cJSON *plan_json(
    const operator_floor_plan_t *plan,
    const operator_floor_plan_camera_t *placements, int placement_count,
    const fleet_camera_t *authorized, int authorized_count) {
    cJSON *root = cJSON_CreateObject();
    cJSON *cameras = root ? cJSON_AddArrayToObject(root, "cameras") : NULL;
    if (!root || !cameras) {
        cJSON_Delete(root);
        return NULL;
    }
    cJSON_AddStringToObject(root, "uuid", plan->uuid);
    cJSON_AddStringToObject(root, "name", plan->name);
    if (plan->location_uuid[0]) {
        cJSON_AddStringToObject(root, "location_uuid", plan->location_uuid);
    } else {
        cJSON_AddNullToObject(root, "location_uuid");
    }
    if (plan->parent_plan_uuid[0]) {
        cJSON_AddStringToObject(root, "parent_plan_uuid",
                                plan->parent_plan_uuid);
    } else {
        cJSON_AddNullToObject(root, "parent_plan_uuid");
    }
    cJSON_AddNumberToObject(root, "canvas_width", plan->canvas_width);
    cJSON_AddNumberToObject(root, "canvas_height", plan->canvas_height);
    bool partially_authorized = false;
    for (int index = 0; index < placement_count; index++) {
        if (!camera_allowed(placements[index].camera_uuid, authorized,
                            authorized_count)) {
            partially_authorized = true;
            continue;
        }
        cJSON *camera = cJSON_CreateObject();
        if (!camera) continue;
        cJSON_AddStringToObject(camera, "camera_uuid",
                                placements[index].camera_uuid);
        cJSON_AddNumberToObject(camera, "x", placements[index].x);
        cJSON_AddNumberToObject(camera, "y", placements[index].y);
        cJSON_AddNumberToObject(camera, "rotation",
                                placements[index].rotation);
        cJSON_AddNumberToObject(camera, "fov", placements[index].fov);
        cJSON_AddItemToArray(cameras, camera);
    }
    cJSON_AddBoolToObject(root, "partially_authorized",
                          partially_authorized);
    cJSON_AddNumberToObject(root, "revision", (double)plan->revision);
    cJSON_AddNumberToObject(root, "created_at", (double)plan->created_at);
    cJSON_AddNumberToObject(root, "updated_at", (double)plan->updated_at);
    return root;
}

static void send_json(http_response_t *res, int status, cJSON *json) {
    char *encoded = json ? cJSON_PrintUnformatted(json) : NULL;
    if (!encoded) {
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    http_response_set_json(res, status, encoded);
    free(encoded);
}

static void set_db_error(http_response_t *res,
                         db_operator_floor_plan_result_t result) {
    switch (result) {
        case DB_OPERATOR_FLOOR_PLAN_NOT_FOUND:
            http_response_set_json_error(res, 404, "Floor plan not found");
            break;
        case DB_OPERATOR_FLOOR_PLAN_INVALID:
            http_response_set_json_error(res, 400, "Invalid floor plan");
            break;
        case DB_OPERATOR_FLOOR_PLAN_CONFLICT:
            http_response_set_json_error(
                res, 409, "Floor plan name or relationship conflicts");
            break;
        case DB_OPERATOR_FLOOR_PLAN_STALE:
            http_response_set_json_error(
                res, 409, "Floor plan changed; reload and retry");
            break;
        case DB_OPERATOR_FLOOR_PLAN_LIMIT:
            http_response_set_json_error(res, 409, "Floor plan limit reached");
            break;
        default:
            http_response_set_json_error(res, 500,
                                         "Floor plan database operation failed");
            break;
    }
}

static bool extract_uuid(const http_request_t *req, char *uuid,
                         size_t uuid_size, http_response_t *res) {
    if (http_request_extract_path_param(req, "/api/live/plans/", uuid,
                                        uuid_size) != 0 ||
        strchr(uuid, '/') || !lightnvr_uuid_is_valid(uuid)) {
        http_response_set_json_error(res, 400, "Invalid floor plan path");
        return false;
    }
    return true;
}

static bool require_modify(const http_request_t *req, http_response_t *res,
                           user_t *user) {
    if (!authenticate(req, res, user)) return false;
    if (!can_modify(user)) {
        http_response_set_json_error(res, 403,
                                     "Floor plan editing is not allowed");
        return false;
    }
    return true;
}

static cJSON *load_one_json(const char *uuid, const user_t *user,
                            http_response_t *res) {
    operator_floor_plan_t plan;
    db_operator_floor_plan_result_t result =
        db_operator_floor_plan_get(uuid, &plan);
    if (result != DB_OPERATOR_FLOOR_PLAN_OK) {
        set_db_error(res, result);
        return NULL;
    }
    operator_floor_plan_camera_t *placements = calloc(
        OPERATOR_FLOOR_PLAN_MAX_CAMERAS, sizeof(*placements));
    fleet_camera_t *authorized = NULL;
    int authorized_count = 0;
    int placement_count = placements
        ? db_operator_floor_plan_camera_list(
            uuid, placements, OPERATOR_FLOOR_PLAN_MAX_CAMERAS) : -1;
    if (placement_count < 0 || !load_authorized_cameras(
            user, AUTHZ_LIVE_VIEW, &authorized, &authorized_count)) {
        free(placements);
        free(authorized);
        http_response_set_json_error(res, 500, "Failed to load floor plan");
        return NULL;
    }
    cJSON *json = plan_json(&plan, placements, placement_count,
                            authorized, authorized_count);
    free(placements);
    free(authorized);
    return json;
}

void handle_get_operator_floor_plans(const http_request_t *req,
                                     http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;
    operator_floor_plan_t *plans = calloc(
        OPERATOR_FLOOR_PLAN_MAX_VISIBLE, sizeof(*plans));
    fleet_camera_t *authorized = NULL;
    int authorized_count = 0;
    int plan_count = plans ? db_operator_floor_plan_list(
        plans, OPERATOR_FLOOR_PLAN_MAX_VISIBLE) : -1;
    if (plan_count < 0 || !load_authorized_cameras(
            &user, AUTHZ_LIVE_VIEW, &authorized, &authorized_count)) {
        free(plans);
        free(authorized);
        http_response_set_json_error(res, 500, "Failed to list floor plans");
        return;
    }
    cJSON *root = cJSON_CreateObject();
    cJSON *items = root ? cJSON_AddArrayToObject(root, "plans") : NULL;
    operator_floor_plan_camera_t *placements = calloc(
        OPERATOR_FLOOR_PLAN_MAX_CAMERAS, sizeof(*placements));
    if (!root || !items || !placements) {
        cJSON_Delete(root);
        free(plans);
        free(authorized);
        free(placements);
        http_response_set_json_error(res, 500, "Memory allocation failed");
        return;
    }
    for (int index = 0; index < plan_count; index++) {
        int placement_count = db_operator_floor_plan_camera_list(
            plans[index].uuid, placements,
            OPERATOR_FLOOR_PLAN_MAX_CAMERAS);
        if (placement_count < 0) continue;
        cJSON *item = plan_json(&plans[index], placements, placement_count,
                                authorized, authorized_count);
        if (item) cJSON_AddItemToArray(items, item);
    }
    cJSON_AddBoolToObject(root, "can_modify", can_modify(&user));
    send_json(res, 200, root);
    cJSON_Delete(root);
    free(placements);
    free(plans);
    free(authorized);
}

void handle_get_operator_floor_plan(const http_request_t *req,
                                    http_response_t *res) {
    user_t user;
    char uuid[CAMERA_UUID_STRING_SIZE];
    if (!authenticate(req, res, &user) ||
        !extract_uuid(req, uuid, sizeof(uuid), res)) return;
    cJSON *json = load_one_json(uuid, &user, res);
    if (!json) return;
    cJSON_AddBoolToObject(json, "can_modify", can_modify(&user));
    send_json(res, 200, json);
    cJSON_Delete(json);
}

static bool placements_allowed(
    const user_t *user, const operator_floor_plan_camera_t *placements,
    int placement_count, http_response_t *res) {
    fleet_camera_t *authorized = NULL;
    int authorized_count = 0;
    if (!load_authorized_cameras(user, AUTHZ_CAMERA_CONFIGURE,
                                 &authorized, &authorized_count)) {
        free(authorized);
        http_response_set_json_error(res, 500,
                                     "Authorization evaluation failed");
        return false;
    }
    bool allowed = true;
    for (int index = 0; index < placement_count; index++) {
        if (!camera_allowed(placements[index].camera_uuid, authorized,
                            authorized_count)) {
            allowed = false;
            break;
        }
    }
    free(authorized);
    if (!allowed) {
        http_response_set_json_error(
            res, 403, "Floor plan contains an unavailable camera");
    }
    return allowed;
}

void handle_post_operator_floor_plan(const http_request_t *req,
                                     http_response_t *res) {
    user_t user;
    if (!require_modify(req, res, &user)) return;
    cJSON *body = cJSON_ParseWithLength((const char *)req->body,
                                        req->body_len);
    operator_floor_plan_t plan;
    operator_floor_plan_camera_t *placements = NULL;
    int placement_count = 0;
    if (!parse_plan(body, true, &plan, &placements, &placement_count, res) ||
        !placements_allowed(&user, placements, placement_count, res)) {
        free(placements);
        cJSON_Delete(body);
        return;
    }
    db_operator_floor_plan_result_t result = db_operator_floor_plan_create(
        &plan, placements, placement_count);
    free(placements);
    cJSON_Delete(body);
    if (result != DB_OPERATOR_FLOOR_PLAN_OK) {
        set_db_error(res, result);
        return;
    }
    cJSON *json = load_one_json(plan.uuid, &user, res);
    if (!json) return;
    send_json(res, 201, json);
    cJSON_Delete(json);
}

void handle_put_operator_floor_plan(const http_request_t *req,
                                    http_response_t *res) {
    user_t user;
    char uuid[CAMERA_UUID_STRING_SIZE];
    if (!require_modify(req, res, &user) ||
        !extract_uuid(req, uuid, sizeof(uuid), res)) return;
    cJSON *body = cJSON_ParseWithLength((const char *)req->body,
                                        req->body_len);
    operator_floor_plan_t plan;
    operator_floor_plan_camera_t *placements = NULL;
    int placement_count = 0;
    int64_t revision = 0;
    if (!parse_plan(body, false, &plan, &placements, &placement_count, res) ||
        !parse_revision(body, &revision) ||
        !placements_allowed(&user, placements, placement_count, res)) {
        if (res->status_code < 400) {
            http_response_set_json_error(res, 400, "Invalid revision");
        }
        free(placements);
        cJSON_Delete(body);
        return;
    }
    safe_strcpy(plan.uuid, uuid, sizeof(plan.uuid), 0);
    db_operator_floor_plan_result_t result = db_operator_floor_plan_update(
        &plan, placements, placement_count, revision);
    free(placements);
    cJSON_Delete(body);
    if (result != DB_OPERATOR_FLOOR_PLAN_OK) {
        set_db_error(res, result);
        return;
    }
    cJSON *json = load_one_json(uuid, &user, res);
    if (!json) return;
    send_json(res, 200, json);
    cJSON_Delete(json);
}

void handle_delete_operator_floor_plan(const http_request_t *req,
                                       http_response_t *res) {
    user_t user;
    char uuid[CAMERA_UUID_STRING_SIZE];
    if (!require_modify(req, res, &user) ||
        !extract_uuid(req, uuid, sizeof(uuid), res)) return;
    cJSON *body = cJSON_ParseWithLength((const char *)req->body,
                                        req->body_len);
    int64_t revision = 0;
    if (!cJSON_IsObject(body) || !parse_revision(body, &revision)) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400, "Invalid revision");
        return;
    }
    cJSON_Delete(body);
    db_operator_floor_plan_result_t result =
        db_operator_floor_plan_delete(uuid, revision);
    if (result != DB_OPERATOR_FLOOR_PLAN_OK) {
        set_db_error(res, result);
        return;
    }
    cJSON *json = cJSON_CreateObject();
    cJSON_AddBoolToObject(json, "deleted", true);
    send_json(res, 200, json);
    cJSON_Delete(json);
}
