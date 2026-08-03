#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include <cjson/cJSON.h>

#include "core/config.h"
#define LOG_COMPONENT "RecordingControlAPI"
#include "core/logger.h"
#include "database/db_auth.h"
#include "database/db_streams.h"
#include "video/go2rtc/go2rtc_integration.h"
#include "video/mp4_recording.h"
#include "video/stream_manager.h"
#include "video/unified_detection_thread.h"
#include "web/api_handlers_recording_control.h"
#include "web/httpd_utils.h"

static int extract_stream_name(const http_request_t *req,
                               char *stream_name, size_t size) {
    if (http_request_extract_path_param(req, "/api/streams/",
                                        stream_name, size) != 0) {
        return -1;
    }
    char *suffix = strstr(stream_name, "/recording");
    if (!suffix || suffix[10] != '\0') {
        return -1;
    }
    *suffix = '\0';
    return stream_name[0] ? 0 : -1;
}

static bool check_stream_access(const http_request_t *req,
                                const stream_config_t *config,
                                user_t *user,
                                bool write_access,
                                http_response_t *res) {
    memset(user, 0, sizeof(*user));
    if (!httpd_check_viewer_access(req, user)) {
        http_response_set_json_error(res, 401, "Unauthorized");
        return false;
    }
    if (!db_auth_stream_allowed_for_user(user, config->tags)) {
        http_response_set_json_error(res, 403, "Stream access denied");
        return false;
    }
    if (write_access && g_config.web_auth_enabled &&
        user->role == USER_ROLE_VIEWER) {
        http_response_set_json_error(res, 403,
            "Viewer role cannot control recordings");
        return false;
    }
    return true;
}

static void add_runtime_json(cJSON *json, const char *stream_name,
                             const recording_runtime_info_t *info,
                             uint64_t detection_id,
                             bool manual_control_allowed,
                             const char *manual_control_reason) {
    bool active = info->active || info->initializing || detection_id != 0;
    const char *state = info->initializing ? "starting"
        : active ? "recording" : "idle";
    const char *capture_method = detection_id != 0
        ? "detection" : info->trigger_type;

    cJSON_AddStringToObject(json, "stream", stream_name);
    cJSON_AddStringToObject(json, "state", state);
    cJSON_AddBoolToObject(json, "recording_active", active);
    if (active && capture_method[0]) {
        cJSON_AddStringToObject(json, "capture_method", capture_method);
    } else {
        cJSON_AddNullToObject(json, "capture_method");
    }
    uint64_t id = detection_id ? detection_id : info->recording_id;
    if (id) {
        cJSON_AddNumberToObject(json, "recording_id", (double)id);
    } else {
        cJSON_AddNullToObject(json, "recording_id");
    }
    cJSON_AddBoolToObject(json, "manual_control_allowed",
                          manual_control_allowed);
    if (manual_control_reason) {
        cJSON_AddStringToObject(json, "manual_control_reason",
                                manual_control_reason);
    } else {
        cJSON_AddNullToObject(json, "manual_control_reason");
    }
}

