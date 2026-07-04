#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <curl/curl.h>
#include "ezxml.h"

#include "video/onvif_ptz.h"
#include "video/onvif_soap.h"
#include "core/logger.h"
#include "core/url_utils.h"
#include "utils/strings.h"

// Structure to store memory for CURL responses
typedef struct {
    char *memory;
    size_t size;
} MemoryStruct;

// Callback function for CURL to write received data
static size_t write_callback(void *contents, size_t size, size_t nmemb, void *userp) {
    size_t realsize = size * nmemb;
    MemoryStruct *mem = (MemoryStruct *)userp;

    char *ptr = realloc(mem->memory, mem->size + realsize + 1);
    if (ptr == NULL) {
        log_error("Not enough memory (realloc returned NULL)");
        return 0;
    }

    mem->memory = ptr;
    memcpy(&(mem->memory[mem->size]), contents, realsize);
    mem->size += realsize;
    mem->memory[mem->size] = 0;

    return realsize;
}

// Send a SOAP request to the ONVIF PTZ service
static char* send_ptz_soap_request(const char *ptz_url, const char *soap_action, const char *request_body,
                                   const char *username, const char *password) {
    CURL *curl;
    CURLcode res;
    MemoryStruct chunk;
    struct curl_slist *headers = NULL;
    char *soap_envelope = NULL;
    char *response = NULL;
    char *security_header = NULL;
    
    chunk.memory = malloc(1);
    chunk.size = 0;
    
    curl = curl_easy_init();
    if (!curl) {
        log_error("Failed to initialize CURL for PTZ request");
        free(chunk.memory);
        return NULL;
    }
    
    if (username && password && strlen(username) > 0 && strlen(password) > 0) {
        security_header = onvif_create_security_header(username, password);
    } else {
        security_header = strdup("");
    }
    
    soap_envelope = malloc(strlen(request_body) + strlen(security_header) + 2048);
    sprintf(soap_envelope,
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>"
        "<s:Envelope xmlns:s=\"http://www.w3.org/2003/05/soap-envelope\" "
        "xmlns:tds=\"http://www.onvif.org/ver10/device/wsdl\" "
        "xmlns:tptz=\"http://www.onvif.org/ver20/ptz/wsdl\" "
        "xmlns:tt=\"http://www.onvif.org/ver10/schema\">"
        "<s:Header>%s</s:Header>"
        "<s:Body>%s</s:Body>"
        "</s:Envelope>",
        security_header, request_body);
    
    headers = curl_slist_append(headers, "Content-Type: application/soap+xml; charset=utf-8");
    if (soap_action) {
        char soap_action_header[256];
        sprintf(soap_action_header, "SOAPAction: %s", soap_action);
        headers = curl_slist_append(headers, soap_action_header);
    }
    
    curl_easy_setopt(curl, CURLOPT_URL, ptz_url);
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
    curl_easy_setopt(curl, CURLOPT_POSTFIELDS, soap_envelope);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_callback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, (void *)&chunk);
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 10L);
    curl_easy_setopt(curl, CURLOPT_SSL_VERIFYPEER, 0L);
    curl_easy_setopt(curl, CURLOPT_SSL_VERIFYHOST, 0L);
    
    res = curl_easy_perform(curl);
    if (res != CURLE_OK) {
        log_error("PTZ CURL request failed: %s", curl_easy_strerror(res));
    } else {
        // Check HTTP response code
        long http_code = 0;
        curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);

        if (http_code != 200) {
            log_error("PTZ request failed with HTTP code %ld", http_code);
            if (chunk.size > 0) {
                onvif_log_soap_fault(chunk.memory, chunk.size, "PTZ");
            }
        } else if (chunk.size > 0) {
            response = chunk.memory;
            chunk.memory = NULL;  // Transfer ownership; caller will free
        }
    }

    curl_slist_free_all(headers);
    curl_easy_cleanup(curl);
    free(soap_envelope);
    free(security_header);
    free(chunk.memory);

    return response;
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

static bool parse_int_text(const char *text, int *value) {
    if (!text || !value || text[0] == '\0') {
        return false;
    }

    char *end = NULL;
    long parsed = strtol(text, &end, 10);
    if (end == text) {
        return false;
    }

    *value = (int)parsed;
    return true;
}

static bool parse_bool_text(const char *text, bool *value) {
    if (!text || !value) {
        return false;
    }

    if (strcasecmp(text, "true") == 0 || strcmp(text, "1") == 0) {
        *value = true;
        return true;
    }
    if (strcasecmp(text, "false") == 0 || strcmp(text, "0") == 0) {
        *value = false;
        return true;
    }

    return false;
}

