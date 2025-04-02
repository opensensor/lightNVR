/**
 * @file storage_manager_streams.h
 * @brief Header for stream-specific storage management functions
 */

#ifndef LIGHTNVR_STORAGE_MANAGER_STREAMS_H
#define LIGHTNVR_STORAGE_MANAGER_STREAMS_H

#include "../../external/cjson/cJSON.h"

/**
 * Add stream storage usage to JSON object
 * 
 * @param json_obj JSON object to add stream storage usage to
 * @return 0 on success, -1 on error
 */
int add_stream_storage_usage_to_json(cJSON *json_obj);

#endif // LIGHTNVR_STORAGE_MANAGER_STREAMS_H
