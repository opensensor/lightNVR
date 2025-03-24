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
#include <unistd.h>

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
    
    // Create a local copy of all stream names that need to be stopped
    char stream_names[MAX_STREAMS][MAX_STREAM_NAME];
    int stream_count = 0;
    
    pthread_mutex_lock(&hls_contexts_mutex);
    
    // Collect all stream names first
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (streaming_contexts[i]) {
            strncpy(stream_names[stream_count], streaming_contexts[i]->config.name, MAX_STREAM_NAME - 1);
            stream_names[stream_count][MAX_STREAM_NAME - 1] = '\0';
            stream_count++;
        }
    }
    
    pthread_mutex_unlock(&hls_contexts_mutex);
    
    // Now stop each stream one by one
    for (int i = 0; i < stream_count; i++) {
        log_info("Stopping HLS stream: %s", stream_names[i]);
        stop_hls_stream(stream_names[i]);
        
        // Add a small delay between stopping streams to avoid resource contention
        usleep(100000); // 100ms
    }
    
    // Clean up the HLS contexts
    cleanup_hls_contexts();
    
    log_info("HLS streaming backend cleaned up");
}
