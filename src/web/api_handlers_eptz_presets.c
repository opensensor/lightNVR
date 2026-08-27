#define _POSIX_C_SOURCE 200809L

#include "web/api_handlers_eptz_presets.h"

#include <cjson/cJSON.h>
#include <math.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "core/authorization.h"
#include "database/db_auth.h"
#include "database/db_eptz_presets.h"
#include "utils/strings.h"
#include "utils/uuid.h"
#include "web/httpd_utils.h"

typedef struct {
    char camera_uuid[CAMERA_UUID_STRING_SIZE];
    char preset_uuid[CAMERA_UUID_STRING_SIZE];
    user_t user;
} eptz_request_context_t;

static bool prepare_request(const http_request_t *req, http_response_t *res,
                            bool require_preset,
                            eptz_request_context_t *context) {
    memset(context, 0, sizeof(*context));
    char path[CAMERA_UUID_STRING_SIZE * 2 + 32];
    if (http_request_extract_path_param(req, "/api/cameras/", path,
                                        sizeof(path)) != 0) {
        http_response_set_json_error(res, 400, "Invalid ePTZ preset path");
        return false;
    }
    char *separator = strchr(path, '/');
    if (!separator) {
        http_response_set_json_error(res, 400, "Invalid ePTZ preset path");
        return false;
    }
    *separator = '\0';
    if (!lightnvr_uuid_is_valid(path) ||
        strncmp(separator + 1, "eptz-presets", 12) != 0) {
        http_response_set_json_error(res, 400, "Invalid camera UUID");
        return false;
    }
    safe_strcpy(context->camera_uuid, path,
                sizeof(context->camera_uuid), 0);
    const char *suffix = separator + 1;
    const char *preset = suffix + 13;
    if (require_preset) {
        if (suffix[12] != '/' || !lightnvr_uuid_is_valid(preset)) {
            http_response_set_json_error(res, 400, "Invalid preset UUID");
            return false;
        }
        safe_strcpy(context->preset_uuid, preset,
                    sizeof(context->preset_uuid), 0);
    } else if (suffix[12] != '\0') {
        http_response_set_json_error(res, 400, "Invalid ePTZ preset path");
        return false;
    }
    fleet_camera_t camera;
    authorization_evaluation_t evaluation;
    if (!httpd_authorize_camera_identity_action_with_context(
            req, res, AUTHZ_LIVE_VIEW, context->camera_uuid, NULL,
            &context->user, &camera, &evaluation)) return false;
    return true;
}

static bool read_only_identity(const user_t *user) {
    return user->authenticated_via_scoped_token ||
        strcmp(user->authentication_method, "demo") == 0;
}

static bool copy_string(const cJSON *body, const char *key,
                        char *destination, size_t size) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(body, key);
    if (!cJSON_IsString(item) || !item->valuestring ||
        item->valuestring[0] == '\0' || strlen(item->valuestring) >= size) {
        return false;
    }
    safe_strcpy(destination, item->valuestring, size, 0);
    return true;
}

static bool copy_number(const cJSON *body, const char *key, double minimum,
                        double maximum, double *value) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(body, key);
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

static bool parse_preset(const cJSON *body,
                         const eptz_request_context_t *context,
                         eptz_operator_preset_t *preset,
                         http_response_t *res) {
    memset(preset, 0, sizeof(*preset));
    preset->owner_user_id = context->user.id;
    safe_strcpy(preset->camera_uuid, context->camera_uuid,
                sizeof(preset->camera_uuid), 0);
    if (!cJSON_IsObject(body) ||
        !copy_string(body, "name", preset->name, sizeof(preset->name)) ||
        !copy_string(body, "mode", preset->mode, sizeof(preset->mode)) ||
        !copy_number(body, "yaw", -180, 180, &preset->yaw) ||
        !copy_number(body, "tilt", -90, 30, &preset->tilt) ||
        !copy_number(body, "view_fov", 20, 120, &preset->view_fov) ||
        !copy_number(body, "secondary_yaw", -180, 180,
                     &preset->secondary_yaw) ||
        !copy_number(body, "secondary_tilt", -90, 30,
                     &preset->secondary_tilt) ||
        !copy_number(body, "secondary_view_fov", 20, 120,
                     &preset->secondary_view_fov)) {
        http_response_set_json_error(res, 400, "Invalid ePTZ preset fields");
        return false;
    }
    const cJSON *shared = cJSON_GetObjectItemCaseSensitive(body, "is_shared");
    if (shared && !cJSON_IsBool(shared)) {
        http_response_set_json_error(res, 400, "is_shared must be boolean");
        return false;
    }
    preset->is_shared = shared && cJSON_IsTrue(shared);
    return true;
}

