#ifndef LIGHTNVR_MOTION_STORAGE_MANAGER_H
#define LIGHTNVR_MOTION_STORAGE_MANAGER_H

#include <stdint.h>
#include <stdbool.h>
#include <time.h>

/**
 * Motion Recording Storage Utilities
 *
 * Utility functions for motion recording file deletion, retention cleanup,
 * and storage statistics. The periodic cleanup thread has been removed;
 * all scheduled cleanup is now handled by the unified storage controller
 * in storage_manager.c.
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
 * Perform immediate cleanup of old recordings for a stream
 *
 * @param stream_name Name of the stream (NULL for all streams)
 * @param retention_days Number of days to keep recordings
 * @return Number of recordings deleted, or -1 on error
 */
int cleanup_old_recordings(const char *stream_name, int retention_days);

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
 * Removes the file from disk
 *
 * @param file_path Path to the recording file
 * @return 0 on success, non-zero on failure
 */
int delete_motion_recording(const char *file_path);

#endif /* LIGHTNVR_MOTION_STORAGE_MANAGER_H */
