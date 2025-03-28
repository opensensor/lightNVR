#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <stdatomic.h>
#include <time.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/time.h>

#include "core/logger.h"
#include "core/shutdown_coordinator.h"
#include "video/hls_writer.h"
#include "video/hls_writer_thread.h"
#include "video/stream_protocol.h"
#include "video/stream_state.h"
#include "video/streams.h"
#include "video/thread_utils.h"

/**
 * HLS writer thread function
 */
static void *hls_writer_thread_func(void *arg) {
    hls_writer_thread_ctx_t *ctx = (hls_writer_thread_ctx_t *)arg;
    AVFormatContext *input_ctx = NULL;
    AVPacket *pkt = NULL;
    int video_stream_idx = -1;
    int ret;
    
    if (!ctx) {
        log_error("NULL context passed to HLS writer thread");
        return NULL;
    }
    
    // Create a local copy of the stream name for thread safety
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, ctx->stream_name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';
    
    log_info("Starting HLS writer thread for stream %s", stream_name);
    
    // Check if we're still running before proceeding
    if (!atomic_load(&ctx->running)) {
        log_warn("HLS writer thread for %s started but already marked as not running", stream_name);
        return NULL;
    }
    
    // Get the stream state manager
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (!state) {
        log_error("Could not find stream state for %s", stream_name);
        atomic_store(&ctx->running, 0);
        return NULL;
    }
    
    // Use the stream_protocol.h functions to open the input stream with appropriate options
    ret = open_input_stream(&input_ctx, ctx->rtsp_url, ctx->protocol);
    if (ret < 0) {
        log_error("Could not open input stream for %s", stream_name);
        atomic_store(&ctx->running, 0);
        return NULL;
    }
    
    // Find video stream
    video_stream_idx = find_video_stream_index(input_ctx);
    if (video_stream_idx == -1) {
        log_error("No video stream found in %s", ctx->rtsp_url);
        avformat_close_input(&input_ctx);
        atomic_store(&ctx->running, 0);
        return NULL;
    }
    
    // Initialize packet
    pkt = av_packet_alloc();
    if (!pkt) {
        log_error("Failed to allocate packet");
        avformat_close_input(&input_ctx);
        atomic_store(&ctx->running, 0);
        return NULL;
    }
    
    // Register with shutdown coordinator
    char component_name[128];
    snprintf(component_name, sizeof(component_name), "hls_writer_thread_%s", stream_name);
    ctx->shutdown_component_id = register_component(component_name, COMPONENT_HLS_WRITER, ctx, 60); // Lowest priority (60)
    if (ctx->shutdown_component_id >= 0) {
        log_info("Registered HLS writer thread %s with shutdown coordinator (ID: %d)", stream_name, ctx->shutdown_component_id);
    }
    
    // Main packet reading loop
    while (atomic_load(&ctx->running)) {
        // Check if shutdown has been initiated
        if (is_shutdown_initiated()) {
            log_info("HLS writer thread for %s stopping due to system shutdown", stream_name);
            atomic_store(&ctx->running, 0);
            break;
        }
        
        // Check if the stream state indicates we should stop
        if (is_stream_state_stopping(state)) {
            log_info("HLS writer thread for %s stopping due to stream state STOPPING", stream_name);
            atomic_store(&ctx->running, 0);
            break;
        }
        
        // Check if we should exit before potentially blocking on av_read_frame
        if (!atomic_load(&ctx->running)) {
            log_info("HLS writer thread for %s detected shutdown before read", stream_name);
            break;
        }
        
        // Read packet
        ret = av_read_frame(input_ctx, pkt);
        
        if (ret < 0) {
            if (ret == AVERROR_EOF || ret == AVERROR(EAGAIN)) {
                // End of stream or resource temporarily unavailable
                // Try to reconnect after a short delay
                av_packet_unref(pkt);
                log_warn("Stream %s disconnected, attempting to reconnect...", stream_name);
                
                av_usleep(1000000);  // 1 second delay for more reliable reconnection
                
                // Close and reopen input
                avformat_close_input(&input_ctx);
                
                // Use the stream_protocol.h function to reopen the input stream
                ret = open_input_stream(&input_ctx, ctx->rtsp_url, ctx->protocol);
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
        
        // Get the stream for this packet
        AVStream *input_stream = NULL;
        if (pkt->stream_index >= 0 && pkt->stream_index < input_ctx->nb_streams) {
            input_stream = input_ctx->streams[pkt->stream_index];
        } else {
            log_warn("Invalid stream index %d for stream %s", pkt->stream_index, stream_name);
            av_packet_unref(pkt);
            continue;
        }
        
        // Validate packet data
        if (!pkt->data || pkt->size <= 0) {
            log_warn("Invalid packet (null data or zero size) for stream %s", stream_name);
            av_packet_unref(pkt);
            continue;
        }
        
        // Only process video packets
        if (pkt->stream_index == video_stream_idx) {
            // Lock the writer mutex
            pthread_mutex_lock(&ctx->writer->mutex);
            
            ret = hls_writer_write_packet(ctx->writer, pkt, input_stream);
            
            // CRITICAL FIX: Completely disable manual flushing on key frames
            // The FFmpeg HLS muxer already handles flushing internally during segment transitions
            // Manual flushing can cause segmentation faults when it happens during a segment transition
            // when the internal state of the output context is not stable
            
            // Instead of manual flushing, we'll rely on FFmpeg's internal flushing mechanism
            // This is safer and more reliable, especially during segment transitions
            
            // For debugging purposes only, log key frames
            bool is_key_frame = (pkt->flags & AV_PKT_FLAG_KEY) != 0;
            if (is_key_frame && ret >= 0) {
                log_debug("Processed key frame for stream %s (no manual flush)", stream_name);
                
                // CRITICAL FIX: Force a small delay after key frames to help prevent ghosting artifacts
                // This gives the HLS muxer more time to properly process segment transitions
                av_usleep(5000); // 5ms delay - small enough not to affect performance but helps with timing
            }
            
            // Ensure we don't leak memory if the packet write fails
            if (ret < 0) {
                char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                log_warn("Error writing packet to HLS for stream %s: %s", stream_name, error_buf);
            }
            
            // Unlock the writer mutex
            pthread_mutex_unlock(&ctx->writer->mutex);
            
            // Process packet for detection if the stream has detection enabled
            // This wires in detection events to the always-on HLS streaming
            if (input_stream && input_stream->codecpar) {
                process_packet_for_detection(stream_name, pkt, input_stream->codecpar);
            }
        }
        
        av_packet_unref(pkt);
    }
    
    // Cleanup resources
    if (pkt) {
        // Make a local copy of the packet pointer and NULL out the original
        // to prevent double-free if another thread accesses it
        AVPacket *pkt_to_free = pkt;
        pkt = NULL;
        
        // Now safely free the packet - first unref then free to prevent memory leaks
        av_packet_unref(pkt_to_free);
        av_packet_free(&pkt_to_free);
    }
    
    if (input_ctx) {
        // Make a local copy of the context pointer and NULL out the original
        AVFormatContext *ctx_to_close = input_ctx;
        input_ctx = NULL;
        
        // Now safely close the input context
        avformat_close_input(&ctx_to_close);
    }
    
    // Update component state in shutdown coordinator
    if (ctx->shutdown_component_id >= 0) {
        update_component_state(ctx->shutdown_component_id, COMPONENT_STOPPED);
        log_info("Updated HLS writer thread %s state to STOPPED in shutdown coordinator", stream_name);
    }
    
    log_info("HLS writer thread for stream %s exited", stream_name);
    return NULL;
}

/**
 * Start a recording thread that reads from the RTSP stream and writes to the HLS files
 */
int hls_writer_start_recording_thread(hls_writer_t *writer, const char *rtsp_url, const char *stream_name, int protocol) {
    if (!writer) {
        log_error("NULL writer passed to hls_writer_start_recording_thread");
        return -1;
    }
    
    if (!rtsp_url) {
        log_error("NULL RTSP URL passed to hls_writer_start_recording_thread");
        return -1;
    }
    
    if (!stream_name) {
        log_error("NULL stream name passed to hls_writer_start_recording_thread");
        return -1;
    }
    
    // Check if thread is already running
    if (writer->thread_ctx) {
        log_warn("HLS writer thread for %s is already running", stream_name);
        return 0;
    }
    
    // Create thread context
    hls_writer_thread_ctx_t *ctx = (hls_writer_thread_ctx_t *)calloc(1, sizeof(hls_writer_thread_ctx_t));
    if (!ctx) {
        log_error("Failed to allocate HLS writer thread context");
        return -1;
    }
    
    // Initialize context
    strncpy(ctx->rtsp_url, rtsp_url, MAX_PATH_LENGTH - 1);
    strncpy(ctx->stream_name, stream_name, MAX_STREAM_NAME - 1);
    ctx->writer = writer;
    ctx->protocol = protocol;
    atomic_store(&ctx->running, 1);
    ctx->shutdown_component_id = -1;
    
    // Store thread context in writer
    writer->thread_ctx = ctx;
    
    // Start thread
    if (pthread_create(&ctx->thread, NULL, hls_writer_thread_func, ctx) != 0) {
        log_error("Failed to create HLS writer thread for %s", stream_name);
        free(ctx);
        writer->thread_ctx = NULL;
        return -1;
    }
    
    log_info("Started HLS writer thread for %s", stream_name);
    return 0;
}

/**
 * Stop the recording thread
 */
void hls_writer_stop_recording_thread(hls_writer_t *writer) {
    if (!writer) {
        log_warn("NULL writer passed to hls_writer_stop_recording_thread");
        return;
    }
    
    // Check if thread is running
    if (!writer->thread_ctx) {
        log_warn("HLS writer thread is not running");
        return;
    }
    
    // Get thread context
    hls_writer_thread_ctx_t *ctx = (hls_writer_thread_ctx_t *)writer->thread_ctx;
    
    // Create a local copy of the stream name for logging
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, ctx->stream_name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';
    
    log_info("Stopping HLS writer thread for %s", stream_name);
    
    // Signal thread to stop
    atomic_store(&ctx->running, 0);
    
    // Join thread with timeout
    int join_result = pthread_join_with_timeout(ctx->thread, NULL, 5);
    if (join_result != 0) {
        log_error("Failed to join HLS writer thread for %s (error: %d), detaching thread", 
                 stream_name, join_result);
        
        // Detach thread to avoid resource leaks
        pthread_detach(ctx->thread);
    } else {
        log_info("Successfully joined HLS writer thread for %s", stream_name);
    }
    
    // Ensure we update the component state even if join failed
    if (ctx->shutdown_component_id >= 0) {
        update_component_state(ctx->shutdown_component_id, COMPONENT_STOPPED);
    }
    
    // Free thread context
    free(ctx);
    writer->thread_ctx = NULL;
    
    log_info("Stopped HLS writer thread for %s", stream_name);
}

/**
 * Check if the recording thread is running
 */
int hls_writer_is_recording(hls_writer_t *writer) {
    if (!writer) {
        return 0;
    }
    
    if (!writer->thread_ctx) {
        return 0;
    }
    
    hls_writer_thread_ctx_t *ctx = (hls_writer_thread_ctx_t *)writer->thread_ctx;
    return atomic_load(&ctx->running);
}