static cJSON *preset_json(const eptz_operator_preset_t *preset,
                          int64_t current_user_id) {
    cJSON *root = cJSON_CreateObject();
    if (!root) return NULL;
    cJSON_AddStringToObject(root, "uuid", preset->uuid);
    cJSON_AddStringToObject(root, "camera_uuid", preset->camera_uuid);
    cJSON_AddStringToObject(root, "name", preset->name);
    cJSON_AddBoolToObject(root, "is_shared", preset->is_shared);
    cJSON_AddBoolToObject(root, "owned",
                          preset->owner_user_id == current_user_id);
    cJSON_AddStringToObject(root, "mode", preset->mode);
    cJSON_AddNumberToObject(root, "yaw", preset->yaw);
    cJSON_AddNumberToObject(root, "tilt", preset->tilt);
    cJSON_AddNumberToObject(root, "view_fov", preset->view_fov);
    cJSON_AddNumberToObject(root, "secondary_yaw", preset->secondary_yaw);
    cJSON_AddNumberToObject(root, "secondary_tilt", preset->secondary_tilt);
    cJSON_AddNumberToObject(root, "secondary_view_fov",
                            preset->secondary_view_fov);
    cJSON_AddNumberToObject(root, "revision", (double)preset->revision);
    cJSON_AddNumberToObject(root, "created_at", (double)preset->created_at);
    cJSON_AddNumberToObject(root, "updated_at", (double)preset->updated_at);
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
                         db_eptz_preset_result_t result) {
    switch (result) {
        case DB_EPTZ_PRESET_NOT_FOUND:
            http_response_set_json_error(res, 404, "ePTZ preset not found");
            break;
        case DB_EPTZ_PRESET_INVALID:
            http_response_set_json_error(res, 400, "Invalid ePTZ preset");
            break;
        case DB_EPTZ_PRESET_CONFLICT:
            http_response_set_json_error(
                res, 409, "An ePTZ preset with this name already exists");
            break;
        case DB_EPTZ_PRESET_STALE:
            http_response_set_json_error(
                res, 409, "ePTZ preset changed; reload and retry");
            break;
        case DB_EPTZ_PRESET_LIMIT:
            http_response_set_json_error(res, 409, "ePTZ preset limit reached");
            break;
        default:
            http_response_set_json_error(res, 500,
                                         "ePTZ preset operation failed");
            break;
    }
}

void handle_get_eptz_presets(const http_request_t *req,
                             http_response_t *res) {
    eptz_request_context_t context;
    if (!prepare_request(req, res, false, &context)) return;
    eptz_operator_preset_t *presets = calloc(
        EPTZ_PRESET_MAX_VISIBLE, sizeof(*presets));
    if (!presets) {
        http_response_set_json_error(res, 500, "Out of memory");
        return;
    }
    int count = db_eptz_preset_list_visible(
        context.camera_uuid, context.user.id, presets,
        EPTZ_PRESET_MAX_VISIBLE);
    cJSON *root = cJSON_CreateObject();
    cJSON *items = root ? cJSON_AddArrayToObject(root, "presets") : NULL;
    if (count < 0 || !root || !items) {
        free(presets);
        cJSON_Delete(root);
        http_response_set_json_error(res, 500, "Failed to list ePTZ presets");
        return;
    }
    for (int index = 0; index < count; index++) {
        cJSON *item = preset_json(&presets[index], context.user.id);
        if (!item) continue;
        cJSON_AddItemToArray(items, item);
    }
    free(presets);
    cJSON_AddNumberToObject(root, "count", count);
    cJSON_AddBoolToObject(root, "can_share",
                          context.user.role == USER_ROLE_ADMIN);
    cJSON_AddBoolToObject(root, "can_modify",
                          !read_only_identity(&context.user));
    send_json(res, 200, root);
    cJSON_Delete(root);
}

