#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <unistd.h>
#include <sys/stat.h>
#include <dirent.h>
#include <curl/curl.h>
#include <cjson/cJSON.h>

#include "web/api_handlers.h"
#include "web/request_response.h"
#include "core/logger.h"
#include "core/config.h"
#include "video/detection.h"
#include "video/detection_result.h"
#include "video/stream_manager.h"
#include "database/database_manager.h"

// Curl response buffer for describe handler
typedef struct {
    char *memory;
    size_t size;
} describe_memory_t;

static size_t describe_write_callback(void *contents, size_t size, size_t nmemb, void *userp) {
    size_t realsize = size * nmemb;
    describe_memory_t *mem = (describe_memory_t *)userp;
    char *ptr = realloc(mem->memory, mem->size + realsize + 1);
    if (!ptr) return 0;
    mem->memory = ptr;
    memcpy(&(mem->memory[mem->size]), contents, realsize);
    mem->size += realsize;
    mem->memory[mem->size] = 0;
    return realsize;
}

// Maximum age of detections to return (in seconds)
// For live view, we want to show only recent detections (5 seconds)
// This prevents detection boxes from being displayed for too long after they occur
#define MAX_DETECTION_AGE 5

/**
 * @brief Backend-agnostic handler for GET /api/detection/results/:stream
 */
void handle_get_detection_results(const http_request_t *req, http_response_t *res) {
    // Extract stream name from URL
    char stream_name[MAX_STREAM_NAME];
    if (http_request_extract_path_param(req, "/api/detection/results/", stream_name, sizeof(stream_name)) != 0) {
        log_error("Failed to extract stream name from URL");
        http_response_set_json_error(res, 400, "Invalid request path");
        return;
    }

    log_info("Handling GET /api/detection/results/%s request", stream_name);

    // Parse query parameters for time range
    time_t start_time = 0;
    time_t end_time = 0;

    // Extract start time parameter
    char start_str[32] = {0};
    if (http_request_get_query_param(req, "start", start_str, sizeof(start_str)) > 0 && start_str[0]) {
        start_time = (time_t)strtoll(start_str, NULL, 10);
        log_info("Using start_time filter: %lld (str='%s')", (long long)start_time, start_str);
    }

    // Extract end time parameter
    char end_str[32] = {0};
    if (http_request_get_query_param(req, "end", end_str, sizeof(end_str)) > 0 && end_str[0]) {
        end_time = (time_t)strtoll(end_str, NULL, 10);
        log_info("Using end_time filter: %lld (str='%s')", (long long)end_time, end_str);
    }

    // If no time range specified, use default MAX_DETECTION_AGE
    uint64_t max_age = MAX_DETECTION_AGE;
    if (start_time > 0 || end_time > 0) {
        // Custom time range specified, don't use max_age
        max_age = 0;
    } else {
        // For live detection queries (no time range), require stream to exist
        stream_handle_t stream = get_stream_by_name(stream_name);
        if (!stream) {
            log_error("Stream not found: %s", stream_name);
            http_response_set_json_error(res, 404, "Stream not found");
            return;
        }
    }

    // Get detection results for the stream
    detection_result_t result;
    memset(&result, 0, sizeof(detection_result_t));

    // Use the time range function
    int count = get_detections_from_db_time_range(stream_name, &result, max_age, start_time, end_time);

    // Also get the timestamps for each detection
    time_t timestamps[MAX_DETECTIONS];
    memset(timestamps, 0, sizeof(timestamps));

    // Get timestamps for the detections
    get_detection_timestamps(stream_name, &result, timestamps, max_age, start_time, end_time);

    if (count < 0) {
        log_error("Failed to get detections from database for stream: %s", stream_name);
        http_response_set_json_error(res, 500, "Failed to get detection results");
        return;
    }

    // Create JSON response
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        log_error("Failed to create response JSON object");
        http_response_set_json_error(res, 500, "Failed to create response JSON");
        return;
    }

    // Create detections array
    cJSON *detections_array = cJSON_CreateArray();
    if (!detections_array) {
        log_error("Failed to create detections JSON array");
        cJSON_Delete(response);
        http_response_set_json_error(res, 500, "Failed to create detections JSON");
        return;
    }

    // Add detections array to response
    cJSON_AddItemToObject(response, "detections", detections_array);

    // Add timestamp
    char timestamp[32];
    time_t now = time(NULL);
    struct tm *tm_info = localtime(&now);
    strftime(timestamp, sizeof(timestamp), "%Y-%m-%d %H:%M:%S", tm_info);
    cJSON_AddStringToObject(response, "timestamp", timestamp);

    // Add each detection to the array
    for (int i = 0; i < result.count; i++) {
        cJSON *detection = cJSON_CreateObject();
        if (!detection) {
            log_error("Failed to create detection JSON object");
            continue;
        }

        // Add detection properties
        cJSON_AddStringToObject(detection, "label", result.detections[i].label);
        cJSON_AddNumberToObject(detection, "confidence", result.detections[i].confidence);
        cJSON_AddNumberToObject(detection, "x", result.detections[i].x);
        cJSON_AddNumberToObject(detection, "y", result.detections[i].y);
        cJSON_AddNumberToObject(detection, "width", result.detections[i].width);
        cJSON_AddNumberToObject(detection, "height", result.detections[i].height);
        cJSON_AddNumberToObject(detection, "timestamp", (double)timestamps[i]);

        cJSON_AddItemToArray(detections_array, detection);
    }

    // Convert to string
    char *json_str = cJSON_PrintUnformatted(response);
    if (!json_str) {
        log_error("Failed to convert response JSON to string");
        cJSON_Delete(response);
        http_response_set_json_error(res, 500, "Failed to convert response JSON to string");
        return;
    }

    // Send response
    http_response_set_json(res, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(response);

    log_info("Successfully handled GET /api/detection/results/%s request, returned %d detections",
             stream_name, result.count);
}

