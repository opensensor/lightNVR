#include "video/ffmpeg_utils.h"
#include "core/logger.h"

/**
 * Log FFmpeg error
 */
void log_ffmpeg_error(int err, const char *message) {
    char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
    av_strerror(err, error_buf, AV_ERROR_MAX_STRING_SIZE);
    log_error("%s: %s", message, error_buf);
}

/**
 * Initialize FFmpeg libraries
 */
void init_ffmpeg(void) {
    // Initialize FFmpeg
#if LIBAVFORMAT_VERSION_INT < AV_VERSION_INT(58, 9, 100)
    av_register_all();
#endif
    avformat_network_init();
    log_info("FFmpeg initialized");
}

/**
 * Cleanup FFmpeg resources
 */
void cleanup_ffmpeg(void) {
    // Cleanup FFmpeg
    avformat_network_deinit();
    log_info("FFmpeg cleaned up");
}

/**
 * Safe cleanup of FFmpeg packet
 * This function provides a thorough cleanup of an AVPacket to prevent memory leaks
 *
 * @param pkt_ptr Pointer to the AVPacket pointer to clean up
 */
void safe_packet_cleanup(AVPacket **pkt_ptr) {
    if (pkt_ptr && *pkt_ptr) {
        AVPacket *pkt_to_free = *pkt_ptr;
        *pkt_ptr = NULL; // Clear the pointer first to prevent double-free

        // Safely unref and free the packet
        av_packet_unref(pkt_to_free);
        av_packet_free(&pkt_to_free);

        log_debug("Safely cleaned up packet");
    }
}

/**
 * Safe cleanup of FFmpeg AVFormatContext
 * This function provides a more thorough cleanup than just avformat_close_input
 * to help prevent memory leaks
 *
 * @param ctx_ptr Pointer to the AVFormatContext pointer to clean up
 */
void safe_avformat_cleanup(AVFormatContext **ctx_ptr) {
    if (ctx_ptr && *ctx_ptr) {
        AVFormatContext *ctx_to_close = *ctx_ptr;
        *ctx_ptr = NULL; // Clear the pointer first to prevent double-free

        // Flush buffers if available
        if (ctx_to_close->pb) {
            avio_flush(ctx_to_close->pb);
        }

        // Safely close the input context
        avformat_close_input(&ctx_to_close);

        log_debug("Safely cleaned up AVFormatContext");
    }
}

/**
 * Perform comprehensive cleanup of FFmpeg resources
 * This function ensures all resources associated with an AVFormatContext are properly freed
 *
 * @param input_ctx Pointer to the AVFormatContext to clean up
 * @param codec_ctx Pointer to the AVCodecContext to clean up (can be NULL)
 * @param packet Pointer to the AVPacket to clean up (can be NULL)
 * @param frame Pointer to the AVFrame to clean up (can be NULL)
 */
void comprehensive_ffmpeg_cleanup(AVFormatContext **input_ctx, AVCodecContext **codec_ctx, AVPacket **packet, AVFrame **frame) {
    log_debug("Starting comprehensive FFmpeg resource cleanup");

    // Clean up frame if provided
    if (frame && *frame) {
        AVFrame *frame_to_free = *frame;
        *frame = NULL; // Clear the pointer first to prevent double-free

        av_frame_free(&frame_to_free);
        log_debug("Cleaned up AVFrame");
    }

    // Clean up packet if provided
    if (packet) {
        safe_packet_cleanup(packet);
    }

    // Clean up codec context if provided
    if (codec_ctx && *codec_ctx) {
        AVCodecContext *codec_to_free = *codec_ctx;
        *codec_ctx = NULL; // Clear the pointer first to prevent double-free

        avcodec_free_context(&codec_to_free);
        log_debug("Cleaned up AVCodecContext");
    }

    // Clean up input context
    if (input_ctx) {
        safe_avformat_cleanup(input_ctx);
    }

    log_info("Comprehensive FFmpeg resource cleanup completed");
}
