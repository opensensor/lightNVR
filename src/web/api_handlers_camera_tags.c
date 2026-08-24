#define _POSIX_C_SOURCE 200809L

#include <cjson/cJSON.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "core/config.h"
#include "database/db_camera_tags.h"
#include "database/db_fleet_query.h"
#include "utils/strings.h"
#include "web/api_handlers_camera_tags.h"
#include "web/httpd_utils.h"
#include "web/request_response.h"

static bool valid_uuid_string(const char *uuid) {
    return uuid && strlen(uuid) == CAMERA_UUID_STRING_SIZE - 1;
}

static bool valid_color(const char *color) {
    if (!color || color[0] == '\0') return true;
    if (strlen(color) != 7 || color[0] != '#') return false;
    for (int i = 1; i < 7; i++) {
        if (!isxdigit((unsigned char)color[i])) return false;
    }
    return true;
}

static cJSON *tag_to_json(const camera_tag_t *tag) {
    cJSON *object = cJSON_CreateObject();
    if (!object) return NULL;
    cJSON_AddStringToObject(object, "uuid", tag->uuid);
    cJSON_AddStringToObject(object, "label", tag->label);
    cJSON_AddStringToObject(object, "color", tag->color);
    cJSON_AddStringToObject(object, "description", tag->description);
    cJSON_AddNumberToObject(object, "camera_count", tag->camera_count);
    cJSON_AddNumberToObject(object, "created_at", (double)tag->created_at);
    cJSON_AddNumberToObject(object, "updated_at", (double)tag->updated_at);
    return object;
}

static void set_tag_json(http_response_t *res, int status,
                         const camera_tag_t *tag) {
    cJSON *object = tag_to_json(tag);
    char *json = object ? cJSON_PrintUnformatted(object) : NULL;
    cJSON_Delete(object);
    if (!json) {
        http_response_set_json_error(res, 500, "Failed to serialize camera tag");
        return;
    }
    http_response_set_json(res, status, json);
    free(json);
}

static void set_db_error(http_response_t *res, db_camera_tag_result_t result) {
    switch (result) {
        case DB_CAMERA_TAG_NOT_FOUND:
            http_response_set_json_error(res, 404, "Camera or tag not found");
            break;
        case DB_CAMERA_TAG_CONFLICT:
            http_response_set_json_error(res, 409,
                                         "A tag with that label already exists");
            break;
        case DB_CAMERA_TAG_LIMIT:
            http_response_set_json_error(
                res, 409,
                "Assigned tag labels exceed the legacy compatibility limit");
            break;
        case DB_CAMERA_TAG_INVALID:
            http_response_set_json_error(res, 400, "Invalid camera tag request");
            break;
        default:
            http_response_set_json_error(res, 500,
                                         "Camera tag database operation failed");
            break;
    }
}

static bool apply_tag_fields(cJSON *body, camera_tag_t *tag, bool creating,
                             http_response_t *res) {
    if (!cJSON_IsObject(body)) {
        http_response_set_json_error(res, 400, "Request body must be an object");
        return false;
    }
    cJSON *label = cJSON_GetObjectItemCaseSensitive(body, "label");
    if (label) {
        if (!cJSON_IsString(label) || !label->valuestring ||
            label->valuestring[0] == '\0' ||
            strlen(label->valuestring) >= sizeof(tag->label)) {
            http_response_set_json_error(res, 400, "Invalid tag label");
            return false;
        }
        safe_strcpy(tag->label, label->valuestring, sizeof(tag->label), 0);
    } else if (creating) {
        http_response_set_json_error(res, 400, "Tag label is required");
        return false;
    }

    cJSON *color = cJSON_GetObjectItemCaseSensitive(body, "color");
    if (color) {
        if (!cJSON_IsString(color) || !color->valuestring ||
            strlen(color->valuestring) >= sizeof(tag->color) ||
            !valid_color(color->valuestring)) {
            http_response_set_json_error(
                res, 400, "color must be empty or a #RRGGBB value");
            return false;
        }
        safe_strcpy(tag->color, color->valuestring, sizeof(tag->color), 0);
    }

    cJSON *description =
        cJSON_GetObjectItemCaseSensitive(body, "description");
    if (description) {
        if (!cJSON_IsString(description) || !description->valuestring ||
            strlen(description->valuestring) >= sizeof(tag->description)) {
            http_response_set_json_error(res, 400, "Invalid tag description");
            return false;
        }
        safe_strcpy(tag->description, description->valuestring,
                    sizeof(tag->description), 0);
    }
    return true;
}

