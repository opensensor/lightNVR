/**
 * MP4 Recording Core
 * 
 * This module is responsible for managing MP4 recording threads.
 * Each recording thread is responsible for starting and stopping an MP4 recorder
 * for a specific stream. The actual RTSP interaction is contained within the
 * MP4 writer module.
 */

#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <fcntl.h>
#include <errno.h>
#include <time.h>
#include <dirent.h>
#include <sys/time.h>
#include <signal.h>

#include "core/logger.h"
#include "core/config.h"
#include "core/shutdown_coordinator.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/mp4_writer.h"
#include "video/mp4_recording.h"
#include "video/mp4_recording_internal.h"
#include "video/stream_transcoding.h"
#include "video/stream_reader.h"
#include "video/stream_state.h"
#include "video/stream_packet_processor.h"
#include "database/database_manager.h"
#include "database/db_events.h"

// Hash map for tracking running MP4 recording contexts
mp4_recording_ctx_t *recording_contexts[MAX_STREAMS];

// Flag to indicate if shutdown is in progress
volatile sig_atomic_t shutdown_in_progress = 0;

// Forward declarations
static void *mp4_recording_thread(void *arg);

/**
 * MP4 recording thread function for a single stream
 * 
 * This thread is responsible for:
 * 1. Creating and managing the output directory
 * 2. Creating the MP4 writer
 * 3. Starting the RTSP recording thread in the MP4 writer
 * 4. Monitoring the recording and restarting if necessary
 * 5. Updating recording metadata
 * 6. Cleaning up resources when done
 */