void handle_get_stream_recording(const http_request_t *req,
                                 http_response_t *res) {
    char stream_name[MAX_STREAM_NAME] = {0};
    if (extract_stream_name(req, stream_name, sizeof(stream_name)) != 0) {
        http_response_set_json_error(res, 400, "Invalid stream name in URL");
        return;
    }

    stream_config_t config = {0};
    if (get_stream_config_by_name(stream_name, &config) != 0) {
        http_response_set_json_error(res, 404, "Stream not found");
        return;
    }
    user_t user;
    if (!check_stream_access(req, &config, &user, false, res)) {
        return;
    }

    recording_runtime_info_t info = {0};
    get_mp4_recording_runtime_info(stream_name, &info);
    uint64_t detection_id =
        get_unified_detection_recording_id(stream_name);
    bool may_write = !g_config.web_auth_enabled ||
                     user.role != USER_ROLE_VIEWER;
    bool continuous_expected =
        config.record && is_recording_scheduled(&config);
    const char *manual_control_reason = !may_write ? "read_only"
        : continuous_expected ? "continuous_recording" : NULL;

    cJSON *json = cJSON_CreateObject();
    if (!json) {
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    add_runtime_json(json, stream_name, &info, detection_id,
                     may_write && !continuous_expected,
                     manual_control_reason);
    char *body = cJSON_PrintUnformatted(json);
    cJSON_Delete(json);
    if (!body) {
        http_response_set_json_error(res, 500, "Failed to serialize response");
        return;
    }
    http_response_set_json(res, 200, body);
    free(body);
}

void handle_post_stream_recording(const http_request_t *req,
                                  http_response_t *res) {
    char stream_name[MAX_STREAM_NAME] = {0};
    if (extract_stream_name(req, stream_name, sizeof(stream_name)) != 0) {
        http_response_set_json_error(res, 400, "Invalid stream name in URL");
        return;
    }
    stream_config_t config = {0};
    if (get_stream_config_by_name(stream_name, &config) != 0) {
        http_response_set_json_error(res, 404, "Stream not found");
        return;
    }
    user_t user;
    if (!check_stream_access(req, &config, &user, true, res)) {
        return;
    }

    cJSON *body = httpd_parse_json_body(req);
    cJSON *action = body
        ? cJSON_GetObjectItemCaseSensitive(body, "action") : NULL;
    if (!cJSON_IsString(action) ||
        (strcmp(action->valuestring, "start") != 0 &&
         strcmp(action->valuestring, "stop") != 0)) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400,
            "action must be 'start' or 'stop'");
        return;
    }

    bool starting = strcmp(action->valuestring, "start") == 0;
    if (starting) {
        /* Reject from configuration immediately.  A newly enabled continuous
         * recorder may spend several seconds creating its writer, during which
         * runtime state alone still looks idle (#473). */
        if (config.record && is_recording_scheduled(&config)) {
            cJSON_Delete(body);
            http_response_set_json_error(res, 409,
                "Continuous recording is configured for this stream");
            return;
        }
    }

    recording_runtime_info_t info = {0};
    int runtime_rc = get_mp4_recording_runtime_info(stream_name, &info);
    uint64_t detection_id =
        get_unified_detection_recording_id(stream_name);
    int result;
    int status;
    if (starting) {
        if (runtime_rc == 0 || detection_id != 0) {
            cJSON_Delete(body);
            http_response_set_json_error(res, 409,
                "A continuous, detection, or manual recording is already active");
            return;
        }
        result = go2rtc_integration_start_recording_with_trigger(
            stream_name, "manual");
        status = 202;
    } else {
        if (runtime_rc != 0 ||
            strcmp(info.trigger_type, "manual") != 0) {
            cJSON_Delete(body);
            http_response_set_json_error(res, 409,
                "Only an active manual recording can be stopped here");
            return;
        }
        result = go2rtc_integration_stop_recording(stream_name);
        status = 200;
    }
    cJSON_Delete(body);

    if (result != 0) {
        http_response_set_json_error(res, 500,
            "Failed to change recording state");
        return;
    }

    cJSON *json = cJSON_CreateObject();
    if (!json) {
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    cJSON_AddBoolToObject(json, "success", true);
    cJSON_AddStringToObject(json, "stream", stream_name);
    cJSON_AddStringToObject(json, "action",
        status == 202 ? "start" : "stop");
    cJSON_AddStringToObject(json, "state",
        status == 202 ? "starting" : "idle");
    char *response_body = cJSON_PrintUnformatted(json);
    cJSON_Delete(json);
    if (!response_body) {
        http_response_set_json_error(res, 500, "Failed to serialize response");
        return;
    }
    http_response_set_json(res, status, response_body);
    free(response_body);
}
