#define _POSIX_C_SOURCE 200809L

#include <cjson/cJSON.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>

#include "core/camera_selector.h"
#include "core/config.h"
#include "database/db_auth.h"
#include "database/db_camera_collections.h"
#include "database/db_fleet_query.h"
#include "utils/strings.h"
#include "web/api_handlers_camera_collections.h"
#include "web/httpd_utils.h"
#include "web/request_response.h"

#define COLLECTION_PREVIEW_MAX 50

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

static bool authenticate(const http_request_t *req, http_response_t *res,
                         user_t *user) {
    memset(user, 0, sizeof(*user));
    if (!httpd_check_viewer_access(req, user)) {
        http_response_set_json_error(res, 401, "Unauthorized");
        return false;
    }
    return true;
}

static bool can_view_collection(const user_t *user,
                                const camera_collection_t *collection) {
    return user->role == USER_ROLE_ADMIN || collection->is_shared ||
           (collection->owner_user_id > 0 &&
            collection->owner_user_id == user->id);
}

static bool extract_collection_uuid(const http_request_t *req, char *uuid,
                                    size_t uuid_size, http_response_t *res) {
    char value[MAX_PATH_LENGTH];
    if (http_request_extract_path_param(req, "/api/camera-collections/", value,
                                        sizeof(value)) != 0) {
        http_response_set_json_error(res, 400, "Invalid collection path");
        return false;
    }
    char *slash = strchr(value, '/');
    if (slash) *slash = '\0';
    if (!valid_uuid(value) || strlen(value) >= uuid_size) {
        http_response_set_json_error(res, 400, "Invalid collection UUID");
        return false;
    }
    safe_strcpy(uuid, value, uuid_size, 0);
    return true;
}

static void set_db_error(http_response_t *res,
                         db_camera_collection_result_t result) {
    switch (result) {
        case DB_CAMERA_COLLECTION_NOT_FOUND:
            http_response_set_json_error(res, 404,
                                         "Camera or collection not found");
            break;
        case DB_CAMERA_COLLECTION_CONFLICT:
            http_response_set_json_error(
                res, 409,
                "Collection name conflicts or the collection is used by an access policy");
            break;
        case DB_CAMERA_COLLECTION_WRONG_TYPE:
            http_response_set_json_error(
                res, 409, "Only static collections have explicit members");
            break;
        case DB_CAMERA_COLLECTION_LIMIT:
            http_response_set_json_error(res, 400,
                                         "Collection member limit exceeded");
            break;
        case DB_CAMERA_COLLECTION_INVALID:
            http_response_set_json_error(res, 400,
                                         "Invalid camera collection request");
            break;
        default:
            http_response_set_json_error(res, 500,
                                         "Camera collection operation failed");
            break;
    }
}

static bool load_authorized_fleet(const user_t *user,
                                  fleet_camera_t **cameras, int *count) {
    if (db_fleet_camera_load(cameras, count) != 0) return false;
    fleet_camera_enrich_runtime_health(*cameras, *count);
    /* Collection previews and membership lists are camera disclosures, so
     * they use the same live.view decision as every other read path. */
    if (authorization_filter_visible_cameras(user, *cameras, count) != 0) {
        free(*cameras);
        *cameras = NULL;
        *count = 0;
        return false;
    }
    return true;
}

static bool uuid_in_members(const char *uuid,
                            char members[][CAMERA_UUID_STRING_SIZE],
                            int member_count) {
    for (int i = 0; i < member_count; i++) {
        if (strcasecmp(uuid, members[i]) == 0) return true;
    }
    return false;
}

