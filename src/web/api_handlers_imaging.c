#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>

#include "web/api_handlers_imaging.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#define LOG_COMPONENT "ImagingAPI"
#include "core/logger.h"
#include "core/config.h"
#include "core/url_utils.h"
#include "utils/strings.h"
#include "database/db_streams.h"
#include "video/onvif_device_management.h"
#include "video/onvif_imaging.h"
#include <cjson/cJSON.h>

static int extract_imaging_stream_name(const http_request_t *req, char *stream_name, size_t name_size) {
    if (http_request_extract_path_param(req, "/api/streams/", stream_name, name_size) != 0) {
        return -1;
    }

    char *suffix = strstr(stream_name, "/imaging");
    if (!suffix) {
        suffix = strstr(stream_name, "/daynight");
    }
    if (suffix) {
        *suffix = '\0';
    }

    return stream_name[0] != '\0' ? 0 : -1;
}

static int get_imaging_stream_config(const char *stream_name, stream_config_t *config) {
    if (!stream_name || !config) {
        return -1;
    }

    if (get_stream_config_by_name(stream_name, config) != 0) {
        log_error("Stream not found: %s", stream_name);
        return -1;
    }

    if (!config->is_onvif) {
        log_error("ONVIF is not enabled for stream: %s", stream_name);
        return -2;
    }

    return 0;
}

static int build_imaging_url(const stream_config_t *config, char *imaging_url, size_t url_size) {
    return url_build_onvif_service_url(config->url, config->onvif_port,
                                       "/onvif/imaging_service", imaging_url, url_size);
}

static int build_device_url(const stream_config_t *config, char *device_url, size_t url_size) {
    return url_build_onvif_service_url(config->url, config->onvif_port,
                                       "/onvif/device_service", device_url, url_size);
}

static const char *config_username(const stream_config_t *config) {
    return config->onvif_username[0] ? config->onvif_username : NULL;
}

static const char *config_password(const stream_config_t *config) {
    return config->onvif_password[0] ? config->onvif_password : NULL;
}

static int resolve_video_source_token(const stream_config_t *config,
                                      char *video_source_token,
                                      size_t token_size) {
    char device_url[MAX_URL_LENGTH];
    if (build_device_url(config, device_url, sizeof(device_url)) == 0) {
        if (get_onvif_video_source_token(device_url,
                                         config_username(config),
                                         config_password(config),
                                         config->onvif_profile,
                                         video_source_token,
                                         token_size) == 0 &&
            video_source_token[0] != '\0') {
            return 0;
        }
    }

    if (config->onvif_profile[0] != '\0') {
        log_warn("Falling back to ONVIF profile token as imaging VideoSourceToken for stream %s",
                 config->name);
        safe_strcpy(video_source_token, config->onvif_profile, token_size, 0);
        return 0;
    }

    log_warn("Falling back to default Thingino/onvif_simple_server VideoSourceToken for stream %s",
             config->name);
    safe_strcpy(video_source_token, "VideoSourceToken", token_size, 0);
    return 0;
}

static void add_float_setting(cJSON *root, const char *name, const onvif_imaging_float_t *setting) {
    if (setting && setting->present) {
        cJSON_AddNumberToObject(root, name, setting->value);
    }
}

static void add_mode_level_setting(cJSON *root, const char *name,
                                   const onvif_imaging_mode_level_t *setting) {
    if (!setting || !setting->present) {
        return;
    }

    cJSON *obj = cJSON_AddObjectToObject(root, name);
    if (!obj) {
        return;
    }
    if (setting->mode[0] != '\0') {
        cJSON_AddStringToObject(obj, "mode", setting->mode);
    }
    if (setting->has_level) {
        cJSON_AddNumberToObject(obj, "level", setting->level);
    }
}

