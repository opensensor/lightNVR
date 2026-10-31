/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#define _GNU_SOURCE
#define _POSIX_C_SOURCE 200809L

#include "web/api_handlers_operator_floor_plans.h"

#include <cjson/cJSON.h>
#include <ctype.h>
#include <fcntl.h>
#include <errno.h>
#ifdef __linux__
#include <sys/syscall.h>
#endif
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
#include "core/logger.h"
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

// Layout sketch bounds. Shapes use normalized plan coordinates (0..1) so
// they survive canvas resizes and background swaps like camera markers do.
#define SKETCH_MAX_SHAPES 400
#define SKETCH_MAX_POINTS 64
#define SKETCH_MIN_POLYLINE_POINTS 2
#define SKETCH_MIN_POLYGON_POINTS 3
#define SKETCH_MIN_RECT_SIDE 0.005
#define SKETCH_ID_MAX 32
#define SKETCH_TEXT_MAX 96

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

static const char *const SKETCH_TONES[] = {
    "slate", "blue", "green", "amber", "red", "violet",
};
static const char *const SKETCH_LABEL_SIZES[] = {"sm", "md", "lg"};

static bool sketch_enum_field(const cJSON *item, const char *key,
                              const char *const *allowed, size_t allowed_count,
                              const char **value) {
    const cJSON *field = cJSON_GetObjectItemCaseSensitive(item, key);
    if (!field) {
        *value = allowed[0];
        return true;
    }
    if (!cJSON_IsString(field) || !field->valuestring) return false;
    for (size_t index = 0; index < allowed_count; index++) {
        if (strcmp(field->valuestring, allowed[index]) == 0) {
            *value = allowed[index];
            return true;
        }
    }
    return false;
}

static bool sketch_text_valid(const char *text, size_t minimum,
                              size_t maximum) {
    size_t length = strnlen(text, maximum + 1);
    if (length < minimum || length > maximum) return false;
    for (const unsigned char *cursor = (const unsigned char *)text; *cursor;
         cursor++) {
        if (*cursor < 0x20 || *cursor == 0x7f) return false;
    }
    return true;
}

static bool sketch_optional_text(const cJSON *item, const char *key,
                                 const char **value) {
    const cJSON *field = cJSON_GetObjectItemCaseSensitive(item, key);
    *value = NULL;
    if (!field || cJSON_IsNull(field)) return true;
    if (!cJSON_IsString(field) || !field->valuestring) return false;
    if (field->valuestring[0] == '\0') return true;
    if (!sketch_text_valid(field->valuestring, 1, SKETCH_TEXT_MAX)) {
        return false;
    }
    *value = field->valuestring;
    return true;
}

static bool sketch_id_valid(const char *id) {
    size_t length = strnlen(id, SKETCH_ID_MAX + 1);
    if (length == 0 || length > SKETCH_ID_MAX) return false;
    for (const char *cursor = id; *cursor; cursor++) {
        if (!isalnum((unsigned char)*cursor) && *cursor != '_' &&
            *cursor != '-') return false;
    }
    return true;
}

static double sketch_round(double value) {
    return round(value * 10000.0) / 10000.0;
}

static bool sketch_unit_field(const cJSON *item, const char *key,
                              double minimum, double *value) {
    const cJSON *field = cJSON_GetObjectItemCaseSensitive(item, key);
    if (!cJSON_IsNumber(field) || !isfinite(field->valuedouble) ||
        field->valuedouble < minimum || field->valuedouble > 1.0) {
        return false;
    }
    *value = sketch_round(field->valuedouble);
    return true;
}

