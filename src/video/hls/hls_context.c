#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include "core/logger.h"
#include "video/stream_state.h"
#include "video/hls/hls_context.h"

// Hash map for tracking running HLS streaming contexts
hls_stream_ctx_t *streaming_contexts[MAX_STREAMS];

// State to track streams in the process of being stopped
char stopping_streams[MAX_STREAMS][MAX_STREAM_NAME];
int stopping_stream_count = 0;

/**
 * Check if a stream is in the process of being stopped
 */
bool is_stream_stopping(const char *stream_name) {
    if (!stream_name) return false;
    
    // First try to use the new state management system
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (state) {
        return is_stream_state_stopping(state);
    }
    
    // Fall back to the old system if the state manager is not available
    bool stopping = false;
    for (int i = 0; i < stopping_stream_count; i++) {
        if (strcmp(stopping_streams[i], stream_name) == 0) {
            stopping = true;
            break;
        }
    }
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
    // Check if already in the list
    for (int i = 0; i < stopping_stream_count; i++) {
        if (strcmp(stopping_streams[i], stream_name) == 0) {
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
}

/**
 * Unmark a stream as being stopped
 */
void unmark_stream_stopping(const char *stream_name) {
    if (!stream_name) return;
    
    // First try to use the new state management system
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (state) {
        // Update the state to INACTIVE if it was STOPPING
        if (state->state == STREAM_STATE_STOPPING) {
            state->state = STREAM_STATE_INACTIVE;
            log_info("Updated stream %s state from STOPPING to INACTIVE", stream_name);
        }

        // Re-enable callbacks
        set_stream_callbacks_enabled(state, true);
        return;
    }
    
    // Fall back to the old system if the state manager is not available
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
                streaming_contexts[i]->running = 0;
                
                // Log that we're cleaning up this context
                log_info("Cleaning up remaining HLS context for stream %s", 
                        streaming_contexts[i]->config.name);
                
                // Free the context
                free(streaming_contexts[i]);
                streaming_contexts[i] = NULL;
            }
        }
    }
    
    // Reset the stopping streams array
    memset(stopping_streams, 0, sizeof(stopping_streams));
    stopping_stream_count = 0;

    log_info("HLS contexts cleaned up");
}
