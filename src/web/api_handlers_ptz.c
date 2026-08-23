#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "web/api_handlers_ptz.h"
#include "web/api_handlers.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "web/audit_log.h"
#define LOG_COMPONENT "PTZAPI"
#include "core/logger.h"
#include "core/config.h"
#include "core/url_utils.h"
#include "utils/strings.h"
#include "database/db_streams.h"
#include "video/onvif_ptz.h"
#include <cjson/cJSON.h>

/**
 * Helper to get stream config and validate PTZ is enabled
 */
static int get_ptz_stream_config(const char *stream_name, stream_config_t *config) {
    if (!stream_name || !config) {
        return -1;
    }
    
    if (get_stream_config_by_name(stream_name, config) != 0) {
        log_error("Stream not found: %s", stream_name);
        return -1;
    }
    
    if (!config->ptz_enabled) {
        log_error("PTZ not enabled for stream: %s", stream_name);
        return -2;
    }
    
    return 0;
}

/**
 * Helper to build PTZ URL from stream config.
 *
 * Delegates to url_build_onvif_service_url() which handles:
 *   - Scheme mapping: rtsps → https, rtsp/onvif/… → http
 *   - Port mapping: 554 → 80, 322 → 443 (when onvif_port not explicitly set)
 *   - Credential stripping
 */
static int build_ptz_url(const stream_config_t *config, char *ptz_url, size_t url_size) {
    if (!config || !ptz_url || url_size == 0) {
        return -1;
    }

    char device_url[MAX_URL_LENGTH];
    // codeql[cpp/non-https-url] - Local ONVIF cameras use HTTP; HTTPS only when rtsps:// detected
    if (url_build_onvif_service_url(config->url, config->onvif_port,
                                    "/onvif/device_service",
                                    device_url, sizeof(device_url)) != 0) {
        return -1;
    }

    return onvif_ptz_get_service_url(device_url, config->onvif_username,
                                     config->onvif_password, ptz_url, url_size);
}

/**
 * Helper to get profile token (use first profile or default)
 */
static const char* get_profile_token(const stream_config_t *config) {
    if (config && config->onvif_profile[0] != '\0') {
        return config->onvif_profile;
    }

    return "Profile_1";
}

/**
 * Helper to extract stream name from PTZ URL path
 * URL format: /api/streams/{stream_name}/ptz/{action}
 */
static int extract_ptz_stream_name(const http_request_t *req, char *stream_name, size_t name_size) {
    // Extract stream name from URL
    if (http_request_extract_path_param(req, "/api/streams/", stream_name, name_size) != 0) {
        return -1;
    }

    // Remove "/ptz..." suffix from stream_name
    char *ptz_suffix = strstr(stream_name, "/ptz");
    if (ptz_suffix) {
        *ptz_suffix = '\0';
    }

    return 0;
}

typedef struct {
    user_t user;
    fleet_camera_t camera;
    authorization_evaluation_t evaluation;
} ptz_authorization_context_t;

static int authorize_ptz_stream(const http_request_t *req,
                                http_response_t *res,
                                const char *stream_name,
                                authorization_action_t action,
                                ptz_authorization_context_t *context) {
    return httpd_authorize_stream_action_with_context(
        req, res, action, stream_name, &context->user, &context->camera,
        &context->evaluation);
}

static void audit_ptz_operation(const http_request_t *req,
                                const ptz_authorization_context_t *context,
                                const char *operation, const char *outcome,
                                const char *reason) {
    cJSON *details = cJSON_CreateObject();
    if (details && reason) {
        cJSON_AddStringToObject(details, "reason", reason);
    }
    audit_log_operation(req, &context->user, "ptz.control", "camera",
                        context->camera.camera_uuid, operation, outcome,
                        details);
    cJSON_Delete(details);
}

