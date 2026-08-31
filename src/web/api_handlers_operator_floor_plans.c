#define _POSIX_C_SOURCE 200809L

#include "web/api_handlers_operator_floor_plans.h"

#include <cjson/cJSON.h>
#include <ctype.h>
#include <math.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <sys/stat.h>
#include <unistd.h>

#include "core/authorization.h"
#include "core/config.h"
#include "core/path_utils.h"
#include "database/db_auth.h"
#include "database/db_fleet_query.h"
#include "database/db_operator_floor_plans.h"
#include "utils/strings.h"
#include "utils/uuid.h"
#include "web/httpd_utils.h"

// The libuv receive buffer caps whole requests at 1 MB, so a raw binary PUT
// must stay comfortably below that once headers are accounted for.
#define OPERATOR_FLOOR_PLAN_BACKGROUND_MAX (768 * 1024)

// A background change spans both SQLite and the filesystem. Serialize the
// whole operation so concurrent replace/remove requests cannot leave the MIME
// metadata pointing at a file removed by another request.
static pthread_mutex_t background_files_mutex = PTHREAD_MUTEX_INITIALIZER;

static void remove_background_files(const char *uuid);

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
    if (plan->background_mime[0]) {
        cJSON_AddStringToObject(root, "background_mime",
                                plan->background_mime);
    } else {
        cJSON_AddNullToObject(root, "background_mime");
    }
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
    remove_background_files(uuid);
    cJSON *json = cJSON_CreateObject();
    cJSON_AddBoolToObject(json, "deleted", true);
    send_json(res, 200, json);
    cJSON_Delete(json);
}

static const char *background_extension(const char *mime) {
    if (strcmp(mime, "image/png") == 0) return ".png";
    if (strcmp(mime, "image/jpeg") == 0) return ".jpg";
    return NULL;
}

static bool background_mime_from_content_type(const char *value,
                                              const char **mime) {
    size_t length;
    if (!value) return false;
    if (strncasecmp(value, "image/png", 9) == 0) {
        *mime = "image/png";
        length = 9;
    } else if (strncasecmp(value, "image/jpeg", 10) == 0) {
        *mime = "image/jpeg";
        length = 10;
    } else {
        return false;
    }
    const char *suffix = value + length;
    while (isspace((unsigned char)*suffix)) suffix++;
    return *suffix == '\0' || *suffix == ';';
}

static bool background_matches_mime(const char *mime,
                                    const unsigned char *body,
                                    size_t body_len) {
    static const unsigned char png_signature[] = {
        0x89, 'P', 'N', 'G', 0x0D, 0x0A, 0x1A, 0x0A};
    if (strcmp(mime, "image/png") == 0) {
        return body_len >= sizeof(png_signature) &&
            memcmp(body, png_signature, sizeof(png_signature)) == 0;
    }
    return body_len >= 3 && body[0] == 0xFF && body[1] == 0xD8 &&
        body[2] == 0xFF;
}

// Storage path is bounded by MAX_PATH_LENGTH; the extra room keeps the
// floor_plans/<uuid>.<ext> suffix from ever truncating.
#define BACKGROUND_PATH_SIZE (MAX_PATH_LENGTH + 128)

static bool background_file_path(const char *uuid, const char *mime,
                                 char *path, size_t size) {
    const char *extension = background_extension(mime);
    if (!extension) return false;
    int length = snprintf(path, size, "%s/floor_plans/%s%s",
                          g_config.storage_path, uuid, extension);
    return length >= 0 && (size_t)length < size;
}

static void remove_background_files_locked(const char *uuid) {
    char path[BACKGROUND_PATH_SIZE];
    if (background_file_path(uuid, "image/png", path, sizeof(path))) {
        unlink(path);
        size_t length = strlen(path);
        if (length + 4 < sizeof(path)) {
            memcpy(path + length, ".bak", 5);
            unlink(path);
        }
    }
    if (background_file_path(uuid, "image/jpeg", path, sizeof(path))) {
        unlink(path);
        size_t length = strlen(path);
        if (length + 4 < sizeof(path)) {
            memcpy(path + length, ".bak", 5);
            unlink(path);
        }
    }
}

static void remove_background_files(const char *uuid) {
    pthread_mutex_lock(&background_files_mutex);
    remove_background_files_locked(uuid);
    pthread_mutex_unlock(&background_files_mutex);
}

