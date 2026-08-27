#define _POSIX_C_SOURCE 200809L

#include "web/api_handlers_live_layouts.h"

#include <cjson/cJSON.h>
#include <math.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "core/authorization.h"
#include "database/db_auth.h"
#include "database/db_fleet_query.h"
#include "database/db_live_saved_layouts.h"
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

static bool copy_string(const cJSON *body, const char *key,
                        char *destination, size_t size, bool required) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(body, key);
    if (!item) {
        if (required) return false;
        destination[0] = '\0';
        return true;
    }
    if (!cJSON_IsString(item) || !item->valuestring ||
        (required && item->valuestring[0] == '\0') ||
        strlen(item->valuestring) >= size) return false;
    safe_strcpy(destination, item->valuestring, size, 0);
    return true;
}

static bool parse_integer(const cJSON *body, const char *key, int minimum,
                          int maximum, int *value) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(body, key);
    if (!cJSON_IsNumber(item) || !isfinite(item->valuedouble) ||
        floor(item->valuedouble) != item->valuedouble ||
        item->valueint < minimum || item->valueint > maximum) return false;
    *value = item->valueint;
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

static bool camera_allowed(const char *camera_uuid,
                           const fleet_camera_t *cameras, int camera_count) {
    for (int index = 0; index < camera_count; index++) {
        if (strcmp(cameras[index].camera_uuid, camera_uuid) == 0) return true;
    }
    return false;
}

static bool load_authorized_cameras(const user_t *user,
                                    fleet_camera_t **cameras,
                                    int *camera_count) {
    *cameras = NULL;
    *camera_count = 0;
    return db_fleet_camera_load(cameras, camera_count) == 0 &&
        authorization_filter_cameras(user, AUTHZ_LIVE_VIEW,
                                     *cameras, camera_count) == 0;
}

static cJSON *authorized_slots(const fleet_camera_t *cameras,
                               int camera_count, const char *encoded,
                               bool *all_allowed) {
    cJSON *source = cJSON_Parse(encoded);
    cJSON *result = cJSON_CreateArray();
    *all_allowed = true;
    if (!cJSON_IsArray(source) || !result) {
        cJSON_Delete(source);
        cJSON_Delete(result);
        return NULL;
    }
    const cJSON *slot = NULL;
    cJSON_ArrayForEach(slot, source) {
        const cJSON *camera = cJSON_GetObjectItemCaseSensitive(
            slot, "camera_uuid");
        if (!cJSON_IsString(camera) || !camera_allowed(
                camera->valuestring, cameras, camera_count)) {
            *all_allowed = false;
            continue;
        }
        cJSON *copy = cJSON_Duplicate(slot, true);
        if (!copy) {
            cJSON_Delete(source);
            cJSON_Delete(result);
            return NULL;
        }
        cJSON_AddItemToArray(result, copy);
    }
    cJSON_Delete(source);
    return result;
}

static bool parse_layout(const cJSON *body, const user_t *user,
                         live_saved_layout_t *layout,
                         http_response_t *res) {
    memset(layout, 0, sizeof(*layout));
    layout->owner_user_id = user->id;
    if (!cJSON_IsObject(body) ||
        !copy_string(body, "name", layout->name,
                     sizeof(layout->name), true) ||
        !copy_string(body, "availability", layout->availability,
                     sizeof(layout->availability), true) ||
        !parse_integer(body, "columns", 1, 9, &layout->columns) ||
        !parse_integer(body, "rows", 1, 9, &layout->rows) ||
        layout->columns * layout->rows > 36) {
        http_response_set_json_error(res, 400, "Invalid live layout fields");
        return false;
    }
    const cJSON *location = cJSON_GetObjectItemCaseSensitive(
        body, "location_uuid");
    if (location && !cJSON_IsNull(location)) {
        if (!cJSON_IsString(location) || !location->valuestring ||
            !lightnvr_uuid_is_valid(location->valuestring)) {
            http_response_set_json_error(res, 400, "Invalid location_uuid");
            return false;
        }
        safe_strcpy(layout->location_uuid, location->valuestring,
                    sizeof(layout->location_uuid), 0);
    }
    const cJSON *shared = cJSON_GetObjectItemCaseSensitive(body, "is_shared");
    if (shared && !cJSON_IsBool(shared)) {
        http_response_set_json_error(res, 400, "is_shared must be boolean");
        return false;
    }
    layout->is_shared = shared && cJSON_IsTrue(shared);
    const cJSON *slots = cJSON_GetObjectItemCaseSensitive(body, "camera_slots");
    char *encoded = cJSON_IsArray(slots) ? cJSON_PrintUnformatted(slots) : NULL;
    if (!encoded || strlen(encoded) >= sizeof(layout->camera_slots_json)) {
        free(encoded);
        http_response_set_json_error(res, 400, "Invalid camera_slots");
        return false;
    }
    fleet_camera_t *cameras = NULL;
    int camera_count = 0;
    bool all_allowed = false;
    cJSON *filtered = load_authorized_cameras(
        user, &cameras, &camera_count)
        ? authorized_slots(cameras, camera_count, encoded, &all_allowed)
        : NULL;
    free(cameras);
    if (!filtered || !all_allowed ||
        cJSON_GetArraySize(filtered) != cJSON_GetArraySize(slots)) {
        cJSON_Delete(filtered);
        free(encoded);
        http_response_set_json_error(
            res, 403, "Layout contains an unavailable camera");
        return false;
    }
    cJSON_Delete(filtered);
    safe_strcpy(layout->camera_slots_json, encoded,
                sizeof(layout->camera_slots_json), 0);
    free(encoded);
    return true;
}

