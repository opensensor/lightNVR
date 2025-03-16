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
        log_error("Invalid parameters for store_detection_result: stream_name=%p, result=%p, initialized=%d",
                 stream_name, result, detection_results_initialized);
        return;
    }
    
    log_info("Storing detection results for stream '%s': %d detections", stream_name, result->count);
    
    // Force initialization if not already done
    if (!detection_results_initialized) {
        log_warn("Detection results not initialized, initializing now");
        init_detection_results();
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
        log_info("Created new detection results slot %d for stream '%s'", slot, stream_name);
    }
    
    // Store detection result
    memcpy(&detection_results[slot].result, result, sizeof(detection_result_t));
    
    // Update timestamp
    detection_results[slot].timestamp = time(NULL);
    
    // Log the stored detections
    log_info("Stored %d detections for stream '%s' in slot %d", 
             detection_results[slot].result.count, stream_name, slot);
    
    for (int i = 0; i < detection_results[slot].result.count; i++) {
        log_info("  Detection %d: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]",
                i, detection_results[slot].result.detections[i].label,
                detection_results[slot].result.detections[i].confidence * 100.0f,
                detection_results[slot].result.detections[i].x,
                detection_results[slot].result.detections[i].y,
                detection_results[slot].result.detections[i].width,
                detection_results[slot].result.detections[i].height);
    }
    
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
    
    // Dump all detection results to help diagnose the issue
    log_info("API: Dumping all detection results before search:");
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (detection_results[i].stream_name[0] != '\0') {
            log_info("  Slot %d: Stream '%s', %d detections", 
                     i, detection_results[i].stream_name, detection_results[i].result.count);
        }
    }
    
    int slot = -1;
    int newest_slot = -1;
    time_t newest_time = 0;
    
    // First try to find an exact match
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (detection_results[i].stream_name[0] != '\0') {
            // Keep track of the newest detection result for fallback
            if (detection_results[i].timestamp > newest_time && detection_results[i].result.count > 0) {
                newest_time = detection_results[i].timestamp;
                newest_slot = i;
            }
            
            // Check for exact match
            if (strcmp(detection_results[i].stream_name, stream_name) == 0) {
                slot = i;
                break;
            }
        }
    }
    
    // Debug logging to help diagnose the issue
    log_info("API: Looking for detection results for stream '%s', found: %s (slot: %d)", 
             stream_name, (slot != -1) ? "yes" : "no", slot);
    
    // If no exact match but we have a recent detection with results, use that as fallback
    if (slot == -1 && newest_slot != -1) {
        log_warn("API: No exact match for stream '%s', using newest detection from stream '%s' as fallback",
                stream_name, detection_results[newest_slot].stream_name);
        slot = newest_slot;
    }
    
    if (slot == -1) {
        // No detection results for this stream
        pthread_mutex_unlock(&detection_results_mutex);
        
        // Create empty response JSON
        char json[256];
        snprintf(json, sizeof(json), "{\"stream\":\"%s\",\"detections\":[],\"timestamp\":0}", stream_name);
        create_json_response(response, 200, json);
        log_warn("API: No detection results found for stream '%s', returning empty array", stream_name);
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

/**
 * Debug function to dump current detection results
 */
void debug_dump_detection_results(void) {
    pthread_mutex_lock(&detection_results_mutex);
    
    log_info("DEBUG: Current detection results:");
    int active_slots = 0;
    
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (detection_results[i].stream_name[0] != '\0') {
            active_slots++;
            pthread_mutex_lock(&detection_results[i].mutex);
            
            log_info("  Slot %d: Stream '%s', %d detections, timestamp: %lld",
                     i, detection_results[i].stream_name,
                     detection_results[i].result.count,
                     (long long)detection_results[i].timestamp);
            
            for (int j = 0; j < detection_results[i].result.count; j++) {
                log_info("    Detection %d: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]",
                         j, detection_results[i].result.detections[j].label,
                         detection_results[i].result.detections[j].confidence * 100.0f,
                         detection_results[i].result.detections[j].x,
                         detection_results[i].result.detections[j].y,
                         detection_results[i].result.detections[j].width,
                         detection_results[i].result.detections[j].height);
            }
            
            pthread_mutex_unlock(&detection_results[i].mutex);
        }
    }
    
    if (active_slots == 0) {
        log_info("  No active detection results found");
    }
    
    pthread_mutex_unlock(&detection_results_mutex);
}