void handle_ptz_move(const http_request_t *req, http_response_t *res) {
    char stream_name[MAX_STREAM_NAME];
    if (extract_ptz_stream_name(req, stream_name, sizeof(stream_name)) != 0) {
        http_response_set_json_error(res, 400, "Invalid stream name");
        return;
    }

    log_info("Handling POST /api/streams/%s/ptz/move", stream_name);

    ptz_authorization_context_t auth;
    if (!authorize_ptz_stream(req, res, stream_name, AUTHZ_PTZ_CONTROL,
                              &auth)) {
        return;
    }

    stream_config_t config;
    int rc = get_ptz_stream_config(stream_name, &config);
    if (rc == -1) {
        audit_ptz_operation(req, &auth, "continuous_move", "error",
                            "stream_configuration_unavailable");
        http_response_set_json_error(res, 404, "Stream not found");
        return;
    } else if (rc == -2) {
        audit_ptz_operation(req, &auth, "continuous_move", "failure",
                            "ptz_disabled");
        http_response_set_json_error(res, 400, "PTZ not enabled for this stream");
        return;
    }

    // Parse request body
    cJSON *body = httpd_parse_json_body(req);
    if (!body) {
        audit_ptz_operation(req, &auth, "continuous_move", "failure",
                            "invalid_request");
        http_response_set_json_error(res, 400, "Invalid JSON body");
        return;
    }
    
    float pan = 0.0f, tilt = 0.0f, zoom = 0.0f;
    cJSON *pan_json = cJSON_GetObjectItem(body, "pan");
    cJSON *tilt_json = cJSON_GetObjectItem(body, "tilt");
    cJSON *zoom_json = cJSON_GetObjectItem(body, "zoom");
    
    if (pan_json && cJSON_IsNumber(pan_json)) pan = (float)pan_json->valuedouble;
    if (tilt_json && cJSON_IsNumber(tilt_json)) tilt = (float)tilt_json->valuedouble;
    if (zoom_json && cJSON_IsNumber(zoom_json)) zoom = (float)zoom_json->valuedouble;
    
    cJSON_Delete(body);
    
    // Build PTZ URL and execute move
    char ptz_url[512];
    if (build_ptz_url(&config, ptz_url, sizeof(ptz_url)) != 0) {
        audit_ptz_operation(req, &auth, "continuous_move", "error",
                            "service_discovery_failed");
        http_response_set_json_error(res, 500, "Failed to build PTZ URL");
        return;
    }
    
    const char *profile_token = get_profile_token(&config);
    rc = onvif_ptz_continuous_move(ptz_url, profile_token, 
                                   config.onvif_username, config.onvif_password,
                                   pan, tilt, zoom);
    
    if (rc != 0) {
        audit_ptz_operation(req, &auth, "continuous_move", "failure",
                            "device_operation_failed");
        http_response_set_json_error(res, 500, "PTZ move failed");
        return;
    }

    audit_ptz_operation(req, &auth, "continuous_move", "success",
                        "completed");
    
    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "message", "PTZ move started");

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    http_response_set_json(res, 200, json_str);
    free(json_str);
}