static void *mp4_recording_thread(void *arg) {
    mp4_recording_ctx_t *ctx = (mp4_recording_ctx_t *)arg;
    
    // Make a local copy of the stream name for thread safety
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, ctx->config.name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    log_info("Starting MP4 recording thread for stream %s", stream_name);

    // Check if we're still running (might have been stopped during initialization)
    if (!ctx->running || shutdown_in_progress) {
        log_info("MP4 recording thread for %s exiting early due to shutdown", stream_name);
        return NULL;
    }

    // Verify output directory exists and is writable
    char mp4_dir[MAX_PATH_LENGTH];
    strncpy(mp4_dir, ctx->output_path, MAX_PATH_LENGTH - 1);
    mp4_dir[MAX_PATH_LENGTH - 1] = '\0';
    
    // Remove filename from path to get directory
    char *last_slash = strrchr(mp4_dir, '/');
    if (last_slash) {
        *last_slash = '\0';
    }
    
    struct stat st;
    if (stat(mp4_dir, &st) != 0 || !S_ISDIR(st.st_mode)) {
        log_error("Output directory does not exist or is not a directory: %s", mp4_dir);

        // Recreate it as a last resort
        char mkdir_cmd[MAX_PATH_LENGTH * 2];
        snprintf(mkdir_cmd, sizeof(mkdir_cmd), "mkdir -p %s", mp4_dir);

        int ret_mkdir = system(mkdir_cmd);
        if (ret_mkdir != 0 || stat(mp4_dir, &st) != 0 || !S_ISDIR(st.st_mode)) {
            log_error("Failed to create output directory: %s (return code: %d)", mp4_dir, ret_mkdir);
            ctx->running = 0;
            return NULL;
        }

        // Set permissions
        snprintf(mkdir_cmd, sizeof(mkdir_cmd), "chmod -R 777 %s", mp4_dir);
        int ret_chmod = system(mkdir_cmd);
        if (ret_chmod != 0) {
            log_warn("Failed to set permissions on directory: %s (return code: %d)", mp4_dir, ret_chmod);
        }
        
        log_info("Successfully created output directory: %s", mp4_dir);
    }

    // Check directory permissions
    if (access(mp4_dir, W_OK) != 0) {
        log_error("Output directory is not writable: %s", mp4_dir);

        // Try to fix permissions
        char chmod_cmd[MAX_PATH_LENGTH * 2];
        snprintf(chmod_cmd, sizeof(chmod_cmd), "chmod -R 777 %s", mp4_dir);
        int ret_chmod = system(chmod_cmd);
        if (ret_chmod != 0) {
            log_warn("Failed to set permissions on directory: %s (return code: %d)", mp4_dir, ret_chmod);
        }

        if (access(mp4_dir, W_OK) != 0) {
            log_error("Still unable to write to output directory: %s", mp4_dir);
            ctx->running = 0;
            return NULL;
        }
        
        log_info("Successfully fixed permissions for output directory: %s", mp4_dir);
    }

    // Check again if we're still running
    if (!ctx->running || shutdown_in_progress) {
        log_info("MP4 recording thread for %s exiting after directory checks due to shutdown", stream_name);
        return NULL;
    }

    // Create MP4 writer
    ctx->mp4_writer = mp4_writer_create(ctx->output_path, stream_name);
    if (!ctx->mp4_writer) {
        log_error("Failed to create MP4 writer for %s", stream_name);
        ctx->running = 0;
        return NULL;
    }
    
    log_info("Created MP4 writer for %s at %s", stream_name, ctx->output_path);
    
    // Set segment duration in the MP4 writer
    int segment_duration = ctx->config.segment_duration > 0 ? ctx->config.segment_duration : 30;
    mp4_writer_set_segment_duration(ctx->mp4_writer, segment_duration);
    log_info("Set segment duration to %d seconds for MP4 writer for stream %s", 
             segment_duration, stream_name);
    
    // Start the RTSP recording thread in the MP4 writer
    int ret = mp4_writer_start_recording_thread(ctx->mp4_writer, ctx->config.url);
    if (ret < 0) {
        log_error("Failed to start RTSP recording thread for %s", stream_name);
        mp4_writer_close(ctx->mp4_writer);
        ctx->mp4_writer = NULL;
        ctx->running = 0;
        return NULL;
    }
    
    log_info("Started RTSP recording thread for stream %s", stream_name);
    
    // Variables for periodic updates
    time_t last_update = 0;
    
    // Main loop to handle metadata updates and check if the recording is still running
    while (ctx->running && !shutdown_in_progress) {
        // Check if shutdown has been initiated
        if (is_shutdown_initiated()) {
            log_info("MP4 recording thread for %s stopping due to system shutdown", stream_name);
            ctx->running = 0;
            break;
        }
        
        // Get current time
        time_t now = time(NULL);
        
        // Periodically update recording metadata (using segment duration as a guide)
        int update_interval = ctx->config.segment_duration > 0 ? 
                             ctx->config.segment_duration : 30;
        if (now - last_update >= update_interval) {
            update_mp4_recording(stream_name);
            last_update = now;
            log_debug("Updated recording metadata for %s (interval: %d seconds)", 
                     stream_name, update_interval);
        }
        
        // Check if the RTSP recording thread is still running
        // The mp4_writer_is_recording function now checks the is_rotating flag
        // so we won't try to restart the thread during rotation
        if (ctx->mp4_writer && !mp4_writer_is_recording(ctx->mp4_writer)) {
            log_warn("RTSP recording thread for %s has stopped", stream_name);
            
            // Variables for retry mechanism
            static int retry_count = 0;
            static time_t last_retry_time = 0;
            
            // Calculate backoff time based on retry count (exponential backoff with max of 60 seconds)
            int backoff_seconds = 1 << (retry_count > 5 ? 5 : retry_count); // 1, 2, 4, 8, 16, 32, 32, ...
            if (backoff_seconds > 60) backoff_seconds = 60;
            
            // Check if we should retry now
            time_t current_time = time(NULL);
            if (last_retry_time == 0 || (current_time - last_retry_time) >= backoff_seconds) {
                log_info("Attempting to restart RTSP recording thread for %s (retry #%d, backoff: %d seconds)", 
                        stream_name, retry_count + 1, backoff_seconds);
                
                // If we've had too many failures, recreate the MP4 writer
                if (retry_count >= 3) {
                    log_warn("Multiple restart failures for %s, recreating MP4 writer", stream_name);
                    
                    // Close the existing writer
                    if (ctx->mp4_writer) {
                        mp4_writer_stop_recording_thread(ctx->mp4_writer);
                        mp4_writer_close(ctx->mp4_writer);
                        ctx->mp4_writer = NULL;
                    }
                    
                    // Create a new timestamp for the MP4 filename
                    char timestamp_str[32];
                    time_t now = time(NULL);
                    struct tm *tm_info = localtime(&now);
                    strftime(timestamp_str, sizeof(timestamp_str), "%Y%m%d_%H%M%S", tm_info);
                    
                    // Extract the directory from the existing output path
                    char mp4_dir[MAX_PATH_LENGTH];
                    strncpy(mp4_dir, ctx->output_path, MAX_PATH_LENGTH - 1);
                    mp4_dir[MAX_PATH_LENGTH - 1] = '\0';
                    
                    // Remove filename from path to get directory
                    char *last_slash = strrchr(mp4_dir, '/');
                    if (last_slash) {
                        *last_slash = '\0';
                    }
                    
                    // Create a new output path
                    snprintf(ctx->output_path, MAX_PATH_LENGTH, "%s/recording_%s.mp4",
                            mp4_dir, timestamp_str);
                    
                    // Create a new MP4 writer
                    ctx->mp4_writer = mp4_writer_create(ctx->output_path, stream_name);
                    if (!ctx->mp4_writer) {
                        log_error("Failed to recreate MP4 writer for %s", stream_name);
                        last_retry_time = current_time;
                        retry_count++;
                        continue;
                    }
                    
                    // Set segment duration in the new MP4 writer
                    int segment_duration = ctx->config.segment_duration > 0 ? ctx->config.segment_duration : 30;
                    mp4_writer_set_segment_duration(ctx->mp4_writer, segment_duration);
                    log_info("Recreated MP4 writer for %s at %s with segment duration %d seconds", 
                            stream_name, ctx->output_path, segment_duration);
                }
                
                // Try to restart the recording thread
                ret = mp4_writer_start_recording_thread(ctx->mp4_writer, ctx->config.url);
                last_retry_time = current_time;
                
                if (ret < 0) {
                    log_error("Failed to restart RTSP recording thread for %s (retry #%d)", 
                            stream_name, retry_count + 1);
                    retry_count++;
                } else {
                    log_info("Successfully restarted RTSP recording thread for %s after %d retries", 
                            stream_name, retry_count);
                    retry_count = 0;  // Reset retry count on success
                    last_retry_time = 0;  // Reset last retry time
                }
            } else {
                log_debug("Waiting %d more seconds before retrying to restart recording for %s (retry #%d)", 
                        backoff_seconds - (int)(current_time - last_retry_time), 
                        stream_name, retry_count + 1);
            }
        }
        
        // Sleep for a bit to avoid busy waiting
        sleep(1);
    }

    // When done, stop the RTSP recording thread and close the writer
    if (ctx->mp4_writer) {
        log_info("Stopping RTSP recording thread for stream %s", stream_name);
        mp4_writer_stop_recording_thread(ctx->mp4_writer);
        
        log_info("Closing MP4 writer for stream %s during thread exit", stream_name);
        mp4_writer_close(ctx->mp4_writer);
        ctx->mp4_writer = NULL;
    }

    log_info("MP4 recording thread for stream %s exited", stream_name);
    return NULL;
}