/**
 * @brief Backend-agnostic handler for POST /api/detection/describe
 *
 * Accepts a raw JPEG image body and proxies it to the configured
 * light-object-detect /describe endpoint (VLM image description).
 * Returns: {"description": "..."}
 */
void handle_post_detection_describe(const http_request_t *req, http_response_t *res) {
    extern config_t g_config;

    if (!req->body || req->body_len == 0) {
        log_error("Describe: No image data in request body");
        http_response_set_json_error(res, 400, "No image data provided");
        return;
    }

    // Derive describe URL by replacing the last path segment of the detection URL with "describe".
    // e.g. http://localhost:9001/api/v1/detect -> http://localhost:9001/api/v1/describe
    const char *detection_url = g_config.api_detection_url;
    char describe_url[MAX_URL_LENGTH];
    const char *last_slash = strrchr(detection_url, '/');
    if (last_slash) {
        size_t base_len = (size_t)(last_slash - detection_url);
        snprintf(describe_url, sizeof(describe_url), "%.*s/describe", (int)base_len, detection_url);
    } else {
        snprintf(describe_url, sizeof(describe_url), "%s/describe", detection_url);
    }

    const char *backend = g_config.api_describe_backend;
    if (!backend || strlen(backend) == 0) backend = "moondream";

    char url_with_params[MAX_URL_LENGTH + 64];
    snprintf(url_with_params, sizeof(url_with_params),
             "%s?backend=%s&length=short&return_image=false",
             describe_url, backend);
    log_info("Describe: Sending %zu bytes to %s", req->body_len, url_with_params);

    // Initialize a fresh curl handle for this one-off request
    CURL *curl = curl_easy_init();
    if (!curl) {
        log_error("Describe: Failed to initialize curl");
        http_response_set_json_error(res, 500, "Failed to initialize HTTP client");
        return;
    }

    // Build multipart form with JPEG image
    curl_mime *mime = curl_mime_init(curl);
    curl_mimepart *part = curl_mime_addpart(mime);
    curl_mime_data(part, (const char *)req->body, req->body_len);
    curl_mime_filename(part, "snapshot.jpg");
    curl_mime_type(part, "image/jpeg");
    curl_mime_name(part, "file");

    describe_memory_t chunk;
    chunk.memory = malloc(1);
    chunk.size = 0;

    struct curl_slist *headers = NULL;
    headers = curl_slist_append(headers, "accept: application/json");

    curl_easy_setopt(curl, CURLOPT_URL, url_with_params);
    curl_easy_setopt(curl, CURLOPT_MIMEPOST, mime);
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, describe_write_callback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, (void *)&chunk);
    curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT, 10L);  // Fail fast if service is down
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 300L);        // 5 min — VLM on CPU can be slow
    curl_easy_setopt(curl, CURLOPT_NOSIGNAL, 1L);

    CURLcode curl_res = curl_easy_perform(curl);

    if (curl_res != CURLE_OK) {
        log_error("Describe: curl_easy_perform() failed: %s", curl_easy_strerror(curl_res));
        free(chunk.memory);
        curl_mime_free(mime);
        curl_slist_free_all(headers);
        curl_easy_cleanup(curl);
        http_response_set_json_error(res, 502, "Failed to reach describe service");
        return;
    }

    long http_code = 0;
    curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
    curl_mime_free(mime);
    curl_slist_free_all(headers);
    curl_easy_cleanup(curl);

    if (http_code != 200) {
        log_error("Describe: Service returned HTTP %ld", http_code);
        free(chunk.memory);
        http_response_set_json_error(res, 502, "Describe service returned an error");
        return;
    }

    // Parse the JSON response — expect {"description": "..."}
    cJSON *root = cJSON_Parse(chunk.memory);
    free(chunk.memory);

    if (!root) {
        log_error("Describe: Failed to parse JSON response");
        http_response_set_json_error(res, 502, "Invalid response from describe service");
        return;
    }

    cJSON *desc_item = cJSON_GetObjectItem(root, "description");
    if (!desc_item || !cJSON_IsString(desc_item)) {
        log_error("Describe: Response missing 'description' field");
        cJSON_Delete(root);
        http_response_set_json_error(res, 502, "Describe service response missing description");
        return;
    }

    // Copy description string before freeing root
    size_t desc_len = strlen(desc_item->valuestring);
    log_info("Describe: Successfully got description (%zu chars)", desc_len);

    // Build and send the response
    cJSON *out = cJSON_CreateObject();
    cJSON_AddStringToObject(out, "description", desc_item->valuestring);
    cJSON_Delete(root);

    char *json_str = cJSON_PrintUnformatted(out);
    cJSON_Delete(out);

    if (!json_str) {
        http_response_set_json_error(res, 500, "Failed to serialize response");
        return;
    }

    http_response_set_json(res, 200, json_str);
    free(json_str);
}
