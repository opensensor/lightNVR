#ifndef TIMEOUT_UTILS_H
#define TIMEOUT_UTILS_H

#include <stdbool.h>
#include <time.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>

/**
 * Timeout context structure for tracking operation timeouts
 */
typedef struct {
    time_t operation_start_time;
    int timeout_seconds;
    bool timeout_occurred;
} timeout_context_t;

/**
 * Initialize a timeout context
 *
 * @param ctx The timeout context to initialize
 * @param seconds The timeout duration in seconds
 */
static inline void init_timeout(timeout_context_t *ctx, int seconds) {
    ctx->operation_start_time = time(NULL);
    ctx->timeout_seconds = seconds;
    ctx->timeout_occurred = false;
}

/**
 * Check if a timeout has occurred
 *
 * @param ctx The timeout context to check
 * @return true if timeout has occurred, false otherwise
 */
static inline bool check_timeout(timeout_context_t *ctx) {
    if (time(NULL) - ctx->operation_start_time > ctx->timeout_seconds) {
        ctx->timeout_occurred = true;
        return true;
    }
    return false;
}

/**
 * Reset a timeout context with a new duration
 *
 * @param ctx The timeout context to reset
 * @param seconds The new timeout duration in seconds
 */
static inline void reset_timeout(timeout_context_t *ctx, int seconds) {
    ctx->operation_start_time = time(NULL);
    ctx->timeout_seconds = seconds;
    ctx->timeout_occurred = false;
}

/**
 * Safely unreference a packet with extensive validation
 * This function performs multiple checks to ensure the packet is valid before unreferencing
 *
 * @param pkt Pointer to the packet to unreference
 * @param source_info String identifying the source of the call for logging
 */
void safe_packet_unref(AVPacket *pkt, const char *source_info);

/**
 * Perform comprehensive cleanup of FFmpeg resources
 * This function ensures all resources associated with an AVFormatContext are properly freed
 *
 * @param input_ctx Pointer to the AVFormatContext to clean up
 * @param codec_ctx Pointer to the AVCodecContext to clean up (can be NULL)
 * @param packet Pointer to the AVPacket to clean up (can be NULL)
 * @param frame Pointer to the AVFrame to clean up (can be NULL)
 */
void comprehensive_ffmpeg_cleanup(AVFormatContext **input_ctx, AVCodecContext **codec_ctx, AVPacket **packet, AVFrame **frame);

/**
 * Handle FFmpeg resource cleanup after a timeout
 *
 * @param url The URL of the stream that timed out
 * @param input_ctx Pointer to the AVFormatContext to clean up
 * @return AVERROR(ETIMEDOUT) to indicate timeout
 */
int handle_timeout_cleanup(const char *url, AVFormatContext **input_ctx);

#endif /* TIMEOUT_UTILS_H */