static bool parse_range(ezxml_t range, float *min_value, float *max_value) {
    if (!range || !min_value || !max_value) {
        return false;
    }

    ezxml_t min_node = find_child_local(range, "Min");
    ezxml_t max_node = find_child_local(range, "Max");
    return min_node && max_node &&
           parse_float_text(ezxml_txt(min_node), min_value) &&
           parse_float_text(ezxml_txt(max_node), max_value);
}

static bool parse_xy_ranges(ezxml_t parent, float *x_min, float *x_max,
                            float *y_min, float *y_max) {
    if (!parent) {
        return false;
    }

    ezxml_t x_range = find_child_local(parent, "XRange");
    ezxml_t y_range = find_child_local(parent, "YRange");
    return parse_range(x_range, x_min, x_max) &&
           parse_range(y_range, y_min, y_max);
}

static bool parse_x_range(ezxml_t parent, float *x_min, float *x_max) {
    if (!parent) {
        return false;
    }

    ezxml_t x_range = find_child_local(parent, "XRange");
    return parse_range(x_range, x_min, x_max);
}

static bool element_has_non_empty_text(ezxml_t parent, const char *name) {
    ezxml_t child = find_child_local(parent, name);
    const char *text = child ? ezxml_txt(child) : NULL;
    return text && text[0] != '\0';
}

static ezxml_t find_space(ezxml_t spaces, const char *space_name, const char *preferred_uri_fragment) {
    if (!spaces || !space_name) {
        return NULL;
    }

    ezxml_t first_match = NULL;
    for (ezxml_t child = spaces->child; child; child = child->sibling) {
        const char *local = xml_local_name(child->name);
        if (!local || strcmp(local, space_name) != 0) {
            continue;
        }

        if (!first_match) {
            first_match = child;
        }

        if (!preferred_uri_fragment) {
            return child;
        }

        ezxml_t uri = find_child_local(child, "URI");
        const char *uri_text = uri ? ezxml_txt(uri) : "";
        if (uri_text && strstr(uri_text, preferred_uri_fragment)) {
            return child;
        }
    }

    return first_match;
}

static void apply_supported_spaces(ezxml_t spaces, onvif_ptz_capabilities_t *capabilities) {
    if (!spaces || !capabilities) {
        return;
    }

    ezxml_t abs_pan_tilt = find_space(spaces, "AbsolutePanTiltPositionSpace",
                                      "PositionGenericSpace");
    if (abs_pan_tilt) {
        capabilities->has_pan_tilt = true;
        capabilities->has_absolute_move = true;
        parse_xy_ranges(abs_pan_tilt, &capabilities->pan_min, &capabilities->pan_max,
                        &capabilities->tilt_min, &capabilities->tilt_max);
    }

    ezxml_t abs_zoom = find_space(spaces, "AbsoluteZoomPositionSpace",
                                  "PositionGenericSpace");
    if (abs_zoom) {
        capabilities->has_zoom = true;
        capabilities->has_absolute_move = true;
        parse_x_range(abs_zoom, &capabilities->zoom_min, &capabilities->zoom_max);
    }

    if (find_space(spaces, "RelativePanTiltTranslationSpace", NULL)) {
        capabilities->has_pan_tilt = true;
        capabilities->has_relative_move = true;
    }
    if (find_space(spaces, "RelativeZoomTranslationSpace", NULL)) {
        capabilities->has_zoom = true;
        capabilities->has_relative_move = true;
    }
    if (find_space(spaces, "ContinuousPanTiltVelocitySpace", NULL)) {
        capabilities->has_pan_tilt = true;
        capabilities->has_continuous_move = true;
    }
    if (find_space(spaces, "ContinuousZoomVelocitySpace", NULL)) {
        capabilities->has_zoom = true;
        capabilities->has_continuous_move = true;
    }
}

static void set_default_ptz_capabilities(onvif_ptz_capabilities_t *capabilities) {
    memset(capabilities, 0, sizeof(*capabilities));
    capabilities->has_pan_tilt = true;
    capabilities->has_zoom = true;
    capabilities->has_continuous_move = true;
    capabilities->has_absolute_move = true;
    capabilities->has_relative_move = true;
    capabilities->pan_min = -1.0f;
    capabilities->pan_max = 1.0f;
    capabilities->tilt_min = -1.0f;
    capabilities->tilt_max = 1.0f;
    capabilities->zoom_min = 0.0f;
    capabilities->zoom_max = 1.0f;
}