static cJSON *layout_json_with_inventory(
    const live_saved_layout_t *layout, const user_t *user,
    const fleet_camera_t *cameras, int camera_count) {
    bool all_allowed = false;
    cJSON *slots = authorized_slots(cameras, camera_count,
                                    layout->camera_slots_json, &all_allowed);
    cJSON *root = cJSON_CreateObject();
    if (!root || !slots) {
        cJSON_Delete(root);
        cJSON_Delete(slots);
        return NULL;
    }
    cJSON_AddStringToObject(root, "uuid", layout->uuid);
    cJSON_AddStringToObject(root, "name", layout->name);
    cJSON_AddBoolToObject(root, "is_shared", layout->is_shared);
    cJSON_AddBoolToObject(root, "owned",
                          layout->owner_user_id == user->id);
    if (layout->location_uuid[0]) {
        cJSON_AddStringToObject(root, "location_uuid", layout->location_uuid);
    } else {
        cJSON_AddNullToObject(root, "location_uuid");
    }
    cJSON_AddStringToObject(root, "availability", layout->availability);
    cJSON_AddNumberToObject(root, "columns", layout->columns);
    cJSON_AddNumberToObject(root, "rows", layout->rows);
    cJSON_AddItemToObject(root, "camera_slots", slots);
    cJSON_AddBoolToObject(root, "partially_authorized", !all_allowed);
    cJSON_AddNumberToObject(root, "revision", (double)layout->revision);
    cJSON_AddNumberToObject(root, "created_at", (double)layout->created_at);
    cJSON_AddNumberToObject(root, "updated_at", (double)layout->updated_at);
    return root;
}

static cJSON *layout_json(const live_saved_layout_t *layout,
                          const user_t *user) {
    fleet_camera_t *cameras = NULL;
    int camera_count = 0;
    if (!load_authorized_cameras(user, &cameras, &camera_count)) {
        free(cameras);
        return NULL;
    }
    cJSON *root = layout_json_with_inventory(
        layout, user, cameras, camera_count);
    free(cameras);
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
                         db_live_layout_result_t result) {
    switch (result) {
        case DB_LIVE_LAYOUT_NOT_FOUND:
            http_response_set_json_error(res, 404, "Live layout not found");
            break;
        case DB_LIVE_LAYOUT_INVALID:
            http_response_set_json_error(res, 400, "Invalid live layout");
            break;
        case DB_LIVE_LAYOUT_CONFLICT:
            http_response_set_json_error(
                res, 409, "A live layout with this name already exists");
            break;
        case DB_LIVE_LAYOUT_STALE:
            http_response_set_json_error(
                res, 409, "Live layout changed; reload and retry");
            break;
        case DB_LIVE_LAYOUT_LIMIT:
            http_response_set_json_error(res, 409, "Live layout limit reached");
            break;
        default:
            http_response_set_json_error(res, 500,
                                         "Live layout operation failed");
            break;
    }
}

static bool extract_uuid(const http_request_t *req, char *uuid,
                         http_response_t *res) {
    if (http_request_extract_path_param(req, "/api/live/layouts/", uuid,
                                        CAMERA_UUID_STRING_SIZE) != 0 ||
        !lightnvr_uuid_is_valid(uuid)) {
        http_response_set_json_error(res, 400, "Invalid live layout UUID");
        return false;
    }
    return true;
}

