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
#include <stdbool.h>

#include "core/logger.h"
#include "video/hls_streaming.h"
#include "video/hls/hls_context.h"
#include "video/hls/hls_stream_thread.h"
#include "video/hls/hls_directory.h"
#include "video/hls/hls_api.h"
#include "video/stream_state.h"

/**
 * Initialize HLS streaming backend
 */
void init_hls_streaming_backend(void) {
    // Initialize the HLS contexts
    init_hls_contexts();
    log_info("HLS streaming backend initialized");
}

/**
 * Cleanup HLS streaming backend with improved robustness
 */
void cleanup_hls_streaming_backend(void) {
    log_info("Cleaning up HLS streaming backend...");
    
    // Create a local copy of all stream names that need to be stopped
    char stream_names[MAX_STREAMS][MAX_STREAM_NAME];
    int stream_count = 0;

    // Collect all stream names first
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (streaming_contexts[i]) {
            strncpy(stream_names[stream_count], streaming_contexts[i]->config.name, MAX_STREAM_NAME - 1);
            stream_names[stream_count][MAX_STREAM_NAME - 1] = '\0';
            stream_count++;
        }
    }

    // Log how many streams we need to stop
    log_info("Found %d active HLS streams to stop during shutdown", stream_count);

    // First, mark all streams as stopping to prevent new packet processing
    for (int i = 0; i < stream_count; i++) {
        log_info("Marking HLS stream as stopping: %s", stream_names[i]);
        
        // Get the stream state manager
        stream_state_manager_t *state = get_stream_state_by_name(stream_names[i]);
        if (state) {
            // Disable callbacks to prevent new packets from being processed
            set_stream_callbacks_enabled(state, false);
            log_info("Disabled callbacks for stream %s during HLS shutdown", stream_names[i]);
        }
        
        // Mark the stream as stopping
        mark_stream_stopping(stream_names[i]);
    }
    
    // Add a small delay to allow in-progress operations to complete
    usleep(500000); // 500ms
    
    // Now stop each stream one by one with increased delays between stops
    for (int i = 0; i < stream_count; i++) {
        log_info("Stopping HLS stream: %s (%d of %d)", stream_names[i], i+1, stream_count);
        
        // Try to stop the stream with multiple attempts if needed
        int max_attempts = 3;
        bool success = false;
        
        for (int attempt = 1; attempt <= max_attempts && !success; attempt++) {
            int result = stop_hls_stream(stream_names[i]);
            if (result == 0) {
                log_info("Successfully stopped HLS stream %s on attempt %d", stream_names[i], attempt);
                success = true;
            } else if (attempt < max_attempts) {
                log_warn("Failed to stop HLS stream %s on attempt %d, retrying...", stream_names[i], attempt);
                usleep(500000); // 500ms delay before retry
            } else {
                log_error("Failed to stop HLS stream %s after %d attempts", stream_names[i], max_attempts);
            }
        }
        
        // Add a longer delay between stopping streams to avoid resource contention
        usleep(500000); // 500ms
    }
    
    // Clean up the HLS contexts with additional logging
    log_info("All HLS streams stopped, cleaning up contexts...");
    cleanup_hls_contexts();
    
    // Final verification that all contexts are cleaned up
    bool any_remaining = false;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (streaming_contexts[i] != NULL) {
            any_remaining = true;
            log_warn("HLS context for stream %s still exists after cleanup", 
                    streaming_contexts[i]->config.name);
            
            // Force cleanup of this context
            log_info("Forcing cleanup of remaining HLS context");
            free(streaming_contexts[i]);
            streaming_contexts[i] = NULL;
        }
    }
    
    if (!any_remaining) {
        log_info("All HLS contexts successfully cleaned up");
    }
    
    log_info("HLS streaming backend cleaned up");
}
