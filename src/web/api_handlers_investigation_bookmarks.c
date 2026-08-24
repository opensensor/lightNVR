#define _POSIX_C_SOURCE 200809L

#include <cjson/cJSON.h>
#include <math.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "core/authorization.h"
#include "core/config.h"
#include "database/db_fleet_query.h"
#include "database/db_investigation_bookmarks.h"
#include "database/db_streams.h"
#include "utils/strings.h"
#include "utils/uuid.h"
#include "web/api_handlers_investigation_bookmarks.h"
#include "web/audit_log.h"
#include "web/httpd_utils.h"

#define BOOKMARK_AUDIT_ACTION "investigation.bookmark"
#define BOOKMARK_TARGET_TYPE "investigation_bookmark"

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

static bool parse_int64(const cJSON *body, const char *key, int64_t *value) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(body, key);
    if (!cJSON_IsNumber(item) || !isfinite(item->valuedouble) ||
        floor(item->valuedouble) != item->valuedouble ||
        item->valuedouble < 1 || item->valuedouble > (double)INT64_MAX) {
        return false;
    }
    *value = (int64_t)item->valuedouble;
    return true;
}

static bool copy_required_string(const cJSON *body, const char *key,
                                 char *destination, size_t size) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(body, key);
    if (!cJSON_IsString(item) || !item->valuestring ||
        item->valuestring[0] == '\0' || strlen(item->valuestring) >= size) {
        return false;
    }
    safe_strcpy(destination, item->valuestring, size, 0);
    return true;
}

static bool copy_optional_string(const cJSON *body, const char *key,
                                 char *destination, size_t size) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(body, key);
    if (!item) return true;
    if (!cJSON_IsString(item) || !item->valuestring ||
        strlen(item->valuestring) >= size) {
        return false;
    }
    safe_strcpy(destination, item->valuestring, size, 0);
    return true;
}

static bool key_allowed(const char *key, const char *const *allowed,
                        size_t allowed_count) {
    for (size_t i = 0; i < allowed_count; i++) {
        if (strcmp(key, allowed[i]) == 0) return true;
    }
    return false;
}

static bool valid_filter_value(const cJSON *item) {
    if (strcmp(item->string, "min_confidence") == 0) {
        return cJSON_IsNumber(item) && isfinite(item->valuedouble) &&
               item->valuedouble >= 0 && item->valuedouble <= 1;
    }
    if (strcmp(item->string, "region") == 0) {
        if (!cJSON_IsObject(item)) return false;
        static const char *const region_keys[] = {
            "camera_uuid", "x", "y", "width", "height", "match",
            "min_intersection"
        };
        const cJSON *part = NULL;
        cJSON_ArrayForEach(part, item) {
            if (!part->string ||
                !key_allowed(part->string, region_keys,
                             sizeof(region_keys) / sizeof(region_keys[0]))) {
                return false;
            }
            if (strcmp(part->string, "camera_uuid") == 0) {
                if (!cJSON_IsString(part) ||
                    !lightnvr_uuid_is_valid(part->valuestring)) return false;
            } else if (strcmp(part->string, "match") == 0) {
                if (!cJSON_IsString(part) ||
                    (strcmp(part->valuestring, "center") != 0 &&
                     strcmp(part->valuestring, "intersects") != 0 &&
                     strcmp(part->valuestring, "minimum_intersection") != 0))
                    return false;
            } else if (!cJSON_IsNumber(part) ||
                       !isfinite(part->valuedouble) ||
                       part->valuedouble < 0 || part->valuedouble > 1) {
                return false;
            }
        }
        const cJSON *x = cJSON_GetObjectItemCaseSensitive(item, "x");
        const cJSON *y = cJSON_GetObjectItemCaseSensitive(item, "y");
        const cJSON *camera_uuid =
            cJSON_GetObjectItemCaseSensitive(item, "camera_uuid");
        const cJSON *width = cJSON_GetObjectItemCaseSensitive(item, "width");
        const cJSON *height = cJSON_GetObjectItemCaseSensitive(item, "height");
        const cJSON *match = cJSON_GetObjectItemCaseSensitive(item, "match");
        const cJSON *minimum =
            cJSON_GetObjectItemCaseSensitive(item, "min_intersection");
        if (!cJSON_IsString(camera_uuid) ||
            !lightnvr_uuid_is_valid(camera_uuid->valuestring) ||
            !cJSON_IsNumber(x) || !cJSON_IsNumber(y) ||
            !cJSON_IsNumber(width) || !cJSON_IsNumber(height) ||
            width->valuedouble <= 0 || height->valuedouble <= 0 ||
            x->valuedouble + width->valuedouble > 1 ||
            y->valuedouble + height->valuedouble > 1 ||
            (cJSON_IsString(match) &&
             strcmp(match->valuestring, "minimum_intersection") == 0 &&
             (!cJSON_IsNumber(minimum) || minimum->valuedouble <= 0))) {
            return false;
        }
        return true;
    }
    return cJSON_IsString(item) && item->valuestring &&
           strlen(item->valuestring) <= 256;
}

