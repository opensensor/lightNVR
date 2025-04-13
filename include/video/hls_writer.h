#ifndef HLS_WRITER_H
#define HLS_WRITER_H

#include <time.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavcodec/bsf.h>

#include "core/config.h"

// Use a different name to avoid conflict with MAX_PATH_LENGTH in config.h
#define HLS_MAX_PATH_LENGTH 1024

// Forward declaration for the DTS tracking structure
typedef struct {
    int64_t first_dts;
    int64_t last_dts;
    AVRational time_base;
    int initialized;
} stream_dts_info_t;

/**
 * HLS Writer structure
 */
typedef struct hls_writer_t {
    char output_dir[MAX_PATH_LENGTH];
    char stream_name[MAX_STREAM_NAME];
    int segment_duration;
    AVFormatContext *output_ctx;
    int initialized;
    time_t last_cleanup_time;

    // Per-stream DTS tracking
    stream_dts_info_t dts_tracker;

    // Counter for DTS jumps to detect stream issues
    int dts_jump_count;

    // Bitstream filter context for H.264 streams
    AVBSFContext *bsf_ctx;

    // Thread context for standalone operation
    void *thread_ctx;

    // Mutex for thread safety
    pthread_mutex_t mutex;
} hls_writer_t;

/**
 * Create new HLS writer
 */
hls_writer_t *hls_writer_create(const char *output_dir, const char *stream_name, int segment_duration);

/**
 * Initialize HLS writer with stream information
 */
int hls_writer_initialize(hls_writer_t *writer, const AVStream *input_stream);

/**
 * Write packet to HLS stream
 */
int hls_writer_write_packet(hls_writer_t *writer, const AVPacket *pkt, const AVStream *input_stream);

/**
 * Close HLS writer and free resources
 */
void hls_writer_close(hls_writer_t *writer);

/**
 * Clean up all HLS writers during shutdown
 * This function should be called during application shutdown
 */
void cleanup_all_hls_writers(void);

/**
 * Check if an HLS writer for a stream name already exists
 * This helps prevent duplicate writers for the same stream
 *
 * @param stream_name Name of the stream to check
 * @return Pointer to existing writer, or NULL if none exists
 */
hls_writer_t *find_hls_writer_by_stream_name(const char *stream_name);

#endif /* HLS_WRITER_H */
