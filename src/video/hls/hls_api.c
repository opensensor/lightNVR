#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <fcntl.h>
#include <errno.h>
#include <stdatomic.h>

#include "core/logger.h"
#include "core/config.h"
#include "video/stream_manager.h"
#include "video/stream_state.h"
#include "video/streams.h"
#include "video/hls_writer.h"
#include "video/hls_writer_thread.h"
#include "video/hls_streaming.h"
#include "video/thread_utils.h"
#include "video/timestamp_manager.h"
#include "video/hls/hls_context.h"
#include "video/hls/hls_stream_thread.h"
#include "video/hls/hls_directory.h"
#include "video/hls/hls_api.h"

/**
 * Start HLS streaming for a stream with improved reliability
 */
int start_hls_stream(const char *stream_name) {
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Stream %s not found", stream_name);
        return -1;
    }

    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get config for stream %s", stream_name);
        return -1;
    }

    // Get the stream state manager
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (!state) {
        // Don't create a new state manager here - it should be created by the stream manager
        // when the stream is added. If it doesn't exist, there's a problem with the stream.
        log_error("Stream state not found for %s", stream_name);
        return -1;
    }
    
    // Check if the stream is in the process of being stopped
    if (is_stream_state_stopping(state)) {
        log_warn("Cannot start HLS stream %s while it is in the process of being stopped", stream_name);
        return -1;
    }
    
    // Add a reference for the HLS component
    stream_state_add_ref(state, STREAM_COMPONENT_HLS);
    log_info("Added HLS reference to stream %s", stream_name);

    // Check if already running
    bool already_running = false;
    int existing_slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (streaming_contexts[i] && strcmp(streaming_contexts[i]->config.name, stream_name) == 0) {
            log_info("HLS stream %s already running", stream_name);
            return 0;  // Already running
        }
    }
    
    // Clear any existing HLS segments for this stream
    // This ensures we start with a clean slate
    log_info("Clearing any existing HLS segments for stream %s before starting", stream_name);
    clear_stream_hls_segments(stream_name);

    // Find empty slot
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (!streaming_contexts[i]) {
            slot = i;
            break;
        }
    }

    if (slot == -1) {
        log_error("No slot available for new HLS stream");
        return -1;
    }

    // Create context
    hls_stream_ctx_t *ctx = malloc(sizeof(hls_stream_ctx_t));
    if (!ctx) {
        log_error("Memory allocation failed for HLS streaming context");
        return -1;
    }

    memset(ctx, 0, sizeof(hls_stream_ctx_t));
    memcpy(&ctx->config, &config, sizeof(stream_config_t));
    atomic_store(&ctx->running, 1);  // Use atomic store for thread-safe initialization

    // Create output paths
    config_t *global_config = get_streaming_config();

    // Use storage_path_hls if specified, otherwise fall back to storage_path
    const char *base_storage_path = global_config->storage_path;
    if (global_config->storage_path_hls[0] != '\0') {
        base_storage_path = global_config->storage_path_hls;
        log_info("Using dedicated HLS storage path: %s", base_storage_path);
    } else {
        log_info("Using default storage path for HLS: %s", base_storage_path);
    }

    // Create HLS output path
    snprintf(ctx->output_path, MAX_PATH_LENGTH, "%s/hls/%s",
             base_storage_path, stream_name);

    // Create HLS directory if it doesn't exist
    char dir_cmd[MAX_PATH_LENGTH * 2];
    snprintf(dir_cmd, sizeof(dir_cmd), "mkdir -p %s", ctx->output_path);
    int ret = system(dir_cmd);
    if (ret != 0) {
        log_error("Failed to create HLS directory: %s (return code: %d)", ctx->output_path, ret);
        free(ctx);
        return -1;
    }

    // Set full permissions to ensure FFmpeg can write files
    snprintf(dir_cmd, sizeof(dir_cmd), "chmod -R 777 %s", ctx->output_path);
    if (system(dir_cmd) != 0) {
        log_warn("Failed to set permissions on HLS directory: %s", ctx->output_path);
    }

    // Also ensure the parent directory of the HLS directory exists and is writable
    char parent_dir[MAX_PATH_LENGTH];
    snprintf(parent_dir, sizeof(parent_dir), "%s/hls", global_config->storage_path);
    snprintf(dir_cmd, sizeof(dir_cmd), "mkdir -p %s && chmod -R 777 %s", 
             parent_dir, parent_dir);
    if (system(dir_cmd) != 0) {
        log_warn("Failed to create or set permissions on parent HLS directory: %s", parent_dir);
    }

    log_info("Created HLS directory with full permissions: %s", ctx->output_path);

    // Check that we can actually write to this directory
    char test_file[MAX_PATH_LENGTH];
    snprintf(test_file, sizeof(test_file), "%s/.test_write", ctx->output_path);
    FILE *test = fopen(test_file, "w");
    if (!test) {
        log_error("Directory is not writable: %s (error: %s)", ctx->output_path, strerror(errno));
        free(ctx);
        return -1;
    }
    fclose(test);
    remove(test_file);
    log_info("Verified HLS directory is writable: %s", ctx->output_path);

    // Set protocol information in the context for the hybrid approach
    ctx->config.protocol = config.protocol;
    
    // Start streaming thread
    if (pthread_create(&ctx->thread, NULL, hls_stream_thread, ctx) != 0) {
        free(ctx);
        log_error("Failed to create HLS streaming thread for %s", stream_name);
        return -1;
    }

    // Store context
    streaming_contexts[slot] = ctx;

    log_info("Started HLS stream for %s in slot %d", stream_name, slot);

    return 0;
}

