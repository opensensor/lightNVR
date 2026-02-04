/**
 * Database Recordings Synchronization
 *
 * This module provides functionality to synchronize recording metadata in the database
 * with actual file sizes on disk. This ensures that the web interface displays accurate
 * file sizes even if the database wasn't updated during recording.
 *
 * Optimization: Only syncs recordings with size_bytes = 0 that were created since startup,
 * rather than scanning all recordings in the database.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>
#include <pthread.h>
#include <time.h>
#include <sqlite3.h>

#include "core/logger.h"
#include "core/shutdown_coordinator.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"

// Thread state
static struct {
    pthread_t thread;
    bool running;
    int interval_seconds;
    pthread_mutex_t mutex;
    time_t startup_time;  // Track when the sync thread started
} sync_thread = {
    .running = false,
    .interval_seconds = 60, // Default to 1 minute
    .startup_time = 0,
};

/**
 * Synchronize a single recording's file size with the database
 */
static int sync_recording_file_size(uint64_t recording_id, const char *file_path) {
    struct stat st;
    
    // Check if file exists and get its size
    if (stat(file_path, &st) != 0) {
        log_debug("File not found for recording %llu: %s", 
                 (unsigned long long)recording_id, file_path);
        return -1;
    }
    
    // Only update if file has non-zero size
    if (st.st_size > 0) {
        // Get current metadata from database
        recording_metadata_t metadata;
        if (get_recording_metadata_by_id(recording_id, &metadata) != 0) {
            log_error("Failed to get metadata for recording %llu", 
                     (unsigned long long)recording_id);
            return -1;
        }
        
        // Only update if the size in database is different
        if (metadata.size_bytes != (uint64_t)st.st_size) {
            log_info("Syncing file size for recording %llu: %llu bytes (was %llu bytes)",
                    (unsigned long long)recording_id, 
                    (unsigned long long)st.st_size,
                    (unsigned long long)metadata.size_bytes);
            
            // Update the database with the actual file size
            // Don't change end_time or is_complete status
            if (update_recording_metadata(recording_id, metadata.end_time, 
                                        (uint64_t)st.st_size, metadata.is_complete) != 0) {
                log_error("Failed to update file size for recording %llu", 
                         (unsigned long long)recording_id);
                return -1;
            }
            
            return 1; // Updated
        }
    }
    
    return 0; // No update needed
}

/**
 * Synchronize recordings with size_bytes = 0 since startup
 *
 * This is much more efficient than scanning all recordings - it only looks at
 * recordings that actually need their file size updated.
 */
static int sync_recordings_needing_size_update(void) {
    // Check if shutdown is in progress - abort sync to allow fast shutdown
    if (is_shutdown_initiated()) {
        log_debug("Shutdown in progress, aborting recording sync");
        return 0;
    }

    int updated_count = 0;
    int error_count = 0;
    int rc;
    sqlite3_stmt *stmt = NULL;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized for recording sync");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    // Query only recordings with size_bytes = 0 that were created since startup
    // This is much more efficient than fetching all 100k+ recordings
    const char *sql = "SELECT id, file_path FROM recordings "
                      "WHERE size_bytes = 0 AND is_complete = 1 AND start_time >= ? "
                      "ORDER BY id ASC LIMIT 1000";  // Limit to prevent runaway queries

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare sync query: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    // Bind startup time parameter
    sqlite3_bind_int64(stmt, 1, (sqlite3_int64)sync_thread.startup_time);

    log_debug("Syncing recordings with size_bytes=0 since startup time %ld",
              (long)sync_thread.startup_time);

    // Collect recordings to sync (we'll update them outside the query loop)
    typedef struct {
        uint64_t id;
        char file_path[512];
    } sync_item_t;

    sync_item_t *items = NULL;
    int item_count = 0;
    int item_capacity = 0;

    while (sqlite3_step(stmt) == SQLITE_ROW) {
        // Check for shutdown periodically
        if (is_shutdown_initiated()) {
            log_debug("Shutdown in progress, aborting recording sync query");
            break;
        }

        // Grow array if needed
        if (item_count >= item_capacity) {
            item_capacity = item_capacity == 0 ? 64 : item_capacity * 2;
            sync_item_t *new_items = realloc(items, item_capacity * sizeof(sync_item_t));
            if (!new_items) {
                log_error("Failed to allocate memory for sync items");
                break;
            }
            items = new_items;
        }

        items[item_count].id = (uint64_t)sqlite3_column_int64(stmt, 0);
        const char *path = (const char *)sqlite3_column_text(stmt, 1);
        if (path) {
            strncpy(items[item_count].file_path, path, sizeof(items[item_count].file_path) - 1);
            items[item_count].file_path[sizeof(items[item_count].file_path) - 1] = '\0';
        } else {
            items[item_count].file_path[0] = '\0';
        }
        item_count++;
    }

    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    if (item_count > 0) {
        log_debug("Found %d recordings needing size sync", item_count);
    }

    // Now sync each recording (outside the database lock)
    for (int i = 0; i < item_count; i++) {
        // Check for shutdown
        if (is_shutdown_initiated()) {
            log_debug("Shutdown in progress, aborting recording sync after %d/%d", i, item_count);
            break;
        }

        if (items[i].file_path[0] != '\0') {
            int result = sync_recording_file_size(items[i].id, items[i].file_path);
            if (result > 0) {
                updated_count++;
            } else if (result < 0) {
                error_count++;
            }
        }
    }

    free(items);

    if (updated_count > 0 || error_count > 0) {
        log_info("Recording sync complete: %d updated, %d errors (checked %d recordings)",
                updated_count, error_count, item_count);
    }

    return updated_count;
}

