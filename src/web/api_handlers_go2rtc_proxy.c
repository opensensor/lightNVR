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
#include "web/thread_pool.h"
#include "web/api_thread_pool.h"

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
 * @brief Direct handler for POST /api/webrtc
 * 
 * This handler proxies WebRTC offer requests to the go2rtc API.
 */
void mg_handle_go2rtc_webrtc_offer(struct mg_connection *c, struct mg_http_message *hm) {
    // Acquire the API thread pool to ensure proper thread management
    bool release_needed = true;
    thread_pool_t *pool = api_thread_pool_acquire(api_thread_pool_get_size(), 10);
    if (!pool) {
        log_error("Failed to acquire API thread pool");
        mg_send_json_error(c, 500, "Internal server error");
        return;
    }
    
    // Check authentication
    http_server_t *server = (http_server_t *)c->fn_data;
    if (server && server->config.auth_enabled) {
        // Check if the user is authenticated
        if (mongoose_server_basic_auth_check(hm, server) != 0) {
            log_error("Authentication failed for go2rtc WebRTC offer request");
            mg_send_json_error(c, 401, "Unauthorized");
            api_thread_pool_release();
            return;
        }
    }
    log_info("Handling POST /api/webrtc request");
    
    // Log request details
    log_info("Request method: %.*s", (int)hm->method.len, hm->method.buf);
    log_info("Request URI: %.*s", (int)hm->uri.len, hm->uri.buf);
    log_info("Request query: %.*s", (int)hm->query.len, hm->query.buf);
    log_info("Request body length: %zu", hm->body.len);
    
    // Extract stream name from query parameter
    struct mg_str src_param_str = mg_str("src");
    struct mg_str *src_param = NULL;
    
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
            char *param_value = malloc(value_len + 1);
            if (!param_value) {
                log_error("Failed to allocate memory for query parameter");
                mg_send_json_error(c, 500, "Internal server error");
                return;
            }
            
            // Copy the parameter value
            memcpy(param_value, value_start, value_len);
            param_value[value_len] = '\0';
            
            // Create a mg_str for the parameter value
            struct mg_str *param = malloc(sizeof(struct mg_str));
            if (!param) {
                free(param_value);
                log_error("Failed to allocate memory for query parameter");
                mg_send_json_error(c, 500, "Internal server error");
                return;
            }
            
            param->buf = param_value;
            param->len = value_len;
            src_param = param;
            break;
        }
    }
    if (!src_param || src_param->len == 0) {
        log_error("Missing 'src' query parameter");
        mg_send_json_error(c, 400, "Missing 'src' query parameter");
        return;
    }
    
    // Extract the stream name
    char stream_name[MAX_STREAM_NAME];
    if (src_param->len >= sizeof(stream_name)) {
        log_error("Stream name too long");
        mg_send_json_error(c, 400, "Stream name too long");
        return;
    }
    
    memcpy(stream_name, src_param->buf, src_param->len);
    stream_name[src_param->len] = '\0';
    
    // URL decode the stream name
    char decoded_name[MAX_STREAM_NAME];
    mg_url_decode(stream_name, strlen(stream_name), decoded_name, sizeof(decoded_name), 0);
    
    log_info("WebRTC offer request for stream: %s", decoded_name);
    
    // Check if stream exists
    stream_handle_t stream = get_stream_by_name(decoded_name);
    if (!stream) {
        log_error("Stream not found: %s", decoded_name);
        mg_send_json_error(c, 404, "Stream not found");
        return;
    }
    
    // Get the request body (WebRTC offer)
    log_info("WebRTC offer length: %zu", hm->body.len);
    
    // Create a null-terminated copy of the request body
    char *offer = malloc(hm->body.len + 1);
    if (!offer) {
        log_error("Failed to allocate memory for WebRTC offer");
        mg_send_json_error(c, 500, "Internal server error");
        return;
    }
    
    memcpy(offer, hm->body.buf, hm->body.len);
    offer[hm->body.len] = '\0';
    
    // Log the first 100 characters of the offer for debugging
    char offer_preview[101] = {0};
    strncpy(offer_preview, offer, 100);
    log_info("WebRTC offer preview: %s", offer_preview);
    
    // Proxy the request to go2rtc API
    CURL *curl;
    CURLcode res;
    struct curl_response response = {0};
    
    // Initialize curl
    curl = curl_easy_init();
    if (!curl) {
        log_error("Failed to initialize curl");
        mg_send_json_error(c, 500, "Failed to initialize curl");
        return;
    }
    
    // Construct the URL for the go2rtc API
    char url[URL_BUFFER_SIZE];
    snprintf(url, sizeof(url), "http://localhost:1984/api/webrtc?src=%s", decoded_name);
    
    // Set curl options
    if (curl_easy_setopt(curl, CURLOPT_URL, url) != CURLE_OK) {
        log_error("Failed to set CURLOPT_URL");
        mg_send_json_error(c, 500, "Failed to set curl options");
        curl_easy_cleanup(curl);
        return;
    }
    
    // Set a connection timeout to prevent hanging on network issues
    if (curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT, 5L) != CURLE_OK) {
        log_error("Failed to set CURLOPT_CONNECTTIMEOUT");
        mg_send_json_error(c, 500, "Failed to set curl options");
        curl_easy_cleanup(curl);
        return;
    }
    
    // Set basic authentication if enabled in the main application
    if (g_config.web_auth_enabled) {
        if (curl_easy_setopt(curl, CURLOPT_HTTPAUTH, CURLAUTH_BASIC) != CURLE_OK) {
            log_error("Failed to set CURLOPT_HTTPAUTH");
            mg_send_json_error(c, 500, "Failed to set curl options");
            curl_easy_cleanup(curl);
            return;
        }
        
        if (curl_easy_setopt(curl, CURLOPT_USERNAME, g_config.web_username) != CURLE_OK) {
            log_error("Failed to set CURLOPT_USERNAME");
            mg_send_json_error(c, 500, "Failed to set curl options");
            curl_easy_cleanup(curl);
            return;
        }
        
        if (curl_easy_setopt(curl, CURLOPT_PASSWORD, g_config.web_password) != CURLE_OK) {
            log_error("Failed to set CURLOPT_PASSWORD");
            mg_send_json_error(c, 500, "Failed to set curl options");
            curl_easy_cleanup(curl);
            return;
        }
        
        log_info("Using authentication for go2rtc API request: username=%s", g_config.web_username);
    } else {
        log_info("Authentication disabled for go2rtc API request");
    }
    
    // Set POST fields directly from the request body
    if (curl_easy_setopt(curl, CURLOPT_POSTFIELDS, offer) != CURLE_OK) {
        log_error("Failed to set CURLOPT_POSTFIELDS");
        mg_send_json_error(c, 500, "Failed to set curl options");
        free(offer);
        curl_easy_cleanup(curl);
        return;
    }
    
    // Set POST field size
    if (curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, (long)hm->body.len) != CURLE_OK) {
        log_error("Failed to set CURLOPT_POSTFIELDSIZE");
        mg_send_json_error(c, 500, "Failed to set curl options");
        free(offer);
        curl_easy_cleanup(curl);
        return;
    }
    
    if (curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_callback) != CURLE_OK) {
        log_error("Failed to set CURLOPT_WRITEFUNCTION");
        mg_send_json_error(c, 500, "Failed to set curl options");
        curl_easy_cleanup(curl);
        return;
    }
    
    if (curl_easy_setopt(curl, CURLOPT_WRITEDATA, &response) != CURLE_OK) {
        log_error("Failed to set CURLOPT_WRITEDATA");
        mg_send_json_error(c, 500, "Failed to set curl options");
        curl_easy_cleanup(curl);
        return;
    }
    
    if (curl_easy_setopt(curl, CURLOPT_TIMEOUT, 10L) != CURLE_OK) { // 10 second timeout
        log_error("Failed to set CURLOPT_TIMEOUT");
        mg_send_json_error(c, 500, "Failed to set curl options");
        curl_easy_cleanup(curl);
        return;
    }
    
    // Set content type header only, let curl handle Content-Length
    struct curl_slist *headers = NULL;
    headers = curl_slist_append(headers, "Content-Type: application/json");
    
    if (!headers) {
        log_error("Failed to create headers list");
        mg_send_json_error(c, 500, "Failed to set curl options");
        curl_easy_cleanup(curl);
        return;
    }
    
    if (curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers) != CURLE_OK) {
        log_error("Failed to set CURLOPT_HTTPHEADER");
        mg_send_json_error(c, 500, "Failed to set curl options");
        curl_slist_free_all(headers);
        curl_easy_cleanup(curl);
        return;
    }
    
    // Perform the request
    res = curl_easy_perform(curl);
    
    // Check for errors
    if (res != CURLE_OK) {
        log_error("curl_easy_perform() failed: %s", curl_easy_strerror(res));
        mg_send_json_error(c, 500, "Failed to proxy request to go2rtc API");
        curl_slist_free_all(headers);
        curl_easy_cleanup(curl);
        if (response.data) free(response.data);
        return;
    }
    
    // Get HTTP response code
    long http_code = 0;
    curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
    
    // Clean up curl
    curl_slist_free_all(headers);
    curl_easy_cleanup(curl);
    
    // Send the response back to the client
    if (http_code == 200 && response.data) {
        // Log the response for debugging
        log_info("Response from go2rtc: %s", response.data);
        
        // Calculate the exact content length
        size_t content_length = response.size;
        
        // Set CORS headers
        mg_printf(c, "HTTP/1.1 200 OK\r\n");
        mg_printf(c, "Content-Type: application/json\r\n");
        mg_printf(c, "Access-Control-Allow-Origin: *\r\n");
        mg_printf(c, "Access-Control-Allow-Methods: POST, OPTIONS\r\n");
        mg_printf(c, "Access-Control-Allow-Headers: Content-Type, Authorization, Origin, X-Requested-With, Accept\r\n");
        mg_printf(c, "Access-Control-Allow-Credentials: true\r\n");
        
        // Ensure response.data is not NULL and response.size is valid
        if (response.data && content_length > 0) {
            // Use mg_http_printf_chunk for chunked encoding to avoid Content-Length issues
            mg_printf(c, "Transfer-Encoding: chunked\r\n\r\n");
            mg_http_printf_chunk(c, "%s", response.data);
            mg_http_printf_chunk(c, ""); // Empty chunk to end the response
        } else {
            // Send empty JSON object if no response data
            mg_printf(c, "Content-Length: 2\r\n\r\n{}");
        }
    } else {
        log_error("go2rtc API returned error: %ld", http_code);
        mg_send_json_error(c, (int)http_code, response.data ? response.data : "Error from go2rtc API");
    }
    
    // Free response data
    if (response.data) free(response.data);
    
    log_info("Successfully handled WebRTC offer request for stream: %s", decoded_name);
    
    // Release the thread pool when done
    if (release_needed) {
        api_thread_pool_release();
    }
}