static bool sketch_points(const cJSON *item, int minimum, cJSON *out) {
    const cJSON *points = cJSON_GetObjectItemCaseSensitive(item, "points");
    if (!cJSON_IsArray(points)) return false;
    int count = cJSON_GetArraySize(points);
    if (count < minimum || count > SKETCH_MAX_POINTS) return false;
    cJSON *canonical = cJSON_AddArrayToObject(out, "points");
    if (!canonical) return false;
    for (int index = 0; index < count; index++) {
        const cJSON *point = cJSON_GetArrayItem(points, index);
        if (!cJSON_IsArray(point) || cJSON_GetArraySize(point) != 2) {
            return false;
        }
        const cJSON *x = cJSON_GetArrayItem(point, 0);
        const cJSON *y = cJSON_GetArrayItem(point, 1);
        if (!cJSON_IsNumber(x) || !cJSON_IsNumber(y) ||
            !isfinite(x->valuedouble) || !isfinite(y->valuedouble) ||
            x->valuedouble < 0.0 || x->valuedouble > 1.0 ||
            y->valuedouble < 0.0 || y->valuedouble > 1.0) return false;
        cJSON *pair = cJSON_CreateArray();
        if (!pair) return false;
        cJSON_AddItemToArray(pair, cJSON_CreateNumber(
            sketch_round(x->valuedouble)));
        cJSON_AddItemToArray(pair, cJSON_CreateNumber(
            sketch_round(y->valuedouble)));
        cJSON_AddItemToArray(canonical, pair);
    }
    return true;
}

/**
 * Validate one client shape and append its canonical form (known fields
 * only, rounded coordinates) to `shapes`.
 */
static bool sketch_shape(const cJSON *item, cJSON *shapes) {
    if (!cJSON_IsObject(item)) return false;
    const cJSON *type = cJSON_GetObjectItemCaseSensitive(item, "type");
    const cJSON *id = cJSON_GetObjectItemCaseSensitive(item, "id");
    const char *tone = NULL;
    if (!cJSON_IsString(type) || !type->valuestring ||
        !cJSON_IsString(id) || !id->valuestring ||
        !sketch_id_valid(id->valuestring) ||
        !sketch_enum_field(item, "tone", SKETCH_TONES,
                           sizeof(SKETCH_TONES) / sizeof(SKETCH_TONES[0]),
                           &tone)) return false;
    cJSON *out = cJSON_CreateObject();
    if (!out) return false;
    cJSON_AddItemToArray(shapes, out);
    cJSON_AddStringToObject(out, "id", id->valuestring);
    cJSON_AddStringToObject(out, "type", type->valuestring);
    cJSON_AddStringToObject(out, "tone", tone);
    const char *text = NULL;
    if (strcmp(type->valuestring, "rect") == 0) {
        double x, y, w, h;
        const cJSON *filled = cJSON_GetObjectItemCaseSensitive(item, "filled");
        if (!sketch_unit_field(item, "x", 0.0, &x) ||
            !sketch_unit_field(item, "y", 0.0, &y) ||
            !sketch_unit_field(item, "w", SKETCH_MIN_RECT_SIDE, &w) ||
            !sketch_unit_field(item, "h", SKETCH_MIN_RECT_SIDE, &h) ||
            x + w > 1.0 + 1e-6 || y + h > 1.0 + 1e-6 ||
            (filled && !cJSON_IsBool(filled)) ||
            !sketch_optional_text(item, "label", &text)) return false;
        cJSON_AddNumberToObject(out, "x", x);
        cJSON_AddNumberToObject(out, "y", y);
        cJSON_AddNumberToObject(out, "w", fmin(w, 1.0 - x));
        cJSON_AddNumberToObject(out, "h", fmin(h, 1.0 - y));
        cJSON_AddBoolToObject(out, "filled", !filled || cJSON_IsTrue(filled));
        if (text) cJSON_AddStringToObject(out, "label", text);
        return true;
    }
    if (strcmp(type->valuestring, "wall") == 0) {
        return sketch_points(item, SKETCH_MIN_POLYLINE_POINTS, out);
    }
    if (strcmp(type->valuestring, "area") == 0) {
        if (!sketch_points(item, SKETCH_MIN_POLYGON_POINTS, out) ||
            !sketch_optional_text(item, "label", &text)) return false;
        if (text) cJSON_AddStringToObject(out, "label", text);
        return true;
    }
    if (strcmp(type->valuestring, "label") == 0) {
        double x, y;
        const char *size = NULL;
        const cJSON *label = cJSON_GetObjectItemCaseSensitive(item, "text");
        if (!sketch_unit_field(item, "x", 0.0, &x) ||
            !sketch_unit_field(item, "y", 0.0, &y) ||
            !cJSON_IsString(label) || !label->valuestring ||
            !sketch_text_valid(label->valuestring, 1, SKETCH_TEXT_MAX) ||
            !sketch_enum_field(item, "size", SKETCH_LABEL_SIZES,
                               sizeof(SKETCH_LABEL_SIZES) /
                                   sizeof(SKETCH_LABEL_SIZES[0]),
                               &size)) return false;
        cJSON_AddNumberToObject(out, "x", x);
        cJSON_AddNumberToObject(out, "y", y);
        cJSON_AddStringToObject(out, "text", label->valuestring);
        cJSON_AddStringToObject(out, "size", size);
        return true;
    }
    return false;
}

