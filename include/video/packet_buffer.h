#ifndef LIGHTNVR_PACKET_BUFFER_H
#define LIGHTNVR_PACKET_BUFFER_H

#include <stdint.h>
#include <stdbool.h>
#include <time.h>
#include <pthread.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>

/**
 * Packet Buffer Module
 * 
 * This module implements a circular buffer for storing video packets.
 * It provides efficient memory management for pre-event buffering in:
 * - Detection-based recording (pre-detection buffer)
 * - Motion-triggered recording (pre-event buffer)
 * 
 * Features:
 * - Circular buffer with configurable size
 * - Stores AVPacket data with timestamps
 * - Memory-efficient packet storage
 * - Thread-safe operations
 * - Optional disk-based fallback for resource-constrained systems
 */

// Maximum buffer size in seconds
#define MAX_BUFFER_SECONDS 30
#define MIN_BUFFER_SECONDS 5
#define DEFAULT_BUFFER_SECONDS 5

// Buffer storage modes
typedef enum {
    BUFFER_MODE_MEMORY = 0,     // Store packets in memory (default)
    BUFFER_MODE_DISK = 1,       // Store packets on disk (for low-memory systems)
    BUFFER_MODE_HYBRID = 2      // Use memory with disk fallback
} buffer_mode_t;

// Buffered packet structure
typedef struct {
    AVPacket *packet;           // The actual packet (NULL if slot is empty)
    time_t timestamp;           // When this packet was captured
    int64_t pts;                // Presentation timestamp
    int64_t dts;                // Decode timestamp
    int stream_index;           // Stream index (video/audio)
    bool is_keyframe;           // Whether this is a keyframe
    size_t data_size;           // Size of packet data
} buffered_packet_t;

// Circular buffer structure
typedef struct {
    char stream_name[256];      // Stream name for this buffer
    
    // Buffer configuration
    int buffer_seconds;         // Buffer duration in seconds
    int max_packets;            // Maximum number of packets to store
    buffer_mode_t mode;         // Storage mode
    
    // Circular buffer
    buffered_packet_t *packets; // Array of buffered packets
    int head;                   // Write position
    int tail;                   // Read position
    int count;                  // Number of packets in buffer
    
    // Statistics
    uint64_t total_packets_buffered;    // Total packets buffered
    uint64_t total_packets_dropped;     // Packets dropped due to full buffer
    uint64_t total_bytes_buffered;      // Total bytes buffered
    size_t current_memory_usage;        // Current memory usage in bytes
    size_t peak_memory_usage;           // Peak memory usage in bytes
    
    // Timing information
    time_t oldest_packet_time;  // Timestamp of oldest packet in buffer
    time_t newest_packet_time;  // Timestamp of newest packet in buffer
    
    // Disk-based buffer (if mode is DISK or HYBRID)
    char disk_buffer_path[512]; // Path to disk buffer directory
    FILE *disk_buffer_file;     // File handle for disk buffer
    
    // Thread safety
    pthread_mutex_t mutex;
    bool active;                // Whether this buffer is in use
} packet_buffer_t;

// Buffer pool for managing multiple stream buffers
typedef struct {
    packet_buffer_t buffers[16];    // One buffer per stream (MAX_STREAMS)
    pthread_mutex_t pool_mutex;
    int active_buffers;
    size_t total_memory_limit;      // Total memory limit for all buffers
    size_t current_memory_usage;    // Current total memory usage
} packet_buffer_pool_t;

/**
 * Initialize the packet buffer pool
 * 
 * @param memory_limit_mb Total memory limit in MB for all buffers (0 = unlimited)
 * @return 0 on success, non-zero on failure
 */
int init_packet_buffer_pool(size_t memory_limit_mb);

/**
 * Cleanup the packet buffer pool
 */
void cleanup_packet_buffer_pool(void);

/**
 * Create a packet buffer for a stream
 * 
 * @param stream_name Name of the stream
 * @param buffer_seconds Duration of buffer in seconds
 * @param mode Buffer storage mode
 * @return Pointer to buffer on success, NULL on failure
 */
