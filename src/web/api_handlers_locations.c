#define _POSIX_C_SOURCE 200809L

#include <cjson/cJSON.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "core/config.h"
#include "database/db_locations.h"
#include "database/db_fleet_query.h"
#include "utils/strings.h"
#include "web/api_handlers_locations.h"
#include "web/httpd_utils.h"
#include "web/request_response.h"

static bool valid_uuid_string(const char *uuid) {
    return uuid && strlen(uuid) == CAMERA_UUID_STRING_SIZE - 1;
}

static cJSON *location_to_json(const camera_location_t *location) {
    cJSON *object = cJSON_CreateObject();
    if (!object) return NULL;

    cJSON_AddStringToObject(object, "uuid", location->uuid);
    if (location->parent_uuid[0]) {
        cJSON_AddStringToObject(object, "parent_uuid", location->parent_uuid);
    } else {
        cJSON_AddNullToObject(object, "parent_uuid");
    }
    cJSON_AddStringToObject(object, "name", location->name);
    cJSON_AddStringToObject(object, "type", location->type);
    cJSON_AddNumberToObject(object, "sort_order", location->sort_order);
    cJSON_AddBoolToObject(object, "is_system", location->is_system != 0);

    cJSON *metadata = cJSON_Parse(location->metadata_json);
    if (!metadata || !cJSON_IsObject(metadata)) {
        cJSON_Delete(metadata);
        metadata = cJSON_CreateObject();
    }
    cJSON_AddItemToObject(object, "metadata", metadata);
    cJSON_AddNumberToObject(object, "child_count",
                           location->direct_child_count);
    cJSON_AddNumberToObject(object, "camera_count",
                           location->direct_camera_count);
    cJSON_AddNumberToObject(object, "created_at", (double)location->created_at);
    cJSON_AddNumberToObject(object, "updated_at", (double)location->updated_at);
    return object;
}

static void set_location_json(http_response_t *res, int status,
                              const camera_location_t *location) {
    cJSON *object = location_to_json(location);
    char *json = object ? cJSON_PrintUnformatted(object) : NULL;
    cJSON_Delete(object);
    if (!json) {
        http_response_set_json_error(res, 500, "Failed to serialize location");
        return;
    }
    http_response_set_json(res, status, json);
    free(json);
}

static void set_db_error(http_response_t *res, db_location_result_t result,
                         const char *conflict_message) {
    switch (result) {
        case DB_LOCATION_NOT_FOUND:
            http_response_set_json_error(res, 404, "Location or camera not found");
            break;
        case DB_LOCATION_CONFLICT:
            http_response_set_json_error(res, 409, conflict_message);
            break;
        case DB_LOCATION_INVALID:
            http_response_set_json_error(res, 400,
                                         "Invalid or immutable location");
            break;
        default:
            http_response_set_json_error(res, 500,
                                         "Location database operation failed");
            break;
    }
}

static bool copy_json_string(cJSON *root, const char *field, char *destination,
                             size_t destination_size, bool required,
                             http_response_t *res) {
    cJSON *item = cJSON_GetObjectItemCaseSensitive(root, field);
    if (!item) {
        if (required) {
            http_response_set_json_error(res, 400, "Missing required field");
            return false;
        }
        return true;
    }
    if (!cJSON_IsString(item) || !item->valuestring ||
        item->valuestring[0] == '\0' ||
        strlen(item->valuestring) >= destination_size) {
        http_response_set_json_error(res, 400, "Invalid string field");
        return false;
    }
    safe_strcpy(destination, item->valuestring, destination_size, 0);
    return true;
}

static bool apply_parent_field(cJSON *root, camera_location_t *location,
                               http_response_t *res) {
    cJSON *parent = cJSON_GetObjectItemCaseSensitive(root, "parent_uuid");
    if (!parent) return true;
    if (cJSON_IsNull(parent)) {
        location->parent_uuid[0] = '\0';
        return true;
    }
    if (!cJSON_IsString(parent) || !valid_uuid_string(parent->valuestring)) {
        http_response_set_json_error(res, 400, "Invalid parent_uuid");
        return false;
    }
    safe_strcpy(location->parent_uuid, parent->valuestring,
                sizeof(location->parent_uuid), 0);
    return true;
}

