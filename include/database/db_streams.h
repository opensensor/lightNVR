#ifndef LIGHTNVR_DB_STREAMS_H
#define LIGHTNVR_DB_STREAMS_H

#include <stdint.h>
#include "core/config.h"

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
 * Delete a stream configuration from the database
 * 
 * @param name Stream name to delete
 * @return 0 on success, non-zero on failure
 */
int delete_stream_config(const char *name);

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

#endif // LIGHTNVR_DB_STREAMS_H
