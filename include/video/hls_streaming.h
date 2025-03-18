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

/**
 * Initialize HLS streaming backend
 */
void init_hls_streaming_backend(void);

/**
 * Cleanup HLS streaming backend
 */
void cleanup_hls_streaming_backend(void);

/**
 * Start HLS streaming for a stream
 * 
 * @param stream_name Name of the stream to stream
 * @return 0 on success, non-zero on failure
 */
int start_hls_stream(const char *stream_name);

/**
 * Stop HLS streaming for a stream
 * 
 * @param stream_name Name of the stream to stop streaming
 * @return 0 on success, non-zero on failure
 */
int stop_hls_stream(const char *stream_name);

/**
 * Clean up HLS directories during shutdown
 */
void cleanup_hls_directories(void);

#endif /* HLS_STREAMING_H */
