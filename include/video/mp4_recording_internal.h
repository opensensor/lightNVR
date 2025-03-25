#ifndef MP4_RECORDING_INTERNAL_H
#define MP4_RECORDING_INTERNAL_H

#include <libavformat/avformat.h>
#include "video/mp4_recording.h"

// Maximum number of frames to pre-buffer (about 2 seconds at 30fps)
#define MAX_PREBUFFER_FRAMES 60

// Structure for buffered packet
typedef struct {
    AVPacket *packet;
    AVRational time_base;
} buffered_packet_t;

// Structure for frame buffer
typedef struct {
    buffered_packet_t *frames;
    int capacity;
    int count;
    int head;
    int tail;
    pthread_mutex_t mutex;
} frame_buffer_t;

// External declarations for shared variables
extern mp4_recording_ctx_t *recording_contexts[MAX_STREAMS];
extern mp4_writer_t *mp4_writers[MAX_STREAMS];
extern char mp4_writer_stream_names[MAX_STREAMS][64];
extern frame_buffer_t frame_buffers[MAX_STREAMS];

// Frame buffer functions
int init_frame_buffer(const char *stream_name, int capacity);
void add_to_frame_buffer(int buffer_idx, const AVPacket *pkt, const AVStream *stream);
void flush_frame_buffer(int buffer_idx, mp4_writer_t *writer);
void free_frame_buffer(int buffer_idx);

// Add a packet to the pre-buffer for a stream
void add_packet_to_prebuffer(const char *stream_name, const AVPacket *pkt, const AVStream *stream);

// Flush the pre-buffered frames to the MP4 writer
void flush_prebuffer_to_mp4(const char *stream_name);

#endif /* MP4_RECORDING_INTERNAL_H */