void handle_ptz_stop(const http_request_t *req, http_response_t *res) {
    char stream_name[MAX_STREAM_NAME];
    if (extract_ptz_stream_name(req, stream_name, sizeof(stream_name)) != 0) {
        http_response_set_json_error(res, 400, "Invalid stream name");
        return;
    }

    log_info("Handling POST /api/streams/%s/ptz/stop", stream_name);

    ptz_authorization_context_t auth;
    if (!authorize_ptz_stream(req, res, stream_name, AUTHZ_PTZ_CONTROL,
                              &auth)) {
        return;
    }

    stream_config_t config;
    int rc = get_ptz_stream_config(stream_name, &config);
    if (rc == -1) {
        audit_ptz_operation(req, &auth, "stop", "error",
                            "stream_configuration_unavailable");
        http_response_set_json_error(res, 404, "Stream not found");
        return;
    } else if (rc == -2) {
        audit_ptz_operation(req, &auth, "stop", "failure", "ptz_disabled");
        http_response_set_json_error(res, 400, "PTZ not enabled for this stream");
        return;
    }

    char ptz_url[512];
    if (build_ptz_url(&config, ptz_url, sizeof(ptz_url)) != 0) {
        audit_ptz_operation(req, &auth, "stop", "error",
                            "service_discovery_failed");
        http_response_set_json_error(res, 500, "Failed to build PTZ URL");
        return;
    }

    const char *profile_token = get_profile_token(&config);
    rc = onvif_ptz_stop(ptz_url, profile_token, config.onvif_username, config.onvif_password, true, true);

    if (rc != 0) {
        audit_ptz_operation(req, &auth, "stop", "failure",
                            "device_operation_failed");
        http_response_set_json_error(res, 500, "PTZ stop failed");
        return;
    }

    audit_ptz_operation(req, &auth, "stop", "success", "completed");

    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "message", "PTZ stopped");

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    http_response_set_json(res, 200, json_str);
    free(json_str);
}

void handle_ptz_absolute(const http_request_t *req, http_response_t *res) {
    char stream_name[MAX_STREAM_NAME];
    if (extract_ptz_stream_name(req, stream_name, sizeof(stream_name)) != 0) {
        http_response_set_json_error(res, 400, "Invalid stream name");
        return;
    }

    log_info("Handling POST /api/streams/%s/ptz/absolute", stream_name);

    ptz_authorization_context_t auth;
    if (!authorize_ptz_stream(req, res, stream_name, AUTHZ_PTZ_CONTROL,
                              &auth)) {
        return;
    }

    stream_config_t config;
    int rc = get_ptz_stream_config(stream_name, &config);
    if (rc == -1) {
        audit_ptz_operation(req, &auth, "absolute_move", "error",
                            "stream_configuration_unavailable");
        http_response_set_json_error(res, 404, "Stream not found");
        return;
    } else if (rc == -2) {
        audit_ptz_operation(req, &auth, "absolute_move", "failure",
                            "ptz_disabled");
        http_response_set_json_error(res, 400, "PTZ not enabled for this stream");
        return;
    }

    cJSON *body = httpd_parse_json_body(req);
    if (!body) {
        audit_ptz_operation(req, &auth, "absolute_move", "failure",
                            "invalid_request");
        http_response_set_json_error(res, 400, "Invalid JSON body");
        return;
    }

    float pan = 0.0f, tilt = 0.0f, zoom = 0.0f;
    bool has_pan = false, has_tilt = false, has_zoom = false;
    cJSON *pan_json = cJSON_GetObjectItem(body, "pan");
    cJSON *tilt_json = cJSON_GetObjectItem(body, "tilt");
    cJSON *zoom_json = cJSON_GetObjectItem(body, "zoom");

    if (pan_json && cJSON_IsNumber(pan_json)) {
        pan = (float)pan_json->valuedouble;
        has_pan = true;
    }
    if (tilt_json && cJSON_IsNumber(tilt_json)) {
        tilt = (float)tilt_json->valuedouble;
        has_tilt = true;
    }
    if (zoom_json && cJSON_IsNumber(zoom_json)) {
        zoom = (float)zoom_json->valuedouble;
        has_zoom = true;
    }

    cJSON_Delete(body);

    if (!has_pan && !has_tilt && !has_zoom) {
        audit_ptz_operation(req, &auth, "absolute_move", "failure",
                            "missing_axis");
        http_response_set_json_error(res, 400, "At least one PTZ axis is required");
        return;
    }

    char ptz_url[512];
    if (build_ptz_url(&config, ptz_url, sizeof(ptz_url)) != 0) {
        audit_ptz_operation(req, &auth, "absolute_move", "error",
                            "service_discovery_failed");
        http_response_set_json_error(res, 500, "Failed to build PTZ URL");
        return;
    }

    const char *profile_token = get_profile_token(&config);
    rc = onvif_ptz_absolute_move_axes(ptz_url, profile_token,
                                      config.onvif_username, config.onvif_password,
                                      has_pan || has_tilt, pan, tilt, has_zoom, zoom);

    if (rc != 0) {
        audit_ptz_operation(req, &auth, "absolute_move", "failure",
                            "device_operation_failed");
        http_response_set_json_error(res, 500, "PTZ absolute move failed");
        return;
    }

    audit_ptz_operation(req, &auth, "absolute_move", "success",
                        "completed");

    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "message", "PTZ absolute move completed");

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    http_response_set_json(res, 200, json_str);
    free(json_str);
}

