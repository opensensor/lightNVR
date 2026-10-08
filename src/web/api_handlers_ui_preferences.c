#define _POSIX_C_SOURCE 200809L

#include "web/api_handlers_ui_preferences.h"

#include <cjson/cJSON.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "core/authorization.h"
#include "database/db_fleet_query.h"
#include "database/db_streams.h"
#include "database/db_ui_preferences.h"
#include "utils/strings.h"
#include "utils/uuid.h"
#include "web/httpd_utils.h"

#define UI_FAVORITES_MAX MAX_STREAMS

static bool authenticate(const http_request_t *req, http_response_t *res,
                         user_t *user) {
    memset(user, 0, sizeof(*user));
    if (!httpd_check_action_access(req, user)) {
        http_response_set_json_error(res, 401, "Требуется аутентификация");
        return false;
    }
    return true;
}

static bool configurable_user(const user_t *user) {
    return user && user->id > 0 && !user->authenticated_via_scoped_token &&
        strcmp(user->authentication_method, "demo") != 0;
}

static void send_json(http_response_t *res, int status, cJSON *json) {
    char *encoded = json ? cJSON_PrintUnformatted(json) : NULL;
    if (!encoded) {
        http_response_set_json_error(res, 500, "Не удалось сформировать ответ");
        return;
    }
    http_response_set_json(res, status, encoded);
    free(encoded);
}

static bool exact_object_keys(const cJSON *body, const char *key) {
    if (!cJSON_IsObject(body) || cJSON_GetArraySize(body) != 1) return false;
    const cJSON *item = body->child;
    return item && item->string && strcmp(item->string, key) == 0;
}

static bool load_cameras(const user_t *user, fleet_camera_t **cameras,
                         int *count) {
    *cameras = NULL;
    *count = 0;
    return db_fleet_camera_load(cameras, count) == 0 &&
        authorization_filter_cameras(user, AUTHZ_LIVE_VIEW, *cameras, count) == 0;
}

static bool camera_is_allowed(const fleet_camera_t *cameras, int count,
                              const char *uuid) {
    for (int i = 0; i < count; i++) {
        if (strcmp(cameras[i].camera_uuid, uuid) == 0) return true;
    }
    return false;
}

static void send_preferences(http_response_t *res, const user_t *user,
                             bool configurable) {
    char mode[8] = "auto";
    if (configurable && db_user_ui_mode_get(user->id, mode, sizeof(mode)) != 0) {
        http_response_set_json_error(res, 500, "Не удалось загрузить настройки интерфейса");
        return;
    }
    cJSON *root = cJSON_CreateObject();
    if (!root) {
        http_response_set_json_error(res, 500, "Не удалось сформировать ответ");
        return;
    }
    cJSON_AddStringToObject(root, "ui_mode", mode);
    cJSON_AddBoolToObject(root, "configurable", configurable);
    send_json(res, 200, root);
    cJSON_Delete(root);
}

void handle_get_ui_preferences(const http_request_t *req, http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;
    send_preferences(res, &user, configurable_user(&user));
}

void handle_put_ui_preferences(const http_request_t *req, http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;
    if (!configurable_user(&user)) {
        http_response_set_json_error(res, 403, "Настройки доступны только вошедшему пользователю");
        return;
    }
    cJSON *body = httpd_parse_json_body(req);
    const cJSON *mode = exact_object_keys(body, "ui_mode")
        ? cJSON_GetObjectItemCaseSensitive(body, "ui_mode") : NULL;
    if (!cJSON_IsString(mode) || !mode->valuestring ||
        (strcmp(mode->valuestring, "auto") != 0 &&
         strcmp(mode->valuestring, "mobile") != 0 &&
         strcmp(mode->valuestring, "desktop") != 0)) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400, "Ожидается объект только с полем ui_mode: auto, mobile или desktop");
        return;
    }
    int rc = db_user_ui_mode_set(user.id, mode->valuestring);
    cJSON_Delete(body);
    if (rc != 0) {
        http_response_set_json_error(res, 500, "Не удалось сохранить настройки интерфейса");
        return;
    }
    send_preferences(res, &user, true);
}

