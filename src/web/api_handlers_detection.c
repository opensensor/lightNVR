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
#include "web/mongoose_adapter.h"
#include "core/logger.h"
#include "core/config.h"
#include "mongoose.h"
#include "video/detection.h"
#include "video/detection_result.h"
#include "video/stream_manager.h"
#include "database/database_manager.h"

// Maximum age of detections to return (in seconds)
#define MAX_DETECTION_AGE 60

/**
 * @brief Direct handler for GET /api/detection/results/:stream
 */
void mg_handle_get_detection_results(struct mg_connection *c, struct mg_http_message *hm) {
    // Extract stream name from URL
    char stream_name[MAX_STREAM_NAME];
    if (mg_extract_path_param(hm, "/api/detection/results/", stream_name, sizeof(stream_name)) != 0) {
        log_error("Failed to extract stream name from URL");
        mg_send_json_error(c, 400, "Invalid request path");
        return;
    }
    
    log_info("Handling GET /api/detection/results/%s request", stream_name);
    
    // Check if stream exists
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Stream not found: %s", stream_name);
        mg_send_json_error(c, 404, "Stream not found");
        return;
    }
    
    // Get detection results for the stream
    detection_result_t result;
    memset(&result, 0, sizeof(detection_result_t));
    
    int count = get_detections_from_db(stream_name, &result, MAX_DETECTION_AGE);
    
    if (count < 0) {
        log_error("Failed to get detections from database for stream: %s", stream_name);
        mg_send_json_error(c, 500, "Failed to get detection results");
        return;
    }
    
    // Create JSON response
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        log_error("Failed to create response JSON object");
        mg_send_json_error(c, 500, "Failed to create response JSON");
        return;
    }
    
    // Create detections array
    cJSON *detections_array = cJSON_CreateArray();
    if (!detections_array) {
        log_error("Failed to create detections JSON array");
        cJSON_Delete(response);
        mg_send_json_error(c, 500, "Failed to create detections JSON");
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
        
        cJSON_AddItemToArray(detections_array, detection);
    }
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(response);
    if (!json_str) {
        log_error("Failed to convert response JSON to string");
        cJSON_Delete(response);
        mg_send_json_error(c, 500, "Failed to convert response JSON to string");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    cJSON_Delete(response);
    
    log_info("Successfully handled GET /api/detection/results/%s request", stream_name);
}
