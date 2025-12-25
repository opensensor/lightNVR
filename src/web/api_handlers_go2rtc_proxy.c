#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <curl/curl.h>

#include "web/api_handlers.h"
#include "web/mongoose_adapter.h"
#include "web/mongoose_server_auth.h"
#include "web/http_server.h"
#include "core/logger.h"
#include "core/config.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "mongoose.h"
#include "video/go2rtc/go2rtc_integration.h"
#include "video/go2rtc/go2rtc_stream.h"
#include "web/mongoose_server_multithreading.h"
#include "web/api_handlers_go2rtc_proxy.h"

// Buffer size for URLs
#define URL_BUFFER_SIZE 2048

// Structure to hold response data from curl
struct curl_response {
    char *data;
    size_t size;
};

// Callback function for curl to write response data
static size_t write_callback(void *contents, size_t size, size_t nmemb, void *userp) {
    size_t realsize = size * nmemb;
    struct curl_response *resp = (struct curl_response *)userp;

    char *ptr = realloc(resp->data, resp->size + realsize + 1);
    if (!ptr) {
        log_error("Failed to allocate memory for curl response");
        return 0;
    }

    resp->data = ptr;
    memcpy(&(resp->data[resp->size]), contents, realsize);
    resp->size += realsize;
    resp->data[resp->size] = 0;

    return realsize;
}

/**
 * @brief Handler for POST /api/webrtc
 *
 * This handler proxies WebRTC offer requests to the go2rtc API.
 * It processes the request directly in the current thread.
 */
void mg_handle_go2rtc_webrtc_offer(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Processing WebRTC offer request");

    // Process the request directly
    mg_handle_go2rtc_webrtc_offer_worker(c, hm);

    log_info("Completed WebRTC offer request");
}

/**
 * @brief Worker function for POST /api/webrtc
 *
 * This function is called by the multithreading system to handle WebRTC offer requests.
 */