static cJSON *settings_to_json(const onvif_imaging_settings_t *settings) {
    cJSON *root = cJSON_CreateObject();
    if (!root) {
        return NULL;
    }

    cJSON_AddStringToObject(root, "video_source_token", settings->video_source_token);
    add_float_setting(root, "brightness", &settings->brightness);
    add_float_setting(root, "color_saturation", &settings->color_saturation);
    add_float_setting(root, "contrast", &settings->contrast);
    add_float_setting(root, "sharpness", &settings->sharpness);
    add_float_setting(root, "noise_reduction", &settings->noise_reduction);
    add_mode_level_setting(root, "backlight_compensation", &settings->backlight_compensation);
    add_mode_level_setting(root, "wide_dynamic_range", &settings->wide_dynamic_range);
    add_mode_level_setting(root, "tone_compensation", &settings->tone_compensation);
    add_mode_level_setting(root, "defogging", &settings->defogging);
    if (settings->has_ir_cut_filter) {
        cJSON_AddStringToObject(root, "ir_cut_filter", settings->ir_cut_filter);
    }

    return root;
}

static void add_float_range(cJSON *root, const char *name,
                            const onvif_imaging_float_range_t *range) {
    if (!range || !range->present) {
        return;
    }

    cJSON *obj = cJSON_AddObjectToObject(root, name);
    if (!obj) {
        return;
    }
    cJSON_AddNumberToObject(obj, "min", range->min);
    cJSON_AddNumberToObject(obj, "max", range->max);
}

static void add_mode_level_options(cJSON *root, const char *name,
                                   const onvif_imaging_mode_level_options_t *options) {
    if (!options || !options->present) {
        return;
    }

    cJSON *obj = cJSON_AddObjectToObject(root, name);
    if (!obj) {
        return;
    }

    cJSON *modes = cJSON_AddArrayToObject(obj, "modes");
    if (modes) {
        for (int i = 0; i < options->mode_count; i++) {
            cJSON_AddItemToArray(modes, cJSON_CreateString(options->modes[i]));
        }
    }

    add_float_range(obj, "level", &options->level);
}

static cJSON *options_to_json(const onvif_imaging_options_t *options) {
    cJSON *root = cJSON_CreateObject();
    if (!root) {
        return NULL;
    }

    cJSON_AddStringToObject(root, "video_source_token", options->video_source_token);
    add_float_range(root, "brightness", &options->brightness);
    add_float_range(root, "color_saturation", &options->color_saturation);
    add_float_range(root, "contrast", &options->contrast);
    add_float_range(root, "sharpness", &options->sharpness);
    add_float_range(root, "noise_reduction", &options->noise_reduction);
    add_mode_level_options(root, "backlight_compensation", &options->backlight_compensation);
    add_mode_level_options(root, "wide_dynamic_range", &options->wide_dynamic_range);
    add_mode_level_options(root, "tone_compensation", &options->tone_compensation);
    add_mode_level_options(root, "defogging", &options->defogging);

    cJSON *ir_modes = cJSON_AddArrayToObject(root, "ir_cut_filter_modes");
    if (ir_modes) {
        for (int i = 0; i < options->ir_cut_filter_mode_count; i++) {
            cJSON_AddItemToArray(ir_modes, cJSON_CreateString(options->ir_cut_filter_modes[i]));
        }
    }

    return root;
}

static int send_json_object(http_response_t *res, cJSON *root) {
    if (!root) {
        http_response_set_json_error(res, 500, "Failed to create JSON response");
        return -1;
    }

    char *json = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if (!json) {
        http_response_set_json_error(res, 500, "Failed to generate JSON response");
        return -1;
    }

    http_response_set_json(res, 200, json);
    free(json);
    return 0;
}

static bool read_number(cJSON *body, const char *name, float *value) {
    cJSON *item = cJSON_GetObjectItem(body, name);
    if (item && cJSON_IsNumber(item)) {
        *value = (float)item->valuedouble;
        return true;
    }
    return false;
}

static bool read_string(cJSON *body, const char *name, char *value, size_t value_size) {
    cJSON *item = cJSON_GetObjectItem(body, name);
    if (item && cJSON_IsString(item)) {
        safe_strcpy(value, item->valuestring, value_size, 0);
        return true;
    }
    return false;
}

static void canonicalize_ir_cut_filter(char *value, size_t value_size) {
    if (!value || value_size == 0) {
        return;
    }

    if (strcasecmp(value, "on") == 0) {
        safe_strcpy(value, "On", value_size, 0);
    } else if (strcasecmp(value, "off") == 0) {
        safe_strcpy(value, "Off", value_size, 0);
    } else if (strcasecmp(value, "auto") == 0) {
        safe_strcpy(value, "Auto", value_size, 0);
    }
}