static void reset_ptz_motion_flags(onvif_ptz_capabilities_t *capabilities) {
    capabilities->has_pan_tilt = false;
    capabilities->has_zoom = false;
    capabilities->has_continuous_move = false;
    capabilities->has_absolute_move = false;
    capabilities->has_relative_move = false;
}

static char *parse_service_url(const char *response, const char *namespace_fragment) {
    if (!response || !namespace_fragment) {
        return NULL;
    }

    char *xml_buffer = strdup(response);
    if (!xml_buffer) {
        return NULL;
    }

    ezxml_t xml = ezxml_parse_str(xml_buffer, strlen(xml_buffer));
    char *service_url = NULL;
    if (xml) {
        ezxml_t services_response = find_descendant_local(xml, "GetServicesResponse");
        if (services_response) {
            for (ezxml_t service = services_response->child; service && !service_url; service = service->sibling) {
                const char *local = xml_local_name(service->name);
                if (!local || strcmp(local, "Service") != 0) {
                    continue;
                }

                ezxml_t ns = find_child_local(service, "Namespace");
                ezxml_t xaddr = find_child_local(service, "XAddr");
                const char *ns_text = ns ? ezxml_txt(ns) : "";
                const char *xaddr_text = xaddr ? ezxml_txt(xaddr) : "";
                if (ns_text && xaddr_text && xaddr_text[0] != '\0' &&
                    strstr(ns_text, namespace_fragment)) {
                    service_url = strdup(xaddr_text);
                }
            }
        }
        ezxml_free(xml);
    }

    free(xml_buffer);
    return service_url;
}

int onvif_ptz_get_service_url(const char *device_url, const char *username,
                              const char *password, char *ptz_url, size_t url_size) {
    if (!device_url || !ptz_url || url_size == 0) {
        return -1;
    }

    const char *request_body =
        "<tds:GetServices>"
            "<tds:IncludeCapability>false</tds:IncludeCapability>"
        "</tds:GetServices>";

    char *response = send_ptz_soap_request(device_url,
        "http://www.onvif.org/ver10/device/wsdl/GetServices",
        request_body, username, password);

    if (response) {
        char *discovered_url = parse_service_url(response, "/ptz/wsdl");
        if (discovered_url && discovered_url[0] != '\0') {
            safe_strcpy(ptz_url, discovered_url, url_size, 0);
            char safe_url[512];
            if (url_redact_for_logging(ptz_url, safe_url, sizeof(safe_url)) != 0) {
                safe_strcpy(safe_url, "[invalid-url]", sizeof(safe_url), 0);
            }
            log_info("Discovered ONVIF PTZ service URL: %s", safe_url);
            free(discovered_url);
            free(response);
            return 0;
        }
        free(discovered_url);
        free(response);
    }

    log_warn("Failed to discover ONVIF PTZ service URL, falling back to /onvif/ptz_service");
    return url_build_onvif_service_url(device_url, 0, "/onvif/ptz_service",
                                       ptz_url, url_size);
}

int onvif_ptz_continuous_move(const char *ptz_url, const char *profile_token,
                              const char *username, const char *password,
                              float pan_velocity, float tilt_velocity, float zoom_velocity) {
    if (!ptz_url || !profile_token) {
        log_error("PTZ URL and profile token are required");
        return -1;
    }

    // Only emit axis elements the caller is actually moving.  Including a
    // <tt:PanTilt> or <tt:Zoom> with no space attribute forces the device to
    // look up the corresponding DefaultContinuous*VelocitySpace from its PTZ
    // configuration; PT-only (or zoom-only) nodes don't expose a default for
    // the missing axis and answer with InvalidArgVal — "A space is referenced
    // in an argument which is not supported by the PTZ Node" (issue #420).
    bool has_pan_tilt = pan_velocity != 0.0f || tilt_velocity != 0.0f;
    bool has_zoom = zoom_velocity != 0.0f;

    if (!has_pan_tilt && !has_zoom) {
        // A ContinuousMove with an empty Velocity is rejected by the spec.
        // Treat an all-zero call as a no-op so the UI can keep firing
        // mouse-move events without spamming SOAP faults.
        return 0;
    }

    char velocity_xml[160];
    int velocity_len = 0;
    if (has_pan_tilt) {
        velocity_len += snprintf(velocity_xml + velocity_len,
                                 sizeof(velocity_xml) - velocity_len,
                                 "<tt:PanTilt x=\"%.2f\" y=\"%.2f\"/>",
                                 pan_velocity, tilt_velocity);
    }
    if (has_zoom) {
        velocity_len += snprintf(velocity_xml + velocity_len,
                                 sizeof(velocity_xml) - velocity_len,
                                 "<tt:Zoom x=\"%.2f\"/>", zoom_velocity);
    }

    char request_body[1024];
    snprintf(request_body, sizeof(request_body),
        "<tptz:ContinuousMove>"
            "<tptz:ProfileToken>%s</tptz:ProfileToken>"
            "<tptz:Velocity>%s</tptz:Velocity>"
        "</tptz:ContinuousMove>",
        profile_token, velocity_xml);

    char *response = send_ptz_soap_request(ptz_url,
        "http://www.onvif.org/ver20/ptz/wsdl/ContinuousMove",
        request_body, username, password);

    if (!response) {
        log_error("Failed to send ContinuousMove request");
        return -1;
    }

    // Check for fault in response
    int result = (strstr(response, "Fault") != NULL) ? -1 : 0;
    if (result != 0) {
        log_error("ContinuousMove returned fault: %s", response);
    } else {
        log_info("PTZ ContinuousMove: pan=%.2f, tilt=%.2f, zoom=%.2f",
                 pan_velocity, tilt_velocity, zoom_velocity);
    }

    free(response);
    return result;
}

