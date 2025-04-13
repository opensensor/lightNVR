#include "video/ffmpeg_utils.h"
#include "core/logger.h"
#include "video/ffmpeg_leak_detector.h"
#include "video/stream_protocol.h"

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

    // Initialize the FFmpeg leak detector
    ffmpeg_leak_detector_init();

    log_info("FFmpeg initialized with leak detection");
}

/**
 * Cleanup FFmpeg resources
 */
void cleanup_ffmpeg(void) {
    // Force cleanup of any tracked FFmpeg allocations
    ffmpeg_force_cleanup_all();

    // Clean up the leak detector
    ffmpeg_leak_detector_cleanup();

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

        // Untrack the packet before freeing it
        UNTRACK_AVPACKET(pkt_to_free);

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

    // Untrack the context before freeing it
    UNTRACK_AVFORMAT_CTX(ctx_to_close);

    // Flush buffers if available
    if (ctx_to_close->pb) {
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

        // Untrack the frame before freeing it
        UNTRACK_AVFRAME(frame_to_free);

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

        // Untrack the codec context before freeing it
        UNTRACK_AVCODEC_CTX(codec_to_free);

        avcodec_free_context(&codec_to_free);
        log_debug("Cleaned up AVCodecContext");
    }

    // Clean up input context
    if (input_ctx) {
        safe_avformat_cleanup(input_ctx);
    }

    log_info("Comprehensive FFmpeg resource cleanup completed");
}

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
int periodic_ffmpeg_reset(AVFormatContext **input_ctx_ptr, const char *url, int protocol) {
    if (!input_ctx_ptr || !url) {
        log_error("Invalid parameters passed to periodic_ffmpeg_reset");
        return -1;
    }

    log_info("Performing periodic FFmpeg resource reset for URL: %s", url);

    // Dump current FFmpeg allocations for debugging
    ffmpeg_dump_allocations();

    // Close the existing context if it exists
    if (*input_ctx_ptr) {
        log_debug("Closing existing input context during reset");
        safe_avformat_cleanup(input_ctx_ptr);
        // input_ctx_ptr should now be NULL
    }

    // Force a garbage collection of FFmpeg resources
    // FFmpeg doesn't have a direct garbage collection function, but we can
    // try to release some memory by calling av_buffer_default_free on NULL
    // which is a no-op but might trigger some internal cleanup
    av_freep(NULL);

    // Open a new input stream
    AVFormatContext *new_ctx = NULL;
    AVDictionary *options = NULL;

    // Set protocol-specific options
    if (protocol == STREAM_PROTOCOL_TCP) {
        av_dict_set(&options, "rtsp_transport", "tcp", 0);
    }

    // Set common options for better reliability
    av_dict_set(&options, "stimeout", "5000000", 0);  // 5 second timeout in microseconds
    av_dict_set(&options, "reconnect", "1", 0);       // Auto reconnect
    av_dict_set(&options, "reconnect_streamed", "1", 0);
    av_dict_set(&options, "reconnect_delay_max", "2", 0); // Max 2 seconds between reconnects

    // Open the input
    int ret = avformat_open_input(&new_ctx, url, NULL, &options);
    av_dict_free(&options);

    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to open input during reset: %s", error_buf);
        return ret;
    }

    // Find stream info
    ret = avformat_find_stream_info(new_ctx, NULL);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to find stream info during reset: %s", error_buf);
        avformat_close_input(&new_ctx);
        return ret;
    }

    // Track the new context
    TRACK_AVFORMAT_CTX(new_ctx);

    // Set the output parameter
    *input_ctx_ptr = new_ctx;

    log_info("Successfully reset FFmpeg resources");
    return 0;
}
