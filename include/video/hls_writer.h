#ifndef HLS_WRITER_H
#define HLS_WRITER_H

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>

#define MAX_PATH_LENGTH 256
#define MAX_STREAM_NAME 64

/**
 * HLS Writer structure
 */
typedef struct {
    char output_dir[MAX_PATH_LENGTH];
    char stream_name[MAX_STREAM_NAME];
    int segment_duration;
    int initialized;
    AVFormatContext *output_ctx;
} hls_writer_t;

/**
 * Create new HLS writer
 */
hls_writer_t *hls_writer_create(const char *output_dir, const char *stream_name, int segment_duration);

/**
 * Write packet to HLS stream
 */
int hls_writer_write_packet(hls_writer_t *writer, const AVPacket *pkt, const AVStream *input_stream);

/**
 * Initialize HLS writer with stream information
 */
int hls_writer_initialize(hls_writer_t *writer, const AVStream *input_stream);

/**
 * Close HLS writer and free resources
 */
void hls_writer_close(hls_writer_t *writer);

#endif /* HLS_WRITER_H */