int onvif_ptz_stop(const char *ptz_url, const char *profile_token,
                   const char *username, const char *password,
                   bool stop_pan_tilt, bool stop_zoom) {
    if (!ptz_url || !profile_token) {
        log_error("PTZ URL and profile token are required");
        return -1;
    }

    char request_body[512];
    snprintf(request_body, sizeof(request_body),
        "<tptz:Stop>"
            "<tptz:ProfileToken>%s</tptz:ProfileToken>"
            "<tptz:PanTilt>%s</tptz:PanTilt>"
            "<tptz:Zoom>%s</tptz:Zoom>"
        "</tptz:Stop>",
        profile_token,
        stop_pan_tilt ? "true" : "false",
        stop_zoom ? "true" : "false");

    char *response = send_ptz_soap_request(ptz_url,
        "http://www.onvif.org/ver20/ptz/wsdl/Stop",
        request_body, username, password);

    if (!response) {
        log_error("Failed to send Stop request");
        return -1;
    }

    int result = (strstr(response, "Fault") != NULL) ? -1 : 0;
    if (result == 0) {
        log_info("PTZ Stop: pan_tilt=%s, zoom=%s",
                 stop_pan_tilt ? "true" : "false",
                 stop_zoom ? "true" : "false");
    }

    free(response);
    return result;
}

int onvif_ptz_absolute_move(const char *ptz_url, const char *profile_token,
                            const char *username, const char *password,
                            float pan, float tilt, float zoom) {
    return onvif_ptz_absolute_move_axes(ptz_url, profile_token, username, password,
                                        true, pan, tilt, true, zoom);
}

int onvif_ptz_absolute_move_axes(const char *ptz_url, const char *profile_token,
                                 const char *username, const char *password,
                                 bool has_pan_tilt, float pan, float tilt,
                                 bool has_zoom, float zoom) {
    if (!ptz_url || !profile_token) {
        log_error("PTZ URL and profile token are required");
        return -1;
    }

    if (!has_pan_tilt && !has_zoom) {
        return 0;
    }

    char token[128];
    if (xml_escape(profile_token, token, sizeof(token)) != 0) {
        return -1;
    }

    char position_xml[256];
    int position_len = 0;
    position_xml[0] = '\0';
    if (has_pan_tilt) {
        position_len += snprintf(position_xml + position_len,
                                 sizeof(position_xml) - (size_t)position_len,
                                 "<tt:PanTilt x=\"%.4f\" y=\"%.4f\"/>",
                                 pan, tilt);
    }
    if (has_zoom) {
        position_len += snprintf(position_xml + position_len,
                                 sizeof(position_xml) - (size_t)position_len,
                                 "<tt:Zoom x=\"%.4f\"/>", zoom);
    }
    if (position_len < 0 || (size_t)position_len >= sizeof(position_xml)) {
        return -1;
    }

    char request_body[1024];
    snprintf(request_body, sizeof(request_body),
        "<tptz:AbsoluteMove>"
            "<tptz:ProfileToken>%s</tptz:ProfileToken>"
            "<tptz:Position>%s</tptz:Position>"
        "</tptz:AbsoluteMove>",
        token, position_xml);

    char *response = send_ptz_soap_request(ptz_url,
        "http://www.onvif.org/ver20/ptz/wsdl/AbsoluteMove",
        request_body, username, password);

    if (!response) {
        log_error("Failed to send AbsoluteMove request");
        return -1;
    }

    int result = (strstr(response, "Fault") != NULL) ? -1 : 0;
    if (result == 0) {
        log_info("PTZ AbsoluteMove: pan_tilt=%s pan=%.4f tilt=%.4f zoom=%s %.4f",
                 has_pan_tilt ? "true" : "false", pan, tilt,
                 has_zoom ? "true" : "false", zoom);
    }

    free(response);
    return result;
}

