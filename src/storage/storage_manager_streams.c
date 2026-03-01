/**
 * @file storage_manager_streams.c
 * @brief Implementation of stream-specific storage management functions
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

#include "storage/storage_manager.h"
#include "storage/storage_manager_streams.h"
#include "database/db_recordings.h"
#include "database/db_streams.h"
#include "core/logger.h"
#include "core/config.h"
#include <cjson/cJSON.h>

/**
 * Get storage usage per stream (DB-driven, no subprocess calls)
 *
 * Uses database aggregation queries instead of blocking popen("du -sb")
 * subprocess calls. This is O(1) per stream vs O(n) filesystem walk,
 * and does not block HTTP handler threads.
 *
 * @param storage_path Base storage path (kept for API compatibility, unused)
 * @param stream_info Array to fill with stream storage information
 * @param max_streams Maximum number of streams to return
 * @return Number of streams found, or -1 on error
 */
int get_stream_storage_usage(const char *storage_path, stream_storage_info_t *stream_info, int max_streams) {
    (void)storage_path; // Unused - stats come from DB now

    if (!stream_info || max_streams <= 0) {
        log_error("Invalid parameters for get_stream_storage_usage");
        return -1;
    }

    // Get all stream names from database
    char stream_names[MAX_STREAMS][64];
    int name_count = get_all_stream_names(stream_names, MAX_STREAMS < max_streams ? MAX_STREAMS : max_streams);

    if (name_count <= 0) {
        log_debug("No streams found in database for storage usage");
        return 0;
    }

    int stream_count = 0;
    for (int i = 0; i < name_count && stream_count < max_streams; i++) {
        // Get total bytes from DB (SUM of size_bytes for completed recordings)
        int64_t total_bytes = get_stream_storage_bytes(stream_names[i]);
        if (total_bytes < 0) {
            total_bytes = 0; // Treat errors as zero
        }

        // Get recording count from DB
        int rec_count = get_recording_count(0, 0, stream_names[i], 0, NULL);
        if (rec_count < 0) {
            rec_count = 0; // Treat errors as zero
        }

        // Only include streams that have recordings or storage
        // (include all to match previous behavior of including dirs with HLS segments)
        strncpy(stream_info[stream_count].name, stream_names[i],
                sizeof(stream_info[stream_count].name) - 1);
        stream_info[stream_count].name[sizeof(stream_info[stream_count].name) - 1] = '\0';
        stream_info[stream_count].size_bytes = (unsigned long)total_bytes;
        stream_info[stream_count].recording_count = rec_count;
        stream_count++;
    }

    return stream_count;
}

/**
 * Get storage usage for all streams (DB-driven)
 *
 * Uses count_stream_configs() to determine allocation size, then
 * delegates to get_stream_storage_usage() for DB-driven stats.
 * No filesystem directory scanning required.
 *
 * @param stream_info Pointer to array that will be allocated and filled with stream storage information
 * @return Number of streams found, or -1 on error
 */
int get_all_stream_storage_usage(stream_storage_info_t **stream_info) {
    if (!stream_info) {
        log_error("Invalid parameter for get_all_stream_storage_usage");
        return -1;
    }

    // Get stream count from database (no filesystem scan)
    int stream_count = count_stream_configs();
    if (stream_count <= 0) {
        *stream_info = NULL;
        return stream_count == 0 ? 0 : -1;
    }

    // Allocate memory for stream info array
    *stream_info = (stream_storage_info_t *)malloc(stream_count * sizeof(stream_storage_info_t));
    if (!*stream_info) {
        log_error("Failed to allocate memory for stream storage info");
        return -1;
    }

    // Get stream storage usage from DB
    int actual_count = get_stream_storage_usage(NULL, *stream_info, stream_count);

    // If no streams found, free memory
    if (actual_count <= 0) {
        free(*stream_info);
        *stream_info = NULL;
    }

    return actual_count;
}

/**
 * Add stream storage usage to JSON object
 * 
 * @param json_obj JSON object to add stream storage usage to
 * @return 0 on success, -1 on error
 */
int add_stream_storage_usage_to_json(cJSON *json_obj) {
    if (!json_obj) {
        log_error("Invalid JSON object for add_stream_storage_usage_to_json");
        return -1;
    }
    
    // Create stream storage array
    cJSON *stream_storage_array = cJSON_CreateArray();
    if (!stream_storage_array) {
        log_error("Failed to create stream storage JSON array");
        return -1;
    }
    
    // Get stream storage usage
    stream_storage_info_t *stream_info = NULL;
    int stream_count = get_all_stream_storage_usage(&stream_info);
    
    if (stream_count <= 0 || !stream_info) {
        log_warn("No stream storage usage information available");
        // Still add the empty array to the JSON object
        cJSON_AddItemToObject(json_obj, "streamStorage", stream_storage_array);
        return 0;
    }
    
    // Add stream storage info to array
    for (int i = 0; i < stream_count; i++) {
        cJSON *stream_obj = cJSON_CreateObject();
        if (stream_obj) {
            cJSON_AddStringToObject(stream_obj, "name", stream_info[i].name);
            cJSON_AddNumberToObject(stream_obj, "size", (double)stream_info[i].size_bytes);
            cJSON_AddNumberToObject(stream_obj, "count", stream_info[i].recording_count);
            
            cJSON_AddItemToArray(stream_storage_array, stream_obj);
        }
    }
    
    // Add stream storage array to JSON object
    cJSON_AddItemToObject(json_obj, "streamStorage", stream_storage_array);
    
    // Free memory
    free(stream_info);
    
    return 0;
}
