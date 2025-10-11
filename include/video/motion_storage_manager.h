#ifndef LIGHTNVR_MOTION_STORAGE_MANAGER_H
#define LIGHTNVR_MOTION_STORAGE_MANAGER_H

#include <stdint.h>
#include <stdbool.h>
#include <time.h>

/**
 * Motion Recording Storage Manager
 * 
 * This module handles automatic cleanup of old motion recordings based on
 * retention policies, disk space monitoring, and quota management.
 */

/**
 * Storage statistics for motion recordings
 */
typedef struct {
    uint64_t total_recordings;      // Total number of recordings
    uint64_t total_size_bytes;      // Total size in bytes
    uint64_t oldest_recording;      // Timestamp of oldest recording
    uint64_t newest_recording;      // Timestamp of newest recording
    uint64_t disk_space_available;  // Available disk space in bytes
    uint64_t disk_space_total;      // Total disk space in bytes
} motion_storage_stats_t;

/**
 * Initialize the motion storage manager
 * Starts the cleanup thread that periodically checks and cleans old recordings
 * 
 * @return 0 on success, non-zero on failure
 */
int init_motion_storage_manager(void);

/**
 * Shutdown the motion storage manager
 * Stops the cleanup thread and releases resources
 */
void shutdown_motion_storage_manager(void);

/**
 * Perform immediate cleanup of old recordings for a stream
 * 
 * @param stream_name Name of the stream (NULL for all streams)
 * @param retention_days Number of days to keep recordings
 * @return Number of recordings deleted, or -1 on error
 */
int cleanup_old_recordings(const char *stream_name, int retention_days);

/**
 * Perform cleanup based on disk space quota
 * Deletes oldest recordings until disk usage is below the quota
 * 
 * @param stream_name Name of the stream (NULL for all streams)
 * @param max_size_mb Maximum size in megabytes
 * @return Number of recordings deleted, or -1 on error
 */
int cleanup_by_quota(const char *stream_name, uint64_t max_size_mb);

/**
 * Get storage statistics for motion recordings
 * 
 * @param stream_name Name of the stream (NULL for all streams)
 * @param stats Output: storage statistics
 * @return 0 on success, non-zero on failure
 */
int get_motion_storage_stats(const char *stream_name, motion_storage_stats_t *stats);

/**
 * Delete a specific motion recording file
 * Removes the file from disk and database
 * 
 * @param file_path Path to the recording file
 * @return 0 on success, non-zero on failure
 */
int delete_motion_recording(const char *file_path);

/**
 * Verify and cleanup orphaned recordings
 * Removes database entries for files that no longer exist on disk
 * 
 * @param stream_name Name of the stream (NULL for all streams)
 * @return Number of orphaned entries removed, or -1 on error
 */
int cleanup_orphaned_recordings(const char *stream_name);

/**
 * Set cleanup interval in seconds
 * Default is 3600 seconds (1 hour)
 * 
 * @param interval_seconds Cleanup interval in seconds
 */
void set_cleanup_interval(int interval_seconds);

/**
 * Trigger immediate cleanup check
 * Forces the cleanup thread to run immediately
 */
void trigger_cleanup_now(void);

#endif /* LIGHTNVR_MOTION_STORAGE_MANAGER_H */

