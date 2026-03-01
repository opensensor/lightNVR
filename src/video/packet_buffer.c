/**
 * Packet Buffer Implementation
 * 
 * Circular buffer for storing video packets for pre-event/pre-detection buffering.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <time.h>
#include <sys/stat.h>
#include <errno.h>

#include "video/packet_buffer.h"
#include "core/logger.h"
#include "core/config.h"

// Global buffer pool
static packet_buffer_pool_t buffer_pool;
static bool pool_initialized = false;

/**
 * Initialize the packet buffer pool
 */
int init_packet_buffer_pool(size_t memory_limit_mb) {
    if (pool_initialized) {
        log_warn("Packet buffer pool already initialized");
        return 0;
    }
    
    memset(&buffer_pool, 0, sizeof(packet_buffer_pool_t));
    
    if (pthread_mutex_init(&buffer_pool.pool_mutex, NULL) != 0) {
        log_error("Failed to initialize buffer pool mutex");
        return -1;
    }
    
    buffer_pool.total_memory_limit = memory_limit_mb * 1024 * 1024; // Convert MB to bytes
    buffer_pool.current_memory_usage = 0;
    buffer_pool.active_buffers = 0;

    // Buffers start inactive and without mutexes — mutexes are lazily initialized
    // per slot when first used (see create_packet_buffer).
    // memset already zeroed active and mutex_initialized fields above.

    pool_initialized = true;
    log_info("Packet buffer pool initialized (memory limit: %zu MB)", memory_limit_mb);
    
    return 0;
}

/**
 * Cleanup the packet buffer pool
 */
void cleanup_packet_buffer_pool(void) {
    if (!pool_initialized) {
        return;
    }

    // Destroy all active buffers — destroy_packet_buffer handles per-slot mutex cleanup
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (buffer_pool.buffers[i].active) {
            destroy_packet_buffer(&buffer_pool.buffers[i]);
        }
    }

    pthread_mutex_destroy(&buffer_pool.pool_mutex);

    pool_initialized = false;
    log_info("Packet buffer pool cleaned up");
}

/**
 * Estimate packet count based on FPS and duration
 */
int packet_buffer_estimate_packet_count(int fps, int duration_seconds) {
    // Add 20% overhead for audio packets and variations
    return (int)((fps * duration_seconds) * 1.2);
}

/**
 * Estimate the buffer memory (in MB) needed for one detection stream.
 *
 * Uses a conservative H.264 compressed-bitrate model:
 *   bytes/sec ≈ (width × height × fps × 0.1 bpp) / 8
 * plus ~64 kbps for an optional audio track.
 *
 * Falls back to sensible defaults when parameters are 0.
 */
static size_t estimate_stream_buffer_mb(int width, int height, int fps, int pre_buffer_seconds) {
    if (width <= 0)  width  = 1280;
    if (height <= 0) height = 720;
    if (fps <= 0)    fps    = 15;
    if (pre_buffer_seconds <= 0) pre_buffer_seconds = DEFAULT_BUFFER_SECONDS;
    if (pre_buffer_seconds > MAX_BUFFER_SECONDS) pre_buffer_seconds = MAX_BUFFER_SECONDS;

    // H.264 compressed: ~0.1 bits-per-pixel is a conservative midpoint
    double bytes_per_sec = ((double)width * height * fps * 0.1) / 8.0;

    // Add ~64 kbps for audio overhead
    bytes_per_sec += 8000.0;

    // Total for the pre-buffer window with 25% structural overhead
    double total_bytes = bytes_per_sec * pre_buffer_seconds * 1.25;

    // Convert to MB, enforce minimum of 2 MB per stream
    size_t mb = (size_t)(total_bytes / (1024.0 * 1024.0));
    return mb < 2 ? 2 : mb;
}

/**
 * Calculate the total packet buffer pool size (in MB) based on all active
 * streams that use detection-based recording.
 *
 * Iterates over the current stream configuration and sums the estimated
 * per-stream buffer requirements.  A minimum of 16 MB is always returned
 * so the pool is useful even when no detection streams are configured yet.
 */