void handle_get_ui_favorites(const http_request_t *req, http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;
    bool configurable = configurable_user(&user);
    cJSON *root = cJSON_CreateObject();
    cJSON *items = root ? cJSON_AddArrayToObject(root, "favorites") : NULL;
    if (!root || !items) {
        cJSON_Delete(root);
        http_response_set_json_error(res, 500, "Не удалось сформировать ответ");
        return;
    }
    if (configurable) {
        user_favorite_t *favorites = calloc(UI_FAVORITES_MAX, sizeof(*favorites));
        fleet_camera_t *cameras = NULL;
        int camera_count = 0;
        int count = favorites ? db_user_favorites_list(user.id, favorites,
            UI_FAVORITES_MAX) : -1;
        bool cameras_ok = load_cameras(&user, &cameras, &camera_count);
        if (count < 0 || !cameras_ok) {
            free(favorites);
            free(cameras);
            cJSON_Delete(root);
            http_response_set_json_error(res, 500, "Не удалось загрузить избранные камеры");
            return;
        }
        for (int i = 0; i < count; i++) {
            if (!camera_is_allowed(cameras, camera_count, favorites[i].camera_uuid)) continue;
            cJSON *item = cJSON_CreateObject();
            if (!item) {
                free(favorites);
                free(cameras);
                cJSON_Delete(root);
                http_response_set_json_error(res, 500, "Не удалось сформировать ответ");
                return;
            }
            cJSON_AddStringToObject(item, "camera_uuid", favorites[i].camera_uuid);
            cJSON_AddNumberToObject(item, "created_at", (double)favorites[i].created_at);
            cJSON_AddItemToArray(items, item);
        }
        free(favorites);
        free(cameras);
    }
    cJSON_AddBoolToObject(root, "configurable", configurable);
    send_json(res, 200, root);
    cJSON_Delete(root);
}

void handle_post_ui_favorite(const http_request_t *req, http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;
    if (!configurable_user(&user)) {
        http_response_set_json_error(res, 403, "Избранное доступно только вошедшему пользователю");
        return;
    }
    cJSON *body = httpd_parse_json_body(req);
    const cJSON *uuid = exact_object_keys(body, "camera_uuid")
        ? cJSON_GetObjectItemCaseSensitive(body, "camera_uuid") : NULL;
    if (!cJSON_IsString(uuid) || !lightnvr_uuid_is_valid(uuid->valuestring)) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400, "Ожидается объект только с корректным полем camera_uuid");
        return;
    }
    char camera_uuid[CAMERA_UUID_STRING_SIZE];
    safe_strcpy(camera_uuid, uuid->valuestring, sizeof(camera_uuid), 0);
    cJSON_Delete(body);
    stream_config_t stream;
    fleet_camera_t *cameras = NULL;
    int camera_count = 0;
    if (get_stream_config_by_uuid(camera_uuid, &stream) != 0) {
        http_response_set_json_error(res, 404, "Камера не найдена");
        return;
    }
    if (!load_cameras(&user, &cameras, &camera_count)) {
        free(cameras);
        http_response_set_json_error(res, 500, "Не удалось проверить доступ к камере");
        return;
    }
    bool allowed = camera_is_allowed(cameras, camera_count, camera_uuid);
    free(cameras);
    if (!allowed) {
        http_response_set_json_error(res, 404, "Камера не найдена");
        return;
    }
    int64_t created_at = 0;
    if (db_user_favorite_add(user.id, camera_uuid, &created_at) != 0) {
        http_response_set_json_error(res, 500, "Не удалось сохранить избранную камеру");
        return;
    }
    cJSON *root = cJSON_CreateObject();
    if (!root) {
        http_response_set_json_error(res, 500, "Не удалось сформировать ответ");
        return;
    }
    cJSON_AddStringToObject(root, "camera_uuid", camera_uuid);
    cJSON_AddNumberToObject(root, "created_at", (double)created_at);
    send_json(res, 200, root);
    cJSON_Delete(root);
}

void handle_delete_ui_favorite(const http_request_t *req, http_response_t *res) {
    user_t user;
    char uuid[CAMERA_UUID_STRING_SIZE] = {0};
    if (!authenticate(req, res, &user)) return;
    if (!configurable_user(&user)) {
        http_response_set_json_error(res, 403, "Избранное доступно только вошедшему пользователю");
        return;
    }
    if (http_request_extract_path_param(req, "/api/ui/favorites/", uuid,
            sizeof(uuid)) != 0 || !lightnvr_uuid_is_valid(uuid)) {
        http_response_set_json_error(res, 400, "Некорректный UUID камеры");
        return;
    }
    if (db_user_favorite_delete(user.id, uuid) != 0) {
        http_response_set_json_error(res, 500, "Не удалось удалить избранную камеру");
        return;
    }
    http_response_set_json(res, 200, "{\"success\":true}");
}
