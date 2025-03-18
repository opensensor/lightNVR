#ifndef FFMPEG_UTILS_H
#define FFMPEG_UTILS_H

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include "video/stream_protocol.h"

/**
 * Initialize FFmpeg library
 */
void init_ffmpeg(void);

/**
 * Cleanup FFmpeg library
 */
void cleanup_ffmpeg(void);

/**
 * Open an input stream with the specified URL and protocol
 * 
 * @param ctx Pointer to AVFormatContext pointer to store the opened context
 * @param url URL of the stream to open
 * @param protocol Protocol to use for opening the stream
 * @return 0 on success, non-zero on failure
 */
int open_input_stream(AVFormatContext **ctx, const char *url, stream_protocol_t protocol);

/**
 * Find the index of the video stream in the format context
 * 
 * @param ctx AVFormatContext to search for video stream
 * @return Index of the video stream, or -1 if not found
 */
int find_video_stream_index(AVFormatContext *ctx);

/**
 * Log an FFmpeg error with the given message
 * 
 * @param err FFmpeg error code
 * @param msg Message to log with the error
 */
void log_ffmpeg_error(int err, const char *msg);

#endif /* FFMPEG_UTILS_H */
