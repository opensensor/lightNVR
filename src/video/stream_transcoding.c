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
 * Thread data structure for join helper
 */
typedef struct {
    pthread_t thread;
    void **retval;
    int result;
    pthread_mutex_t mutex;
    pthread_cond_t cond;
    int done;
} join_helper_data_t;

/**
 * Helper thread function for pthread_join_with_timeout
 */
static void *join_helper(void *arg) {
    join_helper_data_t *data = (join_helper_data_t *)arg;
    
    // Join the target thread
    data->result = pthread_join(data->thread, data->retval);
    
    // Signal completion
    pthread_mutex_lock(&data->mutex);
    data->done = 1;
    pthread_cond_signal(&data->cond);
    pthread_mutex_unlock(&data->mutex);
    
    return NULL;
}

/**
 * Join a thread with timeout
 */
int pthread_join_with_timeout(pthread_t thread, void **retval, int timeout_sec) {
    int ret = 0;
    pthread_t timeout_thread;
    
    // Initialize helper data on the stack to avoid memory leaks
    join_helper_data_t data = {
        .thread = thread,
        .retval = retval,
        .result = -1,
        .done = 0
    };
    
    // Initialize mutex and condition variable
    pthread_mutex_init(&data.mutex, NULL);
    pthread_cond_init(&data.cond, NULL);
    
    // Create helper thread to join the target thread
    if (pthread_create(&timeout_thread, NULL, join_helper, &data) != 0) {
        pthread_mutex_destroy(&data.mutex);
        pthread_cond_destroy(&data.cond);
        return EAGAIN;
    }
    
    // Wait for the helper thread to complete or timeout
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    ts.tv_sec += timeout_sec;
    
    pthread_mutex_lock(&data.mutex);
    while (!data.done) {
        ret = pthread_cond_timedwait(&data.cond, &data.mutex, &ts);
        if (ret == ETIMEDOUT) {
            // Timeout occurred
            pthread_mutex_unlock(&data.mutex);
            
            // Cancel the helper thread
            pthread_cancel(timeout_thread);
            pthread_join(timeout_thread, NULL);
            
            // Clean up resources
            pthread_mutex_destroy(&data.mutex);
            pthread_cond_destroy(&data.cond);
            
            return ETIMEDOUT;
        }
    }
    pthread_mutex_unlock(&data.mutex);
    
    // Join the helper thread to clean up
    pthread_join(timeout_thread, NULL);
    
    // Get the join result
    ret = data.result;
    
    // Clean up resources
    pthread_mutex_destroy(&data.mutex);
    pthread_cond_destroy(&data.cond);
    
    return ret;
}

/**
 * Open input stream with appropriate options based on protocol
 */
int open_input_stream(AVFormatContext **input_ctx, const char *url, int protocol) {
    int ret;
    AVDictionary *input_options = NULL;
    
    if (protocol == STREAM_PROTOCOL_UDP) {
        log_info("Using UDP protocol for stream URL: %s", url);
        // UDP-specific options with improved buffering for smoother playback
        av_dict_set(&input_options, "protocol_whitelist", "file,udp,rtp", 0);
        av_dict_set(&input_options, "buffer_size", "16384000", 0); // Increased buffer for UDP (16MB)
        av_dict_set(&input_options, "reuse", "1", 0); // Allow port reuse
        av_dict_set(&input_options, "timeout", "5000000", 0); // 5 second timeout in microseconds
        av_dict_set(&input_options, "max_delay", "500000", 0); // 500ms max delay for better real-time performance
        av_dict_set(&input_options, "fflags", "nobuffer", 0); // Reduce buffering for lower latency
    } else {
        log_info("Using TCP protocol for stream URL: %s", url);
        // TCP-specific options with improved reliability
        av_dict_set(&input_options, "stimeout", "5000000", 0); // 5 second timeout in microseconds
        av_dict_set(&input_options, "rtsp_transport", "tcp", 0); // Force TCP for RTSP
        av_dict_set(&input_options, "analyzeduration", "2000000", 0); // 2 seconds analyze duration
        av_dict_set(&input_options, "probesize", "1000000", 0); // 1MB probe size
        av_dict_set(&input_options, "reconnect", "1", 0); // Enable reconnection
        av_dict_set(&input_options, "reconnect_streamed", "1", 0); // Reconnect if streaming
        av_dict_set(&input_options, "reconnect_delay_max", "5", 0); // Max 5 seconds between reconnection attempts
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
 * Optimized to reduce contention and blocking with improved frame handling
 * Removed adaptive degrading to improve quality
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
    
    // Create a clean copy of the packet to avoid reference issues
    AVPacket out_pkt;
    av_init_packet(&out_pkt);
    out_pkt.data = NULL;
    out_pkt.size = 0;
    
    if (av_packet_ref(&out_pkt, pkt) < 0) {
        log_error("Failed to reference packet in process_video_packet for stream %s", stream_name);
        return -1;
    }
    
    // Check if this is a key frame - only log at debug level to reduce overhead
    bool is_key_frame = (out_pkt.flags & AV_PKT_FLAG_KEY) != 0;
    
    // Use direct function calls instead of conditional branching for better performance
    if (writer_type == 0) {  // HLS writer
        hls_writer_t *hls_writer = (hls_writer_t *)writer;
        
        // Removed adaptive frame dropping to improve quality
        // Always process all frames for better quality
        
        ret = hls_writer_write_packet(hls_writer, &out_pkt, input_stream);
        if (ret < 0) {
            // Only log errors for keyframes or every 200th packet to reduce log spam
            static int error_count = 0;
            if (is_key_frame || (++error_count % 200 == 0)) {
                log_error("Failed to write packet to HLS for stream %s: %d (count: %d)", 
                         stream_name, ret, error_count);
            }
        }
    } else if (writer_type == 1) {  // MP4 writer
        mp4_writer_t *mp4_writer = (mp4_writer_t *)writer;
        
        // Removed adaptive frame dropping to improve quality
        // Always process all frames for better quality
        
        // Ensure timestamps are properly set before writing
        // This helps prevent glitches in the output file
        if (out_pkt.pts == AV_NOPTS_VALUE && out_pkt.dts != AV_NOPTS_VALUE) {
            out_pkt.pts = out_pkt.dts;
        } else if (out_pkt.dts == AV_NOPTS_VALUE && out_pkt.pts != AV_NOPTS_VALUE) {
            out_pkt.dts = out_pkt.pts;
        }
        
        // Write the packet
        ret = mp4_writer_write_packet(mp4_writer, &out_pkt, input_stream);
        if (ret < 0) {
            // Only log errors for keyframes or every 200th packet to reduce log spam
            static int error_count = 0;
            if (is_key_frame || (++error_count % 200 == 0)) {
                log_error("Failed to write packet to MP4 for stream %s: %d (count: %d)", 
                         stream_name, ret, error_count);
            }
        }
    } else {
        log_error("Unknown writer type: %d", writer_type);
        ret = -1;
    }
    
    // Clean up our packet reference
    av_packet_unref(&out_pkt);
    
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
