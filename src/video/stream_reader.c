#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <sys/time.h>
#include <fcntl.h>
#include <errno.h>
#include <time.h>

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/time.h>

#include "core/logger.h"
#include "core/config.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/stream_reader.h"
#include "video/stream_transcoding.h"

// Hash map for tracking running stream reader contexts
static stream_reader_ctx_t *reader_contexts[MAX_STREAMS];
static pthread_mutex_t contexts_mutex = PTHREAD_MUTEX_INITIALIZER;

// Forward declarations
static void *stream_reader_thread(void *arg);

/**
 * Stream reader thread function
 */
static void *stream_reader_thread(void *arg) {
    stream_reader_ctx_t *ctx = (stream_reader_ctx_t *)arg;
    AVPacket *pkt = NULL;
    int ret;
    
    log_info("Starting stream reader thread for stream %s (dedicated: %d)", 
             ctx->config.name, ctx->dedicated);
    
    // Open input stream with retry logic
    int retry_count = 0;
    const int max_retries = 5;
    
    while (retry_count < max_retries) {
        ret = open_input_stream(&ctx->input_ctx, ctx->config.url, ctx->config.protocol);
        if (ret == 0) {
            // Successfully opened the stream
            break;
        }
        
        log_error("Failed to open input stream for %s (attempt %d/%d)", 
                 ctx->config.name, retry_count + 1, max_retries);
        
        // Wait before retrying - reduced from 1s to 250ms for more responsive handling
        av_usleep(250000);  // 250ms delay
        retry_count++;
    }
    
    if (ret < 0 || !ctx->input_ctx) {
        log_error("Failed to open input stream for %s after %d attempts", 
                 ctx->config.name, max_retries);
        ctx->running = 0;
        return NULL;
    }
    
    // Find video stream
    ctx->video_stream_idx = find_video_stream_index(ctx->input_ctx);
    if (ctx->video_stream_idx == -1) {
        log_error("No video stream found in %s", ctx->config.url);
        avformat_close_input(&ctx->input_ctx);
        ctx->running = 0;
        return NULL;
    }
    
    // Initialize packet
    pkt = av_packet_alloc();
    if (!pkt) {
        log_error("Failed to allocate packet");
        avformat_close_input(&ctx->input_ctx);
        ctx->running = 0;
        return NULL;
    }
    
    // Main packet reading loop
    while (ctx->running) {
        // Check if we have a callback registered
        if (!ctx->packet_callback) {
            // No callback, sleep and check again - reduced from 25ms to 5ms for more responsive handling
            av_usleep(5000);  // 5ms
            continue;
        }
        
        ret = av_read_frame(ctx->input_ctx, pkt);
        
        if (ret < 0) {
            if (ret == AVERROR_EOF || ret == AVERROR(EAGAIN)) {
                // End of stream or resource temporarily unavailable
                // Try to reconnect with exponential backoff
                av_packet_unref(pkt);
                log_warn("Stream %s disconnected, attempting to reconnect...", ctx->config.name);
                
                // Implement exponential backoff for reconnection attempts with reduced delays
                static int reconnect_attempts = 0;
                int backoff_time_ms = 250 * (1 << (reconnect_attempts > 5 ? 5 : reconnect_attempts));
                reconnect_attempts++;
                
                // Cap the backoff time at 4 seconds (reduced from 8 seconds)
                if (backoff_time_ms > 4000) {
                    backoff_time_ms = 4000;
                }
                
                log_info("Reconnection attempt %d for %s, waiting %d ms", 
                        reconnect_attempts, ctx->config.name, backoff_time_ms);
                
                av_usleep(backoff_time_ms * 1000);  // Convert ms to μs
                
                // Close and reopen input
                avformat_close_input(&ctx->input_ctx);
                
                ret = open_input_stream(&ctx->input_ctx, ctx->config.url, ctx->config.protocol);
                if (ret < 0) {
                    log_error("Could not reconnect to input stream for %s", ctx->config.name);
                    continue;  // Keep trying
                }
                
                // Reset reconnection attempts on successful reconnection
                reconnect_attempts = 0;
                
                // Find video stream again
                ctx->video_stream_idx = find_video_stream_index(ctx->input_ctx);
                if (ctx->video_stream_idx == -1) {
                    log_error("No video stream found in %s after reconnect", ctx->config.url);
                    continue;  // Keep trying
                }
                
                continue;
            } else {
                log_ffmpeg_error(ret, "Error reading frame");
                break;
            }
        }
        
        // Process video packets directly
        if (pkt->stream_index == ctx->video_stream_idx) {
            // Check if this is a key frame (for logging)
            int is_key_frame = (pkt->flags & AV_PKT_FLAG_KEY) ? 1 : 0;
            
            if (is_key_frame) {
                log_debug("Processing keyframe: pts=%lld, dts=%lld, size=%d",
                         (long long)pkt->pts, (long long)pkt->dts, pkt->size);
            }
            
            // Removed packet throttling mechanism to improve quality
            // Always process all frames for better quality
            
            // Call the callback function with the packet
            if (ctx->packet_callback) {
                ret = ctx->packet_callback(pkt, ctx->input_ctx->streams[ctx->video_stream_idx], ctx->callback_data);
                if (ret < 0) {
                    log_error("Packet callback failed for stream %s: %d", ctx->config.name, ret);
                    // Continue anyway
                }
            }
        }
        
        av_packet_unref(pkt);
    }
    
    // Cleanup resources
    if (pkt) {
        av_packet_free(&pkt);
    }
    
    if (ctx->input_ctx) {
        avformat_close_input(&ctx->input_ctx);
    }
    
    log_info("Stream reader thread for stream %s exited", ctx->config.name);
    return NULL;
}

