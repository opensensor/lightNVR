/**
 * @file go2rtc_api.c
 * @brief Implementation of the go2rtc API client
 */

#include "video/go2rtc/go2rtc_api.h"
#include "core/logger.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <curl/curl.h>
#include <ctype.h>
#include <cjson/cJSON.h>


// API client configuration
static char *g_api_host = NULL;
static int g_api_port = 0;
static bool g_initialized = false;

// Buffer sizes
// HTTP_RESPONSE_SIZE must be large enough to hold the full JSON from
// go2rtc's /api/streams endpoint, which lists *all* registered streams
// with their source URLs.  4KB was too small for ≥3 cameras with long
// RTSP URLs, causing truncation and failed stream-existence checks.
#define HTTP_RESPONSE_SIZE 65536   // For holding complete HTTP responses (64KB)
#define URL_BUFFER_SIZE   1024

bool go2rtc_api_init(const char *api_host, int api_port) {
    if (g_initialized) {
        log_warn("go2rtc API client already initialized");
        return false;
    }
    
    if (!api_host || api_port <= 0) {
        log_error("Invalid parameters for go2rtc_api_init");
        return false;
    }
    
    g_api_host = strdup(api_host);
    g_api_port = api_port;
    g_initialized = true;
    
    log_info("go2rtc API client initialized with host: %s, port: %d", g_api_host, g_api_port);
    return true;
}

// CURL response callback
struct MemoryStruct {
    char *memory;
    size_t size;
};

static size_t WriteMemoryCallback(void *contents, size_t size, size_t nmemb, void *userp) {
    size_t realsize = size * nmemb;
    struct MemoryStruct *mem = (struct MemoryStruct *)userp;
    
    char *ptr = realloc(mem->memory, mem->size + realsize + 1);
    if (!ptr) {
        log_error("Not enough memory for CURL response");
        return 0;
    }
    
    mem->memory = ptr;
    memcpy(&(mem->memory[mem->size]), contents, realsize);
    mem->size += realsize;
    mem->memory[mem->size] = 0;
    
    return realsize;
}

// Per-request response buffer helper
typedef struct {
    char buffer[HTTP_RESPONSE_SIZE];
    size_t size;
} response_buffer_t;

// CURL write callback that uses a per-request stack buffer
static size_t PerRequestWriteCallback(void *contents, size_t size, size_t nmemb, void *userp) {
    size_t realsize = size * nmemb;
    response_buffer_t *resp = (response_buffer_t *)userp;

    // Check if we have enough space in the buffer
    if (resp->size + realsize + 1 > HTTP_RESPONSE_SIZE) {
        log_warn("CURL response buffer full, truncating");
        if (resp->size + 1 >= HTTP_RESPONSE_SIZE) {
            return 0; // Buffer is already full
        }
        realsize = HTTP_RESPONSE_SIZE - resp->size - 1;
    }

    // Copy data to the buffer
    memcpy(resp->buffer + resp->size, contents, realsize);
    resp->size += realsize;
    resp->buffer[resp->size] = 0;

    return realsize;
}

