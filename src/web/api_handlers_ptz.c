#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "web/api_handlers_ptz.h"
#include "web/api_handlers.h"
#include "web/mongoose_adapter.h"
#include "core/logger.h"
#include "core/config.h"
#include "database/db_streams.h"
#include "video/onvif_ptz.h"
#include "../../external/mongoose/mongoose.h"
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
 * Helper to build PTZ URL from stream URL
 */
static int build_ptz_url(const stream_config_t *config, char *ptz_url, size_t url_size) {
    // Extract base URL from stream URL and build PTZ service URL
    // Stream URL format: rtsp://user:pass@host:port/path
    // We need: http://host:port/onvif/ptz_service
    
    const char *url = config->url;
    char host[256] = {0};
    int port = 80;
    
    // Parse host and port from RTSP URL
    const char *at_sign = strrchr(url, '@');
    const char *host_start = at_sign ? at_sign + 1 : strstr(url, "://");
    if (host_start && strstr(url, "://")) {
        host_start = host_start + (at_sign ? 0 : 3);
    }
    
    if (host_start) {
        const char *port_start = strchr(host_start, ':');
        const char *path_start = strchr(host_start, '/');
        
        if (port_start && (!path_start || port_start < path_start)) {
            size_t host_len = port_start - host_start;
            if (host_len < sizeof(host)) {
                strncpy(host, host_start, host_len);
                host[host_len] = '\0';
            }
            port = atoi(port_start + 1);
            if (port == 554) port = 80;  // RTSP port, use HTTP default
        } else if (path_start) {
            size_t host_len = path_start - host_start;
            if (host_len < sizeof(host)) {
                strncpy(host, host_start, host_len);
                host[host_len] = '\0';
            }
        } else {
            strncpy(host, host_start, sizeof(host) - 1);
        }
    }
    
    if (strlen(host) == 0) {
        log_error("Could not parse host from stream URL: %s", url);
        return -1;
    }
    
    snprintf(ptz_url, url_size, "http://%s:%d/onvif/ptz_service", host, port);
    return 0;
}

/**
 * Helper to get profile token (use first profile or default)
 */
static const char* get_profile_token(const stream_config_t *config) {
    // For now, use a default profile token
    // TODO: Store profile token in stream config or query from device
    (void)config;  // Unused for now
    return "Profile_1";
}

/**
 * Helper to extract stream name from PTZ URL path
 * URL format: /api/streams/{stream_name}/ptz/{action}
 */
static int extract_ptz_stream_name(struct mg_http_message *hm, char *stream_name, size_t name_size) {
    // Extract stream name from URL
    if (mg_extract_path_param(hm, "/api/streams/", stream_name, name_size) != 0) {
        return -1;
    }

    // Remove "/ptz..." suffix from stream_name
    char *ptz_suffix = strstr(stream_name, "/ptz");
    if (ptz_suffix) {
        *ptz_suffix = '\0';
    }

    return 0;
}

void mg_handle_ptz_move(struct mg_connection *c, struct mg_http_message *hm) {
    char stream_name[MAX_STREAM_NAME];
    if (extract_ptz_stream_name(hm, stream_name, sizeof(stream_name)) != 0) {
        mg_send_json_error(c, 400, "Invalid stream name");
        return;
    }

    log_info("Handling POST /api/streams/%s/ptz/move", stream_name);
    
    stream_config_t config;
    int rc = get_ptz_stream_config(stream_name, &config);
    if (rc == -1) {
        mg_send_json_error(c, 404, "Stream not found");
        return;
    } else if (rc == -2) {
        mg_send_json_error(c, 400, "PTZ not enabled for this stream");
        return;
    }
    
    // Parse request body
    cJSON *body = cJSON_ParseWithLength(hm->body.buf, hm->body.len);
    if (!body) {
        mg_send_json_error(c, 400, "Invalid JSON body");
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
        mg_send_json_error(c, 500, "Failed to build PTZ URL");
        return;
    }
    
    const char *profile_token = get_profile_token(&config);
    rc = onvif_ptz_continuous_move(ptz_url, profile_token, 
                                   config.onvif_username, config.onvif_password,
                                   pan, tilt, zoom);
    
    if (rc != 0) {
        mg_send_json_error(c, 500, "PTZ move failed");
        return;
    }
    
    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "message", "PTZ move started");

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    mg_send_json_response(c, 200, json_str);
    free(json_str);
}