static bool apply_metadata_field(cJSON *root, camera_location_t *location,
                                 http_response_t *res) {
    cJSON *metadata = cJSON_GetObjectItemCaseSensitive(root, "metadata");
    if (!metadata) return true;
    if (!cJSON_IsObject(metadata)) {
        http_response_set_json_error(res, 400, "metadata must be an object");
        return false;
    }
    char *serialized = cJSON_PrintUnformatted(metadata);
    if (!serialized || strlen(serialized) >= sizeof(location->metadata_json)) {
        free(serialized);
        http_response_set_json_error(res, 400, "metadata is too large");
        return false;
    }
    safe_strcpy(location->metadata_json, serialized,
                sizeof(location->metadata_json), 0);
    free(serialized);
    return true;
}

static bool apply_location_fields(cJSON *root, camera_location_t *location,
                                  bool creating, http_response_t *res) {
    if (!cJSON_IsObject(root)) {
        http_response_set_json_error(res, 400, "Request body must be an object");
        return false;
    }
    if (!copy_json_string(root, "name", location->name,
                          sizeof(location->name), creating, res) ||
        !copy_json_string(root, "type", location->type,
                          sizeof(location->type), false, res) ||
        !apply_parent_field(root, location, res) ||
        !apply_metadata_field(root, location, res)) {
        return false;
    }

    cJSON *sort_order = cJSON_GetObjectItemCaseSensitive(root, "sort_order");
    if (sort_order) {
        if (!cJSON_IsNumber(sort_order)) {
            http_response_set_json_error(res, 400,
                                         "sort_order must be a number");
            return false;
        }
        location->sort_order = sort_order->valueint;
    }
    return true;
}

static bool extract_location_uuid(const http_request_t *req, char *uuid,
                                  size_t uuid_size, http_response_t *res) {
    if (http_request_extract_path_param(req, "/api/locations/", uuid,
                                        uuid_size) != 0) {
        http_response_set_json_error(res, 400, "Invalid location path");
        return false;
    }
    char *slash = strchr(uuid, '/');
    if (slash) *slash = '\0';
    if (!valid_uuid_string(uuid)) {
        http_response_set_json_error(res, 400, "Invalid location UUID");
        return false;
    }
    return true;
}