/**
 * @brief Direct handler for POST /api/webrtc/ice
 * 
 * This handler proxies WebRTC ICE candidate requests to the go2rtc API.
 */
void mg_handle_go2rtc_webrtc_ice(struct mg_connection *c, struct mg_http_message *hm) {
    // Acquire the API thread pool to ensure proper thread management
    bool release_needed = true;
    thread_pool_t *pool = api_thread_pool_acquire(api_thread_pool_get_size(), 10);
    if (!pool) {
        log_error("Failed to acquire API thread pool");
        mg_send_json_error(c, 500, "Internal server error");
        return;
    }
    
    // Check authentication
    http_server_t *server = (http_server_t *)c->fn_data;
    if (server && server->config.auth_enabled) {
        // Check if the user is authenticated
        if (mongoose_server_basic_auth_check(hm, server) != 0) {
            log_error("Authentication failed for go2rtc WebRTC ICE request");
            mg_send_json_error(c, 401, "Unauthorized");
            api_thread_pool_release();
            return;
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
    struct mg_str *src_param = NULL;
    
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
            char *param_value = malloc(value_len + 1);
            if (!param_value) {
                log_error("Failed to allocate memory for query parameter");
                mg_send_json_error(c, 500, "Internal server error");
                return;
            }
            
            // Copy the parameter value
            memcpy(param_value, value_start, value_len);
            param_value[value_len] = '\0';
            
            // Create a mg_str for the parameter value
            struct mg_str *param = malloc(sizeof(struct mg_str));
            if (!param) {
                free(param_value);
                log_error("Failed to allocate memory for query parameter");
                mg_send_json_error(c, 500, "Internal server error");
                return;
            }
            
            param->buf = param_value;
            param->len = value_len;
            src_param = param;
            break;
        }
    }
    if (!src_param || src_param->len == 0) {
        log_error("Missing 'src' query parameter");
        mg_send_json_error(c, 400, "Missing 'src' query parameter");
        return;
    }
    
    // Extract the stream name
    char stream_name[MAX_STREAM_NAME];
    if (src_param->len >= sizeof(stream_name)) {
        log_error("Stream name too long");
        mg_send_json_error(c, 400, "Stream name too long");
        return;
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
    char *ice_candidate = malloc(hm->body.len + 1);
    if (!ice_candidate) {
        log_error("Failed to allocate memory for ICE candidate");
        mg_send_json_error(c, 500, "Internal server error");
        return;
    }
    
    memcpy(ice_candidate, hm->body.buf, hm->body.len);
    ice_candidate[hm->body.len] = '\0';
    
    // Log the first 100 characters of the ICE candidate for debugging
    char ice_preview[101] = {0};
    strncpy(ice_preview, ice_candidate, 100);
    log_info("ICE candidate preview: %s", ice_preview);
    
    // Proxy the request to go2rtc API
    CURL *curl;
    CURLcode res;
    struct curl_response response = {0};
    
    // Initialize curl
    curl = curl_easy_init();
    if (!curl) {
        log_error("Failed to initialize curl");
        mg_send_json_error(c, 500, "Failed to initialize curl");
        return;
    }
    
    // Construct the URL for the go2rtc API
    char url[URL_BUFFER_SIZE];
    snprintf(url, sizeof(url), "http://localhost:1984/api/webrtc/ice?src=%s", decoded_name);
    
    // Set curl options
    if (curl_easy_setopt(curl, CURLOPT_URL, url) != CURLE_OK) {
        log_error("Failed to set CURLOPT_URL");
        mg_send_json_error(c, 500, "Failed to set curl options");
        curl_easy_cleanup(curl);
        return;
    }
    
    // Set basic authentication if enabled in the main application
    if (g_config.web_auth_enabled) {
        if (curl_easy_setopt(curl, CURLOPT_HTTPAUTH, CURLAUTH_BASIC) != CURLE_OK) {
            log_error("Failed to set CURLOPT_HTTPAUTH");
            mg_send_json_error(c, 500, "Failed to set curl options");
            curl_easy_cleanup(curl);
            return;
        }
        
        if (curl_easy_setopt(curl, CURLOPT_USERNAME, g_config.web_username) != CURLE_OK) {
            log_error("Failed to set CURLOPT_USERNAME");
            mg_send_json_error(c, 500, "Failed to set curl options");
            curl_easy_cleanup(curl);
            return;
        }
        
        if (curl_easy_setopt(curl, CURLOPT_PASSWORD, g_config.web_password) != CURLE_OK) {
            log_error("Failed to set CURLOPT_PASSWORD");
            mg_send_json_error(c, 500, "Failed to set curl options");
            curl_easy_cleanup(curl);
            return;
        }
        
        log_info("Using authentication for go2rtc ICE API request: username=%s", g_config.web_username);
    } else {
        log_info("Authentication disabled for go2rtc ICE API request");
    }
    
    // Set POST fields directly from the request body
    if (curl_easy_setopt(curl, CURLOPT_POSTFIELDS, ice_candidate) != CURLE_OK) {
        log_error("Failed to set CURLOPT_POSTFIELDS");
        mg_send_json_error(c, 500, "Failed to set curl options");
        free(ice_candidate);
        curl_easy_cleanup(curl);
        return;
    }
    
    // Set POST field size
    if (curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, (long)hm->body.len) != CURLE_OK) {
        log_error("Failed to set CURLOPT_POSTFIELDSIZE");
        mg_send_json_error(c, 500, "Failed to set curl options");
        free(ice_candidate);
        curl_easy_cleanup(curl);
        return;
    }
    
    if (curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_callback) != CURLE_OK) {
        log_error("Failed to set CURLOPT_WRITEFUNCTION");
        mg_send_json_error(c, 500, "Failed to set curl options");
        curl_easy_cleanup(curl);
        return;
    }
    
    if (curl_easy_setopt(curl, CURLOPT_WRITEDATA, &response) != CURLE_OK) {
        log_error("Failed to set CURLOPT_WRITEDATA");
        mg_send_json_error(c, 500, "Failed to set curl options");
        curl_easy_cleanup(curl);
        return;
    }
    
    if (curl_easy_setopt(curl, CURLOPT_TIMEOUT, 5L) != CURLE_OK) { // 5 second timeout
        log_error("Failed to set CURLOPT_TIMEOUT");
        mg_send_json_error(c, 500, "Failed to set curl options");
        curl_easy_cleanup(curl);
        return;
    }
    
    // Set content type header only, let curl handle Content-Length
    struct curl_slist *headers = NULL;
    headers = curl_slist_append(headers, "Content-Type: application/json");
    
    if (!headers) {
        log_error("Failed to create headers list");
        mg_send_json_error(c, 500, "Failed to set curl options");
        curl_easy_cleanup(curl);
        return;
    }
    
    if (curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers) != CURLE_OK) {
        log_error("Failed to set CURLOPT_HTTPHEADER");
        mg_send_json_error(c, 500, "Failed to set curl options");
        curl_slist_free_all(headers);
        curl_easy_cleanup(curl);
        return;
    }
    
    // Perform the request
    res = curl_easy_perform(curl);
    
    // Check for errors
    if (res != CURLE_OK) {
        log_error("curl_easy_perform() failed: %s", curl_easy_strerror(res));
        mg_send_json_error(c, 500, "Failed to proxy request to go2rtc API");
        curl_slist_free_all(headers);
        curl_easy_cleanup(curl);
        if (response.data) free(response.data);
        return;
    }
    
    // Get HTTP response code
    long http_code = 0;
    curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
    
    // Clean up curl
    curl_slist_free_all(headers);
    curl_easy_cleanup(curl);
    
    // Send the response back to the client
    if (http_code == 200) {
        // Log the response for debugging
        log_info("ICE response from go2rtc: %s", response.data ? response.data : "(empty)");
        
        // Calculate the exact content length
        size_t content_length = response.size;
        
        // Set CORS headers
        mg_printf(c, "HTTP/1.1 200 OK\r\n");
        mg_printf(c, "Content-Type: application/json\r\n");
        mg_printf(c, "Access-Control-Allow-Origin: *\r\n");
        mg_printf(c, "Access-Control-Allow-Methods: POST, OPTIONS\r\n");
        mg_printf(c, "Access-Control-Allow-Headers: Content-Type, Authorization, Origin, X-Requested-With, Accept\r\n");
        mg_printf(c, "Access-Control-Allow-Credentials: true\r\n");
        
        // Ensure response.data is not NULL and response.size is valid
        if (response.data && content_length > 0) {
            // Use mg_http_printf_chunk for chunked encoding to avoid Content-Length issues
            mg_printf(c, "Transfer-Encoding: chunked\r\n\r\n");
            mg_http_printf_chunk(c, "%s", response.data);
            mg_http_printf_chunk(c, ""); // Empty chunk to end the response
        } else {
            // Send empty JSON object if no response data
            mg_printf(c, "Content-Length: 2\r\n\r\n{}");
        }
    } else {
        log_error("go2rtc API returned error: %ld", http_code);
        mg_send_json_error(c, (int)http_code, response.data ? response.data : "Error from go2rtc API");
    }
    
    // Free response data
    if (response.data) free(response.data);
    
    log_info("Successfully handled WebRTC ICE request for stream: %s", decoded_name);
    
    // Release the thread pool when done
    if (release_needed) {
        api_thread_pool_release();
    }
}

