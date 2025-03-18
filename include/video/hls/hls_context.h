#ifndef HLS_CONTEXT_H
#define HLS_CONTEXT_H

#include <pthread.h>
#include <stdbool.h>
#include "core/config.h"
#include "../hls_writer.h"
#include "../stream_reader.h"
#include "../hls_streaming.h"

// Hash map for tracking running HLS streaming contexts
extern hls_stream_ctx_t *streaming_contexts[MAX_STREAMS];
extern pthread_mutex_t hls_contexts_mutex;

// Mutex and flag to track streams in the process of being stopped
extern pthread_mutex_t stopping_mutex;
extern char stopping_streams[MAX_STREAMS][MAX_STREAM_NAME];
extern int stopping_stream_count;

/**
 * Check if a stream is in the process of being stopped
 * 
 * @param stream_name Name of the stream to check
 * @return true if the stream is being stopped, false otherwise
 */
bool is_stream_stopping(const char *stream_name);

/**
 * Mark a stream as being stopped
 * 
 * @param stream_name Name of the stream to mark
 */
void mark_stream_stopping(const char *stream_name);

/**
 * Unmark a stream as being stopped
 * 
 * @param stream_name Name of the stream to unmark
 */
void unmark_stream_stopping(const char *stream_name);

/**
 * Initialize the HLS context management
 */
void init_hls_contexts(void);

/**
 * Cleanup the HLS context management
 */
void cleanup_hls_contexts(void);

#endif /* HLS_CONTEXT_H */