size_t calculate_packet_buffer_pool_size(void) {
    config_t *cfg = get_streaming_config();
    if (!cfg) {
        log_warn("calculate_packet_buffer_pool_size: config unavailable, using 32 MB default");
        return 32;
    }

    size_t total_mb = 0;
    int detection_streams = 0;

    for (int i = 0; i < cfg->max_streams; i++) {
        const stream_config_t *s = &cfg->streams[i];
        if (!s->enabled || !s->detection_based_recording) {
            continue;
        }

        int pre_buf = s->pre_detection_buffer > 0
                      ? s->pre_detection_buffer
                      : (cfg->default_pre_detection_buffer > 0
                         ? cfg->default_pre_detection_buffer
                         : DEFAULT_BUFFER_SECONDS);

        total_mb += estimate_stream_buffer_mb(s->width, s->height, s->fps, pre_buf);
        detection_streams++;
    }

    if (detection_streams == 0) {
        // No detection streams configured yet — reserve a small pool
        return 16;
    }

    // Add 20% pool-level headroom
    total_mb = (size_t)(total_mb * 1.2);

    // Sanity clamp: 16 MB minimum, 512 MB maximum
    if (total_mb < 16)  total_mb = 16;
    if (total_mb > 512) total_mb = 512;

    log_info("Calculated packet buffer pool size: %zu MB for %d detection stream(s)",
             total_mb, detection_streams);
    return total_mb;
}

/**
 * Reinitialize the packet buffer pool with a new memory limit.
 *
 * If the pool is not yet initialized, delegates to init_packet_buffer_pool().
 * If the limit is unchanged, returns immediately.  Active buffers are NOT
 * disrupted — only the pool-level accounting ceiling is updated.
 */
int reinit_packet_buffer_pool(size_t new_memory_limit_mb) {
    if (!pool_initialized) {
        return init_packet_buffer_pool(new_memory_limit_mb);
    }

    pthread_mutex_lock(&buffer_pool.pool_mutex);

    size_t old_limit_mb = buffer_pool.total_memory_limit / ((size_t)1024 * 1024);

    if (old_limit_mb == new_memory_limit_mb) {
        pthread_mutex_unlock(&buffer_pool.pool_mutex);
        return 0;  // No change needed
    }

    buffer_pool.total_memory_limit = new_memory_limit_mb * 1024 * 1024;

    pthread_mutex_unlock(&buffer_pool.pool_mutex);

    log_info("Packet buffer pool memory limit updated: %zu MB -> %zu MB",
             old_limit_mb, new_memory_limit_mb);
    return 0;
}

/**
 * Create a packet buffer for a stream
 */
packet_buffer_t* create_packet_buffer(const char *stream_name, int buffer_seconds, buffer_mode_t mode) {
    if (!pool_initialized) {
        log_error("Packet buffer pool not initialized");
        return NULL;
    }
    
    if (!stream_name || buffer_seconds < MIN_BUFFER_SECONDS || buffer_seconds > MAX_BUFFER_SECONDS) {
        log_error("Invalid parameters for create_packet_buffer");
        return NULL;
    }
    
    pthread_mutex_lock(&buffer_pool.pool_mutex);

    // Find a free buffer slot
    packet_buffer_t *buffer = NULL;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (!buffer_pool.buffers[i].active) {
            buffer = &buffer_pool.buffers[i];
            break;
        }
    }

    if (!buffer) {
        pthread_mutex_unlock(&buffer_pool.pool_mutex);
        log_error("No free buffer slots available");
        return NULL;
    }

    // Reset slot and lazily initialize the per-slot mutex.
    // The pool_mutex protects this entire initialization, so no need to
    // hold the per-buffer mutex here.  Doing memset BEFORE mutex_init also
    // avoids the prior bug of zeroing an already-locked mutex.
    memset(buffer, 0, sizeof(packet_buffer_t));

    if (pthread_mutex_init(&buffer->mutex, NULL) != 0) {
        log_error("Failed to initialize mutex for packet buffer");
        pthread_mutex_unlock(&buffer_pool.pool_mutex);
        return NULL;
    }
    buffer->mutex_initialized = true;

    strncpy(buffer->stream_name, stream_name, sizeof(buffer->stream_name) - 1);
    buffer->buffer_seconds = buffer_seconds;
    buffer->mode = mode;

    // Estimate packet count (assume 15 FPS average)
    buffer->max_packets = packet_buffer_estimate_packet_count(15, buffer_seconds);

    // Allocate packet array
    buffer->packets = (buffered_packet_t *)calloc(buffer->max_packets, sizeof(buffered_packet_t));
    if (!buffer->packets) {
        log_error("Failed to allocate packet array for buffer");
        pthread_mutex_destroy(&buffer->mutex);
        buffer->mutex_initialized = false;
        pthread_mutex_unlock(&buffer_pool.pool_mutex);
        return NULL;
    }

    buffer->head = 0;
    buffer->tail = 0;
    buffer->count = 0;
    buffer->active = true;

    // Initialize disk buffer path if needed
    if (mode == BUFFER_MODE_DISK || mode == BUFFER_MODE_HYBRID) {
        config_t *config = get_streaming_config();
        if (config) {
            snprintf(buffer->disk_buffer_path, sizeof(buffer->disk_buffer_path),
                    "%s/.packet_buffer_%s", config->storage_path, stream_name);
            mkdir(buffer->disk_buffer_path, 0755);
        }
    }

    buffer_pool.active_buffers++;

    pthread_mutex_unlock(&buffer_pool.pool_mutex);

    log_info("Created packet buffer for stream: %s (duration: %ds, max packets: %d, mode: %d)",
             stream_name, buffer_seconds, buffer->max_packets, mode);

    return buffer;
}

