/**
 * @file storage_manager_streams_cache.c
 * @brief Implementation of stream-specific storage management caching functions
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <time.h>
#include <errno.h>
#include <stdbool.h>
#include <unistd.h>

#include "storage/storage_manager_streams_cache.h"
#include "storage/storage_manager.h"
#include "core/logger.h"
#include "core/config.h"
#include <cjson/cJSON.h>

// Forward declarations for functions from storage_manager_streams.c
extern int get_stream_storage_usage(const char *storage_path, stream_storage_info_t *stream_info, int max_streams);
extern int get_all_stream_storage_usage(stream_storage_info_t **stream_info);

// Cache structure
typedef struct {
    stream_storage_info_t *stream_info;
    int stream_count;
    time_t last_update;
    int ttl_seconds;
    pthread_mutex_t mutex;
    int initialized;
} stream_storage_cache_t;

// Global cache instance
static stream_storage_cache_t cache = {
    .stream_info = NULL,
    .stream_count = 0,
    .last_update = 0,
    .ttl_seconds = 1800, // Default TTL: 30 minutes
    .initialized = 0
};

/**
 * Initialize the storage manager streams cache
 *
 * @param cache_ttl_seconds Time-to-live for cache entries in seconds
 * @return 0 on success, -1 on error
 */
int init_storage_manager_streams_cache(int cache_ttl_seconds) {
    // Initialize mutex
    if (pthread_mutex_init(&cache.mutex, NULL) != 0) {
        log_error("Failed to initialize storage manager streams cache mutex");
        return -1;
    }

    pthread_mutex_lock(&cache.mutex);

    // Set TTL (minimum 10 seconds)
    cache.ttl_seconds = (cache_ttl_seconds < 10) ? 10 : cache_ttl_seconds;
    cache.initialized = 1;

    pthread_mutex_unlock(&cache.mutex);

    log_info("Storage manager streams cache initialized with TTL: %d seconds", cache.ttl_seconds);
    return 0;
}

/**
 * Check if cache is stale and needs refreshing
 *
 * @return 1 if cache is stale, 0 if cache is valid
 */
static int is_cache_stale(void) {
    if (!cache.initialized) {
        return 1;
    }

    if (!cache.stream_info || cache.stream_count <= 0) {
        return 1;
    }

    time_t now = time(NULL);
    if (now - cache.last_update > cache.ttl_seconds) {
        return 1;
    }

    return 0;
}

/**
 * Refresh the cache with current storage usage data
 *
 * @return 0 on success, -1 on error
 */
static int refresh_cache(void) {
    if (!cache.initialized) {
        init_storage_manager_streams_cache(900);
    }

    // Get current stream storage usage
    stream_storage_info_t *stream_info = NULL;
    int stream_count = get_all_stream_storage_usage(&stream_info);

    if (stream_count <= 0 || !stream_info) {
        log_warn("No stream storage usage information available for cache refresh");
        return -1;
    }

    pthread_mutex_lock(&cache.mutex);

    // Free old cache data
    if (cache.stream_info) {
        free(cache.stream_info);
    }

    // Update cache with new data
    cache.stream_info = stream_info;
    cache.stream_count = stream_count;
    cache.last_update = time(NULL);

    pthread_mutex_unlock(&cache.mutex);

    log_debug("Storage manager streams cache refreshed with %d streams", stream_count);
    return 0;
}

/**
 * Get cached stream storage usage
 *
 * @param stream_info Pointer to array that will be allocated and filled with stream storage information
 * @param force_refresh Force a refresh of the cache
 * @return Number of streams found, or -1 on error
 */
int get_cached_stream_storage_usage(stream_storage_info_t **stream_info, int force_refresh) {
    if (!stream_info) {
        log_error("Invalid parameter for get_cached_stream_storage_usage");
        return -1;
    }

    // Initialize output parameter
    *stream_info = NULL;

    // Check if cache needs refreshing
    if (force_refresh || is_cache_stale()) {
        if (refresh_cache() != 0) {
            log_error("Failed to refresh storage manager streams cache");
            return -1;
        }
    }

    pthread_mutex_lock(&cache.mutex);

    // Check if cache is valid
    if (!cache.stream_info || cache.stream_count <= 0) {
        pthread_mutex_unlock(&cache.mutex);
        log_warn("No cached stream storage usage information available");
        return 0;
    }

    // Allocate memory for stream info array
    *stream_info = (stream_storage_info_t *)malloc(cache.stream_count * sizeof(stream_storage_info_t));
    if (!*stream_info) {
        pthread_mutex_unlock(&cache.mutex);
        log_error("Failed to allocate memory for stream storage info");
        return -1;
    }

    // Copy cached data to output array
    memcpy(*stream_info, cache.stream_info, cache.stream_count * sizeof(stream_storage_info_t));
    int stream_count = cache.stream_count;

    pthread_mutex_unlock(&cache.mutex);

    return stream_count;
}

