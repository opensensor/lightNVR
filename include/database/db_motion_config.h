#ifndef LIGHTNVR_DB_MOTION_CONFIG_H
#define LIGHTNVR_DB_MOTION_CONFIG_H

#include <stdint.h>
#include <stdbool.h>
#include "video/onvif_motion_recording.h"

/**
 * Motion Recording Configuration Database Module
 * 
 * This module handles database operations for motion recording configuration,
 * including per-camera settings, buffer configuration, and retention policies.
 */

/**
 * Initialize the motion recording configuration table
 * Creates the table if it doesn't exist
 * 
 * @return 0 on success, non-zero on failure
 */
int init_motion_config_table(void);

/**
 * Save motion recording configuration for a stream
 * 
 * @param stream_name Name of the stream
 * @param config Configuration to save
 * @return 0 on success, non-zero on failure
 */
int save_motion_config(const char *stream_name, const motion_recording_config_t *config);

/**
 * Load motion recording configuration for a stream
 * 
 * @param stream_name Name of the stream
 * @param config Configuration structure to fill
 * @return 0 on success, non-zero on failure (including not found)
 */
int load_motion_config(const char *stream_name, motion_recording_config_t *config);

/**
 * Update motion recording configuration for a stream
 * 
 * @param stream_name Name of the stream
 * @param config Updated configuration
 * @return 0 on success, non-zero on failure
 */
int update_motion_config(const char *stream_name, const motion_recording_config_t *config);

/**
 * Delete motion recording configuration for a stream
 * 
 * @param stream_name Name of the stream
 * @return 0 on success, non-zero on failure
 */
int delete_motion_config(const char *stream_name);

/**
 * Load all motion recording configurations
 * 
 * @param configs Array to fill with configurations
 * @param stream_names Array to fill with stream names
 * @param max_count Maximum number of configurations to load
 * @return Number of configurations loaded, or -1 on error
 */
int load_all_motion_configs(motion_recording_config_t *configs, char stream_names[][256], int max_count);

/**
 * Check if motion recording is enabled for a stream in the database
 * 
 * @param stream_name Name of the stream
 * @return 1 if enabled, 0 if disabled, -1 on error
 */
int is_motion_recording_enabled_in_db(const char *stream_name);

/**
 * Get motion recording statistics from database
 * 
 * @param stream_name Name of the stream
 * @param total_recordings Output: total number of motion recordings
 * @param total_size_bytes Output: total size of motion recordings in bytes
 * @param oldest_recording Output: timestamp of oldest recording
 * @param newest_recording Output: timestamp of newest recording
 * @return 0 on success, non-zero on failure
 */
int get_motion_recording_db_stats(const char *stream_name, 
                                   uint64_t *total_recordings,
                                   uint64_t *total_size_bytes,
                                   time_t *oldest_recording,
                                   time_t *newest_recording);

/**
 * Get list of motion recordings for a stream
 * 
 * @param stream_name Name of the stream
 * @param start_time Start time filter (0 for no filter)
 * @param end_time End time filter (0 for no filter)
 * @param paths Array to fill with file paths
 * @param timestamps Array to fill with timestamps
 * @param sizes Array to fill with file sizes
 * @param max_count Maximum number of recordings to return
 * @return Number of recordings found, or -1 on error
 */
int get_motion_recordings_list(const char *stream_name,
                               time_t start_time,
                               time_t end_time,
                               char paths[][512],
                               time_t *timestamps,
                               uint64_t *sizes,
                               int max_count);

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

/**
 * Mark a motion recording as complete in the database
 * 
 * @param file_path Path to the recording file
 * @param end_time End time of the recording
 * @param size_bytes Size of the recording file
 * @return 0 on success, non-zero on failure
 */
int mark_motion_recording_complete(const char *file_path, time_t end_time, uint64_t size_bytes);

/**
 * Add a motion recording to the database
 * 
 * @param stream_name Name of the stream
 * @param file_path Path to the recording file
 * @param start_time Start time of the recording
 * @param width Video width
 * @param height Video height
 * @param fps Frames per second
 * @param codec Video codec
 * @return Recording ID on success, 0 on failure
 */
uint64_t add_motion_recording(const char *stream_name,
                              const char *file_path,
                              time_t start_time,
                              int width,
                              int height,
                              int fps,
                              const char *codec);

#endif /* LIGHTNVR_DB_MOTION_CONFIG_H */

