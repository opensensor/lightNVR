#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <fcntl.h>
#include <errno.h>
#include <time.h>
#include <stdbool.h>

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/time.h>

#include "core/logger.h"
#include "core/config.h"
#include "video/stream_manager.h"
#include "video/stream_state.h"
#include "video/streams.h"
#include "video/hls_writer.h"
#include "video/hls_streaming.h"
#include "video/stream_transcoding.h"
#include "video/stream_reader.h"
#include "video/detection_stream.h"
#include "video/detection_integration.h"
#include "video/stream_packet_processor.h"
#include "video/thread_utils.h"
#include "video/timestamp_manager.h"
#include "video/hls/hls_context.h"
#include "video/hls/hls_stream_thread.h"
#include "video/hls/hls_directory.h"
#include "video/mp4_recording.h"

// Forward declaration of the detection function from hls_writer.c
extern void process_packet_for_detection(const char *stream_name, const AVPacket *pkt, const AVCodecParameters *codec_params);

// REMOVED: hls_packet_callback function is no longer needed since we're using the single thread approach

/**
 * HLS streaming thread function for a single stream
 * Simplified implementation that uses a single thread approach with improved state management
 */
void *hls_stream_thread(void *arg) {
    hls_stream_ctx_t *ctx = (hls_stream_ctx_t *)arg;
    AVFormatContext *input_ctx = NULL;
    AVPacket *pkt = NULL;
    int video_stream_idx = -1;
    int ret;
    time_t start_time = time(NULL);  // Record when we started

    // CRITICAL FIX: Add extra validation for context
    if (!ctx) {
        log_error("NULL context passed to HLS streaming thread");
        return NULL;
    }
    
    // CRITICAL FIX: Create a local copy of the stream name for thread safety
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, ctx->config.name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Get the stream state manager
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (!state) {
        log_error("Could not find stream state for %s", stream_name);
        ctx->running = 0;
        return NULL;
    }
    
    log_info("Starting HLS streaming thread for stream %s", stream_name);
    
    // CRITICAL FIX: Check if we're still running before proceeding
    if (!ctx->running) {
        log_warn("HLS streaming thread for %s started but already marked as not running", stream_name);
        return NULL;
    }

    // Verify output directory exists and is writable
    if (ensure_hls_directory(ctx->output_path, stream_name) != 0) {
        log_error("Failed to ensure HLS output directory: %s", ctx->output_path);
        ctx->running = 0;
        return NULL;
    }
    
    // CRITICAL FIX: Check if we're still running after directory creation
    if (!ctx->running) {
        log_info("HLS streaming thread for %s stopping after directory creation", stream_name);
        return NULL;
    }

    // Create HLS writer - adding the segment_duration parameter
    // Using a default of 2 seconds if not specified in config
    // Reduced from 4 to 2 seconds to decrease latency
    int segment_duration = ctx->config.segment_duration > 0 ?
                          ctx->config.segment_duration : 2;

    ctx->hls_writer = hls_writer_create(ctx->output_path, stream_name, segment_duration);
    if (!ctx->hls_writer) {
        log_error("Failed to create HLS writer for %s", stream_name);
        ctx->running = 0;
        return NULL;
    }

    // CRITICAL FIX: Check if we're still running after HLS writer creation
    if (!ctx->running) {
        log_info("HLS streaming thread for %s stopping after HLS writer creation", stream_name);
        if (ctx->hls_writer) {
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }
        return NULL;
    }
    
    // SIMPLIFIED APPROACH: Let FFmpeg handle manifest file creation
    // Just ensure the directory exists and is writable
    log_info("Letting FFmpeg handle HLS manifest file creation for stream %s", stream_name);
    
    // Get stream configuration
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Stream %s not found", stream_name);
        
        if (ctx->hls_writer) {
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }
        
        ctx->running = 0;
        return NULL;
    }
    
    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get config for stream %s", stream_name);
        
        if (ctx->hls_writer) {
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }
        
        ctx->running = 0;
        return NULL;
    }
    
    // Use the stream_protocol.h functions to open the input stream with appropriate options
    ret = open_input_stream(&input_ctx, config.url, config.protocol);
    if (ret < 0) {
        log_error("Could not open input stream for %s", stream_name);
        
        if (ctx->hls_writer) {
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }
        
        ctx->running = 0;
        return NULL;
    }
    
    // Find video stream
    video_stream_idx = find_video_stream_index(input_ctx);
    if (video_stream_idx == -1) {
        log_error("No video stream found in %s", config.url);
        
        avformat_close_input(&input_ctx);
        
        if (ctx->hls_writer) {
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }
        
        ctx->running = 0;
        return NULL;
    }
    
    // Initialize packet
    pkt = av_packet_alloc();
    if (!pkt) {
        log_error("Failed to allocate packet");
        
        avformat_close_input(&input_ctx);
        
        if (ctx->hls_writer) {
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }
        
        ctx->running = 0;
        return NULL;
    }
    
    // Main packet reading loop
    while (ctx->running) {
        // Check if the stream state indicates we should stop
        if (state) {
            if (is_stream_state_stopping(state)) {
                log_info("HLS streaming thread for %s stopping due to stream state STOPPING", stream_name);
                ctx->running = 0;
                break;
            }
            
            if (!are_stream_callbacks_enabled(state)) {
                log_info("HLS streaming thread for %s stopping due to callbacks disabled", stream_name);
                ctx->running = 0;
                break;
            }
        }
        
        ret = av_read_frame(input_ctx, pkt);
        
        if (ret < 0) {
            if (ret == AVERROR_EOF || ret == AVERROR(EAGAIN)) {
                // End of stream or resource temporarily unavailable
                // Try to reconnect after a short delay
                av_packet_unref(pkt);
                log_warn("Stream %s disconnected, attempting to reconnect...", stream_name);
                
                av_usleep(2000000);  // 2 second delay
                
                // Close and reopen input
                avformat_close_input(&input_ctx);
                
                // Use the stream_protocol.h function to reopen the input stream
                ret = open_input_stream(&input_ctx, config.url, config.protocol);
                if (ret < 0) {
                    log_error("Could not reconnect to input stream for %s", stream_name);
                    continue;  // Keep trying
                }
                
                // Find video stream again
                video_stream_idx = find_video_stream_index(input_ctx);
                if (video_stream_idx == -1) {
                    log_error("No video stream found after reconnect for %s", stream_name);
                    continue;  // Keep trying
                }
                
                continue;
            } else {
                char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                log_error("Error reading frame: %s", error_buf);
                break;
            }
        }
        
            // Process video packets
            if (pkt->stream_index == video_stream_idx) {
                // Check if this is a key frame
                bool is_key_frame = (pkt->flags & AV_PKT_FLAG_KEY) != 0;
                
                // Write to HLS with error handling
                ret = hls_writer_write_packet(ctx->hls_writer, pkt, input_ctx->streams[video_stream_idx]);
                if (ret < 0) {
                    // Only log errors for key frames to reduce log spam
                    if (is_key_frame) {
                        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                        log_error("Failed to write packet to HLS for stream %s: %s", stream_name, error_buf);
                    }
                    // Continue anyway to keep the stream going
                }
                
                // CRITICAL FIX: Also write to MP4 if there's a registered MP4 writer for this stream
                // This ensures MP4 recording works with the new HLS streaming architecture
                mp4_writer_t *mp4_writer = get_mp4_writer_for_stream(stream_name);
                if (mp4_writer) {
                    ret = mp4_writer_write_packet(mp4_writer, pkt, input_ctx->streams[video_stream_idx]);
                    if (ret < 0) {
                        // Only log errors for key frames to reduce log spam
                        if (is_key_frame) {
                            char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                            av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                            log_error("Failed to write packet to MP4 for stream %s: %s", stream_name, error_buf);
                        }
                        // Continue anyway to keep the stream going
                    } else if (is_key_frame) {
                        log_debug("Successfully wrote key frame to MP4 for stream %s", stream_name);
                    }
                }
                
                // Process packet for detection if it's a key frame
                // This ensures we don't overload the detection system with too many frames
                if (is_key_frame && process_packet_for_detection) {
                    log_debug("Processing key frame for detection for stream %s", stream_name);
                    process_packet_for_detection(stream_name, pkt, input_ctx->streams[video_stream_idx]->codecpar);
                }
            }
        
        av_packet_unref(pkt);
    }
    
    // Cleanup resources
    if (pkt) {
        av_packet_free(&pkt);
    }
    
    if (input_ctx) {
        avformat_close_input(&input_ctx);
    }
    
    // When done, close writer
    if (ctx->hls_writer) {
        hls_writer_close(ctx->hls_writer);
        ctx->hls_writer = NULL;
    }
    
    log_info("HLS streaming thread for stream %s exited", stream_name);
    return NULL;
}