void mg_handle_go2rtc_webrtc_offer_worker(struct mg_connection *c, struct mg_http_message *hm) {
    // Variables for resources that need cleanup
    struct mg_str *src_param = NULL;
    char *param_value = NULL;
    char *offer = NULL;
    CURL *curl = NULL;
    struct curl_slist *headers = NULL;
    struct curl_response response = {0};

    // Check authentication
    http_server_t *server = (http_server_t *)c->fn_data;
    if (server && server->config.auth_enabled) {
        // Check if the user is authenticated
        if (mongoose_server_basic_auth_check(hm, server) != 0) {
            log_error("Authentication failed for go2rtc WebRTC offer request");
            mg_send_json_error(c, 401, "Unauthorized");
            goto cleanup;
        }
    }
    log_info("Handling POST /api/webrtc request");

    // Log request details
    log_info("Request method: %.*s", (int)hm->method.len, hm->method.buf);
    log_info("Request URI: %.*s", (int)hm->uri.len, hm->uri.buf);
    log_info("Request query: %.*s", (int)hm->query.len, hm->query.buf);
    log_info("Request body length: %zu", hm->body.len);

    // Extract stream name from query parameter
    char stream_name[MAX_STREAM_NAME] = {0};
    int result = mg_http_get_var(&hm->query, "src", stream_name, sizeof(stream_name));

    if (result <= 0) {
        log_error("Missing 'src' parameter in WebRTC offer request");
        mg_send_json_error(c, 400, "Missing 'src' parameter");
        goto cleanup;
    }

    log_info("Extracted src parameter: %s", stream_name);

    // URL decode the stream name
    char decoded_name[MAX_STREAM_NAME];
    mg_url_decode(stream_name, strlen(stream_name), decoded_name, sizeof(decoded_name), 0);

    log_info("Looking for stream with name: '%s'", decoded_name);

    // Trim leading and trailing whitespace from decoded name
    char trimmed_name[MAX_STREAM_NAME];
    strncpy(trimmed_name, decoded_name, sizeof(trimmed_name) - 1);
    trimmed_name[sizeof(trimmed_name) - 1] = '\0';

    // Trim leading whitespace
    char *start = trimmed_name;
    while (*start && isspace(*start)) start++;

    // Trim trailing whitespace
    char *end = start + strlen(start) - 1;
    while (end > start && isspace(*end)) *end-- = '\0';

    // Move the trimmed string back to the beginning if needed
    if (start != trimmed_name) {
        memmove(trimmed_name, start, strlen(start) + 1);
    }

    log_info("Trimmed stream name: '%s'", trimmed_name);

    // Check if stream exists using the trimmed name
    stream_handle_t stream = get_stream_by_name(trimmed_name);
    if (!stream) {
        log_error("Stream not found: '%s'", trimmed_name);
        mg_send_json_error(c, 404, "Stream not found");
        goto cleanup;
    }

    log_info("Stream found: '%s'", trimmed_name);

    // Get the request body (WebRTC offer)
    log_info("WebRTC offer length: %zu", hm->body.len);

    // Create a null-terminated copy of the request body
    offer = malloc(hm->body.len + 1);
    if (!offer) {
        log_error("Failed to allocate memory for WebRTC offer");
        mg_send_json_error(c, 500, "Internal server error");
        goto cleanup;
    }

    memcpy(offer, hm->body.buf, hm->body.len);
    offer[hm->body.len] = '\0';

    // Log the first 100 characters of the offer for debugging
    char offer_preview[101] = {0};
    strncpy(offer_preview, offer, 100);
    log_info("WebRTC offer preview: %s", offer_preview);

    // Proxy the request to go2rtc API
    curl = curl_easy_init();
    if (!curl) {
        log_error("Failed to initialize curl");
        mg_send_json_error(c, 500, "Failed to initialize curl");
        goto cleanup;
    }

    // Construct the URL for the go2rtc API
    char url[URL_BUFFER_SIZE];
    char encoded_name[MAX_STREAM_NAME * 3]; // Triple size to account for URL encoding expansion

    // URL encode the stream name for the go2rtc API
    mg_url_encode(trimmed_name, strlen(trimmed_name), encoded_name, sizeof(encoded_name));
    snprintf(url, sizeof(url), "http://localhost:1984/api/webrtc?src=%s", encoded_name);

    // Set curl options
    if (curl_easy_setopt(curl, CURLOPT_URL, url) != CURLE_OK) {
        log_error("Failed to set CURLOPT_URL");
        mg_send_json_error(c, 500, "Failed to set curl options");
        goto cleanup;
    }

    // Set a connection timeout to prevent hanging on network issues
    if (curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT, 5L) != CURLE_OK) {
        log_error("Failed to set CURLOPT_CONNECTTIMEOUT");
        mg_send_json_error(c, 500, "Failed to set curl options");
        goto cleanup;
    }

    // Set POST fields directly from the request body
    if (curl_easy_setopt(curl, CURLOPT_POSTFIELDS, offer) != CURLE_OK) {
        log_error("Failed to set CURLOPT_POSTFIELDS");
        mg_send_json_error(c, 500, "Failed to set curl options");
        goto cleanup;
    }

    // Set POST field size
    if (curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, (long)hm->body.len) != CURLE_OK) {
        log_error("Failed to set CURLOPT_POSTFIELDSIZE");
        mg_send_json_error(c, 500, "Failed to set curl options");
        goto cleanup;
    }

    if (curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_callback) != CURLE_OK) {
        log_error("Failed to set CURLOPT_WRITEFUNCTION");
        mg_send_json_error(c, 500, "Failed to set curl options");
        goto cleanup;
    }

    if (curl_easy_setopt(curl, CURLOPT_WRITEDATA, &response) != CURLE_OK) {
        log_error("Failed to set CURLOPT_WRITEDATA");
        mg_send_json_error(c, 500, "Failed to set curl options");
        goto cleanup;
    }

    if (curl_easy_setopt(curl, CURLOPT_TIMEOUT, 10L) != CURLE_OK) { // 10 second timeout
        log_error("Failed to set CURLOPT_TIMEOUT");
        mg_send_json_error(c, 500, "Failed to set curl options");
        goto cleanup;
    }

    // Set content type header only, let curl handle Content-Length
    headers = curl_slist_append(headers, "Content-Type: application/json");

    if (!headers) {
        log_error("Failed to create headers list");
        mg_send_json_error(c, 500, "Failed to set curl options");
        goto cleanup;
    }

    if (curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers) != CURLE_OK) {
        log_error("Failed to set CURLOPT_HTTPHEADER");
        mg_send_json_error(c, 500, "Failed to set curl options");
        goto cleanup;
    }

    // Perform the request
    CURLcode res = curl_easy_perform(curl);

    // Check for errors
    if (res != CURLE_OK) {
        log_error("curl_easy_perform() failed: %s", curl_easy_strerror(res));
        mg_send_json_error(c, 500, "Failed to proxy request to go2rtc API");
        goto cleanup;
    }

    // Get HTTP response code
    long http_code = 0;
    curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);

    // Send the response back to the client
    if (http_code == 200 && response.data) {
        // Log the response for debugging
        log_info("Response from go2rtc: %s", response.data);

        // Set CORS headers and send the response
        mg_http_reply(c, 200,
                     "Content-Type: application/json\r\n"
                     "Access-Control-Allow-Origin: *\r\n"
                     "Access-Control-Allow-Methods: POST, OPTIONS\r\n"
                     "Access-Control-Allow-Headers: Content-Type, Authorization, Origin, X-Requested-With, Accept\r\n"
                     "Access-Control-Allow-Credentials: true\r\n"
                     "Connection: close\r\n",
                     "%s", response.data ? response.data : "{}");
    } else {
        log_error("go2rtc API returned error: %ld", http_code);
        mg_send_json_error(c, (int)http_code, response.data ? response.data : "Error from go2rtc API");
    }

    log_info("Successfully handled WebRTC offer request for stream: %s", trimmed_name);