void mg_handle_ptz_stop(struct mg_connection *c, struct mg_http_message *hm) {
    char stream_name[MAX_STREAM_NAME];
    if (extract_ptz_stream_name(hm, stream_name, sizeof(stream_name)) != 0) {
        mg_send_json_error(c, 400, "Invalid stream name");
        return;
    }

    log_info("Handling POST /api/streams/%s/ptz/stop", stream_name);

    stream_config_t config;
    int rc = get_ptz_stream_config(stream_name, &config);
    if (rc == -1) {
        mg_send_json_error(c, 404, "Stream not found");
        return;
    } else if (rc == -2) {
        mg_send_json_error(c, 400, "PTZ not enabled for this stream");
        return;
    }

    char ptz_url[512];
    if (build_ptz_url(&config, ptz_url, sizeof(ptz_url)) != 0) {
        mg_send_json_error(c, 500, "Failed to build PTZ URL");
        return;
    }

    const char *profile_token = get_profile_token(&config);
    rc = onvif_ptz_stop(ptz_url, profile_token, config.onvif_username, config.onvif_password, true, true);

    if (rc != 0) {
        mg_send_json_error(c, 500, "PTZ stop failed");
        return;
    }

    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "message", "PTZ stopped");

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    mg_send_json_response(c, 200, json_str);
    free(json_str);
}

void mg_handle_ptz_absolute(struct mg_connection *c, struct mg_http_message *hm) {
    char stream_name[MAX_STREAM_NAME];
    if (extract_ptz_stream_name(hm, stream_name, sizeof(stream_name)) != 0) {
        mg_send_json_error(c, 400, "Invalid stream name");
        return;
    }

    log_info("Handling POST /api/streams/%s/ptz/absolute", stream_name);

    stream_config_t config;
    int rc = get_ptz_stream_config(stream_name, &config);
    if (rc == -1) {
        mg_send_json_error(c, 404, "Stream not found");
        return;
    } else if (rc == -2) {
        mg_send_json_error(c, 400, "PTZ not enabled for this stream");
        return;
    }

    cJSON *body = cJSON_ParseWithLength(hm->body.buf, hm->body.len);
    if (!body) {
        mg_send_json_error(c, 400, "Invalid JSON body");
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

    char ptz_url[512];
    if (build_ptz_url(&config, ptz_url, sizeof(ptz_url)) != 0) {
        mg_send_json_error(c, 500, "Failed to build PTZ URL");
        return;
    }

    const char *profile_token = get_profile_token(&config);
    rc = onvif_ptz_absolute_move(ptz_url, profile_token, config.onvif_username, config.onvif_password, pan, tilt, zoom);

    if (rc != 0) {
        mg_send_json_error(c, 500, "PTZ absolute move failed");
        return;
    }

    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "message", "PTZ absolute move completed");

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    mg_send_json_response(c, 200, json_str);
    free(json_str);
}

void mg_handle_ptz_relative(struct mg_connection *c, struct mg_http_message *hm) {
    char stream_name[MAX_STREAM_NAME];
    if (extract_ptz_stream_name(hm, stream_name, sizeof(stream_name)) != 0) {
        mg_send_json_error(c, 400, "Invalid stream name");
        return;
    }

    log_info("Handling POST /api/streams/%s/ptz/relative", stream_name);

    stream_config_t config;
    int rc = get_ptz_stream_config(stream_name, &config);
    if (rc == -1) {
        mg_send_json_error(c, 404, "Stream not found");
        return;
    } else if (rc == -2) {
        mg_send_json_error(c, 400, "PTZ not enabled for this stream");
        return;
    }

    cJSON *body = cJSON_ParseWithLength(hm->body.buf, hm->body.len);
    if (!body) {
        mg_send_json_error(c, 400, "Invalid JSON body");
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

    char ptz_url[512];
    if (build_ptz_url(&config, ptz_url, sizeof(ptz_url)) != 0) {
        mg_send_json_error(c, 500, "Failed to build PTZ URL");
        return;
    }

    const char *profile_token = get_profile_token(&config);
    rc = onvif_ptz_relative_move(ptz_url, profile_token, config.onvif_username, config.onvif_password, pan, tilt, zoom);

    if (rc != 0) {
        mg_send_json_error(c, 500, "PTZ relative move failed");
        return;
    }

    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "message", "PTZ relative move completed");

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    mg_send_json_response(c, 200, json_str);
    free(json_str);
}