/**
 * Initialize MP4 recording backend
 * 
 * This function initializes the recording contexts array and resets the shutdown flag.
 */
void init_mp4_recording_backend(void) {
    // Initialize contexts array
    memset(recording_contexts, 0, sizeof(recording_contexts));
    
    // Reset shutdown flag
    shutdown_in_progress = 0;

    log_info("MP4 recording backend initialized");
}

/**
 * Cleanup MP4 recording backend
 * 
 * This function stops all recording threads and frees all recording contexts.
 */
void cleanup_mp4_recording_backend(void) {
    log_info("Starting MP4 recording backend cleanup");

    // Set shutdown flag to signal all threads to exit
    shutdown_in_progress = 1;

    // Create a local array to store contexts we need to clean up
    // This prevents race conditions by ensuring we handle each context safely
    typedef struct {
        mp4_recording_ctx_t *ctx;
        pthread_t thread;
        char stream_name[MAX_STREAM_NAME];
        int index;
    } cleanup_item_t;
    
    cleanup_item_t items_to_cleanup[MAX_STREAMS];
    int cleanup_count = 0;
    
    // First collect all contexts under lock
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (recording_contexts[i]) {
            // Mark as not running
            recording_contexts[i]->running = 0;
            
            // Store context info for cleanup
            items_to_cleanup[cleanup_count].ctx = recording_contexts[i];
            items_to_cleanup[cleanup_count].thread = recording_contexts[i]->thread;
            strncpy(items_to_cleanup[cleanup_count].stream_name, 
                    recording_contexts[i]->config.name, 
                    MAX_STREAM_NAME - 1);
            items_to_cleanup[cleanup_count].stream_name[MAX_STREAM_NAME - 1] = '\0';
            items_to_cleanup[cleanup_count].index = i;
            
            cleanup_count++;
        }
    }

    // Now join threads and free contexts outside the lock
    for (int i = 0; i < cleanup_count; i++) {
        // Join thread with timeout
        log_info("Waiting for MP4 recording thread for %s to exit", 
                items_to_cleanup[i].stream_name);
        
        int join_result = pthread_join_with_timeout(items_to_cleanup[i].thread, NULL, 3);
        if (join_result != 0) {
            log_warn("Could not join MP4 recording thread for %s within timeout: %s", 
                    items_to_cleanup[i].stream_name, strerror(join_result));
        } else {
            log_info("Successfully joined MP4 recording thread for %s", 
                    items_to_cleanup[i].stream_name);
        }

        // Double-check that the context is still at the expected index
        if (recording_contexts[items_to_cleanup[i].index] == items_to_cleanup[i].ctx) {
            // Free context
            free(recording_contexts[items_to_cleanup[i].index]);
            recording_contexts[items_to_cleanup[i].index] = NULL;
            log_info("Freed MP4 recording context for %s", items_to_cleanup[i].stream_name);
        } else {
            log_warn("MP4 recording context for %s was already cleaned up or moved", 
                    items_to_cleanup[i].stream_name);
        }
    }

    log_info("MP4 recording backend cleanup complete");
}