int onvif_ptz_relative_move(const char *ptz_url, const char *profile_token,
                            const char *username, const char *password,
                            float pan_delta, float tilt_delta, float zoom_delta) {
    return onvif_ptz_relative_move_axes(ptz_url, profile_token, username, password,
                                        true, pan_delta, tilt_delta, true, zoom_delta);
}

int onvif_ptz_relative_move_axes(const char *ptz_url, const char *profile_token,
                                 const char *username, const char *password,
                                 bool has_pan_tilt, float pan_delta, float tilt_delta,
                                 bool has_zoom, float zoom_delta) {
    if (!ptz_url || !profile_token) {
        log_error("PTZ URL and profile token are required");
        return -1;
    }

    if (!has_pan_tilt && !has_zoom) {
        return 0;
    }

    char token[128];
    if (xml_escape(profile_token, token, sizeof(token)) != 0) {
        return -1;
    }

    char translation_xml[256];
    int translation_len = 0;
    translation_xml[0] = '\0';
    if (has_pan_tilt) {
        translation_len += snprintf(translation_xml + translation_len,
                                    sizeof(translation_xml) - (size_t)translation_len,
                                    "<tt:PanTilt x=\"%.4f\" y=\"%.4f\"/>",
                                    pan_delta, tilt_delta);
    }
    if (has_zoom) {
        translation_len += snprintf(translation_xml + translation_len,
                                    sizeof(translation_xml) - (size_t)translation_len,
                                    "<tt:Zoom x=\"%.4f\"/>", zoom_delta);
    }
    if (translation_len < 0 || (size_t)translation_len >= sizeof(translation_xml)) {
        return -1;
    }

    char request_body[1024];
    snprintf(request_body, sizeof(request_body),
        "<tptz:RelativeMove>"
            "<tptz:ProfileToken>%s</tptz:ProfileToken>"
            "<tptz:Translation>%s</tptz:Translation>"
        "</tptz:RelativeMove>",
        token, translation_xml);

    char *response = send_ptz_soap_request(ptz_url,
        "http://www.onvif.org/ver20/ptz/wsdl/RelativeMove",
        request_body, username, password);

    if (!response) {
        log_error("Failed to send RelativeMove request");
        return -1;
    }

    int result = (strstr(response, "Fault") != NULL) ? -1 : 0;
    if (result == 0) {
        log_info("PTZ RelativeMove: pan_tilt=%s pan=%.4f tilt=%.4f zoom=%s %.4f",
                 has_pan_tilt ? "true" : "false", pan_delta, tilt_delta,
                 has_zoom ? "true" : "false", zoom_delta);
    }

    free(response);
    return result;
}

int onvif_ptz_goto_home(const char *ptz_url, const char *profile_token,
                        const char *username, const char *password) {
    if (!ptz_url || !profile_token) {
        log_error("PTZ URL and profile token are required");
        return -1;
    }

    char request_body[256];
    snprintf(request_body, sizeof(request_body),
        "<tptz:GotoHomePosition>"
            "<tptz:ProfileToken>%s</tptz:ProfileToken>"
        "</tptz:GotoHomePosition>",
        profile_token);

    char *response = send_ptz_soap_request(ptz_url,
        "http://www.onvif.org/ver20/ptz/wsdl/GotoHomePosition",
        request_body, username, password);

    if (!response) {
        log_error("Failed to send GotoHomePosition request");
        return -1;
    }

    int result = (strstr(response, "Fault") != NULL) ? -1 : 0;
    if (result == 0) {
        log_info("PTZ GotoHomePosition");
    }

    free(response);
    return result;
}

