/**
 * @file go2rtc_consumer.c
 * @brief Implementation of the go2rtc consumer module for recording and HLS streaming
 */

#include "video/go2rtc/go2rtc_consumer.h"
#include "video/go2rtc/go2rtc_api.h"
#include "video/go2rtc/go2rtc_stream.h"
#include "core/logger.h"
#include "core/config.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <curl/curl.h>

// Buffer sizes
#define URL_BUFFER_SIZE 2048
#define JSON_BUFFER_SIZE 4096

// Consumer state tracking
typedef struct {
    char stream_id[MAX_STREAM_NAME];
    char output_path[MAX_PATH_LENGTH];
    int segment_duration;
    bool is_active;
} consumer_state_t;

// Arrays to track active consumers
#define MAX_CONSUMERS 16
static consumer_state_t g_recording_consumers[MAX_CONSUMERS] = {0};
static consumer_state_t g_hls_consumers[MAX_CONSUMERS] = {0};
static bool g_initialized = false;

// CURL response handling
static size_t write_callback(void *contents, size_t size, size_t nmemb, void *userp) {
    // We don't need to store the response, just acknowledge it
    return size * nmemb;
}

/**
 * @brief Find a consumer state by stream ID
 * 
 * @param consumers Array of consumer states
 * @param stream_id Stream ID to find
 * @return Pointer to consumer state if found, NULL otherwise
 */
static consumer_state_t *find_consumer(consumer_state_t *consumers, const char *stream_id) {
    for (int i = 0; i < MAX_CONSUMERS; i++) {
        if (consumers[i].is_active && strcmp(consumers[i].stream_id, stream_id) == 0) {
            return &consumers[i];
        }
    }
    return NULL;
}

/**
 * @brief Add a new consumer state
 * 
 * @param consumers Array of consumer states
 * @param stream_id Stream ID
 * @param output_path Output path
 * @param segment_duration Segment duration
 * @return Pointer to new consumer state if successful, NULL otherwise
 */
static consumer_state_t *add_consumer(consumer_state_t *consumers, const char *stream_id, 
                                     const char *output_path, int segment_duration) {
    // First check if consumer already exists
    consumer_state_t *existing = find_consumer(consumers, stream_id);
    if (existing) {
        return existing;
    }
    
    // Find an empty slot
    for (int i = 0; i < MAX_CONSUMERS; i++) {
        if (!consumers[i].is_active) {
            strncpy(consumers[i].stream_id, stream_id, MAX_STREAM_NAME - 1);
            consumers[i].stream_id[MAX_STREAM_NAME - 1] = '\0';
            
            strncpy(consumers[i].output_path, output_path, MAX_PATH_LENGTH - 1);
            consumers[i].output_path[MAX_PATH_LENGTH - 1] = '\0';
            
            consumers[i].segment_duration = segment_duration;
            consumers[i].is_active = true;
            
            return &consumers[i];
        }
    }
    
    return NULL;
}

/**
 * @brief Remove a consumer state
 * 
 * @param consumers Array of consumer states
 * @param stream_id Stream ID
 * @return true if consumer was removed, false otherwise
 */
static bool remove_consumer(consumer_state_t *consumers, const char *stream_id) {
    for (int i = 0; i < MAX_CONSUMERS; i++) {
        if (consumers[i].is_active && strcmp(consumers[i].stream_id, stream_id) == 0) {
            consumers[i].is_active = false;
            return true;
        }
    }
    return false;
}

/**
 * @brief Send a request to go2rtc API to add a recording consumer
 * 
 * @param stream_id Stream ID
 * @param output_path Output path
 * @param segment_duration Segment duration
 * @return true if request was successful, false otherwise
 */
static bool add_go2rtc_recording_consumer(const char *stream_id, const char *output_path, int segment_duration) {
    if (!go2rtc_stream_is_ready()) {
        log_error("go2rtc service not running, cannot add recording consumer");
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
    
    // Format the URL for the API endpoint with query parameters
    // Use the main streams API endpoint with the src parameter
    // The format is: stream_id:ffmpeg with output options
    // We need to ensure the output path exists and is properly formatted
    
    // Create a JSON payload for the request
    char json_payload[JSON_BUFFER_SIZE];
    snprintf(json_payload, sizeof(json_payload), 
             "{\"src\": \"%s:ffmpeg\", \"dst\": [\"ffmpeg:output=%s/%%Y%%m%%d-%%H%%M%%S.mp4:segment_time=%d:segment_format=mp4\"]}", 
             stream_id, output_path, segment_duration);
    
    // Format the URL for the API endpoint
    snprintf(url, sizeof(url), "http://localhost:1984/api/stream/%s-recording", stream_id);
    
    log_info("Adding recording consumer for stream %s with URL: %s", stream_id, url);
    log_info("JSON payload: %s", json_payload);
    
    log_info("Adding recording consumer for stream %s with URL: %s", stream_id, url);
    
    // Set CURL options for PUT request
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_CUSTOMREQUEST, "PUT");
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_callback);
    
    // Set the JSON payload
    curl_easy_setopt(curl, CURLOPT_POSTFIELDS, json_payload);
    
    // Set headers for JSON content
    struct curl_slist *headers = NULL;
    headers = curl_slist_append(headers, "Content-Type: application/json");
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
    
    // Perform the request
    res = curl_easy_perform(curl);
    
    // Check for errors
    if (res != CURLE_OK) {
        log_error("CURL request failed: %s", curl_easy_strerror(res));
    } else {
        long http_code = 0;
        curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
        
        if (http_code == 200) {
            log_info("Successfully added recording consumer for stream %s", stream_id);
            success = true;
        } else {
            log_error("Failed to add recording consumer for stream %s (status %ld)", 
                      stream_id, http_code);
        }
    }
    
    // Clean up
    if (headers) {
        curl_slist_free_all(headers);
    }
    curl_easy_cleanup(curl);
    
    return success;
}

