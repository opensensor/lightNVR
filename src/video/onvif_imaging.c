#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <stdbool.h>
#include <curl/curl.h>

#include "ezxml.h"
#include "video/onvif_imaging.h"
#include "video/onvif_soap.h"
#include "core/config.h"
#include "core/logger.h"
#include "core/url_utils.h"
#include "utils/strings.h"

typedef struct {
    char *memory;
    size_t size;
} MemoryStruct;

static size_t write_callback(void *contents, size_t size, size_t nmemb, void *userp) {
    size_t real_size = size * nmemb;
    MemoryStruct *mem = (MemoryStruct *)userp;

    char *ptr = realloc(mem->memory, mem->size + real_size + 1);
    if (!ptr) {
        log_error("Not enough memory for ONVIF Imaging response");
        return 0;
    }

    mem->memory = ptr;
    memcpy(mem->memory + mem->size, contents, real_size);
    mem->size += real_size;
    mem->memory[mem->size] = '\0';

    return real_size;
}

static int append_escaped(char **dst, size_t *remaining, const char *text) {
    if (!dst || !*dst || !remaining || !text) {
        return -1;
    }

    for (const char *p = text; *p; p++) {
        const char *replacement = NULL;
        char literal[2] = {*p, '\0'};

        switch (*p) {
        case '&': replacement = "&amp;"; break;
        case '<': replacement = "&lt;"; break;
        case '>': replacement = "&gt;"; break;
        case '"': replacement = "&quot;"; break;
        case '\'': replacement = "&apos;"; break;
        default: replacement = literal; break;
        }

        size_t len = strlen(replacement);
        if (*remaining <= len) {
            return -1;
        }

        memcpy(*dst, replacement, len);
        *dst += len;
        *remaining -= len;
    }

    **dst = '\0';
    return 0;
}

static int xml_escape(const char *src, char *dst, size_t dst_size) {
    if (!src || !dst || dst_size == 0) {
        return -1;
    }

    char *out = dst;
    size_t remaining = dst_size;
    *out = '\0';
    return append_escaped(&out, &remaining, src);
}

static char *send_imaging_soap_request(const char *imaging_url,
                                       const char *soap_action,
                                       const char *request_body,
                                       const char *username,
                                       const char *password) {
    if (!imaging_url || !request_body) {
        return NULL;
    }

    CURL *curl = curl_easy_init();
    if (!curl) {
        log_error("Failed to initialize CURL for ONVIF Imaging request");
        return NULL;
    }

    MemoryStruct chunk = {0};
    chunk.memory = malloc(1);
    if (!chunk.memory) {
        curl_easy_cleanup(curl);
        return NULL;
    }
    chunk.memory[0] = '\0';

    char *security_header = NULL;
    if (username && password && username[0] != '\0' && password[0] != '\0') {
        security_header = onvif_create_security_header(username, password);
        if (!security_header) {
            free(chunk.memory);
            curl_easy_cleanup(curl);
            return NULL;
        }
    } else {
        security_header = strdup("");
        if (!security_header) {
            free(chunk.memory);
            curl_easy_cleanup(curl);
            return NULL;
        }
    }

    size_t envelope_size = strlen(request_body) + strlen(security_header) + 2048;
    char *soap_envelope = malloc(envelope_size);
    if (!soap_envelope) {
        free(security_header);
        free(chunk.memory);
        curl_easy_cleanup(curl);
        return NULL;
    }

    snprintf(soap_envelope, envelope_size,
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>"
        "<s:Envelope xmlns:s=\"http://www.w3.org/2003/05/soap-envelope\" "
        "xmlns:timg=\"http://www.onvif.org/ver20/imaging/wsdl\" "
        "xmlns:tt=\"http://www.onvif.org/ver10/schema\">"
        "<s:Header>%s</s:Header>"
        "<s:Body>%s</s:Body>"
        "</s:Envelope>",
        security_header, request_body);

    struct curl_slist *headers = NULL;
    headers = curl_slist_append(headers, "Content-Type: application/soap+xml; charset=utf-8");
    if (soap_action) {
        char soap_action_header[256];
        snprintf(soap_action_header, sizeof(soap_action_header), "SOAPAction: %s", soap_action);
        headers = curl_slist_append(headers, soap_action_header);
    }

    char safe_url[MAX_URL_LENGTH];
    if (url_redact_for_logging(imaging_url, safe_url, sizeof(safe_url)) != 0) {
        safe_strcpy(safe_url, "[invalid-url]", sizeof(safe_url), 0);
    }
    log_info("Sending ONVIF Imaging request to: %s", safe_url);

    curl_easy_setopt(curl, CURLOPT_URL, imaging_url);
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
    curl_easy_setopt(curl, CURLOPT_POSTFIELDS, soap_envelope);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_callback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, (void *)&chunk);
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 10L);
    curl_easy_setopt(curl, CURLOPT_SSL_VERIFYPEER, 0L);
    curl_easy_setopt(curl, CURLOPT_SSL_VERIFYHOST, 0L);

    char *response = NULL;
    CURLcode res = curl_easy_perform(curl);
    if (res != CURLE_OK) {
        log_error("ONVIF Imaging CURL request failed: %s", curl_easy_strerror(res));
    } else {
        long http_code = 0;
        curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
        if (http_code != 200) {
            log_error("ONVIF Imaging request failed with HTTP code %ld", http_code);
            if (chunk.size > 0) {
                onvif_log_soap_fault(chunk.memory, chunk.size, "Imaging");
            }
        } else if (chunk.size > 0) {
            response = chunk.memory;
            chunk.memory = NULL;
        }
    }

    curl_slist_free_all(headers);
    curl_easy_cleanup(curl);
    free(soap_envelope);
    free(security_header);
    free(chunk.memory);

    return response;
}