/**
 * Sync thread function
 */
static void *sync_thread_func(void *arg) {
    log_info("Recording sync thread started with interval: %d seconds (syncing recordings since %ld)",
            sync_thread.interval_seconds, (long)sync_thread.startup_time);

    // Initial sync (only if not shutting down)
    if (!is_shutdown_initiated()) {
        log_debug("Performing initial recording sync");
        sync_recordings_needing_size_update();
    }

    while (sync_thread.running && !is_shutdown_initiated()) {
        // Sleep for the interval, checking for shutdown each second
        for (int i = 0; i < sync_thread.interval_seconds && sync_thread.running && !is_shutdown_initiated(); i++) {
            sleep(1);
        }

        if (!sync_thread.running || is_shutdown_initiated()) {
            break;
        }

        // Sync recordings that need size updates
        sync_recordings_needing_size_update();
    }

    log_info("Recording sync thread exiting");
    return NULL;
}

/**
 * Start the recording sync thread
 */
int start_recording_sync_thread(int interval_seconds) {
    // Initialize mutex if not already initialized
    static bool mutex_initialized = false;
    if (!mutex_initialized) {
        if (pthread_mutex_init(&sync_thread.mutex, NULL) != 0) {
            log_error("Failed to initialize recording sync thread mutex");
            return -1;
        }
        mutex_initialized = true;
    }

    pthread_mutex_lock(&sync_thread.mutex);

    // Check if thread is already running
    if (sync_thread.running) {
        log_warn("Recording sync thread is already running");
        pthread_mutex_unlock(&sync_thread.mutex);
        return 0;
    }

    // Record startup time - only sync recordings created after this time
    sync_thread.startup_time = time(NULL);

    // Set interval (minimum 10 seconds)
    sync_thread.interval_seconds = (interval_seconds < 10) ? 10 : interval_seconds;
    sync_thread.running = true;

    // Create thread
    if (pthread_create(&sync_thread.thread, NULL, sync_thread_func, NULL) != 0) {
        log_error("Failed to create recording sync thread");
        sync_thread.running = false;
        pthread_mutex_unlock(&sync_thread.mutex);
        return -1;
    }

    pthread_mutex_unlock(&sync_thread.mutex);
    log_info("Recording sync thread started with interval: %d seconds",
            sync_thread.interval_seconds);
    return 0;
}

/**
 * Stop the recording sync thread
 */
int stop_recording_sync_thread(void) {
    pthread_mutex_lock(&sync_thread.mutex);
    
    // Check if thread is running
    if (!sync_thread.running) {
        log_warn("Recording sync thread is not running");
        pthread_mutex_unlock(&sync_thread.mutex);
        return 0;
    }
    
    // Signal thread to stop
    sync_thread.running = false;
    pthread_mutex_unlock(&sync_thread.mutex);
    
    // Wait for thread to exit
    if (pthread_join(sync_thread.thread, NULL) != 0) {
        log_error("Failed to join recording sync thread");
        return -1;
    }
    
    log_info("Recording sync thread stopped");
    return 0;
}

/**
 * Force an immediate sync of recordings needing size updates
 */
int force_recording_sync(void) {
    log_info("Forcing immediate recording sync");
    return sync_recordings_needing_size_update();
}