static bool valid_ir_cut_filter(const char *value) {
    return value &&
           (strcmp(value, "On") == 0 || strcmp(value, "Off") == 0 || strcmp(value, "Auto") == 0);
}

static const char *ir_cut_filter_to_daynight_mode(const char *ir_cut_filter) {
    if (!ir_cut_filter) {
        return "unknown";
    }

    if (strcasecmp(ir_cut_filter, "Auto") == 0) {
        return "auto";
    }
    if (strcasecmp(ir_cut_filter, "On") == 0) {
        return "day";
    }
    if (strcasecmp(ir_cut_filter, "Off") == 0) {
        return "night";
    }

    return "unknown";
}

static bool daynight_mode_to_ir_cut_filter(const char *mode, char *ir_cut_filter, size_t value_size) {
    if (!mode || !ir_cut_filter || value_size == 0) {
        return false;
    }

    if (strcasecmp(mode, "auto") == 0) {
        safe_strcpy(ir_cut_filter, "Auto", value_size, 0);
        return true;
    }
    if (strcasecmp(mode, "day") == 0 || strcasecmp(mode, "color") == 0) {
        safe_strcpy(ir_cut_filter, "On", value_size, 0);
        return true;
    }
    if (strcasecmp(mode, "night") == 0 || strcasecmp(mode, "bw") == 0 ||
        strcasecmp(mode, "blackwhite") == 0) {
        safe_strcpy(ir_cut_filter, "Off", value_size, 0);
        return true;
    }

    safe_strcpy(ir_cut_filter, mode, value_size, 0);
    canonicalize_ir_cut_filter(ir_cut_filter, value_size);
    return valid_ir_cut_filter(ir_cut_filter);
}

static void add_supported_daynight_mode(cJSON *array, const char *ir_cut_filter) {
    if (!array || !ir_cut_filter) {
        return;
    }

    const char *mode = ir_cut_filter_to_daynight_mode(ir_cut_filter);
    if (strcmp(mode, "unknown") == 0) {
        return;
    }

    int count = cJSON_GetArraySize(array);
    for (int i = 0; i < count; i++) {
        cJSON *existing = cJSON_GetArrayItem(array, i);
        if (existing && cJSON_IsString(existing) && strcmp(existing->valuestring, mode) == 0) {
            return;
        }
    }

    cJSON_AddItemToArray(array, cJSON_CreateString(mode));
}

static void add_default_supported_daynight_modes(cJSON *array) {
    if (!array || cJSON_GetArraySize(array) > 0) {
        return;
    }

    cJSON_AddItemToArray(array, cJSON_CreateString("day"));
    cJSON_AddItemToArray(array, cJSON_CreateString("night"));
    cJSON_AddItemToArray(array, cJSON_CreateString("auto"));
}

static bool read_mode_level(cJSON *body, const char *name,
                            onvif_imaging_mode_level_t *setting) {
    cJSON *item = cJSON_GetObjectItem(body, name);
    if (!item || !setting) {
        return false;
    }

    if (cJSON_IsNumber(item)) {
        setting->present = true;
        setting->has_level = true;
        setting->level = (float)item->valuedouble;
        return true;
    }

    if (!cJSON_IsObject(item)) {
        return false;
    }

    bool changed = false;
    cJSON *mode = cJSON_GetObjectItem(item, "mode");
    if (mode && cJSON_IsString(mode)) {
        safe_strcpy(setting->mode, mode->valuestring, sizeof(setting->mode), 0);
        changed = true;
    }

    cJSON *level = cJSON_GetObjectItem(item, "level");
    if (level && cJSON_IsNumber(level)) {
        setting->has_level = true;
        setting->level = (float)level->valuedouble;
        changed = true;
    }

    setting->present = changed;
    return changed;
}