/**
 * Add cached stream storage usage to JSON object
 *
 * @param json_obj JSON object to add stream storage usage to
 * @param force_refresh Force a refresh of the cache
 * @return 0 on success, -1 on error
 */
int add_cached_stream_storage_usage_to_json(cJSON *json_obj, int force_refresh) {
    if (!json_obj) {
        log_error("Invalid JSON object for add_cached_stream_storage_usage_to_json");
        return -1;
    }

    // Create stream storage array
    cJSON *stream_storage_array = cJSON_CreateArray();
    if (!stream_storage_array) {
        log_error("Failed to create stream storage JSON array");
        return -1;
    }

    // Check if cache is initialized
    if (!cache.initialized) {
        log_warn("Storage manager streams cache not initialized, initializing with default TTL");
        if (init_storage_manager_streams_cache(300) != 0) {
            log_error("Failed to initialize storage manager streams cache");
            // Still add the empty array to the JSON object
            cJSON_AddItemToObject(json_obj, "streamStorage", stream_storage_array);
            return 0;
        }
        // Force refresh since we just initialized the cache
        force_refresh = 1;
    }

    // Get cached stream storage usage
    stream_storage_info_t *stream_info = NULL;
    int stream_count = get_cached_stream_storage_usage(&stream_info, force_refresh);

    if (stream_count <= 0 || !stream_info) {
        log_warn("No cached stream storage usage information available");

        // Try to get the information directly without caching
        log_info("Attempting to get stream storage usage directly");
        stream_info = NULL;
        stream_count = get_all_stream_storage_usage(&stream_info);

        if (stream_count <= 0 || !stream_info) {
            log_warn("No stream storage usage information available");
            // Still add the empty array to the JSON object
            cJSON_AddItemToObject(json_obj, "streamStorage", stream_storage_array);
            return 0;
        }

        log_info("Successfully retrieved %d streams directly", stream_count);
    }

    // Add stream storage info to array
    for (int i = 0; i < stream_count; i++) {
        cJSON *stream_obj = cJSON_CreateObject();
        if (stream_obj) {
            cJSON_AddStringToObject(stream_obj, "name", stream_info[i].name);
            cJSON_AddNumberToObject(stream_obj, "size", stream_info[i].size_bytes);
            cJSON_AddNumberToObject(stream_obj, "count", stream_info[i].recording_count);

            cJSON_AddItemToArray(stream_storage_array, stream_obj);
        }
    }

    // Add stream storage array to JSON object
    cJSON_AddItemToObject(json_obj, "streamStorage", stream_storage_array);

    // Add cache metadata
    cJSON *cache_meta = cJSON_CreateObject();
    if (cache_meta) {
        cJSON_AddNumberToObject(cache_meta, "lastUpdate", cache.last_update);
        cJSON_AddNumberToObject(cache_meta, "ttlSeconds", cache.ttl_seconds);
        cJSON_AddBoolToObject(cache_meta, "isCached", true);

        cJSON_AddItemToObject(json_obj, "streamStorageCache", cache_meta);
    }

    // Free memory
    free(stream_info);

    return 0;
}

/**
 * Invalidate the stream storage usage cache
 *
 * @return 0 on success, -1 on error
 */
int invalidate_stream_storage_cache(void) {
    if (!cache.initialized) {
        log_error("Storage manager streams cache not initialized");
        return -1;
    }

    pthread_mutex_lock(&cache.mutex);

    // Free cached data
    if (cache.stream_info) {
        free(cache.stream_info);
        cache.stream_info = NULL;
    }

    cache.stream_count = 0;
    cache.last_update = 0;

    pthread_mutex_unlock(&cache.mutex);

    log_info("Storage manager streams cache invalidated");
    return 0;
}

/**
 * Force a refresh of the cache
 *
 * @return 0 on success, -1 on error
 */
int force_refresh_cache(void) {
    return refresh_cache();
}