void handle_ptz_relative(const http_request_t *req, http_response_t *res) {
    char stream_name[MAX_STREAM_NAME];
    if (extract_ptz_stream_name(req, stream_name, sizeof(stream_name)) != 0) {
        http_response_set_json_error(res, 400, "Invalid stream name");
        return;
    }

    log_info("Handling POST /api/streams/%s/ptz/relative", stream_name);

    ptz_authorization_context_t auth;
    if (!authorize_ptz_stream(req, res, stream_name, AUTHZ_PTZ_CONTROL,
                              &auth)) {
        return;
    }

    stream_config_t config;
    int rc = get_ptz_stream_config(stream_name, &config);
    if (rc == -1) {
        audit_ptz_operation(req, &auth, "relative_move", "error",
                            "stream_configuration_unavailable");
        http_response_set_json_error(res, 404, "Stream not found");
        return;
    } else if (rc == -2) {
        audit_ptz_operation(req, &auth, "relative_move", "failure",
                            "ptz_disabled");
        http_response_set_json_error(res, 400, "PTZ not enabled for this stream");
        return;
    }

    cJSON *body = httpd_parse_json_body(req);
    if (!body) {
        audit_ptz_operation(req, &auth, "relative_move", "failure",
                            "invalid_request");
        http_response_set_json_error(res, 400, "Invalid JSON body");
        return;
    }

    float pan = 0.0f, tilt = 0.0f, zoom = 0.0f;
    bool has_pan = false, has_tilt = false, has_zoom = false;
    cJSON *pan_json = cJSON_GetObjectItem(body, "pan");
    cJSON *tilt_json = cJSON_GetObjectItem(body, "tilt");
    cJSON *zoom_json = cJSON_GetObjectItem(body, "zoom");

    if (pan_json && cJSON_IsNumber(pan_json)) {
        pan = (float)pan_json->valuedouble;
        has_pan = true;
    }
    if (tilt_json && cJSON_IsNumber(tilt_json)) {
        tilt = (float)tilt_json->valuedouble;
        has_tilt = true;
    }
    if (zoom_json && cJSON_IsNumber(zoom_json)) {
        zoom = (float)zoom_json->valuedouble;
        has_zoom = true;
    }

    cJSON_Delete(body);

    if (!has_pan && !has_tilt && !has_zoom) {
        audit_ptz_operation(req, &auth, "relative_move", "failure",
                            "missing_axis");
        http_response_set_json_error(res, 400, "At least one PTZ axis is required");
        return;
    }

    char ptz_url[512];
    if (build_ptz_url(&config, ptz_url, sizeof(ptz_url)) != 0) {
        audit_ptz_operation(req, &auth, "relative_move", "error",
                            "service_discovery_failed");
        http_response_set_json_error(res, 500, "Failed to build PTZ URL");
        return;
    }

    const char *profile_token = get_profile_token(&config);
    rc = onvif_ptz_relative_move_axes(ptz_url, profile_token,
                                      config.onvif_username, config.onvif_password,
                                      has_pan || has_tilt, pan, tilt, has_zoom, zoom);

    if (rc != 0) {
        audit_ptz_operation(req, &auth, "relative_move", "failure",
                            "device_operation_failed");
        http_response_set_json_error(res, 500, "PTZ relative move failed");
        return;
    }

    audit_ptz_operation(req, &auth, "relative_move", "success",
                        "completed");

    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "message", "PTZ relative move completed");

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    http_response_set_json(res, 200, json_str);
    free(json_str);
}

