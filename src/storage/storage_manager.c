#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <sys/statvfs.h>
#include <dirent.h>
#include <unistd.h>
#include <errno.h>
#include <pthread.h>

#include "storage/storage_manager.h"
#include "storage/storage_manager_streams_cache.h"
#include "database/db_auth.h"
#include "database/db_streams.h"
#include "database/db_recordings.h"
#include "core/logger.h"

// Maximum number of streams to process at once
#define MAX_STREAMS_BATCH 64
// Maximum recordings to delete per stream per run
#define MAX_RECORDINGS_PER_STREAM 100

// Forward declarations
static int apply_legacy_retention_policy(void);

// Storage manager state
static struct {
    char storage_path[256];
    uint64_t max_size;
    int retention_days;
    bool auto_delete_oldest;
    uint64_t total_space;
    uint64_t used_space;
    uint64_t reserved_space;
} storage_manager = {
    .storage_path = "",
    .max_size = 0,
    .retention_days = 30,
    .auto_delete_oldest = true,
    .total_space = 0,
    .used_space = 0,
    .reserved_space = 0
};

// Initialize the storage manager
int init_storage_manager(const char *storage_path, uint64_t max_size) {
    if (!storage_path) {
        log_error("Storage path is required");
        return -1;
    }

    // Copy storage path
    strncpy(storage_manager.storage_path, storage_path, sizeof(storage_manager.storage_path) - 1);
    storage_manager.storage_path[sizeof(storage_manager.storage_path) - 1] = '\0';

    // Set maximum size
    storage_manager.max_size = max_size;

    // Create storage directory if it doesn't exist
    struct stat st;
    if (stat(storage_path, &st) != 0) {
        if (mkdir(storage_path, 0755) != 0) {
            log_error("Failed to create storage directory: %s", strerror(errno));
            return -1;
        }
    } else if (!S_ISDIR(st.st_mode)) {
        log_error("Storage path is not a directory: %s", storage_path);
        return -1;
    }

    log_info("Storage manager initialized with path: %s", storage_path);

    // Start the storage manager thread with a default interval of 1 hour
    if (start_storage_manager_thread(3600) != 0) {
        log_warn("Failed to start storage manager thread, automatic tasks will not be performed");
    }

    return 0;
}

// Shutdown the storage manager
void shutdown_storage_manager(void) {
    // Stop the storage manager thread
    if (stop_storage_manager_thread() != 0) {
        log_warn("Failed to stop storage manager thread");
    }

    log_info("Storage manager shutdown");
}

// Open a new recording file
void* open_recording_file(const char *stream_name, const char *codec, int width, int height, int fps) {
    // Stub implementation
    log_debug("Opening recording file for stream: %s", stream_name);
    return NULL;
}

// Write frame data to a recording file
int write_frame_to_recording(void *handle, const uint8_t *data, size_t size,
                            uint64_t timestamp, bool is_key_frame) {
    // Stub implementation
    return 0;
}

// Close a recording file
int close_recording_file(void *handle) {
    // Stub implementation
    return 0;
}

