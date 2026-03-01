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
#include <stdatomic.h>

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
extern void *hls_guarded_free(void *ptr);  // Must use this instead of safe_free for HLS contexts

/**
 * Initialize HLS streaming backend
 */
void init_hls_streaming_backend(void) {
    // Initialize the HLS contexts
    init_hls_contexts();

    // Initialize the unified contexts array
    pthread_mutex_lock(&unified_contexts_mutex);
    for (int i = 0; i < g_config.max_streams; i++) {
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
    int already_stopped_count = 0;
    for (int i = 0; i < g_config.max_streams; i++) {
        if (unified_contexts[i] != NULL) {
            // Check current thread state BEFORE modifying
            int current_state = atomic_load(&unified_contexts[i]->thread_state);

            // If thread has already stopped, don't reset its state
            if (current_state == HLS_THREAD_STOPPED) {
                already_stopped_count++;
                if (unified_contexts[i]->stream_name[0] != '\0') {
                    log_info("HLS stream %s already stopped (state=%d)",
                             unified_contexts[i]->stream_name, current_state);
                }
                continue;  // Skip to next context - thread already exited
            }

            // Mark as not running to signal threads to exit
            atomic_store(&unified_contexts[i]->running, 0);

            // Only update thread state to stopping if not already stopping or stopped
            if (current_state != HLS_THREAD_STOPPING && current_state != HLS_THREAD_STOPPED) {
                atomic_store(&unified_contexts[i]->thread_state, HLS_THREAD_STOPPING);
            }

            stream_count++;

            // Log the stream being stopped
            if (unified_contexts[i]->stream_name[0] != '\0') {
                log_info("Signaled HLS stream to stop: %s (was state=%d)",
                         unified_contexts[i]->stream_name, current_state);

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

    if (already_stopped_count > 0) {
        log_info("Found %d HLS streams already stopped", already_stopped_count);
    }
    log_info("Signaled %d HLS streams to stop", stream_count);

    // Step 3: Wait for threads to exit by polling thread_state (threads are detached)
    // CRITICAL FIX: Must wait for threads to exit BEFORE freeing contexts to prevent use-after-free
    if (stream_count > 0) {
        log_info("Waiting for %d HLS threads to exit (detached threads)...", stream_count);

        const int max_wait_ms = 5000;  // 5 second total timeout
        const int poll_interval_ms = 50;  // Check every 50ms
        int elapsed_ms = 0;

        while (elapsed_ms < max_wait_ms) {
            int threads_still_running = 0;

            pthread_mutex_lock(&unified_contexts_mutex);
            for (int i = 0; i < g_config.max_streams; i++) {
                if (unified_contexts[i] != NULL) {
                    int state = atomic_load(&unified_contexts[i]->thread_state);
                    if (state != HLS_THREAD_STOPPED) {
                        threads_still_running++;
                    }
                }
            }
            pthread_mutex_unlock(&unified_contexts_mutex);

            if (threads_still_running == 0) {
                log_info("All HLS threads have exited after %d ms", elapsed_ms);
                break;
            }

            if (elapsed_ms % 1000 == 0 && elapsed_ms > 0) {
                log_info("Still waiting for %d HLS threads to exit (%d ms elapsed)",
                        threads_still_running, elapsed_ms);
            }

            usleep(poll_interval_ms * 1000);
            elapsed_ms += poll_interval_ms;
        }

        if (elapsed_ms >= max_wait_ms) {
            log_warn("Timeout waiting for HLS threads to exit after %d ms", max_wait_ms);
        }
    }

    // Step 4: Only cleanup contexts where threads have actually exited
    // CRITICAL: Do NOT free contexts where threads are still running - this causes use-after-free crashes
    pthread_mutex_lock(&unified_contexts_mutex);
    int cleaned_count = 0;
    int skipped_count = 0;
    for (int i = 0; i < g_config.max_streams; i++) {
        if (unified_contexts[i] != NULL) {
            char stream_name_copy[MAX_STREAM_NAME] = "unknown";
            if (unified_contexts[i]->stream_name[0] != '\0') {
                strncpy(stream_name_copy, unified_contexts[i]->stream_name, MAX_STREAM_NAME - 1);
                stream_name_copy[MAX_STREAM_NAME - 1] = '\0';
            }

            // Check if thread has actually stopped
            int thread_state = atomic_load(&unified_contexts[i]->thread_state);
            if (thread_state == HLS_THREAD_STOPPED) {
                log_info("Cleaning up HLS context for stream %s (thread stopped)", stream_name_copy);

                // Memory barrier before cleanup
                __sync_synchronize();

                // Mark as freed and clean up
                mark_context_as_freed(unified_contexts[i]);
                // CRITICAL FIX: Must use hls_guarded_free instead of safe_free because
                // HLS contexts are allocated with hls_guarded_malloc which adds guard bytes.
                // Using safe_free would try to free at wrong offset, causing a crash.
                hls_guarded_free(unified_contexts[i]);
                unified_contexts[i] = NULL;
                cleaned_count++;
            } else {
                // Thread is still running - DO NOT free the context to prevent crash
                // This is a memory leak, but it's better than crashing
                log_warn("Skipping cleanup of HLS context for stream %s - thread still in state %d (would cause crash)",
                        stream_name_copy, thread_state);
                skipped_count++;
                // Leave unified_contexts[i] as-is so thread can still access it
            }
        }
    }
    pthread_mutex_unlock(&unified_contexts_mutex);

    if (cleaned_count > 0) {
        log_info("Cleaned up %d HLS contexts", cleaned_count);
    }
    if (skipped_count > 0) {
        log_warn("Skipped cleanup of %d HLS contexts (threads still running - minor memory leak to prevent crash)", skipped_count);
    }

    // Step 5: Clean up the legacy HLS contexts
    cleanup_hls_contexts();

    log_info("HLS streaming backend cleaned up");
}