static bool canonicalize_filters(const cJSON *filters, char *destination,
                                 size_t size) {
    static const char *const keys[] = {
        "event_type", "location", "label", "zone", "source",
        "capture_method", "recording_tag", "protection", "min_confidence",
        "region"
    };
    if (!cJSON_IsObject(filters)) return false;
    cJSON *canonical = cJSON_CreateObject();
    if (!canonical) return false;
    const cJSON *item = NULL;
    cJSON_ArrayForEach(item, filters) {
        if (!item->string ||
            !key_allowed(item->string, keys, sizeof(keys) / sizeof(keys[0])) ||
            !valid_filter_value(item)) {
            cJSON_Delete(canonical);
            return false;
        }
        cJSON_AddItemToObject(canonical, item->string, cJSON_Duplicate(item, 1));
    }
    char *encoded = cJSON_PrintUnformatted(canonical);
    cJSON_Delete(canonical);
    if (!encoded || strlen(encoded) >= size) {
        free(encoded);
        return false;
    }
    safe_strcpy(destination, encoded, size, 0);
    free(encoded);
    return true;
}

static bool canonicalize_result(const cJSON *result, char *destination,
                                size_t size) {
    static const char *const keys[] = {
        "result_id", "camera_uuid", "start_time", "end_time", "event_type",
        "recording_id", "detection_id", "label"
    };
    if (!result || cJSON_IsNull(result)) {
        destination[0] = '\0';
        return true;
    }
    if (!cJSON_IsObject(result)) return false;
    cJSON *canonical = cJSON_CreateObject();
    if (!canonical) return false;
    const cJSON *item = NULL;
    cJSON_ArrayForEach(item, result) {
        if (!item->string ||
            !key_allowed(item->string, keys, sizeof(keys) / sizeof(keys[0])) ||
            !(cJSON_IsString(item) || cJSON_IsNumber(item))) {
            cJSON_Delete(canonical);
            return false;
        }
        if (strcmp(item->string, "camera_uuid") == 0 &&
            (!cJSON_IsString(item) ||
             !lightnvr_uuid_is_valid(item->valuestring))) {
            cJSON_Delete(canonical);
            return false;
        }
        cJSON_AddItemToObject(canonical, item->string, cJSON_Duplicate(item, 1));
    }
    char *encoded = cJSON_PrintUnformatted(canonical);
    cJSON_Delete(canonical);
    if (!encoded || strlen(encoded) >= size) {
        free(encoded);
        return false;
    }
    safe_strcpy(destination, encoded, size, 0);
    free(encoded);
    return true;
}