bool go2rtc_api_add_stream(const char *stream_id, const char *stream_url) {
    if (!g_initialized) {
        log_error("go2rtc API client not initialized");
        return false;
    }

    if (!stream_id || !stream_url) {
        log_error("Invalid parameters for go2rtc_api_add_stream");
        return false;
    }

    CURL *curl;
    CURLcode res;
    char url[URL_BUFFER_SIZE];
    bool success = false;

    // Initialize CURL
    curl = curl_easy_init();
    if (!curl) {
        log_error("Failed to initialize CURL");
        return false;
    }

    // Per-request response buffer (no global mutex needed)
    response_buffer_t resp = { .size = 0 };
    resp.buffer[0] = '\0';

    // Format the URL for the API endpoint with query parameters (simple method)
    // This is the method that works according to user feedback
    // URL encode the stream_url to handle special characters
    char encoded_url[URL_BUFFER_SIZE * 3] = {0}; // Extra space for URL encoding

    // Simple URL encoding for special characters
    const char *p = stream_url;
    char *q = encoded_url;
    while (*p && (q - encoded_url < URL_BUFFER_SIZE * 3 - 4)) {
        if (isalnum(*p) || *p == '-' || *p == '_' || *p == '.' || *p == '~') {
            *q++ = *p;
        } else if (*p == ' ') {
            *q++ = '+';
        } else {
            sprintf(q, "%%%02X", (unsigned char)*p);
            q += 3;
        }
        p++;
    }
    *q = '\0';

    snprintf(url, sizeof(url), "http://%s:%d" GO2RTC_BASE_PATH "/api/streams?src=%s&name=%s", // codeql[cpp/non-https-url] - localhost-only internal API
            g_api_host, g_api_port, encoded_url, stream_id);

    log_info("Adding stream with URL: %s", url);

    // Set CURL options for PUT request
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_CUSTOMREQUEST, "PUT");
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, PerRequestWriteCallback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &resp);
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 10L);        // 10s total request timeout
    curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT, 5L);  // 5s connect timeout
    curl_easy_setopt(curl, CURLOPT_NOSIGNAL, 1L);        // Thread-safe: no SIGALRM

    // Perform the request
    res = curl_easy_perform(curl);

    // Check for errors
    if (res != CURLE_OK) {
        log_error("CURL request failed: %s", curl_easy_strerror(res));
    } else {
        long http_code = 0;
        curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);

        if (http_code == 200) {
            log_info("Added stream to go2rtc: %s -> %s", stream_id, stream_url);
            log_info("Response: %s", resp.buffer);
            success = true;
        } else {
            log_error("Failed to add stream to go2rtc (status %ld): %s", http_code, resp.buffer);
        }
    }

    // Clean up
    curl_easy_cleanup(curl);

    return success;
}

bool go2rtc_api_add_stream_multi(const char *stream_id, const char **sources, int num_sources) {
    if (!g_initialized) {
        log_error("go2rtc API client not initialized");
        return false;
    }

    if (!stream_id || !sources || num_sources <= 0) {
        log_error("Invalid parameters for go2rtc_api_add_stream_multi");
        return false;
    }

    CURL *curl;
    CURLcode res;
    bool success = false;

    // Initialize CURL
    curl = curl_easy_init();
    if (!curl) {
        log_error("Failed to initialize CURL");
        return false;
    }

    // Per-request response buffer (no global mutex needed)
    response_buffer_t resp = { .size = 0 };
    resp.buffer[0] = '\0';

    // Build URL with multiple src parameters
    // Format: http://host:port/api/streams?src=url1&src=url2&name=stream_id
    // We need a larger buffer for multiple sources
    char *url = malloc(URL_BUFFER_SIZE * (num_sources + 1));
    if (!url) {
        log_error("Failed to allocate memory for URL");
        curl_easy_cleanup(curl);
        return false;
    }

    size_t url_buf_size = (size_t)URL_BUFFER_SIZE * (num_sources + 1);
    int written = snprintf(url, url_buf_size, "http://%s:%d" GO2RTC_BASE_PATH "/api/streams?", // codeql[cpp/non-https-url] - localhost-only internal API
                           g_api_host, g_api_port);
    if (written < 0 || (size_t)written >= url_buf_size) {
        log_error("URL buffer overflow building go2rtc request");
        free(url);
        curl_easy_cleanup(curl);
        return false;
    }
    size_t offset = (size_t)written;

    // Add each source as a src parameter
    for (int i = 0; i < num_sources; i++) {
        // URL encode the source
        char encoded_url[URL_BUFFER_SIZE * 3] = {0};
        const char *p = sources[i];
        char *q = encoded_url;
        while (*p && (q - encoded_url < URL_BUFFER_SIZE * 3 - 4)) {
            if (isalnum(*p) || *p == '-' || *p == '_' || *p == '.' || *p == '~') {
                *q++ = *p;
            } else if (*p == ' ') {
                *q++ = '+';
            } else {
                sprintf(q, "%%%02X", (unsigned char)*p);
                q += 3;
            }
            p++;
        }
        *q = '\0';

        if (i > 0) {
            written = snprintf(url + offset, url_buf_size - offset, "&");
            if (written < 0 || (size_t)written >= url_buf_size - offset) { break; }
            offset += (size_t)written;
        }
        written = snprintf(url + offset, url_buf_size - offset, "src=%s", encoded_url);
        if (written < 0 || (size_t)written >= url_buf_size - offset) { break; }
        offset += (size_t)written;
    }

    // Add stream name
    written = snprintf(url + offset, url_buf_size - offset, "&name=%s", stream_id);
    if (written < 0 || (size_t)written >= url_buf_size - offset) {
        log_warn("URL truncated building go2rtc request for stream %s", stream_id);
    }

    log_info("Adding stream with multiple sources: %s", url);

    // Set CURL options for PUT request
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_CUSTOMREQUEST, "PUT");
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, PerRequestWriteCallback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &resp);
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 10L);        // 10s total request timeout
    curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT, 5L);  // 5s connect timeout
    curl_easy_setopt(curl, CURLOPT_NOSIGNAL, 1L);        // Thread-safe: no SIGALRM

    // Perform the request
    res = curl_easy_perform(curl);

    // Check for errors
    if (res != CURLE_OK) {
        log_error("CURL request failed: %s", curl_easy_strerror(res));
    } else {
        long http_code = 0;
        curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);

        if (http_code == 200) {
            log_info("Added stream to go2rtc with %d sources: %s", num_sources, stream_id);
            log_info("Response: %s", resp.buffer);
            success = true;
        } else {
            log_error("Failed to add stream to go2rtc (status %ld): %s", http_code, resp.buffer);
        }
    }

    // Clean up
    free(url);
    curl_easy_cleanup(curl);

    return success;
}