void handle_post_eptz_preset(const http_request_t *req,
                             http_response_t *res) {
    eptz_request_context_t context;
    if (!prepare_request(req, res, false, &context)) return;
    if (read_only_identity(&context.user)) {
        http_response_set_json_error(res, 403,
                                     "ePTZ presets require an interactive user");
        return;
    }
    cJSON *body = httpd_parse_json_body(req);
    eptz_operator_preset_t preset;
    bool valid = parse_preset(body, &context, &preset, res);
    cJSON_Delete(body);
    if (!valid) return;
    if (preset.is_shared && context.user.role != USER_ROLE_ADMIN) {
        http_response_set_json_error(res, 403,
                                     "Only administrators can share presets");
        return;
    }
    db_eptz_preset_result_t result = db_eptz_preset_create(&preset);
    if (result != DB_EPTZ_PRESET_OK) {
        set_db_error(res, result);
        return;
    }
    cJSON *root = preset_json(&preset, context.user.id);
    send_json(res, 201, root);
    cJSON_Delete(root);
}

void handle_put_eptz_preset(const http_request_t *req,
                            http_response_t *res) {
    eptz_request_context_t context;
    eptz_operator_preset_t existing;
    if (!prepare_request(req, res, true, &context)) return;
    if (read_only_identity(&context.user)) {
        http_response_set_json_error(res, 403,
                                     "ePTZ presets require an interactive user");
        return;
    }
    db_eptz_preset_result_t result = db_eptz_preset_get_visible(
        context.camera_uuid, context.user.id, context.preset_uuid, &existing);
    if (result != DB_EPTZ_PRESET_OK) {
        set_db_error(res, result);
        return;
    }
    if (existing.owner_user_id != context.user.id) {
        http_response_set_json_error(res, 403,
                                     "Only the owner can edit this preset");
        return;
    }
    cJSON *body = httpd_parse_json_body(req);
    eptz_operator_preset_t preset;
    int64_t revision = 0;
    bool valid = parse_preset(body, &context, &preset, res) &&
        parse_revision(body, &revision);
    cJSON_Delete(body);
    if (!valid) {
        if (res->status_code == 0) {
            http_response_set_json_error(res, 400, "revision is required");
        }
        return;
    }
    if (preset.is_shared && context.user.role != USER_ROLE_ADMIN) {
        http_response_set_json_error(res, 403,
                                     "Only administrators can share presets");
        return;
    }
    safe_strcpy(preset.uuid, context.preset_uuid,
                sizeof(preset.uuid), 0);
    result = db_eptz_preset_update(&preset, revision);
    if (result != DB_EPTZ_PRESET_OK) {
        set_db_error(res, result);
        return;
    }
    cJSON *root = preset_json(&preset, context.user.id);
    send_json(res, 200, root);
    cJSON_Delete(root);
}

void handle_delete_eptz_preset(const http_request_t *req,
                               http_response_t *res) {
    eptz_request_context_t context;
    eptz_operator_preset_t existing;
    if (!prepare_request(req, res, true, &context)) return;
    if (read_only_identity(&context.user)) {
        http_response_set_json_error(res, 403,
                                     "ePTZ presets require an interactive user");
        return;
    }
    db_eptz_preset_result_t result = db_eptz_preset_get_visible(
        context.camera_uuid, context.user.id, context.preset_uuid, &existing);
    if (result != DB_EPTZ_PRESET_OK) {
        set_db_error(res, result);
        return;
    }
    if (existing.owner_user_id != context.user.id) {
        http_response_set_json_error(res, 403,
                                     "Only the owner can delete this preset");
        return;
    }
    cJSON *body = httpd_parse_json_body(req);
    int64_t revision = 0;
    bool valid = cJSON_IsObject(body) && parse_revision(body, &revision);
    cJSON_Delete(body);
    if (!valid) {
        http_response_set_json_error(res, 400, "revision is required");
        return;
    }
    result = db_eptz_preset_delete(
        context.camera_uuid, context.user.id, context.preset_uuid, revision);
    if (result != DB_EPTZ_PRESET_OK) {
        set_db_error(res, result);
        return;
    }
    http_response_set_json(res, 200, "{\"success\":true}");
}
