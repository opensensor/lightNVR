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
#include "core/logger.h"

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

// Apply retention policy
int apply_retention_policy(void) {
    log_info("Applying retention policy (max size: %lu bytes, retention days: %d)",
             storage_manager.max_size, storage_manager.retention_days);

    // Get current storage stats
    storage_stats_t stats;
    if (get_storage_stats(&stats) != 0) {
        log_error("Failed to get storage statistics");
        return -1;
    }

    // Check if we need to apply retention policy
    bool need_cleanup_size = (storage_manager.max_size > 0 && stats.used_space > storage_manager.max_size);
    bool need_cleanup_days = (storage_manager.retention_days > 0);

    if (!need_cleanup_size && !need_cleanup_days) {
        log_debug("No retention policy to apply");
        return 0;
    }

    // Calculate cutoff time for retention days
    time_t now = time(NULL);
    time_t cutoff_time = now - (storage_manager.retention_days * 86400); // 86400 seconds in a day

    // Track deleted files
    int deleted_count = 0;
    uint64_t freed_space = 0;

    // Scan the storage directory
    DIR *dir = opendir(storage_manager.storage_path);
    if (!dir) {
        log_error("Failed to open storage directory: %s", strerror(errno));
        return -1;
    }

    // First pass: collect all recording files with their timestamps and sizes
    typedef struct {
        char path[768];
        time_t mtime;
        off_t size;
    } recording_file_t;

    recording_file_t *files = NULL;
    int file_count = 0;
    int file_capacity = 0;

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
                        if (need_cleanup_days && rec_st.st_mtime < cutoff_time) {
                            // Delete file directly if it's older than retention days
                            if (unlink(rec_path) == 0) {
                                log_debug("Deleted old recording: %s (age: %ld days)",
                                         rec_path, (now - rec_st.st_mtime) / 86400);
                                deleted_count++;
                                freed_space += rec_st.st_size;
                            } else {
                                log_error("Failed to delete old recording: %s (error: %s)",
                                         rec_path, strerror(errno));
                            }
                        } else if (need_cleanup_size) {
                            // Add to files array for size-based cleanup
                            if (file_count >= file_capacity) {
                                file_capacity = file_capacity == 0 ? 1000 : file_capacity * 2;
                                recording_file_t *new_files = realloc(files, file_capacity * sizeof(recording_file_t));
                                if (!new_files) {
                                    log_error("Memory allocation failed for recording files array");
                                    free(files);
                                    closedir(stream_dir);
                                    closedir(dir);
                                    return -1;
                                }
                                files = new_files;
                            }

                            strncpy(files[file_count].path, rec_path, sizeof(files[file_count].path) - 1);
                            files[file_count].path[sizeof(files[file_count].path) - 1] = '\0';
                            files[file_count].mtime = rec_st.st_mtime;
                            files[file_count].size = rec_st.st_size;
                            file_count++;
                        }
                    }
                }

                closedir(stream_dir);
            }
        }
    }

    closedir(dir);

    // If we need to clean up by size and we still have too much used space
    if (need_cleanup_size && stats.used_space - freed_space > storage_manager.max_size && file_count > 0) {
        log_info("Need to free more space: %lu bytes over limit",
                (stats.used_space - freed_space) - storage_manager.max_size);

        // Sort files by modification time (oldest first)
        for (int i = 0; i < file_count - 1; i++) {
            for (int j = 0; j < file_count - i - 1; j++) {
                if (files[j].mtime > files[j + 1].mtime) {
                    recording_file_t temp = files[j];
                    files[j] = files[j + 1];
                    files[j + 1] = temp;
                }
            }
        }

        // Delete oldest files until we're under the limit
        for (int i = 0; i < file_count; i++) {
            if (stats.used_space - freed_space <= storage_manager.max_size) {
                break;
            }

            if (unlink(files[i].path) == 0) {
                log_debug("Deleted recording to free space: %s", files[i].path);
                deleted_count++;
                freed_space += files[i].size;
            } else {
                log_error("Failed to delete recording: %s (error: %s)",
                         files[i].path, strerror(errno));
            }
        }
    }

    // Free memory
    free(files);

    log_info("Retention policy applied: deleted %d files, freed %lu bytes",
             deleted_count, freed_space);

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
    bool running;
    int interval_seconds;
    pthread_mutex_t mutex;
    time_t last_cache_refresh;
    int cache_refresh_interval; // in seconds
} storage_manager_thread = {
    .running = false,
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
int stop_storage_manager_thread(void) {
    pthread_mutex_lock(&storage_manager_thread.mutex);

    // Check if thread is running
    if (!storage_manager_thread.running) {
        log_warn("Storage manager thread is not running");
        pthread_mutex_unlock(&storage_manager_thread.mutex);
        return 0;
    }

    // Signal thread to stop
    storage_manager_thread.running = false;
    pthread_mutex_unlock(&storage_manager_thread.mutex);

    // Wait for thread to exit
    if (pthread_join(storage_manager_thread.thread, NULL) != 0) {
        log_error("Failed to join storage manager thread: %s", strerror(errno));
        return -1;
    }

    log_info("Storage manager thread stopped");
    return 0;
}
