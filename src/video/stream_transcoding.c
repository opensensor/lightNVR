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
 * Check if a URL is a multicast address
 * Multicast IPv4 addresses are in the range 224.0.0.0 to 239.255.255.255
 */
static bool is_multicast_url(const char *url) {
    // Extract IP address from URL
    // Simple parsing for common formats like udp://239.255.0.1:1234
    const char *ip_start = NULL;
    
    // Skip protocol prefix
    if (strncmp(url, "udp://", 6) == 0) {
        ip_start = url + 6;
    } else if (strncmp(url, "rtp://", 6) == 0) {
        ip_start = url + 6;
    } else {
        // Not a UDP or RTP URL
        return false;
    }
    
    // Parse IP address
    unsigned int a, b, c, d;
    if (sscanf(ip_start, "%u.%u.%u.%u", &a, &b, &c, &d) == 4) {
        // Check if it's in multicast range (224.0.0.0 - 239.255.255.255)
        if (a >= 224 && a <= 239) {
            log_info("Detected multicast address: %u.%u.%u.%u", a, b, c, d);
            return true;
        }
    }
    
    return false;
}

/**
 * Open input stream with appropriate options based on protocol
 */
int open_input_stream(AVFormatContext **input_ctx, const char *url, int protocol) {
    int ret;
    AVDictionary *input_options = NULL;
    bool is_multicast = false;
    
    if (protocol == STREAM_PROTOCOL_UDP) {
        // Check if this is a multicast stream
        is_multicast = is_multicast_url(url);
        
        log_info("Using UDP protocol for stream URL: %s (multicast: %s)", 
                url, is_multicast ? "yes" : "no");
        
        // UDP-specific options with improved buffering for smoother playback
        // Expanded protocol whitelist to support more UDP variants
        av_dict_set(&input_options, "protocol_whitelist", "file,udp,rtp,rtsp,tcp,https,tls,http", 0);
        
        // Increased buffer size to 16MB as recommended for UDP jitter handling
        av_dict_set(&input_options, "buffer_size", "16777216", 0); // 16MB buffer
        
        // Allow port reuse
        av_dict_set(&input_options, "reuse", "1", 0);
        
        // Extended timeout for UDP streams which may have more jitter
        av_dict_set(&input_options, "timeout", "10000000", 0); // 10 second timeout in microseconds
        
        // Increased max delay for UDP streams
        av_dict_set(&input_options, "max_delay", "2000000", 0); // 2000ms max delay
        
        // More tolerant timestamp handling for UDP streams
        av_dict_set(&input_options, "fflags", "genpts+discardcorrupt+nobuffer", 0);
        
        // Set UDP-specific socket options
        av_dict_set(&input_options, "recv_buffer_size", "16777216", 0); // 16MB socket receive buffer
        
        // UDP-specific packet reordering settings
        av_dict_set(&input_options, "max_interleave_delta", "1000000", 0); // 1 second max interleave
        
        // Multicast-specific settings
        if (is_multicast) {
            // Set appropriate TTL for multicast
            av_dict_set(&input_options, "ttl", "32", 0);
            
            // Join multicast group
            av_dict_set(&input_options, "multiple_requests", "1", 0);
            
            // Auto-detect the best network interface
            av_dict_set(&input_options, "localaddr", "0.0.0.0", 0);
        }
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

// Structure to track timestamp information per stream
typedef struct {
    char stream_name[MAX_STREAM_NAME];
    int64_t last_pts;
    int64_t last_dts;
    int64_t pts_discontinuity_count;
    int64_t expected_next_pts;
    bool is_udp_stream;
    bool initialized;
} timestamp_tracker_t;

// Array to track timestamps for multiple streams
#define MAX_TIMESTAMP_TRACKERS 16
static timestamp_tracker_t timestamp_trackers[MAX_TIMESTAMP_TRACKERS];
static pthread_mutex_t timestamp_mutex = PTHREAD_MUTEX_INITIALIZER;

/**
 * Get or create a timestamp tracker for a stream
 */
static timestamp_tracker_t *get_timestamp_tracker(const char *stream_name) {
    pthread_mutex_lock(&timestamp_mutex);
    
    // Look for existing tracker
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].initialized && 
            strcmp(timestamp_trackers[i].stream_name, stream_name) == 0) {
            pthread_mutex_unlock(&timestamp_mutex);
            return &timestamp_trackers[i];
        }
    }
    
    // Create new tracker
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (!timestamp_trackers[i].initialized) {
            strncpy(timestamp_trackers[i].stream_name, stream_name, MAX_STREAM_NAME - 1);
            timestamp_trackers[i].stream_name[MAX_STREAM_NAME - 1] = '\0';
            timestamp_trackers[i].last_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i].last_dts = AV_NOPTS_VALUE;
            timestamp_trackers[i].pts_discontinuity_count = 0;
            timestamp_trackers[i].expected_next_pts = AV_NOPTS_VALUE;
            
            // Check if this is a UDP stream by looking at the stream name
            timestamp_trackers[i].is_udp_stream = (strstr(stream_name, "udp://") != NULL || 
                                                  strstr(stream_name, "rtp://") != NULL);
            
            timestamp_trackers[i].initialized = true;
            pthread_mutex_unlock(&timestamp_mutex);
            return &timestamp_trackers[i];
        }
    }
    
    // No slots available
    pthread_mutex_unlock(&timestamp_mutex);
    return NULL;
}

