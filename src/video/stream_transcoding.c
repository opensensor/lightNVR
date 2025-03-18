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
    // Validate input
    if (!url || strlen(url) < 7) {  // Minimum length for "udp://1"
        log_warn("Invalid URL for multicast detection: %s", url ? url : "NULL");
        return false;
    }
    
    // Extract IP address from URL with more robust parsing
    const char *ip_start = NULL;
    
    // Skip protocol prefix with safer checks
    if (strncmp(url, "udp://", 6) == 0) {
        ip_start = url + 6;
    } else if (strncmp(url, "rtp://", 6) == 0) {
        ip_start = url + 6;
    } else {
        // Not a UDP or RTP URL
        log_debug("Not a UDP/RTP URL for multicast detection: %s", url);
        return false;
    }
    
    // Skip any authentication info (user:pass@)
    const char *at_sign = strchr(ip_start, '@');
    if (at_sign) {
        ip_start = at_sign + 1;
    }
    
    // Make a copy of the IP part to avoid modifying the original
    char ip_buffer[256];
    strncpy(ip_buffer, ip_start, sizeof(ip_buffer) - 1);
    ip_buffer[sizeof(ip_buffer) - 1] = '\0';
    
    // Remove port and path information
    char *colon = strchr(ip_buffer, ':');
    if (colon) {
        *colon = '\0';
    }
    
    char *slash = strchr(ip_buffer, '/');
    if (slash) {
        *slash = '\0';
    }
    
    // Parse IP address with additional validation
    unsigned int a = 0, b = 0, c = 0, d = 0;
    if (sscanf(ip_buffer, "%u.%u.%u.%u", &a, &b, &c, &d) == 4) {
        // Validate IP address components
        if (a > 255 || b > 255 || c > 255 || d > 255) {
            log_warn("Invalid IP address components in URL: %s", url);
            return false;
        }
        
        // Check if it's in multicast range (224.0.0.0 - 239.255.255.255)
        if (a >= 224 && a <= 239) {
            log_info("Detected multicast address: %u.%u.%u.%u in URL: %s", a, b, c, d, url);
            return true;
        }
    } else {
        log_debug("Could not parse IP address from URL: %s", url);
    }
    
    return false;
}

/**
 * Open input stream with appropriate options based on protocol
 * Enhanced with more robust error handling and synchronization for UDP streams
 */
