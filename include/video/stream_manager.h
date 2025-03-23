#ifndef LIGHTNVR_STREAM_MANAGER_H
#define LIGHTNVR_STREAM_MANAGER_H

#include <stdint.h>
#include <stdbool.h>
#include <time.h>
#include "core/config.h"

// Stream status enum
typedef enum {
    STREAM_STATUS_UNKNOWN = 0,
    STREAM_STATUS_STOPPED,
    STREAM_STATUS_STARTING,
    STREAM_STATUS_RUNNING,
    STREAM_STATUS_STOPPING,    // Stream is in the process of stopping
    STREAM_STATUS_ERROR,
    STREAM_STATUS_RECONNECTING
} stream_status_t;

// Stream statistics structure
typedef struct {
    uint64_t bytes_received;
    uint64_t frames_received;
    uint64_t frames_dropped;
    uint64_t errors;
    uint64_t reconnects;
    double bitrate;          // in kbps
    double fps;              // actual fps
    uint64_t uptime;         // in seconds
    uint64_t last_frame_time; // timestamp of last frame
} stream_stats_t;

// Stream handle type (opaque)
typedef struct stream_handle_s* stream_handle_t;

/**
 * Initialize the stream manager
 * 
 * @param max_streams Maximum number of streams to support
 * @return 0 on success, non-zero on failure
 */
int init_stream_manager(int max_streams);

/**
 * Shutdown the stream manager
 */
void shutdown_stream_manager(void);

/**
 * Add a new stream
 * 
 * @param config Stream configuration
 * @return Stream handle on success, NULL on failure
 */
stream_handle_t add_stream(const stream_config_t *config);

/**
 * Remove a stream
 * 
 * @param handle Stream handle
 * @return 0 on success, non-zero on failure
 */
int remove_stream(stream_handle_t handle);

/**
 * Start a stream
 * 
 * @param handle Stream handle
 * @return 0 on success, non-zero on failure
 */
int start_stream(stream_handle_t handle);

/**
 * Stop a stream
 * 
 * @param handle Stream handle
 * @return 0 on success, non-zero on failure
 */
int stop_stream(stream_handle_t handle);

/**
 * Get stream status
 * 
 * @param handle Stream handle
 * @return Stream status
 */
stream_status_t get_stream_status(stream_handle_t handle);

/**
 * Get stream statistics
 * 
 * @param handle Stream handle
 * @param stats Pointer to statistics structure to fill
 * @return 0 on success, non-zero on failure
 */
int get_stream_stats(stream_handle_t handle, stream_stats_t *stats);

/**
 * Get stream configuration
 * 
 * @param handle Stream handle
 * @param config Pointer to configuration structure to fill
 * @return 0 on success, non-zero on failure
 */
int get_stream_config(stream_handle_t handle, stream_config_t *config);

/**
 * Get stream by index
 * 
 * @param index Stream index (0-based)
 * @return Stream handle on success, NULL if index is out of range
 */
stream_handle_t get_stream_by_index(int index);

/**
 * Get stream by name
 * 
 * @param name Stream name
 * @return Stream handle on success, NULL if not found
 */
stream_handle_t get_stream_by_name(const char *name);

/**
 * Get number of active streams
 * 
 * @return Number of active streams
 */
int get_active_stream_count(void);

/**
 * Get total number of streams
 * 
 * @return Total number of streams
 */
int get_total_stream_count(void);

/**
 * Set stream priority
 * 
 * @param handle Stream handle
 * @param priority Priority level (1-10, higher = more important)
 * @return 0 on success, non-zero on failure
 */
int set_stream_priority(stream_handle_t handle, int priority);

/**
 * Enable/disable recording for a stream
 * 
 * @param handle Stream handle
 * @param enable True to enable recording, false to disable
 * @return 0 on success, non-zero on failure
 */
int set_stream_recording(stream_handle_t handle, bool enable);

/**
 * Enable/disable detection-based recording for a stream
 * 
 * @param handle Stream handle
 * @param enable True to enable detection-based recording, false to disable
 * @param model_path Path to the detection model file
 * @return 0 on success, non-zero on failure
 */
int set_stream_detection_recording(stream_handle_t handle, bool enable, const char *model_path);

/**
 * Set detection parameters for a stream
 * 
 * @param handle Stream handle
 * @param interval Number of frames between detection checks
 * @param threshold Confidence threshold for detection (0.0-1.0)
 * @param pre_buffer Seconds to keep before detection
 * @param post_buffer Seconds to keep after detection
 * @return 0 on success, non-zero on failure
 */
int set_stream_detection_params(stream_handle_t handle, int interval, float threshold, 
                               int pre_buffer, int post_buffer);

/**
 * Set the last detection time for a stream
 * 
 * @param handle Stream handle
 * @param time Timestamp of the last detection
 * @return 0 on success, non-zero on failure
 */
int set_stream_last_detection_time(stream_handle_t handle, time_t time);

/**
 * Enable/disable streaming for a stream
 * 
 * @param handle Stream handle
 * @param enabled True to enable streaming, false to disable
 * @return 0 on success, non-zero on failure
 */
int set_stream_streaming_enabled(stream_handle_t handle, bool enabled);

/**
 * Set ONVIF flag for a stream
 * 
 * @param handle Stream handle
 * @param is_onvif True if the stream is an ONVIF camera, false otherwise
 * @return 0 on success, non-zero on failure
 */
int set_stream_onvif_flag(stream_handle_t handle, bool is_onvif);

/**
 * Get a snapshot from the stream
 * 
 * @param handle Stream handle
 * @param path Path to save the snapshot
 * @return 0 on success, non-zero on failure
 */
int get_stream_snapshot(stream_handle_t handle, const char *path);

/**
 * Get memory usage statistics for the stream manager
 * 
 * @param used_memory Pointer to store used memory in bytes
 * @param peak_memory Pointer to store peak memory usage in bytes
 * @return 0 on success, non-zero on failure
 */
int get_stream_manager_memory_usage(uint64_t *used_memory, uint64_t *peak_memory);

#endif // LIGHTNVR_STREAM_MANAGER_H
