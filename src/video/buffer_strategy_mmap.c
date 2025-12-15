/**
 * Memory-Mapped Hybrid Buffer Strategy
 * 
 * Uses mmap for memory-like access with automatic disk paging.
 * 
 * This strategy:
 * - Memory-maps a file to store packet data
 * - OS handles paging between memory and disk
 * - Gets benefits of both memory speed and disk capacity
 * - Survives process crashes (data is on disk)
 * 
 * Advantages:
 * - Memory-like access speed for hot data
 * - Automatic disk paging for cold data
 * - Larger buffers than pure memory approach
 * - Data persists across restarts
 * 
 * Disadvantages:
 * - More complex implementation
 * - Disk I/O for cold pages
 * - Fixed file size allocation
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <errno.h>

#include "video/pre_detection_buffer.h"
#include "core/logger.h"
#include "core/config.h"

// Packet entry in mmap buffer (fixed size for simplicity)
typedef struct {
    uint32_t magic;                     // Magic value for validation
    uint32_t data_size;                 // Actual packet data size
    int64_t pts;                        // Presentation timestamp
    int64_t dts;                        // Decode timestamp
    int stream_index;                   // Stream index
    uint32_t flags;                     // Packet flags (keyframe, etc.)
    time_t timestamp;                   // Wall clock timestamp
    uint8_t data[0];                    // Variable length packet data
} __attribute__((packed)) mmap_packet_entry_t;

#define MMAP_MAGIC 0x4D4D5056            // "MMPV" - mmap packet video
#define MAX_PACKET_SIZE (256 * 1024)     // 256KB max per packet
#define ENTRY_SIZE(data_sz) (sizeof(mmap_packet_entry_t) + (data_sz))
#define ENTRY_ALIGNED_SIZE(data_sz) (((ENTRY_SIZE(data_sz) + 4095) / 4096) * 4096)

// Mmap buffer header
typedef struct {
    uint32_t magic;                     // File magic
    uint32_t version;                   // Format version
    uint32_t entry_count;               // Number of entries
    uint32_t head;                      // Write position
    uint32_t tail;                      // Read position
    uint64_t total_size;                // Total mapped size
    uint64_t data_offset;               // Offset to data area
    char stream_name[256];              // Stream name
} __attribute__((packed)) mmap_buffer_header_t;

#define MMAP_FILE_MAGIC 0x4E564D4D       // "NVMM" - NVR mmap

// Strategy private data
typedef struct {
    char stream_name[256];
    char file_path[512];                // Path to mmap file
    int fd;                             // File descriptor
    uint8_t *mapped_data;               // Mmap pointer
    size_t mapped_size;                 // Total mapped size
    mmap_buffer_header_t *header;       // Pointer to header
    uint8_t *data_area;                 // Pointer to data area
    
    int buffer_seconds;
    int max_entries;                    // Maximum packet entries
    size_t entry_size;                  // Size per entry slot
    
    pthread_mutex_t lock;
    
    // Statistics
    int current_count;
    size_t current_bytes;
    time_t oldest_timestamp;
    time_t newest_timestamp;
    int keyframe_count;
} mmap_strategy_data_t;

// --- Private helper functions ---

static int create_mmap_file(mmap_strategy_data_t *data, size_t size) {
    // Open/create file
    data->fd = open(data->file_path, O_RDWR | O_CREAT, 0644);
    if (data->fd < 0) {
        log_error("Failed to open mmap file %s: %s", data->file_path, strerror(errno));
        return -1;
    }
    
    // Truncate to desired size
    if (ftruncate(data->fd, size) < 0) {
        log_error("Failed to resize mmap file: %s", strerror(errno));
        close(data->fd);
        data->fd = -1;
        return -1;
    }
    
    // Memory map the file
    data->mapped_data = mmap(NULL, size, PROT_READ | PROT_WRITE, MAP_SHARED, data->fd, 0);
    if (data->mapped_data == MAP_FAILED) {
        log_error("Failed to mmap file: %s", strerror(errno));
        close(data->fd);
        data->fd = -1;
        return -1;
    }
    
    data->mapped_size = size;
    data->header = (mmap_buffer_header_t *)data->mapped_data;
    data->data_area = data->mapped_data + sizeof(mmap_buffer_header_t);
    
    // Initialize header
    data->header->magic = MMAP_FILE_MAGIC;
    data->header->version = 1;
    data->header->entry_count = 0;
    data->header->head = 0;
    data->header->tail = 0;
    data->header->total_size = size;
    data->header->data_offset = sizeof(mmap_buffer_header_t);
    strncpy(data->header->stream_name, data->stream_name, sizeof(data->header->stream_name) - 1);
    
    // Advise kernel about access pattern
    madvise(data->mapped_data, size, MADV_SEQUENTIAL);

    log_info("Created mmap buffer file: %s (%zu bytes)", data->file_path, size);

    return 0;
}

// --- Strategy interface methods ---

static int mmap_strategy_init(pre_buffer_strategy_t *self, const buffer_config_t *config) {
    mmap_strategy_data_t *data = (mmap_strategy_data_t *)self->private_data;

    data->buffer_seconds = config->buffer_seconds;

    // Calculate buffer size
    // Assume ~30 fps, ~50KB per frame average, 1.5x overhead
    int estimated_frames = config->estimated_fps > 0 ? config->estimated_fps : 30;
    data->max_entries = estimated_frames * config->buffer_seconds * 2;  // 2x for audio+video
    data->entry_size = ENTRY_ALIGNED_SIZE(MAX_PACKET_SIZE);

    size_t total_size = sizeof(mmap_buffer_header_t) +
                        (data->max_entries * data->entry_size);

    // Cap at configured limit if specified
    if (config->disk_limit_bytes > 0 && total_size > config->disk_limit_bytes) {
        total_size = config->disk_limit_bytes;
        data->max_entries = (total_size - sizeof(mmap_buffer_header_t)) / data->entry_size;
    }

    // Set up file path
    extern config_t g_config;
    snprintf(data->file_path, sizeof(data->file_path),
             "%s/buffer/%s_prebuffer.mmap",
             config->storage_path ? config->storage_path : g_config.storage_path,
             data->stream_name);

    // Ensure directory exists
    char dir_path[512];
    snprintf(dir_path, sizeof(dir_path), "%s/buffer",
             config->storage_path ? config->storage_path : g_config.storage_path);
    mkdir(dir_path, 0755);

    pthread_mutex_init(&data->lock, NULL);

    // Create mmap file
    if (create_mmap_file(data, total_size) != 0) {
        pthread_mutex_destroy(&data->lock);
        return -1;
    }

    self->initialized = true;
    log_info("Mmap strategy initialized for %s (%d entries, %zu bytes)",
             data->stream_name, data->max_entries, total_size);

    return 0;
}

static void mmap_strategy_destroy(pre_buffer_strategy_t *self) {
    mmap_strategy_data_t *data = (mmap_strategy_data_t *)self->private_data;

    pthread_mutex_lock(&data->lock);

    if (data->mapped_data && data->mapped_data != MAP_FAILED) {
        msync(data->mapped_data, data->mapped_size, MS_SYNC);
        munmap(data->mapped_data, data->mapped_size);
    }

    if (data->fd >= 0) {
        close(data->fd);
    }

    // Optionally remove the buffer file
    // unlink(data->file_path);

    pthread_mutex_unlock(&data->lock);
    pthread_mutex_destroy(&data->lock);

    log_debug("Mmap strategy destroyed for %s", data->stream_name);
    free(data);
    self->private_data = NULL;
}

static int mmap_strategy_add_packet(pre_buffer_strategy_t *self,
                                     const AVPacket *packet,
                                     time_t timestamp) {
    mmap_strategy_data_t *data = (mmap_strategy_data_t *)self->private_data;

    if (!packet || packet->size > MAX_PACKET_SIZE) {
        return -1;
    }

    pthread_mutex_lock(&data->lock);

    // Calculate entry position
    size_t offset = data->header->head * data->entry_size;
    mmap_packet_entry_t *entry = (mmap_packet_entry_t *)(data->data_area + offset);

    // If buffer is full, advance tail
    if (data->header->entry_count >= data->max_entries) {
        // Remove oldest entry from stats
        size_t tail_offset = data->header->tail * data->entry_size;
        mmap_packet_entry_t *tail_entry = (mmap_packet_entry_t *)(data->data_area + tail_offset);
        if (tail_entry->magic == MMAP_MAGIC) {
            data->current_bytes -= tail_entry->data_size;
            if (tail_entry->flags & AV_PKT_FLAG_KEY) {
                data->keyframe_count--;
            }
        }

        data->header->tail = (data->header->tail + 1) % data->max_entries;
        data->header->entry_count--;
    }

    // Write entry
    entry->magic = MMAP_MAGIC;
    entry->data_size = packet->size;
    entry->pts = packet->pts;
    entry->dts = packet->dts;
    entry->stream_index = packet->stream_index;
    entry->flags = packet->flags;
    entry->timestamp = timestamp;
    memcpy(entry->data, packet->data, packet->size);

    // Update head
    data->header->head = (data->header->head + 1) % data->max_entries;
    data->header->entry_count++;
    data->current_count = data->header->entry_count;
    data->current_bytes += packet->size;
    data->newest_timestamp = timestamp;

    if (data->header->entry_count == 1) {
        data->oldest_timestamp = timestamp;
    }

    if (packet->flags & AV_PKT_FLAG_KEY) {
        data->keyframe_count++;
    }

    pthread_mutex_unlock(&data->lock);

    return 0;
}

static int mmap_strategy_get_stats(pre_buffer_strategy_t *self, buffer_stats_t *stats) {
    mmap_strategy_data_t *data = (mmap_strategy_data_t *)self->private_data;

    memset(stats, 0, sizeof(*stats));

    pthread_mutex_lock(&data->lock);

    stats->packet_count = data->current_count;
    stats->memory_usage_bytes = 0;  // Memory managed by OS
    stats->disk_usage_bytes = data->mapped_size;
    stats->keyframe_count = data->keyframe_count;
    stats->has_complete_gop = (data->keyframe_count > 0);
    stats->oldest_timestamp = data->oldest_timestamp;
    stats->newest_timestamp = data->newest_timestamp;

    if (data->oldest_timestamp > 0 && data->newest_timestamp > 0) {
        stats->buffered_duration_ms = (int)(data->newest_timestamp - data->oldest_timestamp) * 1000;
    }

    pthread_mutex_unlock(&data->lock);

    return 0;
}

static bool mmap_strategy_is_ready(pre_buffer_strategy_t *self) {
    mmap_strategy_data_t *data = (mmap_strategy_data_t *)self->private_data;

    // Ready if we have at least 1 second of content
    return (data->newest_timestamp - data->oldest_timestamp) >= 1;
}

static void mmap_strategy_clear(pre_buffer_strategy_t *self) {
    mmap_strategy_data_t *data = (mmap_strategy_data_t *)self->private_data;

    pthread_mutex_lock(&data->lock);

    data->header->head = 0;
    data->header->tail = 0;
    data->header->entry_count = 0;
    data->current_count = 0;
    data->current_bytes = 0;
    data->keyframe_count = 0;
    data->oldest_timestamp = 0;
    data->newest_timestamp = 0;

    pthread_mutex_unlock(&data->lock);
}

static int mmap_strategy_flush_to_callback(pre_buffer_strategy_t *self,
                                            packet_write_callback_t callback,
                                            void *user_data) {
    mmap_strategy_data_t *data = (mmap_strategy_data_t *)self->private_data;

    pthread_mutex_lock(&data->lock);

    int flushed = 0;
    uint32_t pos = data->header->tail;

    for (uint32_t i = 0; i < data->header->entry_count; i++) {
        size_t offset = pos * data->entry_size;
        mmap_packet_entry_t *entry = (mmap_packet_entry_t *)(data->data_area + offset);

        if (entry->magic != MMAP_MAGIC) {
            log_warn("Invalid mmap entry at position %u", pos);
            pos = (pos + 1) % data->max_entries;
            continue;
        }

        // Reconstruct AVPacket
        AVPacket *pkt = av_packet_alloc();
        if (!pkt) {
            break;
        }

        if (av_new_packet(pkt, entry->data_size) < 0) {
            av_packet_free(&pkt);
            break;
        }

        memcpy(pkt->data, entry->data, entry->data_size);
        pkt->pts = entry->pts;
        pkt->dts = entry->dts;
        pkt->stream_index = entry->stream_index;
        pkt->flags = entry->flags;

        int ret = callback(pkt, user_data);
        av_packet_free(&pkt);

        if (ret < 0) {
            break;
        }

        flushed++;
        pos = (pos + 1) % data->max_entries;
    }

    pthread_mutex_unlock(&data->lock);

    log_debug("Flushed %d packets from mmap buffer", flushed);

    return flushed;
}

// --- Factory function ---

pre_buffer_strategy_t* create_mmap_hybrid_strategy(const char *stream_name,
                                                    const buffer_config_t *config) {
    pre_buffer_strategy_t *strategy = calloc(1, sizeof(pre_buffer_strategy_t));
    if (!strategy) {
        log_error("Failed to allocate mmap strategy");
        return NULL;
    }

    mmap_strategy_data_t *data = calloc(1, sizeof(mmap_strategy_data_t));
    if (!data) {
        log_error("Failed to allocate mmap strategy data");
        free(strategy);
        return NULL;
    }

    strncpy(data->stream_name, stream_name, sizeof(data->stream_name) - 1);
    data->fd = -1;

    strategy->name = "mmap_hybrid";
    strategy->type = BUFFER_STRATEGY_MMAP_HYBRID;
    strncpy(strategy->stream_name, stream_name, sizeof(strategy->stream_name) - 1);
    strategy->private_data = data;

    // Set interface methods
    strategy->init = mmap_strategy_init;
    strategy->destroy = mmap_strategy_destroy;
    strategy->add_packet = mmap_strategy_add_packet;
    strategy->add_segment = NULL;  // Not used
    strategy->protect_segment = NULL;
    strategy->unprotect_segment = NULL;
    strategy->get_segments = NULL;
    strategy->flush_to_file = NULL;  // TODO: implement using flush_to_callback
    strategy->flush_to_writer = NULL;
    strategy->flush_to_callback = mmap_strategy_flush_to_callback;
    strategy->get_stats = mmap_strategy_get_stats;
    strategy->is_ready = mmap_strategy_is_ready;
    strategy->clear = mmap_strategy_clear;

    // Initialize
    if (strategy->init(strategy, config) != 0) {
        log_error("Failed to initialize mmap strategy for %s", stream_name);
        free(data);
        free(strategy);
        return NULL;
    }

    return strategy;
}