int open_input_stream(AVFormatContext **input_ctx, const char *url, int protocol) {
    int ret;
    AVDictionary *input_options = NULL;
    bool is_multicast = false;
    
    // Validate input parameters
    if (!input_ctx || !url || strlen(url) < 5) {
        log_error("Invalid parameters for open_input_stream: ctx=%p, url=%s", 
                 (void*)input_ctx, url ? url : "NULL");
        return AVERROR(EINVAL);
    }
    
    // Make sure we're starting with a NULL context
    if (*input_ctx) {
        log_warn("Input context not NULL, closing existing context before opening new one");
        avformat_close_input(input_ctx);
    }
    
    // Log the stream opening attempt
    log_info("Opening input stream: %s (protocol: %s)", 
            url, protocol == STREAM_PROTOCOL_UDP ? "UDP" : "TCP");
    
    if (protocol == STREAM_PROTOCOL_UDP) {
        // Check if this is a multicast stream with robust error handling
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
        
        // Multicast-specific settings with enhanced error handling
        if (is_multicast) {
            log_info("Configuring multicast-specific settings for %s", url);
            
            // Set appropriate TTL for multicast
            av_dict_set(&input_options, "ttl", "32", 0);
            
            // Join multicast group
            av_dict_set(&input_options, "multiple_requests", "1", 0);
            
            // Auto-detect the best network interface
            av_dict_set(&input_options, "localaddr", "0.0.0.0", 0);
            
            // Additional multicast settings for better reliability
            av_dict_set(&input_options, "pkt_size", "1316", 0); // Standard UDP packet size for MPEG-TS
            av_dict_set(&input_options, "rw_timeout", "10000000", 0); // 10 second read/write timeout
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
    
    // Verify that the context was created
    if (!*input_ctx) {
        log_error("Input context is NULL after successful open");
        return AVERROR(EINVAL);
    }

    // Get stream info with enhanced error handling
    log_debug("Getting stream info for %s", url);
    ret = avformat_find_stream_info(*input_ctx, NULL);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Could not find stream info");
        avformat_close_input(input_ctx);
        return ret;
    }
    
    // Log successful stream opening
    if (*input_ctx && (*input_ctx)->nb_streams > 0) {
        log_info("Successfully opened input stream: %s with %d streams", 
                url, (*input_ctx)->nb_streams);
    } else {
        log_warn("Opened input stream but no streams found: %s", url);
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
 * Set the UDP flag for a stream's timestamp tracker
 * Creates the tracker if it doesn't exist
 */
void set_timestamp_tracker_udp_flag(const char *stream_name, bool is_udp) {
    if (!stream_name) {
        log_error("set_timestamp_tracker_udp_flag: NULL stream name");
        return;
    }
    
    // Make a local copy of the stream name to avoid issues with concurrent access
    char local_stream_name[MAX_STREAM_NAME];
    strncpy(local_stream_name, stream_name, MAX_STREAM_NAME - 1);
    local_stream_name[MAX_STREAM_NAME - 1] = '\0';
    
    pthread_mutex_lock(&timestamp_mutex);
    
    // Look for existing tracker
    int found = 0;
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].initialized && 
            strcmp(timestamp_trackers[i].stream_name, local_stream_name) == 0) {
            // Set the UDP flag
            timestamp_trackers[i].is_udp_stream = is_udp;
            log_info("Set UDP flag to %s for stream %s timestamp tracker", 
                    is_udp ? "true" : "false", local_stream_name);
            found = 1;
            break;
        }
    }
    
    // If not found, create a new tracker
    if (!found) {
        for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
            if (!timestamp_trackers[i].initialized) {
                // Initialize the new tracker
                strncpy(timestamp_trackers[i].stream_name, local_stream_name, MAX_STREAM_NAME - 1);
                timestamp_trackers[i].stream_name[MAX_STREAM_NAME - 1] = '\0';
                timestamp_trackers[i].last_pts = AV_NOPTS_VALUE;
                timestamp_trackers[i].last_dts = AV_NOPTS_VALUE;
                timestamp_trackers[i].pts_discontinuity_count = 0;
                timestamp_trackers[i].expected_next_pts = AV_NOPTS_VALUE;
                timestamp_trackers[i].is_udp_stream = is_udp;
                timestamp_trackers[i].initialized = true;
                
                log_info("Created new timestamp tracker for stream %s at index %d with UDP flag %s", 
                        local_stream_name, i, is_udp ? "true" : "false");
                found = 1;
                break;
            }
        }
        
        if (!found) {
            log_error("No available slots for timestamp tracker for stream %s", local_stream_name);
        }
    }
    
    pthread_mutex_unlock(&timestamp_mutex);
}

/**
 * Get or create a timestamp tracker for a stream
 */
