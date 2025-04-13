/**
 * HLS Streaming Module
 *
 * This file serves as a thin wrapper around the HLS streaming components.
 * The implementation uses a unified thread approach for better efficiency and reliability.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <stdbool.h>
#include <pthread.h>

#include "core/logger.h"
#include "video/hls_streaming.h"
#include "video/stream_state.h"
#include "video/hls/hls_unified_thread.h"
#include "video/hls/hls_context.h"
#include "video/hls/hls_directory.h"

// Forward declarations for the unified thread implementation
extern pthread_mutex_t unified_contexts_mutex;
extern hls_unified_thread_ctx_t *unified_contexts[MAX_STREAMS];

// Forward declarations for memory management functions
extern void mark_context_as_freed(void *ctx);
extern void *safe_free(void *ptr);

/**
 * Initialize HLS streaming backend
 */
void init_hls_streaming_backend(void) {
    // Initialize the HLS contexts
    init_hls_contexts();

    // Initialize the unified contexts array
    pthread_mutex_lock(&unified_contexts_mutex);
    for (int i = 0; i < MAX_STREAMS; i++) {
        unified_contexts[i] = NULL;
    }
    pthread_mutex_unlock(&unified_contexts_mutex);

    log_info("HLS streaming backend initialized with unified thread architecture");
}

/**
 * Cleanup HLS streaming backend with improved robustness
 */
void cleanup_hls_streaming_backend(void) {
    log_info("Cleaning up HLS streaming backend...");

    // Create a local copy of all stream names that need to be stopped
    // CRITICAL FIX: Use a more robust approach to track unique stream names
    char stream_names[MAX_STREAMS][MAX_STREAM_NAME];
    int stream_count = 0;
    bool stream_already_added[MAX_STREAMS] = {false};

    // Collect all unique stream names first with mutex protection
    pthread_mutex_lock(&unified_contexts_mutex);
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (unified_contexts[i] && unified_contexts[i]->stream_name[0] != '\0') {
            // Check if this stream name is already in our list
            bool already_added = false;
            for (int j = 0; j < stream_count; j++) {
                if (strcmp(stream_names[j], unified_contexts[i]->stream_name) == 0) {
                    already_added = true;
                    log_warn("Found duplicate HLS thread for stream %s during shutdown",
                             unified_contexts[i]->stream_name);
                    break;
                }
            }

            // Only add unique stream names to our list
            if (!already_added && stream_count < MAX_STREAMS) {
                strncpy(stream_names[stream_count], unified_contexts[i]->stream_name, MAX_STREAM_NAME - 1);
                stream_names[stream_count][MAX_STREAM_NAME - 1] = '\0';
                stream_already_added[stream_count] = true;
                stream_count++;
            }

            // Mark as not running to ensure threads exit even if stop_hls_stream fails
            atomic_store(&unified_contexts[i]->running, 0);
        }
    }
    pthread_mutex_unlock(&unified_contexts_mutex);

    // Log how many unique streams we need to stop
    log_info("Found %d active unique HLS streams to stop during shutdown", stream_count);

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
                int contexts_found = 0;
                pthread_mutex_lock(&unified_contexts_mutex);
                for (int j = 0; j < MAX_STREAMS; j++) {
                    if (unified_contexts[j] && unified_contexts[j]->stream_name[0] != '\0' &&
                        strcmp(unified_contexts[j]->stream_name, stream_names[i]) == 0) {
                        already_stopped = false;
                        contexts_found++;
                    }
                }

                // CRITICAL FIX: If we found multiple contexts for the same stream, log a warning
                if (contexts_found > 1) {
                    log_warn("Found %d HLS contexts for stream %s during shutdown",
                             contexts_found, stream_names[i]);
                }
                pthread_mutex_unlock(&unified_contexts_mutex);

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
                    pthread_mutex_lock(&unified_contexts_mutex);
                    for (int j = 0; j < MAX_STREAMS; j++) {
                        // CRITICAL FIX: Add additional safety checks to prevent segfault
                        if (unified_contexts[j] && unified_contexts[j]->stream_name[0] != '\0' &&
                            strcmp(unified_contexts[j]->stream_name, stream_names[i]) == 0) {

                            // Mark as not running
                            atomic_store(&unified_contexts[j]->running, 0);

                            // CRITICAL FIX: Add memory barrier before freeing to ensure all accesses are complete
                            __sync_synchronize();

                            // Free the context
                            log_warn("Forcing cleanup of HLS context for stream %s", stream_names[i]);

                            // CRITICAL FIX: Mark the context as freed before actually freeing it
                            extern void mark_context_as_freed(void *ctx);
                            mark_context_as_freed(unified_contexts[j]);

                            // CRITICAL FIX: Use safe_free to free the context
                            extern void *safe_free(void *ptr);
                            safe_free(unified_contexts[j]);
                            unified_contexts[j] = NULL;
                            break;
                        }
                    }
                    pthread_mutex_unlock(&unified_contexts_mutex);
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
    pthread_mutex_lock(&unified_contexts_mutex);
    bool any_remaining = false;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (unified_contexts[i] != NULL) {
            any_remaining = true;

            // CRITICAL FIX: Create a local copy of the stream name to avoid accessing potentially invalid memory
            char stream_name_copy[MAX_STREAM_NAME] = "unknown";

            // CRITICAL FIX: Add safety check before accessing stream_name to prevent segfault
            if (unified_contexts[i] && unified_contexts[i]->stream_name[0] != '\0') {
                strncpy(stream_name_copy, unified_contexts[i]->stream_name, MAX_STREAM_NAME - 1);
                stream_name_copy[MAX_STREAM_NAME - 1] = '\0';
            }

            log_warn("HLS context for stream %s still exists after cleanup", stream_name_copy);

            // Force cleanup of remaining HLS context
            log_info("Cleaning up remaining HLS context for stream %s", stream_name_copy);

            // CRITICAL FIX: Add memory barrier before freeing to ensure all accesses are complete
            __sync_synchronize();

            // Free the context

            // CRITICAL FIX: Mark the context as freed before actually freeing it
            extern void mark_context_as_freed(void *ctx);
            mark_context_as_freed(unified_contexts[i]);

            // CRITICAL FIX: Use safe_free to free the context
            extern void *safe_free(void *ptr);
            safe_free(unified_contexts[i]);
            unified_contexts[i] = NULL;
        }
    }
    pthread_mutex_unlock(&unified_contexts_mutex);

    if (!any_remaining) {
        log_info("All HLS contexts successfully cleaned up");
    }

    log_info("HLS streaming backend cleaned up");
}