cleanup:
    // Free all allocated resources
    if (src_param) {
        free(src_param);
    }
    if (param_value) {
        free(param_value);
    }
    if (offer) {
        free(offer);
    }
    if (headers) {
        curl_slist_free_all(headers);
    }
    if (curl) {
        curl_easy_cleanup(curl);
    }
    if (response.data) {
        free(response.data);
    }
}

/**
 * @brief Handler for POST /api/webrtc/ice
 *
 * This handler proxies WebRTC ICE candidate requests to the go2rtc API.
 * It processes the request directly in the current thread.
 */
void mg_handle_go2rtc_webrtc_ice(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Processing WebRTC ICE request");

    // Process the request directly
    mg_handle_go2rtc_webrtc_ice_worker(c, hm);

    log_info("Completed WebRTC ICE request");
}

/**
 * @brief Worker function for POST /api/webrtc/ice
 *
 * This function is called by the multithreading system to handle WebRTC ICE candidate requests.
 */
void mg_handle_go2rtc_webrtc_ice_worker(struct mg_connection *c, struct mg_http_message *hm) {
    // Variables for resources that need cleanup
    struct mg_str *src_param = NULL;
    char *param_value = NULL;
    char *ice_candidate = NULL;
    CURL *curl = NULL;
    struct curl_slist *headers = NULL;
    struct curl_response response = {0};

    // Check authentication
    http_server_t *server = (http_server_t *)c->fn_data;
    if (server && server->config.auth_enabled) {
        // Check if the user is authenticated
        if (mongoose_server_basic_auth_check(hm, server) != 0) {
            log_error("Authentication failed for go2rtc WebRTC ICE request");
            mg_send_json_error(c, 401, "Unauthorized");
            goto cleanup;
        }
    }
    log_info("Handling POST /api/webrtc/ice request");

    // Log request details
    log_info("Request method: %.*s", (int)hm->method.len, hm->method.buf);
    log_info("Request URI: %.*s", (int)hm->uri.len, hm->uri.buf);
    log_info("Request query: %.*s", (int)hm->query.len, hm->query.buf);
    log_info("Request body length: %zu", hm->body.len);

    // Extract stream name from query parameter
    struct mg_str src_param_str = mg_str("src");

    // Extract query parameters
    for (int i = 0; i < MG_MAX_HTTP_HEADERS; i++) {
        if (hm->query.len == 0) break;

        // Find the src parameter in the query string
        const char *query_str = hm->query.buf;
        size_t query_len = hm->query.len;

        // Simple parsing to find src=value in the query string
        const char *src_pos = strstr(hm->query.buf, "src=");
        if (src_pos) {
            // Found src parameter
            const char *value_start = src_pos + src_param_str.len + 1; // +1 for '='
            const char *value_end = strchr(value_start, '&');
            if (!value_end) {
                value_end = query_str + query_len;
            }

            // Allocate memory for the parameter value
            size_t value_len = value_end - value_start;
            param_value = malloc(value_len + 1);
            if (!param_value) {
                log_error("Failed to allocate memory for query parameter");
                mg_send_json_error(c, 500, "Internal server error");
                goto cleanup;
            }

            // Copy the parameter value
            memcpy(param_value, value_start, value_len);
            param_value[value_len] = '\0';

            // Create a mg_str for the parameter value
            src_param = malloc(sizeof(struct mg_str));
            if (!src_param) {
                log_error("Failed to allocate memory for query parameter");
                mg_send_json_error(c, 500, "Internal server error");
                goto cleanup;
            }

            src_param->buf = param_value;
            src_param->len = value_len;
            break;
        }
    }

    if (!src_param || src_param->len == 0) {
        log_error("Missing 'src' query parameter");
        mg_send_json_error(c, 400, "Missing 'src' query parameter");
        goto cleanup;
    }

    // Extract the stream name
    char stream_name[MAX_STREAM_NAME];
    if (src_param->len >= sizeof(stream_name)) {
        log_error("Stream name too long");
        mg_send_json_error(c, 400, "Stream name too long");
        goto cleanup;
    }

    memcpy(stream_name, src_param->buf, src_param->len);
    stream_name[src_param->len] = '\0';

    // URL decode the stream name
    char decoded_name[MAX_STREAM_NAME];
    mg_url_decode(stream_name, strlen(stream_name), decoded_name, sizeof(decoded_name), 0);

    log_info("WebRTC ICE request for stream: %s", decoded_name);

    // Get the request body (ICE candidate)
    log_info("ICE candidate length: %zu", hm->body.len);

    // Create a null-terminated copy of the request body
    ice_candidate = malloc(hm->body.len + 1);
    if (!ice_candidate) {
        log_error("Failed to allocate memory for ICE candidate");
        mg_send_json_error(c, 500, "Internal server error");
        goto cleanup;
    }

    memcpy(ice_candidate, hm->body.buf, hm->body.len);
    ice_candidate[hm->body.len] = '\0';

    // Log the first 100 characters of the ICE candidate for debugging
    char ice_preview[101] = {0};
    strncpy(ice_preview, ice_candidate, 100);
    log_info("ICE candidate preview: %s", ice_preview);

    // Proxy the request to go2rtc API
    curl = curl_easy_init();
    if (!curl) {
        log_error("Failed to initialize curl");
        mg_send_json_error(c, 500, "Failed to initialize curl");
        goto cleanup;
    }

    // Construct the URL for the go2rtc API
    char url[URL_BUFFER_SIZE];
    char encoded_name[MAX_STREAM_NAME * 3]; // Triple size to account for URL encoding expansion

    // URL encode the stream name for the go2rtc API
    mg_url_encode(decoded_name, strlen(decoded_name), encoded_name, sizeof(encoded_name));
    snprintf(url, sizeof(url), "http://localhost:1984/api/webrtc/ice?src=%s", encoded_name);

    // Set curl options
    if (curl_easy_setopt(curl, CURLOPT_URL, url) != CURLE_OK) {
        log_error("Failed to set CURLOPT_URL");
        mg_send_json_error(c, 500, "Failed to set curl options");
        goto cleanup;
    }

    // Set POST fields directly from the request body
    if (curl_easy_setopt(curl, CURLOPT_POSTFIELDS, ice_candidate) != CURLE_OK) {
        log_error("Failed to set CURLOPT_POSTFIELDS");
        mg_send_json_error(c, 500, "Failed to set curl options");
        goto cleanup;
    }

    // Set POST field size
    if (curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, (long)hm->body.len) != CURLE_OK) {
        log_error("Failed to set CURLOPT_POSTFIELDSIZE");
        mg_send_json_error(c, 500, "Failed to set curl options");
        goto cleanup;
    }

    if (curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_callback) != CURLE_OK) {
        log_error("Failed to set CURLOPT_WRITEFUNCTION");
        mg_send_json_error(c, 500, "Failed to set curl options");
        goto cleanup;
    }

    if (curl_easy_setopt(curl, CURLOPT_WRITEDATA, &response) != CURLE_OK) {
        log_error("Failed to set CURLOPT_WRITEDATA");
        mg_send_json_error(c, 500, "Failed to set curl options");
        goto cleanup;
    }

    if (curl_easy_setopt(curl, CURLOPT_TIMEOUT, 5L) != CURLE_OK) { // 5 second timeout
        log_error("Failed to set CURLOPT_TIMEOUT");
        mg_send_json_error(c, 500, "Failed to set curl options");
        goto cleanup;
    }

    // Set content type header only, let curl handle Content-Length
    headers = curl_slist_append(headers, "Content-Type: application/json");

    if (!headers) {
        log_error("Failed to create headers list");
        mg_send_json_error(c, 500, "Failed to set curl options");
        goto cleanup;
    }

    if (curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers) != CURLE_OK) {
        log_error("Failed to set CURLOPT_HTTPHEADER");
        mg_send_json_error(c, 500, "Failed to set curl options");
        goto cleanup;
    }

    // Perform the request
    CURLcode res = curl_easy_perform(curl);

    // Check for errors
    if (res != CURLE_OK) {
        log_error("curl_easy_perform() failed: %s", curl_easy_strerror(res));
        mg_send_json_error(c, 500, "Failed to proxy request to go2rtc API");
        goto cleanup;
    }

    // Get HTTP response code
    long http_code = 0;
    curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);

    // Send the response back to the client
    if (http_code == 200) {
        // Log the response for debugging
        log_info("ICE response from go2rtc: %s", response.data ? response.data : "(empty)");

        // Set CORS headers and send the response
        mg_http_reply(c, 200,
                     "Content-Type: application/json\r\n"
                     "Access-Control-Allow-Origin: *\r\n"
                     "Access-Control-Allow-Methods: POST, OPTIONS\r\n"
                     "Access-Control-Allow-Headers: Content-Type, Authorization, Origin, X-Requested-With, Accept\r\n"
                     "Access-Control-Allow-Credentials: true\r\n"
                     "Connection: close\r\n",
                     "%s", response.data ? response.data : "{}");
    } else {
        log_error("go2rtc API returned error: %ld", http_code);
        mg_send_json_error(c, (int)http_code, response.data ? response.data : "Error from go2rtc API");
    }

    log_info("Successfully handled WebRTC ICE request for stream: %s", decoded_name);

