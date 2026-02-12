/**
 * @file go2rtc_snapshot.c
 * @brief Implementation of go2rtc snapshot API
 */

#include "video/go2rtc/go2rtc_snapshot.h"
#include "video/go2rtc/go2rtc_api.h"
#include "core/logger.h"
#include "core/curl_init.h"
#include <curl/curl.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>

// Buffer for accumulating response data
typedef struct {
    unsigned char *data;
    size_t size;
    size_t capacity;
} snapshot_buffer_t;

// Thread-local CURL handle for connection reuse
// Each thread gets its own handle to avoid mutex contention
static __thread CURL *tls_curl_handle = NULL;
static __thread bool tls_curl_initialized = false;

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
 * @brief Get or create a thread-local CURL handle for connection reuse
 */
static CURL *get_thread_curl_handle(void) {
    if (!tls_curl_initialized) {
        // Ensure curl is globally initialized (thread-safe, idempotent)
        if (curl_init_global() != 0) {
            log_error("Failed to initialize curl global for snapshot");
            return NULL;
        }

        tls_curl_handle = curl_easy_init();
        if (tls_curl_handle) {
            tls_curl_initialized = true;
            log_debug("Created thread-local CURL handle for snapshot requests");
        }
    }
    return tls_curl_handle;
}

/**
 * @brief Cleanup thread-local CURL handle (call before thread exit if needed)
 */
void go2rtc_snapshot_cleanup_thread(void) {
    if (tls_curl_initialized && tls_curl_handle) {
        curl_easy_cleanup(tls_curl_handle);
        tls_curl_handle = NULL;
        tls_curl_initialized = false;
    }
}

/**
 * @brief Get a JPEG snapshot from go2rtc for a stream
 */
bool go2rtc_get_snapshot(const char *stream_name, unsigned char **jpeg_data, size_t *jpeg_size) {
    if (!stream_name || !jpeg_data || !jpeg_size) {
        log_error("Invalid parameters for go2rtc_get_snapshot");
        return false;
    }

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

    // Get or create thread-local CURL handle for connection reuse
    CURL *curl = get_thread_curl_handle();
    if (!curl) {
        log_error("Failed to get CURL handle for snapshot");
        free(buffer.data);
        return false;
    }

    // Reset the handle for reuse (clears previous request state but keeps connection)
    curl_easy_reset(curl);

    // Format the URL for the go2rtc snapshot API
    // go2rtc runs on port 1984 and provides snapshots at: /api/frame.jpeg?src={stream_name}
    // We add cache=30s to allow go2rtc to return a cached frame if the stream is temporarily
    // unavailable (e.g., video doorbell in sleep mode). This prevents timeouts when the
    // producer is reconnecting, as long as a frame was captured within the last 30 seconds.
    snprintf(url, sizeof(url), "http://localhost:1984" GO2RTC_BASE_PATH "/api/frame.jpeg?src=%s&cache=30s", stream_name);

    log_debug("Fetching snapshot from go2rtc: %s", url);
    
    // Set CURL options
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_callback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &buffer);

    // Timeout settings:
    // - Connection timeout: 5 seconds to establish TCP connection
    // - Total timeout: 10 seconds for the entire operation
    // go2rtc may need time to decode a frame, especially on first access or after stream idle
    curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT, 5L);
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 10L);

    curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, 1L); // Follow redirects
    curl_easy_setopt(curl, CURLOPT_NOSIGNAL, 1L); // Prevent curl from using signals (required for multi-threaded apps)

    // Enable TCP keep-alive to maintain connection health
    curl_easy_setopt(curl, CURLOPT_TCP_KEEPALIVE, 1L);
    curl_easy_setopt(curl, CURLOPT_TCP_KEEPIDLE, 30L);  // Start keep-alive after 30 seconds idle
    curl_easy_setopt(curl, CURLOPT_TCP_KEEPINTVL, 15L); // Send keep-alive every 15 seconds

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

    // Note: We don't cleanup the CURL handle here - it's reused across requests
    // The thread-local handle will be cleaned up when go2rtc_snapshot_cleanup_thread() is called
    // or when the thread exits

    return success;
}