static int collection_matches(const camera_collection_t *collection,
                              fleet_camera_t *cameras, int camera_count,
                              fleet_camera_t ***matched_out) {
    fleet_camera_t **matched = camera_count > 0 ?
        calloc((size_t)camera_count, sizeof(*matched)) : NULL;
    if (camera_count > 0 && !matched) return -1;
    int matched_count = 0;

    if (strcmp(collection->collection_type, "static") == 0) {
        int member_capacity = collection->member_count;
        char (*members)[CAMERA_UUID_STRING_SIZE] = member_capacity > 0 ?
            calloc((size_t)member_capacity, sizeof(*members)) : NULL;
        if (member_capacity > 0 && !members) {
            free(matched);
            return -1;
        }
        int member_count = member_capacity > 0 ?
            db_camera_collection_list_members(collection->uuid, members,
                                               member_capacity) : 0;
        if (member_count < 0) {
            free(members);
            free(matched);
            return -1;
        }
        for (int i = 0; i < camera_count; i++) {
            if (uuid_in_members(cameras[i].camera_uuid, members, member_count)) {
                matched[matched_count++] = &cameras[i];
            }
        }
        free(members);
    } else {
        cJSON *selector_json = cJSON_Parse(collection->selector_json);
        char error[FLEET_SELECTOR_ERROR_MAX] = {0};
        fleet_selector_t *selector = fleet_selector_parse(
            selector_json, error, sizeof(error));
        cJSON_Delete(selector_json);
        if (!selector) {
            free(matched);
            return -1;
        }
        for (int i = 0; i < camera_count; i++) {
            if (fleet_selector_matches(selector, &cameras[i], NULL)) {
                matched[matched_count++] = &cameras[i];
            }
        }
        fleet_selector_free(selector);
    }
    *matched_out = matched;
    return matched_count;
}

static cJSON *collection_to_json(const camera_collection_t *collection,
                                 int member_count, int effective_count,
                                 bool include_selector) {
    cJSON *object = cJSON_CreateObject();
    if (!object) return NULL;
    cJSON_AddStringToObject(object, "uuid", collection->uuid);
    cJSON_AddStringToObject(object, "name", collection->name);
    cJSON_AddStringToObject(object, "description", collection->description);
    cJSON_AddStringToObject(object, "type", collection->collection_type);
    cJSON_AddBoolToObject(object, "shared", collection->is_shared);
    if (collection->owner_user_id > 0) {
        cJSON_AddNumberToObject(object, "owner_user_id",
                                (double)collection->owner_user_id);
    } else {
        cJSON_AddNullToObject(object, "owner_user_id");
    }
    cJSON_AddNumberToObject(object, "member_count", member_count);
    cJSON_AddNumberToObject(object, "effective_count", effective_count);
    cJSON_AddNumberToObject(object, "created_at",
                            (double)collection->created_at);
    cJSON_AddNumberToObject(object, "updated_at",
                            (double)collection->updated_at);
    if (strcmp(collection->collection_type, "smart") == 0 && include_selector) {
        cJSON *selector = cJSON_Parse(collection->selector_json);
        if (!selector) {
            cJSON_Delete(object);
            return NULL;
        }
        cJSON_AddItemToObject(object, "selector", selector);
    } else {
        cJSON_AddNullToObject(object, "selector");
    }
    cJSON_AddBoolToObject(object, "selector_redacted",
                          strcmp(collection->collection_type, "smart") == 0 &&
                          !include_selector);
    return object;
}