static bool write_background_temporary(
    const char *destination, const unsigned char *body, size_t body_len,
    char *temporary, size_t temporary_size) {
    int length = snprintf(temporary, temporary_size, "%s.tmp.XXXXXX",
                          destination);
    if (length < 0 || (size_t)length >= temporary_size) return false;
    int descriptor = mkstemp(temporary);
    if (descriptor < 0) return false;
    FILE *file = fdopen(descriptor, "wb");
    if (!file) {
        close(descriptor);
        unlink(temporary);
        return false;
    }
    bool written = fwrite(body, 1, body_len, file) == body_len &&
        fflush(file) == 0 && fsync(fileno(file)) == 0;
    if (fclose(file) != 0) written = false;
    if (!written) unlink(temporary);
    return written;
}

static bool extract_background_target(const http_request_t *req, char *uuid,
                                      size_t uuid_size, http_response_t *res) {
    char param[MAX_PATH_LENGTH];
    if (http_request_extract_path_param(req, "/api/live/plans/", param,
                                        sizeof(param)) != 0) {
        http_response_set_json_error(res, 400, "Invalid floor plan path");
        return false;
    }
    char *slash = strchr(param, '/');
    if (!slash || slash == param || strcmp(slash + 1, "background") != 0) {
        http_response_set_json_error(res, 400, "Invalid floor plan path");
        return false;
    }
    *slash = '\0';
    if (!lightnvr_uuid_is_valid(param)) {
        http_response_set_json_error(res, 400, "Invalid floor plan path");
        return false;
    }
    safe_strcpy(uuid, param, uuid_size, 0);
    return true;
}

void handle_get_operator_floor_plan_background(const http_request_t *req,
                                               http_response_t *res) {
    user_t user;
    char uuid[CAMERA_UUID_STRING_SIZE];
    if (!authenticate(req, res, &user) ||
        !extract_background_target(req, uuid, sizeof(uuid), res)) return;
    operator_floor_plan_t plan;
    db_operator_floor_plan_result_t result =
        db_operator_floor_plan_get(uuid, &plan);
    if (result != DB_OPERATOR_FLOOR_PLAN_OK) {
        set_db_error(res, result);
        return;
    }
    if (!plan.background_mime[0]) {
        http_response_set_json_error(res, 404,
                                     "Floor plan background not found");
        return;
    }
    char path[BACKGROUND_PATH_SIZE];
    struct stat info;
    if (!background_file_path(uuid, plan.background_mime, path,
                              sizeof(path)) ||
        lstat(path, &info) != 0 || !S_ISREG(info.st_mode)) {
        http_response_set_json_error(res, 404,
                                     "Floor plan background not found");
        return;
    }
    if (http_serve_file(
            req, res, path, plan.background_mime,
            "Cache-Control: private, no-store\r\n"
            "X-Content-Type-Options: nosniff\r\n"
            "Content-Security-Policy: sandbox; default-src 'none'\r\n") != 0) {
        http_response_set_json_error(res, 500,
                                     "Failed to serve floor plan background");
    }
}

