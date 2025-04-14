#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include "core/logger.h"
#include "video/stream_state.h"
#include "video/hls/hls_context.h"

// Forward declarations for memory management functions
extern void mark_context_as_freed(void *ctx);
extern void *safe_free(void *ptr);

// Include the unified thread header
#include "video/hls/hls_unified_thread.h"

// Hash map for tracking running HLS streaming contexts (kept for backward compatibility)
hls_stream_ctx_t *streaming_contexts[MAX_STREAMS];
pthread_mutex_t hls_contexts_mutex = PTHREAD_MUTEX_INITIALIZER;

// Hash map for tracking running unified HLS contexts
extern hls_unified_thread_ctx_t *unified_contexts[MAX_STREAMS];
extern pthread_mutex_t unified_contexts_mutex;

// State to track streams in the process of being stopped
pthread_mutex_t stopping_mutex = PTHREAD_MUTEX_INITIALIZER;
char stopping_streams[MAX_STREAMS][MAX_STREAM_NAME];
int stopping_stream_count = 0;

/**
 * Check if a stream is in the process of being stopped
 * If stream_name is NULL, checks if any stream is being stopped
 */
bool is_stream_stopping(const char *stream_name) {
    // CRITICAL FIX: If stream_name is NULL, check if any stream is stopping
    if (!stream_name) {
        // Check if any stream is in the stopping state
        pthread_mutex_lock(&stopping_mutex);
        bool any_stopping = stopping_stream_count > 0;
        pthread_mutex_unlock(&stopping_mutex);

        if (any_stopping) {
            return true;
        }

        // Also check the new state management system
        for (int i = 0; i < MAX_STREAMS; i++) {
            stream_state_manager_t *state = get_stream_state_by_index(i);
            if (state && is_stream_state_stopping(state)) {
                return true;
            }
        }

        return false;
    }

    // First try to use the new state management system
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (state) {
        return is_stream_state_stopping(state);
    }

    // Fall back to the old system if the state manager is not available
    pthread_mutex_lock(&stopping_mutex);
    bool stopping = false;
    for (int i = 0; i < stopping_stream_count; i++) {
        if (strcmp(stopping_streams[i], stream_name) == 0) {
            stopping = true;
            break;
        }
    }
    pthread_mutex_unlock(&stopping_mutex);
    return stopping;
}

/**
 * Mark a stream as being stopped
 */
void mark_stream_stopping(const char *stream_name) {
    if (!stream_name) return;

    // First try to use the new state management system
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (state) {
        // Update the state to STOPPING
        if (state->state == STREAM_STATE_ACTIVE ||
            state->state == STREAM_STATE_STARTING ||
            state->state == STREAM_STATE_RECONNECTING) {
            state->state = STREAM_STATE_STOPPING;
            log_info("Updated stream %s state to STOPPING", stream_name);
        }

        // Disable callbacks
        set_stream_callbacks_enabled(state, false);
        return;
    }

    // Fall back to the old system if the state manager is not available
    pthread_mutex_lock(&stopping_mutex);

    // Check if already in the list
    for (int i = 0; i < stopping_stream_count; i++) {
        if (strcmp(stopping_streams[i], stream_name) == 0) {
            pthread_mutex_unlock(&stopping_mutex);
            return;
        }
    }

    // Add to the list if there's space
    if (stopping_stream_count < MAX_STREAMS) {
        strncpy(stopping_streams[stopping_stream_count], stream_name, MAX_STREAM_NAME - 1);
        stopping_streams[stopping_stream_count][MAX_STREAM_NAME - 1] = '\0';
        stopping_stream_count++;
        log_info("Marked stream %s as stopping (legacy method)", stream_name);
    }

    pthread_mutex_unlock(&stopping_mutex);
}

/**
 * Unmark a stream as being stopped
 */