void handle_ptz_home(const http_request_t *req, http_response_t *res) {
    char stream_name[MAX_STREAM_NAME];
    if (extract_ptz_stream_name(req, stream_name, sizeof(stream_name)) != 0) {
        http_response_set_json_error(res, 400, "Invalid stream name");
        return;
    }

    log_info("Handling POST /api/streams/%s/ptz/home", stream_name);

    ptz_authorization_context_t auth;
    if (!authorize_ptz_stream(req, res, stream_name, AUTHZ_PTZ_CONTROL,
                              &auth)) {
        return;
    }

    stream_config_t config;
    int rc = get_ptz_stream_config(stream_name, &config);
    if (rc == -1) {
        audit_ptz_operation(req, &auth, "goto_home", "error",
                            "stream_configuration_unavailable");
        http_response_set_json_error(res, 404, "Stream not found");
        return;
    } else if (rc == -2) {
        audit_ptz_operation(req, &auth, "goto_home", "failure",
                            "ptz_disabled");
        http_response_set_json_error(res, 400, "PTZ not enabled for this stream");
        return;
    }

    char ptz_url[512];
    if (build_ptz_url(&config, ptz_url, sizeof(ptz_url)) != 0) {
        audit_ptz_operation(req, &auth, "goto_home", "error",
                            "service_discovery_failed");
        http_response_set_json_error(res, 500, "Failed to build PTZ URL");
        return;
    }

    const char *profile_token = get_profile_token(&config);
    rc = onvif_ptz_goto_home(ptz_url, profile_token, config.onvif_username, config.onvif_password);

    if (rc != 0) {
        audit_ptz_operation(req, &auth, "goto_home", "failure",
                            "device_operation_failed");
        http_response_set_json_error(res, 500, "PTZ go to home failed");
        return;
    }

    audit_ptz_operation(req, &auth, "goto_home", "success", "completed");

    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "message", "PTZ moved to home position");

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    http_response_set_json(res, 200, json_str);
    free(json_str);
}

void handle_ptz_set_home(const http_request_t *req, http_response_t *res) {
    char stream_name[MAX_STREAM_NAME];
    if (extract_ptz_stream_name(req, stream_name, sizeof(stream_name)) != 0) {
        http_response_set_json_error(res, 400, "Invalid stream name");
        return;
    }

    log_info("Handling POST /api/streams/%s/ptz/sethome", stream_name);

    ptz_authorization_context_t auth;
    if (!authorize_ptz_stream(req, res, stream_name, AUTHZ_PTZ_CONTROL,
                              &auth)) {
        return;
    }

    stream_config_t config;
    int rc = get_ptz_stream_config(stream_name, &config);
    if (rc == -1) {
        audit_ptz_operation(req, &auth, "set_home", "error",
                            "stream_configuration_unavailable");
        http_response_set_json_error(res, 404, "Stream not found");
        return;
    } else if (rc == -2) {
        audit_ptz_operation(req, &auth, "set_home", "failure",
                            "ptz_disabled");
        http_response_set_json_error(res, 400, "PTZ not enabled for this stream");
        return;
    }

    char ptz_url[512];
    if (build_ptz_url(&config, ptz_url, sizeof(ptz_url)) != 0) {
        audit_ptz_operation(req, &auth, "set_home", "error",
                            "service_discovery_failed");
        http_response_set_json_error(res, 500, "Failed to build PTZ URL");
        return;
    }

    const char *profile_token = get_profile_token(&config);
    rc = onvif_ptz_set_home(ptz_url, profile_token, config.onvif_username, config.onvif_password);

    if (rc != 0) {
        audit_ptz_operation(req, &auth, "set_home", "failure",
                            "device_operation_failed");
        http_response_set_json_error(res, 500, "PTZ set home failed");
        return;
    }

    audit_ptz_operation(req, &auth, "set_home", "success", "completed");

    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "message", "PTZ home position set");

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    http_response_set_json(res, 200, json_str);
    free(json_str);
}