void handle_put_operator_floor_plan_background(const http_request_t *req,
                                               http_response_t *res) {
    user_t user;
    char uuid[CAMERA_UUID_STRING_SIZE];
    if (!require_modify(req, res, &user) ||
        !extract_background_target(req, uuid, sizeof(uuid), res)) return;
    const char *mime = NULL;
    if (!background_mime_from_content_type(
            http_request_get_header(req, "Content-Type"), &mime)) {
        http_response_set_json_error(
            res, 415, "Background must be an image/png or image/jpeg file");
        return;
    }
    if (!req->body || req->body_len < 4) {
        http_response_set_json_error(res, 400, "Background image is empty");
        return;
    }
    if (req->body_len > OPERATOR_FLOOR_PLAN_BACKGROUND_MAX) {
        http_response_set_json_error(
            res, 413, "Background image too large; maximum is 768 KB");
        return;
    }
    if (!background_matches_mime(mime, req->body, req->body_len)) {
        http_response_set_json_error(
            res, 400, "Background image does not match its declared type");
        return;
    }
    char directory[BACKGROUND_PATH_SIZE];
    char path[BACKGROUND_PATH_SIZE];
    char temporary[BACKGROUND_PATH_SIZE + 16];
    int directory_length = snprintf(directory, sizeof(directory),
                                    "%s/floor_plans", g_config.storage_path);
    if (directory_length < 0 ||
        (size_t)directory_length >= sizeof(directory) ||
        !background_file_path(uuid, mime, path, sizeof(path))) {
        http_response_set_json_error(res, 500,
                                     "Failed to store floor plan background");
        return;
    }

    pthread_mutex_lock(&background_files_mutex);
    operator_floor_plan_t existing;
    db_operator_floor_plan_result_t result =
        db_operator_floor_plan_get(uuid, &existing);
    if (result != DB_OPERATOR_FLOOR_PLAN_OK) {
        pthread_mutex_unlock(&background_files_mutex);
        set_db_error(res, result);
        return;
    }
    if (mkdir_recursive(directory) != 0 ||
        !write_background_temporary(path, req->body, req->body_len,
                                    temporary, sizeof(temporary))) {
        pthread_mutex_unlock(&background_files_mutex);
        http_response_set_json_error(res, 500,
                                     "Failed to store floor plan background");
        return;
    }

    // Preserve a same-type background until the metadata update succeeds, so
    // an SQLite failure does not turn a valid existing background into a 404.
    char backup[BACKGROUND_PATH_SIZE + 8];
    bool backed_up = false;
    int backup_length = snprintf(backup, sizeof(backup), "%s.bak", path);
    bool valid_backup_path = backup_length >= 0 &&
        (size_t)backup_length < sizeof(backup);
    if (valid_backup_path) unlink(backup);
    if (strcmp(existing.background_mime, mime) == 0 && valid_backup_path &&
        access(path, F_OK) == 0) {
        if (rename(path, backup) != 0) {
            unlink(temporary);
            pthread_mutex_unlock(&background_files_mutex);
            http_response_set_json_error(
                res, 500, "Failed to store floor plan background");
            return;
        }
        backed_up = true;
    }
    if (rename(temporary, path) != 0) {
        unlink(temporary);
        if (backed_up) rename(backup, path);
        pthread_mutex_unlock(&background_files_mutex);
        http_response_set_json_error(res, 500,
                                     "Failed to store floor plan background");
        return;
    }
    result = db_operator_floor_plan_set_background(uuid, mime);
    if (result != DB_OPERATOR_FLOOR_PLAN_OK) {
        unlink(path);
        if (backed_up) rename(backup, path);
        pthread_mutex_unlock(&background_files_mutex);
        set_db_error(res, result);
        return;
    }
    if (backed_up) unlink(backup);
    char obsolete[BACKGROUND_PATH_SIZE];
    const char *other = strcmp(mime, "image/png") == 0
        ? "image/jpeg" : "image/png";
    if (background_file_path(uuid, other, obsolete, sizeof(obsolete))) {
        unlink(obsolete);
    }
    pthread_mutex_unlock(&background_files_mutex);
    cJSON *json = load_one_json(uuid, &user, res);
    if (!json) return;
    send_json(res, 200, json);
    cJSON_Delete(json);
}

void handle_delete_operator_floor_plan_background(const http_request_t *req,
                                                  http_response_t *res) {
    user_t user;
    char uuid[CAMERA_UUID_STRING_SIZE];
    if (!require_modify(req, res, &user) ||
        !extract_background_target(req, uuid, sizeof(uuid), res)) return;
    pthread_mutex_lock(&background_files_mutex);
    operator_floor_plan_t existing;
    db_operator_floor_plan_result_t result =
        db_operator_floor_plan_get(uuid, &existing);
    if (result != DB_OPERATOR_FLOOR_PLAN_OK) {
        pthread_mutex_unlock(&background_files_mutex);
        set_db_error(res, result);
        return;
    }
    result = db_operator_floor_plan_set_background(uuid, NULL);
    if (result != DB_OPERATOR_FLOOR_PLAN_OK) {
        pthread_mutex_unlock(&background_files_mutex);
        set_db_error(res, result);
        return;
    }
    remove_background_files_locked(uuid);
    pthread_mutex_unlock(&background_files_mutex);
    cJSON *json = load_one_json(uuid, &user, res);
    if (!json) return;
    send_json(res, 200, json);
    cJSON_Delete(json);
}
