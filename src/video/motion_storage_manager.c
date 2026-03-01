#include "video/motion_storage_manager.h"
#include "database/db_motion_config.h"
#include "core/logger.h"
#include <unistd.h>
#include <sys/statvfs.h>
#include <string.h>
#include <errno.h>

/**
 * Get disk space information for a path
 */
static int get_disk_space(const char *path, uint64_t *total, uint64_t *available) {
    struct statvfs stat;
    
    if (statvfs(path, &stat) != 0) {
        log_error("Failed to get disk space for path: %s", path);
        return -1;
    }
    
    if (total) {
        *total = (uint64_t)stat.f_blocks * stat.f_frsize;
    }
    
    if (available) {
        *available = (uint64_t)stat.f_bavail * stat.f_frsize;
    }
    
    return 0;
}

/**
 * Delete a file and remove it from database
 */
int delete_motion_recording(const char *file_path) {
    if (!file_path) {
        return -1;
    }
    
    // Delete the file from disk
    if (unlink(file_path) != 0) {
        if (errno != ENOENT) {  // Ignore if file doesn't exist
            log_error("Failed to delete recording file: %s (%s)", file_path, strerror(errno));
            return -1;
        }
    }
    
    // Note: Database cleanup is handled by cleanup_old_motion_recordings
    log_debug("Deleted motion recording file: %s", file_path);
    return 0;
}

/**
 * Cleanup old recordings based on retention policy
 */
int cleanup_old_recordings(const char *stream_name, int retention_days) {
    if (retention_days < 0) {
        log_error("Invalid retention days: %d", retention_days);
        return -1;
    }
    
    log_info("Cleaning up recordings older than %d days for stream: %s",
             retention_days, stream_name ? stream_name : "all");
    
    // Get list of recordings to delete
    char paths[1000][512];
    time_t timestamps[1000];
    uint64_t sizes[1000];
    
    time_t cutoff_time = time(NULL) - ((time_t)retention_days * 24 * 60 * 60);
    
    int count = get_motion_recordings_list(stream_name, 0, cutoff_time, paths, timestamps, sizes, 1000);
    if (count < 0) {
        log_error("Failed to get list of old recordings");
        return -1;
    }
    
    if (count == 0) {
        log_debug("No old recordings to clean up");
        return 0;
    }
    
    // Delete each file
    int deleted = 0;
    for (int i = 0; i < count; i++) {
        if (delete_motion_recording(paths[i]) == 0) {
            deleted++;
        }
    }
    
    // Clean up database entries
    int db_deleted = cleanup_old_motion_recordings(stream_name, retention_days);
    if (db_deleted < 0) {
        log_warn("Failed to cleanup database entries for old recordings");
    }
    
    log_info("Cleaned up %d old motion recordings (retention: %d days)", deleted, retention_days);
    return deleted;
}

/**
 * Get storage statistics
 */
int get_motion_storage_stats(const char *stream_name, motion_storage_stats_t *stats) {
    if (!stats) {
        return -1;
    }
    
    memset(stats, 0, sizeof(motion_storage_stats_t));
    
    // Get database statistics
    uint64_t total_recordings = 0;
    uint64_t total_size = 0;
    time_t oldest = 0;
    time_t newest = 0;
    
    if (get_motion_recording_db_stats(stream_name, &total_recordings, &total_size, &oldest, &newest) == 0) {
        stats->total_recordings = total_recordings;
        stats->total_size_bytes = total_size;
        stats->oldest_recording = oldest;
        stats->newest_recording = newest;
    }
    
    // Get disk space information (use recordings directory)
    const char *recordings_path = "recordings/motion";
    get_disk_space(recordings_path, &stats->disk_space_total, &stats->disk_space_available);
    
    return 0;
}

