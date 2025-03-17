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
static void packet_queue_init(packet_queue_t *q);
static void packet_queue_destroy(packet_queue_t *q);
static int packet_queue_put(packet_queue_t *q, AVPacket *pkt);
static int packet_queue_get(packet_queue_t *q, AVPacket *pkt, int consumer_id, int dedicated);
static void packet_queue_abort(packet_queue_t *q);

/**
 * Initialize a packet queue
 */
static void packet_queue_init(packet_queue_t *q) {
    memset(q, 0, sizeof(packet_queue_t));
    pthread_mutex_init(&q->mutex, NULL);
    pthread_cond_init(&q->cond_not_empty, NULL);
    pthread_cond_init(&q->cond_not_full, NULL);
    q->head = 0;
    q->tail = 0;
    q->size = 0;
    q->abort_request = 0;
    q->next_consumer_id = 1;  // Start consumer IDs at 1
    
    // Initialize consumer cursors
    for (int i = 0; i < MAX_QUEUE_CONSUMERS; i++) {
        q->consumers[i].consumer_id = 0;
        q->consumers[i].cursor = 0;
        q->consumers[i].last_read_time = 0;
        q->consumers[i].active = 0;
    }
}

/**
 * Destroy a packet queue
 */
static void packet_queue_destroy(packet_queue_t *q) {
    packet_queue_abort(q);
    
    pthread_mutex_lock(&q->mutex);
    
    // Clean up any remaining packets
    while (q->size > 0) {
        broadcast_packet_t *packet = &q->packets[q->head];
        q->head = (q->head + 1) % MAX_PACKET_QUEUE_SIZE;
        q->size--;
        
        // Free the packet
        if (packet->pkt) {
            av_packet_free(&packet->pkt);
        }
    }
    
    pthread_mutex_unlock(&q->mutex);
    
    pthread_mutex_destroy(&q->mutex);
    pthread_cond_destroy(&q->cond_not_empty);
    pthread_cond_destroy(&q->cond_not_full);
}

/**
 * Put a packet in the queue (non-blocking)
 */