static bool parse_settings_update(cJSON *body, onvif_imaging_settings_t *settings,
                                  bool *force_persistence, char *error, size_t error_size) {
    bool changed = false;
    float value = 0.0f;

    memset(settings, 0, sizeof(*settings));
    *force_persistence = true;

    if (read_number(body, "brightness", &value)) {
        settings->brightness.present = true;
        settings->brightness.value = value;
        changed = true;
    }
    if (read_number(body, "color_saturation", &value) || read_number(body, "saturation", &value)) {
        settings->color_saturation.present = true;
        settings->color_saturation.value = value;
        changed = true;
    }
    if (read_number(body, "contrast", &value)) {
        settings->contrast.present = true;
        settings->contrast.value = value;
        changed = true;
    }
    if (read_number(body, "sharpness", &value)) {
        settings->sharpness.present = true;
        settings->sharpness.value = value;
        changed = true;
    }
    if (read_number(body, "noise_reduction", &value)) {
        settings->noise_reduction.present = true;
        settings->noise_reduction.value = value;
        changed = true;
    }

    changed = read_mode_level(body, "backlight_compensation", &settings->backlight_compensation) || changed;
    changed = read_mode_level(body, "backlight", &settings->backlight_compensation) || changed;
    changed = read_mode_level(body, "wide_dynamic_range", &settings->wide_dynamic_range) || changed;
    changed = read_mode_level(body, "wdr", &settings->wide_dynamic_range) || changed;
    changed = read_mode_level(body, "tone_compensation", &settings->tone_compensation) || changed;
    changed = read_mode_level(body, "tone", &settings->tone_compensation) || changed;
    changed = read_mode_level(body, "defogging", &settings->defogging) || changed;
    changed = read_mode_level(body, "defog", &settings->defogging) || changed;

    if (read_string(body, "ir_cut_filter", settings->ir_cut_filter, sizeof(settings->ir_cut_filter)) ||
        read_string(body, "ircut", settings->ir_cut_filter, sizeof(settings->ir_cut_filter))) {
        canonicalize_ir_cut_filter(settings->ir_cut_filter, sizeof(settings->ir_cut_filter));
        if (!valid_ir_cut_filter(settings->ir_cut_filter)) {
            snprintf(error, error_size, "ir_cut_filter must be On, Off, or Auto");
            return false;
        }
        settings->has_ir_cut_filter = true;
        changed = true;
    }

    cJSON *force = cJSON_GetObjectItem(body, "force_persistence");
    if (force && cJSON_IsBool(force)) {
        *force_persistence = cJSON_IsTrue(force);
    }

    if (!changed) {
        snprintf(error, error_size, "No imaging settings provided");
        return false;
    }

    return true;
}

static int prepare_imaging_request(const http_request_t *req,
                                   http_response_t *res,
                                   stream_config_t *config,
                                   char *imaging_url,
                                   size_t imaging_url_size,
                                   char *video_source_token,
                                   size_t token_size) {
    char stream_name[MAX_STREAM_NAME];
    if (extract_imaging_stream_name(req, stream_name, sizeof(stream_name)) != 0) {
        http_response_set_json_error(res, 400, "Invalid stream name");
        return -1;
    }

    int rc = get_imaging_stream_config(stream_name, config);
    if (rc == -1) {
        http_response_set_json_error(res, 404, "Stream not found");
        return -1;
    } else if (rc == -2) {
        http_response_set_json_error(res, 400, "ONVIF is not enabled for this stream");
        return -1;
    }

    if (build_imaging_url(config, imaging_url, imaging_url_size) != 0) {
        http_response_set_json_error(res, 500, "Failed to build ONVIF Imaging URL");
        return -1;
    }

    if (resolve_video_source_token(config, video_source_token, token_size) != 0) {
        http_response_set_json_error(res, 500, "Failed to resolve ONVIF video source token");
        return -1;
    }

    return 0;
}

void handle_imaging_get_settings(const http_request_t *req, http_response_t *res) {
    stream_config_t config;
    char imaging_url[MAX_URL_LENGTH];
    char video_source_token[64];

    if (prepare_imaging_request(req, res, &config, imaging_url, sizeof(imaging_url),
                                video_source_token, sizeof(video_source_token)) != 0) {
        return;
    }

    onvif_imaging_settings_t settings;
    if (onvif_imaging_get_settings(imaging_url, video_source_token,
                                   config_username(&config), config_password(&config),
                                   &settings) != 0) {
        http_response_set_json_error(res, 502, "Failed to get ONVIF imaging settings");
        return;
    }

    cJSON *root = cJSON_CreateObject();
    if (!root) {
        http_response_set_json_error(res, 500, "Failed to create JSON response");
        return;
    }
    cJSON *settings_json = settings_to_json(&settings);
    if (!settings_json) {
        cJSON_Delete(root);
        http_response_set_json_error(res, 500, "Failed to create JSON response");
        return;
    }
    cJSON_AddBoolToObject(root, "success", true);
    cJSON_AddItemToObject(root, "settings", settings_json);
    send_json_object(res, root);
}