cleanup:
    // Free all allocated resources
    if (src_param) {
        free(src_param);
    }
    if (param_value) {
        free(param_value);
    }
    if (ice_candidate) {
        free(ice_candidate);
    }
    if (headers) {
        curl_slist_free_all(headers);
    }
    if (curl) {
        curl_easy_cleanup(curl);
    }
    if (response.data) {
        free(response.data);
    }
}

/**
 * @brief Handler for OPTIONS /api/webrtc
 *
 * This handler responds to CORS preflight requests for the WebRTC API.
 */
void mg_handle_go2rtc_webrtc_options(struct mg_connection *c, struct mg_http_message *hm) {

    log_info("Handling OPTIONS /api/webrtc request");

    // Set CORS headers
    mg_printf(c, "HTTP/1.1 200 OK\r\n");
    mg_printf(c, "Access-Control-Allow-Origin: *\r\n");
    mg_printf(c, "Access-Control-Allow-Methods: POST, OPTIONS\r\n");
    mg_printf(c, "Access-Control-Allow-Headers: Content-Type, Authorization, Origin, X-Requested-With, Accept\r\n");
    mg_printf(c, "Access-Control-Allow-Credentials: true\r\n");
    mg_printf(c, "Connection: close\r\n");
    mg_printf(c, "Content-Length: 0\r\n");
    mg_printf(c, "\r\n");

    log_info("Successfully handled OPTIONS request for WebRTC API");
}