static const char *xml_local_name(const char *name) {
    const char *colon = name ? strchr(name, ':') : NULL;
    return colon ? colon + 1 : name;
}

static ezxml_t find_child_local(ezxml_t parent, const char *name) {
    if (!parent || !name) {
        return NULL;
    }

    for (ezxml_t child = parent->child; child; child = child->sibling) {
        const char *local = xml_local_name(child->name);
        if (local && strcmp(local, name) == 0) {
            return child;
        }
    }

    return NULL;
}

static ezxml_t find_descendant_local(ezxml_t root, const char *name) {
    if (!root || !name) {
        return NULL;
    }

    const char *local = xml_local_name(root->name);
    if (local && strcmp(local, name) == 0) {
        return root;
    }

    for (ezxml_t child = root->child; child; child = child->sibling) {
        ezxml_t found = find_descendant_local(child, name);
        if (found) {
            return found;
        }
    }

    return NULL;
}

static bool parse_float_text(const char *text, float *value) {
    if (!text || !value || text[0] == '\0') {
        return false;
    }

    char *end = NULL;
    float parsed = strtof(text, &end);
    if (end == text) {
        return false;
    }

    *value = parsed;
    return true;
}

static void parse_float_setting(ezxml_t parent, const char *name, onvif_imaging_float_t *out) {
    ezxml_t node = find_child_local(parent, name);
    if (!node || !out) {
        return;
    }

    if (parse_float_text(ezxml_txt(node), &out->value)) {
        out->present = true;
    }
}

static void parse_mode_level_setting(ezxml_t parent, const char *name,
                                     onvif_imaging_mode_level_t *out) {
    ezxml_t node = find_child_local(parent, name);
    if (!node || !out) {
        return;
    }

    out->present = true;

    ezxml_t mode = find_child_local(node, "Mode");
    if (mode) {
        safe_strcpy(out->mode, ezxml_txt(mode), sizeof(out->mode), 0);
    }

    ezxml_t level = find_child_local(node, "Level");
    if (level && parse_float_text(ezxml_txt(level), &out->level)) {
        out->has_level = true;
    }
}