void unmark_stream_stopping(const char *stream_name) {
    if (!stream_name) return;

    // CRITICAL FIX: Add memory barrier before unmarking to ensure all previous operations are complete
    __sync_synchronize();

    // First try to use the new state management system
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (state) {
        // CRITICAL FIX: Use mutex to protect state changes
        pthread_mutex_lock(&state->mutex);

        // Update the state to INACTIVE if it was STOPPING
        if (state->state == STREAM_STATE_STOPPING) {
            state->state = STREAM_STATE_INACTIVE;
            log_info("Updated stream %s state from STOPPING to INACTIVE", stream_name);
        }

        // CRITICAL FIX: Set callbacks_enabled flag directly while holding the mutex
        // instead of calling set_stream_callbacks_enabled which would try to acquire the mutex again
        bool callbacks_were_disabled = !state->callbacks_enabled;
        state->callbacks_enabled = true;

        pthread_mutex_unlock(&state->mutex);

        if (callbacks_were_disabled) {
            log_info("Re-enabled callbacks for stream %s during unmark_stream_stopping", stream_name);
        }

        return;
    }

    // Fall back to the old system if the state manager is not available
    pthread_mutex_lock(&stopping_mutex);

    for (int i = 0; i < stopping_stream_count; i++) {
        if (strcmp(stopping_streams[i], stream_name) == 0) {
            // Remove by shifting remaining entries
            for (int j = i; j < stopping_stream_count - 1; j++) {
                strncpy(stopping_streams[j], stopping_streams[j + 1], MAX_STREAM_NAME);
            }
            stopping_stream_count--;
            log_info("Unmarked stream %s as stopping (legacy method)", stream_name);
            break;
        }
    }

    pthread_mutex_unlock(&stopping_mutex);
}

/**
 * Initialize the HLS context management
 */
void init_hls_contexts(void) {
    // Initialize contexts array
    memset(streaming_contexts, 0, sizeof(streaming_contexts));

    // Initialize stopping streams array
    memset(stopping_streams, 0, sizeof(stopping_streams));
    stopping_stream_count = 0;

    log_info("HLS context management initialized");
}

/**
 * Cleanup the HLS context management
 */
void cleanup_hls_contexts(void) {
    log_info("Cleaning up HLS contexts...");

    // Lock the contexts mutex to prevent race conditions
    pthread_mutex_lock(&hls_contexts_mutex);

    // Check if there are any contexts left
    bool contexts_remaining = false;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (streaming_contexts[i] != NULL) {
            contexts_remaining = true;
            break;
        }
    }

    // If there are contexts left, log a warning
    if (contexts_remaining) {
        log_warn("Found remaining HLS contexts during cleanup - these should have been cleaned up by cleanup_hls_streaming_backend");

        // Clean up any remaining contexts
        for (int i = 0; i < MAX_STREAMS; i++) {
            if (streaming_contexts[i]) {
                // Mark as not running to ensure threads exit
                atomic_store(&streaming_contexts[i]->running, 0);

                // Log that we're cleaning up this context
                log_info("Cleaning up remaining HLS context for stream %s",
                        streaming_contexts[i]->config.name);

                // Free the context

                // CRITICAL FIX: Mark the context as freed before actually freeing it
                extern void mark_context_as_freed(void *ctx);
                mark_context_as_freed(streaming_contexts[i]);

                // CRITICAL FIX: Use safe_free to free the context
                extern void *safe_free(void *ptr);
                safe_free(streaming_contexts[i]);
                streaming_contexts[i] = NULL;
            }
        }
    }

    pthread_mutex_unlock(&hls_contexts_mutex);

    // Lock the stopping mutex to reset the stopping streams array
    pthread_mutex_lock(&stopping_mutex);
    memset(stopping_streams, 0, sizeof(stopping_streams));
    stopping_stream_count = 0;
    pthread_mutex_unlock(&stopping_mutex);

    log_info("HLS contexts cleaned up");

    // Note: We don't destroy the mutexes here because they are statically initialized
    // and may be used again if the system is restarted
}
