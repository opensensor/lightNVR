#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <ctype.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <fcntl.h>
#include <signal.h>
#include <errno.h>
#include <time.h>

// Define CLOCK_REALTIME if not available
#ifndef CLOCK_REALTIME
#define CLOCK_REALTIME 0
#endif

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/time.h>

#include "core/logger.h"
#include "video/stream_transcoding.h"
#include "video/hls_writer.h"
#include "video/mp4_writer.h"

/**
 * Log FFmpeg error
 */
void log_ffmpeg_error(int err, const char *message) {
    char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
    av_strerror(err, error_buf, AV_ERROR_MAX_STRING_SIZE);
    log_error("%s: %s", message, error_buf);
}

/**
 * Helper thread function for pthread_join_with_timeout
 */
static void *join_helper(void *arg) {
    struct {
        pthread_t thread;
        void **retval;
        int *result;
    } *data = arg;

    *(data->result) = pthread_join(data->thread, data->retval);
    return NULL;
}

/**
 * Join a thread with timeout
 */
int pthread_join_with_timeout(pthread_t thread, void **retval, int timeout_sec) {
    // Simple approach: use a second thread to join
    pthread_t timeout_thread;
    int *result = malloc(sizeof(int));
    *result = -1;

    // Structure to pass data to helper thread
    struct {
        pthread_t thread;
        void **retval;
        int *result;
    } join_data = {thread, retval, result};

    // Create helper thread to join the target thread
    if (pthread_create(&timeout_thread, NULL, join_helper, &join_data) != 0) {
        free(result);
        return EAGAIN;
    }

    // Wait for timeout
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    ts.tv_sec += timeout_sec;

    // Not glibc - use sleep and check
    while (1) {
        // Check if thread has completed
        if (pthread_kill(timeout_thread, 0) != 0) {
            break;
        }

        struct timespec now;
        clock_gettime(CLOCK_REALTIME, &now);
        if (now.tv_sec >= ts.tv_sec &&
            (now.tv_nsec >= ts.tv_nsec || now.tv_sec > ts.tv_sec)) {
            pthread_cancel(timeout_thread);
            pthread_join(timeout_thread, NULL);
            free(result);
            return ETIMEDOUT;
        }

        usleep(100000); // Sleep 100ms and try again
    }

    // Get the join result
    int join_result = *result;
    free(result);
    return join_result;
}

/**
 * Open input stream with appropriate options based on protocol
 */
int open_input_stream(AVFormatContext **input_ctx, const char *url, int protocol) {
    int ret;
    AVDictionary *input_options = NULL;
    
    if (protocol == STREAM_PROTOCOL_UDP) {
        log_info("Using UDP protocol for stream URL: %s", url);
        // UDP-specific options
        av_dict_set(&input_options, "protocol_whitelist", "file,udp,rtp", 0);
        av_dict_set(&input_options, "buffer_size", "8192000", 0); // Larger buffer for UDP
        av_dict_set(&input_options, "reuse", "1", 0); // Allow port reuse
        av_dict_set(&input_options, "timeout", "5000000", 0); // 5 second timeout in microseconds
    } else {
        log_info("Using TCP protocol for stream URL: %s", url);
        // TCP-specific options
        av_dict_set(&input_options, "stimeout", "5000000", 0); // 5 second timeout in microseconds
        av_dict_set(&input_options, "rtsp_transport", "tcp", 0); // Force TCP for RTSP
    }
    
    // Open input with protocol-specific options
    ret = avformat_open_input(input_ctx, url, NULL, &input_options);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Could not open input stream");
        av_dict_free(&input_options);
        return ret;
    }
    
    // Free options
    av_dict_free(&input_options);

    // Get stream info
    ret = avformat_find_stream_info(*input_ctx, NULL);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Could not find stream info");
        avformat_close_input(input_ctx);
        return ret;
    }

    return 0;
}

/**
 * Find video stream index in the input context
 */
int find_video_stream_index(AVFormatContext *input_ctx) {
    if (!input_ctx) {
        return -1;
    }

    for (unsigned int i = 0; i < input_ctx->nb_streams; i++) {
        if (input_ctx->streams[i]->codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
            return i;
        }
    }

    return -1;
}

/**
 * Process a video packet for either HLS streaming or MP4 recording
 */
int process_video_packet(const AVPacket *pkt, const AVStream *input_stream, 
                         void *writer, int writer_type, const char *stream_name) {
    int ret = 0;
    
    if (!pkt || !input_stream || !writer || !stream_name) {
        log_error("Invalid parameters passed to process_video_packet");
        return -1;
    }
    
    // Validate packet data
    if (!pkt->data || pkt->size <= 0) {
        log_warn("Invalid packet (null data or zero size) for stream %s", stream_name);
        return -1;
    }
    
    // Check if this is a key frame
    bool is_key_frame = (pkt->flags & AV_PKT_FLAG_KEY) != 0;
    
    if (is_key_frame) {
        log_debug("Processing keyframe for stream %s: pts=%lld, dts=%lld, size=%d",
                 stream_name, (long long)pkt->pts, (long long)pkt->dts, pkt->size);
    }
    
    if (writer_type == 0) {  // HLS writer
        hls_writer_t *hls_writer = (hls_writer_t *)writer;
        ret = hls_writer_write_packet(hls_writer, pkt, input_stream);
        if (ret < 0) {
            log_error("Failed to write packet to HLS for stream %s: %d", stream_name, ret);
        }
    } else if (writer_type == 1) {  // MP4 writer
        mp4_writer_t *mp4_writer = (mp4_writer_t *)writer;
        ret = mp4_writer_write_packet(mp4_writer, pkt, input_stream);
        if (ret < 0) {
            log_error("Failed to write packet to MP4 for stream %s: %d", stream_name, ret);
        }
    } else {
        log_error("Unknown writer type: %d", writer_type);
        ret = -1;
    }
    
    return ret;
}

/**
 * Initialize FFmpeg libraries
 */
void init_transcoding_backend(void) {
    // Initialize FFmpeg
#if LIBAVFORMAT_VERSION_INT < AV_VERSION_INT(58, 9, 100)
    av_register_all();
#endif
    avformat_network_init();

    log_info("Transcoding backend initialized");
}

/**
 * Cleanup FFmpeg resources
 */
void cleanup_transcoding_backend(void) {
    // Cleanup FFmpeg
    avformat_network_deinit();

    log_info("Transcoding backend cleaned up");
}