static bool parse_create_body(
    const cJSON *body, int64_t owner_user_id,
    investigation_bookmark_t *bookmark,
    char camera_uuids[][CAMERA_UUID_STRING_SIZE], int *camera_count,
    http_response_t *res) {
    memset(bookmark, 0, sizeof(*bookmark));
    bookmark->owner_user_id = owner_user_id;
    if (!cJSON_IsObject(body) ||
        !copy_required_string(body, "title", bookmark->title,
                              sizeof(bookmark->title)) ||
        !copy_optional_string(body, "note", bookmark->note,
                              sizeof(bookmark->note)) ||
        !parse_int64(body, "start_time", &bookmark->start_time) ||
        !parse_int64(body, "end_time", &bookmark->end_time) ||
        !parse_int64(body, "cursor_time", &bookmark->cursor_time) ||
        !copy_required_string(body, "primary_camera_uuid",
                              bookmark->primary_camera_uuid,
                              sizeof(bookmark->primary_camera_uuid)) ||
        !lightnvr_uuid_is_valid(bookmark->primary_camera_uuid)) {
        http_response_set_json_error(res, 400, "Invalid bookmark fields");
        return false;
    }
    const cJSON *cameras =
        cJSON_GetObjectItemCaseSensitive(body, "camera_uuids");
    *camera_count = cJSON_IsArray(cameras) ? cJSON_GetArraySize(cameras) : 0;
    if (*camera_count < 1 ||
        *camera_count > INVESTIGATION_BOOKMARK_MAX_CAMERAS) {
        http_response_set_json_error(
            res, 400, "camera_uuids must contain between 1 and 16 cameras");
        return false;
    }
    for (int i = 0; i < *camera_count; i++) {
        const cJSON *item = cJSON_GetArrayItem(cameras, i);
        if (!cJSON_IsString(item) ||
            !lightnvr_uuid_is_valid(item->valuestring)) {
            http_response_set_json_error(res, 400,
                                         "camera_uuids contains invalid UUID");
            return false;
        }
        for (int previous = 0; previous < i; previous++) {
            if (strcmp(camera_uuids[previous], item->valuestring) == 0) {
                http_response_set_json_error(res, 400,
                                             "camera_uuids contains duplicates");
                return false;
            }
        }
        safe_strcpy(camera_uuids[i], item->valuestring,
                    CAMERA_UUID_STRING_SIZE, 0);
    }
    const cJSON *filters = cJSON_GetObjectItemCaseSensitive(body, "filters");
    const cJSON *result =
        cJSON_GetObjectItemCaseSensitive(body, "representative_result");
    if (!canonicalize_filters(filters, bookmark->filters_json,
                              sizeof(bookmark->filters_json)) ||
        !canonicalize_result(result, bookmark->representative_result_json,
                             sizeof(bookmark->representative_result_json))) {
        http_response_set_json_error(res, 400,
                                     "Invalid bookmark investigation state");
        return false;
    }
    const cJSON *region = cJSON_IsObject(filters)
        ? cJSON_GetObjectItemCaseSensitive(filters, "region") : NULL;
    const cJSON *region_camera = cJSON_IsObject(region)
        ? cJSON_GetObjectItemCaseSensitive(region, "camera_uuid") : NULL;
    const cJSON *result_camera = cJSON_IsObject(result)
        ? cJSON_GetObjectItemCaseSensitive(result, "camera_uuid") : NULL;
    if (region_camera || result_camera) {
        bool region_found = region_camera == NULL;
        bool result_found = result_camera == NULL;
        for (int i = 0; i < *camera_count; i++) {
            if (region_camera && cJSON_IsString(region_camera) &&
                strcmp(camera_uuids[i], region_camera->valuestring) == 0)
                region_found = true;
            if (result_camera && cJSON_IsString(result_camera) &&
                strcmp(camera_uuids[i], result_camera->valuestring) == 0)
                result_found = true;
        }
        if (!region_found || !result_found) {
            http_response_set_json_error(
                res, 400, "Saved investigation state references another camera");
            return false;
        }
    }
    return true;
}

static int authorize_cameras(
    const http_request_t *req, http_response_t *res, const user_t *user,
    char camera_uuids[][CAMERA_UUID_STRING_SIZE], int camera_count,
    bool conceal_denial) {
    authorization_context_t *context = authorization_context_create();
    if (!context) {
        if (res) http_response_set_json_error(res, 500,
                                              "Authorization context unavailable");
        return -1;
    }
    for (int i = 0; i < camera_count; i++) {
        stream_config_t stream = {0};
        fleet_camera_t camera = {0};
        if (get_stream_config_by_uuid(camera_uuids[i], &stream) != 0 ||
            db_fleet_camera_find_by_name(stream.name, &camera) != 0) {
            authorization_context_free(context);
            if (res && !conceal_denial)
                http_response_set_json_error(res, 404, "Camera not found");
            return 0;
        }
        authorization_evaluation_t evaluation = {0};
        if (authorization_evaluate_in_context(
                context, user, AUTHZ_RECORDINGS_REPLAY, &camera,
                &evaluation) != 0) {
            if (!conceal_denial)
                audit_log_authorization(req, user, AUTHZ_RECORDINGS_REPLAY,
                                        &camera, NULL, "error");
            authorization_context_free(context);
            if (res && !conceal_denial)
                http_response_set_json_error(
                    res, 500, "Authorization policy evaluation failed");
            return -1;
        }
        if (evaluation.decision != AUTHZ_DECISION_ALLOW) {
            if (!conceal_denial)
                audit_log_authorization(req, user, AUTHZ_RECORDINGS_REPLAY,
                                        &camera, &evaluation, "denied");
            authorization_context_free(context);
            if (res && !conceal_denial)
                http_response_set_json_error(res, 403, "Forbidden");
            return 0;
        }
    }
    authorization_context_free(context);
    return 1;
}