void handle_imaging_get_options(const http_request_t *req, http_response_t *res) {
    stream_config_t config;
    char imaging_url[MAX_URL_LENGTH];
    char video_source_token[64];

    if (prepare_imaging_request(req, res, &config, imaging_url, sizeof(imaging_url),
                                video_source_token, sizeof(video_source_token)) != 0) {
        return;
    }

    onvif_imaging_options_t options;
    if (onvif_imaging_get_options(imaging_url, video_source_token,
                                  config_username(&config), config_password(&config),
                                  &options) != 0) {
        http_response_set_json_error(res, 502, "Failed to get ONVIF imaging options");
        return;
    }

    cJSON *root = cJSON_CreateObject();
    if (!root) {
        http_response_set_json_error(res, 500, "Failed to create JSON response");
        return;
    }
    cJSON *options_json = options_to_json(&options);
    if (!options_json) {
        cJSON_Delete(root);
        http_response_set_json_error(res, 500, "Failed to create JSON response");
        return;
    }
    cJSON_AddBoolToObject(root, "success", true);
    cJSON_AddItemToObject(root, "options", options_json);
    send_json_object(res, root);
}

void handle_imaging_put_settings(const http_request_t *req, http_response_t *res) {
    stream_config_t config;
    char imaging_url[MAX_URL_LENGTH];
    char video_source_token[64];

    if (prepare_imaging_request(req, res, &config, imaging_url, sizeof(imaging_url),
                                video_source_token, sizeof(video_source_token)) != 0) {
        return;
    }

    cJSON *body = httpd_parse_json_body(req);
    if (!body) {
        http_response_set_json_error(res, 400, "Invalid JSON body");
        return;
    }

    onvif_imaging_settings_t settings;
    bool force_persistence = true;
    char error[128] = {0};
    if (!parse_settings_update(body, &settings, &force_persistence, error, sizeof(error))) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400, error[0] ? error : "Invalid imaging settings");
        return;
    }
    cJSON_Delete(body);

    if (onvif_imaging_set_settings(imaging_url, video_source_token,
                                   config_username(&config), config_password(&config),
                                   &settings, force_persistence) != 0) {
        http_response_set_json_error(res, 502, "Failed to set ONVIF imaging settings");
        return;
    }

    cJSON *root = cJSON_CreateObject();
    if (!root) {
        http_response_set_json_error(res, 500, "Failed to create JSON response");
        return;
    }
    cJSON_AddBoolToObject(root, "success", true);
    cJSON_AddStringToObject(root, "message", "ONVIF imaging settings updated");
    send_json_object(res, root);
}

void handle_daynight_get(const http_request_t *req, http_response_t *res) {
    stream_config_t config;
    char imaging_url[MAX_URL_LENGTH];
    char video_source_token[64];

    if (prepare_imaging_request(req, res, &config, imaging_url, sizeof(imaging_url),
                                video_source_token, sizeof(video_source_token)) != 0) {
        return;
    }

    onvif_imaging_settings_t settings;
    if (onvif_imaging_get_settings(imaging_url, video_source_token,
                                   config_username(&config), config_password(&config),
                                   &settings) != 0) {
        http_response_set_json_error(res, 502, "Failed to get ONVIF day/night settings");
        return;
    }

    cJSON *root = cJSON_CreateObject();
    if (!root) {
        http_response_set_json_error(res, 500, "Failed to create JSON response");
        return;
    }

    cJSON_AddBoolToObject(root, "success", true);
    cJSON_AddStringToObject(root, "source", "onvif-imaging");
    cJSON_AddStringToObject(root, "video_source_token", video_source_token);
    cJSON_AddStringToObject(root, "mode",
                            settings.has_ir_cut_filter
                                ? ir_cut_filter_to_daynight_mode(settings.ir_cut_filter)
                                : "unknown");
    if (settings.has_ir_cut_filter) {
        cJSON_AddStringToObject(root, "ir_cut_filter", settings.ir_cut_filter);
    }

    cJSON *supported_modes = cJSON_AddArrayToObject(root, "supported_modes");
    onvif_imaging_options_t options;
    bool options_available = false;
    if (onvif_imaging_get_options(imaging_url, video_source_token,
                                  config_username(&config), config_password(&config),
                                  &options) == 0) {
        options_available = true;
        for (int i = 0; i < options.ir_cut_filter_mode_count; i++) {
            add_supported_daynight_mode(supported_modes, options.ir_cut_filter_modes[i]);
        }
    } else {
        log_warn("Failed to get ONVIF day/night options for stream %s", config.name);
    }
    add_default_supported_daynight_modes(supported_modes);
    cJSON_AddBoolToObject(root, "options_available", options_available);

    send_json_object(res, root);
}