/**
 * @brief Direct handler for OPTIONS /api/webrtc
 * 
 * This handler responds to CORS preflight requests for the WebRTC API.
 */
void mg_handle_go2rtc_webrtc_options(struct mg_connection *c, struct mg_http_message *hm) {
    // Acquire the API thread pool to ensure proper thread management
    bool release_needed = true;
    thread_pool_t *pool = api_thread_pool_acquire(api_thread_pool_get_size(), 10);
    if (!pool) {
        log_error("Failed to acquire API thread pool");
        mg_send_json_error(c, 500, "Internal server error");
        return;
    }
    log_info("Handling OPTIONS /api/webrtc request");
    
    // Set CORS headers
    mg_printf(c, "HTTP/1.1 200 OK\r\n");
    mg_printf(c, "Access-Control-Allow-Origin: *\r\n");
    mg_printf(c, "Access-Control-Allow-Methods: POST, OPTIONS\r\n");
    mg_printf(c, "Access-Control-Allow-Headers: Content-Type, Authorization, Origin, X-Requested-With, Accept\r\n");
    mg_printf(c, "Access-Control-Allow-Credentials: true\r\n");
    mg_printf(c, "Content-Length: 0\r\n\r\n");
    
    log_info("Successfully handled OPTIONS request for WebRTC API");
    
    // Release the thread pool when done
    if (release_needed) {
        api_thread_pool_release();
    }
}