// Get storage statistics
int get_storage_stats(storage_stats_t *stats) {
    if (!stats) {
        return -1;
    }

    // Initialize stats structure
    memset(stats, 0, sizeof(storage_stats_t));

    // Get filesystem statistics for the storage path
    struct statvfs fs_stats;
    if (statvfs(storage_manager.storage_path, &fs_stats) != 0) {
        log_error("Failed to get filesystem statistics: %s", strerror(errno));
        return -1;
    }

    // Calculate total, used, and free space
    uint64_t block_size = fs_stats.f_frsize;
    stats->total_space = (uint64_t)fs_stats.f_blocks * block_size;
    stats->free_space = (uint64_t)fs_stats.f_bavail * block_size;
    stats->used_space = stats->total_space - stats->free_space;
    stats->reserved_space = storage_manager.reserved_space;

    // Scan the storage directory to get recording statistics
    DIR *dir = opendir(storage_manager.storage_path);
    if (dir) {
        struct dirent *entry;
        stats->total_recordings = 0;
        stats->total_recording_bytes = 0;
        stats->oldest_recording_time = UINT64_MAX;
        stats->newest_recording_time = 0;

        while ((entry = readdir(dir)) != NULL) {
            // Skip . and ..
            if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) {
                continue;
            }

            // Check if it's a directory (stream directory)
            char path[512];
            snprintf(path, sizeof(path), "%s/%s", storage_manager.storage_path, entry->d_name);

            struct stat st;
            if (stat(path, &st) == 0 && S_ISDIR(st.st_mode)) {
                // Scan stream directory for recordings
                DIR *stream_dir = opendir(path);
                if (stream_dir) {
                    struct dirent *rec_entry;

                    while ((rec_entry = readdir(stream_dir)) != NULL) {
                        // Skip . and ..
                        if (strcmp(rec_entry->d_name, ".") == 0 || strcmp(rec_entry->d_name, "..") == 0) {
                            continue;
                        }

                        // Check if it's a file
                        char rec_path[768];
                        snprintf(rec_path, sizeof(rec_path), "%s/%s", path, rec_entry->d_name);

                        struct stat rec_st;
                        if (stat(rec_path, &rec_st) == 0 && S_ISREG(rec_st.st_mode)) {
                            // Count recording and add size
                            stats->total_recordings++;
                            stats->total_recording_bytes += rec_st.st_size;

                            // Update oldest/newest recording time
                            if (rec_st.st_mtime < stats->oldest_recording_time) {
                                stats->oldest_recording_time = rec_st.st_mtime;
                            }
                            if (rec_st.st_mtime > stats->newest_recording_time) {
                                stats->newest_recording_time = rec_st.st_mtime;
                            }
                        }
                    }

                    closedir(stream_dir);
                }
            }
        }

        closedir(dir);

        // If no recordings found, reset timestamps
        if (stats->oldest_recording_time == UINT64_MAX) {
            stats->oldest_recording_time = 0;
        }
    }

    return 0;
}

// List recordings for a stream
int list_recordings(const char *stream_name, time_t start_time, time_t end_time,
                   recording_info_t *recordings, int max_count) {
    // Stub implementation
    return 0;
}

// Delete a recording
int delete_recording(const char *path) {
    if (!path) {
        log_error("Invalid path for delete_recording");
        return -1;
    }

    log_info("Deleting recording file: %s", path);

    // Check if file exists
    struct stat st;
    if (stat(path, &st) != 0) {
        log_error("File not found: %s (error: %s)", path, strerror(errno));
        return -1;
    }

    // Delete the file
    if (unlink(path) != 0) {
        log_error("Failed to delete file: %s (error: %s)", path, strerror(errno));
        return -1;
    }

    log_info("Successfully deleted recording file: %s", path);
    return 0;
}

/**
 * Apply per-stream retention policy
 *
 * This function processes each stream individually, applying:
 * 1. Time-based retention (regular recordings vs detection recordings)
 * 2. Storage quota enforcement per stream
 * 3. Orphaned database entry cleanup
 *
 * Protected recordings are never deleted.
 *
 * @return Number of recordings deleted, or -1 on error
 */
