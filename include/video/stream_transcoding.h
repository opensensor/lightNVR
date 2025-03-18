#ifndef STREAM_TRANSCODING_H
#define STREAM_TRANSCODING_H

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include "core/config.h"

/**
 * Initialize FFmpeg libraries
 */
void init_transcoding_backend(void);

/**
 * Cleanup FFmpeg resources
 */
void cleanup_transcoding_backend(void);

/**
 * Open input stream with appropriate options based on protocol
 * 
 * @param input_ctx Pointer to AVFormatContext pointer to be filled
 * @param url URL of the stream to open
 * @param protocol Protocol type (STREAM_PROTOCOL_TCP or STREAM_PROTOCOL_UDP)
 * @return 0 on success, negative on error
 */
int open_input_stream(AVFormatContext **input_ctx, const char *url, int protocol);

/**
 * Find video stream index in the input context
 * 
 * @param input_ctx Input format context
 * @return Video stream index or -1 if not found
 */
int find_video_stream_index(AVFormatContext *input_ctx);

/**
 * Set the UDP flag for a stream's timestamp tracker
 * 
 * @param stream_name Name of the stream
 * @param is_udp Whether the stream is using UDP protocol
 */
void set_timestamp_tracker_udp_flag(const char *stream_name, bool is_udp);

/**
 * Process a video packet for either HLS streaming or MP4 recording
 * 
 * @param pkt The packet to process
 * @param input_stream The original input stream (for codec parameters)
 * @param writer The writer to use (either HLS or MP4)
 * @param writer_type Type of writer (0 for HLS, 1 for MP4)
 * @param stream_name Name of the stream (for logging)
 * @return 0 on success, negative on error
 */
int process_video_packet(const AVPacket *pkt, const AVStream *input_stream, 
                         void *writer, int writer_type, const char *stream_name);

/**
 * Log FFmpeg error
 * 
 * @param err Error code
 * @param message Error message prefix
 */
void log_ffmpeg_error(int err, const char *message);

/**
 * Join a thread with timeout
 * 
 * @param thread Thread to join
 * @param retval Pointer to store thread return value
 * @param timeout_sec Timeout in seconds
 * @return 0 on success, ETIMEDOUT on timeout, other error code on failure
 */
int pthread_join_with_timeout(pthread_t thread, void **retval, int timeout_sec);

#endif /* STREAM_TRANSCODING_H */
