#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <errno.h>
#include <ctype.h>
#include <dirent.h>
#include <pthread.h>
#include <time.h>

#include "web/api_handlers_detection.h"
#include "web/api_handlers_common.h"
#include "core/logger.h"
#include "core/config.h"
#include "video/detection.h"
#include "video/detection_result.h"
#include "video/stream_manager.h"
#include "database/database_manager.h"

// Maximum age of detections to return (in seconds)
#define MAX_DETECTION_AGE 60
// We no longer use a hardcoded minimum detection confidence threshold
// Instead, we use the user's configured threshold for each stream

/**
 * Initialize detection results storage
 */
void init_detection_results(void) {
    // No initialization needed for database storage
    log_info("Detection results storage initialized (using database)");
}

/**
 * Store detection result for a stream
 */
void store_detection_result(const char *stream_name, const detection_result_t *result) {
    if (!stream_name || !result) {
        log_error("Invalid parameters for store_detection_result: stream_name=%p, result=%p",
                 stream_name, result);
        return;
    }
    
    log_info("Storing detection results for stream '%s': %d detections", stream_name, result->count);
    
    // Store in database
    int ret = store_detections_in_db(stream_name, result, 0); // 0 = use current time
    
    if (ret != 0) {
        log_error("Failed to store detections in database for stream '%s'", stream_name);
        return;
    }
    
    // Log the stored detections
    for (int i = 0; i < result->count; i++) {
        log_info("  Detection %d: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]",
                i, result->detections[i].label,
                result->detections[i].confidence * 100.0f,
                result->detections[i].x,
                result->detections[i].y,
                result->detections[i].width,
                result->detections[i].height);
    }
    
    log_info("Successfully stored %d detections in database for stream '%s'", result->count, stream_name);
}

/**
 * Handle GET request for detection results
 */
void handle_get_detection_results(const http_request_t *request, http_response_t *response) {
    // Extract stream name from URL
    // URL format: /api/detection/results/{stream_name}
    
    const char *path = request->path;
    const char *results_pos = strstr(path, "/results/");
    
    if (!results_pos) {
        create_json_response(response, 400, "{\"error\":\"Invalid request path\"}");
        return;
    }
    
    const char *stream_name_start = results_pos + 9;  // Skip "/results/"
    
    // Extract stream name
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, stream_name_start, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';
    
    // URL decode the stream name
    char decoded_stream[MAX_STREAM_NAME];
    url_decode(stream_name, decoded_stream, sizeof(decoded_stream));
    strncpy(stream_name, decoded_stream, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';
    
    // Check if stream exists
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        create_json_response(response, 404, "{\"error\":\"Stream not found\"}");
        return;
    }
    
    // Get detection results from database
    detection_result_t result;
    memset(&result, 0, sizeof(detection_result_t));
    
    // Debug logging to help diagnose the issue
    log_info("API: Looking for detection results for stream '%s'", stream_name);
    
    int count = get_detections_from_db(stream_name, &result, MAX_DETECTION_AGE);
    
    if (count < 0) {
        log_error("Failed to get detections from database for stream '%s'", stream_name);
        
        // Create empty response JSON
        char json[256];
        snprintf(json, sizeof(json), "{\"stream\":\"%s\",\"detections\":[],\"timestamp\":0}", stream_name);
        create_json_response(response, 200, json);
        log_warn("API: Error getting detection results for stream '%s', returning empty array", stream_name);
        return;
    }
    
    if (count == 0) {
        // No detection results for this stream
        char json[256];
        snprintf(json, sizeof(json), "{\"stream\":\"%s\",\"detections\":[],\"timestamp\":0}", stream_name);
        create_json_response(response, 200, json);
        log_warn("API: No detection results found for stream '%s', returning empty array", stream_name);
        return;
    }
    
    // Build JSON response with detection results
    char json[4096] = "{";
    
    // Add stream name
    char stream_json[MAX_STREAM_NAME + 32];
    snprintf(stream_json, sizeof(stream_json), "\"stream\":\"%s\",", stream_name);
    strcat(json, stream_json);
    
    // Add timestamp (use current time since we're getting recent detections)
    char timestamp_json[64];
    snprintf(timestamp_json, sizeof(timestamp_json), "\"timestamp\":%lld,", 
             (long long)time(NULL));
    strcat(json, timestamp_json);
    
    // Add detections array
    strcat(json, "\"detections\":[");
    
    // Get the stream's configured detection threshold
    stream_config_t stream_config;
    float threshold = 0.5f; // Default fallback threshold
    
    if (get_stream_config(stream, &stream_config) == 0) {
        threshold = stream_config.detection_threshold;
        log_info("Using stream's configured threshold: %.2f (%.0f%%)", 
                threshold, threshold * 100.0f);
    } else {
        log_warn("Failed to get stream config, using default threshold: %.2f (%.0f%%)",
                threshold, threshold * 100.0f);
    }
    
    // Add each detection (only include those with confidence >= threshold)
    int valid_count = 0;
    for (int i = 0; i < result.count; i++) {
        // Skip detections with confidence below threshold
        if (result.detections[i].confidence < threshold) {
            log_info("Skipping detection with confidence %.2f%% (below threshold of %.2f%%)",
                    result.detections[i].confidence * 100.0f, threshold * 100.0f);
            continue;
        }
        
        if (valid_count > 0) {
            strcat(json, ",");
        }
        
        char detection_json[512];
        snprintf(detection_json, sizeof(detection_json),
                "{\"label\":\"%s\",\"confidence\":%.2f,\"x\":%.4f,\"y\":%.4f,\"width\":%.4f,\"height\":%.4f}",
                result.detections[i].label,
                result.detections[i].confidence,
                result.detections[i].x,
                result.detections[i].y,
                result.detections[i].width,
                result.detections[i].height);
        
        strcat(json, detection_json);
        valid_count++;
    }
    
    // Close JSON
    strcat(json, "]}");
    
    // Create response
    log_debug("Creating JSON response with status 200: %s", json);
    create_json_response(response, 200, json);
}

/**
 * Register detection results API handlers
 */
void register_detection_results_api_handlers(void) {
    // Initialize detection results storage
    init_detection_results();
    
    // Register API handlers
    register_request_handler("/api/detection/results/*", "GET", handle_get_detection_results);
    
    // Clean up old detections (older than 1 hour)
    delete_old_detections(3600);
    
    log_info("Detection results API handlers registered");
}

/**
 * Debug function to dump current detection results
 */
void debug_dump_detection_results(void) {
    log_info("DEBUG: Current detection results (from database):");
    
    // Get all stream names
    stream_config_t streams[MAX_STREAMS];
    int stream_count = get_all_stream_configs(streams, MAX_STREAMS);
    
    if (stream_count <= 0) {
        log_info("  No streams found");
        return;
    }
    
    int active_streams = 0;
    
    // For each stream, get detections
    for (int i = 0; i < stream_count; i++) {
        detection_result_t result;
        memset(&result, 0, sizeof(detection_result_t));
        
        int count = get_detections_from_db(streams[i].name, &result, MAX_DETECTION_AGE);
        
        if (count > 0) {
            active_streams++;
            
            log_info("  Stream '%s', %d detections", streams[i].name, result.count);
            
            for (int j = 0; j < result.count; j++) {
                log_info("    Detection %d: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]",
                         j, result.detections[j].label,
                         result.detections[j].confidence * 100.0f,
                         result.detections[j].x,
                         result.detections[j].y,
                         result.detections[j].width,
                         result.detections[j].height);
            }
        }
    }
    
    if (active_streams == 0) {
        log_info("  No active detection results found");
    }
}
