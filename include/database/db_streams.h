#ifndef LIGHTNVR_DB_STREAMS_H
#define LIGHTNVR_DB_STREAMS_H

#include <stdint.h>
#include <stdbool.h>
#include "core/config.h"

/**
 * Stream retention configuration structure
 * Used for per-stream recording retention policies
 */
typedef struct {
    int retention_days;              // Regular recordings retention (0 = unlimited)
    int detection_retention_days;    // Detection recordings retention (0 = unlimited)
    uint64_t max_storage_mb;         // Storage quota in MB (0 = unlimited)
} stream_retention_config_t;

/**
 * Add a stream configuration to the database
 *
 * @param stream Stream configuration to add
 * @return Stream ID on success, 0 on failure
 */
uint64_t add_stream_config(const stream_config_t *stream);

/**
 * Update a stream configuration in the database
 *
 * @param name Stream name to update
 * @param stream Updated stream configuration
 * @return 0 on success, non-zero on failure
 */
int update_stream_config(const char *name, const stream_config_t *stream);

/**
 * Delete a stream configuration from the database (soft delete by disabling)
 *
 * @param name Stream name to delete
 * @return 0 on success, non-zero on failure
 */
int delete_stream_config(const char *name);

/**
 * Delete a stream configuration from the database with option for permanent deletion
 *
 * @param name Stream name to delete
 * @param permanent If true, permanently delete the stream; if false, just disable it
 * @return 0 on success, non-zero on failure
 */
int delete_stream_config_internal(const char *name, bool permanent);

/**
 * Get a stream configuration from the database
 *
 * @param name Stream name to get
 * @param stream Stream configuration to fill
 * @return 0 on success, non-zero on failure
 */
int get_stream_config_by_name(const char *name, stream_config_t *stream);

/**
 * Get all stream configurations from the database
 *
 * @param streams Array to fill with stream configurations
 * @param max_count Maximum number of streams to return
 * @return Number of streams found, or -1 on error
 */
int get_all_stream_configs(stream_config_t *streams, int max_count);

/**
 * Count the number of stream configurations in the database
 *
 * @return Number of streams, or -1 on error
 */
int count_stream_configs(void);

/**
 * Count the number of enabled stream configurations in the database
 *
 * @return Number of enabled streams, or -1 on error
 */
int get_enabled_stream_count(void);

/**
 * Check if a stream is eligible for live streaming
 *
 * @param stream_name Name of the stream to check
 * @return 1 if eligible, 0 if not eligible, -1 on error
 */
int is_stream_eligible_for_live_streaming(const char *stream_name);

/**
 * Get retention configuration for a stream
 *
 * @param stream_name Stream name
 * @param config Pointer to retention config structure to fill
 * @return 0 on success, non-zero on failure
 */
int get_stream_retention_config(const char *stream_name, stream_retention_config_t *config);

/**
 * Set retention configuration for a stream
 *
 * @param stream_name Stream name
 * @param config Pointer to retention config structure with new values
 * @return 0 on success, non-zero on failure
 */
int set_stream_retention_config(const char *stream_name, const stream_retention_config_t *config);

/**
 * Get all stream names for retention policy processing
 *
 * @param names Array of stream name buffers (each should be at least 64 chars)
 * @param max_count Maximum number of stream names to return
 * @return Number of streams found, or -1 on error
 */
int get_all_stream_names(char names[][64], int max_count);

/**
 * Get storage usage for a stream in bytes
 *
 * @param stream_name Stream name
 * @return Total size in bytes, or 0 on error
 */
uint64_t get_stream_storage_usage_db(const char *stream_name);

#endif // LIGHTNVR_DB_STREAMS_H