int apply_retention_policy(void) {
    log_info("Applying per-stream retention policy");

    int total_deleted = 0;
    uint64_t total_freed = 0;

    // Get list of all stream names
    char stream_names[MAX_STREAMS_BATCH][64];
    int stream_count = get_all_stream_names(stream_names, MAX_STREAMS_BATCH);

    if (stream_count < 0) {
        log_error("Failed to get stream names for retention policy");
        return -1;
    }

    if (stream_count == 0) {
        log_debug("No streams found for retention policy");
        return 0;
    }

    log_info("Processing retention policy for %d streams", stream_count);

    // Process each stream
    for (int s = 0; s < stream_count; s++) {
        const char *stream_name = stream_names[s];
        stream_retention_config_t config;

        // Get stream-specific retention config
        if (get_stream_retention_config(stream_name, &config) != 0) {
            log_warn("Failed to get retention config for stream %s, using defaults", stream_name);
            config.retention_days = storage_manager.retention_days > 0 ? storage_manager.retention_days : 30;
            config.detection_retention_days = config.retention_days * 3; // Default: 3x regular retention
            config.max_storage_mb = 0; // No quota
        }

        log_debug("Stream %s: retention=%d days, detection_retention=%d days, max_storage=%lu MB",
                  stream_name, config.retention_days, config.detection_retention_days,
                  (unsigned long)config.max_storage_mb);

        // Skip if no retention policy configured
        if (config.retention_days <= 0 && config.detection_retention_days <= 0 && config.max_storage_mb == 0) {
            log_debug("No retention policy for stream %s, skipping", stream_name);
            continue;
        }

        // Phase 1: Time-based retention cleanup
        if (config.retention_days > 0 || config.detection_retention_days > 0) {
            recording_metadata_t recordings[MAX_RECORDINGS_PER_STREAM];
            int count = get_recordings_for_retention(stream_name,
                                                     config.retention_days,
                                                     config.detection_retention_days,
                                                     recordings,
                                                     MAX_RECORDINGS_PER_STREAM);

            if (count > 0) {
                log_info("Stream %s: found %d recordings past retention", stream_name, count);

                for (int i = 0; i < count; i++) {
                    // Delete the file
                    if (recordings[i].file_path[0] != '\0') {
                        if (unlink(recordings[i].file_path) == 0) {
                            log_debug("Deleted recording: %s (trigger: %s)",
                                     recordings[i].file_path, recordings[i].trigger_type);
                            total_freed += recordings[i].size_bytes;
                            total_deleted++;
                        } else if (errno != ENOENT) {
                            log_error("Failed to delete recording file: %s (error: %s)",
                                     recordings[i].file_path, strerror(errno));
                        }
                    }

                    // Delete the database entry
                    if (delete_recording_metadata(recordings[i].id) != 0) {
                        log_warn("Failed to delete recording metadata for ID %llu",
                                (unsigned long long)recordings[i].id);
                    }
                }
            }
        }

        // Phase 2: Storage quota enforcement
        if (config.max_storage_mb > 0) {
            uint64_t current_usage = get_stream_storage_usage_db(stream_name);
            uint64_t max_bytes = config.max_storage_mb * 1024 * 1024;

            if (current_usage > max_bytes) {
                uint64_t to_free = current_usage - max_bytes;
                log_info("Stream %s: over quota by %lu bytes, need to free space",
                        stream_name, (unsigned long)to_free);

                recording_metadata_t recordings[MAX_RECORDINGS_PER_STREAM];
                int count = get_recordings_for_quota_enforcement(stream_name,
                                                                  recordings,
                                                                  MAX_RECORDINGS_PER_STREAM);

                uint64_t freed = 0;
                for (int i = 0; i < count && freed < to_free; i++) {
                    // Delete the file
                    if (recordings[i].file_path[0] != '\0') {
                        if (unlink(recordings[i].file_path) == 0) {
                            log_debug("Deleted recording for quota: %s", recordings[i].file_path);
                            freed += recordings[i].size_bytes;
                            total_freed += recordings[i].size_bytes;
                            total_deleted++;
                        } else if (errno != ENOENT) {
                            log_error("Failed to delete recording file: %s (error: %s)",
                                     recordings[i].file_path, strerror(errno));
                        }
                    }

                    // Delete the database entry
                    if (delete_recording_metadata(recordings[i].id) != 0) {
                        log_warn("Failed to delete recording metadata for ID %llu",
                                (unsigned long long)recordings[i].id);
                    }
                }

                log_info("Stream %s: freed %lu bytes for quota enforcement",
                        stream_name, (unsigned long)freed);
            }
        }
    }

    // Phase 3: Clean up orphaned database entries (files that no longer exist)
    recording_metadata_t orphaned[100];
    int orphan_count = get_orphaned_db_entries(orphaned, 100);

    if (orphan_count > 0) {
        log_info("Found %d orphaned database entries, cleaning up", orphan_count);

        for (int i = 0; i < orphan_count; i++) {
            if (delete_recording_metadata(orphaned[i].id) == 0) {
                log_debug("Deleted orphaned DB entry: ID %llu, path %s",
                         (unsigned long long)orphaned[i].id, orphaned[i].file_path);
            }
        }
    }

    // Also apply global retention policy as fallback (for files not in database)
    if (storage_manager.retention_days > 0 || storage_manager.max_size > 0) {
        int legacy_deleted = apply_legacy_retention_policy();
        if (legacy_deleted > 0) {
            total_deleted += legacy_deleted;
            log_info("Legacy retention policy deleted %d additional files", legacy_deleted);
        }
    }

    log_info("Retention policy complete: deleted %d recordings, freed %lu bytes",
             total_deleted, (unsigned long)total_freed);

    return total_deleted;
}

/**
 * Legacy retention policy for files not tracked in database
 * This handles cleanup of files that may have been created before database tracking
 *
 * @return Number of files deleted
 */