void handle_ptz_get_presets(const http_request_t *req, http_response_t *res) {
    char stream_name[MAX_STREAM_NAME];
    if (extract_ptz_stream_name(req, stream_name, sizeof(stream_name)) != 0) {
        http_response_set_json_error(res, 400, "Invalid stream name");
        return;
    }

    log_info("Handling GET /api/streams/%s/ptz/presets", stream_name);

    stream_config_t config;
    int rc = get_ptz_stream_config(stream_name, &config);
    if (rc == -1) {
        http_response_set_json_error(res, 404, "Stream not found");
        return;
    } else if (rc == -2) {
        http_response_set_json_error(res, 400, "PTZ not enabled for this stream");
        return;
    }
    ptz_authorization_context_t auth;
    if (!authorize_ptz_stream(req, res, stream_name, AUTHZ_LIVE_VIEW,
                              &auth)) {
        return;
    }

    char ptz_url[512];
    if (build_ptz_url(&config, ptz_url, sizeof(ptz_url)) != 0) {
        http_response_set_json_error(res, 500, "Failed to build PTZ URL");
        return;
    }

    const char *profile_token = get_profile_token(&config);
    onvif_ptz_preset_t presets[32];
    int count = onvif_ptz_get_presets(ptz_url, profile_token, config.onvif_username, config.onvif_password, presets, 32);
    if (count < 0) {
        http_response_set_json_error(res, 502, "Failed to get ONVIF PTZ presets");
        return;
    }

    cJSON *response = cJSON_CreateObject();
    cJSON *presets_array = cJSON_CreateArray();

    for (int i = 0; i < count; i++) {
        cJSON *preset = cJSON_CreateObject();
        cJSON_AddStringToObject(preset, "token", presets[i].token);
        cJSON_AddStringToObject(preset, "name", presets[i].name);
        cJSON_AddItemToArray(presets_array, preset);
    }

    cJSON_AddItemToObject(response, "presets", presets_array);
    cJSON_AddNumberToObject(response, "count", count);

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    http_response_set_json(res, 200, json_str);
    free(json_str);
}

void handle_ptz_goto_preset(const http_request_t *req, http_response_t *res) {
    char stream_name[MAX_STREAM_NAME];
    if (extract_ptz_stream_name(req, stream_name, sizeof(stream_name)) != 0) {
        http_response_set_json_error(res, 400, "Invalid stream name");
        return;
    }

    log_info("Handling POST /api/streams/%s/ptz/preset", stream_name);

    ptz_authorization_context_t auth;
    if (!authorize_ptz_stream(req, res, stream_name, AUTHZ_PTZ_CONTROL,
                              &auth)) {
        return;
    }

    stream_config_t config;
    int rc = get_ptz_stream_config(stream_name, &config);
    if (rc == -1) {
        audit_ptz_operation(req, &auth, "goto_preset", "error",
                            "stream_configuration_unavailable");
        http_response_set_json_error(res, 404, "Stream not found");
        return;
    } else if (rc == -2) {
        audit_ptz_operation(req, &auth, "goto_preset", "failure",
                            "ptz_disabled");
        http_response_set_json_error(res, 400, "PTZ not enabled for this stream");
        return;
    }

    cJSON *body = httpd_parse_json_body(req);
    if (!body) {
        audit_ptz_operation(req, &auth, "goto_preset", "failure",
                            "invalid_request");
        http_response_set_json_error(res, 400, "Invalid JSON body");
        return;
    }

    cJSON *token_json = cJSON_GetObjectItem(body, "token");
    if (!token_json || !cJSON_IsString(token_json)) {
        cJSON_Delete(body);
        audit_ptz_operation(req, &auth, "goto_preset", "failure",
                            "missing_preset_token");
        http_response_set_json_error(res, 400, "Missing preset token");
        return;
    }

    char preset_token[64];
    safe_strcpy(preset_token, token_json->valuestring, sizeof(preset_token), 0);
    cJSON_Delete(body);

    char ptz_url[512];
    if (build_ptz_url(&config, ptz_url, sizeof(ptz_url)) != 0) {
        audit_ptz_operation(req, &auth, "goto_preset", "error",
                            "service_discovery_failed");
        http_response_set_json_error(res, 500, "Failed to build PTZ URL");
        return;
    }

    const char *profile_token = get_profile_token(&config);
    rc = onvif_ptz_goto_preset(ptz_url, profile_token, config.onvif_username, config.onvif_password, preset_token);

    if (rc != 0) {
        audit_ptz_operation(req, &auth, "goto_preset", "failure",
                            "device_operation_failed");
        http_response_set_json_error(res, 500, "PTZ go to preset failed");
        return;
    }

    audit_ptz_operation(req, &auth, "goto_preset", "success", "completed");

    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "message", "PTZ moved to preset");

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    http_response_set_json(res, 200, json_str);
    free(json_str);
}