int onvif_ptz_set_home(const char *ptz_url, const char *profile_token,
                       const char *username, const char *password) {
    if (!ptz_url || !profile_token) {
        log_error("PTZ URL and profile token are required");
        return -1;
    }

    char request_body[256];
    snprintf(request_body, sizeof(request_body),
        "<tptz:SetHomePosition>"
            "<tptz:ProfileToken>%s</tptz:ProfileToken>"
        "</tptz:SetHomePosition>",
        profile_token);

    char *response = send_ptz_soap_request(ptz_url,
        "http://www.onvif.org/ver20/ptz/wsdl/SetHomePosition",
        request_body, username, password);

    if (!response) {
        log_error("Failed to send SetHomePosition request");
        return -1;
    }

    int result = (strstr(response, "Fault") != NULL) ? -1 : 0;
    if (result == 0) {
        log_info("PTZ SetHomePosition");
    }

    free(response);
    return result;
}

int onvif_ptz_get_presets(const char *ptz_url, const char *profile_token,
                          const char *username, const char *password,
                          onvif_ptz_preset_t *presets, int max_presets) {
    if (!ptz_url || !profile_token || !presets || max_presets <= 0) {
        log_error("Invalid parameters for GetPresets");
        return -1;
    }

    char request_body[256];
    snprintf(request_body, sizeof(request_body),
        "<tptz:GetPresets>"
            "<tptz:ProfileToken>%s</tptz:ProfileToken>"
        "</tptz:GetPresets>",
        profile_token);

    char *response = send_ptz_soap_request(ptz_url,
        "http://www.onvif.org/ver20/ptz/wsdl/GetPresets",
        request_body, username, password);

    if (!response) {
        log_error("Failed to send GetPresets request");
        return -1;
    }

    // Parse presets from response
    int count = 0;
    ezxml_t xml = ezxml_parse_str(response, strlen(response));
    if (xml) {
        ezxml_t get_presets_response = find_descendant_local(xml, "GetPresetsResponse");
        if (get_presets_response) {
            for (ezxml_t preset = get_presets_response->child;
                 preset && count < max_presets;
                 preset = preset->sibling) {
                const char *local = xml_local_name(preset->name);
                if (!local || strcmp(local, "Preset") != 0) {
                    continue;
                }

                const char *token = ezxml_attr(preset, "token");
                ezxml_t name_elem = find_child_local(preset, "Name");
                ezxml_t position = find_child_local(preset, "PTZPosition");
                ezxml_t pan_tilt = position ? find_child_local(position, "PanTilt") : NULL;
                ezxml_t zoom = position ? find_child_local(position, "Zoom") : NULL;

                memset(&presets[count], 0, sizeof(presets[count]));
                if (token) {
                    safe_strcpy(presets[count].token, token, sizeof(presets[count].token), 0);
                }
                if (name_elem && name_elem->txt) {
                    safe_strcpy(presets[count].name, name_elem->txt, sizeof(presets[count].name), 0);
                }
                if (pan_tilt) {
                    parse_float_text(ezxml_attr(pan_tilt, "x"), &presets[count].pan);
                    parse_float_text(ezxml_attr(pan_tilt, "y"), &presets[count].tilt);
                }
                if (zoom) {
                    parse_float_text(ezxml_attr(zoom, "x"), &presets[count].zoom);
                }
                count++;
            }
        }
        ezxml_free(xml);
    }

    free(response);
    log_info("PTZ GetPresets: found %d presets", count);
    return count;
}

int onvif_ptz_goto_preset(const char *ptz_url, const char *profile_token,
                          const char *username, const char *password,
                          const char *preset_token) {
    if (!ptz_url || !profile_token || !preset_token) {
        log_error("PTZ URL, profile token, and preset token are required");
        return -1;
    }

    char request_body[512];
    snprintf(request_body, sizeof(request_body),
        "<tptz:GotoPreset>"
            "<tptz:ProfileToken>%s</tptz:ProfileToken>"
            "<tptz:PresetToken>%s</tptz:PresetToken>"
        "</tptz:GotoPreset>",
        profile_token, preset_token);

    char *response = send_ptz_soap_request(ptz_url,
        "http://www.onvif.org/ver20/ptz/wsdl/GotoPreset",
        request_body, username, password);

    if (!response) {
        log_error("Failed to send GotoPreset request");
        return -1;
    }

    int result = (strstr(response, "Fault") != NULL) ? -1 : 0;
    if (result == 0) {
        log_info("PTZ GotoPreset: %s", preset_token);
    }

    free(response);
    return result;
}