static timestamp_tracker_t *get_timestamp_tracker(const char *stream_name) {
    if (!stream_name) {
        log_error("get_timestamp_tracker: NULL stream name");
        return NULL;
    }
    
    // Make a local copy of the stream name to avoid issues with concurrent access
    char local_stream_name[MAX_STREAM_NAME];
    strncpy(local_stream_name, stream_name, MAX_STREAM_NAME - 1);
    local_stream_name[MAX_STREAM_NAME - 1] = '\0';
    
    pthread_mutex_lock(&timestamp_mutex);
    
    // Look for existing tracker
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].initialized && 
            strcmp(timestamp_trackers[i].stream_name, local_stream_name) == 0) {
            pthread_mutex_unlock(&timestamp_mutex);
            return &timestamp_trackers[i];
        }
    }
    
    // Create new tracker
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (!timestamp_trackers[i].initialized) {
            strncpy(timestamp_trackers[i].stream_name, local_stream_name, MAX_STREAM_NAME - 1);
            timestamp_trackers[i].stream_name[MAX_STREAM_NAME - 1] = '\0';
            timestamp_trackers[i].last_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i].last_dts = AV_NOPTS_VALUE;
            timestamp_trackers[i].pts_discontinuity_count = 0;
            timestamp_trackers[i].expected_next_pts = AV_NOPTS_VALUE;
            
            // We'll set this based on the actual protocol when processing packets
            timestamp_trackers[i].is_udp_stream = false;
            
            timestamp_trackers[i].initialized = true;
            
            log_info("Created new timestamp tracker for stream %s at index %d", 
                    local_stream_name, i);
            
            pthread_mutex_unlock(&timestamp_mutex);
            return &timestamp_trackers[i];
        }
    }
    
    // No slots available
    log_error("No available slots for timestamp tracker for stream %s", local_stream_name);
    pthread_mutex_unlock(&timestamp_mutex);
    return NULL;
}

/**
 * Process a video packet for either HLS streaming or MP4 recording
 * Optimized to reduce contention and blocking with improved frame handling
 * Includes enhanced timestamp handling for UDP streams
 * 
 * FIXED: Improved thread safety for multiple UDP streams
 */