static bool apply_fields(cJSON *body, camera_collection_t *collection,
                         bool creating, http_response_t *res) {
    if (!cJSON_IsObject(body)) {
        http_response_set_json_error(res, 400,
                                     "Request body must be an object");
        return false;
    }
    cJSON *name = cJSON_GetObjectItemCaseSensitive(body, "name");
    if (name) {
        if (!cJSON_IsString(name) || !name->valuestring ||
            name->valuestring[0] == '\0' ||
            strlen(name->valuestring) >= sizeof(collection->name)) {
            http_response_set_json_error(res, 400, "Invalid collection name");
            return false;
        }
        safe_strcpy(collection->name, name->valuestring,
                    sizeof(collection->name), 0);
    } else if (creating) {
        http_response_set_json_error(res, 400, "Collection name is required");
        return false;
    }
    cJSON *description =
        cJSON_GetObjectItemCaseSensitive(body, "description");
    if (description) {
        if (!cJSON_IsString(description) || !description->valuestring ||
            strlen(description->valuestring) >= sizeof(collection->description)) {
            http_response_set_json_error(res, 400,
                                         "Invalid collection description");
            return false;
        }
        safe_strcpy(collection->description, description->valuestring,
                    sizeof(collection->description), 0);
    }
    cJSON *type = cJSON_GetObjectItemCaseSensitive(body, "type");
    if (type) {
        if (!cJSON_IsString(type) || !type->valuestring ||
            (strcmp(type->valuestring, "static") != 0 &&
             strcmp(type->valuestring, "smart") != 0)) {
            http_response_set_json_error(res, 400,
                                         "type must be static or smart");
            return false;
        }
        safe_strcpy(collection->collection_type, type->valuestring,
                    sizeof(collection->collection_type), 0);
    } else if (creating) {
        http_response_set_json_error(res, 400, "Collection type is required");
        return false;
    }
    cJSON *shared = cJSON_GetObjectItemCaseSensitive(body, "shared");
    if (shared) {
        if (!cJSON_IsBool(shared)) {
            http_response_set_json_error(res, 400, "shared must be boolean");
            return false;
        }
        collection->is_shared = cJSON_IsTrue(shared);
    }
    cJSON *selector = cJSON_GetObjectItemCaseSensitive(body, "selector");
    if (selector) {
        char error[FLEET_SELECTOR_ERROR_MAX] = {0};
        fleet_selector_t *parsed =
            fleet_selector_parse(selector, error, sizeof(error));
        if (!parsed) {
            http_response_set_json_error(
                res, 400, error[0] ? error : "Invalid collection selector");
            return false;
        }
        fleet_selector_free(parsed);
        char *serialized = cJSON_PrintUnformatted(selector);
        if (!serialized ||
            strlen(serialized) >= sizeof(collection->selector_json)) {
            free(serialized);
            http_response_set_json_error(res, 400,
                                         "Collection selector is too large");
            return false;
        }
        safe_strcpy(collection->selector_json, serialized,
                    sizeof(collection->selector_json), 0);
        free(serialized);
    }
    if (strcmp(collection->collection_type, "smart") == 0 &&
        collection->selector_json[0] == '\0') {
        http_response_set_json_error(res, 400,
                                     "Smart collections require selector");
        return false;
    }
    if (strcmp(collection->collection_type, "static") == 0) {
        collection->selector_json[0] = '\0';
    }
    return true;
}

static void set_collection_response(http_response_t *res, int status,
                                    const camera_collection_t *collection,
                                    const user_t *user) {
    fleet_camera_t *cameras = NULL;
    int camera_count = 0;
    if (!load_authorized_fleet(user, &cameras, &camera_count)) {
        http_response_set_json_error(res, 500, "Failed to load fleet cameras");
        return;
    }
    fleet_camera_t **matched = NULL;
    int effective_count =
        collection_matches(collection, cameras, camera_count, &matched);
    free(matched);
    free(cameras);
    if (effective_count < 0) {
        http_response_set_json_error(res, 500,
                                     "Failed to evaluate collection");
        return;
    }
    bool include_selector = user->role == USER_ROLE_ADMIN ||
        (collection->owner_user_id > 0 &&
         collection->owner_user_id == user->id);
    int member_count = user->has_tag_restriction &&
        strcmp(collection->collection_type, "static") == 0 ?
        effective_count : collection->member_count;
    cJSON *object = collection_to_json(collection, member_count,
                                       effective_count, include_selector);
    char *json = object ? cJSON_PrintUnformatted(object) : NULL;
    cJSON_Delete(object);
    if (!json) {
        http_response_set_json_error(res, 500, "Failed to serialize collection");
        return;
    }
    http_response_set_json(res, status, json);
    free(json);
}