/**
 * @brief Send a request to go2rtc API to remove a consumer
 * 
 * @param stream_id Stream ID
 * @param consumer_type Type of consumer ("ffmpeg" or "hls")
 * @return true if request was successful, false otherwise
 */
static bool remove_go2rtc_consumer(const char *stream_id, const char *consumer_type) {
    if (!go2rtc_stream_is_ready()) {
        log_error("go2rtc service not running, cannot remove consumer");
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
    
    // Format the URL for the API endpoint
    // For recording consumers, we need to remove the stream we created
    if (strcmp(consumer_type, "ffmpeg") == 0) {
        // Use the same endpoint format as when we created the stream
        snprintf(url, sizeof(url), "http://localhost:1984/api/stream/%s-recording", stream_id);
    } else {
        // For HLS, we don't need to remove anything as we're using direct access
        log_info("No need to remove HLS consumer for stream %s", stream_id);
        curl_easy_cleanup(curl);
        return true;
    }
    
    log_info("Removing %s consumer for stream %s with URL: %s", 
             consumer_type, stream_id, url);
    
    // Set CURL options for DELETE request
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_CUSTOMREQUEST, "DELETE");
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_callback);
    
    // Perform the request
    res = curl_easy_perform(curl);
    
    // Check for errors
    if (res != CURLE_OK) {
        log_error("CURL request failed: %s", curl_easy_strerror(res));
    } else {
        long http_code = 0;
        curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
        
        if (http_code == 200) {
            log_info("Successfully removed %s consumer for stream %s", consumer_type, stream_id);
            success = true;
        } else {
            log_error("Failed to remove %s consumer for stream %s (status %ld)", 
                      consumer_type, stream_id, http_code);
        }
    }
    
    // Clean up
    curl_easy_cleanup(curl);
    
    return success;
}

bool go2rtc_consumer_init(void) {
    if (g_initialized) {
        log_warn("go2rtc consumer module already initialized");
        return true;
    }
    
    // Initialize consumer state arrays
    memset(g_recording_consumers, 0, sizeof(g_recording_consumers));
    memset(g_hls_consumers, 0, sizeof(g_hls_consumers));
    
    g_initialized = true;
    log_info("go2rtc consumer module initialized");
    
    return true;
}

bool go2rtc_consumer_start_recording(const char *stream_id, const char *output_path, int segment_duration) {
    if (!g_initialized) {
        log_error("go2rtc consumer module not initialized");
        return false;
    }
    
    if (!stream_id || !output_path || segment_duration <= 0) {
        log_error("Invalid parameters for go2rtc_consumer_start_recording");
        return false;
    }
    
    // Check if recording is already active for this stream
    if (go2rtc_consumer_is_recording(stream_id)) {
        log_warn("Recording already active for stream %s", stream_id);
        return true;
    }
    
    // Ensure go2rtc is running
    if (!go2rtc_stream_is_ready()) {
        log_info("go2rtc not running, starting service");
        if (!go2rtc_stream_start_service()) {
            log_error("Failed to start go2rtc service");
            return false;
        }
        
        // Wait for service to start
        int retries = 10;
        while (retries > 0 && !go2rtc_stream_is_ready()) {
            log_info("Waiting for go2rtc service to start... (%d retries left)", retries);
            sleep(1);
            retries--;
        }
        
        if (!go2rtc_stream_is_ready()) {
            log_error("go2rtc service failed to start in time");
            return false;
        }
    }
    
    // Add the recording consumer to go2rtc
    if (!add_go2rtc_recording_consumer(stream_id, output_path, segment_duration)) {
        log_error("Failed to add recording consumer for stream %s", stream_id);
        return false;
    }
    
    // Add to our tracking array
    consumer_state_t *consumer = add_consumer(g_recording_consumers, stream_id, output_path, segment_duration);
    if (!consumer) {
        log_error("Failed to track recording consumer for stream %s (max consumers reached)", stream_id);
        // Try to remove the consumer from go2rtc since we can't track it
        remove_go2rtc_consumer(stream_id, "ffmpeg");
        return false;
    }
    
    log_info("Started recording for stream %s to %s with segment duration %d seconds", 
             stream_id, output_path, segment_duration);
    
    return true;
}