/**
 * @brief Direct handler for OPTIONS /api/webrtc/ice
 * 
 * This handler responds to CORS preflight requests for the WebRTC ICE API.
 */
void mg_handle_go2rtc_webrtc_ice_options(struct mg_connection *c, struct mg_http_message *hm) {
    // Acquire the API thread pool to ensure proper thread management
    bool release_needed = true;
    thread_pool_t *pool = api_thread_pool_acquire(api_thread_pool_get_size(), 10);
    if (!pool) {
        log_error("Failed to acquire API thread pool");
        mg_send_json_error(c, 500, "Internal server error");
        return;
    }
    log_info("Handling OPTIONS /api/webrtc/ice request");
    
    // Set CORS headers
    mg_printf(c, "HTTP/1.1 200 OK\r\n");
    mg_printf(c, "Access-Control-Allow-Origin: *\r\n");
    mg_printf(c, "Access-Control-Allow-Methods: POST, OPTIONS\r\n");
    mg_printf(c, "Access-Control-Allow-Headers: Content-Type, Authorization, Origin, X-Requested-With, Accept\r\n");
    mg_printf(c, "Access-Control-Allow-Credentials: true\r\n");
    mg_printf(c, "Content-Length: 0\r\n\r\n");
    
    log_info("Successfully handled OPTIONS request for WebRTC ICE API");
    
    // Release the thread pool when done
    if (release_needed) {
        api_thread_pool_release();
    }
}
