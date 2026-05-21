#include <stdio.h>
#include <stdlib.h>
#include <string.h>
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

int onvif_ptz_get_service_url(const char *device_url, const char *username,
                              const char *password, char *ptz_url, size_t url_size) {
    /* device_url is already an http/https ONVIF URL (e.g. from
     * url_build_onvif_device_service_url).  Passing onvif_port=0 preserves
     * the existing port; the scheme is already http/https so it is kept as-is.
     * username/password are accepted for API compatibility but unused here —
     * callers that need credential-aware service discovery should query the
     * device directly. */
    (void)username;
    (void)password;
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
    if (!ptz_url || !profile_token) {
        log_error("PTZ URL and profile token are required");
        return -1;
    }

    char request_body[1024];
    snprintf(request_body, sizeof(request_body),
        "<tptz:AbsoluteMove>"
            "<tptz:ProfileToken>%s</tptz:ProfileToken>"
            "<tptz:Position>"
                "<tt:PanTilt x=\"%.4f\" y=\"%.4f\"/>"
                "<tt:Zoom x=\"%.4f\"/>"
            "</tptz:Position>"
        "</tptz:AbsoluteMove>",
        profile_token, pan, tilt, zoom);

    char *response = send_ptz_soap_request(ptz_url,
        "http://www.onvif.org/ver20/ptz/wsdl/AbsoluteMove",
        request_body, username, password);

    if (!response) {
        log_error("Failed to send AbsoluteMove request");
        return -1;
    }

    int result = (strstr(response, "Fault") != NULL) ? -1 : 0;
    if (result == 0) {
        log_info("PTZ AbsoluteMove: pan=%.4f, tilt=%.4f, zoom=%.4f", pan, tilt, zoom);
    }

    free(response);
    return result;
}

int onvif_ptz_relative_move(const char *ptz_url, const char *profile_token,
                            const char *username, const char *password,
                            float pan_delta, float tilt_delta, float zoom_delta) {
    if (!ptz_url || !profile_token) {
        log_error("PTZ URL and profile token are required");
        return -1;
    }

    char request_body[1024];
    snprintf(request_body, sizeof(request_body),
        "<tptz:RelativeMove>"
            "<tptz:ProfileToken>%s</tptz:ProfileToken>"
            "<tptz:Translation>"
                "<tt:PanTilt x=\"%.4f\" y=\"%.4f\"/>"
                "<tt:Zoom x=\"%.4f\"/>"
            "</tptz:Translation>"
        "</tptz:RelativeMove>",
        profile_token, pan_delta, tilt_delta, zoom_delta);

    char *response = send_ptz_soap_request(ptz_url,
        "http://www.onvif.org/ver20/ptz/wsdl/RelativeMove",
        request_body, username, password);

    if (!response) {
        log_error("Failed to send RelativeMove request");
        return -1;
    }

    int result = (strstr(response, "Fault") != NULL) ? -1 : 0;
    if (result == 0) {
        log_info("PTZ RelativeMove: pan=%.4f, tilt=%.4f, zoom=%.4f", pan_delta, tilt_delta, zoom_delta);
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
        // Find Preset elements in the response
        ezxml_t body = ezxml_child(xml, "s:Body");
        if (!body) body = ezxml_child(xml, "Body");
        if (!body) body = ezxml_child(xml, "SOAP-ENV:Body");

        if (body) {
            ezxml_t get_presets_response = ezxml_child(body, "tptz:GetPresetsResponse");
            if (!get_presets_response) get_presets_response = ezxml_child(body, "GetPresetsResponse");

            if (get_presets_response) {
                for (ezxml_t preset = ezxml_child(get_presets_response, "tptz:Preset");
                     preset && count < max_presets;
                     preset = preset->next) {
                    const char *token = ezxml_attr(preset, "token");
                    ezxml_t name_elem = ezxml_child(preset, "tt:Name");
                    if (!name_elem) name_elem = ezxml_child(preset, "Name");

                    if (token) {
                        safe_strcpy(presets[count].token, token, sizeof(presets[count].token), 0);
                    }
                    if (name_elem && name_elem->txt) {
                        safe_strcpy(presets[count].name, name_elem->txt, sizeof(presets[count].name), 0);
                    }
                    count++;
                }
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
            ezxml_t body = ezxml_child(xml, "s:Body");
            if (!body) body = ezxml_child(xml, "Body");
            if (body) {
                ezxml_t set_preset_response = ezxml_child(body, "tptz:SetPresetResponse");
                if (!set_preset_response) set_preset_response = ezxml_child(body, "SetPresetResponse");
                if (set_preset_response) {
                    ezxml_t token_elem = ezxml_child(set_preset_response, "tptz:PresetToken");
                    if (!token_elem) token_elem = ezxml_child(set_preset_response, "PresetToken");
                    if (token_elem && token_elem->txt) {
                        safe_strcpy(preset_token, token_elem->txt, token_size, 0);
                    }
                }
            }
            ezxml_free(xml);
        }
        log_info("PTZ SetPreset: %s -> %s", preset_name ? preset_name : "(unnamed)", preset_token);
    }

    free(response);
    return result;
}

int onvif_ptz_get_capabilities(const char *ptz_url, const char *profile_token,
                               const char *username, const char *password,
                               onvif_ptz_capabilities_t *capabilities) {
    if (!ptz_url || !capabilities) {
        return -1;
    }

    // Initialize with defaults
    memset(capabilities, 0, sizeof(onvif_ptz_capabilities_t));
    capabilities->has_continuous_move = true;  // Assume basic support
    capabilities->has_absolute_move = true;
    capabilities->has_relative_move = true;
    capabilities->pan_min = -1.0f;
    capabilities->pan_max = 1.0f;
    capabilities->tilt_min = -1.0f;
    capabilities->tilt_max = 1.0f;
    capabilities->zoom_min = 0.0f;
    capabilities->zoom_max = 1.0f;

    // TODO: Query actual capabilities from device
    // For now, return defaults
    return 0;
}
