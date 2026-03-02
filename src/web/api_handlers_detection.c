#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <unistd.h>
#include <sys/stat.h>
#include <dirent.h>

#include "web/api_handlers.h"
#include "web/request_response.h"
#include "core/logger.h"
#include "core/config.h"
#include "video/detection.h"
#include "video/detection_result.h"
#include "video/stream_manager.h"
#include "database/database_manager.h"

// Minimum/default window for live-view detection queries (seconds).
// The actual window is max(MIN_DETECTION_AGE, stream_detection_interval * 2) so
// that a stored detection is never aged out before the next one could be stored.
// Without this, a 5-second window paired with a 5-second detection interval creates
// a guaranteed gap where the API returns nothing (stale box disappears mid-motion).
#define MIN_DETECTION_AGE 30

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

    // If no time range specified, compute a per-stream max_age so that a stored
    // detection is never aged out before the next one can arrive.  We use at
    // least MIN_DETECTION_AGE (30 s) or the stream's detection_interval * 2,
    // whichever is larger.
    uint64_t max_age = MIN_DETECTION_AGE;
    if (start_time > 0 || end_time > 0) {
        // Custom time range specified — bypass max_age entirely
        max_age = 0;
    } else {
        // For live detection queries (no time range), require stream to exist
        stream_handle_t stream = get_stream_by_name(stream_name);
        if (!stream) {
            log_error("Stream not found: %s", stream_name);
            http_response_set_json_error(res, 404, "Stream not found");
            return;
        }

        // Widen the window to cover at least two full detection intervals so
        // the bounding box stays visible between consecutive detection checks.
        stream_config_t stream_cfg;
        if (get_stream_config(stream, &stream_cfg) == 0 && stream_cfg.detection_interval > 0) {
            uint64_t interval_window = (uint64_t)stream_cfg.detection_interval * 2;
            if (interval_window > max_age) {
                max_age = interval_window;
            }
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
    struct tm tm_buf;
    const struct tm *tm_info = localtime_r(&now, &tm_buf);
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
