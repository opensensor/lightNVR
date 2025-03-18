#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/error.h>

#include "core/logger.h"
#include "video/ffmpeg_utils.h"
#include "video/stream_protocol.h"

/**
 * Initialize FFmpeg library
 */
void init_ffmpeg(void) {
#if LIBAVFORMAT_VERSION_INT < AV_VERSION_INT(58, 9, 100)
    // Register all codecs and formats (deprecated in newer FFmpeg versions)
    av_register_all();
#endif
    
    // Initialize network
    avformat_network_init();
    
    // Set logging level
    av_log_set_level(AV_LOG_ERROR);
    
    log_info("FFmpeg initialized");
}

/**
 * Cleanup FFmpeg library
 */
void cleanup_ffmpeg(void) {
    // Cleanup network
    avformat_network_deinit();
    
    log_info("FFmpeg cleaned up");
}

/**
 * Open an input stream with the specified URL and protocol
 */
int open_input_stream(AVFormatContext **ctx, const char *url, stream_protocol_t protocol) {
    int ret;
    AVDictionary *options = NULL;
    
    // Create a new format context
    *ctx = avformat_alloc_context();
    if (!*ctx) {
        log_error("Failed to allocate format context");
        return -1;
    }
    
    // Set options based on protocol
    if (protocol == STREAM_PROTOCOL_UDP) {
        // For UDP streams, set a short timeout and large buffer size
        av_dict_set(&options, "timeout", "2000000", 0); // 2 seconds in microseconds
        av_dict_set(&options, "buffer_size", "16777216", 0); // 16MB buffer - CRITICAL for UDP streams
        av_dict_set(&options, "reuse", "1", 0); // Allow port reuse
        av_dict_set(&options, "overrun_nonfatal", "1", 0); // Don't fail on buffer overrun
        
        // Set UDP-specific socket options
        av_dict_set(&options, "recv_buffer_size", "16777216", 0); // 16MB socket receive buffer
        
        // UDP-specific packet reordering settings
        av_dict_set(&options, "max_interleave_delta", "1000000", 0); // 1 second max interleave
    } else {
        // For TCP/RTSP streams, set a longer timeout
        av_dict_set(&options, "timeout", "5000000", 0); // 5 seconds in microseconds
        av_dict_set(&options, "stimeout", "5000000", 0); // Socket timeout in microseconds
    }
    
    // Set common options
    av_dict_set(&options, "rtsp_transport", "tcp", 0); // Use TCP for RTSP
    av_dict_set(&options, "max_delay", "500000", 0); // 500ms max delay
    
    // Open input
    ret = avformat_open_input(ctx, url, NULL, &options);
    if (ret < 0) {
        char errbuf[AV_ERROR_MAX_STRING_SIZE];
        av_strerror(ret, errbuf, sizeof(errbuf));
        log_error("Failed to open input stream: %s (%d)", errbuf, ret);
        avformat_free_context(*ctx);
        *ctx = NULL;
        av_dict_free(&options);
        return ret;
    }
    
    // Find stream info
    ret = avformat_find_stream_info(*ctx, NULL);
    if (ret < 0) {
        char errbuf[AV_ERROR_MAX_STRING_SIZE];
        av_strerror(ret, errbuf, sizeof(errbuf));
        log_error("Failed to find stream info: %s (%d)", errbuf, ret);
        avformat_close_input(ctx);
        av_dict_free(&options);
        return ret;
    }
    
    // Clean up options
    av_dict_free(&options);
    
    return 0;
}

/**
 * Find the index of the video stream in the format context
 */
int find_video_stream_index(AVFormatContext *ctx) {
    if (!ctx) {
        log_error("NULL format context");
        return -1;
    }
    
    // Find the first video stream
    for (int i = 0; i < ctx->nb_streams; i++) {
        if (ctx->streams[i]->codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
            return i;
        }
    }
    
    log_error("No video stream found");
    return -1;
}

/**
 * Log an FFmpeg error with the given message
 */
void log_ffmpeg_error(int err, const char *msg) {
    char errbuf[AV_ERROR_MAX_STRING_SIZE];
    av_strerror(err, errbuf, sizeof(errbuf));
    log_error("%s: %s (%d)", msg, errbuf, err);
}
