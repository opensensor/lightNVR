#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <sys/statvfs.h>
#include <dirent.h>
#include <unistd.h>
#include <fcntl.h>
#include <errno.h>
#include <pthread.h>

#include "storage/storage_manager.h"
#include "storage/storage_manager_streams_cache.h"
#include "database/db_auth.h"
#include "database/db_streams.h"
#include "database/db_recordings.h"
#include "web/api_handlers_recordings_thumbnail.h"
#include "core/config.h"
#include "core/logger.h"
#include "core/mqtt_client.h"

// Maximum number of streams to process at once
#define MAX_STREAMS_BATCH 64
// Maximum recordings to delete per stream per run
#define MAX_RECORDINGS_PER_STREAM 100

// Maximum length for stream names (including null terminator)
#define MAX_STREAM_NAME_LENGTH 64

// Maximum orphaned recordings to process per run
#define MAX_ORPHANED_BATCH 100

// Default retention period (in days) when no global or stream-specific value is configured
#define DEFAULT_RETENTION_DAYS 30
// Multiplier for detection retention default (detection = N * regular retention)
#define DETECTION_RETENTION_MULTIPLIER 3
// Orphan safety parameters
#define ORPHAN_SAFETY_THRESHOLD 0.5
#define MIN_RECORDINGS_FOR_THRESHOLD 10
#define MP4_SUBDIR "mp4"
#define MAX_STORAGE_PATH_LENGTH      256
#define MAX_RECORDING_PATH_LENGTH    768

// Forward declarations
static int apply_legacy_retention_policy(void);

// Storage manager state
static struct {
    char storage_path[MAX_STORAGE_PATH_LENGTH];
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
            char path[MAX_RECORDING_PATH_LENGTH];
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
                        char rec_path[MAX_RECORDING_PATH_LENGTH];
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

    // Attempt unlink directly instead of stat-then-unlink to avoid TOCTOU race condition.
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
    char stream_names[MAX_STREAMS_BATCH][MAX_STREAM_NAME_LENGTH];
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
            config.retention_days = storage_manager.retention_days > 0 ? storage_manager.retention_days : DEFAULT_RETENTION_DAYS;
            config.detection_retention_days = config.retention_days * DETECTION_RETENTION_MULTIPLIER; // Default: 3x regular retention
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

                    // Delete associated thumbnails
                    delete_recording_thumbnails(recordings[i].id);

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
            uint64_t current_usage = get_stream_storage_bytes(stream_name);
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

                    // Delete associated thumbnails
                    delete_recording_thumbnails(recordings[i].id);

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
    // Safety check: verify storage is actually accessible before orphan cleanup.
    // If storage is unavailable (mount lost, etc.), every recording looks "orphaned"
    // and we'd incorrectly wipe the entire database.
    bool storage_accessible = false;
    {
        char mp4_path[512];
        snprintf(mp4_path, sizeof(mp4_path), "%s/%s", storage_manager.storage_path, MP4_SUBDIR);
        struct stat st;
        if (stat(storage_manager.storage_path, &st) == 0 && S_ISDIR(st.st_mode) &&
            stat(mp4_path, &st) == 0 && S_ISDIR(st.st_mode)) {
            storage_accessible = true;
        }
    }

