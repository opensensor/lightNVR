#include "video/ffmpeg_utils.h"
#include "core/logger.h"
#include "video/stream_protocol.h"
#include "video/timeout_utils.h"  // For safe_packet_unref

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
 * Safe cleanup of FFmpeg AVFormatContext
 * This function provides a more thorough cleanup than just avformat_close_input
 * to help prevent memory leaks
 *
 * @param ctx_ptr Pointer to the AVFormatContext pointer to clean up
 */
void safe_avformat_cleanup(AVFormatContext **ctx_ptr) {
    if (!ctx_ptr || !*ctx_ptr) {
        return; // Nothing to do
    }

    AVFormatContext *ctx = *ctx_ptr;

    // 1. Flush all buffers before closing
    if (ctx->pb) {
        avio_flush(ctx->pb);
    }

    // 2. In newer FFmpeg versions, we don't need to manually close codecs
    // as avformat_close_input will handle this
    // The codec member has been removed from AVStream in newer FFmpeg versions

    // 3. Close the input context
    avformat_close_input(ctx_ptr);

    // 4. If it's still not freed, use free_context
    if (*ctx_ptr) {
        log_warn("Context still exists after avformat_close_input, using avformat_free_context");
        avformat_free_context(*ctx_ptr);
        *ctx_ptr = NULL;
    }

    log_debug("Successfully cleaned up AVFormatContext");
}

/**
 * Safe cleanup of FFmpeg packet
 * This function provides a thorough cleanup of an AVPacket to prevent memory leaks
 *
 * @param pkt_ptr Pointer to the AVPacket pointer to clean up
 */
void safe_packet_cleanup(AVPacket **pkt_ptr) {
    if (!pkt_ptr || !*pkt_ptr) {
        return; // Nothing to do
    }

    // 1. Unref the packet to free any referenced data using our safer function
    // CRITICAL FIX: Use safe_packet_unref instead of av_packet_unref
    safe_packet_unref(*pkt_ptr, "safe_packet_cleanup");

    // 2. Free the packet itself
    av_packet_free(pkt_ptr);

    // 3. Double check that it's really freed
    if (*pkt_ptr) {
        log_warn("Packet still exists after av_packet_free, forcing cleanup");
        free(*pkt_ptr);
        *pkt_ptr = NULL;
    }

    log_debug("Successfully cleaned up AVPacket");
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
    if (!input_ctx_ptr || !*input_ctx_ptr || !url) {
        return AVERROR(EINVAL);
    }

    log_info("Performing periodic FFmpeg resource reset for %s", url);

    // 1. Save important information before closing
    int video_stream_idx = -1;
    int audio_stream_idx = -1;

    // Find video and audio stream indices
    for (unsigned int i = 0; i < (*input_ctx_ptr)->nb_streams; i++) {
        if ((*input_ctx_ptr)->streams[i]->codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
            video_stream_idx = i;
        } else if ((*input_ctx_ptr)->streams[i]->codecpar->codec_type == AVMEDIA_TYPE_AUDIO) {
            audio_stream_idx = i;
        }
    }

    // 2. Close the input context
    safe_avformat_cleanup(input_ctx_ptr);

    // 3. Reopen the input context
    int ret = open_input_stream(input_ctx_ptr, url, protocol);
    if (ret < 0) {
        log_error("Failed to reopen input stream during periodic reset: %s", url);
        return ret;
    }

    // 4. Log success
    log_info("Successfully reset FFmpeg resources for %s", url);
    log_info("Previous video stream index: %d, new video stream index: %d",
             video_stream_idx, find_video_stream_index(*input_ctx_ptr));

    return 0;
}

/**
 * Safe wrapper for av_read_frame
 * This function provides additional safety checks before calling av_read_frame
 * to help prevent segmentation faults
 *
 * @param input_ctx Pointer to the AVFormatContext to read from
 * @param pkt Pointer to the AVPacket to read into
 * @return 0 on success, negative value on error
 */
int safe_av_read_frame(AVFormatContext *input_ctx, AVPacket *pkt) {
    // Validate input parameters
    if (!input_ctx) {
        log_error("safe_av_read_frame: NULL input context");
        return AVERROR(EINVAL);
    }

    if (!pkt) {
        log_error("safe_av_read_frame: NULL packet");
        return AVERROR(EINVAL);
    }

    // Validate input context
    if (!input_ctx->pb) {
        log_error("safe_av_read_frame: Invalid input context (NULL pb)");
        return AVERROR(EINVAL);
    }

    // Check for obviously invalid pointers
    if ((uintptr_t)input_ctx < 1000 || (uintptr_t)input_ctx->pb < 1000) {
        log_error("safe_av_read_frame: Invalid input context pointer");
        return AVERROR(EINVAL);
    }

    // Ensure packet is in a clean state
    safe_packet_unref(pkt, "safe_av_read_frame");

    // Call av_read_frame with error handling
    int ret = av_read_frame(input_ctx, pkt);

    // Check for errors
    if (ret < 0 && ret != AVERROR_EOF && ret != AVERROR(EAGAIN)) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_debug("safe_av_read_frame: Error reading frame: %s (code: %d)", error_buf, ret);
    }

    return ret;
}
