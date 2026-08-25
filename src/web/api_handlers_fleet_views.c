#define _POSIX_C_SOURCE 200809L

#include "web/api_handlers_fleet_views.h"

#include <cjson/cJSON.h>
#include <math.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "core/camera_selector.h"
#include "database/db_auth.h"
#include "database/db_fleet_saved_views.h"
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

static bool demo_read_only(const user_t *user) {
    return user && strcmp(user->authentication_method, "demo") == 0;
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

static bool encode_json(const cJSON *value, char *destination, size_t size) {
    char *encoded = cJSON_PrintUnformatted(value);
    if (!encoded || strlen(encoded) >= size) {
        free(encoded);
        return false;
    }
    safe_strcpy(destination, encoded, size, 0);
    free(encoded);
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

static bool parse_view(const cJSON *body, int64_t owner_user_id,
                       fleet_saved_view_t *view, http_response_t *res) {
    memset(view, 0, sizeof(*view));
    view->owner_user_id = owner_user_id;
    safe_strcpy(view->sort_by, "name", sizeof(view->sort_by), 0);
    safe_strcpy(view->sort_order, "asc", sizeof(view->sort_order), 0);
    safe_strcpy(view->columns_json,
                "[\"camera\",\"health\",\"location\",\"tags\","
                "\"recording\",\"actions\"]",
                sizeof(view->columns_json), 0);
    if (!cJSON_IsObject(body) ||
        !copy_string(body, "name", view->name, sizeof(view->name), true) ||
        !copy_string(body, "search", view->search,
                     sizeof(view->search), false) ||
        !copy_string(body, "collection_uuid", view->collection_uuid,
                     sizeof(view->collection_uuid), false) ||
        (view->collection_uuid[0] &&
         !lightnvr_uuid_is_valid(view->collection_uuid))) {
        http_response_set_json_error(res, 400, "Invalid saved view fields");
        return false;
    }
    const cJSON *shared = cJSON_GetObjectItemCaseSensitive(body, "is_shared");
    if (shared && !cJSON_IsBool(shared)) {
        http_response_set_json_error(res, 400, "is_shared must be boolean");
        return false;
    }
    view->is_shared = shared && cJSON_IsTrue(shared);

    const cJSON *selector = cJSON_GetObjectItemCaseSensitive(body, "selector");
    char selector_error[FLEET_SELECTOR_ERROR_MAX] = {0};
    fleet_selector_t *parsed = cJSON_IsObject(selector)
        ? fleet_selector_parse(selector, selector_error,
                               sizeof(selector_error)) : NULL;
    if (!parsed || !encode_json(selector, view->selector_json,
                                sizeof(view->selector_json))) {
        fleet_selector_free(parsed);
        http_response_set_json_error(
            res, 400, selector_error[0] ? selector_error : "Invalid selector");
        return false;
    }
    fleet_selector_free(parsed);

    const cJSON *columns = cJSON_GetObjectItemCaseSensitive(body, "columns");
    if (columns && (!cJSON_IsArray(columns) ||
                    !encode_json(columns, view->columns_json,
                                 sizeof(view->columns_json)))) {
        http_response_set_json_error(res, 400, "Invalid columns");
        return false;
    }
    if (cJSON_GetObjectItemCaseSensitive(body, "sort_by") &&
        !copy_string(body, "sort_by", view->sort_by,
                     sizeof(view->sort_by), true)) {
        http_response_set_json_error(res, 400, "Invalid sort_by");
        return false;
    }
    if (cJSON_GetObjectItemCaseSensitive(body, "sort_order") &&
        !copy_string(body, "sort_order", view->sort_order,
                     sizeof(view->sort_order), true)) {
        http_response_set_json_error(res, 400, "Invalid sort_order");
        return false;
    }
    return true;
}

static cJSON *view_json(const fleet_saved_view_t *view,
                        int64_t current_user_id) {
    cJSON *root = cJSON_CreateObject();
    cJSON *selector = cJSON_Parse(view->selector_json);
    cJSON *columns = cJSON_Parse(view->columns_json);
    if (!root || !cJSON_IsObject(selector) || !cJSON_IsArray(columns)) {
        cJSON_Delete(root);
        cJSON_Delete(selector);
        cJSON_Delete(columns);
        return NULL;
    }
    cJSON_AddStringToObject(root, "uuid", view->uuid);
    cJSON_AddStringToObject(root, "name", view->name);
    cJSON_AddBoolToObject(root, "is_shared", view->is_shared);
    cJSON_AddBoolToObject(root, "owned", view->owner_user_id == current_user_id);
    cJSON_AddItemToObject(root, "selector", selector);
    cJSON_AddStringToObject(root, "search", view->search);
    if (view->collection_uuid[0]) {
        cJSON_AddStringToObject(root, "collection_uuid", view->collection_uuid);
    } else {
        cJSON_AddNullToObject(root, "collection_uuid");
    }
    cJSON_AddItemToObject(root, "columns", columns);
    cJSON_AddStringToObject(root, "sort_by", view->sort_by);
    cJSON_AddStringToObject(root, "sort_order", view->sort_order);
    cJSON_AddNumberToObject(root, "revision", (double)view->revision);
    cJSON_AddNumberToObject(root, "created_at", (double)view->created_at);
    cJSON_AddNumberToObject(root, "updated_at", (double)view->updated_at);
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
                         db_fleet_saved_view_result_t result) {
    switch (result) {
        case DB_FLEET_SAVED_VIEW_NOT_FOUND:
            http_response_set_json_error(res, 404, "Saved view not found");
            break;
        case DB_FLEET_SAVED_VIEW_INVALID:
            http_response_set_json_error(res, 400, "Invalid saved view");
            break;
        case DB_FLEET_SAVED_VIEW_CONFLICT:
            http_response_set_json_error(res, 409,
                                         "A view with this name already exists");
            break;
        case DB_FLEET_SAVED_VIEW_STALE:
            http_response_set_json_error(res, 409,
                                         "Saved view changed; reload and retry");
            break;
        case DB_FLEET_SAVED_VIEW_LIMIT:
            http_response_set_json_error(res, 409, "Saved view limit reached");
            break;
        default:
            http_response_set_json_error(res, 500,
                                         "Saved view operation failed");
            break;
    }
}

static bool extract_uuid(const http_request_t *req, char *uuid,
                         http_response_t *res) {
    if (http_request_extract_path_param(req, "/api/fleet/views/", uuid,
                                        CAMERA_UUID_STRING_SIZE) != 0 ||
        !lightnvr_uuid_is_valid(uuid)) {
        http_response_set_json_error(res, 400, "Invalid saved view UUID");
        return false;
    }
    return true;
}

void handle_get_fleet_saved_views(const http_request_t *req,
                                  http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;
    fleet_saved_view_t *views = calloc(FLEET_SAVED_VIEW_MAX_VISIBLE,
                                       sizeof(*views));
    if (!views) {
        http_response_set_json_error(res, 500, "Out of memory");
        return;
    }
    int count = db_fleet_saved_view_list_visible(
        user.id, views, FLEET_SAVED_VIEW_MAX_VISIBLE);
    cJSON *root = cJSON_CreateObject();
    cJSON *items = root ? cJSON_AddArrayToObject(root, "views") : NULL;
    if (count < 0 || !root || !items) {
        free(views);
        cJSON_Delete(root);
        http_response_set_json_error(res, 500, "Failed to list saved views");
        return;
    }
    for (int index = 0; index < count; index++) {
        cJSON *item = view_json(&views[index], user.id);
        if (!item) {
            free(views);
            cJSON_Delete(root);
            http_response_set_json_error(res, 500,
                                         "Failed to create saved view response");
            return;
        }
        cJSON_AddItemToArray(items, item);
    }
    free(views);
    cJSON_AddNumberToObject(root, "count", count);
    cJSON_AddBoolToObject(root, "can_share", user.role == USER_ROLE_ADMIN);
    send_json(res, 200, root);
    cJSON_Delete(root);
}

void handle_post_fleet_saved_view(const http_request_t *req,
                                  http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;
    if (demo_read_only(&user)) {
        http_response_set_json_error(res, 403, "Demo mode is read-only");
        return;
    }
    cJSON *body = httpd_parse_json_body(req);
    fleet_saved_view_t view;
    bool valid = parse_view(body, user.id, &view, res);
    cJSON_Delete(body);
    if (!valid) return;
    if (view.is_shared && user.role != USER_ROLE_ADMIN) {
        http_response_set_json_error(res, 403,
                                     "Only administrators can share views");
        return;
    }
    db_fleet_saved_view_result_t result = db_fleet_saved_view_create(&view);
    if (result != DB_FLEET_SAVED_VIEW_OK) {
        set_db_error(res, result);
        return;
    }
    cJSON *root = view_json(&view, user.id);
    send_json(res, 201, root);
    cJSON_Delete(root);
}

static bool load_visible_view(const user_t *user, const char *uuid,
                              fleet_saved_view_t *view,
                              http_response_t *res) {
    db_fleet_saved_view_result_t result =
        db_fleet_saved_view_get_visible(user->id, uuid, view);
    if (result != DB_FLEET_SAVED_VIEW_OK) {
        set_db_error(res, result);
        return false;
    }
    return true;
}

void handle_get_fleet_saved_view(const http_request_t *req,
                                 http_response_t *res) {
    user_t user;
    char uuid[CAMERA_UUID_STRING_SIZE] = {0};
    fleet_saved_view_t view;
    if (!authenticate(req, res, &user) || !extract_uuid(req, uuid, res) ||
        !load_visible_view(&user, uuid, &view, res)) return;
    cJSON *root = view_json(&view, user.id);
    send_json(res, 200, root);
    cJSON_Delete(root);
}

void handle_put_fleet_saved_view(const http_request_t *req,
                                 http_response_t *res) {
    user_t user;
    char uuid[CAMERA_UUID_STRING_SIZE] = {0};
    fleet_saved_view_t existing;
    if (!authenticate(req, res, &user)) return;
    if (demo_read_only(&user)) {
        http_response_set_json_error(res, 403, "Demo mode is read-only");
        return;
    }
    if (!extract_uuid(req, uuid, res) ||
        !load_visible_view(&user, uuid, &existing, res)) return;
    if (existing.owner_user_id != user.id) {
        http_response_set_json_error(res, 403,
                                     "Only the owner can edit this view");
        return;
    }
    cJSON *body = httpd_parse_json_body(req);
    fleet_saved_view_t view;
    int64_t revision = 0;
    bool valid = parse_view(body, user.id, &view, res) &&
        parse_revision(body, &revision);
    cJSON_Delete(body);
    if (!valid) {
        if (res->status_code == 0) {
            http_response_set_json_error(res, 400, "revision is required");
        }
        return;
    }
    if (view.is_shared && user.role != USER_ROLE_ADMIN) {
        http_response_set_json_error(res, 403,
                                     "Only administrators can share views");
        return;
    }
    safe_strcpy(view.uuid, uuid, sizeof(view.uuid), 0);
    db_fleet_saved_view_result_t result =
        db_fleet_saved_view_update(&view, revision);
    if (result != DB_FLEET_SAVED_VIEW_OK) {
        set_db_error(res, result);
        return;
    }
    cJSON *root = view_json(&view, user.id);
    send_json(res, 200, root);
    cJSON_Delete(root);
}

void handle_delete_fleet_saved_view(const http_request_t *req,
                                    http_response_t *res) {
    user_t user;
    char uuid[CAMERA_UUID_STRING_SIZE] = {0};
    fleet_saved_view_t view;
    if (!authenticate(req, res, &user)) return;
    if (demo_read_only(&user)) {
        http_response_set_json_error(res, 403, "Demo mode is read-only");
        return;
    }
    if (!extract_uuid(req, uuid, res) ||
        !load_visible_view(&user, uuid, &view, res)) return;
    if (view.owner_user_id != user.id) {
        http_response_set_json_error(res, 403,
                                     "Only the owner can delete this view");
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
    db_fleet_saved_view_result_t result = db_fleet_saved_view_delete(
        user.id, uuid, revision);
    if (result != DB_FLEET_SAVED_VIEW_OK) {
        set_db_error(res, result);
        return;
    }
    http_response_set_json(res, 200, "{\"success\":true}");
}
