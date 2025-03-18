#ifndef HLS_WRITER_H
#define HLS_WRITER_H

#include <time.h>
#include <libavformat/avformat.h>

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

#endif /* HLS_WRITER_H */