/**
 * Destroy a packet buffer
 */
void destroy_packet_buffer(packet_buffer_t *buffer) {
    if (!buffer || !buffer->active) {
        return;
    }

    // Only lock if mutex was actually initialized (lazy allocation)
    if (buffer->mutex_initialized) {
        pthread_mutex_lock(&buffer->mutex);
    }

    // Free all buffered packets
    if (buffer->packets) {
        for (int i = 0; i < buffer->max_packets; i++) {
            if (buffer->packets[i].packet) {
                av_packet_free(&buffer->packets[i].packet);
            }
        }
        free(buffer->packets);
        buffer->packets = NULL;
    }

    // Close disk buffer if open
    if (buffer->disk_buffer_file) {
        fclose(buffer->disk_buffer_file);
        buffer->disk_buffer_file = NULL;
    }

    // Update pool statistics
    pthread_mutex_lock(&buffer_pool.pool_mutex);
    buffer_pool.current_memory_usage -= buffer->current_memory_usage;
    buffer_pool.active_buffers--;
    pthread_mutex_unlock(&buffer_pool.pool_mutex);

    char stream_name_copy[256];
    strncpy(stream_name_copy, buffer->stream_name, sizeof(stream_name_copy) - 1);
    stream_name_copy[sizeof(stream_name_copy) - 1] = '\0';

    buffer->active = false;

    if (buffer->mutex_initialized) {
        pthread_mutex_unlock(&buffer->mutex);
        pthread_mutex_destroy(&buffer->mutex);
        buffer->mutex_initialized = false;
    }

    log_info("Destroyed packet buffer for stream: %s", stream_name_copy);
}

/**
 * Add a packet to the buffer
 */
int packet_buffer_add_packet(packet_buffer_t *buffer, const AVPacket *packet, time_t timestamp) {
    if (!buffer || !buffer->active || !packet) {
        return -1;
    }

    pthread_mutex_lock(&buffer->mutex);

    // Check if buffer is full
    if (buffer->count >= buffer->max_packets) {
        // Remove oldest packet to make room
        if (buffer->packets[buffer->tail].packet) {
            buffer->current_memory_usage -= buffer->packets[buffer->tail].data_size;
            av_packet_free(&buffer->packets[buffer->tail].packet);
        }
        buffer->tail = (buffer->tail + 1) % buffer->max_packets;
        buffer->count--;
        buffer->total_packets_dropped++;
    }

    // Clone the packet
    AVPacket *cloned_packet = av_packet_clone(packet);
    if (!cloned_packet) {
        log_error("Failed to clone packet for buffer");
        pthread_mutex_unlock(&buffer->mutex);
        return -1;
    }

    // Store packet in buffer
    buffered_packet_t *slot = &buffer->packets[buffer->head];
    slot->packet = cloned_packet;
    slot->timestamp = timestamp;
    slot->pts = packet->pts;
    slot->dts = packet->dts;
    slot->stream_index = packet->stream_index;
    slot->is_keyframe = (packet->flags & AV_PKT_FLAG_KEY) != 0;
    slot->data_size = packet->size;

    // Update statistics
    buffer->current_memory_usage += packet->size;
    if (buffer->current_memory_usage > buffer->peak_memory_usage) {
        buffer->peak_memory_usage = buffer->current_memory_usage;
    }
    buffer->total_bytes_buffered += packet->size;
    buffer->total_packets_buffered++;

    // Update timing
    if (buffer->count == 0) {
        buffer->oldest_packet_time = timestamp;
    }
    buffer->newest_packet_time = timestamp;

    // Advance head
    buffer->head = (buffer->head + 1) % buffer->max_packets;
    buffer->count++;

    pthread_mutex_unlock(&buffer->mutex);

    return 0;
}

