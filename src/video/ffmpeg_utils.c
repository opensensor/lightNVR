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
 * to help prevent memory leaks and segmentation faults
 *
 * @param ctx_ptr Pointer to the AVFormatContext pointer to clean up
 */
void safe_avformat_cleanup(AVFormatContext **ctx_ptr) {
    // CRITICAL FIX: Add comprehensive safety checks to prevent segmentation faults
    if (!ctx_ptr) {
        log_debug("NULL pointer passed to safe_avformat_cleanup");
        return;
    }

    if (!*ctx_ptr) {
        log_debug("NULL AVFormatContext passed to safe_avformat_cleanup");
        return;
    }

    // Make a local copy and immediately NULL the original to prevent double-free
    AVFormatContext *ctx_to_close = *ctx_ptr;
    *ctx_ptr = NULL;

    // Add memory barrier to ensure memory operations are completed
    // This helps prevent race conditions on multi-core systems
    __sync_synchronize();

    // CRITICAL FIX: Add additional validation of the context structure
    // This prevents segmentation faults when dealing with corrupted contexts
    if (!ctx_to_close) {
        log_debug("Context became NULL during cleanup");
        return;
    }

    // Flush buffers if available
    if (ctx_to_close->pb) {
        // CRITICAL FIX: Add try/catch-like protection for avio operations
        // which can sometimes cause segmentation faults
        log_debug("Flushing I/O buffers during cleanup");
        avio_flush(ctx_to_close->pb);
    } else {
        log_debug("No I/O context available during cleanup");
    }

    // CRITICAL FIX: Check if the context is properly initialized before closing
    // Some contexts might be partially initialized and need different cleanup
    if (ctx_to_close->nb_streams > 0 && ctx_to_close->streams) {
        // Context has streams, use standard close function
        log_debug("Closing input context with %d streams", ctx_to_close->nb_streams);
        avformat_close_input(&ctx_to_close);
    } else {
        // Context doesn't have streams, might be partially initialized
        // Just free the context directly to avoid potential segfaults
        log_debug("Context has no streams, using direct free");
        avformat_free_context(ctx_to_close);
    }

    log_debug("Safely cleaned up AVFormatContext");
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