void handle_ptz_set_preset(const http_request_t *req, http_response_t *res) {
    char stream_name[MAX_STREAM_NAME];
    if (extract_ptz_stream_name(req, stream_name, sizeof(stream_name)) != 0) {
        http_response_set_json_error(res, 400, "Invalid stream name");
        return;
    }

    log_info("Handling PUT /api/streams/%s/ptz/preset", stream_name);

    ptz_authorization_context_t auth;
    if (!authorize_ptz_stream(req, res, stream_name, AUTHZ_PTZ_CONTROL,
                              &auth)) {
        return;
    }

    stream_config_t config;
    int rc = get_ptz_stream_config(stream_name, &config);
    if (rc == -1) {
        audit_ptz_operation(req, &auth, "set_preset", "error",
                            "stream_configuration_unavailable");
        http_response_set_json_error(res, 404, "Stream not found");
        return;
    } else if (rc == -2) {
        audit_ptz_operation(req, &auth, "set_preset", "failure",
                            "ptz_disabled");
        http_response_set_json_error(res, 400, "PTZ not enabled for this stream");
        return;
    }

    cJSON *body = httpd_parse_json_body(req);
    if (!body) {
        audit_ptz_operation(req, &auth, "set_preset", "failure",
                            "invalid_request");
        http_response_set_json_error(res, 400, "Invalid JSON body");
        return;
    }

    const char *preset_name = NULL;
    cJSON *name_json = cJSON_GetObjectItem(body, "name");
    if (name_json && cJSON_IsString(name_json)) {
        preset_name = name_json->valuestring;
    }

    char ptz_url[512];
    if (build_ptz_url(&config, ptz_url, sizeof(ptz_url)) != 0) {
        cJSON_Delete(body);
        audit_ptz_operation(req, &auth, "set_preset", "error",
                            "service_discovery_failed");
        http_response_set_json_error(res, 500, "Failed to build PTZ URL");
        return;
    }

    const char *profile_token = get_profile_token(&config);
    char new_token[64] = {0};
    rc = onvif_ptz_set_preset(ptz_url, profile_token, config.onvif_username, config.onvif_password,
                              preset_name, new_token, sizeof(new_token));

    cJSON_Delete(body);

    if (rc != 0) {
        audit_ptz_operation(req, &auth, "set_preset", "failure",
                            "device_operation_failed");
        http_response_set_json_error(res, 500, "PTZ set preset failed");
        return;
    }

    audit_ptz_operation(req, &auth, "set_preset", "success", "completed");

    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "message", "PTZ preset created");
    cJSON_AddStringToObject(response, "token", new_token);

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    http_response_set_json(res, 200, json_str);
    free(json_str);
}