bool go2rtc_api_remove_stream(const char *stream_id) {
    if (!g_initialized) {
        log_error("go2rtc API client not initialized");
        return false;
    }

    if (!stream_id) {
        log_error("Invalid parameter for go2rtc_api_remove_stream");
        return false;
    }

    CURL *curl;
    CURLcode res;
    char url[URL_BUFFER_SIZE];
    bool success = false;

    // Initialize CURL
    curl = curl_easy_init();
    if (!curl) {
        log_error("Failed to initialize CURL");
        return false;
    }

    // Per-request response buffer (no global mutex needed)
    response_buffer_t resp = { .size = 0 };
    resp.buffer[0] = '\0';

    // Format the URL for the API endpoint with the src parameter
    snprintf(url, sizeof(url), "http://%s:%d" GO2RTC_BASE_PATH "/api/streams?src=%s", g_api_host, g_api_port, stream_id); // codeql[cpp/non-https-url] - localhost-only internal API

    // Log the URL for debugging
    log_info("DELETE URL: %s", url);

    // Set CURL options for DELETE request
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_CUSTOMREQUEST, "DELETE");
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, PerRequestWriteCallback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &resp);

    // Perform the request
    res = curl_easy_perform(curl);

    // Check for errors
    if (res != CURLE_OK) {
        log_error("CURL request failed: %s", curl_easy_strerror(res));
    } else {
        long http_code = 0;
        curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);

        if (http_code == 200) {
            log_info("Removed stream from go2rtc: %s", stream_id);
            log_info("Response: %s", resp.buffer);
            success = true;
        } else {
            log_error("Failed to remove stream from go2rtc (status %ld): %s", http_code, resp.buffer);

            // Try the old method as a fallback
            log_info("Trying old method as fallback");

            // Reset response buffer for retry
            resp.size = 0;
            resp.buffer[0] = '\0';

            // Format the URL for the old API endpoint
            snprintf(url, sizeof(url), "http://%s:%d" GO2RTC_BASE_PATH "/api/streams/%s", g_api_host, g_api_port, stream_id); // codeql[cpp/non-https-url] - localhost-only internal API

            log_info("Fallback DELETE URL: %s", url);

            // Set CURL options for DELETE request
            curl_easy_setopt(curl, CURLOPT_URL, url);

            // Perform the request
            res = curl_easy_perform(curl);

            // Check for errors
            if (res != CURLE_OK) {
                log_error("CURL fallback request failed: %s", curl_easy_strerror(res));
            } else {
                curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);

                if (http_code == 200) {
                    log_info("Removed stream from go2rtc using old method: %s", stream_id);
                    log_info("Response: %s", resp.buffer);
                    success = true;
                } else {
                    log_error("Failed to remove stream from go2rtc using old method (status %ld): %s",
                              http_code, resp.buffer);
                }
            }
        }
    }

    // Clean up
    curl_easy_cleanup(curl);

    return success;
}

