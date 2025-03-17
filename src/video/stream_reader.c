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
static int packet_queue_get(packet_queue_t *q, AVPacket *pkt, int consumer_id);
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
    q->active_consumers = 0;
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
 * Put a packet in the queue (blocking if full)
 */
static int packet_queue_put(packet_queue_t *q, AVPacket *pkt) {
    pthread_mutex_lock(&q->mutex);
    
    // Check if we have any active consumers
    if (q->active_consumers <= 0) {
        pthread_mutex_unlock(&q->mutex);
        return 0;  // No consumers, don't bother queueing
    }
    
    while (q->size >= MAX_PACKET_QUEUE_SIZE && !q->abort_request) {
        pthread_cond_wait(&q->cond_not_full, &q->mutex);
    }
    
    if (q->abort_request) {
        pthread_mutex_unlock(&q->mutex);
        return -1;
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
    new_packet->key_frame = (pkt->flags & AV_PKT_FLAG_KEY) ? 1 : 0;
    new_packet->stream_index = pkt->stream_index;
    new_packet->size = pkt->size;
    new_packet->consumer_count = q->active_consumers;
    new_packet->all_processed = 0;
    
    // Initialize consumer status
    for (int i = 0; i < MAX_QUEUE_CONSUMERS; i++) {
        new_packet->consumer_status[i].consumer_id = 0;
        new_packet->consumer_status[i].processed = 0;
    }
    
    // Register all active consumers for this packet
    // We need to iterate through all packets in the queue to find all unique consumer IDs
    int registered_consumers = 0;
    int consumer_ids[MAX_QUEUE_CONSUMERS] = {0};
    
    // First, collect all unique consumer IDs from existing packets
    if (q->size > 0) {
        for (int i = 0; i < q->size; i++) {
            int idx = (q->head + i) % MAX_PACKET_QUEUE_SIZE;
            broadcast_packet_t *packet = &q->packets[idx];
            
            for (int j = 0; j < MAX_QUEUE_CONSUMERS; j++) {
                if (packet->consumer_status[j].consumer_id > 0) {
                    // Check if we already have this consumer ID
                    int found = 0;
                    for (int k = 0; k < registered_consumers; k++) {
                        if (consumer_ids[k] == packet->consumer_status[j].consumer_id) {
                            found = 1;
                            break;
                        }
                    }
                    
                    // If not found and we have space, add it
                    if (!found && registered_consumers < MAX_QUEUE_CONSUMERS) {
                        consumer_ids[registered_consumers++] = packet->consumer_status[j].consumer_id;
                    }
                }
            }
        }
    }
    
    // Now register all these consumers with the new packet
    for (int i = 0; i < registered_consumers && i < MAX_QUEUE_CONSUMERS; i++) {
        new_packet->consumer_status[i].consumer_id = consumer_ids[i];
        new_packet->consumer_status[i].processed = 0;
    }
    
    // Update consumer count
    new_packet->consumer_count = registered_consumers;
    
    // Add to queue
    q->tail = (q->tail + 1) % MAX_PACKET_QUEUE_SIZE;
    q->size++;
    
    pthread_cond_signal(&q->cond_not_empty);
    pthread_mutex_unlock(&q->mutex);
    
    return 0;
}

/**
 * Get a packet from the queue (blocking if empty)
 */
static int packet_queue_get(packet_queue_t *q, AVPacket *pkt, int consumer_id) {
    pthread_mutex_lock(&q->mutex);
    
    if (consumer_id <= 0) {
        log_error("Invalid consumer ID: %d", consumer_id);
        pthread_mutex_unlock(&q->mutex);
        return -1;
    }
    
    // Find the next unprocessed packet for this consumer
    int found = 0;
    int idx = q->head;
    int count = 0;
    
    while (count < q->size && !found) {
        broadcast_packet_t *packet = &q->packets[idx];
        
        // Check if this consumer has already processed this packet
        int consumer_idx = -1;
        for (int i = 0; i < MAX_QUEUE_CONSUMERS; i++) {
            if (packet->consumer_status[i].consumer_id == consumer_id) {
                consumer_idx = i;
                break;
            }
        }
        
        // If consumer not found in this packet's status array, add it
        if (consumer_idx < 0) {
            // Find an empty slot to add this consumer
            for (int i = 0; i < MAX_QUEUE_CONSUMERS; i++) {
                if (packet->consumer_status[i].consumer_id == 0) {
                    packet->consumer_status[i].consumer_id = consumer_id;
                    packet->consumer_status[i].processed = 0;
                    packet->consumer_count++;
                    consumer_idx = i;
                    break;
                }
            }
            
            // If no empty slot found, log an error but continue
            if (consumer_idx < 0) {
                log_error("No empty slot for consumer %d in packet, MAX_QUEUE_CONSUMERS (%d) may be too small",
                         consumer_id, MAX_QUEUE_CONSUMERS);
            }
        }
        
        if (consumer_idx >= 0 && !packet->consumer_status[consumer_idx].processed) {
            // Found an unprocessed packet for this consumer
            found = 1;
            break;
        }
        
        // Move to next packet
        idx = (idx + 1) % MAX_PACKET_QUEUE_SIZE;
        count++;
    }
    
    // If no unprocessed packet found, wait for new packets
    while (!found && !q->abort_request) {
        pthread_cond_wait(&q->cond_not_empty, &q->mutex);
        
        // After waking up, search again
        found = 0;
        idx = q->head;
        count = 0;
        
        while (count < q->size && !found) {
            broadcast_packet_t *packet = &q->packets[idx];
            
            // Check if this consumer has already processed this packet
            int consumer_idx = -1;
            for (int i = 0; i < MAX_QUEUE_CONSUMERS; i++) {
                if (packet->consumer_status[i].consumer_id == consumer_id) {
                    consumer_idx = i;
                    break;
                }
            }
            
            // If consumer not found in this packet's status array, add it
            if (consumer_idx < 0) {
                // Find an empty slot to add this consumer
                for (int i = 0; i < MAX_QUEUE_CONSUMERS; i++) {
                    if (packet->consumer_status[i].consumer_id == 0) {
                        packet->consumer_status[i].consumer_id = consumer_id;
                        packet->consumer_status[i].processed = 0;
                        packet->consumer_count++;
                        consumer_idx = i;
                        break;
                    }
                }
                
                // If no empty slot found, log an error but continue
                if (consumer_idx < 0) {
                    log_error("No empty slot for consumer %d in packet, MAX_QUEUE_CONSUMERS (%d) may be too small",
                             consumer_id, MAX_QUEUE_CONSUMERS);
                }
            }
            
            if (consumer_idx >= 0 && !packet->consumer_status[consumer_idx].processed) {
                // Found an unprocessed packet for this consumer
                found = 1;
                break;
            }
            
            // Move to next packet
            idx = (idx + 1) % MAX_PACKET_QUEUE_SIZE;
            count++;
        }
    }
    
    if (q->abort_request) {
        pthread_mutex_unlock(&q->mutex);
        return -1;
    }
    
    // Get the packet
    broadcast_packet_t *packet = &q->packets[idx];
    
    // Reference the packet to the output
    av_packet_unref(pkt);
    if (av_packet_ref(pkt, packet->pkt) < 0) {
        pthread_mutex_unlock(&q->mutex);
        log_error("Failed to reference packet from queue");
        return -1;
    }
    
    // Mark as processed by this consumer
    for (int i = 0; i < MAX_QUEUE_CONSUMERS; i++) {
        if (packet->consumer_status[i].consumer_id == consumer_id) {
            packet->consumer_status[i].processed = 1;
            break;
        }
    }
    
    // Check if all consumers have processed this packet
    int all_processed = 1;
    for (int i = 0; i < MAX_QUEUE_CONSUMERS; i++) {
        if (packet->consumer_status[i].consumer_id != 0 && 
            !packet->consumer_status[i].processed) {
            all_processed = 0;
            break;
        }
    }
    
    // If all consumers have processed this packet, mark it for removal
    if (all_processed) {
        packet->all_processed = 1;
        
        // If this is the head packet, remove it and any other fully processed packets
        if (idx == q->head) {
            while (q->size > 0 && q->packets[q->head].all_processed) {
                // Free the packet
                if (q->packets[q->head].pkt) {
                    av_packet_free(&q->packets[q->head].pkt);
                }
                
                // Move head pointer
                q->head = (q->head + 1) % MAX_PACKET_QUEUE_SIZE;
                q->size--;
                
                // Signal that the queue is not full
                pthread_cond_signal(&q->cond_not_full);
            }
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
    
    log_info("Starting stream reader thread for stream %s", ctx->config.name);
    
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
            av_usleep(500000);  // 500ms
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
stream_reader_ctx_t *start_stream_reader(const char *stream_name) {
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
    
    // Check if already running
    pthread_mutex_lock(&contexts_mutex);
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (reader_contexts[i] && strcmp(reader_contexts[i]->config.name, stream_name) == 0) {
            pthread_mutex_unlock(&contexts_mutex);
            log_info("Stream reader for %s already running", stream_name);
            return reader_contexts[i];  // Already running
        }
    }
    
    // Find empty slot
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
    
    log_info("Started stream reader for %s in slot %d", stream_name, slot);
    
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
        return -1;
    }
    
    pthread_mutex_lock(&ctx->consumers_mutex);
    ctx->consumers++;
    
    // Assign a consumer ID and register with the queue
    int consumer_id = 0;
    
    pthread_mutex_lock(&ctx->queue.mutex);
    consumer_id = ctx->queue.next_consumer_id++;
    ctx->queue.active_consumers++;
    
    // Initialize consumer status for all packets in the queue
    for (int i = 0; i < ctx->queue.size; i++) {
        int idx = (ctx->queue.head + i) % MAX_PACKET_QUEUE_SIZE;
        broadcast_packet_t *packet = &ctx->queue.packets[idx];
        
        // Check if this consumer is already in the packet's status array
        int consumer_idx = -1;
        for (int j = 0; j < MAX_QUEUE_CONSUMERS; j++) {
            if (packet->consumer_status[j].consumer_id == consumer_id) {
                consumer_idx = j;
                break;
            }
        }
        
        // If consumer not found, add it to an empty slot
        if (consumer_idx < 0) {
            // Find an empty slot for this consumer
            for (int j = 0; j < MAX_QUEUE_CONSUMERS; j++) {
                if (packet->consumer_status[j].consumer_id == 0) {
                    packet->consumer_status[j].consumer_id = consumer_id;
                    packet->consumer_status[j].processed = 0;
                    packet->consumer_count++;
                    consumer_idx = j;
                    break;
                }
            }
            
            // If no empty slot found, log an error
            if (consumer_idx < 0) {
                log_error("No empty slot for consumer %d in packet during registration, MAX_QUEUE_CONSUMERS (%d) may be too small",
                         consumer_id, MAX_QUEUE_CONSUMERS);
            }
        } else {
            // Consumer already exists in this packet, make sure it's marked as unprocessed
            packet->consumer_status[consumer_idx].processed = 0;
        }
    }
    
    pthread_mutex_unlock(&ctx->queue.mutex);
    pthread_mutex_unlock(&ctx->consumers_mutex);
    
    log_info("Registered consumer %d for stream %s, total consumers: %d",
            consumer_id, ctx->config.name, ctx->consumers);
    
    return consumer_id;
}

/**
 * Unregister as a consumer of the stream reader
 */
int unregister_stream_consumer(stream_reader_ctx_t *ctx, int consumer_id) {
    if (!ctx || consumer_id <= 0) {
        return -1;
    }
    
    pthread_mutex_lock(&ctx->consumers_mutex);
    if (ctx->consumers > 0) {
        ctx->consumers--;
    }
    int remaining = ctx->consumers;
    
    // Update the queue
    pthread_mutex_lock(&ctx->queue.mutex);
    ctx->queue.active_consumers--;
    
    // Mark all packets as processed for this consumer
    for (int i = 0; i < ctx->queue.size; i++) {
        int idx = (ctx->queue.head + i) % MAX_PACKET_QUEUE_SIZE;
        broadcast_packet_t *packet = &ctx->queue.packets[idx];
        
        // Find this consumer in the packet's consumer status
        for (int j = 0; j < MAX_QUEUE_CONSUMERS; j++) {
            if (packet->consumer_status[j].consumer_id == consumer_id) {
                packet->consumer_status[j].processed = 1;
                
                // Check if all consumers have processed this packet
                int all_processed = 1;
                for (int k = 0; k < MAX_QUEUE_CONSUMERS; k++) {
                    if (packet->consumer_status[k].consumer_id != 0 && 
                        !packet->consumer_status[k].processed) {
                        all_processed = 0;
                        break;
                    }
                }
                
                // If all consumers have processed this packet, mark it for removal
                if (all_processed) {
                    packet->all_processed = 1;
                }
                
                break;
            }
        }
    }
    
    // Remove any fully processed packets from the head of the queue
    while (ctx->queue.size > 0 && ctx->queue.packets[ctx->queue.head].all_processed) {
        // Free the packet
        if (ctx->queue.packets[ctx->queue.head].pkt) {
            av_packet_free(&ctx->queue.packets[ctx->queue.head].pkt);
        }
        
        // Move head pointer
        ctx->queue.head = (ctx->queue.head + 1) % MAX_PACKET_QUEUE_SIZE;
        ctx->queue.size--;
        
        // Signal that the queue is not full
        pthread_cond_signal(&ctx->queue.cond_not_full);
    }
    
    pthread_mutex_unlock(&ctx->queue.mutex);
    pthread_mutex_unlock(&ctx->consumers_mutex);
    
    log_info("Unregistered consumer %d for stream %s, remaining consumers: %d",
            consumer_id, ctx->config.name, remaining);
    
    // If no more consumers, consider stopping the reader
    if (remaining == 0) {
        log_info("No more consumers for stream %s, reader will be stopped when appropriate",
                ctx->config.name);
    }
    
    return 0;
}

/**
 * Get a packet from the queue (blocking)
 */
int get_packet(stream_reader_ctx_t *ctx, AVPacket *pkt, int consumer_id) {
    if (!ctx || !pkt || consumer_id <= 0) {
        return -1;
    }
    
    return packet_queue_get(&ctx->queue, pkt, consumer_id);
}

/**
 * Get the stream reader for a stream
 */
stream_reader_ctx_t *get_stream_reader(const char *stream_name) {
    if (!stream_name) {
        return NULL;
    }
    
    pthread_mutex_lock(&contexts_mutex);
    
    stream_reader_ctx_t *ctx = NULL;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (reader_contexts[i] && strcmp(reader_contexts[i]->config.name, stream_name) == 0) {
            ctx = reader_contexts[i];
            break;
        }
    }
    
    pthread_mutex_unlock(&contexts_mutex);
    
    return ctx;
}