static bool load_visible_cameras(const http_request_t *req,
                                 http_response_t *res, user_t *user,
                                 fleet_camera_t **cameras, int *count,
                                 bool *all_fleet) {
    memset(user, 0, sizeof(*user));
    *cameras = NULL;
    *count = 0;
    *all_fleet = false;
    if (!httpd_check_action_access(req, user)) {
        http_response_set_json_error(res, 401, "Unauthorized");
        return false;
    }
    authorization_evaluation_t evaluation;
    *all_fleet =
        (authorization_evaluate(user, AUTHZ_LIVE_VIEW, NULL, &evaluation) == 0 &&
         evaluation.decision == AUTHZ_DECISION_ALLOW) ||
        (authorization_evaluate(user, AUTHZ_CAMERA_CONFIGURE, NULL,
                                &evaluation) == 0 &&
         evaluation.decision == AUTHZ_DECISION_ALLOW);
    if (db_fleet_camera_load(cameras, count) != 0 ||
        authorization_filter_cameras(user, AUTHZ_LIVE_VIEW, *cameras,
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

static bool camera_reaches_location(const fleet_camera_t *camera,
                                    const char *location_uuid) {
    if (strcmp(camera->location_uuid, location_uuid) == 0) return true;
    for (int i = 0; i < camera->location_depth; i++) {
        if (strcmp(camera->location_ancestor_uuids[i], location_uuid) == 0)
            return true;
    }
    return false;
}

static int visible_location_camera_count(const fleet_camera_t *cameras,
                                         int camera_count,
                                         const char *location_uuid) {
    int count = 0;
    for (int i = 0; i < camera_count; i++) {
        if (strcmp(cameras[i].location_uuid, location_uuid) == 0) count++;
    }
    return count;
}

static int visible_direct_child_count(const fleet_camera_t *cameras,
                                      int camera_count,
                                      const char *location_uuid) {
    int total = db_location_count();
    if (total < 0) return -1;
    camera_location_t *locations = total > 0
        ? calloc((size_t)total, sizeof(*locations)) : NULL;
    if (total > 0 && !locations) return -1;
    int count = total > 0 ? db_location_list(locations, total) : 0;
    if (count < 0) {
        free(locations);
        return -1;
    }
    int visible_children = 0;
    for (int child = 0; child < count; child++) {
        if (strcmp(locations[child].parent_uuid, location_uuid) != 0) continue;
        for (int camera_index = 0; camera_index < camera_count;
             camera_index++) {
            if (camera_reaches_location(&cameras[camera_index],
                                        locations[child].uuid)) {
                visible_children++;
                break;
            }
        }
    }
    free(locations);
    return visible_children;
}

void handle_get_locations(const http_request_t *req, http_response_t *res) {
    user_t user;
    fleet_camera_t *cameras = NULL;
    int camera_count = 0;
    bool all_fleet = false;
    if (!load_visible_cameras(req, res, &user, &cameras, &camera_count,
                              &all_fleet)) return;

    int total = db_location_count();
    if (total < 0) {
        free(cameras);
        http_response_set_json_error(res, 500, "Failed to count locations");
        return;
    }
    camera_location_t *locations = total > 0
        ? calloc((size_t)total, sizeof(*locations)) : NULL;
    if (total > 0 && !locations) {
        free(cameras);
        http_response_set_json_error(res, 500, "Out of memory");
        return;
    }
    int count = total > 0 ? db_location_list(locations, total) : 0;
    if (count < 0) {
        free(cameras);
        free(locations);
        http_response_set_json_error(res, 500, "Failed to list locations");
        return;
    }

    cJSON *root = cJSON_CreateObject();
    cJSON *items = cJSON_CreateArray();
    if (!root || !items) {
        cJSON_Delete(root);
        cJSON_Delete(items);
        free(locations);
        free(cameras);
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    cJSON_AddItemToObject(root, "locations", items);
    int visible_count = 0;
    for (int i = 0; i < count; i++) {
        bool visible = all_fleet;
        for (int camera_index = 0; camera_index < camera_count;
             camera_index++) {
            if (camera_reaches_location(&cameras[camera_index],
                                        locations[i].uuid)) {
                visible = true;
                break;
            }
        }
        if (!visible) continue;
        camera_location_t filtered = locations[i];
        if (!all_fleet) {
            filtered.direct_camera_count = visible_location_camera_count(
                cameras, camera_count, filtered.uuid);
            filtered.direct_child_count = 0;
            for (int child = 0; child < count; child++) {
                if (strcmp(locations[child].parent_uuid, filtered.uuid) != 0)
                    continue;
                for (int camera_index = 0; camera_index < camera_count;
                     camera_index++) {
                    if (camera_reaches_location(&cameras[camera_index],
                                                locations[child].uuid)) {
                        filtered.direct_child_count++;
                        break;
                    }
                }
            }
        }
        cJSON *item = location_to_json(&filtered);
        if (!item) {
            cJSON_Delete(root);
            free(locations);
            free(cameras);
            http_response_set_json_error(res, 500,
                                         "Failed to create location response");
            return;
        }
        cJSON_AddItemToArray(items, item);
        visible_count++;
        if (locations[i].is_system) {
            cJSON_AddStringToObject(root, "unassigned_uuid", locations[i].uuid);
        }
    }
    cJSON_AddNumberToObject(root, "count", visible_count);
    free(locations);
    free(cameras);

    char *json = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if (!json) {
        http_response_set_json_error(res, 500, "Failed to serialize locations");
        return;
    }
    http_response_set_json(res, 200, json);
    free(json);
}

void handle_post_location(const http_request_t *req, http_response_t *res) {
    if (!httpd_authorize_global_action(req, res, AUTHZ_CAMERA_CONFIGURE)) return;
    cJSON *body = httpd_parse_json_body(req);
    if (!body) {
        http_response_set_json_error(res, 400, "Invalid JSON request body");
        return;
    }

    camera_location_t location;
    memset(&location, 0, sizeof(location));
    safe_strcpy(location.type, "area", sizeof(location.type), 0);
    safe_strcpy(location.metadata_json, "{}", sizeof(location.metadata_json), 0);
    if (!apply_location_fields(body, &location, true, res)) {
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);

    db_location_result_t result = db_location_create(&location);
    if (result != DB_LOCATION_OK) {
        set_db_error(res, result,
                     "A location with that name already exists under this parent");
        return;
    }
    set_location_json(res, 201, &location);
}

void handle_get_location(const http_request_t *req, http_response_t *res) {
    char uuid[CAMERA_UUID_STRING_SIZE];
    if (!extract_location_uuid(req, uuid, sizeof(uuid), res)) return;

    user_t user;
    fleet_camera_t *cameras = NULL;
    int camera_count = 0;
    bool all_fleet = false;
    if (!load_visible_cameras(req, res, &user, &cameras, &camera_count,
                              &all_fleet)) return;
    bool visible = all_fleet;
    for (int i = 0; i < camera_count; i++) {
        if (camera_reaches_location(&cameras[i], uuid)) {
            visible = true;
            break;
        }
    }
    if (!visible) {
        free(cameras);
        http_response_set_json_error(res, 403, "Forbidden");
        return;
    }

    camera_location_t location;
    db_location_result_t result = db_location_get(uuid, &location);
    if (result != DB_LOCATION_OK) {
        free(cameras);
        set_db_error(res, result, "Location conflict");
        return;
    }
    if (!all_fleet) {
        location.direct_camera_count = visible_location_camera_count(
            cameras, camera_count, location.uuid);
        location.direct_child_count = visible_direct_child_count(
            cameras, camera_count, location.uuid);
        if (location.direct_child_count < 0) {
            free(cameras);
            http_response_set_json_error(
                res, 500, "Failed to evaluate visible child locations");
            return;
        }
    }
    free(cameras);
    set_location_json(res, 200, &location);
}

void handle_put_location(const http_request_t *req, http_response_t *res) {
    if (!httpd_authorize_global_action(req, res, AUTHZ_CAMERA_CONFIGURE)) return;
    char uuid[CAMERA_UUID_STRING_SIZE];
    if (!extract_location_uuid(req, uuid, sizeof(uuid), res)) return;

    camera_location_t location;
    db_location_result_t result = db_location_get(uuid, &location);
    if (result != DB_LOCATION_OK) {
        set_db_error(res, result, "Location conflict");
        return;
    }

    cJSON *body = httpd_parse_json_body(req);
    if (!body) {
        http_response_set_json_error(res, 400, "Invalid JSON request body");
        return;
    }
    if (!apply_location_fields(body, &location, false, res)) {
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);

    result = db_location_update(&location);
    if (result != DB_LOCATION_OK) {
        set_db_error(res, result,
                     "Location move creates a cycle or sibling name conflicts");
        return;
    }
    set_location_json(res, 200, &location);
}

void handle_delete_location(const http_request_t *req, http_response_t *res) {
    if (!httpd_authorize_global_action(req, res, AUTHZ_CAMERA_CONFIGURE)) return;
    char uuid[CAMERA_UUID_STRING_SIZE];
    if (!extract_location_uuid(req, uuid, sizeof(uuid), res)) return;

    db_location_result_t result = db_location_delete(uuid);
    if (result != DB_LOCATION_OK) {
        set_db_error(res, result,
                     "Location still contains cameras or child locations");
        return;
    }
    http_response_set_json(res, 200, "{\"success\":true}");
}

void handle_put_camera_location(const http_request_t *req,
                                http_response_t *res) {
    char camera_path[MAX_PATH_LENGTH];
    if (http_request_extract_path_param(req, "/api/cameras/", camera_path,
                                        sizeof(camera_path)) != 0) {
        http_response_set_json_error(res, 400, "Invalid camera path");
        return;
    }
    char *slash = strchr(camera_path, '/');
    if (slash) *slash = '\0';
    if (!valid_uuid_string(camera_path)) {
        http_response_set_json_error(res, 400, "Invalid camera UUID");
        return;
    }
    user_t user;
    fleet_camera_t camera;
    authorization_evaluation_t evaluation;
    if (!httpd_authorize_camera_identity_action_with_context(
            req, res, AUTHZ_CAMERA_CONFIGURE, camera_path, NULL, &user,
            &camera, &evaluation)) {
        return;
    }

    cJSON *body = httpd_parse_json_body(req);
    cJSON *location_item = body ?
        cJSON_GetObjectItemCaseSensitive(body, "location_uuid") : NULL;
    if (!body || !cJSON_IsObject(body) || !cJSON_IsString(location_item) ||
        !valid_uuid_string(location_item->valuestring)) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400,
                                     "A valid location_uuid is required");
        return;
    }
    char location_uuid[CAMERA_UUID_STRING_SIZE];
    safe_strcpy(location_uuid, location_item->valuestring,
                sizeof(location_uuid), 0);
    cJSON_Delete(body);

    db_location_result_t result =
        db_location_assign_camera(camera_path, location_uuid);
    if (result != DB_LOCATION_OK) {
        set_db_error(res, result, "Camera location assignment conflict");
        return;
    }

    // Keep in-memory config consistent (e.g. for backups).
    if (g_config.streams) {
        for (int i = 0; i < g_config.max_streams; i++) {
            if (strcmp(g_config.streams[i].camera_uuid, camera_path) == 0) {
                safe_strcpy(g_config.streams[i].location_uuid, location_uuid,
                            sizeof(g_config.streams[i].location_uuid), 0);
                break;
            }
        }
    }
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "camera_uuid", camera_path);
    cJSON_AddStringToObject(response, "location_uuid", location_uuid);
    char *json = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);
    if (!json) {
        http_response_set_json_error(res, 500, "Failed to serialize response");
        return;
    }
    http_response_set_json(res, 200, json);
    free(json);
}