/**
 * Force restart of HLS streaming for a stream
 * This is used when a stream's URL is changed to ensure the stream thread is restarted
 */
int restart_hls_stream(const char *stream_name) {
    log_info("Force restarting HLS stream for %s", stream_name);
    
    // Clear the HLS segments before stopping the stream
    // This ensures that when the stream is restarted, it will create new segments
    log_info("Clearing HLS segments for stream %s before restart", stream_name);
    clear_stream_hls_segments(stream_name);
    
    // First stop the stream if it's running
    int stop_result = stop_hls_stream(stream_name);
    if (stop_result != 0) {
        log_warn("Failed to stop HLS stream %s for restart, continuing anyway", stream_name);
    }
    
    // Wait a bit to ensure resources are released
    usleep(500000); // 500ms
    
    // Verify that the HLS directory exists and is writable
    config_t *global_config = get_streaming_config();
    if (global_config) {
        // Use storage_path_hls if specified, otherwise fall back to storage_path
        const char *base_storage_path = global_config->storage_path;
        if (global_config->storage_path_hls[0] != '\0') {
            base_storage_path = global_config->storage_path_hls;
            log_info("Using dedicated HLS storage path for restart: %s", base_storage_path);
        }
        
        char hls_dir[MAX_PATH_LENGTH];
        snprintf(hls_dir, MAX_PATH_LENGTH, "%s/hls/%s", 
                base_storage_path, stream_name);
        
        // Ensure the directory exists and has proper permissions
        log_info("Ensuring HLS directory exists and is writable: %s", hls_dir);
        ensure_hls_directory(hls_dir, stream_name);
    }
    
    // Start the stream again
    int start_result = start_hls_stream(stream_name);
    if (start_result != 0) {
        log_error("Failed to restart HLS stream %s", stream_name);
        return -1;
    }
    
    log_info("Successfully restarted HLS stream %s", stream_name);
    return 0;
}

/**
 * Stop HLS streaming for a stream
 */
