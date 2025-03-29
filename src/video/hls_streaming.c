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

    // Collect all stream names first with mutex protection
    pthread_mutex_lock(&hls_contexts_mutex);
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (streaming_contexts[i]) {
            strncpy(stream_names[stream_count], streaming_contexts[i]->config.name, MAX_STREAM_NAME - 1);
            stream_names[stream_count][MAX_STREAM_NAME - 1] = '\0';
            
            // Mark as not running to ensure threads exit even if stop_hls_stream fails
            atomic_store(&streaming_contexts[i]->running, 0);
            
            // Also clear the HLS writer pointer to prevent double-free issues
            if (streaming_contexts[i]->hls_writer) {
                log_info("Marking HLS writer for stream %s as pending cleanup", 
                        streaming_contexts[i]->config.name);
            }
            
            stream_count++;
        }
    }
    pthread_mutex_unlock(&hls_contexts_mutex);

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
            
            // Update the state to STOPPING
            if (state->state != STREAM_STATE_STOPPING && 
                state->state != STREAM_STATE_INACTIVE) {
                state->state = STREAM_STATE_STOPPING;
                log_info("Updated stream %s state to STOPPING", stream_names[i]);
            }
        } else {
            log_warn("Could not find stream state for %s during HLS shutdown", stream_names[i]);
        }
        
        // Mark the stream as stopping in our tracking system
        mark_stream_stopping(stream_names[i]);
        
        // Double-check that callbacks are disabled
        if (state) {
            set_stream_callbacks_enabled(state, false);
            log_info("Callbacks disabled for stream '%s'", stream_names[i]);
        }
    }
    
    // Add a small delay to allow in-progress operations to complete
    usleep(1000000); // 1000ms
    
    // Now stop each stream one by one with increased delays between stops
    for (int i = 0; i < stream_count; i++) {
        log_info("Stopping HLS stream: %s (%d of %d)", stream_names[i], i+1, stream_count);
        
        // Try to stop the stream with multiple attempts if needed
        int max_attempts = 3;
        bool success = false;
        
        for (int attempt = 1; attempt <= max_attempts && !success; attempt++) {
            // Get the stream state manager again to ensure we have the latest state
            stream_state_manager_t *state = get_stream_state_by_name(stream_names[i]);
            if (state) {
                // Disable callbacks again to be extra sure
                set_stream_callbacks_enabled(state, false);
                log_info("Disabled callbacks for stream '%s'", stream_names[i]);
            }
            
            log_info("Attempting to stop HLS stream: %s", stream_names[i]);
            int result = stop_hls_stream(stream_names[i]);
            if (result == 0) {
                log_info("Successfully stopped HLS stream %s on attempt %d", stream_names[i], attempt);
                success = true;
                break;
            } else {
                // Check if the stream is already stopped
                bool already_stopped = true;
                pthread_mutex_lock(&hls_contexts_mutex);
                for (int j = 0; j < MAX_STREAMS; j++) {
                    if (streaming_contexts[j] && 
                        strcmp(streaming_contexts[j]->config.name, stream_names[i]) == 0) {
                        already_stopped = false;
                        break;
                    }
                }
                pthread_mutex_unlock(&hls_contexts_mutex);
                
                if (already_stopped) {
                    log_warn("HLS stream %s is already stopped", stream_names[i]);
                    success = true;
                    break;
                } else if (attempt < max_attempts) {
                    log_warn("Failed to stop HLS stream %s on attempt %d, retrying...", stream_names[i], attempt);
                    usleep(1000000); // 1000ms delay before retry
                } else {
                    log_error("Failed to stop HLS stream %s after %d attempts", stream_names[i], max_attempts);
                    
                    // Force cleanup of this stream's context
                    pthread_mutex_lock(&hls_contexts_mutex);
                    for (int j = 0; j < MAX_STREAMS; j++) {
                        if (streaming_contexts[j] && 
                            strcmp(streaming_contexts[j]->config.name, stream_names[i]) == 0) {
                            
                            // Mark as not running
                            atomic_store(&streaming_contexts[j]->running, 0);
                            
                            // Close the HLS writer if it exists
                            if (streaming_contexts[j]->hls_writer) {
                                log_warn("Forcing close of HLS writer for stream %s", stream_names[i]);
                                hls_writer_close(streaming_contexts[j]->hls_writer);
                                streaming_contexts[j]->hls_writer = NULL;
                            }
                            
                            // Free the context
                            log_warn("Forcing cleanup of HLS context for stream %s", stream_names[i]);
                            free(streaming_contexts[j]);
                            streaming_contexts[j] = NULL;
                            break;
                        }
                    }
                    pthread_mutex_unlock(&hls_contexts_mutex);
                }
            }
        }
        
        // Add a longer delay between stopping streams to avoid resource contention
        usleep(1000000); // 1000ms
    }
    
    // Clean up the HLS contexts with additional logging
    log_info("All HLS streams stopped, cleaning up contexts...");
    cleanup_hls_contexts();
    
    // Final verification that all contexts are cleaned up with mutex protection
    pthread_mutex_lock(&hls_contexts_mutex);
    bool any_remaining = false;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (streaming_contexts[i] != NULL) {
            any_remaining = true;
            log_warn("HLS context for stream %s still exists after cleanup", 
                    streaming_contexts[i]->config.name);
            
            // Force cleanup of remaining HLS context
            log_info("Cleaning up remaining HLS context for stream %s", 
                    streaming_contexts[i]->config.name);
            
            // Close the HLS writer if it exists
            if (streaming_contexts[i]->hls_writer) {
                log_warn("Forcing close of remaining HLS writer for stream %s", 
                        streaming_contexts[i]->config.name);
                hls_writer_close(streaming_contexts[i]->hls_writer);
                streaming_contexts[i]->hls_writer = NULL;
            }
            
            // Free the context
            free(streaming_contexts[i]);
            streaming_contexts[i] = NULL;
        }
    }
    pthread_mutex_unlock(&hls_contexts_mutex);
    
    if (!any_remaining) {
        log_info("All HLS contexts successfully cleaned up");
    }
    
    log_info("HLS streaming backend cleaned up");
}