static int apply_legacy_retention_policy(void) {
    // Calculate cutoff time for retention days
    time_t now = time(NULL);
    time_t cutoff_time = now - (storage_manager.retention_days * 86400);

    int deleted_count = 0;

    // Scan the storage directory
    DIR *dir = opendir(storage_manager.storage_path);
    if (!dir) {
        log_error("Failed to open storage directory: %s", strerror(errno));
        return 0;
    }

    struct dirent *entry;
    while ((entry = readdir(dir)) != NULL) {
        // Skip . and ..
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) {
            continue;
        }

        // Check if it's a directory (stream directory)
        char stream_path[512];
        snprintf(stream_path, sizeof(stream_path), "%s/%s", storage_manager.storage_path, entry->d_name);

        struct stat st;
        if (stat(stream_path, &st) == 0 && S_ISDIR(st.st_mode)) {
            // Scan stream directory for recordings
            DIR *stream_dir = opendir(stream_path);
            if (stream_dir) {
                struct dirent *rec_entry;

                while ((rec_entry = readdir(stream_dir)) != NULL) {
                    // Skip . and ..
                    if (strcmp(rec_entry->d_name, ".") == 0 || strcmp(rec_entry->d_name, "..") == 0) {
                        continue;
                    }

                    // Check if it's a file
                    char rec_path[768];
                    snprintf(rec_path, sizeof(rec_path), "%s/%s", stream_path, rec_entry->d_name);

                    struct stat rec_st;
                    if (stat(rec_path, &rec_st) == 0 && S_ISREG(rec_st.st_mode)) {
                        // Check if file is older than retention days
                        if (storage_manager.retention_days > 0 && rec_st.st_mtime < cutoff_time) {
                            // Check if this file is tracked in database
                            // If not tracked, delete it
                            recording_metadata_t meta;
                            if (get_recording_metadata_by_path(rec_path, &meta) != 0) {
                                // Not in database, safe to delete
                                if (unlink(rec_path) == 0) {
                                    log_debug("Deleted untracked old recording: %s (age: %ld days)",
                                             rec_path, (now - rec_st.st_mtime) / 86400);
                                    deleted_count++;
                                }
                            }
                        }
                    }
                }

                closedir(stream_dir);
            }
        }
    }

    closedir(dir);
    return deleted_count;
}

// Set maximum storage size
int set_max_storage_size(uint64_t max_size) {
    storage_manager.max_size = max_size;
    return 0;
}

// Set retention days
int set_retention_days(int days) {
    if (days < 0) {
        return -1;
    }

    storage_manager.retention_days = days;
    return 0;
}

// Check if storage is available
bool is_storage_available(void) {
    struct stat st;
    return (stat(storage_manager.storage_path, &st) == 0 && S_ISDIR(st.st_mode));
}

// Get path to a recording file
int get_recording_path(const char *stream_name, time_t timestamp, char *path, size_t path_size) {
    // Stub implementation
    return 0;
}

// Create a directory for a stream if it doesn't exist
int create_stream_directory(const char *stream_name) {
    if (!stream_name) {
        return -1;
    }

    char dir_path[512];
    snprintf(dir_path, sizeof(dir_path), "%s/%s", storage_manager.storage_path, stream_name);

    struct stat st;
    if (stat(dir_path, &st) != 0) {
        if (mkdir(dir_path, 0755) != 0) {
            log_error("Failed to create stream directory: %s", strerror(errno));
            return -1;
        }
    } else if (!S_ISDIR(st.st_mode)) {
        log_error("Stream path is not a directory: %s", dir_path);
        return -1;
    }

    return 0;
}

// Check disk space and ensure minimum free space is available
bool ensure_disk_space(uint64_t min_free_bytes) {
    // Stub implementation
    return true;
}

// Storage manager thread state
static struct {
    pthread_t thread;
    volatile bool running;
    volatile bool exited;  // Flag to indicate thread has exited
    int interval_seconds;
    pthread_mutex_t mutex;
    time_t last_cache_refresh;
    int cache_refresh_interval; // in seconds
} storage_manager_thread = {
    .running = false,
    .exited = true,  // Initially true since thread hasn't started
    .interval_seconds = 3600, // Default to 1 hour
    .last_cache_refresh = 0,
    .cache_refresh_interval = 900 // Default to 15 minutes
};

// Forward declaration for the cache refresh function
extern int force_refresh_cache(void);