static int packet_queue_put(packet_queue_t *q, AVPacket *pkt) {
    // Quick check without locking first
    int has_active_consumers = 0;
    for (int i = 0; i < MAX_QUEUE_CONSUMERS; i++) {
        if (q->consumers[i].active) {
            has_active_consumers = 1;
            break;
        }
    }
    
    if (!has_active_consumers) {
        return 0;  // No consumers, don't bother queueing
    }
    
    pthread_mutex_lock(&q->mutex);
    
    // Double-check after locking
    has_active_consumers = 0;
    for (int i = 0; i < MAX_QUEUE_CONSUMERS; i++) {
        if (q->consumers[i].active) {
            has_active_consumers = 1;
            break;
        }
    }
    
    if (!has_active_consumers) {
        pthread_mutex_unlock(&q->mutex);
        return 0;  // No consumers, don't bother queueing
    }
    
    // Remove old packets based on retention time
    time_t current_time = time(NULL);
    while (q->size > 0) {
        broadcast_packet_t *oldest = &q->packets[q->head];
        
        // Check if the packet is older than the retention time
        if (current_time - oldest->arrival_time > MAX_PACKET_RETENTION_TIME) {
            // Free the packet
            if (oldest->pkt) {
                av_packet_free(&oldest->pkt);
            }
            
            // Move head pointer
            q->head = (q->head + 1) % MAX_PACKET_QUEUE_SIZE;
            q->size--;
            
            // Update consumer cursors that point to the removed packet
            for (int i = 0; i < MAX_QUEUE_CONSUMERS; i++) {
                if (q->consumers[i].active && q->consumers[i].cursor == q->head) {
                    // Move cursor to next packet
                    q->consumers[i].cursor = (q->consumers[i].cursor + 1) % MAX_PACKET_QUEUE_SIZE;
                }
            }
        } else {
            // If the oldest packet is still within retention time, stop removing
            break;
        }
    }
    
    // If queue is getting full, be more aggressive about dropping non-keyframes
    if (q->size >= MAX_PACKET_QUEUE_SIZE * 0.8) { // At 80% capacity
        int packets_to_remove = MAX_PACKET_QUEUE_SIZE / 5; // Remove ~20% of packets
        int removed = 0;
        
        // First pass: try to remove non-keyframe packets
        for (int i = 0; i < packets_to_remove && q->size > 0; i++) {
            broadcast_packet_t *oldest = &q->packets[q->head];
            
            // Only remove non-keyframe packets in first pass
            if (!oldest->key_frame) {
                // Free the packet
                if (oldest->pkt) {
                    av_packet_free(&oldest->pkt);
                }
                
                // Move head pointer
                q->head = (q->head + 1) % MAX_PACKET_QUEUE_SIZE;
                q->size--;
                removed++;
                
                // Update consumer cursors that point to the removed packet
                for (int j = 0; j < MAX_QUEUE_CONSUMERS; j++) {
                    if (q->consumers[j].active && q->consumers[j].cursor == q->head) {
                        // Move cursor to next packet
                        q->consumers[j].cursor = (q->consumers[j].cursor + 1) % MAX_PACKET_QUEUE_SIZE;
                    }
                }
            } else {
                // If it's a keyframe, skip it in first pass
                break;
            }
        }
        
        // If we still need to make room and queue is still very full (>90%), remove keyframes too
        if (q->size >= MAX_PACKET_QUEUE_SIZE * 0.9) {
            log_warn("Packet queue critically full, dropping oldest packets including keyframes");
            
            // Remove oldest packets regardless of keyframe status
            while (q->size >= MAX_PACKET_QUEUE_SIZE - packets_to_remove + removed) {
                broadcast_packet_t *oldest = &q->packets[q->head];
                
                // Free the packet
                if (oldest->pkt) {
                    av_packet_free(&oldest->pkt);
                }
                
                // Move head pointer
                q->head = (q->head + 1) % MAX_PACKET_QUEUE_SIZE;
                q->size--;
                
                // Update consumer cursors that point to the removed packet
                for (int j = 0; j < MAX_QUEUE_CONSUMERS; j++) {
                    if (q->consumers[j].active && q->consumers[j].cursor == q->head) {
                        // Move cursor to next packet
                        q->consumers[j].cursor = (q->consumers[j].cursor + 1) % MAX_PACKET_QUEUE_SIZE;
                    }
                }
            }
        }
        
        // Signal that the queue is not full
        pthread_cond_signal(&q->cond_not_full);
    }
    
    if (q->abort_request) {
        pthread_mutex_unlock(&q->mutex);
        return -1;
    }
    
    // Check if this is a keyframe
    int is_key_frame = (pkt->flags & AV_PKT_FLAG_KEY) ? 1 : 0;
    
    // If queue is still nearly full and this is not a keyframe, consider dropping it
    if (q->size >= MAX_PACKET_QUEUE_SIZE * 0.9 && !is_key_frame) {
        // Drop this non-keyframe to make room for more important frames
        pthread_mutex_unlock(&q->mutex);
        return 0; // Return success but don't actually queue it
    }
    
    // Initialize a new broadcast packet entry
    broadcast_packet_t *new_packet = &q->packets[q->tail];
    memset(new_packet, 0, sizeof(broadcast_packet_t));
    
    // Allocate and copy the packet
    new_packet->pkt = av_packet_alloc();
    if (!new_packet->pkt) {
        pthread_mutex_unlock(&q->mutex);
        log_error("Failed to allocate packet in queue");
        return -1;
    }
    
    if (av_packet_ref(new_packet->pkt, pkt) < 0) {
        av_packet_free(&new_packet->pkt);
        pthread_mutex_unlock(&q->mutex);
        log_error("Failed to reference packet in queue");
        return -1;
    }
    
    // Store packet metadata
    new_packet->pts = pkt->pts;
    new_packet->key_frame = is_key_frame;
    new_packet->stream_index = pkt->stream_index;
    new_packet->size = pkt->size;
    new_packet->arrival_time = time(NULL);
    
    // Add to queue
    q->tail = (q->tail + 1) % MAX_PACKET_QUEUE_SIZE;
    q->size++;
    
    pthread_cond_signal(&q->cond_not_empty);
    pthread_mutex_unlock(&q->mutex);
    
    return 0;
}

/**
 * Find a consumer by ID
 * Returns the index in the consumers array, or -1 if not found
 * Assumes the mutex is already locked
 */
static int find_consumer(packet_queue_t *q, int consumer_id) {
    if (consumer_id <= 0) {
        return -1;
    }
    
    for (int i = 0; i < MAX_QUEUE_CONSUMERS; i++) {
        if (q->consumers[i].consumer_id == consumer_id && q->consumers[i].active) {
            return i;
        }
    }
    
    return -1;
}

