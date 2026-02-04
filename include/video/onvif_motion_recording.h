#ifndef LIGHTNVR_ONVIF_MOTION_RECORDING_H
#define LIGHTNVR_ONVIF_MOTION_RECORDING_H

#include <stdint.h>
#include <stdbool.h>
#include <time.h>
#include <pthread.h>
#include <libavformat/avformat.h>
#include "core/config.h"
#include "video/packet_buffer.h"

/**
 * ONVIF Motion Detection Recording Module
 * 
 * This module implements automated recording triggered by ONVIF motion detection events.
 * It provides:
 * - Event-based recording triggered by ONVIF motion events
 * - Configurable pre/post-event buffer recording
 * - Recording state management
 * - Integration with existing LightNVR detection framework
 */

// Maximum number of motion events in queue
#define MAX_MOTION_EVENT_QUEUE 100

// Recording states
typedef enum {
    RECORDING_STATE_IDLE = 0,       // No motion, no recording
    RECORDING_STATE_BUFFERING,      // Pre-event buffering active
    RECORDING_STATE_RECORDING,      // Active recording due to motion
    RECORDING_STATE_FINALIZING      // Post-event buffer, finishing recording
} recording_state_t;

// Motion event structure
typedef struct {
    char stream_name[MAX_STREAM_NAME];
    time_t timestamp;
    char event_type[64];            // Type of motion event
    float confidence;               // Event confidence (if available)
    bool active;                    // Whether motion is currently active
} motion_event_t;

// Event queue structure
typedef struct {
    motion_event_t events[MAX_MOTION_EVENT_QUEUE];
    int head;
    int tail;
    int count;
    pthread_mutex_t mutex;
    pthread_cond_t cond;
} motion_event_queue_t;

// Recording context for a single stream
typedef struct {
    char stream_name[MAX_STREAM_NAME];
    recording_state_t state;

    // Configuration
    int pre_buffer_seconds;         // Pre-event buffer duration
    int post_buffer_seconds;        // Post-event buffer duration
    int max_file_duration;          // Maximum recording file duration
    bool enabled;                   // Whether motion recording is enabled

    // Buffer management
    packet_buffer_t *buffer;        // Circular buffer for pre-event recording
    bool buffer_enabled;            // Whether buffering is enabled

    // State tracking
    time_t last_motion_time;        // Last time motion was detected
    time_t recording_start_time;    // When current recording started
    time_t state_change_time;       // When state last changed
    char current_file_path[MAX_PATH_LENGTH];  // Current recording file
    bool buffer_flushed;            // Whether buffer has been flushed to recording

    // Statistics
    uint64_t total_recordings;      // Total number of recordings created
    uint64_t total_motion_events;   // Total motion events processed
    uint64_t total_buffer_flushes;  // Total times buffer was flushed

    pthread_mutex_t mutex;
    bool active;                    // Whether this context is in use
} motion_recording_context_t;

// Configuration for motion recording (per-camera)
typedef struct {
    bool enabled;                   // Enable motion-based recording
    int pre_buffer_seconds;         // Pre-event buffer (5-30 seconds)
    int post_buffer_seconds;        // Post-event buffer (5-60 seconds)
    int max_file_duration;          // Max duration per file (seconds)
    char codec[16];                 // Codec to use (h264, h265)
    char quality[16];               // Quality setting (low, medium, high)
    int retention_days;             // Days to keep recordings
} motion_recording_config_t;

/**
 * Initialize the ONVIF motion recording system
 * 
 * @return 0 on success, non-zero on failure
 */
int init_onvif_motion_recording(void);

/**
 * Cleanup the ONVIF motion recording system
 */
void cleanup_onvif_motion_recording(void);

/**
 * Enable motion recording for a stream
 * 
 * @param stream_name Name of the stream
 * @param config Recording configuration
 * @return 0 on success, non-zero on failure
 */
int enable_motion_recording(const char *stream_name, const motion_recording_config_t *config);

/**
 * Disable motion recording for a stream
 * 
 * @param stream_name Name of the stream
 * @return 0 on success, non-zero on failure
 */
int disable_motion_recording(const char *stream_name);

/**
 * Process a motion event (called by ONVIF detection system)
 * 
 * @param stream_name Name of the stream
 * @param motion_detected Whether motion was detected
 * @param timestamp Event timestamp
 * @return 0 on success, non-zero on failure
 */
int process_motion_event(const char *stream_name, bool motion_detected, time_t timestamp);

/**
 * Get recording state for a stream
 * 
 * @param stream_name Name of the stream
 * @return Current recording state, or RECORDING_STATE_IDLE if not found
 */
recording_state_t get_motion_recording_state(const char *stream_name);

/**
 * Get recording statistics for a stream
 * 
 * @param stream_name Name of the stream
 * @param total_recordings Output: total number of recordings
 * @param total_events Output: total number of motion events
 * @return 0 on success, non-zero on failure
 */
int get_motion_recording_stats(const char *stream_name, uint64_t *total_recordings, uint64_t *total_events);

/**
 * Update motion recording configuration for a stream
 * 
 * @param stream_name Name of the stream
 * @param config New configuration
 * @return 0 on success, non-zero on failure
 */
int update_motion_recording_config(const char *stream_name, const motion_recording_config_t *config);

/**
 * Check if motion recording is enabled for a stream
 * 
 * @param stream_name Name of the stream
 * @return true if enabled, false otherwise
 */
bool is_motion_recording_enabled(const char *stream_name);

/**
 * Get current recording file path for a stream
 * 
 * @param stream_name Name of the stream
 * @param path Output buffer for file path
 * @param path_size Size of output buffer
 * @return 0 on success, non-zero on failure
 */
int get_current_motion_recording_path(const char *stream_name, char *path, size_t path_size);

/**
 * Force stop recording for a stream (for emergency shutdown)
 *
 * @param stream_name Name of the stream
 * @return 0 on success, non-zero on failure
 */
int force_stop_motion_recording(const char *stream_name);

/**
 * Feed a video packet to the motion recording buffer
 * This should be called for every video packet to maintain the pre-event buffer
 *
 * @param stream_name Name of the stream
 * @param packet Video packet to buffer
 * @return 0 on success, non-zero on failure
 */
int feed_packet_to_event_buffer(const char *stream_name, const AVPacket *packet);

/**
 * Get buffer statistics for a stream
 *
 * @param stream_name Name of the stream
 * @param packet_count Output: number of packets in buffer
 * @param memory_usage Output: memory usage in bytes
 * @param duration Output: duration of buffered content in seconds
 * @return 0 on success, non-zero on failure
 */
int get_event_buffer_stats(const char *stream_name, int *packet_count, size_t *memory_usage, int *duration);

#endif /* LIGHTNVR_ONVIF_MOTION_RECORDING_H */