/**
 * Parse the optional "sketch" field into canonical JSON owned by the caller.
 * Absent -> *sketch_json = NULL (leave stored sketch unchanged). null or no
 * shapes -> "" (clear). Anything else is fully validated and re-encoded so
 * only known fields with bounded values ever reach storage.
 */
static bool parse_sketch(const cJSON *body, char **sketch_json,
                         http_response_t *res) {
    *sketch_json = NULL;
    const cJSON *sketch = cJSON_GetObjectItemCaseSensitive(body, "sketch");
    if (!sketch) return true;
    const cJSON *shapes = cJSON_IsObject(sketch)
        ? cJSON_GetObjectItemCaseSensitive(sketch, "shapes") : NULL;
    if (cJSON_IsNull(sketch) || (cJSON_IsObject(sketch) && !shapes) ||
        (cJSON_IsArray(shapes) && cJSON_GetArraySize(shapes) == 0)) {
        *sketch_json = strdup("");
        if (!*sketch_json) {
            http_response_set_json_error(res, 500, "Memory allocation failed");
            return false;
        }
        return true;
    }
    const cJSON *version = cJSON_IsObject(sketch)
        ? cJSON_GetObjectItemCaseSensitive(sketch, "version") : NULL;
    if (!cJSON_IsArray(shapes) ||
        (version && (!cJSON_IsNumber(version) || version->valueint != 1))) {
        http_response_set_json_error(res, 400, "Invalid floor plan sketch");
        return false;
    }
    int count = cJSON_GetArraySize(shapes);
    if (count > SKETCH_MAX_SHAPES) {
        http_response_set_json_error(res, 400,
                                     "Too many floor plan sketch shapes");
        return false;
    }
    cJSON *canonical = cJSON_CreateObject();
    cJSON *out_shapes = canonical
        ? cJSON_AddArrayToObject(canonical, "shapes") : NULL;
    if (!canonical || !out_shapes ||
        !cJSON_AddNumberToObject(canonical, "version", 1)) {
        cJSON_Delete(canonical);
        http_response_set_json_error(res, 500, "Memory allocation failed");
        return false;
    }
    bool valid = true;
    for (int index = 0; valid && index < count; index++) {
        const cJSON *item = cJSON_GetArrayItem(shapes, index);
        valid = sketch_shape(item, out_shapes);
        const cJSON *id = valid
            ? cJSON_GetObjectItemCaseSensitive(item, "id") : NULL;
        for (int previous = 0; valid && previous < index; previous++) {
            const cJSON *other = cJSON_GetObjectItemCaseSensitive(
                cJSON_GetArrayItem(out_shapes, previous), "id");
            if (other && strcmp(other->valuestring, id->valuestring) == 0) {
                valid = false;
            }
        }
    }
    if (!valid) {
        cJSON_Delete(canonical);
        http_response_set_json_error(res, 400, "Invalid floor plan sketch");
        return false;
    }
    char *encoded = cJSON_PrintUnformatted(canonical);
    cJSON_Delete(canonical);
    if (!encoded) {
        http_response_set_json_error(res, 500, "Memory allocation failed");
        return false;
    }
    if (strlen(encoded) > OPERATOR_FLOOR_PLAN_SKETCH_MAX) {
        free(encoded);
        http_response_set_json_error(res, 400, "Floor plan sketch too large");
        return false;
    }
    *sketch_json = encoded;
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
    const fleet_camera_t *authorized, int authorized_count,
    const char *sketch_json) {
    cJSON *root = cJSON_CreateObject();
    cJSON *cameras = root ? cJSON_AddArrayToObject(root, "cameras") : NULL;
    if (!root || !cameras) {
        cJSON_Delete(root);
        return NULL;
    }
    // The stored sketch is canonical JSON written by parse_sketch, so a
    // parse failure means corruption; expose it as "no sketch" rather than
    // failing the whole plan.
    cJSON *sketch = sketch_json && sketch_json[0]
        ? cJSON_Parse(sketch_json) : NULL;
    if (sketch) cJSON_AddItemToObject(root, "sketch", sketch);
    else cJSON_AddNullToObject(root, "sketch");
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
    char *sketch_json = NULL;
    db_operator_floor_plan_sketch_load(uuid, &sketch_json);
    cJSON *json = plan_json(&plan, placements, placement_count,
                            authorized, authorized_count, sketch_json);
    free(sketch_json);
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
        char *sketch_json = NULL;
        if (plans[index].sketch_bytes > 0) {
            db_operator_floor_plan_sketch_load(plans[index].uuid,
                                               &sketch_json);
        }
        cJSON *item = plan_json(&plans[index], placements, placement_count,
                                authorized, authorized_count, sketch_json);
        free(sketch_json);
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
    char *sketch_json = NULL;
    if (!parse_plan(body, true, &plan, &placements, &placement_count, res) ||
        !parse_sketch(body, &sketch_json, res) ||
        !placements_allowed(&user, placements, placement_count, res)) {
        free(sketch_json);
        free(placements);
        cJSON_Delete(body);
        return;
    }
    db_operator_floor_plan_result_t result = db_operator_floor_plan_create(
        &plan, placements, placement_count, sketch_json);
    free(sketch_json);
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
    char *sketch_json = NULL;
    if (!parse_plan(body, false, &plan, &placements, &placement_count, res) ||
        !parse_revision(body, &revision) ||
        !parse_sketch(body, &sketch_json, res) ||
        !placements_allowed(&user, placements, placement_count, res)) {
        if (res->status_code < 400) {
            http_response_set_json_error(res, 400, "Invalid revision");
        }
        free(sketch_json);
        free(placements);
        cJSON_Delete(body);
        return;
    }
    safe_strcpy(plan.uuid, uuid, sizeof(plan.uuid), 0);
    db_operator_floor_plan_result_t result = db_operator_floor_plan_update(
        &plan, placements, placement_count, revision, sketch_json);
    free(sketch_json);
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

typedef struct {
    char directory[BACKGROUND_PATH_SIZE + 32];
    char path[BACKGROUND_PATH_SIZE + 48];
    int descriptor;
    int parent;
    int old_file;
    int upload_file;
    char upload_name[64];
    struct stat upload_identity;
    struct stat parent_identity;
    bool created;
    bool validated;
    struct stat identity;
} background_backup_t;

static bool create_background_backup(const char *destination, background_backup_t *backup) {
    int length = snprintf(backup->directory, sizeof(backup->directory), "%s.backup.XXXXXX", destination);
    if (length < 0 || (size_t)length >= sizeof(backup->directory) || !mkdtemp(backup->directory)) return false;
    backup->created = true;
    backup->descriptor = open(backup->directory, O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC);
    struct stat named;
    if (backup->descriptor < 0 || fstat(backup->descriptor, &backup->identity) != 0 ||
        lstat(backup->directory, &named) != 0 || !S_ISDIR(backup->identity.st_mode) ||
        backup->identity.st_uid != geteuid() || (backup->identity.st_mode & 0777) != 0700 ||
        named.st_dev != backup->identity.st_dev || named.st_ino != backup->identity.st_ino) return false;
    backup->validated = true;
    length = snprintf(backup->path, sizeof(backup->path), "%s/original", backup->directory);
    return length > 0 && (size_t)length < sizeof(backup->path);
}

static void finish_background_backup(background_backup_t *backup) {
    if (backup->created) {
        struct stat current;
        /* Never recurse. Failed recovery leaves the original in this private
         * directory for inspection; empty-directory retirement is best effort. */
        if (!backup->validated || backup->descriptor < 0 || lstat(backup->directory, &current) != 0 ||
            current.st_dev != backup->identity.st_dev || current.st_ino != backup->identity.st_ino ||
            rmdir(backup->directory) != 0)
            log_warn("Floor-plan backup directory retained for inspection: %s", backup->directory);
    }
    if (backup->descriptor >= 0) close(backup->descriptor);
    backup->descriptor = -1;
    if (backup->parent >= 0) close(backup->parent);
    if (backup->old_file >= 0) close(backup->old_file);
    if (backup->upload_file >= 0) close(backup->upload_file);
    backup->parent = backup->old_file = backup->upload_file = -1;
}

static bool background_same_inode(const struct stat *a, const struct stat *b) {
    return a->st_dev == b->st_dev && a->st_ino == b->st_ino;
}

static int background_move_noreplace(int from, const char *source, int to, const char *destination) {
#if defined(__linux__) && defined(SYS_renameat2)
    return (int)syscall(SYS_renameat2, from, source, to, destination, 1U);
#else
    (void)from; (void)source; (void)to; (void)destination;
    errno = ENOSYS;
    return -1;
#endif
}

/* 1 means a bound original, 0 means absent, -1 rejects without metadata change. */
static int bind_background_original(const char *destination, const char *directory,
                                    background_backup_t *backup) {
    const char *leaf = strrchr(destination, '/');
    leaf = leaf ? leaf + 1 : destination;
    if (backup->parent < 0) backup->parent = open(directory, O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC);
    struct stat parent_identity, named, original;
    if (backup->parent < 0 || fstat(backup->parent, &parent_identity) != 0) return -1;
#ifdef O_PATH
    backup->old_file = openat(backup->parent, leaf, O_PATH | O_NOFOLLOW | O_CLOEXEC);
#else
    errno = ENOTSUP;
    return -1; /* No safe special-file pinning facility on this platform. */
#endif
    if (backup->old_file < 0) return errno == ENOENT ? 0 : -1;
    if (fstat(backup->old_file, &original) != 0 || !S_ISREG(original.st_mode)) return -1;
    if (!backup->created && !create_background_backup(destination, backup)) return -1;
    const char *private_leaf = strrchr(backup->directory, '/');
    private_leaf = private_leaf ? private_leaf + 1 : backup->directory;
    if (lstat(directory, &named) != 0 || !background_same_inode(&parent_identity, &named) ||
        fstatat(backup->parent, private_leaf, &named, AT_SYMLINK_NOFOLLOW) != 0 ||
        !background_same_inode(&backup->identity, &named)) return -1;
    if (background_move_noreplace(backup->parent, leaf, backup->descriptor, "original") != 0) return -1;
    if (fstatat(backup->descriptor, "original", &named, AT_SYMLINK_NOFOLLOW) != 0 ||
        !S_ISREG(named.st_mode) || !background_same_inode(&original, &named) ||
        original.st_size != named.st_size || original.st_mtim.tv_sec != named.st_mtim.tv_sec ||
        original.st_mtim.tv_nsec != named.st_mtim.tv_nsec) {
        if (background_move_noreplace(backup->descriptor, "original", backup->parent, leaf) != 0)
            log_warn("Changed floor-plan entry retained in private backup: %s", backup->path);
        return -1;
    }
    return 1;
}

static bool background_private_matches(const background_backup_t *backup, const char *name, int fd) {
    struct stat held, named;
    return fd >= 0 && fstat(fd, &held) == 0 && S_ISREG(held.st_mode) &&
        fstatat(backup->descriptor, name, &named, AT_SYMLINK_NOFOLLOW) == 0 &&
        S_ISREG(named.st_mode) && background_same_inode(&held, &named);
}

static bool background_remove_private(const background_backup_t *backup, const char *name, int fd) {
    struct stat named;
    if (fstatat(backup->descriptor, name, &named, AT_SYMLINK_NOFOLLOW) != 0)
        return errno == ENOENT;
    if (background_private_matches(backup, name, fd) && unlinkat(backup->descriptor, name, 0) == 0)
        return true;
    log_warn("Unknown or unavailable floor-plan private artifact retained: %s/%s", backup->directory, name);
    return false;
}

static bool background_parent_matches(const background_backup_t *backup, const char *directory) {
    struct stat named;
    return backup->parent >= 0 && lstat(directory, &named) == 0 &&
        S_ISDIR(named.st_mode) && background_same_inode(&backup->parent_identity, &named);
}

static bool background_publication_matches(const background_backup_t *backup, const char *leaf,
                                            const char *directory) {
    struct stat held, named;
    return background_parent_matches(backup, directory) &&
        fstat(backup->upload_file, &held) == 0 &&
        fstatat(backup->parent, leaf, &named, AT_SYMLINK_NOFOLLOW) == 0 &&
        S_ISREG(named.st_mode) && background_same_inode(&held, &named) &&
        held.st_size == backup->upload_identity.st_size &&
        held.st_mtim.tv_sec == backup->upload_identity.st_mtim.tv_sec &&
        held.st_mtim.tv_nsec == backup->upload_identity.st_mtim.tv_nsec;
}

static bool restore_background_original(const background_backup_t *backup, const char *leaf) {
    if (background_private_matches(backup, "original", backup->old_file) &&
        background_move_noreplace(backup->descriptor, "original", backup->parent, leaf) == 0)
        return true;
    log_warn("Floor-plan original retained after restoration conflict/failure: %s", backup->path);
    return false;
}

/* Bind a failed publication inside our private directory before deciding whether
 * it is this request's upload. Never stat then unlink the public pathname. */
static void recover_background_publication(const background_backup_t *backup, const char *leaf) {
    if (background_move_noreplace(backup->parent, leaf, backup->descriptor, "failed-upload") != 0) {
        if (errno != ENOENT) log_warn("Floor-plan failed publication retained at public entry: %s", leaf);
        return;
    }
    if (background_private_matches(backup, "failed-upload", backup->upload_file)) {
        background_remove_private(backup, "failed-upload", backup->upload_file);
    } else if (background_move_noreplace(backup->descriptor, "failed-upload", backup->parent, leaf) != 0) {
        log_warn("Changed floor-plan publication retained privately: %s/failed-upload", backup->directory);
    }
}

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

static bool write_background_temporary(background_backup_t *backup,
                                       const unsigned char *body, size_t body_len) {
    int descriptor = -1;
    for (unsigned attempt = 0; attempt < 8; ++attempt) {
        char uuid[LIGHTNVR_UUID_STRING_SIZE];
        if (lightnvr_uuid_generate_v4(uuid) != 0) return false;
        int length = snprintf(backup->upload_name, sizeof(backup->upload_name), "upload.tmp.%s", uuid);
        if (length < 0 || (size_t)length >= sizeof(backup->upload_name)) return false;
        descriptor = openat(backup->descriptor, backup->upload_name,
                            O_WRONLY | O_CREAT | O_EXCL | O_NOFOLLOW | O_CLOEXEC, 0600);
        if (descriptor >= 0 || errno != EEXIST) break;
    }
    if (descriptor < 0) return false;
    int held = fcntl(descriptor, F_DUPFD_CLOEXEC, 0);
    if (held < 0) {
        background_remove_private(backup, backup->upload_name, descriptor);
        close(descriptor);
        return false;
    }
    FILE *file = fdopen(descriptor, "wb");
    if (!file) {
        background_remove_private(backup, backup->upload_name, held);
        close(descriptor);
        close(held);
        return false;
    }
    bool written = fwrite(body, 1, body_len, file) == body_len &&
        fflush(file) == 0 && fsync(fileno(file)) == 0;
    if (fclose(file) != 0) written = false;
    if (!written) {
        background_remove_private(backup, backup->upload_name, held);
        close(held);
    } else backup->upload_file = held;
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
    db_operator_floor_plan_result_t result = db_operator_floor_plan_get(uuid, &existing);
    if (result != DB_OPERATOR_FLOOR_PLAN_OK) {
        pthread_mutex_unlock(&background_files_mutex);
        set_db_error(res, result);
        return;
    }
    background_backup_t backup = {.descriptor = -1, .parent = -1, .old_file = -1, .upload_file = -1};
    bool backed_up = false, published = false;
    const char *leaf = strrchr(path, '/');
    leaf = leaf ? leaf + 1 : path;
    if (mkdir_recursive(directory) != 0) goto background_failure;
    backup.parent = open(directory, O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC);
    if (backup.parent < 0 || fstat(backup.parent, &backup.parent_identity) != 0 ||
        !create_background_backup(path, &backup) || !background_parent_matches(&backup, directory))
        goto background_failure;
    if (!write_background_temporary(&backup, req->body, req->body_len)) goto background_failure;
    if (!background_private_matches(&backup, backup.upload_name, backup.upload_file) ||
        fstat(backup.upload_file, &backup.upload_identity) != 0) goto background_failure;

    if (strcmp(existing.background_mime, mime) == 0) {
        int bound = bind_background_original(path, directory, &backup);
        if (bound < 0) goto background_failure;
        backed_up = bound > 0;
    }
    if (!background_parent_matches(&backup, directory) ||
        background_move_noreplace(backup.descriptor, backup.upload_name, backup.parent, leaf) != 0)
        goto background_failure;
    published = true;
    if (fsync(backup.parent) != 0 || !background_publication_matches(&backup, leaf, directory))
        goto background_failure;
    /* Filesystem identity and SQLite metadata cannot be committed atomically.
     * This final snapshot detects changes already visible before the DB call. */
    result = db_operator_floor_plan_set_background(uuid, mime);
    if (result != DB_OPERATOR_FLOOR_PLAN_OK) goto background_failure;
    if (backed_up) background_remove_private(&backup, "original", backup.old_file);
    char obsolete[BACKGROUND_PATH_SIZE];
    const char *other = strcmp(mime, "image/png") == 0 ? "image/jpeg" : "image/png";
    if (background_file_path(uuid, other, obsolete, sizeof(obsolete))) unlink(obsolete);
    finish_background_backup(&backup);
    pthread_mutex_unlock(&background_files_mutex);
    cJSON *json = load_one_json(uuid, &user, res);
    if (!json) return;
    send_json(res, 200, json);
    cJSON_Delete(json);
    return;

background_failure:
    if (published) recover_background_publication(&backup, leaf);
    bool restored = !backed_up || restore_background_original(&backup, leaf);
    if (restored && backup.upload_name[0])
        background_remove_private(&backup, backup.upload_name, backup.upload_file);
    finish_background_backup(&backup);
    pthread_mutex_unlock(&background_files_mutex);
    if (result != DB_OPERATOR_FLOOR_PLAN_OK) set_db_error(res, result);
    else http_response_set_json_error(res, 500, "Failed to store floor plan background");
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
