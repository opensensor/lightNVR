#ifndef LIGHTNVR_STORAGE_MANAGER_H
#define LIGHTNVR_STORAGE_MANAGER_H

#include <stdint.h>
#include <stdbool.h>
#include <time.h>

// Recording file information structure
typedef struct {
    char path[256];
    char stream_name[64];
    time_t start_time;
    time_t end_time;
    uint64_t size_bytes;
    char codec[16];
    int width;
    int height;
    int fps;
    bool is_complete; // True if file was properly finalized
} recording_info_t;

// Storage statistics structure
typedef struct {
    uint64_t total_space;
    uint64_t used_space;
    uint64_t free_space;
    uint64_t reserved_space;
    uint64_t total_recordings;
    uint64_t total_recording_bytes;
    uint64_t oldest_recording_time;
    uint64_t newest_recording_time;
} storage_stats_t;

/**
 * Initialize the storage manager
 *
 * @param storage_path Base path for storing recordings
 * @param max_size Maximum storage size in bytes (0 for unlimited)
 * @return 0 on success, non-zero on failure
 */
int init_storage_manager(const char *storage_path, uint64_t max_size);

/**
 * Shutdown the storage manager
 */
void shutdown_storage_manager(void);

/**
 * Open a new recording file
 *
 * @param stream_name Name of the stream
 * @param codec Codec name (e.g., "h264")
 * @param width Video width
 * @param height Video height
 * @param fps Frames per second
 * @return File handle on success, NULL on failure
 */
void* open_recording_file(const char *stream_name, const char *codec, int width, int height, int fps);

/**
 * Write frame data to a recording file
 *
 * @param handle File handle
 * @param data Frame data
 * @param size Size of frame data in bytes
 * @param timestamp Frame timestamp
 * @param is_key_frame True if this is a key frame
 * @return 0 on success, non-zero on failure
 */
int write_frame_to_recording(void *handle, const uint8_t *data, size_t size,
                            uint64_t timestamp, bool is_key_frame);

/**
 * Close a recording file
 *
 * @param handle File handle
 * @return 0 on success, non-zero on failure
 */
int close_recording_file(void *handle);

/**
 * Get storage statistics
 *
 * @param stats Pointer to statistics structure to fill
 * @return 0 on success, non-zero on failure
 */
int get_storage_stats(storage_stats_t *stats);

/**
 * List recordings for a stream
 *
 * @param stream_name Name of the stream (NULL for all streams)
 * @param start_time Start time filter (0 for no filter)
 * @param end_time End time filter (0 for no filter)
 * @param recordings Array to fill with recording information
 * @param max_count Maximum number of recordings to return
 * @return Number of recordings found, or -1 on error
 */
int list_recordings(const char *stream_name, time_t start_time, time_t end_time,
                   recording_info_t *recordings, int max_count);

/**
 * Delete a recording
 *
 * @param path Path to the recording file
 * @return 0 on success, non-zero on failure
 */
int delete_recording(const char *path);

/**
 * Apply retention policy (delete oldest recordings if storage limit is reached)
 *
 * @return Number of recordings deleted, or -1 on error
 */
int apply_retention_policy(void);

/**
 * Set maximum storage size
 *
 * @param max_size Maximum storage size in bytes (0 for unlimited)
 * @return 0 on success, non-zero on failure
 */
int set_max_storage_size(uint64_t max_size);

/**
 * Set retention days
 *
 * @param days Number of days to keep recordings (0 for unlimited)
 * @return 0 on success, non-zero on failure
 */
int set_retention_days(int days);

/**
 * Check if storage is available
 *
 * @return True if storage is available, false otherwise
 */
bool is_storage_available(void);

/**
 * Get path to a recording file
 *
 * @param stream_name Name of the stream
 * @param timestamp Timestamp for the recording
 * @param path Buffer to fill with the path
 * @param path_size Size of the path buffer
 * @return 0 on success, non-zero on failure
 */
int get_recording_path(const char *stream_name, time_t timestamp, char *path, size_t path_size);

/**
 * Create a directory for a stream if it doesn't exist
 *
 * @param stream_name Name of the stream
 * @return 0 on success, non-zero on failure
 */
int create_stream_directory(const char *stream_name);

/**
 * Check disk space and ensure minimum free space is available
 *
 * @param min_free_bytes Minimum free space required in bytes
 * @return True if enough space is available, false otherwise
 */
bool ensure_disk_space(uint64_t min_free_bytes);

/**
 * Start the storage manager thread
 *
 * This thread periodically performs storage management tasks:
 * - Applies the retention policy to delete old recordings based on age and storage limits
 * - Refreshes the storage cache to ensure API responses are fast
 *
 * @param interval_seconds How often to run storage management tasks (in seconds)
 * @return 0 on success, non-zero on failure
 */
int start_storage_manager_thread(int interval_seconds);

/**
 * Stop the storage manager thread
 *
 * @return 0 on success, non-zero on failure
 */
int stop_storage_manager_thread(void);

#endif // LIGHTNVR_STORAGE_MANAGER_H