/**
 * Process a video packet for either HLS streaming or MP4 recording
 * Optimized to reduce contention and blocking with improved frame handling
 * Includes enhanced timestamp handling for UDP streams
 */
int process_video_packet(const AVPacket *pkt, const AVStream *input_stream, 
                         void *writer, int writer_type, const char *stream_name) {
    int ret = 0;
    AVPacket *out_pkt = NULL;
    
    // Get timestamp tracker for this stream
    timestamp_tracker_t *tracker = get_timestamp_tracker(stream_name);
    if (!tracker) {
        log_error("Failed to get timestamp tracker for stream %s", stream_name);
        // Continue without timestamp tracking
    }
    
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
    out_pkt = av_packet_alloc();
    if (!out_pkt) {
        log_error("Failed to allocate packet in process_video_packet for stream %s", stream_name);
        return -1;
    }
    
    if (av_packet_ref(out_pkt, pkt) < 0) {
        log_error("Failed to reference packet in process_video_packet for stream %s", stream_name);
        av_packet_free(&out_pkt);
        return -1;
    }
    
    // Check if this is a key frame - only log at debug level to reduce overhead
    bool is_key_frame = (out_pkt->flags & AV_PKT_FLAG_KEY) != 0;
    
    // Enhanced timestamp handling for UDP streams
    if (tracker && tracker->is_udp_stream) {
        // Handle missing timestamps
        if (out_pkt->pts == AV_NOPTS_VALUE && out_pkt->dts != AV_NOPTS_VALUE) {
            out_pkt->pts = out_pkt->dts;
        } else if (out_pkt->dts == AV_NOPTS_VALUE && out_pkt->pts != AV_NOPTS_VALUE) {
            out_pkt->dts = out_pkt->pts;
        } else if (out_pkt->pts == AV_NOPTS_VALUE && out_pkt->dts == AV_NOPTS_VALUE) {
            // Both timestamps missing, try to generate based on previous packet
            if (tracker->last_pts != AV_NOPTS_VALUE) {
                // Calculate frame duration based on stream timebase and framerate
                int64_t frame_duration = 0;
                if (input_stream->avg_frame_rate.num > 0) {
                    AVRational tb = input_stream->time_base;
                    AVRational fr = input_stream->avg_frame_rate;
                    frame_duration = av_rescale_q(1, av_inv_q(fr), tb);
                } else {
                    // Default to 1/30 second if framerate not available
                    frame_duration = input_stream->time_base.den / (30 * input_stream->time_base.num);
                }
                
                out_pkt->pts = tracker->last_pts + frame_duration;
                out_pkt->dts = out_pkt->pts;
                log_debug("Generated timestamps for UDP stream %s: pts=%lld, dts=%lld", 
                         stream_name, (long long)out_pkt->pts, (long long)out_pkt->dts);
            }
        }
        
        // Detect and handle timestamp discontinuities
        if (tracker->last_pts != AV_NOPTS_VALUE && out_pkt->pts != AV_NOPTS_VALUE) {
            // Calculate expected next PTS
            int64_t frame_duration = 0;
            if (input_stream->avg_frame_rate.num > 0) {
                AVRational tb = input_stream->time_base;
                AVRational fr = input_stream->avg_frame_rate;
                frame_duration = av_rescale_q(1, av_inv_q(fr), tb);
            } else {
                // Default to 1/30 second if framerate not available
                frame_duration = input_stream->time_base.den / (30 * input_stream->time_base.num);
            }
            
            if (tracker->expected_next_pts == AV_NOPTS_VALUE) {
                tracker->expected_next_pts = tracker->last_pts + frame_duration;
            }
            
            // Check for large discontinuities (more than 10x frame duration)
            int64_t pts_diff = llabs(out_pkt->pts - tracker->expected_next_pts);
            if (pts_diff > 10 * frame_duration) {
                tracker->pts_discontinuity_count++;
                
                // Only log occasionally to avoid flooding logs
                if (tracker->pts_discontinuity_count % 10 == 1) {
                    log_warn("Timestamp discontinuity detected in UDP stream %s: expected=%lld, actual=%lld, diff=%lld ms", 
                            stream_name, 
                            (long long)tracker->expected_next_pts, 
                            (long long)out_pkt->pts,
                            (long long)(pts_diff * 1000 * input_stream->time_base.num / input_stream->time_base.den));
                }
                
                // For severe discontinuities, try to correct by using expected PTS
                if (pts_diff > 100 * frame_duration) {
                    int64_t original_pts = out_pkt->pts;
                    out_pkt->pts = tracker->expected_next_pts;
                    out_pkt->dts = out_pkt->pts;
                    
                    log_debug("Corrected severe timestamp discontinuity: original=%lld, corrected=%lld", 
                             (long long)original_pts, (long long)out_pkt->pts);
                }
            }
            
            // Update expected next PTS
            tracker->expected_next_pts = out_pkt->pts + frame_duration;
        }
        
        // Store current timestamps for next packet
        tracker->last_pts = out_pkt->pts;
        tracker->last_dts = out_pkt->dts;
    }
    
    // Use direct function calls instead of conditional branching for better performance
    if (writer_type == 0) {  // HLS writer
        hls_writer_t *hls_writer = (hls_writer_t *)writer;
        
        // Removed adaptive frame dropping to improve quality
        // Always process all frames for better quality
        
        ret = hls_writer_write_packet(hls_writer, out_pkt, input_stream);
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
        if (out_pkt->pts == AV_NOPTS_VALUE && out_pkt->dts != AV_NOPTS_VALUE) {
            out_pkt->pts = out_pkt->dts;
        } else if (out_pkt->dts == AV_NOPTS_VALUE && out_pkt->pts != AV_NOPTS_VALUE) {
            out_pkt->dts = out_pkt->pts;
        }
        
        // Write the packet
        ret = mp4_writer_write_packet(mp4_writer, out_pkt, input_stream);
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
        // Make sure we free the packet before returning on error
        av_packet_free(&out_pkt);
        return ret;
    }
    
    // Clean up our packet reference
    av_packet_free(&out_pkt);
    
    return ret;
}