static bool extract_tag_uuid(const http_request_t *req, char *uuid,
                             size_t uuid_size, http_response_t *res) {
    char path_value[MAX_PATH_LENGTH];
    if (http_request_extract_path_param(req, "/api/camera-tags/", path_value,
                                        sizeof(path_value)) != 0) {
        http_response_set_json_error(res, 400, "Invalid camera tag path");
        return false;
    }
    char *slash = strchr(path_value, '/');
    if (slash) *slash = '\0';
    if (!valid_uuid_string(path_value) || strlen(path_value) >= uuid_size) {
        http_response_set_json_error(res, 400, "Invalid camera tag UUID");
        return false;
    }
    safe_strcpy(uuid, path_value, uuid_size, 0);
    return true;
}

static bool extract_camera_uuid(const http_request_t *req, char *uuid,
                                size_t uuid_size, http_response_t *res) {
    char path_value[MAX_PATH_LENGTH];
    if (http_request_extract_path_param(req, "/api/cameras/", path_value,
                                        sizeof(path_value)) != 0) {
        http_response_set_json_error(res, 400, "Invalid camera path");
        return false;
    }
    char *slash = strchr(path_value, '/');
    if (slash) *slash = '\0';
    if (!valid_uuid_string(path_value) || strlen(path_value) >= uuid_size) {
        http_response_set_json_error(res, 400, "Invalid camera UUID");
        return false;
    }
    safe_strcpy(uuid, path_value, uuid_size, 0);
    return true;
}

static bool load_visible_cameras(const http_request_t *req,
                                 http_response_t *res,
                                 fleet_camera_t **cameras, int *count,
                                 bool *all_fleet) {
    user_t user;
    memset(&user, 0, sizeof(user));
    *cameras = NULL;
    *count = 0;
    *all_fleet = false;
    if (!httpd_check_action_access(req, &user)) {
        http_response_set_json_error(res, 401, "Unauthorized");
        return false;
    }
    authorization_evaluation_t evaluation;
    *all_fleet =
        (authorization_evaluate(&user, AUTHZ_LIVE_VIEW, NULL, &evaluation) == 0 &&
         evaluation.decision == AUTHZ_DECISION_ALLOW) ||
        (authorization_evaluate(&user, AUTHZ_CAMERA_CONFIGURE, NULL,
                                &evaluation) == 0 &&
         evaluation.decision == AUTHZ_DECISION_ALLOW);
    if (db_fleet_camera_load(cameras, count) != 0 ||
        authorization_filter_cameras(&user, AUTHZ_LIVE_VIEW, *cameras,
                                     count) != 0) {
        free(*cameras);
        *cameras = NULL;
        *count = 0;
        http_response_set_json_error(
            res, 500, "Authorization policy evaluation failed");
        return false;
    }
    return true;
}

static int visible_tag_camera_count(const fleet_camera_t *cameras,
                                    int camera_count, const char *tag_uuid) {
    int count = 0;
    for (int i = 0; i < camera_count; i++) {
        for (int j = 0; j < cameras[i].tag_count; j++) {
            if (strcmp(cameras[i].tags[j].uuid, tag_uuid) == 0) {
                count++;
                break;
            }
        }
    }
    return count;
}