bool go2rtc_consumer_stop_recording(const char *stream_id) {
    if (!g_initialized) {
        log_error("go2rtc consumer module not initialized");
        return false;
    }
    
    if (!stream_id) {
        log_error("Invalid parameter for go2rtc_consumer_stop_recording");
        return false;
    }
    
    // Check if recording is active for this stream
    if (!go2rtc_consumer_is_recording(stream_id)) {
        log_warn("Recording not active for stream %s", stream_id);
        return true;
    }
    
    // Remove the recording consumer from go2rtc
    if (!remove_go2rtc_consumer(stream_id, "ffmpeg")) {
        log_error("Failed to remove recording consumer for stream %s", stream_id);
        return false;
    }
    
    // Remove from our tracking array
    if (!remove_consumer(g_recording_consumers, stream_id)) {
        log_error("Failed to untrack recording consumer for stream %s", stream_id);
        return false;
    }
    
    log_info("Stopped recording for stream %s", stream_id);
    
    return true;
}

bool go2rtc_consumer_start_hls(const char *stream_id, const char *output_path, int segment_duration) {
    if (!g_initialized) {
        log_error("go2rtc consumer module not initialized");
        return false;
    }
    
    if (!stream_id || !output_path || segment_duration <= 0) {
        log_error("Invalid parameters for go2rtc_consumer_start_hls");
        return false;
    }
    
    // Check if HLS is already active for this stream
    if (go2rtc_consumer_is_hls_active(stream_id)) {
        log_warn("HLS already active for stream %s", stream_id);
        return true;
    }
    
    // Ensure go2rtc is running
    if (!go2rtc_stream_is_ready()) {
        log_info("go2rtc not running, starting service");
        if (!go2rtc_stream_start_service()) {
            log_error("Failed to start go2rtc service");
            return false;
        }
        
        // Wait for service to start
        int retries = 10;
        while (retries > 0 && !go2rtc_stream_is_ready()) {
            log_info("Waiting for go2rtc service to start... (%d retries left)", retries);
            sleep(1);
            retries--;
        }
        
        if (!go2rtc_stream_is_ready()) {
            log_error("go2rtc service failed to start in time");
            return false;
        }
    }
    
    // For HLS, we don't need to add a consumer, we can just use the direct URL
    // We'll just track that we're using HLS for this stream
    
    // Add to our tracking array
    consumer_state_t *consumer = add_consumer(g_hls_consumers, stream_id, output_path, segment_duration);
    if (!consumer) {
        log_error("Failed to track HLS consumer for stream %s (max consumers reached)", stream_id);
        return false;
    }
    
    log_info("Started HLS streaming for stream %s to %s with segment duration %d seconds", 
             stream_id, output_path, segment_duration);
    
    return true;
}

bool go2rtc_consumer_stop_hls(const char *stream_id) {
    if (!g_initialized) {
        log_error("go2rtc consumer module not initialized");
        return false;
    }
    
    if (!stream_id) {
        log_error("Invalid parameter for go2rtc_consumer_stop_hls");
        return false;
    }
    
    // Check if HLS is active for this stream
    if (!go2rtc_consumer_is_hls_active(stream_id)) {
        log_warn("HLS not active for stream %s", stream_id);
        return true;
    }
    
    // For HLS, we don't need to remove a consumer, we can just stop tracking it
    
    // Remove from our tracking array
    if (!remove_consumer(g_hls_consumers, stream_id)) {
        log_error("Failed to untrack HLS consumer for stream %s", stream_id);
        return false;
    }
    
    log_info("Stopped HLS streaming for stream %s", stream_id);
    
    return true;
}

bool go2rtc_consumer_is_recording(const char *stream_id) {
    if (!g_initialized || !stream_id) {
        return false;
    }
    
    return find_consumer(g_recording_consumers, stream_id) != NULL;
}

bool go2rtc_consumer_is_hls_active(const char *stream_id) {
    if (!g_initialized || !stream_id) {
        return false;
    }
    
    return find_consumer(g_hls_consumers, stream_id) != NULL;
}

void go2rtc_consumer_cleanup(void) {
    if (!g_initialized) {
        return;
    }
    
    // Stop all recording consumers
    for (int i = 0; i < MAX_CONSUMERS; i++) {
        if (g_recording_consumers[i].is_active) {
            go2rtc_consumer_stop_recording(g_recording_consumers[i].stream_id);
        }
    }
    
    // Stop all HLS consumers
    for (int i = 0; i < MAX_CONSUMERS; i++) {
        if (g_hls_consumers[i].is_active) {
            go2rtc_consumer_stop_hls(g_hls_consumers[i].stream_id);
        }
    }
    
    g_initialized = false;
    log_info("go2rtc consumer module cleaned up");
}
