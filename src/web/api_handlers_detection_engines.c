#define _POSIX_C_SOURCE 200809L

#include "web/api_handlers_detection_engines.h"

#include <cjson/cJSON.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>

#include "core/authorization.h"
#include "database/db_detection_engines.h"
#include "utils/strings.h"
#include "web/api_handlers.h"
#include "web/audit_log.h"
#include "web/httpd_utils.h"

static bool stream_from_path(const http_request_t *req, char *stream_name,
                             size_t size) {
    if (http_request_extract_path_param(req, "/api/streams/", stream_name,
                                        size) != 0) return false;
    char *suffix = strstr(stream_name, "/detection-engines");
    if (!suffix || suffix[18] != '\0') return false;
    *suffix = '\0';
    return stream_name[0] != '\0';
}

static cJSON *engine_json(const stream_detection_engine_t *engine) {
    cJSON *object = cJSON_CreateObject();
    if (!object) return NULL;
    cJSON_AddStringToObject(object, "key", engine->engine_key);
    cJSON_AddStringToObject(object, "type", engine->engine_type);
    cJSON_AddStringToObject(object, "model_path", engine->model_path);
    cJSON_AddBoolToObject(object, "enabled", engine->enabled);
    cJSON_AddNumberToObject(object, "threshold", engine->threshold);
    cJSON_AddNumberToObject(object, "interval_seconds", engine->interval_seconds);
    cJSON_AddNumberToObject(object, "sort_order", engine->sort_order);
    cJSON_AddBoolToObject(object, "managed_by_legacy_stream_fields",
                          strcmp(engine->engine_key, "legacy-primary") == 0);
    cJSON *config = cJSON_Parse(engine->config_json);
    cJSON_AddItemToObject(object, "config",
                          config ? config : cJSON_CreateObject());
    return object;
}