int onvif_ptz_set_preset(const char *ptz_url, const char *profile_token,
                         const char *username, const char *password,
                         const char *preset_name, char *preset_token, size_t token_size) {
    if (!ptz_url || !profile_token) {
        log_error("PTZ URL and profile token are required");
        return -1;
    }

    char request_body[512];
    if (preset_name && strlen(preset_name) > 0) {
        snprintf(request_body, sizeof(request_body),
            "<tptz:SetPreset>"
                "<tptz:ProfileToken>%s</tptz:ProfileToken>"
                "<tptz:PresetName>%s</tptz:PresetName>"
            "</tptz:SetPreset>",
            profile_token, preset_name);
    } else {
        snprintf(request_body, sizeof(request_body),
            "<tptz:SetPreset>"
                "<tptz:ProfileToken>%s</tptz:ProfileToken>"
            "</tptz:SetPreset>",
            profile_token);
    }

    char *response = send_ptz_soap_request(ptz_url,
        "http://www.onvif.org/ver20/ptz/wsdl/SetPreset",
        request_body, username, password);

    if (!response) {
        log_error("Failed to send SetPreset request");
        return -1;
    }

    int result = (strstr(response, "Fault") != NULL) ? -1 : 0;

    // Extract preset token from response if successful
    if (result == 0 && preset_token && token_size > 0) {
        ezxml_t xml = ezxml_parse_str(response, strlen(response));
        if (xml) {
            ezxml_t set_preset_response = find_descendant_local(xml, "SetPresetResponse");
            ezxml_t token_elem = set_preset_response
                ? find_child_local(set_preset_response, "PresetToken")
                : NULL;
            if (token_elem && token_elem->txt) {
                safe_strcpy(preset_token, token_elem->txt, token_size, 0);
            }
            ezxml_free(xml);
        }
        log_info("PTZ SetPreset: %s -> %s", preset_name ? preset_name : "(unnamed)", preset_token);
    }

    free(response);
    return result;
}

static bool parse_configurations_response(const char *response,
                                          onvif_ptz_capabilities_t *capabilities,
                                          char *config_token,
                                          size_t config_token_size) {
    if (!response || !capabilities) {
        return false;
    }

    char *xml_buffer = strdup(response);
    if (!xml_buffer) {
        return false;
    }

    ezxml_t xml = ezxml_parse_str(xml_buffer, strlen(xml_buffer));
    bool parsed = false;
    if (xml) {
        ezxml_t config = find_descendant_local(xml, "PTZConfiguration");
        if (config) {
            parsed = true;
            capabilities->queried = true;
            capabilities->has_pan_tilt = false;
            capabilities->has_zoom = false;
            capabilities->has_continuous_move = false;
            capabilities->has_absolute_move = false;
            capabilities->has_relative_move = false;

            const char *token = ezxml_attr(config, "token");
            if (token && config_token && config_token_size > 0) {
                safe_strcpy(config_token, token, config_token_size, 0);
            }

            bool abs_pan_tilt =
                element_has_non_empty_text(config, "DefaultAbsolutePantTiltPositionSpace") ||
                element_has_non_empty_text(config, "DefaultAbsolutePanTiltPositionSpace");
            bool abs_zoom = element_has_non_empty_text(config, "DefaultAbsoluteZoomPositionSpace");
            bool rel_pan_tilt = element_has_non_empty_text(config, "DefaultRelativePanTiltTranslationSpace");
            bool rel_zoom = element_has_non_empty_text(config, "DefaultRelativeZoomTranslationSpace");
            bool cont_pan_tilt = element_has_non_empty_text(config, "DefaultContinuousPanTiltVelocitySpace");
            bool cont_zoom = element_has_non_empty_text(config, "DefaultContinuousZoomVelocitySpace");

            ezxml_t pan_tilt_limits = find_descendant_local(config, "PanTiltLimits");
            ezxml_t zoom_limits = find_descendant_local(config, "ZoomLimits");

            capabilities->has_pan_tilt =
                abs_pan_tilt || rel_pan_tilt || cont_pan_tilt || (pan_tilt_limits != NULL);
            capabilities->has_zoom = abs_zoom || rel_zoom || cont_zoom || (zoom_limits != NULL);
            capabilities->has_absolute_move =
                abs_pan_tilt || abs_zoom || (pan_tilt_limits != NULL) || (zoom_limits != NULL);
            capabilities->has_relative_move = rel_pan_tilt || rel_zoom;
            capabilities->has_continuous_move = cont_pan_tilt || cont_zoom;

            if (pan_tilt_limits) {
                ezxml_t range = find_descendant_local(pan_tilt_limits, "Range");
                parse_xy_ranges(range, &capabilities->pan_min, &capabilities->pan_max,
                                &capabilities->tilt_min, &capabilities->tilt_max);
            }
            if (zoom_limits) {
                ezxml_t range = find_descendant_local(zoom_limits, "Range");
                parse_x_range(range, &capabilities->zoom_min, &capabilities->zoom_max);
            }
        }

        ezxml_free(xml);
    }

    free(xml_buffer);
    return parsed;
}

