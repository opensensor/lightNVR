#ifndef HLS_STREAMING_H
#define HLS_STREAMING_H

#include <pthread.h>
#include <stdatomic.h>
#include "core/config.h"
#include "video/hls_writer.h"
#include "video/stream_reader.h"

// Structure for HLS streaming context (kept for backward compatibility)
typedef struct {
    stream_config_t config;
    atomic_int running;  // Use atomic_int for thread-safe access
    pthread_t thread;
    char output_path[MAX_PATH_LENGTH];
    hls_writer_t *hls_writer;
} hls_stream_ctx_t;

// Include all HLS component headers
#include "video/hls/hls_context.h"
#include "video/hls/hls_directory.h"
#include "video/hls/hls_unified_thread.h"

/**
 * Initialize HLS streaming backend
 */
void init_hls_streaming_backend(void);

/**
 * Cleanup HLS streaming backend
 */
void cleanup_hls_streaming_backend(void);

#endif /* HLS_STREAMING_H */