static void send_engines(http_response_t *res, const char *stream_name) {
    stream_detection_engine_t engines[MAX_DETECTION_ENGINES_PER_STREAM];
    int count = db_detection_engines_list(
        stream_name, engines, MAX_DETECTION_ENGINES_PER_STREAM);
    if (count < 0) {
        http_response_set_json_error(res, 500, "Failed to load detection engines");
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
    cJSON_AddStringToObject(root, "stream_name", stream_name);
    cJSON_AddItemToObject(root, "engines", items);
    for (int i = 0; i < count; ++i) {
        cJSON *item = engine_json(&engines[i]);
        if (item) cJSON_AddItemToArray(items, item);
    }
    cJSON_AddNumberToObject(root, "count", count);
    cJSON_AddStringToObject(root, "trigger_policy", "any_of");
    cJSON_AddBoolToObject(root, "restart_required", false);
    char *encoded = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if (!encoded) {
        http_response_set_json_error(res, 500, "Failed to serialize response");
        return;
    }
    http_response_set_json(res, 200, encoded);
    free(encoded);
}

void handle_get_detection_engines(const http_request_t *req,
                                  http_response_t *res) {
    char stream_name[MAX_STREAM_NAME];
    if (!stream_from_path(req, stream_name, sizeof(stream_name))) {
        http_response_set_json_error(res, 400, "Invalid stream path");
        return;
    }
    if (!httpd_authorize_stream_action(req, res, AUTHZ_CAMERA_CONFIGURE,
                                       stream_name)) return;
    send_engines(res, stream_name);
}

static bool parse_engine(const cJSON *item, stream_detection_engine_t *engine) {
    if (!cJSON_IsObject(item)) return false;
    const cJSON *key = cJSON_GetObjectItemCaseSensitive(item, "key");
    const cJSON *type = cJSON_GetObjectItemCaseSensitive(item, "type");
    const cJSON *model = cJSON_GetObjectItemCaseSensitive(item, "model_path");
    const cJSON *enabled = cJSON_GetObjectItemCaseSensitive(item, "enabled");
    const cJSON *threshold = cJSON_GetObjectItemCaseSensitive(item, "threshold");
    const cJSON *interval = cJSON_GetObjectItemCaseSensitive(item, "interval_seconds");
    const cJSON *order = cJSON_GetObjectItemCaseSensitive(item, "sort_order");
    const cJSON *config = cJSON_GetObjectItemCaseSensitive(item, "config");
    if (!cJSON_IsString(key) || !cJSON_IsString(type) ||
        (model && !cJSON_IsString(model)) ||
        (enabled && !cJSON_IsBool(enabled)) ||
        !cJSON_IsNumber(threshold) || !isfinite(threshold->valuedouble) ||
        !cJSON_IsNumber(interval) || !isfinite(interval->valuedouble) ||
        floor(interval->valuedouble) != interval->valuedouble ||
        (order && (!cJSON_IsNumber(order) || !isfinite(order->valuedouble) ||
                   floor(order->valuedouble) != order->valuedouble)) ||
        (config && !cJSON_IsObject(config))) return false;
    memset(engine, 0, sizeof(*engine));
    safe_strcpy(engine->engine_key, key->valuestring,
                sizeof(engine->engine_key), 0);
    safe_strcpy(engine->engine_type, type->valuestring,
                sizeof(engine->engine_type), 0);
    if (model) safe_strcpy(engine->model_path, model->valuestring,
                           sizeof(engine->model_path), 0);
    engine->enabled = enabled ? cJSON_IsTrue(enabled) : true;
    engine->threshold = (float)threshold->valuedouble;
    engine->interval_seconds = interval->valueint;
    engine->sort_order = order ? order->valueint : 0;
    char *config_text = config ? cJSON_PrintUnformatted(config) : strdup("{}");
    if (!config_text || strlen(config_text) >= sizeof(engine->config_json)) {
        free(config_text);
        return false;
    }
    safe_strcpy(engine->config_json, config_text,
                sizeof(engine->config_json), 0);
    free(config_text);
    return db_detection_engine_validate(engine, NULL, 0) == 0;
}

void handle_put_detection_engines(const http_request_t *req,
                                  http_response_t *res) {
    char stream_name[MAX_STREAM_NAME];
    if (!stream_from_path(req, stream_name, sizeof(stream_name))) {
        http_response_set_json_error(res, 400, "Invalid stream path");
        return;
    }
    user_t user;
    fleet_camera_t camera;
    authorization_evaluation_t evaluation;
    if (!httpd_authorize_stream_action_with_context(
            req, res, AUTHZ_CAMERA_CONFIGURE, stream_name,
            &user, &camera, &evaluation)) return;

    cJSON *body = httpd_parse_json_body(req);
    const cJSON *items = body
        ? cJSON_GetObjectItemCaseSensitive(body, "engines") : NULL;
    int count = cJSON_IsArray(items) ? cJSON_GetArraySize(items) : -1;
    if (count < 0 || count >= MAX_DETECTION_ENGINES_PER_STREAM) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400, "Invalid engines array");
        return;
    }
    stream_detection_engine_t engines[MAX_DETECTION_ENGINES_PER_STREAM];
    memset(engines, 0, sizeof(engines));
    bool valid = true;
    for (int i = 0; i < count; ++i) {
        if (!parse_engine(cJSON_GetArrayItem(items, i), &engines[i])) {
            valid = false;
            break;
        }
    }
    cJSON_Delete(body);
    if (!valid) {
        http_response_set_json_error(res, 400, "Invalid detection engine");
        return;
    }
    int result = db_detection_engines_replace_custom(
        stream_name, engines, (size_t)count);
    cJSON *audit = cJSON_CreateObject();
    if (audit) cJSON_AddNumberToObject(audit, "custom_engine_count", count);
    audit_log_operation(req, &user, "camera.configure", "camera",
                        camera.camera_uuid, "detection_engines_replace",
                        result == 0 ? "success" : "failure", audit);
    cJSON_Delete(audit);
    if (result != 0) {
        http_response_set_json_error(res, 500, "Failed to save detection engines");
        return;
    }

    stream_detection_engine_t current[MAX_DETECTION_ENGINES_PER_STREAM];
    int current_count = db_detection_engines_list(
        stream_name, current, MAX_DETECTION_ENGINES_PER_STREAM);
    cJSON *root = cJSON_CreateObject();
    cJSON *array = cJSON_CreateArray();
    if (!root || !array || current_count < 0) {
        cJSON_Delete(root);
        cJSON_Delete(array);
        http_response_set_json_error(res, 500, "Saved but failed to create response");
        return;
    }
    cJSON_AddStringToObject(root, "stream_name", stream_name);
    cJSON_AddItemToObject(root, "engines", array);
    for (int i = 0; i < current_count; ++i) {
        cJSON *engine = engine_json(&current[i]);
        if (engine) cJSON_AddItemToArray(array, engine);
    }
    cJSON_AddNumberToObject(root, "count", current_count);
    cJSON_AddStringToObject(root, "trigger_policy", "any_of");
    cJSON_AddBoolToObject(root, "restart_required", true);
    char *encoded = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if (!encoded) {
        http_response_set_json_error(res, 500, "Saved but failed to serialize response");
        return;
    }
    http_response_set_json(res, 200, encoded);
    free(encoded);
}
