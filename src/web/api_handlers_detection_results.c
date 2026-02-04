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
#include "core/mqtt_client.h"
#include "video/detection.h"
#include "video/detection_result.h"
#include "video/stream_manager.h"
#include "database/database_manager.h"

// Maximum age of detections to return (in seconds)
// For live view, we want to show only recent detections (5 seconds)
// This prevents detection boxes from being displayed for too long after they occur
#define MAX_DETECTION_AGE 5
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
    
    log_debug("Storing detection results for stream '%s': %d detections", stream_name, result->count);
    
    // Store in database
    time_t timestamp = time(NULL);
    int ret = store_detections_in_db(stream_name, result, timestamp);

    if (ret != 0) {
        log_error("Failed to store detections in database for stream '%s'", stream_name);
        return;
    }

    // Publish to MQTT if enabled
    if (result->count > 0) {
        int mqtt_ret = mqtt_publish_detection(stream_name, result, timestamp);
        if (mqtt_ret != 0) {
            log_debug("MQTT publish skipped or failed for stream '%s'", stream_name);
        }
    }

    // Log the stored detections
    for (int i = 0; i < result->count; i++) {
        log_debug("  Detection %d: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]",
                i, result->detections[i].label,
                result->detections[i].confidence * 100.0f,
                result->detections[i].x,
                result->detections[i].y,
                result->detections[i].width,
                result->detections[i].height);
    }

    log_debug("Successfully stored %d detections in database for stream '%s'", result->count, stream_name);
}

/**
 * Debug function to dump current detection results
 */
void debug_dump_detection_results(void) {
    log_debug("DEBUG: Current detection results (from database):");

    // Get all stream names
    stream_config_t streams[MAX_STREAMS];
    int stream_count = get_all_stream_configs(streams, MAX_STREAMS);

    if (stream_count <= 0) {
        log_debug("  No streams found");
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

            log_debug("  Stream '%s', %d detections", streams[i].name, result.count);

            for (int j = 0; j < result.count; j++) {
                log_debug("    Detection %d: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]",
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
        log_debug("  No active detection results found");
    }
}
