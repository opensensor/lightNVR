#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <errno.h>
#include <unistd.h>
#include <sys/stat.h>
#include <curl/curl.h>
#include <cjson/cJSON.h>
#include <pthread.h>

#include "core/logger.h"
#include "core/config.h"
#include "core/curl_init.h"
#include "core/shutdown_coordinator.h"
#include "core/mqtt_client.h"
#include "video/api_detection.h"
#include "video/detection_result.h"
#include "video/stream_manager.h"
#include "video/stream_state.h"
#include "video/zone_filter.h"
#include "video/ffmpeg_utils.h"
#include "database/db_detections.h"
#include "video/go2rtc/go2rtc_snapshot.h"

// Global variables
static bool initialized = false;
static CURL *curl_handle = NULL;
static pthread_mutex_t curl_mutex = PTHREAD_MUTEX_INITIALIZER;

// Structure to hold memory for curl response
typedef struct {
    char *memory;
    size_t size;
} memory_struct_t;

// Callback function for curl to write data
static size_t write_memory_callback(void *contents, size_t size, size_t nmemb, void *userp) {
    size_t realsize = size * nmemb;
    memory_struct_t *mem = (memory_struct_t *)userp;

    char *ptr = realloc(mem->memory, mem->size + realsize + 1);
    if (!ptr) {
        log_error("Not enough memory for curl response");
        return 0;
    }

    mem->memory = ptr;
    memcpy(&(mem->memory[mem->size]), contents, realsize);
    mem->size += realsize;
    mem->memory[mem->size] = 0;

    return realsize;
}

/**
 * Initialize the API detection system
 */
int init_api_detection_system(void) {
    if (initialized && curl_handle) {
        log_info("API detection system already initialized");
        return 0;  // Already initialized and curl handle is valid
    }

    // If we have a curl handle but initialized is false, clean it up first
    if (curl_handle) {
        log_warn("API detection system has a curl handle but is marked as uninitialized, cleaning up");
        curl_easy_cleanup(curl_handle);
        curl_handle = NULL;
    }

    // Initialize curl global (thread-safe, idempotent)
    if (curl_init_global() != 0) {
        log_error("Failed to initialize curl global");
        return -1;
    }

    curl_handle = curl_easy_init();
    if (!curl_handle) {
        log_error("Failed to initialize curl handle");
        // Note: Don't call curl_global_cleanup() here - it's managed centrally
        return -1;
    }

    initialized = true;
    log_info("API detection system initialized successfully");
    return 0;
}

/**
 * Shutdown the API detection system
 */
void shutdown_api_detection_system(void) {
    // CRITICAL FIX: Always attempt to clean up resources, even if not marked as initialized
    log_info("Shutting down API detection system (initialized: %s, curl_handle: %p)",
             initialized ? "yes" : "no", (void*)curl_handle);

    // Cleanup curl handle if it exists
    if (curl_handle) {
        log_info("Cleaning up curl handle");
        curl_easy_cleanup(curl_handle);
        curl_handle = NULL;
    }

    // Cleanup cached JPEG encoders to free memory
    jpeg_encoder_cleanup_all();

    // Note: Don't call curl_global_cleanup() here - it's managed centrally in curl_init.c
    // The global cleanup will happen at program shutdown

    initialized = false;
    log_info("API detection system shutdown complete");
}

/**
 * Detect objects using the API with go2rtc snapshot
 */