void handle_get_live_layouts(const http_request_t *req,
                             http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;
    live_saved_layout_t *layouts = calloc(
        LIVE_LAYOUT_MAX_VISIBLE, sizeof(*layouts));
    if (!layouts) {
        http_response_set_json_error(res, 500, "Out of memory");
        return;
    }
    int count = db_live_layout_list_visible(
        user.id, layouts, LIVE_LAYOUT_MAX_VISIBLE);
    fleet_camera_t *cameras = NULL;
    int camera_count = 0;
    cJSON *root = cJSON_CreateObject();
    cJSON *items = root ? cJSON_AddArrayToObject(root, "layouts") : NULL;
    if (count < 0 || !root || !items ||
        !load_authorized_cameras(&user, &cameras, &camera_count)) {
        free(layouts);
        free(cameras);
        cJSON_Delete(root);
        http_response_set_json_error(res, 500, "Failed to list live layouts");
        return;
    }
    for (int index = 0; index < count; index++) {
        cJSON *item = layout_json_with_inventory(
            &layouts[index], &user, cameras, camera_count);
        if (!item) {
            free(layouts);
            free(cameras);
            cJSON_Delete(root);
            http_response_set_json_error(res, 500,
                                         "Failed to create layout response");
            return;
        }
        cJSON_AddItemToArray(items, item);
    }
    free(layouts);
    free(cameras);
    cJSON_AddNumberToObject(root, "count", count);
    cJSON_AddBoolToObject(root, "can_share", user.role == USER_ROLE_ADMIN);
    cJSON_AddBoolToObject(root, "can_modify", !read_only_identity(&user));
    send_json(res, 200, root);
    cJSON_Delete(root);
}

void handle_post_live_layout(const http_request_t *req,
                             http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;
    if (read_only_identity(&user)) {
        http_response_set_json_error(res, 403,
                                     "Live layouts require an interactive user");
        return;
    }
    cJSON *body = httpd_parse_json_body(req);
    live_saved_layout_t layout;
    bool valid = parse_layout(body, &user, &layout, res);
    cJSON_Delete(body);
    if (!valid) return;
    if (layout.is_shared && user.role != USER_ROLE_ADMIN) {
        http_response_set_json_error(res, 403,
                                     "Only administrators can share layouts");
        return;
    }
    db_live_layout_result_t result = db_live_layout_create(&layout);
    if (result != DB_LIVE_LAYOUT_OK) {
        set_db_error(res, result);
        return;
    }
    cJSON *root = layout_json(&layout, &user);
    send_json(res, 201, root);
    cJSON_Delete(root);
}

void handle_put_live_layout(const http_request_t *req,
                            http_response_t *res) {
    user_t user;
    char uuid[CAMERA_UUID_STRING_SIZE] = {0};
    live_saved_layout_t existing;
    if (!authenticate(req, res, &user)) return;
    if (read_only_identity(&user)) {
        http_response_set_json_error(res, 403,
                                     "Live layouts require an interactive user");
        return;
    }
    if (!extract_uuid(req, uuid, res)) return;
    db_live_layout_result_t result = db_live_layout_get_visible(
        user.id, uuid, &existing);
    if (result != DB_LIVE_LAYOUT_OK) {
        set_db_error(res, result);
        return;
    }
    if (existing.owner_user_id != user.id) {
        http_response_set_json_error(res, 403,
                                     "Only the owner can edit this layout");
        return;
    }
    cJSON *body = httpd_parse_json_body(req);
    live_saved_layout_t layout;
    int64_t revision = 0;
    bool valid = parse_layout(body, &user, &layout, res) &&
        parse_revision(body, &revision);
    cJSON_Delete(body);
    if (!valid) {
        if (res->status_code == 0) {
            http_response_set_json_error(res, 400, "revision is required");
        }
        return;
    }
    if (layout.is_shared && user.role != USER_ROLE_ADMIN) {
        http_response_set_json_error(res, 403,
                                     "Only administrators can share layouts");
        return;
    }
    safe_strcpy(layout.uuid, uuid, sizeof(layout.uuid), 0);
    result = db_live_layout_update(&layout, revision);
    if (result != DB_LIVE_LAYOUT_OK) {
        set_db_error(res, result);
        return;
    }
    cJSON *root = layout_json(&layout, &user);
    send_json(res, 200, root);
    cJSON_Delete(root);
}

void handle_delete_live_layout(const http_request_t *req,
                               http_response_t *res) {
    user_t user;
    char uuid[CAMERA_UUID_STRING_SIZE] = {0};
    live_saved_layout_t existing;
    if (!authenticate(req, res, &user)) return;
    if (read_only_identity(&user)) {
        http_response_set_json_error(res, 403,
                                     "Live layouts require an interactive user");
        return;
    }
    if (!extract_uuid(req, uuid, res)) return;
    db_live_layout_result_t result = db_live_layout_get_visible(
        user.id, uuid, &existing);
    if (result != DB_LIVE_LAYOUT_OK) {
        set_db_error(res, result);
        return;
    }
    if (existing.owner_user_id != user.id) {
        http_response_set_json_error(res, 403,
                                     "Only the owner can delete this layout");
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
    result = db_live_layout_delete(user.id, uuid, revision);
    if (result != DB_LIVE_LAYOUT_OK) {
        set_db_error(res, result);
        return;
    }
    http_response_set_json(res, 200, "{\"success\":true}");
}
