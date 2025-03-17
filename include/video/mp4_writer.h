/**
 * Header file for MP4 writer
 */

#ifndef MP4_WRITER_H
#define MP4_WRITER_H

#include <libavformat/avformat.h>

// Opaque structure for MP4 writer
typedef struct mp4_writer mp4_writer_t;

struct mp4_writer {
    char output_path[1024];
    char stream_name[64];
    AVFormatContext *output_ctx;
    int video_stream_idx;
    int has_audio;
    int64_t first_dts;
    int64_t first_pts;
    int64_t last_dts;
    AVRational time_base;
    int is_initialized;
    time_t creation_time;
    
    // Pressure indicator - set to 1 when the system is under pressure
    int is_under_pressure;
    
    // Frame counter for adaptive frame dropping
    int frame_counter;
};

/**
 * Create a new MP4 writer
 *
 * @param output_path Full path to the output MP4 file
 * @param stream_name Name of the stream (used for metadata)
 * @return A new MP4 writer instance or NULL on error
 */
mp4_writer_t *mp4_writer_create(const char *output_path, const char *stream_name);

/**
 * Write a packet to the MP4 file
 *
 * @param writer The MP4 writer instance
 * @param pkt The packet to write
 * @param input_stream The original input stream (for codec parameters)
 * @return 0 on success, negative on error
 */
int mp4_writer_write_packet(mp4_writer_t *writer, const AVPacket *pkt, const AVStream *input_stream);

/**
 * Close the MP4 writer and release resources
 *
 * @param writer The MP4 writer instance
 */
void mp4_writer_close(mp4_writer_t *writer);

#endif /* MP4_WRITER_H */
