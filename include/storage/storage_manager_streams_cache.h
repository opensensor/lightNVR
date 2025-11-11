/**
 * @file storage_manager_streams_cache.h
 * @brief Header for stream-specific storage management caching functions
 */

#ifndef LIGHTNVR_STORAGE_MANAGER_STREAMS_CACHE_H
#define LIGHTNVR_STORAGE_MANAGER_STREAMS_CACHE_H

#include <stdint.h>
#include "storage/storage_manager_streams.h"
#include <cjson/cJSON.h>

/**
 * Initialize the storage manager streams cache
 *
 * @param cache_ttl_seconds Time-to-live for cache entries in seconds
 * @return 0 on success, -1 on error
 */
int init_storage_manager_streams_cache(int cache_ttl_seconds);

/**
 * Shutdown the storage manager streams cache
 */
void shutdown_storage_manager_streams_cache(void);

/**
 * Get cached stream storage usage
 *
 * @param stream_info Pointer to array that will be allocated and filled with stream storage information
 * @param force_refresh Force a refresh of the cache
 * @return Number of streams found, or -1 on error
 */
int get_cached_stream_storage_usage(stream_storage_info_t **stream_info, int force_refresh);

/**
 * Add cached stream storage usage to JSON object
 *
 * @param json_obj JSON object to add stream storage usage to
 * @param force_refresh Force a refresh of the cache
 * @return 0 on success, -1 on error
 */
int add_cached_stream_storage_usage_to_json(cJSON *json_obj, int force_refresh);

/**
 * Invalidate the stream storage usage cache
 *
 * @return 0 on success, -1 on error
 */
int invalidate_stream_storage_cache(void);

// Cache priming is now handled by the storage manager thread

/**
 * Force a refresh of the cache
 *
 * @return 0 on success, -1 on error
 */
int force_refresh_cache(void);

#endif // LIGHTNVR_STORAGE_MANAGER_STREAMS_CACHE_H