void handle_get_camera_collections(const http_request_t *req,
                                   http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;
    int total = db_camera_collection_count();
    if (total < 0) {
        http_response_set_json_error(res, 500, "Failed to count collections");
        return;
    }
    camera_collection_t *collections = total > 0 ?
        calloc((size_t)total, sizeof(*collections)) : NULL;
    if (total > 0 && !collections) {
        http_response_set_json_error(res, 500, "Out of memory");
        return;
    }
    int count = total > 0 ? db_camera_collection_list(collections, total) : 0;
    if (count < 0) {
        free(collections);
        http_response_set_json_error(res, 500, "Failed to list collections");
        return;
    }
    fleet_camera_t *cameras = NULL;
    int camera_count = 0;
    if (!load_authorized_fleet(&user, &cameras, &camera_count)) {
        free(collections);
        http_response_set_json_error(res, 500, "Failed to load fleet cameras");
        return;
    }
    cJSON *root = cJSON_CreateObject();
    cJSON *items = cJSON_CreateArray();
    if (!root || !items) {
        cJSON_Delete(root);
        cJSON_Delete(items);
        free(cameras);
        free(collections);
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    cJSON_AddItemToObject(root, "collections", items);
    int visible_count = 0;
    for (int i = 0; i < count; i++) {
        if (!can_view_collection(&user, &collections[i])) continue;
        fleet_camera_t **matched = NULL;
        int effective_count = collection_matches(
            &collections[i], cameras, camera_count, &matched);
        free(matched);
        if (effective_count < 0) {
            cJSON_Delete(root);
            free(cameras);
            free(collections);
            http_response_set_json_error(res, 500,
                                         "Failed to evaluate collection");
            return;
        }
        bool include_selector = user.role == USER_ROLE_ADMIN ||
            (collections[i].owner_user_id > 0 &&
             collections[i].owner_user_id == user.id);
        int member_count = user.has_tag_restriction &&
            strcmp(collections[i].collection_type, "static") == 0 ?
            effective_count : collections[i].member_count;
        cJSON *item = collection_to_json(&collections[i], member_count,
                                         effective_count, include_selector);
        if (!item) {
            cJSON_Delete(root);
            free(cameras);
            free(collections);
            http_response_set_json_error(res, 500,
                                         "Failed to create response");
            return;
        }
        cJSON_AddItemToArray(items, item);
        visible_count++;
    }
    cJSON_AddNumberToObject(root, "count", visible_count);
    free(cameras);
    free(collections);
    char *json = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if (!json) {
        http_response_set_json_error(res, 500, "Failed to serialize response");
        return;
    }
    http_response_set_json(res, 200, json);
    free(json);
}

void handle_post_camera_collection(const http_request_t *req,
                                   http_response_t *res) {
    if (!httpd_check_admin_privileges(req, res)) return;
    user_t user;
    if (!authenticate(req, res, &user)) return;
    cJSON *body = httpd_parse_json_body(req);
    camera_collection_t collection;
    memset(&collection, 0, sizeof(collection));
    collection.is_shared = true;
    collection.owner_user_id = user.id;
    if (!apply_fields(body, &collection, true, res)) {
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);
    db_camera_collection_result_t result =
        db_camera_collection_create(&collection);
    if (result != DB_CAMERA_COLLECTION_OK) {
        set_db_error(res, result);
        return;
    }
    set_collection_response(res, 201, &collection, &user);
}

void handle_get_camera_collection(const http_request_t *req,
                                  http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;
    char uuid[CAMERA_UUID_STRING_SIZE];
    if (!extract_collection_uuid(req, uuid, sizeof(uuid), res)) return;
    camera_collection_t collection;
    db_camera_collection_result_t result =
        db_camera_collection_get(uuid, &collection);
    if (result != DB_CAMERA_COLLECTION_OK) {
        set_db_error(res, result);
        return;
    }
    if (!can_view_collection(&user, &collection)) {
        http_response_set_json_error(res, 404, "Collection not found");
        return;
    }
    set_collection_response(res, 200, &collection, &user);
}

void handle_put_camera_collection(const http_request_t *req,
                                  http_response_t *res) {
    if (!httpd_check_admin_privileges(req, res)) return;
    user_t user;
    if (!authenticate(req, res, &user)) return;
    char uuid[CAMERA_UUID_STRING_SIZE];
    if (!extract_collection_uuid(req, uuid, sizeof(uuid), res)) return;
    camera_collection_t collection;
    db_camera_collection_result_t result =
        db_camera_collection_get(uuid, &collection);
    if (result != DB_CAMERA_COLLECTION_OK) {
        set_db_error(res, result);
        return;
    }
    cJSON *body = httpd_parse_json_body(req);
    if (!apply_fields(body, &collection, false, res)) {
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);
    result = db_camera_collection_update(&collection);
    if (result != DB_CAMERA_COLLECTION_OK) {
        set_db_error(res, result);
        return;
    }
    set_collection_response(res, 200, &collection, &user);
}

void handle_delete_camera_collection(const http_request_t *req,
                                     http_response_t *res) {
    if (!httpd_check_admin_privileges(req, res)) return;
    char uuid[CAMERA_UUID_STRING_SIZE];
    if (!extract_collection_uuid(req, uuid, sizeof(uuid), res)) return;
    db_camera_collection_result_t result = db_camera_collection_delete(uuid);
    if (result != DB_CAMERA_COLLECTION_OK) {
        set_db_error(res, result);
        return;
    }
    http_response_set_json(res, 200, "{\"success\":true}");
}

void handle_get_camera_collection_members(const http_request_t *req,
                                          http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;
    char uuid[CAMERA_UUID_STRING_SIZE];
    if (!extract_collection_uuid(req, uuid, sizeof(uuid), res)) return;
    camera_collection_t collection;
    db_camera_collection_result_t result =
        db_camera_collection_get(uuid, &collection);
    if (result != DB_CAMERA_COLLECTION_OK) {
        set_db_error(res, result);
        return;
    }
    if (!can_view_collection(&user, &collection)) {
        http_response_set_json_error(res, 404, "Collection not found");
        return;
    }
    if (strcmp(collection.collection_type, "static") != 0) {
        set_db_error(res, DB_CAMERA_COLLECTION_WRONG_TYPE);
        return;
    }
    fleet_camera_t *cameras = NULL;
    int camera_count = 0;
    if (!load_authorized_fleet(&user, &cameras, &camera_count)) {
        http_response_set_json_error(res, 500, "Failed to load fleet cameras");
        return;
    }
    fleet_camera_t **matched = NULL;
    int matched_count = collection_matches(
        &collection, cameras, camera_count, &matched);
    if (matched_count < 0) {
        free(cameras);
        http_response_set_json_error(res, 500, "Failed to list members");
        return;
    }
    cJSON *root = cJSON_CreateObject();
    cJSON *items = cJSON_CreateArray();
    if (!root || !items) {
        cJSON_Delete(root);
        cJSON_Delete(items);
        free(matched);
        free(cameras);
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    cJSON_AddStringToObject(root, "collection_uuid", uuid);
    cJSON_AddItemToObject(root, "camera_uuids", items);
    cJSON_AddNumberToObject(root, "count", matched_count);
    for (int i = 0; i < matched_count; i++) {
        cJSON_AddItemToArray(items,
                             cJSON_CreateString(matched[i]->camera_uuid));
    }
    free(matched);
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

void handle_put_camera_collection_members(const http_request_t *req,
                                          http_response_t *res) {
    if (!httpd_check_admin_privileges(req, res)) return;
    char uuid[CAMERA_UUID_STRING_SIZE];
    if (!extract_collection_uuid(req, uuid, sizeof(uuid), res)) return;
    cJSON *body = httpd_parse_json_body(req);
    cJSON *items = body ?
        cJSON_GetObjectItemCaseSensitive(body, "camera_uuids") : NULL;
    if (!cJSON_IsObject(body) || !cJSON_IsArray(items)) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400,
                                     "camera_uuids must be an array");
        return;
    }
    int count = cJSON_GetArraySize(items);
    if (count < 0 || count > CAMERA_COLLECTION_MAX_MEMBERS) {
        cJSON_Delete(body);
        set_db_error(res, DB_CAMERA_COLLECTION_LIMIT);
        return;
    }
    char (*storage)[CAMERA_UUID_STRING_SIZE] = count > 0 ?
        calloc((size_t)count, sizeof(*storage)) : NULL;
    const char **camera_uuids = count > 0 ?
        calloc((size_t)count, sizeof(*camera_uuids)) : NULL;
    if (count > 0 && (!storage || !camera_uuids)) {
        free(storage);
        free(camera_uuids);
        cJSON_Delete(body);
        http_response_set_json_error(res, 500, "Out of memory");
        return;
    }
    for (int i = 0; i < count; i++) {
        cJSON *item = cJSON_GetArrayItem(items, i);
        if (!cJSON_IsString(item) || !valid_uuid(item->valuestring)) {
            free(storage);
            free(camera_uuids);
            cJSON_Delete(body);
            http_response_set_json_error(res, 400,
                                         "camera_uuids contains invalid UUID");
            return;
        }
        safe_strcpy(storage[i], item->valuestring,
                    CAMERA_UUID_STRING_SIZE, 0);
        camera_uuids[i] = storage[i];
    }
    cJSON_Delete(body);
    db_camera_collection_result_t result = db_camera_collection_set_members(
        uuid, camera_uuids, count);
    free(storage);
    free(camera_uuids);
    if (result != DB_CAMERA_COLLECTION_OK) {
        set_db_error(res, result);
        return;
    }
    handle_get_camera_collection_members(req, res);
}

void handle_post_camera_collection_preview(const http_request_t *req,
                                           http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;
    char uuid[CAMERA_UUID_STRING_SIZE];
    if (!extract_collection_uuid(req, uuid, sizeof(uuid), res)) return;
    camera_collection_t collection;
    db_camera_collection_result_t result =
        db_camera_collection_get(uuid, &collection);
    if (result != DB_CAMERA_COLLECTION_OK) {
        set_db_error(res, result);
        return;
    }
    if (!can_view_collection(&user, &collection)) {
        http_response_set_json_error(res, 404, "Collection not found");
        return;
    }
    fleet_camera_t *cameras = NULL;
    int camera_count = 0;
    if (!load_authorized_fleet(&user, &cameras, &camera_count)) {
        http_response_set_json_error(res, 500, "Failed to load fleet cameras");
        return;
    }
    fleet_camera_t **matched = NULL;
    int matched_count = collection_matches(
        &collection, cameras, camera_count, &matched);
    if (matched_count < 0) {
        free(cameras);
        http_response_set_json_error(res, 500,
                                     "Failed to evaluate collection");
        return;
    }
    cJSON *root = cJSON_CreateObject();
    cJSON *sample = cJSON_CreateArray();
    if (!root || !sample) {
        cJSON_Delete(root);
        cJSON_Delete(sample);
        free(matched);
        free(cameras);
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    cJSON_AddStringToObject(root, "collection_uuid", uuid);
    cJSON_AddNumberToObject(root, "matched_count", matched_count);
    cJSON_AddItemToObject(root, "sample", sample);
    int sample_count = matched_count < COLLECTION_PREVIEW_MAX ?
        matched_count : COLLECTION_PREVIEW_MAX;
    for (int i = 0; i < sample_count; i++) {
        cJSON *item = cJSON_CreateObject();
        cJSON_AddStringToObject(item, "camera_uuid", matched[i]->camera_uuid);
        cJSON_AddStringToObject(item, "name", matched[i]->name);
        cJSON_AddStringToObject(item, "location_path",
                                matched[i]->location_path);
        cJSON_AddItemToArray(sample, item);
    }
    free(matched);
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
