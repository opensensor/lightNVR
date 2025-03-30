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
#include "video/detection_thread_pool.h"
#include "video/hls_writer.h"
#include "video/hls_writer_thread.h"
#include "video/stream_protocol.h"
#include "video/stream_state.h"
#include "video/streams.h"
#include "video/thread_utils.h"

// Maximum number of consecutive reconnection failures before giving up
#define MAX_RECONNECTION_FAILURES 20

// Maximum time (in seconds) without receiving a packet before considering the connection dead
// Reduced from 30 to 10 seconds to detect stalled connections faster
#define MAX_PACKET_TIMEOUT 10

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
        // Don't set running to 0, just mark connection as invalid and increment failures
        // This allows the hls_stream_thread to retry
        atomic_store(&ctx->connection_valid, 0);
        atomic_store(&ctx->consecutive_failures, atomic_load(&ctx->consecutive_failures) + 1);
        return NULL;
    }
    
    // Connection is now valid
    atomic_store(&ctx->connection_valid, 1);
    atomic_store(&ctx->consecutive_failures, 0);
    
    // Find video stream
    video_stream_idx = find_video_stream_index(input_ctx);
    if (video_stream_idx == -1) {
        log_error("No video stream found in %s", ctx->rtsp_url);
        avformat_close_input(&input_ctx);
        // Don't set running to 0, just mark connection as invalid and increment failures
        // This allows the hls_stream_thread to retry
        atomic_store(&ctx->connection_valid, 0);
        atomic_store(&ctx->consecutive_failures, atomic_load(&ctx->consecutive_failures) + 1);
        return NULL;
    }
    
    // Initialize packet
    pkt = av_packet_alloc();
    if (!pkt) {
        log_error("Failed to allocate packet");
        avformat_close_input(&input_ctx);
        atomic_store(&ctx->running, 0);
        atomic_store(&ctx->connection_valid, 0);
        return NULL;
    }
    
    // Register with shutdown coordinator
    char component_name[128];
    snprintf(component_name, sizeof(component_name), "hls_writer_thread_%s", stream_name);
    ctx->shutdown_component_id = register_component(component_name, COMPONENT_HLS_WRITER, ctx, 60); // Lowest priority (60)
    if (ctx->shutdown_component_id >= 0) {
        log_info("Registered HLS writer thread %s with shutdown coordinator (ID: %d)", stream_name, ctx->shutdown_component_id);
    }
    
    // Update last packet time
    time_t current_time = time(NULL);
    atomic_store(&ctx->last_packet_time, (int_fast64_t)current_time);
    
    // Main packet reading loop
    while (atomic_load(&ctx->running)) {
        // Check if shutdown has been initiated
        if (is_shutdown_initiated()) {
            log_info("HLS writer thread for %s stopping due to system shutdown", stream_name);
            atomic_store(&ctx->running, 0);
            atomic_store(&ctx->connection_valid, 0);
            break;
        }
        
        // Check if the stream state indicates we should stop
        if (is_stream_state_stopping(state)) {
            log_info("HLS writer thread for %s stopping due to stream state STOPPING", stream_name);
            atomic_store(&ctx->running, 0);
            atomic_store(&ctx->connection_valid, 0);
            break;
        }
        
        // Check if we should exit before potentially blocking on av_read_frame
        if (!atomic_load(&ctx->running)) {
            log_info("HLS writer thread for %s detected shutdown before read", stream_name);
            atomic_store(&ctx->connection_valid, 0);
            break;
        }
        
        // Read packet
        ret = av_read_frame(input_ctx, pkt);
        
        if (ret < 0) {
            if (ret == AVERROR_EOF || ret == AVERROR(EAGAIN) || ret == AVERROR(ETIMEDOUT) || 
                ret == AVERROR(EIO) || ret == AVERROR(ECONNRESET) || ret == AVERROR(ECONNREFUSED)) {
                // End of stream, resource temporarily unavailable, timeout, I/O error, or connection reset
                // Try to reconnect after a short delay
                av_packet_unref(pkt);
                
                // Mark connection as invalid
                atomic_store(&ctx->connection_valid, 0);
                
                // Increment consecutive failures
                int failures = atomic_fetch_add(&ctx->consecutive_failures, 1) + 1;
                
                // Log at ERROR level to ensure visibility regardless of log level setting
                log_error("Stream %s disconnected (error code: %d), attempting to reconnect (failures: %d)...", 
                         stream_name, ret, failures);
                
                // Check if we've exceeded the maximum number of consecutive failures
                if (failures > MAX_RECONNECTION_FAILURES) {
                    log_error("Exceeded maximum number of consecutive reconnection failures (%d) for stream %s, giving up", 
                             MAX_RECONNECTION_FAILURES, stream_name);
                    atomic_store(&ctx->running, 0);
                    break;
                }
                
                // Implement exponential backoff for reconnection attempts
                static int reconnect_delay_ms = 1000; // Start with 1 second
                
                // Log the reconnection attempt at ERROR level
                log_error("Reconnection attempt %d for stream %s, waiting %d ms", 
                         failures, stream_name, reconnect_delay_ms);
                
                av_usleep(reconnect_delay_ms * 1000);  // Convert ms to microseconds
                
                // Increase delay for next attempt (exponential backoff with max of 60 seconds)
                reconnect_delay_ms = reconnect_delay_ms * 2;
                if (reconnect_delay_ms > 60000) {
                    reconnect_delay_ms = 60000;
                }
                
                // Close and reopen input
                avformat_close_input(&input_ctx);
                
                // Use the stream_protocol.h function to reopen the input stream
                ret = open_input_stream(&input_ctx, ctx->rtsp_url, ctx->protocol);
                if (ret < 0) {
                    log_error("Could not reconnect to input stream for %s (attempt %d), will retry", 
                             stream_name, failures);
                    continue;  // Keep trying
                }
                
                // Find video stream again
                video_stream_idx = find_video_stream_index(input_ctx);
                if (video_stream_idx == -1) {
                    log_error("No video stream found after reconnect for %s (attempt %d), will retry", 
                             stream_name, failures);
                    continue;  // Keep trying
                }
                
                // Reset reconnection parameters on successful reconnection
                log_error("Successfully reconnected to stream %s after %d attempts", 
                         stream_name, failures);
                atomic_store(&ctx->consecutive_failures, 0);
                atomic_store(&ctx->connection_valid, 1);
                atomic_store(&ctx->last_packet_time, (int_fast64_t)time(NULL));
                reconnect_delay_ms = 1000;
                
                continue;
            } else {
                char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                log_error("Error reading frame: %s (error code: %d)", error_buf, ret);
                
                // Mark connection as invalid
                atomic_store(&ctx->connection_valid, 0);
                
                // Increment consecutive failures
                int failures = atomic_fetch_add(&ctx->consecutive_failures, 1) + 1;
                
                // Check if we've exceeded the maximum number of consecutive failures
                if (failures > MAX_RECONNECTION_FAILURES) {
                    log_error("Exceeded maximum number of consecutive reconnection failures (%d) for stream %s, giving up", 
                             MAX_RECONNECTION_FAILURES, stream_name);
                    atomic_store(&ctx->running, 0);
                    break;
                }
                
                // Don't break on other errors, try to reconnect instead
                av_packet_unref(pkt);
                log_error("Unexpected error for stream %s (failures: %d), attempting to reconnect...", 
                         stream_name, failures);
                
                av_usleep(1000000);  // 1 second delay
                
                // Close and reopen input
                avformat_close_input(&input_ctx);
                
                // Use the stream_protocol.h function to reopen the input stream
                ret = open_input_stream(&input_ctx, ctx->rtsp_url, ctx->protocol);
                if (ret < 0) {
                    log_error("Could not reconnect to input stream for %s after unexpected error", stream_name);
                    continue;  // Keep trying
                }
                
                // Find video stream again
                video_stream_idx = find_video_stream_index(input_ctx);
                if (video_stream_idx == -1) {
                    log_error("No video stream found after reconnect for %s after unexpected error", stream_name);
                    continue;  // Keep trying
                }
                
                // Reset consecutive failures on successful reconnection
                atomic_store(&ctx->consecutive_failures, 0);
                atomic_store(&ctx->connection_valid, 1);
                atomic_store(&ctx->last_packet_time, (int_fast64_t)time(NULL));
                
                continue;
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
            } else {
                // Successfully processed a packet
                time_t current_time = time(NULL);
                atomic_store(&ctx->last_packet_time, (int_fast64_t)current_time);
                atomic_store(&ctx->consecutive_failures, 0);
                atomic_store(&ctx->connection_valid, 1);
            }
            
            // Unlock the writer mutex
            pthread_mutex_unlock(&ctx->writer->mutex);
            
            // Submit packet to detection thread pool if the stream has detection enabled
            // This wires in detection events to the always-on HLS streaming
            if (input_stream && input_stream->codecpar) {
                // Use the detection thread pool instead of direct processing
                // Note: submit_detection_task creates its own copies of the packet and codec parameters
                // so we don't need to worry about memory management here
                submit_detection_task(stream_name, pkt, input_stream->codecpar);
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
        pkt_to_free = NULL;  // Set to NULL after freeing to prevent double-free
    }
    
    if (input_ctx) {
        // Make a local copy of the context pointer and NULL out the original
        AVFormatContext *ctx_to_close = input_ctx;
        input_ctx = NULL;
        
        // Now safely close the input context
        avformat_close_input(&ctx_to_close);
        // No need to set ctx_to_close to NULL as avformat_close_input already does this
    }
    
    // Mark connection as invalid
    atomic_store(&ctx->connection_valid, 0);
    
    // Update component state in shutdown coordinator
    if (ctx->shutdown_component_id >= 0) {
        update_component_state(ctx->shutdown_component_id, COMPONENT_STOPPED);
        log_info("Updated HLS writer thread %s state to STOPPED in shutdown coordinator", stream_name);
    }
    
    log_info("HLS writer thread for stream %s exited", stream_name);
    return NULL;
}

// Forward declaration for go2rtc integration
extern bool go2rtc_integration_is_using_go2rtc_for_hls(const char *stream_name);
extern bool go2rtc_get_rtsp_url(const char *stream_name, char *url, size_t url_size);

/**
 * Start a recording thread that reads from the RTSP stream and writes to the HLS files
 * Enhanced with robust error handling for go2rtc integration
 */
int hls_writer_start_recording_thread(hls_writer_t *writer, const char *rtsp_url, const char *stream_name, int protocol) {
    // Validate input parameters with detailed error messages
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
    
    // Check if this stream is using go2rtc for HLS
    char actual_url[MAX_PATH_LENGTH];
    
    // Use the original URL by default
    strncpy(actual_url, rtsp_url, sizeof(actual_url) - 1);
    actual_url[sizeof(actual_url) - 1] = '\0';
    
    // If the stream is using go2rtc for HLS, get the go2rtc RTSP URL
    // The go2rtc_integration_is_using_go2rtc_for_hls function will only return true
    // if go2rtc is fully ready and the stream is registered
    if (go2rtc_integration_is_using_go2rtc_for_hls(stream_name)) {
        // Get the go2rtc RTSP URL
        if (go2rtc_get_rtsp_url(stream_name, actual_url, sizeof(actual_url))) {
            log_info("Using go2rtc RTSP URL for HLS streaming: %s", actual_url);
        } else {
            log_warn("Failed to get go2rtc RTSP URL for stream %s, falling back to original URL", stream_name);
        }
    }
    
    // Check if thread is already running
    if (writer->thread_ctx) {
        log_warn("HLS writer thread for %s is already running", stream_name);
        return 0;
    }
    
    // Create thread context with additional error handling
    hls_writer_thread_ctx_t *ctx = NULL;
    
    // Use a try/catch style approach with goto for cleanup
    int result = -1;
    
    // Allocate thread context
    ctx = (hls_writer_thread_ctx_t *)calloc(1, sizeof(hls_writer_thread_ctx_t));
    if (!ctx) {
        log_error("Failed to allocate HLS writer thread context");
        return -1;
    }
    
    // Initialize context with safe string operations
    if (strlen(actual_url) >= MAX_PATH_LENGTH) {
        log_error("RTSP URL too long for HLS writer thread context");
        goto cleanup;
    }
    
    if (strlen(stream_name) >= MAX_STREAM_NAME) {
        log_error("Stream name too long for HLS writer thread context");
        goto cleanup;
    }
    
    // Use the go2rtc URL if available, otherwise use the original URL
    strncpy(ctx->rtsp_url, actual_url, MAX_PATH_LENGTH - 1);
    ctx->rtsp_url[MAX_PATH_LENGTH - 1] = '\0';
    
    // Log the URL being used
    log_info("Using URL for HLS streaming of stream %s: %s", stream_name, actual_url);
    
    strncpy(ctx->stream_name, stream_name, MAX_STREAM_NAME - 1);
    ctx->stream_name[MAX_STREAM_NAME - 1] = '\0';
    
    ctx->writer = writer;
    ctx->protocol = protocol;
    atomic_store(&ctx->running, 1);
    ctx->shutdown_component_id = -1;
    
    // Initialize new fields
    atomic_store(&ctx->last_packet_time, (int_fast64_t)time(NULL));
    atomic_store(&ctx->connection_valid, 0); // Start with connection invalid
    atomic_store(&ctx->consecutive_failures, 0);
    
    // Store thread context in writer BEFORE creating the thread
    // This ensures the thread context is available to other threads
    writer->thread_ctx = ctx;
    
    // Start thread with error handling
    int thread_result = pthread_create(&ctx->thread, NULL, hls_writer_thread_func, ctx);
    if (thread_result != 0) {
        log_error("Failed to create HLS writer thread for %s (error: %s)", 
                 stream_name, strerror(thread_result));
        writer->thread_ctx = NULL; // Reset the thread context pointer
        goto cleanup;
    }
    
    log_info("Started HLS writer thread for %s", stream_name);
    return 0;
    
cleanup:
    // Clean up resources if thread creation failed
    if (ctx) {
        free(ctx);
        ctx = NULL;
    }
    
    return result;
}

/**
 * Stop the recording thread
 * This function is now safer with go2rtc integration
 */
void hls_writer_stop_recording_thread(hls_writer_t *writer) {
    if (!writer) {
        log_warn("NULL writer passed to hls_writer_stop_recording_thread");
        return;
    }
    
    // Safely check if thread is running
    void *thread_ctx_ptr = writer->thread_ctx;
    if (!thread_ctx_ptr) {
        log_warn("HLS writer thread is not running");
        return;
    }
    
    // Make a safe local copy of the thread context pointer
    hls_writer_thread_ctx_t *ctx = (hls_writer_thread_ctx_t *)thread_ctx_ptr;
    
    // Create a local copy of the stream name for logging
    char stream_name[MAX_STREAM_NAME] = {0};
    if (ctx->stream_name[0] != '\0') {
        strncpy(stream_name, ctx->stream_name, MAX_STREAM_NAME - 1);
        stream_name[MAX_STREAM_NAME - 1] = '\0';
    } else {
        strcpy(stream_name, "unknown");
    }
    
    log_info("Stopping HLS writer thread for %s", stream_name);
    
    // Signal thread to stop
    atomic_store(&ctx->running, 0);
    atomic_store(&ctx->connection_valid, 0);
    
    // Store a local copy of the thread ID
    pthread_t thread_id = ctx->thread;
    
    // Mark the thread context as NULL in the writer BEFORE joining
    // This prevents other threads from trying to access it while we're shutting down
    writer->thread_ctx = NULL;
    
    // Update component state in shutdown coordinator
    if (ctx->shutdown_component_id >= 0) {
        update_component_state(ctx->shutdown_component_id, COMPONENT_STOPPING);
    }
    
    // Join thread with timeout - increased to 15 seconds for more reliable shutdown with go2rtc
    int join_result = pthread_join_with_timeout(thread_id, NULL, 15);
    if (join_result != 0) {
        log_error("Failed to join HLS writer thread for %s (error: %s), detaching thread", 
                 stream_name, strerror(join_result));
        
        // Detach thread to avoid resource leaks
        pthread_detach(thread_id);
        
        // Force the thread to be considered stopped in the shutdown coordinator
        if (ctx->shutdown_component_id >= 0) {
            update_component_state(ctx->shutdown_component_id, COMPONENT_STOPPED);
            log_info("Forced HLS writer thread %s state to STOPPED in shutdown coordinator after join failure", stream_name);
        }
    } else {
        log_info("Successfully joined HLS writer thread for %s", stream_name);
        
        // Update component state in shutdown coordinator
        if (ctx->shutdown_component_id >= 0) {
            update_component_state(ctx->shutdown_component_id, COMPONENT_STOPPED);
        }
    }
    
    // Add a small delay to ensure any in-progress operations complete
    usleep(100000); // 100ms
    
    // Free thread context
    free(ctx);
    ctx = NULL; // Set to NULL after freeing to prevent double-free
    
    log_info("Stopped HLS writer thread for %s", stream_name);
}

/**
 * Check if the recording thread is running and has a valid connection
 * This function is now safer with go2rtc integration
 */
int hls_writer_is_recording(hls_writer_t *writer) {
    // Basic validation
    if (!writer) {
        return 0;
    }
    
    // Safely check thread context
    void *thread_ctx_ptr = writer->thread_ctx;
    if (!thread_ctx_ptr) {
        return 0;
    }
    
    // Make a safe local copy of the thread context pointer
    hls_writer_thread_ctx_t *ctx = (hls_writer_thread_ctx_t *)thread_ctx_ptr;
    
    // Create a local copy of the stream name for logging
    char stream_name[MAX_STREAM_NAME] = {0};
    if (ctx->stream_name[0] != '\0') {
        strncpy(stream_name, ctx->stream_name, MAX_STREAM_NAME - 1);
        stream_name[MAX_STREAM_NAME - 1] = '\0';
    } else {
        strcpy(stream_name, "unknown");
    }
    
    // Check if the thread is running
    if (!atomic_load(&ctx->running)) {
        return 0;
    }
    
    // Check if the connection is valid
    if (!atomic_load(&ctx->connection_valid)) {
        return 0;
    }
    
    // Check if we've received a packet recently
    time_t now = time(NULL);
    time_t last_packet_time = (time_t)atomic_load(&ctx->last_packet_time);
    if (now - last_packet_time > MAX_PACKET_TIMEOUT) {
        log_error("HLS writer thread for %s has not received a packet in %ld seconds, considering it dead",
                 stream_name, now - last_packet_time);
        
        // Mark connection as invalid to trigger reconnection
        atomic_store(&ctx->connection_valid, 0);
        return 0;
    }
    
    return 1;
}
