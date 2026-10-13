/**
 * Per-user favorite cameras.
 *
 * Favorites are a personal preference rather than camera configuration, so
 * they are not audited. Reads are limited to cameras inside the caller's
 * current live-view scope; writes require an interactive user.
 */

#define _POSIX_C_SOURCE 200809L

#include "web/api_handlers_camera_favorites.h"

#include <cjson/cJSON.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "core/authorization.h"
#include "database/db_auth.h"
#include "database/db_camera_favorites.h"
#include "database/db_fleet_query.h"
#include "utils/strings.h"
#include "utils/uuid.h"
#include "web/httpd_utils.h"

#define FAVORITES_PATH_PREFIX "/api/camera-favorites/"

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

static bool require_interactive_user(const user_t *user, http_response_t *res) {
    if (!read_only_identity(user)) return true;
    http_response_set_json_error(res, 403,
                                 "Camera favorites require an interactive user");
    return false;
}

static bool extract_camera_uuid(const http_request_t *req, http_response_t *res,
                                char *uuid, size_t size) {
    if (http_request_extract_path_param(req, FAVORITES_PATH_PREFIX, uuid,
                                        size) != 0 ||
        !lightnvr_uuid_is_valid(uuid)) {
        http_response_set_json_error(res, 400, "Invalid camera UUID");
        return false;
    }
    return true;
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

static bool camera_allowed(const char *camera_uuid,
                           const fleet_camera_t *cameras, int camera_count) {
    for (int index = 0; index < camera_count; index++) {
        if (strcmp(cameras[index].camera_uuid, camera_uuid) == 0) return true;
    }
    return false;
}

static cJSON *favorite_json(const camera_favorite_t *favorite) {
    cJSON *object = cJSON_CreateObject();
    if (!object) return NULL;
    cJSON_AddStringToObject(object, "camera_uuid", favorite->camera_uuid);
    cJSON_AddNumberToObject(object, "created_at", (double)favorite->created_at);
    return object;
}

/* Serialises and frees root. */
static void send_json(http_response_t *res, int status, cJSON *root) {
    char *encoded = root ? cJSON_PrintUnformatted(root) : NULL;
    cJSON_Delete(root);
    if (!encoded) {
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    http_response_set_json(res, status, encoded);
    free(encoded);
}

static void send_db_error(http_response_t *res,
                          db_camera_favorite_result_t result) {
    switch (result) {
    case DB_CAMERA_FAVORITE_INVALID:
        http_response_set_json_error(res, 400, "Invalid camera UUID");
        break;
    case DB_CAMERA_FAVORITE_NOT_FOUND:
        http_response_set_json_error(res, 404, "Camera not found");
        break;
    default:
        http_response_set_json_error(res, 500, "Failed to update camera favorites");
        break;
    }
}

void handle_get_camera_favorites(const http_request_t *req,
                                 http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;

    cJSON *root = cJSON_CreateObject();
    cJSON *items = root ? cJSON_AddArrayToObject(root, "favorites") : NULL;
    if (!root || !items) {
        cJSON_Delete(root);
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    bool can_modify = !read_only_identity(&user);
    cJSON_AddBoolToObject(root, "can_modify", can_modify);

    if (can_modify) {
        camera_favorite_t *favorites = calloc(CAMERA_FAVORITES_MAX_LIST,
                                              sizeof(*favorites));
        fleet_camera_t *cameras = NULL;
        int camera_count = 0;
        int count = favorites
            ? db_camera_favorites_list(user.id, favorites, CAMERA_FAVORITES_MAX_LIST)
            : -1;
        bool cameras_ok = count >= 0 &&
            load_authorized_cameras(&user, &cameras, &camera_count);
        if (count < 0 || !cameras_ok) {
            free(favorites);
            free(cameras);
            cJSON_Delete(root);
            http_response_set_json_error(res, 500, "Failed to load camera favorites");
            return;
        }
        for (int index = 0; index < count; index++) {
            if (!camera_allowed(favorites[index].camera_uuid, cameras,
                                camera_count)) continue;
            cJSON *item = favorite_json(&favorites[index]);
            if (item) cJSON_AddItemToArray(items, item);
        }
        free(favorites);
        free(cameras);
    }

    send_json(res, 200, root);
}

void handle_put_camera_favorite(const http_request_t *req,
                                http_response_t *res) {
    user_t user;
    char camera_uuid[CAMERA_UUID_STRING_SIZE] = {0};
    if (!authenticate(req, res, &user)) return;
    if (!require_interactive_user(&user, res)) return;
    if (!extract_camera_uuid(req, res, camera_uuid, sizeof(camera_uuid))) return;

    fleet_camera_t *cameras = NULL;
    int camera_count = 0;
    if (!load_authorized_cameras(&user, &cameras, &camera_count)) {
        free(cameras);
        http_response_set_json_error(res, 500, "Failed to check camera access");
        return;
    }
    bool allowed = camera_allowed(camera_uuid, cameras, camera_count);
    free(cameras);
    if (!allowed) {
        /* Unknown and out-of-scope cameras look the same to the caller. */
        http_response_set_json_error(res, 404, "Camera not found");
        return;
    }

    camera_favorite_t favorite;
    db_camera_favorite_result_t result =
        db_camera_favorite_add(user.id, camera_uuid, &favorite);
    if (result != DB_CAMERA_FAVORITE_OK) {
        send_db_error(res, result);
        return;
    }
    send_json(res, 200, favorite_json(&favorite));
}

void handle_delete_camera_favorite(const http_request_t *req,
                                   http_response_t *res) {
    user_t user;
    char camera_uuid[CAMERA_UUID_STRING_SIZE] = {0};
    if (!authenticate(req, res, &user)) return;
    if (!require_interactive_user(&user, res)) return;
    if (!extract_camera_uuid(req, res, camera_uuid, sizeof(camera_uuid))) return;

    bool removed = false;
    db_camera_favorite_result_t result =
        db_camera_favorite_remove(user.id, camera_uuid, &removed);
    if (result != DB_CAMERA_FAVORITE_OK) {
        send_db_error(res, result);
        return;
    }
    cJSON *root = cJSON_CreateObject();
    if (root) {
        cJSON_AddBoolToObject(root, "success", true);
        cJSON_AddBoolToObject(root, "removed", removed);
    }
    send_json(res, 200, root);
}
