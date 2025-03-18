#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <fcntl.h>
#include <errno.h>

#include "core/logger.h"
#include "core/config.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/hls_writer.h"
#include "video/hls_streaming.h"
#include "video/thread_utils.h"
#include "video/timestamp_manager.h"
#include "video/hls/hls_context.h"
#include "video/hls/hls_stream_thread.h"
#include "video/hls/hls_directory.h"
#include "video/hls/hls_api.h"

/**
 * Start HLS streaming for a stream
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

    // CRITICAL FIX: Check if the stream is in the process of being stopped
    if (is_stream_stopping(stream_name)) {
        log_warn("Cannot start HLS stream %s while it is in the process of being stopped", stream_name);
        return -1;
    }

    // Check if already running
    pthread_mutex_lock(&hls_contexts_mutex);
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (streaming_contexts[i] && strcmp(streaming_contexts[i]->config.name, stream_name) == 0) {
            pthread_mutex_unlock(&hls_contexts_mutex);
            log_info("HLS stream %s already running", stream_name);
            return 0;  // Already running
        }
    }

    // Find empty slot
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (!streaming_contexts[i]) {
            slot = i;
            break;
        }
    }

    if (slot == -1) {
        pthread_mutex_unlock(&hls_contexts_mutex);
        log_error("No slot available for new HLS stream");
        return -1;
    }

    // Create context
    hls_stream_ctx_t *ctx = malloc(sizeof(hls_stream_ctx_t));
    if (!ctx) {
        pthread_mutex_unlock(&hls_contexts_mutex);
        log_error("Memory allocation failed for HLS streaming context");
        return -1;
    }

    memset(ctx, 0, sizeof(hls_stream_ctx_t));
    memcpy(&ctx->config, &config, sizeof(stream_config_t));
    ctx->running = 1;

    // Create output paths
    config_t *global_config = get_streaming_config();

    // Log the storage path for debugging
    log_info("Using storage path for HLS: %s", global_config->storage_path);

    // Create HLS output path
    snprintf(ctx->output_path, MAX_PATH_LENGTH, "%s/hls/%s",
             global_config->storage_path, stream_name);

    // Create HLS directory if it doesn't exist
    char dir_cmd[MAX_PATH_LENGTH * 2];
    snprintf(dir_cmd, sizeof(dir_cmd), "mkdir -p %s", ctx->output_path);
    int ret = system(dir_cmd);
    if (ret != 0) {
        log_error("Failed to create HLS directory: %s (return code: %d)", ctx->output_path, ret);
        free(ctx);
        pthread_mutex_unlock(&hls_contexts_mutex);
        return -1;
    }

    // Set full permissions to ensure FFmpeg can write files
    snprintf(dir_cmd, sizeof(dir_cmd), "chmod -R 777 %s", ctx->output_path);
    system(dir_cmd);

    // Also ensure the parent directory of the HLS directory exists and is writable
    char parent_dir[MAX_PATH_LENGTH];
    snprintf(parent_dir, sizeof(parent_dir), "%s/hls", global_config->storage_path);
    snprintf(dir_cmd, sizeof(dir_cmd), "mkdir -p %s && chmod -R 777 %s", 
             parent_dir, parent_dir);
    system(dir_cmd);

    log_info("Created HLS directory with full permissions: %s", ctx->output_path);

    // Check that we can actually write to this directory
    char test_file[MAX_PATH_LENGTH];
    snprintf(test_file, sizeof(test_file), "%s/.test_write", ctx->output_path);
    FILE *test = fopen(test_file, "w");
    if (!test) {
        log_error("Directory is not writable: %s (error: %s)", ctx->output_path, strerror(errno));
        free(ctx);
        pthread_mutex_unlock(&hls_contexts_mutex);
        return -1;
    }
    fclose(test);
    remove(test_file);
    log_info("Verified HLS directory is writable: %s", ctx->output_path);

    // Start streaming thread
    if (pthread_create(&ctx->thread, NULL, hls_stream_thread, ctx) != 0) {
        free(ctx);
        pthread_mutex_unlock(&hls_contexts_mutex);
        log_error("Failed to create HLS streaming thread for %s", stream_name);
        return -1;
    }

    // Store context
    streaming_contexts[slot] = ctx;
    pthread_mutex_unlock(&hls_contexts_mutex);

    log_info("Started HLS stream for %s in slot %d", stream_name, slot);

    return 0;
}

/**
 * Stop HLS streaming for a stream
 */
