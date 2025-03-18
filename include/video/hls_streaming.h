#ifndef HLS_STREAMING_H
#define HLS_STREAMING_H

#include <pthread.h>
#include "core/config.h"
#include "video/hls_writer.h"
#include "video/stream_reader.h"

// Structure for HLS streaming context
typedef struct {
    stream_config_t config;
    int running;
    pthread_t thread;
    char output_path[MAX_PATH_LENGTH];
    hls_writer_t *hls_writer;
    stream_reader_ctx_t *reader_ctx;  // Stream reader context
} hls_stream_ctx_t;

// Include all HLS component headers
#include "video/hls/hls_context.h"
#include "video/hls/hls_stream_thread.h"
#include "video/hls/hls_directory.h"
#include "video/hls/hls_api.h"

/**
 * Initialize HLS streaming backend
 */
void init_hls_streaming_backend(void);

/**
 * Cleanup HLS streaming backend
 */
void cleanup_hls_streaming_backend(void);

#endif /* HLS_STREAMING_H */