bool go2rtc_api_stream_exists(const char *stream_id) {
    if (!g_initialized) {
        log_error("go2rtc API client not initialized");
        return false;
    }

    if (!stream_id) {
        log_error("Invalid parameter for go2rtc_api_stream_exists");
        return false;
    }

    CURL *curl;
    CURLcode res;
    char url[URL_BUFFER_SIZE];
    bool exists = false;

    // Initialize CURL
    curl = curl_easy_init();
    if (!curl) {
        log_error("Failed to initialize CURL");
        return false;
    }

    // Per-request response buffer (no global mutex needed)
    response_buffer_t resp = { .size = 0 };
    resp.buffer[0] = '\0';

    // Format the URL for the API endpoint
    snprintf(url, sizeof(url), "http://%s:%d" GO2RTC_BASE_PATH "/api/streams", // codeql[cpp/non-https-url] - localhost-only internal API
             g_api_host, g_api_port);

    // Set CURL options for GET request
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, PerRequestWriteCallback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &resp);

    // Perform the request
    res = curl_easy_perform(curl);

    if (res != CURLE_OK) {
        log_error("CURL request failed for stream_exists: %s", curl_easy_strerror(res));
    } else {
        long http_code = 0;
        curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);

        if (http_code == 200) {
            // Parse JSON response and check if stream_id exists as a key
            // The /api/streams response is a JSON object with stream names as keys
            cJSON *json = cJSON_Parse(resp.buffer);
            if (json) {
                exists = cJSON_HasObjectItem(json, stream_id);
                cJSON_Delete(json);
                if (exists) {
                    log_debug("Stream %s exists in go2rtc", stream_id);
                } else {
                    log_debug("Stream %s not found in go2rtc JSON response (stream not a key)", stream_id);
                }
            } else {
                // Log first 200 chars of body to help diagnose parsing issues
                char preview[201];
                strncpy(preview, resp.buffer, 200);
                preview[200] = '\0';
                log_warn("Failed to parse go2rtc /api/streams response as JSON. "
                         "Body preview (first 200 chars): %.200s", preview);
            }
        } else {
            log_debug("go2rtc /api/streams returned status=%ld for stream %s check", http_code, stream_id);
        }
    }

    curl_easy_cleanup(curl);

    if (!exists) {
        log_debug("Stream %s not found in go2rtc", stream_id);
    }
    return exists;
}

bool go2rtc_api_get_webrtc_url(const char *stream_id, char *buffer, size_t buffer_size) {
    if (!g_initialized) {
        log_error("go2rtc API client not initialized");
        return false;
    }
    
    if (!stream_id || !buffer || buffer_size == 0) {
        log_error("Invalid parameters for go2rtc_api_get_webrtc_url");
        return false;
    }
    
    // Format the WebRTC URL
    snprintf(buffer, buffer_size, "http://%s:%d" GO2RTC_BASE_PATH "/webrtc/%s", g_api_host, g_api_port, stream_id); // codeql[cpp/non-https-url] - localhost-only internal API
    return true;
}