/**
 * Peek at the oldest packet without removing it
 */
int packet_buffer_peek_oldest(packet_buffer_t *buffer, AVPacket **packet) {
    if (!buffer || !buffer->active || !packet) {
        return -1;
    }

    pthread_mutex_lock(&buffer->mutex);

    if (buffer->count == 0) {
        pthread_mutex_unlock(&buffer->mutex);
        return -1;
    }

    // Clone the oldest packet
    *packet = av_packet_clone(buffer->packets[buffer->tail].packet);

    pthread_mutex_unlock(&buffer->mutex);

    return (*packet != NULL) ? 0 : -1;
}

/**
 * Pop the oldest packet from the buffer
 */
int packet_buffer_pop_oldest(packet_buffer_t *buffer, AVPacket **packet) {
    if (!buffer || !buffer->active || !packet) {
        return -1;
    }

    pthread_mutex_lock(&buffer->mutex);

    if (buffer->count == 0) {
        pthread_mutex_unlock(&buffer->mutex);
        return -1;
    }

    // Transfer ownership of the packet
    *packet = buffer->packets[buffer->tail].packet;
    buffer->packets[buffer->tail].packet = NULL;

    // Update statistics
    buffer->current_memory_usage -= buffer->packets[buffer->tail].data_size;

    // Advance tail
    buffer->tail = (buffer->tail + 1) % buffer->max_packets;
    buffer->count--;

    // Update oldest packet time
    if (buffer->count > 0) {
        buffer->oldest_packet_time = buffer->packets[buffer->tail].timestamp;
    }

    pthread_mutex_unlock(&buffer->mutex);

    return 0;
}

/**
 * Flush all packets from the buffer
 */
int packet_buffer_flush(packet_buffer_t *buffer,
                       int (*callback)(const AVPacket *packet, void *user_data),
                       void *user_data) {
    if (!buffer || !buffer->active || !callback) {
        return -1;
    }

    pthread_mutex_lock(&buffer->mutex);

    int flushed_count = 0;
    int current_count = buffer->count;

    // Process all packets in order (oldest to newest)
    for (int i = 0; i < current_count; i++) {
        int index = (buffer->tail + i) % buffer->max_packets;

        if (buffer->packets[index].packet) {
            // Call callback with the packet
            int result = callback(buffer->packets[index].packet, user_data);
            if (result == 0) {
                flushed_count++;
            }

            // Free the packet
            buffer->current_memory_usage -= buffer->packets[index].data_size;
            av_packet_free(&buffer->packets[index].packet);
            buffer->packets[index].packet = NULL;
        }
    }

    // Reset buffer
    buffer->head = 0;
    buffer->tail = 0;
    buffer->count = 0;

    pthread_mutex_unlock(&buffer->mutex);

    log_info("Flushed %d packets from buffer for stream: %s", flushed_count, buffer->stream_name);

    return flushed_count;
}

/**
 * Clear all packets from the buffer
 */
void packet_buffer_clear(packet_buffer_t *buffer) {
    if (!buffer || !buffer->active) {
        return;
    }

    pthread_mutex_lock(&buffer->mutex);

    // Free all packets
    for (int i = 0; i < buffer->max_packets; i++) {
        if (buffer->packets[i].packet) {
            buffer->current_memory_usage -= buffer->packets[i].data_size;
            av_packet_free(&buffer->packets[i].packet);
            buffer->packets[i].packet = NULL;
        }
    }

    // Reset buffer
    buffer->head = 0;
    buffer->tail = 0;
    buffer->count = 0;

    pthread_mutex_unlock(&buffer->mutex);

    log_debug("Cleared buffer for stream: %s", buffer->stream_name);
}