int stop_hls_stream(const char *stream_name) {
    int found = 0;

    // Log that we're attempting to stop the stream
    log_info("Attempting to stop HLS stream: %s", stream_name);
    
    // CRITICAL FIX: Mark the stream as being stopped to prevent new packets from being processed
    mark_stream_stopping(stream_name);
    
    // CRITICAL FIX: Use a static mutex to prevent concurrent access during stopping
    static pthread_mutex_t stop_mutex = PTHREAD_MUTEX_INITIALIZER;
    pthread_mutex_lock(&stop_mutex);

    pthread_mutex_lock(&hls_contexts_mutex);

    // Find the stream context
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

    if (!found) {
        log_warn("HLS stream %s not found for stopping", stream_name);
        pthread_mutex_unlock(&hls_contexts_mutex);
        pthread_mutex_unlock(&stop_mutex);
        unmark_stream_stopping(stream_name); // CRITICAL FIX: Unmark the stream
        return -1;
    }
    
    // CRITICAL FIX: Check if the stream is already stopped
    if (!ctx->running) {
        log_warn("HLS stream %s is already stopped", stream_name);
        pthread_mutex_unlock(&hls_contexts_mutex);
        pthread_mutex_unlock(&stop_mutex);
        unmark_stream_stopping(stream_name); // CRITICAL FIX: Unmark the stream
        return 0;
    }

    // CRITICAL FIX: First clear the packet callback to prevent any further processing
    // This must be done before marking the stream as not running to prevent race conditions
    if (ctx->reader_ctx) {
        log_info("Clearing packet callback for HLS stream %s", stream_name);
        set_packet_callback(ctx->reader_ctx, NULL, NULL);
        
        // CRITICAL FIX: Add a small delay to ensure any in-progress callbacks complete
        usleep(50000); // 50ms delay
    }
    
    // Now mark as not running
    ctx->running = 0;
    log_info("Marked HLS stream %s as stopping (index: %d)", stream_name, index);
    
    // Reset the timestamp tracker for this stream to ensure clean state when restarted
    // This is especially important for UDP streams
    reset_timestamp_tracker(stream_name);
    log_info("Reset timestamp tracker for stream %s", stream_name);
    
    // Store a local copy of the thread to join
    pthread_t thread_to_join = ctx->thread;

    // Unlock before joining thread to prevent deadlocks
    pthread_mutex_unlock(&hls_contexts_mutex);

    // Join thread with timeout
    int join_result = pthread_join_with_timeout(thread_to_join, NULL, 5);
    if (join_result != 0) {
        log_error("Failed to join thread for stream %s (error: %d), will continue cleanup",
                 stream_name, join_result);
    } else {
        log_info("Successfully joined thread for stream %s", stream_name);
    }

    // Re-lock for cleanup
    pthread_mutex_lock(&hls_contexts_mutex);

    // Verify context is still valid
    if (index >= 0 && index < MAX_STREAMS && streaming_contexts[index] == ctx) {
        // Cleanup resources
        if (ctx->hls_writer) {
            log_info("Closing HLS writer for stream %s", stream_name);
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }
        
        // Stop the dedicated stream reader if it exists
        if (ctx->reader_ctx) {
            log_info("Stopping dedicated stream reader for HLS stream %s", stream_name);
            // CRITICAL FIX: We already cleared the callback above, so the reader should be safe to stop now
            stop_stream_reader(ctx->reader_ctx);
            ctx->reader_ctx = NULL;
            log_info("Successfully stopped dedicated stream reader for HLS stream %s", stream_name);
        }

        // Free context and clear slot
        free(ctx);
        streaming_contexts[index] = NULL;

        log_info("Successfully cleaned up resources for stream %s", stream_name);
    } else {
        log_warn("Context for stream %s was modified during cleanup", stream_name);
    }

    pthread_mutex_unlock(&hls_contexts_mutex);
    pthread_mutex_unlock(&stop_mutex);

    log_info("Stopped HLS stream %s", stream_name);
    return 0;
}