void handle_get_camera_tags(const http_request_t *req, http_response_t *res) {
    fleet_camera_t *cameras = NULL;
    int camera_count = 0;
    bool all_fleet = false;
    if (!load_visible_cameras(req, res, &cameras, &camera_count, &all_fleet)) return;
    int total = db_camera_tag_count();
    if (total < 0) {
        free(cameras);
        http_response_set_json_error(res, 500, "Failed to count camera tags");
        return;
    }

    camera_tag_t *tags = total > 0 ? calloc((size_t)total, sizeof(*tags)) : NULL;
    if (total > 0 && !tags) {
        free(cameras);
        http_response_set_json_error(res, 500, "Out of memory");
        return;
    }
    int count = total > 0 ? db_camera_tag_list(tags, total) : 0;
    if (count < 0) {
        free(cameras);
        free(tags);
        http_response_set_json_error(res, 500, "Failed to list camera tags");
        return;
    }

    cJSON *root = cJSON_CreateObject();
    cJSON *items = cJSON_CreateArray();
    if (!root || !items) {
        cJSON_Delete(root);
        cJSON_Delete(items);
        free(cameras);
        free(tags);
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    cJSON_AddItemToObject(root, "tags", items);
    int visible_count = 0;
    for (int i = 0; i < count; i++) {
        int scoped_count = all_fleet ? tags[i].camera_count :
            visible_tag_camera_count(cameras, camera_count, tags[i].uuid);
        if (!all_fleet && scoped_count == 0) continue;
        camera_tag_t filtered = tags[i];
        filtered.camera_count = scoped_count;
        cJSON *item = tag_to_json(&filtered);
        if (!item) {
            cJSON_Delete(root);
            free(tags);
            free(cameras);
            http_response_set_json_error(res, 500, "Failed to create response");
            return;
        }
        cJSON_AddItemToArray(items, item);
        visible_count++;
    }
    cJSON_AddNumberToObject(root, "count", visible_count);
    free(tags);
    free(cameras);
    char *json = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if (!json) {
        http_response_set_json_error(res, 500, "Failed to serialize response");
        return;
    }
    http_response_set_json(res, 200, json);
    free(json);
}

void handle_post_camera_tag(const http_request_t *req, http_response_t *res) {
    if (!httpd_authorize_global_action(req, res, AUTHZ_CAMERA_CONFIGURE)) return;
    cJSON *body = httpd_parse_json_body(req);
    if (!body) {
        http_response_set_json_error(res, 400, "Invalid JSON request body");
        return;
    }
    camera_tag_t tag;
    memset(&tag, 0, sizeof(tag));
    if (!apply_tag_fields(body, &tag, true, res)) {
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);
    db_camera_tag_result_t result = db_camera_tag_create(&tag);
    if (result != DB_CAMERA_TAG_OK) {
        set_db_error(res, result);
        return;
    }
    set_tag_json(res, 201, &tag);
}

void handle_get_camera_tag(const http_request_t *req, http_response_t *res) {
    char uuid[CAMERA_UUID_STRING_SIZE];
    if (!extract_tag_uuid(req, uuid, sizeof(uuid), res)) return;
    fleet_camera_t *cameras = NULL;
    int camera_count = 0;
    bool all_fleet = false;
    if (!load_visible_cameras(req, res, &cameras, &camera_count, &all_fleet)) return;
    int scoped_count = visible_tag_camera_count(cameras, camera_count, uuid);
    free(cameras);
    if (!all_fleet && scoped_count == 0) {
        http_response_set_json_error(res, 403, "Forbidden");
        return;
    }
    camera_tag_t tag;
    db_camera_tag_result_t result = db_camera_tag_get(uuid, &tag);
    if (result != DB_CAMERA_TAG_OK) {
        set_db_error(res, result);
        return;
    }
    if (!all_fleet) tag.camera_count = scoped_count;
    set_tag_json(res, 200, &tag);
}

void handle_put_camera_tag(const http_request_t *req, http_response_t *res) {
    if (!httpd_authorize_global_action(req, res, AUTHZ_CAMERA_CONFIGURE)) return;
    char uuid[CAMERA_UUID_STRING_SIZE];
    if (!extract_tag_uuid(req, uuid, sizeof(uuid), res)) return;
    camera_tag_t tag;
    db_camera_tag_result_t result = db_camera_tag_get(uuid, &tag);
    if (result != DB_CAMERA_TAG_OK) {
        set_db_error(res, result);
        return;
    }
    cJSON *body = httpd_parse_json_body(req);
    if (!body) {
        http_response_set_json_error(res, 400, "Invalid JSON request body");
        return;
    }
    if (!apply_tag_fields(body, &tag, false, res)) {
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);
    result = db_camera_tag_update(&tag);
    if (result != DB_CAMERA_TAG_OK) {
        set_db_error(res, result);
        return;
    }
    set_tag_json(res, 200, &tag);
}

void handle_delete_camera_tag(const http_request_t *req, http_response_t *res) {
    if (!httpd_authorize_global_action(req, res, AUTHZ_CAMERA_CONFIGURE)) return;
    char uuid[CAMERA_UUID_STRING_SIZE];
    if (!extract_tag_uuid(req, uuid, sizeof(uuid), res)) return;
    db_camera_tag_result_t result = db_camera_tag_delete(uuid);
    if (result != DB_CAMERA_TAG_OK) {
        set_db_error(res, result);
        return;
    }
    http_response_set_json(res, 200, "{\"success\":true}");
}

void handle_post_camera_tag_merge(const http_request_t *req,
                                  http_response_t *res) {
    if (!httpd_authorize_global_action(req, res, AUTHZ_CAMERA_CONFIGURE)) return;
    char source_uuid[CAMERA_UUID_STRING_SIZE];
    if (!extract_tag_uuid(req, source_uuid, sizeof(source_uuid), res)) return;
    cJSON *body = httpd_parse_json_body(req);
    cJSON *target = body ?
        cJSON_GetObjectItemCaseSensitive(body, "target_uuid") : NULL;
    if (!body || !cJSON_IsObject(body) || !cJSON_IsString(target) ||
        !valid_uuid_string(target->valuestring)) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400, "A valid target_uuid is required");
        return;
    }
    char target_uuid[CAMERA_UUID_STRING_SIZE];
    safe_strcpy(target_uuid, target->valuestring, sizeof(target_uuid), 0);
    cJSON_Delete(body);
    db_camera_tag_result_t result =
        db_camera_tag_merge(source_uuid, target_uuid);
    if (result != DB_CAMERA_TAG_OK) {
        set_db_error(res, result);
        return;
    }
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "source_uuid", source_uuid);
    cJSON_AddStringToObject(response, "target_uuid", target_uuid);
    char *json = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);
    if (!json) {
        http_response_set_json_error(res, 500, "Failed to serialize response");
        return;
    }
    http_response_set_json(res, 200, json);
    free(json);
}