/**
 * Initialize the stream reader backend
 */
void init_stream_reader_backend(void) {
    // Initialize contexts array
    memset(reader_contexts, 0, sizeof(reader_contexts));
    
    log_info("Stream reader backend initialized");
}

/**
 * Cleanup the stream reader backend
 */
void cleanup_stream_reader_backend(void) {
    log_info("Cleaning up stream reader backend...");
    pthread_mutex_lock(&contexts_mutex);
    
    // Stop all running readers
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (reader_contexts[i]) {
            log_info("Stopping stream reader in slot %d: %s", i,
                    reader_contexts[i]->config.name);
            
            // Copy the stream name for later use
            char stream_name[MAX_STREAM_NAME];
            strncpy(stream_name, reader_contexts[i]->config.name,
                    MAX_STREAM_NAME - 1);
            stream_name[MAX_STREAM_NAME - 1] = '\0';
            
            // Mark as not running
            reader_contexts[i]->running = 0;
            
            // Attempt to join the thread with a timeout
            pthread_t thread = reader_contexts[i]->thread;
            pthread_mutex_unlock(&contexts_mutex);
            
            // Try to join with a timeout
            if (pthread_join_with_timeout(thread, NULL, 2) != 0) {
                log_warn("Could not join thread for stream %s within timeout",
                        stream_name);
            }
            
            pthread_mutex_lock(&contexts_mutex);
            
            // Clean up resources
            if (reader_contexts[i]) {
                free(reader_contexts[i]);
                reader_contexts[i] = NULL;
            }
        }
    }
    
    pthread_mutex_unlock(&contexts_mutex);
    
    log_info("Stream reader backend cleaned up");
}

/**
 * Start a stream reader for a stream with a callback for packet processing
 */
