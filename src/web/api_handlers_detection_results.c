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

#include <cjson/cJSON.h>
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
