/**
 * @file storage_manager_streams.h
 * @brief Header for stream-specific storage management functions
 */

#ifndef LIGHTNVR_STORAGE_MANAGER_STREAMS_H
#define LIGHTNVR_STORAGE_MANAGER_STREAMS_H

#include <cjson/cJSON.h>

/**
 * Stream storage usage information
 */
typedef struct {
    char name[64];
    unsigned long size_bytes;
    int recording_count;
} stream_storage_info_t;

/**
 * Get storage usage per stream
 * 
 * @param storage_path Base storage path
 * @param stream_info Array to fill with stream storage information
 * @param max_streams Maximum number of streams to return
 * @return Number of streams found, or -1 on error
 */
int get_stream_storage_usage(const char *storage_path, stream_storage_info_t *stream_info, int max_streams);

/**
 * Get storage usage for all streams
 * 
 * @param stream_info Pointer to array that will be allocated and filled with stream storage information
 * @return Number of streams found, or -1 on error
 */
int get_all_stream_storage_usage(stream_storage_info_t **stream_info);

/**
 * Add stream storage usage to JSON object
 * 
 * @param json_obj JSON object to add stream storage usage to
 * @return 0 on success, -1 on error
 */
int add_stream_storage_usage_to_json(cJSON *json_obj);

#endif // LIGHTNVR_STORAGE_MANAGER_STREAMS_H
