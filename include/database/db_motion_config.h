#ifndef LIGHTNVR_DB_MOTION_CONFIG_H
#define LIGHTNVR_DB_MOTION_CONFIG_H

#include <stdint.h>
#include <stdbool.h>
#include "video/cross_stream_motion_trigger.h"

/**
 * Motion Recording Configuration Database Module
 *
 * This module handles database operations for motion recording configuration,
 * including per-camera settings, buffer configuration, and retention policies.
 *
 * Note: Tables are created via SQL migration 0020_add_motion_recording_config.sql
 */

/**
 * Delete motion recording configuration for a stream
 * 
 * @param stream_name Name of the stream
 * @return 0 on success, non-zero on failure
 */
int delete_motion_config(const char *stream_name);

/**
 * Delete old motion recordings based on retention policy
 * 
 * @param stream_name Name of the stream (NULL for all streams)
 * @param retention_days Number of days to keep recordings
 * @return Number of recordings deleted, or -1 on error
 */
int cleanup_old_motion_recordings(const char *stream_name, int retention_days);

/**
 * Get total disk space used by motion recordings
 * 
 * @param stream_name Name of the stream (NULL for all streams)
 * @return Total size in bytes, or -1 on error
 */
int64_t get_motion_recordings_disk_usage(const char *stream_name);

#endif /* LIGHTNVR_DB_MOTION_CONFIG_H */