static bool parse_configuration_options_response(const char *response,
                                                 onvif_ptz_capabilities_t *capabilities) {
    if (!response || !capabilities) {
        return false;
    }

    char *xml_buffer = strdup(response);
    if (!xml_buffer) {
        return false;
    }

    ezxml_t xml = ezxml_parse_str(xml_buffer, strlen(xml_buffer));
    bool parsed = false;
    if (xml) {
        ezxml_t options = find_descendant_local(xml, "PTZConfigurationOptions");
        ezxml_t spaces = options ? find_descendant_local(options, "Spaces") : NULL;
        if (spaces) {
            bool had_prior_query = capabilities->queried;
            capabilities->queried = true;
            if (!had_prior_query) {
                reset_ptz_motion_flags(capabilities);
            }
            apply_supported_spaces(spaces, capabilities);
            parsed = true;
        }
        ezxml_free(xml);
    }

    free(xml_buffer);
    return parsed;
}

static bool parse_nodes_response(const char *response,
                                 onvif_ptz_capabilities_t *capabilities) {
    if (!response || !capabilities) {
        return false;
    }

    char *xml_buffer = strdup(response);
    if (!xml_buffer) {
        return false;
    }

    ezxml_t xml = ezxml_parse_str(xml_buffer, strlen(xml_buffer));
    bool parsed = false;
    if (xml) {
        ezxml_t node = find_descendant_local(xml, "PTZNode");
        if (node) {
            parsed = true;
            bool had_prior_query = capabilities->queried;
            capabilities->queried = true;
            if (!had_prior_query) {
                reset_ptz_motion_flags(capabilities);
            }

            ezxml_t spaces = find_descendant_local(node, "SupportedPTZSpaces");
            if (spaces) {
                apply_supported_spaces(spaces, capabilities);
            }

            ezxml_t max_presets = find_descendant_local(node, "MaximumNumberOfPresets");
            if (max_presets) {
                int value = 0;
                if (parse_int_text(ezxml_txt(max_presets), &value)) {
                    capabilities->max_presets = value;
                    capabilities->has_presets = value > 0;
                }
            }

            ezxml_t home_supported = find_descendant_local(node, "HomeSupported");
            if (home_supported) {
                bool value = false;
                if (parse_bool_text(ezxml_txt(home_supported), &value)) {
                    capabilities->has_home_position = value;
                }
            } else {
                const char *fixed_home = ezxml_attr(node, "FixedHomePosition");
                if (fixed_home) {
                    capabilities->has_home_position = true;
                }
            }
        }

        ezxml_free(xml);
    }

    free(xml_buffer);
    return parsed;
}

int onvif_ptz_get_capabilities(const char *ptz_url, const char *profile_token,
                               const char *username, const char *password,
                               onvif_ptz_capabilities_t *capabilities) {
    if (!ptz_url || !capabilities) {
        return -1;
    }

    (void)profile_token;

    set_default_ptz_capabilities(capabilities);

    char config_token[128] = {0};
    char *response = send_ptz_soap_request(ptz_url,
        "http://www.onvif.org/ver20/ptz/wsdl/GetConfigurations",
        "<tptz:GetConfigurations/>", username, password);
    if (response) {
        parse_configurations_response(response, capabilities,
                                      config_token, sizeof(config_token));
        free(response);
    }

    if (config_token[0] != '\0') {
        char escaped_token[256];
        if (xml_escape(config_token, escaped_token, sizeof(escaped_token)) == 0) {
            char request_body[512];
            snprintf(request_body, sizeof(request_body),
                "<tptz:GetConfigurationOptions>"
                    "<tptz:ConfigurationToken>%s</tptz:ConfigurationToken>"
                "</tptz:GetConfigurationOptions>",
                escaped_token);

            response = send_ptz_soap_request(ptz_url,
                "http://www.onvif.org/ver20/ptz/wsdl/GetConfigurationOptions",
                request_body, username, password);
            if (response) {
                parse_configuration_options_response(response, capabilities);
                free(response);
            }
        }
    }

    response = send_ptz_soap_request(ptz_url,
        "http://www.onvif.org/ver20/ptz/wsdl/GetNodes",
        "<tptz:GetNodes/>", username, password);
    if (response) {
        parse_nodes_response(response, capabilities);
        free(response);
    }

    return 0;
}