/**
 * Get a packet from the queue (with timeout)
 */
static int packet_queue_get(packet_queue_t *q, AVPacket *pkt, int consumer_id, int dedicated) {
    struct timespec timeout;
    
    // Use the fallback CLOCK_REALTIME definition if needed
    #ifdef CLOCK_REALTIME
    clock_gettime(CLOCK_REALTIME, &timeout);
    #else
    // Fallback to time() + gettimeofday() if CLOCK_REALTIME is not available
    struct timeval tv;
    gettimeofday(&tv, NULL);
    timeout.tv_sec = tv.tv_sec;
    timeout.tv_nsec = tv.tv_usec * 1000;
    #endif
    
    timeout.tv_nsec += 100000000; // 100ms timeout
    if (timeout.tv_nsec >= 1000000000) {
        timeout.tv_sec++;
        timeout.tv_nsec -= 1000000000;
    }
    
    pthread_mutex_lock(&q->mutex);
    
    if (consumer_id <= 0) {
        log_error("Invalid consumer ID: %d", consumer_id);
        pthread_mutex_unlock(&q->mutex);
        return -1;
    }
    
    // Find the consumer in our array
    int consumer_idx = find_consumer(q, consumer_id);
    if (consumer_idx < 0) {
        log_error("Consumer %d not found or not active", consumer_id);
        pthread_mutex_unlock(&q->mutex);
        return -1;
    }
    
    // Update last read time
    q->consumers[consumer_idx].last_read_time = time(NULL);
    
    // If queue is empty, wait for new packets
    if (q->size == 0) {
        if (q->abort_request) {
            pthread_mutex_unlock(&q->mutex);
            return -1;
        }
        
        int wait_result = pthread_cond_timedwait(&q->cond_not_empty, &q->mutex, &timeout);
        
        // If we timed out, return a special code
        if (wait_result == ETIMEDOUT) {
            pthread_mutex_unlock(&q->mutex);
            return 1; // Timeout, no packet available
        }
        
        // If queue is still empty after waiting, return timeout
        if (q->size == 0) {
            pthread_mutex_unlock(&q->mutex);
            return 1; // No packet available
        }
    }
    
    // For dedicated stream readers, we don't need to reset the cursor
    // This allows them to maintain their own position in the stream
    if (!dedicated) {
        // For shared stream readers, always reset cursor to head to ensure sequential processing
        // This ensures monotonically increasing DTS values
        q->consumers[consumer_idx].cursor = q->head;
    } else {
        // For dedicated stream readers, if the cursor is at the head, keep it there
        // This ensures we don't miss any packets when starting a new dedicated reader
        if (q->consumers[consumer_idx].cursor == q->head) {
            // Already at head, no need to change
        } else if (q->consumers[consumer_idx].cursor < q->head || 
                  q->consumers[consumer_idx].cursor >= q->head + q->size) {
            // If cursor is before head or beyond the valid range, reset it to head
            q->consumers[consumer_idx].cursor = q->head;
            log_debug("Reset dedicated consumer cursor to head: consumer_id=%d", consumer_id);
        }
        // Otherwise, leave the cursor where it is
    }
    
    // Get the packet at the consumer's cursor
    int idx = q->consumers[consumer_idx].cursor;
    broadcast_packet_t *packet = &q->packets[idx];
    
    // Create a clean copy of the packet to avoid reference issues
    av_packet_unref(pkt);
    if (av_packet_ref(pkt, packet->pkt) < 0) {
        pthread_mutex_unlock(&q->mutex);
        log_error("Failed to reference packet from queue");
        return -1;
    }
    
    // Advance the consumer's cursor for next time
    q->consumers[consumer_idx].cursor = (q->consumers[consumer_idx].cursor + 1) % MAX_PACKET_QUEUE_SIZE;
    
    // Check if we need to clean up the queue
    // We'll use a time-based approach to remove old packets
    time_t current_time = time(NULL);
    
    // Remove packets that are older than the retention time
    while (q->size > 1) { // Keep at least one packet in the queue
        broadcast_packet_t *oldest = &q->packets[q->head];
        
        // Check if the packet is older than the retention time
        if (current_time - oldest->arrival_time > MAX_PACKET_RETENTION_TIME) {
            // Free the packet
            if (oldest->pkt) {
                av_packet_free(&oldest->pkt);
            }
            
            // Move head pointer
            q->head = (q->head + 1) % MAX_PACKET_QUEUE_SIZE;
            q->size--;
            
            // Update consumer cursors that point to the removed packet
            for (int i = 0; i < MAX_QUEUE_CONSUMERS; i++) {
                if (q->consumers[i].active && q->consumers[i].cursor == q->head) {
                    // Move cursor to next packet
                    q->consumers[i].cursor = (q->consumers[i].cursor + 1) % MAX_PACKET_QUEUE_SIZE;
                }
            }
            
            // Signal that the queue is not full
            pthread_cond_signal(&q->cond_not_full);
        } else {
            // If the oldest packet is still within retention time, stop removing
            break;
        }
    }
    
    pthread_mutex_unlock(&q->mutex);
    return 0;
}


