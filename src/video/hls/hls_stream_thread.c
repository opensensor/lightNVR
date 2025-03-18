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
#include "video/packet_processor.h"
#include "video/thread_utils.h"
#include "video/timestamp_manager.h"
#include "video/hls/hls_context.h"
#include "video/hls/hls_stream_thread.h"
#include "video/hls/hls_directory.h"

/**
 * HLS packet processing callback function
 * Removed adaptive degrading to improve quality
 */
int hls_packet_callback(const AVPacket *pkt, const AVStream *stream, void *user_data) {
    // CRITICAL FIX: Add extra validation for all parameters
    if (!pkt) {
        log_error("HLS packet callback received NULL packet");
        return -1;
    }
    
    if (!stream) {
        log_error("HLS packet callback received NULL stream");
        return -1;
    }
    
    if (!user_data) {
        log_error("HLS packet callback received NULL user_data");
        return -1;
    }
    
    // CRITICAL FIX: Create a local copy of the stream name for thread safety before locking mutex
    hls_stream_ctx_t *streaming_ctx = (hls_stream_ctx_t *)user_data;
    char stream_name[MAX_STREAM_NAME] = {0};
    
    // Safely extract the stream name first
    if (streaming_ctx) {
        strncpy(stream_name, streaming_ctx->config.name, MAX_STREAM_NAME - 1);
        stream_name[MAX_STREAM_NAME - 1] = '\0';
    } else {
        log_error("HLS packet callback received invalid streaming context");
        return -1;
    }
    
    // Get the stream state manager
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (state) {
        // Check if the stream is in the process of being stopped or callbacks are disabled
        if (is_stream_state_stopping(state)) {
            log_debug("HLS packet callback: stream %s is stopping, skipping packet", stream_name);
            return 0; // Return success but don't process the packet
        }
        
        if (!are_stream_callbacks_enabled(state)) {
            log_debug("HLS packet callback: callbacks disabled for stream %s, skipping packet", stream_name);
            return 0; // Return success but don't process the packet
        }
    } else {
        log_warn("HLS packet callback: could not find stream state for %s", stream_name);
    }
    
    // CRITICAL FIX: Check if the streaming context is still running
    if (!streaming_ctx->running) {
        log_debug("HLS packet callback: streaming context for %s is no longer running, skipping packet", 
                 stream_name);
        return 0; // Return success but don't process the packet
    }
    
    // CRITICAL FIX: Use a static mutex for thread safety during packet processing
    static pthread_mutex_t hls_packet_mutex = PTHREAD_MUTEX_INITIALIZER;
    pthread_mutex_lock(&hls_packet_mutex);
    
    int ret = 0;
    
    // CRITICAL FIX: Validate that the stream has a valid writer
    if (!streaming_ctx->hls_writer) {
        log_error("HLS packet callback: streaming context has NULL hls_writer for stream %s", 
                 stream_name);
        pthread_mutex_unlock(&hls_packet_mutex);
        return 0; // Return success but don't process the packet
    }
    
    // Check if this is a key frame
    bool is_key_frame = (pkt->flags & AV_PKT_FLAG_KEY) != 0;

    // CRITICAL FIX: Validate that process_video_packet function exists
    if (!process_video_packet) {
        log_error("HLS packet callback: process_video_packet function is NULL for stream %s", 
                 stream_name);
        pthread_mutex_unlock(&hls_packet_mutex);
        return -1;
    }
    
    // CRITICAL FIX: Make a local copy of the HLS writer to avoid race conditions
    hls_writer_t *local_hls_writer = streaming_ctx->hls_writer;
    
    // CRITICAL FIX: Additional validation of the HLS writer
    if (!local_hls_writer || !local_hls_writer->output_ctx) {
        log_error("HLS packet callback: invalid HLS writer for stream %s", stream_name);
        pthread_mutex_unlock(&hls_packet_mutex);
        return -1;
    }
    
    // Process all frames for better quality
    ret = process_video_packet(pkt, stream, local_hls_writer, 0, stream_name);
    
    // Only log errors for key frames to reduce log spam
    if (ret < 0 && is_key_frame) {
        log_error("Failed to write keyframe to HLS for stream %s: %d", stream_name, ret);
    }
    
    pthread_mutex_unlock(&hls_packet_mutex);
    return ret;
}

/**
 * HLS streaming thread function for a single stream
 */
void *hls_stream_thread(void *arg) {
    hls_stream_ctx_t *ctx = (hls_stream_ctx_t *)arg;
    int ret;
    time_t start_time = time(NULL);  // Record when we started
    stream_reader_ctx_t *reader_ctx = NULL;

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
    // Using a default of 4 seconds if not specified in config
    // Increased from 2 to 4 seconds for better compatibility with low-powered devices
    int segment_duration = ctx->config.segment_duration > 0 ?
                          ctx->config.segment_duration : 4;

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

    // Always use a dedicated stream reader for HLS streaming
    // This ensures that HLS streaming doesn't interfere with detection or recording
    log_info("Starting new dedicated stream reader for HLS stream %s", stream_name);
    reader_ctx = start_stream_reader(stream_name, 1, NULL, NULL); // 1 for dedicated stream reader
    if (!reader_ctx) {
        log_error("Failed to start dedicated stream reader for %s", stream_name);
        
        if (ctx->hls_writer) {
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }
        
        ctx->running = 0;
        return NULL;
    }
    log_info("Successfully started new dedicated stream reader for HLS stream %s", stream_name);
    
    // CRITICAL FIX: Check if we're still running after stream reader creation
    if (!ctx->running) {
        log_info("HLS streaming thread for %s stopping after stream reader creation", stream_name);
        
        if (ctx->hls_writer) {
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }
        
        if (reader_ctx) {
            stop_stream_reader(reader_ctx);
        }
        
        return NULL;
    }
    
    // Store the reader context first
    ctx->reader_ctx = reader_ctx;
    
    // Now set our callback - doing this separately ensures we don't have a race condition
    // where the callback might be called before ctx->reader_ctx is set
    if (set_packet_callback(reader_ctx, hls_packet_callback, ctx) != 0) {
        log_error("Failed to set packet callback for stream %s", stream_name);
        
        if (ctx->hls_writer) {
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }
        
        // Don't stop the reader if we didn't create it
        if (reader_ctx->dedicated) {
            stop_stream_reader(reader_ctx);
        }
        
        ctx->running = 0;
        return NULL;
    }
    
    log_info("Set packet callback for HLS stream %s", stream_name);

    // Main loop to monitor stream status
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
        
        // Sleep to avoid busy waiting - reduced from 100ms to 50ms for more responsive handling
        av_usleep(50000);  // 50ms
    }
    
    // CRITICAL FIX: Use a mutex to safely clear the callback
    static pthread_mutex_t callback_mutex = PTHREAD_MUTEX_INITIALIZER;
    pthread_mutex_lock(&callback_mutex);
    
    // Remove our callback from the reader but don't stop it here
    // Let the cleanup function handle stopping the stream reader to avoid double-free
    if (ctx->reader_ctx) {
        set_packet_callback(ctx->reader_ctx, NULL, NULL);
        log_info("Cleared packet callback for HLS stream %s on thread exit", stream_name);
    }
    
    pthread_mutex_unlock(&callback_mutex);

    // When done, close writer
    if (ctx->hls_writer) {
        hls_writer_close(ctx->hls_writer);
        ctx->hls_writer = NULL;
    }

    log_info("HLS streaming thread for stream %s exited", stream_name);
    return NULL;
}