int process_video_packet(const AVPacket *pkt, const AVStream *input_stream, 
                         void *writer, int writer_type, const char *stream_name) {
    int ret = 0;
    AVPacket *out_pkt = NULL;
    
    // CRITICAL FIX: Add extra validation for all parameters with early return
    if (!pkt) {
        log_error("process_video_packet: NULL packet");
        return -1;
    }
    
    if (!input_stream) {
        log_error("process_video_packet: NULL input stream");
        return -1;
    }
    
    if (!writer) {
        log_error("process_video_packet: NULL writer");
        return -1;
    }
    
    if (!stream_name) {
        log_error("process_video_packet: NULL stream name");
        return -1;
    }
    
    // Validate packet data
    if (!pkt->data || pkt->size <= 0) {
        log_warn("Invalid packet (null data or zero size) for stream %s", stream_name);
        return -1;
    }
    
    // Log entry point for debugging
    log_debug("Processing video packet for stream %s, size=%d", stream_name, pkt->size);
    
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
    
    // FIXED: Create local copies of timestamp data to avoid race conditions
    bool is_udp_stream = false;
    int64_t last_pts = AV_NOPTS_VALUE;
    int64_t expected_next_pts = AV_NOPTS_VALUE;
    int pts_discontinuity_count = 0;
    bool tracker_found = false;
    int tracker_idx = -1;
    
    // FIXED: Use a single mutex lock/unlock pair to get all timestamp data
    pthread_mutex_lock(&timestamp_mutex);
    
    // Find the tracker for this stream
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].initialized && 
            strcmp(timestamp_trackers[i].stream_name, stream_name) == 0) {
            // Copy all needed data to local variables
            is_udp_stream = timestamp_trackers[i].is_udp_stream;
            last_pts = timestamp_trackers[i].last_pts;
            expected_next_pts = timestamp_trackers[i].expected_next_pts;
            pts_discontinuity_count = timestamp_trackers[i].pts_discontinuity_count;
            tracker_found = true;
            tracker_idx = i;
            break;
        }
    }
    
    // If not found, create a new tracker
    if (!tracker_found) {
        for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
            if (!timestamp_trackers[i].initialized) {
                // Initialize the new tracker
                strncpy(timestamp_trackers[i].stream_name, stream_name, MAX_STREAM_NAME - 1);
                timestamp_trackers[i].stream_name[MAX_STREAM_NAME - 1] = '\0';
                timestamp_trackers[i].last_pts = AV_NOPTS_VALUE;
                timestamp_trackers[i].last_dts = AV_NOPTS_VALUE;
                timestamp_trackers[i].pts_discontinuity_count = 0;
                timestamp_trackers[i].expected_next_pts = AV_NOPTS_VALUE;
                timestamp_trackers[i].is_udp_stream = false; // Default to false
                timestamp_trackers[i].initialized = true;
                
                // Copy data to local variables
                is_udp_stream = false;
                last_pts = AV_NOPTS_VALUE;
                expected_next_pts = AV_NOPTS_VALUE;
                pts_discontinuity_count = 0;
                
                tracker_found = true;
                tracker_idx = i;
                
                log_info("Created new timestamp tracker for stream %s at index %d", 
                        stream_name, i);
                break;
            }
        }
    }
    
    // Release the mutex after getting all the data we need
    pthread_mutex_unlock(&timestamp_mutex);
    
    if (!tracker_found) {
        log_error("Failed to get or create timestamp tracker for stream %s", stream_name);
        // Continue without timestamp tracking
    } else {
        log_debug("Using timestamp tracker for stream %s with UDP flag: %s", 
                 stream_name, is_udp_stream ? "true" : "false");
                 
        // Process UDP streams with special timestamp handling
        if (is_udp_stream) {
            log_debug("Applying UDP timestamp handling for stream %s", stream_name);
            
            // Handle missing timestamps
            if (out_pkt->pts == AV_NOPTS_VALUE && out_pkt->dts != AV_NOPTS_VALUE) {
                out_pkt->pts = out_pkt->dts;
            } else if (out_pkt->dts == AV_NOPTS_VALUE && out_pkt->pts != AV_NOPTS_VALUE) {
                out_pkt->dts = out_pkt->pts;
            } else if (out_pkt->pts == AV_NOPTS_VALUE && out_pkt->dts == AV_NOPTS_VALUE) {
                // Both timestamps missing, try to generate based on previous packet
                if (last_pts != AV_NOPTS_VALUE) {
                    // Calculate frame duration based on stream timebase and framerate
                    int64_t frame_duration = 0;
                    
                    // Add safety checks for input_stream
                    if (input_stream && input_stream->avg_frame_rate.num > 0) {
                        AVRational tb = input_stream->time_base;
                        AVRational fr = input_stream->avg_frame_rate;
                        
                        // Avoid division by zero
                        if (fr.den > 0) {
                            frame_duration = av_rescale_q(1, av_inv_q(fr), tb);
                        } else {
                            // Default to a reasonable value if framerate is invalid
                            frame_duration = 3000; // Assume 30fps with timebase 1/90000
                        }
                    } else {
                        // Default to 1/30 second if framerate not available
                        // With additional safety checks
                        if (input_stream && input_stream->time_base.num > 0 && input_stream->time_base.den > 0) {
                            frame_duration = input_stream->time_base.den / (30 * input_stream->time_base.num);
                        } else {
                            // Fallback to a reasonable default
                            frame_duration = 3000; // Assume 30fps with timebase 1/90000
                        }
                    }
                    
                    // Sanity check on frame duration
                    if (frame_duration <= 0) {
                        frame_duration = 3000; // Reasonable default
                    }
                    
                    out_pkt->pts = last_pts + frame_duration;
                    out_pkt->dts = out_pkt->pts;
                    log_debug("Generated timestamps for UDP stream %s: pts=%lld, dts=%lld", 
                             stream_name, (long long)out_pkt->pts, (long long)out_pkt->dts);
                } else {
                    // If we don't have a last_pts, set a default starting value
                    out_pkt->pts = 1;
                    out_pkt->dts = 1;
                    log_debug("Set default initial timestamps for UDP stream %s with no previous reference", 
                             stream_name);
                }
            }
            
            // Detect and handle timestamp discontinuities with additional safety checks
            if (last_pts != AV_NOPTS_VALUE && out_pkt->pts != AV_NOPTS_VALUE) {
                // Calculate frame duration
                int64_t frame_duration = 0;
                
                // Add safety checks for input_stream
                if (input_stream && input_stream->avg_frame_rate.num > 0 && input_stream->avg_frame_rate.den > 0) {
                    // Ensure time_base is valid before using it
                    if (input_stream->time_base.num > 0 && input_stream->time_base.den > 0) {
                        AVRational tb = input_stream->time_base;
                        AVRational fr = input_stream->avg_frame_rate;
                        frame_duration = av_rescale_q(1, av_inv_q(fr), tb);
                    } else {
                        // Default if time_base is invalid
                        frame_duration = 3000; // Assume 30fps with timebase 1/90000
                    }
                } else {
                    // Default to 1/30 second if framerate not available
                    // With additional safety checks
                    if (input_stream && input_stream->time_base.num > 0 && input_stream->time_base.den > 0) {
                        frame_duration = input_stream->time_base.den / (30 * input_stream->time_base.num);
                    } else {
                        // Fallback to a reasonable default
                        frame_duration = 3000; // Assume 30fps with timebase 1/90000
                    }
                }
            
                // Sanity check on frame duration
                if (frame_duration <= 0) {
                    frame_duration = 3000; // Reasonable default
                }
                
                // Calculate expected next PTS locally
                int64_t local_expected_next_pts = expected_next_pts;
                if (local_expected_next_pts == AV_NOPTS_VALUE) {
                    local_expected_next_pts = last_pts + frame_duration;
                }
                
                // Check for large discontinuities (more than 10x frame duration)
                int64_t pts_diff = llabs(out_pkt->pts - local_expected_next_pts);
                if (pts_diff > 10 * frame_duration) {
                    // Update discontinuity count safely
                    int local_count = 0;
                    
                    // FIXED: Use a single atomic update for the discontinuity count
                    pthread_mutex_lock(&timestamp_mutex);
                    // Verify the tracker is still valid
                    if (tracker_idx >= 0 && tracker_idx < MAX_TIMESTAMP_TRACKERS && 
                        timestamp_trackers[tracker_idx].initialized && 
                        strcmp(timestamp_trackers[tracker_idx].stream_name, stream_name) == 0) {
                        
                        timestamp_trackers[tracker_idx].pts_discontinuity_count++;
                        local_count = timestamp_trackers[tracker_idx].pts_discontinuity_count;
                    }
                    pthread_mutex_unlock(&timestamp_mutex);
                    
                    // Only log occasionally to avoid flooding logs
                    if (local_count % 10 == 1) {
                        log_warn("Timestamp discontinuity detected in UDP stream %s: expected=%lld, actual=%lld, diff=%lld ms", 
                                stream_name, 
                                (long long)local_expected_next_pts, 
                                (long long)out_pkt->pts,
                                (long long)(pts_diff * 1000 * 
                                           (input_stream && input_stream->time_base.den > 0 ? 
                                            input_stream->time_base.num : 1) / 
                                           (input_stream && input_stream->time_base.den > 0 ? 
                                            input_stream->time_base.den : 90000)));
                    }
                    
                    // For severe discontinuities, try to correct by using expected PTS
                    if (pts_diff > 100 * frame_duration) {
                        int64_t original_pts = out_pkt->pts;
                        out_pkt->pts = local_expected_next_pts;
                        out_pkt->dts = out_pkt->pts;
                        
                        log_debug("Corrected severe timestamp discontinuity: original=%lld, corrected=%lld", 
                                 (long long)original_pts, (long long)out_pkt->pts);
                    }
                }
                
                // FIXED: Calculate the next expected PTS locally
                int64_t next_expected_pts = out_pkt->pts + frame_duration;
                
                // FIXED: Update all timestamp data in a single atomic operation
                pthread_mutex_lock(&timestamp_mutex);
                // Verify the tracker is still valid
                if (tracker_idx >= 0 && tracker_idx < MAX_TIMESTAMP_TRACKERS && 
                    timestamp_trackers[tracker_idx].initialized && 
                    strcmp(timestamp_trackers[tracker_idx].stream_name, stream_name) == 0) {
                    
                    timestamp_trackers[tracker_idx].last_pts = out_pkt->pts;
                    timestamp_trackers[tracker_idx].last_dts = out_pkt->dts;
                    timestamp_trackers[tracker_idx].expected_next_pts = next_expected_pts;
                }
                pthread_mutex_unlock(&timestamp_mutex);
            } else {
                // FIXED: Update timestamps even if we don't have a previous reference
                pthread_mutex_lock(&timestamp_mutex);
                // Verify the tracker is still valid
                if (tracker_idx >= 0 && tracker_idx < MAX_TIMESTAMP_TRACKERS && 
                    timestamp_trackers[tracker_idx].initialized && 
                    strcmp(timestamp_trackers[tracker_idx].stream_name, stream_name) == 0) {
                    
                    timestamp_trackers[tracker_idx].last_pts = out_pkt->pts;
                    timestamp_trackers[tracker_idx].last_dts = out_pkt->dts;
                }
                pthread_mutex_unlock(&timestamp_mutex);
            }
        }
    }
    
    // CRITICAL FIX: Use try/catch style with goto for better error handling
    // Use direct function calls instead of conditional branching for better performance
    if (writer_type == 0) {  // HLS writer
        hls_writer_t *hls_writer = (hls_writer_t *)writer;
        
        // CRITICAL FIX: Validate HLS writer before using
        if (!hls_writer) {
            log_error("NULL HLS writer for stream %s", stream_name);
            av_packet_free(&out_pkt);
            return -1;
        }
        
        // CRITICAL FIX: Check if the writer has been closed
        if (!hls_writer->output_ctx) {
            log_error("HLS writer has been closed for stream %s", stream_name);
            av_packet_free(&out_pkt);
            return -1;
        }
        
        // CRITICAL FIX: Ensure timestamps are valid before writing
        // This is especially important for UDP streams
        if (out_pkt->pts == AV_NOPTS_VALUE && out_pkt->dts != AV_NOPTS_VALUE) {
            out_pkt->pts = out_pkt->dts;
        } else if (out_pkt->dts == AV_NOPTS_VALUE && out_pkt->pts != AV_NOPTS_VALUE) {
            out_pkt->dts = out_pkt->pts;
        } else if (out_pkt->pts == AV_NOPTS_VALUE && out_pkt->dts == AV_NOPTS_VALUE) {
            // If both timestamps are missing, set them to 1 to avoid segfault
            out_pkt->pts = 1;
            out_pkt->dts = 1;
            log_debug("Set default timestamps for packet with no PTS/DTS in stream %s", stream_name);
        }
        
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
        
        // CRITICAL FIX: Validate MP4 writer before using
        if (!mp4_writer) {
            log_error("NULL MP4 writer for stream %s", stream_name);
            av_packet_free(&out_pkt);
            return -1;
        }
        
        // Removed adaptive frame dropping to improve quality
        // Always process all frames for better quality
        
        // CRITICAL FIX: Ensure timestamps are valid before writing
        // This is especially important for UDP streams
        if (out_pkt->pts == AV_NOPTS_VALUE && out_pkt->dts != AV_NOPTS_VALUE) {
            out_pkt->pts = out_pkt->dts;
        } else if (out_pkt->dts == AV_NOPTS_VALUE && out_pkt->pts != AV_NOPTS_VALUE) {
            out_pkt->dts = out_pkt->pts;
        } else if (out_pkt->pts == AV_NOPTS_VALUE && out_pkt->dts == AV_NOPTS_VALUE) {
            // If both timestamps are missing, set them to 1 to avoid segfault
            out_pkt->pts = 1;
            out_pkt->dts = 1;
            log_debug("Set default timestamps for packet with no PTS/DTS in stream %s", stream_name);
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
    }
    
    // Clean up our packet reference
    if (out_pkt) {
        av_packet_free(&out_pkt);
    }
    
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
 * Reset timestamp tracker for a specific stream
 * This should be called when a stream is stopped to ensure clean state when restarted
 */
void reset_timestamp_tracker(const char *stream_name) {
    if (!stream_name) {
        log_error("reset_timestamp_tracker: NULL stream name");
        return;
    }
    
    pthread_mutex_lock(&timestamp_mutex);
    
    // Find the tracker for this stream
    bool found = false;
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].initialized && 
            strcmp(timestamp_trackers[i].stream_name, stream_name) == 0) {
            // Reset the tracker but keep the stream name and initialized flag
            // This ensures we don't lose the UDP flag setting
            timestamp_trackers[i].last_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i].last_dts = AV_NOPTS_VALUE;
            timestamp_trackers[i].pts_discontinuity_count = 0;
            timestamp_trackers[i].expected_next_pts = AV_NOPTS_VALUE;
            
            log_info("Reset timestamp tracker for stream %s (UDP flag: %s)", 
                    stream_name, timestamp_trackers[i].is_udp_stream ? "true" : "false");
            found = true;
            break;
        }
    }
    
    if (!found) {
        log_debug("No timestamp tracker found for stream %s during reset", stream_name);
    }
    
    pthread_mutex_unlock(&timestamp_mutex);
}