int stop_hls_stream(const char *stream_name) {
    int found = 0;

    // Log that we're attempting to stop the stream
    log_info("Attempting to stop HLS stream: %s", stream_name);
    
    // Get the stream state manager
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (state) {
        // Only disable callbacks if we're actually stopping the stream
        // This prevents disabling callbacks when the stream doesn't exist
        bool found = false;
        for (int i = 0; i < MAX_STREAMS; i++) {
            if (streaming_contexts[i] && strcmp(streaming_contexts[i]->config.name, stream_name) == 0) {
                found = true;
                break;
            }
        }

        if (found) {
            // Disable callbacks to prevent new packets from being processed
            set_stream_callbacks_enabled(state, false);
            log_info("Disabled callbacks for stream %s during HLS shutdown", stream_name);
        } else {
            log_info("Stream %s not found in HLS contexts, not disabling callbacks", stream_name);
        }
    }

    // Find the stream context with mutex protection
    pthread_mutex_lock(&hls_contexts_mutex);
    
    hls_stream_ctx_t *ctx = NULL;
    int index = -1;

    for (int i = 0; i < MAX_STREAMS; i++) {
        if (streaming_contexts[i] && strcmp(streaming_contexts[i]->config.name, stream_name) == 0) {
            ctx = streaming_contexts[i];
            index = i;
            found = 1;
            break;
        }
    }
    
    // If not found, unlock and return
    if (!found) {
        pthread_mutex_unlock(&hls_contexts_mutex);
        log_warn("HLS stream %s not found for stopping", stream_name);
        // Don't re-enable callbacks here - the stream wasn't found so we never disabled them
        return -1;
    }
    
    // Check if the stream is already stopped
    if (!atomic_load(&ctx->running)) {
        pthread_mutex_unlock(&hls_contexts_mutex);
        log_warn("HLS stream %s is already stopped", stream_name);
        // Don't re-enable callbacks here - the stream is already stopped
        return 0;
    }
    
    // Mark as stopping in the global stopping list to prevent race conditions
    mark_stream_stopping(stream_name);
    
    // Now mark as not running using atomic store for thread safety
    atomic_store(&ctx->running, 0);
    log_info("Marked HLS stream %s as stopping (index: %d)", stream_name, index);
    
    // Store a local copy of the thread to join
    pthread_t thread_to_join = ctx->thread;
    
    // Reset the timestamp tracker for this stream to ensure clean state when restarted
    // This is especially important for UDP streams
    reset_timestamp_tracker(stream_name);
    log_info("Reset timestamp tracker for stream %s", stream_name);
    
    // Unlock the mutex before joining the thread to prevent deadlocks
    pthread_mutex_unlock(&hls_contexts_mutex);
    
    // Join thread with a longer timeout (15 seconds instead of 10)
    int join_result = pthread_join_with_timeout(thread_to_join, NULL, 15);
    if (join_result != 0) {
        log_error("Failed to join thread for stream %s (error: %d), will continue cleanup",
                 stream_name, join_result);
        
        // Even if we couldn't join the thread, we need to clean up resources
        // The thread will eventually exit when it checks ctx->running
    } else {
        log_info("Successfully joined thread for stream %s", stream_name);
    }
    
    // Re-acquire the mutex for cleanup
    pthread_mutex_lock(&hls_contexts_mutex);
    
    // Verify context is still valid
    if (index >= 0 && index < MAX_STREAMS && streaming_contexts[index] == ctx) {
        // Cleanup resources - make a local copy of the writer pointer
        hls_writer_t *writer_to_close = ctx->hls_writer;
        ctx->hls_writer = NULL;  // Clear the pointer first to prevent double close
        
        // Free context and clear slot before closing the writer
        // This ensures no other thread can access the context while we're closing the writer
        streaming_contexts[index] = NULL;
        
        // Unlock the mutex before closing the writer to prevent deadlocks
        pthread_mutex_unlock(&hls_contexts_mutex);
        
        // Free the context structure
        free(ctx);
        
        // Now close the writer after the context has been freed
        if (writer_to_close) {
            log_info("Closing HLS writer for stream %s", stream_name);
            
            // First stop any running recording thread
            hls_writer_stop_recording_thread(writer_to_close);
            
            // Add a small delay to ensure the thread has fully stopped
            usleep(100000); // 100ms
            
            // Now safely close the writer
            hls_writer_close(writer_to_close);
            writer_to_close = NULL;
        }

        log_info("Successfully cleaned up resources for stream %s", stream_name);
    } else {
        pthread_mutex_unlock(&hls_contexts_mutex);
        log_warn("Context for stream %s was modified during cleanup", stream_name);
    }
    
    // Remove from the stopping list
    unmark_stream_stopping(stream_name);

    log_info("Stopped HLS stream %s", stream_name);
    
    // Release the HLS reference
    if (state) {
        // Re-enable callbacks before releasing the reference
        set_stream_callbacks_enabled(state, true);
        log_info("Re-enabled callbacks for stream %s after HLS shutdown", stream_name);
        
        stream_state_release_ref(state, STREAM_COMPONENT_HLS);
        log_info("Released HLS reference to stream %s", stream_name);
    }
    
    return 0;
}
