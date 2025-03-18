/**
 * HLS Streaming Module
 * 
 * This file serves as a thin wrapper around the HLS streaming components.
 * The actual implementation has been split into separate files for better maintainability:
 * - hls_context.c: Manages HLS streaming contexts
 * - hls_stream_thread.c: Implements the HLS streaming thread and packet callback
 * - hls_directory.c: Handles HLS directory management
 * - hls_api.c: Provides the API for starting and stopping HLS streams
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "core/logger.h"
#include "video/hls_streaming.h"
#include "video/hls/hls_context.h"
#include "video/hls/hls_stream_thread.h"
#include "video/hls/hls_directory.h"
#include "video/hls/hls_api.h"

/**
 * Initialize HLS streaming backend
 */
void init_hls_streaming_backend(void) {
    // Initialize the HLS contexts
    init_hls_contexts();
    log_info("HLS streaming backend initialized");
}

/**
 * Cleanup HLS streaming backend
 */
void cleanup_hls_streaming_backend(void) {
    log_info("Cleaning up HLS streaming backend...");
    pthread_mutex_lock(&hls_contexts_mutex);

    // Stop all running streams
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (streaming_contexts[i]) {
            log_info("Stopping HLS stream in slot %d: %s", i,
                    streaming_contexts[i]->config.name);

            // Copy the stream name for later use
            char stream_name[MAX_STREAM_NAME];
            strncpy(stream_name, streaming_contexts[i]->config.name,
                    MAX_STREAM_NAME - 1);
            stream_name[MAX_STREAM_NAME - 1] = '\0';
            
            // Unlock before stopping the stream to prevent deadlocks
            pthread_mutex_unlock(&hls_contexts_mutex);
            
            // Stop the stream
            stop_hls_stream(stream_name);
            
            // Re-lock for the next iteration
            pthread_mutex_lock(&hls_contexts_mutex);
        }
    }

    pthread_mutex_unlock(&hls_contexts_mutex);
    
    // Clean up the HLS contexts
    cleanup_hls_contexts();
    
    log_info("HLS streaming backend cleaned up");
}