void mg_handle_ptz_home(struct mg_connection *c, struct mg_http_message *hm) {
    char stream_name[MAX_STREAM_NAME];
    if (extract_ptz_stream_name(hm, stream_name, sizeof(stream_name)) != 0) {
        mg_send_json_error(c, 400, "Invalid stream name");
        return;
    }

    log_info("Handling POST /api/streams/%s/ptz/home", stream_name);

    stream_config_t config;
    int rc = get_ptz_stream_config(stream_name, &config);
    if (rc == -1) {
        mg_send_json_error(c, 404, "Stream not found");
        return;
    } else if (rc == -2) {
        mg_send_json_error(c, 400, "PTZ not enabled for this stream");
        return;
    }

    char ptz_url[512];
    if (build_ptz_url(&config, ptz_url, sizeof(ptz_url)) != 0) {
        mg_send_json_error(c, 500, "Failed to build PTZ URL");
        return;
    }

    const char *profile_token = get_profile_token(&config);
    rc = onvif_ptz_goto_home(ptz_url, profile_token, config.onvif_username, config.onvif_password);

    if (rc != 0) {
        mg_send_json_error(c, 500, "PTZ go to home failed");
        return;
    }

    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "message", "PTZ moved to home position");

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    mg_send_json_response(c, 200, json_str);
    free(json_str);
}

void mg_handle_ptz_set_home(struct mg_connection *c, struct mg_http_message *hm) {
    char stream_name[MAX_STREAM_NAME];
    if (extract_ptz_stream_name(hm, stream_name, sizeof(stream_name)) != 0) {
        mg_send_json_error(c, 400, "Invalid stream name");
        return;
    }

    log_info("Handling POST /api/streams/%s/ptz/sethome", stream_name);

    stream_config_t config;
    int rc = get_ptz_stream_config(stream_name, &config);
    if (rc == -1) {
        mg_send_json_error(c, 404, "Stream not found");
        return;
    } else if (rc == -2) {
        mg_send_json_error(c, 400, "PTZ not enabled for this stream");
        return;
    }

    char ptz_url[512];
    if (build_ptz_url(&config, ptz_url, sizeof(ptz_url)) != 0) {
        mg_send_json_error(c, 500, "Failed to build PTZ URL");
        return;
    }

    const char *profile_token = get_profile_token(&config);
    rc = onvif_ptz_set_home(ptz_url, profile_token, config.onvif_username, config.onvif_password);

    if (rc != 0) {
        mg_send_json_error(c, 500, "PTZ set home failed");
        return;
    }

    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "message", "PTZ home position set");

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    mg_send_json_response(c, 200, json_str);
    free(json_str);
}

void mg_handle_ptz_get_presets(struct mg_connection *c, struct mg_http_message *hm) {
    char stream_name[MAX_STREAM_NAME];
    if (extract_ptz_stream_name(hm, stream_name, sizeof(stream_name)) != 0) {
        mg_send_json_error(c, 400, "Invalid stream name");
        return;
    }

    log_info("Handling GET /api/streams/%s/ptz/presets", stream_name);

    stream_config_t config;
    int rc = get_ptz_stream_config(stream_name, &config);
    if (rc == -1) {
        mg_send_json_error(c, 404, "Stream not found");
        return;
    } else if (rc == -2) {
        mg_send_json_error(c, 400, "PTZ not enabled for this stream");
        return;
    }

    char ptz_url[512];
    if (build_ptz_url(&config, ptz_url, sizeof(ptz_url)) != 0) {
        mg_send_json_error(c, 500, "Failed to build PTZ URL");
        return;
    }

    const char *profile_token = get_profile_token(&config);
    onvif_ptz_preset_t presets[32];
    int count = onvif_ptz_get_presets(ptz_url, profile_token, config.onvif_username, config.onvif_password, presets, 32);

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

    mg_send_json_response(c, 200, json_str);
    free(json_str);
}