int detect_objects_api(const char *api_url, const unsigned char *frame_data,
                      int width, int height, int channels, detection_result_t *result,
                      const char *stream_name, float threshold) {
    // CRITICAL FIX: Check if we're in shutdown mode or if the stream has been stopped
    if (is_shutdown_initiated()) {
        log_info("API Detection: System shutdown in progress, skipping detection");
        return -1;
    }

    // CRITICAL FIX: Add thread safety for curl operations
    pthread_mutex_lock(&curl_mutex);

    // Initialize result to empty at the beginning to prevent segmentation fault
    if (result) {
        memset(result, 0, sizeof(detection_result_t));
    } else {
        log_error("API Detection: NULL result pointer provided");
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // CRITICAL FIX: Check if api_url is the special "api-detection" string
    // If so, get the actual URL from the global config
    const char *actual_api_url = api_url;
    if (api_url && strcmp(api_url, "api-detection") == 0) {
        // Get the API URL from the global config
        extern config_t g_config;
        actual_api_url = g_config.api_detection_url;
        log_info("API Detection: Using API URL from config: %s", actual_api_url ? actual_api_url : "NULL");
    }

    log_info("API Detection: Starting detection with API URL: %s", actual_api_url);
    log_info("API Detection: Stream name: %s", stream_name ? stream_name : "NULL");

    if (!initialized || !curl_handle) {
        log_error("API detection system not initialized");
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // CRITICAL FIX: Reset the curl handle to prevent heap corruption from stale state
    // When reusing a curl handle, options from previous requests can persist and
    // cause memory corruption. curl_easy_reset() clears all previously set options.
    curl_easy_reset(curl_handle);

    if (!actual_api_url || !result) {
        log_error("Invalid parameters for detect_objects_api");
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // Check if the URL is valid (must start with http:// or https://)
    if (strncmp(actual_api_url, "http://", 7) != 0 && strncmp(actual_api_url, "https://", 8) != 0) {
        log_error("API Detection: Invalid URL format: %s (must start with http:// or https://)", actual_api_url);
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // NEW APPROACH: Use go2rtc to get a JPEG snapshot directly
    // This eliminates the need for ffmpeg conversion and all the associated logs
    unsigned char *jpeg_data = NULL;
    size_t jpeg_size = 0;

    if (stream_name && go2rtc_get_snapshot(stream_name, &jpeg_data, &jpeg_size)) {
        log_info("API Detection: Successfully fetched snapshot from go2rtc: %zu bytes", jpeg_size);
    } else {
        log_warn("API Detection: Failed to get snapshot from go2rtc, falling back to cached JPEG encoding");

        // FALLBACK: Use cached JPEG encoder to encode raw frame to JPEG in memory
        // This reuses the expensive AVCodecContext instead of recreating it every time
        jpeg_encoder_cache_t *encoder = jpeg_encoder_get_cached(width, height, channels, 85);
        if (!encoder) {
            log_error("API Detection: Failed to get cached JPEG encoder");
            pthread_mutex_unlock(&curl_mutex);
            return -1;
        }

        // Encode directly to memory - no temp file needed
        int encode_result = jpeg_encoder_cache_encode_to_memory(encoder, frame_data, &jpeg_data, &jpeg_size);
        if (encode_result != 0) {
            log_error("API Detection: Failed to encode frame to JPEG using cached encoder");
            pthread_mutex_unlock(&curl_mutex);
            return -1;
        }

        log_info("API Detection: Encoded frame to JPEG using cached encoder: %zu bytes", jpeg_size);
    }

    // Validate JPEG data
    if (!jpeg_data || jpeg_size == 0) {
        log_error("API Detection: No JPEG data available");
        if (jpeg_data) free(jpeg_data);
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // Set up curl for multipart/form-data using modern mime API
    // Note: curl_mime_* API replaced deprecated curl_formadd in libcurl 7.56.0
    curl_mime *mime = NULL;
    curl_mimepart *part = NULL;
    memory_struct_t chunk = {0};
    chunk.memory = NULL;
    struct curl_slist *headers = NULL;

    // Create the mime structure
    mime = curl_mime_init(curl_handle);
    if (!mime) {
        log_error("API Detection: Failed to create mime structure");
        free(jpeg_data);
        result->count = 0;
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // Add the file part
    part = curl_mime_addpart(mime);
    if (!part) {
        log_error("API Detection: Failed to add mime part");
        curl_mime_free(mime);
        free(jpeg_data);
        result->count = 0;
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // Set the part name
    CURLcode mime_result;
    mime_result = curl_mime_name(part, "file");
    if (mime_result != CURLE_OK) {
        log_error("API Detection: Failed to set mime name: %s", curl_easy_strerror(mime_result));
        curl_mime_free(mime);
        free(jpeg_data);
        result->count = 0;
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // Use curl_mime_data to pass data directly from memory (CURL_ZERO_TERMINATED not used, we pass size)
    // Note: curl_mime_data copies the data, so we can free jpeg_data after this call
    mime_result = curl_mime_data(part, (const char *)jpeg_data, jpeg_size);
    if (mime_result != CURLE_OK) {
        log_error("API Detection: Failed to set mime data: %s", curl_easy_strerror(mime_result));
        curl_mime_free(mime);
        free(jpeg_data);
        result->count = 0;
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // Set the filename (required for proper multipart form upload)
    mime_result = curl_mime_filename(part, "snapshot.jpg");
    if (mime_result != CURLE_OK) {
        log_error("API Detection: Failed to set mime filename: %s", curl_easy_strerror(mime_result));
        curl_mime_free(mime);
        free(jpeg_data);
        result->count = 0;
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // Set the content type
    mime_result = curl_mime_type(part, "image/jpeg");
    if (mime_result != CURLE_OK) {
        log_error("API Detection: Failed to set mime type: %s", curl_easy_strerror(mime_result));
        curl_mime_free(mime);
        free(jpeg_data);
        result->count = 0;
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // Free the JPEG data now that curl has copied it
    free(jpeg_data);
    jpeg_data = NULL;

    log_info("API Detection: Successfully added JPEG data to form (%zu bytes)", jpeg_size);

    // Get the backend from config (default to "onnx" if not set)
    extern config_t g_config;
    const char *backend = g_config.api_detection_backend;
    if (!backend || strlen(backend) == 0) {
        backend = "onnx";
    }

    // Use passed threshold, or default to 0.5 if negative/zero
    float actual_threshold = (threshold > 0.0f) ? threshold : 0.5f;

    // Construct the URL with query parameters
    char url_with_params[1024];
    snprintf(url_with_params, sizeof(url_with_params),
             "%s?backend=%s&confidence_threshold=%.2f&return_image=false",
             actual_api_url, backend, actual_threshold);
    log_info("API Detection: Using URL with parameters: %s (backend: %s, threshold: %.2f)",
             url_with_params, backend, actual_threshold);

    // Set up the request with the URL including query parameters
    curl_easy_setopt(curl_handle, CURLOPT_URL, url_with_params);
    curl_easy_setopt(curl_handle, CURLOPT_MIMEPOST, mime);

    // Add the accept header
    headers = curl_slist_append(headers, "accept: application/json");
    curl_easy_setopt(curl_handle, CURLOPT_HTTPHEADER, headers);

    // Set up the response buffer
    chunk.memory = malloc(1);
    chunk.size = 0;

    curl_easy_setopt(curl_handle, CURLOPT_WRITEFUNCTION, write_memory_callback);
    curl_easy_setopt(curl_handle, CURLOPT_WRITEDATA, (void *)&chunk);

    // Set a timeout
    curl_easy_setopt(curl_handle, CURLOPT_TIMEOUT, 10);
    curl_easy_setopt(curl_handle, CURLOPT_NOSIGNAL, 1L); // Prevent curl from using signals (required for multi-threaded apps)

    // Perform the request
    log_info("API Detection: Sending request to %s", url_with_params);

    // CRITICAL FIX: Check if curl handle is still valid before performing the request
    if (!curl_handle || !initialized) {
        log_error("API Detection: curl handle is invalid or API detection system not initialized");
        free(chunk.memory);
        curl_mime_free(mime);
        curl_slist_free_all(headers);

        // Initialize result to empty to prevent segmentation fault
        result->count = 0;
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    CURLcode res = curl_easy_perform(curl_handle);

    // Check for errors
    if (res != CURLE_OK) {
        log_error("API Detection: curl_easy_perform() failed: %s", curl_easy_strerror(res));

        // Check if it's a connection error
        if (res == CURLE_COULDNT_CONNECT) {
            log_error("API Detection: Could not connect to server at %s. Is the API server running?", url_with_params);
        } else if (res == CURLE_OPERATION_TIMEDOUT) {
            log_error("API Detection: Connection to %s timed out. Server might be slow or unreachable.", url_with_params);
        } else if (res == CURLE_COULDNT_RESOLVE_HOST) {
            log_error("API Detection: Could not resolve host %s. Check your network connection and DNS settings.", url_with_params);
        } else if (res == CURLE_FAILED_INIT) {
            log_error("API Detection: Curl initialization failed. Reinitializing curl handle...");

            // Attempt to reinitialize curl
            if (curl_handle) {
                curl_easy_cleanup(curl_handle);
                curl_handle = NULL;
            }

            curl_handle = curl_easy_init();
            if (!curl_handle) {
                log_error("API Detection: Failed to reinitialize curl handle");
                initialized = false; // Mark as uninitialized to force reinitialization next time
            } else {
                log_info("API Detection: Successfully reinitialized curl handle");
            }
        }

        free(chunk.memory);
        curl_mime_free(mime);
        curl_slist_free_all(headers);

        // Initialize result to empty to prevent segmentation fault
        result->count = 0;
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // Get the HTTP response code
    long http_code = 0;
    curl_easy_getinfo(curl_handle, CURLINFO_RESPONSE_CODE, &http_code);

    if (http_code != 200) {
        log_error("API request failed with HTTP code %ld", http_code);
        free(chunk.memory);
        curl_mime_free(mime);
        curl_slist_free_all(headers);

        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // Parse the JSON response
    // CRITICAL FIX: Add additional logging and validation for JSON parsing
    if (!chunk.memory || chunk.size == 0) {
        log_error("API Detection: Empty response from server");
        free(chunk.memory);
        curl_mime_free(mime);
        curl_slist_free_all(headers);

        // Initialize result to empty to prevent segmentation fault
        result->count = 0;
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // Log the first few bytes of the response for debugging
    char preview[64] = {0};
    int preview_len = chunk.size < 63 ? chunk.size : 63;
    memcpy(preview, chunk.memory, preview_len);
    preview[preview_len] = '\0';
    // Replace non-printable characters with dots
    for (int i = 0; i < preview_len; i++) {
        if (preview[i] < 32 || preview[i] > 126) {
            preview[i] = '.';
        }
    }
    log_info("API Detection: Response preview: %s", preview);

    cJSON *root = cJSON_Parse(chunk.memory);

    if (!root) {
        const char *error_ptr = cJSON_GetErrorPtr();
        log_error("Failed to parse JSON response: %s", error_ptr ? error_ptr : "Unknown error");
        // Log more details about the response
        log_error("API Detection: Response size: %zu bytes", chunk.size);
        log_error("API Detection: Response preview: %s", preview);
        free(chunk.memory);
        curl_mime_free(mime);
        curl_slist_free_all(headers);

        // Initialize result to empty to prevent segmentation fault
        result->count = 0;
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // Extract the detections
    cJSON *detections = cJSON_GetObjectItem(root, "detections");
    if (!detections || !cJSON_IsArray(detections)) {
        log_error("Invalid JSON response: missing or invalid 'detections' array");
        // Log the JSON for debugging
        char *json_str = cJSON_Print(root);
        if (json_str) {
            log_error("API Detection: Full JSON response: %s", json_str);
            free(json_str);
        }
        cJSON_Delete(root);
        free(chunk.memory);
        curl_mime_free(mime);
        curl_slist_free_all(headers);

        // Initialize result to empty to prevent segmentation fault
        result->count = 0;
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // Process each detection
    int array_size = cJSON_GetArraySize(detections);
    for (int i = 0; i < array_size; i++) {
        if (result->count >= MAX_DETECTIONS) {
            log_warn("Maximum number of detections reached (%d)", MAX_DETECTIONS);
            break;
        }

        cJSON *detection = cJSON_GetArrayItem(detections, i);
        if (!detection) continue;

        // Extract the detection data
        cJSON *label = cJSON_GetObjectItem(detection, "label");
        cJSON *confidence = cJSON_GetObjectItem(detection, "confidence");

        // The bounding box coordinates might be in a nested object
        cJSON *bounding_box = cJSON_GetObjectItem(detection, "bounding_box");
        cJSON *x_min = NULL;
        cJSON *y_min = NULL;
        cJSON *x_max = NULL;
        cJSON *y_max = NULL;

        if (bounding_box) {
            // Get coordinates from the nested bounding_box object
            x_min = cJSON_GetObjectItem(bounding_box, "x_min");
            y_min = cJSON_GetObjectItem(bounding_box, "y_min");
            x_max = cJSON_GetObjectItem(bounding_box, "x_max");
            y_max = cJSON_GetObjectItem(bounding_box, "y_max");
            log_info("API Detection: Found bounding_box object in JSON response");
        } else {
            // Try to get coordinates directly from the detection object (old format)
            x_min = cJSON_GetObjectItem(detection, "x_min");
            y_min = cJSON_GetObjectItem(detection, "y_min");
            x_max = cJSON_GetObjectItem(detection, "x_max");
            y_max = cJSON_GetObjectItem(detection, "y_max");
            log_info("API Detection: Using direct coordinates from JSON response");
        }

        if (!label || !cJSON_IsString(label) ||
            !confidence || !cJSON_IsNumber(confidence) ||
            !x_min || !cJSON_IsNumber(x_min) ||
            !y_min || !cJSON_IsNumber(y_min) ||
            !x_max || !cJSON_IsNumber(x_max) ||
            !y_max || !cJSON_IsNumber(y_max)) {
            log_warn("Invalid detection data in JSON response");
            // Log the actual JSON for debugging
            char *json_str = cJSON_Print(detection);
            if (json_str) {
                log_warn("Detection JSON: %s", json_str);
                free(json_str);
            }
            continue;
        }

        // Add the detection to the result
        strncpy(result->detections[result->count].label, label->valuestring, MAX_LABEL_LENGTH - 1);
        result->detections[result->count].label[MAX_LABEL_LENGTH - 1] = '\0';
        result->detections[result->count].confidence = confidence->valuedouble;
        result->detections[result->count].x = x_min->valuedouble;
        result->detections[result->count].y = y_min->valuedouble;
        result->detections[result->count].width = x_max->valuedouble - x_min->valuedouble;
        result->detections[result->count].height = y_max->valuedouble - y_min->valuedouble;

        // Parse optional track_id field
        cJSON *track_id = cJSON_GetObjectItem(detection, "track_id");
        if (track_id && cJSON_IsNumber(track_id)) {
            result->detections[result->count].track_id = (int)track_id->valuedouble;
        } else {
            result->detections[result->count].track_id = -1; // No tracking
        }

        // Parse optional zone_id field
        cJSON *zone_id = cJSON_GetObjectItem(detection, "zone_id");
        if (zone_id && cJSON_IsString(zone_id)) {
            strncpy(result->detections[result->count].zone_id, zone_id->valuestring, MAX_ZONE_ID_LENGTH - 1);
            result->detections[result->count].zone_id[MAX_ZONE_ID_LENGTH - 1] = '\0';
        } else {
            result->detections[result->count].zone_id[0] = '\0'; // Empty zone
        }

        result->count++;
    }

    // Filter detections by zones before storing
    if (stream_name && stream_name[0] != '\0') {
        log_info("API Detection: Filtering %d detections by zones for stream %s", result->count, stream_name);
        int filter_ret = filter_detections_by_zones(stream_name, result);
        if (filter_ret != 0) {
            log_warn("Failed to filter detections by zones, storing all detections");
        }

        // Store the detections in the database
        time_t timestamp = time(NULL);
        store_detections_in_db(stream_name, result, timestamp);

        // Publish to MQTT if enabled
        if (result->count > 0) {
            mqtt_publish_detection(stream_name, result, timestamp);
        }
    } else {
        log_warn("No stream name provided, skipping database storage");
    }

    // Clean up
    cJSON_Delete(root);
    free(chunk.memory);
    curl_mime_free(mime);
    curl_slist_free_all(headers);

    pthread_mutex_unlock(&curl_mutex);
    return 0;
}

/**
 * Detect objects using the API with go2rtc snapshot only (no frame data required)
 *
 * This function fetches a snapshot directly from go2rtc and sends it to the detection API.
 * It does NOT require decoded frame data, which saves significant memory by avoiding
 * the need to decode video segments.
 *
 * Returns: 0 on success, -1 on general failure, -2 if go2rtc snapshot failed
 */
int detect_objects_api_snapshot(const char *api_url, const char *stream_name,
                                detection_result_t *result, float threshold) {
    // Check if we're in shutdown mode
    if (is_shutdown_initiated()) {
        log_info("API Detection (snapshot): System shutdown in progress, skipping detection");
        return -1;
    }

    // Stream name is required for go2rtc snapshot
    if (!stream_name || stream_name[0] == '\0') {
        log_error("API Detection (snapshot): Stream name is required");
        return -1;
    }

    // Thread safety for curl operations
    pthread_mutex_lock(&curl_mutex);

    // Initialize result
    if (result) {
        memset(result, 0, sizeof(detection_result_t));
    } else {
        log_error("API Detection (snapshot): NULL result pointer provided");
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // Handle "api-detection" special string
    const char *actual_api_url = api_url;
    if (api_url && strcmp(api_url, "api-detection") == 0) {
        extern config_t g_config;
        actual_api_url = g_config.api_detection_url;
        log_info("API Detection (snapshot): Using API URL from config: %s", actual_api_url ? actual_api_url : "NULL");
    }

    if (!initialized || !curl_handle) {
        log_error("API Detection (snapshot): System not initialized");
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    if (!actual_api_url) {
        log_error("API Detection (snapshot): Invalid API URL");
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // Check URL format
    if (strncmp(actual_api_url, "http://", 7) != 0 && strncmp(actual_api_url, "https://", 8) != 0) {
        log_error("API Detection (snapshot): Invalid URL format: %s", actual_api_url);
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // Try to get snapshot from go2rtc
    unsigned char *jpeg_data = NULL;
    size_t jpeg_size = 0;

    if (!go2rtc_get_snapshot(stream_name, &jpeg_data, &jpeg_size)) {
        log_warn("API Detection (snapshot): Failed to get snapshot from go2rtc for stream %s", stream_name);
        pthread_mutex_unlock(&curl_mutex);
        return -2;  // Special return code: go2rtc failed, caller should fall back
    }

    log_info("API Detection (snapshot): Successfully fetched snapshot from go2rtc: %zu bytes", jpeg_size);

    // Validate JPEG data
    if (!jpeg_data || jpeg_size == 0) {
        log_error("API Detection (snapshot): No JPEG data available");
        if (jpeg_data) free(jpeg_data);
        pthread_mutex_unlock(&curl_mutex);
        return -2;
    }

    // Reset curl handle
    curl_easy_reset(curl_handle);

    // Set up curl for multipart/form-data
    curl_mime *mime = NULL;
    curl_mimepart *part = NULL;
    memory_struct_t chunk = {0};
    chunk.memory = NULL;
    struct curl_slist *headers = NULL;

    // Create the mime structure
    mime = curl_mime_init(curl_handle);
    if (!mime) {
        log_error("API Detection (snapshot): Failed to create mime structure");
        free(jpeg_data);
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // Add the file part
    part = curl_mime_addpart(mime);
    if (!part) {
        log_error("API Detection (snapshot): Failed to add mime part");
        curl_mime_free(mime);
        free(jpeg_data);
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // Set up the mime part
    CURLcode mime_result;
    mime_result = curl_mime_name(part, "file");
    if (mime_result != CURLE_OK) {
        curl_mime_free(mime);
        free(jpeg_data);
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    mime_result = curl_mime_data(part, (const char *)jpeg_data, jpeg_size);
    if (mime_result != CURLE_OK) {
        curl_mime_free(mime);
        free(jpeg_data);
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    mime_result = curl_mime_filename(part, "snapshot.jpg");
    if (mime_result != CURLE_OK) {
        curl_mime_free(mime);
        free(jpeg_data);
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    mime_result = curl_mime_type(part, "image/jpeg");
    if (mime_result != CURLE_OK) {
        curl_mime_free(mime);
        free(jpeg_data);
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // Free JPEG data now that curl has copied it
    free(jpeg_data);
    jpeg_data = NULL;

    // Get backend from config
    extern config_t g_config;
    const char *backend = g_config.api_detection_backend;
    if (!backend || strlen(backend) == 0) {
        backend = "onnx";
    }

    float actual_threshold = (threshold > 0.0f) ? threshold : 0.5f;

    // Construct URL with parameters
    char url_with_params[1024];
    snprintf(url_with_params, sizeof(url_with_params),
             "%s?backend=%s&confidence_threshold=%.2f&return_image=false",
             actual_api_url, backend, actual_threshold);

    log_info("API Detection (snapshot): Sending request to %s", url_with_params);

    // Set up the request
    curl_easy_setopt(curl_handle, CURLOPT_URL, url_with_params);
    curl_easy_setopt(curl_handle, CURLOPT_MIMEPOST, mime);

    headers = curl_slist_append(headers, "accept: application/json");
    curl_easy_setopt(curl_handle, CURLOPT_HTTPHEADER, headers);

    chunk.memory = malloc(1);
    chunk.size = 0;

    curl_easy_setopt(curl_handle, CURLOPT_WRITEFUNCTION, write_memory_callback);
    curl_easy_setopt(curl_handle, CURLOPT_WRITEDATA, (void *)&chunk);
    curl_easy_setopt(curl_handle, CURLOPT_TIMEOUT, 10);
    curl_easy_setopt(curl_handle, CURLOPT_NOSIGNAL, 1L); // Prevent curl from using signals (required for multi-threaded apps)

    // Perform the request
    CURLcode res = curl_easy_perform(curl_handle);

    if (res != CURLE_OK) {
        log_error("API Detection (snapshot): curl_easy_perform() failed: %s", curl_easy_strerror(res));
        free(chunk.memory);
        curl_mime_free(mime);
        curl_slist_free_all(headers);
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // Check HTTP response code
    long http_code = 0;
    curl_easy_getinfo(curl_handle, CURLINFO_RESPONSE_CODE, &http_code);

    if (http_code != 200) {
        log_error("API Detection (snapshot): HTTP error %ld", http_code);
        free(chunk.memory);
        curl_mime_free(mime);
        curl_slist_free_all(headers);
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // Parse JSON response
    if (!chunk.memory || chunk.size == 0) {
        log_error("API Detection (snapshot): Empty response");
        free(chunk.memory);
        curl_mime_free(mime);
        curl_slist_free_all(headers);
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    cJSON *root = cJSON_Parse(chunk.memory);
    if (!root) {
        log_error("API Detection (snapshot): Failed to parse JSON response");
        free(chunk.memory);
        curl_mime_free(mime);
        curl_slist_free_all(headers);
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // Extract detections
    cJSON *detections = cJSON_GetObjectItem(root, "detections");
    if (!detections || !cJSON_IsArray(detections)) {
        log_error("API Detection (snapshot): Invalid JSON response");
        cJSON_Delete(root);
        free(chunk.memory);
        curl_mime_free(mime);
        curl_slist_free_all(headers);
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    // Process each detection
    int array_size = cJSON_GetArraySize(detections);
    for (int i = 0; i < array_size; i++) {
        if (result->count >= MAX_DETECTIONS) {
            log_warn("API Detection (snapshot): Maximum detections reached");
            break;
        }

        cJSON *detection = cJSON_GetArrayItem(detections, i);
        if (!detection) continue;

        cJSON *label = cJSON_GetObjectItem(detection, "label");
        cJSON *confidence = cJSON_GetObjectItem(detection, "confidence");

        cJSON *bounding_box = cJSON_GetObjectItem(detection, "bounding_box");
        cJSON *x_min = NULL, *y_min = NULL, *x_max = NULL, *y_max = NULL;

        if (bounding_box) {
            x_min = cJSON_GetObjectItem(bounding_box, "x_min");
            y_min = cJSON_GetObjectItem(bounding_box, "y_min");
            x_max = cJSON_GetObjectItem(bounding_box, "x_max");
            y_max = cJSON_GetObjectItem(bounding_box, "y_max");
        } else {
            x_min = cJSON_GetObjectItem(detection, "x_min");
            y_min = cJSON_GetObjectItem(detection, "y_min");
            x_max = cJSON_GetObjectItem(detection, "x_max");
            y_max = cJSON_GetObjectItem(detection, "y_max");
        }

        if (!label || !cJSON_IsString(label) ||
            !confidence || !cJSON_IsNumber(confidence) ||
            !x_min || !cJSON_IsNumber(x_min) ||
            !y_min || !cJSON_IsNumber(y_min) ||
            !x_max || !cJSON_IsNumber(x_max) ||
            !y_max || !cJSON_IsNumber(y_max)) {
            continue;
        }

        // Add detection to result
        strncpy(result->detections[result->count].label, label->valuestring, MAX_LABEL_LENGTH - 1);
        result->detections[result->count].label[MAX_LABEL_LENGTH - 1] = '\0';
        result->detections[result->count].confidence = confidence->valuedouble;
        result->detections[result->count].x = x_min->valuedouble;
        result->detections[result->count].y = y_min->valuedouble;
        result->detections[result->count].width = x_max->valuedouble - x_min->valuedouble;
        result->detections[result->count].height = y_max->valuedouble - y_min->valuedouble;

        cJSON *track_id = cJSON_GetObjectItem(detection, "track_id");
        result->detections[result->count].track_id = (track_id && cJSON_IsNumber(track_id))
            ? (int)track_id->valuedouble : -1;

        cJSON *zone_id = cJSON_GetObjectItem(detection, "zone_id");
        if (zone_id && cJSON_IsString(zone_id)) {
            strncpy(result->detections[result->count].zone_id, zone_id->valuestring, MAX_ZONE_ID_LENGTH - 1);
            result->detections[result->count].zone_id[MAX_ZONE_ID_LENGTH - 1] = '\0';
        } else {
            result->detections[result->count].zone_id[0] = '\0';
        }

        result->count++;
    }

    // Filter by zones and store in database
    if (stream_name && stream_name[0] != '\0') {
        log_info("API Detection (snapshot): Filtering %d detections by zones for stream %s",
                 result->count, stream_name);
        filter_detections_by_zones(stream_name, result);
        time_t timestamp = time(NULL);
        store_detections_in_db(stream_name, result, timestamp);

        // Publish to MQTT if enabled
        if (result->count > 0) {
            mqtt_publish_detection(stream_name, result, timestamp);
        }
    }

    // Clean up
    cJSON_Delete(root);
    free(chunk.memory);
    curl_mime_free(mime);
    curl_slist_free_all(headers);

    pthread_mutex_unlock(&curl_mutex);
    log_info("API Detection (snapshot): Successfully detected %d objects", result->count);
    return 0;
}