/**
 * @brief Handler for OPTIONS /api/webrtc/ice
 *
 * This handler responds to CORS preflight requests for the WebRTC ICE API.
 */
void mg_handle_go2rtc_webrtc_ice_options(struct mg_connection *c, struct mg_http_message *hm) {

    log_info("Handling OPTIONS /api/webrtc/ice request");

    // Set CORS headers
    mg_printf(c, "HTTP/1.1 200 OK\r\n");
    mg_printf(c, "Access-Control-Allow-Origin: *\r\n");
    mg_printf(c, "Access-Control-Allow-Methods: POST, OPTIONS\r\n");
    mg_printf(c, "Access-Control-Allow-Headers: Content-Type, Authorization, Origin, X-Requested-With, Accept\r\n");
    mg_printf(c, "Access-Control-Allow-Credentials: true\r\n");
    mg_printf(c, "Connection: close\r\n");
    mg_printf(c, "Content-Length: 0\r\n");
    mg_printf(c, "\r\n");

    log_info("Successfully handled OPTIONS request for WebRTC ICE API");
}


/**
 * @brief Handler for GET /api/webrtc/config
 *
 * Returns WebRTC configuration (including ICE servers) from the go2rtc config.
 * Since go2rtc doesn't expose this via API, we read it from the global config.
 */