bool go2rtc_api_update_config(void) {
    if (!g_initialized) {
        log_error("go2rtc API client not initialized");
        return false;
    }

    CURL *curl;
    CURLcode res;
    char url[URL_BUFFER_SIZE];
    bool success = false;

    curl = curl_easy_init();
    if (!curl) {
        log_error("Failed to initialize CURL");
        return false;
    }

    response_buffer_t resp = { .size = 0 };
    resp.buffer[0] = '\0';

    snprintf(url, sizeof(url), "http://%s:%d" GO2RTC_BASE_PATH "/api/config", // codeql[cpp/non-https-url] - localhost-only internal API
             g_api_host, g_api_port);

    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, PerRequestWriteCallback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &resp);

    res = curl_easy_perform(curl);

    if (res != CURLE_OK) {
        log_error("CURL request failed for update_config: %s", curl_easy_strerror(res));
    } else {
        long http_code = 0;
        curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
        if (http_code == 200) {
            log_info("Successfully updated go2rtc configuration");
            success = true;
        } else {
            log_error("Failed to update go2rtc configuration (status %ld): %s", http_code, resp.buffer);
        }
    }

    curl_easy_cleanup(curl);
    return success;
}

/**
 * @brief Get all streams from the go2rtc API
 * 
 * @param buffer Buffer to store the response
 * @param buffer_size Size of the buffer
 * @return true if successful, false otherwise
 */
bool go2rtc_api_get_streams(char *buffer, size_t buffer_size) {
    if (!g_initialized) {
        log_error("go2rtc API client not initialized");
        return false;
    }

    if (!buffer || buffer_size == 0) {
        log_error("Invalid parameters for go2rtc_api_get_streams");
        return false;
    }

    CURL *curl;
    CURLcode res;
    char url[URL_BUFFER_SIZE];
    bool success = false;

    curl = curl_easy_init();
    if (!curl) {
        log_error("Failed to initialize CURL");
        return false;
    }

    response_buffer_t resp = { .size = 0 };
    resp.buffer[0] = '\0';

    snprintf(url, sizeof(url), "http://%s:%d" GO2RTC_BASE_PATH "/api/streams", // codeql[cpp/non-https-url] - localhost-only internal API
             g_api_host, g_api_port);

    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, PerRequestWriteCallback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &resp);

    res = curl_easy_perform(curl);

    if (res != CURLE_OK) {
        log_error("CURL request failed for get_streams: %s", curl_easy_strerror(res));
    } else {
        long http_code = 0;
        curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
        if (http_code == 200) {
            strncpy(buffer, resp.buffer, buffer_size - 1);
            buffer[buffer_size - 1] = '\0';
            success = true;
        }
    }

    curl_easy_cleanup(curl);
    return success;
}