/**
 * Initialize timestamp trackers
 */
static void init_timestamp_trackers(void) {
    pthread_mutex_lock(&timestamp_mutex);
    
    // Initialize all trackers to unused state
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        timestamp_trackers[i].initialized = false;
        timestamp_trackers[i].last_pts = AV_NOPTS_VALUE;
        timestamp_trackers[i].last_dts = AV_NOPTS_VALUE;
        timestamp_trackers[i].pts_discontinuity_count = 0;
        timestamp_trackers[i].expected_next_pts = AV_NOPTS_VALUE;
        timestamp_trackers[i].is_udp_stream = false;
        timestamp_trackers[i].stream_name[0] = '\0';
    }
    
    pthread_mutex_unlock(&timestamp_mutex);
    log_info("Timestamp trackers initialized");
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
    
    // Initialize timestamp trackers
    init_timestamp_trackers();

    log_info("Transcoding backend initialized");
}

/**
 * Cleanup timestamp trackers
 */
static void cleanup_timestamp_trackers(void) {
    pthread_mutex_lock(&timestamp_mutex);
    
    // Reset all trackers to unused state
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        timestamp_trackers[i].initialized = false;
        timestamp_trackers[i].stream_name[0] = '\0';
    }
    
    pthread_mutex_unlock(&timestamp_mutex);
    log_info("Timestamp trackers cleaned up");
}

/**
 * Cleanup FFmpeg resources
 */
void cleanup_transcoding_backend(void) {
    // Cleanup timestamp trackers
    cleanup_timestamp_trackers();
    
    // Cleanup FFmpeg
    avformat_network_deinit();

    log_info("Transcoding backend cleaned up");
}