stream_reader_ctx_t *start_stream_reader(const char *stream_name, int dedicated, 
                                        packet_callback_t callback, void *user_data) {
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Stream %s not found for stream reader", stream_name);
        return NULL;
    }
    
    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get config for stream %s for stream reader", stream_name);
        return NULL;
    }
    
    // For dedicated readers, we don't check if already running
    // For shared readers, we check if already running
    if (!dedicated) {
        pthread_mutex_lock(&contexts_mutex);
        for (int i = 0; i < MAX_STREAMS; i++) {
            if (reader_contexts[i] && 
                strcmp(reader_contexts[i]->config.name, stream_name) == 0 && 
                !reader_contexts[i]->dedicated) {
                
                // Update the callback if provided
                if (callback) {
                    reader_contexts[i]->packet_callback = callback;
                    reader_contexts[i]->callback_data = user_data;
                    log_info("Updated callback for existing stream reader %s", stream_name);
                }
                
                pthread_mutex_unlock(&contexts_mutex);
                log_info("Stream reader for %s already running", stream_name);
                return reader_contexts[i];  // Already running
            }
        }
        pthread_mutex_unlock(&contexts_mutex);
    }
    
    // Find empty slot
    pthread_mutex_lock(&contexts_mutex);
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (!reader_contexts[i]) {
            slot = i;
            break;
        }
    }
    
    if (slot == -1) {
        pthread_mutex_unlock(&contexts_mutex);
        log_error("No slot available for new stream reader");
        return NULL;
    }
    
    // Create context
    stream_reader_ctx_t *ctx = malloc(sizeof(stream_reader_ctx_t));
    if (!ctx) {
        pthread_mutex_unlock(&contexts_mutex);
        log_error("Memory allocation failed for stream reader context");
        return NULL;
    }
    
    memset(ctx, 0, sizeof(stream_reader_ctx_t));
    memcpy(&ctx->config, &config, sizeof(stream_config_t));
    ctx->running = 1;
    ctx->dedicated = dedicated;
    ctx->packet_callback = callback;
    ctx->callback_data = user_data;
    
    // Start reader thread
    if (pthread_create(&ctx->thread, NULL, stream_reader_thread, ctx) != 0) {
        free(ctx);
        pthread_mutex_unlock(&contexts_mutex);
        log_error("Failed to create stream reader thread for %s", stream_name);
        return NULL;
    }
    
    // Store context
    reader_contexts[slot] = ctx;
    pthread_mutex_unlock(&contexts_mutex);
    
    log_info("Started stream reader for %s in slot %d (dedicated: %d)", 
             stream_name, slot, dedicated);
    
    return ctx;
}

/**
 * Stop a stream reader
 */
int stop_stream_reader(stream_reader_ctx_t *ctx) {
    if (!ctx) {
        return -1;
    }
    
    // Log that we're attempting to stop the reader
    log_info("Attempting to stop stream reader: %s", ctx->config.name);
    
    pthread_mutex_lock(&contexts_mutex);
    
    // Find the reader context in the array
    int index = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (reader_contexts[i] == ctx) {
            index = i;
            break;
        }
    }
    
    if (index == -1) {
        pthread_mutex_unlock(&contexts_mutex);
        log_warn("Stream reader context not found in array");
        return -1;
    }
    
    // Mark as not running
    ctx->running = 0;
    
    // Attempt to join the thread with a timeout
    pthread_t thread = ctx->thread;
    pthread_mutex_unlock(&contexts_mutex);
    
    // Try to join with a timeout
    if (pthread_join_with_timeout(thread, NULL, 5) != 0) {
        log_warn("Could not join thread for stream %s within timeout",
                ctx->config.name);
    }
    
    pthread_mutex_lock(&contexts_mutex);
    
    // Clean up resources
    if (reader_contexts[index] == ctx) {
        free(ctx);
        reader_contexts[index] = NULL;
        
        log_info("Successfully stopped stream reader for %s", ctx->config.name);
    } else {
        log_warn("Stream reader context was modified during cleanup");
    }
    
    pthread_mutex_unlock(&contexts_mutex);
    
    return 0;
}

/**
 * Set or update the packet callback for a stream reader
 */
int set_packet_callback(stream_reader_ctx_t *ctx, packet_callback_t callback, void *user_data) {
    if (!ctx) {
        log_error("Cannot set callback: NULL context");
        return -1;
    }
    
    if (!callback) {
        log_error("Cannot set NULL callback");
        return -1;
    }
    
    ctx->packet_callback = callback;
    ctx->callback_data = user_data;
    
    log_info("Set packet callback for stream %s", ctx->config.name);
    return 0;
}

/**
 * Get the stream reader for a stream
 */
stream_reader_ctx_t *get_stream_reader(const char *stream_name) {
    if (!stream_name) {
        return NULL;
    }
    
    pthread_mutex_lock(&contexts_mutex);
    
    // First look for an existing dedicated reader for this stream
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (reader_contexts[i] && 
            strcmp(reader_contexts[i]->config.name, stream_name) == 0 && 
            reader_contexts[i]->dedicated) {
            pthread_mutex_unlock(&contexts_mutex);
            return reader_contexts[i];
        }
    }
    
    // If no dedicated reader exists, look for a shared reader
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (reader_contexts[i] && 
            strcmp(reader_contexts[i]->config.name, stream_name) == 0 && 
            !reader_contexts[i]->dedicated) {
            pthread_mutex_unlock(&contexts_mutex);
            return reader_contexts[i];
        }
    }
    
    // No existing reader found
    pthread_mutex_unlock(&contexts_mutex);
    return NULL;
}
