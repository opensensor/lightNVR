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
 * Disk pressure levels for proactive storage management
 * Thresholds based on percentage of free space remaining
 */
typedef enum {
    DISK_PRESSURE_NORMAL   = 0,  // >20% free - normal operations
    DISK_PRESSURE_WARNING  = 1,  // 10-20% free - increase cleanup frequency
    DISK_PRESSURE_CRITICAL = 2,  // 5-10% free - aggressive cleanup
    DISK_PRESSURE_EMERGENCY = 3  // <5% free - emergency deletion
} disk_pressure_level_t;

// Disk pressure thresholds (percentage of free space)
#define DISK_PRESSURE_WARNING_PCT   20.0
#define DISK_PRESSURE_CRITICAL_PCT  10.0
#define DISK_PRESSURE_EMERGENCY_PCT  5.0

/**
 * Storage health information updated by heartbeat cycle
 */
typedef struct {
    disk_pressure_level_t pressure_level;
    double free_space_pct;       // Percentage of free space (0-100)
    uint64_t free_space_bytes;   // Absolute free space in bytes
    uint64_t total_space_bytes;  // Total filesystem space in bytes
    uint64_t used_space_bytes;   // Used space in bytes
    time_t last_check_time;      // Time of last heartbeat check
    time_t last_cleanup_time;    // Time of last standard cleanup
    time_t last_deep_time;       // Time of last deep maintenance
    int last_cleanup_deleted;    // Number of recordings deleted in last cleanup
    uint64_t last_cleanup_freed; // Bytes freed in last cleanup
} storage_health_t;

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
 * Start the storage manager thread (unified controller)
 *
 * This thread implements a tiered wake cycle:
 * - Heartbeat (60s): Disk pressure detection via statvfs()
 * - Standard cleanup (15min): Tiered retention, quota enforcement, cache refresh
 * - Deep maintenance (6h): Full analytics, daily stats update
 *
 * @param interval_seconds Base interval (used for standard cleanup cycle)
 * @return 0 on success, non-zero on failure
 */
int start_storage_manager_thread(int interval_seconds);

/**
 * Stop the storage manager thread
 *
 * @return 0 on success, non-zero on failure
 */
int stop_storage_manager_thread(void);

/**
 * Get current storage health information
 * Thread-safe: returns a snapshot of the latest health data
 *
 * @param health Pointer to storage_health_t to fill
 * @return 0 on success, -1 on error
 */
int get_storage_health(storage_health_t *health);

/**
 * Get current disk pressure level
 * Thread-safe: returns the latest pressure level from heartbeat
 *
 * @return Current disk_pressure_level_t
 */
disk_pressure_level_t get_disk_pressure_level(void);

/**
 * Trigger an immediate cleanup cycle
 * Wakes the storage controller thread to perform cleanup now
 *
 * @param force_aggressive If true, perform aggressive cleanup regardless of pressure
 */
void trigger_storage_cleanup(bool force_aggressive);

/**
 * Get human-readable string for a disk pressure level.
 *
 * Pure function: no I/O, no global state.  Static inline so tests can call
 * it without linking the full storage module.
 */
static inline const char* disk_pressure_level_str(disk_pressure_level_t level) {
    switch (level) {
        case DISK_PRESSURE_NORMAL:    return "Normal";
        case DISK_PRESSURE_WARNING:   return "Warning";
        case DISK_PRESSURE_CRITICAL:  return "Critical";
        case DISK_PRESSURE_EMERGENCY: return "Emergency";
        default:                      return "Unknown";
    }
}

/**
 * Classify a free-space percentage into a disk pressure level.
 *
 * Pure function: no I/O, no global state.  Safe to call from any thread
 * and easy to unit-test in isolation.
 *
 * @param free_pct  Percentage of filesystem space that is free (0.0–100.0)
 * @return          The corresponding disk_pressure_level_t
 */
static inline disk_pressure_level_t evaluate_disk_pressure_level(double free_pct) {
    if (free_pct < DISK_PRESSURE_EMERGENCY_PCT) return DISK_PRESSURE_EMERGENCY;
    if (free_pct < DISK_PRESSURE_CRITICAL_PCT)  return DISK_PRESSURE_CRITICAL;
    if (free_pct < DISK_PRESSURE_WARNING_PCT)   return DISK_PRESSURE_WARNING;
    return DISK_PRESSURE_NORMAL;
}

#endif // LIGHTNVR_STORAGE_MANAGER_H