/**
 * Start MP4 recording for a stream
 */
int start_mp4_recording(const char *stream_name) {
    // Check if shutdown is in progress
    if (shutdown_in_progress) {
        log_warn("Cannot start MP4 recording for %s during shutdown", stream_name);
        return -1;
    }

    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Stream %s not found for MP4 recording", stream_name);
        return -1;
    }

    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get config for stream %s for MP4 recording", stream_name);
        return -1;
    }

    // Check if already running
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (recording_contexts[i] && strcmp(recording_contexts[i]->config.name, stream_name) == 0) {
            log_info("MP4 recording for stream %s already running", stream_name);
            return 0;  // Already running
        }
    }
    
    // MAJOR ARCHITECTURAL CHANGE: We no longer need to start the HLS streaming thread
    // since we're using a standalone recording thread that directly reads from the RTSP stream
    log_info("Using standalone recording thread for stream %s", stream_name);

    // Find empty slot
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (!recording_contexts[i]) {
            slot = i;
            break;
        }
    }

    if (slot == -1) {
        log_error("No slot available for new MP4 recording");
        return -1;
    }

    // Create context
    mp4_recording_ctx_t *ctx = malloc(sizeof(mp4_recording_ctx_t));
    if (!ctx) {
        log_error("Memory allocation failed for MP4 recording context");
        return -1;
    }

    memset(ctx, 0, sizeof(mp4_recording_ctx_t));
    memcpy(&ctx->config, &config, sizeof(stream_config_t));
    ctx->running = 1;

    // Create output paths
    config_t *global_config = get_streaming_config();

    // Create timestamp for MP4 filename
    char timestamp_str[32];
    time_t now = time(NULL);
    struct tm *tm_info = localtime(&now);
    strftime(timestamp_str, sizeof(timestamp_str), "%Y%m%d_%H%M%S", tm_info);

    // Create MP4 directory path
    char mp4_dir[MAX_PATH_LENGTH];
    if (global_config->record_mp4_directly && global_config->mp4_storage_path[0] != '\0') {
        // Use configured MP4 storage path if available
        snprintf(mp4_dir, MAX_PATH_LENGTH, "%s/%s",
                global_config->mp4_storage_path, stream_name);
    } else {
        // Use mp4 directory parallel to hls, NOT inside it
        snprintf(mp4_dir, MAX_PATH_LENGTH, "%s/mp4/%s",
                global_config->storage_path, stream_name);
    }

    // Create MP4 directory if it doesn't exist
    char dir_cmd[MAX_PATH_LENGTH * 2];
    snprintf(dir_cmd, sizeof(dir_cmd), "mkdir -p %s", mp4_dir);
    int ret = system(dir_cmd);
    if (ret != 0) {
        log_error("Failed to create MP4 directory: %s (return code: %d)", mp4_dir, ret);
        
        // Try to create the parent directory first
        char parent_dir[MAX_PATH_LENGTH];
        if (global_config->record_mp4_directly && global_config->mp4_storage_path[0] != '\0') {
            strncpy(parent_dir, global_config->mp4_storage_path, MAX_PATH_LENGTH - 1);
        } else {
            snprintf(parent_dir, MAX_PATH_LENGTH, "%s/mp4", global_config->storage_path);
        }
        
        snprintf(dir_cmd, sizeof(dir_cmd), "mkdir -p %s", parent_dir);
        ret = system(dir_cmd);
        if (ret != 0) {
            log_error("Failed to create parent MP4 directory: %s (return code: %d)", parent_dir, ret);
            free(ctx);
            return -1;
        }
        
        // Try again to create the stream-specific directory
        snprintf(dir_cmd, sizeof(dir_cmd), "mkdir -p %s", mp4_dir);
        ret = system(dir_cmd);
        if (ret != 0) {
            log_error("Still failed to create MP4 directory: %s (return code: %d)", mp4_dir, ret);
            free(ctx);
            return -1;
        }
    }

    // Set full permissions for MP4 directory
    snprintf(dir_cmd, sizeof(dir_cmd), "chmod -R 777 %s", mp4_dir);
    int ret_chmod = system(dir_cmd);
    if (ret_chmod != 0) {
        log_warn("Failed to set permissions on MP4 directory: %s (return code: %d)", mp4_dir, ret_chmod);
    }
    
    // Full path for the MP4 file
    snprintf(ctx->output_path, MAX_PATH_LENGTH, "%s/recording_%s.mp4",
             mp4_dir, timestamp_str);

    // Start recording thread
    if (pthread_create(&ctx->thread, NULL, mp4_recording_thread, ctx) != 0) {
        free(ctx);
        log_error("Failed to create MP4 recording thread for %s", stream_name);
        return -1;
    }

    // Store context
    recording_contexts[slot] = ctx;

    log_info("Started MP4 recording for %s in slot %d", stream_name, slot);

    return 0;
}