packet_buffer_t* create_packet_buffer(const char *stream_name, int buffer_seconds, buffer_mode_t mode);

/**
 * Destroy a packet buffer
 * 
 * @param buffer Buffer to destroy
 */
void destroy_packet_buffer(packet_buffer_t *buffer);

/**
 * Add a packet to the buffer
 * 
 * @param buffer Buffer to add to
 * @param packet Packet to add (will be cloned)
 * @param timestamp Timestamp of the packet
 * @return 0 on success, non-zero on failure
 */
int packet_buffer_add_packet(packet_buffer_t *buffer, const AVPacket *packet, time_t timestamp);

/**
 * Get the oldest packet from the buffer (without removing it)
 * 
 * @param buffer Buffer to read from
 * @param packet Output packet (caller must free with av_packet_free)
 * @return 0 on success, -1 if buffer is empty
 */
int packet_buffer_peek_oldest(packet_buffer_t *buffer, AVPacket **packet);

/**
 * Remove and return the oldest packet from the buffer
 * 
 * @param buffer Buffer to read from
 * @param packet Output packet (caller must free with av_packet_free)
 * @return 0 on success, -1 if buffer is empty
 */
int packet_buffer_pop_oldest(packet_buffer_t *buffer, AVPacket **packet);

/**
 * Flush all packets from the buffer to a callback function
 * This is used when detection/motion is triggered to write the pre-buffer to the recording
 *
 * @param buffer Buffer to flush
 * @param callback Function to call for each packet
 * @param user_data User data to pass to callback
 * @return Number of packets flushed, -1 on error
 */
int packet_buffer_flush(packet_buffer_t *buffer,
                       int (*callback)(const AVPacket *packet, void *user_data),
                       void *user_data);

/**
 * Clear all packets from the buffer
 *
 * @param buffer Buffer to clear
 */
void packet_buffer_clear(packet_buffer_t *buffer);

/**
 * Get buffer statistics
 *
 * @param buffer Buffer to query
 * @param count Output: number of packets in buffer
 * @param memory_usage Output: current memory usage in bytes
 * @param duration Output: duration of buffered content in seconds
 * @return 0 on success, non-zero on failure
 */
int packet_buffer_get_stats(packet_buffer_t *buffer, int *count, size_t *memory_usage, int *duration);

/**
 * Get buffer by stream name
 *
 * @param stream_name Name of the stream
 * @return Pointer to buffer, or NULL if not found
 */
packet_buffer_t* get_packet_buffer(const char *stream_name);

/**
 * Check if buffer has enough data for the configured duration
 *
 * @param buffer Buffer to check
 * @return true if buffer has enough data, false otherwise
 */
bool packet_buffer_is_ready(packet_buffer_t *buffer);

/**
 * Get the number of keyframes in the buffer
 *
 * @param buffer Buffer to query
 * @return Number of keyframes
 */
int packet_buffer_get_keyframe_count(packet_buffer_t *buffer);

/**
 * Estimate the number of packets needed for a given duration
 * This is used to calculate max_packets based on buffer_seconds and stream FPS
 *
 * @param fps Frames per second of the stream
 * @param duration_seconds Duration in seconds
 * @return Estimated number of packets
 */
int packet_buffer_estimate_packet_count(int fps, int duration_seconds);

/**
 * Set memory limit for a specific buffer
 *
 * @param buffer Buffer to configure
 * @param limit_mb Memory limit in MB
 * @return 0 on success, non-zero on failure
 */
int packet_buffer_set_memory_limit(packet_buffer_t *buffer, size_t limit_mb);

/**
 * Get total memory usage across all buffers
 *
 * @return Total memory usage in bytes
 */
size_t packet_buffer_get_total_memory_usage(void);

/**
 * Enable/disable disk-based fallback for a buffer
 *
 * @param buffer Buffer to configure
 * @param enable true to enable disk fallback, false to disable
 * @param disk_path Path to disk buffer directory (NULL to use default)
 * @return 0 on success, non-zero on failure
 */
int packet_buffer_set_disk_fallback(packet_buffer_t *buffer, bool enable, const char *disk_path);

#endif /* LIGHTNVR_PACKET_BUFFER_H */