/**
 * Abort packet queue operations
 */
static void packet_queue_abort(packet_queue_t *q) {
    pthread_mutex_lock(&q->mutex);
    q->abort_request = 1;
    pthread_cond_signal(&q->cond_not_empty);
    pthread_cond_signal(&q->cond_not_full);
    pthread_mutex_unlock(&q->mutex);
}

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
        
        // Wait before retrying
        av_usleep(2000000);  // 2 second delay
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
        // Check if we have any consumers
        pthread_mutex_lock(&ctx->consumers_mutex);
        int has_consumers = (ctx->consumers > 0);
        pthread_mutex_unlock(&ctx->consumers_mutex);
        
        // If no consumers, sleep and check again
        if (!has_consumers) {
            av_usleep(50000);  // 50ms
            continue;
        }
        
        ret = av_read_frame(ctx->input_ctx, pkt);
        
        if (ret < 0) {
            if (ret == AVERROR_EOF || ret == AVERROR(EAGAIN)) {
                // End of stream or resource temporarily unavailable
                // Try to reconnect after a short delay
                av_packet_unref(pkt);
                log_warn("Stream %s disconnected, attempting to reconnect...", ctx->config.name);
                
                av_usleep(2000000);  // 2 second delay
                
                // Close and reopen input
                avformat_close_input(&ctx->input_ctx);
                
                ret = open_input_stream(&ctx->input_ctx, ctx->config.url, ctx->config.protocol);
                if (ret < 0) {
                    log_error("Could not reconnect to input stream for %s", ctx->config.name);
                    continue;  // Keep trying
                }
                
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
        
        // Process video packets
        if (pkt->stream_index == ctx->video_stream_idx) {
            // Put packet in queue for consumers
            ret = packet_queue_put(&ctx->queue, pkt);
            if (ret < 0) {
                log_error("Failed to queue packet for stream %s", ctx->config.name);
                // Continue anyway
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
            
            // Abort packet queue
            packet_queue_abort(&reader_contexts[i]->queue);
            
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
                packet_queue_destroy(&reader_contexts[i]->queue);
                pthread_mutex_destroy(&reader_contexts[i]->consumers_mutex);
                free(reader_contexts[i]);
                reader_contexts[i] = NULL;
            }
        }
    }
    
    pthread_mutex_unlock(&contexts_mutex);
    
    log_info("Stream reader backend cleaned up");
}

/**
 * Start a stream reader for a stream
 */