/**
 * Remove timestamp tracker for a specific stream
 * This should be called when a stream is completely removed
 */
void remove_timestamp_tracker(const char *stream_name) {
    if (!stream_name) {
        log_error("remove_timestamp_tracker: NULL stream name");
        return;
    }
    
    pthread_mutex_lock(&timestamp_mutex);
    
    // Find the tracker for this stream
    bool found = false;
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        if (timestamp_trackers[i].initialized && 
            strcmp(timestamp_trackers[i].stream_name, stream_name) == 0) {
            // Completely reset the tracker
            timestamp_trackers[i].initialized = false;
            timestamp_trackers[i].stream_name[0] = '\0';
            timestamp_trackers[i].last_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i].last_dts = AV_NOPTS_VALUE;
            timestamp_trackers[i].pts_discontinuity_count = 0;
            timestamp_trackers[i].expected_next_pts = AV_NOPTS_VALUE;
            timestamp_trackers[i].is_udp_stream = false;
            
            log_info("Removed timestamp tracker for stream %s", stream_name);
            found = true;
            break;
        }
    }
    
    if (!found) {
        log_debug("No timestamp tracker found for stream %s during removal", stream_name);
    }
    
    pthread_mutex_unlock(&timestamp_mutex);
}

/**
 * Cleanup all timestamp trackers
 */
static void cleanup_timestamp_trackers(void) {
    pthread_mutex_lock(&timestamp_mutex);
    
    // Reset all trackers to unused state
    for (int i = 0; i < MAX_TIMESTAMP_TRACKERS; i++) {
        timestamp_trackers[i].initialized = false;
        timestamp_trackers[i].stream_name[0] = '\0';
        timestamp_trackers[i].last_pts = AV_NOPTS_VALUE;
        timestamp_trackers[i].last_dts = AV_NOPTS_VALUE;
        timestamp_trackers[i].pts_discontinuity_count = 0;
        timestamp_trackers[i].expected_next_pts = AV_NOPTS_VALUE;
        timestamp_trackers[i].is_udp_stream = false;
    }
    
    pthread_mutex_unlock(&timestamp_mutex);
    log_info("All timestamp trackers cleaned up");
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