static void parse_float_range(ezxml_t parent, const char *name,
                              onvif_imaging_float_range_t *out) {
    ezxml_t node = find_child_local(parent, name);
    if (!node || !out) {
        return;
    }

    ezxml_t min_node = find_child_local(node, "Min");
    ezxml_t max_node = find_child_local(node, "Max");
    float min_value = 0.0f;
    float max_value = 0.0f;

    if (min_node && max_node &&
        parse_float_text(ezxml_txt(min_node), &min_value) &&
        parse_float_text(ezxml_txt(max_node), &max_value)) {
        out->present = true;
        out->min = min_value;
        out->max = max_value;
    }
}

static void parse_mode_level_options(ezxml_t parent, const char *name,
                                     onvif_imaging_mode_level_options_t *out) {
    ezxml_t node = find_child_local(parent, name);
    if (!node || !out) {
        return;
    }

    out->present = true;

    for (ezxml_t child = node->child; child && out->mode_count < 4; child = child->sibling) {
        const char *local = xml_local_name(child->name);
        if (local && strcmp(local, "Mode") == 0) {
            safe_strcpy(out->modes[out->mode_count], ezxml_txt(child),
                        sizeof(out->modes[out->mode_count]), 0);
            out->mode_count++;
        }
    }

    parse_float_range(node, "Level", &out->level);
}

static int parse_settings_response(const char *response, onvif_imaging_settings_t *settings) {
    if (!response || !settings) {
        return -1;
    }

    memset(settings, 0, sizeof(*settings));

    ezxml_t xml = ezxml_parse_str((char *)response, strlen(response));
    if (!xml) {
        log_error("Failed to parse ONVIF Imaging settings XML");
        return -1;
    }

    ezxml_t settings_node = find_descendant_local(xml, "ImagingSettings");
    if (!settings_node) {
        log_error("ONVIF Imaging settings response did not include ImagingSettings");
        ezxml_free(xml);
        return -1;
    }

    ezxml_t token = find_child_local(settings_node, "VideoSourceToken");
    if (token) {
        safe_strcpy(settings->video_source_token, ezxml_txt(token),
                    sizeof(settings->video_source_token), 0);
    }

    parse_float_setting(settings_node, "Brightness", &settings->brightness);
    parse_float_setting(settings_node, "ColorSaturation", &settings->color_saturation);
    parse_float_setting(settings_node, "Contrast", &settings->contrast);
    parse_float_setting(settings_node, "Sharpness", &settings->sharpness);
    parse_float_setting(settings_node, "NoiseReduction", &settings->noise_reduction);
    parse_mode_level_setting(settings_node, "BacklightCompensation", &settings->backlight_compensation);
    parse_mode_level_setting(settings_node, "WideDynamicRange", &settings->wide_dynamic_range);
    parse_mode_level_setting(settings_node, "ToneCompensation", &settings->tone_compensation);
    parse_mode_level_setting(settings_node, "Defogging", &settings->defogging);

    ezxml_t ir_cut = find_child_local(settings_node, "IrCutFilter");
    if (ir_cut) {
        settings->has_ir_cut_filter = true;
        safe_strcpy(settings->ir_cut_filter, ezxml_txt(ir_cut), sizeof(settings->ir_cut_filter), 0);
    }

    ezxml_free(xml);
    return 0;
}