stream_reader_ctx_t *start_stream_reader(const char *stream_name, int dedicated) {
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
                pthread_mutex_unlock(&contexts_mutex);
                log_info("Stream reader for %s already running", stream_name);
                return (stream_reader_ctx_t *)reader_contexts[i];  // Already running
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
    ctx->consumers = 0;
    ctx->dedicated = dedicated;
    pthread_mutex_init(&ctx->consumers_mutex, NULL);
    packet_queue_init(&ctx->queue);
    
    // Start reader thread
    if (pthread_create(&ctx->thread, NULL, stream_reader_thread, ctx) != 0) {
        packet_queue_destroy(&ctx->queue);
        pthread_mutex_destroy(&ctx->consumers_mutex);
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
    
    return (stream_reader_ctx_t *)ctx;
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
    
    // Check if there are still consumers
    pthread_mutex_lock(&ctx->consumers_mutex);
    if (ctx->consumers > 0) {
        log_warn("Stream reader for %s still has %d consumers, not stopping",
                ctx->config.name, ctx->consumers);
        pthread_mutex_unlock(&ctx->consumers_mutex);
        return -1;
    }
    pthread_mutex_unlock(&ctx->consumers_mutex);
    
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
    
    // Abort packet queue
    packet_queue_abort(&ctx->queue);
    
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
        packet_queue_destroy(&ctx->queue);
        pthread_mutex_destroy(&ctx->consumers_mutex);
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
 * Register as a consumer of the stream reader
 */
int register_stream_consumer(stream_reader_ctx_t *ctx) {
    if (!ctx) {
        log_error("Cannot register consumer: NULL context");
        return -1;
    }
    
    pthread_mutex_lock(&ctx->consumers_mutex);
    ctx->consumers++;
    
    // Assign a consumer ID and register with the queue
    int consumer_id = 0;
    
    pthread_mutex_lock(&ctx->queue.mutex);
    consumer_id = ctx->queue.next_consumer_id++;
    
    // Find an empty slot in the consumers array
    int slot = -1;
    for (int i = 0; i < MAX_QUEUE_CONSUMERS; i++) {
        if (!ctx->queue.consumers[i].active) {
            slot = i;
            break;
        }
    }
    
    if (slot < 0) {
        log_error("No empty slot for new consumer in stream %s", ctx->config.name);
        pthread_mutex_unlock(&ctx->queue.mutex);
        pthread_mutex_unlock(&ctx->consumers_mutex);
        return -1;
    }
    
    // Initialize the consumer cursor
    ctx->queue.consumers[slot].consumer_id = consumer_id;
    ctx->queue.consumers[slot].cursor = ctx->queue.head; // Start at the head of the queue
    ctx->queue.consumers[slot].last_read_time = time(NULL);
    ctx->queue.consumers[slot].active = 1;
    
    pthread_mutex_unlock(&ctx->queue.mutex);
    pthread_mutex_unlock(&ctx->consumers_mutex);
    
    log_info("Registered consumer %d for stream %s (dedicated: %d)", 
             consumer_id, ctx->config.name, ctx->dedicated);
    return consumer_id;
}

/**
 * Unregister as a consumer of the stream reader
 */
int unregister_stream_consumer(stream_reader_ctx_t *ctx, int consumer_id) {
    if (!ctx) {
        return -1;
    }
    
    pthread_mutex_lock(&ctx->consumers_mutex);
    
    // Verify the consumer ID is valid
    pthread_mutex_lock(&ctx->queue.mutex);
    int consumer_idx = find_consumer(&ctx->queue, consumer_id);
    if (consumer_idx < 0) {
        pthread_mutex_unlock(&ctx->queue.mutex);
        pthread_mutex_unlock(&ctx->consumers_mutex);
        log_warn("Consumer %d not found for stream %s", consumer_id, ctx->config.name);
        return -1;
    }
    
    // Mark the consumer as inactive
    ctx->queue.consumers[consumer_idx].active = 0;
    pthread_mutex_unlock(&ctx->queue.mutex);
    
    // Decrement the consumer count
    ctx->consumers--;
    pthread_mutex_unlock(&ctx->consumers_mutex);
    
    log_info("Unregistered consumer %d for stream %s", consumer_id, ctx->config.name);
    return 0;
}

/**
 * Get a packet from the queue (blocking)
 */
int get_packet(stream_reader_ctx_t *ctx, AVPacket *pkt, int consumer_id) {
    if (!ctx) {
        log_error("Invalid stream reader context in get_packet");
        return -1;
    }
    
    if (pkt == NULL) {
        log_error("NULL packet passed to get_packet");
        return -1;
    }
    
    if (consumer_id <= 0) {
        log_error("Invalid consumer ID (%d) in get_packet", consumer_id);
        return -1;
    }
    
    // Pass the dedicated flag from the context
    // This ensures proper handling of shared vs dedicated stream readers
    int ret = packet_queue_get(&ctx->queue, pkt, consumer_id, ctx->dedicated);
    
    // Add additional logging for debugging
    if (ret < 0) {
        log_error("Failed to get packet for consumer %d (dedicated: %d)", 
                 consumer_id, ctx->dedicated);
    }
    
    return ret;
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
            return (stream_reader_ctx_t *)reader_contexts[i];
        }
    }
    
    // If no dedicated reader exists, look for a shared reader
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (reader_contexts[i] && 
            strcmp(reader_contexts[i]->config.name, stream_name) == 0 && 
            !reader_contexts[i]->dedicated) {
            pthread_mutex_unlock(&contexts_mutex);
            return (stream_reader_ctx_t *)reader_contexts[i];
        }
    }
    
    // No existing reader found
    pthread_mutex_unlock(&contexts_mutex);
    return NULL;
}
