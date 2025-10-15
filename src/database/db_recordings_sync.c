/**
 * Database Recordings Synchronization
 * 
 * This module provides functionality to synchronize recording metadata in the database
 * with actual file sizes on disk. This ensures that the web interface displays accurate
 * file sizes even if the database wasn't updated during recording.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>
#include <pthread.h>
#include <time.h>

#include "core/logger.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"

// Thread state
static struct {
    pthread_t thread;
    bool running;
    int interval_seconds;
    pthread_mutex_t mutex;
} sync_thread = {
    .running = false,
    .interval_seconds = 60, // Default to 1 minute
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
 * Synchronize all recordings in the database with their file sizes
 */
static int sync_all_recordings(void) {
    int updated_count = 0;
    int error_count = 0;

    // First get the count of recordings
    int total_count = get_recording_count(0, 0, NULL, 0);

    if (total_count < 0) {
        log_error("Failed to get recording count from database for sync");
        return -1;
    }

    if (total_count == 0) {
        log_debug("No recordings to sync");
        return 0;
    }

    // Allocate memory for recordings
    recording_metadata_t *recordings = (recording_metadata_t *)malloc(total_count * sizeof(recording_metadata_t));
    if (!recordings) {
        log_error("Failed to allocate memory for recordings sync");
        return -1;
    }

    // Get recordings with allocated memory
    int count = get_recording_metadata_paginated(0, 0, NULL, 0, "id", "asc",
                                                recordings, total_count, 0);

    if (count < 0) {
        log_error("Failed to get recordings from database for sync");
        free(recordings);
        return -1;
    }

    // Sync each recording
    for (int i = 0; i < count; i++) {
        int result = sync_recording_file_size(recordings[i].id, recordings[i].file_path);
        if (result > 0) {
            updated_count++;
        } else if (result < 0) {
            error_count++;
        }
    }

    free(recordings);

    if (updated_count > 0 || error_count > 0) {
        log_info("Recording sync complete: %d updated, %d errors",
                updated_count, error_count);
    }

    return updated_count;
}

/**
 * Sync thread function
 */
static void *sync_thread_func(void *arg) {
    log_info("Recording sync thread started with interval: %d seconds", 
            sync_thread.interval_seconds);
    
    // Initial sync
    log_info("Performing initial recording sync");
    sync_all_recordings();
    
    while (sync_thread.running) {
        // Sleep for the interval
        for (int i = 0; i < sync_thread.interval_seconds && sync_thread.running; i++) {
            sleep(1);
        }
        
        if (!sync_thread.running) {
            break;
        }
        
        // Sync recordings
        sync_all_recordings();
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
 * Force an immediate sync of all recordings
 */
int force_recording_sync(void) {
    log_info("Forcing immediate recording sync");
    return sync_all_recordings();
}

