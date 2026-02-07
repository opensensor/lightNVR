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
 * Cleanup HLS streaming backend - optimized for fast shutdown
 */
void cleanup_hls_streaming_backend(void) {
    log_info("Cleaning up HLS streaming backend (optimized)...");

    // Step 1: Stop the watchdog FIRST to prevent any stream restarts during shutdown
    stop_hls_watchdog();

    // Step 2: Mark ALL contexts as not running in a single pass (with mutex held)
    pthread_mutex_lock(&unified_contexts_mutex);
    int stream_count = 0;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (unified_contexts[i] != NULL) {
            // Mark as not running to signal threads to exit
            atomic_store(&unified_contexts[i]->running, 0);

            // Also update thread state to stopping
            atomic_store(&unified_contexts[i]->thread_state, HLS_THREAD_STOPPING);

            stream_count++;

            // Log the stream being stopped
            if (unified_contexts[i]->stream_name[0] != '\0') {
                log_info("Signaled HLS stream to stop: %s", unified_contexts[i]->stream_name);

                // Mark as stopping in stream state system
                stream_state_manager_t *state = get_stream_state_by_name(unified_contexts[i]->stream_name);
                if (state) {
                    set_stream_callbacks_enabled(state, false);
                    if (state->state != STREAM_STATE_STOPPING && state->state != STREAM_STATE_INACTIVE) {
                        state->state = STREAM_STATE_STOPPING;
                    }
                }
            }
        }
    }
    pthread_mutex_unlock(&unified_contexts_mutex);

    log_info("Signaled %d HLS streams to stop", stream_count);

    // Step 3: Brief wait to allow threads to begin shutdown (reduced from 1000ms)
    if (stream_count > 0) {
        usleep(100000); // 100ms - just enough for threads to notice the flag
    }

    // Step 4: Force cleanup of all remaining contexts
    // Don't wait for threads - they will clean up on their own, but we need to proceed
    pthread_mutex_lock(&unified_contexts_mutex);
    int cleaned_count = 0;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (unified_contexts[i] != NULL) {
            char stream_name_copy[MAX_STREAM_NAME] = "unknown";
            if (unified_contexts[i]->stream_name[0] != '\0') {
                strncpy(stream_name_copy, unified_contexts[i]->stream_name, MAX_STREAM_NAME - 1);
                stream_name_copy[MAX_STREAM_NAME - 1] = '\0';
            }

            log_info("Force cleanup of HLS context for stream %s", stream_name_copy);

            // Ensure running flag is cleared
            atomic_store(&unified_contexts[i]->running, 0);

            // Memory barrier before cleanup
            __sync_synchronize();

            // Mark as freed and clean up
            mark_context_as_freed(unified_contexts[i]);
            safe_free(unified_contexts[i]);
            unified_contexts[i] = NULL;
            cleaned_count++;
        }
    }
    pthread_mutex_unlock(&unified_contexts_mutex);

    if (cleaned_count > 0) {
        log_info("Force cleaned %d remaining HLS contexts", cleaned_count);
    }

    // Step 5: Clean up the legacy HLS contexts
    cleanup_hls_contexts();

    log_info("HLS streaming backend cleaned up");
}