static int parse_options_response(const char *response, onvif_imaging_options_t *options) {
    if (!response || !options) {
        return -1;
    }

    memset(options, 0, sizeof(*options));

    ezxml_t xml = ezxml_parse_str((char *)response, strlen(response));
    if (!xml) {
        log_error("Failed to parse ONVIF Imaging options XML");
        return -1;
    }

    ezxml_t options_node = find_descendant_local(xml, "ImagingOptions");
    if (!options_node) {
        log_error("ONVIF Imaging options response did not include ImagingOptions");
        ezxml_free(xml);
        return -1;
    }

    ezxml_t token = find_child_local(options_node, "VideoSourceToken");
    if (token) {
        safe_strcpy(options->video_source_token, ezxml_txt(token),
                    sizeof(options->video_source_token), 0);
    }

    parse_float_range(options_node, "Brightness", &options->brightness);
    parse_float_range(options_node, "ColorSaturation", &options->color_saturation);
    parse_float_range(options_node, "Contrast", &options->contrast);
    parse_float_range(options_node, "Sharpness", &options->sharpness);
    parse_float_range(options_node, "NoiseReduction", &options->noise_reduction);
    parse_mode_level_options(options_node, "BacklightCompensationOptions",
                             &options->backlight_compensation);
    parse_mode_level_options(options_node, "WideDynamicRangeOptions",
                             &options->wide_dynamic_range);
    parse_mode_level_options(options_node, "ToneCompensationOptions",
                             &options->tone_compensation);
    parse_mode_level_options(options_node, "DefoggingOptions",
                             &options->defogging);

    for (ezxml_t child = options_node->child;
         child && options->ir_cut_filter_mode_count < 4;
         child = child->sibling) {
        const char *local = xml_local_name(child->name);
        if (local && strcmp(local, "IrCutFilterModes") == 0) {
            safe_strcpy(options->ir_cut_filter_modes[options->ir_cut_filter_mode_count],
                        ezxml_txt(child),
                        sizeof(options->ir_cut_filter_modes[options->ir_cut_filter_mode_count]),
                        0);
            options->ir_cut_filter_mode_count++;
        }
    }

    ezxml_free(xml);
    return 0;
}

int onvif_imaging_get_settings(const char *imaging_url,
                               const char *video_source_token,
                               const char *username,
                               const char *password,
                               onvif_imaging_settings_t *settings) {
    if (!imaging_url || !video_source_token || !settings) {
        return -1;
    }

    char token[256];
    if (xml_escape(video_source_token, token, sizeof(token)) != 0) {
        return -1;
    }

    char request_body[512];
    snprintf(request_body, sizeof(request_body),
        "<timg:GetImagingSettings>"
            "<timg:VideoSourceToken>%s</timg:VideoSourceToken>"
        "</timg:GetImagingSettings>",
        token);

    char *response = send_imaging_soap_request(imaging_url,
        "http://www.onvif.org/ver20/imaging/wsdl/GetImagingSettings",
        request_body, username, password);
    if (!response) {
        return -1;
    }

    int rc = parse_settings_response(response, settings);
    free(response);
    return rc;
}

int onvif_imaging_get_options(const char *imaging_url,
                              const char *video_source_token,
                              const char *username,
                              const char *password,
                              onvif_imaging_options_t *options) {
    if (!imaging_url || !video_source_token || !options) {
        return -1;
    }

    char token[256];
    if (xml_escape(video_source_token, token, sizeof(token)) != 0) {
        return -1;
    }

    char request_body[512];
    snprintf(request_body, sizeof(request_body),
        "<timg:GetOptions>"
            "<timg:VideoSourceToken>%s</timg:VideoSourceToken>"
        "</timg:GetOptions>",
        token);

    char *response = send_imaging_soap_request(imaging_url,
        "http://www.onvif.org/ver20/imaging/wsdl/GetOptions",
        request_body, username, password);
    if (!response) {
        return -1;
    }

    int rc = parse_options_response(response, options);
    free(response);
    return rc;
}

static int append_float_setting_xml(char **cursor, size_t *remaining,
                                    const char *name, const onvif_imaging_float_t *setting) {
    if (!setting || !setting->present) {
        return 0;
    }

    int n = snprintf(*cursor, *remaining, "<tt:%s>%.4f</tt:%s>", name, setting->value, name);
    if (n < 0 || (size_t)n >= *remaining) {
        return -1;
    }

    *cursor += n;
    *remaining -= (size_t)n;
    return 0;
}