static bool extract_uuid(const http_request_t *req, char *uuid,
                         http_response_t *res) {
    if (http_request_extract_path_param(
            req, "/api/investigation-bookmarks/", uuid,
            CAMERA_UUID_STRING_SIZE) != 0 || !lightnvr_uuid_is_valid(uuid)) {
        http_response_set_json_error(res, 400, "Invalid bookmark UUID");
        return false;
    }
    return true;
}

static bool load_cameras(const investigation_bookmark_t *bookmark,
                         char camera_uuids[][CAMERA_UUID_STRING_SIZE],
                         http_response_t *res) {
    int count = db_investigation_bookmark_list_cameras(
        bookmark->uuid, camera_uuids, INVESTIGATION_BOOKMARK_MAX_CAMERAS);
    if (count != bookmark->camera_count || count < 1) {
        if (res) http_response_set_json_error(res, 500,
                                              "Failed to load bookmark cameras");
        return false;
    }
    return true;
}

static cJSON *bookmark_json(
    const investigation_bookmark_t *bookmark,
    char camera_uuids[][CAMERA_UUID_STRING_SIZE]) {
    cJSON *object = cJSON_CreateObject();
    cJSON *cameras = cJSON_CreateArray();
    if (!object || !cameras) {
        cJSON_Delete(object);
        cJSON_Delete(cameras);
        return NULL;
    }
    cJSON_AddStringToObject(object, "uuid", bookmark->uuid);
    cJSON_AddStringToObject(object, "title", bookmark->title);
    cJSON_AddStringToObject(object, "note", bookmark->note);
    cJSON_AddNumberToObject(object, "start_time", (double)bookmark->start_time);
    cJSON_AddNumberToObject(object, "end_time", (double)bookmark->end_time);
    cJSON_AddNumberToObject(object, "cursor_time", (double)bookmark->cursor_time);
    cJSON_AddStringToObject(object, "primary_camera_uuid",
                            bookmark->primary_camera_uuid);
    for (int i = 0; i < bookmark->camera_count; i++)
        cJSON_AddItemToArray(cameras, cJSON_CreateString(camera_uuids[i]));
    cJSON_AddItemToObject(object, "camera_uuids", cameras);
    cJSON *filters = cJSON_Parse(bookmark->filters_json);
    cJSON_AddItemToObject(object, "filters",
                          filters ? filters : cJSON_CreateObject());
    cJSON *result = bookmark->representative_result_json[0]
        ? cJSON_Parse(bookmark->representative_result_json) : NULL;
    if (result)
        cJSON_AddItemToObject(object, "representative_result", result);
    else
        cJSON_AddNullToObject(object, "representative_result");
    cJSON_AddNumberToObject(object, "revision", (double)bookmark->revision);
    cJSON_AddNumberToObject(object, "created_at", (double)bookmark->created_at);
    cJSON_AddNumberToObject(object, "updated_at", (double)bookmark->updated_at);
    cJSON_AddBoolToObject(object, "holds_recordings", false);
    return object;
}

static void send_json(http_response_t *res, int status, cJSON *root) {
    char *encoded = cJSON_PrintUnformatted(root);
    if (!encoded) {
        http_response_set_json_error(res, 500, "Failed to serialize bookmark");
        return;
    }
    http_response_set_json(res, status, encoded);
    free(encoded);
}