bool go2rtc_api_get_server_info(int *rtsp_port) {
    if (!g_initialized) {
        log_error("go2rtc API client not initialized");
        return false;
    }

    CURL *curl;
    CURLcode res;
    char url[URL_BUFFER_SIZE];
    bool success = false;

    // Initialize CURL
    curl = curl_easy_init();
    if (!curl) {
        log_error("Failed to initialize CURL");
        return false;
    }

    // Per-request response buffer (no global mutex needed)
    response_buffer_t resp = { .size = 0 };
    resp.buffer[0] = '\0';

    // Format the URL for the API endpoint
    snprintf(url, sizeof(url), "http://%s:%d" GO2RTC_BASE_PATH "/api", g_api_host, g_api_port); // codeql[cpp/non-https-url] - localhost-only internal API

    // Set CURL options for GET request
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, PerRequestWriteCallback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &resp);

    // Perform the request
    res = curl_easy_perform(curl);

    // Check for errors
    if (res != CURLE_OK) {
        log_error("CURL request failed: %s", curl_easy_strerror(res));
    } else {
        long http_code = 0;
        curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);

        if (http_code == 200) {
            log_info("Got go2rtc server info: %s", resp.buffer);

            // Parse JSON response
            cJSON *json = cJSON_Parse(resp.buffer);
            if (json) {
                // Extract RTSP port if requested
                if (rtsp_port) {
                    cJSON *rtsp = cJSON_GetObjectItem(json, "rtsp");
                    if (rtsp && cJSON_IsObject(rtsp)) {
                        cJSON *listen = cJSON_GetObjectItem(rtsp, "listen");
                        if (listen && cJSON_IsString(listen)) {
                            const char *listen_str = cJSON_GetStringValue(listen);
                            if (listen_str && listen_str[0] == ':') {
                                // Parse port number from ":8554"
                                *rtsp_port = atoi(listen_str + 1);
                                log_info("Extracted RTSP port: %d", *rtsp_port);
                            } else {
                                log_warn("Invalid RTSP listen format: %s", listen_str ? listen_str : "NULL");
                                *rtsp_port = 8554; // Default RTSP port
                            }
                        } else {
                            log_warn("RTSP listen not found in server info");
                            *rtsp_port = 8554; // Default RTSP port
                        }
                    } else {
                        log_warn("RTSP section not found in server info");
                        *rtsp_port = 8554; // Default RTSP port
                    }
                }

                cJSON_Delete(json);
                success = true;
            } else {
                log_error("Failed to parse server info JSON: %s", resp.buffer);
            }
        } else {
            log_error("Failed to get go2rtc server info (status %ld): %s", http_code, resp.buffer);
        }
    }

    // Clean up
    curl_easy_cleanup(curl);

    return success;
}

bool go2rtc_api_preload_stream(const char *stream_id) {
    if (!g_initialized) {
        log_error("go2rtc API client not initialized");
        return false;
    }

    if (!stream_id) {
        log_error("Invalid parameters for go2rtc_api_preload_stream");
        return false;
    }

    CURL *curl;
    CURLcode res;
    char url[URL_BUFFER_SIZE];
    bool success = false;

    // Initialize CURL
    curl = curl_easy_init();
    if (!curl) {
        log_error("Failed to initialize CURL");
        return false;
    }

    // Per-request response buffer (no global mutex needed)
    response_buffer_t resp = { .size = 0 };
    resp.buffer[0] = '\0';

    // Build the preload URL: PUT /go2rtc/api/preload?src={stream_id}&video&audio
    // This tells go2rtc to keep a persistent consumer connected to the stream
    snprintf(url, sizeof(url), "http://%s:%d" GO2RTC_BASE_PATH "/api/preload?src=%s&video&audio", // codeql[cpp/non-https-url] - localhost-only internal API
            g_api_host, g_api_port, stream_id);

    log_info("Preloading stream with URL: %s", url);

    // Set CURL options for PUT request
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_CUSTOMREQUEST, "PUT");
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, PerRequestWriteCallback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &resp);
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 10L);        // 10s total request timeout
    curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT, 5L);  // 5s connect timeout
    curl_easy_setopt(curl, CURLOPT_NOSIGNAL, 1L);        // Thread-safe: no SIGALRM

    // Perform the request
    res = curl_easy_perform(curl);

    // Check for errors
    if (res != CURLE_OK) {
        log_error("CURL request failed for preload: %s", curl_easy_strerror(res));
    } else {
        long http_code = 0;
        curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);

        if (http_code == 200) {
            log_info("Successfully preloaded stream in go2rtc: %s", stream_id);
            success = true;
        } else {
            log_error("Failed to preload stream in go2rtc (status %ld): %s", http_code, resp.buffer);
        }
    }

    // Clean up
    curl_easy_cleanup(curl);

    return success;
}

void go2rtc_api_cleanup(void) {
    if (!g_initialized) {
        return;
    }

    free(g_api_host);
    g_api_host = NULL;
    g_api_port = 0;
    g_initialized = false;

    log_info("go2rtc API client cleaned up");
}