static void set_camera_assignments_json(http_response_t *res,
                                        const char *camera_uuid) {
    camera_tag_t tags[CAMERA_TAG_MAX_ASSIGNMENTS];
    int count = db_camera_tag_list_for_camera(
        camera_uuid, tags, CAMERA_TAG_MAX_ASSIGNMENTS);
    if (count == -2) {
        http_response_set_json_error(res, 404, "Camera not found");
        return;
    }
    if (count < 0) {
        http_response_set_json_error(res, 500, "Failed to list camera tags");
        return;
    }
    cJSON *root = cJSON_CreateObject();
    cJSON *items = cJSON_CreateArray();
    if (!root || !items) {
        cJSON_Delete(root);
        cJSON_Delete(items);
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    cJSON_AddStringToObject(root, "camera_uuid", camera_uuid);
    cJSON_AddItemToObject(root, "tags", items);
    cJSON_AddNumberToObject(root, "count", count);
    for (int i = 0; i < count; i++) {
        cJSON *item = tag_to_json(&tags[i]);
        if (!item) {
            cJSON_Delete(root);
            http_response_set_json_error(res, 500, "Failed to create response");
            return;
        }
        cJSON_AddItemToArray(items, item);
    }
    char *json = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if (!json) {
        http_response_set_json_error(res, 500, "Failed to serialize response");
        return;
    }
    http_response_set_json(res, 200, json);
    free(json);
}

void handle_get_camera_tag_assignments(const http_request_t *req,
                                       http_response_t *res) {
    char camera_uuid[CAMERA_UUID_STRING_SIZE];
    if (!extract_camera_uuid(req, camera_uuid, sizeof(camera_uuid), res)) return;
    if (!httpd_authorize_camera_identity_action_with_context(
            req, res, AUTHZ_LIVE_VIEW, camera_uuid, NULL, &(user_t){0},
            &(fleet_camera_t){0}, &(authorization_evaluation_t){0})) return;
    set_camera_assignments_json(res, camera_uuid);
}

void handle_put_camera_tag_assignments(const http_request_t *req,
                                       http_response_t *res) {
    char camera_uuid[CAMERA_UUID_STRING_SIZE];
    if (!extract_camera_uuid(req, camera_uuid, sizeof(camera_uuid), res)) return;
    if (!httpd_authorize_camera_identity_action_with_context(
            req, res, AUTHZ_CAMERA_CONFIGURE, camera_uuid, NULL, &(user_t){0},
            &(fleet_camera_t){0}, &(authorization_evaluation_t){0})) return;
    cJSON *body = httpd_parse_json_body(req);
    cJSON *items = body ?
        cJSON_GetObjectItemCaseSensitive(body, "tag_uuids") : NULL;
    if (!body || !cJSON_IsObject(body) || !cJSON_IsArray(items)) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400, "tag_uuids must be an array");
        return;
    }
    int count = cJSON_GetArraySize(items);
    if (count < 0 || count > CAMERA_TAG_MAX_ASSIGNMENTS) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400, "Too many camera tags");
        return;
    }
    char tag_storage[CAMERA_TAG_MAX_ASSIGNMENTS][CAMERA_UUID_STRING_SIZE];
    const char *tag_uuids[CAMERA_TAG_MAX_ASSIGNMENTS];
    for (int i = 0; i < count; i++) {
        cJSON *item = cJSON_GetArrayItem(items, i);
        if (!cJSON_IsString(item) || !valid_uuid_string(item->valuestring)) {
            cJSON_Delete(body);
            http_response_set_json_error(res, 400,
                                         "tag_uuids contains an invalid UUID");
            return;
        }
        safe_strcpy(tag_storage[i], item->valuestring,
                    sizeof(tag_storage[i]), 0);
        tag_uuids[i] = tag_storage[i];
    }
    cJSON_Delete(body);
    db_camera_tag_result_t result =
        db_camera_tag_set_for_camera(camera_uuid, tag_uuids, count);
    if (result != DB_CAMERA_TAG_OK) {
        set_db_error(res, result);
        return;
    }
    set_camera_assignments_json(res, camera_uuid);
}
