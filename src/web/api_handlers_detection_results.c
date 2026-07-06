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
#include "web/api_handlers_detection_results.h"
#include "web/api_handlers_common.h"
#include "web/httpd_utils.h"
#define LOG_COMPONENT "DetectionAPI"
#include "core/logger.h"
#include "core/config.h"
#include "core/mqtt_client.h"
#include "video/detection.h"
#include "video/detection_result.h"
#include "video/stream_manager.h"
#include "video/zone_filter.h"
#include "database/database_manager.h"

// See api_handlers_detection.c — this constant is only used by the debug helper
// (debug_dump_detection_results) which still benefits from a wider window.
#define MAX_DETECTION_AGE 30
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

    // Make a mutable copy for filtering
    detection_result_t filtered_result;
    memcpy(&filtered_result, result, sizeof(detection_result_t));

    // Apply per-stream object include/exclude filter
    filter_detections_by_stream_objects(stream_name, &filtered_result);

    // Store in database (no recording_id linkage for direct API calls)
    time_t timestamp = time(NULL);
    int ret = store_detections_in_db(stream_name, &filtered_result, timestamp, 0);

    if (ret != 0) {
        log_error("Failed to store detections in database for stream '%s'", stream_name);
        return;
    }

    // Publish to MQTT if enabled (use filtered result)
    if (filtered_result.count > 0) {
        int mqtt_ret = mqtt_publish_detection(stream_name, &filtered_result, timestamp);
        // cppcheck-suppress knownConditionTrueFalse
        if (mqtt_ret != 0) {
            log_debug("MQTT publish skipped or failed for stream '%s'", stream_name);
        }
        mqtt_set_motion_state(stream_name, result);
    }

    // Log the stored detections
    for (int i = 0; i < filtered_result.count; i++) {
        log_debug("  Detection %d: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]",
                i, filtered_result.detections[i].label,
                filtered_result.detections[i].confidence * 100.0f,
                filtered_result.detections[i].x,
                filtered_result.detections[i].y,
                filtered_result.detections[i].width,
                filtered_result.detections[i].height);
    }

    log_debug("Successfully stored %d detections in database for stream '%s'", filtered_result.count, stream_name);
}

/**
 * Debug function to dump current detection results
 */
void debug_dump_detection_results(void) {
    log_debug("DEBUG: Current detection results (from database):");

    // Get all stream names (heap-allocated)
    int ms = g_config.max_streams > 0 ? g_config.max_streams : 32;
    stream_config_t *streams = calloc(ms, sizeof(stream_config_t));
    if (!streams) return;
    int stream_count = get_all_stream_configs(streams, ms);

    if (stream_count <= 0) {
        log_debug("  No streams found");
        free(streams);
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
    free(streams);
}

/**
 * Handle GET /api/snapshots/{stream}/{file}.jpg
 *
 * Serves detection event snapshots saved by mqtt_publish_detection() under
 * {storage_path}/snapshots/.  The MQTT detection payload references these
 * files via its snapshot_url field (issue #449).
 */
void handle_get_detection_snapshot(const http_request_t *req, http_response_t *res) {
    if (!req || !res) {
        return;
    }

    // Check authentication if enabled (same policy as recording thumbnails)
    if (g_config.web_auth_enabled) {
        user_t user;
        if (g_config.demo_mode) {
            if (!httpd_check_viewer_access(req, &user)) {
                http_response_set_json_error(res, 401, "Unauthorized");
                return;
            }
        } else {
            if (!httpd_get_authenticated_user(req, &user)) {
                http_response_set_json_error(res, 401, "Unauthorized");
                return;
            }
        }
    }

    // Extract "{stream}/{file}.jpg"
    char param_buf[512];
    if (http_request_extract_path_param(req, "/api/snapshots/",
                                        param_buf, sizeof(param_buf)) != 0) {
        http_response_set_json_error(res, 400, "Invalid request path");
        return;
    }

    char *slash = strchr(param_buf, '/');
    if (!slash || slash == param_buf || slash[1] == '\0') {
        http_response_set_json_error(res, 400, "Expected /api/snapshots/{stream}/{file}.jpg");
        return;
    }
    *slash = '\0';
    const char *stream_part = param_buf;
    const char *file_part = slash + 1;

    // Both components must be plain filenames: no traversal, no separators,
    // only characters sanitize_stream_name / the snapshot writer produce.
    for (const char *p = stream_part; *p; p++) {
        if (!isalnum((unsigned char)*p) && *p != '_' && *p != '-') {
            http_response_set_json_error(res, 400, "Invalid stream name");
            return;
        }
    }
    size_t file_len = strlen(file_part);
    if (file_len < 5 || strcmp(file_part + file_len - 4, ".jpg") != 0) {
        http_response_set_json_error(res, 400, "Invalid snapshot filename");
        return;
    }
    for (const char *p = file_part; *p; p++) {
        if (!isalnum((unsigned char)*p) && *p != '_' && *p != '-' && *p != '.') {
            http_response_set_json_error(res, 400, "Invalid snapshot filename");
            return;
        }
    }
    if (strstr(file_part, "..")) {
        http_response_set_json_error(res, 400, "Invalid snapshot filename");
        return;
    }

    char snapshot_path[MAX_PATH_LENGTH];
    snprintf(snapshot_path, sizeof(snapshot_path), "%s/snapshots/%s/%s",
             g_config.storage_path, stream_part, file_part);

    struct stat st;
    if (stat(snapshot_path, &st) != 0 || !S_ISREG(st.st_mode)) {
        http_response_set_json_error(res, 404, "Snapshot not found");
        return;
    }

    if (http_serve_file(req, res, snapshot_path, "image/jpeg",
                        "Cache-Control: private, max-age=86400\r\n") != 0) {
        http_response_set_json_error(res, 500, "Failed to serve snapshot");
    }
}
