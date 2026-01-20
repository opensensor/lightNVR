#include "video/motion_storage_manager.h"
#include "database/db_motion_config.h"
#include "core/logger.h"
#include <pthread.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/statvfs.h>
#include <dirent.h>
#include <string.h>
#include <stdlib.h>

// Cleanup thread state
static pthread_t cleanup_thread;
static bool cleanup_running = false;
static bool cleanup_initialized = false;  // Track if init was successful
static pthread_mutex_t cleanup_mutex = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t cleanup_cond = PTHREAD_COND_INITIALIZER;
static int cleanup_interval_seconds = 3600;  // Default: 1 hour
static bool force_cleanup = false;

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
    
    time_t cutoff_time = time(NULL) - (retention_days * 24 * 60 * 60);
    
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
 * Cleanup by quota - delete oldest recordings until under quota
 */
int cleanup_by_quota(const char *stream_name, uint64_t max_size_mb) {
    if (max_size_mb == 0) {
        // 0 means unlimited
        return 0;
    }
    
    uint64_t max_size_bytes = max_size_mb * 1024 * 1024;
    
    // Get current disk usage
    int64_t current_size = get_motion_recordings_disk_usage(stream_name);
    if (current_size < 0) {
        log_error("Failed to get current disk usage");
        return -1;
    }
    
    if ((uint64_t)current_size <= max_size_bytes) {
        log_debug("Disk usage (%llu MB) is under quota (%llu MB)",
                 (unsigned long long)(current_size / 1024 / 1024),
                 (unsigned long long)max_size_mb);
        return 0;
    }
    
    log_info("Disk usage (%llu MB) exceeds quota (%llu MB), cleaning up oldest recordings",
             (unsigned long long)(current_size / 1024 / 1024),
             (unsigned long long)max_size_mb);
    
    // Get list of all recordings, sorted by oldest first
    char paths[1000][512];
    time_t timestamps[1000];
    uint64_t sizes[1000];
    
    int count = get_motion_recordings_list(stream_name, 0, 0, paths, timestamps, sizes, 1000);
    if (count < 0) {
        log_error("Failed to get list of recordings");
        return -1;
    }
    
    // Delete oldest recordings until under quota
    int deleted = 0;
    uint64_t freed_bytes = 0;
    
    // Note: recordings are returned newest first, so we need to delete from the end
    for (int i = count - 1; i >= 0 && (current_size - freed_bytes) > max_size_bytes; i--) {
        if (delete_motion_recording(paths[i]) == 0) {
            freed_bytes += sizes[i];
            deleted++;
        }
    }
    
    log_info("Cleaned up %d recordings to meet quota, freed %llu MB",
             deleted, (unsigned long long)(freed_bytes / 1024 / 1024));
    
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

/**
 * Cleanup orphaned recordings
 */
int cleanup_orphaned_recordings(const char *stream_name) {
    log_info("Checking for orphaned recording entries for stream: %s",
             stream_name ? stream_name : "all");
    
    // Get list of all recordings from database
    char paths[1000][512];
    time_t timestamps[1000];
    uint64_t sizes[1000];
    
    int count = get_motion_recordings_list(stream_name, 0, 0, paths, timestamps, sizes, 1000);
    if (count < 0) {
        log_error("Failed to get list of recordings");
        return -1;
    }
    
    // Check each file and remove database entry if file doesn't exist
    int orphaned = 0;
    for (int i = 0; i < count; i++) {
        struct stat st;
        if (stat(paths[i], &st) != 0) {
            // File doesn't exist, it's orphaned
            log_debug("Found orphaned recording entry: %s", paths[i]);
            // Note: Would need a delete_motion_recording_by_path function in db_motion_config
            orphaned++;
        }
    }
    
    log_info("Found %d orphaned recording entries", orphaned);
    return orphaned;
}

/**
 * Cleanup thread function
 */
static void* cleanup_thread_func(void* arg) {
    (void)arg;
    
    log_info("Motion storage cleanup thread started");
    
    while (cleanup_running) {
        pthread_mutex_lock(&cleanup_mutex);
        
        if (!force_cleanup) {
            // Wait for interval or forced cleanup
            struct timespec ts;
            clock_gettime(CLOCK_REALTIME, &ts);
            ts.tv_sec += cleanup_interval_seconds;
            pthread_cond_timedwait(&cleanup_cond, &cleanup_mutex, &ts);
        }
        
        force_cleanup = false;
        pthread_mutex_unlock(&cleanup_mutex);
        
        if (!cleanup_running) {
            break;
        }
        
        // Perform cleanup - this would iterate through all streams
        // For now, just log that we're running
        log_debug("Running periodic motion recording cleanup");
        
        // TODO: Load all stream configs and apply their retention policies
        // For now, we'll just cleanup orphaned entries
        cleanup_orphaned_recordings(NULL);
    }
    
    log_info("Motion storage cleanup thread stopped");
    return NULL;
}

/**
 * Initialize the motion storage manager
 */
int init_motion_storage_manager(void) {
    log_info("Initializing motion storage manager");

    cleanup_running = true;

    if (pthread_create(&cleanup_thread, NULL, cleanup_thread_func, NULL) != 0) {
        log_error("Failed to create cleanup thread");
        cleanup_running = false;
        return -1;
    }

    cleanup_initialized = true;
    log_info("Motion storage manager initialized successfully");
    return 0;
}

/**
 * Shutdown the motion storage manager
 */
void shutdown_motion_storage_manager(void) {
    log_info("Shutting down motion storage manager");

    // Only attempt shutdown if we were successfully initialized
    if (!cleanup_initialized) {
        log_info("Motion storage manager was not initialized, skipping shutdown");
        return;
    }

    cleanup_running = false;
    pthread_cond_broadcast(&cleanup_cond);
    pthread_join(cleanup_thread, NULL);

    cleanup_initialized = false;
    log_info("Motion storage manager shut down");
}

/**
 * Set cleanup interval
 */
void set_cleanup_interval(int interval_seconds) {
    pthread_mutex_lock(&cleanup_mutex);
    cleanup_interval_seconds = interval_seconds;
    pthread_mutex_unlock(&cleanup_mutex);
    
    log_info("Cleanup interval set to %d seconds", interval_seconds);
}

/**
 * Trigger immediate cleanup
 */
void trigger_cleanup_now(void) {
    pthread_mutex_lock(&cleanup_mutex);
    force_cleanup = true;
    pthread_cond_broadcast(&cleanup_cond);
    pthread_mutex_unlock(&cleanup_mutex);
    
    log_info("Triggered immediate cleanup");
}