/**
 * Get buffer statistics
 */
int packet_buffer_get_stats(packet_buffer_t *buffer, int *count, size_t *memory_usage, int *duration) {
    if (!buffer || !buffer->active) {
        return -1;
    }

    pthread_mutex_lock(&buffer->mutex);

    if (count) {
        *count = buffer->count;
    }

    if (memory_usage) {
        *memory_usage = buffer->current_memory_usage;
    }

    if (duration && buffer->count > 0) {
        *duration = (int)(buffer->newest_packet_time - buffer->oldest_packet_time);
    } else if (duration) {
        *duration = 0;
    }

    pthread_mutex_unlock(&buffer->mutex);

    return 0;
}

/**
 * Get buffer by stream name
 */
packet_buffer_t* get_packet_buffer(const char *stream_name) {
    if (!pool_initialized || !stream_name) {
        return NULL;
    }

    pthread_mutex_lock(&buffer_pool.pool_mutex);

    for (int i = 0; i < MAX_STREAMS; i++) {
        if (buffer_pool.buffers[i].active &&
            strcmp(buffer_pool.buffers[i].stream_name, stream_name) == 0) {
            pthread_mutex_unlock(&buffer_pool.pool_mutex);
            return &buffer_pool.buffers[i];
        }
    }

    pthread_mutex_unlock(&buffer_pool.pool_mutex);
    return NULL;
}

/**
 * Check if buffer is ready
 */
bool packet_buffer_is_ready(packet_buffer_t *buffer) {
    if (!buffer || !buffer->active) {
        return false;
    }

    pthread_mutex_lock(&buffer->mutex);

    bool ready = false;
    if (buffer->count > 0) {
        int duration = (int)(buffer->newest_packet_time - buffer->oldest_packet_time);
        ready = (duration >= buffer->buffer_seconds);
    }

    pthread_mutex_unlock(&buffer->mutex);

    return ready;
}

/**
 * Get keyframe count
 */
int packet_buffer_get_keyframe_count(packet_buffer_t *buffer) {
    if (!buffer || !buffer->active) {
        return 0;
    }

    pthread_mutex_lock(&buffer->mutex);

    int keyframe_count = 0;
    for (int i = 0; i < buffer->count; i++) {
        int index = (buffer->tail + i) % buffer->max_packets;
        if (buffer->packets[index].is_keyframe) {
            keyframe_count++;
        }
    }

    pthread_mutex_unlock(&buffer->mutex);

    return keyframe_count;
}

/**
 * Set memory limit for a buffer
 */
int packet_buffer_set_memory_limit(packet_buffer_t *buffer, size_t limit_mb) {
    if (!buffer || !buffer->active) {
        return -1;
    }

    // This is a placeholder for future implementation
    // Currently, memory is managed at the pool level
    log_info("Memory limit set to %zu MB for buffer: %s", limit_mb, buffer->stream_name);

    return 0;
}

/**
 * Get total memory usage
 */
size_t packet_buffer_get_total_memory_usage(void) {
    if (!pool_initialized) {
        return 0;
    }

    pthread_mutex_lock(&buffer_pool.pool_mutex);
    size_t total = buffer_pool.current_memory_usage;
    pthread_mutex_unlock(&buffer_pool.pool_mutex);

    return total;
}

/**
 * Set disk fallback
 */
int packet_buffer_set_disk_fallback(packet_buffer_t *buffer, bool enable, const char *disk_path) {
    if (!buffer || !buffer->active) {
        return -1;
    }

    pthread_mutex_lock(&buffer->mutex);

    if (enable) {
        buffer->mode = BUFFER_MODE_HYBRID;
        if (disk_path) {
            strncpy(buffer->disk_buffer_path, disk_path, sizeof(buffer->disk_buffer_path) - 1);
        }
        log_info("Enabled disk fallback for buffer: %s (path: %s)",
                 buffer->stream_name, buffer->disk_buffer_path);
    } else {
        buffer->mode = BUFFER_MODE_MEMORY;
        log_info("Disabled disk fallback for buffer: %s", buffer->stream_name);
    }

    pthread_mutex_unlock(&buffer->mutex);

    return 0;
}