void mg_handle_ptz_goto_preset(struct mg_connection *c, struct mg_http_message *hm) {
    char stream_name[MAX_STREAM_NAME];
    if (extract_ptz_stream_name(hm, stream_name, sizeof(stream_name)) != 0) {
        mg_send_json_error(c, 400, "Invalid stream name");
        return;
    }

    // Get preset token from request body
    cJSON *body = cJSON_ParseWithLength(hm->body.buf, hm->body.len);
    if (!body) {
        mg_send_json_error(c, 400, "Invalid JSON body");
        return;
    }

    cJSON *token_json = cJSON_GetObjectItem(body, "token");
    if (!token_json || !cJSON_IsString(token_json)) {
        cJSON_Delete(body);
        mg_send_json_error(c, 400, "Missing preset token");
        return;
    }

    char preset_token[64];
    strncpy(preset_token, token_json->valuestring, sizeof(preset_token) - 1);
    preset_token[sizeof(preset_token) - 1] = '\0';
    cJSON_Delete(body);

    log_info("Handling POST /api/streams/%s/ptz/preset (goto %s)", stream_name, preset_token);

    stream_config_t config;
    int rc = get_ptz_stream_config(stream_name, &config);
    if (rc == -1) {
        mg_send_json_error(c, 404, "Stream not found");
        return;
    } else if (rc == -2) {
        mg_send_json_error(c, 400, "PTZ not enabled for this stream");
        return;
    }

    char ptz_url[512];
    if (build_ptz_url(&config, ptz_url, sizeof(ptz_url)) != 0) {
        mg_send_json_error(c, 500, "Failed to build PTZ URL");
        return;
    }

    const char *profile_token = get_profile_token(&config);
    rc = onvif_ptz_goto_preset(ptz_url, profile_token, config.onvif_username, config.onvif_password, preset_token);

    if (rc != 0) {
        mg_send_json_error(c, 500, "PTZ go to preset failed");
        return;
    }

    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "message", "PTZ moved to preset");

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    mg_send_json_response(c, 200, json_str);
    free(json_str);
}

void mg_handle_ptz_set_preset(struct mg_connection *c, struct mg_http_message *hm) {
    char stream_name[MAX_STREAM_NAME];
    if (extract_ptz_stream_name(hm, stream_name, sizeof(stream_name)) != 0) {
        mg_send_json_error(c, 400, "Invalid stream name");
        return;
    }

    log_info("Handling PUT /api/streams/%s/ptz/preset", stream_name);

    stream_config_t config;
    int rc = get_ptz_stream_config(stream_name, &config);
    if (rc == -1) {
        mg_send_json_error(c, 404, "Stream not found");
        return;
    } else if (rc == -2) {
        mg_send_json_error(c, 400, "PTZ not enabled for this stream");
        return;
    }

    cJSON *body = cJSON_ParseWithLength(hm->body.buf, hm->body.len);
    if (!body) {
        mg_send_json_error(c, 400, "Invalid JSON body");
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
        mg_send_json_error(c, 500, "Failed to build PTZ URL");
        return;
    }

    const char *profile_token = get_profile_token(&config);
    char new_token[64] = {0};
    rc = onvif_ptz_set_preset(ptz_url, profile_token, config.onvif_username, config.onvif_password,
                              preset_name, new_token, sizeof(new_token));

    cJSON_Delete(body);

    if (rc != 0) {
        mg_send_json_error(c, 500, "PTZ set preset failed");
        return;
    }

    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "message", "PTZ preset created");
    cJSON_AddStringToObject(response, "token", new_token);

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    mg_send_json_response(c, 200, json_str);
    free(json_str);
}

void mg_handle_ptz_capabilities(struct mg_connection *c, struct mg_http_message *hm) {
    char stream_name[MAX_STREAM_NAME];
    if (extract_ptz_stream_name(hm, stream_name, sizeof(stream_name)) != 0) {
        mg_send_json_error(c, 400, "Invalid stream name");
        return;
    }

    log_info("Handling GET /api/streams/%s/ptz/capabilities", stream_name);

    stream_config_t config;
    int rc = get_ptz_stream_config(stream_name, &config);
    if (rc == -1) {
        mg_send_json_error(c, 404, "Stream not found");
        return;
    } else if (rc == -2) {
        mg_send_json_error(c, 400, "PTZ not enabled for this stream");
        return;
    }

    char ptz_url[512];
    if (build_ptz_url(&config, ptz_url, sizeof(ptz_url)) != 0) {
        mg_send_json_error(c, 500, "Failed to build PTZ URL");
        return;
    }

    const char *profile_token = get_profile_token(&config);
    onvif_ptz_capabilities_t caps;
    rc = onvif_ptz_get_capabilities(ptz_url, profile_token, config.onvif_username, config.onvif_password, &caps);

    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "ptz_enabled", config.ptz_enabled);
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

    mg_send_json_response(c, 200, json_str);
    free(json_str);
}
