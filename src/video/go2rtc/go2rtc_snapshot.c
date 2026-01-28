/**
 * @file go2rtc_snapshot.c
 * @brief Implementation of go2rtc snapshot API
 */

#include "video/go2rtc/go2rtc_snapshot.h"
#include "core/logger.h"
#include "core/curl_init.h"
#include <curl/curl.h>
#include <stdlib.h>
#include <string.h>

// Buffer for accumulating response data
typedef struct {
    unsigned char *data;
    size_t size;
    size_t capacity;
} snapshot_buffer_t;

/**
 * @brief Callback function for writing received data
 */
static size_t write_callback(void *contents, size_t size, size_t nmemb, void *userp) {
    size_t realsize = size * nmemb;
    snapshot_buffer_t *buffer = (snapshot_buffer_t *)userp;
    
    // Check if we need to expand the buffer
    size_t new_size = buffer->size + realsize;
    if (new_size > buffer->capacity) {
        // Double the capacity or use new_size, whichever is larger
        size_t new_capacity = buffer->capacity * 2;
        if (new_capacity < new_size) {
            new_capacity = new_size;
        }
        
        unsigned char *new_data = realloc(buffer->data, new_capacity);
        if (!new_data) {
            log_error("Failed to allocate memory for snapshot data");
            return 0;
        }
        
        buffer->data = new_data;
        buffer->capacity = new_capacity;
    }
    
    // Copy the data
    memcpy(buffer->data + buffer->size, contents, realsize);
    buffer->size += realsize;
    
    return realsize;
}

/**
 * @brief Get a JPEG snapshot from go2rtc for a stream
 */
bool go2rtc_get_snapshot(const char *stream_name, unsigned char **jpeg_data, size_t *jpeg_size) {
    if (!stream_name || !jpeg_data || !jpeg_size) {
        log_error("Invalid parameters for go2rtc_get_snapshot");
        return false;
    }
    
    CURL *curl;
    CURLcode res;
    char url[512];
    bool success = false;
    
    // Initialize the buffer
    snapshot_buffer_t buffer = {
        .data = malloc(65536), // Start with 64KB
        .size = 0,
        .capacity = 65536
    };
    
    if (!buffer.data) {
        log_error("Failed to allocate initial buffer for snapshot");
        return false;
    }
    
    // Ensure curl is globally initialized (thread-safe, idempotent)
    if (curl_init_global() != 0) {
        log_error("Failed to initialize curl global for snapshot");
        free(buffer.data);
        return false;
    }

    // Initialize CURL handle
    curl = curl_easy_init();
    if (!curl) {
        log_error("Failed to initialize CURL for snapshot");
        free(buffer.data);
        return false;
    }
    
    // Format the URL for the go2rtc snapshot API
    // go2rtc runs on port 1984 and provides snapshots at: /api/frame.jpeg?src={stream_name}
    snprintf(url, sizeof(url), "http://localhost:1984/api/frame.jpeg?src=%s", stream_name);
    
    log_info("Fetching snapshot from go2rtc: %s", url);
    
    // Set CURL options
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_callback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &buffer);
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 10L); // 10 second timeout
    curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, 1L); // Follow redirects
    curl_easy_setopt(curl, CURLOPT_NOSIGNAL, 1L); // Prevent curl from using signals (required for multi-threaded apps)
    
    // Perform the request
    res = curl_easy_perform(curl);
    
    // Check for errors
    if (res != CURLE_OK) {
        log_error("CURL request failed for snapshot: %s", curl_easy_strerror(res));
        free(buffer.data);
    } else {
        long http_code = 0;
        curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
        
        if (http_code == 200 && buffer.size > 0) {
            log_info("Successfully fetched snapshot from go2rtc: %zu bytes", buffer.size);
            *jpeg_data = buffer.data;
            *jpeg_size = buffer.size;
            success = true;
        } else {
            log_error("Failed to fetch snapshot from go2rtc (HTTP %ld, size: %zu)", http_code, buffer.size);
            free(buffer.data);
        }
    }
    
    // Clean up CURL
    curl_easy_cleanup(curl);
    
    return success;
}