// Storage manager thread function
static void* storage_manager_thread_func(void *arg) {
    log_info("Storage manager thread started with interval: %d seconds", storage_manager_thread.interval_seconds);
    log_info("Cache refresh interval: %d seconds", storage_manager_thread.cache_refresh_interval);

    // Initialize last cache refresh time
    storage_manager_thread.last_cache_refresh = time(NULL);

    // Initial cache refresh
    if (force_refresh_cache() == 0) {
        log_info("Initial cache refresh successful");
    } else {
        log_warn("Initial cache refresh failed");
    }

    while (storage_manager_thread.running) {
        time_t now = time(NULL);

        // Apply retention policy
        int deleted = apply_retention_policy();
        if (deleted > 0) {
            log_info("Storage manager thread deleted %d recordings", deleted);
        } else if (deleted < 0) {
            log_error("Storage manager thread encountered an error applying retention policy");
        }

        // Clean up expired authentication sessions
        int sessions_deleted = db_auth_cleanup_sessions();
        if (sessions_deleted > 0) {
            log_info("Storage manager thread cleaned up %d expired sessions", sessions_deleted);
        } else if (sessions_deleted < 0) {
            log_warn("Storage manager thread encountered an error cleaning up sessions");
        }

        // Check if it's time to refresh the cache
        if (now - storage_manager_thread.last_cache_refresh >= storage_manager_thread.cache_refresh_interval) {
            log_info("Refreshing storage cache");
            if (force_refresh_cache() == 0) {
                log_info("Cache refresh successful");
            } else {
                log_warn("Cache refresh failed");
            }
            storage_manager_thread.last_cache_refresh = now;
        }

        // Sleep for 1 second at a time to be responsive to shutdown requests
        for (int i = 0; i < storage_manager_thread.interval_seconds && storage_manager_thread.running; i++) {
            sleep(1);
        }
    }

    log_info("Storage manager thread exiting");
    storage_manager_thread.exited = true;  // Signal that thread has exited
    return NULL;
}

// Start the storage manager thread
int start_storage_manager_thread(int interval_seconds) {
    // Initialize mutex if not already initialized
    static bool mutex_initialized = false;
    if (!mutex_initialized) {
        if (pthread_mutex_init(&storage_manager_thread.mutex, NULL) != 0) {
            log_error("Failed to initialize storage manager thread mutex");
            return -1;
        }
        mutex_initialized = true;
    }

    pthread_mutex_lock(&storage_manager_thread.mutex);

    // Check if thread is already running
    if (storage_manager_thread.running) {
        log_warn("Storage manager thread is already running");
        pthread_mutex_unlock(&storage_manager_thread.mutex);
        return 0;
    }

    // Set interval (minimum 60 seconds)
    storage_manager_thread.interval_seconds = (interval_seconds < 60) ? 60 : interval_seconds;
    storage_manager_thread.running = true;
    storage_manager_thread.exited = false;  // Reset the exited flag

    // Create thread
    if (pthread_create(&storage_manager_thread.thread, NULL, storage_manager_thread_func, NULL) != 0) {
        log_error("Failed to create storage manager thread: %s", strerror(errno));
        storage_manager_thread.running = false;
        pthread_mutex_unlock(&storage_manager_thread.mutex);
        return -1;
    }

    pthread_mutex_unlock(&storage_manager_thread.mutex);
    log_info("Storage manager thread started with interval: %d seconds", storage_manager_thread.interval_seconds);
    return 0;
}

// Stop the storage manager thread
// Uses portable polling approach instead of pthread_timedjoin_np (GNU extension)
// to work on both glibc and musl (e.g., Linux 4.4 with thingino-firmware)
int stop_storage_manager_thread(void) {
    pthread_mutex_lock(&storage_manager_thread.mutex);

    // Check if thread is running
    if (!storage_manager_thread.running && storage_manager_thread.exited) {
        log_info("Storage manager thread is not running");
        pthread_mutex_unlock(&storage_manager_thread.mutex);
        return 0;
    }

    // Signal thread to stop
    storage_manager_thread.running = false;
    pthread_mutex_unlock(&storage_manager_thread.mutex);

    log_info("Waiting for storage manager thread to exit...");

    // Use portable polling approach with timeout (5 seconds)
    // Poll every 100ms to check if thread has exited
    int timeout_ms = 5000;
    int elapsed_ms = 0;
    const int poll_interval_ms = 100;

    while (elapsed_ms < timeout_ms) {
        if (storage_manager_thread.exited) {
            // Thread has exited, we can join it
            pthread_join(storage_manager_thread.thread, NULL);
            log_info("Storage manager thread stopped successfully");
            return 0;
        }
        usleep(poll_interval_ms * 1000);
        elapsed_ms += poll_interval_ms;
    }

    // Thread did not exit in time, detach it
    log_warn("Storage manager thread did not exit in time (%d ms), detaching", timeout_ms);
    pthread_detach(storage_manager_thread.thread);
    return 0;
}