    if (!storage_accessible) {
        log_error("Storage path %s or mp4 subdirectory is not accessible - "
                  "skipping orphan cleanup to prevent incorrect mass deletion",
                  storage_manager.storage_path);
    } else {
        int total_checked = 0;
        recording_metadata_t orphaned[MAX_ORPHANED_BATCH];
        int orphan_count = get_orphaned_db_entries(orphaned, MAX_ORPHANED_BATCH, &total_checked);

        if (orphan_count > 0 && total_checked > 0) {
            // Safety threshold: if more than 50% of checked recordings appear orphaned,
            // this is almost certainly a storage availability problem, not genuine orphans.
            double orphan_ratio = (double)orphan_count / (double)total_checked;
            if (orphan_ratio > ORPHAN_SAFETY_THRESHOLD &&
                total_checked >= MIN_RECORDINGS_FOR_THRESHOLD) {
                log_error("Orphan safety threshold exceeded: %d of %d checked recordings (%.0f%%) "
                          "appear orphaned - this likely indicates a storage availability issue, "
                          "skipping orphan cleanup to protect database integrity",
                          orphan_count, total_checked, orphan_ratio * 100.0);
            } else {
                log_info("Found %d orphaned database entries (checked %d, ratio %.0f%%), cleaning up",
                         orphan_count, total_checked, orphan_ratio * 100.0);

                for (int i = 0; i < orphan_count; i++) {
                    if (delete_recording_metadata(orphaned[i].id) == 0) {
                        log_debug("Deleted orphaned DB entry: ID %llu, path %s",
                                 (unsigned long long)orphaned[i].id, orphaned[i].file_path);
                    }
                }
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
        char stream_path[MAX_RECORDING_PATH_LENGTH];
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

                    // Check if it's a file.
                    // Use fstatat+unlinkat with the directory fd to eliminate the TOCTOU
                    // race between lstat() and unlink() on the same path (#27).
                    int dir_fd = dirfd(stream_dir);
                    struct stat rec_st;
                    if (fstatat(dir_fd, rec_entry->d_name, &rec_st, AT_SYMLINK_NOFOLLOW) == 0
                            && S_ISREG(rec_st.st_mode)) {
                        // Build path only for database lookup and logging (not for unlink)
                        char rec_path[MAX_RECORDING_PATH_LENGTH];
                        snprintf(rec_path, sizeof(rec_path), "%s/%s", stream_path, rec_entry->d_name);

                        // Check if file is older than retention days
                        if (storage_manager.retention_days > 0 && rec_st.st_mtime < cutoff_time) {
                            // Check if this file is tracked in database
                            // If not tracked, delete it
                            recording_metadata_t meta;
                            if (get_recording_metadata_by_path(rec_path, &meta) != 0) {
                                // Not in database, safe to delete via unlinkat (no TOCTOU)
                                if (unlinkat(dir_fd, rec_entry->d_name, 0) == 0) {
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

// ============================================================================
// Unified Storage Controller - Tiered Wake Cycle Architecture
// ============================================================================
//
// Heartbeat (60s):   Disk pressure detection via statvfs(), health monitoring
// Standard (15min):  Tiered retention cleanup, quota enforcement, cache refresh
// Deep (6h):         Full analytics, daily stats, session cleanup
// Emergency:         On-demand triggered by pressure or API
//
// Memory budget: Heartbeat <4KB, Standard <256KB, Deep <1MB, Emergency <512KB
// ============================================================================

// Tiered wake cycle intervals (seconds)
#define HEARTBEAT_INTERVAL_SEC    60
#define STANDARD_INTERVAL_SEC    900   // 15 minutes
#define DEEP_INTERVAL_SEC      21600   // 6 hours

// Maximum recordings to process per emergency cleanup
#define MAX_EMERGENCY_RECORDINGS 200

// Unified storage controller thread state
static struct {
    pthread_t thread;
    volatile bool running;
    volatile bool exited;
    pthread_mutex_t mutex;
    pthread_cond_t cond;

    // Tiered cycle tracking
    time_t last_heartbeat;
    time_t last_standard;
    time_t last_deep;

    // Disk pressure state (protected by mutex)
    storage_health_t health;

    // Force flags (set by API, cleared by thread)
    volatile bool force_cleanup;
    volatile bool force_aggressive;
} unified_ctrl = {
    .running = false,
    .exited = true,
    .mutex = PTHREAD_MUTEX_INITIALIZER,
    .cond = PTHREAD_COND_INITIALIZER,
    .last_heartbeat = 0,
    .last_standard = 0,
    .last_deep = 0,
    .health = {
        .pressure_level = DISK_PRESSURE_NORMAL,
        .free_space_pct = 100.0,
        .free_space_bytes = 0,
        .total_space_bytes = 0,
        .used_space_bytes = 0,
        .last_check_time = 0,
        .last_cleanup_time = 0,
        .last_deep_time = 0,
        .last_cleanup_deleted = 0,
        .last_cleanup_freed = 0
    },
    .force_cleanup = false,
    .force_aggressive = false
};


// ---- Heartbeat Cycle: Disk Pressure Detection ----

/**
 * Detect disk pressure level using statvfs()
 * Updates unified_ctrl.health under mutex
 * Memory budget: <4KB (stack only, no heap allocations)
 */
static void heartbeat_check_disk_pressure(void) {
    struct statvfs fs;
    if (statvfs(storage_manager.storage_path, &fs) != 0) {
        log_error("Heartbeat: failed to statvfs(%s): %s",
                  storage_manager.storage_path, strerror(errno));
        return;
    }

    uint64_t total = (uint64_t)fs.f_blocks * fs.f_frsize;
    uint64_t avail = (uint64_t)fs.f_bavail * fs.f_frsize;
    uint64_t used  = total > avail ? total - avail : 0;
    double free_pct = total > 0 ? ((double)avail / (double)total) * 100.0 : 0.0;

    // Determine pressure level (pure function defined in storage_manager.h)
    disk_pressure_level_t new_level = evaluate_disk_pressure_level(free_pct);

    // Update health state under mutex
    pthread_mutex_lock(&unified_ctrl.mutex);
    disk_pressure_level_t old_level = unified_ctrl.health.pressure_level;
    unified_ctrl.health.pressure_level = new_level;
    unified_ctrl.health.free_space_pct = free_pct;
    unified_ctrl.health.free_space_bytes = avail;
    unified_ctrl.health.total_space_bytes = total;
    unified_ctrl.health.used_space_bytes = used;
    unified_ctrl.health.last_check_time = time(NULL);
    pthread_mutex_unlock(&unified_ctrl.mutex);

    // Log and publish MQTT event on pressure level changes
    if (new_level != old_level) {
        if (new_level > old_level) {
            log_warn("Disk pressure INCREASED: %s -> %s (%.1f%% free, %llu MB available)",
                     disk_pressure_level_str(old_level), disk_pressure_level_str(new_level),
                     free_pct, (unsigned long long)(avail / (1024 * 1024)));
        } else {
            log_info("Disk pressure decreased: %s -> %s (%.1f%% free)",
                     disk_pressure_level_str(old_level), disk_pressure_level_str(new_level),
                     free_pct);
        }

        // Publish pressure change to MQTT: {prefix}/storage/pressure
        char mqtt_topic[256];
        char mqtt_payload[512];
        snprintf(mqtt_topic, sizeof(mqtt_topic), "%s/storage/pressure",
                 g_config.mqtt_topic_prefix);
        snprintf(mqtt_payload, sizeof(mqtt_payload),
                 "{\"previous\":\"%s\",\"current\":\"%s\","
                 "\"free_pct\":%.1f,\"free_mb\":%llu,\"total_mb\":%llu}",
                 disk_pressure_level_str(old_level),
                 disk_pressure_level_str(new_level),
                 free_pct,
                 (unsigned long long)(avail / (1024 * 1024)),
                 (unsigned long long)(total / (1024 * 1024)));
        mqtt_publish_raw(mqtt_topic, mqtt_payload, true);
    }

    log_debug("Heartbeat: %.1f%% free (%llu MB), pressure=%s",
              free_pct, (unsigned long long)(avail / (1024 * 1024)),
              disk_pressure_level_str(new_level));
}

// ---- Emergency Cleanup: Pressure-Driven Deletion ----

/**
 * Emergency cleanup triggered by Critical/Emergency disk pressure
 * Deletes disk_pressure_eligible recordings starting with ephemeral tier
 * Memory budget: <512KB
 */
static void emergency_cleanup(bool aggressive) {
    log_warn("Emergency cleanup triggered (aggressive=%s)", aggressive ? "true" : "false");

    int max_to_delete = aggressive ? MAX_EMERGENCY_RECORDINGS : (MAX_EMERGENCY_RECORDINGS / 2);

    recording_metadata_t *recordings = calloc(max_to_delete, sizeof(recording_metadata_t));
    if (!recordings) {
        log_error("Emergency cleanup: failed to allocate recording buffer");
        return;
    }

    int count = get_recordings_for_pressure_cleanup(recordings, max_to_delete);
    if (count <= 0) {
        log_info("Emergency cleanup: no eligible recordings to delete");
        free(recordings);
        return;
    }

    int deleted = 0;
    uint64_t freed = 0;

    for (int i = 0; i < count; i++) {
        if (!unified_ctrl.running) break;  // Respect shutdown

        if (recordings[i].file_path[0] != '\0') {
            if (unlink(recordings[i].file_path) == 0) {
                log_debug("Emergency: deleted %s (tier=%d, size=%llu)",
                         recordings[i].file_path, recordings[i].retention_tier,
                         (unsigned long long)recordings[i].size_bytes);
                freed += recordings[i].size_bytes;
                deleted++;
            } else if (errno != ENOENT) {
                log_error("Emergency: failed to delete %s: %s",
                         recordings[i].file_path, strerror(errno));
            }
        }

        // Delete associated thumbnails and DB entry
        delete_recording_thumbnails(recordings[i].id);
        if (delete_recording_metadata(recordings[i].id) != 0) {
            log_warn("Emergency: failed to delete DB entry for ID %llu",
                     (unsigned long long)recordings[i].id);
        }
    }

    free(recordings);

    // Update health stats
    pthread_mutex_lock(&unified_ctrl.mutex);
    unified_ctrl.health.last_cleanup_deleted = deleted;
    unified_ctrl.health.last_cleanup_freed = freed;
    unified_ctrl.health.last_cleanup_time = time(NULL);
    pthread_mutex_unlock(&unified_ctrl.mutex);

    log_warn("Emergency cleanup complete: deleted %d recordings, freed %llu MB",
             deleted, (unsigned long long)(freed / (1024 * 1024)));

    // Re-check pressure after cleanup
    heartbeat_check_disk_pressure();
}

// ---- Standard Cleanup Cycle (15min) ----

/**
 * Standard cleanup: tiered retention + quota enforcement + cache refresh
 * Memory budget: <256KB
 */
static void standard_cleanup_cycle(void) {
    log_info("Standard cleanup cycle starting");
    time_t cycle_start = time(NULL);

    // 1. Apply existing per-stream retention policy (already DB-driven)
    int deleted = apply_retention_policy();
    if (deleted > 0) {
        log_info("Standard cleanup: retention policy deleted %d recordings", deleted);
    } else if (deleted < 0) {
        log_error("Standard cleanup: retention policy error");
    }

    // 2. Tiered retention cleanup using new tier-aware queries
    int tier_deleted = 0;
    uint64_t tier_freed = 0;
    recording_metadata_t *tier_recs = calloc(MAX_RECORDINGS_PER_STREAM, sizeof(recording_metadata_t));
    if (tier_recs) {
        // Get all stream names
        char stream_names[MAX_STREAMS_BATCH][MAX_STREAM_NAME_LENGTH];
        int stream_count = get_all_stream_names(stream_names, MAX_STREAMS_BATCH);

        for (int s = 0; s < stream_count && unified_ctrl.running; s++) {
            // Get stream config for tier multipliers
            stream_config_t sconfig;
            if (get_stream_config_by_name(stream_names[s], &sconfig) != 0) {
                continue;
            }

            int base_retention = sconfig.retention_days > 0 ? sconfig.retention_days : storage_manager.retention_days;
            if (base_retention <= 0) base_retention = 30;

            // Build tier multipliers array: [critical, important, standard, ephemeral]
            double tier_mults[4] = {
                sconfig.tier_critical_multiplier,   // RETENTION_TIER_CRITICAL = 0
                sconfig.tier_important_multiplier,  // RETENTION_TIER_IMPORTANT = 1
                1.0,                                // RETENTION_TIER_STANDARD = 2
                sconfig.tier_ephemeral_multiplier   // RETENTION_TIER_EPHEMERAL = 3
            };

            int count = get_recordings_for_tiered_retention(
                stream_names[s], base_retention,
                tier_mults,
                tier_recs, MAX_RECORDINGS_PER_STREAM);

            for (int i = 0; i < count && unified_ctrl.running; i++) {
                if (tier_recs[i].file_path[0] != '\0') {
                    if (unlink(tier_recs[i].file_path) == 0) {
                        tier_freed += tier_recs[i].size_bytes;
                        tier_deleted++;
                    } else if (errno != ENOENT) {
                        log_error("Tiered cleanup: failed to delete %s: %s",
                                 tier_recs[i].file_path, strerror(errno));
                    }
                }
                delete_recording_thumbnails(tier_recs[i].id);
                delete_recording_metadata(tier_recs[i].id);
            }
        }
        free(tier_recs);
    }

    if (tier_deleted > 0) {
        log_info("Standard cleanup: tiered retention deleted %d recordings, freed %llu MB",
                 tier_deleted, (unsigned long long)(tier_freed / (1024 * 1024)));
    }

    // 3. Refresh storage cache
    if (force_refresh_cache() == 0) {
        log_debug("Standard cleanup: cache refresh successful");
    } else {
        log_warn("Standard cleanup: cache refresh failed");
    }

    // 4. Check if pressure requires escalated cleanup
    pthread_mutex_lock(&unified_ctrl.mutex);
    disk_pressure_level_t pressure = unified_ctrl.health.pressure_level;
    unified_ctrl.health.last_cleanup_deleted = deleted + tier_deleted;
    unified_ctrl.health.last_cleanup_freed = tier_freed;
    unified_ctrl.health.last_cleanup_time = time(NULL);
    pthread_mutex_unlock(&unified_ctrl.mutex);

    if (pressure >= DISK_PRESSURE_CRITICAL) {
        log_warn("Standard cleanup: pressure is %s, triggering emergency cleanup",
                 disk_pressure_level_str(pressure));
        emergency_cleanup(pressure == DISK_PRESSURE_EMERGENCY);
    }

    time_t elapsed = time(NULL) - cycle_start;
    log_info("Standard cleanup cycle complete in %ld seconds (deleted=%d, tier_deleted=%d)",
             (long)elapsed, deleted, tier_deleted);

    // Publish cleanup results to MQTT: {prefix}/storage/cleanup
    if (deleted + tier_deleted > 0) {
        char mqtt_topic[256];
        char mqtt_payload[512];
        snprintf(mqtt_topic, sizeof(mqtt_topic), "%s/storage/cleanup",
                 g_config.mqtt_topic_prefix);
        snprintf(mqtt_payload, sizeof(mqtt_payload),
                 "{\"deleted\":%d,\"tier_deleted\":%d,\"freed_bytes\":%llu,"
                 "\"elapsed_sec\":%ld,\"pressure\":\"%s\"}",
                 deleted, tier_deleted,
                 (unsigned long long)tier_freed,
                 (long)elapsed,
                 disk_pressure_level_str(pressure));
        mqtt_publish_raw(mqtt_topic, mqtt_payload, false);
    }
}

// ---- Deep Maintenance Cycle (6h) ----

/**
 * Deep maintenance: session cleanup, full analytics
 * Memory budget: <1MB
 */
static void deep_maintenance_cycle(void) {
    log_info("Deep maintenance cycle starting");

    // 1. Clean up expired authentication sessions
    int sessions_deleted = db_auth_cleanup_sessions();
    if (sessions_deleted > 0) {
        log_info("Deep maintenance: cleaned up %d expired sessions", sessions_deleted);
    } else if (sessions_deleted < 0) {
        log_warn("Deep maintenance: session cleanup error");
    }

    // 2. Run a standard cleanup as part of deep maintenance
    standard_cleanup_cycle();

    // 3. Update deep maintenance timestamp
    pthread_mutex_lock(&unified_ctrl.mutex);
    unified_ctrl.health.last_deep_time = time(NULL);
    pthread_mutex_unlock(&unified_ctrl.mutex);

    log_info("Deep maintenance cycle complete");
}

// ---- Unified Storage Controller Thread ----

/**
 * Main thread function implementing tiered wake cycle
 *
 * Uses pthread_cond_timedwait for responsive sleep with the shortest
 * interval (heartbeat = 60s). Each wake checks which cycles are due
 * and executes them in order: heartbeat -> standard -> deep.
 */
static void* unified_storage_controller_func(void *arg) {
    (void)arg;
    log_info("Unified storage controller started");
    log_info("  Heartbeat interval: %d seconds", HEARTBEAT_INTERVAL_SEC);
    log_info("  Standard cleanup interval: %d seconds", STANDARD_INTERVAL_SEC);
    log_info("  Deep maintenance interval: %d seconds", DEEP_INTERVAL_SEC);

    time_t now = time(NULL);
    unified_ctrl.last_heartbeat = now;
    unified_ctrl.last_standard = now;
    unified_ctrl.last_deep = now;

    // Initial cache refresh
    if (force_refresh_cache() == 0) {
        log_info("Initial cache refresh successful");
    } else {
        log_warn("Initial cache refresh failed");
    }

    // Initial heartbeat to establish baseline pressure
    heartbeat_check_disk_pressure();

    while (unified_ctrl.running) {
        // Sleep until next heartbeat or signal
        pthread_mutex_lock(&unified_ctrl.mutex);

        if (!unified_ctrl.force_cleanup) {
            struct timespec ts;
            clock_gettime(CLOCK_REALTIME, &ts);
            ts.tv_sec += HEARTBEAT_INTERVAL_SEC;
            pthread_cond_timedwait(&unified_ctrl.cond, &unified_ctrl.mutex, &ts);
        }

        bool do_force = unified_ctrl.force_cleanup;
        bool do_aggressive = unified_ctrl.force_aggressive;
        unified_ctrl.force_cleanup = false;
        unified_ctrl.force_aggressive = false;
        pthread_mutex_unlock(&unified_ctrl.mutex);

        if (!unified_ctrl.running) break;

        now = time(NULL);

        // Always run heartbeat (disk pressure detection)
        heartbeat_check_disk_pressure();
        unified_ctrl.last_heartbeat = now;

        // Check if forced cleanup was requested
        if (do_force) {
            log_info("Forced cleanup requested (aggressive=%s)", do_aggressive ? "true" : "false");
            if (do_aggressive) {
                emergency_cleanup(true);
            } else {
                standard_cleanup_cycle();
            }
            continue;  // Skip normal cycle checks after forced cleanup
        }

        // Check for emergency pressure (trigger immediate action)
        pthread_mutex_lock(&unified_ctrl.mutex);
        disk_pressure_level_t pressure = unified_ctrl.health.pressure_level;
        pthread_mutex_unlock(&unified_ctrl.mutex);

        if (pressure >= DISK_PRESSURE_EMERGENCY) {
            log_warn("Emergency pressure detected in heartbeat, triggering emergency cleanup");
            emergency_cleanup(true);
        } else if (pressure >= DISK_PRESSURE_CRITICAL) {
            // At critical pressure, run standard cleanup more frequently
            if (now - unified_ctrl.last_standard >= (STANDARD_INTERVAL_SEC / 3)) {
                standard_cleanup_cycle();
                unified_ctrl.last_standard = now;
            }
        }

        // Check if standard cleanup is due
        if (now - unified_ctrl.last_standard >= STANDARD_INTERVAL_SEC) {
            standard_cleanup_cycle();
            unified_ctrl.last_standard = now;
        }

        // Check if deep maintenance is due
        if (now - unified_ctrl.last_deep >= DEEP_INTERVAL_SEC) {
            deep_maintenance_cycle();
            unified_ctrl.last_deep = now;
        }
    }

    log_info("Unified storage controller exiting");
    unified_ctrl.exited = true;
    return NULL;
}

// ---- Public API: Start/Stop/Query ----

// Check disk space and ensure minimum free space is available
bool ensure_disk_space(uint64_t min_free_bytes) {
    struct statvfs fs;
    if (statvfs(storage_manager.storage_path, &fs) != 0) {
        log_error("ensure_disk_space: statvfs failed: %s", strerror(errno));
        return false;
    }
    uint64_t avail = (uint64_t)fs.f_bavail * fs.f_frsize;
    return avail >= min_free_bytes;
}

// Start the unified storage controller thread
int start_storage_manager_thread(int interval_seconds) {
    (void)interval_seconds;  // Intervals are now fixed by tiered architecture

    pthread_mutex_lock(&unified_ctrl.mutex);

    if (unified_ctrl.running) {
        log_warn("Storage controller thread is already running");
        pthread_mutex_unlock(&unified_ctrl.mutex);
        return 0;
    }

    unified_ctrl.running = true;
    unified_ctrl.exited = false;

    if (pthread_create(&unified_ctrl.thread, NULL, unified_storage_controller_func, NULL) != 0) {
        log_error("Failed to create storage controller thread: %s", strerror(errno));
        unified_ctrl.running = false;
        pthread_mutex_unlock(&unified_ctrl.mutex);
        return -1;
    }

    pthread_mutex_unlock(&unified_ctrl.mutex);
    log_info("Unified storage controller thread started (heartbeat=%ds, standard=%ds, deep=%ds)",
             HEARTBEAT_INTERVAL_SEC, STANDARD_INTERVAL_SEC, DEEP_INTERVAL_SEC);
    return 0;
}

// Stop the unified storage controller thread
// Uses portable polling approach for musl/glibc compatibility
int stop_storage_manager_thread(void) {
    pthread_mutex_lock(&unified_ctrl.mutex);

    if (!unified_ctrl.running && unified_ctrl.exited) {
        log_info("Storage controller thread is not running");
        pthread_mutex_unlock(&unified_ctrl.mutex);
        return 0;
    }

    unified_ctrl.running = false;
    pthread_cond_broadcast(&unified_ctrl.cond);  // Wake thread immediately
    pthread_mutex_unlock(&unified_ctrl.mutex);

    log_info("Waiting for storage controller thread to exit...");

    // Poll for thread exit with 5-second timeout
    int timeout_ms = 5000;
    int elapsed_ms = 0;
    const int poll_interval_ms = 100;

    while (elapsed_ms < timeout_ms) {
        if (unified_ctrl.exited) {
            pthread_join(unified_ctrl.thread, NULL);
            log_info("Storage controller thread stopped successfully");
            return 0;
        }
        usleep(poll_interval_ms * 1000);
        elapsed_ms += poll_interval_ms;
    }

    log_warn("Storage controller thread did not exit in time (%d ms), detaching", timeout_ms);
    pthread_detach(unified_ctrl.thread);
    return 0;
}

// Get current storage health (thread-safe snapshot)
int get_storage_health(storage_health_t *health) {
    if (!health) return -1;

    pthread_mutex_lock(&unified_ctrl.mutex);
    *health = unified_ctrl.health;
    pthread_mutex_unlock(&unified_ctrl.mutex);
    return 0;
}

// Get current disk pressure level (thread-safe)
disk_pressure_level_t get_disk_pressure_level(void) {
    pthread_mutex_lock(&unified_ctrl.mutex);
    disk_pressure_level_t level = unified_ctrl.health.pressure_level;
    pthread_mutex_unlock(&unified_ctrl.mutex);
    return level;
}

// Trigger an immediate cleanup cycle (thread-safe)
void trigger_storage_cleanup(bool force_aggressive) {
    pthread_mutex_lock(&unified_ctrl.mutex);
    unified_ctrl.force_cleanup = true;
    unified_ctrl.force_aggressive = force_aggressive;
    pthread_cond_broadcast(&unified_ctrl.cond);
    pthread_mutex_unlock(&unified_ctrl.mutex);
    log_info("Triggered immediate storage cleanup (aggressive=%s)",
             force_aggressive ? "true" : "false");
}