void handle_ptz_capabilities(const http_request_t *req, http_response_t *res) {
    char stream_name[MAX_STREAM_NAME];
    if (extract_ptz_stream_name(req, stream_name, sizeof(stream_name)) != 0) {
        http_response_set_json_error(res, 400, "Invalid stream name");
        return;
    }

    log_info("Handling GET /api/streams/%s/ptz/capabilities", stream_name);

    stream_config_t config;
    int rc = get_ptz_stream_config(stream_name, &config);
    if (rc == -1) {
        http_response_set_json_error(res, 404, "Stream not found");
        return;
    } else if (rc == -2) {
        http_response_set_json_error(res, 400, "PTZ not enabled for this stream");
        return;
    }
    ptz_authorization_context_t auth;
    if (!authorize_ptz_stream(req, res, stream_name, AUTHZ_LIVE_VIEW,
                              &auth)) {
        return;
    }

    char ptz_url[512];
    if (build_ptz_url(&config, ptz_url, sizeof(ptz_url)) != 0) {
        http_response_set_json_error(res, 500, "Failed to build PTZ URL");
        return;
    }

    const char *profile_token = get_profile_token(&config);
    onvif_ptz_capabilities_t caps;
    rc = onvif_ptz_get_capabilities(ptz_url, profile_token,
                                    config.onvif_username, config.onvif_password, &caps);
    if (rc != 0) {
        http_response_set_json_error(res, 502, "Failed to get ONVIF PTZ capabilities");
        return;
    }

    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "ptz_enabled", config.ptz_enabled);
    cJSON_AddBoolToObject(response, "onvif_queried", caps.queried);
    cJSON_AddStringToObject(response, "ptz_service_url", ptz_url);
    cJSON_AddBoolToObject(response, "has_pan_tilt", caps.has_pan_tilt);
    cJSON_AddBoolToObject(response, "has_zoom", caps.has_zoom);
    cJSON_AddBoolToObject(response, "has_continuous_move", caps.has_continuous_move);
    cJSON_AddBoolToObject(response, "has_absolute_move", caps.has_absolute_move);
    cJSON_AddBoolToObject(response, "has_relative_move", caps.has_relative_move);
    cJSON_AddBoolToObject(response, "has_home_position", caps.has_home_position);
    cJSON_AddBoolToObject(response, "has_presets", caps.has_presets);
    cJSON_AddNumberToObject(response, "max_presets", caps.max_presets);

    cJSON *pan_range = cJSON_CreateObject();
    cJSON_AddNumberToObject(pan_range, "min", caps.pan_min);
    cJSON_AddNumberToObject(pan_range, "max", caps.pan_max);
    cJSON_AddItemToObject(response, "pan_range", pan_range);

    cJSON *tilt_range = cJSON_CreateObject();
    cJSON_AddNumberToObject(tilt_range, "min", caps.tilt_min);
    cJSON_AddNumberToObject(tilt_range, "max", caps.tilt_max);
    cJSON_AddItemToObject(response, "tilt_range", tilt_range);

    cJSON *zoom_range = cJSON_CreateObject();
    cJSON_AddNumberToObject(zoom_range, "min", caps.zoom_min);
    cJSON_AddNumberToObject(zoom_range, "max", caps.zoom_max);
    cJSON_AddItemToObject(response, "zoom_range", zoom_range);

    // Add stream-specific PTZ limits from config
    cJSON_AddNumberToObject(response, "max_x", config.ptz_max_x);
    cJSON_AddNumberToObject(response, "max_y", config.ptz_max_y);
    cJSON_AddNumberToObject(response, "max_z", config.ptz_max_z);
    cJSON_AddBoolToObject(response, "has_home", config.ptz_has_home);

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    http_response_set_json(res, 200, json_str);
    free(json_str);
}