void mg_handle_go2rtc_webrtc_config(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Processing WebRTC config request");

    // Check authentication
    http_server_t *server = (http_server_t *)c->fn_data;
    if (server && server->config.auth_enabled) {
        if (mongoose_server_basic_auth_check(hm, server) != 0) {
            log_error("Authentication failed for go2rtc WebRTC config request");
            mg_send_json_error(c, 401, "Unauthorized");
            return;
        }
    }

    // Get the global config to read ICE server settings
    extern config_t g_config;

    // Build the ICE servers JSON response
    // Format: {"iceServers": [{"urls": ["stun:server:port"]}, ...]}
    char json_response[2048];
    int offset = 0;

    offset += snprintf(json_response + offset, sizeof(json_response) - offset,
                      "{\"iceServers\":[");

    // Add STUN servers if enabled - include multiple for redundancy
    if (g_config.go2rtc_stun_enabled && g_config.go2rtc_stun_server[0] != '\0') {
        offset += snprintf(json_response + offset, sizeof(json_response) - offset,
                          "{\"urls\":[\"stun:%s\",\"stun:stun1.l.google.com:19302\",\"stun:stun2.l.google.com:19302\",\"stun:stun3.l.google.com:19302\",\"stun:stun4.l.google.com:19302\"]}",
                          g_config.go2rtc_stun_server);
    }

    // Add custom ICE servers if specified
    if (g_config.go2rtc_ice_servers[0] != '\0') {
        // Parse comma-separated ICE servers
        char ice_servers_copy[512];
        strncpy(ice_servers_copy, g_config.go2rtc_ice_servers, sizeof(ice_servers_copy) - 1);
        ice_servers_copy[sizeof(ice_servers_copy) - 1] = '\0';

        char *token = strtok(ice_servers_copy, ",");
        while (token != NULL) {
            // Trim whitespace
            while (*token == ' ') token++;
            char *end = token + strlen(token) - 1;
            while (end > token && *end == ' ') end--;
            *(end + 1) = '\0';

            // Add comma if not first entry
            if (offset > strlen("{\"iceServers\":[")) {
                offset += snprintf(json_response + offset, sizeof(json_response) - offset, ",");
            }

            offset += snprintf(json_response + offset, sizeof(json_response) - offset,
                              "{\"urls\":[\"%s\"]}", token);
            token = strtok(NULL, ",");
        }
    }

    offset += snprintf(json_response + offset, sizeof(json_response) - offset, "]}");

    log_info("WebRTC config response: %s", json_response);

    // Send the response
    mg_http_reply(c, 200,
                  "Content-Type: application/json\r\n"
                  "Access-Control-Allow-Origin: *\r\n"
                  "Access-Control-Allow-Methods: GET, OPTIONS\r\n"
                  "Access-Control-Allow-Headers: Content-Type, Authorization, Origin, X-Requested-With, Accept\r\n"
                  "Access-Control-Allow-Credentials: true\r\n"
                  "Connection: close\r\n",
                  "%s", json_response);
}
