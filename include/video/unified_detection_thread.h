/**
 * Unified Detection Recording Thread
 * 
 * This module implements a unified thread that handles:
 * - Continuous RTSP packet reading from go2rtc
 * - Circular buffer for pre-detection content
 * - Object detection on keyframes
 * - MP4 recording with proper pre/post buffer support
 * 
 * The key innovation is that a single thread manages the entire pipeline,
 * ensuring that pre-buffer content is always available when detection triggers.
 */

#ifndef UNIFIED_DETECTION_THREAD_H
#define UNIFIED_DETECTION_THREAD_H

#include <pthread.h>
#include <stdatomic.h>
#include <stdbool.h>
#include <time.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>

#include "core/config.h"
#include "video/packet_buffer.h"
#include "video/detection_model.h"
#include "video/mp4_writer.h"
#include "video/stream_manager.h"

// Maximum number of unified detection threads
#define MAX_UNIFIED_DETECTION_THREADS MAX_STREAMS

/**
 * Thread state machine states
 */
typedef enum {
    UDT_STATE_INITIALIZING = 0,  // Thread starting up
    UDT_STATE_CONNECTING,        // Connecting to RTSP stream
    UDT_STATE_BUFFERING,         // Connected, buffering packets, running detection
    UDT_STATE_RECORDING,         // Detection triggered, recording to MP4
    UDT_STATE_POST_BUFFER,       // Detection ended, recording post-buffer
    UDT_STATE_RECONNECTING,      // Lost connection, attempting reconnect
    UDT_STATE_STOPPING,          // Thread shutting down
    UDT_STATE_STOPPED            // Thread has stopped
} unified_detection_state_t;

/**
 * Unified Detection Thread Context
 * 
 * Contains all state needed for a single stream's detection and recording.
 */
typedef struct {
    // Stream identification
    char stream_name[MAX_STREAM_NAME];
    char rtsp_url[MAX_PATH_LENGTH];
    char output_dir[MAX_PATH_LENGTH];
    
    // Thread management
    pthread_t thread;
    atomic_int running;
    atomic_int state;  // Uses unified_detection_state_t values
    int shutdown_component_id;
    int slot_idx;  // Index in the global detection_contexts array
    
    // Detection configuration
    char model_path[MAX_PATH_LENGTH];
    detection_model_t model;
    float detection_threshold;
    int detection_interval;  // Seconds between detection checks
    
    // Buffer configuration
    int pre_buffer_seconds;   // Seconds to keep before detection
    int post_buffer_seconds;  // Seconds to record after last detection
    int segment_duration;     // Maximum segment duration in seconds (for chunking detection recordings)
    
    // Circular buffer for pre-detection content
    packet_buffer_t *packet_buffer;
    
    // MP4 recording
    mp4_writer_t *mp4_writer;
    char current_recording_path[MAX_PATH_LENGTH];
    uint64_t current_recording_id;
    
    // Detection state
    atomic_llong last_detection_time;      // When last detection occurred (stored as atomic epoch seconds)
    atomic_llong last_detection_check_time; // When last detection check was attempted (for time-based interval)
    atomic_llong post_buffer_end_time;     // When post-buffer recording should end
    atomic_int log_counter;          // Counter for periodic logging; intentionally accessed without ctx->mutex,
                                     // but all accesses must use atomic operations, and exact accuracy is not critical.
    
    // Connection state
    atomic_int_fast64_t last_packet_time;
    atomic_int consecutive_failures;
    int reconnect_attempt;
    
    // Audio recording configuration
    bool record_audio;  // Whether to include audio in recordings

    // Annotation-only mode: when true, detection runs but does NOT create separate MP4 files
    // Detections are stored in the database and linked to the continuous recording
    bool annotation_only;

    // FFmpeg contexts (managed exclusively by the unified detection thread).
    // Concurrency / lifetime:
    // - These pointers and stream indices are created, updated, and freed only
    //   by the unified detection thread during connect/reconnect/shutdown.
    // - Other threads MUST NOT read or modify these fields directly unless they
    //   first acquire the appropriate synchronization primitive (for example,
    //   the context mutex defined alongside this struct in the implementation).
    // - Accessing these fields without synchronization while the detection
    //   thread may be reconnecting or shutting down can lead to data races or
    //   use-after-free of the FFmpeg contexts.
    AVFormatContext *input_ctx;
    AVCodecContext *decoder_ctx;
    int video_stream_idx;
    int audio_stream_idx;
    
    // Statistics
    uint64_t total_packets_processed;
    uint64_t total_detections;
    uint64_t total_recordings;

    // Runtime FPS measurement (used when SDP omits frame rate). After enough frames
    // have been observed in the measurement window, the measured FPS is finalized,
    // fps_is_provisional is set to false, and the resulting FPS value is persisted
    // to the database for future use.
    atomic_bool fps_is_provisional;           // true when stored FPS is a fallback guess
    atomic_int fps_measurement_frame_count;   // video frames counted during measurement window
    atomic_llong fps_measurement_start_ns;    // start of measurement window, in nanoseconds since an epoch

    // Thread safety
    pthread_mutex_t mutex;
} unified_detection_ctx_t;

