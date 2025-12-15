/**
 * Pre-Detection Buffer Strategy Interface
 * 
 * This module provides a pluggable strategy pattern for pre-detection buffering.
 * Multiple implementations can be selected based on system resources and requirements.
 * 
 * Strategies:
 * - GO2RTC_NATIVE: Leverage go2rtc's internal HLS buffering (default, lowest overhead)
 * - HLS_SEGMENT: Track existing HLS segments on disk without copying
 * - MEMORY_PACKET: AVPacket-based circular buffer in memory (highest precision)
 * - MMAP_HYBRID: Memory-mapped files with automatic disk paging
 */

#ifndef LIGHTNVR_PRE_DETECTION_BUFFER_H
#define LIGHTNVR_PRE_DETECTION_BUFFER_H

#include <stdint.h>
#include <stdbool.h>
#include <time.h>
#include <stddef.h>
#include <libavformat/avformat.h>

// Forward declarations
typedef struct pre_buffer_strategy pre_buffer_strategy_t;
typedef struct mp4_writer mp4_writer_t;

/**
 * Buffer strategy types
 */
typedef enum {
    BUFFER_STRATEGY_NONE = 0,           // No pre-buffering (disabled)
    BUFFER_STRATEGY_GO2RTC_NATIVE,      // Query go2rtc HLS buffer directly (default)
    BUFFER_STRATEGY_HLS_SEGMENT,        // Track existing HLS segments on disk
    BUFFER_STRATEGY_MEMORY_PACKET,      // AVPacket circular buffer in memory
    BUFFER_STRATEGY_MMAP_HYBRID,        // Memory-mapped file with disk paging
    BUFFER_STRATEGY_AUTO,               // Auto-select based on system resources
    BUFFER_STRATEGY_COUNT               // Number of strategies (for iteration)
} buffer_strategy_type_t;

/**
 * Buffer flush mode - how to output buffered content
 */
typedef enum {
    FLUSH_MODE_TO_FILE,                 // Write to a standalone file
    FLUSH_MODE_TO_WRITER,               // Flush packets to an active writer
    FLUSH_MODE_TO_CALLBACK,             // Call user callback for each packet
} flush_mode_t;

/**
 * Segment information for HLS-based strategies
 */
typedef struct {
    char path[512];                     // Path to segment file
    time_t timestamp;                   // Creation timestamp
    float duration;                     // Estimated duration in seconds
    size_t size_bytes;                  // File size
    bool protected;                     // Protected from cleanup
    int64_t first_pts;                  // First PTS in segment (if known)
    int64_t last_pts;                   // Last PTS in segment (if known)
} segment_info_t;

/**
 * Buffer statistics
 */
typedef struct {
    int buffered_duration_ms;           // Estimated buffered duration in milliseconds
    int segment_count;                  // Number of segments (for segment-based)
    int packet_count;                   // Number of packets (for packet-based)
    size_t memory_usage_bytes;          // Current memory usage
    size_t disk_usage_bytes;            // Current disk usage
    int keyframe_count;                 // Number of keyframes in buffer
    bool has_complete_gop;              // Buffer starts with keyframe
    time_t oldest_timestamp;            // Oldest buffered content timestamp
    time_t newest_timestamp;            // Newest buffered content timestamp
} buffer_stats_t;

/**
 * Configuration for creating a buffer strategy
 */
typedef struct {
    int buffer_seconds;                 // Target buffer duration in seconds
    size_t memory_limit_bytes;          // Maximum memory usage (0 = default)
    size_t disk_limit_bytes;            // Maximum disk usage (0 = unlimited)
    const char *storage_path;           // Base storage path for disk-based buffers
    const char *go2rtc_url;             // go2rtc API URL (for GO2RTC_NATIVE)
    int estimated_fps;                  // Estimated FPS for packet count estimation
    bool prefer_keyframe_alignment;     // Try to align flush to keyframes
} buffer_config_t;

/**
 * Packet write callback signature
 */
typedef int (*packet_write_callback_t)(const AVPacket *packet, void *user_data);

/**
 * Pre-detection buffer strategy interface
 * 
 * All strategies implement this interface for consistent usage.
 */
struct pre_buffer_strategy {
    // Strategy identification
    const char *name;                   // Human-readable name
    buffer_strategy_type_t type;        // Strategy type enum
    char stream_name[256];              // Associated stream name
    
    // Lifecycle methods
    int (*init)(pre_buffer_strategy_t *self, const buffer_config_t *config);
    void (*destroy)(pre_buffer_strategy_t *self);
    
    // Data ingestion - strategies implement one or both
    int (*add_packet)(pre_buffer_strategy_t *self, const AVPacket *packet, time_t timestamp);
    int (*add_segment)(pre_buffer_strategy_t *self, const char *segment_path, float duration);
    
    // Notification that a segment should be protected from cleanup
    int (*protect_segment)(pre_buffer_strategy_t *self, const char *segment_path);
    int (*unprotect_segment)(pre_buffer_strategy_t *self, const char *segment_path);
    
    // Flush operations - output buffered content
    int (*flush_to_file)(pre_buffer_strategy_t *self, const char *output_path);
    int (*flush_to_writer)(pre_buffer_strategy_t *self, mp4_writer_t *writer);
    int (*flush_to_callback)(pre_buffer_strategy_t *self, 
                             packet_write_callback_t callback, void *user_data);
    
    // Query buffered segments (for segment-based strategies)
    int (*get_segments)(pre_buffer_strategy_t *self, segment_info_t *segments, 
                        int max_segments, int *out_count);
    
    // State queries
    int (*get_stats)(pre_buffer_strategy_t *self, buffer_stats_t *stats);
    bool (*is_ready)(pre_buffer_strategy_t *self);  // Has enough content
    
    // Clear/reset
    void (*clear)(pre_buffer_strategy_t *self);
    
    // Private implementation data
    void *private_data;
    bool initialized;
};

// --- Factory Functions ---

/**
 * Create a buffer strategy for a stream
 * 
 * @param type Strategy type to create
 * @param stream_name Name of the stream
 * @param config Configuration parameters
 * @return Strategy instance or NULL on failure
 */
pre_buffer_strategy_t* create_buffer_strategy(buffer_strategy_type_t type,
                                               const char *stream_name,
                                               const buffer_config_t *config);

/**
 * Destroy a buffer strategy and free resources
 */
void destroy_buffer_strategy(pre_buffer_strategy_t *strategy);

/**
 * Get the default/recommended strategy type based on system resources
 */
buffer_strategy_type_t get_recommended_strategy_type(void);

/**
 * Convert strategy type to string name
 */
const char* buffer_strategy_type_to_string(buffer_strategy_type_t type);

/**
 * Parse strategy type from string name
 */
buffer_strategy_type_t buffer_strategy_type_from_string(const char *name);

#endif /* LIGHTNVR_PRE_DETECTION_BUFFER_H */

