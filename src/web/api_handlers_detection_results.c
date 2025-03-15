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

// Structure to store detection results for each stream
typedef struct {
    char stream_name[MAX_STREAM_NAME];
    detection_result_t result;
    time_t timestamp;
    pthread_mutex_t mutex;
} stream_detection_result_t;

// Array to store detection results for all streams
static stream_detection_result_t detection_results[MAX_STREAMS];
static pthread_mutex_t detection_results_mutex = PTHREAD_MUTEX_INITIALIZER;
static bool detection_results_initialized = false;

/**
 * Initialize detection results storage
 */
void init_detection_results(void) {
    if (detection_results_initialized) {
        return;
    }
    
    pthread_mutex_lock(&detection_results_mutex);
    
    for (int i = 0; i < MAX_STREAMS; i++) {
        memset(&detection_results[i], 0, sizeof(stream_detection_result_t));
        pthread_mutex_init(&detection_results[i].mutex, NULL);
    }
    
    detection_results_initialized = true;
    pthread_mutex_unlock(&detection_results_mutex);
    
    log_info("Detection results storage initialized");
}

/**
 * Store detection result for a stream
 */
void store_detection_result(const char *stream_name, const detection_result_t *result) {
    if (!stream_name || !result || !detection_results_initialized) {
        return;
    }
    
    pthread_mutex_lock(&detection_results_mutex);
    
    // Find the slot for this stream or an empty slot
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (detection_results[i].stream_name[0] == '\0') {
            if (slot == -1) {
                slot = i;
            }
        } else if (strcmp(detection_results[i].stream_name, stream_name) == 0) {
            slot = i;
            break;
        }
    }
    
    if (slot == -1) {
        // No available slots
        pthread_mutex_unlock(&detection_results_mutex);
        log_error("No available slots for detection results");
        return;
    }
    
    pthread_mutex_lock(&detection_results[slot].mutex);
    
    // Store stream name if this is a new entry
    if (detection_results[slot].stream_name[0] == '\0') {
        strncpy(detection_results[slot].stream_name, stream_name, MAX_STREAM_NAME - 1);
        detection_results[slot].stream_name[MAX_STREAM_NAME - 1] = '\0';
    }
    
    // Store detection result
    memcpy(&detection_results[slot].result, result, sizeof(detection_result_t));
    
    // Update timestamp
    detection_results[slot].timestamp = time(NULL);
    
    pthread_mutex_unlock(&detection_results[slot].mutex);
    pthread_mutex_unlock(&detection_results_mutex);
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
    
    // Find detection results for this stream
    pthread_mutex_lock(&detection_results_mutex);
    
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (detection_results[i].stream_name[0] != '\0' && 
            strcmp(detection_results[i].stream_name, stream_name) == 0) {
            slot = i;
            break;
        }
    }
    
    if (slot == -1) {
        // No detection results for this stream
        pthread_mutex_unlock(&detection_results_mutex);
        
        // Create empty response JSON
        char json[256];
        snprintf(json, sizeof(json), "{\"stream\":\"%s\",\"detections\":[],\"timestamp\":0}", stream_name);
        create_json_response(response, 200, json);
        return;
    }
    
    pthread_mutex_lock(&detection_results[slot].mutex);
    
    // Build JSON response with detection results
    char json[4096] = "{";
    
    // Add stream name
    char stream_json[MAX_STREAM_NAME + 32];
    snprintf(stream_json, sizeof(stream_json), "\"stream\":\"%s\",", stream_name);
    strcat(json, stream_json);
    
    // Add timestamp
    char timestamp_json[64];
    snprintf(timestamp_json, sizeof(timestamp_json), "\"timestamp\":%lld,", 
             (long long)detection_results[slot].timestamp);
    strcat(json, timestamp_json);
    
    // Add detections array
    strcat(json, "\"detections\":[");
    
    // Add each detection
    for (int i = 0; i < detection_results[slot].result.count; i++) {
        if (i > 0) {
            strcat(json, ",");
        }
        
        char detection_json[512];
        snprintf(detection_json, sizeof(detection_json),
                "{\"label\":\"%s\",\"confidence\":%.2f,\"x\":%.4f,\"y\":%.4f,\"width\":%.4f,\"height\":%.4f}",
                detection_results[slot].result.detections[i].label,
                detection_results[slot].result.detections[i].confidence,
                detection_results[slot].result.detections[i].x,
                detection_results[slot].result.detections[i].y,
                detection_results[slot].result.detections[i].width,
                detection_results[slot].result.detections[i].height);
        
        strcat(json, detection_json);
    }
    
    // Close JSON
    strcat(json, "]}");
    
    pthread_mutex_unlock(&detection_results[slot].mutex);
    pthread_mutex_unlock(&detection_results_mutex);
    
    // Create response
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
    
    log_info("Detection results API handlers registered");
}