static int append_mode_level_setting_xml(char **cursor, size_t *remaining,
                                         const char *name,
                                         const onvif_imaging_mode_level_t *setting) {
    if (!setting || !setting->present) {
        return 0;
    }

    int n = snprintf(*cursor, *remaining, "<tt:%s>", name);
    if (n < 0 || (size_t)n >= *remaining) {
        return -1;
    }
    *cursor += n;
    *remaining -= (size_t)n;

    if (setting->mode[0] != '\0') {
        char escaped_mode[64];
        if (xml_escape(setting->mode, escaped_mode, sizeof(escaped_mode)) != 0) {
            return -1;
        }
        n = snprintf(*cursor, *remaining, "<tt:Mode>%s</tt:Mode>", escaped_mode);
        if (n < 0 || (size_t)n >= *remaining) {
            return -1;
        }
        *cursor += n;
        *remaining -= (size_t)n;
    }

    if (setting->has_level) {
        n = snprintf(*cursor, *remaining, "<tt:Level>%.4f</tt:Level>", setting->level);
        if (n < 0 || (size_t)n >= *remaining) {
            return -1;
        }
        *cursor += n;
        *remaining -= (size_t)n;
    }

    n = snprintf(*cursor, *remaining, "</tt:%s>", name);
    if (n < 0 || (size_t)n >= *remaining) {
        return -1;
    }
    *cursor += n;
    *remaining -= (size_t)n;
    return 0;
}

int onvif_imaging_set_settings(const char *imaging_url,
                               const char *video_source_token,
                               const char *username,
                               const char *password,
                               const onvif_imaging_settings_t *settings,
                               bool force_persistence) {
    if (!imaging_url || !video_source_token || !settings) {
        return -1;
    }

    char token[256];
    if (xml_escape(video_source_token, token, sizeof(token)) != 0) {
        return -1;
    }

    char settings_xml[2048];
    char *cursor = settings_xml;
    size_t remaining = sizeof(settings_xml);
    settings_xml[0] = '\0';

    if (append_float_setting_xml(&cursor, &remaining, "Brightness", &settings->brightness) != 0 ||
        append_float_setting_xml(&cursor, &remaining, "ColorSaturation", &settings->color_saturation) != 0 ||
        append_float_setting_xml(&cursor, &remaining, "Contrast", &settings->contrast) != 0 ||
        append_float_setting_xml(&cursor, &remaining, "Sharpness", &settings->sharpness) != 0 ||
        append_float_setting_xml(&cursor, &remaining, "NoiseReduction", &settings->noise_reduction) != 0 ||
        append_mode_level_setting_xml(&cursor, &remaining, "BacklightCompensation",
                                      &settings->backlight_compensation) != 0 ||
        append_mode_level_setting_xml(&cursor, &remaining, "WideDynamicRange",
                                      &settings->wide_dynamic_range) != 0 ||
        append_mode_level_setting_xml(&cursor, &remaining, "ToneCompensation",
                                      &settings->tone_compensation) != 0 ||
        append_mode_level_setting_xml(&cursor, &remaining, "Defogging",
                                      &settings->defogging) != 0) {
        log_error("Failed to build ONVIF Imaging settings XML");
        return -1;
    }

    if (settings->has_ir_cut_filter) {
        char escaped_ir_cut[64];
        if (xml_escape(settings->ir_cut_filter, escaped_ir_cut, sizeof(escaped_ir_cut)) != 0) {
            return -1;
        }
        int n = snprintf(cursor, remaining, "<tt:IrCutFilter>%s</tt:IrCutFilter>", escaped_ir_cut);
        if (n < 0 || (size_t)n >= remaining) {
            return -1;
        }
        cursor += n;
        remaining -= (size_t)n;
    }

    char request_body[3072];
    snprintf(request_body, sizeof(request_body),
        "<timg:SetImagingSettings>"
            "<timg:VideoSourceToken>%s</timg:VideoSourceToken>"
            "<timg:ImagingSettings>%s</timg:ImagingSettings>"
            "<timg:ForcePersistence>%s</timg:ForcePersistence>"
        "</timg:SetImagingSettings>",
        token, settings_xml, force_persistence ? "true" : "false");

    char *response = send_imaging_soap_request(imaging_url,
        "http://www.onvif.org/ver20/imaging/wsdl/SetImagingSettings",
        request_body, username, password);
    if (!response) {
        return -1;
    }

    int rc = strstr(response, "Fault") ? -1 : 0;
    if (rc != 0) {
        onvif_log_soap_fault(response, strlen(response), "SetImagingSettings");
    }

    free(response);
    return rc;
}
