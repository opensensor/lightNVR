#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include "core/logger.h"
#include "video/hls/hls_context.h"

// Hash map for tracking running HLS streaming contexts
hls_stream_ctx_t *streaming_contexts[MAX_STREAMS];
pthread_mutex_t hls_contexts_mutex = PTHREAD_MUTEX_INITIALIZER;

// Mutex and flag to track streams in the process of being stopped
pthread_mutex_t stopping_mutex = PTHREAD_MUTEX_INITIALIZER;
char stopping_streams[MAX_STREAMS][MAX_STREAM_NAME];
int stopping_stream_count = 0;

/**
 * Check if a stream is in the process of being stopped
 */
bool is_stream_stopping(const char *stream_name) {
    if (!stream_name) return false;
    
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
        log_info("Marked stream %s as stopping", stream_name);
    }
    pthread_mutex_unlock(&stopping_mutex);
}

/**
 * Unmark a stream as being stopped
 */
void unmark_stream_stopping(const char *stream_name) {
    if (!stream_name) return;
    
    pthread_mutex_lock(&stopping_mutex);
    for (int i = 0; i < stopping_stream_count; i++) {
        if (strcmp(stopping_streams[i], stream_name) == 0) {
            // Remove by shifting remaining entries
            for (int j = i; j < stopping_stream_count - 1; j++) {
                strncpy(stopping_streams[j], stopping_streams[j + 1], MAX_STREAM_NAME);
            }
            stopping_stream_count--;
            log_info("Unmarked stream %s as stopping", stream_name);
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
    
    // Nothing to do here as the actual cleanup is done in cleanup_hls_streaming_backend
    
    log_info("HLS contexts cleaned up");
}
