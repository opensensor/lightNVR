#ifndef FFMPEG_UTILS_H
#define FFMPEG_UTILS_H

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/dict.h>

/**
 * Log FFmpeg error
 */
void log_ffmpeg_error(int err, const char *message);

/**
 * Initialize FFmpeg libraries
 */
void init_ffmpeg(void);

/**
 * Cleanup FFmpeg resources
 */
void cleanup_ffmpeg(void);

/**
 * Safe cleanup of FFmpeg AVFormatContext
 * This function provides a more thorough cleanup than just avformat_close_input
 * to help prevent memory leaks
 *
 * @param ctx_ptr Pointer to the AVFormatContext pointer to clean up
 */
void safe_avformat_cleanup(AVFormatContext **ctx_ptr);

/**
 * Safe cleanup of FFmpeg packet
 * This function provides a thorough cleanup of an AVPacket to prevent memory leaks
 *
 * @param pkt_ptr Pointer to the AVPacket pointer to clean up
 */
void safe_packet_cleanup(AVPacket **pkt_ptr);

/**
 * Periodic FFmpeg resource reset
 * This function performs a periodic reset of FFmpeg resources to prevent memory growth
 * It should be called periodically during long-running operations
 *
 * @param input_ctx_ptr Pointer to the AVFormatContext pointer to reset
 * @param url The URL to reopen after reset
 * @param protocol The protocol to use (TCP/UDP)
 * @return 0 on success, negative value on error
 */
int periodic_ffmpeg_reset(AVFormatContext **input_ctx_ptr, const char *url, int protocol);

/**
 * Safe wrapper for av_read_frame
 * This function provides additional safety checks before calling av_read_frame
 * to help prevent segmentation faults
 *
 * @param input_ctx Pointer to the AVFormatContext to read from
 * @param pkt Pointer to the AVPacket to read into
 * @return 0 on success, negative value on error
 */
int safe_av_read_frame(AVFormatContext *input_ctx, AVPacket *pkt);

#endif /* FFMPEG_UTILS_H */