/**
 * Initialize the unified detection thread system
 * Must be called before starting any threads
 * 
 * @return 0 on success, -1 on error
 */
int init_unified_detection_system(void);

/**
 * Shutdown the unified detection thread system
 * Stops all running threads and cleans up resources
 */
void shutdown_unified_detection_system(void);

/**
 * Start unified detection recording for a stream
 *
 * @param stream_name Name of the stream
 * @param model_path Path to detection model
 * @param threshold Detection confidence threshold (0.0-1.0)
 * @param pre_buffer_seconds Seconds of pre-detection buffer
 * @param post_buffer_seconds Seconds of post-detection recording
 * @param annotation_only If true, detection runs but does NOT create separate MP4 files.
 *                        Detections are stored in DB and linked to the continuous recording.
 * @return 0 on success, -1 on error
 */
int start_unified_detection_thread(const char *stream_name, const char *model_path,
                                   float threshold, int pre_buffer_seconds,
                                   int post_buffer_seconds, bool annotation_only);

/**
 * Stop unified detection recording for a stream
 * 
 * @param stream_name Name of the stream
 * @return 0 on success, -1 on error
 */
int stop_unified_detection_thread(const char *stream_name);

/**
 * Check if unified detection is running for a stream
 * 
 * @param stream_name Name of the stream
 * @return true if running, false otherwise
 */
bool is_unified_detection_running(const char *stream_name);

/**
 * Get the current state of a unified detection thread
 *
 * @param stream_name Name of the stream
 * @return Current state, or UDT_STATE_STOPPED if not found
 */
unified_detection_state_t get_unified_detection_state(const char *stream_name);

/**
 * Get the effective stream status based on the UDT state for API reporting.
 *
 * Maps the internal UDT state (and reconnect attempt count) to a
 * stream_status_t value so callers can report an accurate status without
 * needing access to the raw UDT context.
 *
 * Mapping:
 *  UDT_STATE_INITIALIZING               -> STREAM_STATUS_STARTING
 *  UDT_STATE_CONNECTING (attempt == 0)  -> STREAM_STATUS_STARTING
 *  UDT_STATE_CONNECTING (attempt  > 0)  -> STREAM_STATUS_RECONNECTING
 *  UDT_STATE_BUFFERING / RECORDING / POST_BUFFER -> STREAM_STATUS_RUNNING
 *  UDT_STATE_RECONNECTING               -> STREAM_STATUS_RECONNECTING
 *  UDT_STATE_STOPPING                   -> STREAM_STATUS_STOPPING
 *  UDT_STATE_STOPPED (or not found)     -> STREAM_STATUS_STOPPED
 *
 * @param stream_name Name of the stream
 * @return Appropriate stream_status_t value
 */
stream_status_t get_unified_detection_effective_status(const char *stream_name);

/**
 * Get the number of reconnect attempts made by a unified detection thread.
 *
 * This counter increments each time the UDT fails to open the RTSP stream
 * and transitions through UDT_STATE_RECONNECTING back to UDT_STATE_CONNECTING.
 * A value > 0 means the initial connection was never established (or was lost),
 * making it a reliable proxy for "how many times has this stream failed" for
 * go2rtc-managed streams where the state manager stays at STREAM_STATE_INACTIVE.
 *
 * @param stream_name Name of the stream
 * @return Reconnect attempt count, or 0 if not found
 */
int get_unified_detection_reconnect_attempts(const char *stream_name);

/**
 * Get statistics for a unified detection thread
 *
 * @param stream_name Name of the stream
 * @param packets_processed Output: total packets processed
 * @param detections Output: total detections
 * @param recordings Output: total recordings created
 * @return 0 on success, -1 if not found
 */
int get_unified_detection_stats(const char *stream_name,
                                uint64_t *packets_processed,
                                uint64_t *detections,
                                uint64_t *recordings);

#endif /* UNIFIED_DETECTION_THREAD_H */