void handle_daynight_put(const http_request_t *req, http_response_t *res) {
    stream_config_t config;
    char imaging_url[MAX_URL_LENGTH];
    char video_source_token[64];

    if (prepare_imaging_request(req, res, &config, imaging_url, sizeof(imaging_url),
                                video_source_token, sizeof(video_source_token)) != 0) {
        return;
    }

    cJSON *body = httpd_parse_json_body(req);
    if (!body) {
        http_response_set_json_error(res, 400, "Invalid JSON body");
        return;
    }

    char ir_cut_filter[16] = {0};
    cJSON *mode = cJSON_GetObjectItem(body, "mode");
    cJSON *daynight = cJSON_GetObjectItem(body, "daynight");
    cJSON *ir_cut = cJSON_GetObjectItem(body, "ir_cut_filter");
    cJSON *ircut = cJSON_GetObjectItem(body, "ircut");

    bool valid = false;
    if (mode && cJSON_IsString(mode)) {
        valid = daynight_mode_to_ir_cut_filter(mode->valuestring, ir_cut_filter, sizeof(ir_cut_filter));
    } else if (daynight && cJSON_IsString(daynight)) {
        valid = daynight_mode_to_ir_cut_filter(daynight->valuestring, ir_cut_filter, sizeof(ir_cut_filter));
    } else if (ir_cut && cJSON_IsString(ir_cut)) {
        valid = daynight_mode_to_ir_cut_filter(ir_cut->valuestring, ir_cut_filter, sizeof(ir_cut_filter));
    } else if (ircut && cJSON_IsString(ircut)) {
        valid = daynight_mode_to_ir_cut_filter(ircut->valuestring, ir_cut_filter, sizeof(ir_cut_filter));
    }

    cJSON *force = cJSON_GetObjectItem(body, "force_persistence");
    bool force_persistence = true;
    if (force && cJSON_IsBool(force)) {
        force_persistence = cJSON_IsTrue(force);
    }
    cJSON_Delete(body);

    if (!valid) {
        http_response_set_json_error(res, 400,
                                     "mode must be auto, day, or night; ir_cut_filter must be On, Off, or Auto");
        return;
    }

    onvif_imaging_settings_t settings;
    memset(&settings, 0, sizeof(settings));
    settings.has_ir_cut_filter = true;
    safe_strcpy(settings.ir_cut_filter, ir_cut_filter, sizeof(settings.ir_cut_filter), 0);

    if (onvif_imaging_set_settings(imaging_url, video_source_token,
                                   config_username(&config), config_password(&config),
                                   &settings, force_persistence) != 0) {
        http_response_set_json_error(res, 502, "Failed to set ONVIF day/night mode");
        return;
    }

    cJSON *root = cJSON_CreateObject();
    if (!root) {
        http_response_set_json_error(res, 500, "Failed to create JSON response");
        return;
    }
    cJSON_AddBoolToObject(root, "success", true);
    cJSON_AddStringToObject(root, "source", "onvif-imaging");
    cJSON_AddStringToObject(root, "mode", ir_cut_filter_to_daynight_mode(ir_cut_filter));
    cJSON_AddStringToObject(root, "ir_cut_filter", ir_cut_filter);
    send_json_object(res, root);
}