/**
 * Stop MP4 recording for a stream
 */
int stop_mp4_recording(const char *stream_name) {
    int found = 0;

    // Log that we're attempting to stop the recording
    log_info("Attempting to stop MP4 recording: %s", stream_name);

    // Find the recording context
    mp4_recording_ctx_t *ctx = NULL;
    int index = -1;

    for (int i = 0; i < MAX_STREAMS; i++) {
        if (recording_contexts[i] && strcmp(recording_contexts[i]->config.name, stream_name) == 0) {
            ctx = recording_contexts[i];
            index = i;
            found = 1;
            break;
        }
    }

    if (!found) {
        log_warn("MP4 recording for stream %s not found for stopping", stream_name);
        return -1;
    }

    // Mark as not running first
    ctx->running = 0;
    log_info("Marked MP4 recording for stream %s as stopping (index: %d)", stream_name, index);

    // Join thread with timeout
    int join_result = pthread_join_with_timeout(ctx->thread, NULL, 5);
    if (join_result != 0) {
        log_error("Failed to join thread for stream %s (error: %d), will continue cleanup",
                 stream_name, join_result);
    } else {
        log_info("Successfully joined thread for stream %s", stream_name);
    }

    // Verify context is still valid
    if (index >= 0 && index < MAX_STREAMS && recording_contexts[index] == ctx) {
        // Cleanup resources
        if (ctx->mp4_writer) {
            log_info("Closing MP4 writer for stream %s", stream_name);
            mp4_writer_close(ctx->mp4_writer);
            ctx->mp4_writer = NULL;
        }

        // Free context and clear slot
        free(ctx);
        recording_contexts[index] = NULL;

        log_info("Successfully cleaned up resources for stream %s", stream_name);
    } else {
        log_warn("Context for stream %s was modified during cleanup", stream_name);
    }

    log_info("Stopped MP4 recording for stream %s", stream_name);
    return 0;
}