static void set_db_error(http_response_t *res,
                         db_investigation_bookmark_result_t result) {
    switch (result) {
        case DB_INVESTIGATION_BOOKMARK_NOT_FOUND:
            http_response_set_json_error(res, 404, "Bookmark not found");
            break;
        case DB_INVESTIGATION_BOOKMARK_STALE:
            http_response_set_json_error(res, 409,
                                         "Bookmark was changed elsewhere");
            break;
        case DB_INVESTIGATION_BOOKMARK_LIMIT:
            http_response_set_json_error(res, 409, "Bookmark limit reached");
            break;
        case DB_INVESTIGATION_BOOKMARK_INVALID:
            http_response_set_json_error(res, 400, "Invalid bookmark");
            break;
        default:
            http_response_set_json_error(res, 500, "Bookmark operation failed");
            break;
    }
}

static void audit_mutation(const http_request_t *req, const user_t *user,
                           const char *uuid, const char *operation,
                           const char *outcome) {
    audit_log_operation(req, user, BOOKMARK_AUDIT_ACTION,
                        BOOKMARK_TARGET_TYPE, uuid, operation, outcome, NULL);
}

void handle_get_investigation_bookmarks(const http_request_t *req,
                                        http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;
    cJSON *root = cJSON_CreateObject();
    cJSON *items = cJSON_CreateArray();
    if (!root || !items) {
        cJSON_Delete(root);
        cJSON_Delete(items);
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    cJSON_AddItemToObject(root, "bookmarks", items);
    if (!demo_read_only(&user)) {
        investigation_bookmark_t *bookmarks = calloc(
            INVESTIGATION_BOOKMARK_MAX_PER_OWNER, sizeof(*bookmarks));
        if (!bookmarks) {
            cJSON_Delete(root);
            http_response_set_json_error(res, 500, "Out of memory");
            return;
        }
        int count = db_investigation_bookmark_list(
            user.id, bookmarks, INVESTIGATION_BOOKMARK_MAX_PER_OWNER);
        if (count < 0) {
            free(bookmarks);
            cJSON_Delete(root);
            http_response_set_json_error(res, 500, "Failed to list bookmarks");
            return;
        }
        for (int i = 0; i < count; i++) {
            char cameras[INVESTIGATION_BOOKMARK_MAX_CAMERAS]
                        [CAMERA_UUID_STRING_SIZE] = {{0}};
            if (!load_cameras(&bookmarks[i], cameras, NULL) ||
                authorize_cameras(req, NULL, &user, cameras,
                                  bookmarks[i].camera_count, true) != 1) {
                continue;
            }
            cJSON *item = bookmark_json(&bookmarks[i], cameras);
            if (item) cJSON_AddItemToArray(items, item);
        }
        free(bookmarks);
    }
    cJSON_AddNumberToObject(root, "count", cJSON_GetArraySize(items));
    send_json(res, 200, root);
    cJSON_Delete(root);
}

void handle_post_investigation_bookmark(const http_request_t *req,
                                        http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;
    if (demo_read_only(&user)) {
        http_response_set_json_error(res, 403, "Demo mode is read-only");
        return;
    }
    cJSON *body = httpd_parse_json_body(req);
    investigation_bookmark_t bookmark;
    char cameras[INVESTIGATION_BOOKMARK_MAX_CAMERAS]
                [CAMERA_UUID_STRING_SIZE] = {{0}};
    int camera_count = 0;
    if (!parse_create_body(body, user.id, &bookmark, cameras, &camera_count,
                           res)) {
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);
    if (authorize_cameras(req, res, &user, cameras, camera_count, false) != 1)
        return;
    db_investigation_bookmark_result_t result =
        db_investigation_bookmark_create(&bookmark, cameras, camera_count);
    if (result != DB_INVESTIGATION_BOOKMARK_OK) {
        audit_mutation(req, &user, NULL, "bookmark_create", "failure");
        set_db_error(res, result);
        return;
    }
    audit_mutation(req, &user, bookmark.uuid, "bookmark_create", "success");
    cJSON *root = bookmark_json(&bookmark, cameras);
    if (!root) {
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    send_json(res, 201, root);
    cJSON_Delete(root);
}

static bool load_authorized_bookmark(
    const http_request_t *req, http_response_t *res, const user_t *user,
    const char *uuid, investigation_bookmark_t *bookmark,
    char cameras[][CAMERA_UUID_STRING_SIZE]) {
    db_investigation_bookmark_result_t result =
        db_investigation_bookmark_get(user->id, uuid, bookmark);
    if (result != DB_INVESTIGATION_BOOKMARK_OK) {
        set_db_error(res, result);
        return false;
    }
    return load_cameras(bookmark, cameras, res) &&
           authorize_cameras(req, res, user, cameras, bookmark->camera_count,
                             false) == 1;
}

void handle_get_investigation_bookmark(const http_request_t *req,
                                       http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;
    if (demo_read_only(&user)) {
        http_response_set_json_error(res, 404, "Bookmark not found");
        return;
    }
    char uuid[CAMERA_UUID_STRING_SIZE] = {0};
    investigation_bookmark_t bookmark;
    char cameras[INVESTIGATION_BOOKMARK_MAX_CAMERAS]
                [CAMERA_UUID_STRING_SIZE] = {{0}};
    if (!extract_uuid(req, uuid, res) ||
        !load_authorized_bookmark(req, res, &user, uuid, &bookmark, cameras))
        return;
    cJSON *root = bookmark_json(&bookmark, cameras);
    if (!root) {
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    send_json(res, 200, root);
    cJSON_Delete(root);
}

void handle_put_investigation_bookmark(const http_request_t *req,
                                       http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;
    if (demo_read_only(&user)) {
        http_response_set_json_error(res, 403, "Demo mode is read-only");
        return;
    }
    char uuid[CAMERA_UUID_STRING_SIZE] = {0};
    investigation_bookmark_t bookmark;
    char cameras[INVESTIGATION_BOOKMARK_MAX_CAMERAS]
                [CAMERA_UUID_STRING_SIZE] = {{0}};
    if (!extract_uuid(req, uuid, res) ||
        !load_authorized_bookmark(req, res, &user, uuid, &bookmark, cameras))
        return;
    cJSON *body = httpd_parse_json_body(req);
    int64_t revision = 0;
    bool valid = cJSON_IsObject(body) &&
        copy_required_string(body, "title", bookmark.title,
                             sizeof(bookmark.title)) &&
        copy_optional_string(body, "note", bookmark.note,
                             sizeof(bookmark.note)) &&
        parse_int64(body, "revision", &revision);
    cJSON_Delete(body);
    if (!valid) {
        http_response_set_json_error(res, 400,
                                     "title and revision are required");
        return;
    }
    db_investigation_bookmark_result_t result =
        db_investigation_bookmark_update_metadata(&bookmark, revision);
    if (result != DB_INVESTIGATION_BOOKMARK_OK) {
        audit_mutation(req, &user, uuid, "bookmark_update", "failure");
        set_db_error(res, result);
        return;
    }
    audit_mutation(req, &user, uuid, "bookmark_update", "success");
    cJSON *root = bookmark_json(&bookmark, cameras);
    if (!root) {
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    send_json(res, 200, root);
    cJSON_Delete(root);
}

void handle_delete_investigation_bookmark(const http_request_t *req,
                                          http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;
    if (demo_read_only(&user)) {
        http_response_set_json_error(res, 403, "Demo mode is read-only");
        return;
    }
    char uuid[CAMERA_UUID_STRING_SIZE] = {0};
    investigation_bookmark_t bookmark;
    char cameras[INVESTIGATION_BOOKMARK_MAX_CAMERAS]
                [CAMERA_UUID_STRING_SIZE] = {{0}};
    if (!extract_uuid(req, uuid, res) ||
        !load_authorized_bookmark(req, res, &user, uuid, &bookmark, cameras))
        return;
    cJSON *body = httpd_parse_json_body(req);
    int64_t revision = 0;
    bool valid = cJSON_IsObject(body) && parse_int64(body, "revision", &revision);
    cJSON_Delete(body);
    if (!valid) {
        http_response_set_json_error(res, 400, "revision is required");
        return;
    }
    db_investigation_bookmark_result_t result =
        db_investigation_bookmark_delete(user.id, uuid, revision);
    if (result != DB_INVESTIGATION_BOOKMARK_OK) {
        audit_mutation(req, &user, uuid, "bookmark_delete", "failure");
        set_db_error(res, result);
        return;
    }
    audit_mutation(req, &user, uuid, "bookmark_delete", "success");
    http_response_set_json(res, 200, "{\"success\":true}");
}
