#ifndef LIGHTNVR_STREAM_STATE_H
#define LIGHTNVR_STREAM_STATE_H

#include <stdint.h>
#include <stdbool.h>
#include <pthread.h>
#include "core/config.h"
#include "video/stream_manager.h"

/**
 * Stream state enum - represents the current operational state of a stream
 */
typedef enum {
    STREAM_STATE_INACTIVE = 0,   // Stream is not active
    STREAM_STATE_STARTING,       // Stream is in the process of starting
    STREAM_STATE_ACTIVE,         // Stream is active and running
    STREAM_STATE_ERROR,          // Stream encountered an error
    STREAM_STATE_RECONNECTING    // Stream is attempting to reconnect
} stream_state_t;

/**
 * Stream feature flags - represents which features are enabled for a stream
 */
typedef struct {
    bool streaming_enabled;      // HLS streaming is enabled
    bool recording_enabled;      // Recording is enabled
    bool detection_enabled;      // Object detection is enabled
    bool motion_detection_enabled; // Motion detection is enabled
} stream_features_t;

/**
 * Stream protocol state - contains protocol-specific settings and state
 */
typedef struct {
    stream_protocol_t protocol;  // Current protocol (TCP or UDP)
    bool is_multicast;           // Whether this is a multicast stream
    int reconnect_attempts;      // Number of reconnection attempts
    int64_t last_reconnect_time; // Timestamp of last reconnection attempt
    int buffer_size;             // Protocol-specific buffer size
    int timeout_ms;              // Protocol-specific timeout in milliseconds
} stream_protocol_state_t;

/**
 * Stream timestamp state - for handling timestamps in different protocols
 */
typedef struct {
    int64_t last_pts;            // Last presentation timestamp
    int64_t last_dts;            // Last decoding timestamp
    int64_t expected_next_pts;   // Expected next PTS
    int64_t pts_discontinuity_count; // Count of PTS discontinuities
    bool timestamps_initialized; // Whether timestamps have been initialized
} stream_timestamp_state_t;

// Use stream_stats_t from stream_manager.h

/**
 * Stream state manager - central structure for managing stream state
 */
typedef struct {
    char name[MAX_STREAM_NAME];  // Stream name
    stream_state_t state;        // Current operational state
    stream_features_t features;  // Enabled features
    stream_protocol_state_t protocol_state; // Protocol-specific state
    stream_timestamp_state_t timestamp_state; // Timestamp handling state
    stream_stats_t stats;        // Stream statistics
    stream_config_t config;      // Stream configuration
    pthread_mutex_t mutex;       // Mutex for thread-safe access
    void *reader_ctx;            // Opaque pointer to reader context
    void *hls_ctx;               // Opaque pointer to HLS context
    void *mp4_ctx;               // Opaque pointer to MP4 context
    void *detection_ctx;         // Opaque pointer to detection context
} stream_state_manager_t;

/**
 * Initialize the stream state management system
 * 
 * @param max_streams Maximum number of streams to support
 * @return 0 on success, non-zero on failure
 */
int init_stream_state_manager(int max_streams);

/**
 * Shutdown the stream state management system
 */
void shutdown_stream_state_manager(void);

/**
 * Create a new stream state manager
 * 
 * @param config Stream configuration
 * @return Pointer to stream state manager on success, NULL on failure
 */
stream_state_manager_t *create_stream_state(const stream_config_t *config);

/**
 * Get stream state manager by name
 * 
 * @param name Stream name
 * @return Pointer to stream state manager on success, NULL if not found
 */
stream_state_manager_t *get_stream_state_by_name(const char *name);

/**
 * Update stream state configuration
 * This function safely updates the configuration and propagates changes to all components
 * 
 * @param state Stream state manager
 * @param config New configuration
 * @return 0 on success, non-zero on failure
 */
int update_stream_state_config(stream_state_manager_t *state, const stream_config_t *config);

/**
 * Update stream protocol
 * This function safely changes the protocol and handles all necessary state transitions
 * 
 * @param state Stream state manager
 * @param protocol New protocol
 * @return 0 on success, non-zero on failure
 */
int update_stream_protocol(stream_state_manager_t *state, stream_protocol_t protocol);

/**
 * Set stream feature state
 * 
 * @param state Stream state manager
 * @param feature Feature to set (streaming, recording, detection, motion_detection)
 * @param enabled Whether the feature should be enabled
 * @return 0 on success, non-zero on failure
 */
int set_stream_feature(stream_state_manager_t *state, const char *feature, bool enabled);

/**
 * Start stream
 * This function starts the stream with all enabled features
 * 
 * @param state Stream state manager
 * @return 0 on success, non-zero on failure
 */
int start_stream_with_state(stream_state_manager_t *state);

/**
 * Stop stream
 * This function stops the stream and all its features
 * 
 * @param state Stream state manager
 * @return 0 on success, non-zero on failure
 */
int stop_stream_with_state(stream_state_manager_t *state);

/**
 * Get stream state
 * 
 * @param state Stream state manager
 * @return Current stream state
 */
stream_state_t get_stream_operational_state(stream_state_manager_t *state);

/**
 * Get stream statistics
 * 
 * @param state Stream state manager
 * @param stats Pointer to statistics structure to fill
 * @return 0 on success, non-zero on failure
 */
int get_stream_statistics(stream_state_manager_t *state, stream_stats_t *stats);

/**
 * Handle stream error
 * This function updates the stream state and initiates recovery if appropriate
 * 
 * @param state Stream state manager
 * @param error_code Error code
 * @param error_message Error message
 * @return 0 on success, non-zero on failure
 */
int handle_stream_error(stream_state_manager_t *state, int error_code, const char *error_message);

/**
 * Update stream timestamp state
 * This function is called by the packet processing code to update timestamp tracking
 * 
 * @param state Stream state manager
 * @param pts Presentation timestamp
 * @param dts Decoding timestamp
 * @return 0 on success, non-zero on failure
 */
int update_stream_timestamps(stream_state_manager_t *state, int64_t pts, int64_t dts);

/**
 * Get total number of streams
 * 
 * @return Number of streams
 */
int get_stream_state_count(void);

/**
 * Get stream state manager by index
 * 
 * @param index Stream index
 * @return Pointer to stream state manager on success, NULL if index is out of range
 */
stream_state_manager_t *get_stream_state_by_index(int index);

/**
 * Remove stream state manager
 * 
 * @param state Stream state manager
 * @return 0 on success, non-zero on failure
 */
int remove_stream_state(stream_state_manager_t *state);

#endif // LIGHTNVR_STREAM_STATE_H
