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
    
    // CRITICAL FIX: Add extra validation for context
    if (!ctx) {
        log_error("NULL context passed to stream reader thread");
        return NULL;
    }
    
    // CRITICAL FIX: Create a local copy of the stream name for thread safety
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, ctx->config.name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';
    
    log_info("Starting stream reader thread for stream %s (dedicated: %d)", 
             stream_name, ctx->dedicated);
    
    // CRITICAL FIX: Check if we're still running before proceeding
    if (!ctx->running) {
        log_warn("Stream reader thread for %s started but already marked as not running", stream_name);
        return NULL;
    }
    
    // Open input stream with retry logic
    int retry_count = 0;
    const int max_retries = 5;
    
    while (retry_count < max_retries && ctx->running) {  // CRITICAL FIX: Check running state in loop condition
        ret = open_input_stream(&ctx->input_ctx, ctx->config.url, ctx->config.protocol);
        if (ret == 0) {
            // Successfully opened the stream
            
            // Set the UDP flag in the timestamp tracker based on the protocol
            // This ensures proper timestamp handling for UDP streams
            set_timestamp_tracker_udp_flag(stream_name, 
                                          ctx->config.protocol == STREAM_PROTOCOL_UDP);
            
            log_info("Set UDP flag to %s for stream %s timestamp tracker", 
                    ctx->config.protocol == STREAM_PROTOCOL_UDP ? "true" : "false", 
                    stream_name);
            
            break;
        }
        
        log_error("Failed to open input stream for %s (attempt %d/%d)", 
                 stream_name, retry_count + 1, max_retries);
        
        // CRITICAL FIX: Check if we're still running before sleeping
        if (!ctx->running) {
            log_info("Stream reader thread for %s stopping during connection attempts", stream_name);
            return NULL;
        }
        
        // Wait before retrying - reduced from 1s to 250ms for more responsive handling
        av_usleep(250000);  // 250ms delay
        retry_count++;
    }
    
    // CRITICAL FIX: Check running state again after connection attempts
    if (!ctx->running) {
        log_info("Stream reader thread for %s stopping after connection attempts", stream_name);
        if (ctx->input_ctx) {
            avformat_close_input(&ctx->input_ctx);
        }
        return NULL;
    }
    
    if (ret < 0 || !ctx->input_ctx) {
        log_error("Failed to open input stream for %s after %d attempts", 
                 stream_name, max_retries);
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
    
    // CRITICAL FIX: Initialize timestamp tracking variables
    ctx->last_pts_initialized = 0;
    ctx->last_pts = AV_NOPTS_VALUE;
    ctx->frame_duration = 0;
    
    // CRITICAL FIX: Create a mutex for thread-safe packet processing
    pthread_mutex_t packet_mutex = PTHREAD_MUTEX_INITIALIZER;
    
    // Main packet reading loop
    while (ctx->running) {
        // Check if we have a callback registered
        if (!ctx->packet_callback) {
            // No callback, sleep and check again - reduced from 25ms to 5ms for more responsive handling
            av_usleep(5000);  // 5ms
            continue;
        }
        
        // CRITICAL FIX: Check if we're still running before reading a frame
        // This prevents race conditions during shutdown
        if (!ctx->running) {
            av_usleep(5000);  // 5ms
            continue;
        }
        
        // CRITICAL FIX: Use a local copy of the input context to avoid race conditions
        AVFormatContext *local_input_ctx = ctx->input_ctx;
        if (!local_input_ctx) {
            av_usleep(5000);  // 5ms
            continue;
        }
        
        // CRITICAL FIX: Lock mutex before reading frame to prevent concurrent access
        pthread_mutex_lock(&packet_mutex);
        
        // CRITICAL FIX: Check running state again after acquiring lock
        if (!ctx->running) {
            pthread_mutex_unlock(&packet_mutex);
            av_usleep(5000);  // 5ms
            continue;
        }
        
        ret = av_read_frame(local_input_ctx, pkt);
        
        // Handle timestamp recovery for UDP streams - AFTER reading the packet
        if (ret >= 0) {
            // CRITICAL FIX: Check if we're still running after reading a frame
            // This prevents race conditions during shutdown
            if (!ctx->running) {
                av_packet_unref(pkt);
                pthread_mutex_unlock(&packet_mutex);
                continue;
            }
            
            // Log for debugging
            log_debug("Processing packet for stream %s (protocol: %s): pts=%lld, dts=%lld, size=%d", 
                 stream_name, 
                 ctx->config.protocol == STREAM_PROTOCOL_UDP ? "UDP" : "TCP",
                 (long long)(pkt->pts != AV_NOPTS_VALUE ? pkt->pts : -1), 
                 (long long)(pkt->dts != AV_NOPTS_VALUE ? pkt->dts : -1), 
                 pkt->size);
            
            // CRITICAL FIX: We're already holding packet_mutex, no need for additional mutex
            // This simplifies the code and reduces the risk of deadlocks
            if (ctx->config.protocol == STREAM_PROTOCOL_UDP) {
                
                // For all streams, but especially UDP streams, ensure timestamps are valid
                // Use per-context variables to avoid issues with multiple streams
                if (!ctx->last_pts_initialized) {
                    ctx->last_pts = 0;
                    ctx->frame_duration = 0;
                    ctx->last_pts_initialized = 1;
                    log_info("Initialized timestamp tracking for stream %s (protocol: UDP)", 
                            stream_name);
                }
                
                // CRITICAL FIX: Add additional validation for local_input_ctx and stream index
                if (ctx->frame_duration == 0 && local_input_ctx && 
                    ctx->video_stream_idx >= 0 && ctx->video_stream_idx < local_input_ctx->nb_streams &&
                    local_input_ctx->streams[ctx->video_stream_idx]) {
                    
                    // Add null check for avg_frame_rate
                    if (local_input_ctx->streams[ctx->video_stream_idx]->avg_frame_rate.num > 0 &&
                        local_input_ctx->streams[ctx->video_stream_idx]->avg_frame_rate.den > 0) {
                        
                        AVRational tb = local_input_ctx->streams[ctx->video_stream_idx]->time_base;
                        AVRational fr = local_input_ctx->streams[ctx->video_stream_idx]->avg_frame_rate;
                        
                        // Avoid division by zero
                        if (fr.den > 0 && tb.num > 0 && tb.den > 0) {  // CRITICAL FIX: Validate timebase
                            ctx->frame_duration = av_rescale_q(1, av_inv_q(fr), tb);
                        } else {
                            // Default to a reasonable value if framerate or timebase is invalid
                            ctx->frame_duration = 3000; // Assume 30fps with timebase 1/90000
                        }
                        
                        log_debug("Calculated frame duration for stream %s: %lld", 
                             stream_name, (long long)ctx->frame_duration);
                    } else {
                        // Default to a reasonable value if framerate is invalid
                        ctx->frame_duration = 3000; // Assume 30fps with timebase 1/90000
                        log_debug("Using default frame duration for stream %s (invalid framerate): %lld", 
                             stream_name, (long long)ctx->frame_duration);
                    }
                } else if (ctx->frame_duration == 0) {
                    // Default to a reasonable value if we can't calculate
                    ctx->frame_duration = 3000; // Assume 30fps with timebase 1/90000
                    log_debug("Using default frame duration for stream %s: %lld", 
                         stream_name, (long long)ctx->frame_duration);
                }
                
                // Handle missing timestamps for all streams, but especially important for UDP
                if (pkt->pts == AV_NOPTS_VALUE && pkt->dts != AV_NOPTS_VALUE) {
                    pkt->pts = pkt->dts;
                    log_debug("Using DTS as PTS for stream %s: pts=%lld", 
                         stream_name, (long long)pkt->pts);
                } else if (pkt->dts == AV_NOPTS_VALUE && pkt->pts != AV_NOPTS_VALUE) {
                    pkt->dts = pkt->pts;
                    log_debug("Using PTS as DTS for stream %s: dts=%lld", 
                         stream_name, (long long)pkt->dts);
                } else if (pkt->pts == AV_NOPTS_VALUE && pkt->dts == AV_NOPTS_VALUE) {
                    // Both timestamps missing, generate based on previous packet
                    if (ctx->last_pts > 0) {
                        pkt->pts = ctx->last_pts + ctx->frame_duration;
                        pkt->dts = pkt->pts;
                        log_debug("Generated timestamps for stream %s: pts=%lld, dts=%lld", 
                             stream_name, (long long)pkt->pts, (long long)pkt->dts);
                    } else {
                        // First packet with no timestamp, use a default
                        pkt->pts = 1;  // Use 1 instead of frame_duration for safer initialization
                        pkt->dts = pkt->pts;
                        log_debug("Generated initial timestamps for stream %s: pts=%lld, dts=%lld", 
                             stream_name, (long long)pkt->pts, (long long)pkt->dts);
                    }
                }
                
                // Additional safety check: ensure timestamps are positive
                if (pkt->pts <= 0 || pkt->dts <= 0) {
                    log_warn("Non-positive timestamps detected in stream %s: pts=%lld, dts=%lld", 
                            stream_name, (long long)pkt->pts, (long long)pkt->dts);
                    
                    // Set to safe values
                    if (pkt->pts <= 0) {
                        if (pkt->dts > 0) {
                            pkt->pts = pkt->dts;
                        } else {
                            pkt->pts = 1;
                        }
                    }
                    
                    if (pkt->dts <= 0) {
                        if (pkt->pts > 0) {
                            pkt->dts = pkt->pts;
                        } else {
                            pkt->dts = 1;
                        }
                    }
                    
                    log_debug("Corrected non-positive timestamps for stream %s: pts=%lld, dts=%lld", 
                             stream_name, (long long)pkt->pts, (long long)pkt->dts);
                }
                
                // Store current timestamp for next packet if valid
                if (pkt->pts != AV_NOPTS_VALUE) {
                    ctx->last_pts = pkt->pts;
                }
            } else {
                // For TCP streams, we can handle timestamps without a separate mutex
                // since they're more reliable and don't need as much correction
                
                // For all streams, ensure timestamps are valid
                if (!ctx->last_pts_initialized) {
                    ctx->last_pts = 0;
                    ctx->frame_duration = 0;
                    ctx->last_pts_initialized = 1;
                    log_info("Initialized timestamp tracking for stream %s (protocol: TCP)", 
                            stream_name);
                }
                
                // Handle missing timestamps
                if (pkt->pts == AV_NOPTS_VALUE && pkt->dts != AV_NOPTS_VALUE) {
                    pkt->pts = pkt->dts;
                } else if (pkt->dts == AV_NOPTS_VALUE && pkt->pts != AV_NOPTS_VALUE) {
                    pkt->dts = pkt->pts;
                } else if (pkt->pts == AV_NOPTS_VALUE && pkt->dts == AV_NOPTS_VALUE) {
                    // Both timestamps missing, use a default
                    pkt->pts = 1;
                    pkt->dts = 1;
                }
                
                // CRITICAL FIX: Additional safety check for negative timestamps
                if (pkt->pts <= 0 || pkt->dts <= 0) {
                    log_warn("Non-positive timestamps detected in TCP stream %s: pts=%lld, dts=%lld", 
                            stream_name, (long long)pkt->pts, (long long)pkt->dts);
                    
                    if (pkt->pts <= 0) pkt->pts = 1;
                    if (pkt->dts <= 0) pkt->dts = 1;
                }
                
                // Store current timestamp for next packet if valid
                if (pkt->pts != AV_NOPTS_VALUE) {
                    ctx->last_pts = pkt->pts;
                }
            }
        }
        
        if (ret < 0) {
            if (ret == AVERROR_EOF || ret == AVERROR(EAGAIN)) {
                // End of stream or resource temporarily unavailable
                av_packet_unref(pkt);
                pthread_mutex_unlock(&packet_mutex);  // CRITICAL FIX: Unlock mutex before reconnection logic
                log_warn("Stream %s disconnected, attempting to reconnect...", stream_name);
                
                // CRITICAL FI
                
                // Use different reconnection strategies based on protocol
                static int reconnect_attempts = 0;
                int backoff_time_ms;
                
                // Check if this is an ONVIF stream
                bool is_onvif = strstr(ctx->config.url, "onvif") != NULL;
                
                if (is_onvif) {
                    // For ONVIF streams, use a more aggressive reconnection strategy
                    // with shorter initial delays but longer maximum delays
                    if (reconnect_attempts < 3) {
                        // Start with shorter delays for quick recovery
                        backoff_time_ms = 200 * (reconnect_attempts + 1);
                    } else {
                        // Then use exponential backoff with a higher cap
                        backoff_time_ms = 500 * (1 << ((reconnect_attempts - 3) > 4 ? 4 : (reconnect_attempts - 3)));
                        
                        // Cap the backoff time at 6 seconds
                        if (backoff_time_ms > 6000) {
                            backoff_time_ms = 6000;
                        }
                    }
                    
                    log_info("ONVIF reconnection attempt %d for %s, waiting %d ms", 
                            reconnect_attempts, ctx->config.name, backoff_time_ms);
                } else if (ctx->config.protocol == STREAM_PROTOCOL_UDP) {
                    // For UDP streams, use a gentler fixed delay strategy
                    // UDP streams often have transient issues that resolve quickly
                    backoff_time_ms = 500; // Fixed 500ms delay for UDP
                    
                    log_info("UDP reconnection attempt %d for %s, using fixed delay of %d ms", 
                            reconnect_attempts, ctx->config.name, backoff_time_ms);
                    
                    // For UDP, we'll try a more conservative approach first:
                    // Instead of immediately closing and reopening, try reading again after a delay
                    if (reconnect_attempts < 3) {
                        reconnect_attempts++;
                        av_usleep(backoff_time_ms * 1000);  // Convert ms to μs
                        continue; // Try reading again without closing/reopening
                    }
                    
                    // After a few attempts with the conservative approach, try a full reconnect
                    log_info("Conservative UDP reconnection failed for %s, trying full reconnect", 
                            ctx->config.name);
                } else {
                    // For TCP streams, use the existing exponential backoff strategy
                    backoff_time_ms = 250 * (1 << (reconnect_attempts > 5 ? 5 : reconnect_attempts));
                    
                    // Cap the backoff time at 4 seconds (reduced from 8 seconds)
                    if (backoff_time_ms > 4000) {
                        backoff_time_ms = 4000;
                    }
                    
                    log_info("TCP reconnection attempt %d for %s, waiting %d ms", 
                            reconnect_attempts, ctx->config.name, backoff_time_ms);
                }
                
                reconnect_attempts++;
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
                
                // Set the UDP flag in the timestamp tracker based on the protocol
                // This ensures proper timestamp handling for UDP streams after reconnection
                set_timestamp_tracker_udp_flag(ctx->config.name, 
                                             ctx->config.protocol == STREAM_PROTOCOL_UDP);
                
                log_info("Reset UDP flag to %s for stream %s timestamp tracker after reconnection", 
                        ctx->config.protocol == STREAM_PROTOCOL_UDP ? "true" : "false", 
                        ctx->config.name);
                
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
            
            // CRITICAL FIX: Double-check that we're still running and have a valid callback
            // This prevents use-after-free issues during shutdown
            if (!ctx->running || !ctx->packet_callback) {
                // Skip processing if we're shutting down or callback was cleared
                continue;
            }
            
            // Make a local copy of the callback to avoid race conditions
            packet_callback_t callback = ctx->packet_callback;
            void *callback_data = ctx->callback_data;
            
            // Final check before calling the callback
            if (callback) {
                ret = callback(pkt, ctx->input_ctx->streams[ctx->video_stream_idx], callback_data);
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
            
            // Copy the stream name and thread for later use
            char stream_name[MAX_STREAM_NAME];
            strncpy(stream_name, reader_contexts[i]->config.name,
                    MAX_STREAM_NAME - 1);
            stream_name[MAX_STREAM_NAME - 1] = '\0';
            
            pthread_t thread_to_join = reader_contexts[i]->thread;
            
            // Mark as not running
            reader_contexts[i]->running = 0;
            
            // Clear the callback to prevent any further processing
            reader_contexts[i]->packet_callback = NULL;
            reader_contexts[i]->callback_data = NULL;
            
            // Unlock before joining thread to prevent deadlocks
            pthread_mutex_unlock(&contexts_mutex);
            
            // Try to join with a timeout
            log_info("Waiting for stream reader thread for %s to exit", stream_name);
            int join_result = pthread_join_with_timeout(thread_to_join, NULL, 3);
            if (join_result != 0) {
                log_warn("Could not join stream reader thread for %s within timeout: %s",
                        stream_name, strerror(join_result));
            } else {
                log_info("Successfully joined stream reader thread for %s", stream_name);
            }
            
            // Re-lock for cleanup
            pthread_mutex_lock(&contexts_mutex);
            
            // Check if the context is still valid
            int found = 0;
            for (int j = 0; j < MAX_STREAMS; j++) {
                if (reader_contexts[j] && strcmp(reader_contexts[j]->config.name, stream_name) == 0) {
                    // Close input context if it exists
                    if (reader_contexts[j]->input_ctx) {
                        avformat_close_input(&reader_contexts[j]->input_ctx);
                    }
                    
                    // Free context
                    free(reader_contexts[j]);
                    reader_contexts[j] = NULL;
                    found = 1;
                    break;
                }
            }
            
            if (!found) {
                log_warn("Stream reader context for %s was already cleaned up", stream_name);
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
    
    // Make a local copy of the stream name for logging after ctx is freed
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, ctx->config.name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';
    
    // Log that we're attempting to stop the reader
    log_info("Attempting to stop stream reader: %s", stream_name);
    
    // CRITICAL FIX: Use a static mutex to prevent concurrent access during stopping
    static pthread_mutex_t stop_mutex = PTHREAD_MUTEX_INITIALIZER;
    pthread_mutex_lock(&stop_mutex);
    
    // CRITICAL FIX: Check if the reader is already stopped
    if (!ctx->running) {
        log_warn("Stream reader for %s is already stopped", stream_name);
        pthread_mutex_unlock(&stop_mutex);
        return 0;
    }
    
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
        pthread_mutex_unlock(&stop_mutex);
        log_warn("Stream reader context not found in array for %s", stream_name);
        return -1;
    }
    
    // CRITICAL FIX: First safely clear the callback to prevent any further processing
    // This must be done before marking as not running to prevent race conditions
    ctx->packet_callback = NULL;
    ctx->callback_data = NULL;
    
    // Now mark as not running
    ctx->running = 0;
    
    // Store a local copy of the thread to join
    pthread_t thread_to_join = ctx->thread;
    
    // Close input context if it exists - this will force any blocking read operations to return
    if (ctx->input_ctx) {
        avformat_close_input(&ctx->input_ctx);
        ctx->input_ctx = NULL; // Set to NULL to prevent double-free or use-after-free
    }
    
    // Create a local copy of the context pointer before removing it from the array
    stream_reader_ctx_t *ctx_copy = ctx;
    
    // Remove the context from the array before unlocking to prevent other threads from accessing it
    reader_contexts[index] = NULL;
    
    // Unlock before joining thread to prevent deadlocks
    pthread_mutex_unlock(&contexts_mutex);
    
    // Try to join with a timeout
    int join_result = pthread_join_with_timeout(thread_to_join, NULL, 5);
    if (join_result != 0) {
        log_warn("Could not join thread for stream %s within timeout (error: %s)", 
                stream_name, strerror(join_result));
        
        // Even if we couldn't join the thread, we still need to free the context
        // This is safe because we've already removed it from the array and cleared the callback
        log_warn("Freeing context for stream %s despite join failure", stream_name);
    } else {
        log_info("Successfully joined thread for stream %s", stream_name);
    }
    
    // Now it's safe to free the context since we've removed it from the array
    // and either joined the thread or at least tried to
    free(ctx_copy);
    
    log_info("Successfully stopped stream reader for %s", stream_name);
    
    // CRITICAL FIX: Unlock the stop mutex
    pthread_mutex_unlock(&stop_mutex);
    
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
    
    // Allow NULL callback for clearing during shutdown
    if (!callback) {
        log_info("Clearing packet callback for stream %s", ctx->config.name);
        ctx->packet_callback = NULL;
        ctx->callback_data = NULL;
        return 0;
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

/**
 * Get stream reader by index
 */
stream_reader_ctx_t *get_stream_reader_by_index(int index) {
    if (index < 0 || index >= MAX_STREAMS) {
        return NULL;
    }
    
    pthread_mutex_lock(&contexts_mutex);
    stream_reader_ctx_t *reader = reader_contexts[index];
    pthread_mutex_unlock(&contexts_mutex);
    
    return reader;
}
